//! Word's positional table formula fields (ECMA-376 §17.16.3.4–5).
//!
//! Resolve the complete table in Word's print-update order. A single formula
//! reads current cell values; batch field scheduling owns dependency updates.
use smallvec::SmallVec;

use super::model::{Block, InlineItem, ParagraphFieldEvent, Table, TableCell, TextStyle};
use crate::localization::OfficeNumberSeparators;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Aggregate {
  Sum,
  Average,
  Count,
  Product,
  Min,
  Max,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
  Left,
  Right,
  Above,
  Below,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Formula {
  function: Aggregate,
  directions: SmallVec<[Direction; 2]>,
}

impl Formula {
  pub(super) fn parse(instruction: &str) -> Option<Self> {
    let expression = instruction.trim().strip_prefix('=')?.trim();
    let (name, arguments) = expression.split_once('(')?;
    let function = match name.trim().to_ascii_uppercase().as_str() {
      "SUM" => Aggregate::Sum,
      "AVERAGE" => Aggregate::Average,
      "COUNT" => Aggregate::Count,
      "PRODUCT" => Aggregate::Product,
      "MIN" => Aggregate::Min,
      "MAX" => Aggregate::Max,
      _ => return None,
    };
    let (arguments, suffix) = arguments.split_once(')')?;
    if !suffix.trim().is_empty() && !suffix.trim().eq_ignore_ascii_case(r"\* MERGEFORMAT") {
      return None;
    }
    let directions = arguments
      .split(',')
      .map(
        |argument| match argument.trim().to_ascii_uppercase().as_str() {
          "LEFT" => Some(Direction::Left),
          "RIGHT" => Some(Direction::Right),
          "ABOVE" => Some(Direction::Above),
          "BELOW" => Some(Direction::Below),
          _ => None,
        },
      )
      .collect::<Option<SmallVec<_>>>()?;
    Some(Self {
      function,
      directions,
    })
  }
}

#[derive(Clone, Copy, Debug, Default)]
struct Number {
  value: f64,
  decimals: usize,
  grouped: bool,
}

fn cell_number(text: &str, symbols: OfficeNumberSeparators) -> Option<Number> {
  let text = text.trim();
  let (text, percent) = text
    .strip_suffix('%')
    .map_or((text, false), |s| (s.trim(), true));
  if text.is_empty() {
    return None;
  }
  let integral = text.split(symbols.decimal).next()?;
  let mut groups = integral.split(symbols.grouping);
  let first = groups.next()?;
  let grouped = integral.contains(symbols.grouping)
    && !first.trim_start_matches(['+', '-']).is_empty()
    && groups.all(|group| group.len() == 3 && group.bytes().all(|b| b.is_ascii_digit()));
  if !grouped && text.contains(symbols.grouping) {
    // Word accepts a numeric list inside a cell: "1,5" in an English-number
    // environment contributes 1+5, while a valid "1,234" is one grouped number.
    let mut result = Number::default();
    for part in text.split(symbols.grouping) {
      let part = cell_number(part, symbols)?;
      result.value += part.value;
      result.decimals = result.decimals.max(part.decimals);
    }
    if percent {
      result.value /= 100.0;
      result.decimals = (result.decimals + 2).min(15);
    }
    return result.value.is_finite().then_some(result);
  }
  let mut normalized = String::with_capacity(text.len());
  for (index, ch) in text.chars().enumerate() {
    if ch == symbols.grouping && grouped {
      continue;
    }
    if ch == symbols.decimal {
      normalized.push('.');
    } else if ch.is_ascii_digit() || (index == 0 && matches!(ch, '+' | '-')) {
      normalized.push(ch);
    } else {
      return None;
    }
  }
  let mut value = normalized.parse::<f64>().ok()?;
  let mut decimals = normalized.split_once('.').map_or(0, |(_, part)| part.len());
  if percent {
    value /= 100.0;
    decimals += 2;
  }
  value.is_finite().then_some(Number {
    value,
    decimals: decimals.min(15),
    grouped,
  })
}

fn format_number(number: Number, symbols: OfficeNumberSeparators) -> String {
  let scale = 10_f64.powi(number.decimals as i32);
  let scaled = number.value * scale;
  let rounded = if scaled.is_finite() {
    scaled.round() / scale
  } else {
    number.value
  };
  let rounded = if rounded == 0.0 { 0.0 } else { rounded };
  let mut text = format!("{rounded:.precision$}", precision = number.decimals);
  if text.contains('.') {
    while text.ends_with('0') {
      text.pop();
    }
    if text.ends_with('.') {
      text.pop();
    }
  }
  let (integral, fractional) = text
    .split_once('.')
    .map_or((text.as_str(), None), |(a, b)| (a, Some(b)));
  let mut output = String::with_capacity(text.len() + text.len() / 3);
  for (index, ch) in integral.chars().enumerate() {
    if number.grouped
      && index > 0
      && ch.is_ascii_digit()
      && integral.as_bytes()[index - 1].is_ascii_digit()
      && (integral.len() - index).is_multiple_of(3)
    {
      output.push(symbols.grouping);
    }
    output.push(ch);
  }
  if let Some(fractional) = fractional {
    output.push(symbols.decimal);
    output.push_str(fractional);
  }
  output
}

struct Replacement {
  paragraph: usize,
  start: usize,
  end: usize,
  text: String,
  style: TextStyle,
}

struct Evaluation<'a> {
  cells: Vec<&'a TableCell>,
  columns: usize,
  values: Vec<Option<Number>>,
  replacements: Vec<Vec<Replacement>>,
  symbols: OfficeNumberSeparators,
}

impl Evaluation<'_> {
  fn cell(&mut self, index: usize, update_fields: bool) -> Result<(), ()> {
    let cell = self.cells[index];
    let mut text = String::new();
    for (paragraph_index, block) in cell.blocks.iter().enumerate() {
      let Block::Paragraph(paragraph) = block else {
        // A nested table is a different formula coordinate space.
        text.push('\n');
        continue;
      };
      if paragraph_index != 0 {
        text.push('\n');
      }
      let mut fields = paragraph
        .field_events
        .iter()
        .filter(|_| update_fields)
        .filter_map(|event| match event {
          ParagraphFieldEvent::FormulaResultSpan {
            formula,
            style,
            inline_start,
            inline_end,
          } => Some((*inline_start, *inline_end, formula, style)),
          _ => None,
        })
        .collect::<Vec<_>>();
      fields.sort_by_key(|(start, _, _, _)| *start);
      let mut fields = fields.into_iter().peekable();
      let mut cursor = 0;
      while cursor < paragraph.inlines.len() {
        if let Some(&(start, end, formula, style)) = fields.peek()
          && start == cursor
          && end > start
          && end <= paragraph.inlines.len()
        {
          let value = self.formula(index, formula)?;
          text.push_str(&value);
          self.replacements[index].push(Replacement {
            paragraph: paragraph_index,
            start,
            end,
            text: value,
            style: style.as_ref().clone(),
          });
          cursor = end;
          fields.next();
        } else {
          match &paragraph.inlines[cursor] {
            InlineItem::Text(run) => text.push_str(&run.text),
            _ => text.push('\n'),
          }
          cursor += 1;
        }
      }
    }
    self.values[index] = cell_number(&text, self.symbols);
    Ok(())
  }

  fn range(&self, owner: usize, direction: Direction) -> SmallVec<[usize; 8]> {
    let mut cells = SmallVec::new();
    let vertical = matches!(direction, Direction::Above | Direction::Below);
    let mut index = owner;
    let mut found = false;
    loop {
      let next = match direction {
        Direction::Left => (!index.is_multiple_of(self.columns)).then(|| index - 1),
        Direction::Right => (index % self.columns + 1 < self.columns).then(|| index + 1),
        Direction::Above => index.checked_sub(self.columns),
        Direction::Below => (index + self.columns < self.cells.len()).then(|| index + self.columns),
      };
      let Some(next) = next else { break };
      index = next;
      if self.values[index].is_some() {
        cells.push(index);
        found = true;
      } else if found {
        break;
      } else if vertical {
        // Leading vertical nonnumeric cells contribute zero; horizontal
        // ranges skip them. Once a number is found, either range terminates.
        cells.push(index);
      }
    }
    cells
  }

  fn dependencies(&self, formula_cells: &[bool]) -> Vec<SmallVec<[usize; 4]>> {
    self
      .cells
      .iter()
      .enumerate()
      .map(|(owner, cell)| {
        let mut dependencies = SmallVec::new();
        for block in &cell.blocks {
          let Block::Paragraph(paragraph) = block else {
            continue;
          };
          for event in &paragraph.field_events {
            let ParagraphFieldEvent::FormulaResultSpan { formula, .. } = event else {
              continue;
            };
            for direction in &formula.directions {
              let range = self.range(owner, *direction);
              // Native batch updates schedule fields in single-row references.
              // A multi-row BELOW range reads the later fields' current caches.
              // Use the actual numeric range, respecting blank terminators,
              // rather than the direction token alone.
              if let (Some(first), Some(last)) = (range.first(), range.last())
                && first / self.columns == last / self.columns
              {
                dependencies.extend(range.into_iter().filter(|&index| formula_cells[index]));
              }
            }
          }
        }
        dependencies.sort_unstable();
        dependencies.dedup();
        dependencies
      })
      .collect()
  }

  fn refresh(&mut self, formula_cells: &[bool]) -> Result<(), ()> {
    // A single Field.Update consumes cached operands; Fields.Update determines
    // their order. Snapshot those operands before updating any field.
    for index in 0..self.cells.len() {
      self.cell(index, false)?;
    }
    let dependencies = self.dependencies(formula_cells);
    let mut states = vec![0_u8; self.cells.len()];
    let mut cursors = vec![0; self.cells.len()];
    let mut stack = SmallVec::<[usize; 8]>::new();
    for (root, &has_formula) in formula_cells.iter().enumerate() {
      if !has_formula || states[root] == 2 {
        continue;
      }
      states[root] = 1;
      stack.push(root);
      while let Some(&owner) = stack.last() {
        if let Some(&dependency) = dependencies[owner].get(cursors[owner]) {
          if states[dependency] == 1 {
            // Word defers the field which directly meets a back-reference.
            // Its waiting consumers finish using its old value, without
            // visiting their remaining dependencies during this unwind.
            // The deferred field gets its ordinary later turn. This differs
            // from both a circular-reference error and SCC fixed-point math.
            states[owner] = 0;
            stack.pop();
            while let Some(waiting) = stack.pop() {
              self.cell(waiting, true)?;
              states[waiting] = 2;
            }
            break;
          }
          cursors[owner] += 1;
          if states[dependency] == 0 {
            states[dependency] = 1;
            stack.push(dependency);
          }
        } else {
          self.cell(owner, true)?;
          states[owner] = 2;
          stack.pop();
        }
      }
    }
    Ok(())
  }

  fn formula(&self, owner: usize, formula: &Formula) -> Result<String, ()> {
    let mut values = SmallVec::<[Number; 8]>::new();
    let mut numeric = false;
    for direction in &formula.directions {
      for index in self.range(owner, *direction) {
        numeric |= self.values[index].is_some();
        values.push(self.values[index].unwrap_or_default());
      }
    }
    if matches!(formula.function, Aggregate::Average) && !numeric {
      return Ok(if values.is_empty() {
        String::new()
      } else {
        format!("0{}0", self.symbols.decimal)
      });
    }
    let decimals = values.iter().map(|n| n.decimals).max().unwrap_or(0);
    let mut number = Number {
      decimals: if matches!(formula.function, Aggregate::Average) {
        decimals.max(2)
      } else {
        decimals
      },
      grouped: values.iter().any(|n| n.grouped),
      ..Number::default()
    };
    number.value = match formula.function {
      Aggregate::Sum => values.iter().map(|n| n.value).sum(),
      Aggregate::Average => values.iter().map(|n| n.value).sum::<f64>() / values.len() as f64,
      Aggregate::Count => values.len() as f64,
      Aggregate::Product if !values.is_empty() => values.iter().map(|n| n.value).product(),
      Aggregate::Product => 0.0,
      Aggregate::Min => values
        .iter()
        .map(|n| n.value)
        .reduce(f64::min)
        .unwrap_or(0.0),
      Aggregate::Max => values
        .iter()
        .map(|n| n.value)
        .reduce(f64::max)
        .unwrap_or(0.0),
    };
    number
      .value
      .is_finite()
      .then(|| format_number(number, self.symbols))
      .ok_or(())
  }
}

pub(super) fn refresh(table: &mut Table, symbols: OfficeNumberSeparators) {
  let has_formula = |cell: &TableCell| {
    cell.blocks.iter().any(|block| {
      matches!(block, Block::Paragraph(p) if p.field_events.iter().any(|e|
      matches!(e, ParagraphFieldEvent::FormulaResultSpan { .. })))
    })
  };
  if !table
    .rows
    .iter()
    .flat_map(|row| &row.cells)
    .any(has_formula)
  {
    return;
  }
  let columns = table.rows.first().map_or(0, |row| row.cells.len());
  // Merged/skipped grids need their own Word cell-reference mapping. Preserve
  // their cache until that mapping is established rather than aliasing cells.
  if columns == 0
    || table.rows.iter().any(|row| {
      row.cells.len() != columns
        || row.grid_before != 0
        || row.grid_after != 0
        || row
          .cells
          .iter()
          .any(|cell| cell.grid_span != 1 || cell.vertical_merge_continue)
    })
  {
    return;
  }
  let replacements = {
    let cells = table
      .rows
      .iter()
      .flat_map(|row| &row.cells)
      .collect::<Vec<_>>();
    let count = cells.len();
    let formula_cells = cells
      .iter()
      .map(|cell| has_formula(cell))
      .collect::<Vec<_>>();
    let mut evaluation = Evaluation {
      cells,
      columns,
      values: vec![None; count],
      replacements: (0..count).map(|_| Vec::new()).collect(),
      symbols,
    };
    if evaluation.refresh(&formula_cells).is_err() {
      return;
    }
    evaluation.replacements
  };
  for (cell, replacements) in table
    .rows
    .iter_mut()
    .flat_map(|row| &mut row.cells)
    .zip(replacements)
  {
    for replacement in replacements {
      let Block::Paragraph(paragraph) = &mut cell.blocks[replacement.paragraph] else {
        continue;
      };
      // Keep inline indices stable for neighboring REF/bookmark metadata.
      for (offset, inline) in paragraph.inlines[replacement.start..replacement.end]
        .iter_mut()
        .enumerate()
      {
        if let InlineItem::Text(run) = inline {
          run.text = if offset == 0 {
            replacement.text.clone()
          } else {
            String::new()
          };
          run.style = replacement.style.clone();
          run.style.wordprocessingml_field_group = true;
          run.style_ref_text = None;
          run.style_ref_numbering_text = None;
        }
      }
    }
    for block in &mut cell.blocks {
      if let Block::Paragraph(paragraph) = block {
        paragraph
          .field_events
          .retain(|e| !matches!(e, ParagraphFieldEvent::FormulaResultSpan { .. }));
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::LayoutOptions;
  use ooxmlsdk::parts::wordprocessing_document::WordprocessingDocument;
  use ooxmlsdk::sdk::WordprocessingDocumentType;

  fn text(value: &str) -> String {
    format!("<w:r><w:t>{value}</w:t></w:r>")
  }

  fn field(instruction: &str, cached: &str, simple: bool, locked: bool) -> String {
    let lock = if locked { r#" w:fldLock="1""# } else { "" };
    let result = format!(r#"<w:r><w:rPr><w:sz w:val="40"/></w:rPr><w:t>{cached}</w:t></w:r>"#);
    if simple {
      format!(r#"<w:fldSimple w:instr="{instruction}"{lock}>{result}</w:fldSimple>"#)
    } else {
      format!(
        r#"<w:r><w:fldChar w:fldCharType="begin"{lock}/></w:r><w:r><w:rPr><w:sz w:val="16"/></w:rPr><w:instrText>{instruction}</w:instrText></w:r><w:r><w:fldChar w:fldCharType="separate"/></w:r>{result}<w:r><w:fldChar w:fldCharType="end"/></w:r>"#
      )
    }
  }

  fn imported(rows: &[Vec<String>]) -> Table {
    imported_with_print_update(rows, None)
  }

  fn imported_with_print_update(
    rows: &[Vec<String>],
    update_fields_at_print: Option<bool>,
  ) -> Table {
    let mut package = WordprocessingDocument::create(WordprocessingDocumentType::Document);
    let main = package.add_main_document_part().unwrap();
    let grid = r#"<w:gridCol w:w="2000"/>"#.repeat(rows[0].len());
    let rows = rows
      .iter()
      .map(|row| {
        let cells = row
          .iter()
          .map(|value| {
            format!(
              r#"<w:tc><w:p><w:pPr><w:rPr><w:sz w:val="22"/></w:rPr></w:pPr>{value}</w:p></w:tc>"#
            )
          })
          .collect::<String>();
        format!("<w:tr>{cells}</w:tr>")
      })
      .collect::<String>();
    main.set_data(&mut package, format!(r#"<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:body><w:tbl><w:tblGrid>{grid}</w:tblGrid>{rows}</w:tbl></w:body></w:document>"#).into_bytes()).unwrap();
    let document = super::super::extract(
      &package,
      &LayoutOptions {
        update_fields_at_print,
        format_locale: Some("en-US".into()),
        ..Default::default()
      },
    )
    .unwrap();
    let Block::Table(table) = document
      .sections
      .into_iter()
      .next()
      .unwrap()
      .blocks
      .into_iter()
      .next()
      .unwrap()
    else {
      panic!("expected imported table");
    };
    table
  }

  fn cell_text(table: &Table, row: usize, column: usize) -> String {
    table.rows[row].cells[column]
      .blocks
      .iter()
      .filter_map(|block| {
        let Block::Paragraph(p) = block else {
          return None;
        };
        Some(
          p.inlines
            .iter()
            .filter_map(|inline| match inline {
              InlineItem::Text(run) => Some(run.text.as_str()),
              _ => None,
            })
            .collect::<String>(),
        )
      })
      .collect()
  }

  #[test]
  fn print_field_update_controls_cached_table_formulas() {
    for enabled in [false, true] {
      for locked in [false, true] {
        for simple in [false, true] {
          let table = imported_with_print_update(
            &[vec![
              text("2"),
              text("3"),
              field("=SUM(LEFT)", "99", simple, locked),
            ]],
            Some(enabled),
          );
          assert_eq!(
            cell_text(&table, 0, 2),
            if enabled && !locked { "5" } else { "99" }
          );
        }
      }
    }
  }

  #[test]
  fn positional_ranges_follow_native_word_blank_and_numeric_rules() {
    // Native Word controls cover all four directions, not just a horizontal
    // approximation. Expected columns: SUM, AVERAGE, COUNT, PRODUCT, MIN, MAX.
    let functions = ["SUM", "AVERAGE", "COUNT", "PRODUCT", "MIN", "MAX"];
    for (values, horizontal, vertical) in [
      (
        vec!["3", "5", "8"],
        ["16", "5.33", "3", "120", "3", "8"],
        ["16", "5.33", "3", "120", "3", "8"],
      ),
      (
        vec!["", "3", "5", "8"],
        ["16", "5.33", "3", "120", "3", "8"],
        ["16", "4", "4", "0", "0", "8"],
      ),
      (
        vec!["3", "", "5", "8"],
        ["3", "3", "1", "3", "3", "3"],
        ["3", "3", "1", "3", "3", "3"],
      ),
      (
        vec!["3", "text", "5"],
        ["3", "3", "1", "3", "3", "3"],
        ["3", "3", "1", "3", "3", "3"],
      ),
      (
        vec!["text", "3"],
        ["3", "3", "1", "3", "3", "3"],
        ["3", "1.5", "2", "0", "0", "3"],
      ),
      (
        vec!["", "", ""],
        ["0", "", "0", "0", "0", "0"],
        ["0", "0.0", "3", "0", "0", "0"],
      ),
      (
        vec!["400%", "8"],
        ["12", "6", "2", "32", "4", "8"],
        ["12", "6", "2", "32", "4", "8"],
      ),
      (
        vec!["1,5", "2,5"],
        ["13", "6.5", "2", "42", "6", "7"],
        ["13", "6.5", "2", "42", "6", "7"],
      ),
      (
        vec!["1.5", "2.5"],
        ["4", "2", "2", "3.8", "1.5", "2.5"],
        ["4", "2", "2", "3.8", "1.5", "2.5"],
      ),
    ] {
      for (direction, vertical_range, reverse) in [
        ("LEFT", false, true),
        ("RIGHT", false, false),
        ("ABOVE", true, true),
        ("BELOW", true, false),
      ] {
        for (i, function) in functions.iter().enumerate() {
          let mut cells = vec![field(
            &format!("={function}({direction})"),
            "999",
            false,
            false,
          )];
          cells.extend(values.iter().map(|value| text(value)));
          if reverse {
            cells.reverse();
          }
          let position = if reverse { cells.len() - 1 } else { 0 };
          let rows = if vertical_range {
            cells.into_iter().map(|cell| vec![cell]).collect()
          } else {
            vec![cells]
          };
          let table = imported(&rows);
          let actual = if vertical_range {
            cell_text(&table, position, 0)
          } else {
            cell_text(&table, 0, position)
          };
          assert_eq!(
            actual,
            if vertical_range {
              vertical[i]
            } else {
              horizontal[i]
            },
            "{function}({direction}), {values:?}"
          );
        }
      }
    }
  }

  #[test]
  fn forward_dependencies_use_recalculated_values_and_respect_locks() {
    for simple in [false, true] {
      for locked in [false, true] {
        let table = imported(&[vec![
          field("=SUM(RIGHT)", "111", simple, false),
          field("=SUM(RIGHT)", "999", simple, locked),
          text("5"),
        ]]);
        assert_eq!(cell_text(&table, 0, 0), if locked { "1004" } else { "10" });
        assert_eq!(cell_text(&table, 0, 1), if locked { "999" } else { "5" });
        let table = imported(&[vec![
          text("5"),
          field("=SUM(LEFT)", "999", simple, locked),
          field("=SUM(LEFT)", "111", simple, false),
        ]]);
        assert_eq!(cell_text(&table, 0, 2), if locked { "1004" } else { "10" });
      }
    }
  }

  #[test]
  fn formula_result_keeps_surrounding_runs_and_mergeformat_style() {
    for simple in [false, true] {
      for merge in [false, true] {
        for cache in ["", "999"] {
          let instruction = if merge {
            r"=SUM(LEFT) \* MERGEFORMAT"
          } else {
            "=SUM(LEFT)"
          };
          let table = imported(&[vec![
            text("2"),
            text("3"),
            format!(
              "{}{}{}",
              text("PREFIX|"),
              field(instruction, cache, simple, false),
              text("|SUFFIX")
            ),
          ]]);
          assert_eq!(cell_text(&table, 0, 2), "PREFIX|5|SUFFIX");
          let Block::Paragraph(p) = &table.rows[0].cells[2].blocks[0] else {
            panic!()
          };
          let run = p
            .inlines
            .iter()
            .find_map(|inline| match inline {
              InlineItem::Text(run) if run.text == "5" => Some(run),
              _ => None,
            })
            .unwrap();
          // An empty source run has no cache formatting to merge.
          if !cache.is_empty() || !merge {
            assert_eq!(
              run.style.font_size_pt,
              if merge {
                20.0
              } else if simple {
                11.0
              } else {
                8.0
              }
            );
          }
          assert!(run.style.wordprocessingml_field_group);
        }
      }
    }
  }

  #[test]
  fn cell_numeric_tokens_and_precision_match_native_word() {
    for (input, value, decimals, grouped) in [
      ("1,234", 1234.0, 0, true),
      ("1,234.5", 1234.5, 1, true),
      ("1,5", 6.0, 0, false),
      ("50%", 0.5, 2, false),
      ("1.25", 1.25, 2, false),
      ("-3", -3.0, 0, false),
    ] {
      let number = cell_number(input, OfficeNumberSeparators::default()).unwrap();
      assert_eq!(
        (number.value, number.decimals, number.grouped),
        (value, decimals, grouped)
      );
    }
    for input in [
      "3 apples",
      "apples 3",
      "3 apples 4",
      "(3)",
      "$3",
      "3;5",
      "3 5",
      "1e2",
      "1/2",
      "2+3",
    ] {
      assert!(
        cell_number(input, OfficeNumberSeparators::default()).is_none(),
        "{input}"
      );
    }
    for (function, a, b, expected) in [
      ("SUM", "1,234", "2", "1,236"),
      ("AVERAGE", "1.25", "2.5", "1.88"),
      ("PRODUCT", "1.25", "2.5", "3.13"),
      ("AVERAGE", "0.001", "2", "1.001"),
    ] {
      let table = imported(&[vec![
        text(a),
        text(b),
        field(&format!("={function}(LEFT)"), "999", false, false),
      ]]);
      assert_eq!(cell_text(&table, 0, 2), expected);
    }
  }

  #[test]
  fn combined_directions_and_cycle_caches_preserve_field_boundaries() {
    let table = imported(&[vec![
      text("2"),
      field("=SUM(LEFT,RIGHT)", "999", false, false),
      text("3"),
    ]]);
    assert_eq!(cell_text(&table, 0, 1), "5");
    let table = imported(&[vec![
      field("=SUM(RIGHT)", "111", false, false),
      field("=SUM(LEFT)", "222", false, false),
    ]]);
    assert_eq!(cell_text(&table, 0, 0), "222");
    assert_eq!(cell_text(&table, 0, 1), "222");
    for code in [
      "=A1+B1",
      "=SUM(LEFT;RIGHT)",
      r#"=SUM(LEFT) \# "0.00""#,
      "=NOTSUPPORTED(LEFT)",
    ] {
      assert!(Formula::parse(code).is_none(), "{code}");
    }
  }

  #[test]
  fn print_update_reads_later_multirow_values_before_their_turn() {
    for simple in [false, true] {
      for (function, expected) in [
        ("SUM", "1004"),
        ("AVERAGE", "502"),
        ("COUNT", "2"),
        ("PRODUCT", "4995"),
        ("MIN", "5"),
        ("MAX", "999"),
      ] {
        let table = imported(&[
          vec![field(&format!("={function}(BELOW)"), "111", simple, false)],
          vec![field(&format!("={function}(BELOW)"), "999", simple, false)],
          vec![text("5")],
        ]);
        assert_eq!(cell_text(&table, 0, 0), expected, "{function}");
        assert_eq!(
          cell_text(&table, 1, 0),
          if function == "COUNT" { "1" } else { "5" }
        );
      }
      // A blank terminates the referenced range before the third row, making
      // the first formula depend on the second row's still-unupdated formula.
      for last in ["", "text", "0"] {
        let table = imported(&[
          vec![field("=SUM(BELOW)", "111", simple, false), text("0")],
          vec![field("=SUM(RIGHT)", "999", simple, false), text("5")],
          vec![text(last), text("0")],
        ]);
        assert_eq!(
          cell_text(&table, 0, 0),
          if last == "0" { "999" } else { "5" }
        );
      }
    }
  }

  #[test]
  fn cycle_unwind_defers_the_back_reference_owner_and_its_remaining_dependencies() {
    for simple in [false, true] {
      let f = |code, cache| field(code, cache, simple, false);
      for (rows, expected) in [
        (
          vec![
            vec![f("=SUM(RIGHT,BELOW)", "111"), f("=SUM(LEFT)", "999")],
            vec![f("=SUM(RIGHT)", "500"), text("5")],
          ],
          vec![vec!["1499", "1499"], vec!["5", "5"]],
        ),
        (
          vec![
            vec![
              f("=SUM(RIGHT)", "111"),
              f("=SUM(LEFT,BELOW)", "999"),
              text("text"),
            ],
            vec![text("text"), f("=SUM(RIGHT)", "500"), text("5")],
          ],
          vec![vec!["999", "1004", "text"], vec!["text", "5", "5"]],
        ),
        (
          vec![
            vec![
              f("=SUM(RIGHT)", "111"),
              f("=SUM(BELOW)", "999"),
              f("=SUM(BELOW)", "500"),
            ],
            vec![f("=SUM(ABOVE)", "11"), f("=SUM(LEFT)", "22"), text("5")],
          ],
          vec![vec!["511", "11", "5"], vec!["511", "11", "5"]],
        ),
      ] {
        let table = imported(&rows);
        for (ri, row) in expected.iter().enumerate() {
          for (ci, &value) in row.iter().enumerate() {
            assert_eq!(cell_text(&table, ri, ci), value, "row{ri}, column{ci}");
          }
        }
      }
    }
  }

  #[test]
  fn independent_rows_continue_after_a_cyclic_formula_pair() {
    let f = |code, cache| field(code, cache, false, false);
    let table = imported(&[
      vec![
        f("=MAX(RIGHT)", "18"),
        f("=MAX(BELOW)", "10"),
        text("3"),
        f("=MAX(BELOW)", "18"),
      ],
      vec![
        f("=MAX(RIGHT)", "8"),
        text("6"),
        text("7"),
        f("=MAX(LEFT)", "8"),
      ],
      vec![f("=SUM(RIGHT)", "30"), text("10"), text("8"), text("12")],
      vec![text("13"), f("=MAX(RIGHT)", "15"), text("9"), text("5")],
    ]);
    for (ri, row) in [
      ["15", "15", "3", "12"],
      ["8", "6", "7", "8"],
      ["30", "10", "8", "12"],
      ["13", "9", "9", "5"],
    ]
    .iter()
    .enumerate()
    {
      for (ci, &value) in row.iter().enumerate() {
        assert_eq!(cell_text(&table, ri, ci), value);
      }
    }
  }

  #[test]
  fn forward_chain_uses_completed_dependency_values() {
    let mut row = (0..62)
      .map(|_| field("=MAX(RIGHT)", "999", false, false))
      .collect::<Vec<_>>();
    row.push(text("5"));
    let table = imported(&[row]);
    for column in 0..63 {
      assert_eq!(cell_text(&table, 0, column), "5");
    }
  }
}
