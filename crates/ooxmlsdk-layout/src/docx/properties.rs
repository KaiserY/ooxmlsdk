use super::{
  LO_SUBSCRIPT_BASELINE_SHIFT_SCALE, LO_SUPERSCRIPT_BASELINE_SHIFT_SCALE, LegacyTextRelief,
  MIN_ESCAPEMENT_FONT_SIZE_PT, ParagraphFormat, ParagraphProps, RunProps, RunStyleOverrides,
  StylesCatalog, TextStyle, ThemeColors, ThemeFonts, WORD_DEFAULT_ESCAPEMENT_HEIGHT_SCALE,
  apply_w14_rgb_transforms, apply_w14_scheme_transforms, automatic_text_color_for_background,
  drawingml_text_effect_common_fill, drawingml_text_outline_effect_common_fill,
  merge_paragraph_format_with_theme, opacity_from_w14_rgb_transforms,
  opacity_from_w14_scheme_transforms, parse_hex_color, resolve_run_color, resolve_text_fill,
  resolve_text_outline, text_background_shading_fill, wordprocessing_text_outline_common_stroke,
};
use crate::common;
use crate::units;
use ooxmlsdk::schemas::schemas_microsoft_com_office_word_2010_wordml as w14;
use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;
use ooxmlsdk::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main as w;
use ooxmlsdk::sdk::SdkEnum;
use ooxmlsdk::units::CoordinateValue;
use std::sync::Arc;

pub(super) fn paragraph_format(
  styles: &StylesCatalog,
  style_id: Option<&str>,
  base_format: ParagraphFormat,
  direct_properties: Option<ParagraphProps<'_>>,
) -> ParagraphFormat {
  let mut format = styles.paragraph_format_with_base(style_id, base_format);
  merge_paragraph_format_with_theme(
    &mut format,
    direct_properties,
    styles.import_settings,
    &styles.theme_colors,
  );
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
  styles.apply_word_font_table_mappings(&mut style, RunProps::Direct(properties).run_fonts());
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
  styles
    .apply_word_font_table_mappings(&mut style, RunProps::ParagraphMark(properties).run_fonts());
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
    // [MS-OI29500] section 2.1.88 preserves four independent effective font
    // slots. A theme attribute overrides its direct counterpart only within
    // this rFonts element; inheritance has already supplied earlier values.
    if let Some(font_family) =
      resolve_word_run_font(fonts.ascii.as_deref(), fonts.ascii_theme, theme_fonts)
    {
      style.font_family = Some(font_family);
      style.fallback_font_family = None;
      style.font_family_class = None;
    }
    if let Some(font_family) = resolve_word_run_font(
      fonts.high_ansi.as_deref(),
      fonts.high_ansi_theme,
      theme_fonts,
    ) {
      style.high_ansi_font_family = Some(font_family);
      style.high_ansi_fallback_font_family = None;
      style.high_ansi_font_family_class = None;
    }
    if let Some(font_family) = resolve_word_run_font(
      fonts.east_asia.as_deref(),
      fonts.east_asia_theme,
      theme_fonts,
    ) {
      style.east_asia_font_family = Some(font_family);
      style.east_asia_fallback_font_family = None;
      style.east_asia_font_family_class = None;
      style.east_asia_font_charset = None;
    }
    if let Some(font_family) = resolve_word_run_font(
      fonts.complex_script.as_deref(),
      fonts.complex_script_theme,
      theme_fonts,
    ) {
      style.complex_font_family = Some(font_family);
      style.complex_fallback_font_family = None;
      style.complex_font_family_class = None;
    }
    if let Some(hint) = fonts.hint {
      style.wordprocessingml_font_hint = Some(match hint {
        w::FontTypeHintValues::Default => ooxmlsdk_fonts::WordprocessingFontTypeHint::Default,
        w::FontTypeHintValues::Ascii => ooxmlsdk_fonts::WordprocessingFontTypeHint::Ascii,
        w::FontTypeHintValues::EastAsia => ooxmlsdk_fonts::WordprocessingFontTypeHint::EastAsia,
        w::FontTypeHintValues::ComplexScript => {
          ooxmlsdk_fonts::WordprocessingFontTypeHint::ComplexScript
        }
      });
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
  if let Some(vertical) = properties
    .east_asian_layout()
    .and_then(|layout| layout.vertical)
  {
    // LibreOffice's WordprocessingML mapper imports w:eastAsianLayout/@w:vert
    // as CharRotation: true is 900 tenths of a degree and false is 0.
    style.rotation_deg = if vertical.as_bool() { 90.0 } else { 0.0 };
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
    && let Some(background) = text_background_shading_fill(shading, theme_colors).solid_color()
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
        // Word fixed output emits a painted w14:textOutline as glyph paths.
        // It keeps a separate semantic layer only when w14:textFill removes
        // the interior, as in fdo80897's outlined warped text.
        options.semantic_text_overlay = style.opacity <= f32::EPSILON;
        options.outline_fill = Some(fill);
        options.outline_stroke =
          wordprocessing_text_outline_common_stroke(outline_effect, theme_colors);
        style.pdf_glyph_outline_options = Some(Arc::new(options));
      }
      Some(common::Fill::Solid(_)) | None => {
        if let Some(resolved) = resolve_text_outline(outline_effect, theme_colors) {
          style.outline_color = Some(resolved.color);
          style.outline_opacity = resolved.opacity;
          style.pdf_glyph_outlines = true;
          let mut options = style
            .pdf_glyph_outline_options
            .as_deref()
            .cloned()
            .unwrap_or_default();
          options.semantic_text_overlay = style.opacity <= f32::EPSILON;
          options.outline_stroke =
            wordprocessing_text_outline_common_stroke(outline_effect, theme_colors);
          style.pdf_glyph_outline_options = Some(Arc::new(options));
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
  if let Some(scene) = properties.text_scene_3d() {
    style.wordprocessing_text_3d = true;
    if let Some(scene) = wordprocessing_text_scene_3d(scene) {
      style
        .wordprocessing_text_3d_parts
        .get_or_insert_default()
        .scene = Some(Box::new(scene));
    }
  }
  if let Some(properties) = properties.text_properties_3d() {
    style.wordprocessing_text_3d = true;
    let (shape, extrusion_color, contour_color) =
      wordprocessing_text_shape_3d(properties, theme_colors);
    let parts = style.wordprocessing_text_3d_parts.get_or_insert_default();
    parts.shape = Some(Box::new(shape));
    parts.extrusion_color = Some(extrusion_color);
    parts.contour_color = Some(contour_color);
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
  if let Some(numbering_format) = properties.numbering_format() {
    style.open_type_features.number_form = Some(match numbering_format.val {
      w14::NumberFormValues::Default => common::OpenTypeNumberForm::Default,
      w14::NumberFormValues::Lining => common::OpenTypeNumberForm::Lining,
      w14::NumberFormValues::OldStyle => common::OpenTypeNumberForm::OldStyle,
    });
  }
  if let Some(number_spacing) = properties.number_spacing() {
    style.open_type_features.number_spacing = Some(match number_spacing.val {
      w14::NumberSpacingValues::Default => common::OpenTypeNumberSpacing::Default,
      w14::NumberSpacingValues::Proportional => common::OpenTypeNumberSpacing::Proportional,
      w14::NumberSpacingValues::Tabular => common::OpenTypeNumberSpacing::Tabular,
    });
  }
  if let Some(contextual_alternatives) = properties.contextual_alternatives() {
    style.open_type_features.contextual_alternates = Some(
      contextual_alternatives
        .val
        .is_none_or(wordprocessing_2010_on_off),
    );
  }
  if let Some(stylistic_sets) = properties.stylistic_sets() {
    let mut enabled = common::OpenTypeStylisticSets::default();
    for style_set in &stylistic_sets.style_set {
      // [MS-DOCX] CT_StyleSet limits ids to 1..=20, defaults an omitted
      // `val` to true, and requires false entries to be ignored.
      if style_set.val.is_none_or(wordprocessing_2010_on_off) {
        enabled.enable(style_set.id);
      }
    }
    style.open_type_features.stylistic_sets = Some(enabled);
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

fn wordprocessing_2010_on_off(value: w14::OnOffValues) -> bool {
  matches!(value, w14::OnOffValues::True | w14::OnOffValues::One)
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
    && let Some(background) = effective
      .shading
      .as_ref()
      .and_then(|shading| text_background_shading_fill(shading, theme_colors).solid_color())
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

fn cast_sdk_enum<S: SdkEnum, T: SdkEnum>(value: &S) -> Option<T> {
  T::try_from_xml_bytes(value.as_xml_bytes())
}

fn wordprocessing_text_scene_3d(scene: &w14::Scene3D) -> Option<a::Scene3DType> {
  Some(a::Scene3DType {
    xmlns: Vec::new(),
    camera: Box::new(a::Camera {
      preset: cast_sdk_enum(&scene.camera.preset_camera_type)?,
      field_of_view: None,
      zoom: None,
      rotation: None,
    }),
    light_rig: Box::new(a::LightRig {
      rig: cast_sdk_enum(&scene.light_rig.light_rig_type)?,
      direction: cast_sdk_enum(&scene.light_rig.light_direction_type)?,
      rotation: scene
        .light_rig
        .sphere_coordinates
        .as_ref()
        .map(|rotation| a::Rotation {
          latitude: rotation.lattitude,
          longitude: rotation.longitude,
          revolution: rotation.revolution,
        }),
    }),
    backdrop: None,
    extension_list: None,
  })
}

fn wordprocessing_text_shape_3d(
  properties: &w14::Properties3D,
  theme_colors: &ThemeColors,
) -> (
  a::Shape3DType,
  common::drawingml_3d::Static3dColor,
  common::drawingml_3d::Static3dColor,
) {
  let default_color = common::drawingml_3d::Static3dColor {
    color: super::RgbColor { r: 0, g: 0, b: 0 },
    alpha: u8::MAX,
  };
  let extrusion_color = properties
    .extrusion_color
    .as_deref()
    .and_then(|color| color.extrusion_color_choice.as_ref())
    .and_then(|choice| match choice {
      w14::ExtrusionColorChoice::RgbColorModelHex(color) => resolve_w14_rgb_effect_color(color),
      w14::ExtrusionColorChoice::SchemeColor(color) => {
        resolve_w14_scheme_effect_color(color, theme_colors)
      }
    })
    .map(|color| common::drawingml_3d::Static3dColor {
      color: color.color,
      alpha: color.alpha,
    })
    // [MS-DOCX] §2.6.3.23 specifies black when extrusionClr is absent.
    .unwrap_or(default_color);
  let contour_color = properties
    .contour_color
    .as_deref()
    .and_then(|color| color.contour_color_choice.as_ref())
    .and_then(|choice| match choice {
      w14::ContourColorChoice::RgbColorModelHex(color) => resolve_w14_rgb_effect_color(color),
      w14::ContourColorChoice::SchemeColor(color) => {
        resolve_w14_scheme_effect_color(color, theme_colors)
      }
    })
    .map(|color| common::drawingml_3d::Static3dColor {
      color: color.color,
      alpha: color.alpha,
    })
    // [MS-DOCX] §2.6.3.23 specifies black when contourClr is absent.
    .unwrap_or(default_color);
  let bevel_top = properties.bevel_top.as_ref().map(|bevel| a::BevelTop {
    width: bevel.width.map(CoordinateValue::Emu),
    height: bevel.height.map(CoordinateValue::Emu),
    preset: bevel.preset_profile_type.as_ref().and_then(cast_sdk_enum),
  });
  let bevel_bottom = properties
    .bevel_bottom
    .as_ref()
    .map(|bevel| a::BevelBottom {
      width: bevel.width.map(CoordinateValue::Emu),
      height: bevel.height.map(CoordinateValue::Emu),
      preset: bevel.preset_profile_type.as_ref().and_then(cast_sdk_enum),
    });
  (
    a::Shape3DType {
      xmlns: Vec::new(),
      z: None,
      extrusion_height: properties.extrusion_height.map(CoordinateValue::Emu),
      contour_width: properties.contour_width.map(CoordinateValue::Emu),
      preset_material: properties
        .preset_material_type
        .as_ref()
        .and_then(cast_sdk_enum),
      bevel_top,
      bevel_bottom,
      extrusion_color: None,
      contour_color: None,
      extension_list: None,
    },
    extrusion_color,
    contour_color,
  )
}

fn resolve_w14_rgb_effect_color(
  color: &w14::RgbColorModelHex,
) -> Option<common::drawingml_image_effects::ResolvedEffectColor> {
  Some(common::drawingml_image_effects::ResolvedEffectColor {
    color: apply_w14_rgb_transforms(
      parse_hex_color(color.val.as_str())?,
      &color.rgb_color_model_hex_choice,
    ),
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
  match alignment.unwrap_or(w14::RectangleAlignmentValues::None) {
    w14::RectangleAlignmentValues::TopLeft => (0.0, 0.0),
    w14::RectangleAlignmentValues::Top => (0.5, 0.0),
    w14::RectangleAlignmentValues::TopRight => (1.0, 0.0),
    w14::RectangleAlignmentValues::Left => (0.0, 0.5),
    w14::RectangleAlignmentValues::Center | w14::RectangleAlignmentValues::None => (0.5, 0.5),
    w14::RectangleAlignmentValues::Right => (1.0, 0.5),
    w14::RectangleAlignmentValues::BottomLeft => (0.0, 1.0),
    w14::RectangleAlignmentValues::Bottom => (0.5, 1.0),
    w14::RectangleAlignmentValues::BottomRight => (1.0, 1.0),
  }
}

fn is_explicit_font_family(value: &str) -> bool {
  let value = value.trim();
  !value.is_empty()
    && !value.eq_ignore_ascii_case("default")
    && !value.eq_ignore_ascii_case("inherit")
}

fn resolve_word_run_font(
  direct: Option<&str>,
  theme: Option<w::ThemeFontValues>,
  theme_fonts: &ThemeFonts,
) -> Option<Arc<str>> {
  if theme.is_some() {
    return theme_fonts
      .resolve(theme)
      // [MS-OI29500] section 2.1.88(c) applies Times New Roman only after an
      // actual Theme part or the recovered Office application theme cannot
      // resolve the authored token. It must not replace that missing-package-
      // theme recovery itself.
      .or_else(|| Some(Arc::from("Times New Roman")));
  }
  direct
    .filter(|value| is_explicit_font_family(value))
    .map(Arc::from)
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn word_2010_effect_alignment_defaults_none_to_center() {
    assert_eq!(w14_effect_alignment(None), (0.5, 0.5));
    assert_eq!(
      w14_effect_alignment(Some(w14::RectangleAlignmentValues::None)),
      (0.5, 0.5)
    );
    assert_eq!(
      w14_effect_alignment(Some(w14::RectangleAlignmentValues::Bottom)),
      (0.5, 1.0)
    );
  }
}
