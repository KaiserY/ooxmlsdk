use ooxmlsdk::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main as x;

use crate::model::TextStyle;
use crate::units;

// Excel's fixed-output rich-text superscript is a printer-device operation.
// The immutable 45540 Footer/Header and 45544 Office PDFs independently map a
// 10pt Arial run at the 95% paper transform from 79 device dots (9.48pt) to
// 53 dots (6.36pt), and raise its baseline by 39 dots (4.68pt). This is a
// two-thirds glyph scale and a floored half-height baseline shift; it is
// deliberately separate from Word's 65% automatic escapement.
const XLSX_SUPERSCRIPT_FONT_SCALE: f32 = 2.0 / 3.0;
const XLSX_SUPERSCRIPT_BASELINE_SHIFT_SCALE: f32 = 0.5;
// No matching rich-text Office Golden in the local corpus fixes SpreadsheetML
// subscript's exact device transform. LibreOffice's OOXML importer and
// EditEngine provide the source-backed fallback for that opposite state.
const XLSX_SUBSCRIPT_FONT_SCALE: f32 = 0.58;
const XLSX_SUBSCRIPT_BASELINE_SHIFT_SCALE: f32 = -0.08;

pub(crate) fn decode_excel_escaped_text(text: &str) -> String {
  let mut output = String::with_capacity(text.len());
  let mut index = 0;
  while index < text.len() {
    let rest = &text[index..];
    if rest.len() >= 7 && rest.as_bytes()[0] == b'_' && rest.as_bytes()[1] == b'x' {
      let hex = &rest[2..6];
      if hex.as_bytes().iter().all(u8::is_ascii_hexdigit) && rest.as_bytes()[6] == b'_' {
        if hex.eq_ignore_ascii_case("005F") && rest.as_bytes().get(7) == Some(&b'x') {
          output.push('_');
          index += 7;
          continue;
        }
        if let Ok(value) = u32::from_str_radix(hex, 16)
          && let Some(decoded) = char::from_u32(value)
        {
          output.push(decoded);
          index += 7;
          continue;
        }
      }
    }
    let ch = rest.chars().next().expect("non-empty rest has a char");
    output.push(ch);
    index += ch.len_utf8();
  }
  output
}

pub(crate) fn apply_vertical_text_alignment(
  style: &mut TextStyle,
  vertical_alignment: x::VerticalAlignmentRunValues,
) {
  restore_automatic_escapement(style);
  if vertical_alignment == x::VerticalAlignmentRunValues::Baseline {
    return;
  }

  let original_size_pt = style.font_size_pt;
  let original_complex_size_pt = style.complex_font_size_pt;
  let (font_scale, baseline_shift_scale) = vertical_alignment_parameters(vertical_alignment);
  style.automatic_escapement_font_size_pt = Some(original_size_pt);
  style.automatic_escapement_complex_font_size_pt = original_complex_size_pt;
  style.font_size_pt = (original_size_pt * font_scale).max(1.0);
  style.complex_font_size_pt =
    original_complex_size_pt.map(|size_pt| (size_pt * font_scale).max(1.0));
  style.baseline_shift_pt = original_size_pt * baseline_shift_scale;
}

pub(crate) fn apply_fixed_output_vertical_text_alignment(
  style: &mut TextStyle,
  vertical_alignment: x::VerticalAlignmentRunValues,
) {
  restore_automatic_escapement(style);
  if vertical_alignment == x::VerticalAlignmentRunValues::Baseline {
    return;
  }

  let original_size_pt = style.font_size_pt;
  let original_complex_size_pt = style.complex_font_size_pt;
  let (font_scale, baseline_shift_scale) = vertical_alignment_parameters(vertical_alignment);
  style.automatic_escapement_font_size_pt = Some(original_size_pt);
  style.automatic_escapement_complex_font_size_pt = original_complex_size_pt;
  style.font_size_pt =
    units::quantize_points_to_office_print_grid(original_size_pt * font_scale).max(1.0);
  style.complex_font_size_pt = original_complex_size_pt
    .map(|size_pt| units::quantize_points_to_office_print_grid(size_pt * font_scale).max(1.0));
  let raw_shift_pt = original_size_pt * baseline_shift_scale;
  style.baseline_shift_pt = if vertical_alignment == x::VerticalAlignmentRunValues::Superscript {
    // GDI records an integer baseline displacement. The 79-dot 45540 font
    // therefore raises by floor(79 / 2) = 39 dots, not a rounded 40 dots.
    (raw_shift_pt * units::OFFICE_FIXED_OUTPUT_DPI / units::POINTS_PER_INCH).floor()
      * units::POINTS_PER_INCH
      / units::OFFICE_FIXED_OUTPUT_DPI
  } else {
    units::quantize_points_to_office_print_grid(raw_shift_pt)
  };
}

pub(crate) fn scale_text_style_for_fixed_output(style: &mut TextStyle, print_scale: f32) {
  let automatic_alignment = style.automatic_escapement_font_size_pt.map(|_| {
    if style.baseline_shift_pt < 0.0 {
      x::VerticalAlignmentRunValues::Subscript
    } else {
      x::VerticalAlignmentRunValues::Superscript
    }
  });
  if let Some(vertical_alignment) = automatic_alignment {
    let original_size_pt = style
      .automatic_escapement_font_size_pt
      .expect("automatic alignment retains its original size");
    let original_complex_size_pt = style.automatic_escapement_complex_font_size_pt;
    restore_automatic_escapement(style);
    style.font_size_pt =
      units::quantize_points_to_office_print_grid(original_size_pt * print_scale);
    style.complex_font_size_pt = original_complex_size_pt
      .map(|size_pt| units::quantize_points_to_office_print_grid(size_pt * print_scale));
    apply_fixed_output_vertical_text_alignment(style, vertical_alignment);
    return;
  }

  style.font_size_pt =
    units::quantize_points_to_office_print_grid(style.font_size_pt * print_scale);
  style.complex_font_size_pt = style
    .complex_font_size_pt
    .map(|size_pt| units::quantize_points_to_office_print_grid(size_pt * print_scale));
  style.baseline_shift_pt *= print_scale;
}

fn restore_automatic_escapement(style: &mut TextStyle) {
  if let Some(original_size_pt) = style.automatic_escapement_font_size_pt {
    style.font_size_pt = original_size_pt;
    style.complex_font_size_pt = style.automatic_escapement_complex_font_size_pt;
  }
  style.baseline_shift_pt = 0.0;
  style.automatic_escapement_font_size_pt = None;
  style.automatic_escapement_complex_font_size_pt = None;
}

fn vertical_alignment_parameters(vertical_alignment: x::VerticalAlignmentRunValues) -> (f32, f32) {
  match vertical_alignment {
    x::VerticalAlignmentRunValues::Superscript => (
      XLSX_SUPERSCRIPT_FONT_SCALE,
      XLSX_SUPERSCRIPT_BASELINE_SHIFT_SCALE,
    ),
    x::VerticalAlignmentRunValues::Subscript => (
      XLSX_SUBSCRIPT_FONT_SCALE,
      XLSX_SUBSCRIPT_BASELINE_SHIFT_SCALE,
    ),
    x::VerticalAlignmentRunValues::Baseline => (1.0, 0.0),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn excel_superscript_fixed_output_uses_the_frozen_printer_device_transform() {
    let mut style = TextStyle {
      font_size_pt: 10.0,
      ..TextStyle::default()
    };
    apply_vertical_text_alignment(&mut style, x::VerticalAlignmentRunValues::Superscript);
    scale_text_style_for_fixed_output(&mut style, 0.95);

    assert_eq!(style.automatic_escapement_font_size_pt, Some(9.48));
    assert!((style.font_size_pt - 6.36).abs() < 1.0e-5);
    assert!((style.baseline_shift_pt - 4.68).abs() < 1.0e-5);
  }

  #[test]
  fn explicit_baseline_restores_the_unshrunk_font() {
    let mut style = TextStyle {
      font_size_pt: 10.0,
      ..TextStyle::default()
    };
    apply_vertical_text_alignment(&mut style, x::VerticalAlignmentRunValues::Superscript);
    apply_vertical_text_alignment(&mut style, x::VerticalAlignmentRunValues::Baseline);

    assert_eq!(style.font_size_pt, 10.0);
    assert_eq!(style.baseline_shift_pt, 0.0);
    assert_eq!(style.automatic_escapement_font_size_pt, None);
  }
}
