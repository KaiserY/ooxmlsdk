use super::{
  LO_SUBSCRIPT_BASELINE_SHIFT_SCALE, LO_SUPERSCRIPT_BASELINE_SHIFT_SCALE, LegacyTextRelief,
  MIN_ESCAPEMENT_FONT_SIZE_PT, ParagraphFormat, ParagraphProps, RunProps, RunStyleOverrides,
  StylesCatalog, TextStyle, ThemeColors, ThemeFonts, WORD_DEFAULT_ESCAPEMENT_HEIGHT_SCALE,
  apply_w14_scheme_transforms, automatic_text_color_for_background,
  drawingml_text_effect_common_fill, drawingml_text_outline_effect_common_fill,
  merge_paragraph_format, opacity_from_w14_rgb_transforms, opacity_from_w14_scheme_transforms,
  parse_hex_color, resolve_run_color, resolve_text_fill, resolve_text_outline, shading_fill,
};
use crate::common;
use crate::units;
use ooxmlsdk::schemas::schemas_microsoft_com_office_word_2010_wordml as w14;
use ooxmlsdk::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main as w;
use std::sync::Arc;

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
  run_style_with_character_style_policy(properties, base_style, styles, true)
}

pub(super) fn run_style_without_hyperlink_character_style(
  properties: Option<&w::RunProperties>,
  base_style: TextStyle,
  styles: &StylesCatalog,
) -> TextStyle {
  run_style_with_character_style_policy(properties, base_style, styles, false)
}

fn run_style_with_character_style_policy(
  properties: Option<&w::RunProperties>,
  base_style: TextStyle,
  styles: &StylesCatalog,
  apply_hyperlink_character_style: bool,
) -> TextStyle {
  let mut style = base_style;
  let Some(properties) = properties else {
    return style;
  };

  let character_style_id =
    super::run_properties_run_style(properties).map(|run_style| run_style.val.as_str());
  if apply_hyperlink_character_style
    || !character_style_id.is_some_and(|style_id| styles.is_hyperlink_character_style(style_id))
  {
    style = styles.character_run_style(character_style_id, style);
  }
  merge_run_style(
    &mut style,
    Some(RunProps::Direct(properties)),
    &styles.theme_fonts,
    &styles.theme_colors,
  );
  styles.apply_mapped_reserved_font(
    &mut style,
    RunProps::Direct(properties)
      .run_fonts()
      .and_then(|fonts| fonts.ascii.as_deref()),
  );
  styles.apply_font_substitution(&mut style);
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
  styles.apply_mapped_reserved_font(
    &mut style,
    RunProps::ParagraphMark(properties)
      .run_fonts()
      .and_then(|fonts| fonts.ascii.as_deref()),
  );
  styles.apply_font_substitution(&mut style);
  style
}

pub(super) fn merge_run_style(
  style: &mut TextStyle,
  properties: Option<RunProps<'_>>,
  theme_fonts: &ThemeFonts,
  theme_colors: &ThemeColors,
) {
  // This merger is WordprocessingML-specific. Preserve the rFonts slot
  // classifier even for styles constructed outside the document-default
  // style chain (for example cached field-result runs).
  style.wordprocessingml_font_slots = true;
  let Some(properties) = properties else {
    return;
  };
  let records_field_formatting = matches!(
    &properties,
    RunProps::Direct(_) | RunProps::ParagraphMark(_)
  );

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
    let has_complex_theme_reference = fonts.complex_script_theme.is_some();
    if let Some(font_family) = fonts
      .complex_script
      .as_deref()
      .filter(|value| is_explicit_font_family(value))
      .map(std::sync::Arc::<str>::from)
      .or_else(|| theme_fonts.resolve(fonts.complex_script_theme))
    {
      style.complex_font_family = Some(font_family);
    } else if has_complex_theme_reference {
      // ECMA-376 Part 1 §17.15.1.88 resolves a bidi theme token through
      // w:themeFontLang/@w:bidi and otherwise through the theme's a:cs face.
      // If that lookup is empty, [MS-OI29500] §2.1.87(c) supplies Word's
      // application-defined rFonts default instead of retaining another
      // slot's face or an inherited complex-script face.
      style.complex_font_family = Some(Arc::from("Times New Roman"));
    }
  }
  if let Some(languages) = properties.languages() {
    if let Some(language) = languages.val.as_deref() {
      style.language = Some(Arc::<str>::from(language));
    }
    if let Some(language) = languages.east_asia.as_deref() {
      style.east_asia_language = Some(Arc::<str>::from(language));
    }
    if let Some(language) = languages.bidi.as_deref() {
      style.bidi_language = Some(Arc::<str>::from(language));
    }
  }
  if let Some(bold) = properties.bold() {
    let bold = bold.val.is_none_or(|value| value.as_bool());
    style.bold = bold;
    if records_field_formatting {
      style.wordprocessingml_field_bold_override = Some(bold);
    }
  }
  if let Some(bold) = properties.bold_complex_script() {
    style.complex_bold = Some(bold.val.is_none_or(|value| value.as_bool()));
  }
  if let Some(italic) = properties.italic() {
    style.italic = italic.val.is_none_or(|value| value.as_bool());
  }
  if let Some(italic) = properties.italic_complex_script() {
    style.complex_italic = Some(italic.val.is_none_or(|value| value.as_bool()));
  }
  if let Some(outline) = properties.outline() {
    style.legacy_outline = outline.val.is_none_or(|value| value.as_bool());
  }
  if let Some(shadow) = properties.shadow() {
    style.legacy_shadow = shadow.val.is_none_or(|value| value.as_bool());
  }
  if let Some(emboss) = properties.emboss() {
    let enabled = emboss.val.is_none_or(|value| value.as_bool());
    if enabled {
      style.legacy_relief = LegacyTextRelief::Embossed;
    } else if style.legacy_relief == LegacyTextRelief::Embossed {
      style.legacy_relief = LegacyTextRelief::None;
    }
  }
  if let Some(imprint) = properties.imprint() {
    let enabled = imprint.val.is_none_or(|value| value.as_bool());
    if enabled {
      style.legacy_relief = LegacyTextRelief::Engraved;
    } else if style.legacy_relief == LegacyTextRelief::Engraved {
      style.legacy_relief = LegacyTextRelief::None;
    }
  }
  if let Some(complex_script) = properties.complex_script() {
    style.complex_script = Some(complex_script.val.is_none_or(|value| value.as_bool()));
  }
  if let Some(right_to_left) = properties.right_to_left_text() {
    style.right_to_left = Some(right_to_left.val.is_none_or(|value| value.as_bool()));
  }
  if let Some(font_size) = properties.font_size() {
    let size = font_size.val;
    set_font_size_preserving_automatic_escapement(
      style,
      (size.to_points() as f32).max(MIN_ESCAPEMENT_FONT_SIZE_PT),
    );
  }
  if let Some(font_size) = properties.complex_script_font_size() {
    let size = font_size.val;
    // imports w:szCs as CharHeightComplex. Keep it separate from Western
    // CharHeight so Latin shaping width remains source-backed, while layout
    // line height can still see the complex-script font height.
    set_complex_font_size_preserving_automatic_escapement(
      style,
      (size.to_points() as f32).max(MIN_ESCAPEMENT_FONT_SIZE_PT),
    );
  }
  if let Some(color) = properties.color() {
    if color
      .val
      .as_deref()
      .is_some_and(|value| value.eq_ignore_ascii_case("auto"))
    {
      if matches!(&properties, RunProps::Numbering(_)) {
        style.color = super::RgbColor {
          r: 255,
          g: 255,
          b: 255,
        };
        style.color_is_automatic = false;
      } else {
        style.color_is_automatic = true;
      }
    } else if let Some(rgb) = resolve_run_color(color, theme_colors) {
      style.color = rgb;
      style.color_is_automatic = false;
    }
  }
  if let Some(shading) = properties.shading()
    && let Some(background) = shading_fill(shading)
  {
    // ECMA-376 Part 1 §17.3.2.32 makes run shading a background behind
    // the run contents. Automatic text remains context-sensitive, and Word
    // selects the higher-contrast neutral for a dark run background.
    style.highlight = Some(background);
  }
  if let Some(fill_effect) = properties.text_fill() {
    match drawingml_text_effect_common_fill(fill_effect, theme_colors) {
      Some(common::Fill::None) => {
        // w14:textFill supersedes w:color. Keep the run for layout and for an
        // independently authored outline, but do not paint its interior.
        style.opacity = 0.0;
        style.color_is_automatic = false;
      }
      Some(fill @ common::Fill::Gradient(_)) => {
        // Word's fixed-format writer clips the authored gradient to glyph
        // outlines and retains a separate searchable text layer.
        style.pdf_glyph_outlines = true;
        style.color_is_automatic = false;
        style.opacity = 1.0;
        let mut options = style
          .pdf_glyph_outline_options
          .as_deref()
          .cloned()
          .unwrap_or_default();
        options.semantic_text_overlay = true;
        options.fill = Some(fill);
        style.pdf_glyph_outline_options = Some(Arc::new(options));
      }
      Some(common::Fill::Solid(_)) | None => {
        if let Some(resolved) = resolve_text_fill(fill_effect, theme_colors) {
          style.color = resolved.color;
          style.color_is_automatic = false;
          style.opacity = resolved.opacity;
        }
      }
      Some(common::Fill::Pattern(_))
      | Some(common::Fill::Theme(_))
      | Some(common::Fill::Image { .. }) => {
        // The Word 2010 text-effect schema cannot produce these variants.
      }
    }
  }
  if let Some(outline_effect) = properties.text_outline() {
    style.outline_width_pt = outline_effect
      .line_width
      .map(|width| units::emu_to_points(width as i64))
      .unwrap_or(style.outline_width_pt);
    match drawingml_text_outline_effect_common_fill(outline_effect, theme_colors) {
      Some(common::Fill::None) => {
        style.outline_color = None;
        style.outline_width_pt = 0.0;
        if let Some(options) = style.pdf_glyph_outline_options.as_deref() {
          let mut options = options.clone();
          options.outline_fill = None;
          options.outline_stroke = None;
          style.pdf_glyph_outline_options = Some(Arc::new(options));
        }
      }
      Some(fill @ common::Fill::Gradient(_)) => {
        style.pdf_glyph_outlines = true;
        let mut options = style
          .pdf_glyph_outline_options
          .as_deref()
          .cloned()
          .unwrap_or_default();
        options.semantic_text_overlay = true;
        options.outline_fill = Some(fill);
        style.pdf_glyph_outline_options = Some(Arc::new(options));
      }
      Some(common::Fill::Solid(_)) | None => {
        if let Some(resolved) = resolve_text_outline(outline_effect, theme_colors) {
          style.outline_color = Some(resolved.color);
          style.outline_opacity = resolved.opacity;
        }
      }
      Some(common::Fill::Pattern(_))
      | Some(common::Fill::Theme(_))
      | Some(common::Fill::Image { .. }) => {}
    }
  }
  if let Some(glow) = properties.text_glow()
    && let Some(color) = glow
      .glow_choice
      .as_ref()
      .and_then(|choice| resolve_w14_glow_color(choice, theme_colors))
  {
    style.text_glow = Some(common::drawingml_image_effects::WordprocessingTextGlow {
      radius_px: glow.glow_radius.unwrap_or_default() as f32 / 9_525.0,
      raster_length_scale: 1.0,
      color,
    });
  }
  if let Some(shadow) = properties.text_shadow()
    && let Some(color) = shadow
      .shadow_choice
      .as_ref()
      .and_then(|choice| resolve_w14_shadow_color(choice, theme_colors))
  {
    style.text_shadow = Some(common::drawingml_image_effects::WordprocessingTextShadow {
      blur_radius_px: shadow.blur_radius.unwrap_or_default() as f32 / 9_525.0,
      distance_px: shadow.distance_from_text.unwrap_or_default() as f32 / 9_525.0,
      raster_length_scale: 1.0,
      direction_degrees: shadow.direction_angle.unwrap_or_default() as f32 / 60_000.0,
      scale_x: shadow.horizontal_scaling_factor.unwrap_or(100_000) as f32 / 100_000.0,
      scale_y: shadow.vertical_scaling_factor.unwrap_or(100_000) as f32 / 100_000.0,
      skew_x_degrees: shadow.horizontal_skew_angle.unwrap_or_default() as f32 / 60_000.0,
      skew_y_degrees: shadow.vertical_skew_angle.unwrap_or_default() as f32 / 60_000.0,
      alignment: w14_effect_alignment(shadow.alignment),
      color,
    });
  }
  if let Some(reflection) = properties.text_reflection() {
    style.text_reflection = Some(
      common::drawingml_image_effects::WordprocessingTextReflection {
        blur_radius_px: reflection.blur_radius.unwrap_or_default() as f32 / 9_525.0,
        start_opacity: reflection.starting_opacity.unwrap_or(100_000) as f32 / 100_000.0,
        start_position: reflection.start_position.unwrap_or_default() as f32 / 100_000.0,
        end_opacity: reflection.ending_opacity.unwrap_or_default() as f32 / 100_000.0,
        end_position: reflection.end_position.unwrap_or(100_000) as f32 / 100_000.0,
        distance_px: reflection.distance_from_text.unwrap_or_default() as f32 / 9_525.0,
        direction_degrees: reflection.direction_angle.unwrap_or_default() as f32 / 60_000.0,
        fade_direction_degrees: reflection.fade_direction.unwrap_or_default() as f32 / 60_000.0,
        scale_x: reflection.horizontal_scaling_factor.unwrap_or(100_000) as f32 / 100_000.0,
        scale_y: reflection.vertical_scaling_factor.unwrap_or(100_000) as f32 / 100_000.0,
        skew_x_degrees: reflection.horizontal_skew_angle.unwrap_or_default() as f32 / 60_000.0,
        skew_y_degrees: reflection.vertical_skew_angle.unwrap_or_default() as f32 / 60_000.0,
        alignment: w14_effect_alignment(reflection.alignment),
      },
    );
  }
  if let Some(spacing) = properties.spacing() {
    style.character_spacing_pt = units::twips_to_points(spacing.val as f32);
  }
  if let Some(scale) = properties.character_scale() {
    // ECMA-376 Part 1 §17.3.2.43 scales the character outlines and advances,
    // unlike w:spacing, which only adds pitch. LibreOffice accepts 1..=600
    // and resets an omitted/out-of-range value to 100%.
    let percentage = scale
      .val
      .filter(|percentage| (1..=600).contains(percentage))
      .unwrap_or(100);
    style.horizontal_scale = Some(percentage as f32 / 100.0);
  }
  if let Some(kern) = properties.kern() {
    style.kerning_minimum_size_pt = Some(kern.val as f32 / 2.0);
  }
  if let Some(ligatures) = properties.ligatures() {
    use w14::LigaturesValues as Value;

    style.ligatures = Some(common::OpenTypeLigatures {
      standard: matches!(
        ligatures.val,
        Value::Standard
          | Value::StandardContextual
          | Value::StandardHistorical
          | Value::StandardDiscretional
          | Value::StandardContextualHistorical
          | Value::StandardContextualDiscretional
          | Value::StandardHistoricalDiscretional
          | Value::All
      ),
      contextual: matches!(
        ligatures.val,
        Value::Contextual
          | Value::StandardContextual
          | Value::ContextualHistorical
          | Value::ContextualDiscretional
          | Value::StandardContextualHistorical
          | Value::StandardContextualDiscretional
          | Value::ContextualHistoricalDiscretional
          | Value::All
      ),
      historical: matches!(
        ligatures.val,
        Value::Historical
          | Value::StandardHistorical
          | Value::ContextualHistorical
          | Value::HistoricalDiscretional
          | Value::StandardContextualHistorical
          | Value::StandardHistoricalDiscretional
          | Value::ContextualHistoricalDiscretional
          | Value::All
      ),
      discretionary: matches!(
        ligatures.val,
        Value::Discretional
          | Value::StandardDiscretional
          | Value::ContextualDiscretional
          | Value::HistoricalDiscretional
          | Value::StandardContextualDiscretional
          | Value::StandardHistoricalDiscretional
          | Value::ContextualHistoricalDiscretional
          | Value::All
      ),
    });
  }
  if let Some(position) = properties.position() {
    // ECMA-376 Part 1 §17.3.2.24 defines w:position as a signed half-point
    // displacement from the surrounding text baseline without resizing the
    // font. LibreOffice defers this property until the final run size is
    // known, then imports the same physical displacement as CharEscapement.
    style.baseline_shift_pt = position.val.to_points() as f32;
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
    apply_vertical_text_alignment(style, vertical_alignment.val);
  }
  if let Some(highlight) = properties.highlight() {
    style.highlight = highlight_color(highlight.val);
  }
  if style.color_is_automatic {
    style.color = super::RgbColor { r: 0, g: 0, b: 0 };
  }
}

pub(super) fn apply_vertical_text_alignment(
  style: &mut TextStyle,
  vertical_alignment: w::VerticalPositionValues,
) {
  match vertical_alignment {
    w::VerticalPositionValues::Superscript => {
      apply_automatic_escapement(style, LO_SUPERSCRIPT_BASELINE_SHIFT_SCALE);
    }
    w::VerticalPositionValues::Subscript => {
      apply_automatic_escapement(style, LO_SUBSCRIPT_BASELINE_SHIFT_SCALE);
    }
    w::VerticalPositionValues::Baseline => {
      style.baseline_shift_pt = 0.0;
      style.automatic_escapement_font_size_pt = None;
      style.automatic_escapement_complex_font_size_pt = None;
    }
  }
}

fn apply_automatic_escapement(style: &mut TextStyle, baseline_shift_scale: f32) {
  // LibreOffice's SwSubFont stores the unscaled height/ascent separately:
  // automatic escapement reduces the glyph, but CalcEscHeight returns the
  // original height for line formatting. Reapplying an inherited
  // w:vertAlign must likewise start from that original size, not shrink it a
  // second time.
  let original_font_size = style
    .automatic_escapement_font_size_pt
    .unwrap_or(style.font_size_pt);
  let original_complex_font_size = if style.automatic_escapement_font_size_pt.is_some() {
    style.automatic_escapement_complex_font_size_pt
  } else {
    style.complex_font_size_pt
  };
  let effective_original_size =
    if style.complex_script == Some(true) || style.right_to_left == Some(true) {
      original_complex_font_size.unwrap_or(original_font_size)
    } else {
      original_font_size
    };

  style.automatic_escapement_font_size_pt = Some(original_font_size);
  style.automatic_escapement_complex_font_size_pt = original_complex_font_size;
  style.baseline_shift_pt = effective_original_size * baseline_shift_scale;
  style.font_size_pt =
    (original_font_size * WORD_DEFAULT_ESCAPEMENT_HEIGHT_SCALE).max(MIN_ESCAPEMENT_FONT_SIZE_PT);
  style.complex_font_size_pt = original_complex_font_size
    .map(|size| (size * WORD_DEFAULT_ESCAPEMENT_HEIGHT_SCALE).max(MIN_ESCAPEMENT_FONT_SIZE_PT));
}

pub(super) fn set_font_size_preserving_automatic_escapement(
  style: &mut TextStyle,
  font_size_pt: f32,
) {
  if style.automatic_escapement_font_size_pt.is_none() {
    style.font_size_pt = font_size_pt;
    return;
  }
  style.automatic_escapement_font_size_pt = Some(font_size_pt);
  reapply_automatic_escapement(style);
}

pub(super) fn set_complex_font_size_preserving_automatic_escapement(
  style: &mut TextStyle,
  font_size_pt: f32,
) {
  if style.automatic_escapement_font_size_pt.is_none() {
    style.complex_font_size_pt = Some(font_size_pt);
    return;
  }
  style.automatic_escapement_complex_font_size_pt = Some(font_size_pt);
  reapply_automatic_escapement(style);
}

fn reapply_automatic_escapement(style: &mut TextStyle) {
  let shift_scale = if style.baseline_shift_pt < 0.0 {
    LO_SUBSCRIPT_BASELINE_SHIFT_SCALE
  } else {
    LO_SUPERSCRIPT_BASELINE_SHIFT_SCALE
  };
  // Restore the unscaled values before calling the common transform; this
  // covers a direct w:sz/w:szCs override layered on an inherited character-
  // style w:vertAlign (tdf82173_endnoteStyle).
  style.font_size_pt = style
    .automatic_escapement_font_size_pt
    .unwrap_or(style.font_size_pt);
  style.complex_font_size_pt = style.automatic_escapement_complex_font_size_pt;
  style.automatic_escapement_font_size_pt = None;
  style.automatic_escapement_complex_font_size_pt = None;
  apply_automatic_escapement(style, shift_scale);
}

pub(super) fn merge_doc_default_run_style(
  style: &mut TextStyle,
  properties: Option<&w::RunPropertiesBaseStyle>,
  theme_fonts: &ThemeFonts,
  theme_colors: &ThemeColors,
) {
  let Some(properties) = properties else {
    return;
  };
  // Word ignores w:position in docDefaults even though the same property is
  // inherited from ordinary paragraph/character styles. LibreOffice keeps
  // this distinction in DomainMapper::lcl_sprm() and regression
  // testTdf140572_docDefault_superscript.
  let mut effective = properties.clone();
  effective.position = None;
  merge_run_style(
    style,
    Some(RunProps::BaseStyle(&effective)),
    theme_fonts,
    theme_colors,
  );
  if style.color_is_automatic
    && let Some(background) = effective.shading.as_ref().and_then(shading_fill)
  {
    // Office adapts automatic text to dark shading inherited from
    // docDefaults. Direct run shading and w:highlight retain black automatic
    // text, so this producer behavior stays scoped to its source level.
    style.color = automatic_text_color_for_background(background);
  }
}

fn resolve_w14_glow_color(
  choice: &w14::GlowChoice,
  theme_colors: &ThemeColors,
) -> Option<common::drawingml_image_effects::ResolvedEffectColor> {
  match choice {
    w14::GlowChoice::RgbColorModelHex(color) => resolve_w14_rgb_effect_color(color),
    w14::GlowChoice::SchemeColor(color) => resolve_w14_scheme_effect_color(color, theme_colors),
  }
}

fn resolve_w14_shadow_color(
  choice: &w14::ShadowChoice,
  theme_colors: &ThemeColors,
) -> Option<common::drawingml_image_effects::ResolvedEffectColor> {
  match choice {
    w14::ShadowChoice::RgbColorModelHex(color) => resolve_w14_rgb_effect_color(color),
    w14::ShadowChoice::SchemeColor(color) => resolve_w14_scheme_effect_color(color, theme_colors),
  }
}

fn resolve_w14_rgb_effect_color(
  color: &w14::RgbColorModelHex,
) -> Option<common::drawingml_image_effects::ResolvedEffectColor> {
  Some(common::drawingml_image_effects::ResolvedEffectColor {
    color: parse_hex_color(color.val.as_str())?,
    alpha: (opacity_from_w14_rgb_transforms(&color.rgb_color_model_hex_choice) * 255.0)
      .round()
      .clamp(0.0, 255.0) as u8,
  })
}

fn resolve_w14_scheme_effect_color(
  color: &w14::SchemeColor,
  theme_colors: &ThemeColors,
) -> Option<common::drawingml_image_effects::ResolvedEffectColor> {
  Some(common::drawingml_image_effects::ResolvedEffectColor {
    color: apply_w14_scheme_transforms(
      theme_colors.resolve_word2010(color.val)?,
      &color.scheme_color_choice,
    ),
    alpha: (opacity_from_w14_scheme_transforms(&color.scheme_color_choice) * 255.0)
      .round()
      .clamp(0.0, 255.0) as u8,
  })
}

fn w14_effect_alignment(alignment: Option<w14::RectangleAlignmentValues>) -> (f32, f32) {
  match alignment.unwrap_or(w14::RectangleAlignmentValues::Bottom) {
    w14::RectangleAlignmentValues::TopLeft => (0.0, 0.0),
    w14::RectangleAlignmentValues::Top => (0.5, 0.0),
    w14::RectangleAlignmentValues::TopRight => (1.0, 0.0),
    w14::RectangleAlignmentValues::Left => (0.0, 0.5),
    w14::RectangleAlignmentValues::Center => (0.5, 0.5),
    w14::RectangleAlignmentValues::Right => (1.0, 0.5),
    w14::RectangleAlignmentValues::BottomLeft => (0.0, 1.0),
    w14::RectangleAlignmentValues::Bottom | w14::RectangleAlignmentValues::None => (0.5, 1.0),
    w14::RectangleAlignmentValues::BottomRight => (1.0, 1.0),
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
