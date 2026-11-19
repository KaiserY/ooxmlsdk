//! Recursive operands of legacy display-time EQ fields.

use std::ops::Range;

use super::{TextRun, TextStyle, styled_runs};

#[derive(Clone, Debug)]
pub(crate) struct ExpressionInline {
  pub(crate) runs: Vec<TextRun>,
  pub(crate) style: TextStyle,
  pub(crate) root: Node,
}

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct BorderSides {
  pub(crate) top: bool,
  pub(crate) bottom: bool,
  pub(crate) left: bool,
  pub(crate) right: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum NaryOperator {
  Integral,
  Sum,
  Product,
}

#[derive(Clone, Debug)]
pub(crate) enum Node {
  Text(Range<usize>),
  Row(Vec<Self>),
  Array {
    cells: Vec<Self>,
    columns: usize,
    alignment: super::RubyAlignment,
    horizontal_gap_pt: f32,
    vertical_gap_pt: f32,
  },
  Fraction(Box<[Self; 2]>),
  Nary {
    operator: NaryOperator,
    arguments: Box<[Self; 3]>,
  },
  Radical {
    degree: Option<Box<Self>>,
    radicand: Box<Self>,
  },
  Brackets {
    content: Box<Self>,
    left: char,
    right: char,
  },
  Shift {
    content: Box<Self>,
    raise_pt: f32,
  },
  Advance {
    width_pt: f32,
  },
  Border {
    content: Box<Self>,
    sides: BorderSides,
  },
}

struct Parser<'a> {
  instruction: &'a str,
  styles: &'a [(usize, TextStyle)],
  fallback: &'a TextStyle,
  hyperlink: Option<&'a str>,
  offset: usize,
  runs: Vec<TextRun>,
  has_operator: bool,
}

pub(in crate::docx) fn parse(
  instruction: &str,
  styles: &[(usize, TextStyle)],
  fallback: &TextStyle,
  hyperlink: Option<&str>,
) -> Option<ExpressionInline> {
  let rest = super::super::strip_ascii_case_prefix(instruction.trim_start(), "eq")?;
  if !rest.starts_with(|c: char| c.is_whitespace() || c == '\\') {
    return None;
  }
  let rest = rest.trim_start();
  let mut parser = Parser {
    instruction,
    styles,
    fallback,
    hyperlink,
    offset: instruction.len() - rest.len(),
    runs: Vec::new(),
    has_operator: false,
  };
  let root = parser.sequence(false, 0)?;
  (parser.has_operator && parser.offset == instruction.len()).then(|| ExpressionInline {
    root,
    runs: parser.runs,
    style: fallback.clone(),
  })
}

impl Parser<'_> {
  fn remaining(&self) -> &str {
    &self.instruction[self.offset..]
  }

  fn spaces(&mut self) {
    self.offset = self.instruction.len() - self.remaining().trim_start().len();
  }

  fn text(&mut self, range: Range<usize>) -> Option<Node> {
    let start = self.runs.len();
    self.runs.extend(styled_runs(
      self.instruction,
      range,
      self.styles,
      self.fallback,
      self.hyperlink,
    )?);
    Some(Node::Text(start..self.runs.len()))
  }

  fn sequence(&mut self, argument: bool, depth: usize) -> Option<Node> {
    if depth > 64 {
      return None;
    }
    let mut nodes = Vec::new();
    let mut parentheses = 0usize;
    while let Some(ch) = self.remaining().chars().next() {
      if argument && parentheses == 0 && matches!(ch, ',' | ')') {
        break;
      }
      if ch == '\\' {
        nodes.push(self.operator(depth + 1)?);
      } else {
        let start = self.offset;
        for ch in self.instruction[self.offset..].chars() {
          if ch == '\\' || (argument && parentheses == 0 && matches!(ch, ',' | ')')) {
            break;
          }
          if ch == '(' {
            parentheses += 1;
          } else if ch == ')' {
            parentheses = parentheses.checked_sub(1)?;
          }
          self.offset += ch.len_utf8();
        }
        if self.offset == start {
          return None;
        }
        nodes.push(self.text(start..self.offset)?);
      }
    }
    (parentheses == 0).then_some(Node::Row(nodes))
  }

  fn option(&mut self) -> Option<&str> {
    self.spaces();
    let token = self.remaining().get(..3)?;
    (token.starts_with('\\') && token[1..].bytes().all(|b| b.is_ascii_alphabetic()))
      .then_some(&self.instruction[self.offset + 1..self.offset + 3])
  }

  fn integer(&mut self, default: Option<i16>) -> Option<i16> {
    self.spaces();
    let start = self.offset;
    if self.remaining().starts_with(['-', '+']) {
      self.offset += 1;
    }
    while self.remaining().starts_with(|ch: char| ch.is_ascii_digit()) {
      self.offset += 1;
    }
    if start == self.offset {
      default
    } else {
      self.instruction[start..self.offset].parse().ok()
    }
  }

  fn arguments(&mut self, depth: usize) -> Option<Vec<Node>> {
    self.spaces();
    self.remaining().strip_prefix('(')?;
    self.offset += 1;
    let mut args = Vec::new();
    loop {
      args.push(self.sequence(true, depth)?);
      let separator = self.remaining().chars().next()?;
      self.offset += separator.len_utf8();
      match separator {
        ')' => return Some(args),
        ',' => {}
        _ => return None,
      }
    }
  }

  fn operator(&mut self, depth: usize) -> Option<Node> {
    self.offset += 1;
    let ch = self.remaining().chars().next()?;
    self.offset += ch.len_utf8();
    if matches!(ch, '\\' | '(' | ')' | ',' | ';') {
      return self.text(self.offset - ch.len_utf8()..self.offset);
    }
    self.has_operator = true;
    match ch.to_ascii_lowercase() {
      'b' => {
        let (mut left, mut right) = ('(', ')');
        let (mut specified_left, mut specified_right) = (false, false);
        while let Some(option) = self.option() {
          let (set_left, set_right) = if option.eq_ignore_ascii_case("bc") {
            (true, true)
          } else if option.eq_ignore_ascii_case("lc") {
            (true, false)
          } else if option.eq_ignore_ascii_case("rc") {
            (false, true)
          } else {
            return None;
          };
          self.offset += 3;
          self.spaces();
          if self.remaining().starts_with('\\') {
            self.offset += 1;
          }
          let character = self.remaining().chars().next()?;
          self.offset += character.len_utf8();
          if set_left {
            left = character;
            specified_left = true;
          }
          if set_right {
            right = if set_left {
              match character {
                '(' => ')',
                '[' => ']',
                '{' => '}',
                '<' => '>',
                _ => character,
              }
            } else {
              character
            };
            specified_right = true;
          }
        }
        // Other delimiters and one-sided scopes retain the cached path
        // until their separate Word assembly contracts are supported.
        if specified_left != specified_right
          || !matches!(left, '(' | '[')
          || !matches!(right, ')' | ']')
        {
          return None;
        }
        let [content]: [Node; 1] = self.arguments(depth)?.try_into().ok()?;
        Some(Node::Brackets {
          content: Box::new(content),
          left,
          right,
        })
      }
      'a' => {
        let mut columns = 1usize;
        let mut alignment = super::RubyAlignment::Center;
        let mut horizontal_gap_pt = 0.0;
        let mut vertical_gap_pt = 0.0;
        while let Some(option) = self.option() {
          let option = option.to_ascii_lowercase();
          self.offset += 3;
          match option.as_str() {
            "al" => alignment = super::RubyAlignment::Left,
            "ac" => alignment = super::RubyAlignment::Center,
            "ar" => alignment = super::RubyAlignment::Right,
            "co" => {
              columns = usize::try_from(self.integer(None)?).ok()?;
              if columns == 0 {
                return None;
              }
            }
            "hs" | "vs" => {
              let gap = self.integer(None)?;
              if !(0..=127).contains(&gap) {
                return None;
              }
              if option == "hs" {
                horizontal_gap_pt = f32::from(gap);
              } else {
                vertical_gap_pt = f32::from(gap);
              }
            }
            _ => return None,
          }
        }
        let cells = self.arguments(depth)?;
        columns = columns.min(cells.len());
        Some(Node::Array {
          cells,
          columns,
          alignment,
          horizontal_gap_pt,
          vertical_gap_pt,
        })
      }
      'f' => {
        let [numerator, denominator]: [Node; 2] = self.arguments(depth)?.try_into().ok()?;
        Some(Node::Fraction(Box::new([numerator, denominator])))
      }
      'i' => {
        let mut operator = NaryOperator::Integral;
        while let Some(option) = self.option() {
          operator = if option.eq_ignore_ascii_case("su") {
            NaryOperator::Sum
          } else if option.eq_ignore_ascii_case("pr") {
            NaryOperator::Product
          } else {
            // Inline limits and authored operator glyphs have different
            // placement contracts; retain their cached result for now.
            return None;
          };
          self.offset += 3;
        }
        Some(Node::Nary {
          operator,
          arguments: Box::new(self.arguments(depth)?.try_into().ok()?),
        })
      }
      'r' => {
        let mut arguments = self.arguments(depth)?;
        if !matches!(arguments.len(), 1 | 2) {
          return None;
        }
        let radicand = Box::new(arguments.pop()?);
        let degree = arguments.pop().map(Box::new);
        Some(Node::Radical { degree, radicand })
      }
      's' => {
        // Bare single-argument\\s is Word's full-size3pt raised portion;
        // explicit\\up/\\do instead use the authored integral-point distance.
        let mut raise_pt = 3.0;
        if let Some(option) = self.option() {
          let direction = if option.eq_ignore_ascii_case("up") {
            1.0
          } else if option.eq_ignore_ascii_case("do") {
            -1.0
          } else {
            return None;
          };
          self.offset += 3;
          raise_pt = f32::from(self.integer(Some(2))?) * direction;
        }
        let [content]: [Node; 1] = self.arguments(depth)?.try_into().ok()?;
        Some(Node::Shift {
          content: Box::new(content),
          raise_pt,
        })
      }
      'd' => {
        let mut width_pt = 0.0;
        while let Some(option) = self.option() {
          let direction = if option.eq_ignore_ascii_case("ba") {
            -1.0
          } else if option.eq_ignore_ascii_case("fo") {
            1.0
          } else {
            return None;
          };
          self.offset += 3;
          width_pt = f32::from(self.integer(None)?) * direction;
        }
        let [Node::Row(empty)]: [Node; 1] = self.arguments(depth)?.try_into().ok()? else {
          return None;
        };
        empty.is_empty().then_some(Node::Advance { width_pt })
      }
      'x' => {
        let mut sides = BorderSides::default();
        let mut explicit = false;
        while let Some(option) = self.option() {
          if option.eq_ignore_ascii_case("to") {
            sides.top = true;
          } else if option.eq_ignore_ascii_case("bo") {
            sides.bottom = true;
          } else if option.eq_ignore_ascii_case("le") {
            sides.left = true;
          } else if option.eq_ignore_ascii_case("ri") {
            sides.right = true;
          } else {
            return None;
          }
          explicit = true;
          self.offset += 3;
        }
        if !explicit {
          sides = BorderSides {
            top: true,
            bottom: true,
            left: true,
            right: true,
          };
        }
        let [content]: [Node; 1] = self.arguments(depth)?.try_into().ok()?;
        Some(Node::Border {
          content: Box::new(content),
          sides,
        })
      }
      _ => None,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn nested_operands_preserve_utf8_source_styles_and_escaped_punctuation() {
    let base = TextStyle::default();
    let raised = TextStyle {
      baseline_shift_pt: 3.0,
      ..base.clone()
    };
    let code = r"EQ z=\f(\f(α2,β),\x(p=(x,y,z))) ";
    let offset = code.find('2').unwrap();
    let result = parse(
      code,
      &[(offset, raised), (offset + 1, base.clone())],
      &base,
      Some("#eq"),
    )
    .unwrap();
    assert_eq!(
      result
        .runs
        .iter()
        .map(|run| run.text.as_str())
        .collect::<String>(),
      "z=α2βp=(x,y,z) "
    );
    assert!(
      result
        .runs
        .iter()
        .all(|run| run.hyperlink_url.as_deref() == Some("#eq"))
    );
    assert_eq!(
      result
        .runs
        .iter()
        .find(|run| run.text == "2")
        .unwrap()
        .style
        .baseline_shift_pt,
      3.0
    );
    assert!(parse(r"EQ \f(a\,b,c\(d\))", &[], &base, None).is_some());
    let Node::Row(nodes) = result.root else {
      panic!("row")
    };
    assert!(matches!(nodes[1], Node::Fraction(_)));
  }

  #[test]
  fn malformed_unknown_or_unbalanced_operators_retain_the_cached_path() {
    let style = TextStyle::default();
    for code in [
      r"EQ",
      r"EQ plain",
      r"EQ \f(a)",
      r"EQ \f(a,b,c)",
      r"EQ \f(a,b",
      r"EQ \f(a,b))",
      r"EQ \r(a,b,c)",
      r"EQ \d\ba8(a)",
      r"EQ \s(a,b)",
      r"EQ \x(a,b)",
      r"EQ \unknown(a)",
      r"OTHER \f(a,b)",
    ] {
      assert!(parse(code, &[], &style, None).is_none(), "{code}");
    }
    let too_deep = format!("EQ {}x{}", r"\f(,".repeat(70), ")".repeat(70));
    assert!(parse(&too_deep, &[], &style, None).is_none());
  }

  #[test]
  fn cursor_displacements_and_script_distances_remain_independent() {
    let style = TextStyle::default();
    let result = parse(r"EQ V\d\ba8()\s\up9(re)\d\fo1()n\s(2)", &[], &style, None).unwrap();
    let Node::Row(nodes) = result.root else {
      panic!("row")
    };
    assert!(matches!(nodes[1], Node::Advance { width_pt: -8.0 }));
    assert!(matches!(nodes[2], Node::Shift { raise_pt: 9.0, .. }));
    assert!(matches!(nodes[3], Node::Advance { width_pt: 1.0, .. }));
    assert!(matches!(nodes[5], Node::Shift { raise_pt: 3.0, .. }));
  }

  #[test]
  fn recursive_arrays_keep_columns_gaps_and_delimiter_arity() {
    let style = TextStyle::default();
    let parsed = parse(
      r"EQ \b\lc\[\rc\](\a\co3\al\hs3\vs2(1,\f(2,3),4,5,6,7))",
      &[],
      &style,
      None,
    )
    .unwrap();
    let Node::Row(row) = parsed.root else {
      panic!("row")
    };
    let Node::Brackets {
      content,
      left: '[',
      right: ']',
    } = &row[0]
    else {
      panic!("brackets")
    };
    let Node::Row(content) = content.as_ref() else {
      panic!("content")
    };
    let Node::Array {
      columns,
      cells,
      alignment: super::super::RubyAlignment::Left,
      horizontal_gap_pt: 3.0,
      vertical_gap_pt: 2.0,
    } = &content[0]
    else {
      panic!("array")
    };
    assert_eq!(*columns, 3);
    assert_eq!(cells.len(), 6);
    for code in [
      r"EQ \a\co0(1)",
      r"EQ \a\hs128(1,2)",
      r"EQ \b(a,b)",
      r"EQ \b\lc\[(a)",
    ] {
      assert!(parse(code, &[], &style, None).is_none(), "{code}");
    }
  }

  #[test]
  fn nary_operators_keep_three_styled_recursive_operands() {
    let style = TextStyle::default();
    for (switch, expected) in [
      ("", NaryOperator::Integral),
      (r"\su", NaryOperator::Sum),
      (r"\pr", NaryOperator::Product),
    ] {
      let parsed = parse(
        &format!(r"EQ \i{switch}(i=0,n\s(2),\r(i))"),
        &[],
        &style,
        None,
      )
      .unwrap();
      let Node::Row(row) = parsed.root else {
        panic!("row")
      };
      let Node::Nary {
        operator,
        arguments,
      } = &row[0]
      else {
        panic!("nary")
      };
      assert_eq!(*operator, expected);
      assert_eq!(arguments.len(), 3);
      assert_eq!(
        parsed
          .runs
          .iter()
          .map(|r| r.text.as_str())
          .collect::<String>(),
        "i=0n2i"
      );
    }
    for code in [
      r"EQ \i(a,b)",
      r"EQ \i(a,b,c,d)",
      r"EQ \i\in(a,b,c)",
      r"EQ \i\fc\{(a,b,c)",
    ] {
      assert!(parse(code, &[], &style, None).is_none(), "{code}");
    }
  }

  #[test]
  fn display_field_forms_rebuild_geometry_and_non_eq_parents_suppress_the_value() {
    use crate::docx::{Block, InlineItem};
    use crate::options::LayoutOptions;
    use ooxmlsdk::parts::wordprocessing_document::WordprocessingDocument;
    use ooxmlsdk::sdk::WordprocessingDocumentType;
    let complex = r#"<w:r><w:fldChar w:fldCharType="begin" w:fldLock="1"/></w:r><w:r><w:instrText>EQ \f(1,2)</w:instrText></w:r><w:r><w:fldChar w:fldCharType="separate"/></w:r><w:r><w:t>STALE</w:t></w:r><w:r><w:fldChar w:fldCharType="end"/></w:r>"#;
    let simple = r#"<w:fldSimple w:instr="EQ \f(1,2)"><w:r><w:t>STALE</w:t></w:r></w:fldSimple>"#;
    for field in [complex, simple] {
      for nested in [false, true] {
        let body = if nested {
          format!(
            r#"<w:r><w:fldChar w:fldCharType="begin" w:fldLock="1"/></w:r><w:r><w:instrText>QUOTE "cache"</w:instrText></w:r><w:r><w:fldChar w:fldCharType="separate"/></w:r><w:r><w:t>A</w:t></w:r>{field}<w:r><w:t>B</w:t></w:r><w:r><w:fldChar w:fldCharType="end"/></w:r>"#
          )
        } else {
          field.to_string()
        };
        let mut package = WordprocessingDocument::create(WordprocessingDocumentType::Document);
        let main = package.add_main_document_part().unwrap();
        main.set_data(&mut package,format!(r#"<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:body><w:p>{body}</w:p></w:body></w:document>"#).into_bytes()).unwrap();
        let document = crate::docx::extract(&package, &LayoutOptions::default()).unwrap();
        let Block::Paragraph(paragraph) = &document.sections[0].blocks[0] else {
          panic!("paragraph")
        };
        assert_eq!(
          paragraph
            .inlines
            .iter()
            .filter(|i| matches!(i, InlineItem::LegacyEquation(_)))
            .count(),
          usize::from(!nested)
        );
        let text = paragraph
          .inlines
          .iter()
          .filter_map(|i| match i {
            InlineItem::Text(run) => Some(run.text.as_str()),
            _ => None,
          })
          .collect::<String>();
        assert_eq!(text, if nested { "AB" } else { "" });
      }
    }
  }
}
