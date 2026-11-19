//! Legacy EQ keeps source line cells separate from hinted printer geometry.
//!
//! ECMA-376 Part4 §14.10.4.6 defines the recursive operands. Native Word
//! fraction controls and independently observed GDI TEXTMETRICW establish the
//! raw-ascent axis, full-size operands and invisible lower reserve. OfficeMath
//! MATH constants and automatic script scaling describe different objects.

use super::*;
use crate::docx::eq::{ExpressionInline, NaryOperator, Node};
use smallvec::SmallVec;

const DEVICE_DOT_PT: f32 = 72.0 / 600.0;

#[derive(Clone, Copy, Debug, Default)]
struct Extents {
  ascent: f32,
  descent: f32,
}

impl Extents {
  fn height(self) -> f32 {
    self.ascent + self.descent
  }

  fn include(&mut self, other: Self) {
    self.ascent = self.ascent.max(other.ascent);
    self.descent = self.descent.max(other.descent);
  }

  fn raised(self, raise_pt: f32) -> Self {
    Self {
      ascent: (self.ascent + raise_pt).max(0.0),
      descent: (self.descent - raise_pt).max(0.0),
    }
  }
}

#[derive(Clone, Copy, Debug)]
struct FontCell {
  logical: Extents,
  printer: Extents,
  logical_axis: f32,
  printer_axis: f32,
  printer_raw_ascent: f32,
}

fn font_cell(text: &str, style: &TextStyle, metrics: &mut TextMetrics) -> FontCell {
  let font = metrics.line_vertical_metrics_for_text(text, style);
  let ascent = font.directwrite_baseline_offset_pt;
  let logical = Extents {
    ascent,
    descent: (font.line_height_pt() - ascent).max(0.0),
  };
  let logical_axis = font.baseline_offset_pt / 2.0;
  let mut physical = style.clone();
  crate::docx::quantize_word_fixed_output_text_style(&mut physical);
  let (printer, printer_axis, printer_raw_ascent) = metrics
    .gdi_printer_cell_metrics(text, &physical, 600.0)
    .map_or_else(
      || {
        (
          Extents {
            ascent: device(logical.ascent),
            descent: device(logical.descent),
          },
          (logical_axis / DEVICE_DOT_PT).floor() * DEVICE_DOT_PT,
          device(2.0 * logical_axis),
        )
      },
      |cell| {
        (
          Extents {
            ascent: cell.ascent_with_leading_pt,
            descent: cell.descent_pt,
          },
          // Halve the integral device ascent before converting back to
          // points. Odd ascents retain the lower integer half in Word.
          ((cell.ascent_pt / DEVICE_DOT_PT).round() / 2.0).floor() * DEVICE_DOT_PT,
          cell.ascent_pt,
        )
      },
    );
  FontCell {
    // Automatic escapement already restores the original source-font cell
    // in line_vertical_metrics_for_text. Its paint shift moves only the
    // reduced glyph, as in WordLineTextExtents::include; applying it to the
    // restored cell again enlarges a surrounding radical.
    logical: logical.raised(if style.automatic_escapement_font_size_pt.is_some() {
      0.0
    } else {
      style.baseline_shift_pt
    }),
    printer: printer.raised(style.baseline_shift_pt),
    logical_axis,
    printer_axis,
    printer_raw_ascent,
  }
}

fn device(pt: f32) -> f32 {
  units::quantize_points_to_office_print_grid(pt)
}

fn nary_axis(operator: NaryOperator, ascent: f32, body: Extents, unit: f32) -> (f32, f32) {
  // Word centres and adjusts the operator in integral units. In particular,
  // floor(A/2)-floor(H/2) is not floor((A-H)/2). Native Word's metric
  // adjustment and96 independent GDI/PDF controls distinguish these cases.
  let a = (ascent / unit).round() as i64;
  let h = (body.height() / unit).round() as i64;
  let d = (body.descent / unit).round() as i64;
  let mut down = (a + 1) / 2 - h / 2 + d;
  let adjusted = if operator == NaryOperator::Integral {
    down -= 4;
    a
  } else {
    let adjusted = (8 * a + 5) / 10;
    down -= (adjusted + 3) / 7;
    down = if a < 18 {
      1
    } else {
      down - if a == 18 { 1 } else { 3 }
    };
    adjusted
  };
  (adjusted as f32 * unit, down as f32 * unit)
}

struct Layout<'a> {
  width: f32,
  logical: Extents,
  printer: Extents,
  paint: Paint<'a>,
}

#[derive(Clone, Copy, Debug, Default)]
struct ArrayRow {
  logical: Extents,
  printer: Extents,
}

struct GlyphPlacement {
  character: char,
  x: f32,
  raise: f32,
}

enum Paint<'a> {
  Text(&'a [TextRun]),
  Row(Vec<Layout<'a>>),
  Array {
    cells: Vec<Layout<'a>>,
    columns: SmallVec<[f32; 4]>,
    rows: SmallVec<[ArrayRow; 4]>,
    alignment: RubyAlignment,
    horizontal_gap_pt: f32,
    vertical_gap_pt: f32,
  },
  Fraction {
    arguments: Box<[Layout<'a>; 2]>,
    axis: f32,
    up: f32,
    down: f32,
  },
  Radical {
    radicand: Box<Layout<'a>>,
    degree: Option<Box<Layout<'a>>>,
    head_width: f32,
    height: f32,
    leading_width: f32,
    degree_raise: f32,
    bottom: f32,
  },
  Brackets {
    content: Box<Layout<'a>>,
    style: Box<TextStyle>,
    glyphs: SmallVec<[GlyphPlacement; 8]>,
    left_width: f32,
  },
  Nary {
    arguments: Box<[Layout<'a>; 3]>,
    style: Box<TextStyle>,
    glyphs: SmallVec<[GlyphPlacement; 8]>,
    limit_width: f32,
    body_x: f32,
    lower_down: f32,
    upper_raise: f32,
  },
  Shift {
    content: Box<Layout<'a>>,
    raise_pt: f32,
  },
  Advance,
  Border {
    content: Box<Layout<'a>>,
    sides: crate::docx::eq::BorderSides,
  },
}

fn layout<'a>(
  node: &'a Node,
  equation: &'a ExpressionInline,
  reference: FontCell,
  metrics: &mut TextMetrics,
) -> Layout<'a> {
  match node {
    Node::Text(range) => {
      let runs = &equation.runs[range.clone()];
      let mut logical = Extents::default();
      let mut printer = Extents::default();
      for run in runs {
        let cell = font_cell(&run.text, &run.style, metrics);
        logical.include(cell.logical);
        printer.include(cell.printer);
      }
      Layout {
        width: text_runs_width(runs, metrics),
        logical,
        printer,
        paint: Paint::Text(runs),
      }
    }
    Node::Row(nodes) => {
      let mut logical = Extents::default();
      let mut printer = Extents::default();
      let mut width = 0.0;
      let mut children = Vec::with_capacity(nodes.len());
      for node in nodes {
        let child = layout(node, equation, reference, metrics);
        logical.include(child.logical);
        printer.include(child.printer);
        width += child.width;
        children.push(child);
      }
      Layout {
        width,
        logical,
        printer,
        paint: Paint::Row(children),
      }
    }
    Node::Array {
      cells,
      columns: count,
      alignment,
      horizontal_gap_pt,
      vertical_gap_pt,
    } => {
      let mut columns: SmallVec<[f32; 4]> = SmallVec::from_elem(0.0, *count);
      let mut rows: SmallVec<[ArrayRow; 4]> =
        SmallVec::from_elem(ArrayRow::default(), cells.len().div_ceil(*count));
      let cells = cells
        .iter()
        .enumerate()
        .map(|(i, node)| {
          let cell = layout(node, equation, reference, metrics);
          columns[i % *count] = columns[i % *count].max(cell.width);
          let row = &mut rows[i / *count];
          row.logical.include(cell.logical);
          row.printer.include(cell.printer);
          cell
        })
        .collect();
      for row in &mut rows {
        if row.logical.height() == 0.0 {
          row.logical = reference.logical;
          row.printer = reference.printer;
        }
      }
      let gaps = rows.len().saturating_sub(1) as f32;
      let height =
        rows.iter().map(|row| row.logical.height()).sum::<f32>() + gaps * vertical_gap_pt;
      let printer_height =
        rows.iter().map(|row| row.printer.height()).sum::<f32>() + gaps * device(*vertical_gap_pt);
      // Recursive EQ arrays centre around the raw carrier ascent's axis;
      // its separately rounded external leading belongs to each row only.
      // Native bracketed8/12/18pt arrays distinguish these two owners.
      let logical_ascent = height / 2.0 + reference.logical_axis;
      // Native odd-height arrays halve integral printer dots before adding
      // the carrier axis. A fractional half would move both row baselines
      // and assembled delimiters; even-height arrays retain the same box.
      let printer_ascent = ((printer_height / DEVICE_DOT_PT).round() / 2.0).floor() * DEVICE_DOT_PT
        + reference.printer_axis;
      Layout {
        width: columns.iter().sum::<f32>() + columns.len() as f32 * horizontal_gap_pt,
        logical: Extents {
          ascent: logical_ascent,
          descent: (height - logical_ascent).max(0.0),
        },
        printer: Extents {
          ascent: printer_ascent,
          descent: (printer_height - printer_ascent).max(0.0),
        },
        paint: Paint::Array {
          cells,
          columns,
          rows,
          alignment: *alignment,
          horizontal_gap_pt: device(*horizontal_gap_pt),
          vertical_gap_pt: device(*vertical_gap_pt),
        },
      }
    }
    Node::Fraction(arguments) => {
      let mut numerator = layout(&arguments[0], equation, reference, metrics);
      let mut denominator = layout(&arguments[1], equation, reference, metrics);
      for argument in [&mut numerator, &mut denominator] {
        if argument.logical.height() == 0.0 {
          argument.logical = reference.logical;
          argument.printer = reference.printer;
        }
      }
      let logical = Extents {
        ascent: reference.logical_axis + numerator.logical.height(),
        descent: (denominator.logical.height() - reference.logical_axis + 1.0).max(0.0),
      };
      let printer = Extents {
        ascent: reference.printer_axis + numerator.printer.height(),
        descent: (denominator.printer.height() - reference.printer_axis + device(1.0)).max(0.0),
      };
      let up = reference.printer_axis + numerator.printer.descent;
      let down = denominator.printer.ascent - reference.printer_axis + DEVICE_DOT_PT;
      Layout {
        width: numerator.width.max(denominator.width),
        logical,
        printer,
        paint: Paint::Fraction {
          arguments: Box::new([numerator, denominator]),
          axis: reference.printer_axis,
          up,
          down,
        },
      }
    }
    Node::Nary {
      operator,
      arguments,
    } => {
      let lower = layout(&arguments[0], equation, reference, metrics);
      let upper = layout(&arguments[1], equation, reference, metrics);
      let mut body = layout(&arguments[2], equation, reference, metrics);
      if body.logical.height() == 0.0 {
        body.logical = reference.logical;
        body.printer = reference.printer;
      }
      let nominal_height = (body.logical.height() * 2.0).round() / 2.0;
      let integral = *operator == NaryOperator::Integral;
      let size = if !integral {
        nominal_height
      } else if nominal_height <= 48.0 {
        12.0
      } else if nominal_height <= 72.0 {
        18.0
      } else {
        24.0
      };
      let character = match operator {
        NaryOperator::Sum => '\u{f0e5}',
        NaryOperator::Product => '\u{f0d5}',
        NaryOperator::Integral => '\u{f0f3}',
      };
      let style = Box::new(operator_style(&equation.style, Some("Symbol"), size));
      let text = character.to_string();
      let cell = font_cell(&text, &style, metrics);
      let operator_width = device(metrics.measure_text(&text, &style));
      let limit_width = operator_width.max(lower.width).max(upper.width);
      let glyph_x = (limit_width - operator_width) / 2.0;
      let body_x = glyph_x + operator_width;
      let raw_ascent = if integral {
        (device(nominal_height) - cell.printer.descent).max(cell.printer_raw_ascent + device(3.0))
      } else {
        cell.printer_raw_ascent
      };
      let (adjusted_ascent, down) = nary_axis(*operator, raw_ascent, body.printer, DEVICE_DOT_PT);
      let operator_descent = if integral {
        cell.printer.descent
      } else {
        cell.printer.descent.max(
          ((adjusted_ascent / DEVICE_DOT_PT).round() + 3.0)
            .trunc()
            .div_euclid(7.0)
            * DEVICE_DOT_PT,
        )
      };
      let lower_down = down + operator_descent + lower.printer.ascent;
      let upper_raise = adjusted_ascent - down + upper.printer.descent;
      let printer = Extents {
        ascent: (adjusted_ascent - down).max(body.printer.ascent)
          + upper.printer.height().max(3.0 * DEVICE_DOT_PT),
        descent: (operator_descent + down).max(body.printer.descent) + lower.printer.height(),
      };
      // Source layout and printer conversion are separate native passes.
      // The source pass retains the operator's adjusted ascent/descent at
      //4096units/pt, including sum/product descent below a shorter body.
      // The printer pass can overwrite its source-descent slot with the
      // body's; that later value must not size the paragraph's line advance.
      // Integrals retain the12/18/24pt piece and3pt minimum extension.
      let source_ascent = if integral {
        (nominal_height - cell.logical.descent).max(2.0 * cell.logical_axis + 3.0)
      } else {
        2.0 * cell.logical_axis
      };
      let (source_ascent, source_down) =
        nary_axis(*operator, source_ascent, body.logical, 1.0 / 4096.0);
      let logical = Extents {
        ascent: (source_ascent - source_down).max(body.logical.ascent)
          + upper.logical.height().max(1.0),
        descent: (cell.logical.descent + source_down).max(body.logical.descent)
          + lower.logical.height(),
      };
      let mut glyphs = SmallVec::new();
      if integral {
        let distance =
          (device(nominal_height) - cell.printer_raw_ascent - cell.printer.descent - device(1.0))
            .max(device(3.0));
        let pitch = (cell.printer_raw_ascent - device(1.0)).max(DEVICE_DOT_PT);
        let count = ((distance - cell.printer_raw_ascent).max(0.0) / pitch).ceil() as usize;
        glyphs.push(GlyphPlacement {
          character: '\u{f0f5}',
          x: glyph_x,
          raise: -down,
        });
        if count != 0 {
          let edge = (distance - count.saturating_sub(1) as f32 * pitch) / 2.0;
          for index in 0..count {
            glyphs.push(GlyphPlacement {
              character: '\u{f0f4}',
              x: glyph_x,
              raise: -down + edge + index as f32 * pitch,
            });
          }
        }
        glyphs.push(GlyphPlacement {
          character,
          x: glyph_x,
          raise: -down + distance,
        });
      } else {
        glyphs.push(GlyphPlacement {
          character,
          x: glyph_x,
          raise: -down,
        });
      }
      Layout {
        width: body_x + body.width,
        logical,
        printer,
        paint: Paint::Nary {
          arguments: Box::new([lower, upper, body]),
          style,
          glyphs,
          limit_width,
          body_x,
          lower_down,
          upper_raise,
        },
      }
    }
    Node::Radical { degree, radicand } => {
      let mut radicand = Box::new(layout(radicand, equation, reference, metrics));
      if radicand.logical.height() == 0.0 {
        radicand.logical = reference.logical;
        radicand.printer = reference.printer;
      }
      let degree = degree
        .as_ref()
        .map(|node| Box::new(layout(node, equation, reference, metrics)));
      // Native Word uses the argument's source cell height rounded to a
      // half point, then realizes the separate Symbol advance and height
      // on the printer grid. Quantizing the height before its advance
      // changes tall roots by one dot.
      let nominal_height = (radicand.logical.height() * 2.0).round() / 2.0;
      let symbol = TextStyle {
        font_family: Some("Symbol".into()),
        high_ansi_font_family: Some("Symbol".into()),
        symbol_font_family: Some("Symbol".into()),
        font_size_pt: nominal_height,
        ..equation.style.clone()
      };
      let height = device(nominal_height);
      let head_width = device(metrics.measure_text("\u{f0d6}", &symbol));
      let leading_width = degree
        .as_ref()
        .map_or(0.0, |degree| (degree.width - head_width / 2.0).max(0.0));
      let degree_raise = (3.0 * height / DEVICE_DOT_PT / 4.0).floor() * DEVICE_DOT_PT
        + 3.0 * DEVICE_DOT_PT
        - radicand.printer.descent;
      let degree_logical_reserve = degree
        .as_ref()
        .map_or(0.0, |degree| degree.logical.height() * 0.75);
      let degree_printer_reserve = degree
        .as_ref()
        .map_or(0.0, |degree| degree.printer.height() * 0.75);
      // The two-point reserve is above the argument. Word separately
      // reserves three quarters of the full-size degree's cell above it;
      // taking only the painted degree bounds loses natural line height.
      let logical = Extents {
        ascent: nominal_height + 2.0 - radicand.logical.descent + degree_logical_reserve,
        descent: radicand.logical.descent,
      };
      let printer = Extents {
        ascent: height - radicand.printer.descent
          + 2.0 * DEVICE_DOT_PT
          + device(2.0)
          + degree_printer_reserve,
        descent: radicand.printer.descent,
      };
      let bottom = radicand.printer.descent - 2.0 * DEVICE_DOT_PT;
      Layout {
        width: leading_width + head_width + device(1.0) + radicand.width,
        logical,
        printer,
        paint: Paint::Radical {
          radicand,
          degree,
          head_width,
          height,
          leading_width,
          degree_raise,
          bottom,
        },
      }
    }
    Node::Brackets {
      content,
      left,
      right,
    } => {
      let mut content = Box::new(layout(content, equation, reference, metrics));
      if content.logical.height() == 0.0 {
        content.logical = reference.logical;
        content.printer = reference.printer;
      }
      let nominal_height = (content.logical.height() * 2.0).round() / 2.0;
      // Native Word retains the carrier font through22pt. Larger
      // cells select12/18/24pt Symbol pieces at48/72pt target boundaries;
      // the final tier adds extenders, rather than growing the font again.
      let assembled = nominal_height > 22.0;
      let size = if !assembled {
        nominal_height
      } else if nominal_height <= 48.0 {
        12.0
      } else if nominal_height <= 72.0 {
        18.0
      } else {
        24.0
      };
      let family = assembled.then_some("Symbol");
      let style = Box::new(operator_style(&equation.style, family, size));
      let left_character = if assembled {
        if *left == '(' { '\u{f0e6}' } else { '\u{f0e9}' }
      } else {
        *left
      };
      let right_character = if assembled {
        if *right == ')' {
          '\u{f0f6}'
        } else {
          '\u{f0f9}'
        }
      } else {
        *right
      };
      let left_width = metrics.measure_text(&left_character.to_string(), style.as_ref());
      let right_width = metrics.measure_text(&right_character.to_string(), style.as_ref());
      let cell = font_cell(&left_character.to_string(), &style, metrics);
      let printer_height = device(nominal_height);
      let operator_ascent = if assembled {
        printer_height - cell.printer.descent
      } else {
        cell.printer_raw_ascent
      };
      let dots = |pt: f32| (pt / DEVICE_DOT_PT).round();
      // Word centres the complete raw font cell against the argument cell.
      // Its integer half truncates toward zero, independently of the line's
      // final baseline. External leading remains owned by argument rows.
      let down = dots(operator_ascent)
        - ((dots(cell.printer.descent) - dots(content.printer.height()) + dots(operator_ascent))
          / 2.0)
          .trunc()
        - dots(content.printer.ascent);
      let raise = -down * DEVICE_DOT_PT;
      let mut glyphs = SmallVec::new();
      for (character, x) in [
        (left_character, 0.0),
        (right_character, left_width + content.width),
      ] {
        if assembled {
          let (bottom, middle) = match character {
            '\u{f0e6}' => ('\u{f0e8}', '\u{f0e7}'),
            '\u{f0f6}' => ('\u{f0f8}', '\u{f0f7}'),
            '\u{f0e9}' => ('\u{f0eb}', '\u{f0ea}'),
            '\u{f0f9}' => ('\u{f0fb}', '\u{f0fa}'),
            _ => unreachable!(),
          };
          let distance =
            (printer_height - cell.printer_raw_ascent - cell.printer.descent - device(1.0))
              .max(0.0);
          let ink_height = metrics
            .font_character_ink_extents(middle, style.as_ref())
            .map_or(size, |(a, d)| a + d);
          let pitch = device((ink_height - 1.0).max(DEVICE_DOT_PT));
          let count = ((distance - device(ink_height)).max(0.0) / pitch).ceil() as usize;
          glyphs.push(GlyphPlacement {
            character: bottom,
            x,
            raise,
          });
          if count != 0 {
            let edge = (distance - count.saturating_sub(1) as f32 * pitch) / 2.0;
            for index in 0..count {
              glyphs.push(GlyphPlacement {
                character: middle,
                x,
                raise: raise + edge + index as f32 * pitch,
              });
            }
          }
          glyphs.push(GlyphPlacement {
            character,
            x,
            raise: raise + distance,
          });
        } else {
          glyphs.push(GlyphPlacement {
            character,
            x,
            raise,
          });
        }
      }
      Layout {
        width: left_width + content.width + right_width,
        logical: content.logical,
        printer: content.printer,
        paint: Paint::Brackets {
          content,
          style,
          glyphs,
          left_width,
        },
      }
    }
    Node::Shift { content, raise_pt } => {
      let content = Box::new(layout(content, equation, reference, metrics));
      Layout {
        width: content.width,
        logical: content.logical.raised(*raise_pt),
        printer: content.printer.raised(device(*raise_pt)),
        paint: Paint::Shift {
          content,
          raise_pt: device(*raise_pt),
        },
      }
    }
    Node::Advance { width_pt } => Layout {
      width: *width_pt,
      logical: Extents::default(),
      printer: Extents::default(),
      paint: Paint::Advance,
    },
    Node::Border { content, sides } => {
      let content = Box::new(layout(content, equation, reference, metrics));
      // Native borders reserve4pt at each enabled side; the rule sits1pt
      // inside that outer reserve, leaving3pt between its centre and text.
      let left = if sides.left { 4.0 } else { 0.0 };
      let right = if sides.right { 4.0 } else { 0.0 };
      Layout {
        width: content.width + left + right,
        logical: Extents {
          ascent: content.logical.ascent + if sides.top { 4.0 } else { 0.0 },
          descent: content.logical.descent + if sides.bottom { 4.0 } else { 0.0 },
        },
        printer: Extents {
          ascent: content.printer.ascent + if sides.top { device(4.0) } else { 0.0 },
          descent: content.printer.descent + if sides.bottom { device(4.0) } else { 0.0 },
        },
        paint: Paint::Border {
          content,
          sides: *sides,
        },
      }
    }
  }
}

fn equation_layout<'a>(
  equation: &'a ExpressionInline,
  metrics: &mut TextMetrics,
) -> (Layout<'a>, FontCell) {
  let reference = font_cell("A", &equation.style, metrics);
  (
    layout(&equation.root, equation, reference, metrics),
    reference,
  )
}

fn operator_style(reference: &TextStyle, family: Option<&str>, size: f32) -> TextStyle {
  let mut style = TextStyle {
    font_size_pt: size,
    complex_font_size_pt: Some(size),
    automatic_escapement_font_size_pt: None,
    automatic_escapement_complex_font_size_pt: None,
    baseline_shift_pt: 0.0,
    ..reference.clone()
  };
  if let Some(family) = family {
    style.font_family = Some(family.into());
    style.high_ansi_font_family = Some(family.into());
    style.east_asia_font_family = Some(family.into());
    style.complex_font_family = Some(family.into());
    style.symbol_font_family = (family == "Symbol").then(|| family.into());
  }
  style
}

pub(super) fn metrics(
  equation: &ExpressionInline,
  text_metrics: &mut TextMetrics,
) -> CompoundTextMetrics {
  let (layout, reference) = equation_layout(equation, text_metrics);
  let extents = WordLineTextExtents {
    ascent_pt: layout.logical.ascent,
    descent_pt: layout.logical.descent,
    font_ascent_pt: layout.logical.ascent,
    font_descent_pt: layout.logical.descent,
    centered_font_ascent_pt: layout.logical.ascent,
    line_spacing_height_pt: reference.logical.height(),
    has_spacing_excluded_portion: true,
    has_legacy_equation: true,
    has_text: true,
    ..Default::default()
  };
  CompoundTextMetrics {
    width_pt: layout.width,
    height_pt: extents.height_pt(),
    line_extents: Some(extents),
  }
}

struct Painter<'a> {
  page: &'a mut Page,
  paragraph: &'a crate::docx::Paragraph,
  equation: &'a ExpressionInline,
  placement: TextPlacement,
  segmentation: TextSegmentation,
  provisional_baseline: f32,
  text_metrics: &'a mut TextMetrics,
}

impl Painter<'_> {
  fn glyph(&mut self, character: char, style: &TextStyle, x: f32, raise: f32) {
    let fallback;
    let run = if let Some(run) = self.equation.runs.first() {
      run
    } else {
      fallback = TextRun {
        text: String::new(),
        style: TextStyle::default(),
        hyperlink_url: None,
        dynamic_field: None,
        style_ref_keys: Vec::new(),
        style_ref_text: None,
        style_ref_numbering_text: None,
        preserve_text_portion: false,
      };
      &fallback
    };
    let mut style = style.clone();
    style.baseline_shift_pt += raise;
    push_ruby_text(
      self.page,
      self.paragraph,
      run,
      character.to_string(),
      style,
      TextPlacement {
        x_pt: x,
        ..self.placement
      },
      self.segmentation,
    );
  }

  fn rule(&mut self, x1: f32, y1: f32, x2: f32, y2: f32) {
    if self.equation.style.hidden {
      return;
    }
    let baseline = self.placement.y_pt + self.provisional_baseline;
    self.page.items.push(PageItem::Line(LineItem {
      x1_pt: x1,
      y1_pt: baseline - y1,
      x2_pt: x2,
      y2_pt: baseline - y2,
      width_pt: device(0.75),
      color: self.equation.style.color,
      dash_pattern: BorderDashPattern::Solid,
      kind: LineItemKind::LegacyEquationRule {
        y1_from_baseline_pt: -y1,
        y2_from_baseline_pt: -y2,
      },
    }));
  }

  fn paint(&mut self, layout: &Layout<'_>, x: f32, raise: f32) {
    match &layout.paint {
      Paint::Text(runs) => push_ruby_horizontal_line(
        self.page,
        self.paragraph,
        runs,
        RubyLineLayout {
          alignment: RubyAlignment::Left,
          target_width: layout.width,
          baseline_shift_pt: raise,
          segmentation: self.segmentation,
          placement: TextPlacement {
            x_pt: x,
            ..self.placement
          },
        },
        self.text_metrics,
      ),
      Paint::Row(children) => {
        let mut cursor = x;
        for child in children {
          self.paint(child, cursor, raise);
          cursor += child.width;
        }
      }
      Paint::Array {
        cells,
        columns,
        rows,
        alignment,
        horizontal_gap_pt,
        vertical_gap_pt,
      } => {
        let mut top = 0.0;
        for (row_index, row) in rows.iter().enumerate() {
          let row_raise = raise + layout.printer.ascent - top - row.printer.ascent;
          // Word allocates one full horizontal gap to every column,
          // placing half of the outer gap at each edge. Halve integral
          // printer dots before converting to points (hs3 starts12dots in).
          let mut cursor = x + (horizontal_gap_pt / DEVICE_DOT_PT / 2.0).floor() * DEVICE_DOT_PT;
          for (column_index, width) in columns.iter().enumerate() {
            if let Some(cell) = cells.get(row_index * columns.len() + column_index) {
              let cell_x = ruby_line_start_x(*alignment, cursor, cell.width, *width);
              self.paint(cell, cell_x, row_raise);
            }
            cursor += width + horizontal_gap_pt;
          }
          top += row.printer.height() + vertical_gap_pt;
        }
      }
      Paint::Fraction {
        arguments,
        axis,
        up,
        down,
      } => {
        let [numerator, denominator] = arguments.as_ref();
        self.paint(
          numerator,
          x + (layout.width - numerator.width) / 2.0,
          raise + up,
        );
        self.paint(
          denominator,
          x + (layout.width - denominator.width) / 2.0,
          raise - down,
        );
        self.rule(x, raise + axis, x + layout.width, raise + axis);
      }
      Paint::Radical {
        radicand,
        degree,
        head_width,
        height,
        leading_width,
        degree_raise,
        bottom,
      } => {
        if let Some(degree) = degree {
          self.paint(degree, x + device(1.0), raise + degree_raise);
        }
        let left = x + leading_width;
        self.paint(radicand, left + head_width + device(1.0), raise);
        let width_dots = (head_width / DEVICE_DOT_PT).round();
        let height_dots = (height / DEVICE_DOT_PT).round();
        let vertices = [
          (0.0, (height_dots / 2.0).floor()),
          (
            (width_dots / 5.0).floor(),
            (5.0 * height_dots / 9.0).round(),
          ),
          ((7.0 * width_dots / 10.0).floor(), 0.0),
          (width_dots, height_dots),
        ];
        for pair in vertices.windows(2) {
          self.rule(
            left + pair[0].0 * DEVICE_DOT_PT,
            raise - bottom + pair[0].1 * DEVICE_DOT_PT,
            left + pair[1].0 * DEVICE_DOT_PT,
            raise - bottom + pair[1].1 * DEVICE_DOT_PT,
          );
        }
        self.rule(
          left + head_width,
          raise - bottom + height,
          left + head_width + radicand.width,
          raise - bottom + height,
        );
      }
      Paint::Brackets {
        content,
        style,
        glyphs,
        left_width,
      } => {
        for glyph in glyphs {
          self.glyph(glyph.character, style, x + glyph.x, raise + glyph.raise);
        }
        self.paint(content, x + left_width, raise);
      }
      Paint::Nary {
        arguments,
        style,
        glyphs,
        limit_width,
        body_x,
        lower_down,
        upper_raise,
      } => {
        let [lower, upper, body] = arguments.as_ref();
        for glyph in glyphs {
          self.glyph(glyph.character, style, x + glyph.x, raise + glyph.raise);
        }
        self.paint(
          lower,
          x + (limit_width - lower.width) / 2.0,
          raise - lower_down,
        );
        self.paint(
          upper,
          x + (limit_width - upper.width) / 2.0,
          raise + upper_raise,
        );
        self.paint(body, x + body_x, raise);
      }
      Paint::Shift { content, raise_pt } => self.paint(content, x, raise + raise_pt),
      Paint::Advance => {}
      Paint::Border { content, sides } => {
        let left = if sides.left { 4.0 } else { 0.0 };
        self.paint(content, x + left, raise);
        let top = raise + content.printer.ascent + 2.0 * DEVICE_DOT_PT;
        let bottom = raise - content.printer.descent - DEVICE_DOT_PT;
        let right = x + layout.width - 3.0;
        if sides.top {
          self.rule(x + 1.0, top, right, top);
        }
        if sides.bottom {
          self.rule(x + 1.0, bottom, right, bottom);
        }
        if sides.left {
          self.rule(x + 1.0, bottom, x + 1.0, top);
        }
        if sides.right {
          self.rule(right, bottom, right, top);
        }
      }
    }
  }
}

pub(super) fn push_items(
  page: &mut Page,
  paragraph: &crate::docx::Paragraph,
  equation: &ExpressionInline,
  placement: TextPlacement,
  segmentation: TextSegmentation,
  text_metrics: &mut TextMetrics,
) {
  let (layout, _) = equation_layout(equation, text_metrics);
  Painter {
    page,
    paragraph,
    equation,
    placement,
    segmentation,
    provisional_baseline: layout.logical.ascent,
    text_metrics,
  }
  .paint(&layout, placement.x_pt, 0.0);
}

#[cfg(test)]
mod tests {
  use super::*;

  fn style(family: &str, size: f32) -> TextStyle {
    TextStyle {
      font_family: Some(family.into()),
      high_ansi_font_family: Some(family.into()),
      font_size_pt: size,
      wordprocessingml_font_slots: true,
      use_windows_font_metrics: true,
      ..Default::default()
    }
  }

  #[test]
  fn fraction_axes_and_nested_reserves_match_native_gdi_across_fonts_and_sizes() {
    let mut metrics = TextMetrics::new();
    // Actual600DPI GDI A/D/leading and Word PDF positions. These predict
    //192native field-form/compatibility controls, including nested fractions.
    for (family, size, axis, up, down, nested_top) in [
      ("Arial", 8.0, 3.6, 5.4, 3.96, 15.6),
      ("Arial", 12.0, 5.4, 8.04, 5.88, 22.8),
      ("Arial", 18.0, 8.04, 12.0, 8.88, 33.72),
      ("Times New Roman", 8.0, 3.6, 5.52, 4.08, 15.96),
      ("Times New Roman", 12.0, 5.4, 8.28, 6.0, 23.4),
      ("Times New Roman", 18.0, 8.04, 12.36, 8.88, 34.44),
    ] {
      let equation =
        crate::docx::eq::expression(r"EQ \f(1,2)", &[], &style(family, size), None).unwrap();
      let (actual, _) = equation_layout(&equation, &mut metrics);
      let Paint::Row(nodes) = actual.paint else {
        panic!("row")
      };
      let Paint::Fraction {
        axis: actual_axis,
        up: actual_up,
        down: actual_down,
        ..
      } = nodes[0].paint
      else {
        panic!("fraction")
      };
      for (a, e) in [(actual_axis, axis), (actual_up, up), (actual_down, down)] {
        assert!((a - e).abs() < 0.001, "{family}/{size}:actual{a},native{e}");
      }
      let nested =
        crate::docx::eq::expression(r"EQ \f(\f(1,2),3)", &[], &style(family, size), None).unwrap();
      let (actual, _) = equation_layout(&nested, &mut metrics);
      let Paint::Row(nodes) = actual.paint else {
        panic!("row")
      };
      let Paint::Fraction { up: outer_up, .. } = nodes[0].paint else {
        panic!("fraction")
      };
      assert!(
        (outer_up + up - nested_top).abs() < 0.001,
        "{family}/{size}"
      );
    }
  }

  #[test]
  fn radicals_keep_native_font_advances_and_full_size_degree_reserves() {
    let mut metrics = TextMetrics::new();
    // Native PDF/XPS controls distinguish the source half-point height,
    // Symbol head advance, full-size degree and nested fraction reserve.
    for (family, size, height, width, degree_raise, tall_height, tall_width) in [
      ("Arial", 8.0, 9.0, 4.92, 5.28, 19.56, 10.68),
      ("Arial", 12.0, 14.04, 7.68, 8.16, 28.56, 15.6),
      ("Arial", 18.0, 20.52, 11.28, 11.76, 42.48, 23.28),
      ("Times New Roman", 8.0, 9.0, 4.92, 5.16, 19.56, 10.68),
      ("Times New Roman", 12.0, 14.04, 7.68, 7.92, 28.56, 15.6),
      ("Times New Roman", 18.0, 20.52, 11.28, 11.4, 42.48, 23.28),
    ] {
      for (code, expected_height, expected_width, has_degree) in [
        (r"EQ \r(b)", height, width, false),
        (r"EQ \r(-3,a)", height, width, true),
        (r"EQ \r(\f(1,2))", tall_height, tall_width, false),
      ] {
        let equation = crate::docx::eq::expression(code, &[], &style(family, size), None).unwrap();
        let (actual, _) = equation_layout(&equation, &mut metrics);
        let Paint::Row(children) = actual.paint else {
          panic!("row")
        };
        let root = &children[0];
        let Paint::Radical {
          height,
          head_width,
          degree,
          degree_raise: actual_raise,
          radicand,
          ..
        } = &root.paint
        else {
          panic!("root")
        };
        for (a, e) in [(*height, expected_height), (*head_width, expected_width)] {
          assert!(
            (a - e).abs() < 0.001,
            "{family}/{size}/{code}:actual{a},native{e}"
          );
        }
        assert_eq!(degree.is_some(), has_degree);
        assert_eq!(root.logical.descent, radicand.logical.descent);
        if let Some(degree) = degree {
          assert!((*actual_raise - degree_raise).abs() < 0.001);
          assert!(
            (root.logical.height()
              - (radicand.logical.height() * 2.0).round() / 2.0
              - 2.0
              - 0.75 * degree.logical.height())
            .abs()
              < 0.001
          );
        }
      }
    }
  }

  #[test]
  fn automatic_script_paint_does_not_enlarge_the_restored_root_cell() {
    let mut metrics = TextMetrics::new();
    let base = style("Arial", 12.0);
    let code = r"EQ \f(\r(b2-4ac),2a)";
    let start = code.find('2').unwrap();
    let automatic = TextStyle {
      font_size_pt: 8.0,
      automatic_escapement_font_size_pt: Some(12.0),
      baseline_shift_pt: 4.0,
      ..base.clone()
    };
    for (script, expected_height) in [
      (automatic, 14.04),
      (
        TextStyle {
          baseline_shift_pt: 4.0,
          ..base.clone()
        },
        18.0,
      ),
    ] {
      let equation = crate::docx::eq::expression(
        code,
        &[
          (0, base.clone()),
          (start, script),
          (start + 1, base.clone()),
        ],
        &base,
        None,
      )
      .unwrap();
      let (actual, _) = equation_layout(&equation, &mut metrics);
      let Paint::Row(children) = actual.paint else {
        panic!("row")
      };
      let Paint::Fraction { arguments, .. } = &children[0].paint else {
        panic!("fraction")
      };
      let Paint::Row(numerator) = &arguments[0].paint else {
        panic!("numerator")
      };
      let Paint::Radical { height, .. } = numerator[0].paint else {
        panic!("root")
      };
      assert!((height - expected_height).abs() < 0.001);
    }
  }

  #[test]
  fn recursive_array_printer_centers_match_native_odd_and_even_row_boxes() {
    let mut metrics = TextMetrics::new();
    // Native48 label-relative digit controls, independently paired with GDI
    // A/D/leading. These are observed ascents in600DPI dots, not an expected
    // value recalculated by the implementation's centering formula.
    for (family, size, ascents) in [
      ("Arial", 8.0, [68.0, 107.0, 145.0, 222.0]),
      ("Arial", 12.0, [102.0, 160.0, 217.0, 332.0]),
      ("Arial", 18.0, [153.0, 240.0, 326.0, 499.0]),
      ("Times New Roman", 8.0, [69.0, 109.0, 148.0, 227.0]),
      ("Times New Roman", 12.0, [104.0, 163.0, 222.0, 340.0]),
      ("Times New Roman", 18.0, [155.0, 243.0, 331.0, 507.0]),
    ] {
      for (rows, native_ascent) in [1, 2, 3, 5].into_iter().zip(ascents) {
        let cells = std::iter::repeat_n("1", rows).collect::<Vec<_>>().join(",");
        let equation = crate::docx::eq::expression(
          &format!(r"EQ \b(\a({cells}))"),
          &[],
          &style(family, size),
          None,
        )
        .unwrap();
        let (actual, _) = equation_layout(&equation, &mut metrics);
        let Paint::Row(children) = &actual.paint else {
          panic!("row")
        };
        let Paint::Brackets { content, .. } = &children[0].paint else {
          panic!("brackets")
        };
        assert!(
          (content.printer.ascent / DEVICE_DOT_PT - native_ascent).abs() < 0.001,
          "{family}/{size}/{rows}rows: {:?}",
          content.printer,
        );
      }
    }
  }

  #[test]
  fn delimiter_tiers_and_extender_counts_match_native_matrix_growth() {
    let mut metrics = TextMetrics::new();
    let base = style("Arial", 12.0);
    // Actual1..20row controls retain24pt as the final Symbol tier. This
    // distinguishes capped piece selection from continuous font growth.
    for (rows, font_size, glyph_count) in [
      (1, 14.0, 2),
      (2, 12.0, 4),
      (3, 12.0, 8),
      (4, 18.0, 6),
      (5, 18.0, 8),
      (6, 24.0, 8),
      (10, 24.0, 12),
      (20, 24.0, 24),
    ] {
      let cells = (0..rows)
        .map(|i| ((i % 9) + 1).to_string())
        .collect::<Vec<_>>()
        .join(",");
      let code = format!(r"EQ \b(\a({cells}))");
      let equation = crate::docx::eq::expression(&code, &[], &base, None).unwrap();
      let (actual, _) = equation_layout(&equation, &mut metrics);
      let Paint::Row(children) = actual.paint else {
        panic!("row")
      };
      let Paint::Brackets {
        style,
        glyphs,
        content,
        ..
      } = &children[0].paint
      else {
        panic!("brackets")
      };
      assert_eq!(style.font_size_pt, font_size, "{rows}rows");
      assert_eq!(glyphs.len(), glyph_count, "{rows}rows");
      let Paint::Row(content) = &content.paint else {
        panic!("bracket argument row")
      };
      let Paint::Array {
        cells,
        rows: actual_rows,
        ..
      } = &content[0].paint
      else {
        panic!("array")
      };
      assert_eq!(cells.len(), rows);
      assert_eq!(actual_rows.len(), rows);
    }
    let equation =
      crate::docx::eq::expression(r"EQ \b(\f(1,2))", &[], &style("Arial", 8.0), None).unwrap();
    let (actual, _) = equation_layout(&equation, &mut metrics);
    let Paint::Row(children) = actual.paint else {
      panic!("row")
    };
    let Paint::Brackets { glyphs, .. } = &children[0].paint else {
      panic!("brackets")
    };
    assert!((glyphs[0].raise + 3.6).abs() < 0.001);
  }

  #[test]
  fn nary_source_boxes_and_printer_baselines_match_independent_native_observations() {
    let mut metrics = TextMetrics::new();
    // Post-metric and final-box hardware observations in owned native Word
    // derivatives. Source-layout records have paired equal4096unit metrics;
    // the later printer pass has mixed device/source units and can overwrite
    // its source descent. Keep the initial source box for paragraph flow.
    for (size, code, a, d, pa, pd, glyph_raise, lower_down, upper_raise, count) in [
      (
        12.0,
        r"EQ \i\su(i=0,n,c)",
        25.054688,
        17.946777,
        24.96,
        17.88,
        -0.96,
        15.24,
        12.96,
        1,
      ),
      (
        12.0,
        r"EQ \i\su(i=0,n,\r(i))",
        27.256348,
        18.061035,
        27.48,
        17.64,
        -0.36,
        15.0,
        15.12,
        1,
      ),
      (
        8.0,
        r"EQ \i\su(i=0,n,c)",
        16.703125,
        11.761719,
        16.68,
        11.52,
        -0.36,
        9.72,
        8.64,
        1,
      ),
      (
        8.0,
        r"EQ \i\pr(i=0,n,\r(i))",
        18.504395,
        12.076172,
        18.72,
        11.64,
        0.0,
        9.84,
        10.68,
        1,
      ),
      (
        12.0,
        r"EQ \i(a,b,c)",
        25.688477,
        19.610352,
        25.92,
        19.32,
        -2.88,
        16.68,
        14.76,
        2,
      ),
      (
        12.0,
        r"EQ \i(a,b,\f(1,2))",
        33.029297,
        24.43457,
        33.0,
        24.0,
        -7.56,
        21.36,
        21.0,
        3,
      ),
      (
        8.0,
        r"EQ \i(a,b,c)",
        19.636719,
        16.462_89,
        19.92,
        16.2,
        -4.32,
        14.4,
        12.48,
        2,
      ),
      (
        8.0,
        r"EQ \i(a,b,\f(1,2))",
        22.019531,
        17.145508,
        22.08,
        16.8,
        -4.92,
        15.0,
        13.8,
        2,
      ),
    ] {
      let equation = crate::docx::eq::expression(code, &[], &style("Arial", size), None).unwrap();
      let (actual, _) = equation_layout(&equation, &mut metrics);
      let Paint::Row(nodes) = actual.paint else {
        panic!("row")
      };
      let actual = &nodes[0];
      let Paint::Nary {
        glyphs,
        lower_down: lower,
        upper_raise: upper,
        ..
      } = &actual.paint
      else {
        panic!("nary")
      };
      for (actual, expected) in [
        (actual.logical.ascent, a),
        (actual.logical.descent, d),
        (actual.printer.ascent, pa),
        (actual.printer.descent, pd),
        (glyphs[0].raise, glyph_raise),
        (*lower, lower_down),
        (*upper, upper_raise),
      ] {
        assert!(
          (actual - expected).abs() < 0.001,
          "{size}/{code}:actual{actual},native{expected}"
        );
      }
      assert_eq!(glyphs.len(), count, "{size}/{code}");
    }
  }

  #[test]
  fn ordinary_delimiters_retain_carrier_font_and_complete_column_spacing() {
    let mut metrics = TextMetrics::new();
    let base = style("Times New Roman", 8.0);
    // Native hs0/3/6 controls, both bare and bracketed, allocate9pt for
    // three hs3 columns. The small delimiter remains Times New Roman,
    // whereas the larger fraction/matrix tiers switch to Symbol.
    let equation =
      crate::docx::eq::expression(r"EQ \b(\a\co3\hs3(1,2,3,4,5,6))", &[], &base, None).unwrap();
    let (actual, _) = equation_layout(&equation, &mut metrics);
    let Paint::Row(children) = actual.paint else {
      panic!("row")
    };
    let actual = &children[0];
    let Paint::Brackets { style, .. } = &actual.paint else {
      panic!("brackets")
    };
    assert_eq!(style.font_family.as_deref(), Some("Times New Roman"));
    assert_eq!(
      style.high_ansi_font_family.as_deref(),
      Some("Times New Roman")
    );
    assert_eq!(style.font_size_pt, 18.5);
    assert!(
      (actual.width - 33.36).abs() < 0.12,
      "native complete field width: {}",
      actual.width
    );
  }

  #[test]
  fn rules_wait_for_the_complete_mixed_size_line_baseline() {
    use ooxmlsdk::parts::wordprocessing_document::WordprocessingDocument;
    use ooxmlsdk::sdk::WordprocessingDocumentType;
    let mut package = WordprocessingDocument::create(WordprocessingDocumentType::Document);
    let main = package.add_main_document_part().unwrap();
    main.set_data(&mut package,br#"<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:body><w:p><w:pPr><w:spacing w:line="1800" w:lineRule="atLeast"/></w:pPr><w:r><w:rPr><w:rFonts w:ascii="Arial" w:hAnsi="Arial"/><w:sz w:val="16"/></w:rPr><w:t>LEFT</w:t></w:r><w:r><w:rPr><w:rFonts w:ascii="Arial" w:hAnsi="Arial"/><w:sz w:val="24"/></w:rPr><w:fldChar w:fldCharType="begin"/></w:r><w:r><w:rPr><w:rFonts w:ascii="Arial" w:hAnsi="Arial"/><w:sz w:val="24"/></w:rPr><w:instrText>EQ \f(1,2)</w:instrText></w:r><w:r><w:fldChar w:fldCharType="end"/></w:r><w:r><w:rPr><w:rFonts w:ascii="Arial" w:hAnsi="Arial"/><w:sz w:val="40"/></w:rPr><w:t>RIGHT</w:t></w:r></w:p><w:sectPr><w:pgSz w:w="12240" w:h="15840"/><w:pgMar w:top="1440" w:right="1440" w:bottom="1440" w:left="1440"/></w:sectPr></w:body></w:document>"#.to_vec()).unwrap();
    let document = crate::docx::extract(&package, &LayoutOptions::default()).unwrap();
    let result = RootFrameLayout::new(&document, &LayoutOptions::default()).format();
    assert_eq!(result.pages.len(), 1);
    let items = &result.pages[0].items;
    let right = items
      .iter()
      .find_map(|item| match item {
        PageItem::Text(t) if t.text == "RIGHT" => Some(t),
        _ => None,
      })
      .unwrap();
    let baseline = common::wordprocessing_device::line_baseline(
      right.wordprocessing_line_metrics.unwrap(),
      right.y_pt,
    )
    .unwrap();
    let rules = items
      .iter()
      .filter_map(|item| match item {
        PageItem::Line(line) => Some(line),
        _ => None,
      })
      .collect::<Vec<_>>();
    assert_eq!(rules.len(), 1);
    assert_eq!(rules[0].kind, LineItemKind::Stroke);
    assert!((rules[0].y1_pt + 5.4 - baseline).abs() < 0.001);
  }
}
