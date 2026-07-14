use super::{
  LO_DEFAULT_ESCAPEMENT_HEIGHT_SCALE, LO_SUBSCRIPT_BASELINE_SHIFT_SCALE,
  LO_SUPERSCRIPT_BASELINE_SHIFT_SCALE, MIN_ESCAPEMENT_FONT_SIZE_PT, ParagraphFormat,
  ParagraphProps, RunProps, RunStyleOverrides, StylesCatalog, TextStyle, ThemeColors, ThemeFonts,
  merge_paragraph_format, resolve_run_color, resolve_text_fill, resolve_text_outline,
};
use crate::units;
use ooxmlsdk::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main as w;

pub(super) fn paragraph_format(
  styles: &StylesCatalog,
  style_id: Option<&str>,
  base_format: ParagraphFormat,
  direct_properties: Option<ParagraphProps<'_>>,
) -> ParagraphFormat {
  let mut format = styles.paragraph_format_with_base(style_id, base_format);
  merge_paragraph_format(&mut format, direct_properties, styles.import_settings);
  format
}

pub(super) fn paragraph_run_style(
  styles: &StylesCatalog,
  style_id: Option<&str>,
  base_style: TextStyle,
  base_overrides: RunStyleOverrides,
) -> TextStyle {
  styles.run_style_with_base(style_id, base_style, base_overrides)
}

pub(super) fn run_style(
  properties: Option<&w::RunProperties>,
  base_style: TextStyle,
  styles: &StylesCatalog,
) -> TextStyle {
  let mut style = base_style;
  let Some(properties) = properties else {
    return style;
  };

  style = styles.character_run_style(
    super::run_properties_run_style(properties).map(|run_style| run_style.val.as_str()),
    style,
  );
  merge_run_style(
    &mut style,
    Some(RunProps::Direct(properties)),
    &styles.theme_fonts,
    &styles.theme_colors,
  );
  style
}

pub(super) fn paragraph_mark_run_style(
  properties: Option<&w::ParagraphMarkRunProperties>,
  base_style: TextStyle,
  styles: &StylesCatalog,
) -> TextStyle {
  let mut style = base_style;
  let Some(properties) = properties else {
    return style;
  };

  style = styles.character_run_style(
    super::paragraph_mark_run_properties_run_style(properties)
      .map(|run_style| run_style.val.as_str()),
    style,
  );
  merge_run_style(
    &mut style,
    Some(RunProps::ParagraphMark(properties)),
    &styles.theme_fonts,
    &styles.theme_colors,
  );
  style
}

pub(super) fn merge_run_style(
  style: &mut TextStyle,
  properties: Option<RunProps<'_>>,
  theme_fonts: &ThemeFonts,
  theme_colors: &ThemeColors,
) {
  let Some(properties) = properties else {
    return;
  };

  if let Some(fonts) = properties.run_fonts() {
    // LibreOffice maps DOCX rFonts into separate Writer character properties:
    // ascii -> CharFontName, eastAsia -> CharFontNameAsian, cs -> CharFontNameComplex.
    // hAnsi is kept only as interop metadata in writerfilter/dmapper/DomainMapper.cxx.
    if let Some(font_family) = fonts
      .ascii
      .as_deref()
      .filter(|value| is_explicit_font_family(value))
      .map(std::sync::Arc::<str>::from)
      .or_else(|| theme_fonts.resolve(fonts.ascii_theme))
    {
      style.font_family = Some(font_family);
    }
    if let Some(font_family) = fonts
      .east_asia
      .as_deref()
      .filter(|value| is_explicit_font_family(value))
      .map(std::sync::Arc::<str>::from)
      .or_else(|| theme_fonts.resolve(fonts.east_asia_theme))
    {
      style.east_asia_font_family = Some(font_family);
    }
    if let Some(font_family) = fonts
      .complex_script
      .as_deref()
      .filter(|value| is_explicit_font_family(value))
      .map(std::sync::Arc::<str>::from)
      .or_else(|| theme_fonts.resolve(fonts.complex_script_theme))
    {
      style.complex_font_family = Some(font_family);
    }
  }

  if let Some(bold) = properties.bold() {
    style.bold = bold.val.is_none_or(|value| value.as_bool());
  }
  if let Some(italic) = properties.italic() {
    style.italic = italic.val.is_none_or(|value| value.as_bool());
  }
  if let Some(font_size) = properties.font_size() {
    let size = font_size.val;
    style.font_size_pt = (size.to_points() as f32).max(MIN_ESCAPEMENT_FONT_SIZE_PT);
  }
  if let Some(font_size) = properties.complex_script_font_size() {
    let size = font_size.val;
    // imports w:szCs as CharHeightComplex. Keep it separate from Western
    // CharHeight so Latin shaping width remains source-backed, while layout
    // line height can still see the complex-script font height.
    style.complex_font_size_pt = Some((size.to_points() as f32).max(MIN_ESCAPEMENT_FONT_SIZE_PT));
  }
  if let Some(color) = properties.color() {
    if matches!(&properties, RunProps::Numbering(_))
      && color
        .val
        .as_deref()
        .is_some_and(|value| value.eq_ignore_ascii_case("auto"))
    {
      style.color = super::RgbColor {
        r: 255,
        g: 255,
        b: 255,
      };
    } else if let Some(rgb) = resolve_run_color(color, theme_colors) {
      style.color = rgb;
    }
  }
  if let Some(fill) = properties.text_fill()
    && let Some(resolved) = resolve_text_fill(fill, theme_colors)
  {
    style.color = resolved.color;
    style.opacity = resolved.opacity;
  }
  if let Some(outline) = properties.text_outline()
    && let Some(resolved) = resolve_text_outline(outline, theme_colors)
  {
    style.outline_color = Some(resolved.color);
    style.outline_opacity = resolved.opacity;
    style.outline_width_pt = outline
      .line_width
      .map(|width| units::emu_to_points(width as i64))
      .unwrap_or(style.outline_width_pt);
  }
  if let Some(spacing) = properties.spacing() {
    style.character_spacing_pt = units::twips_to_points(spacing.val as f32);
  }
  if let Some(underline) = properties.underline() {
    style.underline = !matches!(underline.val, Some(w::UnderlineValues::None));
  }
  if let Some(strike) = properties.strike() {
    style.strikethrough = strike.val.is_none_or(|value| value.as_bool());
  }
  if let Some(double_strike) = properties.double_strike() {
    style.strikethrough = double_strike.val.is_none_or(|value| value.as_bool());
  }
  if let Some(caps) = properties.caps() {
    style.uppercase = caps.val.is_none_or(|value| value.as_bool());
  }
  if let Some(small_caps) = properties.small_caps() {
    style.small_caps = small_caps.val.is_none_or(|value| value.as_bool());
  }
  if let Some(vanish) = properties.vanish() {
    style.hidden = vanish.val.is_none_or(|value| value.as_bool());
  }
  if let Some(vertical_alignment) = properties.vertical_text_alignment() {
    match vertical_alignment.val {
      w::VerticalPositionValues::Superscript => {
        style.baseline_shift_pt = style.font_size_pt * LO_SUPERSCRIPT_BASELINE_SHIFT_SCALE;
        style.font_size_pt = (style.font_size_pt * LO_DEFAULT_ESCAPEMENT_HEIGHT_SCALE)
          .max(MIN_ESCAPEMENT_FONT_SIZE_PT);
      }
      w::VerticalPositionValues::Subscript => {
        style.baseline_shift_pt = style.font_size_pt * LO_SUBSCRIPT_BASELINE_SHIFT_SCALE;
        style.font_size_pt = (style.font_size_pt * LO_DEFAULT_ESCAPEMENT_HEIGHT_SCALE)
          .max(MIN_ESCAPEMENT_FONT_SIZE_PT);
      }
      w::VerticalPositionValues::Baseline => {
        style.baseline_shift_pt = 0.0;
      }
    }
  }
  if let Some(highlight) = properties.highlight() {
    style.highlight = highlight_color(highlight.val);
  }
}

fn is_explicit_font_family(value: &str) -> bool {
  let value = value.trim();
  !value.is_empty()
    && !value.eq_ignore_ascii_case("default")
    && !value.eq_ignore_ascii_case("inherit")
}

fn highlight_color(value: w::HighlightColorValues) -> Option<super::RgbColor> {
  Some(match value {
    w::HighlightColorValues::Black => super::RgbColor { r: 0, g: 0, b: 0 },
    w::HighlightColorValues::Blue => super::RgbColor { r: 0, g: 0, b: 255 },
    w::HighlightColorValues::Cyan => super::RgbColor {
      r: 0,
      g: 255,
      b: 255,
    },
    w::HighlightColorValues::Green => super::RgbColor { r: 0, g: 255, b: 0 },
    w::HighlightColorValues::Magenta => super::RgbColor {
      r: 255,
      g: 0,
      b: 255,
    },
    w::HighlightColorValues::Red => super::RgbColor { r: 255, g: 0, b: 0 },
    w::HighlightColorValues::Yellow => super::RgbColor {
      r: 255,
      g: 255,
      b: 0,
    },
    w::HighlightColorValues::White => super::RgbColor {
      r: 255,
      g: 255,
      b: 255,
    },
    w::HighlightColorValues::DarkBlue => super::RgbColor { r: 0, g: 0, b: 128 },
    w::HighlightColorValues::DarkCyan => super::RgbColor {
      r: 0,
      g: 128,
      b: 128,
    },
    w::HighlightColorValues::DarkGreen => super::RgbColor { r: 0, g: 128, b: 0 },
    w::HighlightColorValues::DarkMagenta => super::RgbColor {
      r: 128,
      g: 0,
      b: 128,
    },
    w::HighlightColorValues::DarkRed => super::RgbColor { r: 128, g: 0, b: 0 },
    w::HighlightColorValues::DarkYellow => super::RgbColor {
      r: 128,
      g: 128,
      b: 0,
    },
    w::HighlightColorValues::DarkGray => super::RgbColor {
      r: 128,
      g: 128,
      b: 128,
    },
    w::HighlightColorValues::LightGray => super::RgbColor {
      r: 192,
      g: 192,
      b: 192,
    },
    w::HighlightColorValues::None => return None,
  })
}
