mod custom_xml;
mod drawing;
mod field_datetime;
mod hyphenation;
mod layout;
mod model;
mod package;
mod properties;
mod settings;
mod table;
mod text;
mod toc;

use std::borrow::Cow;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::sync::Arc;

use crate::common::{self, color_math};
use image::{ImageEncoder, codecs::png::PngEncoder};
use kurbo::Affine;
use ooxmlsdk::parts::{
  main_document_part::MainDocumentPart, wordprocessing_document::WordprocessingDocument,
};
use ooxmlsdk::schemas::{
  schemas_microsoft_com_office_drawing_2008_diagram as dsp,
  schemas_microsoft_com_office_office as o, schemas_microsoft_com_office_word as w10,
  schemas_microsoft_com_office_word_2010_wordml as w14,
  schemas_microsoft_com_office_word_2010_wordprocessing_canvas as wpc,
  schemas_microsoft_com_office_word_2010_wordprocessing_drawing as wp14,
  schemas_microsoft_com_office_word_2010_wordprocessing_group as wpg,
  schemas_microsoft_com_office_word_2010_wordprocessing_shape as wps,
  schemas_microsoft_com_vml as v, schemas_openxmlformats_org_drawingml_2006_chart as c,
  schemas_openxmlformats_org_drawingml_2006_diagram as dgm,
  schemas_openxmlformats_org_drawingml_2006_locked_canvas as lc,
  schemas_openxmlformats_org_drawingml_2006_main as a,
  schemas_openxmlformats_org_drawingml_2006_picture as pic,
  schemas_openxmlformats_org_drawingml_2006_wordprocessing_drawing as wp,
  schemas_openxmlformats_org_wordprocessingml_2006_main as w, www_w3_org_xml_1998_namespace as xml,
};
use ooxmlsdk::sdk::SdkType;
use ooxmlsdk::simple_type::{
  DrawingmlPercentageValue, MeasurementOrPercentValue, SignedTwipsMeasureValue, TwipsMeasureValue,
};
use ooxmlsdk::units as sdk_units;
use smallvec::SmallVec;

use crate::common::drawingml_image_effects::{
  ImageEffect, ImageEffectColorResolver, ResolvedEffectColor,
};
use crate::error::Result;
use crate::model::common_rgb;
use crate::options::{
  FieldUpdateDateTime, LayoutActionOptions, LayoutDiagnosticsOptions, LayoutOptions,
};
use crate::pptx::drawingml::color::{Color, RgbHexColor};
use crate::render::chart as shared_chart;
use crate::render::math as shared_math;
use crate::render::symbol as shared_symbol;
use crate::units;

pub(crate) use custom_xml::CustomXmlBindings;
pub(crate) use model::*;
use package::{
  AltChunkCatalog, AltChunkResource, ExtendedChartResource, HyperlinkCatalog, ImageCatalog,
};
use settings::{
  adjust_line_height_in_table, compatibility_mode, default_tab_stop_pt,
  do_not_break_wrapped_tables, do_not_expand_shift_return, do_not_use_html_paragraph_auto_spacing,
  hyphenation_settings, no_column_balance, split_page_break_and_paragraph_mark,
  update_fields_on_open,
};
use table::{TableConditionalStyleMask, TableLookModel};
use text::{
  ParagraphImportBase, paragraph_mark_is_deleted, paragraph_model, paragraph_model_with_base,
};
use toc::{body_level_bookmark_names, paragraph_field_events, refresh_tables_of_contents};

#[derive(Clone, Debug)]
pub struct DocxLayoutSummary {
  pub lines: Vec<DocxLayoutLineSummary>,
  pub rows: Vec<DocxLayoutRowSummary>,
}

#[derive(Clone, Debug)]
pub struct DocxLayoutLineSummary {
  pub page_index: usize,
  pub section_index: usize,
  pub section_page_index: usize,
  pub block_index: Option<usize>,
  pub line_index: usize,
  pub x_pt: f32,
  pub y_pt: f32,
  pub width_pt: f32,
  pub height_pt: f32,
}

#[derive(Clone, Debug)]
pub struct DocxLayoutRowSummary {
  pub page_index: usize,
  pub section_index: usize,
  pub section_page_index: usize,
  pub block_index: Option<usize>,
  pub row_index: usize,
  pub x_pt: f32,
  pub y_pt: f32,
  pub width_pt: f32,
  pub height_pt: f32,
}

const DEFAULT_TAB_STOP_PT: f32 = 36.0;
// Internal marker for a literal U+0009 preserved inside w:t. U+000B is not
// legal XML 1.0 document text, so it cannot collide with source content.
pub(crate) const PRESERVED_WORD_TEXT_TAB: char = '\u{000b}';
// initializes w:cols/@space to 720 twips.
const DEFAULT_SECTION_COLUMN_GAP_PT: f32 = 720.0 / units::TWIPS_PER_POINT;
const DEFAULT_TEXTBOX_MIN_WIDTH_PT: f32 = 11.0;
const DEFAULT_TEXTBOX_MIN_HEIGHT_PT: f32 = 14.0;
const DEFAULT_TEXTBOX_AUTO_FIT_WIDTH_PT: f32 = 200.0;
// OOXML spec defaults: left/right 91440 EMU, top/bottom 45720 EMU.
const DEFAULT_TEXTBOX_LEFT_RIGHT_INSET_PT: f32 = 91_440.0 / sdk_units::EMUS_PER_POINT as f32;
const DEFAULT_TEXTBOX_TOP_BOTTOM_INSET_PT: f32 = 45_720.0 / sdk_units::EMUS_PER_POINT as f32;
const WML_DEFAULT_BORDER_WIDTH_PT: f32 = 0.5;
const WML_MIN_BORDER_WIDTH_PT: f32 = 0.25;
const DRAWINGML_DEFAULT_LINE_WIDTH_EMU: i64 = 0;
const VML_DEFAULT_STROKE_WEIGHT_EMU: i64 = 1;
// Word fixed output scales automatic w:vertAlign superscript/subscript text to
// 65% of the authored size. Writer maps the same markup to its older 58%
// DFLT_ESC_PROP; keep that as importer evidence, not the Office PDF metric.
const WORD_DEFAULT_ESCAPEMENT_HEIGHT_SCALE: f32 = 0.65;
const LO_SUPERSCRIPT_BASELINE_SHIFT_SCALE: f32 = 0.33;
const LO_SUBSCRIPT_BASELINE_SHIFT_SCALE: f32 = -0.08;
const MIN_ESCAPEMENT_FONT_SIZE_PT: f32 = 1.0;
const MIN_IMPORTED_LINE_HEIGHT_PT: f32 = 0.1;
const TAB_STOP_DEDUP_EPSILON_PT: f32 = 0.1;
const MAX_WORD_TABLE_MARGIN_TWIPS: f32 = 31_680.0;
// Word repairs packages without the required main-document Styles part from
// its application defaults. Office-authored Styles parts materialize these as
// 160 twips after the paragraph and 259/240 automatic line spacing.
const OFFICE_RECOVERED_PARAGRAPH_AFTER_PT: f32 = 8.0;
// Writer's OOXML DomainMapper uses the Word binary importer's 280-twip
// paragraph auto-spacing value in print layout. Word fixed-format output
// exposes the same 14pt distance.
const OFFICE_AUTOMATIC_PARAGRAPH_SPACING_PT: f32 = 14.0;
// ECMA-376 Part 4 §14.8.3.15 fixes HTML automatic paragraph spacing at
// 5pt before and 10pt after when w:doNotUseHTMLParagraphAutoSpacing is enabled.
const OFFICE_FIXED_AUTOMATIC_PARAGRAPH_BEFORE_PT: f32 = 5.0;
const OFFICE_FIXED_AUTOMATIC_PARAGRAPH_AFTER_PT: f32 = 10.0;
const OFFICE_RECOVERED_LINE_HEIGHT_MULTIPLE: f32 = 276.0 / 240.0;
// The Simplified Chinese Word application Table Normal context uses 1.5-line
// spacing. This is observable in Microsoft's fixed-format output for a
// package without a Styles part: four DengXian line breaks are 17.28pt apart.
const OFFICE_RECOVERED_ZH_HANS_TABLE_LINE_HEIGHT_MULTIPLE: f32 = 360.0 / 240.0;
// The same repair path synthesizes the normal 360-twip section line pitch.
const OFFICE_RECOVERED_DOCUMENT_GRID_LINE_PITCH_PT: f32 = 18.0;
// ECMA-376 Part 1 §17.6.8 deliberately leaves an omitted w:distance
// implementation-defined. Current Word fixed-format output places the right
// edge of an automatic line number 18pt from the text margin; both Open XML
// SDK Line numbers-Continuous fixtures exercise that omitted-attribute path.
const OFFICE_AUTOMATIC_LINE_NUMBER_DISTANCE_PT: f32 = 18.0;

#[derive(Clone, Copy, Debug, Default)]
struct ImportSettings {
  compatibility_mode: u16,
  justify_lines_with_shrinking: bool,
  fixed_html_paragraph_auto_spacing: bool,
  do_not_break_wrapped_tables: bool,
  do_not_expand_shift_return: bool,
  field_update_datetime: Option<FieldUpdateDateTime>,
  exchange_left_right: bool,
  use_literal_direction: bool,
}

pub(crate) fn extract(
  package: &mut WordprocessingDocument,
  options: &LayoutOptions,
) -> Result<DocxDocument> {
  let main = package.main_document_part()?;
  let compatibility_mode = compatibility_mode(package, &main);
  let fixed_html_paragraph_auto_spacing = do_not_use_html_paragraph_auto_spacing(package, &main);
  let do_not_break_wrapped_tables = do_not_break_wrapped_tables(package, &main);
  let do_not_expand_shift_return = do_not_expand_shift_return(package, &main);
  let document_math_settings = document_math_settings(package, &main);
  let import_settings = ImportSettings {
    compatibility_mode,
    justify_lines_with_shrinking: compatibility_mode >= 15,
    fixed_html_paragraph_auto_spacing,
    do_not_break_wrapped_tables,
    do_not_expand_shift_return,
    field_update_datetime: options.field_update_datetime,
    ..Default::default()
  };
  let mut styles = StylesCatalog::load(
    package,
    &main,
    import_settings,
    options.ui_language.as_deref(),
  )?;
  styles.display_math_alignment = document_math_settings.display_alignment;
  styles.math_font_family = document_math_settings.font_family;
  let mut numbering = NumberingCatalog::load(package, &main, import_settings, &styles)?;
  let images = ImageCatalog::load(package, &main);
  let alt_chunks = AltChunkCatalog::load(package, &main);
  let hyperlinks = HyperlinkCatalog::load(package, &main);
  let custom_xml_bindings = CustomXmlBindings::load(package, &main);
  let mut form_widget_ids = FormWidgetIdAllocator::default();
  let default_tab_stop_pt = default_tab_stop_pt(package, &main);
  let hyphenation = hyphenation_settings(package, &main);
  let even_and_odd_headers = even_and_odd_headers(package, &main);
  let no_column_balance = no_column_balance(package, &main);
  let adjust_line_height_in_table = adjust_line_height_in_table(package, &main);
  let split_page_break_and_paragraph_mark = split_page_break_and_paragraph_mark(package, &main);
  let update_fields_on_open = update_fields_on_open(package, &main);
  let mirror_margins = mirror_margins(package, &main);
  let gutter_at_top = gutter_at_top(package, &main);
  let document = main.root_element(package)?;
  let body_level_bookmarks = document
    .body
    .as_deref()
    .map(body_level_bookmark_names)
    .unwrap_or_default();
  let mut body_styles = styles.clone();
  body_styles.preserve_word_text_whitespace =
    document.space == Some(xml::SpaceProcessingModeValues::Preserve);
  let page_background = document
    .document_background
    .as_deref()
    .and_then(document_background_color);
  let page_background_image = document
    .document_background
    .as_deref()
    .and_then(|background| document_background_image(background, &images));
  let mut sections = document
    .body
    .as_deref()
    .map(|body| {
      body_sections(
        body,
        BodySectionEnv {
          styles: &body_styles,
          numbering: &mut numbering,
          images: &images,
          alt_chunks: &alt_chunks,
          hyperlinks: &hyperlinks,
          custom_xml_bindings: &custom_xml_bindings,
          form_widget_ids: &mut form_widget_ids,
          no_column_balance,
        },
      )
    })
    .unwrap_or_else(|| vec![default_section(Vec::new())]);
  refresh_tables_of_contents(
    &mut sections,
    &body_styles,
    update_fields_on_open,
    options.ui_language.as_deref(),
    &body_level_bookmarks,
  );
  if body_styles.uses_office_recovered_paragraph_defaults() {
    for section in &mut sections {
      if section.page.doc_grid_line_pitch_pt.is_none()
        && section.blocks.iter().any(|block| {
          matches!(block, Block::Paragraph(paragraph) if paragraph.format.style_id.is_none() && paragraph_has_recoverable_main_story_text(paragraph))
        })
      {
        section.page.doc_grid_line_pitch_pt = Some(OFFICE_RECOVERED_DOCUMENT_GRID_LINE_PITCH_PT);
      }
    }
  }
  if let Some(first_section) = sections.first_mut()
    && let Some(image) = page_background_image
  {
    first_section
      .blocks
      .insert(0, page_background_image_block(image, first_section.page));
  }
  for section in &mut sections {
    section.page.background = page_background;
    section.page.mirror_margins = mirror_margins;
    section.page.gutter_at_top = gutter_at_top;
    section.page.adjust_table_line_heights_to_grid = adjust_line_height_in_table;
  }
  resolve_section_repeating_blocks(
    package,
    &main,
    &styles,
    &custom_xml_bindings,
    &mut sections,
    &mut form_widget_ids,
  );
  let (footnote_labels, endnote_labels, footnote_numbering, endnote_numbering) =
    note_reference_labels(package, &main, &sections);
  let footnotes = footnotes(
    package,
    &main,
    &styles,
    &custom_xml_bindings,
    &mut form_widget_ids,
    &footnote_labels,
  )?;
  let footnote_blocks = flatten_note_blocks(&footnotes);
  let endnotes = endnotes(
    package,
    &main,
    &styles,
    &custom_xml_bindings,
    &mut form_widget_ids,
    &endnote_labels,
  )?;
  let endnote_blocks = flatten_note_blocks(&endnotes);
  apply_note_reference_labels(&mut sections, &footnote_labels, &endnote_labels);
  let page = sections
    .first()
    .map(|section| section.page)
    .unwrap_or_default();
  let blocks = sections
    .iter()
    .flat_map(|section| section.blocks.iter().cloned())
    .collect();
  let header_blocks = sections
    .first()
    .map(|section| section.header_blocks.clone())
    .unwrap_or_default();
  let footer_blocks = sections
    .first()
    .map(|section| section.footer_blocks.clone())
    .unwrap_or_default();
  let first_header_blocks = sections
    .first()
    .map(|section| section.first_header_blocks.clone())
    .unwrap_or_default();
  let first_footer_blocks = sections
    .first()
    .map(|section| section.first_footer_blocks.clone())
    .unwrap_or_default();
  let title_page = sections
    .first()
    .map(|section| section.title_page)
    .unwrap_or(false);
  let form_widgets = form_widget_ids.into_widgets();

  Ok(DocxDocument {
    page,
    line_number_style: styles
      .character_run_style(Some("LineNumber"), styles.doc_default_run.clone()),
    has_styles_part: styles.has_styles_part,
    default_tab_stop_pt,
    hyphenation,
    compatibility_mode,
    justify_lines_with_shrinking: import_settings.justify_lines_with_shrinking,
    do_not_expand_shift_return: import_settings.do_not_expand_shift_return,
    even_and_odd_headers,
    split_page_break_and_paragraph_mark,
    form_widgets,
    sections,
    header_blocks,
    footer_blocks,
    first_header_blocks,
    first_footer_blocks,
    footnote_blocks,
    footnotes,
    footnote_numbering,
    endnote_blocks,
    endnotes,
    endnote_numbering,
    title_page,
    blocks,
  })
}

pub fn layout(
  package: &mut WordprocessingDocument,
  options: &LayoutOptions,
) -> Result<crate::common::LayoutDocument<'static>> {
  layout_document(package, options)
}

pub fn layout_document(
  package: &mut WordprocessingDocument,
  options: &LayoutOptions,
) -> Result<crate::common::LayoutDocument<'static>> {
  let document = extract(package, options)?;
  Ok(layout::layout_common_document(&document, options))
}

pub fn layout_anchor_pages(
  package: &mut WordprocessingDocument,
  options: &LayoutOptions,
) -> Result<Vec<crate::common::AnchorPage<'static>>> {
  let document = extract(package, options)?;
  let anchor_options = LayoutOptions {
    source_file_name: None,
    ui_language: options.ui_language.clone(),
    field_update_datetime: options.field_update_datetime,
    action: LayoutActionOptions {
      paint: false,
      ..options.action
    },
    diagnostics: LayoutDiagnosticsOptions::default(),
  };
  let layout = layout::layout(&document, &anchor_options)?;
  Ok(
    layout
      .anchor_pages
      .into_iter()
      .map(|anchor| crate::common::AnchorPage {
        name: std::borrow::Cow::Owned(anchor.name),
        page_index: anchor.page_index,
        section_index: anchor.section_index,
        section_page_index: anchor.section_page_index,
        physical_page_number: anchor.physical_page_number,
        virtual_page_number: anchor.virtual_page_number,
      })
      .collect(),
  )
}

pub fn inspect_layout(
  package: &mut WordprocessingDocument,
  options: &LayoutOptions,
) -> Result<DocxLayoutSummary> {
  let document = extract(package, options)?;
  let layout = layout::layout(&document, options)?;
  Ok(layout::layout_summary(layout))
}

fn simple_text_block(text: String, style: TextStyle) -> Block {
  Block::paragraph(Paragraph {
    inlines: vec![InlineItem::Text(TextRun {
      text,
      style: style.clone(),
      hyperlink_url: None,
      dynamic_field: None,
      style_ref_keys: Vec::new(),
      style_ref_text: None,
      style_ref_numbering_text: None,
      preserve_text_portion: false,
    })],
    field_events: Vec::new(),
    footnote_reference_ids: Vec::new(),
    endnote_reference_ids: Vec::new(),
    starts_after_last_rendered_page_break: false,
    base_style: style.clone(),
    #[cfg(test)]
    runs: Vec::new(),
    format: Box::new(ParagraphFormat::default()),
    style_ref_keys: Vec::new(),
    style_ref_text: None,
    style_ref_numbering_text: None,
    list_label: None,
    list_label_style: TextStyle::default(),
    list_label_hyperlink_url: None,
    list_label_tab_stop_pt: None,
  })
}

fn page_background_image_block(image: InlineShapeImageFill, page: PageSetup) -> Block {
  Block::paragraph(Paragraph {
    inlines: vec![InlineItem::Image(InlineImage {
      data: image.data,
      content_type: image.content_type,
      picture_frame: None,
      effects: None,
      static3d: None,
      width_pt: page.width_pt,
      height_pt: page.height_pt,
      effect_left_pt: 0.0,
      effect_top_pt: 0.0,
      effect_right_pt: 0.0,
      effect_bottom_pt: 0.0,
      crop: image.crop,
      rotation_deg: image.rotation_deg,
      flip_horizontal: image.flip_horizontal,
      flip_vertical: image.flip_vertical,
      metafile_background_color: None,
      alt_text: None,
      hyperlink_url: None,
      semantic_metafile_text: false,
      metafile_native_size: false,
      picture_content_control: false,
      placement: ImagePlacement::Floating(FloatingImagePlacement {
        horizontal_relative_to: HorizontalImageReference::Page,
        vertical_relative_to: VerticalImageReference::Page,
        horizontal_alignment: None,
        vertical_alignment: None,
        horizontal_offset_pt: 0.0,
        vertical_offset_pt: 0.0,
        wrap: ImageWrapMode::None,
        wrap_side: ImageWrapSide::BothSides,
        behind_text: true,
        layout_in_cell: true,
        allow_overlap: true,
        paint_order: FloatingPaintOrder::Unspecified,
        relative_width_to: None,
        relative_width_pct: None,
        relative_height_to: None,
        relative_height_pct: None,
        margin_top_pt: 0.0,
        margin_right_pt: 0.0,
        margin_bottom_pt: 0.0,
        margin_left_pt: 0.0,
      }),
    })],
    field_events: Vec::new(),
    footnote_reference_ids: Vec::new(),
    endnote_reference_ids: Vec::new(),
    starts_after_last_rendered_page_break: false,
    base_style: TextStyle::default(),
    #[cfg(test)]
    runs: Vec::new(),
    format: Box::new(ParagraphFormat::default()),
    style_ref_keys: Vec::new(),
    style_ref_text: None,
    style_ref_numbering_text: None,
    list_label: None,
    list_label_style: TextStyle::default(),
    list_label_hyperlink_url: None,
    list_label_tab_stop_pt: None,
  })
}

fn text_style_with_color(styles: &StylesCatalog, color: RgbColor) -> TextStyle {
  let mut style = styles.doc_default_run.clone();
  style.color = color;
  style
}

fn apply_drawingml_shade(color: RgbColor, amount: f32) -> RgbColor {
  let red = drawingml_rgb_component_to_crgb(color.r);
  let green = drawingml_rgb_component_to_crgb(color.g);
  let blue = drawingml_rgb_component_to_crgb(color.b);
  RgbColor {
    r: drawingml_crgb_component_to_rgb(((red as f32) * amount) as i32),
    g: drawingml_crgb_component_to_rgb(((green as f32) * amount) as i32),
    b: drawingml_crgb_component_to_rgb(((blue as f32) * amount) as i32),
  }
}

fn apply_drawingml_tint(color: RgbColor, amount: f32) -> RgbColor {
  let scale = sdk_units::DRAWINGML_PERCENT_SCALE as f32;
  let red = drawingml_rgb_component_to_crgb(color.r);
  let green = drawingml_rgb_component_to_crgb(color.g);
  let blue = drawingml_rgb_component_to_crgb(color.b);
  RgbColor {
    r: drawingml_crgb_component_to_rgb((scale - (scale - red as f32) * amount) as i32),
    g: drawingml_crgb_component_to_rgb((scale - (scale - green as f32) * amount) as i32),
    b: drawingml_crgb_component_to_rgb((scale - (scale - blue as f32) * amount) as i32),
  }
}

fn drawingml_rgb_component_to_crgb(value: u8) -> i32 {
  color_math::drawingml_srgb8_to_scrgb(value)
}

fn drawingml_crgb_component_to_rgb(value: i32) -> u8 {
  color_math::drawingml_scrgb_to_srgb8(value)
}

fn even_and_odd_headers(package: &mut WordprocessingDocument, main: &MainDocumentPart) -> bool {
  main
    .document_settings_part(package)
    .and_then(|part| part.root_element(package).ok())
    .and_then(|settings| {
      settings
        .even_and_odd_headers
        .as_ref()
        .map(|setting| setting.val.is_none_or(|value| value.as_bool()))
    })
    .unwrap_or(false)
}

fn mirror_margins(package: &mut WordprocessingDocument, main: &MainDocumentPart) -> bool {
  main
    .document_settings_part(package)
    .and_then(|part| part.root_element(package).ok())
    .and_then(|settings| {
      settings
        .mirror_margins
        .as_ref()
        .map(|setting| setting.val.is_none_or(|value| value.as_bool()))
    })
    .unwrap_or(false)
}

fn gutter_at_top(package: &mut WordprocessingDocument, main: &MainDocumentPart) -> bool {
  main
    .document_settings_part(package)
    .and_then(|part| part.root_element(package).ok())
    .and_then(|settings| {
      settings
        .gutter_at_top
        .as_ref()
        .map(|setting| setting.val.is_none_or(|value| value.as_bool()))
    })
    .unwrap_or(false)
}

#[derive(Clone, Debug)]
struct DocumentMathSettings {
  display_alignment: Option<ParagraphAlignment>,
  font_family: Option<Arc<str>>,
}

fn document_math_settings(
  package: &mut WordprocessingDocument,
  main: &MainDocumentPart,
) -> DocumentMathSettings {
  let math_properties = main
    .document_settings_part(package)
    .and_then(|part| part.root_element(package).ok())
    .and_then(|settings| settings.math_properties.as_deref().cloned());
  let font_family = math_properties
    .as_ref()
    .and_then(|properties| properties.math_font.as_ref())
    .map(|font| Arc::<str>::from(font.val.as_str()));
  let display_defaults = math_properties
    .as_ref()
    .and_then(|properties| properties.display_defaults.as_ref())
    .and_then(|display| display.val)
    .is_none_or(|value| {
      matches!(
        value,
        ooxmlsdk::schemas::m::BooleanValues::True
          | ooxmlsdk::schemas::m::BooleanValues::On
          | ooxmlsdk::schemas::m::BooleanValues::One
      )
    });
  if !display_defaults {
    return DocumentMathSettings {
      display_alignment: None,
      font_family,
    };
  }
  let justification = math_properties
    .as_ref()
    .and_then(|properties| properties.default_justification.as_ref())
    .map(|justification| justification.val)
    .unwrap_or(ooxmlsdk::schemas::m::JustificationValues::CenterGroup);
  DocumentMathSettings {
    display_alignment: Some(math_justification_alignment(justification)),
    font_family,
  }
}

fn resolve_section_repeating_blocks(
  package: &mut WordprocessingDocument,
  main: &MainDocumentPart,
  styles: &StylesCatalog,
  custom_xml_bindings: &CustomXmlBindings,
  sections: &mut [ImportedSection],
  form_widget_ids: &mut FormWidgetIdAllocator,
) {
  let mut previous_default_header = Vec::new();
  let mut previous_default_footer = Vec::new();
  let mut previous_first_header = Vec::new();
  let mut previous_first_footer = Vec::new();
  let mut previous_even_header = Vec::new();
  let mut previous_even_footer = Vec::new();

  for section in sections {
    let Some(section_properties) = section.section_properties.as_ref() else {
      section.header_blocks.clone_from(&previous_default_header);
      section.footer_blocks.clone_from(&previous_default_footer);
      section
        .first_header_blocks
        .clone_from(&previous_first_header);
      section
        .first_footer_blocks
        .clone_from(&previous_first_footer);
      section.even_header_blocks.clone_from(&previous_even_header);
      section.even_footer_blocks.clone_from(&previous_even_footer);
      continue;
    };

    section.header_blocks = referenced_header_blocks(
      package,
      main,
      section_properties,
      styles,
      w::HeaderFooterValues::Default,
      custom_xml_bindings,
      form_widget_ids,
    )
    .unwrap_or_else(|| previous_default_header.clone());
    section.footer_blocks = referenced_footer_blocks(
      package,
      main,
      section_properties,
      styles,
      w::HeaderFooterValues::Default,
      custom_xml_bindings,
      form_widget_ids,
    )
    .unwrap_or_else(|| previous_default_footer.clone());
    section.first_header_blocks = referenced_header_blocks(
      package,
      main,
      section_properties,
      styles,
      w::HeaderFooterValues::First,
      custom_xml_bindings,
      form_widget_ids,
    )
    .unwrap_or_else(|| previous_first_header.clone());
    section.first_footer_blocks = referenced_footer_blocks(
      package,
      main,
      section_properties,
      styles,
      w::HeaderFooterValues::First,
      custom_xml_bindings,
      form_widget_ids,
    )
    .unwrap_or_else(|| previous_first_footer.clone());
    section.even_header_blocks = referenced_header_blocks(
      package,
      main,
      section_properties,
      styles,
      w::HeaderFooterValues::Even,
      custom_xml_bindings,
      form_widget_ids,
    )
    .unwrap_or_else(|| previous_even_header.clone());
    section.even_footer_blocks = referenced_footer_blocks(
      package,
      main,
      section_properties,
      styles,
      w::HeaderFooterValues::Even,
      custom_xml_bindings,
      form_widget_ids,
    )
    .unwrap_or_else(|| previous_even_footer.clone());

    previous_default_header.clone_from(&section.header_blocks);
    previous_default_footer.clone_from(&section.footer_blocks);
    previous_first_header.clone_from(&section.first_header_blocks);
    previous_first_footer.clone_from(&section.first_footer_blocks);
    previous_even_header.clone_from(&section.even_header_blocks);
    previous_even_footer.clone_from(&section.even_footer_blocks);
  }
}

struct BodySectionEnv<'a> {
  styles: &'a StylesCatalog,
  numbering: &'a mut NumberingCatalog,
  images: &'a ImageCatalog,
  alt_chunks: &'a AltChunkCatalog,
  hyperlinks: &'a HyperlinkCatalog,
  custom_xml_bindings: &'a CustomXmlBindings,
  form_widget_ids: &'a mut FormWidgetIdAllocator,
  no_column_balance: bool,
}

fn body_sections(body: &w::Body, env: BodySectionEnv<'_>) -> Vec<ImportedSection> {
  let mut sections = Vec::new();
  let mut current_blocks = Vec::new();
  let mut previous_properties = None;
  let mut pending_drop_cap_text = None;
  let mut pending_out_of_place_breaks = Vec::new();
  let BodySectionEnv {
    styles,
    numbering,
    images,
    alt_chunks,
    hyperlinks,
    custom_xml_bindings,
    form_widget_ids,
    no_column_balance,
  } = env;

  for choice in &body.body_choice {
    match choice {
      w::BodyChoice::Paragraph(paragraph) => {
        let section_properties = paragraph
          .paragraph_properties
          .as_deref()
          .and_then(|properties| properties.section_properties.as_deref())
          .cloned();
        let deleted_paragraph_mark = paragraph_mark_is_deleted(paragraph);
        let numbering_state = (section_properties.is_some() || deleted_paragraph_mark)
          .then(|| numbering.counter_state());
        let mut model = paragraph_model(
          paragraph,
          styles,
          numbering,
          images,
          hyperlinks,
          custom_xml_bindings,
          form_widget_ids,
        );
        apply_recovered_body_paragraph_defaults(paragraph, styles, &mut model);
        if prepend_out_of_place_breaks_to_paragraph(&mut model, &pending_out_of_place_breaks) {
          pending_out_of_place_breaks.clear();
        }
        model.format.hidden_separator = paragraph_mark_is_hidden(paragraph);
        model.format.deleted_separator =
          section_properties.is_none() && paragraph_mark_joins_following(paragraph);
        if paragraph_has_drop_cap_frame(&model) {
          pending_drop_cap_text = paragraph_drop_cap_text(&model);
          continue;
        }
        if let Some(text) = pending_drop_cap_text.take() {
          prepend_drop_cap_text(&mut model, text);
        }
        if deleted_paragraph_mark
          && section_properties.is_none()
          && paragraph_body_is_effectively_empty(&model)
        {
          // A paragraph whose mark and entire visible body are deleted or
          // moved-from is absent from Word's current story. It neither paints
          // an empty list label nor advances the numbering counter
          // (tdf#149711).
          numbering.restore_counter_state(
            numbering_state.expect("deleted paragraph captured numbering state"),
          );
          continue;
        }
        let empty_section_carrier =
          section_properties.is_some() && paragraph_body_is_effectively_empty(&model);
        let mut section_metadata_only = false;
        if empty_section_carrier {
          let had_numbering_label = model.list_label.is_some();
          let section_break =
            normalized_section_break(section_properties.as_ref(), previous_properties.as_ref());
          let suppresses_numbering = had_numbering_label
            && empty_section_carrier_suppresses_numbering(paragraph, section_break);
          if suppresses_numbering {
            // Directly numbered, directly indented, continuous, and deleted
            // content-less sectPr paragraphs do not paint or consume a list
            // label. Continuous breaks retain one unnumbered flow line; page
            // breaks and deleted marks leave only section metadata
            // (tdf#97417, tdf#113608, tdf#138892).
            numbering.restore_counter_state(
              numbering_state.expect("section paragraph captured numbering state"),
            );
            model.list_label = None;
            model.style_ref_text = None;
            model.style_ref_numbering_text = None;
            model.list_label_tab_stop_pt = None;
            model.format.list_label_width_aware_tab = false;
            model.format.list_label_uses_explicit_tab_stop = false;
            let keeps_flow_height = !paragraph_mark_is_deleted(paragraph)
              && section_break == SectionBreakKind::Continuous;
            section_metadata_only = !keeps_flow_height;
          } else {
            // Word can retain an inherited style number on an otherwise empty
            // page-section carrier. Besides the visible outline item, this
            // preserves section pagination and STYLEREF state (tdf#170602).
            section_metadata_only = !had_numbering_label;
          }
          if section_metadata_only {
            promote_preceding_table_to_anchored_frame(&mut current_blocks, &model);
          }
        }
        if paragraph_is_effectively_empty(&model)
          && model.field_events.is_empty()
          && section_properties.is_none()
          && current_blocks
            .last()
            .is_some_and(|block| matches!(block, Block::Table(table) if table.placement.is_none()))
        {
          continue;
        }
        if let Some(section_properties) = section_properties {
          if !section_metadata_only {
            push_body_paragraph(&mut current_blocks, model);
          }
          // treats the paragraph carrying sectPr as discarded section metadata;
          // its below spacing is emulated separately instead of creating an
          // extra empty layout paragraph.
          close_section(
            &mut sections,
            &mut current_blocks,
            Some(section_properties),
            &mut previous_properties,
          );
        } else {
          push_body_paragraph(&mut current_blocks, model);
        }
      }
      w::BodyChoice::Table(table) => {
        let mut block = Block::Table(table_model(
          table,
          &mut TableModelEnv {
            styles,
            numbering,
            images,
            hyperlinks,
            custom_xml_bindings,
            form_widget_ids,
          },
          TableModelContext {
            nested_table_level: 1,
            in_header_footer: false,
          },
        ));
        if prepend_out_of_place_breaks_to_first_character_group(
          std::slice::from_mut(&mut block),
          &pending_out_of_place_breaks,
        ) {
          pending_out_of_place_breaks.clear();
        }
        current_blocks.push(block);
      }
      w::BodyChoice::AltChunk(alt_chunk) => {
        let Some(relationship_id) = alt_chunk.id.as_deref() else {
          continue;
        };
        let Some(resource) = alt_chunks.by_relationship_id.get(relationship_id) else {
          continue;
        };
        current_blocks.extend(alt_chunk_blocks(resource, styles.doc_default_run.clone()));
      }
      w::BodyChoice::SdtBlock(sdt) => {
        let mut blocks = sdt_block_blocks(
          sdt,
          styles,
          numbering,
          images,
          hyperlinks,
          SdtBlockControls {
            custom_xml_bindings,
            form_widget_ids,
            in_header_footer: false,
          },
        );
        if prepend_out_of_place_breaks_to_first_character_group(
          &mut blocks,
          &pending_out_of_place_breaks,
        ) {
          pending_out_of_place_breaks.clear();
        }
        current_blocks.extend(blocks);
      }
      w::BodyChoice::Break(br) => {
        // Word postpones a non-conformant block-level break until the next
        // character group. Empty paragraphs therefore remain before a page
        // break, and a break immediately before a table becomes the first
        // character in its first populated cell (tdf#108714 Office golden).
        pending_out_of_place_breaks.push(br.clone());
      }
      _ => {}
    }
  }

  if body.section_properties.is_some() || sections.is_empty() || !current_blocks.is_empty() {
    close_section(
      &mut sections,
      &mut current_blocks,
      body.section_properties.as_deref().cloned(),
      &mut previous_properties,
    );
  }

  for index in 0..sections.len() {
    if sections[index].columns.count <= 1 {
      continue;
    }
    let next_is_continuous = sections
      .get(index + 1)
      .is_some_and(|next| next.break_kind == SectionBreakKind::Continuous);
    if no_column_balance || !next_is_continuous {
      // and sw/source/filter/ww8/ww8par.cxx set DontBalanceTextColumns
      // from w:noColumnBalance, and for multi-column sections followed by a
      // non-continuous break or by the end of the section group.
      sections[index].columns.unbalanced = true;
    }
  }

  sections
}

fn alt_chunk_blocks(resource: &AltChunkResource, style: TextStyle) -> Vec<Block> {
  let content_type = resource
    .content_type
    .as_deref()
    .unwrap_or_default()
    .split(';')
    .next()
    .unwrap_or_default()
    .trim()
    .to_ascii_lowercase();
  let paragraphs = match content_type.as_str() {
    "text/html" | "application/xhtml+xml" => {
      html_alt_chunk_paragraphs(&resource.data, resource.content_type.as_deref())
    }
    "text/plain" => String::from_utf8_lossy(&resource.data)
      .lines()
      .map(str::trim)
      .filter(|line| !line.is_empty())
      .map(str::to_string)
      .collect(),
    _ => Vec::new(),
  };
  paragraphs
    .into_iter()
    .map(|text| simple_text_block(text, style.clone()))
    .collect()
}

fn html_alt_chunk_paragraphs(data: &[u8], content_type: Option<&str>) -> Vec<String> {
  use std::cell::RefCell;

  use html5ever::tendril::StrTendril;
  use html5ever::tokenizer::{
    BufferQueue, TagKind, Token, TokenSink, TokenSinkResult, Tokenizer, TokenizerOpts,
  };

  #[derive(Default)]
  struct HtmlTextState {
    paragraphs: Vec<String>,
    text: String,
    hidden_depth: usize,
  }

  #[derive(Default)]
  struct HtmlTextSink {
    state: RefCell<HtmlTextState>,
  }

  impl TokenSink for HtmlTextSink {
    type Handle = ();

    fn process_token(&self, token: Token, _line_number: u64) -> TokenSinkResult<Self::Handle> {
      let mut state = self.state.borrow_mut();
      match token {
        Token::TagToken(tag) => {
          let name = tag.name.as_ref().as_bytes();
          let hidden = html_hidden_tag(name);
          match tag.kind {
            TagKind::StartTag if state.hidden_depth > 0 => {
              state.hidden_depth += usize::from(hidden);
            }
            TagKind::StartTag if hidden => state.hidden_depth = 1,
            TagKind::StartTag if html_block_tag(name) => {
              let HtmlTextState {
                paragraphs, text, ..
              } = &mut *state;
              push_alt_chunk_paragraph(paragraphs, text);
            }
            TagKind::StartTag if name.eq_ignore_ascii_case(b"br") => state.text.push('\n'),
            TagKind::EndTag if state.hidden_depth > 0 => {
              if hidden {
                state.hidden_depth -= 1;
              }
            }
            TagKind::EndTag if html_block_tag(name) => {
              let HtmlTextState {
                paragraphs, text, ..
              } = &mut *state;
              push_alt_chunk_paragraph(paragraphs, text);
            }
            _ => {}
          }
        }
        Token::CharacterTokens(value) if state.hidden_depth == 0 => state.text.push_str(&value),
        _ => {}
      }
      TokenSinkResult::Continue
    }
  }

  let decoded = decode_html_alt_chunk(data, content_type);
  let input = BufferQueue::default();
  input.push_back(StrTendril::from_slice(&decoded));
  let tokenizer = Tokenizer::new(HtmlTextSink::default(), TokenizerOpts::default());
  let _ = tokenizer.feed(&input);
  tokenizer.end();
  let mut state = tokenizer.sink.state.into_inner();
  push_alt_chunk_paragraph(&mut state.paragraphs, &mut state.text);
  state.paragraphs
}

fn decode_html_alt_chunk<'a>(data: &'a [u8], content_type: Option<&str>) -> Cow<'a, str> {
  if let Some((encoding, bom_len)) = encoding_rs::Encoding::for_bom(data) {
    return encoding.decode(&data[bom_len..]).0;
  }
  if let Some(encoding) = content_type
    .and_then(html_content_type_charset)
    .and_then(|label| encoding_rs::Encoding::for_label(label.as_bytes()))
  {
    return encoding.decode(data).0;
  }
  if std::str::from_utf8(data).is_ok() {
    return String::from_utf8_lossy(data);
  }
  encoding_rs::WINDOWS_1252.decode(data).0
}

fn html_content_type_charset(content_type: &str) -> Option<&str> {
  content_type.split(';').skip(1).find_map(|parameter| {
    let (name, value) = parameter.split_once('=')?;
    name
      .trim()
      .eq_ignore_ascii_case("charset")
      .then(|| value.trim().trim_matches(['"', '\'']))
  })
}

fn html_block_tag(name: &[u8]) -> bool {
  [
    b"address".as_slice(),
    b"article".as_slice(),
    b"aside".as_slice(),
    b"blockquote".as_slice(),
    b"dd".as_slice(),
    b"p".as_slice(),
    b"div".as_slice(),
    b"dl".as_slice(),
    b"dt".as_slice(),
    b"figcaption".as_slice(),
    b"figure".as_slice(),
    b"footer".as_slice(),
    b"form".as_slice(),
    b"li".as_slice(),
    b"main".as_slice(),
    b"nav".as_slice(),
    b"ol".as_slice(),
    b"pre".as_slice(),
    b"section".as_slice(),
    b"table".as_slice(),
    b"tbody".as_slice(),
    b"td".as_slice(),
    b"tfoot".as_slice(),
    b"th".as_slice(),
    b"thead".as_slice(),
    b"tr".as_slice(),
    b"ul".as_slice(),
    b"h1".as_slice(),
    b"h2".as_slice(),
    b"h3".as_slice(),
    b"h4".as_slice(),
    b"h5".as_slice(),
    b"h6".as_slice(),
  ]
  .iter()
  .any(|tag| name.eq_ignore_ascii_case(tag))
}

fn html_hidden_tag(name: &[u8]) -> bool {
  [
    b"head".as_slice(),
    b"noscript".as_slice(),
    b"script".as_slice(),
    b"style".as_slice(),
    b"template".as_slice(),
    b"title".as_slice(),
  ]
  .iter()
  .any(|tag| name.eq_ignore_ascii_case(tag))
}

fn push_alt_chunk_paragraph(paragraphs: &mut Vec<String>, text: &mut String) {
  let normalized = text.split_whitespace().collect::<Vec<_>>().join(" ");
  text.clear();
  if !normalized.is_empty() {
    paragraphs.push(normalized);
  }
}

fn push_body_paragraph(blocks: &mut Vec<Block>, mut paragraph: Paragraph) {
  if let Some(Block::Paragraph(previous)) = blocks.last_mut()
    && (previous.format.hidden_separator || previous.format.deleted_separator)
  {
    if previous.format.hidden_separator {
      previous
        .format
        .outline_text_inlines
        .get_or_insert(previous.inlines.len());
    }
    previous.format.hidden_separator = paragraph.format.hidden_separator;
    previous.format.deleted_separator = paragraph.format.deleted_separator;
    previous
      .footnote_reference_ids
      .append(&mut paragraph.footnote_reference_ids);
    previous
      .endnote_reference_ids
      .append(&mut paragraph.endnote_reference_ids);
    previous.field_events.append(&mut paragraph.field_events);
    previous.inlines.append(&mut paragraph.inlines);
    return;
  }
  if let Some(frame) = paragraph.format.frame {
    paragraph.format.frame = None;
    if let Some(Block::Frame(previous)) = blocks.last_mut()
      && paragraph_belongs_to_frame(previous, frame, &paragraph)
    {
      previous.blocks.push(Block::paragraph(paragraph));
      return;
    }
    let fill_color = paragraph.format.shading;
    let borders = paragraph.format.borders;
    blocks.push(Block::Frame(FloatingFrame {
      blocks: vec![Block::paragraph(paragraph)],
      width_pt: frame.width_pt,
      height_pt: frame.height_pt,
      height_rule: frame.height_rule,
      placement: frame.placement,
      fill_color,
      borders,
    }));
    return;
  }
  blocks.push(Block::paragraph(paragraph));
}

fn prepend_out_of_place_breaks_to_paragraph(
  paragraph: &mut Paragraph,
  breaks: &[w::Break],
) -> bool {
  if breaks.is_empty() || paragraph.inlines.is_empty() {
    return false;
  }
  let style = paragraph
    .inlines
    .iter()
    .find_map(|inline| match inline {
      InlineItem::Text(run) => Some(run.style.clone()),
      _ => None,
    })
    .unwrap_or_else(|| paragraph.base_style.clone());
  let mut recovered = breaks
    .iter()
    .map(|br| match br.r#type {
      Some(w::BreakValues::Page) => InlineItem::PageBreak,
      Some(w::BreakValues::Column) => InlineItem::ColumnBreak,
      Some(w::BreakValues::TextWrapping) | None => InlineItem::Text(TextRun {
        text: "\n".to_string(),
        style: style.clone(),
        hyperlink_url: None,
        dynamic_field: None,
        style_ref_keys: Vec::new(),
        style_ref_text: None,
        style_ref_numbering_text: None,
        preserve_text_portion: false,
      }),
    })
    .collect::<Vec<_>>();
  recovered.append(&mut paragraph.inlines);
  paragraph.inlines = recovered;
  true
}

fn prepend_out_of_place_breaks_to_first_character_group(
  blocks: &mut [Block],
  breaks: &[w::Break],
) -> bool {
  if breaks.is_empty() {
    return false;
  }
  for block in blocks {
    let consumed = match block {
      Block::Paragraph(paragraph) => prepend_out_of_place_breaks_to_paragraph(paragraph, breaks),
      Block::Table(table) => table.rows.iter_mut().any(|row| {
        row.cells.iter_mut().any(|cell| {
          prepend_out_of_place_breaks_to_first_character_group(&mut cell.blocks, breaks)
        })
      }),
      Block::Frame(frame) => {
        prepend_out_of_place_breaks_to_first_character_group(&mut frame.blocks, breaks)
      }
    };
    if consumed {
      return true;
    }
  }
  false
}

fn promote_preceding_table_to_anchored_frame(blocks: &mut Vec<Block>, anchor: &Paragraph) {
  let Some(frame) = anchor.format.frame else {
    return;
  };
  let Some(Block::Table(_)) = blocks.last() else {
    return;
  };
  let Some(Block::Table(mut table)) = blocks.pop() else {
    unreachable!("last block was checked as a table");
  };
  flatten_promoted_table_cell_frames(&mut table);
  blocks.push(Block::Frame(FloatingFrame {
    blocks: vec![Block::Table(table)],
    width_pt: frame.width_pt,
    height_pt: frame.height_pt,
    height_rule: frame.height_rule,
    placement: frame.placement,
    fill_color: anchor.format.shading,
    borders: anchor.format.borders,
  }));
}

fn flatten_promoted_table_cell_frames(table: &mut Table) {
  for row in &mut table.rows {
    for cell in &mut row.cells {
      let mut flattened = Vec::new();
      for block in std::mem::take(&mut cell.blocks) {
        match block {
          Block::Frame(frame) => flattened.extend(frame.blocks),
          block => flattened.push(block),
        }
      }
      cell.blocks = flattened;
    }
  }
}

fn paragraph_belongs_to_frame(
  frame: &FloatingFrame,
  properties: ParagraphFrameProperties,
  paragraph: &Paragraph,
) -> bool {
  frame.width_pt == properties.width_pt
    && frame.height_pt == properties.height_pt
    && frame.height_rule == properties.height_rule
    && frame.placement == properties.placement
    && frame.fill_color == paragraph.format.shading
    && frame.borders == paragraph.format.borders
}

fn paragraph_mark_is_hidden(paragraph: &w::Paragraph) -> bool {
  paragraph
    .paragraph_properties
    .as_deref()
    .and_then(|properties| properties.paragraph_mark_run_properties.as_deref())
    .and_then(paragraph_mark_run_properties_vanish)
    .is_some_and(|vanish| vanish.val.is_none_or(|value| value.as_bool()))
}

fn paragraph_mark_joins_following(paragraph: &w::Paragraph) -> bool {
  // ECMA-376 Part 1 §17.13.5.15: deleting the paragraph mark removes
  // the delimiter and combines this paragraph's current contents with the
  // following paragraph. An inserted or move-source mark is a distinct
  // revision state and is intentionally not inferred from w:del.
  paragraph
    .paragraph_properties
    .as_deref()
    .and_then(|properties| properties.paragraph_mark_run_properties.as_deref())
    .is_some_and(|properties| properties.deleted.is_some())
}

fn paragraph_is_effectively_empty(paragraph: &Paragraph) -> bool {
  paragraph.list_label.is_none() && paragraph_body_is_effectively_empty(paragraph)
}

fn paragraph_body_is_effectively_empty(paragraph: &Paragraph) -> bool {
  paragraph.footnote_reference_ids.is_empty()
    && paragraph.endnote_reference_ids.is_empty()
    && paragraph.inlines.iter().all(|inline| match inline {
      InlineItem::Text(run) => run.text.trim().is_empty(),
      InlineItem::PositionalTab(_) => true,
      InlineItem::Ruby(_) | InlineItem::Image(_) | InlineItem::Shape(_) => false,
      InlineItem::BookmarkStart(_) => true,
      InlineItem::FormWidgetStart(_) | InlineItem::FormWidgetEnd(_) => true,
      InlineItem::DrawingGroupStart(_) | InlineItem::DrawingGroupEnd => true,
      InlineItem::LastRenderedPageBreak => true,
      InlineItem::PageBreak | InlineItem::ColumnBreak => false,
    })
}

fn empty_section_carrier_suppresses_numbering(
  paragraph: &w::Paragraph,
  section_break: SectionBreakKind,
) -> bool {
  let direct_properties = paragraph
    .paragraph_properties
    .as_deref()
    .map(ParagraphProps::Direct);
  let has_direct_numbering = direct_properties
    .as_ref()
    .is_some_and(|properties| properties.numbering_properties().is_some());
  let has_direct_indentation =
    NumberingFormatMergeContext::from_direct_properties(direct_properties).has_direct_indentation();

  has_direct_numbering
    || has_direct_indentation
    || paragraph_mark_is_deleted(paragraph)
    || section_break == SectionBreakKind::Continuous
}

fn paragraph_has_drop_cap_frame(paragraph: &Paragraph) -> bool {
  paragraph.format.frame.is_some_and(|frame| frame.drop_cap)
}

fn paragraph_drop_cap_text(paragraph: &Paragraph) -> Option<String> {
  let text = paragraph
    .inlines
    .iter()
    .filter_map(|inline| match inline {
      InlineItem::Text(run) => Some(run.text.as_str()),
      InlineItem::Ruby(ruby) => ruby.base.first().map(|run| run.text.as_str()),
      InlineItem::PositionalTab(_) => None,
      InlineItem::Image(_)
      | InlineItem::Shape(_)
      | InlineItem::BookmarkStart(_)
      | InlineItem::FormWidgetStart(_)
      | InlineItem::FormWidgetEnd(_)
      | InlineItem::DrawingGroupStart(_)
      | InlineItem::DrawingGroupEnd
      | InlineItem::LastRenderedPageBreak
      | InlineItem::PageBreak
      | InlineItem::ColumnBreak => None,
    })
    .collect::<String>();
  (!text.is_empty()).then_some(text)
}

fn prepend_drop_cap_text(paragraph: &mut Paragraph, text: String) {
  // saves DOCX framePr/dropCap paragraphs and applies them to the following
  // paragraph as DropCapFormat instead of converting them to text frames.
  if let Some(InlineItem::Text(run)) = paragraph
    .inlines
    .iter_mut()
    .find(|inline| matches!(inline, InlineItem::Text(_)))
  {
    run.text.insert_str(0, &text);
    return;
  }
  paragraph.inlines.insert(
    0,
    InlineItem::Text(TextRun {
      text,
      style: paragraph.base_style.clone(),
      hyperlink_url: None,
      dynamic_field: None,
      style_ref_keys: Vec::new(),
      style_ref_text: None,
      style_ref_numbering_text: None,
      preserve_text_portion: false,
    }),
  );
}

fn close_section(
  sections: &mut Vec<ImportedSection>,
  current_blocks: &mut Vec<Block>,
  section_properties: Option<w::SectionProperties>,
  previous_properties: &mut Option<w::SectionProperties>,
) {
  if let Some(rotation_deg) = section_properties
    .as_ref()
    .and_then(section_text_rotation_degrees)
  {
    rotate_blocks_text(current_blocks, rotation_deg);
  }
  let break_kind =
    normalized_section_break(section_properties.as_ref(), previous_properties.as_ref());
  let page = section_properties
    .as_ref()
    .map(page_setup)
    .unwrap_or_else(|| default_word_page_setup_with_size(PageSetup::default()));
  let columns = section_properties
    .as_ref()
    .map(section_columns)
    .unwrap_or_default();
  let title_page = section_properties
    .as_ref()
    .and_then(|section| section.title_page.as_ref())
    .map(|title_page| title_page.val.is_none_or(|value| value.as_bool()))
    .unwrap_or(false);

  sections.push(ImportedSection {
    break_kind,
    section_properties: section_properties.clone(),
    page,
    columns,
    title_page,
    header_blocks: Vec::new(),
    footer_blocks: Vec::new(),
    first_header_blocks: Vec::new(),
    first_footer_blocks: Vec::new(),
    even_header_blocks: Vec::new(),
    even_footer_blocks: Vec::new(),
    blocks: std::mem::take(current_blocks),
  });

  if let Some(section_properties) = section_properties {
    *previous_properties = Some(section_properties);
  }
}

fn default_section(blocks: Vec<Block>) -> ImportedSection {
  ImportedSection {
    break_kind: SectionBreakKind::NextPage,
    section_properties: None,
    page: default_word_page_setup_with_size(PageSetup::default()),
    columns: SectionColumns::default(),
    title_page: false,
    header_blocks: Vec::new(),
    footer_blocks: Vec::new(),
    first_header_blocks: Vec::new(),
    first_footer_blocks: Vec::new(),
    even_header_blocks: Vec::new(),
    even_footer_blocks: Vec::new(),
    blocks,
  }
}

fn normalized_section_break(
  section: Option<&w::SectionProperties>,
  previous: Option<&w::SectionProperties>,
) -> SectionBreakKind {
  let Some(section) = section else {
    return SectionBreakKind::NextPage;
  };

  let kind = section
    .section_type
    .as_ref()
    .map(|section_type| match section_type.val {
      w::SectionMarkValues::Continuous => SectionBreakKind::Continuous,
      w::SectionMarkValues::NextColumn => SectionBreakKind::NextColumn,
      w::SectionMarkValues::EvenPage => SectionBreakKind::EvenPage,
      w::SectionMarkValues::OddPage => SectionBreakKind::OddPage,
      w::SectionMarkValues::NextPage => SectionBreakKind::NextPage,
    })
    .unwrap_or(SectionBreakKind::NextPage);

  match kind {
    SectionBreakKind::Continuous
      if previous
        .map(|previous| section_orientation(previous) != section_orientation(section))
        .unwrap_or(false) =>
    {
      SectionBreakKind::NextPage
    }
    SectionBreakKind::NextColumn
      if previous
        .map(|previous| {
          section_column_count(section) <= 1
            || section_column_count(previous) != section_column_count(section)
        })
        .unwrap_or(true) =>
    {
      SectionBreakKind::NextPage
    }
    _ => kind,
  }
}

fn section_orientation(section: &w::SectionProperties) -> w::PageOrientationValues {
  section
    .page_size
    .as_ref()
    .and_then(|size| size.orient)
    .or_else(|| {
      let size = section.page_size.as_ref()?;
      Some(
        if size
          .width
          .as_ref()
          .and_then(twips_measure_to_twips)
          .unwrap_or(0.0)
          > size
            .height
            .as_ref()
            .and_then(twips_measure_to_twips)
            .unwrap_or(0.0)
        {
          w::PageOrientationValues::Landscape
        } else {
          w::PageOrientationValues::Portrait
        },
      )
    })
    .unwrap_or_default()
}

fn section_text_rotation_degrees(section: &w::SectionProperties) -> Option<f32> {
  let direction = section.text_direction.as_ref()?.val;
  match direction {
    w::TextDirectionValues::TopToBottomRightToLeft
    | w::TextDirectionValues::TopToBottomRightToLeft2010
    | w::TextDirectionValues::TopToBottomRightToLeftRotated
    | w::TextDirectionValues::TopToBottomRightToLeftRotated2010
    | w::TextDirectionValues::TopToBottomLeftToRightRotated
    | w::TextDirectionValues::TopToBottomLeftToRightRotated2010 => Some(-90.0),
    w::TextDirectionValues::BottomToTopLeftToRight
    | w::TextDirectionValues::BottomToTopLeftToRight2010 => Some(90.0),
    w::TextDirectionValues::LefToRightTopToBottom
    | w::TextDirectionValues::LeftToRightTopToBottom2010
    | w::TextDirectionValues::LefttoRightTopToBottomRotated
    | w::TextDirectionValues::LeftToRightTopToBottomRotated2010 => None,
  }
}

fn table_cell_text_rotation_degrees(properties: &w::TableCellProperties) -> Option<f32> {
  let direction = properties.text_direction.as_ref()?.val;
  match direction {
    w::TextDirectionValues::TopToBottomRightToLeft
    | w::TextDirectionValues::TopToBottomRightToLeft2010
    | w::TextDirectionValues::TopToBottomRightToLeftRotated
    | w::TextDirectionValues::TopToBottomRightToLeftRotated2010 => Some(90.0),
    w::TextDirectionValues::BottomToTopLeftToRight
    | w::TextDirectionValues::BottomToTopLeftToRight2010 => Some(-90.0),
    w::TextDirectionValues::LefToRightTopToBottom
    | w::TextDirectionValues::LeftToRightTopToBottom2010
    | w::TextDirectionValues::LefttoRightTopToBottomRotated
    | w::TextDirectionValues::LeftToRightTopToBottomRotated2010
    | w::TextDirectionValues::TopToBottomLeftToRightRotated
    | w::TextDirectionValues::TopToBottomLeftToRightRotated2010 => None,
  }
}

fn section_column_count(section: &w::SectionProperties) -> i16 {
  let Some(columns) = section.columns.as_ref() else {
    return 1;
  };
  if !columns.equal_width.is_none_or(|value| value.as_bool()) && !columns.column.is_empty() {
    return (columns.column.len() as i16).max(1);
  }
  columns.column_count.unwrap_or(1).max(1)
}

fn section_columns(section: &w::SectionProperties) -> SectionColumns {
  let Some(columns) = section.columns.as_ref() else {
    return SectionColumns::default();
  };
  let equal_width = columns.equal_width.is_none_or(|value| value.as_bool());
  let gap_pt = columns
    .space
    .as_ref()
    .and_then(twips_measure_to_points)
    .filter(|gap| gap.is_finite() && *gap >= 0.0)
    .unwrap_or(DEFAULT_SECTION_COLUMN_GAP_PT);
  if !equal_width && !columns.column.is_empty() {
    let explicit_widths_pt = columns
      .column
      .iter()
      .filter_map(|column| {
        column
          .width
          .as_ref()
          .and_then(signed_twips_measure_to_points)
          .filter(|width| width.is_finite() && *width > 0.0)
      })
      .collect::<Vec<_>>();
    if explicit_widths_pt.len() == columns.column.len() {
      let explicit_gaps_pt = columns
        .column
        .iter()
        .take(columns.column.len().saturating_sub(1))
        .map(|column| {
          column
            .space
            .as_ref()
            .and_then(signed_twips_measure_to_points)
            .filter(|gap| gap.is_finite() && *gap >= 0.0)
            .unwrap_or(gap_pt)
        })
        .collect::<Vec<_>>();
      let explicit_count = explicit_widths_pt.len().min(45);
      let mut widths = [0.0; 45];
      let mut gaps = [0.0; 44];
      for (index, width) in explicit_widths_pt.iter().copied().take(45).enumerate() {
        widths[index] = width;
      }
      for (index, gap) in explicit_gaps_pt.iter().copied().take(44).enumerate() {
        gaps[index] = gap;
      }
      return SectionColumns {
        count: explicit_count.max(1),
        gap_pt,
        separator: columns.separator.is_some_and(|value| value.as_bool()),
        unbalanced: false,
        explicit_count,
        explicit_widths_pt: widths,
        explicit_gaps_pt: gaps,
      };
    }
  }

  let count = columns
    .column_count
    .map(|count| count.max(1) as usize)
    .unwrap_or(1);
  SectionColumns {
    count,
    gap_pt,
    separator: columns.separator.is_some_and(|value| value.as_bool()),
    unbalanced: false,
    explicit_count: 0,
    explicit_widths_pt: [0.0; 45],
    explicit_gaps_pt: [0.0; 44],
  }
}

struct SdtBlockControls<'a> {
  custom_xml_bindings: &'a CustomXmlBindings,
  form_widget_ids: &'a mut FormWidgetIdAllocator,
  in_header_footer: bool,
}

fn sdt_block_blocks(
  sdt: &w::SdtBlock,
  styles: &StylesCatalog,
  numbering: &mut NumberingCatalog,
  images: &ImageCatalog,
  hyperlinks: &HyperlinkCatalog,
  controls: SdtBlockControls<'_>,
) -> Vec<Block> {
  sdt_block_blocks_with_base(sdt, styles, numbering, images, hyperlinks, controls, None)
}

fn sdt_block_blocks_with_base(
  sdt: &w::SdtBlock,
  styles: &StylesCatalog,
  numbering: &mut NumberingCatalog,
  images: &ImageCatalog,
  hyperlinks: &HyperlinkCatalog,
  controls: SdtBlockControls<'_>,
  paragraph_base_style: Option<&TextStyle>,
) -> Vec<Block> {
  let SdtBlockControls {
    custom_xml_bindings,
    form_widget_ids,
    in_header_footer,
  } = controls;
  let Some(content) = sdt.sdt_content_block.as_ref() else {
    return Vec::new();
  };

  let bound_value = sdt
    .sdt_properties
    .as_ref()
    .and_then(|properties| sdt_bound_replacement(custom_xml_bindings, properties));
  let mut blocks = content
    .sdt_content_block_choice
    .iter()
    .filter_map(|choice| match choice {
      w::SdtContentBlockChoice::Paragraph(paragraph) => {
        let mut model = if let Some(base_style) = paragraph_base_style {
          paragraph_model_with_base(
            paragraph.as_ref(),
            styles,
            numbering,
            images,
            hyperlinks,
            form_widget_ids,
            ParagraphImportBase {
              run_style: base_style.clone(),
              custom_xml_bindings: Some(custom_xml_bindings),
              ..Default::default()
            },
          )
        } else {
          paragraph_model(
            paragraph.as_ref(),
            styles,
            numbering,
            images,
            hyperlinks,
            custom_xml_bindings,
            form_widget_ids,
          )
        };
        if !in_header_footer {
          apply_recovered_body_paragraph_defaults(paragraph, styles, &mut model);
        }
        Some(vec![Block::paragraph(model)])
      }
      w::SdtContentBlockChoice::Table(table) => Some(vec![Block::Table(table_model(
        table.as_ref(),
        &mut TableModelEnv {
          styles,
          numbering,
          images,
          hyperlinks,
          custom_xml_bindings,
          form_widget_ids,
        },
        TableModelContext {
          nested_table_level: 1,
          in_header_footer,
        },
      ))]),
      w::SdtContentBlockChoice::SdtBlock(sdt) => Some(sdt_block_blocks_with_base(
        sdt.as_ref(),
        styles,
        numbering,
        images,
        hyperlinks,
        SdtBlockControls {
          custom_xml_bindings,
          form_widget_ids: &mut *form_widget_ids,
          in_header_footer,
        },
        paragraph_base_style,
      )),
      _ => None,
    })
    .flatten()
    .collect::<Vec<_>>();
  if let Some(value) = bound_value {
    replace_sdt_block_text(&mut blocks, value);
  }
  blocks
}

fn replace_sdt_block_text(blocks: &mut [Block], value: String) {
  fn replace(blocks: &mut [Block], value: &mut Option<String>) {
    for block in blocks {
      match block {
        Block::Paragraph(paragraph) => {
          for inline in &mut paragraph.inlines {
            match inline {
              InlineItem::Text(run) => {
                run.text = value.take().unwrap_or_default();
              }
              InlineItem::Shape(shape) => replace(&mut shape.text_box_blocks, value),
              _ => {}
            }
          }
        }
        Block::Table(table) => {
          for row in &mut table.rows {
            for cell in &mut row.cells {
              replace(&mut cell.blocks, value);
            }
          }
        }
        Block::Frame(frame) => replace(&mut frame.blocks, value),
      }
    }
  }

  replace(blocks, &mut Some(value));
}

fn apply_recovered_body_paragraph_defaults(
  _paragraph: &w::Paragraph,
  styles: &StylesCatalog,
  model: &mut Paragraph,
) {
  if !styles.uses_office_recovered_paragraph_defaults() {
    return;
  }
  if !paragraph_has_recoverable_main_story_text(model)
    && model
      .inlines
      .iter()
      .any(|inline| matches!(inline, InlineItem::Image(_) | InlineItem::Shape(_)))
  {
    return;
  }

  // Word repairs main-story paragraphs from the application Normal style.
  // Header/footer parts and text boxes have their own built-in style contexts,
  // so this recovery must not leak into those stories. A referenced style and
  // an empty paragraph still inherit these application defaults; resolved
  // style/direct spacing remains authoritative when it supplies a value.
  if !model.format.spacing_after_set {
    model.format.spacing_after_pt = OFFICE_RECOVERED_PARAGRAPH_AFTER_PT;
    model.format.spacing_after_set = true;
  }
  if model.format.line_height_pt.is_none() {
    model.format.line_height_pt = Some(OFFICE_RECOVERED_LINE_HEIGHT_MULTIPLE);
    model.format.line_height_rule = LineHeightRule::Auto;
  }
}

fn apply_recovered_table_cell_paragraph_defaults(
  paragraph: &w::Paragraph,
  styles: &StylesCatalog,
  model: &mut Paragraph,
) {
  let inherited_line_height = model.format.line_height_pt.is_some();
  apply_recovered_body_paragraph_defaults(paragraph, styles, model);
  if styles.simplified_chinese_ui && !inherited_line_height && model.format.line_height_pt.is_some()
  {
    // The Simplified Chinese application Table Normal context differs from
    // the body Normal context. Microsoft's fixed PDF emits consecutive blank
    // DengXian lines 17.28pt apart, matching a 360/240 auto line multiple.
    model.format.line_height_pt = Some(OFFICE_RECOVERED_ZH_HANS_TABLE_LINE_HEIGHT_MULTIPLE);
  }
}

fn paragraph_has_recoverable_main_story_text(paragraph: &Paragraph) -> bool {
  paragraph.list_label.is_some()
    || paragraph
      .inlines
      .iter()
      .any(|inline| matches!(inline, InlineItem::Text(run) if !run.text.is_empty()))
}

fn header_blocks(
  package: &mut WordprocessingDocument,
  main: &MainDocumentPart,
  section: &w::SectionProperties,
  styles: &StylesCatalog,
  header_type: w::HeaderFooterValues,
  custom_xml_bindings: &CustomXmlBindings,
  form_widget_ids: &mut FormWidgetIdAllocator,
) -> Option<Vec<Block>> {
  let relationship_id =
    section
      .section_properties_choice
      .iter()
      .find_map(|choice| match choice {
        w::SectionPropertiesChoice::HeaderReference(reference)
          if reference.r#type == header_type =>
        {
          Some(reference.id.as_str())
        }
        _ => None,
      })?;
  let header_part = main
    .header_parts(package)
    .find(|part| main.get_id_of_part(package, part) == Some(relationship_id))?;
  let images = ImageCatalog::load_from_header(package, &header_part);
  let hyperlinks = HyperlinkCatalog::load(package, &header_part);
  let header = header_part.root_element(package).ok()?;
  let mut numbering = NumberingCatalog::default();
  Some(
    header
      .header_choice
      .iter()
      .filter_map(|choice| match choice {
        w::HeaderChoice::Paragraph(paragraph) => {
          let model = paragraph_model(
            paragraph,
            styles,
            &mut numbering,
            &images,
            &hyperlinks,
            custom_xml_bindings,
            form_widget_ids,
          );
          Some(vec![Block::paragraph(model)])
        }
        w::HeaderChoice::Table(table) => Some(vec![Block::Table(table_model(
          table,
          &mut TableModelEnv {
            styles,
            numbering: &mut numbering,
            images: &images,
            hyperlinks: &hyperlinks,
            custom_xml_bindings,
            form_widget_ids,
          },
          TableModelContext {
            nested_table_level: 1,
            in_header_footer: true,
          },
        ))]),
        w::HeaderChoice::SdtBlock(sdt) => Some(sdt_block_blocks(
          sdt,
          styles,
          &mut numbering,
          &images,
          &hyperlinks,
          SdtBlockControls {
            custom_xml_bindings,
            form_widget_ids,
            in_header_footer: true,
          },
        )),
        _ => None,
      })
      .flatten()
      .collect(),
  )
}

fn referenced_header_blocks(
  package: &mut WordprocessingDocument,
  main: &MainDocumentPart,
  section: &w::SectionProperties,
  styles: &StylesCatalog,
  header_type: w::HeaderFooterValues,
  custom_xml_bindings: &CustomXmlBindings,
  form_widget_ids: &mut FormWidgetIdAllocator,
) -> Option<Vec<Block>> {
  header_blocks(
    package,
    main,
    section,
    styles,
    header_type,
    custom_xml_bindings,
    form_widget_ids,
  )
}

fn footer_blocks(
  package: &mut WordprocessingDocument,
  main: &MainDocumentPart,
  section: &w::SectionProperties,
  styles: &StylesCatalog,
  footer_type: w::HeaderFooterValues,
  custom_xml_bindings: &CustomXmlBindings,
  form_widget_ids: &mut FormWidgetIdAllocator,
) -> Option<Vec<Block>> {
  let relationship_id =
    section
      .section_properties_choice
      .iter()
      .find_map(|choice| match choice {
        w::SectionPropertiesChoice::FooterReference(reference)
          if reference.r#type == footer_type =>
        {
          Some(reference.id.as_str())
        }
        _ => None,
      })?;
  let footer_part = main
    .footer_parts(package)
    .find(|part| main.get_id_of_part(package, part) == Some(relationship_id))?;
  let images = ImageCatalog::load_from_footer(package, &footer_part);
  let hyperlinks = HyperlinkCatalog::load(package, &footer_part);
  let footer = footer_part.root_element(package).ok()?;
  let mut numbering = NumberingCatalog::default();
  Some(
    footer
      .footer_choice
      .iter()
      .filter_map(|choice| match choice {
        w::FooterChoice::Paragraph(paragraph) => {
          let model = paragraph_model(
            paragraph,
            styles,
            &mut numbering,
            &images,
            &hyperlinks,
            custom_xml_bindings,
            form_widget_ids,
          );
          Some(vec![Block::paragraph(model)])
        }
        w::FooterChoice::Table(table) => Some(vec![Block::Table(table_model(
          table,
          &mut TableModelEnv {
            styles,
            numbering: &mut numbering,
            images: &images,
            hyperlinks: &hyperlinks,
            custom_xml_bindings,
            form_widget_ids,
          },
          TableModelContext {
            nested_table_level: 1,
            in_header_footer: true,
          },
        ))]),
        w::FooterChoice::SdtBlock(sdt) => Some(sdt_block_blocks(
          sdt,
          styles,
          &mut numbering,
          &images,
          &hyperlinks,
          SdtBlockControls {
            custom_xml_bindings,
            form_widget_ids,
            in_header_footer: true,
          },
        )),
        _ => None,
      })
      .flatten()
      .collect(),
  )
}

fn referenced_footer_blocks(
  package: &mut WordprocessingDocument,
  main: &MainDocumentPart,
  section: &w::SectionProperties,
  styles: &StylesCatalog,
  footer_type: w::HeaderFooterValues,
  custom_xml_bindings: &CustomXmlBindings,
  form_widget_ids: &mut FormWidgetIdAllocator,
) -> Option<Vec<Block>> {
  footer_blocks(
    package,
    main,
    section,
    styles,
    footer_type,
    custom_xml_bindings,
    form_widget_ids,
  )
}

#[derive(Clone, Copy)]
enum NoteKind {
  Footnote,
  Endnote,
}

impl NoteNumberingSpec {
  fn default_for(kind: NoteKind) -> Self {
    Self {
      format: match kind {
        NoteKind::Footnote => w::NumberFormatValues::Decimal,
        NoteKind::Endnote => w::NumberFormatValues::LowerRoman,
      },
      start: 1,
      restart: w::RestartNumberValues::Continuous,
    }
  }

  fn formatted(self, kind: NoteKind, value: i32) -> String {
    let format = if matches!(self.format, w::NumberFormatValues::None) {
      Self::default_for(kind).format
    } else {
      self.format
    };
    format_numbering_value(value, format, false)
  }
}

type NoteReferenceLabels = (
  HashMap<i64, String>,
  HashMap<i64, String>,
  Vec<NoteNumberingSpec>,
  Vec<NoteNumberingSpec>,
);

fn note_reference_labels(
  package: &mut WordprocessingDocument,
  main: &MainDocumentPart,
  sections: &[ImportedSection],
) -> NoteReferenceLabels {
  let settings = main
    .document_settings_part(package)
    .and_then(|part| part.root_element(package).ok());
  let footnote_default = settings
    .as_ref()
    .and_then(|settings| settings.footnote_document_wide_properties.as_deref())
    .map_or_else(
      || NoteNumberingSpec::default_for(NoteKind::Footnote),
      footnote_document_numbering_spec,
    );
  let endnote_default = settings
    .as_ref()
    .and_then(|settings| settings.endnote_document_wide_properties.as_deref())
    .map_or_else(
      || NoteNumberingSpec::default_for(NoteKind::Endnote),
      endnote_document_numbering_spec,
    );

  let footnote_numbering = sections
    .iter()
    .map(|section| section_note_numbering_spec(section, NoteKind::Footnote, footnote_default))
    .collect::<Vec<_>>();
  let endnote_numbering = sections
    .iter()
    .map(|section| section_note_numbering_spec(section, NoteKind::Endnote, endnote_default))
    .collect::<Vec<_>>();
  let footnote_labels = note_labels_for_sections(sections, NoteKind::Footnote, &footnote_numbering);
  let endnote_labels = note_labels_for_sections(sections, NoteKind::Endnote, &endnote_numbering);

  (
    footnote_labels,
    endnote_labels,
    footnote_numbering,
    endnote_numbering,
  )
}

fn footnote_document_numbering_spec(
  properties: &w::FootnoteDocumentWideProperties,
) -> NoteNumberingSpec {
  let mut spec = NoteNumberingSpec::default_for(NoteKind::Footnote);
  apply_note_numbering_values(
    &mut spec,
    properties.numbering_format.as_ref(),
    properties.numbering_start.as_ref(),
    properties.numbering_restart.as_ref(),
  );
  spec
}

fn endnote_document_numbering_spec(
  properties: &w::EndnoteDocumentWideProperties,
) -> NoteNumberingSpec {
  let mut spec = NoteNumberingSpec::default_for(NoteKind::Endnote);
  apply_note_numbering_values(
    &mut spec,
    properties.numbering_format.as_ref(),
    properties.numbering_start.as_ref(),
    properties.numbering_restart.as_ref(),
  );
  spec
}

fn apply_note_numbering_values(
  spec: &mut NoteNumberingSpec,
  format: Option<&w::NumberingFormat>,
  start: Option<&w::NumberingStart>,
  restart: Option<&w::NumberingRestart>,
) {
  if let Some(format) = format {
    spec.format = format.val;
  }
  if let Some(start) = start {
    spec.start = i32::from(start.val);
  }
  if let Some(restart) = restart {
    spec.restart = restart.val;
  }
}

fn section_note_numbering_spec(
  section: &ImportedSection,
  kind: NoteKind,
  mut spec: NoteNumberingSpec,
) -> NoteNumberingSpec {
  let Some(properties) = section.section_properties.as_ref() else {
    return spec;
  };
  match kind {
    NoteKind::Footnote => {
      if let Some(properties) = properties.footnote_properties.as_deref() {
        apply_note_numbering_values(
          &mut spec,
          properties.numbering_format.as_ref(),
          properties.numbering_start.as_ref(),
          properties.numbering_restart.as_ref(),
        );
      }
    }
    NoteKind::Endnote => {
      if let Some(properties) = properties.endnote_properties.as_deref() {
        apply_note_numbering_values(
          &mut spec,
          properties.numbering_format.as_ref(),
          properties.numbering_start.as_ref(),
          properties.numbering_restart.as_ref(),
        );
      }
    }
  }
  spec
}

fn note_labels_for_sections(
  sections: &[ImportedSection],
  kind: NoteKind,
  specs: &[NoteNumberingSpec],
) -> HashMap<i64, String> {
  let mut labels = HashMap::new();
  let mut value = specs
    .first()
    .copied()
    .unwrap_or_else(|| NoteNumberingSpec::default_for(kind))
    .start;
  for (section_index, section) in sections.iter().enumerate() {
    let spec = specs
      .get(section_index)
      .copied()
      .unwrap_or_else(|| NoteNumberingSpec::default_for(kind));
    if section_index == 0 || matches!(spec.restart, w::RestartNumberValues::EachSection) {
      value = spec.start;
    }
    let mut ids = Vec::new();
    collect_note_reference_ids_from_blocks(&section.blocks, kind, &mut ids);
    for id in ids {
      if labels.contains_key(&id) {
        continue;
      }
      labels.insert(id, spec.formatted(kind, value));
      value = value.saturating_add(1);
    }
  }
  labels
}

fn collect_note_reference_ids_from_blocks(blocks: &[Block], kind: NoteKind, ids: &mut Vec<i64>) {
  for block in blocks {
    match block {
      Block::Paragraph(paragraph) => {
        ids.extend(match kind {
          NoteKind::Footnote => &paragraph.footnote_reference_ids,
          NoteKind::Endnote => &paragraph.endnote_reference_ids,
        });
        for inline in &paragraph.inlines {
          if let InlineItem::Shape(shape) = inline {
            collect_note_reference_ids_from_blocks(&shape.text_box_blocks, kind, ids);
          }
        }
      }
      Block::Table(table) => {
        for row in &table.rows {
          for cell in &row.cells {
            collect_note_reference_ids_from_blocks(&cell.blocks, kind, ids);
          }
        }
      }
      Block::Frame(frame) => collect_note_reference_ids_from_blocks(&frame.blocks, kind, ids),
    }
  }
}

fn apply_note_reference_labels(
  sections: &mut [ImportedSection],
  footnote_labels: &HashMap<i64, String>,
  endnote_labels: &HashMap<i64, String>,
) {
  for section in sections {
    apply_note_reference_labels_to_blocks(&mut section.blocks, footnote_labels, endnote_labels);
    apply_note_reference_labels_to_blocks(
      &mut section.header_blocks,
      footnote_labels,
      endnote_labels,
    );
    apply_note_reference_labels_to_blocks(
      &mut section.footer_blocks,
      footnote_labels,
      endnote_labels,
    );
    apply_note_reference_labels_to_blocks(
      &mut section.first_header_blocks,
      footnote_labels,
      endnote_labels,
    );
    apply_note_reference_labels_to_blocks(
      &mut section.first_footer_blocks,
      footnote_labels,
      endnote_labels,
    );
    apply_note_reference_labels_to_blocks(
      &mut section.even_header_blocks,
      footnote_labels,
      endnote_labels,
    );
    apply_note_reference_labels_to_blocks(
      &mut section.even_footer_blocks,
      footnote_labels,
      endnote_labels,
    );
  }
}

fn apply_note_reference_labels_to_blocks(
  blocks: &mut [Block],
  footnote_labels: &HashMap<i64, String>,
  endnote_labels: &HashMap<i64, String>,
) {
  for block in blocks {
    match block {
      Block::Paragraph(paragraph) => {
        for inline in &mut paragraph.inlines {
          match inline {
            InlineItem::Text(run) => {
              let Some(url) = run.hyperlink_url.as_deref() else {
                continue;
              };
              if let Some(id) = url
                .strip_prefix("ooxmlsdk-pdf:footnote-reference:")
                .and_then(|id| id.parse::<i64>().ok())
                && let Some(label) = footnote_labels.get(&id)
              {
                run.text.clone_from(label);
              } else if let Some(id) = url
                .strip_prefix("ooxmlsdk-pdf:endnote-reference:")
                .and_then(|id| id.parse::<i64>().ok())
                && let Some(label) = endnote_labels.get(&id)
              {
                run.text.clone_from(label);
              }
            }
            InlineItem::Shape(shape) => apply_note_reference_labels_to_blocks(
              &mut shape.text_box_blocks,
              footnote_labels,
              endnote_labels,
            ),
            _ => {}
          }
        }
      }
      Block::Table(table) => {
        for row in &mut table.rows {
          for cell in &mut row.cells {
            apply_note_reference_labels_to_blocks(
              &mut cell.blocks,
              footnote_labels,
              endnote_labels,
            );
          }
        }
      }
      Block::Frame(frame) => {
        apply_note_reference_labels_to_blocks(&mut frame.blocks, footnote_labels, endnote_labels)
      }
    }
  }
}

fn footnotes(
  package: &mut WordprocessingDocument,
  main: &MainDocumentPart,
  styles: &StylesCatalog,
  custom_xml_bindings: &CustomXmlBindings,
  form_widget_ids: &mut FormWidgetIdAllocator,
  labels: &HashMap<i64, String>,
) -> Result<BTreeMap<i64, Vec<Block>>> {
  let Some(part) = main.footnotes_part(package) else {
    return Ok(BTreeMap::new());
  };
  let images = ImageCatalog::load_from_footnotes(package, &part);
  let hyperlinks = HyperlinkCatalog::load(package, &part);
  let footnotes = part.root_element(package)?;
  let mut numbering = NumberingCatalog::default();
  let mut context = NoteImportContext {
    styles,
    numbering: &mut numbering,
    images: &images,
    hyperlinks: &hyperlinks,
    custom_xml_bindings,
    form_widget_ids,
  };
  let mut notes = BTreeMap::new();

  for footnote in &footnotes.footnote {
    if !normal_note_type(footnote.r#type) {
      continue;
    }
    let mut blocks = Vec::new();
    append_note_blocks(
      &mut blocks,
      NoteLabel::new(
        labels
          .get(&footnote.id)
          .map_or_else(|| footnote.id.to_string(), Clone::clone),
        Some(note_backlink_url("footnote", footnote.id)),
      ),
      footnote
        .footnote_choice
        .iter()
        .filter_map(|choice| match choice {
          w::FootnoteChoice::Paragraph(paragraph) => {
            Some(NoteBlockChoice::Paragraph(paragraph.as_ref()))
          }
          w::FootnoteChoice::Table(table) => Some(NoteBlockChoice::Table(table.as_ref())),
          _ => None,
        }),
      &mut context,
    );
    notes.insert(footnote.id, blocks);
  }

  Ok(notes)
}

fn endnotes(
  package: &mut WordprocessingDocument,
  main: &MainDocumentPart,
  styles: &StylesCatalog,
  custom_xml_bindings: &CustomXmlBindings,
  form_widget_ids: &mut FormWidgetIdAllocator,
  labels: &HashMap<i64, String>,
) -> Result<BTreeMap<i64, Vec<Block>>> {
  let Some(part) = main.endnotes_part(package) else {
    return Ok(BTreeMap::new());
  };
  let images = ImageCatalog::load_from_endnotes(package, &part);
  let hyperlinks = HyperlinkCatalog::load(package, &part);
  let endnotes = part.root_element(package)?;
  let mut numbering = NumberingCatalog::default();
  let mut context = NoteImportContext {
    styles,
    numbering: &mut numbering,
    images: &images,
    hyperlinks: &hyperlinks,
    custom_xml_bindings,
    form_widget_ids,
  };
  let mut notes = BTreeMap::new();

  for endnote in &endnotes.endnote {
    if !normal_note_type(endnote.r#type) {
      continue;
    }
    let mut blocks = Vec::new();
    append_note_blocks(
      &mut blocks,
      NoteLabel::new(
        labels
          .get(&endnote.id)
          .map_or_else(|| endnote.id.to_string(), Clone::clone),
        Some(note_backlink_url("endnote", endnote.id)),
      ),
      endnote
        .endnote_choice
        .iter()
        .filter_map(|choice| match choice {
          w::EndnoteChoice::Paragraph(paragraph) => {
            Some(NoteBlockChoice::Paragraph(paragraph.as_ref()))
          }
          w::EndnoteChoice::Table(table) => Some(NoteBlockChoice::Table(table.as_ref())),
          _ => None,
        }),
      &mut context,
    );
    notes.insert(endnote.id, blocks);
  }

  Ok(notes)
}

fn flatten_note_blocks(notes: &BTreeMap<i64, Vec<Block>>) -> Vec<Block> {
  notes
    .values()
    .flat_map(|blocks| blocks.iter().cloned())
    .collect()
}

fn normal_note_type(r#type: Option<w::FootnoteEndnoteValues>) -> bool {
  matches!(r#type, None | Some(w::FootnoteEndnoteValues::Normal))
}

#[derive(Clone, Debug)]
struct NoteLabel {
  text: String,
  hyperlink_url: Option<String>,
}

impl NoteLabel {
  fn new(text: impl Into<String>, hyperlink_url: Option<String>) -> Self {
    Self {
      text: text.into(),
      hyperlink_url,
    }
  }
}

struct NoteImportContext<'a> {
  styles: &'a StylesCatalog,
  numbering: &'a mut NumberingCatalog,
  images: &'a ImageCatalog,
  hyperlinks: &'a HyperlinkCatalog,
  custom_xml_bindings: &'a CustomXmlBindings,
  form_widget_ids: &'a mut FormWidgetIdAllocator,
}

enum NoteBlockChoice<'a> {
  Paragraph(&'a w::Paragraph),
  Table(&'a w::Table),
}

fn append_note_blocks<'a>(
  blocks: &mut Vec<Block>,
  label: NoteLabel,
  choices: impl Iterator<Item = NoteBlockChoice<'a>>,
  context: &mut NoteImportContext<'_>,
) {
  let mut is_first_paragraph = true;
  for choice in choices {
    match choice {
      NoteBlockChoice::Paragraph(paragraph) => {
        let mut model = paragraph_model(
          paragraph,
          context.styles,
          context.numbering,
          context.images,
          context.hyperlinks,
          context.custom_xml_bindings,
          context.form_widget_ids,
        );
        if is_first_paragraph {
          let marker_style = note_marker_run_style(paragraph, &model.base_style, context.styles);
          prepend_note_marker(&mut model, &label, marker_style);
          is_first_paragraph = false;
        }
        preserve_note_text_portions(&mut model);
        blocks.push(Block::paragraph(model));
      }
      NoteBlockChoice::Table(table) => {
        let mut block = Block::Table(table_model(
          table,
          &mut TableModelEnv {
            styles: context.styles,
            numbering: context.numbering,
            images: context.images,
            hyperlinks: context.hyperlinks,
            custom_xml_bindings: context.custom_xml_bindings,
            form_widget_ids: context.form_widget_ids,
          },
          TableModelContext {
            nested_table_level: 1,
            in_header_footer: false,
          },
        ));
        preserve_note_text_portions_in_block(&mut block);
        blocks.push(block);
      }
    }
  }
}

fn note_marker_run_style(
  paragraph: &w::Paragraph,
  base_style: &TextStyle,
  styles: &StylesCatalog,
) -> Option<TextStyle> {
  paragraph.paragraph_choice.iter().find_map(|choice| {
    let w::ParagraphChoice::WRun(run) = choice else {
      return None;
    };
    run
      .run_choice
      .iter()
      .any(|choice| {
        matches!(
          choice,
          w::RunChoice::FootnoteReferenceMark | w::RunChoice::EndnoteReferenceMark
        )
      })
      .then(|| properties::run_style(run.run_properties.as_deref(), base_style.clone(), styles))
  })
}

fn prepend_note_marker(
  paragraph: &mut Paragraph,
  label: &NoteLabel,
  marker_style: Option<TextStyle>,
) {
  let base_style = marker_style.unwrap_or_else(|| {
    paragraph
      .inlines
      .iter()
      .find_map(|inline| match inline {
        InlineItem::Text(run) => Some(run.style.clone()),
        _ => None,
      })
      .unwrap_or_default()
  });
  paragraph.inlines.insert(
    0,
    InlineItem::Text(TextRun {
      text: label.text.clone(),
      style: note_reference_style(&base_style),
      hyperlink_url: label.hyperlink_url.clone(),
      dynamic_field: None,
      style_ref_keys: Vec::new(),
      style_ref_text: None,
      style_ref_numbering_text: None,
      preserve_text_portion: false,
    }),
  );
}

fn preserve_note_text_portions(paragraph: &mut Paragraph) {
  for inline in &mut paragraph.inlines {
    if let InlineItem::Text(run) = inline {
      run.preserve_text_portion = true;
    }
  }
}

fn preserve_note_text_portions_in_block(block: &mut Block) {
  match block {
    Block::Paragraph(paragraph) => preserve_note_text_portions(paragraph),
    Block::Table(table) => {
      for row in &mut table.rows {
        for cell in &mut row.cells {
          for block in &mut cell.blocks {
            preserve_note_text_portions_in_block(block);
          }
        }
      }
    }
    Block::Frame(frame) => {
      for block in &mut frame.blocks {
        preserve_note_text_portions_in_block(block);
      }
    }
  }
}

fn table_model(
  table: &w::Table,
  env: &mut TableModelEnv<'_>,
  model_context: TableModelContext,
) -> Table {
  let properties = table.table_properties.as_deref();
  let right_to_left = properties
    .and_then(|properties| properties.bi_di_visual.as_ref())
    .is_some_and(|bidi| on_off_only_value(bidi.val));
  let table_style_id = properties
    .and_then(|properties| properties.table_style.as_ref())
    .map(|style| style.val.as_str());
  let table_style = env.styles.table_style(table_style_id);
  let table_look = properties
    .and_then(|properties| properties.table_look.as_ref())
    .map(table_look_model)
    .unwrap_or_default();
  let style_cell_margins = table_style.cell_margins.unwrap_or_default();
  let direct_cell_margins =
    properties.is_some_and(|properties| properties.table_cell_margin_default.is_some());
  let cell_margins = properties
    .and_then(|properties| properties.table_cell_margin_default.as_deref())
    .map(|margins| table_cell_margin_default_with_base(margins, style_cell_margins))
    .unwrap_or(style_cell_margins);
  let table_shading = properties
    .and_then(|properties| properties.shading.as_ref())
    .map(shading_fill)
    .or(table_style.table_shading)
    .flatten();
  let table_borders = properties
    .and_then(|properties| properties.table_borders.as_deref())
    .map(|borders| direct_table_borders_model(table_style.table_borders, borders))
    .or(table_style.table_borders);
  let rows = table
    .table_choice2
    .iter()
    .filter_map(|choice| match choice {
      w::TableChoice2::TableRow(row) if !table_row_is_deleted(row) => Some(row.as_ref()),
      _ => None,
    })
    .collect::<Vec<_>>();
  let row_count = rows.len();
  let explicit_no_repeat_header = rows.first().is_some_and(|row| {
    direct_table_row_style(row.table_row_properties.as_deref()).repeat_header == Some(false)
  });
  let rows = {
    let mut context = TableImportContext {
      styles: env.styles,
      numbering: env.numbering,
      images: env.images,
      hyperlinks: env.hyperlinks,
      custom_xml_bindings: env.custom_xml_bindings,
      form_widget_ids: env.form_widget_ids,
      cell_margins,
      direct_cell_margins,
      table_shading,
      table_borders,
      table_style: &table_style,
      table_look,
      row_count,
      nested_table_level: model_context.nested_table_level,
      in_header_footer: model_context.in_header_footer,
    };
    rows
      .iter()
      .enumerate()
      .map(|(row_index, row)| table_row_model(row, &mut context, row_index))
      .collect::<Vec<_>>()
  };
  let starts_after_last_rendered_page_break = table_starts_after_last_rendered_page_break(&rows);
  let placement = properties
    .and_then(|properties| properties.table_position_properties.as_ref())
    .map(table_position_placement);
  // ECMA-376 Part 4 §14.8.3.10 makes this a document-level compatibility
  // boundary: floating tables split by default, but an enabled
  // doNotBreakWrappedTables keeps the whole fly on one page. LibreOffice
  // SwFlyFrame::IsFlySplitAllowed() applies the same setting before any
  // master/follow table is created.
  let split_allowed =
    placement.is_some() && !env.styles.import_settings.do_not_break_wrapped_tables;
  let following_text_flow = placement.is_some()
    && (model_context.nested_table_level >= 2 || model_context.in_header_footer)
    && !(model_context.in_header_footer
      && placement
        .is_some_and(|placement| matches!(placement.vertical_anchor, FrameVerticalAnchor::Page)));
  let mut model = Table {
    column_widths_pt: table
      .table_grid
      .as_deref()
      .into_iter()
      .flat_map(|grid| &grid.grid_column)
      .filter_map(|column| column.width.as_ref().and_then(twips_measure_to_points))
      .collect(),
    preferred_width_pt: properties
      .and_then(|properties| properties.table_width.as_ref())
      .and_then(table_width_to_points),
    preferred_width_pct: properties
      .and_then(|properties| properties.table_width.as_ref())
      .and_then(table_width_to_percent),
    layout: properties
      .and_then(|properties| properties.table_layout.as_ref())
      .map(table_layout_mode)
      .or(table_style.layout)
      .unwrap_or_default(),
    indent_left_pt: properties
      .and_then(|properties| properties.table_indentation.as_ref())
      .and_then(table_indentation_to_points)
      .or(table_style.indent_left_pt)
      .unwrap_or(0.0),
    alignment: properties
      .and_then(|properties| properties.table_justification.as_ref())
      .map(table_alignment)
      .or(table_style.alignment)
      .unwrap_or_default(),
    right_to_left,
    align_leading_cell_content: should_align_leading_cell_content(
      model_context,
      env.styles.import_settings.compatibility_mode,
      env.styles.has_styles_part,
    ),
    placement,
    allow_overlap: table_allows_overlap(properties),
    split_allowed,
    following_text_flow,
    explicit_no_repeat_header,
    starts_after_last_rendered_page_break,
    borders: table_borders,
    cell_spacing_pt: properties
      .and_then(|properties| properties.table_cell_spacing.as_ref())
      .and_then(table_cell_spacing_to_points)
      .or(table_style.cell_spacing_pt)
      .unwrap_or(0.0),
    rows,
  };
  if right_to_left {
    normalize_right_to_left_table(&mut model);
  }
  model
}

fn normalize_right_to_left_table(table: &mut Table) {
  // ECMA-376 Part 1 §17.4.1: table data stays in logical order, while cells
  // and every table-level leading/trailing property are displayed right to
  // left. Normalize the imported model to physical left-to-right order so the
  // rest of table layout, merging, border conflict resolution, and painting
  // all operate on one coordinate system.
  table.column_widths_pt.reverse();
  table.alignment = match table.alignment {
    TableAlignment::Left => TableAlignment::Right,
    TableAlignment::Right => TableAlignment::Left,
    TableAlignment::Center => TableAlignment::Center,
  };
  if let Some(borders) = &mut table.borders {
    std::mem::swap(&mut borders.left, &mut borders.right);
  }
  for row in &mut table.rows {
    std::mem::swap(&mut row.grid_before, &mut row.grid_after);
    row.cells.reverse();
    for cell in &mut row.cells {
      std::mem::swap(&mut cell.margins.left_pt, &mut cell.margins.right_pt);
      std::mem::swap(&mut cell.borders.left, &mut cell.borders.right);
      std::mem::swap(
        &mut cell.border_suppressions.left,
        &mut cell.border_suppressions.right,
      );
    }
  }
}

fn should_align_leading_cell_content(
  context: TableModelContext,
  compatibility_mode: u16,
  has_styles_part: bool,
) -> bool {
  if context.nested_table_level != 1 {
    return false;
  }

  // LibreOffice's OOXML importer documents the Word positioning rule in
  // DomainMapperTableHandler: before compatibility mode 15, top-level table
  // placement makes the first cell's text (rather than its border) start at
  // `w:tblInd`, so the leading cell margin is subtracted from the table origin.
  // Keep the existing missing-Styles recovery behavior for modern packages.
  compatibility_mode < 15 || !has_styles_part
}

fn table_starts_after_last_rendered_page_break(rows: &[TableRow]) -> bool {
  rows
    .iter()
    .flat_map(|row| &row.cells)
    .flat_map(|cell| &cell.blocks)
    .find_map(|block| match block {
      Block::Paragraph(paragraph) if !paragraph_is_effectively_empty(paragraph) => {
        Some(paragraph.starts_after_last_rendered_page_break)
      }
      Block::Table(table) if !table.rows.is_empty() => {
        Some(table_starts_after_last_rendered_page_break(&table.rows))
      }
      _ => None,
    })
    .unwrap_or(false)
}

pub(super) fn paragraph_starts_after_last_rendered_page_break(inlines: &[InlineItem]) -> bool {
  let mut saw_last_rendered_page_break = false;
  for inline in inlines {
    match inline {
      InlineItem::LastRenderedPageBreak => saw_last_rendered_page_break = true,
      InlineItem::Text(run) if !run.text.trim().is_empty() => {
        return saw_last_rendered_page_break;
      }
      InlineItem::PositionalTab(_) => {}
      InlineItem::Ruby(_) | InlineItem::Image(_) | InlineItem::Shape(_) => {
        return saw_last_rendered_page_break;
      }
      InlineItem::PageBreak | InlineItem::ColumnBreak => return false,
      InlineItem::Text(_)
      | InlineItem::BookmarkStart(_)
      | InlineItem::FormWidgetStart(_)
      | InlineItem::FormWidgetEnd(_)
      | InlineItem::DrawingGroupStart(_)
      | InlineItem::DrawingGroupEnd => {}
    }
  }
  false
}

fn table_position_placement(properties: &w::TablePositionProperties) -> FloatingFramePlacement {
  let margin_left_pt = properties
    .left_from_text
    .as_ref()
    .and_then(twips_measure_to_points)
    .unwrap_or(0.0);
  FloatingFramePlacement {
    // MS-OI29500 §2.1.161(c): Word defaults omitted tblpPr anchors to text
    // horizontally and margin vertically, rather than the ECMA page/page
    // defaults.
    horizontal_anchor: properties
      .horizontal_anchor
      .map(|anchor| frame_horizontal_anchor(Some(anchor)))
      .unwrap_or(FrameHorizontalAnchor::Text),
    vertical_anchor: properties
      .vertical_anchor
      .map(|anchor| frame_vertical_anchor(Some(anchor)))
      .unwrap_or(FrameVerticalAnchor::Margin),
    horizontal_alignment: properties
      .table_position_x_alignment
      .map(frame_horizontal_alignment),
    vertical_alignment: properties
      .table_position_y_alignment
      .map(frame_vertical_alignment),
    horizontal_offset_pt: properties
      .table_position_x
      .as_ref()
      .and_then(signed_twips_measure_to_points)
      .unwrap_or(0.0),
    vertical_offset_pt: properties
      .table_position_y
      .as_ref()
      .and_then(signed_twips_measure_to_points)
      .unwrap_or(0.0),
    vertical_offset_explicit: properties.table_position_y.is_some(),
    wrap: FrameWrapMode::Around,
    margin_top_pt: properties
      .top_from_text
      .as_ref()
      .and_then(twips_measure_to_points)
      .unwrap_or(0.0),
    margin_right_pt: properties
      .right_from_text
      .as_ref()
      .and_then(twips_measure_to_points)
      .unwrap_or(0.0),
    margin_bottom_pt: properties
      .bottom_from_text
      .as_ref()
      .and_then(twips_measure_to_points)
      .unwrap_or(0.0),
    margin_left_pt,
  }
}

fn table_allows_overlap(properties: Option<&w::TableProperties>) -> bool {
  properties
    .and_then(|properties| properties.table_overlap.as_ref())
    .is_none_or(|overlap| matches!(overlap.val, w::TableOverlapValues::Overlap))
}

fn frame_horizontal_anchor(value: Option<w::HorizontalAnchorValues>) -> FrameHorizontalAnchor {
  match value.unwrap_or_default() {
    w::HorizontalAnchorValues::Text => FrameHorizontalAnchor::Text,
    w::HorizontalAnchorValues::Margin => FrameHorizontalAnchor::Margin,
    w::HorizontalAnchorValues::Page => FrameHorizontalAnchor::Page,
  }
}

fn frame_vertical_anchor(value: Option<w::VerticalAnchorValues>) -> FrameVerticalAnchor {
  match value.unwrap_or_default() {
    w::VerticalAnchorValues::Text => FrameVerticalAnchor::Text,
    w::VerticalAnchorValues::Margin => FrameVerticalAnchor::Margin,
    w::VerticalAnchorValues::Page => FrameVerticalAnchor::Page,
  }
}

fn frame_horizontal_alignment(value: w::HorizontalAlignmentValues) -> FrameHorizontalAlignment {
  match value {
    w::HorizontalAlignmentValues::Left => FrameHorizontalAlignment::Left,
    w::HorizontalAlignmentValues::Center => FrameHorizontalAlignment::Center,
    w::HorizontalAlignmentValues::Right => FrameHorizontalAlignment::Right,
    w::HorizontalAlignmentValues::Inside => FrameHorizontalAlignment::Inside,
    w::HorizontalAlignmentValues::Outside => FrameHorizontalAlignment::Outside,
  }
}

fn frame_vertical_alignment(value: w::VerticalAlignmentValues) -> FrameVerticalAlignment {
  match value {
    w::VerticalAlignmentValues::Inline => FrameVerticalAlignment::Inline,
    w::VerticalAlignmentValues::Top => FrameVerticalAlignment::Top,
    w::VerticalAlignmentValues::Center => FrameVerticalAlignment::Center,
    w::VerticalAlignmentValues::Bottom => FrameVerticalAlignment::Bottom,
    w::VerticalAlignmentValues::Inside => FrameVerticalAlignment::Inside,
    w::VerticalAlignmentValues::Outside => FrameVerticalAlignment::Outside,
  }
}

fn frame_wrap_mode(value: Option<w::TextWrappingValues>) -> FrameWrapMode {
  match value.unwrap_or_default() {
    w::TextWrappingValues::Auto => FrameWrapMode::Auto,
    w::TextWrappingValues::Around => FrameWrapMode::Around,
    w::TextWrappingValues::Tight => FrameWrapMode::Tight,
    w::TextWrappingValues::Through => FrameWrapMode::Through,
    w::TextWrappingValues::None => FrameWrapMode::None,
    w::TextWrappingValues::NotBeside => FrameWrapMode::NotBeside,
  }
}

fn frame_height_rule(
  value: Option<w::HeightRuleValues>,
  height_pt: Option<f32>,
) -> FrameHeightRule {
  match value {
    Some(w::HeightRuleValues::Auto) => FrameHeightRule::Auto,
    Some(w::HeightRuleValues::AtLeast) => FrameHeightRule::AtLeast,
    Some(w::HeightRuleValues::Exact) => FrameHeightRule::Exact,
    // [MS-OI29500] Part 1 §17.3.1.11(f): unlike the ECMA `auto`
    // default, Word treats an omitted hRule as `atLeast` when the authored
    // frame height is non-zero. An omitted/zero height remains automatic.
    None if height_pt.is_some_and(|height| height > 0.0) => FrameHeightRule::AtLeast,
    None => FrameHeightRule::Auto,
  }
}

fn table_row_model(
  row: &w::TableRow,
  context: &mut TableImportContext<'_>,
  row_index: usize,
) -> TableRow {
  let (grid_before, grid_after) = table_row_grid_properties(row.table_row_properties.as_deref());
  // ECMA-376 Part 1 §17.4.60 makes tblPrEx properties replace tblPr for the
  // current row. In particular, the row's tblLook controls which conditional
  // table-style regions apply to both its row and its cells.
  let row_table_look = table_row_look(row, context.table_look);
  let row_condition = table_row_conditional_style(row.table_row_properties.as_deref())
    .unwrap_or_else(|| {
      TableConditionalStyleMask::from_row_position(row_table_look, row_index, context.row_count)
    });
  let mut row_style = table_row_style_for(
    context.table_style,
    row_table_look,
    row_index,
    context.row_count,
    row_condition,
  );
  merge_table_row_style(
    &mut row_style,
    &direct_table_row_style(row.table_row_properties.as_deref()),
  );
  let row_table_shading = row
    .table_property_exceptions
    .as_deref()
    .and_then(|properties| properties.shading.as_ref())
    .map(shading_fill)
    .unwrap_or(context.table_shading);
  let cells = table_row_cells(row);
  let cell_count = cells.len();
  let cells = cells
    .iter()
    .enumerate()
    .map(|(cell_index, source)| {
      table_cell_model(
        source.cell,
        source.sdt_properties,
        context,
        row.table_property_exceptions.as_deref(),
        row_table_shading,
        table_cell_style_for(
          context.table_style,
          TableCellStyleContext {
            look: row_table_look,
            row_index,
            row_count: context.row_count,
            cell_index,
            cell_count,
            row_condition,
            cell_condition: source
              .cell
              .table_cell_properties
              .as_deref()
              .and_then(table_cell_conditional_style),
          },
        ),
      )
    })
    .collect::<Vec<_>>();
  TableRow {
    height_pt: row_style.height_pt,
    exact_height: row_style.exact_height.unwrap_or(false),
    repeat_header: row_style.repeat_header.unwrap_or(false),
    keep_with_next: table_row_keep_with_next(&cells, context.nested_table_level),
    cant_split: row_style.cant_split.unwrap_or(false),
    cell_spacing_pt: row_style.cell_spacing_pt,
    grid_before,
    grid_after,
    width_before_pt: row_style.width_before_pt,
    width_after_pt: row_style.width_after_pt,
    layout: row
      .table_property_exceptions
      .as_deref()
      .and_then(|properties| properties.table_layout.as_ref())
      .map(table_layout_mode),
    borders: row
      .table_property_exceptions
      .as_deref()
      .and_then(|properties| properties.table_borders.as_deref())
      .map(|borders| direct_table_borders_model(context.table_borders, borders)),
    redline_color: None,
    cells,
  }
}

fn table_row_look(row: &w::TableRow, table_look: TableLookModel) -> TableLookModel {
  row
    .table_property_exceptions
    .as_deref()
    .and_then(|properties| properties.table_look.as_ref())
    .map(table_look_model)
    .unwrap_or(table_look)
}

#[derive(Clone, Copy)]
struct TableRowCellSource<'a> {
  cell: &'a w::TableCell,
  sdt_properties: Option<&'a w::SdtProperties>,
}

fn table_row_cells(row: &w::TableRow) -> Vec<TableRowCellSource<'_>> {
  let mut cells = Vec::new();
  for choice in &row.table_row_choice {
    match choice {
      w::TableRowChoice::TableCell(cell) => cells.push(TableRowCellSource {
        cell: cell.as_ref(),
        sdt_properties: None,
      }),
      w::TableRowChoice::SdtCell(sdt) => collect_sdt_cells(sdt, None, &mut cells),
      _ => {}
    }
  }
  cells
}

fn collect_sdt_cells<'a>(
  sdt: &'a w::SdtCell,
  inherited_properties: Option<&'a w::SdtProperties>,
  cells: &mut Vec<TableRowCellSource<'a>>,
) {
  let Some(content) = sdt.sdt_content_cell.as_ref() else {
    return;
  };
  // ECMA-376 Part 1 §17.5.2.33 defines cell-level `sdtContent` as a cache
  // updated from its data binding. Preserve the nearest text/date SDT
  // properties while exposing the contained physical table cell.
  let properties = sdt
    .sdt_properties
    .as_ref()
    .filter(|properties| sdt_supports_bound_text(properties))
    .or(inherited_properties);
  for choice in &content.sdt_content_cell_choice {
    match choice {
      w::SdtContentCellChoice::TableCell(cell) => cells.push(TableRowCellSource {
        cell: cell.as_ref(),
        sdt_properties: properties,
      }),
      w::SdtContentCellChoice::SdtCell(nested) => collect_sdt_cells(nested, properties, cells),
      _ => {}
    }
  }
}

fn table_row_keep_with_next(cells: &[TableCell], nested_table_level: usize) -> bool {
  if nested_table_level > 0 {
    return false;
  }
  let Some(cell) = cells.first() else {
    return false;
  };
  let Some(Block::Paragraph(paragraph)) = cell.blocks.first() else {
    return false;
  };
  paragraph.format.keep_with_next
}

fn table_row_is_deleted(row: &w::TableRow) -> bool {
  row
    .table_row_properties
    .as_deref()
    .is_some_and(|properties| properties.deleted.is_some())
}

fn table_row_style_for(
  table_style: &TableStyleModel,
  look: TableLookModel,
  row_index: usize,
  row_count: usize,
  condition_mask: TableConditionalStyleMask,
) -> TableRowStyle {
  let mut style = table_style.whole_row;
  for (condition, conditional_style) in &table_style.conditional_rows {
    let applies = table::row_style_condition_applies(*condition, look, row_index, row_count)
      || condition_mask.row_condition_applies(*condition);
    if applies {
      merge_table_row_style(&mut style, conditional_style);
    }
  }
  style
}

#[derive(Clone, Copy, Debug)]
struct TableCellStyleContext {
  look: TableLookModel,
  row_index: usize,
  row_count: usize,
  cell_index: usize,
  cell_count: usize,
  row_condition: TableConditionalStyleMask,
  cell_condition: Option<TableConditionalStyleMask>,
}

fn table_cell_style_for(
  table_style: &TableStyleModel,
  context: TableCellStyleContext,
) -> TableCellStyle {
  let mut style = table_style.whole_table.clone();
  let position_mask = TableConditionalStyleMask::from_row_position(
    context.look,
    context.row_index,
    context.row_count,
  )
  .with_cell_mask(TableConditionalStyleMask::from_cell_position(
    context.look,
    context.cell_index,
    context.cell_count,
  ));
  let condition_mask = context
    .row_condition
    .with_cell_mask(context.cell_condition.unwrap_or_else(|| {
      TableConditionalStyleMask::from_cell_position(
        context.look,
        context.cell_index,
        context.cell_count,
      )
    }));
  for (condition, conditional_style) in &table_style.conditional {
    let applies = table::cell_style_condition_applies(
      *condition,
      context.look,
      context.row_index,
      context.row_count,
      context.cell_index,
      context.cell_count,
    ) || position_mask.cell_condition_applies(*condition)
      || condition_mask.cell_condition_applies(*condition);
    if applies {
      merge_table_cell_style(&mut style, conditional_style);
    }
  }
  style
}

fn table_cell_model(
  cell: &w::TableCell,
  sdt_properties: Option<&w::SdtProperties>,
  context: &mut TableImportContext<'_>,
  row_table_exceptions: Option<&w::TablePropertyExceptions>,
  row_table_shading: Option<RgbColor>,
  style: TableCellStyle,
) -> TableCell {
  let properties = cell.table_cell_properties.as_deref();
  let vertical_merge_continue = properties
    .and_then(|properties| properties.vertical_merge.as_ref())
    .map(|merge| matches!(merge.val, None | Some(w::MergedCellValues::Continue)))
    .unwrap_or(false);
  let numbering_state = vertical_merge_continue.then(|| context.numbering.counter_state());
  let base_margins = if context.direct_cell_margins {
    context.cell_margins
  } else {
    style.margins.unwrap_or(context.cell_margins)
  };
  let row_cell_margins = row_table_exceptions
    .and_then(|exceptions| exceptions.table_cell_margin_default.as_deref())
    .map(|margins| table_cell_margin_default_with_base(margins, base_margins))
    .unwrap_or(base_margins);
  let mut blocks = Vec::new();
  let mut pending_out_of_place_breaks = Vec::new();
  for choice in &cell.table_cell_choice {
    let block_start = blocks.len();
    match choice {
      w::TableCellChoice::Paragraph(paragraph) => {
        let mut model = paragraph_model_with_base(
          paragraph,
          context.styles,
          context.numbering,
          context.images,
          context.hyperlinks,
          context.form_widget_ids,
          ParagraphImportBase {
            format: style.paragraph_format.clone(),
            run_style: style.run_style.clone(),
            run_overrides: style.run_overrides,
            custom_xml_bindings: Some(context.custom_xml_bindings),
          },
        );
        if !context.in_header_footer {
          apply_recovered_table_cell_paragraph_defaults(paragraph, context.styles, &mut model);
        }
        let out_of_place_table = paragraph.paragraph_choice.iter().find_map(|choice| {
          let w::ParagraphChoice::Table(table) = choice else {
            return None;
          };
          Some(table.as_ref())
        });
        if let Some(table) = out_of_place_table {
          let mut table = table_model(
            table,
            &mut TableModelEnv {
              styles: context.styles,
              numbering: context.numbering,
              images: context.images,
              hyperlinks: context.hyperlinks,
              custom_xml_bindings: context.custom_xml_bindings,
              form_widget_ids: context.form_widget_ids,
            },
            TableModelContext {
              nested_table_level: 2,
              in_header_footer: context.in_header_footer,
            },
          );
          if prepend_out_of_place_paragraph_to_nested_table(&mut table, &mut model) {
            blocks.push(Block::Table(table));
          } else {
            push_cell_paragraph(&mut blocks, model);
            blocks.push(Block::Table(table));
          }
        } else {
          push_cell_paragraph(&mut blocks, model);
        }
      }
      w::TableCellChoice::Table(table) => blocks.push(Block::Table(table_model(
        table,
        &mut TableModelEnv {
          styles: context.styles,
          numbering: context.numbering,
          images: context.images,
          hyperlinks: context.hyperlinks,
          custom_xml_bindings: context.custom_xml_bindings,
          form_widget_ids: context.form_widget_ids,
        },
        TableModelContext {
          nested_table_level: 2,
          in_header_footer: context.in_header_footer,
        },
      ))),
      w::TableCellChoice::SdtBlock(sdt) => blocks.extend(sdt_block_blocks(
        sdt,
        context.styles,
        context.numbering,
        context.images,
        context.hyperlinks,
        SdtBlockControls {
          custom_xml_bindings: context.custom_xml_bindings,
          form_widget_ids: context.form_widget_ids,
          in_header_footer: context.in_header_footer,
        },
      )),
      w::TableCellChoice::Break(br) => pending_out_of_place_breaks.push(br.clone()),
      _ => {}
    }
    if prepend_out_of_place_breaks_to_first_character_group(
      &mut blocks[block_start..],
      &pending_out_of_place_breaks,
    ) {
      pending_out_of_place_breaks.clear();
    }
  }
  if let Some(value) = sdt_properties
    .and_then(|properties| sdt_bound_replacement(context.custom_xml_bindings, properties))
  {
    replace_sdt_block_text(&mut blocks, value);
  }
  let text_rotation_deg = properties.and_then(table_cell_text_rotation_degrees);
  if let Some(rotation_deg) = text_rotation_deg {
    rotate_blocks_text(&mut blocks, rotation_deg);
  }
  if let Some(numbering_state) = numbering_state {
    // LibreOffice tdf#147646: paragraphs stored in the continuation cells of
    // a vertical merge are not counted list items. Their content is hidden by
    // the merged cell, so importing it must not advance numbering seen by
    // later visible rows.
    context.numbering.restore_counter_state(numbering_state);
  }
  TableCell {
    blocks,
    shading: resolved_table_cell_shading(
      properties
        .and_then(|properties| properties.shading.as_ref())
        .map(shading_fill),
      style.shading,
      row_table_shading,
    ),
    borders: properties
      .and_then(|properties| properties.table_cell_borders.as_deref())
      .map(|borders| direct_cell_borders_model(style.borders, borders))
      .unwrap_or(style.borders),
    border_suppressions: properties
      .and_then(|properties| properties.table_cell_borders.as_deref())
      .map(cell_border_suppressions)
      .unwrap_or_default(),
    margins: properties
      .and_then(|properties| properties.table_cell_margin.as_deref())
      .map(|margins| table_cell_margin(margins, row_cell_margins))
      .unwrap_or(row_cell_margins),
    preferred_width_pt: properties
      .and_then(|properties| properties.table_cell_width.as_ref())
      .and_then(table_cell_width_to_points),
    preferred_width_pct: properties
      .and_then(|properties| properties.table_cell_width.as_ref())
      .and_then(table_cell_width_to_percent),
    grid_span: properties
      .and_then(|properties| properties.grid_span.as_ref())
      .map(|span| span.val.max(1) as usize)
      .unwrap_or(1),
    vertical_merge_continue,
    no_wrap: properties
      .and_then(|properties| properties.no_wrap.as_ref())
      .map(|no_wrap| on_off_only_value(no_wrap.val))
      .or(style.no_wrap)
      .unwrap_or(false),
    fit_text: properties
      .and_then(|properties| properties.table_cell_fit_text.as_ref())
      .is_some_and(|fit_text| on_off_only_value(fit_text.val)),
    hide_end_mark: properties
      .and_then(|properties| properties.hide_mark.as_ref())
      .is_some_and(|hide_mark| on_off_only_value(hide_mark.val)),
    vertical_alignment: properties
      .and_then(|properties| properties.table_cell_vertical_alignment.as_ref())
      .map(table_cell_vertical_alignment)
      .or(style.vertical_alignment)
      .unwrap_or_default(),
    text_rotation_deg,
  }
}

fn resolved_table_cell_shading(
  direct_cell: Option<Option<RgbColor>>,
  styled_cell: Option<Option<RgbColor>>,
  row_or_table: Option<RgbColor>,
) -> Option<RgbColor> {
  direct_cell.or(styled_cell).unwrap_or(row_or_table)
}

fn prepend_out_of_place_paragraph_to_nested_table(
  table: &mut Table,
  source: &mut Paragraph,
) -> bool {
  if source.format.frame.is_some() || source.list_label.is_some() {
    return false;
  }
  let Some(target) = table
    .rows
    .first_mut()
    .and_then(|row| row.cells.first_mut())
    .and_then(|cell| {
      cell.blocks.iter_mut().find_map(|block| match block {
        Block::Paragraph(paragraph) => Some(paragraph),
        _ => None,
      })
    })
  else {
    return false;
  };

  source.inlines.append(&mut target.inlines);
  target.inlines = std::mem::take(&mut source.inlines);
  source
    .footnote_reference_ids
    .append(&mut target.footnote_reference_ids);
  target.footnote_reference_ids = std::mem::take(&mut source.footnote_reference_ids);
  source
    .endnote_reference_ids
    .append(&mut target.endnote_reference_ids);
  target.endnote_reference_ids = std::mem::take(&mut source.endnote_reference_ids);
  target.starts_after_last_rendered_page_break |= source.starts_after_last_rendered_page_break;
  #[cfg(test)]
  {
    source.runs.append(&mut target.runs);
    target.runs = std::mem::take(&mut source.runs);
  }
  true
}

fn push_cell_paragraph(blocks: &mut Vec<Block>, mut paragraph: Paragraph) {
  let Some(frame) = paragraph.format.frame else {
    blocks.push(Block::paragraph(paragraph));
    return;
  };
  paragraph.format.frame = None;
  if let Some(Block::Frame(previous)) = blocks.last_mut()
    && paragraph_belongs_to_frame(previous, frame, &paragraph)
  {
    previous.blocks.push(Block::paragraph(paragraph));
    return;
  }
  let fill_color = paragraph.format.shading;
  let borders = paragraph.format.borders;
  blocks.push(Block::Frame(FloatingFrame {
    blocks: vec![Block::paragraph(paragraph)],
    width_pt: frame.width_pt,
    height_pt: frame.height_pt,
    height_rule: frame.height_rule,
    placement: frame.placement,
    fill_color,
    borders,
  }));
}

fn table_row_grid_properties(properties: Option<&w::TableRowProperties>) -> (usize, usize) {
  let Some(properties) = properties else {
    return (0, 0);
  };
  let mut grid_before = 0;
  let mut grid_after = 0;
  for choice in &properties.table_row_properties_choice1 {
    match choice {
      w::TableRowPropertiesChoice::GridBefore(before) => {
        grid_before = before.val.max(0) as usize;
      }
      w::TableRowPropertiesChoice::GridAfter(after) => {
        grid_after = after.val.max(0) as usize;
      }
      _ => {}
    }
  }
  (grid_before, grid_after)
}

fn table_row_conditional_style(
  properties: Option<&w::TableRowProperties>,
) -> Option<TableConditionalStyleMask> {
  properties.and_then(|properties| {
    properties
      .table_row_properties_choice1
      .iter()
      .find_map(|choice| {
        if let w::TableRowPropertiesChoice::ConditionalFormatStyle(style) = choice {
          Some(TableConditionalStyleMask::from_cnf_style(style))
        } else {
          None
        }
      })
  })
}

fn table_cell_conditional_style(
  properties: &w::TableCellProperties,
) -> Option<TableConditionalStyleMask> {
  properties
    .conditional_format_style
    .as_ref()
    .map(TableConditionalStyleMask::from_cnf_style)
}

fn table_cell_margin_default(margins: &w::TableCellMarginDefault) -> CellMargins {
  table_cell_margin_default_with_base(margins, CellMargins::default())
}

fn table_cell_margin_default_with_base(
  margins: &w::TableCellMarginDefault,
  mut model: CellMargins,
) -> CellMargins {
  if let Some(top) = &margins.top_margin
    && let Some(value) = margin_width_to_points(top.width.as_ref(), top.r#type)
  {
    model.top_pt = value;
  }
  if let Some(bottom) = &margins.bottom_margin
    && let Some(value) = margin_width_to_points(bottom.width.as_ref(), bottom.r#type)
  {
    model.bottom_pt = value;
  }
  if let Some(left) = &margins.table_cell_left_margin
    && let Some(value) = margin_width_to_points(left.width.as_ref(), left.r#type)
  {
    model.left_pt = value;
  }
  if let Some(start) = &margins.start_margin
    && let Some(value) = margin_width_to_points(start.width.as_ref(), start.r#type)
  {
    model.left_pt = value;
  }
  if let Some(right) = &margins.table_cell_right_margin
    && let Some(value) = margin_width_to_points(right.width.as_ref(), right.r#type)
  {
    model.right_pt = value;
  }
  if let Some(end) = &margins.end_margin
    && let Some(value) = margin_width_to_points(end.width.as_ref(), end.r#type)
  {
    model.right_pt = value;
  }
  model
}

fn table_cell_margin(margins: &w::TableCellMargin, mut model: CellMargins) -> CellMargins {
  if let Some(top) = &margins.top_margin
    && let Some(value) = margin_width_to_points(top.width.as_ref(), top.r#type)
  {
    model.top_pt = value;
  }
  if let Some(bottom) = &margins.bottom_margin
    && let Some(value) = margin_width_to_points(bottom.width.as_ref(), bottom.r#type)
  {
    model.bottom_pt = value;
  }
  if let Some(left) = &margins.left_margin
    && let Some(value) = margin_width_to_points(left.width.as_ref(), left.r#type)
  {
    model.left_pt = value;
  }
  if let Some(start) = &margins.start_margin
    && let Some(value) = margin_width_to_points(start.width.as_ref(), start.r#type)
  {
    model.left_pt = value;
  }
  if let Some(right) = &margins.right_margin
    && let Some(value) = margin_width_to_points(right.width.as_ref(), right.r#type)
  {
    model.right_pt = value;
  }
  if let Some(end) = &margins.end_margin
    && let Some(value) = margin_width_to_points(end.width.as_ref(), end.r#type)
  {
    model.right_pt = value;
  }
  model
}

fn margin_width_to_points(
  width: Option<&MeasurementOrPercentValue>,
  width_type: Option<w::TableWidthUnitValues>,
) -> Option<f32> {
  if !matches!(width_type, None | Some(w::TableWidthUnitValues::Dxa)) {
    return None;
  }
  width.and_then(table_margin_measurement_to_points)
}

fn table_width_to_points(width: &w::TableWidth) -> Option<f32> {
  match width.r#type {
    Some(w::TableWidthUnitValues::Dxa) | None => width
      .width
      .as_ref()
      .and_then(measurement_or_percent_to_points),
    _ => None,
  }
}

fn row_width_to_points(
  width: Option<&MeasurementOrPercentValue>,
  width_type: Option<w::TableWidthUnitValues>,
) -> Option<f32> {
  if !matches!(width_type, None | Some(w::TableWidthUnitValues::Dxa)) {
    return None;
  }
  width.and_then(measurement_or_percent_to_points)
}

fn table_cell_spacing_to_points(spacing: &w::TableCellSpacing) -> Option<f32> {
  if !matches!(spacing.r#type, None | Some(w::TableWidthUnitValues::Dxa)) {
    return None;
  }
  spacing
    .width
    .as_ref()
    .and_then(measurement_or_percent_to_points)
}

fn table_width_to_percent(width: &w::TableWidth) -> Option<f32> {
  if !matches!(width.r#type, Some(w::TableWidthUnitValues::Pct)) {
    return None;
  }
  width
    .width
    .as_ref()
    .and_then(measurement_or_percent_to_percent)
}

fn table_cell_width_to_points(width: &w::TableCellWidth) -> Option<f32> {
  match width.r#type {
    Some(w::TableWidthUnitValues::Dxa) | None => width
      .width
      .as_ref()
      .and_then(measurement_or_percent_to_points),
    _ => None,
  }
}

fn table_cell_width_to_percent(width: &w::TableCellWidth) -> Option<f32> {
  if !matches!(width.r#type, Some(w::TableWidthUnitValues::Pct)) {
    return None;
  }
  width
    .width
    .as_ref()
    .and_then(measurement_or_percent_to_percent)
}

fn table_indentation_to_points(indentation: &w::TableIndentation) -> Option<f32> {
  if !matches!(
    indentation.r#type,
    None | Some(w::TableWidthUnitValues::Dxa)
  ) {
    return None;
  }
  indentation
    .width
    .as_ref()
    // ECMA-376 Part 1 §17.4.50 adds tblInd to the leading edge. Unlike cell
    // margins, this offset can be negative: Word uses it to move a leading
    // table into the page margin (for example, -118 twips in n780645.docx).
    .and_then(measurement_or_percent_to_twips)
    .map(|twips| {
      // Word's legacy table-indent path stores the low 16 bits as a signed
      // twip value. In large-twips.docx, 65000 therefore becomes -536 twips
      // and moves the table 26.8pt into the leading margin. Keep ordinary
      // signed and larger modern measurements unchanged.
      if twips.fract() == 0.0 && (32_768.0..=65_535.0).contains(&twips) {
        (twips as u16 as i16) as f32
      } else {
        twips
      }
    })
    .map(units::twips_to_points)
}

fn table_alignment(justification: &w::TableJustification) -> TableAlignment {
  match justification.val {
    w::TableRowAlignmentValues::Center => TableAlignment::Center,
    w::TableRowAlignmentValues::Right | w::TableRowAlignmentValues::End => TableAlignment::Right,
    w::TableRowAlignmentValues::Left | w::TableRowAlignmentValues::Start => TableAlignment::Left,
  }
}

fn table_layout_mode(layout: &w::TableLayout) -> TableLayoutMode {
  match layout.r#type.unwrap_or(w::TableLayoutValues::Autofit) {
    w::TableLayoutValues::Fixed => TableLayoutMode::Fixed,
    w::TableLayoutValues::Autofit => TableLayoutMode::AutoFit,
  }
}

fn shading_fill(shading: &w::Shading) -> Option<RgbColor> {
  shading.fill.as_deref().and_then(parse_hex_color)
}

fn table_cell_vertical_alignment(
  alignment: &w::TableCellVerticalAlignment,
) -> TableCellVerticalAlignment {
  match alignment.val {
    w::TableVerticalAlignmentValues::Center => TableCellVerticalAlignment::Center,
    w::TableVerticalAlignmentValues::Bottom => TableCellVerticalAlignment::Bottom,
    w::TableVerticalAlignmentValues::Top => TableCellVerticalAlignment::Top,
  }
}

fn table_borders_model(borders: &w::TableBorders) -> TableBordersModel {
  TableBordersModel {
    top: borders.top_border.as_ref().and_then(top_border_style),
    right: borders
      .end_border
      .as_ref()
      .and_then(end_border_style)
      .or_else(|| borders.right_border.as_ref().and_then(right_border_style)),
    bottom: borders.bottom_border.as_ref().and_then(bottom_border_style),
    left: borders
      .start_border
      .as_ref()
      .and_then(start_border_style)
      .or_else(|| borders.left_border.as_ref().and_then(left_border_style)),
    inside_horizontal: borders
      .inside_horizontal_border
      .as_ref()
      .and_then(inside_horizontal_border_style),
    inside_vertical: borders
      .inside_vertical_border
      .as_ref()
      .and_then(inside_vertical_border_style),
  }
}

fn direct_table_borders_model(
  base: Option<TableBordersModel>,
  borders: &w::TableBorders,
) -> TableBordersModel {
  let mut base = base.unwrap_or_default();
  if let Some(top) = borders.top_border.as_ref().map(top_border_override) {
    base.top = top;
  }
  if let Some(right) = borders
    .end_border
    .as_ref()
    .map(end_border_override)
    .or_else(|| borders.right_border.as_ref().map(right_border_override))
  {
    base.right = right;
  }
  if let Some(bottom) = borders.bottom_border.as_ref().map(bottom_border_override) {
    base.bottom = bottom;
  }
  if let Some(left) = borders
    .start_border
    .as_ref()
    .map(start_border_override)
    .or_else(|| borders.left_border.as_ref().map(left_border_override))
  {
    base.left = left;
  }
  if let Some(inside_horizontal) = borders
    .inside_horizontal_border
    .as_ref()
    .map(inside_horizontal_border_override)
  {
    base.inside_horizontal = inside_horizontal;
  }
  if let Some(inside_vertical) = borders
    .inside_vertical_border
    .as_ref()
    .map(inside_vertical_border_override)
  {
    base.inside_vertical = inside_vertical;
  }
  base
}

fn cell_borders_model(borders: &w::TableCellBorders) -> CellBordersModel {
  CellBordersModel {
    top: borders.top_border.as_ref().and_then(top_border_style),
    right: borders
      .end_border
      .as_ref()
      .and_then(end_border_style)
      .or_else(|| borders.right_border.as_ref().and_then(right_border_style)),
    bottom: borders.bottom_border.as_ref().and_then(bottom_border_style),
    left: borders
      .start_border
      .as_ref()
      .and_then(start_border_style)
      .or_else(|| borders.left_border.as_ref().and_then(left_border_style)),
  }
}

fn cell_border_suppressions(borders: &w::TableCellBorders) -> crate::docx::CellBorderSuppressions {
  fn suppresses(value: w::BorderValues) -> bool {
    matches!(value, w::BorderValues::Nil | w::BorderValues::None)
  }

  crate::docx::CellBorderSuppressions {
    top: borders
      .top_border
      .as_ref()
      .is_some_and(|border| suppresses(border.val)),
    right: borders
      .end_border
      .as_ref()
      .map(|border| suppresses(border.val))
      .or_else(|| {
        borders
          .right_border
          .as_ref()
          .map(|border| suppresses(border.val))
      })
      .unwrap_or(false),
    bottom: borders
      .bottom_border
      .as_ref()
      .is_some_and(|border| suppresses(border.val)),
    left: borders
      .start_border
      .as_ref()
      .map(|border| suppresses(border.val))
      .or_else(|| {
        borders
          .left_border
          .as_ref()
          .map(|border| suppresses(border.val))
      })
      .unwrap_or(false),
  }
}

fn direct_cell_borders_model(
  mut base: CellBordersModel,
  borders: &w::TableCellBorders,
) -> CellBordersModel {
  if let Some(top) = borders.top_border.as_ref().map(top_border_override) {
    base.top = top;
  }
  if let Some(right) = borders
    .end_border
    .as_ref()
    .map(end_border_override)
    .or_else(|| borders.right_border.as_ref().map(right_border_override))
  {
    base.right = right;
  }
  if let Some(bottom) = borders.bottom_border.as_ref().map(bottom_border_override) {
    base.bottom = bottom;
  }
  if let Some(left) = borders
    .start_border
    .as_ref()
    .map(start_border_override)
    .or_else(|| borders.left_border.as_ref().map(left_border_override))
  {
    base.left = left;
  }
  base
}

fn paragraph_borders_model(borders: &w::ParagraphBorders) -> CellBordersModel {
  CellBordersModel {
    top: borders.top_border.as_ref().and_then(top_border_style),
    right: borders.right_border.as_ref().and_then(right_border_style),
    bottom: borders.bottom_border.as_ref().and_then(bottom_border_style),
    left: borders.left_border.as_ref().and_then(left_border_style),
  }
}

fn page_borders_model(borders: &w::PageBorders) -> CellBordersModel {
  CellBordersModel {
    top: borders.top_border.as_ref().and_then(top_border_style),
    right: borders.right_border.as_ref().and_then(right_border_style),
    bottom: borders.bottom_border.as_ref().and_then(bottom_border_style),
    left: borders.left_border.as_ref().and_then(left_border_style),
  }
}

macro_rules! border_style_fn {
  ($name:ident, $ty:ty) => {
    fn $name(border: &$ty) -> Option<BorderStyle> {
      border_style(
        border.val,
        border.size,
        border.space,
        border.color.as_deref(),
        border.shadow,
      )
    }
  };
}

macro_rules! border_override_fn {
  ($name:ident, $ty:ty) => {
    fn $name(border: &$ty) -> Option<BorderStyle> {
      border_style(
        border.val,
        border.size,
        border.space,
        border.color.as_deref(),
        border.shadow,
      )
    }
  };
}

border_style_fn!(top_border_style, w::TopBorder);
border_style_fn!(right_border_style, w::RightBorder);
border_style_fn!(bottom_border_style, w::BottomBorder);
border_style_fn!(left_border_style, w::LeftBorder);
border_style_fn!(start_border_style, w::StartBorder);
border_style_fn!(end_border_style, w::EndBorder);
border_style_fn!(inside_horizontal_border_style, w::InsideHorizontalBorder);
border_style_fn!(inside_vertical_border_style, w::InsideVerticalBorder);
border_override_fn!(top_border_override, w::TopBorder);
border_override_fn!(right_border_override, w::RightBorder);
border_override_fn!(bottom_border_override, w::BottomBorder);
border_override_fn!(left_border_override, w::LeftBorder);
border_override_fn!(start_border_override, w::StartBorder);
border_override_fn!(end_border_override, w::EndBorder);
border_override_fn!(inside_horizontal_border_override, w::InsideHorizontalBorder);
border_override_fn!(inside_vertical_border_override, w::InsideVerticalBorder);

fn border_style(
  value: w::BorderValues,
  size: Option<u32>,
  space: Option<u32>,
  color: Option<&str>,
  shadow: Option<ooxmlsdk::simple_type::OnOffValue>,
) -> Option<BorderStyle> {
  if matches!(value, w::BorderValues::Nil | w::BorderValues::None) {
    return None;
  }

  Some(BorderStyle {
    width_pt: size
      .map(|value| value as f32 / units::WORD_BORDER_SIZE_UNITS_PER_POINT)
      .unwrap_or(WML_DEFAULT_BORDER_WIDTH_PT)
      .max(WML_MIN_BORDER_WIDTH_PT),
    spacing_pt: space.unwrap_or(0) as f32,
    color: color.and_then(parse_hex_color).unwrap_or_default(),
    compound: border_value_is_compound(value),
    dash_pattern: border_value_dash_pattern(value),
    shadow: shadow.is_some_and(ooxmlsdk::simple_type::OnOffValue::as_bool),
  })
}

fn border_value_dash_pattern(value: w::BorderValues) -> BorderDashPattern {
  match value {
    w::BorderValues::Dotted => BorderDashPattern::Dotted,
    w::BorderValues::Dashed => BorderDashPattern::Dashed,
    w::BorderValues::DashSmallGap => BorderDashPattern::FineDashed,
    w::BorderValues::DotDash => BorderDashPattern::DashDot,
    w::BorderValues::DotDotDash => BorderDashPattern::DashDotDot,
    _ => BorderDashPattern::Solid,
  }
}

fn border_value_is_compound(value: w::BorderValues) -> bool {
  matches!(
    value,
    w::BorderValues::Double
      | w::BorderValues::Triple
      | w::BorderValues::ThinThickSmallGap
      | w::BorderValues::ThickThinSmallGap
      | w::BorderValues::ThinThickThinSmallGap
      | w::BorderValues::ThinThickMediumGap
      | w::BorderValues::ThickThinMediumGap
      | w::BorderValues::ThinThickThinMediumGap
      | w::BorderValues::ThinThickLargeGap
      | w::BorderValues::ThickThinLargeGap
      | w::BorderValues::ThinThickThinLargeGap
      | w::BorderValues::DoubleWave
  )
}

fn document_background_color(background: &w::DocumentBackground) -> Option<RgbColor> {
  background
    .background
    .as_deref()
    .and_then(vml_background_pattern_color)
    .or_else(|| background.color.as_deref().and_then(parse_hex_color))
}

fn document_background_image(
  background: &w::DocumentBackground,
  images: &ImageCatalog,
) -> Option<InlineShapeImageFill> {
  background.color.as_ref()?;
  let fill = background.background.as_deref()?.fill.as_deref()?;
  if fill.r#type != Some(v::FillTypeValues::Frame) {
    return None;
  }
  vml_fill_image(fill, None, images)
}

fn vml_background_pattern_color(background: &v::Background) -> Option<RgbColor> {
  let fill = background.fill.as_deref()?;
  if fill.r#type != Some(v::FillTypeValues::Pattern) {
    return None;
  }
  fill
    .color2
    .as_deref()
    .and_then(parse_vml_color)
    .or_else(|| fill.color.as_deref().and_then(parse_vml_color))
    .or_else(|| background.fillcolor.as_deref().and_then(parse_vml_color))
}

fn merge_paragraph_format(
  format: &mut ParagraphFormat,
  properties: Option<ParagraphProps<'_>>,
  import_settings: ImportSettings,
) {
  let Some(properties) = properties else {
    return;
  };

  if let Some(page_break_before) = properties.page_break_before() {
    format.page_break_before = page_break_before.val.is_none_or(|value| value.as_bool());
    format.page_break_before_set = true;
  }
  if let Some(keep_next) = properties.keep_next() {
    format.keep_with_next = keep_next.val.is_none_or(|value| value.as_bool());
    format.keep_with_next_set = true;
  }
  if let Some(keep_lines) = properties.keep_lines() {
    format.keep_lines = keep_lines.val.is_none_or(|value| value.as_bool());
    format.keep_lines_set = true;
  }
  if let Some(widow_control) = properties.widow_control() {
    format.widow_control = Some(widow_control.val.is_none_or(|value| value.as_bool()));
  }
  if let Some(contextual_spacing) = properties.contextual_spacing() {
    format.contextual_spacing = contextual_spacing.val.is_none_or(|value| value.as_bool());
    format.contextual_spacing_set = true;
  }
  if let Some(suppress_auto_hyphens) = properties.suppress_auto_hyphens() {
    format.suppress_auto_hyphens = Some(
      suppress_auto_hyphens
        .val
        .is_none_or(|value| value.as_bool()),
    );
  }
  if let Some(suppress_line_numbers) = properties.suppress_line_numbers() {
    format.suppress_line_numbers = Some(
      suppress_line_numbers
        .val
        .is_none_or(|value| value.as_bool()),
    );
  }
  if let Some(auto_space_de) = properties.auto_space_de() {
    format.auto_space_de = Some(auto_space_de.val.is_none_or(|value| value.as_bool()));
  }
  if let Some(auto_space_dn) = properties.auto_space_dn() {
    format.auto_space_dn = Some(auto_space_dn.val.is_none_or(|value| value.as_bool()));
  }
  if let Some(snap_to_grid) = properties.snap_to_grid() {
    format.snap_to_grid = Some(snap_to_grid.val.is_none_or(|value| value.as_bool()));
  }
  if let Some(text_alignment) = properties.text_alignment() {
    format.line_vertical_alignment = Some(match text_alignment.val {
      w::VerticalTextAlignmentValues::Top => common::LineVerticalAlignment::Top,
      w::VerticalTextAlignmentValues::Center => common::LineVerticalAlignment::Center,
      w::VerticalTextAlignmentValues::Baseline => common::LineVerticalAlignment::Baseline,
      w::VerticalTextAlignmentValues::Bottom => common::LineVerticalAlignment::Bottom,
      w::VerticalTextAlignmentValues::Auto => common::LineVerticalAlignment::Auto,
    });
  }

  if let Some(spacing) = properties.spacing_between_lines() {
    if let Some(before) = spacing.before.as_ref() {
      format.spacing_before_set = true;
      format.spacing_before_pt = signed_twips_measure_to_points(before).unwrap_or(0.0);
      format.spacing_before_lines = None;
    }
    if let Some(before_lines) = spacing.before_lines {
      format.spacing_before_set = true;
      format.spacing_before_pt = 0.0;
      format.spacing_before_lines = Some((before_lines as f32 / 100.0).max(0.0));
    }
    if let Some(before_auto_spacing) = spacing.before_auto_spacing {
      // Preserve explicit false independently from the resolved ordinary
      // spacing. It cancels inherited automatic spacing without erasing an
      // inherited or sibling w:before value.
      let enabled = before_auto_spacing.as_bool();
      format.spacing_before_auto = Some(enabled);
      format.spacing_before_auto_pt =
        enabled.then_some(if import_settings.fixed_html_paragraph_auto_spacing {
          OFFICE_FIXED_AUTOMATIC_PARAGRAPH_BEFORE_PT
        } else {
          OFFICE_AUTOMATIC_PARAGRAPH_SPACING_PT
        });
    }
    if let Some(after) = spacing.after.as_ref() {
      format.spacing_after_set = true;
      format.spacing_after_pt = signed_twips_measure_to_points(after).unwrap_or(0.0);
    }
    if let Some(after_auto_spacing) = spacing.after_auto_spacing {
      // Preserve explicit false independently from the resolved ordinary
      // spacing. It cancels inherited automatic spacing without erasing an
      // inherited or sibling w:after value.
      let enabled = after_auto_spacing.as_bool();
      format.spacing_after_auto = Some(enabled);
      format.spacing_after_auto_pt =
        enabled.then_some(if import_settings.fixed_html_paragraph_auto_spacing {
          OFFICE_FIXED_AUTOMATIC_PARAGRAPH_AFTER_PT
        } else {
          OFFICE_AUTOMATIC_PARAGRAPH_SPACING_PT
        });
    }
    if let Some(line) = spacing.line.as_ref() {
      let negative_line = signed_twips_measure_to_twips(line).is_some_and(|value| value < 0.0);
      if negative_line {
        // LibreOffice's OOXML DomainMapper_Impl::SetLineSpacing() treats the
        // sign as a compatibility mode switch, never as a negative physical
        // height.  An explicit negative `exact` value flips to `atLeast`;
        // negative auto/atLeast values and a negative value with an inherited
        // rule become fixed spacing.  See testTdf125469_singleSpacing.
        format.line_height_rule = if spacing.line_rule == Some(w::LineSpacingRuleValues::Exact) {
          LineHeightRule::AtLeast
        } else {
          LineHeightRule::Exact
        };
        format.line_height_pt = signed_twips_measure_to_points(line).map(f32::abs);
      } else {
        match spacing.line_rule {
          None | Some(w::LineSpacingRuleValues::Auto) => {
            format.line_height_rule = LineHeightRule::Auto;
            if let Some(value) = signed_twips_measure_to_twips(line) {
              format.line_height_pt = Some(
                (value / units::WORD_LINE_HEIGHT_UNITS_PER_LINE).max(MIN_IMPORTED_LINE_HEIGHT_PT),
              );
            }
          }
          Some(w::LineSpacingRuleValues::AtLeast) => {
            format.line_height_rule = LineHeightRule::AtLeast;
            format.line_height_pt = signed_twips_measure_to_points(line);
          }
          Some(w::LineSpacingRuleValues::Exact) => {
            format.line_height_rule = LineHeightRule::Exact;
            format.line_height_pt = signed_twips_measure_to_points(line);
          }
        }
      }
    }
  }

  if let Some(indentation) = properties.indentation() {
    let left_character_units = indentation
      .start_characters
      .or(indentation.left_chars)
      .map(|value| value as f32 / 100.0);
    let right_character_units = indentation
      .end_characters
      .or(indentation.right_chars)
      .map(|value| value as f32 / 100.0);
    let first_line_character_units = indentation
      .hanging_chars
      .map(|value| -(value as f32) / 100.0)
      .or_else(|| {
        indentation
          .first_line_chars
          .map(|value| value as f32 / 100.0)
      });
    if indentation.start.is_some() || indentation.left.is_some() {
      format.indent_left_set = true;
      format.indent_left_pt = indentation
        .start
        .as_ref()
        .or(indentation.left.as_ref())
        .and_then(signed_twips_measure_to_points)
        .unwrap_or(0.0);
    }
    if left_character_units.is_some() {
      format.indent_left_set = true;
      format.indent_left_character_units = left_character_units;
    }
    if indentation.end.is_some() || indentation.right.is_some() {
      format.indent_right_set = true;
      format.indent_right_pt = indentation
        .end
        .as_ref()
        .or(indentation.right.as_ref())
        .and_then(signed_twips_measure_to_points)
        .unwrap_or(0.0);
    }
    if right_character_units.is_some() {
      format.indent_right_set = true;
      format.indent_right_character_units = right_character_units;
    }
    if indentation.first_line.is_some() || indentation.hanging.is_some() {
      format.first_line_indent_set = true;
      format.first_line_indent_pt = indentation.hanging.as_ref().map_or_else(
        || {
          indentation
            .first_line
            .as_ref()
            .and_then(twips_measure_to_points)
            .unwrap_or(0.0)
        },
        |hanging| -signed_twips_measure_to_points(hanging).unwrap_or(0.0),
      );
    }
    if first_line_character_units.is_some() {
      format.first_line_indent_set = true;
      format.first_line_indent_character_units = first_line_character_units;
    }
  }

  if let Some(tabs) = properties.tabs() {
    // LN_CT_PPrBase_tabs initializes the current tab-stop vector from the
    // paragraph style, then DomainMapper_Impl::IncorporateTabStop() applies
    // each direct tab. A w:val="clear" entry removes an inherited tab at the
    // same position instead of being ignored.
    apply_tab_stops(format, tabs);
    format.tab_stops_set = true;
  }

  if let Some(justification) = properties.justification() {
    format.justification = paragraph_justification(justification.val, import_settings);
    format.alignment = format.justification.alignment();
  }

  if let Some(bidi) = properties.bidi() {
    format.bidi = bidi.val.is_none_or(|value| value.as_bool());
  }

  if let Some(shading) = properties.shading() {
    format.shading = shading_fill(shading);
  }

  if let Some(borders) = properties.paragraph_borders() {
    format.borders = paragraph_borders_model(borders);
  }

  if let Some(outline_level) = properties.outline_level() {
    format.outline_level = u8::try_from(outline_level.val)
      .ok()
      .filter(|level| *level <= 8);
  }

  // ECMA-376 §17.3.1.11 makes the presence of framePr in the current
  // paragraph's properties the signal that the paragraph belongs to a text
  // frame. A framePr stored in docDefaults is not that direct signal and
  // must not turn every paragraph in the document into a frame.
  if !matches!(properties, ParagraphProps::BaseStyle(_))
    && let Some(frame) = properties.frame_properties()
  {
    merge_paragraph_frame_properties(format, frame);
  }
}

fn paragraph_justification(
  value: w::JustificationValues,
  import_settings: ImportSettings,
) -> ParagraphJustification {
  let mut justification = ParagraphJustification::default();
  match value {
    w::JustificationValues::Center => {
      justification.adjust = ParagraphAdjust::Center;
    }
    w::JustificationValues::Right | w::JustificationValues::End => {
      justification.adjust = if import_settings.use_literal_direction {
        if import_settings.exchange_left_right {
          ParagraphAdjust::Left
        } else {
          ParagraphAdjust::Right
        }
      } else {
        ParagraphAdjust::End
      };
    }
    w::JustificationValues::Distribute => {
      justification.last_line_adjust = ParagraphAdjust::Block;
      justification.adjust = ParagraphAdjust::Block;
      if import_settings.justify_lines_with_shrinking {
        justification.word_spacing.minimum_pct = 75;
        justification.word_spacing.maximum_pct = 133;
      }
    }
    w::JustificationValues::Both | w::JustificationValues::ThaiDistribute => {
      justification.adjust = ParagraphAdjust::Block;
      if import_settings.justify_lines_with_shrinking {
        justification.word_spacing.minimum_pct = 75;
        justification.word_spacing.maximum_pct = 133;
      }
    }
    w::JustificationValues::LowKashida => {
      justification.adjust = ParagraphAdjust::Block;
      justification.word_spacing = JustificationWordSpacing {
        desired_pct: 133,
        minimum_pct: 133,
        maximum_pct: 133,
      };
    }
    w::JustificationValues::MediumKashida => {
      justification.adjust = ParagraphAdjust::Block;
      justification.word_spacing = JustificationWordSpacing {
        desired_pct: 200,
        minimum_pct: 200,
        maximum_pct: 200,
      };
    }
    w::JustificationValues::HighKashida => {
      justification.adjust = ParagraphAdjust::Block;
      justification.word_spacing = JustificationWordSpacing {
        desired_pct: 300,
        minimum_pct: 300,
        maximum_pct: 300,
      };
    }
    w::JustificationValues::Left
    | w::JustificationValues::Start
    | w::JustificationValues::NumTab => {
      justification.adjust = if import_settings.use_literal_direction {
        if import_settings.exchange_left_right {
          ParagraphAdjust::Right
        } else {
          ParagraphAdjust::Left
        }
      } else {
        ParagraphAdjust::Start
      };
    }
  }
  justification
}

fn merge_paragraph_frame_properties(format: &mut ParagraphFormat, frame: &w::FrameProperties) {
  if matches!(frame.y_align, Some(w::VerticalAlignmentValues::Inline)) {
    format.frame = None;
    return;
  }

  let Some(mut merged) = format.frame else {
    format.frame = Some(paragraph_frame_properties(frame));
    return;
  };

  if frame.width.is_some() {
    merged.width_pt = frame.width.as_ref().and_then(twips_measure_to_points);
  }
  if frame.height.is_some() {
    merged.height_pt = frame.height.as_ref().and_then(twips_measure_to_points);
  }
  if frame.height_type.is_some() {
    merged.height_rule = frame_height_rule(frame.height_type, merged.height_pt);
  }
  if frame.horizontal_position.is_some() {
    merged.placement.horizontal_anchor = frame_horizontal_anchor(frame.horizontal_position);
  }
  if frame.vertical_position.is_some() {
    merged.placement.vertical_anchor = frame_vertical_anchor(frame.vertical_position);
  }
  if let Some(alignment) = frame.x_align {
    merged.placement.horizontal_alignment = Some(frame_horizontal_alignment(alignment));
  }
  if let Some(alignment) = frame.y_align {
    merged.placement.vertical_alignment = Some(frame_vertical_alignment(alignment));
  }
  if frame.x.is_some() {
    merged.placement.horizontal_offset_pt = frame
      .x
      .as_ref()
      .and_then(signed_twips_measure_to_points)
      .unwrap_or(0.0);
  }
  if frame.y.is_some() {
    merged.placement.vertical_offset_pt = frame
      .y
      .as_ref()
      .and_then(signed_twips_measure_to_points)
      .unwrap_or(0.0);
    merged.placement.vertical_offset_explicit = true;
  }
  if frame.wrap.is_some() {
    merged.placement.wrap = frame_wrap_mode(frame.wrap);
  }
  if frame.drop_cap.is_some() {
    merged.drop_cap = matches!(
      frame.drop_cap,
      Some(w::DropCapLocationValues::Drop | w::DropCapLocationValues::Margin)
    );
  }
  if frame.horizontal_space.is_some() {
    let horizontal_space = frame
      .horizontal_space
      .as_ref()
      .and_then(twips_measure_to_points)
      .unwrap_or(0.0);
    merged.placement.margin_right_pt = horizontal_space;
    merged.placement.margin_left_pt = horizontal_space;
  }
  if frame.vertical_space.is_some() {
    let vertical_space = frame
      .vertical_space
      .as_ref()
      .and_then(twips_measure_to_points)
      .unwrap_or(0.0);
    merged.placement.margin_top_pt = vertical_space;
    merged.placement.margin_bottom_pt = vertical_space;
  }
  format.frame = Some(merged);
}

fn paragraph_frame_properties(frame: &w::FrameProperties) -> ParagraphFrameProperties {
  let height_pt = frame.height.as_ref().and_then(twips_measure_to_points);
  let horizontal_space = frame
    .horizontal_space
    .as_ref()
    .and_then(twips_measure_to_points)
    .unwrap_or(0.0);
  let vertical_space = frame
    .vertical_space
    .as_ref()
    .and_then(twips_measure_to_points)
    .unwrap_or(0.0);
  ParagraphFrameProperties {
    width_pt: frame.width.as_ref().and_then(twips_measure_to_points),
    height_pt,
    height_rule: frame_height_rule(frame.height_type, height_pt),
    drop_cap: matches!(
      frame.drop_cap,
      Some(w::DropCapLocationValues::Drop | w::DropCapLocationValues::Margin)
    ),
    placement: FloatingFramePlacement {
      horizontal_anchor: frame_horizontal_anchor(frame.horizontal_position),
      vertical_anchor: frame_vertical_anchor(frame.vertical_position),
      horizontal_alignment: frame.x_align.map(frame_horizontal_alignment),
      vertical_alignment: frame.y_align.map(frame_vertical_alignment),
      horizontal_offset_pt: frame
        .x
        .as_ref()
        .and_then(signed_twips_measure_to_points)
        .unwrap_or(0.0),
      vertical_offset_pt: frame
        .y
        .as_ref()
        .and_then(signed_twips_measure_to_points)
        .unwrap_or(0.0),
      vertical_offset_explicit: frame.y.is_some(),
      wrap: frame_wrap_mode(frame.wrap),
      margin_top_pt: vertical_space,
      margin_right_pt: horizontal_space,
      margin_bottom_pt: vertical_space,
      margin_left_pt: horizontal_space,
    },
  }
}

fn apply_tab_stops(format: &mut ParagraphFormat, tabs: &w::Tabs) {
  for tab in &tabs.tab_stop {
    let Some(position_pt) = signed_twips_measure_to_points(&tab.position)
      .filter(|position| position.is_finite() && *position >= 0.0)
    else {
      continue;
    };
    if matches!(tab.val, w::TabStopValues::Clear) {
      format
        .tab_stops
        .retain(|stop| (stop.position_pt - position_pt).abs() >= TAB_STOP_DEDUP_EPSILON_PT);
      if !format
        .tab_stop_clear_positions_pt
        .iter()
        .any(|clear| (*clear - position_pt).abs() < TAB_STOP_DEDUP_EPSILON_PT)
      {
        format.tab_stop_clear_positions_pt.push(position_pt);
      }
      continue;
    }
    let alignment = match tab.val {
      w::TabStopValues::Left | w::TabStopValues::Start | w::TabStopValues::Decimal => {
        TabStopAlignment::Left
      }
      w::TabStopValues::Center => TabStopAlignment::Center,
      w::TabStopValues::Right | w::TabStopValues::End | w::TabStopValues::Number => {
        TabStopAlignment::Right
      }
      w::TabStopValues::Clear | w::TabStopValues::Bar => continue,
    };
    format
      .tab_stop_clear_positions_pt
      .retain(|clear| (*clear - position_pt).abs() >= TAB_STOP_DEDUP_EPSILON_PT);
    if let Some(existing) = format
      .tab_stops
      .iter_mut()
      .find(|stop| (stop.position_pt - position_pt).abs() < TAB_STOP_DEDUP_EPSILON_PT)
    {
      existing.alignment = alignment;
      existing.leader = tab.leader.map(tab_leader).unwrap_or_default();
    } else {
      format.tab_stops.push(TabStop {
        position_pt,
        alignment,
        leader: tab.leader.map(tab_leader).unwrap_or_default(),
      });
    }
  }
  format
    .tab_stops
    .sort_by(|a, b| a.position_pt.total_cmp(&b.position_pt));
  format
    .tab_stops
    .dedup_by(|a, b| (a.position_pt - b.position_pt).abs() < TAB_STOP_DEDUP_EPSILON_PT);
  format.tab_stop_clear_positions_pt.sort_by(f32::total_cmp);
  format
    .tab_stop_clear_positions_pt
    .dedup_by(|a, b| (*a - *b).abs() < TAB_STOP_DEDUP_EPSILON_PT);
}

fn merge_tab_stop_values(target: &mut ParagraphFormat, values: &ParagraphFormat) {
  for clear in &values.tab_stop_clear_positions_pt {
    target
      .tab_stops
      .retain(|stop| (stop.position_pt - clear).abs() >= TAB_STOP_DEDUP_EPSILON_PT);
    if !target
      .tab_stop_clear_positions_pt
      .iter()
      .any(|existing| (*existing - clear).abs() < TAB_STOP_DEDUP_EPSILON_PT)
    {
      target.tab_stop_clear_positions_pt.push(*clear);
    }
  }
  for stop in &values.tab_stops {
    target
      .tab_stop_clear_positions_pt
      .retain(|clear| (*clear - stop.position_pt).abs() >= TAB_STOP_DEDUP_EPSILON_PT);
    if let Some(existing) = target
      .tab_stops
      .iter_mut()
      .find(|existing| (existing.position_pt - stop.position_pt).abs() < TAB_STOP_DEDUP_EPSILON_PT)
    {
      *existing = *stop;
    } else {
      target.tab_stops.push(*stop);
    }
  }
  target
    .tab_stops
    .sort_by(|a, b| a.position_pt.total_cmp(&b.position_pt));
  target
    .tab_stops
    .dedup_by(|a, b| (a.position_pt - b.position_pt).abs() < TAB_STOP_DEDUP_EPSILON_PT);
  target.tab_stop_clear_positions_pt.sort_by(f32::total_cmp);
  target
    .tab_stop_clear_positions_pt
    .dedup_by(|a, b| (*a - *b).abs() < TAB_STOP_DEDUP_EPSILON_PT);
  target.tab_stops_set = true;
}

fn tab_leader(leader: w::TabStopLeaderCharValues) -> TabLeader {
  match leader {
    w::TabStopLeaderCharValues::Dot => TabLeader::Dot,
    w::TabStopLeaderCharValues::Hyphen => TabLeader::Hyphen,
    w::TabStopLeaderCharValues::Underscore => TabLeader::Underscore,
    w::TabStopLeaderCharValues::Heavy => TabLeader::Heavy,
    w::TabStopLeaderCharValues::MiddleDot => TabLeader::MiddleDot,
    w::TabStopLeaderCharValues::None => TabLeader::None,
  }
}

fn positional_tab(tab: &w::PositionalTab, style: TextStyle) -> PositionalTab {
  PositionalTab {
    alignment: match tab.alignment {
      w::AbsolutePositionTabAlignmentValues::Left => TabStopAlignment::Left,
      w::AbsolutePositionTabAlignmentValues::Center => TabStopAlignment::Center,
      w::AbsolutePositionTabAlignmentValues::Right => TabStopAlignment::Right,
    },
    relative_to: match tab.relative_to {
      w::AbsolutePositionTabPositioningBaseValues::Margin => PositionalTabBase::Margin,
      w::AbsolutePositionTabPositioningBaseValues::Indent => PositionalTabBase::Indent,
    },
    leader: match tab.leader {
      w::AbsolutePositionTabLeaderCharValues::None => TabLeader::None,
      w::AbsolutePositionTabLeaderCharValues::Dot => TabLeader::Dot,
      w::AbsolutePositionTabLeaderCharValues::Hyphen => TabLeader::Hyphen,
      w::AbsolutePositionTabLeaderCharValues::Underscore => TabLeader::Underscore,
      w::AbsolutePositionTabLeaderCharValues::MiddleDot => TabLeader::MiddleDot,
    },
    style,
  }
}

#[cfg(test)]
fn paragraph_inlines(
  paragraph: &w::Paragraph,
  base_style: TextStyle,
  styles: &StylesCatalog,
  images: &ImageCatalog,
  hyperlinks: &HyperlinkCatalog,
  custom_xml_bindings: &CustomXmlBindings,
  form_widget_ids: &mut FormWidgetIdAllocator,
) -> Vec<InlineItem> {
  paragraph_inlines_with_policy(
    paragraph,
    base_style,
    styles,
    images,
    hyperlinks,
    ParagraphInlineImport {
      custom_xml_bindings,
      form_widget_ids,
      suppress_toc_hyperlink_style: false,
    },
  )
}

struct ParagraphInlineImport<'a> {
  custom_xml_bindings: &'a CustomXmlBindings,
  form_widget_ids: &'a mut FormWidgetIdAllocator,
  suppress_toc_hyperlink_style: bool,
}

fn paragraph_inlines_with_policy(
  paragraph: &w::Paragraph,
  base_style: TextStyle,
  styles: &StylesCatalog,
  images: &ImageCatalog,
  hyperlinks: &HyperlinkCatalog,
  import: ParagraphInlineImport<'_>,
) -> Vec<InlineItem> {
  let ParagraphInlineImport {
    custom_xml_bindings,
    form_widget_ids,
    suppress_toc_hyperlink_style,
  } = import;
  let mut inlines = Vec::new();
  let mut inline_context = InlineImportContext {
    styles,
    images,
    hyperlinks,
    custom_xml_bindings,
    form_widget_ids,
    suppress_toc_hyperlink_style,
  };
  let mut complex_fields = Vec::new();

  for choice in &paragraph.paragraph_choice {
    match choice {
      w::ParagraphChoice::WRun(run) => {
        push_run_or_complex_field(
          run,
          &mut inlines,
          base_style.clone(),
          RunImportContext {
            styles,
            images,
            hyperlinks,
            suppress_toc_hyperlink_style,
          },
          None,
          &mut complex_fields,
        );
      }
      w::ParagraphChoice::SimpleField(field) => {
        push_simple_field(field, &mut inlines, base_style.clone(), &mut inline_context);
      }
      w::ParagraphChoice::Hyperlink(hyperlink) => {
        push_hyperlink_content(
          hyperlink.as_ref(),
          &mut inlines,
          base_style.clone(),
          None,
          &mut inline_context,
          &mut complex_fields,
        );
      }
      w::ParagraphChoice::CustomXmlRun(custom_xml)
      | w::ParagraphChoice::SmartTagRun(custom_xml) => push_custom_xml_run(
        custom_xml,
        &mut inlines,
        base_style.clone(),
        None,
        &mut inline_context,
        &mut complex_fields,
      ),
      w::ParagraphChoice::BookmarkStart(bookmark) => {
        let name = bookmark.name.as_str();
        if !name.is_empty() {
          inlines.push(InlineItem::BookmarkStart(name.to_string()));
        }
      }
      w::ParagraphChoice::InsertedRun(inserted) => {
        push_inserted_run_or_complex_field(
          inserted,
          &mut inlines,
          base_style.clone(),
          RunImportContext {
            styles,
            images,
            hyperlinks,
            suppress_toc_hyperlink_style,
          },
          None,
          &mut complex_fields,
        );
      }
      w::ParagraphChoice::DeletedRun(deleted) => {
        push_deleted_run(
          deleted,
          &mut inlines,
          base_style.clone(),
          styles,
          images,
          hyperlinks,
          None,
        );
      }
      w::ParagraphChoice::MoveFromRun(moved) => {
        push_move_from_run(
          moved,
          &mut inlines,
          base_style.clone(),
          styles,
          images,
          hyperlinks,
          None,
        );
      }
      w::ParagraphChoice::MoveToRun(moved) => {
        push_move_to_run_or_complex_field(
          moved,
          &mut inlines,
          base_style.clone(),
          RunImportContext {
            styles,
            images,
            hyperlinks,
            suppress_toc_hyperlink_style,
          },
          None,
          &mut complex_fields,
        );
      }
      w::ParagraphChoice::SdtRun(sdt) => push_sdt_run(
        sdt,
        &mut inlines,
        base_style.clone(),
        None,
        &mut inline_context,
      ),
      w::ParagraphChoice::AlternateContent(_) => {}
      w::ParagraphChoice::Break(br) => {
        let run = w::Run {
          run_choice: vec![w::RunChoice::Break(br.clone())],
          ..Default::default()
        };
        push_run(
          &run,
          &mut inlines,
          base_style.clone(),
          styles,
          images,
          hyperlinks,
          None,
        );
      }
      choice => {
        if let Some(text) = shared_math::wordprocessing_math_text(choice)
          && !text.is_empty()
        {
          let mut style = base_style.clone();
          style = properties::run_style(
            shared_math::wordprocessing_math_run_properties(choice),
            style,
            styles,
          );
          // ECMA-376 Part 1 §22.1.2.61 makes m:mathFont the document-wide
          // typeface for mathematical text. Per-run w:rPr still supplies
          // size, color, and other character properties, but its ordinary
          // rFonts slot does not replace the math font.
          style.font_family = Some(
            styles
              .math_font_family
              .clone()
              .unwrap_or_else(|| Arc::from("Cambria Math")),
          );
          inlines.push(InlineItem::Text(TextRun {
            text,
            style,
            hyperlink_url: None,
            dynamic_field: None,
            style_ref_keys: Vec::new(),
            style_ref_text: None,
            style_ref_numbering_text: None,
            preserve_text_portion: false,
          }));
        }
      }
    }
  }
  flush_unclosed_complex_fields(&mut inlines, &mut complex_fields, styles);

  inlines
}

fn math_paragraph_alignment(
  paragraph: &w::Paragraph,
  display_math_alignment: Option<ParagraphAlignment>,
) -> Option<ParagraphAlignment> {
  let explicit = paragraph.paragraph_choice.iter().find_map(|choice| {
    let w::ParagraphChoice::Paragraph(math_paragraph) = choice else {
      return None;
    };
    let justification = math_paragraph
      .paragraph_properties
      .as_deref()
      .and_then(|properties| properties.justification.as_ref())
      .map(|justification| justification.val)
      .unwrap_or(ooxmlsdk::schemas::m::JustificationValues::CenterGroup);
    Some(math_justification_alignment(justification))
  });
  explicit.or_else(|| {
    (paragraph
      .paragraph_choice
      .iter()
      .any(|choice| shared_math::wordprocessing_math_text(choice).is_some())
      && paragraph.paragraph_choice.iter().all(|choice| {
        shared_math::wordprocessing_math_text(choice).is_some()
          || matches!(choice, w::ParagraphChoice::WRun(run) if run.run_choice.is_empty())
      }))
    .then_some(display_math_alignment)
    .flatten()
  })
}

fn math_justification_alignment(
  justification: ooxmlsdk::schemas::m::JustificationValues,
) -> ParagraphAlignment {
  match justification {
    ooxmlsdk::schemas::m::JustificationValues::Left => ParagraphAlignment::Left,
    ooxmlsdk::schemas::m::JustificationValues::Right => ParagraphAlignment::Right,
    ooxmlsdk::schemas::m::JustificationValues::Center
    | ooxmlsdk::schemas::m::JustificationValues::CenterGroup => ParagraphAlignment::Center,
  }
}

#[derive(Clone, Debug)]
struct ComplexFieldState {
  instr: String,
  result: Vec<InlineItem>,
  form_drop_down_value: Option<String>,
  form_date_time_tokens: Option<Vec<String>>,
  field_locked: bool,
  in_result: bool,
  style: TextStyle,
  hyperlink_url: Option<String>,
}

#[derive(Clone, Copy)]
struct RunImportContext<'a> {
  styles: &'a StylesCatalog,
  images: &'a ImageCatalog,
  hyperlinks: &'a HyperlinkCatalog,
  suppress_toc_hyperlink_style: bool,
}

struct InlineImportContext<'a> {
  styles: &'a StylesCatalog,
  images: &'a ImageCatalog,
  hyperlinks: &'a HyperlinkCatalog,
  custom_xml_bindings: &'a CustomXmlBindings,
  form_widget_ids: &'a mut FormWidgetIdAllocator,
  suppress_toc_hyperlink_style: bool,
}

fn push_run_or_complex_field(
  run: &w::Run,
  inlines: &mut Vec<InlineItem>,
  base_style: TextStyle,
  context: RunImportContext<'_>,
  hyperlink_url: Option<&str>,
  fields: &mut Vec<ComplexFieldState>,
) {
  if fields.is_empty() && !run_starts_complex_field(run) {
    push_run_with_character_style_policy(
      run,
      inlines,
      base_style,
      context,
      hyperlink_url,
      !context.suppress_toc_hyperlink_style,
    );
    return;
  }

  let suppress_toc_hyperlink_style = context.suppress_toc_hyperlink_style
    || fields.iter().any(|field| {
      field.in_result && field_instruction_name(&field.instr).is_some_and(|name| name == "TOC")
    });
  let style = if suppress_toc_hyperlink_style {
    properties::run_style_without_hyperlink_character_style(
      run.run_properties.as_deref(),
      base_style.clone(),
      context.styles,
    )
  } else {
    properties::run_style(
      run.run_properties.as_deref(),
      base_style.clone(),
      context.styles,
    )
  };
  for choice in &run.run_choice {
    match choice {
      w::RunChoice::FieldChar(field_char)
        if field_char.field_char_type == w::FieldCharValues::Begin =>
      {
        fields.push(ComplexFieldState {
          instr: String::new(),
          result: Vec::new(),
          form_drop_down_value: form_drop_down_value(field_char),
          form_date_time_tokens: form_date_time_tokens(field_char),
          field_locked: field_char
            .field_lock
            .is_some_and(ooxmlsdk::simple_type::OnOffValue::as_bool),
          in_result: false,
          style: style.clone(),
          hyperlink_url: hyperlink_url.map(ToString::to_string),
        });
      }
      w::RunChoice::FieldChar(field_char)
        if field_char.field_char_type == w::FieldCharValues::Separate =>
      {
        if let Some(field) = fields.last_mut() {
          field.in_result = true;
        }
      }
      w::RunChoice::FieldChar(field_char)
        if field_char.field_char_type == w::FieldCharValues::End =>
      {
        flush_complex_field(inlines, fields, true, context.styles);
      }
      w::RunChoice::FieldCode(code) => {
        if let Some(field) = fields.last_mut()
          && !field.in_result
          && let Some(content) = word_text_value(code, context.styles.preserve_word_text_whitespace)
        {
          field.instr.push_str(content);
        }
      }
      _ => {
        if let Some(field) = fields.last_mut()
          && field.in_result
        {
          push_run_with_character_style_policy(
            run,
            &mut field.result,
            base_style.clone(),
            context,
            hyperlink_url,
            !suppress_toc_hyperlink_style,
          );
          break;
        }
      }
    }
  }
}

fn run_starts_complex_field(run: &w::Run) -> bool {
  run.run_choice.iter().any(|choice| {
    matches!(
      choice,
      w::RunChoice::FieldChar(field_char)
        if field_char.field_char_type == w::FieldCharValues::Begin
    )
  })
}

fn flush_complex_field(
  inlines: &mut Vec<InlineItem>,
  fields: &mut Vec<ComplexFieldState>,
  closed: bool,
  styles: &StylesCatalog,
) {
  let Some(state) = fields.pop() else {
    return;
  };
  let field_hyperlink_url = closed
    .then(|| complex_field_hyperlink_url(&state.instr))
    .flatten();
  let mut resolved = Vec::new();
  if closed && state.instr.trim().is_empty() {
    // A separator does not make a useful field without field-code content.
    // Word drops the cached result of that closed malformed field. Keep the
    // ECMA-376 §17.16.18 recovery below for an unclosed field, whose result
    // must instead be interpreted as literal text.
  } else if closed && state.field_locked {
    // ECMA-376 Part 1 §17.16.18: fldLock on the begin character prevents
    // recalculation even when an application explicitly requests an update.
    // The persisted result is therefore authoritative.
    resolved = state.result;
  } else if closed && field_instruction_name(&state.instr).is_some_and(|name| name == "SET") {
    // ECMA-376 §17.16.5.57 defines SET as assigning a bookmark and gives it
    // no field value. Its cached result is metadata, not visible document text.
  } else if closed
    && let Some(text) =
      refreshed_form_date_time_field(state.form_date_time_tokens.as_deref(), &state.style, styles)
  {
    let style = field_result_style(&state.result).unwrap_or(state.style);
    push_resolved_field_text(&mut resolved, text, style, state.hyperlink_url.as_deref());
  } else if closed && let Some(text) = refreshed_date_time_field(&state.instr, &state.style, styles)
  {
    let style = field_result_style(&state.result).unwrap_or(state.style);
    push_resolved_field_text(&mut resolved, text, style, state.hyperlink_url.as_deref());
  } else if let Some(kind) = dynamic_field_kind(&state.instr) {
    if let DynamicFieldKind::StyleRef { style_name, .. } = &kind
      && styles.style_ref_name_requires_localized_error(style_name)
    {
      push_localized_missing_style_ref(
        &mut resolved,
        style_name,
        state.style,
        state.hyperlink_url.as_deref(),
      );
    } else {
      // ECMA-376 Part 1 §17.16.4.3.3 makes \* MERGEFORMAT preserve the
      // existing field-result run/paragraph structure when an application
      // replaces the result text.  In particular, the generated PAGE value
      // must retain direct/theme formatting authored on its cached result
      // rather than inheriting the field-begin character's formatting.
      let style = if field_uses_merge_format(&state.instr) {
        field_result_style(&state.result).unwrap_or(state.style)
      } else {
        state.style
      };
      push_dynamic_field(
        &mut resolved,
        kind,
        style,
        state.hyperlink_url.as_deref(),
        field_result_text(&state.result),
      );
    }
  } else if state.result.is_empty()
    && fields.is_empty()
    && let Some(run) = symbol_field_run(
      &state.instr,
      state.style.clone(),
      state.hyperlink_url.as_deref(),
    )
  {
    resolved.push(InlineItem::Text(run));
  } else if state.result.is_empty()
    && field_instruction_name(&state.instr).is_some_and(|name| name == "FORMDROPDOWN")
    && let Some(value) = state.form_drop_down_value
  {
    resolved.push(InlineItem::Text(TextRun {
      text: value,
      style: state.style,
      hyperlink_url: state.hyperlink_url,
      dynamic_field: None,
      style_ref_keys: Vec::new(),
      style_ref_text: None,
      style_ref_numbering_text: None,
      preserve_text_portion: false,
    }));
  } else {
    resolved = state.result;
  }
  if let Some(url) = field_hyperlink_url.as_deref() {
    apply_field_hyperlink_url(&mut resolved, url);
  }
  if let Some(parent) = fields.last_mut() {
    if parent.in_result {
      parent.result.extend(resolved);
    } else if parent.form_date_time_tokens.is_none() {
      inlines.extend(resolved);
    }
  } else {
    inlines.extend(resolved);
  }
}

fn field_instruction_name(instr: &str) -> Option<String> {
  field_instruction_tokens(instr)
    .first()
    .map(|name| name.to_ascii_uppercase())
}

fn complex_field_hyperlink_url(instr: &str) -> Option<String> {
  let tokens = field_instruction_tokens(instr);
  if !tokens
    .first()
    .is_some_and(|name| name.eq_ignore_ascii_case("HYPERLINK"))
  {
    return None;
  }
  let mut target = None;
  let mut anchor = None;
  let mut index = 1;
  while index < tokens.len() {
    let token = &tokens[index];
    if token.eq_ignore_ascii_case(r"\l") {
      anchor = tokens
        .get(index + 1)
        .filter(|value| !value.is_empty())
        .cloned();
      index += 2;
    } else if token.eq_ignore_ascii_case(r"\o") || token.eq_ignore_ascii_case(r"\t") {
      index += 2;
    } else if token.starts_with('\\') {
      index += 1;
    } else {
      if target.is_none() && !token.is_empty() {
        target = Some(token.clone());
      }
      index += 1;
    }
  }
  match (target, anchor) {
    (Some(mut target), Some(anchor)) => {
      target.push('#');
      target.push_str(&anchor);
      Some(target)
    }
    (Some(target), None) => Some(target),
    (None, Some(anchor)) => Some(format!("ooxmlsdk-pdf:bookmark:{anchor}")),
    (None, None) => None,
  }
}

fn apply_field_hyperlink_url(result: &mut [InlineItem], url: &str) {
  for item in result {
    match item {
      InlineItem::Text(run) => {
        run.hyperlink_url.get_or_insert_with(|| url.to_string());
      }
      InlineItem::PositionalTab(_) => {}
      InlineItem::Ruby(ruby) => {
        for run in ruby.base.iter_mut().chain(&mut ruby.guide) {
          run.hyperlink_url.get_or_insert_with(|| url.to_string());
        }
      }
      InlineItem::Image(_)
      | InlineItem::Shape(_)
      | InlineItem::DrawingGroupStart(_)
      | InlineItem::DrawingGroupEnd
      | InlineItem::BookmarkStart(_)
      | InlineItem::FormWidgetStart(_)
      | InlineItem::FormWidgetEnd(_)
      | InlineItem::LastRenderedPageBreak
      | InlineItem::PageBreak
      | InlineItem::ColumnBreak => {}
    }
  }
}

fn symbol_field_run(
  instr: &str,
  mut style: TextStyle,
  hyperlink_url: Option<&str>,
) -> Option<TextRun> {
  let tokens = field_instruction_tokens(instr);
  let name = tokens.first()?.trim_start_matches('\\');
  if !name.eq_ignore_ascii_case("SYMBOL") {
    return None;
  }
  let code = parse_symbol_field_code(tokens.get(1)?)?;
  let mut font = None;
  let mut ansi = false;
  let mut shift_jis = false;
  let mut unicode = false;
  let mut index = 2;
  while index < tokens.len() {
    let switch = tokens[index].strip_prefix('\\');
    match switch {
      Some(value) if value.eq_ignore_ascii_case("f") => {
        index += 1;
        font = tokens.get(index).filter(|value| !value.is_empty()).cloned();
      }
      Some(value) if value.eq_ignore_ascii_case("s") => {
        index += 1;
        let size = tokens.get(index)?.parse::<f32>().ok()?;
        if size <= 0.0 {
          return None;
        }
        style.font_size_pt = size;
        style.complex_font_size_pt = Some(size);
      }
      Some(value) if value.eq_ignore_ascii_case("a") => ansi = true,
      Some(value) if value.eq_ignore_ascii_case("j") => shift_jis = true,
      Some(value) if value.eq_ignore_ascii_case("u") => unicode = true,
      // The \h switch requires a non-line-height-bearing text portion. Keep
      // unsupported instructions cached-only rather than silently changing
      // paragraph geometry.
      Some(value) if value.eq_ignore_ascii_case("h") => return None,
      _ => {}
    }
    index += 1;
  }

  let character = if unicode {
    // [MS-OI29500] §2.1.489: Word truncates values outside plane zero.
    char::from_u32(code & 0xFFFF)?
  } else if shift_jis {
    decode_symbol_field_character(encoding_rs::SHIFT_JIS, code)?
  } else if ansi {
    decode_symbol_field_character(encoding_rs::WINDOWS_1252, code)?
  } else {
    let character = shared_symbol::font_symbol_code(font.as_deref(), code)?;
    if font.as_deref().is_some_and(symbol_transport_font) && code <= u32::from(u8::MAX) {
      char::from_u32(0xF000 | code)?
    } else {
      character
    }
  };

  if let Some(font) = font {
    let font = Arc::<str>::from(font);
    style.font_family = Some(font.clone());
    style.east_asia_font_family = Some(font.clone());
    style.complex_font_family = Some(font.clone());
    style.symbol_font_family = Some(font);
  }

  Some(TextRun {
    text: character.to_string(),
    style,
    hyperlink_url: hyperlink_url.map(ToString::to_string),
    dynamic_field: None,
    style_ref_keys: Vec::new(),
    style_ref_text: None,
    style_ref_numbering_text: None,
    preserve_text_portion: false,
  })
}

fn symbol_transport_font(font: &str) -> bool {
  font.eq_ignore_ascii_case("Symbol")
    || font
      .get(font.len().saturating_sub(" symbol".len())..)
      .is_some_and(|suffix| suffix.eq_ignore_ascii_case(" symbol"))
    || font.to_ascii_lowercase().contains("wingdings")
}

fn parse_symbol_field_code(value: &str) -> Option<u32> {
  value
    .strip_prefix("0x")
    .or_else(|| value.strip_prefix("0X"))
    .map_or_else(
      || value.parse::<u32>().ok(),
      |hex| u32::from_str_radix(hex, 16).ok(),
    )
}

fn decode_symbol_field_character(
  encoding: &'static encoding_rs::Encoding,
  code: u32,
) -> Option<char> {
  let bytes = if code <= u32::from(u8::MAX) {
    vec![code as u8]
  } else if code <= u32::from(u16::MAX) {
    vec![(code >> 8) as u8, code as u8]
  } else {
    return None;
  };
  let (text, had_errors) = encoding.decode_without_bom_handling(&bytes);
  if had_errors {
    return None;
  }
  let mut characters = text.chars();
  let character = characters.next()?;
  characters.next().is_none().then_some(character)
}

fn form_drop_down_value(field_char: &w::FieldChar) -> Option<String> {
  let w::FieldCharChoice::FormFieldData(form_field) = field_char.field_char_choice.as_ref()? else {
    return None;
  };
  let drop_down = form_field
    .form_field_data_choice
    .iter()
    .find_map(|choice| match choice {
      w::FormFieldDataChoice::DropDownListFormField(drop_down) => Some(drop_down.as_ref()),
      _ => None,
    })?;
  let entries = &drop_down.list_entry_form_field;
  let selected_index = drop_down
    .drop_down_list_selection
    .as_ref()
    .map(|selection| selection.val)
    .filter(|index| usize::try_from(*index).is_ok_and(|index| index < entries.len()))
    .or_else(|| {
      drop_down
        .default_drop_down_list_item_index
        .as_ref()
        .map(|default| default.val)
        .filter(|index| usize::try_from(*index).is_ok_and(|index| index < entries.len()))
    })
    .unwrap_or(0);
  entries
    .get(usize::try_from(selected_index).ok()?)
    .map(|entry| entry.val.to_string())
}

fn form_date_time_tokens(field_char: &w::FieldChar) -> Option<Vec<String>> {
  let w::FieldCharChoice::FormFieldData(form_field) = field_char.field_char_choice.as_ref()? else {
    return None;
  };
  let text_input = form_field
    .form_field_data_choice
    .iter()
    .find_map(|choice| match choice {
      w::FormFieldDataChoice::TextInput(text_input) => Some(text_input.as_ref()),
      _ => None,
    })?;
  let field_name = match text_input.text_box_form_field_type.as_ref()?.val {
    w::TextBoxFormFieldValues::CurrentDate => "DATE",
    w::TextBoxFormFieldValues::CurrentTime => "TIME",
    _ => return None,
  };
  let mut tokens = vec![field_name.to_string()];
  if let Some(picture) = text_input.format.as_ref() {
    tokens.push(r"\@".to_string());
    tokens.push(picture.val.to_string());
  }
  Some(tokens)
}

fn flush_unclosed_complex_fields(
  inlines: &mut Vec<InlineItem>,
  fields: &mut Vec<ComplexFieldState>,
  styles: &StylesCatalog,
) {
  while !fields.is_empty() {
    flush_complex_field(inlines, fields, false, styles);
  }
}

fn dynamic_field_kind(instr: &str) -> Option<DynamicFieldKind> {
  let tokens = field_instruction_tokens(instr);
  let name = field_instruction_name(instr)?;
  match name.as_str() {
    "PAGE" => Some(DynamicFieldKind::Page {
      number_format: field_number_format(&tokens[1..]).unwrap_or(FieldNumberFormat::PageStyle),
    }),
    "NUMPAGES" => Some(DynamicFieldKind::NumPages {
      number_format: field_number_format(&tokens[1..]).unwrap_or(FieldNumberFormat::Decimal),
    }),
    "PAGEREF" => page_ref_field_kind(&tokens[1..]),
    "STYLEREF" => style_ref_field_kind(&tokens[1..]),
    _ => None,
  }
}

fn refreshed_date_time_field(
  instr: &str,
  style: &TextStyle,
  styles: &StylesCatalog,
) -> Option<String> {
  let value = styles.import_settings.field_update_datetime?;
  let tokens = field_instruction_tokens(instr);
  field_datetime::format_date_time_field(&tokens, style.language.as_deref(), value)
}

fn refreshed_form_date_time_field(
  tokens: Option<&[String]>,
  style: &TextStyle,
  styles: &StylesCatalog,
) -> Option<String> {
  field_datetime::format_date_time_field(
    tokens?,
    style.language.as_deref(),
    styles.import_settings.field_update_datetime?,
  )
}

fn field_number_format(tokens: &[String]) -> Option<FieldNumberFormat> {
  tokens.windows(2).find_map(|tokens| {
    tokens[0]
      .eq_ignore_ascii_case(r"\*")
      .then(|| match tokens[1].as_str() {
        value if value.eq_ignore_ascii_case("roman") => {
          if value == "ROMAN" {
            Some(FieldNumberFormat::UpperRoman)
          } else {
            Some(FieldNumberFormat::LowerRoman)
          }
        }
        value if value.eq_ignore_ascii_case("alphabetic") => {
          if value == "ALPHABETIC" {
            Some(FieldNumberFormat::UpperLetter)
          } else {
            Some(FieldNumberFormat::LowerLetter)
          }
        }
        value if value.eq_ignore_ascii_case("arabic") => Some(FieldNumberFormat::Decimal),
        _ => None,
      })
      .flatten()
  })
}

fn field_uses_merge_format(instr: &str) -> bool {
  field_instruction_tokens(instr)
    .windows(2)
    .any(|tokens| tokens[0] == r"\*" && tokens[1].eq_ignore_ascii_case("MERGEFORMAT"))
}

fn format_field_number(value: usize, format: FieldNumberFormat) -> String {
  let value = i32::try_from(value).unwrap_or(i32::MAX);
  let format = match format {
    FieldNumberFormat::PageStyle => w::NumberFormatValues::Decimal,
    FieldNumberFormat::Decimal => w::NumberFormatValues::Decimal,
    FieldNumberFormat::LowerRoman => w::NumberFormatValues::LowerRoman,
    FieldNumberFormat::UpperRoman => w::NumberFormatValues::UpperRoman,
    FieldNumberFormat::LowerLetter => w::NumberFormatValues::LowerLetter,
    FieldNumberFormat::UpperLetter => w::NumberFormatValues::UpperLetter,
  };
  format_numbering_value(value, format, false)
}

fn page_ref_field_kind(tokens: &[String]) -> Option<DynamicFieldKind> {
  let mut bookmark_name = None;
  let mut relative_position = false;
  let mut skip_switch_argument = false;
  for token in tokens {
    if skip_switch_argument {
      skip_switch_argument = false;
      continue;
    }
    if let Some(switch) = token.strip_prefix('\\') {
      if switch.eq_ignore_ascii_case("p") {
        relative_position = true;
      } else if matches!(switch, "*" | "#" | "@") {
        skip_switch_argument = true;
      }
      continue;
    }
    if bookmark_name.is_none() {
      bookmark_name = Some(Arc::<str>::from(token.as_str()));
    }
  }
  bookmark_name.map(|bookmark_name| DynamicFieldKind::PageRef {
    bookmark_name,
    number_format: field_number_format(tokens).unwrap_or(FieldNumberFormat::PageStyle),
    relative_position,
  })
}

fn style_ref_field_kind(tokens: &[String]) -> Option<DynamicFieldKind> {
  let mut style_name = None;
  let mut from_bottom = false;
  let mut numbering_only = false;
  let mut suppress_non_numerical = false;
  let mut skip_switch_arg = false;
  for token in tokens {
    if skip_switch_arg {
      skip_switch_arg = false;
      continue;
    }
    if let Some(switch) = token.strip_prefix('\\') {
      if switch.eq_ignore_ascii_case("l") {
        from_bottom = true;
      } else if matches!(switch.to_ascii_lowercase().as_str(), "n" | "r" | "t" | "w") {
        // All four switches request a numbering result rather than paragraph
        // text. The retained numbering form includes the appropriate ancestor
        // context. Only \t preserves Word's allowed authored delimiters at the
        // result edges; \n, \r, and \w omit trailing list punctuation.
        numbering_only = true;
        suppress_non_numerical |= switch.eq_ignore_ascii_case("t");
      } else if switch.len() > 1 && switch.chars().all(|ch| ch.is_ascii_alphabetic()) {
        skip_switch_arg = true;
      } else if style_name.is_none() && switch.len() == 1 && switch.as_bytes()[0].is_ascii_digit() {
        style_name = Some(switch.to_string());
      }
      continue;
    }
    if style_name.is_none() {
      style_name = Some(token.clone());
    }
  }
  style_name.map(|style_name| DynamicFieldKind::StyleRef {
    style_name: Arc::<str>::from(style_name),
    from_bottom,
    numbering_only,
    suppress_non_numerical,
  })
}

fn field_instruction_tokens(instr: &str) -> Vec<String> {
  let mut tokens = Vec::new();
  let mut current = String::new();
  let mut quoted = false;
  for ch in instr.chars() {
    match ch {
      '"' => {
        if quoted || !current.is_empty() {
          tokens.push(std::mem::take(&mut current));
        }
        quoted = !quoted;
      }
      ch if ch.is_whitespace() && !quoted => {
        if !current.is_empty() {
          tokens.push(std::mem::take(&mut current));
        }
      }
      _ => current.push(ch),
    }
  }
  if !current.is_empty() {
    tokens.push(current);
  }
  tokens
}

fn push_dynamic_field(
  inlines: &mut Vec<InlineItem>,
  kind: DynamicFieldKind,
  style: TextStyle,
  hyperlink_url: Option<&str>,
  result_text: Option<String>,
) {
  inlines.push(InlineItem::Text(TextRun {
    text: result_text
      .filter(|text| !text.is_empty())
      .unwrap_or_else(|| "1".to_string()),
    style,
    hyperlink_url: hyperlink_url.map(ToString::to_string),
    dynamic_field: Some(kind),
    style_ref_keys: Vec::new(),
    style_ref_text: None,
    style_ref_numbering_text: None,
    preserve_text_portion: false,
  }));
}

fn push_resolved_field_text(
  inlines: &mut Vec<InlineItem>,
  text: String,
  style: TextStyle,
  hyperlink_url: Option<&str>,
) {
  inlines.push(InlineItem::Text(TextRun {
    text,
    style,
    hyperlink_url: hyperlink_url.map(ToString::to_string),
    dynamic_field: None,
    style_ref_keys: Vec::new(),
    style_ref_text: None,
    style_ref_numbering_text: None,
    preserve_text_portion: false,
  }));
}

fn push_localized_missing_style_ref(
  inlines: &mut Vec<InlineItem>,
  style_name: &str,
  mut style: TextStyle,
  hyperlink_url: Option<&str>,
) {
  // Microsoft Word's Simplified Chinese STYLEREF diagnostic is a bold UI
  // resource. It uses DengXian for Chinese while preserving the field's Latin
  // face for the embedded style name and punctuation.
  style.east_asia_font_family = Some(office_default_font_family(Some("zh-CN")));
  style.east_asia_language = Some(Arc::<str>::from("zh-CN"));
  style.bold = true;
  style.complex_bold = Some(true);
  style.line_height_override_pt =
    Some(style.font_size_pt * WORD_ZH_STYLE_REF_ERROR_LINE_HEIGHT_PER_FONT_SIZE);
  inlines.push(InlineItem::Text(TextRun {
    text: format!("错误!使用“开始”选项卡将 {style_name} 应用于要在此处显示的文字。"),
    style,
    hyperlink_url: hyperlink_url.map(ToString::to_string),
    dynamic_field: None,
    style_ref_keys: Vec::new(),
    style_ref_text: None,
    style_ref_numbering_text: None,
    preserve_text_portion: false,
  }));
}

fn apply_generated_field_error_font(style: &mut TextStyle, ui_language: Option<&str>) {
  let language = ui_language
    .unwrap_or("en-US")
    .replace('_', "-")
    .to_ascii_lowercase();
  if language == "zh-cn" || language == "zh-sg" || language.starts_with("zh-hans") {
    // Word's localized field-error resource is emitted through the Simplified
    // Chinese East Asian slot. Keep the field's Latin face for punctuation,
    // but use the legacy Office SimSun resource face for Chinese glyphs.
    style.east_asia_font_family = Some(Arc::<str>::from("SimSun"));
    style.east_asia_language = Some(Arc::<str>::from("zh-CN"));
    // Direct outline effects and bold from the stale cached result do not
    // carry across Word's generated replacement. Its fill color and Latin
    // punctuation face do.
    style.outline_color = None;
    style.outline_opacity = 1.0;
    style.outline_width_pt = 0.0;
    style.bold = false;
    style.complex_bold = Some(false);
    if let Some(options) = style.pdf_glyph_outline_options.as_deref() {
      let mut options = options.clone();
      options.outline_fill = None;
      options.outline_stroke = None;
      style.pdf_glyph_outline_options = Some(Arc::new(options));
    }
  }
}

fn field_result_text(result: &[InlineItem]) -> Option<String> {
  let mut text = String::new();
  for item in result {
    match item {
      InlineItem::Text(run) => text.push_str(&run.text),
      InlineItem::PositionalTab(_) => text.push('\t'),
      InlineItem::Ruby(ruby) => {
        for run in &ruby.base {
          text.push_str(&run.text);
        }
      }
      InlineItem::PageBreak | InlineItem::ColumnBreak | InlineItem::LastRenderedPageBreak => {}
      InlineItem::DrawingGroupStart(_) | InlineItem::DrawingGroupEnd => {}
      InlineItem::Image(_)
      | InlineItem::Shape(_)
      | InlineItem::BookmarkStart(_)
      | InlineItem::FormWidgetStart(_)
      | InlineItem::FormWidgetEnd(_) => {}
    }
  }
  (!text.is_empty()).then_some(text)
}

fn hyperlink_url(hyperlink: &w::Hyperlink, hyperlinks: &HyperlinkCatalog) -> Option<String> {
  let target = hyperlink
    .id
    .as_deref()
    .and_then(|relationship_id| hyperlinks.target(relationship_id))
    .map(ToString::to_string);
  let anchor = hyperlink
    .anchor
    .as_deref()
    .filter(|anchor| !anchor.is_empty());
  match (target, anchor) {
    (Some(mut target), Some(anchor)) => {
      target.push('#');
      target.push_str(anchor);
      Some(target)
    }
    (Some(target), None) => Some(target),
    (None, Some(anchor)) => Some(format!("ooxmlsdk-pdf:bookmark:{anchor}")),
    (None, None) => None,
  }
}

fn push_hyperlink_content(
  hyperlink: &w::Hyperlink,
  inlines: &mut Vec<InlineItem>,
  base_style: TextStyle,
  inherited_url: Option<&str>,
  context: &mut InlineImportContext<'_>,
  complex_fields: &mut Vec<ComplexFieldState>,
) {
  let hyperlink_url = self::hyperlink_url(hyperlink, context.hyperlinks)
    .or_else(|| inherited_url.map(ToString::to_string));
  for item in &hyperlink.hyperlink_choice {
    match item {
      w::HyperlinkChoice::WRun(run) => push_run_or_complex_field(
        run,
        inlines,
        base_style.clone(),
        RunImportContext {
          styles: context.styles,
          images: context.images,
          hyperlinks: context.hyperlinks,
          suppress_toc_hyperlink_style: context.suppress_toc_hyperlink_style,
        },
        hyperlink_url.as_deref(),
        complex_fields,
      ),
      w::HyperlinkChoice::SimpleField(field) => {
        push_simple_field(field, inlines, base_style.clone(), context)
      }
      w::HyperlinkChoice::Hyperlink(nested) => push_hyperlink_content(
        nested,
        inlines,
        base_style.clone(),
        hyperlink_url.as_deref(),
        context,
        complex_fields,
      ),
      w::HyperlinkChoice::CustomXmlRun(custom_xml) => push_custom_xml_run(
        custom_xml,
        inlines,
        base_style.clone(),
        hyperlink_url.as_deref(),
        context,
        complex_fields,
      ),
      w::HyperlinkChoice::SdtRun(sdt) => push_sdt_run(
        sdt,
        inlines,
        base_style.clone(),
        hyperlink_url.as_deref(),
        context,
      ),
      w::HyperlinkChoice::InsertedRun(inserted) => push_inserted_run_or_complex_field(
        inserted,
        inlines,
        base_style.clone(),
        RunImportContext {
          styles: context.styles,
          images: context.images,
          hyperlinks: context.hyperlinks,
          suppress_toc_hyperlink_style: context.suppress_toc_hyperlink_style,
        },
        hyperlink_url.as_deref(),
        complex_fields,
      ),
      w::HyperlinkChoice::DeletedRun(deleted) => push_deleted_run(
        deleted,
        inlines,
        base_style.clone(),
        context.styles,
        context.images,
        context.hyperlinks,
        hyperlink_url.as_deref(),
      ),
      w::HyperlinkChoice::MoveFromRun(moved) => push_move_from_run(
        moved,
        inlines,
        base_style.clone(),
        context.styles,
        context.images,
        context.hyperlinks,
        hyperlink_url.as_deref(),
      ),
      w::HyperlinkChoice::MoveToRun(moved) => push_move_to_run_or_complex_field(
        moved,
        inlines,
        base_style.clone(),
        RunImportContext {
          styles: context.styles,
          images: context.images,
          hyperlinks: context.hyperlinks,
          suppress_toc_hyperlink_style: context.suppress_toc_hyperlink_style,
        },
        hyperlink_url.as_deref(),
        complex_fields,
      ),
      _ => {}
    }
  }
}

fn push_custom_xml_run(
  custom_xml: &w::CustomXmlRun,
  inlines: &mut Vec<InlineItem>,
  base_style: TextStyle,
  hyperlink_url: Option<&str>,
  context: &mut InlineImportContext<'_>,
  complex_fields: &mut Vec<ComplexFieldState>,
) {
  for choice in &custom_xml.custom_xml_run_choice {
    match choice {
      w::CustomXmlRunChoice::WRun(run) => push_run_or_complex_field(
        run,
        inlines,
        base_style.clone(),
        RunImportContext {
          styles: context.styles,
          images: context.images,
          hyperlinks: context.hyperlinks,
          suppress_toc_hyperlink_style: context.suppress_toc_hyperlink_style,
        },
        hyperlink_url,
        complex_fields,
      ),
      w::CustomXmlRunChoice::SimpleField(field) => {
        push_simple_field(field, inlines, base_style.clone(), context)
      }
      w::CustomXmlRunChoice::Hyperlink(hyperlink) => push_hyperlink_content(
        hyperlink,
        inlines,
        base_style.clone(),
        hyperlink_url,
        context,
        complex_fields,
      ),
      w::CustomXmlRunChoice::SdtRun(sdt) => {
        push_sdt_run(sdt, inlines, base_style.clone(), hyperlink_url, context)
      }
      w::CustomXmlRunChoice::CustomXmlRun(nested) | w::CustomXmlRunChoice::SmartTagRun(nested) => {
        push_custom_xml_run(
          nested,
          inlines,
          base_style.clone(),
          hyperlink_url,
          context,
          complex_fields,
        )
      }
      w::CustomXmlRunChoice::InsertedRun(inserted) => push_inserted_run_or_complex_field(
        inserted,
        inlines,
        base_style.clone(),
        RunImportContext {
          styles: context.styles,
          images: context.images,
          hyperlinks: context.hyperlinks,
          suppress_toc_hyperlink_style: context.suppress_toc_hyperlink_style,
        },
        hyperlink_url,
        complex_fields,
      ),
      w::CustomXmlRunChoice::DeletedRun(deleted) => push_deleted_run(
        deleted,
        inlines,
        base_style.clone(),
        context.styles,
        context.images,
        context.hyperlinks,
        hyperlink_url,
      ),
      w::CustomXmlRunChoice::MoveFromRun(moved) => push_move_from_run(
        moved,
        inlines,
        base_style.clone(),
        context.styles,
        context.images,
        context.hyperlinks,
        hyperlink_url,
      ),
      w::CustomXmlRunChoice::MoveToRun(moved) => push_move_to_run_or_complex_field(
        moved,
        inlines,
        base_style.clone(),
        RunImportContext {
          styles: context.styles,
          images: context.images,
          hyperlinks: context.hyperlinks,
          suppress_toc_hyperlink_style: context.suppress_toc_hyperlink_style,
        },
        hyperlink_url,
        complex_fields,
      ),
      w::CustomXmlRunChoice::BookmarkStart(bookmark) if !bookmark.name.is_empty() => {
        inlines.push(InlineItem::BookmarkStart(bookmark.name.to_string()));
      }
      _ => {}
    }
  }
}

fn paragraph_note_reference_ids(paragraph: &w::Paragraph) -> (Vec<i64>, Vec<i64>) {
  let mut footnotes = Vec::new();
  let mut endnotes = Vec::new();
  for choice in &paragraph.paragraph_choice {
    match choice {
      w::ParagraphChoice::WRun(run) => {
        collect_run_note_reference_ids(run, &mut footnotes, &mut endnotes)
      }
      w::ParagraphChoice::SimpleField(field) => {
        collect_simple_field_note_reference_ids(field, &mut footnotes, &mut endnotes);
      }
      w::ParagraphChoice::Hyperlink(hyperlink) => {
        collect_hyperlink_note_reference_ids(hyperlink, &mut footnotes, &mut endnotes);
      }
      w::ParagraphChoice::InsertedRun(inserted) => {
        collect_inserted_run_note_reference_ids(inserted, &mut footnotes, &mut endnotes);
      }
      w::ParagraphChoice::DeletedRun(_) | w::ParagraphChoice::MoveFromRun(_) => {}
      w::ParagraphChoice::MoveToRun(moved) => {
        collect_move_to_run_note_reference_ids(moved, &mut footnotes, &mut endnotes);
      }
      w::ParagraphChoice::SdtRun(sdt) => {
        collect_sdt_run_note_reference_ids(sdt, &mut footnotes, &mut endnotes);
      }
      _ => {}
    }
  }
  footnotes.sort_unstable();
  footnotes.dedup();
  endnotes.sort_unstable();
  endnotes.dedup();
  (footnotes, endnotes)
}

fn collect_run_note_reference_ids(run: &w::Run, footnotes: &mut Vec<i64>, endnotes: &mut Vec<i64>) {
  for choice in &run.run_choice {
    match choice {
      w::RunChoice::FootnoteReference(reference) if reference.id >= 0 => {
        footnotes.push(reference.id);
      }
      w::RunChoice::EndnoteReference(reference) if reference.id >= 0 => {
        endnotes.push(reference.id);
      }
      _ => {}
    }
  }
}

fn collect_simple_field_note_reference_ids(
  field: &w::SimpleField,
  footnotes: &mut Vec<i64>,
  endnotes: &mut Vec<i64>,
) {
  for choice in &field.simple_field_choice {
    match choice {
      w::SimpleFieldChoice::WRun(run) => {
        collect_run_note_reference_ids(run.as_ref(), footnotes, endnotes)
      }
      w::SimpleFieldChoice::SimpleField(field) => {
        collect_simple_field_note_reference_ids(field.as_ref(), footnotes, endnotes);
      }
      w::SimpleFieldChoice::Hyperlink(hyperlink) => {
        collect_hyperlink_note_reference_ids(hyperlink.as_ref(), footnotes, endnotes);
      }
      w::SimpleFieldChoice::SdtRun(sdt) => {
        collect_sdt_run_note_reference_ids(sdt, footnotes, endnotes);
      }
      _ => {}
    }
  }
}

fn collect_hyperlink_note_reference_ids(
  hyperlink: &w::Hyperlink,
  footnotes: &mut Vec<i64>,
  endnotes: &mut Vec<i64>,
) {
  for choice in &hyperlink.hyperlink_choice {
    match choice {
      w::HyperlinkChoice::WRun(run) => {
        collect_run_note_reference_ids(run.as_ref(), footnotes, endnotes)
      }
      w::HyperlinkChoice::SimpleField(field) => {
        collect_simple_field_note_reference_ids(field.as_ref(), footnotes, endnotes);
      }
      w::HyperlinkChoice::Hyperlink(hyperlink) => {
        collect_hyperlink_note_reference_ids(hyperlink.as_ref(), footnotes, endnotes);
      }
      w::HyperlinkChoice::SdtRun(sdt) => {
        collect_sdt_run_note_reference_ids(sdt, footnotes, endnotes);
      }
      w::HyperlinkChoice::InsertedRun(inserted) => {
        collect_inserted_run_note_reference_ids(inserted.as_ref(), footnotes, endnotes);
      }
      w::HyperlinkChoice::DeletedRun(_) | w::HyperlinkChoice::MoveFromRun(_) => {}
      w::HyperlinkChoice::MoveToRun(moved) => {
        collect_move_to_run_note_reference_ids(moved.as_ref(), footnotes, endnotes);
      }
      _ => {}
    }
  }
}

fn collect_sdt_run_note_reference_ids(
  sdt: &w::SdtRun,
  footnotes: &mut Vec<i64>,
  endnotes: &mut Vec<i64>,
) {
  let Some(content) = sdt.sdt_content_run.as_ref() else {
    return;
  };
  for choice in &content.sdt_content_run_choice {
    match choice {
      w::SdtContentRunChoice::WRun(run) => {
        collect_run_note_reference_ids(run.as_ref(), footnotes, endnotes)
      }
      w::SdtContentRunChoice::SimpleField(field) => {
        collect_simple_field_note_reference_ids(field.as_ref(), footnotes, endnotes);
      }
      w::SdtContentRunChoice::Hyperlink(hyperlink) => {
        collect_hyperlink_note_reference_ids(hyperlink.as_ref(), footnotes, endnotes);
      }
      w::SdtContentRunChoice::SdtRun(sdt) => {
        collect_sdt_run_note_reference_ids(sdt, footnotes, endnotes);
      }
      w::SdtContentRunChoice::InsertedRun(inserted) => {
        collect_inserted_run_note_reference_ids(inserted.as_ref(), footnotes, endnotes);
      }
      w::SdtContentRunChoice::DeletedRun(_) | w::SdtContentRunChoice::MoveFromRun(_) => {}
      w::SdtContentRunChoice::MoveToRun(moved) => {
        collect_move_to_run_note_reference_ids(moved.as_ref(), footnotes, endnotes);
      }
      _ => {}
    }
  }
}

fn collect_inserted_run_note_reference_ids(
  inserted: &w::InsertedRun,
  footnotes: &mut Vec<i64>,
  endnotes: &mut Vec<i64>,
) {
  for choice in &inserted.inserted_run_choice {
    match choice {
      w::InsertedRunChoice::WRun(run) => {
        collect_run_note_reference_ids(run.as_ref(), footnotes, endnotes)
      }
      w::InsertedRunChoice::InsertedRun(inserted) => {
        collect_inserted_run_note_reference_ids(inserted.as_ref(), footnotes, endnotes);
      }
      w::InsertedRunChoice::DeletedRun(_) | w::InsertedRunChoice::MoveFromRun(_) => {}
      w::InsertedRunChoice::MoveToRun(moved) => {
        collect_move_to_run_note_reference_ids(moved.as_ref(), footnotes, endnotes);
      }
      _ => {}
    }
  }
}

fn collect_move_to_run_note_reference_ids(
  moved: &w::MoveToRun,
  footnotes: &mut Vec<i64>,
  endnotes: &mut Vec<i64>,
) {
  for choice in &moved.move_to_run_choice {
    match choice {
      w::MoveToRunChoice::WRun(run) => {
        collect_run_note_reference_ids(run.as_ref(), footnotes, endnotes)
      }
      w::MoveToRunChoice::InsertedRun(inserted) => {
        collect_inserted_run_note_reference_ids(inserted.as_ref(), footnotes, endnotes);
      }
      w::MoveToRunChoice::DeletedRun(_) | w::MoveToRunChoice::MoveFromRun(_) => {}
      w::MoveToRunChoice::MoveToRun(moved) => {
        collect_move_to_run_note_reference_ids(moved.as_ref(), footnotes, endnotes);
      }
      _ => {}
    }
  }
}

fn push_simple_field(
  field: &w::SimpleField,
  inlines: &mut Vec<InlineItem>,
  base_style: TextStyle,
  context: &mut InlineImportContext<'_>,
) {
  let field_locked = field
    .field_lock
    .is_some_and(ooxmlsdk::simple_type::OnOffValue::as_bool);
  if !field_locked {
    let refreshed_date_time =
      refreshed_date_time_field(&field.instruction, &base_style, context.styles);
    let dynamic_kind = dynamic_field_kind(&field.instruction);
    if refreshed_date_time.is_some() || dynamic_kind.is_some() {
      let (result_text, result_style) =
        simple_field_result_text_and_style(field, base_style.clone(), context);
      let style = result_style.unwrap_or(base_style);
      if let Some(text) = refreshed_date_time {
        push_resolved_field_text(inlines, text, style, None);
      } else if let Some(DynamicFieldKind::StyleRef { style_name, .. }) = dynamic_kind.as_ref()
        && context
          .styles
          .style_ref_name_requires_localized_error(style_name)
      {
        push_localized_missing_style_ref(inlines, style_name, style, None);
      } else if let Some(kind) = dynamic_kind {
        push_dynamic_field(inlines, kind, style, None, result_text);
      }
      return;
    }
  }

  for choice in &field.simple_field_choice {
    match choice {
      w::SimpleFieldChoice::WRun(run) => push_run(
        run,
        inlines,
        base_style.clone(),
        context.styles,
        context.images,
        context.hyperlinks,
        None,
      ),
      w::SimpleFieldChoice::Hyperlink(hyperlink) => {
        let mut complex_fields = Vec::new();
        push_hyperlink_content(
          hyperlink.as_ref(),
          inlines,
          base_style.clone(),
          None,
          context,
          &mut complex_fields,
        );
        flush_unclosed_complex_fields(inlines, &mut complex_fields, context.styles);
      }
      w::SimpleFieldChoice::SimpleField(field) => {
        push_simple_field(field, inlines, base_style.clone(), context);
      }
      w::SimpleFieldChoice::SdtRun(sdt) => {
        push_sdt_run(sdt, inlines, base_style.clone(), None, context)
      }
      _ => {}
    }
  }
}

fn simple_field_result_text_and_style(
  field: &w::SimpleField,
  base_style: TextStyle,
  context: &mut InlineImportContext<'_>,
) -> (Option<String>, Option<TextStyle>) {
  let mut result = Vec::new();
  for choice in &field.simple_field_choice {
    match choice {
      w::SimpleFieldChoice::WRun(run) => push_run(
        run,
        &mut result,
        base_style.clone(),
        context.styles,
        context.images,
        context.hyperlinks,
        None,
      ),
      w::SimpleFieldChoice::Hyperlink(hyperlink) => {
        let mut complex_fields = Vec::new();
        push_hyperlink_content(
          hyperlink.as_ref(),
          &mut result,
          base_style.clone(),
          None,
          context,
          &mut complex_fields,
        );
        flush_unclosed_complex_fields(&mut result, &mut complex_fields, context.styles);
      }
      w::SimpleFieldChoice::SimpleField(nested) => {
        push_simple_field(nested, &mut result, base_style.clone(), context);
      }
      w::SimpleFieldChoice::SdtRun(sdt) => {
        push_sdt_run(sdt, &mut result, base_style.clone(), None, context);
      }
      _ => {}
    }
  }
  let style = field_result_style(&result);
  (field_result_text(&result), style)
}

fn field_result_style(result: &[InlineItem]) -> Option<TextStyle> {
  result.iter().find_map(|inline| match inline {
    InlineItem::Text(run) => Some(run.style.clone()),
    InlineItem::Ruby(ruby) => ruby.base.first().map(|run| run.style.clone()),
    _ => None,
  })
}

fn push_run(
  run: &w::Run,
  inlines: &mut Vec<InlineItem>,
  base_style: TextStyle,
  styles: &StylesCatalog,
  images: &ImageCatalog,
  hyperlinks: &HyperlinkCatalog,
  hyperlink_url: Option<&str>,
) {
  push_run_with_character_style_policy(
    run,
    inlines,
    base_style,
    RunImportContext {
      styles,
      images,
      hyperlinks,
      suppress_toc_hyperlink_style: false,
    },
    hyperlink_url,
    true,
  );
}

fn push_run_with_character_style_policy(
  run: &w::Run,
  inlines: &mut Vec<InlineItem>,
  base_style: TextStyle,
  context: RunImportContext<'_>,
  hyperlink_url: Option<&str>,
  apply_hyperlink_character_style: bool,
) {
  let RunImportContext {
    styles,
    images,
    hyperlinks,
    suppress_toc_hyperlink_style: _,
  } = context;
  let style = if apply_hyperlink_character_style {
    properties::run_style(run.run_properties.as_deref(), base_style.clone(), styles)
  } else {
    properties::run_style_without_hyperlink_character_style(
      run.run_properties.as_deref(),
      base_style.clone(),
      styles,
    )
  };
  let style_ref_keys = run
    .run_properties
    .as_deref()
    .and_then(run_properties_style_id)
    .filter(|style_id| {
      apply_hyperlink_character_style || !styles.is_hyperlink_character_style(style_id)
    })
    .map(|style_id| styles.style_ref_keys(style_id))
    .unwrap_or_default();
  if style.hidden {
    push_hidden_style_ref_run(
      run,
      inlines,
      style,
      hyperlink_url,
      &style_ref_keys,
      styles.preserve_word_text_whitespace,
    );
    return;
  }
  let mut text = String::new();

  for choice in &run.run_choice {
    match choice {
      w::RunChoice::Text(text_node) => {
        append_word_text(&mut text, text_node, styles.preserve_word_text_whitespace);
      }
      w::RunChoice::DeletedText(text_node) => {
        append_word_text(&mut text, text_node, styles.preserve_word_text_whitespace);
      }
      w::RunChoice::TabChar => text.push('\t'),
      w::RunChoice::CarriageReturn => text.push('\n'),
      w::RunChoice::Break(br) => match br.r#type {
        Some(w::BreakValues::Page) => {
          flush_run_text(
            inlines,
            &mut text,
            style.clone(),
            hyperlink_url,
            &style_ref_keys,
          );
          inlines.push(InlineItem::PageBreak);
        }
        Some(w::BreakValues::Column) => {
          flush_run_text(
            inlines,
            &mut text,
            style.clone(),
            hyperlink_url,
            &style_ref_keys,
          );
          inlines.push(InlineItem::ColumnBreak);
        }
        Some(w::BreakValues::TextWrapping) | None => text.push('\n'),
      },
      // This is a cached layout artifact from Word, not an author-authored break.
      w::RunChoice::LastRenderedPageBreak => {
        flush_run_text(
          inlines,
          &mut text,
          style.clone(),
          hyperlink_url,
          &style_ref_keys,
        );
        inlines.push(InlineItem::LastRenderedPageBreak);
      }
      w::RunChoice::SymbolChar(symbol) => {
        if let Some(symbol_char) = symbol_transport_char(symbol) {
          flush_run_text(
            inlines,
            &mut text,
            style.clone(),
            hyperlink_url,
            &style_ref_keys,
          );
          let mut symbol_style = style.clone();
          let uses_declared_font_transport = symbol
            .char
            .as_deref()
            .and_then(|code| u32::from_str_radix(code, 16).ok())
            == Some(symbol_char as u32);
          if uses_declared_font_transport
            && let Some(font) = symbol.font.as_deref().filter(|font| !font.is_empty())
          {
            symbol_style.font_family = Some(Arc::from(font));
            symbol_style.symbol_font_family = Some(Arc::from(font));
          }
          let mut symbol_text = symbol_char.to_string();
          flush_run_text(
            inlines,
            &mut symbol_text,
            symbol_style,
            hyperlink_url,
            &style_ref_keys,
          );
        }
      }
      w::RunChoice::PageNumber => {
        flush_run_text(
          inlines,
          &mut text,
          style.clone(),
          hyperlink_url,
          &style_ref_keys,
        );
        push_dynamic_field(
          inlines,
          DynamicFieldKind::Page {
            number_format: FieldNumberFormat::PageStyle,
          },
          style.clone(),
          hyperlink_url,
          None,
        );
      }
      w::RunChoice::NoBreakHyphen => text.push('\u{2011}'),
      w::RunChoice::SoftHyphen => text.push('\u{00ad}'),
      w::RunChoice::FootnoteReference(reference) => {
        flush_run_text(
          inlines,
          &mut text,
          style.clone(),
          hyperlink_url,
          &style_ref_keys,
        );
        push_note_reference(
          inlines,
          reference.id,
          style.clone(),
          Some(note_reference_url("footnote", reference.id)),
        );
      }
      w::RunChoice::EndnoteReference(reference) => {
        flush_run_text(
          inlines,
          &mut text,
          style.clone(),
          hyperlink_url,
          &style_ref_keys,
        );
        push_note_reference(
          inlines,
          reference.id,
          style.clone(),
          Some(note_reference_url("endnote", reference.id)),
        );
      }
      w::RunChoice::CommentReference(_) => {}
      w::RunChoice::Drawing(drawing) => {
        flush_run_text(
          inlines,
          &mut text,
          style.clone(),
          hyperlink_url,
          &style_ref_keys,
        );
        if let Some(image) = drawing::inline_image(drawing, styles, images, hyperlinks) {
          inlines.push(InlineItem::Image(image));
        }
        drawing::push_drawing_shapes(drawing, inlines, styles, images, hyperlinks);
        drawing::push_drawing_textboxes(
          drawing,
          inlines,
          style.clone(),
          styles,
          images,
          hyperlinks,
        );
      }
      w::RunChoice::Picture(picture) => {
        flush_run_text(
          inlines,
          &mut text,
          style.clone(),
          hyperlink_url,
          &style_ref_keys,
        );
        if let Some(image) = drawing::pict_image(picture, images) {
          inlines.push(InlineItem::Image(image));
        }
        drawing::push_pict_shapes(picture, inlines, images);
        drawing::push_pict_textboxes(
          picture,
          inlines,
          base_style.clone(),
          styles,
          images,
          hyperlinks,
        );
      }
      w::RunChoice::EmbeddedObject(object) => {
        flush_run_text(
          inlines,
          &mut text,
          style.clone(),
          hyperlink_url,
          &style_ref_keys,
        );
        if let Some(image) = embedded_object_image(object, images) {
          inlines.push(InlineItem::Image(image));
        }
      }
      w::RunChoice::PositionalTab(tab) => {
        flush_run_text(
          inlines,
          &mut text,
          style.clone(),
          hyperlink_url,
          &style_ref_keys,
        );
        inlines.push(InlineItem::PositionalTab(positional_tab(
          tab,
          style.clone(),
        )));
      }
      w::RunChoice::AlternateContent(_) => {}
      w::RunChoice::Ruby(ruby) => {
        flush_run_text(
          inlines,
          &mut text,
          style.clone(),
          hyperlink_url,
          &style_ref_keys,
        );
        push_ruby(
          ruby,
          inlines,
          base_style.clone(),
          styles,
          images,
          hyperlinks,
          hyperlink_url,
        );
      }
      w::RunChoice::Run(nested) => {
        flush_run_text(
          inlines,
          &mut text,
          style.clone(),
          hyperlink_url,
          &style_ref_keys,
        );
        // LibreOffice's writerfilter/ooxml testNestedRuns preserves malformed
        // nested `w:r` content found in shape text.  Treat the outer run's
        // resolved style as the inherited base for the nested run.
        push_run_with_character_style_policy(
          nested,
          inlines,
          style.clone(),
          RunImportContext {
            styles,
            images,
            hyperlinks,
            suppress_toc_hyperlink_style: !apply_hyperlink_character_style,
          },
          hyperlink_url,
          apply_hyperlink_character_style,
        );
      }
      _ => {}
    }
  }

  flush_run_text(inlines, &mut text, style, hyperlink_url, &style_ref_keys);
}

fn push_hidden_style_ref_run(
  run: &w::Run,
  inlines: &mut Vec<InlineItem>,
  style: TextStyle,
  hyperlink_url: Option<&str>,
  style_ref_keys: &[Arc<str>],
  inherited_space_preserve: bool,
) {
  if style_ref_keys.is_empty() {
    return;
  }
  let text = hidden_run_text(run, inherited_space_preserve);
  let text = text.trim();
  if text.is_empty() {
    return;
  }
  inlines.push(InlineItem::Text(TextRun {
    text: String::new(),
    style,
    hyperlink_url: hyperlink_url.map(ToString::to_string),
    dynamic_field: None,
    style_ref_keys: style_ref_keys.to_vec(),
    style_ref_text: Some(Arc::<str>::from(text)),
    style_ref_numbering_text: None,
    preserve_text_portion: false,
  }));
}

fn hidden_run_text(run: &w::Run, inherited_space_preserve: bool) -> String {
  let mut text = String::new();
  for choice in &run.run_choice {
    match choice {
      w::RunChoice::Text(text_node) => {
        append_word_text(&mut text, text_node, inherited_space_preserve);
      }
      w::RunChoice::DeletedText(text_node) => {
        append_word_text(&mut text, text_node, inherited_space_preserve);
      }
      w::RunChoice::TabChar | w::RunChoice::PositionalTab(_) => text.push('\t'),
      w::RunChoice::CarriageReturn => text.push('\n'),
      w::RunChoice::Break(br)
        if !matches!(
          br.r#type,
          Some(w::BreakValues::Page | w::BreakValues::Column)
        ) =>
      {
        text.push('\n');
      }
      w::RunChoice::SymbolChar(symbol) => {
        if let Some(symbol) = symbol_text(symbol) {
          text.push(symbol);
        }
      }
      _ => {}
    }
  }
  text
}

fn word_text_value(text: &w::TextType, inherited_space_preserve: bool) -> Option<&str> {
  text.xml_content.as_deref().map(|content| {
    if inherited_space_preserve || text.space == Some(xml::SpaceProcessingModeValues::Preserve) {
      content
    } else {
      // ECMA-376 examples mark runs with significant edge whitespace using
      // xml:space="preserve"; the default XML whitespace mode discards those
      // edge characters while retaining whitespace inside the text node.
      content.trim_matches([' ', '\t', '\r', '\n'])
    }
  })
}

fn append_word_text(output: &mut String, text: &w::TextType, inherited_space_preserve: bool) {
  let Some(content) = word_text_value(text, inherited_space_preserve) else {
    return;
  };
  let preserve =
    inherited_space_preserve || text.space == Some(xml::SpaceProcessingModeValues::Preserve);
  let mut chars = content.chars().peekable();
  while let Some(ch) = chars.next() {
    if ch == '\t' {
      // A U+0009 inside w:t is text, not the semantic tab represented by
      // w:tab (§17.3.3.32). Word places a preserved literal tab on its own
      // 420-twip text grid; without xml:space="preserve", it is one ordinary
      // internal whitespace character. Keep w:tab as '\t' so the paragraph
      // tab-stop machinery remains separate.
      output.push(if preserve {
        PRESERVED_WORD_TEXT_TAB
      } else {
        ' '
      });
    } else if ch == '\r' {
      // XML 1.0 normally normalizes CRLF to LF before schema deserialization,
      // but package producers and alternate readers can still expose CR here.
      // Writer's OOXML import treats line endings inside w:t as one ordinary
      // text space (tdf#108806), never as a manual line break. Consume a
      // following LF so an unnormalized CRLF pair still becomes one space.
      if chars.peek() == Some(&'\n') {
        chars.next();
      }
      output.push(' ');
    } else if ch == '\n' {
      // A visible line break is represented by w:br. Literal XML line endings
      // in CT_Text are whitespace and must not enter TextFrameLayout as hard
      // line-break markers.
      output.push(' ');
    } else if ch == '\u{f020}' {
      // U+F020 is the historical Symbol-font transport form of byte 0x20.
      // Some Word documents retain it in w:t after changing the run font.
      output.push(' ');
    } else {
      output.push(ch);
    }
  }
}

fn run_properties_style_id(properties: &w::RunProperties) -> Option<&str> {
  run_properties_run_style(properties).map(|run_style| run_style.val.as_str())
}

fn push_ruby(
  ruby: &w::Ruby,
  inlines: &mut Vec<InlineItem>,
  base_style: TextStyle,
  styles: &StylesCatalog,
  images: &ImageCatalog,
  hyperlinks: &HyperlinkCatalog,
  hyperlink_url: Option<&str>,
) {
  let mut base_items = Vec::new();
  for choice in &ruby.ruby_base.ruby_base_choice {
    match choice {
      w::RubyBaseChoice::WRun(run) => push_run(
        run,
        &mut base_items,
        base_style.clone(),
        styles,
        images,
        hyperlinks,
        hyperlink_url,
      ),
      w::RubyBaseChoice::InsertedRun(inserted) => {
        push_inserted_run(
          inserted,
          &mut base_items,
          base_style.clone(),
          styles,
          images,
          hyperlinks,
          hyperlink_url,
        );
      }
      w::RubyBaseChoice::DeletedRun(deleted) => {
        push_deleted_run(
          deleted,
          &mut base_items,
          base_style.clone(),
          styles,
          images,
          hyperlinks,
          hyperlink_url,
        );
      }
      w::RubyBaseChoice::MoveFromRun(moved) => {
        push_move_from_run(
          moved,
          &mut base_items,
          base_style.clone(),
          styles,
          images,
          hyperlinks,
          hyperlink_url,
        );
      }
      w::RubyBaseChoice::MoveToRun(moved) => {
        push_move_to_run(
          moved,
          &mut base_items,
          base_style.clone(),
          styles,
          images,
          hyperlinks,
          hyperlink_url,
        );
      }
      _ => {}
    }
  }

  let Some(base) = ruby_text_runs(&base_items) else {
    inlines.extend(base_items);
    return;
  };
  if base.is_empty() {
    return;
  }

  let mut guide_items = Vec::new();
  for choice in &ruby.ruby_content.ruby_content_choice {
    match choice {
      w::RubyContentChoice::WRun(run) => push_run(
        run,
        &mut guide_items,
        base_style.clone(),
        styles,
        images,
        hyperlinks,
        hyperlink_url,
      ),
      w::RubyContentChoice::InsertedRun(inserted) => push_inserted_run(
        inserted,
        &mut guide_items,
        base_style.clone(),
        styles,
        images,
        hyperlinks,
        hyperlink_url,
      ),
      w::RubyContentChoice::DeletedRun(deleted) => push_deleted_run(
        deleted,
        &mut guide_items,
        base_style.clone(),
        styles,
        images,
        hyperlinks,
        hyperlink_url,
      ),
      w::RubyContentChoice::MoveFromRun(moved) => push_move_from_run(
        moved,
        &mut guide_items,
        base_style.clone(),
        styles,
        images,
        hyperlinks,
        hyperlink_url,
      ),
      w::RubyContentChoice::MoveToRun(moved) => push_move_to_run(
        moved,
        &mut guide_items,
        base_style.clone(),
        styles,
        images,
        hyperlinks,
        hyperlink_url,
      ),
      _ => {}
    }
  }
  let Some(mut guide) = ruby_text_runs(&guide_items) else {
    inlines.extend(base_items);
    return;
  };
  if guide.is_empty() {
    inlines.extend(base_items);
    return;
  }

  let properties = ruby.ruby_properties.as_ref();
  let guide_size_pt = properties.phonetic_guide_text_font_size.val.to_points() as f32;
  if guide_size_pt > 0.0 {
    for run in &mut guide {
      // ECMA-376 Part 1 §17.3.3.10: w:rubyPr/w:hps overrides a
      // disagreeing w:rt run size.
      run.style.font_size_pt = guide_size_pt;
      run.style.complex_font_size_pt = Some(guide_size_pt);
    }
  }
  let alignment = match properties.ruby_align.val {
    w::RubyAlignValues::Center => RubyAlignment::Center,
    w::RubyAlignValues::DistributeLetter => RubyAlignment::DistributeLetter,
    w::RubyAlignValues::DistributeSpace => RubyAlignment::DistributeSpace,
    w::RubyAlignValues::Left => RubyAlignment::Left,
    w::RubyAlignValues::Right => RubyAlignment::Right,
    w::RubyAlignValues::RightVertical => RubyAlignment::RightVertical,
  };
  inlines.push(InlineItem::Ruby(RubyInline {
    base,
    guide,
    alignment,
    raise_pt: properties.phonetic_guide_raise.val as f32 / 2.0,
  }));
}

fn ruby_text_runs(items: &[InlineItem]) -> Option<Vec<TextRun>> {
  let mut runs = Vec::new();
  for item in items {
    match item {
      InlineItem::Text(run) => runs.push(run.clone()),
      InlineItem::PositionalTab(_) => return None,
      InlineItem::BookmarkStart(_)
      | InlineItem::DrawingGroupStart(_)
      | InlineItem::DrawingGroupEnd => {}
      InlineItem::Ruby(_)
      | InlineItem::Image(_)
      | InlineItem::Shape(_)
      | InlineItem::FormWidgetStart(_)
      | InlineItem::FormWidgetEnd(_)
      | InlineItem::LastRenderedPageBreak
      | InlineItem::PageBreak
      | InlineItem::ColumnBreak => return None,
    }
  }
  Some(runs)
}

fn push_sdt_run(
  sdt: &w::SdtRun,
  inlines: &mut Vec<InlineItem>,
  base_style: TextStyle,
  hyperlink_url: Option<&str>,
  context: &mut InlineImportContext<'_>,
) {
  let Some(content) = sdt.sdt_content_run.as_ref() else {
    return;
  };
  let start = inlines.len();
  let showing_placeholder = sdt
    .sdt_properties
    .as_ref()
    .is_some_and(sdt_showing_placeholder);
  let picture_content_control = !showing_placeholder
    && sdt.sdt_properties.as_ref().is_some_and(|properties| {
      properties
        .sdt_properties_choice
        .iter()
        .any(|choice| matches!(choice, w::SdtPropertiesChoice::SdtContentPicture))
    });
  let widget_id = sdt
    .sdt_properties
    .as_ref()
    .and_then(sdt_form_widget)
    .map(|(kind, entries)| context.form_widget_ids.next_widget(kind, entries));
  if let Some(widget_id) = widget_id {
    inlines.push(InlineItem::FormWidgetStart(widget_id));
  }
  if let Some(value) = sdt
    .sdt_properties
    .as_ref()
    .and_then(|properties| sdt_bound_replacement(context.custom_xml_bindings, properties))
  {
    inlines.push(InlineItem::Text(TextRun {
      text: value,
      style: base_style,
      hyperlink_url: hyperlink_url.map(str::to_owned),
      dynamic_field: None,
      style_ref_keys: Vec::new(),
      style_ref_text: None,
      style_ref_numbering_text: None,
      preserve_text_portion: false,
    }));
    if let Some(widget_id) = widget_id {
      inlines.push(InlineItem::FormWidgetEnd(widget_id));
    }
    return;
  }

  let mut complex_fields = Vec::new();
  for choice in &content.sdt_content_run_choice {
    match choice {
      w::SdtContentRunChoice::WRun(run) => push_run_or_complex_field(
        run.as_ref(),
        inlines,
        base_style.clone(),
        RunImportContext {
          styles: context.styles,
          images: context.images,
          hyperlinks: context.hyperlinks,
          suppress_toc_hyperlink_style: context.suppress_toc_hyperlink_style,
        },
        hyperlink_url,
        &mut complex_fields,
      ),
      w::SdtContentRunChoice::SimpleField(field) => {
        push_simple_field(field.as_ref(), inlines, base_style.clone(), context);
      }
      w::SdtContentRunChoice::Hyperlink(hyperlink) => {
        push_hyperlink_content(
          hyperlink.as_ref(),
          inlines,
          base_style.clone(),
          hyperlink_url,
          context,
          &mut complex_fields,
        );
      }
      w::SdtContentRunChoice::SdtRun(sdt) => push_sdt_run(
        sdt.as_ref(),
        inlines,
        base_style.clone(),
        hyperlink_url,
        context,
      ),
      w::SdtContentRunChoice::InsertedRun(inserted) => {
        push_inserted_run_or_complex_field(
          inserted.as_ref(),
          inlines,
          base_style.clone(),
          RunImportContext {
            styles: context.styles,
            images: context.images,
            hyperlinks: context.hyperlinks,
            suppress_toc_hyperlink_style: context.suppress_toc_hyperlink_style,
          },
          hyperlink_url,
          &mut complex_fields,
        );
      }
      w::SdtContentRunChoice::DeletedRun(deleted) => {
        push_deleted_run(
          deleted.as_ref(),
          inlines,
          base_style.clone(),
          context.styles,
          context.images,
          context.hyperlinks,
          hyperlink_url,
        );
      }
      w::SdtContentRunChoice::MoveFromRun(moved) => {
        push_move_from_run(
          moved.as_ref(),
          inlines,
          base_style.clone(),
          context.styles,
          context.images,
          context.hyperlinks,
          hyperlink_url,
        );
      }
      w::SdtContentRunChoice::MoveToRun(moved) => {
        push_move_to_run_or_complex_field(
          moved.as_ref(),
          inlines,
          base_style.clone(),
          RunImportContext {
            styles: context.styles,
            images: context.images,
            hyperlinks: context.hyperlinks,
            suppress_toc_hyperlink_style: context.suppress_toc_hyperlink_style,
          },
          hyperlink_url,
          &mut complex_fields,
        );
      }
      _ => {}
    }
  }
  flush_unclosed_complex_fields(inlines, &mut complex_fields, context.styles);
  if picture_content_control {
    // ECMA-376 Part 1 §17.5.2.24 makes this a distinct inline control whose
    // content is one DrawingML picture. Writer likewise retains it as a
    // ContentControl text portion containing the frame (SdtHelper.cxx), not
    // as an ordinary bare image. Preserve that ownership for line placement.
    // Section 17.5.2.39 is a separate state: when `showingPlcHdr` is true,
    // `sdtContent` is placeholder content rather than the control's regular
    // current content, so its cached picture keeps ordinary inline placement.
    for inline in &mut inlines[start..] {
      if let InlineItem::Image(image) = inline {
        image.picture_content_control = true;
      }
    }
  }
  if showing_placeholder {
    for inline in &mut inlines[start..] {
      if let InlineItem::Text(run) = inline {
        run.preserve_text_portion = true;
      }
    }
  }
  if let Some(widget_id) = widget_id {
    inlines.push(InlineItem::FormWidgetEnd(widget_id));
  }
}

fn sdt_bound_display_text(properties: &w::SdtProperties, value: String) -> String {
  let date_format = properties
    .sdt_properties_choice
    .iter()
    .find_map(|choice| match choice {
      w::SdtPropertiesChoice::SdtContentDate(date) => {
        date.date_format.as_ref().map(|format| format.val.as_str())
      }
      _ => None,
    });
  let Some(date_format) = date_format else {
    return value;
  };
  let Some(date) = value.get(..10) else {
    return value;
  };
  let mut components = date.split('-');
  let (Some(year), Some(month), Some(day), None) = (
    components.next(),
    components.next(),
    components.next(),
    components.next(),
  ) else {
    return value;
  };
  let (Ok(month), Ok(day)) = (month.parse::<u8>(), day.parse::<u8>()) else {
    return value;
  };
  format_sdt_date(date_format, year, month, day).unwrap_or(value)
}

fn sdt_supports_bound_text(properties: &w::SdtProperties) -> bool {
  properties.sdt_properties_choice.iter().any(|choice| {
    matches!(
      choice,
      w::SdtPropertiesChoice::SdtContentText(_) | w::SdtPropertiesChoice::SdtContentDate(_)
    )
  })
}

fn sdt_bound_replacement(
  custom_xml_bindings: &CustomXmlBindings,
  properties: &w::SdtProperties,
) -> Option<String> {
  if !sdt_supports_bound_text(properties) {
    return None;
  }
  let value = custom_xml_bindings.value_for_sdt(properties)?;
  if value.is_empty()
    && sdt_has_data_binding(properties)
    && let Some(name) = sdt_placeholder_doc_part(properties)
    && let Some(body) = custom_xml_bindings.glossary_placeholder(name)
    && let Some(text) = simple_glossary_placeholder_text(body)
  {
    // ECMA-376 Part 1 §17.5.2.25 requires an empty mapped XML element to
    // display the named placeholder from a bbPlcHdr Glossary Document entry.
    // Keep this distinct from the Word deviations in [MS-OI29500]
    // §§2.1.195 and 2.1.199, which delay or suppress a placeholder selected
    // only because run contents are empty.
    return Some(text);
  }
  sdt_bound_replacement_text(properties, value)
}

fn sdt_has_data_binding(properties: &w::SdtProperties) -> bool {
  properties
    .sdt_properties_choice
    .iter()
    .any(|choice| matches!(choice, w::SdtPropertiesChoice::WDataBinding(_)))
}

fn sdt_placeholder_doc_part(properties: &w::SdtProperties) -> Option<&str> {
  properties
    .sdt_properties_choice
    .iter()
    .find_map(|choice| match choice {
      w::SdtPropertiesChoice::SdtPlaceholder(placeholder) => placeholder
        .doc_part_reference
        .as_ref()
        .map(|reference| reference.val.as_str())
        .filter(|name| !name.is_empty()),
      _ => None,
    })
}

fn simple_glossary_placeholder_text(body: &w::DocPartBody) -> Option<String> {
  let [w::DocPartBodyChoice::Paragraph(paragraph)] = body.doc_part_body_choice.as_slice() else {
    return None;
  };
  let mut text = String::new();
  for choice in &paragraph.paragraph_choice {
    let w::ParagraphChoice::WRun(run) = choice else {
      return None;
    };
    text.push_str(&hidden_run_text(run, false));
  }
  (!text.is_empty()).then_some(text)
}

fn sdt_bound_replacement_text(properties: &w::SdtProperties, value: String) -> Option<String> {
  // ECMA-376 Part 1 §17.5.2.25 makes an empty mapped XML element a
  // placeholder condition. Section 17.5.2.39 says that a true
  // `showingPlcHdr` marks the cached `sdtContent` as that placeholder, so keep
  // the cache instead of replacing it with an empty bound value.
  if value.is_empty() && sdt_showing_placeholder(properties) {
    return None;
  }
  Some(sdt_bound_display_text(properties, value))
}

fn format_sdt_date(format: &str, year: &str, month: u8, day: u8) -> Option<String> {
  let mut output = String::new();
  let mut chars = format.chars().peekable();
  while let Some(ch) = chars.next() {
    let token = matches!(ch, 'd' | 'M' | 'y');
    if !token {
      if ch.is_ascii_alphabetic() {
        return None;
      }
      output.push(ch);
      continue;
    }
    let mut count = 1usize;
    while chars.peek() == Some(&ch) {
      chars.next();
      count += 1;
    }
    match ch {
      'd' if count == 1 => output.push_str(&day.to_string()),
      'd' => output.push_str(&format!("{day:02}")),
      'M' if count == 1 => output.push_str(&month.to_string()),
      'M' => output.push_str(&format!("{month:02}")),
      'y' if count == 2 => output.push_str(year.get(year.len().saturating_sub(2)..)?),
      'y' => output.push_str(year),
      _ => unreachable!(),
    }
  }
  Some(output)
}

fn sdt_showing_placeholder(properties: &w::SdtProperties) -> bool {
  properties
    .sdt_properties_choice
    .iter()
    .any(|choice| match choice {
      // The common boolean definition in §17.17.4 defaults an omitted `val`
      // to true. Keep the transitional `on`/`off` spellings supported by the
      // SDK's typed OnOffValue as well.
      w::SdtPropertiesChoice::ShowingPlaceholder(placeholder) => {
        placeholder.val.is_none_or(|value| value.as_bool())
      }
      _ => false,
    })
}

fn sdt_form_widget(properties: &w::SdtProperties) -> Option<(FormWidgetKind, Vec<String>)> {
  let mut kind = None;
  let mut entries = Vec::new();
  let showing_placeholder = sdt_showing_placeholder(properties);
  for choice in &properties.sdt_properties_choice {
    match choice {
      w::SdtPropertiesChoice::SdtContentComboBox(combo_box) => {
        kind = Some(FormWidgetKind::ComboBox);
        entries = sdt_list_item_display_texts(&combo_box.list_item);
      }
      w::SdtPropertiesChoice::SdtContentDropDownList(drop_down) => {
        kind = Some(FormWidgetKind::DropDownList);
        entries = sdt_list_item_display_texts(&drop_down.list_item);
      }
      w::SdtPropertiesChoice::SdtContentDate(_) => {
        kind = Some(FormWidgetKind::Text);
      }
      w::SdtPropertiesChoice::SdtContentRichText | w::SdtPropertiesChoice::SdtContentText(_) => {
        kind = Some(FormWidgetKind::Text);
      }
      _ => {}
    }
  }
  if kind.is_none() && showing_placeholder {
    kind = Some(FormWidgetKind::Text);
  }
  kind.map(|kind| (kind, entries))
}

fn sdt_list_item_display_texts(items: &[w::ListItem]) -> Vec<String> {
  items
    .iter()
    .map(|item| {
      item
        .display_text
        .as_ref()
        .or(item.value.as_ref())
        .cloned()
        .unwrap_or_default()
    })
    .collect()
}

fn push_inserted_run_or_complex_field(
  inserted: &w::InsertedRun,
  inlines: &mut Vec<InlineItem>,
  base_style: TextStyle,
  context: RunImportContext<'_>,
  hyperlink_url: Option<&str>,
  complex_fields: &mut Vec<ComplexFieldState>,
) {
  for choice in &inserted.inserted_run_choice {
    match choice {
      w::InsertedRunChoice::WRun(run) => push_run_or_complex_field(
        run,
        inlines,
        base_style.clone(),
        context,
        hyperlink_url,
        complex_fields,
      ),
      w::InsertedRunChoice::InsertedRun(nested) => push_inserted_run_or_complex_field(
        nested,
        inlines,
        base_style.clone(),
        context,
        hyperlink_url,
        complex_fields,
      ),
      w::InsertedRunChoice::DeletedRun(_) | w::InsertedRunChoice::MoveFromRun(_) => {}
      w::InsertedRunChoice::MoveToRun(moved) => push_move_to_run_or_complex_field(
        moved,
        inlines,
        base_style.clone(),
        context,
        hyperlink_url,
        complex_fields,
      ),
      _ => {}
    }
  }
}

fn push_move_to_run_or_complex_field(
  moved: &w::MoveToRun,
  inlines: &mut Vec<InlineItem>,
  base_style: TextStyle,
  context: RunImportContext<'_>,
  hyperlink_url: Option<&str>,
  complex_fields: &mut Vec<ComplexFieldState>,
) {
  for choice in &moved.move_to_run_choice {
    match choice {
      w::MoveToRunChoice::WRun(run) => push_run_or_complex_field(
        run,
        inlines,
        base_style.clone(),
        context,
        hyperlink_url,
        complex_fields,
      ),
      w::MoveToRunChoice::InsertedRun(inserted) => push_inserted_run_or_complex_field(
        inserted,
        inlines,
        base_style.clone(),
        context,
        hyperlink_url,
        complex_fields,
      ),
      w::MoveToRunChoice::DeletedRun(_) | w::MoveToRunChoice::MoveFromRun(_) => {}
      w::MoveToRunChoice::MoveToRun(nested) => push_move_to_run_or_complex_field(
        nested,
        inlines,
        base_style.clone(),
        context,
        hyperlink_url,
        complex_fields,
      ),
      _ => {}
    }
  }
}

fn push_inserted_run(
  inserted: &w::InsertedRun,
  inlines: &mut Vec<InlineItem>,
  base_style: TextStyle,
  styles: &StylesCatalog,
  images: &ImageCatalog,
  hyperlinks: &HyperlinkCatalog,
  hyperlink_url: Option<&str>,
) {
  for choice in &inserted.inserted_run_choice {
    match choice {
      w::InsertedRunChoice::WRun(run) => push_run(
        run,
        inlines,
        base_style.clone(),
        styles,
        images,
        hyperlinks,
        hyperlink_url,
      ),
      w::InsertedRunChoice::InsertedRun(nested) => {
        push_inserted_run(
          nested,
          inlines,
          base_style.clone(),
          styles,
          images,
          hyperlinks,
          hyperlink_url,
        );
      }
      w::InsertedRunChoice::DeletedRun(_) | w::InsertedRunChoice::MoveFromRun(_) => {}
      w::InsertedRunChoice::MoveToRun(moved) => {
        push_move_to_run(
          moved,
          inlines,
          base_style.clone(),
          styles,
          images,
          hyperlinks,
          hyperlink_url,
        );
      }
      _ => {}
    }
  }
}

fn push_deleted_run(
  _deleted: &w::DeletedRun,
  _inlines: &mut Vec<InlineItem>,
  _base_style: TextStyle,
  _styles: &StylesCatalog,
  _images: &ImageCatalog,
  _hyperlinks: &HyperlinkCatalog,
  _hyperlink_url: Option<&str>,
) {
}

fn push_move_from_run(
  _moved: &w::MoveFromRun,
  _inlines: &mut Vec<InlineItem>,
  _base_style: TextStyle,
  _styles: &StylesCatalog,
  _images: &ImageCatalog,
  _hyperlinks: &HyperlinkCatalog,
  _hyperlink_url: Option<&str>,
) {
}

fn push_move_to_run(
  moved: &w::MoveToRun,
  inlines: &mut Vec<InlineItem>,
  base_style: TextStyle,
  styles: &StylesCatalog,
  images: &ImageCatalog,
  hyperlinks: &HyperlinkCatalog,
  hyperlink_url: Option<&str>,
) {
  for choice in &moved.move_to_run_choice {
    match choice {
      w::MoveToRunChoice::WRun(run) => push_run(
        run,
        inlines,
        base_style.clone(),
        styles,
        images,
        hyperlinks,
        hyperlink_url,
      ),
      w::MoveToRunChoice::InsertedRun(inserted) => push_inserted_run(
        inserted,
        inlines,
        base_style.clone(),
        styles,
        images,
        hyperlinks,
        hyperlink_url,
      ),
      w::MoveToRunChoice::DeletedRun(_) | w::MoveToRunChoice::MoveFromRun(_) => {}
      w::MoveToRunChoice::MoveToRun(moved) => push_move_to_run(
        moved,
        inlines,
        base_style.clone(),
        styles,
        images,
        hyperlinks,
        hyperlink_url,
      ),
      _ => {}
    }
  }
}

fn push_note_reference(
  inlines: &mut Vec<InlineItem>,
  id: i64,
  style: TextStyle,
  hyperlink_url: Option<String>,
) {
  if id < 0 {
    return;
  }
  inlines.push(InlineItem::Text(TextRun {
    text: id.to_string(),
    style: note_reference_style(&style),
    hyperlink_url,
    dynamic_field: None,
    style_ref_keys: Vec::new(),
    style_ref_text: None,
    style_ref_numbering_text: None,
    preserve_text_portion: false,
  }));
}

fn note_reference_style(style: &TextStyle) -> TextStyle {
  if style.baseline_shift_pt.abs() > f32::EPSILON {
    return style.clone();
  }
  let mut reference_style = style.clone();
  reference_style.baseline_shift_pt =
    crate::fonts::effective_font_size_pt(style, None) * LO_SUPERSCRIPT_BASELINE_SHIFT_SCALE;
  reference_style.font_size_pt =
    (style.font_size_pt * WORD_DEFAULT_ESCAPEMENT_HEIGHT_SCALE).max(MIN_ESCAPEMENT_FONT_SIZE_PT);
  reference_style.complex_font_size_pt = style
    .complex_font_size_pt
    .map(|size| (size * WORD_DEFAULT_ESCAPEMENT_HEIGHT_SCALE).max(MIN_ESCAPEMENT_FONT_SIZE_PT));
  reference_style
}

fn note_reference_url(kind: &str, id: i64) -> String {
  format!("ooxmlsdk-pdf:{kind}-reference:{id}")
}

fn note_backlink_url(kind: &str, id: i64) -> String {
  format!("ooxmlsdk-pdf:{kind}-backlink:{id}")
}

fn flush_run_text(
  inlines: &mut Vec<InlineItem>,
  text: &mut String,
  style: TextStyle,
  hyperlink_url: Option<&str>,
  style_ref_keys: &[Arc<str>],
) {
  if !text.is_empty() {
    let text = run_display_text(std::mem::take(text), style.clone());
    inlines.push(InlineItem::Text(TextRun {
      text,
      style,
      hyperlink_url: hyperlink_url.map(ToString::to_string),
      dynamic_field: None,
      style_ref_keys: style_ref_keys.to_vec(),
      style_ref_text: None,
      style_ref_numbering_text: None,
      preserve_text_portion: false,
    }));
  }
}

fn run_display_text(text: String, style: TextStyle) -> String {
  let text = if style.uppercase {
    text.to_uppercase()
  } else {
    text
  };
  shared_symbol::font_symbol_transport_text(style.font_family.as_deref(), &text).into_owned()
}

fn symbol_text(symbol: &w::SymbolChar) -> Option<char> {
  let code = u32::from_str_radix(symbol.char.as_deref()?, 16).ok()?;
  shared_symbol::font_symbol_code(symbol.font.as_deref(), code)
}

fn symbol_transport_char(symbol: &w::SymbolChar) -> Option<char> {
  let code = u32::from_str_radix(symbol.char.as_deref()?, 16).ok()?;
  let font = symbol.font.as_deref().unwrap_or("");
  let is_wingdings = font.to_ascii_lowercase().contains("wingdings");
  let is_symbol_font = font.eq_ignore_ascii_case("Symbol")
    || font
      .get(font.len().saturating_sub(" symbol".len())..)
      .is_some_and(|suffix| suffix.eq_ignore_ascii_case(" symbol"))
    || is_wingdings;
  let mapped = shared_symbol::font_symbol_code(symbol.font.as_deref(), code)?;
  if font.eq_ignore_ascii_case("Symbol") && code & 0xFF == 0x94 {
    Some(mapped)
  } else if is_symbol_font && (!is_wingdings || code & 0xFF >= 0x80) {
    char::from_u32(code)
  } else {
    Some(mapped)
  }
}

fn inline_image_impl(
  drawing: &w::Drawing,
  styles: &StylesCatalog,
  images: &ImageCatalog,
  hyperlinks: &HyperlinkCatalog,
) -> Option<InlineImage> {
  if drawing_is_hidden(drawing) {
    return None;
  }

  match drawing.drawing_choice.as_ref()? {
    w::DrawingChoice::Inline(inline) => {
      let properties = drawing_image_properties(
        &inline.graphic.graphic_data,
        &styles.theme_colors,
        Some(images),
      )?;
      let image_data = drawing_image_data(images, &properties)?;
      let (image_data, crop) = materialize_source_rectangle_crop(
        image_data,
        properties.crop,
        properties.source_rectangle_crop,
      );
      let hyperlink_url = inline
        .doc_properties
        .hyperlink_on_click
        .as_deref()
        .and_then(|hyperlink| hyperlink.id.as_deref())
        .and_then(|relationship_id| hyperlinks.target(relationship_id))
        .or_else(|| {
          properties
            .hyperlink_relationship_id
            .as_deref()
            .and_then(|relationship_id| hyperlinks.target(relationship_id))
        })
        .map(ToString::to_string);
      Some(InlineImage {
        data: image_data.data,
        content_type: image_data.content_type,
        picture_frame: properties.picture_frame,
        effects: properties.shape_effects,
        static3d: properties.static3d,
        width_pt: units::emu_to_points(inline.extent.cx),
        height_pt: units::emu_to_points(inline.extent.cy),
        effect_left_pt: effect_extent_left(inline.effect_extent.as_ref()),
        effect_top_pt: effect_extent_top(inline.effect_extent.as_ref()),
        effect_right_pt: effect_extent_right(inline.effect_extent.as_ref()),
        effect_bottom_pt: effect_extent_bottom(inline.effect_extent.as_ref()),
        crop,
        rotation_deg: properties.rotation_deg,
        flip_horizontal: properties.flip_horizontal,
        flip_vertical: properties.flip_vertical,
        metafile_background_color: None,
        alt_text: inline.doc_properties.description.clone(),
        hyperlink_url,
        semantic_metafile_text: false,
        metafile_native_size: true,
        picture_content_control: false,
        placement: ImagePlacement::Inline,
      })
    }
    w::DrawingChoice::Anchor(anchor) => {
      let graphic = anchor.graphic.as_ref();
      let extent = &anchor.extent;
      let properties =
        drawing_image_properties(&graphic.graphic_data, &styles.theme_colors, Some(images))?;
      let image_data = drawing_image_data(images, &properties)?;
      let (image_data, crop) = materialize_source_rectangle_crop(
        image_data,
        properties.crop,
        properties.source_rectangle_crop,
      );
      let hyperlink_url = anchor
        .doc_properties
        .as_deref()
        .and_then(|properties| properties.hyperlink_on_click.as_deref())
        .and_then(|hyperlink| hyperlink.id.as_deref())
        .and_then(|relationship_id| hyperlinks.target(relationship_id))
        .or_else(|| {
          properties
            .hyperlink_relationship_id
            .as_deref()
            .and_then(|relationship_id| hyperlinks.target(relationship_id))
        })
        .map(ToString::to_string);
      let effect_extent = DrawingEffectExtent {
        left_pt: effect_extent_left(anchor.effect_extent.as_ref()),
        top_pt: effect_extent_top(anchor.effect_extent.as_ref()),
        right_pt: effect_extent_right(anchor.effect_extent.as_ref()),
        bottom_pt: effect_extent_bottom(anchor.effect_extent.as_ref()),
      };
      Some(InlineImage {
        data: image_data.data,
        content_type: image_data.content_type,
        picture_frame: properties.picture_frame,
        effects: properties.shape_effects,
        static3d: properties.static3d,
        width_pt: units::emu_to_points(extent.cx),
        height_pt: units::emu_to_points(extent.cy),
        effect_left_pt: effect_extent.left_pt,
        effect_top_pt: effect_extent.top_pt,
        effect_right_pt: effect_extent.right_pt,
        effect_bottom_pt: effect_extent.bottom_pt,
        crop,
        rotation_deg: properties.rotation_deg,
        flip_horizontal: properties.flip_horizontal,
        flip_vertical: properties.flip_vertical,
        metafile_background_color: None,
        alt_text: anchor
          .doc_properties
          .as_ref()
          .and_then(|properties| properties.description.clone()),
        hyperlink_url,
        semantic_metafile_text: false,
        metafile_native_size: true,
        picture_content_control: false,
        placement: drawing_placement_with_effect_extent(
          ImagePlacement::Floating(floating_image_placement(anchor)),
          effect_extent,
        ),
      })
    }
  }
}

fn effect_extent_left(extent: Option<&wp::EffectExtent>) -> f32 {
  extent
    .map(|extent| units::emu_to_points(extent.left_edge.to_emu()))
    .unwrap_or(0.0)
}

fn effect_extent_top(extent: Option<&wp::EffectExtent>) -> f32 {
  extent
    .map(|extent| units::emu_to_points(extent.top_edge.to_emu()))
    .unwrap_or(0.0)
}

fn effect_extent_right(extent: Option<&wp::EffectExtent>) -> f32 {
  extent
    .map(|extent| units::emu_to_points(extent.right_edge.to_emu()))
    .unwrap_or(0.0)
}

fn effect_extent_bottom(extent: Option<&wp::EffectExtent>) -> f32 {
  extent
    .map(|extent| units::emu_to_points(extent.bottom_edge.to_emu()))
    .unwrap_or(0.0)
}

fn floating_image_placement(anchor: &wp::Anchor) -> FloatingImagePlacement {
  let margins = floating_wrap_margins(anchor);
  let horizontal_position = anchor.horizontal_position.as_deref();
  let vertical_position = anchor.vertical_position.as_deref();
  let simple_position = anchor
    .simple_pos
    .as_ref()
    .is_some_and(|value| value.as_bool())
    .then_some(anchor.simple_position.as_ref())
    .flatten();
  let horizontal_relative_to = simple_position
    .map(|_| HorizontalImageReference::Page)
    .or_else(|| horizontal_position.map(horizontal_image_reference))
    .unwrap_or_default();
  let vertical_relative_to = simple_position
    .map(|_| VerticalImageReference::Page)
    .or_else(|| vertical_position.map(vertical_image_reference))
    .unwrap_or_default();
  let layout_in_cell = anchor.layout_in_cell.as_bool()
    || (simple_position.is_none()
      && matches!(
        (horizontal_relative_to, vertical_relative_to),
        (HorizontalImageReference::Character, _) | (_, VerticalImageReference::Line)
      ));
  FloatingImagePlacement {
    horizontal_relative_to,
    vertical_relative_to,
    horizontal_alignment: simple_position
      .map(|_| None)
      .unwrap_or_else(|| horizontal_position.and_then(horizontal_position_alignment)),
    vertical_alignment: simple_position
      .map(|_| None)
      .unwrap_or_else(|| vertical_position.and_then(vertical_position_alignment)),
    horizontal_offset_pt: simple_position
      .map(|position| units::emu_to_points(position.x.to_emu()))
      .or_else(|| horizontal_position.and_then(horizontal_position_offset))
      .unwrap_or(0.0),
    vertical_offset_pt: simple_position
      .map(|position| units::emu_to_points(position.y.to_emu()))
      .or_else(|| vertical_position.and_then(vertical_position_offset))
      .unwrap_or(0.0),
    wrap: anchor
      .anchor_choice
      .as_ref()
      .map(image_wrap_mode)
      .unwrap_or(ImageWrapMode::None),
    wrap_side: anchor
      .anchor_choice
      .as_ref()
      .map(image_wrap_side)
      .unwrap_or_default(),
    behind_text: anchor.behind_doc.as_bool(),
    layout_in_cell,
    allow_overlap: anchor.allow_overlap.as_bool(),
    paint_order: FloatingPaintOrder::DrawingMlRelativeHeight(
      anchor.relative_height.unwrap_or_default(),
    ),
    relative_width_to: anchor
      .relative_width
      .as_ref()
      .map(|relative| relative_width_reference(relative.object_id)),
    relative_width_pct: anchor
      .relative_width
      .as_ref()
      .and_then(|relative| drawingml_percent_to_ratio(&relative.percentage_width)),
    relative_height_to: anchor
      .wp14_relative_height
      .as_ref()
      .map(|relative| relative_height_reference(relative.relative_from)),
    relative_height_pct: anchor
      .wp14_relative_height
      .as_ref()
      .and_then(|relative| drawingml_percent_to_ratio(&relative.percentage_height)),
    margin_top_pt: margins.top_pt,
    margin_right_pt: margins.right_pt,
    margin_bottom_pt: margins.bottom_pt,
    margin_left_pt: margins.left_pt,
  }
}

fn relative_width_reference(
  value: wp14::SizeRelativeHorizontallyValues,
) -> HorizontalImageReference {
  match value {
    wp14::SizeRelativeHorizontallyValues::Margin => HorizontalImageReference::Margin,
    wp14::SizeRelativeHorizontallyValues::Page => HorizontalImageReference::Page,
    wp14::SizeRelativeHorizontallyValues::LeftMargin => HorizontalImageReference::LeftMargin,
    wp14::SizeRelativeHorizontallyValues::RightMargin => HorizontalImageReference::RightMargin,
    wp14::SizeRelativeHorizontallyValues::InsideMargin => HorizontalImageReference::InsideMargin,
    wp14::SizeRelativeHorizontallyValues::OutsideMargin => HorizontalImageReference::OutsideMargin,
  }
}

fn relative_height_reference(value: wp14::SizeRelativeVerticallyValues) -> VerticalImageReference {
  match value {
    wp14::SizeRelativeVerticallyValues::Margin => VerticalImageReference::Margin,
    wp14::SizeRelativeVerticallyValues::Page => VerticalImageReference::Page,
    wp14::SizeRelativeVerticallyValues::TopMargin => VerticalImageReference::TopMargin,
    wp14::SizeRelativeVerticallyValues::BottomMargin => VerticalImageReference::BottomMargin,
    wp14::SizeRelativeVerticallyValues::InsideMargin => VerticalImageReference::InsideMargin,
    wp14::SizeRelativeVerticallyValues::OutsideMargin => VerticalImageReference::OutsideMargin,
  }
}

#[derive(Clone, Copy, Debug, Default)]
struct ImageWrapMargins {
  top_pt: f32,
  right_pt: f32,
  bottom_pt: f32,
  left_pt: f32,
}

fn floating_wrap_margins(anchor: &wp::Anchor) -> ImageWrapMargins {
  if matches!(
    anchor.anchor_choice.as_ref(),
    Some(wp::AnchorChoice::WrapNone)
  ) {
    return ImageWrapMargins::default();
  }

  let mut margins = ImageWrapMargins {
    top_pt: optional_emu_to_points(anchor.distance_from_top),
    right_pt: optional_emu_to_points(anchor.distance_from_right),
    bottom_pt: optional_emu_to_points(anchor.distance_from_bottom),
    left_pt: optional_emu_to_points(anchor.distance_from_left),
  };

  match anchor.anchor_choice.as_ref() {
    Some(wp::AnchorChoice::WrapSquare(square)) => {
      margins.top_pt = optional_emu_to_points(square.distance_from_top).max(margins.top_pt);
      margins.right_pt = optional_emu_to_points(square.distance_from_right).max(margins.right_pt);
      margins.bottom_pt =
        optional_emu_to_points(square.distance_from_bottom).max(margins.bottom_pt);
      margins.left_pt = optional_emu_to_points(square.distance_from_left).max(margins.left_pt);
    }
    Some(wp::AnchorChoice::WrapTight(tight)) => {
      margins.right_pt = optional_emu_to_points(tight.distance_from_right).max(margins.right_pt);
      margins.left_pt = optional_emu_to_points(tight.distance_from_left).max(margins.left_pt);
    }
    Some(wp::AnchorChoice::WrapThrough(through)) => {
      margins.right_pt = optional_emu_to_points(through.distance_from_right).max(margins.right_pt);
      margins.left_pt = optional_emu_to_points(through.distance_from_left).max(margins.left_pt);
    }
    Some(wp::AnchorChoice::WrapTopBottom(top_bottom)) => {
      margins.top_pt = optional_emu_to_points(top_bottom.distance_from_top).max(margins.top_pt);
      margins.bottom_pt =
        optional_emu_to_points(top_bottom.distance_from_bottom).max(margins.bottom_pt);
    }
    Some(wp::AnchorChoice::WrapNone) | None => {}
  }

  margins
}

fn optional_emu_to_points(value: Option<u32>) -> f32 {
  value
    .map(|value| units::emu_to_points(value as i64))
    .unwrap_or(0.0)
}

fn horizontal_image_reference(position: &wp::HorizontalPosition) -> HorizontalImageReference {
  match position.relative_from {
    wp::HorizontalRelativePositionValues::Page => HorizontalImageReference::Page,
    wp::HorizontalRelativePositionValues::Column => HorizontalImageReference::Column,
    wp::HorizontalRelativePositionValues::Character => HorizontalImageReference::Character,
    wp::HorizontalRelativePositionValues::Margin => HorizontalImageReference::Margin,
    wp::HorizontalRelativePositionValues::LeftMargin => HorizontalImageReference::LeftMargin,
    wp::HorizontalRelativePositionValues::RightMargin => HorizontalImageReference::RightMargin,
    wp::HorizontalRelativePositionValues::InsideMargin => HorizontalImageReference::InsideMargin,
    wp::HorizontalRelativePositionValues::OutsideMargin => HorizontalImageReference::OutsideMargin,
  }
}

fn vertical_image_reference(position: &wp::VerticalPosition) -> VerticalImageReference {
  match position.relative_from {
    wp::VerticalRelativePositionValues::Page => VerticalImageReference::Page,
    wp::VerticalRelativePositionValues::Paragraph => VerticalImageReference::Paragraph,
    wp::VerticalRelativePositionValues::Line => VerticalImageReference::Line,
    wp::VerticalRelativePositionValues::Margin => VerticalImageReference::Margin,
    wp::VerticalRelativePositionValues::TopMargin => VerticalImageReference::TopMargin,
    wp::VerticalRelativePositionValues::BottomMargin => VerticalImageReference::BottomMargin,
    wp::VerticalRelativePositionValues::InsideMargin => VerticalImageReference::InsideMargin,
    wp::VerticalRelativePositionValues::OutsideMargin => VerticalImageReference::OutsideMargin,
  }
}

fn horizontal_position_offset(position: &wp::HorizontalPosition) -> Option<f32> {
  match position.horizontal_position_choice.as_ref()? {
    wp::HorizontalPositionChoice::PositionOffset(offset) => {
      Some(units::emu_to_points(*offset as i64))
    }
    wp::HorizontalPositionChoice::HorizontalAlignment(_)
    | wp::HorizontalPositionChoice::PercentagePositionHeightOffset(_) => None,
  }
}

fn horizontal_position_alignment(
  position: &wp::HorizontalPosition,
) -> Option<HorizontalImageAlignment> {
  match position.horizontal_position_choice.as_ref()? {
    wp::HorizontalPositionChoice::HorizontalAlignment(alignment) => match alignment {
      wp::HorizontalAlignmentValues::Left => Some(HorizontalImageAlignment::Left),
      wp::HorizontalAlignmentValues::Center => Some(HorizontalImageAlignment::Center),
      wp::HorizontalAlignmentValues::Right => Some(HorizontalImageAlignment::Right),
      wp::HorizontalAlignmentValues::Inside => Some(HorizontalImageAlignment::Inside),
      wp::HorizontalAlignmentValues::Outside => Some(HorizontalImageAlignment::Outside),
    },
    wp::HorizontalPositionChoice::PositionOffset(_)
    | wp::HorizontalPositionChoice::PercentagePositionHeightOffset(_) => None,
  }
}

fn vertical_position_offset(position: &wp::VerticalPosition) -> Option<f32> {
  match position.vertical_position_choice.as_ref()? {
    wp::VerticalPositionChoice::PositionOffset(offset) => {
      Some(units::emu_to_points(*offset as i64))
    }
    wp::VerticalPositionChoice::VerticalAlignment(_)
    | wp::VerticalPositionChoice::PercentagePositionVerticalOffset(_) => None,
  }
}

fn vertical_position_alignment(position: &wp::VerticalPosition) -> Option<VerticalImageAlignment> {
  match position.vertical_position_choice.as_ref()? {
    wp::VerticalPositionChoice::VerticalAlignment(alignment) => match alignment {
      wp::VerticalAlignmentValues::Top
        if position.relative_from == wp::VerticalRelativePositionValues::Line =>
      {
        Some(VerticalImageAlignment::Bottom)
      }
      wp::VerticalAlignmentValues::Bottom
        if position.relative_from == wp::VerticalRelativePositionValues::Line =>
      {
        Some(VerticalImageAlignment::Top)
      }
      wp::VerticalAlignmentValues::Top => Some(VerticalImageAlignment::Top),
      wp::VerticalAlignmentValues::Center => Some(VerticalImageAlignment::Center),
      wp::VerticalAlignmentValues::Bottom => Some(VerticalImageAlignment::Bottom),
      wp::VerticalAlignmentValues::Inside => Some(VerticalImageAlignment::Inside),
      wp::VerticalAlignmentValues::Outside => Some(VerticalImageAlignment::Outside),
    },
    wp::VerticalPositionChoice::PositionOffset(_)
    | wp::VerticalPositionChoice::PercentagePositionVerticalOffset(_) => None,
  }
}

fn image_wrap_mode(choice: &wp::AnchorChoice) -> ImageWrapMode {
  match choice {
    wp::AnchorChoice::WrapNone => ImageWrapMode::Through,
    wp::AnchorChoice::WrapSquare(_) => ImageWrapMode::Square,
    wp::AnchorChoice::WrapTight(_) => ImageWrapMode::Tight,
    wp::AnchorChoice::WrapThrough(_) => ImageWrapMode::Through,
    wp::AnchorChoice::WrapTopBottom(_) => ImageWrapMode::TopBottom,
  }
}

fn image_wrap_side(choice: &wp::AnchorChoice) -> ImageWrapSide {
  match choice {
    wp::AnchorChoice::WrapSquare(square) => wrap_text_side(square.wrap_text),
    wp::AnchorChoice::WrapTight(tight) => wrap_text_side(tight.wrap_text),
    wp::AnchorChoice::WrapThrough(through) => wrap_text_side(through.wrap_text),
    wp::AnchorChoice::WrapNone | wp::AnchorChoice::WrapTopBottom(_) => ImageWrapSide::BothSides,
  }
}

fn wrap_text_side(value: wp::WrapTextValues) -> ImageWrapSide {
  match value {
    wp::WrapTextValues::BothSides => ImageWrapSide::BothSides,
    wp::WrapTextValues::Left => ImageWrapSide::Left,
    wp::WrapTextValues::Right => ImageWrapSide::Right,
    wp::WrapTextValues::Largest => ImageWrapSide::Largest,
  }
}

fn push_drawing_textboxes_impl(
  drawing: &w::Drawing,
  inlines: &mut Vec<InlineItem>,
  base_style: TextStyle,
  styles: &StylesCatalog,
  images: &ImageCatalog,
  hyperlinks: &HyperlinkCatalog,
) {
  if drawing_is_hidden(drawing) {
    return;
  }

  let Some(graphic_data) = drawing_graphic_data(drawing) else {
    return;
  };
  if drawing_image_properties(graphic_data, &styles.theme_colors, None).is_some() {
    return;
  }

  let placement = match drawing.drawing_choice.as_ref() {
    Some(w::DrawingChoice::Inline(_)) => ImagePlacement::Inline,
    Some(w::DrawingChoice::Anchor(anchor)) => {
      ImagePlacement::Floating(floating_image_placement(anchor))
    }
    None => return,
  };

  for child in graphic_data.graphic_data_choice.iter() {
    let textbox_context = DrawingTextBoxImportContext {
      base_style: base_style.clone(),
      styles,
      images,
      hyperlinks,
    };
    let text_box_frames = drawing_graphic_data_choice_textbox_frames(
      child,
      placement,
      DrawingMlGroupTransform::identity(),
      textbox_context,
    );
    if !text_box_frames.is_empty() {
      for text_box_frame in text_box_frames {
        if let Err(text_box_frame) = merge_textbox_frame_into_owning_shape(inlines, text_box_frame)
        {
          inlines.push(InlineItem::Shape(*text_box_frame));
        }
      }
      continue;
    }
    if let Some(content) = drawing_graphic_data_choice_textbox_content(child) {
      push_textbox_content(
        &content,
        inlines,
        base_style.clone(),
        styles,
        images,
        hyperlinks,
      );
    }
  }
}

fn merge_textbox_frame_into_owning_shape(
  inlines: &mut [InlineItem],
  mut text_box_frame: InlineShape,
) -> std::result::Result<(), Box<InlineShape>> {
  // wps:spPr and wps:txbx are children of the same wps:wsp. A textbox frame
  // without independent visual treatment is therefore content of the
  // preceding geometry, not a second layout object. Horizontal spAutoFit
  // frames carry a synthetic fallback stroke and wrap mode, but those must
  // not duplicate or replace the owning wps:spPr geometry.
  if text_box_frame.fill_color.is_some()
    || text_box_frame.fill_image.is_some()
    || (text_box_frame.stroke.is_some() && !text_box_frame.text_box_auto_fit)
    || !text_box_frame.additional_fill_colors.is_empty()
    || text_box_frame.chart.is_some()
    || (text_box_frame.placement != ImagePlacement::Inline
      && !text_box_frame.text_box_resizes_height_to_fit)
  {
    return Err(Box::new(text_box_frame));
  }
  let Some(shape) = inlines.iter_mut().rev().find_map(|inline| {
    let InlineItem::Shape(shape) = inline else {
      return None;
    };
    (shape.text_box_blocks.is_empty()
      && shape.chart.is_none()
      && textbox_owner_placement_matches(
        shape.placement,
        text_box_frame.placement,
        text_box_frame.text_box_resizes_height_to_fit,
      )
      && (shape.width_pt - text_box_frame.width_pt).abs() <= 0.01
      && (shape.height_pt - text_box_frame.height_pt).abs() <= 0.01
      && (shape.offset_x_pt - text_box_frame.offset_x_pt).abs() <= 0.01
      && (shape.offset_y_pt - text_box_frame.offset_y_pt).abs() <= 0.01)
      .then_some(shape)
  }) else {
    return Err(Box::new(text_box_frame));
  };

  shape.text_box_blocks = std::mem::take(&mut text_box_frame.text_box_blocks);
  shape.text_inset_left_pt = text_box_frame.text_inset_left_pt;
  shape.text_inset_top_pt = text_box_frame.text_inset_top_pt;
  shape.text_inset_right_pt = text_box_frame.text_inset_right_pt;
  shape.text_inset_bottom_pt = text_box_frame.text_inset_bottom_pt;
  shape.text_box_auto_fit = text_box_frame.text_box_auto_fit;
  shape.text_box_resizes_height_to_fit = text_box_frame.text_box_resizes_height_to_fit;
  shape.text_box_word_wrap = text_box_frame.text_box_word_wrap;
  shape.text_vertical_alignment = text_box_frame.text_vertical_alignment;
  shape.text_fill = text_box_frame.text_fill.take();
  Ok(())
}

fn textbox_owner_placement_matches(
  owner: ImagePlacement,
  frame: ImagePlacement,
  allow_auto_fit_fallbacks: bool,
) -> bool {
  if owner == frame {
    return true;
  }
  let (ImagePlacement::Floating(owner), ImagePlacement::Floating(frame)) = (owner, frame) else {
    return false;
  };
  allow_auto_fit_fallbacks
    && owner.horizontal_relative_to == frame.horizontal_relative_to
    && owner.vertical_relative_to == frame.vertical_relative_to
    && owner.horizontal_alignment == frame.horizontal_alignment
    && owner.vertical_alignment == frame.vertical_alignment
    && owner.horizontal_offset_pt == frame.horizontal_offset_pt
    && owner.vertical_offset_pt == frame.vertical_offset_pt
    && owner.wrap_side == frame.wrap_side
    && owner.behind_text == frame.behind_text
    && owner.layout_in_cell == frame.layout_in_cell
    && owner.allow_overlap == frame.allow_overlap
    && owner.paint_order == frame.paint_order
    && owner.relative_width_to == frame.relative_width_to
    && owner.relative_width_pct == frame.relative_width_pct
    && owner.relative_height_to == frame.relative_height_to
    && owner.relative_height_pct == frame.relative_height_pct
}

#[derive(Clone)]
struct DrawingTextBoxImportContext<'a> {
  base_style: TextStyle,
  styles: &'a StylesCatalog,
  images: &'a ImageCatalog,
  hyperlinks: &'a HyperlinkCatalog,
}

#[derive(Clone, Copy)]
struct DrawingShapeImportContext<'a> {
  effect_extent: DrawingEffectExtent,
  styles: &'a StylesCatalog,
  images: &'a ImageCatalog,
  hyperlinks: &'a HyperlinkCatalog,
  smartart_text_colors_by_model_id: Option<&'a HashMap<String, RgbColor>>,
}

fn autofit_textbox_placement(placement: ImagePlacement) -> ImagePlacement {
  match placement {
    ImagePlacement::Floating(mut placement) => {
      // content inside the owning draw shape (SwTextBoxHelper), so text flow
      // must not be wrapped into the shape's textbox area.
      placement.wrap = ImageWrapMode::TopBottom;
      ImagePlacement::Floating(placement)
    }
    ImagePlacement::Inline => ImagePlacement::Inline,
  }
}

fn wordprocessing_shape_textbox_uses_auto_fit(shape: &wps::WordprocessingShape) -> bool {
  matches!(
    shape
      .text_body_properties
      .as_deref()
      .and_then(|properties| properties.text_body_properties_choice1.as_ref()),
    Some(wps::TextBodyPropertiesChoice::ShapeAutoFit)
  )
}

fn wordprocessing_shape_textbox_is_vertical(shape: &wps::WordprocessingShape) -> bool {
  matches!(
    shape
      .text_body_properties
      .as_ref()
      .and_then(|properties| properties.vertical),
    Some(a::TextVerticalValues::Vertical)
  )
}

fn wordprocessing_shape_textbox_fontwork_warp(
  shape: &wps::WordprocessingShape,
) -> Option<Box<a::PresetTextWarp>> {
  shape
    .text_body_properties
    .as_deref()
    .and_then(|properties| properties.preset_text_warp.as_ref())
    .filter(|warp| warp.preset != a::TextShapeValues::TextNoShape)
    .cloned()
}

fn legacy_fontwork_warp_geometry() -> InlineShapeGeometry {
  const SEGMENTS: usize = 16;
  let mut points = Vec::with_capacity(SEGMENTS * 2 + 1);
  for index in 0..=SEGMENTS {
    let t = index as f32 / SEGMENTS as f32;
    let y = 0.12 + (t * std::f32::consts::PI).sin() * 0.18;
    points.push((t, y));
  }
  for index in (0..=SEGMENTS).rev() {
    let t = index as f32 / SEGMENTS as f32;
    let y = 0.88 - (t * std::f32::consts::PI).sin() * 0.18;
    points.push((t, y));
  }
  if points.last() != points.first() {
    points.push(points[0]);
  }
  InlineShapeGeometry::Polyline {
    points,
    closed: true,
  }
}

fn drawingml_text_fill_colors_from_effect(
  fill: &w14::FillTextEffect,
  theme_colors: &ThemeColors,
) -> Vec<RgbColor> {
  match fill.fill_text_effect_choice.as_ref() {
    None => Vec::new(),
    Some(w14::FillTextEffectChoice::NoFillEmpty) => Vec::new(),
    Some(w14::FillTextEffectChoice::SolidColorFillProperties(fill)) => {
      resolve_solid_text_fill(fill, theme_colors)
        .map(|color| color.color)
        .into_iter()
        .collect()
    }
    Some(w14::FillTextEffectChoice::GradientFillProperties(fill)) => {
      drawingml_w14_gradient_fill_colors(fill, theme_colors)
    }
  }
}

fn drawingml_w14_gradient_fill_colors(
  fill: &w14::GradientFillProperties,
  theme_colors: &ThemeColors,
) -> Vec<RgbColor> {
  fill
    .gradient_stop_list
    .as_ref()
    .into_iter()
    .flat_map(|list| &list.gradient_stop)
    .filter_map(|stop| match stop.gradient_stop_choice.as_ref()? {
      w14::GradientStopChoice::RgbColorModelHex(color) => parse_hex_color(color.val.as_str()),
      w14::GradientStopChoice::SchemeColor(color) => {
        let mut resolved = theme_colors.resolve_word2010(color.val)?;
        resolved = apply_w14_scheme_transforms(resolved, &color.scheme_color_choice);
        Some(resolved)
      }
    })
    .collect()
}

fn drawingml_w14_gradient_fill(
  fill: &w14::GradientFillProperties,
  theme_colors: &ThemeColors,
) -> Option<common::Fill<'static>> {
  let mut stops = fill
    .gradient_stop_list
    .as_ref()?
    .gradient_stop
    .iter()
    .filter_map(|stop| {
      let resolved = match stop.gradient_stop_choice.as_ref()? {
        w14::GradientStopChoice::RgbColorModelHex(color) => ResolvedColor {
          color: parse_hex_color(color.val.as_str())?,
          opacity: opacity_from_w14_rgb_transforms(&color.rgb_color_model_hex_choice),
        },
        w14::GradientStopChoice::SchemeColor(color) => ResolvedColor {
          color: apply_w14_scheme_transforms(
            theme_colors.resolve_word2010(color.val)?,
            &color.scheme_color_choice,
          ),
          opacity: opacity_from_w14_scheme_transforms(&color.scheme_color_choice),
        },
      };
      Some(common::GradientStop {
        position: stop.stop_position as f32 / 100_000.0,
        color: common_rgb(resolved.color, resolved.opacity),
        scheme: None,
      })
    })
    .collect::<Vec<_>>();
  stops.sort_by(|left, right| left.position.total_cmp(&right.position));
  if stops.is_empty() {
    return None;
  }
  let (angle_degrees, scaled, path) = match fill.gradient_fill_properties_choice.as_ref()? {
    w14::GradientFillPropertiesChoice::LinearShadeProperties(linear) => (
      Some(linear.angle.unwrap_or_default() as f32 / 60_000.0),
      linear
        .scaled
        .is_some_and(|value| matches!(value, w14::OnOffValues::True | w14::OnOffValues::One)),
      None,
    ),
    w14::GradientFillPropertiesChoice::PathShadeProperties(path) => {
      let fill_to = path
        .fill_to_rectangle
        .as_ref()
        .map(|rect| common::RelativeRect {
          left: rect.left.unwrap_or(50_000) as f32 / 100_000.0,
          top: rect.top.unwrap_or(50_000) as f32 / 100_000.0,
          right: rect.right.unwrap_or(50_000) as f32 / 100_000.0,
          bottom: rect.bottom.unwrap_or(50_000) as f32 / 100_000.0,
        })
        .unwrap_or(common::RelativeRect {
          left: 0.5,
          top: 0.5,
          right: 0.5,
          bottom: 0.5,
        });
      let kind = match path.path.unwrap_or_default() {
        w14::PathShadeTypeValues::Shape => common::GradientPathKind::Shape,
        w14::PathShadeTypeValues::Circle => common::GradientPathKind::Circle,
        w14::PathShadeTypeValues::Rect => common::GradientPathKind::Rectangle,
      };
      (
        None,
        false,
        Some(common::GradientPath {
          kind,
          fill_to,
          transform: common::Transform::default(),
          mirror_tile: false,
        }),
      )
    }
  };
  Some(common::Fill::Gradient(common::GradientFill {
    stops,
    angle_degrees,
    definition_bounds: None,
    line: None,
    interpolation: common::GradientInterpolation::LinearSrgb,
    scaled,
    rotate_with_shape: Some(true),
    path,
  }))
}

pub(super) fn drawingml_text_effect_common_fill(
  fill: &w14::FillTextEffect,
  theme_colors: &ThemeColors,
) -> Option<common::Fill<'static>> {
  match fill.fill_text_effect_choice.as_ref()? {
    w14::FillTextEffectChoice::NoFillEmpty => Some(common::Fill::None),
    w14::FillTextEffectChoice::SolidColorFillProperties(fill) => {
      let resolved = resolve_solid_text_fill(fill, theme_colors)?;
      Some(common::Fill::Solid(common_rgb(
        resolved.color,
        resolved.opacity,
      )))
    }
    w14::FillTextEffectChoice::GradientFillProperties(fill) => {
      drawingml_w14_gradient_fill(fill, theme_colors)
    }
  }
}

pub(super) fn drawingml_text_outline_effect_common_fill(
  outline: &w14::TextOutlineEffect,
  theme_colors: &ThemeColors,
) -> Option<common::Fill<'static>> {
  match outline.text_outline_effect_choice1.as_ref()? {
    w14::TextOutlineEffectChoice::NoFillEmpty => Some(common::Fill::None),
    w14::TextOutlineEffectChoice::SolidColorFillProperties(fill) => {
      let resolved = resolve_solid_text_fill(fill, theme_colors)?;
      Some(common::Fill::Solid(common_rgb(
        resolved.color,
        resolved.opacity,
      )))
    }
    w14::TextOutlineEffectChoice::GradientFillProperties(fill) => {
      drawingml_w14_gradient_fill(fill, theme_colors)
    }
  }
}

fn wordprocessing_textbox_common_fill(
  content: &w::TextBoxContent,
  theme_colors: &ThemeColors,
) -> Option<common::Fill<'static>> {
  for paragraph in content
    .text_box_content_choice
    .iter()
    .filter_map(|choice| match choice {
      w::TextBoxContentChoice::Paragraph(paragraph) => Some(paragraph.as_ref()),
      _ => None,
    })
  {
    if let Some(fill) = paragraph
      .paragraph_properties
      .as_deref()
      .and_then(|properties| properties.paragraph_mark_run_properties.as_deref())
      .and_then(|properties| properties.fill_text_effect.as_deref())
      .and_then(|fill| drawingml_text_effect_common_fill(fill, theme_colors))
    {
      return Some(fill);
    }
    for run in paragraph
      .paragraph_choice
      .iter()
      .filter_map(|choice| match choice {
        w::ParagraphChoice::WRun(run) => Some(run.as_ref()),
        _ => None,
      })
    {
      if let Some(fill) = wordprocessing_run_common_fill(run, theme_colors) {
        return Some(fill);
      }
    }
  }
  None
}

fn wordprocessing_run_common_fill(
  run: &w::Run,
  theme_colors: &ThemeColors,
) -> Option<common::Fill<'static>> {
  if let Some(fill) = run
    .run_properties
    .as_deref()
    .and_then(|properties| properties.fill_text_effect.as_deref())
    .and_then(|fill| drawingml_text_effect_common_fill(fill, theme_colors))
  {
    return Some(fill);
  }
  run
    .run_choice
    .iter()
    .filter_map(|choice| match choice {
      w::RunChoice::Run(run) => Some(run.as_ref()),
      _ => None,
    })
    .find_map(|run| wordprocessing_run_common_fill(run, theme_colors))
}

fn wordprocessing_textbox_fill_colors(
  content: &w::TextBoxContent,
  theme_colors: &ThemeColors,
) -> Vec<RgbColor> {
  let mut colors = Vec::new();
  for paragraph in content
    .text_box_content_choice
    .iter()
    .filter_map(|choice| match choice {
      w::TextBoxContentChoice::Paragraph(paragraph) => Some(paragraph.as_ref()),
      _ => None,
    })
  {
    if let Some(fill) = paragraph
      .paragraph_properties
      .as_deref()
      .and_then(|properties| properties.paragraph_mark_run_properties.as_deref())
      .and_then(|properties| properties.fill_text_effect.as_deref())
    {
      colors.extend(drawingml_text_fill_colors_from_effect(fill, theme_colors));
    }
    for run in paragraph
      .paragraph_choice
      .iter()
      .filter_map(|choice| match choice {
        w::ParagraphChoice::WRun(run) => Some(run.as_ref()),
        _ => None,
      })
    {
      wordprocessing_run_fill_colors(run, theme_colors, &mut colors);
    }
  }
  colors
}

fn wordprocessing_run_fill_colors(
  run: &w::Run,
  theme_colors: &ThemeColors,
  colors: &mut Vec<RgbColor>,
) {
  if let Some(fill) = run
    .run_properties
    .as_deref()
    .and_then(|properties| properties.fill_text_effect.as_deref())
  {
    colors.extend(drawingml_text_fill_colors_from_effect(fill, theme_colors));
  }
  for nested in run.run_choice.iter().filter_map(|choice| match choice {
    w::RunChoice::Run(run) => Some(run.as_ref()),
    _ => None,
  }) {
    wordprocessing_run_fill_colors(nested, theme_colors, colors);
  }
}

fn first_text_color_in_blocks(blocks: &[Block]) -> Option<RgbColor> {
  blocks.iter().find_map(first_text_color_in_block)
}

fn first_text_color_in_block(block: &Block) -> Option<RgbColor> {
  match block {
    Block::Paragraph(paragraph) => paragraph.inlines.iter().find_map(|inline| match inline {
      InlineItem::Text(run) if !run.text.is_empty() => Some(run.style.color),
      InlineItem::Shape(shape) => first_text_color_in_blocks(&shape.text_box_blocks),
      _ => None,
    }),
    Block::Table(table) => table
      .rows
      .iter()
      .flat_map(|row| &row.cells)
      .find_map(|cell| first_text_color_in_blocks(&cell.blocks)),
    Block::Frame(frame) => first_text_color_in_blocks(&frame.blocks),
  }
}

fn wordprocessing_shape_textbox_frame_stroke(
  shape: &wps::WordprocessingShape,
  auto_fit: bool,
  placement: ImagePlacement,
) -> Option<BorderStyle> {
  if wordprocessing_shape_has_no_line(shape) {
    return None;
  }
  let suppress_zero_width_relative_frame = matches!(
    placement,
    ImagePlacement::Floating(FloatingImagePlacement {
      relative_width_pct: Some(width_pct),
      relative_height_pct: Some(height_pct),
      ..
    }) if width_pct <= 0.0 && height_pct > 0.0
  );
  (auto_fit && !suppress_zero_width_relative_frame).then_some(BorderStyle::default())
}

#[derive(Clone, Debug)]
struct TextBoxFrameContent {
  blocks: Vec<Block>,
  left_pt: f32,
  top_pt: f32,
  right_pt: f32,
  bottom_pt: f32,
  word_wrap: bool,
  vertical_alignment: TextBoxVerticalAlignment,
}

impl TextBoxFrameContent {
  fn new(blocks: Vec<Block>) -> Self {
    Self {
      blocks,
      left_pt: DEFAULT_TEXTBOX_LEFT_RIGHT_INSET_PT,
      top_pt: DEFAULT_TEXTBOX_TOP_BOTTOM_INSET_PT,
      right_pt: DEFAULT_TEXTBOX_LEFT_RIGHT_INSET_PT,
      bottom_pt: DEFAULT_TEXTBOX_TOP_BOTTOM_INSET_PT,
      word_wrap: true,
      vertical_alignment: TextBoxVerticalAlignment::Top,
    }
  }
}

fn text_box_frame_from_wordprocessing_shape(
  shape: &wps::WordprocessingShape,
  content: &w::TextBoxContent,
  mut base_style: TextStyle,
  styles: &StylesCatalog,
  images: &ImageCatalog,
  hyperlinks: &HyperlinkCatalog,
) -> TextBoxFrameContent {
  // ECMA-376 Part 1 §20.1.4.1.17 carries an explicit shape-style text
  // color through a:fontRef, while §17.3.2.6 lets an automatic run color
  // adapt to its background. [MS-OI29500] §20.1.2.2.37 gives textbox
  // content precedence over the shape style.
  if let Some(color) = shape
    .shape_style
    .as_ref()
    .and_then(|style| drawingml_font_reference_color(&style.font_reference, &styles.theme_colors))
    .or_else(|| {
      wordprocessing_shape_fill_color(shape, &styles.theme_colors)
        .map(automatic_text_color_for_background)
    })
  {
    base_style.color = color;
  }
  let mut frame = TextBoxFrameContent::new(textbox_blocks_with_base(
    content, base_style, styles, images, hyperlinks,
  ));
  if let Some(properties) = shape.text_body_properties.as_deref() {
    apply_wordprocessing_shape_textbox_body_properties(properties, &mut frame);
  }
  if let Some(rotation_deg) = wordprocessing_shape_textbox_text_rotation(shape) {
    rotate_textbox_blocks(&mut frame.blocks, rotation_deg);
  }
  let shape_auto_fit = wordprocessing_shape_textbox_uses_auto_fit(shape);
  let fixed_inline_picture_outline_inset_pt = (!shape_auto_fit
    && text_box_is_single_inline_picture(&frame.blocks))
  .then(|| wordprocessing_shape_no_fill_outline_half_width_pt(shape))
  .flatten();
  apply_drawingml_textbox_layout_adjustments(
    &mut frame,
    shape_auto_fit,
    fixed_inline_picture_outline_inset_pt,
  );
  frame
}

fn text_box_is_single_inline_picture(blocks: &[Block]) -> bool {
  matches!(
    blocks,
    [Block::Paragraph(paragraph)]
      if matches!(
        paragraph.inlines.as_slice(),
        [InlineItem::Image(image)] if image.placement == ImagePlacement::Inline
      )
  )
}

fn wordprocessing_shape_no_fill_outline_half_width_pt(
  shape: &wps::WordprocessingShape,
) -> Option<f32> {
  let outline = shape.shape_properties.as_deref()?.outline.as_deref()?;
  matches!(
    outline.outline_choice1.as_ref(),
    Some(a::OutlineChoice::NoFill(_))
  )
  .then(|| {
    outline
      .width
      .map(i64::from)
      .map(units::emu_to_points)
      .unwrap_or_else(|| units::emu_to_points(DRAWINGML_DEFAULT_LINE_WIDTH_EMU))
      / 2.0
  })
}

fn wordprocessing_shape_textbox_text_rotation(shape: &wps::WordprocessingShape) -> Option<f32> {
  let properties = shape.text_body_properties.as_deref()?;
  let vertical_rotation = match properties.vertical {
    Some(a::TextVerticalValues::Vertical)
    | Some(a::TextVerticalValues::WordArtVertical)
    | Some(a::TextVerticalValues::EastAsianVetical) => 90.0,
    Some(a::TextVerticalValues::Vertical270) | Some(a::TextVerticalValues::WordArtLeftToRight) => {
      -90.0
    }
    _ => 0.0,
  };
  let text_area_rotation = if properties
    .up_right
    .as_ref()
    .is_some_and(|value| value.as_bool())
  {
    0.0
  } else {
    properties
      .rotation
      .map(|value| sdk_units::drawingml_angle_to_degrees(value) as f32)
      .unwrap_or_default()
  };
  let rotation = vertical_rotation + text_area_rotation;
  (rotation.abs() > f32::EPSILON).then_some(rotation)
}

fn rotate_textbox_blocks(blocks: &mut [Block], rotation_deg: f32) {
  rotate_blocks_text(blocks, rotation_deg);
}

fn rotate_blocks_text(blocks: &mut [Block], rotation_deg: f32) {
  for block in blocks {
    match block {
      Block::Paragraph(paragraph) => rotate_paragraph_text(paragraph, rotation_deg),
      Block::Table(table) => {
        for row in &mut table.rows {
          for cell in &mut row.cells {
            rotate_textbox_blocks(&mut cell.blocks, rotation_deg);
          }
        }
      }
      Block::Frame(frame) => rotate_textbox_blocks(&mut frame.blocks, rotation_deg),
    }
  }
}

fn rotate_paragraph_text(paragraph: &mut Paragraph, rotation_deg: f32) {
  for inline in &mut paragraph.inlines {
    if let InlineItem::Text(run) = inline {
      run.style.rotation_deg = rotation_deg;
    }
  }
  #[cfg(test)]
  for run in &mut paragraph.runs {
    run.style.rotation_deg = rotation_deg;
  }
  paragraph.list_label_style.rotation_deg = rotation_deg;
}

fn apply_drawingml_textbox_layout_adjustments(
  frame: &mut TextBoxFrameContent,
  shape_auto_fit: bool,
  fixed_inline_picture_outline_inset_pt: Option<f32>,
) {
  // WpsContext maps spAutoFit text boxes to an automatically sized text
  // frame. Word keeps the authored bodyPr inset for that frame; the legacy
  // fixed-size custom-shape path retains the existing drawing adjustment.
  if shape_auto_fit {
    return;
  }
  if let Some(outline_inset_pt) = fixed_inline_picture_outline_inset_pt {
    // WpsContext.cxx maps bodyPr insets literally and textboxhelper.cxx owns
    // the paired Writer text frame. In a fixed frame containing one ordinary
    // inline picture, Word positions that frame from the inside edge of the
    // authored (possibly hidden) outline. Keep the correction bounded:
    // locked canvases and mixed/text content retain the legacy shape path.
    frame.left_pt += outline_inset_pt;
    frame.top_pt += outline_inset_pt;
    frame.right_pt += outline_inset_pt;
    frame.bottom_pt += outline_inset_pt;
  } else {
    frame.left_pt = (frame.left_pt - 1.67).max(0.0);
  }
}

fn automatic_text_color_for_background(color: RgbColor) -> RgbColor {
  // Black and white have equal WCAG contrast against a background at
  // sqrt(1.05 * 0.05) - 0.05 = 0.1791 relative luminance.
  if color_wcag_relative_luminance(color) <= 0.179_129 {
    RgbColor {
      r: 255,
      g: 255,
      b: 255,
    }
  } else {
    RgbColor { r: 0, g: 0, b: 0 }
  }
}

fn color_wcag_relative_luminance(color: RgbColor) -> f32 {
  color_math::relative_luminance([color.r, color.g, color.b])
}

fn apply_wordprocessing_shape_textbox_body_properties(
  properties: &wps::TextBodyProperties,
  frame: &mut TextBoxFrameContent,
) {
  // oox/source/shape/WpsContext.cxx maps bodyPr@wrap=square to
  // TextWordWrap=true and every other authored value to false. Preserve the
  // schema default (square) when the attribute is absent.
  frame.word_wrap = properties
    .wrap
    .is_none_or(|wrap| wrap == a::TextWrappingValues::Square);
  let body_properties = DrawingMlBodyProperties {
    left_inset_emu: properties.left_inset.map(i64::from),
    top_inset_emu: properties.top_inset.map(i64::from),
    right_inset_emu: properties.right_inset.map(i64::from),
    bottom_inset_emu: properties.bottom_inset.map(i64::from),
    anchor: properties.anchor,
  };
  apply_drawingml_textbox_body_properties_model(body_properties, frame);
}

fn apply_drawingml_textbox_body_properties_model(
  properties: DrawingMlBodyProperties,
  frame: &mut TextBoxFrameContent,
) {
  frame.left_pt = properties
    .left_inset_emu
    .map(units::emu_to_points)
    .unwrap_or(frame.left_pt);
  frame.top_pt = properties
    .top_inset_emu
    .map(units::emu_to_points)
    .unwrap_or(frame.top_pt);
  frame.right_pt = properties
    .right_inset_emu
    .map(units::emu_to_points)
    .unwrap_or(frame.right_pt);
  frame.bottom_pt = properties
    .bottom_inset_emu
    .map(units::emu_to_points)
    .unwrap_or(frame.bottom_pt);
  frame.vertical_alignment = match properties.anchor {
    Some(a::TextAnchoringTypeValues::Center) => TextBoxVerticalAlignment::Center,
    Some(a::TextAnchoringTypeValues::Bottom) => TextBoxVerticalAlignment::Bottom,
    _ => frame.vertical_alignment,
  };
}

#[derive(Clone, Copy, Debug, Default)]
struct DrawingMlBodyProperties {
  left_inset_emu: Option<i64>,
  top_inset_emu: Option<i64>,
  right_inset_emu: Option<i64>,
  bottom_inset_emu: Option<i64>,
  anchor: Option<a::TextAnchoringTypeValues>,
}

fn drawingml_body_properties_from_model(properties: &a::BodyProperties) -> DrawingMlBodyProperties {
  DrawingMlBodyProperties {
    left_inset_emu: properties.left_inset.map(|value| value.to_emu()),
    top_inset_emu: properties.top_inset.map(|value| value.to_emu()),
    right_inset_emu: properties.right_inset.map(|value| value.to_emu()),
    bottom_inset_emu: properties.bottom_inset.map(|value| value.to_emu()),
    anchor: properties.anchor,
  }
}

fn root_qname(xml: &str) -> Option<&str> {
  let start = xml.trim_start().strip_prefix('<')?;
  start
    .split(|character: char| {
      character.is_ascii_whitespace() || character == '>' || character == '/'
    })
    .next()
}

fn drawing_graphic_data(drawing: &w::Drawing) -> Option<&ooxmlsdk::schemas::a::GraphicData> {
  match drawing.drawing_choice.as_ref()? {
    w::DrawingChoice::Inline(inline) => Some(&inline.graphic.graphic_data),
    w::DrawingChoice::Anchor(anchor) => Some(&anchor.graphic.graphic_data),
  }
}

fn drawing_graphic_data_choice_textbox_frames(
  choice: &a::GraphicDataChoice,
  placement: ImagePlacement,
  transform: DrawingMlGroupTransform,
  context: DrawingTextBoxImportContext<'_>,
) -> Vec<InlineShape> {
  match choice {
    a::GraphicDataChoice::WordprocessingShape(shape) => {
      wordprocessing_shape_textbox_frame(shape, placement, transform, context)
        .into_iter()
        .collect()
    }
    a::GraphicDataChoice::WordprocessingGroup(group) => {
      wordprocessing_group_textbox_frames(group, placement, transform, context)
    }
    a::GraphicDataChoice::WordprocessingCanvas(canvas) => {
      wordprocessing_canvas_textbox_frames(canvas, placement, transform, context)
    }
    a::GraphicDataChoice::XmlAny(xml) => strict_wordprocessing_shape(xml)
      .and_then(|shape| wordprocessing_shape_textbox_frame(&shape, placement, transform, context))
      .into_iter()
      .collect(),
    _ => Vec::new(),
  }
}

fn strict_wordprocessing_shape(xml: &[u8]) -> Option<wps::WordprocessingShape> {
  let xml = std::str::from_utf8(xml).ok()?;
  if root_qname(xml) != Some("wp:wsp") {
    return None;
  }
  wps::WordprocessingShape::from_bytes(xml.as_bytes()).ok()
}

fn wordprocessing_canvas_textbox_frames(
  canvas: &wpc::WordprocessingCanvas,
  placement: ImagePlacement,
  transform: DrawingMlGroupTransform,
  context: DrawingTextBoxImportContext<'_>,
) -> Vec<InlineShape> {
  canvas
    .wordprocessing_canvas_choice
    .iter()
    .flat_map(|choice| {
      wordprocessing_canvas_choice_textbox_frames(choice, placement, transform, context.clone())
    })
    .collect()
}

fn wordprocessing_canvas_choice_textbox_frames(
  choice: &wpc::WordprocessingCanvasChoice,
  placement: ImagePlacement,
  transform: DrawingMlGroupTransform,
  context: DrawingTextBoxImportContext<'_>,
) -> Vec<InlineShape> {
  match choice {
    wpc::WordprocessingCanvasChoice::WordprocessingShape(shape) => {
      wordprocessing_shape_textbox_frame(shape, placement, transform, context)
        .into_iter()
        .collect()
    }
    wpc::WordprocessingCanvasChoice::WordprocessingGroup(group) => {
      wordprocessing_group_textbox_frames(group, placement, transform, context)
    }
    _ => Vec::new(),
  }
}

fn wordprocessing_group_textbox_frames(
  group: &wpg::WordprocessingGroup,
  placement: ImagePlacement,
  transform: DrawingMlGroupTransform,
  context: DrawingTextBoxImportContext<'_>,
) -> Vec<InlineShape> {
  let child_transform = drawingml_group_transform_from_properties(
    &group.group_shape_properties,
    transform.raw_coordinates,
  )
  .map(|xfrm| transform.child(xfrm))
  .unwrap_or(transform);
  group
    .wordprocessing_group_choice
    .iter()
    .flat_map(|choice| {
      wordprocessing_group_choice_textbox_frames(
        choice,
        drawingml_group_child_placement(placement),
        child_transform,
        context.clone(),
      )
    })
    .collect()
}

fn wordprocessing_group_shape_textbox_frames(
  group: &wpg::GroupShape,
  placement: ImagePlacement,
  transform: DrawingMlGroupTransform,
  context: DrawingTextBoxImportContext<'_>,
) -> Vec<InlineShape> {
  let child_transform = drawingml_group_transform_from_properties(
    &group.group_shape_properties,
    transform.raw_coordinates,
  )
  .map(|xfrm| transform.child(xfrm))
  .unwrap_or(transform);
  group
    .group_shape_choice
    .iter()
    .flat_map(|choice| {
      wordprocessing_group_shape_choice_textbox_frames(
        choice,
        drawingml_group_child_placement(placement),
        child_transform,
        context.clone(),
      )
    })
    .collect()
}

fn wordprocessing_group_choice_textbox_frames(
  choice: &wpg::WordprocessingGroupChoice,
  placement: ImagePlacement,
  transform: DrawingMlGroupTransform,
  context: DrawingTextBoxImportContext<'_>,
) -> Vec<InlineShape> {
  match choice {
    wpg::WordprocessingGroupChoice::WordprocessingShape(shape) => {
      wordprocessing_shape_textbox_frame(shape, placement, transform, context)
        .into_iter()
        .collect()
    }
    wpg::WordprocessingGroupChoice::GroupShape(group) => {
      wordprocessing_group_shape_textbox_frames(group, placement, transform, context)
    }
    _ => Vec::new(),
  }
}

fn wordprocessing_group_shape_choice_textbox_frames(
  choice: &wpg::GroupShapeChoice,
  placement: ImagePlacement,
  transform: DrawingMlGroupTransform,
  context: DrawingTextBoxImportContext<'_>,
) -> Vec<InlineShape> {
  match choice {
    wpg::GroupShapeChoice::WordprocessingShape(shape) => {
      wordprocessing_shape_textbox_frame(shape, placement, transform, context)
        .into_iter()
        .collect()
    }
    wpg::GroupShapeChoice::GroupShape(group) => {
      wordprocessing_group_shape_textbox_frames(group, placement, transform, context)
    }
    _ => Vec::new(),
  }
}

fn wordprocessing_shape_textbox_frame(
  shape: &wps::WordprocessingShape,
  placement: ImagePlacement,
  transform: DrawingMlGroupTransform,
  context: DrawingTextBoxImportContext<'_>,
) -> Option<InlineShape> {
  let content = wordprocessing_shape_textbox_content(shape)?;
  let shape_properties = shape
    .shape_properties
    .as_deref()
    .cloned()
    .unwrap_or_default();
  let mut text_box = text_box_frame_from_wordprocessing_shape(
    shape,
    content,
    context.base_style,
    context.styles,
    context.images,
    context.hyperlinks,
  );
  let auto_fit = wordprocessing_shape_textbox_uses_auto_fit(shape);
  let expands_auto_fit = auto_fit && wordprocessing_shape_textbox_is_vertical(shape);
  let frame_stroke = wordprocessing_shape_textbox_frame_stroke(shape, auto_fit, placement);
  let properties = DrawingMlShapeProperties::Wordprocessing(shape_properties);
  let geometry = properties
    .geometry_kind()
    .unwrap_or(InlineShapeGeometry::Rectangle);
  let (offset_x_pt, offset_y_pt, shape_width_pt, shape_height_pt) =
    drawingml_geometry_from_shape_properties(
      Some(&properties),
      &geometry,
      transform.raw_coordinates,
      None,
    )?;
  let mapped = transform.map_rect(
    (offset_x_pt, offset_y_pt, shape_width_pt, shape_height_pt),
    (
      properties.rotation_deg(),
      properties.flip_horizontal(),
      properties.flip_vertical(),
    ),
  );
  apply_wordprocessing_shape_preset_text_rectangle(
    &properties,
    (mapped.width_pt, mapped.height_pt),
    (
      wordprocessing_shape_textbox_text_rotation(shape).unwrap_or_default(),
      mapped.rotation_deg,
    ),
    (mapped.flip_horizontal, mapped.flip_vertical),
    &mut text_box,
  );
  let (offset_x_pt, offset_y_pt, shape_width_pt, shape_height_pt) =
    (mapped.x_pt, mapped.y_pt, mapped.width_pt, mapped.height_pt);
  let width_pt = if expands_auto_fit {
    shape_width_pt.max(DEFAULT_TEXTBOX_AUTO_FIT_WIDTH_PT)
  } else {
    shape_width_pt.max(DEFAULT_TEXTBOX_MIN_WIDTH_PT)
  };
  let height_pt = if expands_auto_fit {
    shape_height_pt.max(300.0)
  } else {
    shape_height_pt.max(DEFAULT_TEXTBOX_MIN_HEIGHT_PT)
  };
  let text_warp = wordprocessing_shape_textbox_fontwork_warp(shape);
  let has_fontwork_warp = text_warp.is_some();
  let text_fill = has_fontwork_warp
    .then(|| wordprocessing_textbox_common_fill(content, &context.styles.theme_colors))
    .flatten();
  let mut wordart_fill_colors = if has_fontwork_warp {
    wordprocessing_textbox_fill_colors(content, &context.styles.theme_colors)
  } else {
    Vec::new()
  };
  if wordart_fill_colors.is_empty()
    && has_fontwork_warp
    && let Some(color) = first_text_color_in_blocks(&text_box.blocks)
  {
    wordart_fill_colors.push(color);
  }
  let fill_color = wordart_fill_colors.first().copied();
  let additional_fill_colors = wordart_fill_colors.into_iter().skip(1).collect();
  let geometry = InlineShapeGeometry::Rectangle;
  let placement = if auto_fit {
    autofit_textbox_placement(placement)
  } else {
    placement
  };

  Some(InlineShape {
    width_pt,
    height_pt,
    effect_left_pt: 0.0,
    effect_top_pt: 0.0,
    effect_right_pt: 0.0,
    effect_bottom_pt: 0.0,
    geometry,
    offset_x_pt,
    offset_y_pt,
    rotation_deg: properties.camera_adjusted_rotation_deg(mapped.rotation_deg),
    flip_horizontal: mapped.flip_horizontal,
    flip_vertical: mapped.flip_vertical,
    fill_color,
    fill_pattern: None,
    fill_override: None,
    additional_fill_colors,
    fill_image: None,
    stroke: frame_stroke.or_else(|| expands_auto_fit.then_some(BorderStyle::default())),
    stroke_pattern: None,
    stroke_override: None,
    suppress_zero_relative_background: false,
    allow_outside_page: false,
    inline_anchor_after_line: matches!(placement, ImagePlacement::Inline),
    placement,
    chart: None,
    text_warp,
    text_fill: text_fill.map(Box::new),
    effects: properties.effects(&context.styles.theme_colors, Some(context.images)),
    static3d: properties.static3d(&context.styles.theme_colors),
    text_upright: shape
      .text_body_properties
      .as_ref()
      .and_then(|properties| properties.up_right.as_ref())
      .is_some_and(|value| value.as_bool()),
    text_box_blocks: text_box.blocks,
    text_inset_left_pt: text_box.left_pt,
    text_inset_top_pt: text_box.top_pt,
    text_inset_right_pt: text_box.right_pt,
    text_inset_bottom_pt: text_box.bottom_pt,
    text_box_auto_fit: auto_fit,
    text_box_resizes_height_to_fit: auto_fit && !expands_auto_fit,
    text_box_word_wrap: text_box.word_wrap,
    text_vertical_alignment: text_box.vertical_alignment,
  })
}

fn apply_wordprocessing_shape_preset_text_rectangle(
  properties: &DrawingMlShapeProperties,
  size_pt: (f32, f32),
  rotations_deg: (f32, f32),
  flips: (bool, bool),
  frame: &mut TextBoxFrameContent,
) {
  let (width_pt, height_pt) = size_pt;
  let (text_rotation_deg, shape_rotation_deg) = rotations_deg;
  let (flip_horizontal, flip_vertical) = flips;
  let Some(mut insets) = properties
    .preset_geometry()
    .and_then(|preset| drawingml_preset_text_rectangle_insets(preset, width_pt, height_pt))
  else {
    return;
  };
  if flip_horizontal {
    insets.swap(0, 2);
  }
  if flip_vertical {
    insets.swap(1, 3);
  }
  if rotations_cancel(text_rotation_deg, shape_rotation_deg) {
    // The PDF display model stores the text-body and owning-shape rotations
    // as one angle. When they cancel, no final rotation transform remains to
    // carry the preset text rectangle's off-center placement into page
    // coordinates. Rotate that center bias eagerly; retain the existing
    // wrapping extent until rotated text rectangles become a first-class
    // layout primitive.
    let center_dx = (insets[0] - insets[2]) * 0.5;
    let center_dy = (insets[1] - insets[3]) * 0.5;
    let angle = shape_rotation_deg.to_radians();
    let rotated_dx = center_dx * angle.cos() - center_dy * angle.sin();
    let rotated_dy = center_dx * angle.sin() + center_dy * angle.cos();
    insets = [
      (rotated_dx * 2.0).max(0.0),
      (rotated_dy * 2.0).max(0.0),
      (-rotated_dx * 2.0).max(0.0),
      (-rotated_dy * 2.0).max(0.0),
    ];
  }
  frame.left_pt += insets[0];
  frame.top_pt += insets[1];
  frame.right_pt += insets[2];
  frame.bottom_pt += insets[3];
}

fn rotations_cancel(left_deg: f32, right_deg: f32) -> bool {
  let normalized = (left_deg + right_deg).rem_euclid(360.0);
  normalized.min(360.0 - normalized) <= 0.001
    && left_deg.abs() > f32::EPSILON
    && right_deg.abs() > f32::EPSILON
}

fn drawingml_preset_text_rectangle_insets(
  preset: &a::PresetGeometry,
  width_pt: f32,
  height_pt: f32,
) -> Option<[f32; 4]> {
  if width_pt <= 0.0 || height_pt <= 0.0 {
    return None;
  }
  let guide = |index: usize, default: f32| {
    preset
      .adjust_value_list
      .as_ref()
      .and_then(|list| list.shape_guide.get(index))
      .and_then(|guide| {
        guide
          .formula
          .strip_prefix("val ")
          .unwrap_or(guide.formula.as_str())
          .parse::<f32>()
          .ok()
      })
      .unwrap_or(default)
  };
  match preset.preset {
    a::ShapeTypeValues::RightTriangle => {
      // presetShapeDefinitions.xml: rect=(wd12, 7h/12, 7w/12, 11h/12).
      // The asymmetric rectangle keeps text inside the triangular face
      // instead of centering it in the shape's complete bounding box.
      Some([
        width_pt / 12.0,
        height_pt * 7.0 / 12.0,
        width_pt * 5.0 / 12.0,
        height_pt / 12.0,
      ])
    }
    a::ShapeTypeValues::RightArrow => {
      // ECMA-376 presetShapeDefinitions.xml defines the right-arrow text
      // rectangle as (l, y1, x1 + dx2, y2), not the complete shape bounds.
      // This matters after shape rotation: the rectangle is biased toward the
      // shaft, so its center rotates away from the geometric center.
      let min_size = width_pt.min(height_pt);
      let a1 = guide(0, 50_000.0).clamp(0.0, 100_000.0);
      let a2 = guide(1, 50_000.0).clamp(0.0, 100_000.0 * width_pt / min_size);
      let dx1 = min_size * a2 / 100_000.0;
      let x1 = width_pt - dx1;
      let dy1 = height_pt * a1 / 200_000.0;
      let y1 = height_pt / 2.0 - dy1;
      let y2 = height_pt / 2.0 + dy1;
      let dx2 = y1 * dx1 / (height_pt / 2.0);
      let right = x1 + dx2;
      Some([
        0.0,
        y1,
        (width_pt - right).max(0.0),
        (height_pt - y2).max(0.0),
      ])
    }
    _ => None,
  }
}

fn drawing_graphic_data_choice_textbox_content(
  choice: &a::GraphicDataChoice,
) -> Option<w::TextBoxContent> {
  match choice {
    a::GraphicDataChoice::WordprocessingShape(shape) => {
      wordprocessing_shape_textbox_content(shape).cloned()
    }
    a::GraphicDataChoice::WordprocessingGroup(group) => {
      wordprocessing_group_textbox_content(group).cloned()
    }
    a::GraphicDataChoice::WordprocessingCanvas(canvas) => {
      wordprocessing_canvas_textbox_content(canvas).cloned()
    }
    a::GraphicDataChoice::XmlAny(xml) => strict_wordprocessing_shape(xml)
      .as_ref()
      .and_then(wordprocessing_shape_textbox_content)
      .cloned(),
    _ => None,
  }
}

fn wordprocessing_canvas_textbox_content(
  canvas: &wpc::WordprocessingCanvas,
) -> Option<&w::TextBoxContent> {
  canvas
    .wordprocessing_canvas_choice
    .iter()
    .find_map(|choice| match choice {
      wpc::WordprocessingCanvasChoice::WordprocessingShape(shape) => {
        wordprocessing_shape_textbox_content(shape)
      }
      wpc::WordprocessingCanvasChoice::WordprocessingGroup(group) => {
        wordprocessing_group_textbox_content(group)
      }
      _ => None,
    })
}

fn wordprocessing_group_textbox_content(
  group: &wpg::WordprocessingGroup,
) -> Option<&w::TextBoxContent> {
  group
    .wordprocessing_group_choice
    .iter()
    .find_map(|choice| match choice {
      wpg::WordprocessingGroupChoice::WordprocessingShape(shape) => {
        wordprocessing_shape_textbox_content(shape)
      }
      wpg::WordprocessingGroupChoice::GroupShape(group) => {
        wordprocessing_group_shape_textbox_content(group)
      }
      _ => None,
    })
}

fn wordprocessing_group_shape_textbox_content(
  group: &wpg::GroupShape,
) -> Option<&w::TextBoxContent> {
  group
    .group_shape_choice
    .iter()
    .find_map(|choice| match choice {
      wpg::GroupShapeChoice::WordprocessingShape(shape) => {
        wordprocessing_shape_textbox_content(shape)
      }
      wpg::GroupShapeChoice::GroupShape(group) => wordprocessing_group_shape_textbox_content(group),
      _ => None,
    })
}

fn wordprocessing_shape_textbox_content(
  shape: &wps::WordprocessingShape,
) -> Option<&w::TextBoxContent> {
  match shape.wordprocessing_shape_choice2.as_ref()? {
    wps::WordprocessingShapeChoice2::TextBoxInfo2(textbox) => textbox.text_box_content.as_ref(),
    wps::WordprocessingShapeChoice2::LinkedTextBox(_) => None,
  }
}

fn push_drawing_shapes_impl(
  drawing: &w::Drawing,
  inlines: &mut Vec<InlineItem>,
  styles: &StylesCatalog,
  images: &ImageCatalog,
  hyperlinks: &HyperlinkCatalog,
) {
  if drawing_is_hidden(drawing) {
    return;
  }

  let Some(graphic_data) = drawing_graphic_data(drawing) else {
    return;
  };
  let is_top_level_picture =
    drawing_image_properties(graphic_data, &styles.theme_colors, None).is_some();

  let placement = match drawing.drawing_choice.as_ref() {
    Some(w::DrawingChoice::Inline(_)) => ImagePlacement::Inline,
    Some(w::DrawingChoice::Anchor(anchor)) => {
      ImagePlacement::Floating(floating_image_placement(anchor))
    }
    None => return,
  };

  if let Some(w::DrawingChoice::Anchor(anchor)) = drawing.drawing_choice.as_ref()
    && let Some(shape) = anchor_wrap_polygon_shape(anchor, placement)
  {
    inlines.push(InlineItem::Shape(shape));
  }

  if is_top_level_picture {
    return;
  }

  let transform =
    DrawingMlGroupTransform::identity().with_fallback_size(drawing_extent_size(drawing));
  let effect_extent = drawing_effect_extent(drawing);
  let placement = drawing_placement_with_effect_extent(placement, effect_extent);
  for choice in &graphic_data.graphic_data_choice {
    match choice {
      a::GraphicDataChoice::ChartReference(reference) => {
        if let Some(chart_shapes) = drawing_chart_shapes(
          drawing,
          reference,
          &images.charts_by_relationship_id,
          &images.extended_charts_by_relationship_id,
          styles,
        ) {
          inlines.extend(chart_shapes.into_iter().map(InlineItem::Shape));
        }
      }
      a::GraphicDataChoice::ExtendedChartReference(reference) => {
        if let Some(chart_shapes) = drawing_extended_chart_shapes(
          drawing,
          reference.r_id.as_str(),
          &images.extended_charts_by_relationship_id,
          styles,
        ) {
          inlines.extend(chart_shapes.into_iter().map(InlineItem::Shape));
        }
      }
      a::GraphicDataChoice::RelationshipIds(relationship_ids) => {
        if let Some(diagram_shapes) = drawing_diagram_shapes(
          relationship_ids,
          placement,
          transform,
          DrawingShapeImportContext {
            effect_extent,
            styles,
            images,
            hyperlinks,
            smartart_text_colors_by_model_id: None,
          },
        ) {
          inlines.extend(diagram_shapes);
        }
      }
      _ => {
        inlines.extend(drawing_graphic_data_choice_shapes(
          choice,
          placement,
          transform,
          DrawingShapeImportContext {
            effect_extent,
            styles,
            images,
            hyperlinks,
            smartart_text_colors_by_model_id: None,
          },
        ));
      }
    }
  }
}

fn drawing_graphic_data_choice_shapes(
  choice: &a::GraphicDataChoice,
  placement: ImagePlacement,
  transform: DrawingMlGroupTransform,
  context: DrawingShapeImportContext<'_>,
) -> Vec<InlineItem> {
  match choice {
    a::GraphicDataChoice::WordprocessingShape(shape) => {
      wordprocessing_shape_shape(shape, placement, transform, context)
        .into_iter()
        .map(InlineItem::Shape)
        .collect()
    }
    a::GraphicDataChoice::WordprocessingGroup(group) => {
      wordprocessing_group_shapes(group, placement, transform, context)
    }
    a::GraphicDataChoice::WordprocessingCanvas(canvas) => {
      wordprocessing_canvas_shapes(canvas, placement, transform, context)
    }
    a::GraphicDataChoice::LockedCanvas(canvas) => {
      drawingml_locked_canvas_shapes(canvas, placement, transform, context)
    }
    a::GraphicDataChoice::Picture(picture) => {
      drawingml_picture_items(picture, placement, transform, context)
    }
    a::GraphicDataChoice::Drawing(drawing) => {
      drawingml_diagram_drawing_shapes(drawing, placement, transform, context)
    }
    a::GraphicDataChoice::XmlAny(xml) => strict_wordprocessing_shape(xml)
      .map(|shape| wordprocessing_shape_shape(&shape, placement, transform, context))
      .into_iter()
      .flatten()
      .map(InlineItem::Shape)
      .collect(),
    _ => Vec::new(),
  }
}

fn drawingml_locked_canvas_shapes(
  canvas: &lc::LockedCanvas,
  placement: ImagePlacement,
  transform: DrawingMlGroupTransform,
  context: DrawingShapeImportContext<'_>,
) -> Vec<InlineItem> {
  let child_transform = canvas
    .visual_group_shape_properties
    .transform_group
    .as_deref()
    .map(|model| {
      let mut xfrm = drawingml_group_transform_from_model(model, transform.raw_coordinates);
      // A locked canvas is the graphic payload of a WordprocessingML
      // drawing. Its host wp:extent is the displayed object boundary; the
      // root grpSpPr/chOff/chExt still declares the child coordinate space.
      if let Some((width_pt, height_pt)) = transform.fallback_size {
        xfrm.width_pt = width_pt;
        xfrm.height_pt = height_pt;
      }
      xfrm
    })
    .map(|xfrm| transform.child(xfrm))
    .unwrap_or(transform);
  let child_context = DrawingShapeImportContext {
    effect_extent: DrawingEffectExtent::default(),
    ..context
  };
  canvas
    .locked_canvas_choice
    .iter()
    .flat_map(|choice| {
      drawingml_generic_group_choice_shapes(
        choice,
        drawingml_group_child_placement(placement),
        child_transform,
        child_context,
      )
    })
    .collect()
}

fn drawingml_generic_group_shapes(
  group: &a::GroupShape,
  placement: ImagePlacement,
  transform: DrawingMlGroupTransform,
  context: DrawingShapeImportContext<'_>,
) -> Vec<InlineItem> {
  let child_transform = group
    .visual_group_shape_properties
    .transform_group
    .as_deref()
    .map(|model| {
      let mut xfrm = drawingml_group_transform_from_model(model, transform.raw_coordinates);
      if let Some((left, top, width, height)) = drawingml_locked_canvas_line_group_bounds(group) {
        // LibreOffice's locked-canvas import restores LineShape position and
        // size as absolute values after group insertion
        // (oox/source/drawingml/shape.cxx). Microsoft fixed output likewise
        // fits the visible line union to the nested group's extent, rather
        // than retaining unused chOff/chExt margins.
        xfrm.child_offset_x = left;
        xfrm.child_offset_y = top;
        xfrm.child_width = width;
        xfrm.child_height = height;
      }
      xfrm
    })
    .map(|xfrm| transform.child(xfrm))
    .unwrap_or(transform);
  let child_context = DrawingShapeImportContext {
    effect_extent: DrawingEffectExtent::default(),
    ..context
  };
  group
    .group_shape_choice
    .iter()
    .flat_map(|choice| {
      drawingml_generic_group_shape_choice_shapes(
        choice,
        drawingml_group_child_placement(placement),
        child_transform,
        child_context,
      )
    })
    .collect()
}

fn drawingml_locked_canvas_line_group_bounds(
  group: &a::GroupShape,
) -> Option<(f32, f32, f32, f32)> {
  let mut bounds: Option<(f32, f32, f32, f32)> = None;
  for choice in &group.group_shape_choice {
    let properties = match choice {
      a::GroupShapeChoice::Shape(shape) => &shape.shape_properties,
      a::GroupShapeChoice::ConnectionShape(shape) => &shape.shape_properties,
      _ => return None,
    };
    let is_line = matches!(
      properties.shape_properties_choice1.as_ref(),
      Some(a::ShapePropertiesChoice::PresetGeometry(geometry))
        if geometry.preset == a::ShapeTypeValues::Line
    );
    if !is_line {
      return None;
    }
    let transform = properties.transform2_d.as_deref()?;
    let offset = transform.offset.as_ref()?;
    let extents = transform.extents.as_ref()?;
    let left = offset.x.to_emu() as f32;
    let top = offset.y.to_emu() as f32;
    let right = left + extents.cx.to_emu() as f32;
    let bottom = top + extents.cy.to_emu() as f32;
    bounds = Some(match bounds {
      Some((min_x, min_y, max_x, max_y)) => (
        min_x.min(left),
        min_y.min(top),
        max_x.max(right),
        max_y.max(bottom),
      ),
      None => (left, top, right, bottom),
    });
  }
  let (left, top, right, bottom) = bounds?;
  let width = right - left;
  let height = bottom - top;
  (width > 0.0 && height > 0.0).then_some((left, top, width, height))
}

fn drawingml_generic_group_choice_shapes(
  choice: &lc::LockedCanvasChoice,
  placement: ImagePlacement,
  transform: DrawingMlGroupTransform,
  context: DrawingShapeImportContext<'_>,
) -> Vec<InlineItem> {
  match choice {
    lc::LockedCanvasChoice::Shape(shape) => drawingml_generic_shape_shape(
      &shape.shape_properties,
      shape.shape_style.as_deref(),
      placement,
      transform,
      context,
    )
    .into_iter()
    .map(InlineItem::Shape)
    .collect(),
    lc::LockedCanvasChoice::ConnectionShape(shape) => drawingml_generic_shape_shape(
      &shape.shape_properties,
      shape.shape_style.as_deref(),
      placement,
      transform,
      context,
    )
    .into_iter()
    .map(InlineItem::Shape)
    .collect(),
    lc::LockedCanvasChoice::GroupShape(group) => {
      drawingml_generic_group_shapes(group, placement, transform, context)
    }
    _ => Vec::new(),
  }
}

fn drawingml_generic_group_shape_choice_shapes(
  choice: &a::GroupShapeChoice,
  placement: ImagePlacement,
  transform: DrawingMlGroupTransform,
  context: DrawingShapeImportContext<'_>,
) -> Vec<InlineItem> {
  match choice {
    a::GroupShapeChoice::Shape(shape) => drawingml_generic_shape_shape(
      &shape.shape_properties,
      shape.shape_style.as_deref(),
      placement,
      transform,
      context,
    )
    .into_iter()
    .map(InlineItem::Shape)
    .collect(),
    a::GroupShapeChoice::ConnectionShape(shape) => drawingml_generic_shape_shape(
      &shape.shape_properties,
      shape.shape_style.as_deref(),
      placement,
      transform,
      context,
    )
    .into_iter()
    .map(InlineItem::Shape)
    .collect(),
    a::GroupShapeChoice::GroupShape(group) => {
      drawingml_generic_group_shapes(group, placement, transform, context)
    }
    _ => Vec::new(),
  }
}

fn drawingml_generic_shape_shape(
  shape_properties: &a::ShapeProperties,
  shape_style: Option<&a::ShapeStyle>,
  placement: ImagePlacement,
  transform: DrawingMlGroupTransform,
  context: DrawingShapeImportContext<'_>,
) -> Option<InlineShape> {
  let properties = DrawingMlShapeProperties::Generic(shape_properties.clone());
  let has_explicit_fill = properties.fill().is_some();
  let explicit_fill_color =
    drawingml_shape_properties_fill_color(&properties, &context.styles.theme_colors);
  let fill_pattern =
    drawingml_shape_properties_pattern_fill(&properties, &context.styles.theme_colors);
  let fill_override =
    drawingml_shape_properties_common_fill(&properties, &context.styles.theme_colors);
  let fill_color = if drawingml_shape_properties_has_no_fill(&properties) {
    None
  } else {
    explicit_fill_color.or_else(|| {
      (!has_explicit_fill)
        .then(|| {
          shape_style.and_then(|style| {
            drawingml_fill_reference_color(&style.fill_reference, &context.styles.theme_colors)
          })
        })
        .flatten()
    })
  };
  let fill_image = drawingml_generic_shape_image_fill(shape_properties, context.images);
  let stroke_override = shape_properties
    .outline
    .as_deref()
    .and_then(|outline| drawingml_outline_common_stroke(outline, &context.styles.theme_colors));
  let stroke = stroke_override
    .as_ref()
    .map(drawingml_border_style_from_common_stroke)
    .or_else(|| {
      shape_style.map(|style| {
        drawingml_line_reference_stroke(
          &style.line_reference,
          &context.styles.theme_colors,
          &context.styles.theme_lines,
        )
      })?
    });
  let stroke_pattern = shape_properties.outline.as_deref().and_then(|outline| {
    match outline.outline_choice1.as_ref()? {
      a::OutlineChoice::PatternFill(fill) => {
        drawingml_pattern_fill(fill, &context.styles.theme_colors)
      }
      _ => None,
    }
  });
  // Generic a:txSp content has independent DrawingML run properties, text
  // warp, and effects. Word commonly emits it as non-semantic vector
  // outlines, so it must not be downgraded to default WordprocessingML text.
  if fill_color.is_none()
    && fill_pattern.is_none()
    && fill_override
      .as_ref()
      .is_none_or(|fill| matches!(fill, common::Fill::None))
    && fill_image.is_none()
    && stroke.is_none()
    && stroke_override.is_none()
  {
    return None;
  }

  let mut geometry = properties
    .geometry_kind()
    .unwrap_or(InlineShapeGeometry::Rectangle);
  let has_path_geometry = properties.has_path_geometry();
  if geometry == InlineShapeGeometry::Rectangle && has_path_geometry {
    geometry = InlineShapeGeometry::Polyline {
      points: Vec::new(),
      closed: false,
    };
  }
  let (offset_x_pt, offset_y_pt, width_pt, height_pt) = drawingml_geometry_from_shape_properties(
    Some(&properties),
    &geometry,
    transform.raw_coordinates,
    transform.fallback_size,
  )?;
  let mapped = transform.map_rect(
    (offset_x_pt, offset_y_pt, width_pt, height_pt),
    (
      properties.rotation_deg(),
      properties.flip_horizontal(),
      properties.flip_vertical(),
    ),
  );
  let (offset_x_pt, offset_y_pt, width_pt, height_pt) =
    (mapped.x_pt, mapped.y_pt, mapped.width_pt, mapped.height_pt);
  if has_path_geometry
    && let Some(path_geometry) =
      drawingml_path_geometry_from_properties(&properties, width_pt, height_pt)
  {
    geometry = path_geometry;
  }

  Some(InlineShape {
    width_pt,
    height_pt,
    effect_left_pt: context.effect_extent.left_pt,
    effect_top_pt: context.effect_extent.top_pt,
    effect_right_pt: context.effect_extent.right_pt,
    effect_bottom_pt: context.effect_extent.bottom_pt,
    geometry,
    offset_x_pt,
    offset_y_pt,
    rotation_deg: properties.camera_adjusted_rotation_deg(mapped.rotation_deg),
    flip_horizontal: mapped.flip_horizontal,
    flip_vertical: mapped.flip_vertical,
    fill_color,
    fill_pattern,
    fill_override: fill_override.map(Box::new),
    additional_fill_colors: Vec::new(),
    fill_image,
    stroke,
    stroke_pattern,
    stroke_override: stroke_override.map(Box::new),
    suppress_zero_relative_background: explicit_fill_color.is_some(),
    allow_outside_page: false,
    inline_anchor_after_line: false,
    placement,
    chart: None,
    text_warp: None,
    text_fill: None,
    effects: properties.effects(&context.styles.theme_colors, Some(context.images)),
    static3d: properties.static3d(&context.styles.theme_colors),
    text_upright: false,
    text_box_blocks: Vec::new(),
    text_inset_left_pt: 0.0,
    text_inset_top_pt: 0.0,
    text_inset_right_pt: 0.0,
    text_inset_bottom_pt: 0.0,
    text_box_auto_fit: false,
    text_box_resizes_height_to_fit: false,
    text_box_word_wrap: true,
    text_vertical_alignment: TextBoxVerticalAlignment::Top,
  })
}

fn wordprocessing_canvas_shapes(
  canvas: &wpc::WordprocessingCanvas,
  placement: ImagePlacement,
  transform: DrawingMlGroupTransform,
  context: DrawingShapeImportContext<'_>,
) -> Vec<InlineItem> {
  canvas
    .wordprocessing_canvas_choice
    .iter()
    .flat_map(|choice| wordprocessing_canvas_choice_shapes(choice, placement, transform, context))
    .collect()
}

fn wordprocessing_canvas_choice_shapes(
  choice: &wpc::WordprocessingCanvasChoice,
  placement: ImagePlacement,
  transform: DrawingMlGroupTransform,
  context: DrawingShapeImportContext<'_>,
) -> Vec<InlineItem> {
  match choice {
    wpc::WordprocessingCanvasChoice::WordprocessingShape(shape) => {
      wordprocessing_shape_shape(shape, placement, transform, context)
        .into_iter()
        .map(InlineItem::Shape)
        .collect()
    }
    wpc::WordprocessingCanvasChoice::WordprocessingGroup(group) => {
      wordprocessing_group_shapes(group, placement, transform, context)
    }
    wpc::WordprocessingCanvasChoice::Picture(picture) => {
      drawingml_picture_items(picture, placement, transform, context)
    }
    _ => Vec::new(),
  }
}

fn wordprocessing_group_shapes(
  group: &wpg::WordprocessingGroup,
  placement: ImagePlacement,
  transform: DrawingMlGroupTransform,
  context: DrawingShapeImportContext<'_>,
) -> Vec<InlineItem> {
  let child_transform = drawingml_group_transform_from_properties(
    &group.group_shape_properties,
    transform.raw_coordinates,
  )
  .map(|xfrm| transform.child(xfrm))
  .unwrap_or(transform);
  let child_context = DrawingShapeImportContext {
    effect_extent: DrawingEffectExtent::default(),
    ..context
  };
  let children = group
    .wordprocessing_group_choice
    .iter()
    .flat_map(|choice| {
      wordprocessing_group_choice_shapes(
        choice,
        drawingml_group_child_placement(placement),
        child_transform,
        child_context,
      )
    })
    .collect();
  wrap_wordprocessing_group_effects(
    children,
    &group.group_shape_properties,
    child_transform.rotation_degrees(),
    placement,
    context,
  )
}

fn wordprocessing_group_shape_shapes(
  group: &wpg::GroupShape,
  placement: ImagePlacement,
  transform: DrawingMlGroupTransform,
  context: DrawingShapeImportContext<'_>,
) -> Vec<InlineItem> {
  let child_transform = drawingml_group_transform_from_properties(
    &group.group_shape_properties,
    transform.raw_coordinates,
  )
  .map(|xfrm| transform.child(xfrm))
  .unwrap_or(transform);
  let child_context = DrawingShapeImportContext {
    effect_extent: DrawingEffectExtent::default(),
    ..context
  };
  let children = group
    .group_shape_choice
    .iter()
    .flat_map(|choice| {
      wordprocessing_group_shape_choice_shapes(
        choice,
        drawingml_group_child_placement(placement),
        child_transform,
        child_context,
      )
    })
    .collect();
  wrap_wordprocessing_group_effects(
    children,
    &group.group_shape_properties,
    child_transform.rotation_degrees(),
    placement,
    context,
  )
}

fn wrap_wordprocessing_group_effects(
  children: Vec<InlineItem>,
  properties: &wpg::GroupShapeProperties,
  rotation_deg: f32,
  placement: ImagePlacement,
  context: DrawingShapeImportContext<'_>,
) -> Vec<InlineItem> {
  let Some(mut effects) = properties
    .group_shape_properties_choice2
    .as_ref()
    .map(|choice| {
      let resolver = DocxImageEffectColorResolver {
        theme_colors: &context.styles.theme_colors,
        images: Some(context.images),
        placeholder_color: None,
        word_group_glow: true,
      };
      match choice {
        wpg::GroupShapePropertiesChoice2::EffectList(source) => common::DrawingEffectSource::List {
          source: source.clone(),
          resolved: Some(common::drawingml_image_effects::from_effect_list(
            source, None, &resolver,
          )),
        },
        wpg::GroupShapePropertiesChoice2::EffectDag(source) => common::DrawingEffectSource::Dag {
          source: source.clone(),
          resolved: Some(common::drawingml_image_effects::from_effect_dag(
            source, None, &resolver,
          )),
        },
      }
    })
  else {
    return children;
  };
  match &mut effects {
    common::DrawingEffectSource::List {
      resolved: Some(value),
      ..
    }
    | common::DrawingEffectSource::Dag {
      resolved: Some(value),
      ..
    } => common::drawingml_image_effects::use_word_group_glow_profile(value),
    _ => {}
  }
  let has_runtime_effects = match &effects {
    common::DrawingEffectSource::List {
      resolved: Some(value),
      ..
    }
    | common::DrawingEffectSource::Dag {
      resolved: Some(value),
      ..
    } => !value.effects.is_empty(),
    _ => false,
  };
  if !has_runtime_effects || children.is_empty() {
    return children;
  }
  let mut children = children;
  suppress_group_child_wrap(&mut children);
  let mut grouped = Vec::with_capacity(children.len() + 2);
  grouped.push(InlineItem::DrawingGroupStart(InlineDrawingGroupEffect {
    effects,
    rotation_deg,
    placement,
  }));
  grouped.extend(children);
  grouped.push(InlineItem::DrawingGroupEnd);
  grouped
}

fn suppress_group_child_wrap(children: &mut [InlineItem]) {
  for child in children {
    let placement = match child {
      InlineItem::Image(image) => &mut image.placement,
      InlineItem::Shape(shape) => &mut shape.placement,
      InlineItem::DrawingGroupStart(group) => &mut group.placement,
      _ => continue,
    };
    if let ImagePlacement::Floating(placement) = placement {
      // The host group owns one wrap contour. Its children keep floating
      // coordinates for paint, but must not each advance the paragraph.
      placement.wrap = ImageWrapMode::Inline;
    }
  }
}

fn wordprocessing_group_choice_shapes(
  choice: &wpg::WordprocessingGroupChoice,
  placement: ImagePlacement,
  transform: DrawingMlGroupTransform,
  context: DrawingShapeImportContext<'_>,
) -> Vec<InlineItem> {
  match choice {
    wpg::WordprocessingGroupChoice::WordprocessingShape(shape) => {
      wordprocessing_shape_shape(shape, placement, transform, context)
        .into_iter()
        .map(InlineItem::Shape)
        .collect()
    }
    wpg::WordprocessingGroupChoice::GroupShape(group) => {
      wordprocessing_group_shape_shapes(group, placement, transform, context)
    }
    wpg::WordprocessingGroupChoice::Picture(picture) => {
      drawingml_picture_items(picture, placement, transform, context)
    }
    _ => Vec::new(),
  }
}

fn wordprocessing_group_shape_choice_shapes(
  choice: &wpg::GroupShapeChoice,
  placement: ImagePlacement,
  transform: DrawingMlGroupTransform,
  context: DrawingShapeImportContext<'_>,
) -> Vec<InlineItem> {
  match choice {
    wpg::GroupShapeChoice::WordprocessingShape(shape) => {
      wordprocessing_shape_shape(shape, placement, transform, context)
        .into_iter()
        .map(InlineItem::Shape)
        .collect()
    }
    wpg::GroupShapeChoice::GroupShape(group) => {
      wordprocessing_group_shape_shapes(group, placement, transform, context)
    }
    wpg::GroupShapeChoice::Picture(picture) => {
      drawingml_picture_items(picture, placement, transform, context)
    }
    _ => Vec::new(),
  }
}

fn wordprocessing_shape_shape(
  shape: &wps::WordprocessingShape,
  placement: ImagePlacement,
  transform: DrawingMlGroupTransform,
  context: DrawingShapeImportContext<'_>,
) -> Option<InlineShape> {
  let shape_properties = shape
    .shape_properties
    .as_deref()
    .cloned()
    .unwrap_or_default();
  let properties = DrawingMlShapeProperties::Wordprocessing(shape_properties.clone());
  let has_explicit_fill = properties.fill().is_some();
  let explicit_fill_color =
    drawingml_shape_properties_fill_color(&properties, &context.styles.theme_colors);
  let fill_pattern =
    drawingml_shape_properties_pattern_fill(&properties, &context.styles.theme_colors);
  let fill_override =
    drawingml_shape_properties_common_fill(&properties, &context.styles.theme_colors);
  let fill_color = if drawingml_shape_properties_has_no_fill(&properties) {
    None
  } else {
    explicit_fill_color.or_else(|| {
      (!has_explicit_fill)
        .then(|| {
          shape.shape_style.as_ref().and_then(|style| {
            drawingml_fill_reference_color(&style.fill_reference, &context.styles.theme_colors)
          })
        })
        .flatten()
    })
  };
  let fill_image = wordprocessing_shape_image_fill(&shape_properties, context.images);
  let stroke = if wordprocessing_shape_has_no_line(shape) {
    None
  } else {
    wordprocessing_shape_stroke(shape, &context.styles.theme_colors).or_else(|| {
      shape.shape_style.as_ref().and_then(|style| {
        drawingml_line_reference_stroke(
          &style.line_reference,
          &context.styles.theme_colors,
          &context.styles.theme_lines,
        )
      })
    })
  };
  let stroke_pattern = shape
    .shape_properties
    .as_deref()
    .and_then(|properties| properties.outline.as_deref())
    .and_then(|outline| match outline.outline_choice1.as_ref()? {
      a::OutlineChoice::PatternFill(fill) => {
        drawingml_pattern_fill(fill, &context.styles.theme_colors)
      }
      _ => None,
    });
  let stroke_override = shape
    .shape_properties
    .as_deref()
    .and_then(|properties| properties.outline.as_deref())
    .and_then(|outline| drawingml_outline_common_stroke(outline, &context.styles.theme_colors));
  if fill_color.is_none()
    && fill_pattern.is_none()
    && fill_override
      .as_ref()
      .is_none_or(|fill| matches!(fill, common::Fill::None))
    && fill_image.is_none()
    && stroke.is_none()
    && stroke_override.is_none()
  {
    return None;
  }

  let mut geometry = properties
    .geometry_kind()
    .unwrap_or(InlineShapeGeometry::Rectangle);
  let has_path_geometry = properties.has_path_geometry();
  if geometry == InlineShapeGeometry::Rectangle && has_path_geometry {
    geometry = InlineShapeGeometry::Polyline {
      points: Vec::new(),
      closed: false,
    };
  }
  let (offset_x_pt, offset_y_pt, width_pt, height_pt) = drawingml_geometry_from_shape_properties(
    Some(&properties),
    &geometry,
    transform.raw_coordinates,
    transform.fallback_size,
  )?;
  let mapped = transform.map_rect(
    (offset_x_pt, offset_y_pt, width_pt, height_pt),
    (
      properties.rotation_deg(),
      properties.flip_horizontal(),
      properties.flip_vertical(),
    ),
  );
  let (offset_x_pt, offset_y_pt, width_pt, height_pt) =
    (mapped.x_pt, mapped.y_pt, mapped.width_pt, mapped.height_pt);
  if has_path_geometry
    && let Some(path_geometry) =
      drawingml_path_geometry_from_properties(&properties, width_pt, height_pt)
  {
    geometry = path_geometry;
  }
  let effects = properties
    .effects(&context.styles.theme_colors, Some(context.images))
    .or_else(|| {
      shape.shape_style.as_ref().and_then(|style| {
        drawingml_effect_reference_effects(
          &style.effect_reference,
          &context.styles.theme_effects,
          &context.styles.theme_colors,
          Some(context.images),
        )
      })
    });

  Some(InlineShape {
    width_pt,
    height_pt,
    effect_left_pt: context.effect_extent.left_pt,
    effect_top_pt: context.effect_extent.top_pt,
    effect_right_pt: context.effect_extent.right_pt,
    effect_bottom_pt: context.effect_extent.bottom_pt,
    geometry,
    offset_x_pt,
    offset_y_pt,
    rotation_deg: properties.camera_adjusted_rotation_deg(mapped.rotation_deg),
    flip_horizontal: mapped.flip_horizontal,
    flip_vertical: mapped.flip_vertical,
    fill_color,
    fill_pattern,
    fill_override: fill_override.map(Box::new),
    additional_fill_colors: Vec::new(),
    fill_image,
    stroke,
    stroke_pattern,
    stroke_override: stroke_override.map(Box::new),
    suppress_zero_relative_background: explicit_fill_color.is_some(),
    allow_outside_page: false,
    inline_anchor_after_line: false,
    placement,
    chart: None,
    text_warp: None,
    text_fill: None,
    effects,
    static3d: properties.static3d(&context.styles.theme_colors),
    text_upright: false,
    text_box_blocks: Vec::new(),
    text_inset_left_pt: 0.0,
    text_inset_top_pt: 0.0,
    text_inset_right_pt: 0.0,
    text_inset_bottom_pt: 0.0,
    text_box_auto_fit: false,
    text_box_resizes_height_to_fit: false,
    text_box_word_wrap: true,
    text_vertical_alignment: TextBoxVerticalAlignment::Top,
  })
}

fn drawingml_picture_items(
  picture: &pic::Picture,
  placement: ImagePlacement,
  transform: DrawingMlGroupTransform,
  context: DrawingShapeImportContext<'_>,
) -> Vec<InlineItem> {
  let mut items = Vec::new();
  if let Some(image) = drawingml_picture_image(
    picture,
    placement,
    transform,
    context.styles,
    context.images,
    context.hyperlinks,
  ) {
    items.push(InlineItem::Image(image));
  }
  items
}

fn drawing_diagram_shapes(
  relationship_ids: &dgm::RelationshipIds,
  placement: ImagePlacement,
  transform: DrawingMlGroupTransform,
  context: DrawingShapeImportContext<'_>,
) -> Option<Vec<InlineItem>> {
  // resolves dgm:relIds through the diagram data part, then imports the
  // persisted diagramDrawing extDrawing fallback when present.
  let data_relationship_id = relationship_ids.data_part.as_str();
  let data_xml = context
    .images
    .diagram_data_by_relationship_id
    .get(data_relationship_id)?;
  let text_colors_by_model_id = (!relationship_ids.color_part.is_empty())
    .then(|| {
      context
        .images
        .diagram_colors_by_relationship_id
        .get(relationship_ids.color_part.as_str())
    })
    .flatten()
    .map(|colors| {
      diagram_text_fill_colors_by_model_id(data_xml, colors, &context.styles.theme_colors)
    });
  let drawing_relationship_id = diagram_ext_drawing_relationship_id(data_xml)?;
  let drawing = context
    .images
    .diagram_drawings_by_relationship_id
    .get(&drawing_relationship_id)?;
  Some(drawingml_diagram_drawing_shapes(
    drawing,
    placement,
    transform,
    DrawingShapeImportContext {
      smartart_text_colors_by_model_id: text_colors_by_model_id.as_ref(),
      ..context
    },
  ))
}

fn drawingml_diagram_drawing_shapes(
  drawing: &dsp::Drawing,
  placement: ImagePlacement,
  transform: DrawingMlGroupTransform,
  context: DrawingShapeImportContext<'_>,
) -> Vec<InlineItem> {
  let child_transform = drawingml_group_transform_from_diagram_properties(
    &drawing.shape_tree.group_shape_properties,
    transform.raw_coordinates,
  )
  .map(|xfrm| transform.child(xfrm))
  .unwrap_or(transform);
  let children = drawing
    .shape_tree
    .shape_tree_choice
    .iter()
    .flat_map(|choice| {
      drawingml_diagram_shape_tree_choice_shapes(
        choice,
        drawingml_group_child_placement(placement),
        child_transform,
        context,
      )
    })
    .collect();
  wrap_diagram_group_effects(
    children,
    &drawing.shape_tree.group_shape_properties,
    child_transform.rotation_degrees(),
    placement,
    context,
  )
}

fn drawingml_diagram_group_shapes(
  group: &dsp::GroupShape,
  placement: ImagePlacement,
  transform: DrawingMlGroupTransform,
  context: DrawingShapeImportContext<'_>,
) -> Vec<InlineItem> {
  let child_transform = drawingml_group_transform_from_diagram_properties(
    &group.group_shape_properties,
    transform.raw_coordinates,
  )
  .map(|xfrm| transform.child(xfrm))
  .unwrap_or(transform);
  let children = group
    .group_shape_choice
    .iter()
    .flat_map(|choice| {
      drawingml_diagram_group_choice_shapes(
        choice,
        drawingml_group_child_placement(placement),
        child_transform,
        context,
      )
    })
    .collect();
  wrap_diagram_group_effects(
    children,
    &group.group_shape_properties,
    child_transform.rotation_degrees(),
    placement,
    context,
  )
}

fn wrap_diagram_group_effects(
  children: Vec<InlineItem>,
  properties: &dsp::GroupShapeProperties,
  rotation_deg: f32,
  placement: ImagePlacement,
  context: DrawingShapeImportContext<'_>,
) -> Vec<InlineItem> {
  let Some(effects) = properties
    .group_shape_properties_choice2
    .as_ref()
    .map(|choice| {
      let resolver = DocxImageEffectColorResolver {
        theme_colors: &context.styles.theme_colors,
        images: Some(context.images),
        placeholder_color: None,
        word_group_glow: false,
      };
      match choice {
        dsp::GroupShapePropertiesChoice2::EffectList(source) => common::DrawingEffectSource::List {
          source: source.clone(),
          resolved: Some(common::drawingml_image_effects::from_effect_list(
            source, None, &resolver,
          )),
        },
        dsp::GroupShapePropertiesChoice2::EffectDag(source) => common::DrawingEffectSource::Dag {
          source: source.clone(),
          resolved: Some(common::drawingml_image_effects::from_effect_dag(
            source, None, &resolver,
          )),
        },
      }
    })
  else {
    return children;
  };
  let has_runtime_effects = match &effects {
    common::DrawingEffectSource::List {
      resolved: Some(value),
      ..
    }
    | common::DrawingEffectSource::Dag {
      resolved: Some(value),
      ..
    } => !value.effects.is_empty(),
    _ => false,
  };
  if !has_runtime_effects || children.is_empty() {
    return children;
  }
  let mut children = children;
  suppress_group_child_wrap(&mut children);
  let mut grouped = Vec::with_capacity(children.len() + 2);
  grouped.push(InlineItem::DrawingGroupStart(InlineDrawingGroupEffect {
    effects,
    rotation_deg,
    placement,
  }));
  grouped.extend(children);
  grouped.push(InlineItem::DrawingGroupEnd);
  grouped
}

fn drawingml_diagram_shape_tree_choice_shapes(
  choice: &dsp::ShapeTreeChoice,
  placement: ImagePlacement,
  transform: DrawingMlGroupTransform,
  context: DrawingShapeImportContext<'_>,
) -> Vec<InlineItem> {
  match choice {
    dsp::ShapeTreeChoice::Shape(shape) => {
      drawingml_diagram_shape_shape(shape, placement, transform, context)
        .into_iter()
        .map(InlineItem::Shape)
        .collect()
    }
    dsp::ShapeTreeChoice::GroupShape(group) => {
      drawingml_diagram_group_shapes(group, placement, transform, context)
    }
  }
}

fn drawingml_diagram_group_choice_shapes(
  choice: &dsp::GroupShapeChoice,
  placement: ImagePlacement,
  transform: DrawingMlGroupTransform,
  context: DrawingShapeImportContext<'_>,
) -> Vec<InlineItem> {
  match choice {
    dsp::GroupShapeChoice::Shape(shape) => {
      drawingml_diagram_shape_shape(shape, placement, transform, context)
        .into_iter()
        .map(InlineItem::Shape)
        .collect()
    }
    dsp::GroupShapeChoice::GroupShape(group) => {
      drawingml_diagram_group_shapes(group, placement, transform, context)
    }
  }
}

fn drawingml_diagram_shape_shape(
  shape: &dsp::Shape,
  placement: ImagePlacement,
  transform: DrawingMlGroupTransform,
  context: DrawingShapeImportContext<'_>,
) -> Option<InlineShape> {
  let properties = DrawingMlShapeProperties::Diagram((*shape.shape_properties).clone());
  let has_explicit_fill = properties.fill().is_some();
  let explicit_fill_color =
    drawingml_shape_properties_fill_color(&properties, &context.styles.theme_colors);
  let fill_pattern =
    drawingml_shape_properties_pattern_fill(&properties, &context.styles.theme_colors);
  let fill_override =
    drawingml_shape_properties_common_fill(&properties, &context.styles.theme_colors);
  let fill_color = if drawingml_shape_properties_has_no_fill(&properties) {
    None
  } else {
    explicit_fill_color.or_else(|| {
      (!has_explicit_fill)
        .then(|| {
          shape.shape_style.as_ref().and_then(|style| {
            drawingml_fill_reference_color(&style.fill_reference, &context.styles.theme_colors)
          })
        })
        .flatten()
    })
  };
  let fill_image = drawingml_diagram_shape_image_fill(&shape.shape_properties, context.images);
  let stroke = if drawingml_diagram_shape_has_no_line(shape) {
    None
  } else {
    drawingml_diagram_shape_stroke(shape, &context.styles.theme_colors).or_else(|| {
      shape.shape_style.as_ref().and_then(|style| {
        drawingml_line_reference_stroke(
          &style.line_reference,
          &context.styles.theme_colors,
          &context.styles.theme_lines,
        )
      })
    })
  };
  let stroke_pattern = shape
    .shape_properties
    .outline
    .as_deref()
    .and_then(|outline| match outline.outline_choice1.as_ref()? {
      a::OutlineChoice::PatternFill(fill) => {
        drawingml_pattern_fill(fill, &context.styles.theme_colors)
      }
      _ => None,
    });
  let stroke_override = shape
    .shape_properties
    .outline
    .as_deref()
    .and_then(|outline| drawingml_outline_common_stroke(outline, &context.styles.theme_colors));
  let smartart_text_color = context
    .smartart_text_colors_by_model_id
    .and_then(|colors| colors.get(shape.model_id.as_str()).copied());
  let mut text_box = drawingml_diagram_shape_text_box(shape, context.styles, smartart_text_color);
  if fill_color.is_none()
    && fill_pattern.is_none()
    && fill_override.is_none()
    && fill_image.is_none()
    && stroke.is_none()
    && stroke_override.is_none()
    && text_box.is_none()
  {
    return None;
  }

  let mut geometry = properties
    .geometry_kind()
    .unwrap_or(InlineShapeGeometry::Rectangle);
  let has_path_geometry = properties.has_path_geometry();
  if geometry == InlineShapeGeometry::Rectangle && has_path_geometry {
    geometry = InlineShapeGeometry::Polyline {
      points: Vec::new(),
      closed: false,
    };
  }
  let (offset_x_pt, offset_y_pt, width_pt, height_pt) = drawingml_geometry_from_shape_properties(
    Some(&properties),
    &geometry,
    transform.raw_coordinates,
    transform.fallback_size,
  )?;
  let mapped = transform.map_rect(
    (offset_x_pt, offset_y_pt, width_pt, height_pt),
    (
      properties.rotation_deg(),
      properties.flip_horizontal(),
      properties.flip_vertical(),
    ),
  );
  let (offset_x_pt, offset_y_pt, width_pt, height_pt) =
    (mapped.x_pt, mapped.y_pt, mapped.width_pt, mapped.height_pt);
  if has_path_geometry
    && let Some(path_geometry) =
      drawingml_path_geometry_from_properties(&properties, width_pt, height_pt)
  {
    geometry = path_geometry;
  }
  let mut shape = InlineShape {
    width_pt,
    height_pt,
    effect_left_pt: context.effect_extent.left_pt,
    effect_top_pt: context.effect_extent.top_pt,
    effect_right_pt: context.effect_extent.right_pt,
    effect_bottom_pt: context.effect_extent.bottom_pt,
    geometry,
    offset_x_pt,
    offset_y_pt,
    rotation_deg: properties.camera_adjusted_rotation_deg(mapped.rotation_deg),
    flip_horizontal: mapped.flip_horizontal,
    flip_vertical: mapped.flip_vertical,
    fill_color,
    fill_pattern,
    fill_override: fill_override.map(Box::new),
    additional_fill_colors: Vec::new(),
    fill_image,
    stroke,
    stroke_pattern,
    stroke_override: stroke_override.map(Box::new),
    suppress_zero_relative_background: explicit_fill_color.is_some(),
    allow_outside_page: false,
    inline_anchor_after_line: matches!(placement, ImagePlacement::Inline)
      && shape.text_body.is_some(),
    placement,
    chart: None,
    text_warp: None,
    text_fill: None,
    effects: properties.effects(&context.styles.theme_colors, Some(context.images)),
    static3d: properties.static3d(&context.styles.theme_colors),
    text_upright: false,
    text_box_blocks: Vec::new(),
    text_inset_left_pt: 0.0,
    text_inset_top_pt: 0.0,
    text_inset_right_pt: 0.0,
    text_inset_bottom_pt: 0.0,
    text_box_auto_fit: false,
    text_box_resizes_height_to_fit: false,
    text_box_word_wrap: true,
    text_vertical_alignment: TextBoxVerticalAlignment::Top,
  };
  if let Some(text_box) = text_box.take() {
    shape.text_box_blocks = text_box.blocks;
    shape.text_inset_left_pt = text_box.left_pt;
    shape.text_inset_top_pt = text_box.top_pt;
    shape.text_inset_right_pt = text_box.right_pt;
    shape.text_inset_bottom_pt = text_box.bottom_pt;
    shape.text_vertical_alignment = text_box.vertical_alignment;
  }
  Some(shape)
}

fn drawingml_diagram_shape_text_box(
  shape: &dsp::Shape,
  styles: &StylesCatalog,
  smartart_text_color: Option<RgbColor>,
) -> Option<TextBoxFrameContent> {
  let text_body = shape.text_body.as_ref()?;
  let texts = drawingml_text_body_texts(&text_body.paragraph);
  if texts.is_empty() {
    return None;
  }
  let color = smartart_text_color.unwrap_or_else(|| TextStyle::default().color);
  let blocks = texts
    .into_iter()
    .map(|text| simple_text_block(text, text_style_with_color(styles, color)))
    .collect();
  let mut frame = TextBoxFrameContent::new(blocks);
  apply_drawingml_textbox_body_properties_model(
    drawingml_body_properties_from_model(&text_body.body_properties),
    &mut frame,
  );
  apply_drawingml_textbox_layout_adjustments(&mut frame, false, None);
  Some(frame)
}

fn diagram_text_fill_colors_by_model_id(
  data: &dgm::DataModelRoot,
  colors: &dgm::ColorsDefinition,
  theme_colors: &ThemeColors,
) -> HashMap<String, RgbColor> {
  let text_colors_by_style_label = diagram_text_fill_colors_by_style_label(colors, theme_colors);
  if text_colors_by_style_label.is_empty() {
    return HashMap::new();
  }

  let mut colors = HashMap::new();
  for point in data
    .point_list
    .xml_children
    .iter()
    .filter_map(|child| match child {
      dgm::PointListChoice::Point(point) => Some(point.as_ref()),
      dgm::PointListChoice::AlternateContent(_) => None,
    })
  {
    let Some(style_label) = point
      .property_set
      .as_deref()
      .and_then(|properties| properties.presentation_style_label.as_deref())
    else {
      continue;
    };
    let Some(color) = text_colors_by_style_label.get(style_label) else {
      continue;
    };
    colors.insert(point.model_id.clone(), *color);
  }

  colors
}

fn diagram_text_fill_colors_by_style_label(
  colors_definition: &dgm::ColorsDefinition,
  theme_colors: &ThemeColors,
) -> HashMap<String, RgbColor> {
  let mut colors = HashMap::new();
  for label in &colors_definition.color_transform_style_label {
    let Some(color) = diagram_style_text_fill_color(label, theme_colors) else {
      continue;
    };
    colors.insert(label.name.clone(), color);
  }
  colors
}

fn diagram_style_text_fill_color(
  label: &dgm::ColorTransformStyleLabel,
  theme_colors: &ThemeColors,
) -> Option<RgbColor> {
  label
    .text_fill_color_list
    .as_ref()?
    .text_fill_color_list_choice
    .iter()
    .find_map(|choice| match choice {
      dgm::TextFillColorListChoice::RgbColorModelHex(color) => parse_hex_color(color.val.as_str()),
      dgm::TextFillColorListChoice::SchemeColor(color) => {
        resolve_drawingml_scheme_color(color, theme_colors)
      }
      dgm::TextFillColorListChoice::PresetColor(color) => drawingml_preset_color_value(color.val),
      _ => None,
    })
}

fn diagram_ext_drawing_relationship_id(data: &dgm::DataModelRoot) -> Option<String> {
  data
    .data_model_extension_list
    .as_ref()?
    .data_model_extension
    .iter()
    .find_map(
      |extension| match extension.data_model_extension_choice.as_ref()? {
        a::DataModelExtensionChoice::DataModelExtensionBlock(block) => block.rel_id.clone(),
        _ => None,
      },
    )
}

fn drawing_chart_shapes(
  drawing: &w::Drawing,
  reference: &c::ChartReference,
  charts_by_relationship_id: &HashMap<String, c::ChartSpace>,
  extended_charts_by_relationship_id: &HashMap<String, ExtendedChartResource>,
  styles: &StylesCatalog,
) -> Option<Vec<InlineShape>> {
  let Some(chart_space) = charts_by_relationship_id.get(reference.id.as_str()) else {
    return drawing_extended_chart_shapes(
      drawing,
      reference.id.as_str(),
      extended_charts_by_relationship_id,
      styles,
    );
  };
  let (width_pt, height_pt, placement) = drawing_chart_extent_and_placement(drawing)?;
  let effect_extent = drawing_effect_extent(drawing);
  let placement = drawing_placement_with_effect_extent(placement, effect_extent);
  let theme_series_colors = [
    styles.theme_colors.accent1,
    styles.theme_colors.accent2,
    styles.theme_colors.accent3,
    styles.theme_colors.accent4,
    styles.theme_colors.accent5,
    styles.theme_colors.accent6,
  ];
  let fallback_series_colors = [
    RgbColor {
      r: 79,
      g: 129,
      b: 189,
    },
    RgbColor {
      r: 192,
      g: 80,
      b: 77,
    },
    RgbColor {
      r: 155,
      g: 187,
      b: 89,
    },
    RgbColor {
      r: 128,
      g: 100,
      b: 162,
    },
    RgbColor {
      r: 75,
      g: 172,
      b: 198,
    },
    RgbColor {
      r: 247,
      g: 150,
      b: 70,
    },
  ];
  let cartesian = shared_chart::cartesian_chart_for_ui_language(
    chart_space,
    styles.simplified_chinese_ui.then_some("zh-CN"),
  );
  let series_count = shared_chart::series(chart_space).len();
  let series_colors = (0..series_count)
    .map(|index| {
      cartesian
        .as_ref()
        .and_then(|chart| chart.series.get(index))
        .and_then(|series| series.solid_fill)
        .and_then(|fill| resolve_drawingml_solid_fill(fill, &styles.theme_colors))
        .map(|fill| fill.color)
        .or(theme_series_colors[index % theme_series_colors.len()])
        .unwrap_or(fallback_series_colors[index % fallback_series_colors.len()])
    })
    .collect();
  let series_gradient_fills = shared_chart::series(chart_space)
    .into_iter()
    .map(|series| {
      let fill = series
        .chart_shape_properties?
        .chart_shape_properties_choice2
        .as_ref()?;
      let c::ChartShapePropertiesChoice2::GradientFill(fill) = fill else {
        return None;
      };
      match drawingml_gradient_fill(fill, &styles.theme_colors)? {
        common::Fill::Gradient(gradient) => Some(gradient),
        common::Fill::None
        | common::Fill::Solid(_)
        | common::Fill::Theme(_)
        | common::Fill::Image { .. }
        | common::Fill::Pattern(_) => None,
      }
    })
    .collect();
  let series_point_colors = cartesian
    .as_ref()
    .map(|chart| {
      chart
        .series
        .iter()
        .map(|series| {
          (0..series.values.len())
            .map(|point_index| {
              series
                .data_point_fills
                .iter()
                .find(|fill| fill.index as usize == point_index)
                .and_then(|fill| resolve_drawingml_solid_fill(fill.fill, &styles.theme_colors))
                .map(|fill| fill.color)
            })
            .collect()
        })
        .collect()
    })
    .unwrap_or_default();
  let surface_band_colors = cartesian
    .as_ref()
    .map(|chart| {
      chart
        .surface_groups
        .iter()
        .map(|group| {
          group
            .band_fills
            .iter()
            .filter_map(|fill| {
              resolve_drawingml_solid_fill(fill.fill, &styles.theme_colors)
                .map(|resolved| (fill.index, resolved.color))
            })
            .collect()
        })
        .collect()
    })
    .unwrap_or_default();
  let pie_point_colors = shared_chart::pie_chart_model(chart_space)
    .map(|pie| {
      (0..pie.values.len())
        .map(|index| {
          pie
            .data_point_fills
            .iter()
            .find(|fill| fill.index as usize == index)
            .and_then(|fill| resolve_drawingml_solid_fill(fill.fill, &styles.theme_colors))
            .or_else(|| {
              pie
                .series_solid_fill
                .and_then(|fill| resolve_drawingml_solid_fill(fill, &styles.theme_colors))
            })
            .map(|fill| fill.color)
            .or_else(|| {
              let color_index = if pie.vary_colors { index } else { 0 };
              theme_series_colors[color_index % theme_series_colors.len()]
            })
            .unwrap_or_else(|| {
              let color_index = if pie.vary_colors { index } else { 0 };
              fallback_series_colors[color_index % fallback_series_colors.len()]
            })
        })
        .collect()
    })
    .unwrap_or_default();
  let chart_area_fill_color = chart_space
    .shape_properties
    .as_deref()
    .and_then(shared_chart::shape_properties_solid_fill)
    .and_then(|fill| resolve_drawingml_solid_fill(fill, &styles.theme_colors))
    .map(|fill| fill.color);
  let plot_area_fill_color = chart_space
    .chart
    .plot_area
    .shape_properties
    .as_deref()
    .and_then(shared_chart::shape_properties_solid_fill)
    .and_then(|fill| resolve_drawingml_solid_fill(fill, &styles.theme_colors))
    .map(|fill| fill.color);
  let chart_area_stroke_color = chart_space
    .shape_properties
    .as_deref()
    .and_then(shared_chart::shape_properties_outline_solid_fill)
    .and_then(|fill| resolve_drawingml_solid_fill(fill, &styles.theme_colors))
    .map(|fill| fill.color);
  let plot_area_stroke_color = chart_space
    .chart
    .plot_area
    .shape_properties
    .as_deref()
    .and_then(shared_chart::shape_properties_outline_solid_fill)
    .and_then(|fill| resolve_drawingml_solid_fill(fill, &styles.theme_colors))
    .map(|fill| fill.color);
  let title_fill_color = chart_space
    .chart
    .title
    .as_deref()
    .and_then(|title| title.chart_shape_properties.as_deref())
    .and_then(
      |properties| match properties.chart_shape_properties_choice2.as_ref()? {
        c::ChartShapePropertiesChoice2::SolidFill(fill) => {
          resolve_drawingml_solid_fill(fill, &styles.theme_colors).map(|fill| fill.color)
        }
        c::ChartShapePropertiesChoice2::GradientFill(fill) => {
          drawingml_first_gradient_fill_color(fill, &styles.theme_colors)
        }
        _ => None,
      },
    );
  let chart_font = shared_chart::fixed_output_latin_font_family(chart_space)
    .map(|typeface| styles.theme_fonts.resolve_drawingml_typeface(typeface))
    .or_else(|| styles.theme_fonts.minor_high_ansi.clone())
    .or_else(|| styles.theme_fonts.minor_ascii.clone());
  let mut title_style = TextStyle {
    font_family: chart_font.clone(),
    font_size_pt: 18.0,
    bold: true,
    ..TextStyle::default()
  };
  let mut label_style = TextStyle {
    font_family: chart_font,
    font_size_pt: 10.0,
    ..TextStyle::default()
  };
  title_style.fallback_font_family = styles.doc_default_run.fallback_font_family.clone();
  label_style.fallback_font_family = styles.doc_default_run.fallback_font_family.clone();
  let chart_east_asia_font = if styles.simplified_chinese_ui {
    Some(
      styles
        .theme_fonts
        .resolve_drawingml_typeface_for_language("+mn-ea", Some("zh-CN")),
    )
  } else {
    styles.doc_default_run.east_asia_font_family.clone()
  };
  title_style.east_asia_font_family = chart_east_asia_font.clone();
  label_style.east_asia_font_family = chart_east_asia_font;
  let mut data_label_style = label_style.clone();
  if let Some(properties) = chart_space.text_properties.as_deref() {
    apply_chart_text_properties(&mut title_style, properties, styles);
    apply_chart_text_properties(&mut label_style, properties, styles);
    apply_chart_text_properties(&mut data_label_style, properties, styles);
  }
  if let Some(properties) = chart_space
    .chart
    .title
    .as_deref()
    .and_then(|title| title.text_properties.as_deref())
  {
    apply_chart_text_properties(&mut title_style, properties, styles);
  }
  if let Some(title) = chart_space.chart.title.as_deref() {
    apply_chart_rich_title_properties(&mut title_style, title, styles);
  }
  let mut legend_style = label_style.clone();
  if let Some(properties) = chart_space
    .chart
    .legend
    .as_deref()
    .and_then(|legend| legend.text_properties.as_deref())
  {
    apply_chart_text_properties(&mut legend_style, properties, styles);
  }
  let mut category_label_style = label_style.clone();
  if let Some(properties) = cartesian.as_ref().and_then(|chart| {
    chart
      .category_axis
      .and_then(|axis| axis.text_properties.as_deref())
      .or_else(|| {
        chart
          .date_axis
          .and_then(|axis| axis.text_properties.as_deref())
      })
      .or_else(|| {
        chart
          .series
          .iter()
          .all(|series| {
            matches!(
              series.kind,
              shared_chart::ChartSeriesKind::Scatter | shared_chart::ChartSeriesKind::Bubble
            )
          })
          .then(|| {
            chart
              .horizontal_value_axis
              .and_then(|axis| axis.text_properties.as_deref())
          })
          .flatten()
      })
  }) {
    apply_chart_text_properties(&mut category_label_style, properties, styles);
  }
  let mut value_label_style = label_style.clone();
  if let Some(properties) = cartesian
    .as_ref()
    .and_then(|chart| chart.value_axis)
    .and_then(|axis| axis.text_properties.as_deref())
  {
    apply_chart_text_properties(&mut value_label_style, properties, styles);
  }
  let mut series_label_style = label_style.clone();
  if let Some(properties) = cartesian.as_ref().and_then(|chart| {
    chart
      .axis_sets
      .iter()
      .find_map(|axes| axes.series_axis?.text_properties.as_deref())
  }) {
    apply_chart_text_properties(&mut series_label_style, properties, styles);
  }
  if let Some(properties) = shared_chart::pie_chart_model(chart_space)
    .and_then(|model| model.data_label_text_properties)
    .or_else(|| {
      cartesian
        .as_ref()
        .and_then(|model| model.data_label_text_properties)
    })
  {
    apply_chart_text_properties(&mut data_label_style, properties, styles);
  }
  let data_label_styles = cartesian
    .as_ref()
    .map(|chart| {
      chart
        .series
        .iter()
        .map(|series| {
          series
            .data_labels
            .iter()
            .map(|label| {
              label.text_properties.map(|properties| {
                let mut style = label_style.clone();
                apply_chart_text_properties(&mut style, properties, styles);
                style
              })
            })
            .collect()
        })
        .collect()
    })
    .unwrap_or_default();
  let data_label_fill_colors = cartesian
    .as_ref()
    .map(|chart| {
      chart
        .series
        .iter()
        .map(|series| {
          series
            .data_labels
            .iter()
            .map(|label| {
              label
                .shape_properties
                .and_then(shared_chart::chart_shape_solid_fill)
                .and_then(|fill| resolve_drawingml_solid_fill(fill, &styles.theme_colors))
                .map(|fill| fill.color)
            })
            .collect()
        })
        .collect()
    })
    .unwrap_or_default();
  let ui_language = if styles.simplified_chinese_ui {
    Some("zh-CN".to_string())
  } else {
    chart_space
      .editing_language
      .as_ref()
      .map(|language| language.val.to_string())
  };
  let automatic_title = shared_chart::automatic_chart_title(ui_language.as_deref()).to_string();
  let gridline_color = cartesian
    .as_ref()
    .and_then(|chart| chart.value_axis)
    .and_then(|axis| axis.major_gridlines.as_deref())
    .and_then(|gridlines| gridlines.chart_shape_properties.as_deref())
    .and_then(shared_chart::chart_shape_outline_solid_fill)
    .and_then(|fill| resolve_drawingml_solid_fill(fill, &styles.theme_colors))
    .map(|fill| fill.color)
    .unwrap_or(RgbColor {
      r: 134,
      g: 134,
      b: 134,
    });
  let value_gridline_width_pt = cartesian
    .as_ref()
    .and_then(|chart| chart.value_axis)
    .and_then(|axis| axis.major_gridlines.as_deref())
    .and_then(|gridlines| gridlines.chart_shape_properties.as_deref())
    .and_then(chart_shape_outline_width_pt);
  let axis_line_width_pt = cartesian
    .as_ref()
    .and_then(|chart| {
      chart
        .date_axis
        .and_then(|axis| axis.chart_shape_properties.as_deref())
        .or_else(|| {
          chart
            .category_axis
            .and_then(|axis| axis.chart_shape_properties.as_deref())
        })
        .or_else(|| {
          chart
            .horizontal_value_axis
            .and_then(|axis| axis.chart_shape_properties.as_deref())
        })
    })
    .and_then(chart_shape_outline_width_pt);
  let category_major_gridline = cartesian.as_ref().and_then(|chart| {
    let properties = chart
      .date_axis?
      .major_gridlines
      .as_deref()?
      .chart_shape_properties
      .as_deref()?;
    let color = shared_chart::chart_shape_outline_solid_fill(properties)
      .and_then(|fill| resolve_drawingml_solid_fill(fill, &styles.theme_colors))?
      .color;
    Some((color, chart_shape_outline_width_pt(properties)?))
  });
  let category_minor_gridline = cartesian.as_ref().and_then(|chart| {
    let properties = chart
      .date_axis?
      .minor_gridlines
      .as_deref()?
      .chart_shape_properties
      .as_deref()?;
    let color = shared_chart::chart_shape_outline_solid_fill(properties)
      .and_then(|fill| resolve_drawingml_solid_fill(fill, &styles.theme_colors))?
      .color;
    Some((color, chart_shape_outline_width_pt(properties)?))
  });
  let chart_area_stroke_width_pt = chart_space
    .shape_properties
    .as_deref()
    .and_then(shape_properties_outline_width_pt);
  let plot_area_stroke_width_pt = chart_space
    .chart
    .plot_area
    .shape_properties
    .as_deref()
    .and_then(shape_properties_outline_width_pt);
  let mut shape = chart_shape(width_pt, height_pt, 0.0, placement, None);
  apply_drawing_effect_extent_to_shape(&mut shape, effect_extent);
  shape.chart = Some(Box::new(InlineChart {
    chart_space: Some(Box::new(chart_space.clone())),
    extended_chart_space: None,
    extended_chart_styles: Vec::new(),
    extended_chart_color_styles: Vec::new(),
    extended_chart_theme: crate::render::chartex::ChartExTheme::default(),
    ui_language,
    automatic_title,
    title_style,
    label_style: legend_style,
    category_label_style,
    value_label_style,
    series_label_style,
    data_label_style,
    data_label_styles,
    gridline_color,
    value_gridline_width_pt,
    axis_line_width_pt,
    category_major_gridline,
    category_minor_gridline,
    series_colors,
    series_gradient_fills,
    series_point_colors,
    surface_band_colors,
    data_label_fill_colors,
    pie_point_colors,
    title_fill_color,
    chart_area_fill_color,
    plot_area_fill_color,
    chart_area_stroke_color,
    chart_area_stroke_width_pt,
    plot_area_stroke_color,
    plot_area_stroke_width_pt,
  }));
  Some(vec![shape])
}

fn chart_shape_outline_width_pt(properties: &c::ChartShapeProperties) -> Option<f32> {
  properties
    .outline
    .as_deref()?
    .width
    .map(|width| units::emu_to_points(i64::from(width)))
}

fn shape_properties_outline_width_pt(properties: &c::ShapeProperties) -> Option<f32> {
  properties
    .outline
    .as_deref()?
    .width
    .map(|width| units::emu_to_points(i64::from(width)))
}

fn chart_ex_theme(colors: &ThemeColors) -> crate::render::chartex::ChartExTheme {
  let defaults = crate::render::chartex::ChartExTheme::default();
  crate::render::chartex::ChartExTheme {
    dark1: colors.dark1.unwrap_or(defaults.dark1),
    light1: colors.light1.unwrap_or(defaults.light1),
    dark2: colors.dark2.unwrap_or(defaults.dark2),
    light2: colors.light2.unwrap_or(defaults.light2),
    accents: [
      colors.accent1.unwrap_or(defaults.accents[0]),
      colors.accent2.unwrap_or(defaults.accents[1]),
      colors.accent3.unwrap_or(defaults.accents[2]),
      colors.accent4.unwrap_or(defaults.accents[3]),
      colors.accent5.unwrap_or(defaults.accents[4]),
      colors.accent6.unwrap_or(defaults.accents[5]),
    ],
    hyperlink: colors.hyperlink.unwrap_or(defaults.hyperlink),
    followed_hyperlink: colors
      .followed_hyperlink
      .unwrap_or(defaults.followed_hyperlink),
  }
}

fn drawing_extended_chart_shapes(
  drawing: &w::Drawing,
  relationship_id: &str,
  charts_by_relationship_id: &HashMap<String, ExtendedChartResource>,
  styles: &StylesCatalog,
) -> Option<Vec<InlineShape>> {
  let resource = charts_by_relationship_id.get(relationship_id)?;
  let chart_space = &resource.chart_space;
  let (width_pt, height_pt, placement) = drawing_chart_extent_and_placement(drawing)?;
  let effect_extent = drawing_effect_extent(drawing);
  let placement = drawing_placement_with_effect_extent(placement, effect_extent);
  let chart_font = styles
    .theme_fonts
    .minor_high_ansi
    .clone()
    .or_else(|| styles.theme_fonts.minor_ascii.clone());
  let mut title_style = TextStyle {
    font_family: chart_font.clone(),
    font_size_pt: 14.0,
    bold: true,
    ..TextStyle::default()
  };
  let mut label_style = TextStyle {
    font_family: chart_font,
    font_size_pt: 9.0,
    ..TextStyle::default()
  };
  title_style.fallback_font_family = styles.doc_default_run.fallback_font_family.clone();
  label_style.fallback_font_family = styles.doc_default_run.fallback_font_family.clone();
  let chart_east_asia_font = if styles.simplified_chinese_ui {
    Some(
      styles
        .theme_fonts
        .resolve_drawingml_typeface_for_language("+mn-ea", Some("zh-CN")),
    )
  } else {
    styles.doc_default_run.east_asia_font_family.clone()
  };
  title_style.east_asia_font_family = chart_east_asia_font.clone();
  label_style.east_asia_font_family = chart_east_asia_font;
  let mut shape = chart_shape(width_pt, height_pt, 0.0, placement, None);
  apply_drawing_effect_extent_to_shape(&mut shape, effect_extent);
  shape.chart = Some(Box::new(InlineChart {
    chart_space: None,
    extended_chart_space: Some(Box::new(chart_space.clone())),
    extended_chart_styles: resource.chart_styles.clone(),
    extended_chart_color_styles: resource.color_styles.clone(),
    extended_chart_theme: chart_ex_theme(&styles.theme_colors),
    ui_language: styles.simplified_chinese_ui.then(|| "zh-CN".to_string()),
    automatic_title: shared_chart::automatic_chart_title(
      styles.simplified_chinese_ui.then_some("zh-CN"),
    )
    .to_string(),
    title_style,
    label_style: label_style.clone(),
    category_label_style: label_style.clone(),
    value_label_style: label_style.clone(),
    series_label_style: label_style.clone(),
    data_label_style: label_style,
    data_label_styles: Vec::new(),
    gridline_color: RgbColor {
      r: 134,
      g: 134,
      b: 134,
    },
    value_gridline_width_pt: None,
    axis_line_width_pt: None,
    category_major_gridline: None,
    category_minor_gridline: None,
    series_colors: Vec::new(),
    series_gradient_fills: Vec::new(),
    series_point_colors: Vec::new(),
    surface_band_colors: Vec::new(),
    data_label_fill_colors: Vec::new(),
    pie_point_colors: Vec::new(),
    title_fill_color: None,
    chart_area_fill_color: None,
    plot_area_fill_color: None,
    chart_area_stroke_color: None,
    chart_area_stroke_width_pt: None,
    plot_area_stroke_color: None,
    plot_area_stroke_width_pt: None,
  }));
  Some(vec![shape])
}

fn apply_chart_text_properties(
  style: &mut TextStyle,
  properties: &c::TextProperties,
  styles: &StylesCatalog,
) {
  let Some(properties) = properties
    .paragraph
    .iter()
    .filter_map(|paragraph| paragraph.paragraph_properties.as_deref())
    .find_map(|paragraph| paragraph.default_run_properties.as_deref())
    .or_else(|| {
      properties
        .list_style
        .as_deref()
        .and_then(|style| style.default_paragraph_properties.as_deref())
        .and_then(|paragraph| paragraph.default_run_properties.as_deref())
    })
  else {
    return;
  };
  apply_chart_default_run_properties(style, properties, styles);
}

fn apply_chart_rich_title_properties(
  style: &mut TextStyle,
  title: &c::Title,
  styles: &StylesCatalog,
) {
  let Some(c::ChartTextChoice::RichText(rich)) = title
    .chart_text
    .as_deref()
    .and_then(|text| text.chart_text_choice.as_ref())
  else {
    return;
  };
  let Some(paragraph) = rich.paragraph.first() else {
    return;
  };
  if let Some(properties) = paragraph
    .paragraph_properties
    .as_deref()
    .and_then(|properties| properties.default_run_properties.as_deref())
  {
    apply_chart_default_run_properties(style, properties, styles);
  }
  if let Some(properties) = paragraph
    .paragraph_choice
    .iter()
    .find_map(|choice| match choice {
      a::ParagraphChoice::Run(run) => run.run_properties.as_deref(),
      a::ParagraphChoice::Field(field) => field.run_properties.as_deref(),
      a::ParagraphChoice::Break(_)
      | a::ParagraphChoice::TextMath(_)
      | a::ParagraphChoice::AlternateContent(_) => None,
    })
  {
    apply_chart_run_properties(style, properties, styles);
  }
}

fn apply_chart_default_run_properties(
  style: &mut TextStyle,
  properties: &a::DefaultRunProperties,
  styles: &StylesCatalog,
) {
  if let Some(size) = properties.font_size.filter(|size| *size > 0) {
    style.font_size_pt = size as f32 / 100.0;
  }
  if let Some(bold) = properties.bold.as_ref() {
    style.bold = bold.as_bool();
  }
  if let Some(italic) = properties.italic.as_ref() {
    style.italic = italic.as_bool();
  }
  if let Some(typeface) = properties
    .latin_font
    .as_ref()
    .and_then(|font| font.typeface.as_deref())
    .filter(|typeface| !typeface.trim().is_empty())
  {
    style.font_family = Some(styles.theme_fonts.resolve_drawingml_typeface(typeface));
  }
  if let Some(typeface) = properties
    .east_asian_font
    .as_ref()
    .and_then(|font| font.typeface.as_deref())
    .filter(|typeface| !typeface.trim().is_empty())
  {
    style.east_asia_font_family = Some(styles.theme_fonts.resolve_drawingml_typeface_for_language(
      typeface,
      styles.simplified_chinese_ui.then_some("zh-CN"),
    ));
  }
  if let Some(a::DefaultRunPropertiesChoice::SolidFill(fill)) =
    properties.default_run_properties_choice1.as_ref()
    && let Some(color) = resolve_drawingml_solid_fill(fill, &styles.theme_colors)
  {
    style.color = color.color;
    style.opacity = color.opacity;
  }
}

fn apply_chart_run_properties(
  style: &mut TextStyle,
  properties: &a::RunProperties,
  styles: &StylesCatalog,
) {
  if let Some(size) = properties.font_size.filter(|size| *size > 0) {
    style.font_size_pt = size as f32 / 100.0;
  }
  if let Some(bold) = properties.bold.as_ref() {
    style.bold = bold.as_bool();
  }
  if let Some(italic) = properties.italic.as_ref() {
    style.italic = italic.as_bool();
  }
  if let Some(typeface) = properties
    .latin_font
    .as_ref()
    .and_then(|font| font.typeface.as_deref())
    .filter(|typeface| !typeface.trim().is_empty())
  {
    style.font_family = Some(styles.theme_fonts.resolve_drawingml_typeface(typeface));
  }
  if let Some(typeface) = properties
    .east_asian_font
    .as_ref()
    .and_then(|font| font.typeface.as_deref())
    .filter(|typeface| !typeface.trim().is_empty())
  {
    style.east_asia_font_family = Some(styles.theme_fonts.resolve_drawingml_typeface_for_language(
      typeface,
      styles.simplified_chinese_ui.then_some("zh-CN"),
    ));
  }
  if let Some(a::RunPropertiesChoice::SolidFill(fill)) = properties.run_properties_choice1.as_ref()
    && let Some(color) = resolve_drawingml_solid_fill(fill, &styles.theme_colors)
  {
    style.color = color.color;
    style.opacity = color.opacity;
  }
}

fn drawing_chart_extent_and_placement(drawing: &w::Drawing) -> Option<(f32, f32, ImagePlacement)> {
  match drawing.drawing_choice.as_ref()? {
    w::DrawingChoice::Inline(inline) => Some((
      units::emu_to_points(inline.extent.cx),
      units::emu_to_points(inline.extent.cy),
      ImagePlacement::Inline,
    )),
    w::DrawingChoice::Anchor(anchor) => {
      let extent = &anchor.extent;
      Some((
        units::emu_to_points(extent.cx),
        units::emu_to_points(extent.cy),
        ImagePlacement::Floating(floating_image_placement(anchor)),
      ))
    }
  }
}

fn drawing_extent_size(drawing: &w::Drawing) -> Option<(f32, f32)> {
  match drawing.drawing_choice.as_ref()? {
    w::DrawingChoice::Inline(inline) => Some((
      units::emu_to_points(inline.extent.cx),
      units::emu_to_points(inline.extent.cy),
    )),
    w::DrawingChoice::Anchor(anchor) => {
      let extent = &anchor.extent;
      Some((
        units::emu_to_points(extent.cx),
        units::emu_to_points(extent.cy),
      ))
    }
  }
}

fn drawing_effect_extent(drawing: &w::Drawing) -> DrawingEffectExtent {
  let extent = match drawing.drawing_choice.as_ref() {
    Some(w::DrawingChoice::Inline(inline)) => inline.effect_extent.as_ref(),
    Some(w::DrawingChoice::Anchor(anchor)) => anchor.effect_extent.as_ref(),
    None => None,
  };
  DrawingEffectExtent {
    left_pt: effect_extent_left(extent),
    top_pt: effect_extent_top(extent),
    right_pt: effect_extent_right(extent),
    bottom_pt: effect_extent_bottom(extent),
  }
}

fn drawing_placement_with_effect_extent(
  placement: ImagePlacement,
  extent: DrawingEffectExtent,
) -> ImagePlacement {
  match placement {
    ImagePlacement::Floating(mut placement) => {
      // wp:effectExtent is part of the floating object's wrap bounds. The
      // child geometry remains in the authored wp:extent coordinate space;
      // folding these distances into the wrap margins expands the exclusion
      // once without translating or independently resizing group children.
      placement.margin_left_pt += extent.left_pt.max(0.0);
      placement.margin_top_pt += extent.top_pt.max(0.0);
      placement.margin_right_pt += extent.right_pt.max(0.0);
      placement.margin_bottom_pt += extent.bottom_pt.max(0.0);
      ImagePlacement::Floating(placement)
    }
    ImagePlacement::Inline => ImagePlacement::Inline,
  }
}

fn apply_drawing_effect_extent_to_shape(
  shape: &mut InlineShape,
  effect_extent: DrawingEffectExtent,
) {
  shape.effect_left_pt = effect_extent.left_pt;
  shape.effect_top_pt = effect_extent.top_pt;
  shape.effect_right_pt = effect_extent.right_pt;
  shape.effect_bottom_pt = effect_extent.bottom_pt;
}

fn chart_shape(
  width_pt: f32,
  height_pt: f32,
  offset_y_pt: f32,
  placement: ImagePlacement,
  stroke: Option<BorderStyle>,
) -> InlineShape {
  InlineShape {
    width_pt,
    height_pt,
    effect_left_pt: 0.0,
    effect_top_pt: 0.0,
    effect_right_pt: 0.0,
    effect_bottom_pt: 0.0,
    geometry: InlineShapeGeometry::Rectangle,
    offset_x_pt: 0.0,
    offset_y_pt,
    rotation_deg: 0.0,
    flip_horizontal: false,
    flip_vertical: false,
    fill_color: None,
    fill_pattern: None,
    fill_override: None,
    additional_fill_colors: Vec::new(),
    fill_image: None,
    stroke,
    stroke_pattern: None,
    stroke_override: None,
    suppress_zero_relative_background: false,
    allow_outside_page: false,
    inline_anchor_after_line: false,
    placement,
    chart: None,
    text_warp: None,
    text_fill: None,
    effects: None,
    static3d: None,
    text_upright: false,
    text_box_blocks: Vec::new(),
    text_inset_left_pt: 0.0,
    text_inset_top_pt: 0.0,
    text_inset_right_pt: 0.0,
    text_inset_bottom_pt: 0.0,
    text_box_auto_fit: false,
    text_box_resizes_height_to_fit: false,
    text_box_word_wrap: true,
    text_vertical_alignment: TextBoxVerticalAlignment::Top,
  }
}

#[derive(Clone, Copy, Debug)]
struct DrawingMlGroupTransform {
  affine: Affine,
  raw_coordinates: bool,
  fallback_size: Option<(f32, f32)>,
}

#[derive(Clone, Copy, Debug, Default)]
struct DrawingEffectExtent {
  left_pt: f32,
  top_pt: f32,
  right_pt: f32,
  bottom_pt: f32,
}

impl DrawingMlGroupTransform {
  fn identity() -> Self {
    Self {
      affine: Affine::IDENTITY,
      raw_coordinates: false,
      fallback_size: None,
    }
  }

  fn with_fallback_size(mut self, fallback_size: Option<(f32, f32)>) -> Self {
    self.fallback_size = fallback_size;
    self
  }

  fn child(self, xfrm: DrawingMlGroupXfrm) -> Self {
    let child_coordinates = common::drawingml_geometry::group_child_affine(
      kurbo::Point::new(f64::from(xfrm.offset_x_pt), f64::from(xfrm.offset_y_pt)),
      kurbo::Vec2::new(f64::from(xfrm.width_pt), f64::from(xfrm.height_pt)),
      kurbo::Point::new(
        f64::from(xfrm.child_offset_x),
        f64::from(xfrm.child_offset_y),
      ),
      kurbo::Vec2::new(f64::from(xfrm.child_width), f64::from(xfrm.child_height)),
    );
    let center_x = xfrm.offset_x_pt + xfrm.width_pt / 2.0;
    let center_y = xfrm.offset_y_pt + xfrm.height_pt / 2.0;
    let orientation = Affine::translate((-f64::from(center_x), -f64::from(center_y)))
      .then_scale_non_uniform(
        if xfrm.flip_horizontal { -1.0 } else { 1.0 },
        if xfrm.flip_vertical { -1.0 } else { 1.0 },
      )
      .then_rotate(f64::from(xfrm.rotation_deg.to_radians()))
      .then_translate((f64::from(center_x), f64::from(center_y)).into());
    Self {
      affine: self.affine * orientation * child_coordinates,
      raw_coordinates: true,
      fallback_size: None,
    }
  }

  fn rotation_degrees(self) -> f32 {
    let horizontal =
      common::drawingml_geometry::transform_vector(kurbo::Vec2::new(1.0, 0.0), self.affine);
    horizontal.y.atan2(horizontal.x).to_degrees() as f32
  }

  fn map_rect(
    self,
    rect: (f32, f32, f32, f32),
    orientation: (f32, bool, bool),
  ) -> DrawingMlMappedRect {
    let (x_pt, y_pt, width_pt, height_pt) = rect;
    let (rotation_deg, flip_horizontal, flip_vertical) = orientation;
    let center = kurbo::Point::new(
      f64::from(x_pt + width_pt * 0.5),
      f64::from(y_pt + height_pt * 0.5),
    );
    let local_orientation = Affine::translate((-center.x, -center.y))
      .then_scale_non_uniform(
        if flip_horizontal { -1.0 } else { 1.0 },
        if flip_vertical { -1.0 } else { 1.0 },
      )
      .then_rotate(f64::from(rotation_deg.to_radians()))
      .then_translate(center.to_vec2());
    let transform = self.affine * local_orientation;
    let mapped_center = transform * center;
    let horizontal =
      common::drawingml_geometry::transform_vector(kurbo::Vec2::new(1.0, 0.0), transform);
    let vertical =
      common::drawingml_geometry::transform_vector(kurbo::Vec2::new(0.0, 1.0), transform);
    let mapped_width = f64::from(width_pt) * horizontal.hypot();
    let mapped_height = f64::from(height_pt) * vertical.hypot();
    let determinant = horizontal.x * vertical.y - horizontal.y * vertical.x;
    DrawingMlMappedRect {
      x_pt: (mapped_center.x - mapped_width * 0.5) as f32,
      y_pt: (mapped_center.y - mapped_height * 0.5) as f32,
      width_pt: mapped_width as f32,
      height_pt: mapped_height as f32,
      rotation_deg: horizontal.y.atan2(horizontal.x).to_degrees() as f32,
      // Any reflected affine can be represented by one axis flip. Keeping it
      // on the vertical axis lets the first column retain the authored
      // rotation angle and avoids an arbitrary extra 180-degree turn.
      flip_horizontal: false,
      flip_vertical: determinant < 0.0,
    }
  }
}

#[derive(Clone, Copy, Debug)]
struct DrawingMlMappedRect {
  x_pt: f32,
  y_pt: f32,
  width_pt: f32,
  height_pt: f32,
  rotation_deg: f32,
  flip_horizontal: bool,
  flip_vertical: bool,
}

#[derive(Clone, Copy, Debug, Default)]
struct DrawingMlGroupXfrm {
  rotation_deg: f32,
  flip_horizontal: bool,
  flip_vertical: bool,
  offset_x_pt: f32,
  offset_y_pt: f32,
  width_pt: f32,
  height_pt: f32,
  child_offset_x: f32,
  child_offset_y: f32,
  child_width: f32,
  child_height: f32,
}

enum DrawingMlShapeProperties {
  Diagram(dsp::ShapeProperties),
  Generic(a::ShapeProperties),
  Wordprocessing(wps::ShapeProperties),
  Picture(pic::ShapeProperties),
}

enum DrawingMlFillProperties<'a> {
  NoFill,
  Solid(&'a a::SolidFill),
  Gradient(&'a a::GradientFill),
  Pattern(&'a a::PatternFill),
}

impl DrawingMlShapeProperties {
  fn transform2_d(&self) -> Option<&a::Transform2D> {
    match self {
      Self::Diagram(properties) => properties.transform2_d.as_deref(),
      Self::Generic(properties) => properties.transform2_d.as_deref(),
      Self::Wordprocessing(properties) => properties.transform2_d.as_deref(),
      Self::Picture(properties) => properties.transform2_d.as_deref(),
    }
  }

  fn rotation_deg(&self) -> f32 {
    self
      .transform2_d()
      .and_then(|transform| transform.rotation)
      .map(|value| sdk_units::drawingml_angle_to_degrees(value) as f32)
      .unwrap_or_default()
  }

  fn flip_horizontal(&self) -> bool {
    self
      .transform2_d()
      .and_then(|transform| transform.horizontal_flip.as_ref())
      .is_some_and(|value| value.as_bool())
  }

  fn flip_vertical(&self) -> bool {
    self
      .transform2_d()
      .and_then(|transform| transform.vertical_flip.as_ref())
      .is_some_and(|value| value.as_bool())
  }

  fn geometry_kind(&self) -> Option<InlineShapeGeometry> {
    let is_line = match self {
      Self::Diagram(properties) => matches!(
        properties.shape_properties_choice1.as_ref(),
        Some(dsp::ShapePropertiesChoice::PresetGeometry(geometry))
          if geometry.preset == a::ShapeTypeValues::Line
      ),
      Self::Generic(properties) => matches!(
        properties.shape_properties_choice1.as_ref(),
        Some(a::ShapePropertiesChoice::PresetGeometry(geometry))
          if geometry.preset == a::ShapeTypeValues::Line
      ),
      Self::Wordprocessing(properties) => matches!(
        properties.shape_properties_choice1.as_ref(),
        Some(wps::ShapePropertiesChoice::PresetGeometry(geometry))
          if geometry.preset == a::ShapeTypeValues::Line
      ),
      Self::Picture(properties) => matches!(
        properties.shape_properties_choice1.as_ref(),
        Some(pic::ShapePropertiesChoice::PresetGeometry(geometry))
          if geometry.preset == a::ShapeTypeValues::Line
      ),
    };

    Some(if is_line {
      InlineShapeGeometry::Line
    } else {
      InlineShapeGeometry::Rectangle
    })
  }

  fn custom_geometry(&self) -> Option<&a::CustomGeometry> {
    match self {
      Self::Diagram(properties) => match properties.shape_properties_choice1.as_ref()? {
        dsp::ShapePropertiesChoice::CustomGeometry(geometry) => Some(geometry.as_ref()),
        dsp::ShapePropertiesChoice::PresetGeometry(_) => None,
      },
      Self::Generic(properties) => match properties.shape_properties_choice1.as_ref()? {
        a::ShapePropertiesChoice::CustomGeometry(geometry) => Some(geometry.as_ref()),
        a::ShapePropertiesChoice::PresetGeometry(_) => None,
      },
      Self::Wordprocessing(properties) => match properties.shape_properties_choice1.as_ref()? {
        wps::ShapePropertiesChoice::CustomGeometry(geometry) => Some(geometry.as_ref()),
        wps::ShapePropertiesChoice::PresetGeometry(_) => None,
      },
      Self::Picture(properties) => match properties.shape_properties_choice1.as_ref()? {
        pic::ShapePropertiesChoice::CustomGeometry(geometry) => Some(geometry.as_ref()),
        pic::ShapePropertiesChoice::PresetGeometry(_) => None,
      },
    }
  }

  fn preset_geometry(&self) -> Option<&a::PresetGeometry> {
    match self {
      Self::Diagram(properties) => match properties.shape_properties_choice1.as_ref()? {
        dsp::ShapePropertiesChoice::PresetGeometry(geometry) => Some(geometry.as_ref()),
        dsp::ShapePropertiesChoice::CustomGeometry(_) => None,
      },
      Self::Generic(properties) => match properties.shape_properties_choice1.as_ref()? {
        a::ShapePropertiesChoice::PresetGeometry(geometry) => Some(geometry.as_ref()),
        a::ShapePropertiesChoice::CustomGeometry(_) => None,
      },
      Self::Wordprocessing(properties) => match properties.shape_properties_choice1.as_ref()? {
        wps::ShapePropertiesChoice::PresetGeometry(geometry) => Some(geometry.as_ref()),
        wps::ShapePropertiesChoice::CustomGeometry(_) => None,
      },
      Self::Picture(properties) => match properties.shape_properties_choice1.as_ref()? {
        pic::ShapePropertiesChoice::PresetGeometry(geometry) => Some(geometry.as_ref()),
        pic::ShapePropertiesChoice::CustomGeometry(_) => None,
      },
    }
  }

  fn has_path_geometry(&self) -> bool {
    self.custom_geometry().is_some()
      || self
        .preset_geometry()
        .is_some_and(|geometry| geometry.preset != a::ShapeTypeValues::Line)
  }

  fn outline(&self) -> Option<&a::Outline> {
    match self {
      Self::Diagram(properties) => properties.outline.as_deref(),
      Self::Generic(properties) => properties.outline.as_deref(),
      Self::Wordprocessing(properties) => properties.outline.as_deref(),
      Self::Picture(properties) => properties.outline.as_deref(),
    }
  }

  fn fill(&self) -> Option<DrawingMlFillProperties<'_>> {
    match self {
      Self::Diagram(properties) => match properties.shape_properties_choice2.as_ref()? {
        // MS-OI29500 §5.9.3.7: Office displays a diagram shape's grpFill
        // as an empty fill, regardless of its parent group paint.
        dsp::ShapePropertiesChoice2::NoFill(_) | dsp::ShapePropertiesChoice2::GroupFill => {
          Some(DrawingMlFillProperties::NoFill)
        }
        dsp::ShapePropertiesChoice2::SolidFill(fill) => {
          Some(DrawingMlFillProperties::Solid(fill.as_ref()))
        }
        dsp::ShapePropertiesChoice2::GradientFill(fill) => {
          Some(DrawingMlFillProperties::Gradient(fill.as_ref()))
        }
        dsp::ShapePropertiesChoice2::PatternFill(fill) => {
          Some(DrawingMlFillProperties::Pattern(fill.as_ref()))
        }
        _ => None,
      },
      Self::Generic(properties) => match properties.shape_properties_choice2.as_ref()? {
        a::ShapePropertiesChoice2::NoFill(_) | a::ShapePropertiesChoice2::GroupFill => {
          Some(DrawingMlFillProperties::NoFill)
        }
        a::ShapePropertiesChoice2::SolidFill(fill) => {
          Some(DrawingMlFillProperties::Solid(fill.as_ref()))
        }
        a::ShapePropertiesChoice2::GradientFill(fill) => {
          Some(DrawingMlFillProperties::Gradient(fill.as_ref()))
        }
        a::ShapePropertiesChoice2::PatternFill(fill) => {
          Some(DrawingMlFillProperties::Pattern(fill.as_ref()))
        }
        _ => None,
      },
      Self::Wordprocessing(properties) => match properties.shape_properties_choice2.as_ref()? {
        wps::ShapePropertiesChoice2::NoFill(_) => Some(DrawingMlFillProperties::NoFill),
        wps::ShapePropertiesChoice2::SolidFill(fill) => {
          Some(DrawingMlFillProperties::Solid(fill.as_ref()))
        }
        wps::ShapePropertiesChoice2::GradientFill(fill) => {
          Some(DrawingMlFillProperties::Gradient(fill.as_ref()))
        }
        wps::ShapePropertiesChoice2::PatternFill(fill) => {
          Some(DrawingMlFillProperties::Pattern(fill.as_ref()))
        }
        _ => None,
      },
      Self::Picture(properties) => match properties.shape_properties_choice2.as_ref()? {
        pic::ShapePropertiesChoice2::NoFill(_) => Some(DrawingMlFillProperties::NoFill),
        pic::ShapePropertiesChoice2::SolidFill(fill) => {
          Some(DrawingMlFillProperties::Solid(fill.as_ref()))
        }
        pic::ShapePropertiesChoice2::GradientFill(fill) => {
          Some(DrawingMlFillProperties::Gradient(fill.as_ref()))
        }
        pic::ShapePropertiesChoice2::PatternFill(fill) => {
          Some(DrawingMlFillProperties::Pattern(fill.as_ref()))
        }
        _ => None,
      },
    }
  }

  fn effects(
    &self,
    theme_colors: &ThemeColors,
    images: Option<&ImageCatalog>,
  ) -> Option<common::DrawingEffectSource> {
    let resolve_dag = |source: Box<a::EffectDag>| {
      let resolved = common::drawingml_image_effects::from_effect_dag(
        &source,
        None,
        &DocxImageEffectColorResolver {
          theme_colors,
          images,
          placeholder_color: None,
          word_group_glow: false,
        },
      );
      common::DrawingEffectSource::Dag {
        source,
        resolved: Some(resolved),
      }
    };
    let resolve_list = |source: Box<a::EffectList>| {
      let resolved = common::drawingml_image_effects::from_effect_list(
        &source,
        None,
        &DocxImageEffectColorResolver {
          theme_colors,
          images,
          placeholder_color: None,
          word_group_glow: false,
        },
      );
      common::DrawingEffectSource::List {
        source,
        resolved: Some(resolved),
      }
    };
    match self {
      Self::Diagram(properties) => match properties.shape_properties_choice3.as_ref()? {
        dsp::ShapePropertiesChoice3::EffectList(list) => Some(resolve_list(list.clone())),
        dsp::ShapePropertiesChoice3::EffectDag(dag) => Some(resolve_dag(dag.clone())),
      },
      Self::Generic(properties) => match properties.shape_properties_choice3.as_ref()? {
        a::ShapePropertiesChoice3::EffectList(list) => Some(resolve_list(list.clone())),
        a::ShapePropertiesChoice3::EffectDag(dag) => Some(resolve_dag(dag.clone())),
      },
      Self::Wordprocessing(properties) => match properties.shape_properties_choice3.as_ref()? {
        wps::ShapePropertiesChoice3::EffectList(list) => Some(resolve_list(list.clone())),
        wps::ShapePropertiesChoice3::EffectDag(dag) => Some(resolve_dag(dag.clone())),
      },
      Self::Picture(properties) => match properties.shape_properties_choice3.as_ref()? {
        pic::ShapePropertiesChoice3::EffectList(list) => Some(resolve_list(list.clone())),
        pic::ShapePropertiesChoice3::EffectDag(dag) => Some(resolve_dag(dag.clone())),
      },
    }
  }

  fn camera_adjusted_rotation_deg(&self, mapped_rotation_deg: f32) -> f32 {
    let (scene, shape) = match self {
      Self::Diagram(properties) => (
        properties.scene3_d_type.as_deref(),
        properties.shape3_d_type.as_deref(),
      ),
      Self::Generic(properties) => (
        properties.scene3_d_type.as_deref(),
        properties.shape3_d_type.as_deref(),
      ),
      Self::Wordprocessing(properties) => (
        properties.scene3_d_type.as_deref(),
        properties.shape3_d_type.as_deref(),
      ),
      Self::Picture(properties) => (
        properties.scene3_d_type.as_deref(),
        properties.shape3_d_type.as_deref(),
      ),
    };
    let Some(scene) = scene else {
      return mapped_rotation_deg;
    };
    if shape.is_some() {
      // The shared static-3D renderer owns the complete camera transform when
      // a:sp3d is present.
      return mapped_rotation_deg;
    }

    // A scene-only camera still rotates the visible 2-D face. LibreOffice's
    // Scene3DHelper folds a:xfrm/@rot into the camera revolution before
    // setting the shape's RotateAngle. Preserve any enclosing group rotation
    // already present in `mapped_rotation_deg`, while replacing the local
    // shape rotation with that camera-adjusted face angle.
    let local_rotation_deg = self.rotation_deg();
    mapped_rotation_deg
      - common::drawingml_3d::camera_projection(scene, local_rotation_deg).face_rotation_degrees
      - local_rotation_deg
  }

  fn static3d(&self, theme_colors: &ThemeColors) -> Option<common::drawingml_3d::Static3dStyle> {
    let (scene, shape) = match self {
      Self::Diagram(properties) => (
        properties.scene3_d_type.as_ref()?,
        properties.shape3_d_type.as_ref()?,
      ),
      Self::Generic(properties) => (
        properties.scene3_d_type.as_ref()?,
        properties.shape3_d_type.as_ref()?,
      ),
      Self::Wordprocessing(properties) => (
        properties.scene3_d_type.as_ref()?,
        properties.shape3_d_type.as_ref()?,
      ),
      Self::Picture(properties) => (
        properties.scene3_d_type.as_ref()?,
        properties.shape3_d_type.as_ref()?,
      ),
    };
    let resolver = DocxImageEffectColorResolver {
      theme_colors,
      images: None,
      placeholder_color: None,
      word_group_glow: false,
    };
    let extrusion_color = shape
      .extrusion_color
      .as_deref()
      .and_then(|color| color.extrusion_color_choice.as_ref())
      .and_then(Color::from_extrusion_color_choice)
      .and_then(|color| resolver.resolve(Some(color)))
      .map(|color| common::drawingml_3d::Static3dColor {
        color: color.color,
        alpha: color.alpha,
      });
    let contour_color = shape
      .contour_color
      .as_deref()
      .and_then(|color| color.contour_color_choice.as_ref())
      .and_then(Color::from_contour_color_choice)
      .and_then(|color| resolver.resolve(Some(color)))
      .map(|color| common::drawingml_3d::Static3dColor {
        color: color.color,
        alpha: color.alpha,
      });
    Some(common::drawingml_3d::Static3dStyle {
      scene: scene.clone(),
      shape: shape.clone(),
      extrusion_color,
      contour_color,
    })
  }
}

fn anchor_wrap_polygon_shape(
  anchor: &wp::Anchor,
  placement: ImagePlacement,
) -> Option<InlineShape> {
  let extent = &anchor.extent;
  let width_pt = units::emu_to_points(extent.cx);
  let height_pt = units::emu_to_points(extent.cy);
  let geometry = anchor_wrap_polygon_geometry(anchor, width_pt, height_pt)?;

  Some(InlineShape {
    width_pt,
    height_pt,
    effect_left_pt: 0.0,
    effect_top_pt: 0.0,
    effect_right_pt: 0.0,
    effect_bottom_pt: 0.0,
    geometry,
    offset_x_pt: 0.0,
    offset_y_pt: 0.0,
    rotation_deg: 0.0,
    flip_horizontal: false,
    flip_vertical: false,
    fill_color: None,
    fill_pattern: None,
    fill_override: None,
    additional_fill_colors: Vec::new(),
    fill_image: None,
    stroke: None,
    stroke_pattern: None,
    stroke_override: None,
    suppress_zero_relative_background: false,
    allow_outside_page: false,
    inline_anchor_after_line: false,
    placement,
    chart: None,
    text_warp: None,
    text_fill: None,
    effects: None,
    static3d: None,
    text_upright: false,
    text_box_blocks: Vec::new(),
    text_inset_left_pt: 0.0,
    text_inset_top_pt: 0.0,
    text_inset_right_pt: 0.0,
    text_inset_bottom_pt: 0.0,
    text_box_auto_fit: false,
    text_box_resizes_height_to_fit: false,
    text_box_word_wrap: true,
    text_vertical_alignment: TextBoxVerticalAlignment::Top,
  })
}

fn anchor_wrap_polygon_geometry(
  anchor: &wp::Anchor,
  width_pt: f32,
  height_pt: f32,
) -> Option<InlineShapeGeometry> {
  let polygon = match anchor.anchor_choice.as_ref()? {
    wp::AnchorChoice::WrapTight(tight) => tight.wrap_polygon.as_ref(),
    wp::AnchorChoice::WrapThrough(through) => through.wrap_polygon.as_ref(),
    _ => return None,
  };
  let mut points = Vec::with_capacity(polygon.line_to.len() + 2);
  points.push(wrap_polygon_point(
    polygon.start_point.x,
    polygon.start_point.y,
    width_pt,
    height_pt,
  ));
  for point in &polygon.line_to {
    points.push(wrap_polygon_point(point.x, point.y, width_pt, height_pt));
  }
  if points.len() < 3 {
    return None;
  }
  if points.last() != points.first() {
    points.push(points[0]);
  }

  Some(InlineShapeGeometry::Polyline {
    points,
    closed: true,
  })
}

fn wrap_polygon_point(x: i64, y: i64, width_pt: f32, height_pt: f32) -> (f32, f32) {
  const WRAP_POLYGON_COORDINATE_SCALE: f32 = 21_600.0;
  (
    x as f32 / WRAP_POLYGON_COORDINATE_SCALE * width_pt,
    y as f32 / WRAP_POLYGON_COORDINATE_SCALE * height_pt,
  )
}

fn drawingml_group_child_placement(placement: ImagePlacement) -> ImagePlacement {
  match placement {
    ImagePlacement::Floating(mut placement) => {
      placement.relative_width_to = None;
      placement.relative_width_pct = None;
      placement.relative_height_to = None;
      placement.relative_height_pct = None;
      ImagePlacement::Floating(placement)
    }
    ImagePlacement::Inline => ImagePlacement::Inline,
  }
}

fn drawing_is_hidden(drawing: &w::Drawing) -> bool {
  match drawing.drawing_choice.as_ref() {
    Some(w::DrawingChoice::Inline(inline)) => inline
      .doc_properties
      .hidden
      .as_ref()
      .is_some_and(|hidden| hidden.as_bool()),
    Some(w::DrawingChoice::Anchor(anchor)) => {
      anchor
        .hidden
        .as_ref()
        .is_some_and(|hidden| hidden.as_bool())
        || anchor
          .doc_properties
          .as_ref()
          .and_then(|properties| properties.hidden.as_ref())
          .is_some_and(|hidden| hidden.as_bool())
    }
    None => false,
  }
}

fn drawingml_text_body_texts(paragraphs: &[a::Paragraph]) -> Vec<String> {
  paragraphs
    .iter()
    .filter_map(drawingml_paragraph_text)
    .collect()
}

fn drawingml_paragraph_text(paragraph: &a::Paragraph) -> Option<String> {
  let mut text = String::new();
  for choice in &paragraph.paragraph_choice {
    match choice {
      a::ParagraphChoice::Run(run) => text.push_str(run.text.as_str()),
      a::ParagraphChoice::Field(field) => {
        if let Some(text_node) = &field.text {
          text.push_str(text_node.as_str());
        }
      }
      _ => {}
    }
  }
  (!text.is_empty()).then_some(text)
}

fn drawingml_group_transform_from_properties(
  properties: &wpg::GroupShapeProperties,
  raw_coordinates: bool,
) -> Option<DrawingMlGroupXfrm> {
  properties
    .transform_group
    .as_deref()
    .map(|transform| drawingml_group_transform_from_model(transform, raw_coordinates))
}

fn drawingml_group_transform_from_diagram_properties(
  properties: &dsp::GroupShapeProperties,
  raw_coordinates: bool,
) -> Option<DrawingMlGroupXfrm> {
  properties
    .transform_group
    .as_deref()
    .map(|transform| drawingml_group_transform_from_model(transform, raw_coordinates))
}

fn drawingml_group_transform_from_model(
  transform: &a::TransformGroup,
  raw_coordinates: bool,
) -> DrawingMlGroupXfrm {
  let mut group = DrawingMlGroupXfrm {
    rotation_deg: transform
      .rotation
      .map(|value| sdk_units::drawingml_angle_to_degrees(value) as f32)
      .unwrap_or_default(),
    flip_horizontal: transform
      .horizontal_flip
      .as_ref()
      .is_some_and(|value| value.as_bool()),
    flip_vertical: transform
      .vertical_flip
      .as_ref()
      .is_some_and(|value| value.as_bool()),
    ..DrawingMlGroupXfrm::default()
  };
  if let Some(offset) = &transform.offset {
    group.offset_x_pt = drawingml_coordinate_to_points(offset.x.to_emu(), raw_coordinates);
    group.offset_y_pt = drawingml_coordinate_to_points(offset.y.to_emu(), raw_coordinates);
  }
  if let Some(extents) = &transform.extents {
    group.width_pt = drawingml_coordinate_to_points(extents.cx.to_emu(), raw_coordinates);
    group.height_pt = drawingml_coordinate_to_points(extents.cy.to_emu(), raw_coordinates);
  }
  if let Some(child_offset) = &transform.child_offset {
    group.child_offset_x = child_offset.x.to_emu() as f32;
    group.child_offset_y = child_offset.y.to_emu() as f32;
  }
  if let Some(child_extents) = &transform.child_extents {
    group.child_width = child_extents.cx.to_emu() as f32;
    group.child_height = child_extents.cy.to_emu() as f32;
  }

  group
}

fn drawingml_path_geometry_from_properties(
  properties: &DrawingMlShapeProperties,
  width_pt: f32,
  height_pt: f32,
) -> Option<InlineShapeGeometry> {
  if let Some(geometry) = properties.custom_geometry() {
    if matches!(
      geometry.path_list.path.as_slice(),
      [path]
        if matches!(
          path.path_choice.as_slice(),
          [a::PathChoice::MoveTo(_), a::PathChoice::LineTo(_)]
        )
    ) {
      return Some(InlineShapeGeometry::Line);
    }
    return Some(InlineShapeGeometry::Path {
      paths: common::drawingml_custom_geometry::paths(geometry, 0.0, 0.0, width_pt, height_pt)?,
      outline: properties
        .outline()
        .map(|outline| Box::new(outline.clone())),
    });
  }
  let preset = properties.preset_geometry()?;
  let paths = common::drawingml_preset_geometry::paths(Some(preset), 0.0, 0.0, width_pt, height_pt)
    .or_else(|| {
      // A straight connector may legally have a zero-width or zero-height
      // extent. The shared preset evaluator needs a two-dimensional viewport,
      // but this preset is exactly the segment from (l,t) to (r,b), so retain
      // that degenerate path without changing non-degenerate preset lowering.
      (preset.preset == a::ShapeTypeValues::StraightConnector1
        && ((width_pt <= 0.0 && height_pt > 0.0) || (height_pt <= 0.0 && width_pt > 0.0)))
        .then(|| {
          vec![common::DrawingPath {
            commands: vec![
              common::PathCommand::MoveTo(common::Point {
                x: common::Pt(0.0),
                y: common::Pt(0.0),
              }),
              common::PathCommand::LineTo(common::Point {
                x: common::Pt(width_pt),
                y: common::Pt(height_pt),
              }),
            ],
            fill_mode: common::DrawingPathFillMode::None,
            stroke: true,
            extrusion_allowed: true,
          }]
        })
    })?;
  Some(InlineShapeGeometry::Path {
    paths,
    outline: properties
      .outline()
      .map(|outline| Box::new(outline.clone())),
  })
}

fn drawingml_geometry_from_shape_properties(
  properties: Option<&DrawingMlShapeProperties>,
  geometry: &InlineShapeGeometry,
  raw_coordinates: bool,
  fallback_size: Option<(f32, f32)>,
) -> Option<(f32, f32, f32, f32)> {
  let mut offset_x_pt = 0.0f32;
  let mut offset_y_pt = 0.0f32;
  let mut width_pt = 0.0f32;
  let mut height_pt = 0.0f32;
  let mut saw_ext = false;

  if let Some(transform) = properties.and_then(DrawingMlShapeProperties::transform2_d) {
    if let Some(offset) = &transform.offset {
      offset_x_pt = drawingml_coordinate_to_points(offset.x.to_emu(), raw_coordinates);
      offset_y_pt = drawingml_coordinate_to_points(offset.y.to_emu(), raw_coordinates);
    }
    if let Some(extents) = &transform.extents {
      saw_ext = true;
      width_pt = drawingml_coordinate_to_points(extents.cx.to_emu(), raw_coordinates);
      height_pt = drawingml_coordinate_to_points(extents.cy.to_emu(), raw_coordinates);
    }
  }

  if !saw_ext && let Some((fallback_width_pt, fallback_height_pt)) = fallback_size {
    width_pt = fallback_width_pt;
    height_pt = fallback_height_pt;
  }

  match geometry {
    InlineShapeGeometry::Rectangle if width_pt <= 0.0 || height_pt <= 0.0 => return None,
    InlineShapeGeometry::Line if width_pt <= 0.0 && height_pt <= 0.0 => return None,
    InlineShapeGeometry::Rectangle
    | InlineShapeGeometry::Line
    | InlineShapeGeometry::Path { .. }
    | InlineShapeGeometry::Polyline { .. } => {}
  }

  Some((offset_x_pt, offset_y_pt, width_pt, height_pt))
}

fn drawingml_coordinate_to_points(value: i64, raw_coordinates: bool) -> f32 {
  if raw_coordinates {
    value as f32
  } else {
    units::emu_to_points(value)
  }
}

fn drawingml_picture_frame(
  picture: &pic::Picture,
  placement: ImagePlacement,
  transform: DrawingMlGroupTransform,
  theme_colors: &ThemeColors,
) -> Option<InlineShape> {
  let properties = DrawingMlShapeProperties::Picture(
    picture
      .shape_properties
      .as_deref()
      .cloned()
      .unwrap_or_default(),
  );
  let fill_override = drawingml_shape_properties_common_fill(&properties, theme_colors);
  let fill_color = drawingml_shape_properties_fill_color(&properties, theme_colors);
  let fill_pattern = drawingml_shape_properties_pattern_fill(&properties, theme_colors);
  let stroke_override = properties
    .outline()
    .and_then(|outline| drawingml_outline_common_stroke(outline, theme_colors));
  let stroke = stroke_override.as_ref().map(|stroke| BorderStyle {
    width_pt: stroke.width.0,
    spacing_pt: 0.0,
    color: RgbColor {
      r: stroke.color.r,
      g: stroke.color.g,
      b: stroke.color.b,
    },
    compound: false,
    dash_pattern: BorderDashPattern::Solid,
    shadow: false,
  });
  let mut geometry = properties
    .geometry_kind()
    .unwrap_or(InlineShapeGeometry::Rectangle);
  let has_path_geometry = properties.has_path_geometry();
  if geometry == InlineShapeGeometry::Rectangle && has_path_geometry {
    geometry = InlineShapeGeometry::Polyline {
      points: Vec::new(),
      closed: false,
    };
  }
  let (offset_x_pt, offset_y_pt, width_pt, height_pt) = drawingml_geometry_from_shape_properties(
    Some(&properties),
    &geometry,
    transform.raw_coordinates,
    transform.fallback_size,
  )?;
  let mapped = transform.map_rect(
    (offset_x_pt, offset_y_pt, width_pt, height_pt),
    (
      properties.rotation_deg(),
      properties.flip_horizontal(),
      properties.flip_vertical(),
    ),
  );
  let (offset_x_pt, offset_y_pt, width_pt, height_pt) =
    (mapped.x_pt, mapped.y_pt, mapped.width_pt, mapped.height_pt);
  if has_path_geometry
    && let Some(path_geometry) =
      drawingml_path_geometry_from_properties(&properties, width_pt, height_pt)
  {
    geometry = path_geometry;
  }

  Some(InlineShape {
    width_pt,
    height_pt,
    effect_left_pt: 0.0,
    effect_top_pt: 0.0,
    effect_right_pt: 0.0,
    effect_bottom_pt: 0.0,
    geometry,
    offset_x_pt,
    offset_y_pt,
    rotation_deg: properties.camera_adjusted_rotation_deg(mapped.rotation_deg),
    flip_horizontal: mapped.flip_horizontal,
    flip_vertical: mapped.flip_vertical,
    fill_color,
    fill_pattern,
    fill_override: fill_override.map(Box::new),
    additional_fill_colors: Vec::new(),
    fill_image: None,
    stroke,
    stroke_pattern: None,
    stroke_override: stroke_override.map(Box::new),
    suppress_zero_relative_background: false,
    allow_outside_page: false,
    inline_anchor_after_line: false,
    placement,
    chart: None,
    text_warp: None,
    text_fill: None,
    effects: None,
    static3d: None,
    text_upright: false,
    text_box_blocks: Vec::new(),
    text_inset_left_pt: 0.0,
    text_inset_top_pt: 0.0,
    text_inset_right_pt: 0.0,
    text_inset_bottom_pt: 0.0,
    text_box_auto_fit: false,
    text_box_resizes_height_to_fit: false,
    text_box_word_wrap: true,
    text_vertical_alignment: TextBoxVerticalAlignment::Top,
  })
}

fn drawingml_picture_image(
  picture: &pic::Picture,
  placement: ImagePlacement,
  transform: DrawingMlGroupTransform,
  styles: &StylesCatalog,
  images: &ImageCatalog,
  hyperlinks: &HyperlinkCatalog,
) -> Option<InlineImage> {
  let properties = drawing_picture_image_properties(picture, &styles.theme_colors, Some(images))?;
  let relationship_id = properties.relationship_id.as_deref()?;
  let resource = images.by_relationship_id.get(relationship_id)?;
  let image_data = image_data_with_effects(resource, &properties);
  let (image_data, crop) = materialize_source_rectangle_crop(
    image_data,
    properties.crop,
    properties.source_rectangle_crop,
  );
  let shape_properties = DrawingMlShapeProperties::Picture(
    picture
      .shape_properties
      .as_deref()
      .cloned()
      .unwrap_or_default(),
  );
  let geometry = shape_properties
    .geometry_kind()
    .unwrap_or(InlineShapeGeometry::Rectangle);
  let (offset_x_pt, offset_y_pt, width_pt, height_pt) = drawingml_geometry_from_shape_properties(
    Some(&shape_properties),
    &geometry,
    transform.raw_coordinates,
    None,
  )?;
  let mapped = transform.map_rect(
    (offset_x_pt, offset_y_pt, width_pt, height_pt),
    (
      properties.rotation_deg,
      properties.flip_horizontal,
      properties.flip_vertical,
    ),
  );
  let (offset_x_pt, offset_y_pt, width_pt, height_pt) =
    (mapped.x_pt, mapped.y_pt, mapped.width_pt, mapped.height_pt);
  let hyperlink_url = properties
    .hyperlink_relationship_id
    .as_deref()
    .and_then(|relationship_id| hyperlinks.target(relationship_id))
    .map(ToString::to_string);
  Some(InlineImage {
    data: image_data.data,
    content_type: image_data.content_type,
    picture_frame: drawingml_picture_frame(picture, placement, transform, &styles.theme_colors)
      .map(Box::new),
    effects: properties.shape_effects,
    static3d: properties.static3d,
    width_pt,
    height_pt,
    effect_left_pt: 0.0,
    effect_top_pt: 0.0,
    effect_right_pt: 0.0,
    effect_bottom_pt: 0.0,
    crop,
    rotation_deg: shape_properties.camera_adjusted_rotation_deg(mapped.rotation_deg),
    flip_horizontal: mapped.flip_horizontal,
    flip_vertical: mapped.flip_vertical,
    metafile_background_color: None,
    alt_text: drawingml_picture_alt_text(picture),
    hyperlink_url,
    semantic_metafile_text: false,
    metafile_native_size: true,
    picture_content_control: false,
    placement: drawingml_child_placement(placement, offset_x_pt, offset_y_pt),
  })
}

fn wordprocessing_shape_image_fill(
  properties: &wps::ShapeProperties,
  images: &ImageCatalog,
) -> Option<InlineShapeImageFill> {
  let wps::ShapePropertiesChoice2::BlipFill(blip_fill) =
    properties.shape_properties_choice2.as_ref()?
  else {
    return None;
  };
  let image_properties =
    drawing_blip_fill_image_properties(blip_fill, &ThemeColors::default(), Some(images))?;
  let relationship_id = image_properties.relationship_id.as_deref()?;
  let resource = images.by_relationship_id.get(relationship_id)?;
  let image_data = image_data_with_effects(resource, &image_properties);

  Some(InlineShapeImageFill {
    data: image_data.data,
    content_type: image_data.content_type,
    crop: image_properties.crop,
    rotation_deg: image_properties.rotation_deg,
    flip_horizontal: image_properties.flip_horizontal,
    flip_vertical: image_properties.flip_vertical,
    rotate_with_shape: blip_fill
      .rotate_with_shape
      .as_ref()
      .is_some_and(|value| value.as_bool()),
    mode: drawingml_image_fill_mode(blip_fill),
  })
}

fn drawingml_diagram_shape_image_fill(
  properties: &dsp::ShapeProperties,
  images: &ImageCatalog,
) -> Option<InlineShapeImageFill> {
  let dsp::ShapePropertiesChoice2::BlipFill(blip_fill) =
    properties.shape_properties_choice2.as_ref()?
  else {
    return None;
  };
  let image_properties =
    drawing_blip_fill_image_properties(blip_fill, &ThemeColors::default(), Some(images))?;
  let relationship_id = image_properties.relationship_id.as_deref()?;
  let resource = images.by_relationship_id.get(relationship_id)?;
  let image_data = image_data_with_effects(resource, &image_properties);

  Some(InlineShapeImageFill {
    data: image_data.data,
    content_type: image_data.content_type,
    crop: image_properties.crop,
    rotation_deg: image_properties.rotation_deg,
    flip_horizontal: image_properties.flip_horizontal,
    flip_vertical: image_properties.flip_vertical,
    rotate_with_shape: blip_fill
      .rotate_with_shape
      .as_ref()
      .is_some_and(|value| value.as_bool()),
    mode: drawingml_image_fill_mode(blip_fill),
  })
}

fn drawingml_generic_shape_image_fill(
  properties: &a::ShapeProperties,
  images: &ImageCatalog,
) -> Option<InlineShapeImageFill> {
  let a::ShapePropertiesChoice2::BlipFill(blip_fill) =
    properties.shape_properties_choice2.as_ref()?
  else {
    return None;
  };
  let image_properties =
    drawing_blip_fill_image_properties(blip_fill, &ThemeColors::default(), Some(images))?;
  let relationship_id = image_properties.relationship_id.as_deref()?;
  let resource = images.by_relationship_id.get(relationship_id)?;
  let image_data = image_data_with_effects(resource, &image_properties);

  Some(InlineShapeImageFill {
    data: image_data.data,
    content_type: image_data.content_type,
    crop: image_properties.crop,
    rotation_deg: image_properties.rotation_deg,
    flip_horizontal: image_properties.flip_horizontal,
    flip_vertical: image_properties.flip_vertical,
    rotate_with_shape: blip_fill
      .rotate_with_shape
      .as_ref()
      .is_some_and(|value| value.as_bool()),
    mode: drawingml_image_fill_mode(blip_fill),
  })
}

fn drawingml_image_fill_mode(fill: &a::BlipFill) -> InlineShapeImageFillMode {
  match fill.blip_fill_choice.as_ref() {
    Some(a::BlipFillChoice::Stretch(_)) => InlineShapeImageFillMode::Stretch,
    Some(a::BlipFillChoice::Tile(tile)) => InlineShapeImageFillMode::DrawingMlTile(tile.clone()),
    // Office treats an omitted bitmap mode on a shape fill as tiled.
    None => InlineShapeImageFillMode::DrawingMlTile(Box::default()),
  }
}

fn resolve_drawingml_solid_fill(
  fill: &a::SolidFill,
  theme_colors: &ThemeColors,
) -> Option<ResolvedColor> {
  resolved_docx_drawing_color(
    Color::from_solid_fill_choice(fill.solid_fill_choice.as_ref()?)?,
    theme_colors,
  )
}

fn drawingml_pattern_fill(
  fill: &a::PatternFill,
  theme_colors: &ThemeColors,
) -> Option<common::PatternFill> {
  let foreground = match fill
    .foreground_color
    .as_ref()
    .and_then(|color| color.foreground_color_choice.as_ref())
  {
    Some(choice) => resolve_drawingml_foreground_color(choice, theme_colors)?,
    None => ResolvedColor {
      color: RgbColor { r: 0, g: 0, b: 0 },
      opacity: 1.0,
    },
  };
  let background = match fill
    .background_color
    .as_ref()
    .and_then(|color| color.background_color_choice.as_ref())
  {
    Some(choice) => resolve_drawingml_background_color(choice, theme_colors)?,
    None => ResolvedColor {
      color: RgbColor {
        r: u8::MAX,
        g: u8::MAX,
        b: u8::MAX,
      },
      opacity: 1.0,
    },
  };
  Some(common::PatternFill {
    hatch_style: common::drawingml_pattern::hatch_style(fill.preset),
    foreground: common_rgb(foreground.color, foreground.opacity),
    background: common_rgb(background.color, background.opacity),
  })
}

fn drawingml_gradient_fill(
  fill: &a::GradientFill,
  theme_colors: &ThemeColors,
) -> Option<common::Fill<'static>> {
  let mut stops = fill
    .gradient_stop_list
    .as_ref()?
    .gradient_stop
    .iter()
    .filter_map(|stop| {
      let color = resolved_docx_drawing_color(
        Color::from_gradient_stop_choice(stop.gradient_stop_choice.as_ref()?)?,
        theme_colors,
      )?;
      Some(common::GradientStop {
        position: stop.position.as_ratio() as f32,
        color: common_rgb(color.color, color.opacity),
        scheme: None,
      })
    })
    .collect::<Vec<_>>();
  stops.sort_by(|left, right| left.position.total_cmp(&right.position));
  if stops.is_empty() {
    return None;
  }
  let (angle_degrees, scaled, path) = match fill.gradient_fill_choice.as_ref()? {
    a::GradientFillChoice::LinearGradientFill(linear) => (
      Some(linear.angle.unwrap_or_default() as f32 / 60_000.0),
      linear.scaled.as_ref().is_some_and(|value| value.as_bool()),
      None,
    ),
    a::GradientFillChoice::PathGradientFill(path) => {
      let fill_to = path
        .fill_to_rectangle
        .as_ref()
        .map(|rect| common::RelativeRect {
          left: rect
            .left
            .as_ref()
            .map_or(0.5, |value| value.as_ratio() as f32),
          top: rect
            .top
            .as_ref()
            .map_or(0.5, |value| value.as_ratio() as f32),
          right: rect
            .right
            .as_ref()
            .map_or(0.5, |value| value.as_ratio() as f32),
          bottom: rect
            .bottom
            .as_ref()
            .map_or(0.5, |value| value.as_ratio() as f32),
        })
        .unwrap_or(common::RelativeRect {
          left: 0.5,
          top: 0.5,
          right: 0.5,
          bottom: 0.5,
        });
      let kind = match path.path.unwrap_or(a::PathShadeValues::Shape) {
        a::PathShadeValues::Shape => common::GradientPathKind::Shape,
        a::PathShadeValues::Circle => common::GradientPathKind::Circle,
        a::PathShadeValues::Rectangle => common::GradientPathKind::Rectangle,
      };
      (
        None,
        false,
        Some(common::GradientPath {
          kind,
          fill_to,
          transform: common::Transform::default(),
          mirror_tile: false,
        }),
      )
    }
  };
  Some(common::Fill::Gradient(common::GradientFill {
    stops,
    angle_degrees,
    definition_bounds: None,
    line: None,
    interpolation: common::GradientInterpolation::LinearSrgb,
    scaled,
    rotate_with_shape: Some(
      fill
        .rotate_with_shape
        .as_ref()
        .is_none_or(|value| value.as_bool()),
    ),
    path,
  }))
}

fn drawingml_shape_properties_common_fill(
  properties: &DrawingMlShapeProperties,
  theme_colors: &ThemeColors,
) -> Option<common::Fill<'static>> {
  match properties.fill()? {
    DrawingMlFillProperties::NoFill => Some(common::Fill::None),
    DrawingMlFillProperties::Solid(fill) => {
      let color = resolve_drawingml_solid_fill(fill, theme_colors)?;
      Some(common::Fill::Solid(common_rgb(color.color, color.opacity)))
    }
    DrawingMlFillProperties::Gradient(fill) => drawingml_gradient_fill(fill, theme_colors),
    DrawingMlFillProperties::Pattern(fill) => {
      drawingml_pattern_fill(fill, theme_colors).map(common::Fill::Pattern)
    }
  }
}

fn resolve_drawingml_foreground_color(
  choice: &a::ForegroundColorChoice,
  theme_colors: &ThemeColors,
) -> Option<ResolvedColor> {
  resolved_docx_drawing_color(Color::from_foreground_color_choice(choice)?, theme_colors)
}

fn resolve_drawingml_background_color(
  choice: &a::BackgroundColorChoice,
  theme_colors: &ThemeColors,
) -> Option<ResolvedColor> {
  resolved_docx_drawing_color(Color::from_background_color_choice(choice)?, theme_colors)
}

fn drawingml_first_gradient_fill_color(
  fill: &a::GradientFill,
  theme_colors: &ThemeColors,
) -> Option<RgbColor> {
  let stop = fill.gradient_stop_list.as_ref()?.gradient_stop.first()?;
  resolved_docx_drawing_color(
    Color::from_gradient_stop_choice(stop.gradient_stop_choice.as_ref()?)?,
    theme_colors,
  )
  .map(|color| color.color)
}

fn resolved_docx_drawing_color(color: Color, theme_colors: &ThemeColors) -> Option<ResolvedColor> {
  let color = docx_image_color(color, theme_colors)?;
  Some(ResolvedColor {
    color: RgbColor {
      r: color.r,
      g: color.g,
      b: color.b,
    },
    opacity: f32::from(color.a) / f32::from(u8::MAX),
  })
}

struct ImportedImageData {
  data: Arc<[u8]>,
  content_type: Option<String>,
}

fn drawing_image_data(
  images: &ImageCatalog,
  properties: &DrawingImageProperties,
) -> Option<ImportedImageData> {
  let relationship_id = properties.relationship_id.as_deref()?;
  if let Some(resource) = images.by_relationship_id.get(relationship_id) {
    return Some(image_data_with_effects(resource, properties));
  }
  properties.external_link.then(|| ImportedImageData {
    data: Arc::from([]),
    content_type: None,
  })
}

fn image_data_with_effects(
  resource: &package::ImageResource,
  properties: &DrawingImageProperties,
) -> ImportedImageData {
  if properties.effects.is_empty() {
    return ImportedImageData {
      data: resource.data.clone(),
      content_type: resource.content_type.clone(),
    };
  }

  let mut effects = properties.effects.clone();
  let color_change_tolerance =
    common::drawingml_image_effects::color_change_tolerance(resource.content_type.as_deref());
  common::drawingml_image_effects::set_color_change_tolerance(&mut effects, color_change_tolerance);
  let Some(data) = common::drawingml_image_effects::apply(
    &resource.data,
    resource.content_type.as_deref(),
    &effects,
  ) else {
    return ImportedImageData {
      data: resource.data.clone(),
      content_type: resource.content_type.clone(),
    };
  };

  ImportedImageData {
    data: data.into(),
    content_type: Some("image/png".into()),
  }
}

fn materialize_source_rectangle_crop(
  image_data: ImportedImageData,
  crop: ImageCrop,
  source_rectangle_crop: bool,
) -> (ImportedImageData, ImageCrop) {
  if !source_rectangle_crop {
    return (image_data, crop);
  }

  // ECMA-376 Part 1 §20.1.8.55 defines srcRect over the source bitmap.
  // LibreOffice's CropQuotientsFromSrcRect/lclCropGraphic path clamps only
  // positive edges, rounds them against source pixels, and physically crops
  // the bitmap. Negative edges remain outsets in the destination transform.
  let residual_crop = ImageCrop {
    left: crop.left.min(0.0),
    top: crop.top.min(0.0),
    right: crop.right.min(0.0),
    bottom: crop.bottom.min(0.0),
  };
  let Ok(source) = image::load_from_memory(&image_data.data) else {
    return (image_data, crop);
  };
  let width = source.width();
  let height = source.height();
  let crop_pixels = |length: u32, ratio: f32| {
    ((f64::from(length) * f64::from(ratio.max(0.0))).round()).clamp(0.0, f64::from(length)) as u32
  };
  let left = crop_pixels(width, crop.left);
  let top = crop_pixels(height, crop.top);
  let right = crop_pixels(width, crop.right);
  let bottom = crop_pixels(height, crop.bottom);
  let Some(cropped_width) = width.checked_sub(left.saturating_add(right)) else {
    return (image_data, crop);
  };
  let Some(cropped_height) = height.checked_sub(top.saturating_add(bottom)) else {
    return (image_data, crop);
  };
  if cropped_width == 0 || cropped_height == 0 {
    return (image_data, crop);
  }
  if left == 0 && top == 0 && right == 0 && bottom == 0 {
    return (image_data, residual_crop);
  }

  let cropped = source.crop_imm(left, top, cropped_width, cropped_height);
  let mut png = Vec::new();
  if PngEncoder::new(&mut png)
    .write_image(
      cropped.as_bytes(),
      cropped_width,
      cropped_height,
      cropped.color().into(),
    )
    .is_err()
  {
    return (image_data, crop);
  }

  (
    ImportedImageData {
      data: png.into(),
      content_type: Some("image/png".into()),
    },
    residual_crop,
  )
}

fn drawingml_picture_alt_text(picture: &pic::Picture) -> Option<String> {
  let properties = &picture
    .non_visual_picture_properties
    .as_deref()?
    .non_visual_drawing_properties;
  properties
    .description
    .clone()
    .or_else(|| Some(properties.name.clone()))
}

fn drawingml_child_placement(
  placement: ImagePlacement,
  offset_x_pt: f32,
  offset_y_pt: f32,
) -> ImagePlacement {
  match placement {
    ImagePlacement::Inline => ImagePlacement::Inline,
    ImagePlacement::Floating(mut floating) => {
      floating.horizontal_alignment = None;
      floating.vertical_alignment = None;
      floating.horizontal_offset_pt += offset_x_pt;
      floating.vertical_offset_pt += offset_y_pt;
      ImagePlacement::Floating(floating)
    }
  }
}

fn wordprocessing_shape_fill_color(
  shape: &wps::WordprocessingShape,
  theme_colors: &ThemeColors,
) -> Option<RgbColor> {
  let properties = DrawingMlShapeProperties::Wordprocessing(
    shape
      .shape_properties
      .as_deref()
      .cloned()
      .unwrap_or_default(),
  );
  drawingml_shape_properties_fill_color(&properties, theme_colors)
}

fn drawingml_shape_properties_fill_color(
  properties: &DrawingMlShapeProperties,
  theme_colors: &ThemeColors,
) -> Option<RgbColor> {
  match properties.fill()? {
    DrawingMlFillProperties::NoFill => None,
    DrawingMlFillProperties::Solid(fill) => {
      resolve_drawingml_solid_fill(fill, theme_colors).map(|color| color.color)
    }
    DrawingMlFillProperties::Gradient(fill) => {
      drawingml_first_gradient_fill_color(fill, theme_colors)
    }
    DrawingMlFillProperties::Pattern(_) => None,
  }
}

fn drawingml_shape_properties_pattern_fill(
  properties: &DrawingMlShapeProperties,
  theme_colors: &ThemeColors,
) -> Option<common::PatternFill> {
  let DrawingMlFillProperties::Pattern(fill) = properties.fill()? else {
    return None;
  };
  drawingml_pattern_fill(fill, theme_colors)
}

fn drawingml_shape_properties_has_no_fill(properties: &DrawingMlShapeProperties) -> bool {
  properties
    .fill()
    .is_some_and(|fill| matches!(fill, DrawingMlFillProperties::NoFill))
}

fn drawingml_line_reference_stroke(
  reference: &a::LineReference,
  theme_colors: &ThemeColors,
  theme_lines: &ThemeLineStyles,
) -> Option<BorderStyle> {
  let index = usize::try_from(reference.index).ok()?;
  let width_pt = theme_lines.width_pt(index)?;
  let color = drawingml_line_reference_color(reference, theme_colors)?;
  Some(BorderStyle {
    width_pt,
    spacing_pt: 0.0,
    color,
    compound: false,
    dash_pattern: BorderDashPattern::Solid,
    shadow: false,
  })
}

fn drawingml_effect_reference_effects(
  reference: &a::EffectReference,
  theme_effects: &ThemeEffectStyles,
  theme_colors: &ThemeColors,
  images: Option<&ImageCatalog>,
) -> Option<common::DrawingEffectSource> {
  let index = usize::try_from(reference.index).ok()?;
  let style = theme_effects.get(index)?;
  let resolver = DocxImageEffectColorResolver {
    theme_colors,
    images,
    placeholder_color: reference
      .effect_reference_choice
      .as_ref()
      .and_then(Color::from_effect_reference_choice),
    word_group_glow: false,
  };
  match style.effect_style_choice.as_ref()? {
    a::EffectStyleChoice::EffectList(source) => {
      let source = source.clone();
      let resolved = common::drawingml_image_effects::from_effect_list(&source, None, &resolver);
      Some(common::DrawingEffectSource::List {
        source,
        resolved: Some(resolved),
      })
    }
    a::EffectStyleChoice::EffectDag(source) => {
      let source = source.clone();
      let resolved = common::drawingml_image_effects::from_effect_dag(&source, None, &resolver);
      Some(common::DrawingEffectSource::Dag {
        source,
        resolved: Some(resolved),
      })
    }
  }
}

fn drawingml_fill_reference_color(
  reference: &a::FillReference,
  theme_colors: &ThemeColors,
) -> Option<RgbColor> {
  match reference.fill_reference_choice.as_ref()? {
    a::FillReferenceChoice::RgbColorModelHex(color) => parse_hex_color(color.val.as_str()),
    a::FillReferenceChoice::SystemColor(color) => {
      color.last_color.as_deref().and_then(parse_hex_color)
    }
    a::FillReferenceChoice::SchemeColor(color) => {
      resolve_drawingml_scheme_color(color, theme_colors)
    }
    a::FillReferenceChoice::PresetColor(color) => drawingml_preset_color_value(color.val),
    _ => None,
  }
}

fn drawingml_font_reference_color(
  reference: &a::FontReference,
  theme_colors: &ThemeColors,
) -> Option<RgbColor> {
  match reference.font_reference_choice.as_ref()? {
    a::FontReferenceChoice::RgbColorModelHex(color) => parse_hex_color(color.val.as_str()),
    a::FontReferenceChoice::SystemColor(color) => {
      color.last_color.as_deref().and_then(parse_hex_color)
    }
    a::FontReferenceChoice::SchemeColor(color) => {
      resolve_drawingml_scheme_color(color, theme_colors)
    }
    a::FontReferenceChoice::PresetColor(color) => drawingml_preset_color_value(color.val),
    _ => None,
  }
}

fn drawingml_line_reference_color(
  reference: &a::LineReference,
  theme_colors: &ThemeColors,
) -> Option<RgbColor> {
  match reference.line_reference_choice.as_ref()? {
    a::LineReferenceChoice::RgbColorModelHex(color) => parse_hex_color(color.val.as_str()),
    a::LineReferenceChoice::SystemColor(color) => {
      color.last_color.as_deref().and_then(parse_hex_color)
    }
    a::LineReferenceChoice::SchemeColor(color) => {
      resolve_drawingml_scheme_color(color, theme_colors)
    }
    a::LineReferenceChoice::PresetColor(color) => drawingml_preset_color_value(color.val),
    _ => None,
  }
}

fn wordprocessing_shape_stroke(
  shape: &wps::WordprocessingShape,
  theme_colors: &ThemeColors,
) -> Option<BorderStyle> {
  let line = shape.shape_properties.as_ref()?.outline.as_ref()?;
  let color = match line.outline_choice1.as_ref()? {
    a::OutlineChoice::NoFill(_) => return None,
    a::OutlineChoice::SolidFill(fill) => {
      resolve_drawingml_solid_fill(fill.as_ref(), theme_colors)?.color
    }
    a::OutlineChoice::GradientFill(fill) => {
      drawingml_first_gradient_fill_color(fill.as_ref(), theme_colors)?
    }
    a::OutlineChoice::PatternFill(fill) => {
      let pattern = drawingml_pattern_fill(fill, theme_colors)?;
      RgbColor {
        r: pattern.foreground.r,
        g: pattern.foreground.g,
        b: pattern.foreground.b,
      }
    }
  };
  let width_pt = line
    .width
    .map(i64::from)
    .map(units::emu_to_points)
    .unwrap_or_else(|| units::emu_to_points(DRAWINGML_DEFAULT_LINE_WIDTH_EMU));

  Some(BorderStyle {
    width_pt,
    spacing_pt: 0.0,
    color,
    compound: false,
    dash_pattern: BorderDashPattern::Solid,
    shadow: false,
  })
}

fn drawingml_outline_common_stroke(
  outline: &a::Outline,
  theme_colors: &ThemeColors,
) -> Option<common::Stroke<'static>> {
  let width_pt = outline
    .width
    .map(i64::from)
    .map(units::emu_to_points)
    .unwrap_or_else(|| units::emu_to_points(DRAWINGML_DEFAULT_LINE_WIDTH_EMU));
  let (color, pattern, gradient) = match outline.outline_choice1.as_ref()? {
    a::OutlineChoice::NoFill(_) => return None,
    a::OutlineChoice::SolidFill(fill) => {
      let color = resolve_drawingml_solid_fill(fill, theme_colors)?;
      (common_rgb(color.color, color.opacity), None, None)
    }
    a::OutlineChoice::PatternFill(fill) => {
      let pattern = drawingml_pattern_fill(fill, theme_colors)?;
      (pattern.foreground, Some(pattern), None)
    }
    a::OutlineChoice::GradientFill(fill) => {
      let common::Fill::Gradient(gradient) = drawingml_gradient_fill(fill, theme_colors)? else {
        return None;
      };
      let color = gradient
        .stops
        .first()
        .map(|stop| stop.color)
        .unwrap_or_default();
      (color, None, Some(gradient))
    }
  };
  let mut stroke = common::Stroke {
    width: common::Pt(width_pt),
    color,
    pattern,
    gradient,
    ..Default::default()
  };
  common::drawingml_stroke::apply_outline_style(&mut stroke, outline);
  Some(stroke)
}

fn drawingml_border_style_from_common_stroke(stroke: &common::Stroke<'_>) -> BorderStyle {
  BorderStyle {
    width_pt: stroke.width.0,
    spacing_pt: 0.0,
    color: RgbColor {
      r: stroke.color.r,
      g: stroke.color.g,
      b: stroke.color.b,
    },
    compound: false,
    dash_pattern: BorderDashPattern::Solid,
    shadow: false,
  }
}

fn drawingml_diagram_shape_stroke(
  shape: &dsp::Shape,
  theme_colors: &ThemeColors,
) -> Option<BorderStyle> {
  let line = shape.shape_properties.outline.as_ref()?;
  let color = match line.outline_choice1.as_ref()? {
    a::OutlineChoice::NoFill(_) => return None,
    a::OutlineChoice::SolidFill(fill) => resolve_drawingml_solid_fill(fill, theme_colors)?.color,
    a::OutlineChoice::GradientFill(fill) => {
      drawingml_first_gradient_fill_color(fill, theme_colors)?
    }
    a::OutlineChoice::PatternFill(fill) => {
      let pattern = drawingml_pattern_fill(fill, theme_colors)?;
      RgbColor {
        r: pattern.foreground.r,
        g: pattern.foreground.g,
        b: pattern.foreground.b,
      }
    }
  };
  let width_pt = line
    .width
    .map(i64::from)
    .map(units::emu_to_points)
    .unwrap_or_else(|| units::emu_to_points(DRAWINGML_DEFAULT_LINE_WIDTH_EMU));

  Some(BorderStyle {
    width_pt,
    spacing_pt: 0.0,
    color,
    compound: false,
    dash_pattern: BorderDashPattern::Solid,
    shadow: false,
  })
}

fn wordprocessing_shape_has_no_line(shape: &wps::WordprocessingShape) -> bool {
  shape
    .shape_properties
    .as_deref()
    .and_then(|properties| properties.outline.as_ref())
    .and_then(|line| line.outline_choice1.as_ref())
    .is_some_and(|choice| matches!(choice, a::OutlineChoice::NoFill(_)))
}

fn drawingml_diagram_shape_has_no_line(shape: &dsp::Shape) -> bool {
  shape
    .shape_properties
    .outline
    .as_ref()
    .and_then(|line| line.outline_choice1.as_ref())
    .is_some_and(|choice| matches!(choice, a::OutlineChoice::NoFill(_)))
}

fn push_pict_shapes_impl(
  picture: &w::Picture,
  inlines: &mut Vec<InlineItem>,
  images: &ImageCatalog,
) {
  let shape_types = picture
    .picture_choice
    .iter()
    .flat_map(vml_picture_choice_shape_types)
    .collect::<Vec<_>>();
  for choice in &picture.picture_choice {
    push_picture_choice_shapes(choice, inlines, images, &shape_types);
  }
}

fn vml_picture_choice_shape_types(choice: &w::PictureChoice) -> Vec<&v::Shapetype> {
  match choice {
    w::PictureChoice::Shapetype(shape_type) => vec![shape_type],
    w::PictureChoice::Group(group) => vml_group_shape_types(group),
    _ => Vec::new(),
  }
}

fn vml_group_shape_types(group: &v::Group) -> Vec<&v::Shapetype> {
  group
    .group_choice
    .iter()
    .flat_map(|choice| match choice {
      v::GroupChoice::Shapetype(shape_type) => vec![shape_type.as_ref()],
      v::GroupChoice::Group(group) => vml_group_shape_types(group),
      _ => Vec::new(),
    })
    .collect()
}

fn push_picture_choice_shapes(
  choice: &w::PictureChoice,
  inlines: &mut Vec<InlineItem>,
  images: &ImageCatalog,
  shape_types: &[&v::Shapetype],
) {
  match choice {
    w::PictureChoice::Group(group) => push_group_shapes(group, inlines, images, shape_types),
    w::PictureChoice::Arc(shape) => {
      if let Some(shape) = vml_special_shape(
        crate::xlsx::object_resources::vml_arc_model(shape),
        shape.style.as_deref(),
      ) {
        inlines.push(InlineItem::Shape(shape));
      }
    }
    w::PictureChoice::Curve(shape) => {
      if let Some(shape) = vml_special_shape(
        crate::xlsx::object_resources::vml_curve_model(shape),
        shape.style.as_deref(),
      ) {
        inlines.push(InlineItem::Shape(shape));
      }
    }
    w::PictureChoice::Line(shape) => {
      if let Some(shape) = vml_special_shape(
        crate::xlsx::object_resources::vml_line_model(shape),
        shape.style.as_deref(),
      ) {
        inlines.push(InlineItem::Shape(shape));
      }
    }
    w::PictureChoice::Oval(shape) => {
      if let Some(shape) = vml_special_shape(
        crate::xlsx::object_resources::vml_oval_model(shape),
        shape.style.as_deref(),
      ) {
        inlines.push(InlineItem::Shape(shape));
      }
    }
    w::PictureChoice::Rectangle(rectangle) => {
      if let Some(shape) = vml_rectangle_shape(rectangle, images) {
        inlines.push(InlineItem::Shape(shape));
      }
    }
    w::PictureChoice::RoundRectangle(round_rectangle) => {
      if let Some(shape) = vml_round_rectangle_shape(round_rectangle) {
        inlines.push(InlineItem::Shape(shape));
      }
    }
    w::PictureChoice::Shape(shape) => {
      if let Some(shape) = vml_shape_shape(shape, images, shape_types) {
        inlines.push(InlineItem::Shape(shape));
      }
    }
    w::PictureChoice::PolyLine(polyline) => {
      if let Some(shape) = vml_polyline_shape(polyline) {
        inlines.push(InlineItem::Shape(shape));
      }
    }
    _ => {}
  }
}

fn push_group_shapes(
  group: &v::Group,
  inlines: &mut Vec<InlineItem>,
  images: &ImageCatalog,
  inherited_shape_types: &[&v::Shapetype],
) {
  push_group_child_shapes(group, inlines, images, inherited_shape_types);
}

fn push_group_child_shapes(
  group: &v::Group,
  inlines: &mut Vec<InlineItem>,
  images: &ImageCatalog,
  inherited_shape_types: &[&v::Shapetype],
) {
  let transform = VmlGroupTransform::from_group(group);
  for choice in &group.group_choice {
    match choice {
      v::GroupChoice::Group(group) => {
        push_group_child_shapes(group, inlines, images, inherited_shape_types)
      }
      v::GroupChoice::Arc(shape) => {
        let style = transform.and_then(|transform| {
          transform.child_anchor_style(group.style.as_deref(), shape.style.as_deref())
        });
        if let Some(shape) = vml_special_shape(
          crate::xlsx::object_resources::vml_arc_model(shape),
          style.as_deref().or(shape.style.as_deref()),
        ) {
          inlines.push(InlineItem::Shape(shape));
        }
      }
      v::GroupChoice::Curve(shape) => {
        let style = transform.and_then(|transform| {
          transform.child_anchor_style(group.style.as_deref(), shape.style.as_deref())
        });
        if let Some(shape) = vml_special_shape(
          crate::xlsx::object_resources::vml_curve_model(shape),
          style.as_deref().or(shape.style.as_deref()),
        ) {
          inlines.push(InlineItem::Shape(shape));
        }
      }
      v::GroupChoice::Line(shape) => {
        let style = transform.and_then(|transform| {
          transform.child_anchor_style(group.style.as_deref(), shape.style.as_deref())
        });
        if let Some(shape) = vml_special_shape(
          crate::xlsx::object_resources::vml_line_model(shape),
          style.as_deref().or(shape.style.as_deref()),
        ) {
          inlines.push(InlineItem::Shape(shape));
        }
      }
      v::GroupChoice::Oval(shape) => {
        let style = transform.and_then(|transform| {
          transform.child_anchor_style(group.style.as_deref(), shape.style.as_deref())
        });
        if let Some(shape) = vml_special_shape(
          crate::xlsx::object_resources::vml_oval_model(shape),
          style.as_deref().or(shape.style.as_deref()),
        ) {
          inlines.push(InlineItem::Shape(shape));
        }
      }
      v::GroupChoice::Rectangle(rectangle) => {
        let style = transform.and_then(|transform| {
          transform.child_anchor_style(group.style.as_deref(), rectangle.style.as_deref())
        });
        if let Some(shape) = vml_rectangle_shape_with_style(rectangle, style.as_deref(), images) {
          inlines.push(InlineItem::Shape(shape));
        }
      }
      v::GroupChoice::RoundRectangle(round_rectangle) => {
        let style = transform.and_then(|transform| {
          transform.child_anchor_style(group.style.as_deref(), round_rectangle.style.as_deref())
        });
        if let Some(shape) = vml_round_rectangle_shape_with_style(round_rectangle, style.as_deref())
        {
          inlines.push(InlineItem::Shape(shape));
        }
      }
      v::GroupChoice::Shape(shape) => {
        let style = transform.and_then(|transform| {
          transform.child_anchor_style(group.style.as_deref(), shape.style.as_deref())
        });
        if let Some(shape) =
          vml_shape_shape_with_style(shape, style.as_deref(), images, inherited_shape_types)
        {
          inlines.push(InlineItem::Shape(shape));
        }
      }
      v::GroupChoice::PolyLine(polyline) => {
        if let Some(shape) = vml_polyline_shape(polyline) {
          inlines.push(InlineItem::Shape(shape));
        }
      }
      _ => {}
    }
  }
}

fn vml_special_shape(
  model: crate::xlsx::object_resources::VmlShapeModel,
  style: Option<&str>,
) -> Option<InlineShape> {
  let fill_override = crate::xlsx::vml_shape_common_fill(&model, Affine::IDENTITY);
  let stroke_override = crate::xlsx::vml_shape_common_stroke(&model);
  let mut shape = vml_inline_shape(
    style.or(model.style.as_deref()),
    model.allow_in_cell,
    model
      .filled
      .then_some(model.fill_color.as_deref().unwrap_or("white")),
    None,
    model
      .stroked
      .then_some(model.stroke_color.as_deref().unwrap_or("black")),
    model.stroke_weight.as_deref(),
    None,
  )?;
  shape.geometry = InlineShapeGeometry::Path {
    paths: crate::xlsx::vml_shape_drawing_paths(&model, shape.width_pt, shape.height_pt)?,
    outline: None,
  };
  shape.fill_override = Some(Box::new(fill_override));
  shape.stroke_override = stroke_override.map(Box::new);
  apply_vml_model_wrap(&mut shape, &model);
  if !model.text.is_empty() {
    shape.text_box_blocks = vec![simple_text_block(model.text, TextStyle::default())];
  }
  Some(shape)
}

fn vml_rectangle_shape(rectangle: &v::Rectangle, images: &ImageCatalog) -> Option<InlineShape> {
  vml_rectangle_shape_with_style(rectangle, rectangle.style.as_deref(), images)
}

fn vml_rectangle_shape_with_style(
  rectangle: &v::Rectangle,
  style: Option<&str>,
  images: &ImageCatalog,
) -> Option<InlineShape> {
  let model = crate::xlsx::object_resources::vml_rectangle_model(rectangle);
  let fill_image = vml_rectangle_fill_image(rectangle, images);
  let has_fill_image = fill_image.is_some();
  let mut shape = vml_inline_shape(
    style,
    model.allow_in_cell,
    model
      .filled
      .then_some(model.fill_color.as_deref().unwrap_or("white"))
      .filter(|_| !has_fill_image),
    fill_image,
    model
      .stroked
      .then_some(model.stroke_color.as_deref().unwrap_or("black")),
    model.stroke_weight.as_deref(),
    None,
  )?;
  shape.fill_override = (model.filled && !has_fill_image)
    .then(|| Box::new(crate::xlsx::vml_shape_common_fill(&model, Affine::IDENTITY)));
  shape.stroke_override = crate::xlsx::vml_shape_common_stroke(&model).map(Box::new);
  apply_vml_model_wrap(&mut shape, &model);
  Some(shape)
}

fn vml_round_rectangle_shape(round_rectangle: &v::RoundRectangle) -> Option<InlineShape> {
  vml_round_rectangle_shape_with_style(round_rectangle, round_rectangle.style.as_deref())
}

fn vml_round_rectangle_shape_with_style(
  round_rectangle: &v::RoundRectangle,
  style: Option<&str>,
) -> Option<InlineShape> {
  vml_special_shape(
    crate::xlsx::object_resources::vml_round_rectangle_model(round_rectangle),
    style,
  )
}

fn vml_shape_shape(
  shape: &v::Shape,
  images: &ImageCatalog,
  shape_types: &[&v::Shapetype],
) -> Option<InlineShape> {
  vml_shape_shape_with_style(shape, shape.style.as_deref(), images, shape_types)
}

fn vml_shape_shape_with_style(
  shape: &v::Shape,
  style: Option<&str>,
  images: &ImageCatalog,
  shape_types: &[&v::Shapetype],
) -> Option<InlineShape> {
  let shape_type = shape.r#type.as_deref().and_then(|reference| {
    let id = reference.strip_prefix('#').unwrap_or(reference);
    shape_types
      .iter()
      .copied()
      .rev()
      .find(|shape_type| shape_type.id.as_deref() == Some(id))
  });
  let is_undeclared_picture_frame = shape_type.is_none()
    && shape.r#type.as_deref().is_some_and(|reference| {
      reference
        .strip_prefix('#')
        .unwrap_or(reference)
        .eq_ignore_ascii_case("_x0000_t75")
    });
  let merged_style = merge_vml_style(
    shape_type.and_then(|shape_type| shape_type.style.as_deref()),
    style,
  );
  let style = merged_style.as_deref().or(style);
  let common_model = crate::xlsx::object_resources::vml_shape_model(shape, shape_type);
  let direct_path = vml_shape_path(shape);
  let inherited_path = shape_type.and_then(vml_shapetype_path);
  let path = direct_path
    .and_then(|path| path.value.as_deref())
    .or_else(|| {
      shape
        .edge_path
        .as_deref()
        .filter(|path| !path.trim().is_empty())
    })
    .or_else(|| inherited_path.and_then(|path| path.value.as_deref()))
    .or_else(|| {
      shape_type
        .and_then(|shape_type| shape_type.edge_path.as_deref())
        .filter(|path| !path.trim().is_empty())
    });
  let path_properties = direct_path.or(inherited_path);
  let direct_fill = vml_shape_fill(shape);
  let inherited_fill = shape_type.and_then(vml_shapetype_fill);
  let direct_stroke = vml_shape_stroke(shape);
  let inherited_stroke = shape_type.and_then(vml_shapetype_stroke);
  let fill_image = direct_fill
    .and_then(|fill| vml_fill_image(fill, style, images))
    .or_else(|| inherited_fill.and_then(|fill| vml_fill_image(fill, style, images)));
  let has_fill_image = fill_image.is_some();
  let filled = direct_fill
    .and_then(|fill| fill.on.map(|value| value.as_bool()))
    .or_else(|| shape.filled.map(|value| value.as_bool()))
    .or_else(|| inherited_fill.and_then(|fill| fill.on.map(|value| value.as_bool())))
    .or_else(|| shape_type.and_then(|value| value.filled.map(|value| value.as_bool())))
    .unwrap_or(!is_undeclared_picture_frame);
  let stroked = direct_stroke
    .and_then(|stroke| stroke.on.map(|value| value.as_bool()))
    .or_else(|| shape.stroked.map(|value| value.as_bool()))
    .or_else(|| inherited_stroke.and_then(|stroke| stroke.on.map(|value| value.as_bool())))
    .or_else(|| shape_type.and_then(|value| value.stroked.map(|value| value.as_bool())))
    .unwrap_or(!is_undeclared_picture_frame);
  let mut inline = vml_inline_shape(
    style,
    vml_allow_in_cell(
      shape
        .allow_in_cell
        .or_else(|| shape_type.and_then(|value| value.allow_in_cell)),
    ),
    filled
      .then_some(
        direct_fill
          .and_then(|fill| fill.color.as_deref())
          .or(shape.fill_color.as_deref())
          .or_else(|| inherited_fill.and_then(|fill| fill.color.as_deref()))
          .or_else(|| shape_type.and_then(|shape_type| shape_type.fill_color.as_deref()))
          .unwrap_or("white"),
      )
      .filter(|_| fill_image.is_none()),
    fill_image,
    stroked.then_some(
      direct_stroke
        .and_then(|stroke| stroke.color.as_deref())
        .or(shape.stroke_color.as_deref())
        .or_else(|| inherited_stroke.and_then(|stroke| stroke.color.as_deref()))
        .or_else(|| shape_type.and_then(|shape_type| shape_type.stroke_color.as_deref()))
        .unwrap_or("black"),
    ),
    direct_stroke
      .and_then(|stroke| stroke.weight.as_deref())
      .or(shape.stroke_weight.as_deref())
      .or_else(|| inherited_stroke.and_then(|stroke| stroke.weight.as_deref()))
      .or_else(|| shape_type.and_then(|shape_type| shape_type.stroke_weight.as_deref())),
    path
      .is_none()
      .then(|| vml_fontwork_shape_geometry(shape.r#type.as_deref(), shape.id.as_deref()))
      .flatten(),
  )?;
  if !has_fill_image {
    inline.fill_override = Some(Box::new(crate::xlsx::vml_shape_common_fill(
      &common_model,
      Affine::IDENTITY,
    )));
  }
  inline.stroke_override = crate::xlsx::vml_shape_common_stroke(&common_model).map(Box::new);
  if let Some(path) = path
    && let Some(geometry) = vml_path_geometry(
      path,
      VmlPathGeometryOptions {
        coordinate_origin: shape
          .coordinate_origin
          .as_deref()
          .or_else(|| shape_type.and_then(|shape_type| shape_type.coordinate_origin.as_deref())),
        coordinate_size: shape
          .coordinate_size
          .as_deref()
          .or_else(|| shape_type.and_then(|shape_type| shape_type.coordinate_size.as_deref())),
        width_pt: inline.width_pt,
        height_pt: inline.height_pt,
        adjustment: shape
          .adjustment
          .as_deref()
          .or_else(|| shape_type.and_then(|shape_type| shape_type.adjustment.as_deref())),
        formulas: vml_shape_formulas(shape).or_else(|| shape_type.and_then(vml_shapetype_formulas)),
        allow_fill: path_properties
          .and_then(|path| path.allow_fill)
          .is_none_or(|value| value.as_bool()),
        allow_stroke: path_properties
          .and_then(|path| path.allow_stroke)
          .is_none_or(|value| value.as_bool()),
        allow_extrusion: path_properties
          .and_then(|path| path.allow_extrusion)
          .is_none_or(|value| value.as_bool()),
      },
    )
  {
    inline.geometry = geometry;
  }
  if let Some((preset, text_path)) = vml_fontwork_text_path(shape, shape_type) {
    inline.text_warp = Some(Box::new(a::PresetTextWarp {
      preset,
      ..a::PresetTextWarp::default()
    }));
    inline.text_box_blocks = vec![simple_text_block(
      text_path.string.as_deref()?.to_string(),
      vml_text_path_style(text_path.style.as_deref()),
    )];
    inline.text_inset_left_pt = 0.0;
    inline.text_inset_top_pt = 0.0;
    inline.text_inset_right_pt = 0.0;
    inline.text_inset_bottom_pt = 0.0;
  }
  apply_vml_model_wrap(&mut inline, &common_model);
  Some(inline)
}

fn vml_shape_path(shape: &v::Shape) -> Option<&v::Path> {
  shape.shape_choice.iter().find_map(|choice| match choice {
    v::ShapeChoice::Path(path) => Some(path.as_ref()),
    _ => None,
  })
}

fn vml_shapetype_path(shape_type: &v::Shapetype) -> Option<&v::Path> {
  shape_type
    .shapetype_choice
    .iter()
    .find_map(|choice| match choice {
      v::ShapetypeChoice::Path(path) => Some(path.as_ref()),
      _ => None,
    })
}

fn vml_shape_fill(shape: &v::Shape) -> Option<&v::Fill> {
  shape.shape_choice.iter().find_map(|choice| match choice {
    v::ShapeChoice::Fill(fill) => Some(fill.as_ref()),
    _ => None,
  })
}

fn vml_shapetype_fill(shape_type: &v::Shapetype) -> Option<&v::Fill> {
  shape_type
    .shapetype_choice
    .iter()
    .find_map(|choice| match choice {
      v::ShapetypeChoice::Fill(fill) => Some(fill.as_ref()),
      _ => None,
    })
}

fn vml_shape_stroke(shape: &v::Shape) -> Option<&v::Stroke> {
  shape.shape_choice.iter().find_map(|choice| match choice {
    v::ShapeChoice::Stroke(stroke) => Some(stroke.as_ref()),
    _ => None,
  })
}

fn vml_shapetype_stroke(shape_type: &v::Shapetype) -> Option<&v::Stroke> {
  shape_type
    .shapetype_choice
    .iter()
    .find_map(|choice| match choice {
      v::ShapetypeChoice::Stroke(stroke) => Some(stroke.as_ref()),
      _ => None,
    })
}

fn vml_shape_formulas(shape: &v::Shape) -> Option<&v::Formulas> {
  shape.shape_choice.iter().find_map(|choice| match choice {
    v::ShapeChoice::Formulas(formulas) => Some(formulas),
    _ => None,
  })
}

fn vml_shapetype_formulas(shape_type: &v::Shapetype) -> Option<&v::Formulas> {
  shape_type
    .shapetype_choice
    .iter()
    .find_map(|choice| match choice {
      v::ShapetypeChoice::Formulas(formulas) => Some(formulas),
      _ => None,
    })
}

fn merge_vml_style(base: Option<&str>, direct: Option<&str>) -> Option<String> {
  match (base, direct) {
    (None, None) => None,
    (Some(value), None) | (None, Some(value)) => Some(value.to_string()),
    (Some(base), Some(direct)) => {
      let mut declarations = Vec::<(String, String)>::new();
      for source in [base, direct] {
        for declaration in source.split(';') {
          let Some((name, value)) = declaration.split_once(':') else {
            continue;
          };
          let name = name.trim().to_ascii_lowercase();
          if let Some(existing) = declarations
            .iter_mut()
            .find(|(existing, _)| *existing == name)
          {
            existing.1 = value.trim().to_string();
          } else {
            declarations.push((name, value.trim().to_string()));
          }
        }
      }
      Some(
        declarations
          .into_iter()
          .map(|(name, value)| format!("{name}:{value}"))
          .collect::<Vec<_>>()
          .join(";"),
      )
    }
  }
}

fn vml_fontwork_text_path<'a>(
  shape: &'a v::Shape,
  shape_type: Option<&'a v::Shapetype>,
) -> Option<(a::TextShapeValues, &'a v::TextPath)> {
  let text_path = shape
    .shape_choice
    .iter()
    .find_map(|choice| match choice {
      v::ShapeChoice::TextPath(path) => Some(path.as_ref()),
      _ => None,
    })
    .filter(|path| path.on.is_none_or(|value| value.as_bool()))?;
  let shape_type_number = shape_type
    .and_then(|shape_type| shape_type.optional_number)
    .or_else(|| {
      shape
        .r#type
        .as_deref()
        .or(shape.id.as_deref())
        .and_then(vml_shape_type_number)
    })?;
  Some((vml_fontwork_preset(shape_type_number)?, text_path))
}

fn vml_shape_type_number(value: &str) -> Option<i32> {
  value
    .rsplit_once("_x0000_t")
    .or_else(|| value.rsplit_once("mso-spt"))
    .and_then(|(_, value)| value.trim_start_matches('#').parse().ok())
}

fn vml_fontwork_preset(shape_type: i32) -> Option<a::TextShapeValues> {
  Some(match shape_type {
    136 => a::TextShapeValues::TextPlain,
    137 => a::TextShapeValues::TextStop,
    138 => a::TextShapeValues::TextTriangle,
    139 => a::TextShapeValues::TextTriangleInverted,
    140 => a::TextShapeValues::TextChevron,
    141 => a::TextShapeValues::TextChevronInverted,
    142 => a::TextShapeValues::TextRingInside,
    143 => a::TextShapeValues::TextRingOutside,
    144 => a::TextShapeValues::TextArchUp,
    145 => a::TextShapeValues::TextArchDown,
    146 => a::TextShapeValues::TextCircle,
    147 => a::TextShapeValues::TextButton,
    148 => a::TextShapeValues::TextArchUpPour,
    149 => a::TextShapeValues::TextArchDownPour,
    150 => a::TextShapeValues::TextCirclePour,
    151 => a::TextShapeValues::TextButtonPour,
    152 => a::TextShapeValues::TextCurveUp,
    153 => a::TextShapeValues::TextCurveDown,
    154 => a::TextShapeValues::TextCascadeUp,
    155 => a::TextShapeValues::TextCascadeDown,
    156 => a::TextShapeValues::TextWave1,
    157 => a::TextShapeValues::TextWave2,
    158 => a::TextShapeValues::TextDoubleWave1,
    159 => a::TextShapeValues::TextWave4,
    160 => a::TextShapeValues::TextInflate,
    161 => a::TextShapeValues::TextDeflate,
    162 => a::TextShapeValues::TextInflateBottom,
    163 => a::TextShapeValues::TextDeflateBottom,
    164 => a::TextShapeValues::TextInflateTop,
    165 => a::TextShapeValues::TextDeflateTop,
    166 => a::TextShapeValues::TextDeflateInflate,
    167 => a::TextShapeValues::TextDeflateInflateDeflate,
    168 => a::TextShapeValues::TextFadeRight,
    169 => a::TextShapeValues::TextFadeLeft,
    170 => a::TextShapeValues::TextFadeUp,
    171 => a::TextShapeValues::TextFadeDown,
    172 => a::TextShapeValues::TextSlantUp,
    173 => a::TextShapeValues::TextSlantDown,
    174 => a::TextShapeValues::TextCanUp,
    175 => a::TextShapeValues::TextCanDown,
    _ => return None,
  })
}

fn vml_text_path_style(style: Option<&str>) -> TextStyle {
  let mut text_style = TextStyle::default();
  for declaration in style.into_iter().flat_map(|style| style.split(';')) {
    let Some((name, value)) = declaration.split_once(':') else {
      continue;
    };
    match name.trim().to_ascii_lowercase().as_str() {
      "font-family" => {
        let family = value.trim().trim_matches(['\'', '"']);
        if !family.is_empty() {
          text_style.font_family = Some(Arc::from(family));
        }
      }
      "font-size" => {
        if let Some(size) = vml_measure_to_points(value.trim()) {
          text_style.font_size_pt = size;
        }
      }
      "font-weight" => {
        text_style.bold = matches!(
          value.trim().to_ascii_lowercase().as_str(),
          "bold" | "bolder" | "600" | "700" | "800" | "900"
        );
      }
      "font-style" => {
        text_style.italic = matches!(
          value.trim().to_ascii_lowercase().as_str(),
          "italic" | "oblique"
        );
      }
      _ => {}
    }
  }
  text_style
}

fn vml_fontwork_shape_geometry(
  shape_type: Option<&str>,
  shape_id: Option<&str>,
) -> Option<InlineShapeGeometry> {
  let value = shape_type.or(shape_id)?;
  let is_legacy_fontwork = (25..=31).any(|index| {
    let marker = format!("_x0000_t{index}");
    value.contains(&marker)
  });
  is_legacy_fontwork.then(legacy_fontwork_warp_geometry)
}

#[derive(Clone, Copy)]
enum VmlPathToken<'a> {
  Command(&'a str),
  Value(VmlFormulaValue),
}

#[derive(Clone, Copy)]
enum VmlFormulaValue {
  Number(f64),
  Adjustment(usize),
  Formula(usize),
}

pub(crate) struct VmlPathGeometryOptions<'a> {
  pub(crate) coordinate_origin: Option<&'a str>,
  pub(crate) coordinate_size: Option<&'a str>,
  pub(crate) width_pt: f32,
  pub(crate) height_pt: f32,
  pub(crate) adjustment: Option<&'a str>,
  pub(crate) formulas: Option<&'a v::Formulas>,
  pub(crate) allow_fill: bool,
  pub(crate) allow_stroke: bool,
  pub(crate) allow_extrusion: bool,
}

pub(crate) fn vml_path_geometry(
  source: &str,
  options: VmlPathGeometryOptions<'_>,
) -> Option<InlineShapeGeometry> {
  let tokens = vml_path_tokens(source)?;
  let (origin_x, origin_y) = options
    .coordinate_origin
    .and_then(vml_path_coordinate_pair)
    .unwrap_or((0.0, 0.0));
  let (coordinate_width, coordinate_height) = options
    .coordinate_size
    .and_then(vml_path_coordinate_pair)
    .unwrap_or((21_600.0, 21_600.0));
  if coordinate_width.abs() <= f32::EPSILON || coordinate_height.abs() <= f32::EPSILON {
    return None;
  }
  let adjustments = options
    .adjustment
    .into_iter()
    .flat_map(|values| values.split([',', ' ']))
    .filter(|value| !value.is_empty())
    .map(str::parse::<f64>)
    .collect::<std::result::Result<Vec<_>, _>>()
    .ok()?;
  let formula_values = vml_formula_values(
    options.formulas,
    &adjustments,
    f64::from(coordinate_width),
    f64::from(coordinate_height),
  )?;
  let resolve = |value: VmlFormulaValue| -> Option<f32> {
    Some(match value {
      VmlFormulaValue::Number(value) => value,
      VmlFormulaValue::Adjustment(index) => *adjustments.get(index)?,
      VmlFormulaValue::Formula(index) => *formula_values.get(index)?,
    } as f32)
  };
  let map = |x: f32, y: f32| common::Point {
    x: common::Pt((x - origin_x) * options.width_pt / coordinate_width),
    y: common::Pt((y - origin_y) * options.height_pt / coordinate_height),
  };
  let mut paths = Vec::new();
  let mut commands = Vec::new();
  let mut index = 0;
  let mut current = (0.0, 0.0);
  let mut subpath_start = (0.0, 0.0);
  let mut fill = true;
  let mut stroke = true;
  while index < tokens.len() {
    let VmlPathToken::Command(command) = tokens[index] else {
      return None;
    };
    index += 1;
    let start = index;
    while index < tokens.len() && matches!(tokens[index], VmlPathToken::Value(_)) {
      index += 1;
    }
    let values = tokens[start..index]
      .iter()
      .map(|token| match token {
        VmlPathToken::Value(value) => resolve(*value),
        VmlPathToken::Command(_) => None,
      })
      .collect::<Option<Vec<_>>>()?;
    match command {
      "m" | "t" => {
        if command == "m" && values.is_empty() {
          // ECMA-376 Part 4 §19.1 uses `m,l21600,21600e` for the
          // canonical VML line shapetype. Office interprets the omitted
          // absolute moveto pair as the coordinate-space origin.
          current = (0.0, 0.0);
          subpath_start = current;
          commands.push(common::PathCommand::MoveTo(map(current.0, current.1)));
          continue;
        }
        if values.len() < 2 || values.len() % 2 != 0 {
          return None;
        }
        for (pair_index, pair) in values.chunks_exact(2).enumerate() {
          let point = if command == "t" {
            (current.0 + pair[0], current.1 + pair[1])
          } else {
            (pair[0], pair[1])
          };
          if pair_index == 0 {
            commands.push(common::PathCommand::MoveTo(map(point.0, point.1)));
            subpath_start = point;
          } else {
            commands.push(common::PathCommand::LineTo(map(point.0, point.1)));
          }
          current = point;
        }
      }
      "l" | "r" => {
        if values.len() < 2 || values.len() % 2 != 0 {
          return None;
        }
        for pair in values.chunks_exact(2) {
          let point = if command == "r" {
            (current.0 + pair[0], current.1 + pair[1])
          } else {
            (pair[0], pair[1])
          };
          commands.push(common::PathCommand::LineTo(map(point.0, point.1)));
          current = point;
        }
      }
      "c" | "v" => {
        if values.len() % 6 != 0 {
          return None;
        }
        for curve in values.chunks_exact(6) {
          let relative = command == "v";
          let point = |x: f32, y: f32| {
            if relative {
              (current.0 + x, current.1 + y)
            } else {
              (x, y)
            }
          };
          let control1 = point(curve[0], curve[1]);
          let control2 = point(curve[2], curve[3]);
          let end = point(curve[4], curve[5]);
          commands.push(common::PathCommand::CubicTo {
            control1: map(control1.0, control1.1),
            control2: map(control2.0, control2.1),
            end: map(end.0, end.1),
          });
          current = end;
        }
      }
      "qx" | "qy" => {
        if values.len() % 2 != 0 {
          return None;
        }
        for end in values.chunks_exact(2) {
          let end = (end[0], end[1]);
          append_vml_quadrant(&mut commands, map, current, end, command == "qx");
          current = end;
        }
      }
      "qb" => {
        if values.len() < 4 || values.len() % 2 != 0 {
          return None;
        }
        let points = values
          .chunks_exact(2)
          .map(|pair| (pair[0], pair[1]))
          .collect::<Vec<_>>();
        for (point_index, control) in points[..points.len() - 1].iter().copied().enumerate() {
          let end = if point_index + 1 == points.len() - 1 {
            points[points.len() - 1]
          } else {
            let next = points[point_index + 1];
            ((control.0 + next.0) / 2.0, (control.1 + next.1) / 2.0)
          };
          append_vml_quadratic(&mut commands, map, current, control, end);
          current = end;
        }
      }
      "at" | "ar" | "wa" | "wr" => {
        if values.len() % 8 != 0 {
          return None;
        }
        for arc in values.chunks_exact(8) {
          let left = arc[0].min(arc[2]);
          let top = arc[1].min(arc[3]);
          let right = arc[0].max(arc[2]);
          let bottom = arc[1].max(arc[3]);
          let center = ((left + right) / 2.0, (top + bottom) / 2.0);
          let radii = ((right - left) / 2.0, (bottom - top) / 2.0);
          if radii.0 <= f32::EPSILON || radii.1 <= f32::EPSILON {
            return None;
          }
          let angle = |point: (f32, f32)| {
            ((point.1 - center.1) / radii.1).atan2((point.0 - center.0) / radii.0)
          };
          let start_angle = angle((arc[4], arc[5]));
          let end_angle = angle((arc[6], arc[7]));
          let clockwise = matches!(command, "wa" | "wr");
          let sweep = vml_arc_sweep(start_angle, end_angle, clockwise);
          let move_to_start = matches!(command, "ar" | "wr");
          current = append_vml_arc(
            &mut commands,
            map,
            center,
            radii,
            start_angle,
            sweep,
            move_to_start,
          )?;
          if move_to_start {
            subpath_start = (
              center.0 + radii.0 * start_angle.cos(),
              center.1 + radii.1 * start_angle.sin(),
            );
          }
        }
      }
      "ae" | "al" => {
        if values.len() % 6 != 0 {
          return None;
        }
        for arc in values.chunks_exact(6) {
          let radii = (arc[2].abs(), arc[3].abs());
          if radii.0 <= f32::EPSILON || radii.1 <= f32::EPSILON {
            return None;
          }
          let start_angle = (arc[4] / 65_536.0).to_radians();
          let end_angle = (arc[5] / 65_536.0).to_radians();
          let sweep = vml_arc_sweep(start_angle, end_angle, true);
          let move_to_start = command == "al";
          current = append_vml_arc(
            &mut commands,
            map,
            (arc[0], arc[1]),
            radii,
            start_angle,
            sweep,
            move_to_start,
          )?;
          if move_to_start {
            subpath_start = (
              arc[0] + radii.0 * start_angle.cos(),
              arc[1] + radii.1 * start_angle.sin(),
            );
          }
        }
      }
      "x" => {
        if !values.is_empty() {
          return None;
        }
        commands.push(common::PathCommand::Close);
        current = subpath_start;
      }
      "e" => {
        if !values.is_empty() {
          return None;
        }
        push_vml_drawing_path(
          &mut paths,
          &mut commands,
          fill && options.allow_fill,
          stroke && options.allow_stroke,
          options.allow_extrusion,
        );
        fill = true;
        stroke = true;
      }
      "nf" if values.is_empty() => fill = false,
      "ns" if values.is_empty() => stroke = false,
      _ => return None,
    }
  }
  push_vml_drawing_path(
    &mut paths,
    &mut commands,
    fill && options.allow_fill,
    stroke && options.allow_stroke,
    options.allow_extrusion,
  );
  if paths.is_empty() {
    return None;
  }
  Some(InlineShapeGeometry::Path {
    paths,
    outline: None,
  })
}

fn push_vml_drawing_path(
  paths: &mut Vec<common::DrawingPath>,
  commands: &mut Vec<common::PathCommand>,
  fill: bool,
  stroke: bool,
  extrusion_allowed: bool,
) {
  if commands.is_empty() {
    return;
  }
  paths.push(common::DrawingPath {
    commands: std::mem::take(commands),
    fill_mode: if fill {
      common::DrawingPathFillMode::Normal
    } else {
      common::DrawingPathFillMode::None
    },
    stroke,
    extrusion_allowed,
  });
}

fn append_vml_quadrant(
  commands: &mut Vec<common::PathCommand>,
  map: impl Fn(f32, f32) -> common::Point,
  start: (f32, f32),
  end: (f32, f32),
  horizontal_tangent: bool,
) {
  const KAPPA: f32 = 0.552_284_8;
  let dx = end.0 - start.0;
  let dy = end.1 - start.1;
  let (control1, control2) = if horizontal_tangent {
    ((start.0 + KAPPA * dx, start.1), (end.0, end.1 - KAPPA * dy))
  } else {
    ((start.0, start.1 + KAPPA * dy), (end.0 - KAPPA * dx, end.1))
  };
  commands.push(common::PathCommand::CubicTo {
    control1: map(control1.0, control1.1),
    control2: map(control2.0, control2.1),
    end: map(end.0, end.1),
  });
}

fn append_vml_quadratic(
  commands: &mut Vec<common::PathCommand>,
  map: impl Fn(f32, f32) -> common::Point,
  start: (f32, f32),
  control: (f32, f32),
  end: (f32, f32),
) {
  commands.push(common::PathCommand::CubicTo {
    control1: map(
      start.0 + (control.0 - start.0) * (2.0 / 3.0),
      start.1 + (control.1 - start.1) * (2.0 / 3.0),
    ),
    control2: map(
      end.0 + (control.0 - end.0) * (2.0 / 3.0),
      end.1 + (control.1 - end.1) * (2.0 / 3.0),
    ),
    end: map(end.0, end.1),
  });
}

fn vml_arc_sweep(start: f32, end: f32, clockwise: bool) -> f32 {
  let full_turn = std::f32::consts::TAU;
  let mut sweep = (end - start) % full_turn;
  if clockwise {
    if sweep <= f32::EPSILON {
      sweep += full_turn;
    }
  } else if sweep >= -f32::EPSILON {
    sweep -= full_turn;
  }
  sweep
}

fn append_vml_arc(
  commands: &mut Vec<common::PathCommand>,
  map: impl Fn(f32, f32) -> common::Point,
  center: (f32, f32),
  radii: (f32, f32),
  start_angle: f32,
  sweep_angle: f32,
  move_to_start: bool,
) -> Option<(f32, f32)> {
  let segment_count = (sweep_angle.abs() / std::f32::consts::FRAC_PI_2)
    .ceil()
    .max(1.0) as usize;
  let step = sweep_angle / segment_count as f32;
  let start = (
    center.0 + radii.0 * start_angle.cos(),
    center.1 + radii.1 * start_angle.sin(),
  );
  if move_to_start || commands.is_empty() {
    commands.push(common::PathCommand::MoveTo(map(start.0, start.1)));
  } else {
    commands.push(common::PathCommand::LineTo(map(start.0, start.1)));
  }
  let mut angle = start_angle;
  let mut end = start;
  for _ in 0..segment_count {
    let next_angle = angle + step;
    let alpha = (4.0 / 3.0) * (step / 4.0).tan();
    end = (
      center.0 + radii.0 * next_angle.cos(),
      center.1 + radii.1 * next_angle.sin(),
    );
    commands.push(common::PathCommand::CubicTo {
      control1: map(
        center.0 + radii.0 * (angle.cos() - alpha * angle.sin()),
        center.1 + radii.1 * (angle.sin() + alpha * angle.cos()),
      ),
      control2: map(
        center.0 + radii.0 * (next_angle.cos() + alpha * next_angle.sin()),
        center.1 + radii.1 * (next_angle.sin() - alpha * next_angle.cos()),
      ),
      end: map(end.0, end.1),
    });
    angle = next_angle;
  }
  Some(end)
}

fn vml_path_coordinate_pair(value: &str) -> Option<(f32, f32)> {
  let mut values = value
    .split([',', ' '])
    .filter(|value| !value.is_empty())
    .map(str::parse::<f32>);
  Some((values.next()?.ok()?, values.next()?.ok()?))
}

fn vml_path_tokens(source: &str) -> Option<Vec<VmlPathToken<'_>>> {
  let bytes = source.as_bytes();
  let mut tokens = Vec::new();
  let mut index = 0;
  let mut previous_was_comma = false;
  while index < bytes.len() {
    match bytes[index] {
      b' ' | b'\t' | b'\r' | b'\n' => index += 1,
      b',' => {
        // W3C VML path syntax permits a zero parameter to be omitted between
        // two commas: `c10,10,,,25,13` is `c10,10,0,0,25,13`.
        // A comma immediately after a command remains an ordinary separator
        // (`m,l...` is the canonical zero-origin moveto shorthand).
        if previous_was_comma {
          tokens.push(VmlPathToken::Value(VmlFormulaValue::Number(0.0)));
        }
        previous_was_comma = true;
        index += 1;
      }
      byte if byte.is_ascii_alphabetic() => {
        let start = index;
        index += 1;
        if index < bytes.len()
          && bytes[index].is_ascii_alphabetic()
          && matches!(bytes[start], b'n' | b'a' | b'w' | b'q' | b'h')
        {
          index += 1;
        }
        tokens.push(VmlPathToken::Command(&source[start..index]));
        previous_was_comma = false;
      }
      marker @ (b'@' | b'#') => {
        index += 1;
        let start = index;
        while index < bytes.len() && bytes[index].is_ascii_digit() {
          index += 1;
        }
        let reference = source[start..index].parse::<usize>().ok()?;
        tokens.push(VmlPathToken::Value(if marker == b'@' {
          VmlFormulaValue::Formula(reference)
        } else {
          VmlFormulaValue::Adjustment(reference)
        }));
        previous_was_comma = false;
      }
      _ => {
        let start = index;
        if matches!(bytes[index], b'+' | b'-') {
          index += 1;
        }
        while index < bytes.len() && (bytes[index].is_ascii_digit() || bytes[index] == b'.') {
          index += 1;
        }
        if start == index {
          return None;
        }
        tokens.push(VmlPathToken::Value(VmlFormulaValue::Number(
          source[start..index].parse().ok()?,
        )));
        previous_was_comma = false;
      }
    }
  }
  Some(tokens)
}

fn vml_formula_values(
  formulas: Option<&v::Formulas>,
  adjustments: &[f64],
  width: f64,
  height: f64,
) -> Option<Vec<f64>> {
  let mut values = Vec::new();
  for formula in formulas.into_iter().flat_map(|formulas| &formulas.formula) {
    let equation = formula.equation.as_deref()?;
    values.push(vml_formula(equation, adjustments, &values, width, height)?);
  }
  Some(values)
}

fn vml_formula(
  equation: &str,
  adjustments: &[f64],
  formulas: &[f64],
  width: f64,
  height: f64,
) -> Option<f64> {
  let mut tokens = equation.split_ascii_whitespace();
  let operation = tokens.next()?;
  let operands = tokens
    .map(|token| vml_formula_operand(token, adjustments, formulas, width, height))
    .collect::<Option<Vec<_>>>()?;
  let operand = |index: usize, default: f64| operands.get(index).copied().unwrap_or(default);
  let v = operand(0, 0.0);
  let p1 = operand(1, 0.0);
  let p2 = operand(2, 0.0);
  Some(match operation {
    "val" => v,
    "sum" => v + p1 - p2,
    "prod" | "product" => {
      if p2.abs() <= f64::EPSILON {
        return None;
      }
      v * p1 / p2
    }
    "mid" => (v + p1) / 2.0,
    "abs" => v.abs(),
    "min" => v.min(p1),
    "max" => v.max(p1),
    "if" => {
      if v > 0.0 {
        p1
      } else {
        p2
      }
    }
    "mod" => (v * v + p1 * p1 + p2 * p2).sqrt(),
    "atan2" => p1.atan2(v).to_degrees() * 65_536.0,
    "sin" => v * (p1 / 65_536.0).to_radians().sin(),
    "cos" => v * (p1 / 65_536.0).to_radians().cos(),
    "cosatan2" => v * p2.atan2(p1).cos(),
    "sinatan2" => v * p2.atan2(p1).sin(),
    "sqrt" => v.max(0.0).sqrt(),
    "sumangle" => v + (p1 + p2) * 65_536.0,
    "ellipse" => {
      if p1.abs() <= f64::EPSILON {
        return None;
      }
      p2 * (1.0 - (v / p1).powi(2)).max(0.0).sqrt()
    }
    "tan" => v * (p1 / 65_536.0).to_radians().tan(),
    _ => return None,
  })
}

fn vml_formula_operand(
  token: &str,
  adjustments: &[f64],
  formulas: &[f64],
  width: f64,
  height: f64,
) -> Option<f64> {
  if let Some(index) = token.strip_prefix('#') {
    return adjustments.get(index.parse::<usize>().ok()?).copied();
  }
  if let Some(index) = token.strip_prefix('@') {
    return formulas.get(index.parse::<usize>().ok()?).copied();
  }
  Some(match token {
    "width" | "pixelWidth" => width,
    "height" | "pixelHeight" => height,
    "xcenter" => width / 2.0,
    "ycenter" => height / 2.0,
    "pixelLineWidth" | "lineDrawn" => 1.0,
    _ => token.parse().ok()?,
  })
}

fn vml_polyline_shape(polyline: &v::PolyLine) -> Option<InlineShape> {
  if vml_style_is_hidden(polyline.style.as_deref()) {
    return None;
  }
  let points = vml_polyline_points(polyline.points.as_deref()?)?;
  let (min_x, min_y, max_x, max_y) = polyline_bounds(&points)?;
  let width_pt = max_x - min_x;
  let height_pt = max_y - min_y;
  if width_pt <= f32::EPSILON || height_pt <= f32::EPSILON {
    return None;
  }
  let repeated_endpoint = points
    .first()
    .zip(points.last())
    .is_some_and(|(first, last)| {
      (first.0 - last.0).abs() <= 0.01 && (first.1 - last.1).abs() <= 0.01
    });
  let relative_points = points
    .into_iter()
    .map(|(x, y)| (x - min_x, y - min_y))
    .collect();
  let filled = polyline.filled.is_none_or(|value| value.as_bool());
  let stroked = polyline.stroked.is_none_or(|value| value.as_bool());
  let closed = filled || repeated_endpoint;
  let common_model = crate::xlsx::object_resources::vml_polyline_model(polyline);
  let fill_override = crate::xlsx::vml_shape_common_fill(&common_model, Affine::IDENTITY);
  let stroke_override = crate::xlsx::vml_shape_common_stroke(&common_model);
  let fill_color = filled
    .then(|| polyline.fill_color.as_deref().and_then(parse_vml_color))
    .flatten();
  let stroke = if stroked {
    Some(BorderStyle {
      width_pt: polyline
        .stroke_weight
        .as_deref()
        .and_then(vml_measure_to_points)
        .unwrap_or_else(|| units::emu_to_points(VML_DEFAULT_STROKE_WEIGHT_EMU)),
      spacing_pt: 0.0,
      color: polyline
        .stroke_color
        .as_deref()
        .and_then(parse_vml_color)
        .unwrap_or(RgbColor { r: 0, g: 0, b: 0 }),
      compound: false,
      dash_pattern: BorderDashPattern::Solid,
      shadow: false,
    })
  } else {
    None
  };
  if matches!(&fill_override, common::Fill::None) && stroke_override.is_none() {
    return None;
  }
  let mut style = vml_image_style(polyline.style.as_deref());
  style.layout_in_cell = vml_allow_in_cell(polyline.allow_in_cell);
  let mut shape = InlineShape {
    width_pt,
    height_pt,
    effect_left_pt: 0.0,
    effect_top_pt: 0.0,
    effect_right_pt: 0.0,
    effect_bottom_pt: 0.0,
    geometry: InlineShapeGeometry::Polyline {
      points: relative_points,
      closed,
    },
    offset_x_pt: min_x,
    offset_y_pt: min_y,
    rotation_deg: style.rotation_deg,
    flip_horizontal: style.flip_horizontal,
    flip_vertical: style.flip_vertical,
    fill_color,
    fill_pattern: None,
    fill_override: Some(Box::new(fill_override)),
    additional_fill_colors: Vec::new(),
    fill_image: None,
    stroke,
    stroke_pattern: None,
    stroke_override: stroke_override.map(Box::new),
    suppress_zero_relative_background: false,
    allow_outside_page: style.absolute_position,
    inline_anchor_after_line: false,
    placement: style.placement(),
    chart: None,
    text_warp: None,
    text_fill: None,
    effects: None,
    static3d: None,
    text_upright: false,
    text_box_blocks: Vec::new(),
    text_inset_left_pt: 0.0,
    text_inset_top_pt: 0.0,
    text_inset_right_pt: 0.0,
    text_inset_bottom_pt: 0.0,
    text_box_auto_fit: false,
    text_box_resizes_height_to_fit: false,
    text_box_word_wrap: true,
    text_vertical_alignment: TextBoxVerticalAlignment::Top,
  };
  apply_vml_model_wrap(&mut shape, &common_model);
  Some(shape)
}

fn vml_rectangle_fill_image(
  rectangle: &v::Rectangle,
  images: &ImageCatalog,
) -> Option<InlineShapeImageFill> {
  rectangle
    .rectangle_choice
    .iter()
    .find_map(|choice| match choice {
      v::RectangleChoice::Fill(fill) => vml_fill_image(fill, rectangle.style.as_deref(), images),
      _ => None,
    })
}

fn vml_fill_image(
  fill: &v::Fill,
  _shape_style: Option<&str>,
  images: &ImageCatalog,
) -> Option<InlineShapeImageFill> {
  let relationship_id = fill.relationship_id.as_ref().or(fill.id.as_ref())?;
  let resource = images.by_relationship_id.get(relationship_id)?;
  let recolored_pattern =
    crate::xlsx::recolor_typed_vml_pattern_image(fill, resource.data.as_ref());
  let data = recolored_pattern
    .map(Arc::from)
    .unwrap_or_else(|| resource.data.clone());
  let content_type = if data.as_ref() == resource.data.as_ref() {
    resource.content_type.clone()
  } else {
    Some("image/png".to_string())
  };

  Some(InlineShapeImageFill {
    data,
    content_type,
    crop: ImageCrop::default(),
    rotation_deg: 0.0,
    flip_horizontal: false,
    flip_vertical: false,
    rotate_with_shape: fill.rotate.is_some_and(|value| value.as_bool()),
    mode: match fill.r#type {
      Some(v::FillTypeValues::Tile | v::FillTypeValues::Pattern) => {
        InlineShapeImageFillMode::Tile {
          size: fill.size.clone(),
          origin: fill.origin.clone(),
          position: fill.position.clone(),
        }
      }
      _ => match fill.aspect.unwrap_or_default() {
        v::ImageAspectValues::AtMost => InlineShapeImageFillMode::Contain,
        v::ImageAspectValues::AtLeast => InlineShapeImageFillMode::Cover,
        v::ImageAspectValues::Ignore => InlineShapeImageFillMode::Stretch,
      },
    },
  })
}

fn vml_inline_shape(
  style: Option<&str>,
  layout_in_cell: bool,
  fill_color: Option<&str>,
  fill_image: Option<InlineShapeImageFill>,
  stroke_color: Option<&str>,
  stroke_weight: Option<&str>,
  geometry_override: Option<InlineShapeGeometry>,
) -> Option<InlineShape> {
  let fill_color = fill_color.and_then(parse_vml_color);
  let stroke = stroke_color
    .and_then(parse_vml_color)
    .map(|color| BorderStyle {
      width_pt: stroke_weight
        .and_then(vml_measure_to_points)
        .unwrap_or_else(|| units::emu_to_points(VML_DEFAULT_STROKE_WEIGHT_EMU)),
      spacing_pt: 0.0,
      color,
      compound: false,
      dash_pattern: BorderDashPattern::Solid,
      shadow: false,
    });
  if fill_color.is_none() && fill_image.is_none() && stroke.is_none() {
    return None;
  }

  let mut shape = vml_shape_frame(
    style,
    layout_in_cell,
    geometry_override.unwrap_or(InlineShapeGeometry::Rectangle),
  )?;
  shape.fill_color = fill_color;
  shape.fill_image = fill_image;
  shape.stroke = stroke;
  Some(shape)
}

fn vml_inline_group_frame(group: &v::Group) -> Option<InlineShape> {
  if vml_group_has_explicit_floating_position(group.style.as_deref()) {
    return None;
  }
  let mut frame = vml_shape_frame(
    group.style.as_deref(),
    vml_allow_in_cell(group.allow_in_cell),
    InlineShapeGeometry::Rectangle,
  )?;
  // mso-position-*-relative selects the coordinate system used by the
  // group's positioned children. Without an authored absolute position,
  // those declarations do not remove the root group box from line flow.
  frame.placement = ImagePlacement::Inline;
  frame.allow_outside_page = false;
  Some(frame)
}

fn vml_group_has_explicit_floating_position(style: Option<&str>) -> bool {
  style.is_some_and(|style| {
    style.split(';').any(|declaration| {
      let Some((name, value)) = declaration.split_once(':') else {
        return false;
      };
      match name.trim().to_ascii_lowercase().as_str() {
        "position" => value.trim().eq_ignore_ascii_case("absolute"),
        "left"
        | "margin-left"
        | "top"
        | "margin-top"
        | "z-index"
        | "mso-position-horizontal"
        | "mso-position-vertical" => true,
        _ => false,
      }
    })
  })
}

fn vml_shape_frame(
  style: Option<&str>,
  layout_in_cell: bool,
  geometry: InlineShapeGeometry,
) -> Option<InlineShape> {
  if vml_style_is_hidden(style) {
    return None;
  }

  let mut style = vml_image_style(style);
  style.layout_in_cell = layout_in_cell;
  let (width_pt, height_pt) = style.size_pt?;
  Some(InlineShape {
    width_pt,
    height_pt,
    effect_left_pt: 0.0,
    effect_top_pt: 0.0,
    effect_right_pt: 0.0,
    effect_bottom_pt: 0.0,
    geometry,
    offset_x_pt: 0.0,
    offset_y_pt: 0.0,
    rotation_deg: style.rotation_deg,
    flip_horizontal: style.flip_horizontal,
    flip_vertical: style.flip_vertical,
    fill_color: None,
    fill_pattern: None,
    fill_override: None,
    additional_fill_colors: Vec::new(),
    fill_image: None,
    stroke: None,
    stroke_pattern: None,
    stroke_override: None,
    suppress_zero_relative_background: false,
    allow_outside_page: style.absolute_position,
    inline_anchor_after_line: false,
    placement: style.placement(),
    chart: None,
    text_warp: None,
    text_fill: None,
    effects: None,
    static3d: None,
    text_upright: false,
    text_box_blocks: Vec::new(),
    text_inset_left_pt: 0.0,
    text_inset_top_pt: 0.0,
    text_inset_right_pt: 0.0,
    text_inset_bottom_pt: 0.0,
    text_box_auto_fit: false,
    text_box_resizes_height_to_fit: false,
    text_box_word_wrap: true,
    text_vertical_alignment: TextBoxVerticalAlignment::Top,
  })
}

fn vml_textbox_frame(
  shape_style: Option<&str>,
  layout_in_cell: bool,
  textbox: &v::TextBox,
  styles: &StylesCatalog,
  images: &ImageCatalog,
  hyperlinks: &HyperlinkCatalog,
) -> Option<InlineShape> {
  if vml_style_is_hidden(shape_style) {
    return None;
  }

  let Some(v::TextBoxChoice::TextBoxContent(content)) = textbox.text_box_choice.as_ref() else {
    return None;
  };
  let mut style = vml_image_style(shape_style);
  style.layout_in_cell = layout_in_cell;
  let (shape_width_pt, shape_height_pt) = style.size_pt?;
  let mut frame = TextBoxFrameContent::new(textbox_blocks(content, styles, images, hyperlinks));
  apply_vml_textbox_properties(textbox, &mut frame);
  let auto_fit = vml_textbox_fits_shape_to_text(textbox);
  let width_pt = if auto_fit {
    // frames that can grow horizontally instead of wrapping on the narrow
    // imported shape width.
    shape_width_pt.max(DEFAULT_TEXTBOX_AUTO_FIT_WIDTH_PT)
  } else {
    (shape_width_pt - frame.left_pt - frame.right_pt).max(DEFAULT_TEXTBOX_MIN_WIDTH_PT)
  };
  let height_pt =
    (shape_height_pt - frame.top_pt - frame.bottom_pt).max(DEFAULT_TEXTBOX_MIN_HEIGHT_PT);

  Some(InlineShape {
    width_pt,
    height_pt,
    effect_left_pt: 0.0,
    effect_top_pt: 0.0,
    effect_right_pt: 0.0,
    effect_bottom_pt: 0.0,
    geometry: InlineShapeGeometry::Rectangle,
    offset_x_pt: frame.left_pt,
    offset_y_pt: frame.top_pt,
    rotation_deg: style.rotation_deg,
    flip_horizontal: style.flip_horizontal,
    flip_vertical: style.flip_vertical,
    fill_color: None,
    fill_pattern: None,
    fill_override: None,
    additional_fill_colors: Vec::new(),
    fill_image: None,
    stroke: None,
    stroke_pattern: None,
    stroke_override: None,
    suppress_zero_relative_background: false,
    allow_outside_page: style.absolute_position,
    inline_anchor_after_line: false,
    placement: style.placement(),
    chart: None,
    text_warp: None,
    text_fill: None,
    effects: None,
    static3d: None,
    text_upright: false,
    text_box_blocks: frame.blocks,
    text_inset_left_pt: 0.0,
    text_inset_top_pt: 0.0,
    text_inset_right_pt: 0.0,
    text_inset_bottom_pt: 0.0,
    text_box_auto_fit: auto_fit,
    text_box_resizes_height_to_fit: false,
    text_box_word_wrap: true,
    text_vertical_alignment: frame.vertical_alignment,
  })
}

fn vml_textbox_fits_shape_to_text(textbox: &v::TextBox) -> bool {
  textbox.style.as_deref().is_some_and(|style| {
    style.split(';').any(|declaration| {
      let Some((name, value)) = declaration.split_once(':') else {
        return false;
      };
      name.trim().eq_ignore_ascii_case("mso-fit-shape-to-text")
        && matches!(
          value.trim().to_ascii_lowercase().as_str(),
          "t" | "true" | "1"
        )
    })
  })
}

fn apply_vml_textbox_properties(textbox: &v::TextBox, frame: &mut TextBoxFrameContent) {
  if let Some(inset) = textbox.inset.as_deref() {
    let mut values = inset.split(',').map(str::trim);
    frame.left_pt = values
      .next()
      .and_then(vml_measure_to_points)
      .unwrap_or(frame.left_pt);
    frame.top_pt = values
      .next()
      .and_then(vml_measure_to_points)
      .unwrap_or(frame.top_pt);
    frame.right_pt = values
      .next()
      .and_then(vml_measure_to_points)
      .unwrap_or(frame.right_pt);
    frame.bottom_pt = values
      .next()
      .and_then(vml_measure_to_points)
      .unwrap_or(frame.bottom_pt);
  }

  if let Some(style) = textbox.style.as_deref() {
    for declaration in style.split(';') {
      let Some((name, value)) = declaration.split_once(':') else {
        continue;
      };
      if name.trim().eq_ignore_ascii_case("v-text-anchor") {
        frame.vertical_alignment = match value.trim().to_ascii_lowercase().as_str() {
          "middle" => TextBoxVerticalAlignment::Center,
          "bottom" => TextBoxVerticalAlignment::Bottom,
          _ => frame.vertical_alignment,
        };
      }
    }
  }
}

pub(crate) fn parse_vml_color(value: &str) -> Option<RgbColor> {
  let value = value.trim().trim_matches('"');
  let base = value.split_whitespace().next()?;
  if let Some(hex) = base.strip_prefix('#') {
    parse_hex_color(hex)
  } else {
    parse_hex_color(base).or_else(|| vml_named_color(base))
  }
}

fn vml_named_color(value: &str) -> Option<RgbColor> {
  match value.to_ascii_lowercase().as_str() {
    "aqua" => Some(RgbColor {
      r: 0,
      g: 255,
      b: 255,
    }),
    "black" => Some(RgbColor { r: 0, g: 0, b: 0 }),
    "blue" => Some(RgbColor { r: 0, g: 0, b: 255 }),
    "fuchsia" => Some(RgbColor {
      r: 255,
      g: 0,
      b: 255,
    }),
    "gray" => Some(RgbColor {
      r: 128,
      g: 128,
      b: 128,
    }),
    "green" => Some(RgbColor { r: 0, g: 128, b: 0 }),
    "lime" => Some(RgbColor { r: 0, g: 255, b: 0 }),
    "maroon" => Some(RgbColor { r: 128, g: 0, b: 0 }),
    "navy" => Some(RgbColor { r: 0, g: 0, b: 128 }),
    "olive" => Some(RgbColor {
      r: 128,
      g: 128,
      b: 0,
    }),
    "purple" => Some(RgbColor {
      r: 128,
      g: 0,
      b: 128,
    }),
    "red" => Some(RgbColor { r: 255, g: 0, b: 0 }),
    "silver" => Some(RgbColor {
      r: 192,
      g: 192,
      b: 192,
    }),
    "teal" => Some(RgbColor {
      r: 0,
      g: 128,
      b: 128,
    }),
    "white" => Some(RgbColor {
      r: 255,
      g: 255,
      b: 255,
    }),
    "yellow" => Some(RgbColor {
      r: 255,
      g: 255,
      b: 0,
    }),
    _ => None,
  }
}

fn vml_polyline_points(value: &str) -> Option<Vec<(f32, f32)>> {
  let values = value
    .split(',')
    .map(|part| vml_measure_to_points(part.trim()))
    .collect::<Option<Vec<_>>>()?;
  let mut points = Vec::new();
  for pair in values.chunks_exact(2) {
    points.push((pair[0], pair[1]));
  }
  (points.len() >= 2).then_some(points)
}

fn polyline_bounds(points: &[(f32, f32)]) -> Option<(f32, f32, f32, f32)> {
  let bounds = common::drawingml_geometry::point_bounds(
    points
      .iter()
      .map(|&(x, y)| kurbo::Point::new(f64::from(x), f64::from(y))),
  )?;
  Some((
    bounds.x0 as f32,
    bounds.y0 as f32,
    bounds.x1 as f32,
    bounds.y1 as f32,
  ))
}

fn pict_image_impl(picture: &w::Picture, images: &ImageCatalog) -> Option<InlineImage> {
  let mut image = picture
    .picture_choice
    .iter()
    .find_map(|choice| picture_choice_image(choice, images))?;
  // Word controls use the VML picture as their static fixed-output
  // representation. TextOut records in that metafile are real control
  // content, while strings in an ordinary VML image are not automatically
  // document text.
  image.semantic_metafile_text |= picture.control.is_some();
  Some(image)
}

fn push_pict_textboxes_impl(
  picture: &w::Picture,
  inlines: &mut Vec<InlineItem>,
  base_style: TextStyle,
  styles: &StylesCatalog,
  images: &ImageCatalog,
  hyperlinks: &HyperlinkCatalog,
) {
  for choice in &picture.picture_choice {
    push_picture_choice_textboxes(
      choice,
      inlines,
      base_style.clone(),
      styles,
      images,
      hyperlinks,
    );
  }
}

fn picture_choice_image(choice: &w::PictureChoice, images: &ImageCatalog) -> Option<InlineImage> {
  match choice {
    w::PictureChoice::Group(group) => group_image(group, images),
    w::PictureChoice::ImageFile(image) => image_file_image(image, images),
    w::PictureChoice::Rectangle(rectangle) => rectangle_image(rectangle, images),
    w::PictureChoice::RoundRectangle(round_rectangle) => {
      round_rectangle_image(round_rectangle, images)
    }
    w::PictureChoice::Shape(shape) => shape_image(shape, images),
    _ => None,
  }
}

fn embedded_object_image(object: &w::EmbeddedObject, images: &ImageCatalog) -> Option<InlineImage> {
  let content_representation = object.embedded_object_choice1.iter().any(|choice| {
    matches!(
      choice,
      w::EmbeddedObjectChoice::OleObject(ole)
        if ole.draw_aspect == Some(o::OleDrawAspectValues::Content)
    )
  }) || matches!(
    object.embedded_object_choice2.as_ref(),
    Some(w::EmbeddedObjectChoice2::ObjectEmbed(embed))
      if embed.draw_aspect == Some(w::ObjectDrawAspect::Content)
  ) || matches!(
    object.embedded_object_choice2.as_ref(),
    Some(w::EmbeddedObjectChoice2::ObjectLink(link))
      if link.draw_aspect == Some(w::ObjectDrawAspect::Content)
  );
  let mut image = object
    .embedded_object_choice1
    .iter()
    .find_map(|choice| match choice {
      w::EmbeddedObjectChoice::Group(group) => group_image(group, images),
      w::EmbeddedObjectChoice::ImageFile(image) => image_file_image(image, images),
      w::EmbeddedObjectChoice::Rectangle(rectangle) => rectangle_image(rectangle, images),
      w::EmbeddedObjectChoice::RoundRectangle(round_rectangle) => {
        round_rectangle_image(round_rectangle, images)
      }
      w::EmbeddedObjectChoice::Shape(shape) => shape_image(shape, images),
      _ => None,
    })?;
  image.metafile_background_color = embedded_object_metafile_background_color(object);
  // ECMA-376 Part 1 §17.3.3.19 and Annex L.7.2 require the associated
  // shape/image as the static visual representation when an embedded object
  // is not loaded. Preserve real EMF/WMF TextOut records from that
  // representation for PDF text semantics regardless of the OLE server.
  image.semantic_metafile_text |= content_representation
    && crate::render::emf_wmf::supports_semantic_text(image.content_type.as_deref());
  Some(image)
}

fn embedded_object_metafile_background_color(object: &w::EmbeddedObject) -> Option<[u8; 3]> {
  let shape_types = object
    .embedded_object_choice1
    .iter()
    .filter_map(|choice| match choice {
      w::EmbeddedObjectChoice::Shapetype(shape_type) => Some(shape_type.as_ref()),
      _ => None,
    })
    .collect::<Vec<_>>();
  let shape = object
    .embedded_object_choice1
    .iter()
    .find_map(|choice| match choice {
      w::EmbeddedObjectChoice::Shape(shape) => Some(shape.as_ref()),
      _ => None,
    })?;
  let shape_type = shape.r#type.as_deref().and_then(|reference| {
    let id = reference.strip_prefix('#').unwrap_or(reference);
    shape_types
      .iter()
      .copied()
      .rev()
      .find(|shape_type| shape_type.id.as_deref() == Some(id))
  });
  let model = crate::xlsx::object_resources::vml_shape_model(shape, shape_type);
  match crate::xlsx::vml_shape_common_fill(&model, Affine::IDENTITY) {
    common::Fill::Solid(color) if color.a == u8::MAX => Some([color.r, color.g, color.b]),
    _ => None,
  }
}

fn push_picture_choice_textboxes(
  choice: &w::PictureChoice,
  inlines: &mut Vec<InlineItem>,
  base_style: TextStyle,
  styles: &StylesCatalog,
  images: &ImageCatalog,
  hyperlinks: &HyperlinkCatalog,
) {
  match choice {
    w::PictureChoice::Group(group) => {
      push_group_textboxes(group, inlines, base_style, styles, images, hyperlinks);
    }
    w::PictureChoice::ImageFile(image) => {
      push_image_file_textboxes(image, None, inlines, base_style, styles, images, hyperlinks);
    }
    w::PictureChoice::Rectangle(rectangle) => {
      push_rectangle_textboxes(
        rectangle, None, inlines, base_style, styles, images, hyperlinks,
      );
    }
    w::PictureChoice::RoundRectangle(round_rectangle) => {
      push_round_rectangle_textboxes(
        round_rectangle,
        None,
        inlines,
        base_style,
        styles,
        images,
        hyperlinks,
      );
    }
    w::PictureChoice::Shape(shape) => {
      push_shape_textboxes(shape, None, inlines, base_style, styles, images, hyperlinks);
    }
    _ => {}
  }
}

fn group_image(group: &v::Group, images: &ImageCatalog) -> Option<InlineImage> {
  let transform = VmlGroupTransform::from_group(group);
  group.group_choice.iter().find_map(|choice| match choice {
    v::GroupChoice::Group(group) => group_image(group, images),
    v::GroupChoice::ImageFile(image) => {
      let style = transform.and_then(|transform| {
        transform.child_anchor_style(group.style.as_deref(), image.style.as_deref())
      });
      image_file_image_with_style(image, style.as_deref(), images)
    }
    v::GroupChoice::Rectangle(rectangle) => {
      let style = transform.and_then(|transform| {
        transform.child_anchor_style(group.style.as_deref(), rectangle.style.as_deref())
      });
      rectangle_image_with_style(rectangle, style.as_deref(), images)
    }
    v::GroupChoice::RoundRectangle(round_rectangle) => {
      let style = transform.and_then(|transform| {
        transform.child_anchor_style(group.style.as_deref(), round_rectangle.style.as_deref())
      });
      round_rectangle_image_with_style(round_rectangle, style.as_deref(), images)
    }
    v::GroupChoice::Shape(shape) => {
      let style = transform.and_then(|transform| {
        transform.child_anchor_style(group.style.as_deref(), shape.style.as_deref())
      });
      shape_image_with_style(shape, style.as_deref(), images)
    }
    _ => None,
  })
}

fn push_group_textboxes(
  group: &v::Group,
  inlines: &mut Vec<InlineItem>,
  base_style: TextStyle,
  styles: &StylesCatalog,
  images: &ImageCatalog,
  hyperlinks: &HyperlinkCatalog,
) {
  push_group_child_textboxes(group, inlines, base_style, styles, images, hyperlinks);
  if let Some(frame) = vml_inline_group_frame(group) {
    // Shape paint and VML textboxes are imported in separate passes. Append
    // the root flow frame only after both passes so character/line-relative
    // children are located against the unexpanded anchor line.
    inlines.push(InlineItem::Shape(frame));
  }
}

fn push_group_child_textboxes(
  group: &v::Group,
  inlines: &mut Vec<InlineItem>,
  base_style: TextStyle,
  styles: &StylesCatalog,
  images: &ImageCatalog,
  hyperlinks: &HyperlinkCatalog,
) {
  let transform = VmlGroupTransform::from_group(group);
  for choice in &group.group_choice {
    match choice {
      v::GroupChoice::Group(group) => {
        push_group_child_textboxes(
          group,
          inlines,
          base_style.clone(),
          styles,
          images,
          hyperlinks,
        );
      }
      v::GroupChoice::ImageFile(image) => {
        let style = transform.and_then(|transform| {
          transform.child_anchor_style(group.style.as_deref(), image.style.as_deref())
        });
        push_image_file_textboxes(
          image,
          style.as_deref(),
          inlines,
          base_style.clone(),
          styles,
          images,
          hyperlinks,
        );
      }
      v::GroupChoice::Rectangle(rectangle) => {
        let style = transform.and_then(|transform| {
          transform.child_anchor_style(group.style.as_deref(), rectangle.style.as_deref())
        });
        push_rectangle_textboxes(
          rectangle,
          style.as_deref(),
          inlines,
          base_style.clone(),
          styles,
          images,
          hyperlinks,
        );
      }
      v::GroupChoice::RoundRectangle(round_rectangle) => {
        let style = transform.and_then(|transform| {
          transform.child_anchor_style(group.style.as_deref(), round_rectangle.style.as_deref())
        });
        push_round_rectangle_textboxes(
          round_rectangle,
          style.as_deref(),
          inlines,
          base_style.clone(),
          styles,
          images,
          hyperlinks,
        );
      }
      v::GroupChoice::Shape(shape) => {
        let style = transform.and_then(|transform| {
          transform.child_anchor_style(group.style.as_deref(), shape.style.as_deref())
        });
        push_shape_textboxes(
          shape,
          style.as_deref(),
          inlines,
          base_style.clone(),
          styles,
          images,
          hyperlinks,
        );
      }
      _ => {}
    }
  }
}

fn image_file_image(image: &v::ImageFile, images: &ImageCatalog) -> Option<InlineImage> {
  image_file_image_with_style(image, image.style.as_deref(), images)
}

fn image_file_image_with_style(
  image: &v::ImageFile,
  style: Option<&str>,
  images: &ImageCatalog,
) -> Option<InlineImage> {
  if vml_style_is_hidden(style) {
    return None;
  }

  image
    .image_file_choice
    .iter()
    .find_map(|choice| match choice {
      v::ImageFileChoice::ImageData(data) => vml_image_data(
        data,
        style,
        vml_allow_in_cell(image.allow_in_cell),
        image.alternate.clone(),
        images,
      ),
      _ => None,
    })
}

fn push_image_file_textboxes(
  image: &v::ImageFile,
  style_override: Option<&str>,
  inlines: &mut Vec<InlineItem>,
  base_style: TextStyle,
  styles: &StylesCatalog,
  images: &ImageCatalog,
  hyperlinks: &HyperlinkCatalog,
) {
  let style = style_override.or(image.style.as_deref());
  if vml_style_is_hidden(style) {
    return;
  }

  for choice in &image.image_file_choice {
    if let v::ImageFileChoice::TextBox(textbox) = choice {
      if let Some(frame) = vml_textbox_frame(
        style,
        vml_allow_in_cell(image.allow_in_cell),
        textbox,
        styles,
        images,
        hyperlinks,
      ) {
        inlines.push(InlineItem::Shape(frame));
      } else {
        push_vml_textbox(
          textbox,
          inlines,
          base_style.clone(),
          styles,
          images,
          hyperlinks,
        );
      }
    }
  }
}

fn rectangle_image(rectangle: &v::Rectangle, images: &ImageCatalog) -> Option<InlineImage> {
  rectangle_image_with_style(rectangle, rectangle.style.as_deref(), images)
}

fn rectangle_image_with_style(
  rectangle: &v::Rectangle,
  style: Option<&str>,
  images: &ImageCatalog,
) -> Option<InlineImage> {
  if vml_style_is_hidden(style) {
    return None;
  }

  rectangle
    .rectangle_choice
    .iter()
    .find_map(|choice| match choice {
      v::RectangleChoice::ImageData(data) => vml_image_data(
        data,
        style,
        vml_allow_in_cell(rectangle.allow_in_cell),
        rectangle.alternate.clone(),
        images,
      ),
      _ => None,
    })
}

fn round_rectangle_image(
  round_rectangle: &v::RoundRectangle,
  images: &ImageCatalog,
) -> Option<InlineImage> {
  round_rectangle_image_with_style(round_rectangle, round_rectangle.style.as_deref(), images)
}

fn round_rectangle_image_with_style(
  round_rectangle: &v::RoundRectangle,
  style: Option<&str>,
  images: &ImageCatalog,
) -> Option<InlineImage> {
  if vml_style_is_hidden(style) {
    return None;
  }

  round_rectangle
    .round_rectangle_choice
    .iter()
    .find_map(|choice| match choice {
      v::RoundRectangleChoice::ImageData(data) => vml_image_data(
        data,
        style,
        vml_allow_in_cell(round_rectangle.allow_in_cell),
        round_rectangle.alternate.clone(),
        images,
      ),
      _ => None,
    })
}

fn push_rectangle_textboxes(
  rectangle: &v::Rectangle,
  style_override: Option<&str>,
  inlines: &mut Vec<InlineItem>,
  base_style: TextStyle,
  styles: &StylesCatalog,
  images: &ImageCatalog,
  hyperlinks: &HyperlinkCatalog,
) {
  let style = style_override.or(rectangle.style.as_deref());
  if vml_style_is_hidden(style) {
    return;
  }

  for choice in &rectangle.rectangle_choice {
    if let v::RectangleChoice::TextBox(textbox) = choice {
      if let Some(frame) = vml_textbox_frame(
        style,
        vml_allow_in_cell(rectangle.allow_in_cell),
        textbox,
        styles,
        images,
        hyperlinks,
      ) {
        inlines.push(InlineItem::Shape(frame));
      } else {
        push_vml_textbox(
          textbox,
          inlines,
          base_style.clone(),
          styles,
          images,
          hyperlinks,
        );
      }
    }
  }
}

fn push_round_rectangle_textboxes(
  round_rectangle: &v::RoundRectangle,
  style_override: Option<&str>,
  inlines: &mut Vec<InlineItem>,
  base_style: TextStyle,
  styles: &StylesCatalog,
  images: &ImageCatalog,
  hyperlinks: &HyperlinkCatalog,
) {
  let style = style_override.or(round_rectangle.style.as_deref());
  if vml_style_is_hidden(style) {
    return;
  }

  for choice in &round_rectangle.round_rectangle_choice {
    if let v::RoundRectangleChoice::TextBox(textbox) = choice {
      if let Some(frame) = vml_textbox_frame(
        style,
        vml_allow_in_cell(round_rectangle.allow_in_cell),
        textbox,
        styles,
        images,
        hyperlinks,
      ) {
        inlines.push(InlineItem::Shape(frame));
      } else {
        push_vml_textbox(
          textbox,
          inlines,
          base_style.clone(),
          styles,
          images,
          hyperlinks,
        );
      }
    }
  }
}

fn shape_image(shape: &v::Shape, images: &ImageCatalog) -> Option<InlineImage> {
  shape_image_with_style(shape, shape.style.as_deref(), images)
}

fn shape_image_with_style(
  shape: &v::Shape,
  style: Option<&str>,
  images: &ImageCatalog,
) -> Option<InlineImage> {
  if vml_style_is_hidden(style) {
    return None;
  }

  shape.shape_choice.iter().find_map(|choice| match choice {
    v::ShapeChoice::ImageData(data) => vml_image_data(
      data,
      style,
      vml_allow_in_cell(shape.allow_in_cell),
      shape.alternate.clone(),
      images,
    ),
    _ => None,
  })
}

fn push_shape_textboxes(
  shape: &v::Shape,
  style_override: Option<&str>,
  inlines: &mut Vec<InlineItem>,
  base_style: TextStyle,
  styles: &StylesCatalog,
  images: &ImageCatalog,
  hyperlinks: &HyperlinkCatalog,
) {
  let style = style_override.or(shape.style.as_deref());
  if vml_style_is_hidden(style) {
    return;
  }

  for choice in &shape.shape_choice {
    if let v::ShapeChoice::TextBox(textbox) = choice {
      if let Some(frame) = vml_textbox_frame(
        style,
        vml_allow_in_cell(shape.allow_in_cell),
        textbox,
        styles,
        images,
        hyperlinks,
      ) {
        inlines.push(InlineItem::Shape(frame));
      } else {
        push_vml_textbox(
          textbox,
          inlines,
          base_style.clone(),
          styles,
          images,
          hyperlinks,
        );
      }
    }
  }
}

fn vml_style_is_hidden(style: Option<&str>) -> bool {
  style.is_some_and(|style| {
    style.split(';').any(|entry| {
      let Some((name, value)) = entry.split_once(':') else {
        return false;
      };
      name.trim().eq_ignore_ascii_case("visibility") && value.trim().eq_ignore_ascii_case("hidden")
    })
  })
}

fn push_vml_textbox(
  textbox: &v::TextBox,
  inlines: &mut Vec<InlineItem>,
  base_style: TextStyle,
  styles: &StylesCatalog,
  images: &ImageCatalog,
  hyperlinks: &HyperlinkCatalog,
) {
  let Some(v::TextBoxChoice::TextBoxContent(content)) = textbox.text_box_choice.as_ref() else {
    return;
  };
  push_textbox_content(content, inlines, base_style, styles, images, hyperlinks);
}

fn push_textbox_content(
  content: &w::TextBoxContent,
  inlines: &mut Vec<InlineItem>,
  base_style: TextStyle,
  styles: &StylesCatalog,
  images: &ImageCatalog,
  hyperlinks: &HyperlinkCatalog,
) {
  let blocks = textbox_blocks(content, styles, images, hyperlinks);
  for block in blocks {
    match block {
      Block::Paragraph(paragraph) => {
        inlines.extend(paragraph.inlines);
        inlines.push(InlineItem::Text(TextRun {
          text: "\n".into(),
          style: base_style.clone(),
          hyperlink_url: None,
          dynamic_field: None,
          style_ref_keys: Vec::new(),
          style_ref_text: None,
          style_ref_numbering_text: None,
          preserve_text_portion: false,
        }));
      }
      Block::Table(table) => push_table_text(&table, inlines, base_style.clone()),
      Block::Frame(frame) => {
        for block in frame.blocks {
          match block {
            Block::Paragraph(paragraph) => inlines.extend(paragraph.inlines),
            Block::Table(table) => push_table_text(&table, inlines, base_style.clone()),
            Block::Frame(_) => {}
          }
        }
      }
    }
  }
}

fn textbox_blocks(
  content: &w::TextBoxContent,
  styles: &StylesCatalog,
  images: &ImageCatalog,
  hyperlinks: &HyperlinkCatalog,
) -> Vec<Block> {
  textbox_blocks_with_base(content, TextStyle::default(), styles, images, hyperlinks)
}

fn textbox_blocks_with_base(
  content: &w::TextBoxContent,
  base_style: TextStyle,
  styles: &StylesCatalog,
  images: &ImageCatalog,
  hyperlinks: &HyperlinkCatalog,
) -> Vec<Block> {
  let mut blocks = Vec::new();
  let mut numbering = NumberingCatalog::default();
  let mut form_widget_ids = FormWidgetIdAllocator::default();
  let custom_xml_bindings = CustomXmlBindings::default();
  for choice in &content.text_box_content_choice {
    match choice {
      w::TextBoxContentChoice::Paragraph(paragraph) => {
        let paragraph = paragraph_model_with_base(
          paragraph,
          styles,
          &mut numbering,
          images,
          hyperlinks,
          &mut form_widget_ids,
          ParagraphImportBase {
            run_style: base_style.clone(),
            custom_xml_bindings: Some(&custom_xml_bindings),
            ..Default::default()
          },
        );
        blocks.push(Block::paragraph(paragraph));
      }
      w::TextBoxContentChoice::Table(table) => {
        let mut table = table_model(
          table,
          &mut TableModelEnv {
            styles,
            numbering: &mut numbering,
            images,
            hyperlinks,
            custom_xml_bindings: &custom_xml_bindings,
            form_widget_ids: &mut form_widget_ids,
          },
          TableModelContext {
            nested_table_level: 1,
            in_header_footer: false,
          },
        );
        clear_shape_text_table_placements(&mut table);
        blocks.push(Block::Table(table));
      }
      w::TextBoxContentChoice::SdtBlock(sdt) => {
        let mut sdt_blocks = sdt_block_blocks_with_base(
          sdt,
          styles,
          &mut numbering,
          images,
          hyperlinks,
          SdtBlockControls {
            custom_xml_bindings: &custom_xml_bindings,
            form_widget_ids: &mut form_widget_ids,
            // Text boxes are separate stories: do not apply main-story
            // recovery defaults to their paragraphs or nested tables.
            in_header_footer: true,
          },
          Some(&base_style),
        );
        for block in &mut sdt_blocks {
          if let Block::Table(table) = block {
            clear_shape_text_table_placements(table);
          }
        }
        blocks.extend(sdt_blocks);
      }
      _ => {}
    }
  }
  blocks
}

fn clear_shape_text_table_placements(table: &mut Table) {
  table.placement = None;
  for row in &mut table.rows {
    for cell in &mut row.cells {
      for block in &mut cell.blocks {
        match block {
          Block::Table(table) => clear_shape_text_table_placements(table),
          Block::Frame(frame) => {
            for block in &mut frame.blocks {
              if let Block::Table(table) = block {
                clear_shape_text_table_placements(table);
              }
            }
          }
          Block::Paragraph(_) => {}
        }
      }
    }
  }
}

fn push_table_text(table: &Table, inlines: &mut Vec<InlineItem>, style: TextStyle) {
  for row in &table.rows {
    for (index, cell) in row.cells.iter().enumerate() {
      if index > 0 {
        inlines.push(InlineItem::Text(TextRun {
          text: "\t".into(),
          style: style.clone(),
          hyperlink_url: None,
          dynamic_field: None,
          style_ref_keys: Vec::new(),
          style_ref_text: None,
          style_ref_numbering_text: None,
          preserve_text_portion: false,
        }));
      }
      for block in &cell.blocks {
        match block {
          Block::Paragraph(paragraph) => {
            inlines.extend(paragraph.inlines.clone());
          }
          Block::Table(table) => push_table_text(table, inlines, style.clone()),
          Block::Frame(frame) => {
            for block in &frame.blocks {
              match block {
                Block::Paragraph(paragraph) => inlines.extend(paragraph.inlines.clone()),
                Block::Table(table) => push_table_text(table, inlines, style.clone()),
                Block::Frame(_) => {}
              }
            }
          }
        }
      }
    }
    inlines.push(InlineItem::Text(TextRun {
      text: "\n".into(),
      style: style.clone(),
      hyperlink_url: None,
      dynamic_field: None,
      style_ref_keys: Vec::new(),
      style_ref_text: None,
      style_ref_numbering_text: None,
      preserve_text_portion: false,
    }));
  }
}

fn vml_image_data(
  data: &v::ImageData,
  style: Option<&str>,
  layout_in_cell: bool,
  alt_text: Option<String>,
  images: &ImageCatalog,
) -> Option<InlineImage> {
  let relationship_id = data.relationship_id.as_ref().or(data.rel_id.as_ref())?;
  let resource = images.by_relationship_id.get(relationship_id)?;
  let mut style = vml_image_style(style);
  style.layout_in_cell = layout_in_cell;
  let (width_pt, height_pt) = style.size_pt.unwrap_or((72.0, 72.0));

  Some(InlineImage {
    data: resource.data.clone(),
    content_type: resource.content_type.clone(),
    picture_frame: None,
    effects: None,
    static3d: None,
    width_pt,
    height_pt,
    effect_left_pt: 0.0,
    effect_top_pt: 0.0,
    effect_right_pt: 0.0,
    effect_bottom_pt: 0.0,
    crop: vml_image_crop(data),
    rotation_deg: style.rotation_deg,
    flip_horizontal: style.flip_horizontal,
    flip_vertical: style.flip_vertical,
    metafile_background_color: None,
    alt_text: alt_text.or_else(|| data.title.clone()),
    hyperlink_url: None,
    semantic_metafile_text: false,
    metafile_native_size: false,
    picture_content_control: false,
    placement: style.placement(),
  })
}

#[derive(Clone, Copy, Debug)]
struct VmlImageStyle {
  size_pt: Option<(f32, f32)>,
  rotation_deg: f32,
  flip_horizontal: bool,
  flip_vertical: bool,
  absolute_position: bool,
  horizontal_relative_to: HorizontalImageReference,
  horizontal_alignment: Option<HorizontalImageAlignment>,
  vertical_relative_to: VerticalImageReference,
  vertical_alignment: Option<VerticalImageAlignment>,
  horizontal_offset_pt: f32,
  vertical_offset_pt: f32,
  wrap: ImageWrapMode,
  behind_text: bool,
  z_index: Option<i32>,
  layout_in_cell: bool,
  margin_top_pt: f32,
  margin_right_pt: f32,
  margin_bottom_pt: f32,
  margin_left_pt: f32,
}

#[derive(Clone, Copy, Debug)]
struct VmlGroupTransform {
  affine: Affine,
}

impl VmlGroupTransform {
  fn new(origin_x: f32, origin_y: f32, scale_x: f32, scale_y: f32) -> Self {
    Self {
      affine: Affine::translate((-f64::from(origin_x), -f64::from(origin_y)))
        .then_scale_non_uniform(f64::from(scale_x), f64::from(scale_y)),
    }
  }

  fn from_group(group: &v::Group) -> Option<Self> {
    Self::from_group_with_style(group, group.style.as_deref())
  }

  fn from_group_with_style(group: &v::Group, group_style: Option<&str>) -> Option<Self> {
    let style = vml_image_style(group_style);
    let (width_pt, height_pt) = style.size_pt?;
    let (coord_width, coord_height) = vml_coordinate_pair(group.coordinate_size.as_deref())?;
    if coord_width.abs() <= f32::EPSILON || coord_height.abs() <= f32::EPSILON {
      return None;
    }
    let (origin_x, origin_y) =
      vml_coordinate_pair(group.coordinate_origin.as_deref()).unwrap_or((0.0, 0.0));

    Some(Self::new(
      origin_x,
      origin_y,
      width_pt / coord_width,
      height_pt / coord_height,
    ))
  }

  fn child_style(self, style: Option<&str>) -> Option<String> {
    let style = style?;
    let mut output = Vec::new();
    for declaration in style.split(';') {
      let Some((name, value)) = declaration.split_once(':') else {
        output.push(declaration.to_string());
        continue;
      };
      let name = name.trim();
      let value = value.trim();
      let transformed = match name.to_ascii_lowercase().as_str() {
        "left" => vml_raw_coordinate(value)
          .map(|coord| (self.affine * kurbo::Point::new(f64::from(coord), 0.0)).x as f32),
        "top" => vml_raw_coordinate(value)
          .map(|coord| (self.affine * kurbo::Point::new(0.0, f64::from(coord))).y as f32),
        "width" => vml_raw_coordinate(value).map(|coord| {
          common::drawingml_geometry::transform_vector(
            kurbo::Vec2::new(f64::from(coord), 0.0),
            self.affine,
          )
          .x as f32
        }),
        "height" => vml_raw_coordinate(value).map(|coord| {
          common::drawingml_geometry::transform_vector(
            kurbo::Vec2::new(0.0, f64::from(coord)),
            self.affine,
          )
          .y as f32
        }),
        _ => None,
      };
      if let Some(value_pt) = transformed {
        output.push(format!("{name}:{value_pt}pt"));
      } else {
        output.push(declaration.to_string());
      }
    }
    Some(output.join(";"))
  }

  fn child_anchor_style(
    self,
    group_style: Option<&str>,
    child_style: Option<&str>,
  ) -> Option<String> {
    let transformed = self.child_style(child_style)?;
    let parent = vml_image_style(group_style);
    let inline_group = !vml_group_has_explicit_floating_position(group_style);
    if !parent.absolute_position && !inline_group {
      return Some(transformed);
    }

    let child = vml_image_style(Some(&transformed));
    let mut output = vec![
      transformed,
      "position:absolute".to_string(),
      format!(
        "margin-left:{}pt",
        parent.horizontal_offset_pt + child.horizontal_offset_pt
      ),
      format!(
        "margin-top:{}pt",
        parent.vertical_offset_pt + child.vertical_offset_pt
      ),
    ];
    let horizontal_reference = if inline_group {
      HorizontalImageReference::Character
    } else {
      parent.horizontal_relative_to
    };
    let vertical_reference = if inline_group {
      VerticalImageReference::Paragraph
    } else {
      parent.vertical_relative_to
    };
    output.push(vml_horizontal_reference_style(horizontal_reference).to_string());
    output.push(vml_vertical_reference_style(vertical_reference).to_string());
    if parent.behind_text {
      output.push("z-index:-1".to_string());
    }
    Some(output.join(";"))
  }
}

pub(crate) fn vml_group_child_style(
  group: &v::Group,
  group_style: Option<&str>,
  child_style: Option<&str>,
) -> Option<String> {
  VmlGroupTransform::from_group_with_style(group, group_style)?
    .child_anchor_style(group_style, child_style)
}

fn vml_horizontal_reference_style(reference: HorizontalImageReference) -> &'static str {
  match reference {
    HorizontalImageReference::Page => "mso-position-horizontal-relative:page",
    HorizontalImageReference::Margin => "mso-position-horizontal-relative:margin",
    HorizontalImageReference::Character => "mso-position-horizontal-relative:char",
    HorizontalImageReference::Column
    | HorizontalImageReference::LeftMargin
    | HorizontalImageReference::RightMargin
    | HorizontalImageReference::InsideMargin
    | HorizontalImageReference::OutsideMargin => "mso-position-horizontal-relative:text",
  }
}

fn vml_vertical_reference_style(reference: VerticalImageReference) -> &'static str {
  match reference {
    VerticalImageReference::Page => "mso-position-vertical-relative:page",
    VerticalImageReference::Margin => "mso-position-vertical-relative:margin",
    VerticalImageReference::Line => "mso-position-vertical-relative:line",
    VerticalImageReference::TopMargin => "mso-position-vertical-relative:top-margin-area",
    VerticalImageReference::BottomMargin => "mso-position-vertical-relative:bottom-margin-area",
    VerticalImageReference::Paragraph
    | VerticalImageReference::InsideMargin
    | VerticalImageReference::OutsideMargin => "mso-position-vertical-relative:text",
  }
}

fn vml_coordinate_pair(value: Option<&str>) -> Option<(f32, f32)> {
  let mut parts = value?.split(',').map(str::trim);
  let x = parts.next()?.parse::<f32>().ok()?;
  let y = parts.next()?.parse::<f32>().ok()?;
  Some((x, y))
}

fn vml_raw_coordinate(value: &str) -> Option<f32> {
  let value = value.trim();
  (!value.is_empty()
    && value
      .chars()
      .all(|c| c.is_ascii_digit() || matches!(c, '-' | '.' | '+')))
  .then(|| value.parse::<f32>().ok())
  .flatten()
}

impl Default for VmlImageStyle {
  fn default() -> Self {
    Self {
      size_pt: None,
      rotation_deg: 0.0,
      flip_horizontal: false,
      flip_vertical: false,
      absolute_position: false,
      horizontal_relative_to: HorizontalImageReference::Column,
      horizontal_alignment: None,
      vertical_relative_to: VerticalImageReference::Paragraph,
      vertical_alignment: None,
      horizontal_offset_pt: 0.0,
      vertical_offset_pt: 0.0,
      wrap: ImageWrapMode::Square,
      behind_text: false,
      z_index: None,
      layout_in_cell: true,
      margin_top_pt: 0.0,
      margin_right_pt: 0.0,
      margin_bottom_pt: 0.0,
      margin_left_pt: 0.0,
    }
  }
}

impl VmlImageStyle {
  fn placement(self) -> ImagePlacement {
    if self.absolute_position {
      ImagePlacement::Floating(FloatingImagePlacement {
        horizontal_relative_to: self.horizontal_relative_to,
        vertical_relative_to: self.vertical_relative_to,
        horizontal_alignment: self.horizontal_alignment,
        vertical_alignment: self.vertical_alignment,
        horizontal_offset_pt: self.horizontal_offset_pt,
        vertical_offset_pt: self.vertical_offset_pt,
        wrap: self.wrap,
        wrap_side: ImageWrapSide::BothSides,
        behind_text: self.behind_text,
        layout_in_cell: self.layout_in_cell,
        allow_overlap: true,
        paint_order: self
          .z_index
          .map(FloatingPaintOrder::VmlZIndex)
          .unwrap_or_default(),
        relative_width_to: None,
        relative_width_pct: None,
        relative_height_to: None,
        relative_height_pct: None,
        margin_top_pt: self.margin_top_pt,
        margin_right_pt: self.margin_right_pt,
        margin_bottom_pt: self.margin_bottom_pt,
        margin_left_pt: self.margin_left_pt,
      })
    } else {
      ImagePlacement::Inline
    }
  }
}

fn vml_image_crop(data: &v::ImageData) -> ImageCrop {
  ImageCrop {
    left: vml_crop_fraction(data.crop_left.as_deref()),
    top: vml_crop_fraction(data.crop_top.as_deref()),
    right: vml_crop_fraction(data.crop_right.as_deref()),
    bottom: vml_crop_fraction(data.crop_bottom.as_deref()),
  }
}

fn vml_crop_fraction(value: Option<&str>) -> f32 {
  let Some(value) = value.map(str::trim).filter(|value| !value.is_empty()) else {
    return 0.0;
  };

  let fraction = if let Some(percent) = value.strip_suffix('%') {
    percent
      .trim()
      .parse::<f32>()
      .ok()
      .map(|value| value / sdk_units::VML_PERCENT_SCALE as f32)
  } else if let Some(fixed) = value.strip_suffix('f') {
    fixed
      .trim()
      .parse::<sdk_units::VmlFixedValue>()
      .ok()
      .map(|value| sdk_units::vml_fixed_to_ratio(value) as f32)
  } else {
    value.trim().parse::<f32>().ok()
  };

  fraction
    .unwrap_or(0.0)
    .clamp(0.0, units::DRAWINGML_MAX_FRACTION_BELOW_ONE)
}

fn vml_image_style(style: Option<&str>) -> VmlImageStyle {
  let mut width = None;
  let mut height = None;
  let mut wrap_set = false;
  let mut output = VmlImageStyle::default();

  let Some(style) = style else {
    return output;
  };

  for declaration in style.split(';') {
    let Some((name, value)) = declaration.split_once(':') else {
      continue;
    };
    match name.trim().to_ascii_lowercase().as_str() {
      "position" if value.trim().eq_ignore_ascii_case("absolute") => {
        output.absolute_position = true;
      }
      "left" | "margin-left" => {
        output.horizontal_offset_pt = vml_measure_to_points(value).unwrap_or(0.0);
        output.absolute_position = true;
      }
      "top" | "margin-top" => {
        output.vertical_offset_pt = vml_measure_to_points(value).unwrap_or(0.0);
        output.absolute_position = true;
      }
      "width" => width = vml_measure_to_points(value),
      "height" => height = vml_measure_to_points(value),
      "z-index" => {
        output.z_index = value.trim().parse::<i32>().ok();
        output.behind_text = output.z_index.is_some_and(|value| value < 0);
        output.absolute_position = true;
      }
      "mso-position-horizontal-relative" => {
        output.horizontal_relative_to = vml_horizontal_reference(value);
        output.absolute_position = true;
      }
      "mso-position-horizontal" => {
        output.horizontal_alignment = vml_horizontal_alignment(value);
        output.absolute_position = true;
      }
      "mso-position-vertical-relative" => {
        output.vertical_relative_to = vml_vertical_reference(value);
        output.absolute_position = true;
      }
      "mso-position-vertical" => {
        output.vertical_alignment = vml_vertical_alignment(value);
        output.absolute_position = true;
      }
      "mso-wrap-style" => {
        output.wrap = vml_wrap_mode(value);
        wrap_set = true;
      }
      "mso-wrap-distance-left" => {
        output.margin_left_pt = vml_measure_to_points(value).unwrap_or(0.0);
      }
      "mso-wrap-distance-right" => {
        output.margin_right_pt = vml_measure_to_points(value).unwrap_or(0.0);
      }
      "mso-wrap-distance-top" => {
        output.margin_top_pt = vml_measure_to_points(value).unwrap_or(0.0);
      }
      "mso-wrap-distance-bottom" => {
        output.margin_bottom_pt = vml_measure_to_points(value).unwrap_or(0.0);
      }
      "rotation" => output.rotation_deg = vml_rotation_degrees(value),
      "flip" => {
        let value = value.to_ascii_lowercase();
        output.flip_horizontal = value.split_whitespace().any(|token| token == "x");
        output.flip_vertical = value.split_whitespace().any(|token| token == "y");
      }
      _ => {}
    }
  }

  // LibreOffice's VML importer initializes the object surround mode to
  // WrapTextMode_THROUGH and changes it only when a wrap type is authored
  // (oox/source/vml/vmlshape.cxx::lcl_setSurround). Word commonly omits the
  // wrap declaration on absolute legacy shapes, including positive-z shapes
  // that still follow text flow inside a table cell.
  if output.absolute_position && !wrap_set {
    output.wrap = ImageWrapMode::Through;
  }
  output.size_pt = width.zip(height);
  output
}

fn vml_allow_in_cell(value: Option<ooxmlsdk::simple_type::TrueFalseValue>) -> bool {
  value.is_none_or(|value| value.as_bool())
}

fn vml_horizontal_reference(value: &str) -> HorizontalImageReference {
  match value.trim().to_ascii_lowercase().as_str() {
    "page" => HorizontalImageReference::Page,
    "margin" => HorizontalImageReference::Margin,
    "char" | "character" => HorizontalImageReference::Character,
    _ => HorizontalImageReference::Column,
  }
}

fn vml_vertical_reference(value: &str) -> VerticalImageReference {
  match value.trim().to_ascii_lowercase().as_str() {
    "page" => VerticalImageReference::Page,
    "margin" => VerticalImageReference::Margin,
    "top-margin-area" => VerticalImageReference::TopMargin,
    "bottom-margin-area" => VerticalImageReference::BottomMargin,
    "line" => VerticalImageReference::Line,
    _ => VerticalImageReference::Paragraph,
  }
}

fn vml_horizontal_alignment(value: &str) -> Option<HorizontalImageAlignment> {
  match value.trim().to_ascii_lowercase().as_str() {
    "left" => Some(HorizontalImageAlignment::Left),
    "center" => Some(HorizontalImageAlignment::Center),
    "right" => Some(HorizontalImageAlignment::Right),
    "inside" => Some(HorizontalImageAlignment::Inside),
    "outside" => Some(HorizontalImageAlignment::Outside),
    _ => None,
  }
}

fn vml_vertical_alignment(value: &str) -> Option<VerticalImageAlignment> {
  match value.trim().to_ascii_lowercase().as_str() {
    "top" => Some(VerticalImageAlignment::Top),
    "center" => Some(VerticalImageAlignment::Center),
    "bottom" => Some(VerticalImageAlignment::Bottom),
    "inside" => Some(VerticalImageAlignment::Inside),
    "outside" => Some(VerticalImageAlignment::Outside),
    _ => None,
  }
}

fn vml_wrap_mode(value: &str) -> ImageWrapMode {
  match value.trim().to_ascii_lowercase().as_str() {
    "topandbottom" | "top-bottom" | "top_bottom" => ImageWrapMode::TopBottom,
    "none" => ImageWrapMode::Through,
    "through" | "tight" | "square" => ImageWrapMode::Square,
    "inline" => ImageWrapMode::Inline,
    _ => ImageWrapMode::Square,
  }
}

fn apply_vml_model_wrap(
  shape: &mut InlineShape,
  model: &crate::xlsx::object_resources::VmlShapeModel,
) {
  let ImagePlacement::Floating(placement) = &mut shape.placement else {
    return;
  };
  if let Some(wrap_type) = model.wrap_type {
    placement.wrap = match wrap_type {
      w10::WrapValues::TopAndBottom => ImageWrapMode::TopBottom,
      w10::WrapValues::None => ImageWrapMode::Through,
      w10::WrapValues::Square => ImageWrapMode::Square,
      w10::WrapValues::Tight | w10::WrapValues::Through => ImageWrapMode::Tight,
    };
  }
  if let Some(side) = model.wrap_side {
    placement.wrap_side = match side {
      w10::WrapSideValues::Both => ImageWrapSide::BothSides,
      w10::WrapSideValues::Left => ImageWrapSide::Left,
      w10::WrapSideValues::Right => ImageWrapSide::Right,
      w10::WrapSideValues::Largest => ImageWrapSide::Largest,
    };
  }
  // w10:wrap/@anchorx and @anchory locate the wrap coordinate system; they
  // do not replace the shape position references authored by
  // mso-position-*-relative. Word commonly emits anchory="page" together
  // with mso-position-vertical-relative:bottom-margin-area.
}

fn vml_rotation_degrees(value: &str) -> f32 {
  let value = value.trim();
  let rotation = if let Some(fixed) = value.strip_suffix("fd") {
    fixed
      .trim()
      .parse::<sdk_units::VmlFixedValue>()
      .ok()
      .map(|value| sdk_units::vml_fixed_to_ratio(value) as f32)
  } else {
    value.parse::<f32>().ok()
  };
  -rotation.unwrap_or(0.0)
}

pub(crate) fn vml_measure_to_points(value: &str) -> Option<f32> {
  let value = value.trim();
  if value.is_empty() {
    return None;
  }

  if let Some(hex) = value.strip_prefix("0x") {
    return i64::from_str_radix(hex, 16).ok().map(units::emu_to_points);
  }

  let (number, multiplier) = if let Some(number) = value.strip_suffix("pt") {
    (number, 1.0)
  } else if let Some(number) = value.strip_suffix("in") {
    (number, units::POINTS_PER_INCH)
  } else if let Some(number) = value.strip_suffix("cm") {
    (number, units::POINTS_PER_INCH / units::CENTIMETERS_PER_INCH)
  } else if let Some(number) = value.strip_suffix("mm") {
    (number, units::POINTS_PER_INCH / units::MILLIMETERS_PER_INCH)
  } else if let Some(number) = value.strip_suffix("px") {
    (number, units::POINTS_PER_CSS_PIXEL)
  } else {
    (value, 1.0)
  };

  number
    .trim()
    .parse::<f32>()
    .ok()
    .map(|points| points * multiplier)
}

#[derive(Clone, Debug, Default)]
struct DrawingImageProperties {
  relationship_id: Option<String>,
  external_link: bool,
  hyperlink_relationship_id: Option<String>,
  crop: ImageCrop,
  source_rectangle_crop: bool,
  effects: Vec<ImageEffect>,
  picture_frame: Option<Box<InlineShape>>,
  shape_effects: Option<common::DrawingEffectSource>,
  static3d: Option<common::drawingml_3d::Static3dStyle>,
  rotation_deg: f32,
  flip_horizontal: bool,
  flip_vertical: bool,
}

struct DocxImageEffectColorResolver<'a> {
  theme_colors: &'a ThemeColors,
  images: Option<&'a ImageCatalog>,
  placeholder_color: Option<Color>,
  word_group_glow: bool,
}

impl DocxImageEffectColorResolver<'_> {
  fn resolve(&self, color: Option<Color>) -> Option<ResolvedEffectColor> {
    let color = docx_image_color_with_placeholder(
      color?,
      self.theme_colors,
      self.placeholder_color.as_ref(),
    )?;
    Some(ResolvedEffectColor {
      color: RgbColor {
        r: color.r,
        g: color.g,
        b: color.b,
      },
      alpha: color.a,
    })
  }
}

impl ImageEffectColorResolver for DocxImageEffectColorResolver<'_> {
  fn alpha_inverse(&self, choice: &a::AlphaInverseChoice) -> Option<ResolvedEffectColor> {
    self.resolve(Color::from_alpha_inverse_choice(choice))
  }

  fn color_from(&self, choice: &a::ColorFromChoice) -> Option<ResolvedEffectColor> {
    self.resolve(Color::from_color_from_choice(choice))
  }

  fn color_to(&self, choice: &a::ColorToChoice) -> Option<ResolvedEffectColor> {
    self.resolve(Color::from_color_to_choice(choice))
  }

  fn color_replacement(&self, choice: &a::ColorReplacementChoice) -> Option<ResolvedEffectColor> {
    self.resolve(Color::from_color_replacement_choice(choice))
  }

  fn duotone(&self, choice: &a::DuotoneChoice) -> Option<ResolvedEffectColor> {
    self.resolve(Color::from_duotone_choice(choice))
  }

  fn solid_fill(&self, choice: &a::SolidFillChoice) -> Option<ResolvedEffectColor> {
    self.resolve(Color::from_solid_fill_choice(choice))
  }

  fn gradient_stop(&self, choice: &a::GradientStopChoice) -> Option<ResolvedEffectColor> {
    self.resolve(Color::from_gradient_stop_choice(choice))
  }

  fn foreground(&self, choice: &a::ForegroundColorChoice) -> Option<ResolvedEffectColor> {
    self.resolve(Color::from_foreground_color_choice(choice))
  }

  fn background(&self, choice: &a::BackgroundColorChoice) -> Option<ResolvedEffectColor> {
    self.resolve(Color::from_background_color_choice(choice))
  }

  fn glow(&self, choice: &a::GlowChoice) -> Option<ResolvedEffectColor> {
    if self.word_group_glow
      && let a::GlowChoice::SchemeColor(color) = choice
      && let [a::SchemeColorChoice::SaturationModulation(modulation)] =
        color.scheme_color_choice.as_slice()
    {
      let base = resolve_drawingml_scheme_color_value(color.val, self.theme_colors)?;
      let amount = drawingml_percent_to_ratio(&modulation.val)?;
      let [r, g, b] = color_math::apply_linear_saturation_mod([base.r, base.g, base.b], amount);
      return Some(ResolvedEffectColor {
        color: RgbColor { r, g, b },
        alpha: u8::MAX,
      });
    }
    self.resolve(Color::from_glow_choice(choice))
  }

  fn inner_shadow(&self, choice: &a::InnerShadowChoice) -> Option<ResolvedEffectColor> {
    self.resolve(Color::from_inner_shadow_choice(choice))
  }

  fn outer_shadow(&self, choice: &a::OuterShadowChoice) -> Option<ResolvedEffectColor> {
    self.resolve(Color::from_outer_shadow_choice(choice))
  }

  fn preset_shadow(&self, choice: &a::PresetShadowChoice) -> Option<ResolvedEffectColor> {
    self.resolve(Color::from_preset_shadow_choice(choice))
  }

  fn blip_fill(
    &self,
    fill: &a::BlipFill,
  ) -> Option<common::drawingml_image_effects::ImageEffectFill> {
    let blip = fill.blip.as_ref()?;
    let resource = self
      .images?
      .by_relationship_id
      .get(blip.embed.as_deref()?)?;
    let effects = common::drawingml_image_effects::from_blip_choices(
      &blip.blip_choice,
      resource.content_type.as_deref(),
      self,
    );
    common::drawingml_image_effects::raster_fill_image(
      &resource.data,
      resource.content_type.as_deref(),
      &effects,
    )
  }
}

fn drawing_image_properties(
  graphic_data: &ooxmlsdk::schemas::a::GraphicData,
  theme_colors: &ThemeColors,
  images: Option<&ImageCatalog>,
) -> Option<DrawingImageProperties> {
  if graphic_data.uri != "http://schemas.openxmlformats.org/drawingml/2006/picture" {
    return None;
  }
  graphic_data.graphic_data_choice.iter().find_map(|choice| {
    if let a::GraphicDataChoice::Picture(picture) = choice {
      drawing_picture_image_properties(picture, theme_colors, images)
    } else {
      None
    }
  })
}

fn drawing_picture_image_properties(
  picture: &pic::Picture,
  theme_colors: &ThemeColors,
  images: Option<&ImageCatalog>,
) -> Option<DrawingImageProperties> {
  let blip_fill = picture.blip_fill.as_deref()?;
  let blip = blip_fill.blip.as_ref()?;
  let shape_properties = DrawingMlShapeProperties::Picture(
    picture
      .shape_properties
      .as_deref()
      .cloned()
      .unwrap_or_default(),
  );
  let mut properties = DrawingImageProperties {
    relationship_id: blip.embed.clone().or_else(|| blip.link.clone()),
    external_link: blip.embed.is_none() && blip.link.is_some(),
    hyperlink_relationship_id: picture
      .non_visual_picture_properties
      .as_deref()
      .and_then(|properties| {
        properties
          .non_visual_drawing_properties
          .hyperlink_on_click
          .as_ref()
      })
      .and_then(|hyperlink| hyperlink.id.clone()),
    picture_frame: drawingml_picture_frame(
      picture,
      ImagePlacement::Inline,
      DrawingMlGroupTransform::identity(),
      theme_colors,
    )
    .map(Box::new),
    shape_effects: shape_properties.effects(theme_colors, images),
    static3d: shape_properties.static3d(theme_colors),
    ..DrawingImageProperties::default()
  };

  if let Some(crop) = blip_fill
    .source_rectangle
    .as_ref()
    .map(image_crop_from_source_rectangle)
  {
    properties.crop = crop;
    properties.source_rectangle_crop = true;
  } else if let Some(crop) = blip_fill
    .blip_fill_choice
    .as_ref()
    .and_then(|choice| match choice {
      pic::BlipFillChoice::Stretch(stretch) => stretch.fill_rectangle.as_ref(),
      pic::BlipFillChoice::Tile(_) => None,
    })
    .map(image_crop_from_fill_rectangle)
  {
    properties.crop = crop;
  }

  if let Some(transform) = picture
    .shape_properties
    .as_ref()
    .and_then(|properties| properties.transform2_d.as_ref())
  {
    apply_image_transform(&mut properties, transform);
  }

  if let Some(blip) = blip_fill.blip.as_ref() {
    apply_image_effects_from_blip(&mut properties, blip, theme_colors, images);
  }

  Some(properties)
}

fn drawing_blip_fill_image_properties(
  blip_fill: &a::BlipFill,
  theme_colors: &ThemeColors,
  images: Option<&ImageCatalog>,
) -> Option<DrawingImageProperties> {
  let blip = blip_fill.blip.as_ref()?;
  let mut properties = DrawingImageProperties {
    relationship_id: blip.embed.clone().or_else(|| blip.link.clone()),
    external_link: blip.embed.is_none() && blip.link.is_some(),
    ..DrawingImageProperties::default()
  };

  if let Some(crop) = blip_fill
    .source_rectangle
    .as_ref()
    .map(image_crop_from_source_rectangle)
  {
    properties.crop = crop;
    properties.source_rectangle_crop = true;
  } else if let Some(crop) = blip_fill
    .blip_fill_choice
    .as_ref()
    .and_then(|choice| match choice {
      a::BlipFillChoice::Stretch(stretch) => stretch.fill_rectangle.as_ref(),
      a::BlipFillChoice::Tile(_) => None,
    })
    .map(image_crop_from_fill_rectangle)
  {
    properties.crop = crop;
  }

  if let Some(blip) = blip_fill.blip.as_ref() {
    apply_image_effects_from_blip(&mut properties, blip, theme_colors, images);
  }

  Some(properties)
}

fn image_crop_from_source_rectangle(rect: &a::SourceRectangle) -> ImageCrop {
  ImageCrop {
    left: rect
      .left
      .as_ref()
      .and_then(drawingml_percent_to_ratio)
      .unwrap_or(0.0),
    top: rect
      .top
      .as_ref()
      .and_then(drawingml_percent_to_ratio)
      .unwrap_or(0.0),
    right: rect
      .right
      .as_ref()
      .and_then(drawingml_percent_to_ratio)
      .unwrap_or(0.0),
    bottom: rect
      .bottom
      .as_ref()
      .and_then(drawingml_percent_to_ratio)
      .unwrap_or(0.0),
  }
}

fn image_crop_from_fill_rectangle(rect: &a::FillRectangle) -> ImageCrop {
  ImageCrop {
    left: rect
      .left
      .as_ref()
      .and_then(drawingml_percent_to_ratio)
      .unwrap_or(0.0),
    top: rect
      .top
      .as_ref()
      .and_then(drawingml_percent_to_ratio)
      .unwrap_or(0.0),
    right: rect
      .right
      .as_ref()
      .and_then(drawingml_percent_to_ratio)
      .unwrap_or(0.0),
    bottom: rect
      .bottom
      .as_ref()
      .and_then(drawingml_percent_to_ratio)
      .unwrap_or(0.0),
  }
}

fn apply_image_transform(properties: &mut DrawingImageProperties, transform: &a::Transform2D) {
  properties.rotation_deg = transform
    .rotation
    .map(|value| sdk_units::drawingml_angle_to_degrees(value) as f32)
    .unwrap_or(0.0);
  properties.flip_horizontal = transform
    .horizontal_flip
    .as_ref()
    .is_some_and(|value| value.as_bool());
  properties.flip_vertical = transform
    .vertical_flip
    .as_ref()
    .is_some_and(|value| value.as_bool());
}

fn apply_image_effects_from_blip(
  properties: &mut DrawingImageProperties,
  blip: &a::Blip,
  theme_colors: &ThemeColors,
  images: Option<&ImageCatalog>,
) {
  properties
    .effects
    .extend(common::drawingml_image_effects::from_blip_choices(
      &blip.blip_choice,
      None,
      &DocxImageEffectColorResolver {
        theme_colors,
        images,
        placeholder_color: None,
        word_group_glow: false,
      },
    ));
}

fn docx_image_color(color: Color, theme_colors: &ThemeColors) -> Option<common::Color> {
  docx_image_color_with_placeholder(color, theme_colors, None)
}

fn docx_image_color_with_placeholder(
  color: Color,
  theme_colors: &ThemeColors,
  placeholder_color: Option<&Color>,
) -> Option<common::Color> {
  let mut scheme_resolver = |value| {
    let color = resolve_drawingml_scheme_color_value(value, theme_colors)?;
    Some(Color::RgbHex(RgbHexColor {
      value: format!("{:02X}{:02X}{:02X}", color.r, color.g, color.b),
      transformations: Vec::new(),
    }))
  };
  let color = color.resolve_rgb(&mut scheme_resolver, placeholder_color)?;
  Some(common::Color {
    r: color.r,
    g: color.g,
    b: color.b,
    a: ((color.alpha.clamp(0, 100_000) as u32 * u32::from(u8::MAX)) / 100_000) as u8,
  })
}

fn resolve_drawingml_scheme_color(
  color: &a::SchemeColor,
  theme_colors: &ThemeColors,
) -> Option<RgbColor> {
  let mut resolved = resolve_drawingml_scheme_color_value(color.val, theme_colors)?;
  for transform in &color.scheme_color_choice {
    match transform {
      a::SchemeColorChoice::Tint(value) => {
        if let Some(amount) = drawingml_percent_to_ratio(&value.val) {
          resolved = apply_drawingml_tint(resolved, amount);
        }
      }
      a::SchemeColorChoice::Shade(value) => {
        if let Some(amount) = drawingml_percent_to_ratio(&value.val) {
          resolved = apply_drawingml_shade(resolved, amount);
        }
      }
      a::SchemeColorChoice::SaturationModulation(value) => {
        if let Some(amount) = drawingml_percent_to_ratio(&value.val) {
          let mut hsl = hsl_color(resolved);
          hsl.apply_saturation_mod(amount);
          resolved = rgb_color(hsl);
        }
      }
      a::SchemeColorChoice::LuminanceModulation(value) => {
        if let Some(amount) = drawingml_percent_to_ratio(&value.val) {
          let mut hsl = hsl_color(resolved);
          hsl.apply_luminance_mod(amount);
          resolved = rgb_color(hsl);
        }
      }
      a::SchemeColorChoice::LuminanceOffset(value) => {
        if let Some(amount) = drawingml_percent_to_ratio(&value.val) {
          let mut hsl = hsl_color(resolved);
          hsl.apply_luminance_offset(amount);
          resolved = rgb_color(hsl);
        }
      }
      _ => {}
    }
  }
  Some(resolved)
}

fn resolve_drawingml_scheme_color_value(
  value: a::SchemeColorValues,
  theme_colors: &ThemeColors,
) -> Option<RgbColor> {
  match value {
    a::SchemeColorValues::Dark1 | a::SchemeColorValues::Text1 => theme_colors.dark1,
    a::SchemeColorValues::Light1 | a::SchemeColorValues::Background1 => theme_colors.light1,
    a::SchemeColorValues::Dark2 | a::SchemeColorValues::Text2 => theme_colors.dark2,
    a::SchemeColorValues::Light2 | a::SchemeColorValues::Background2 => theme_colors.light2,
    a::SchemeColorValues::Accent1 => theme_colors.accent1,
    a::SchemeColorValues::Accent2 => theme_colors.accent2,
    a::SchemeColorValues::Accent3 => theme_colors.accent3,
    a::SchemeColorValues::Accent4 => theme_colors.accent4,
    a::SchemeColorValues::Accent5 => theme_colors.accent5,
    a::SchemeColorValues::Accent6 => theme_colors.accent6,
    a::SchemeColorValues::Hyperlink => theme_colors.hyperlink,
    a::SchemeColorValues::FollowedHyperlink => theme_colors.followed_hyperlink,
    a::SchemeColorValues::PhColor => None,
  }
}

fn drawingml_preset_color_value(value: a::PresetColorValues) -> Option<RgbColor> {
  match value {
    a::PresetColorValues::White => Some(RgbColor {
      r: 255,
      g: 255,
      b: 255,
    }),
    a::PresetColorValues::Black => Some(RgbColor { r: 0, g: 0, b: 0 }),
    _ => None,
  }
}

#[derive(Clone, Debug, Default)]
struct StylesCatalog {
  import_settings: ImportSettings,
  display_math_alignment: Option<ParagraphAlignment>,
  math_font_family: Option<Arc<str>>,
  simplified_chinese_ui: bool,
  preserve_word_text_whitespace: bool,
  has_styles_part: bool,
  has_default_paragraph_properties: bool,
  doc_default_paragraph: ParagraphFormat,
  doc_default_run: TextStyle,
  default_paragraph_style_id: Option<String>,
  default_table_style_id: Option<String>,
  theme_fonts: ThemeFonts,
  theme_colors: ThemeColors,
  theme_lines: ThemeLineStyles,
  theme_effects: ThemeEffectStyles,
  font_substitutions: HashMap<String, FontSubstitution>,
  styles: HashMap<String, StyleEntry>,
}

#[derive(Clone, Debug, Default)]
struct FontSubstitution {
  alternate_family: Option<Arc<str>>,
  family_class: Option<ooxmlsdk_fonts::FontFamilyClass>,
}

#[derive(Clone, Debug, Default)]
struct ThemeData {
  fonts: ThemeFonts,
  colors: ThemeColors,
  lines: ThemeLineStyles,
  effects: ThemeEffectStyles,
  cjk_punctuation_compression: bool,
}

#[derive(Clone, Debug, Default)]
struct ThemeFonts {
  major_ascii: Option<Arc<str>>,
  major_high_ansi: Option<Arc<str>>,
  major_east_asia: Option<Arc<str>>,
  major_bidi: Option<Arc<str>>,
  minor_ascii: Option<Arc<str>>,
  minor_high_ansi: Option<Arc<str>>,
  minor_east_asia: Option<Arc<str>>,
  minor_bidi: Option<Arc<str>>,
  major_supplemental: Vec<(Arc<str>, Arc<str>)>,
  minor_supplemental: Vec<(Arc<str>, Arc<str>)>,
  latin_language: Option<Arc<str>>,
  east_asia_language: Option<Arc<str>>,
  bidi_language: Option<Arc<str>>,
}

type ThemeFontLanguages = (Option<Arc<str>>, Option<Arc<str>>, Option<Arc<str>>);

#[derive(Clone, Debug, Default)]
struct ThemeLineStyles {
  widths_pt: Vec<f32>,
}

impl ThemeLineStyles {
  fn width_pt(&self, index: usize) -> Option<f32> {
    index
      .checked_sub(1)
      .and_then(|index| self.widths_pt.get(index))
      .copied()
      .filter(|width| *width > 0.0)
  }
}

#[derive(Clone, Debug, Default)]
struct ThemeEffectStyles {
  styles: Vec<a::EffectStyle>,
}

impl ThemeEffectStyles {
  fn get(&self, index: usize) -> Option<&a::EffectStyle> {
    index
      .checked_sub(1)
      .and_then(|index| self.styles.get(index))
  }
}

#[derive(Clone, Debug)]
pub(super) struct ThemeColors {
  dark1: Option<RgbColor>,
  light1: Option<RgbColor>,
  dark2: Option<RgbColor>,
  light2: Option<RgbColor>,
  accent1: Option<RgbColor>,
  accent2: Option<RgbColor>,
  accent3: Option<RgbColor>,
  accent4: Option<RgbColor>,
  accent5: Option<RgbColor>,
  accent6: Option<RgbColor>,
  hyperlink: Option<RgbColor>,
  followed_hyperlink: Option<RgbColor>,
}

impl Default for ThemeColors {
  fn default() -> Self {
    // A Theme part is optional. When it is absent, current Word resolves
    // scheme-color references through the built-in Office theme. These are
    // the colors emitted by current Microsoft Office documents (including
    // the Open XML SDK's Office/Aptos reference package).
    Self {
      dark1: Some(RgbColor { r: 0, g: 0, b: 0 }),
      light1: Some(RgbColor {
        r: 0xFF,
        g: 0xFF,
        b: 0xFF,
      }),
      dark2: Some(RgbColor {
        r: 0x0E,
        g: 0x28,
        b: 0x41,
      }),
      light2: Some(RgbColor {
        r: 0xE8,
        g: 0xE8,
        b: 0xE8,
      }),
      accent1: Some(RgbColor {
        r: 0x15,
        g: 0x60,
        b: 0x82,
      }),
      accent2: Some(RgbColor {
        r: 0xE9,
        g: 0x71,
        b: 0x32,
      }),
      accent3: Some(RgbColor {
        r: 0x19,
        g: 0x6B,
        b: 0x24,
      }),
      accent4: Some(RgbColor {
        r: 0x0F,
        g: 0x9E,
        b: 0xD5,
      }),
      accent5: Some(RgbColor {
        r: 0xA0,
        g: 0x2B,
        b: 0x93,
      }),
      accent6: Some(RgbColor {
        r: 0x4E,
        g: 0xA7,
        b: 0x2E,
      }),
      hyperlink: Some(RgbColor {
        r: 0x46,
        g: 0x78,
        b: 0x86,
      }),
      followed_hyperlink: Some(RgbColor {
        r: 0x96,
        g: 0x60,
        b: 0x7D,
      }),
    }
  }
}

#[derive(Clone, Copy, Debug)]
pub(super) struct ResolvedColor {
  pub color: RgbColor,
  pub opacity: f32,
}

#[derive(Clone, Debug, Default)]
struct StyleEntry {
  style_type: Option<w::StyleValues>,
  custom_style: bool,
  name: Option<String>,
  based_on: Option<String>,
  paragraph_format: ParagraphFormat,
  paragraph_numbering: Option<Box<w::NumberingProperties>>,
  run_style: TextStyle,
  run_overrides: RunStyleOverrides,
  table_style: TableStyleModel,
}

#[derive(Clone, Copy, Debug, Default)]
struct RunStyleOverrides {
  font_size_pt: Option<f32>,
  complex_font_size_pt: Option<f32>,
  vertical_alignment: Option<w::VerticalPositionValues>,
  bold: Option<bool>,
  italic: Option<bool>,
  underline: Option<bool>,
  strikethrough: Option<bool>,
  uppercase: Option<bool>,
  small_caps: Option<bool>,
  hidden: Option<bool>,
  legacy_outline: Option<bool>,
  legacy_shadow: Option<bool>,
  legacy_emboss: Option<bool>,
  legacy_imprint: Option<bool>,
}

#[derive(Clone, Debug, Default)]
struct TableStyleModel {
  table_borders: Option<TableBordersModel>,
  table_shading: Option<Option<RgbColor>>,
  cell_margins: Option<CellMargins>,
  cell_spacing_pt: Option<f32>,
  indent_left_pt: Option<f32>,
  alignment: Option<TableAlignment>,
  layout: Option<TableLayoutMode>,
  whole_row: TableRowStyle,
  conditional_rows: Vec<(w::TableStyleOverrideValues, TableRowStyle)>,
  whole_table: TableCellStyle,
  conditional: Vec<(w::TableStyleOverrideValues, TableCellStyle)>,
}

#[derive(Clone, Copy, Debug, Default)]
struct TableRowStyle {
  height_pt: Option<f32>,
  exact_height: Option<bool>,
  repeat_header: Option<bool>,
  cant_split: Option<bool>,
  cell_spacing_pt: Option<f32>,
  width_before_pt: Option<f32>,
  width_after_pt: Option<f32>,
}

#[derive(Clone, Debug, Default)]
struct TableCellStyle {
  shading: Option<Option<RgbColor>>,
  borders: CellBordersModel,
  margins: Option<CellMargins>,
  vertical_alignment: Option<TableCellVerticalAlignment>,
  no_wrap: Option<bool>,
  paragraph_format: ParagraphFormat,
  run_style: TextStyle,
  run_overrides: RunStyleOverrides,
}

struct TableImportContext<'a> {
  styles: &'a StylesCatalog,
  numbering: &'a mut NumberingCatalog,
  images: &'a ImageCatalog,
  hyperlinks: &'a HyperlinkCatalog,
  custom_xml_bindings: &'a CustomXmlBindings,
  form_widget_ids: &'a mut FormWidgetIdAllocator,
  cell_margins: CellMargins,
  direct_cell_margins: bool,
  table_shading: Option<RgbColor>,
  table_borders: Option<TableBordersModel>,
  table_style: &'a TableStyleModel,
  table_look: TableLookModel,
  row_count: usize,
  nested_table_level: usize,
  in_header_footer: bool,
}

#[derive(Clone, Copy)]
struct TableModelContext {
  nested_table_level: usize,
  in_header_footer: bool,
}

struct TableModelEnv<'a> {
  styles: &'a StylesCatalog,
  numbering: &'a mut NumberingCatalog,
  images: &'a ImageCatalog,
  hyperlinks: &'a HyperlinkCatalog,
  custom_xml_bindings: &'a CustomXmlBindings,
  form_widget_ids: &'a mut FormWidgetIdAllocator,
}

impl StylesCatalog {
  fn load(
    package: &mut WordprocessingDocument,
    main: &MainDocumentPart,
    import_settings: ImportSettings,
    ui_language: Option<&str>,
  ) -> Result<Self> {
    let theme = ThemeData::load(package, main);
    let font_substitutions = load_font_substitutions(package, main);
    let cjk_punctuation_compression = theme.cjk_punctuation_compression;
    let Some(styles_part) = main.style_definitions_part(package) else {
      let mut catalog = Self {
        import_settings,
        simplified_chinese_ui: is_simplified_chinese_ui_language(ui_language),
        theme_fonts: theme.fonts,
        theme_colors: theme.colors,
        theme_lines: theme.lines,
        theme_effects: theme.effects,
        font_substitutions,
        ..Self::default()
      };
      catalog.doc_default_run.wordprocessingml_font_slots = true;
      catalog.doc_default_run.cjk_punctuation_compression_ratio = if cjk_punctuation_compression {
        1.0
      } else {
        0.0
      };
      // ECMA-376 Part 1 §17.3.2.19: when w:kern is never applied in the
      // style hierarchy, kerning is disabled for WordprocessingML runs.
      catalog.doc_default_run.kerning_minimum_size_pt = Some(f32::INFINITY);
      // [MS-DOCX] §2.3.32: in the absence of w14:ligatures, no ligatures
      // are used. This overrides HarfRust's native liga/clig defaults only
      // for WordprocessingML.
      catalog.doc_default_run.ligatures = Some(common::OpenTypeLigatures::default());
      if catalog.doc_default_run.font_family.is_none() {
        catalog.doc_default_run.font_family = Some(office_default_font_family(ui_language));
      }
      return Ok(catalog);
    };
    let styles = styles_part.root_element(package)?;
    let default_run_properties = styles
      .doc_defaults
      .as_deref()
      .and_then(|defaults| defaults.run_properties_default.as_deref())
      .and_then(|default| default.run_properties_base_style.as_deref());
    let has_default_run_properties = default_run_properties.is_some();
    let has_default_run_fonts = default_run_properties
      .and_then(|properties| properties.run_fonts.as_ref())
      .is_some();
    let has_default_paragraph_properties = styles
      .doc_defaults
      .as_deref()
      .and_then(|defaults| defaults.paragraph_properties_default.as_deref())
      .is_some();
    let mut catalog = Self {
      import_settings,
      simplified_chinese_ui: is_simplified_chinese_ui_language(ui_language),
      has_styles_part: true,
      has_default_paragraph_properties,
      doc_default_run: word_doc_default_run_seed(has_default_run_properties),
      theme_fonts: theme.fonts,
      theme_colors: theme.colors,
      theme_lines: theme.lines,
      theme_effects: theme.effects,
      font_substitutions,
      ..Self::default()
    };
    catalog.doc_default_run.kerning_minimum_size_pt = Some(f32::INFINITY);
    catalog.doc_default_run.wordprocessingml_font_slots = true;
    catalog.doc_default_run.cjk_punctuation_compression_ratio = if cjk_punctuation_compression {
      1.0
    } else {
      0.0
    };
    catalog.doc_default_run.ligatures = Some(common::OpenTypeLigatures::default());

    if let Some(defaults) = styles.doc_defaults.as_deref() {
      merge_paragraph_format(
        &mut catalog.doc_default_paragraph,
        defaults
          .paragraph_properties_default
          .as_deref()
          .and_then(|default| default.paragraph_properties_base_style.as_deref())
          .map(ParagraphProps::BaseStyle),
        catalog.import_settings,
      );
      properties::merge_doc_default_run_style(
        &mut catalog.doc_default_run,
        defaults
          .run_properties_default
          .as_deref()
          .and_then(|default| default.run_properties_base_style.as_deref()),
        &catalog.theme_fonts,
        &catalog.theme_colors,
      );
    }

    for style in &styles.style {
      let Some(style_id) = &style.style_id else {
        continue;
      };
      if matches!(style.r#type, Some(w::StyleValues::Paragraph))
        && style.default.is_some_and(|value| value.as_bool())
      {
        catalog.default_paragraph_style_id = Some(style_id.to_string());
      }
      if matches!(style.r#type, Some(w::StyleValues::Table))
        && style.default.is_some_and(|value| value.as_bool())
      {
        catalog.default_table_style_id = Some(style_id.to_string());
      }
      let mut entry = StyleEntry {
        style_type: style.r#type,
        custom_style: style
          .custom_style
          .is_some_and(ooxmlsdk::simple_type::OnOffValue::as_bool),
        name: style
          .style_name
          .as_ref()
          .map(|style_name| style_name.val.to_string()),
        based_on: style
          .based_on
          .as_ref()
          .map(|based_on| based_on.val.to_string()),
        paragraph_format: ParagraphFormat::default(),
        paragraph_numbering: None,
        run_style: TextStyle::default(),
        run_overrides: RunStyleOverrides::default(),
        table_style: TableStyleModel::default(),
      };
      merge_paragraph_format(
        &mut entry.paragraph_format,
        style
          .style_paragraph_properties
          .as_deref()
          .map(ParagraphProps::Style),
        catalog.import_settings,
      );
      entry.paragraph_numbering = style
        .style_paragraph_properties
        .as_ref()
        .and_then(|properties| properties.numbering_properties.clone());
      properties::merge_run_style(
        &mut entry.run_style,
        style.style_run_properties.as_deref().map(RunProps::Style),
        &catalog.theme_fonts,
        &catalog.theme_colors,
      );
      entry.run_overrides =
        run_style_overrides(style.style_run_properties.as_deref().map(RunProps::Style));
      normalize_relative_run_style(&mut entry.run_style, entry.run_overrides);
      entry.table_style = table_style_model(
        style,
        &catalog.theme_fonts,
        &catalog.theme_colors,
        catalog.import_settings,
      );
      catalog.styles.insert(style_id.to_string(), entry);
    }

    if catalog.doc_default_run.font_family.is_none() {
      catalog.doc_default_run.font_family = catalog
        .theme_fonts
        .minor_high_ansi
        .clone()
        .or_else(|| catalog.theme_fonts.minor_ascii.clone())
        // [MS-OI29500] §2.1.87(c) specifies Word's application default for
        // every missing slot in an existing w:rFonts context as Times New
        // Roman. Office applies that legacy default when rPrDefault exists
        // but supplies no font; if the entire run-default context is absent,
        // modern Word recovers the UI-specific default instead.
        .or_else(|| {
          Some(if has_default_run_properties && !has_default_run_fonts {
            Arc::from("Times New Roman")
          } else {
            office_default_font_family(ui_language)
          })
        });
    }

    Ok(catalog)
  }

  fn uses_office_recovered_paragraph_defaults(&self) -> bool {
    !self.has_styles_part || !self.has_default_paragraph_properties
  }

  fn paragraph_format_with_base(
    &self,
    style_id: Option<&str>,
    base_format: ParagraphFormat,
  ) -> ParagraphFormat {
    let mut format = self.doc_default_paragraph.clone();
    merge_format_values(&mut format, &base_format);
    let style_id = style_id.or(self.default_paragraph_style_id.as_deref());
    for entry in self.style_chain(style_id) {
      merge_format_values(&mut format, &entry.paragraph_format);
    }
    format
  }

  fn paragraph_numbering_reference(&self, style_id: Option<&str>) -> Option<NumberingReference> {
    let mut reference = NumberingReference::default();
    let style_id = style_id.or(self.default_paragraph_style_id.as_deref());
    for entry in self.style_chain(style_id) {
      if let Some(properties) = entry.paragraph_numbering.as_deref() {
        reference.merge_properties(properties);
      }
    }
    reference.resolved()
  }

  fn numbering_style_num_id(&self, style_id: &str) -> Option<i32> {
    let style = self.styles.get(style_id)?;
    if !matches!(style.style_type, Some(w::StyleValues::Numbering)) {
      return None;
    }
    self
      .paragraph_numbering_reference(Some(style_id))
      .map(NumberingReference::num_id)
  }

  fn numbering_matched_style_indent_context(
    &self,
    style_id: Option<&str>,
  ) -> NumberingFormatMergeContext {
    let style_id = style_id.or(self.default_paragraph_style_id.as_deref());
    let Some(format) = style_id
      .and_then(|style_id| self.styles.get(style_id))
      .map(|entry| &entry.paragraph_format)
    else {
      return NumberingFormatMergeContext::default();
    };
    NumberingFormatMergeContext {
      matched_style_indent_left: format.indent_left_set,
      matched_style_indent_right: format.indent_right_set,
      matched_style_first_line_indent: format.first_line_indent_set,
      ..Default::default()
    }
  }

  fn paragraph_indents_without_numbering(
    &self,
    style_id: Option<&str>,
  ) -> ((f32, Option<f32>), (f32, Option<f32>)) {
    let mut left = None;
    let mut first_line = None;
    let mut numbered_style_seen = false;
    let mut current = style_id.or(self.default_paragraph_style_id.as_deref());
    let mut visited = HashSet::new();
    while let Some(style_id) = current
      && visited.insert(style_id)
      && let Some(entry) = self.styles.get(style_id)
    {
      numbered_style_seen |= entry.paragraph_numbering.is_some();
      if left.is_none() && entry.paragraph_format.indent_left_set {
        left = Some(if numbered_style_seen {
          (0.0, None)
        } else {
          (
            entry.paragraph_format.indent_left_pt,
            entry.paragraph_format.indent_left_character_units,
          )
        });
      }
      if first_line.is_none() && entry.paragraph_format.first_line_indent_set {
        first_line = Some(if numbered_style_seen {
          (0.0, None)
        } else {
          (
            entry.paragraph_format.first_line_indent_pt,
            entry.paragraph_format.first_line_indent_character_units,
          )
        });
      }
      if left.is_some() && first_line.is_some() {
        break;
      }
      current = entry.based_on.as_deref();
    }
    (left.unwrap_or_default(), first_line.unwrap_or_default())
  }

  fn run_style_with_base(
    &self,
    style_id: Option<&str>,
    base_style: TextStyle,
    base_overrides: RunStyleOverrides,
  ) -> TextStyle {
    let mut style = self.doc_default_run.clone();
    merge_style_values(&mut style, &base_style);
    apply_run_style_overrides(&mut style, base_overrides);
    let mut vertical_alignment = base_overrides.vertical_alignment;
    let style_id = style_id.or(self.default_paragraph_style_id.as_deref());
    for entry in self.style_chain(style_id) {
      merge_style_values(&mut style, &entry.run_style);
      apply_run_style_overrides(&mut style, entry.run_overrides);
      if entry.run_overrides.vertical_alignment.is_some() {
        vertical_alignment = entry.run_overrides.vertical_alignment;
      }
    }
    if let Some(vertical_alignment) = vertical_alignment {
      properties::apply_vertical_text_alignment(&mut style, vertical_alignment);
    }
    self.apply_font_substitution(&mut style);
    style
  }

  fn character_run_style(&self, style_id: Option<&str>, base_style: TextStyle) -> TextStyle {
    let Some(style_id) = style_id else {
      return base_style;
    };
    let mut style = base_style;
    let mut matched = false;
    let mut vertical_alignment = None;
    for entry in self.style_chain(Some(style_id)) {
      if matches!(entry.style_type, Some(w::StyleValues::Character)) {
        matched = true;
        merge_style_values(&mut style, &entry.run_style);
        apply_run_style_overrides(&mut style, entry.run_overrides);
        if entry.run_overrides.vertical_alignment.is_some() {
          vertical_alignment = entry.run_overrides.vertical_alignment;
        }
      }
    }
    if !matched {
      merge_builtin_character_style(&mut style, style_id);
    }
    if let Some(vertical_alignment) = vertical_alignment {
      properties::apply_vertical_text_alignment(&mut style, vertical_alignment);
    }
    self.apply_font_substitution(&mut style);
    style
  }

  fn is_hyperlink_character_style(&self, style_id: &str) -> bool {
    style_id.eq_ignore_ascii_case("Hyperlink")
      || self.styles.get(style_id).is_some_and(|entry| {
        matches!(entry.style_type, Some(w::StyleValues::Character))
          && entry
            .name
            .as_deref()
            .is_some_and(|name| name.eq_ignore_ascii_case("Hyperlink"))
      })
  }

  fn is_toc_entry_paragraph_style(&self, style_id: Option<&str>) -> bool {
    let mut current = style_id;
    let mut visited = HashSet::<&str>::default();
    while let Some(id) = current
      && visited.insert(id)
    {
      if is_toc_entry_style_name(id) {
        return true;
      }
      let Some(entry) = self.styles.get(id) else {
        return false;
      };
      if matches!(entry.style_type, Some(w::StyleValues::Paragraph))
        && entry.name.as_deref().is_some_and(is_toc_entry_style_name)
      {
        return true;
      }
      current = entry.based_on.as_deref();
    }
    false
  }

  fn apply_font_substitution(&self, style: &mut TextStyle) {
    let Some(family) = style.font_family.as_deref() else {
      return;
    };
    let Some(substitution) = self.font_substitutions.get(&family.to_ascii_lowercase()) else {
      return;
    };
    if let Some(substitute) = &substitution.alternate_family
      && !family.eq_ignore_ascii_case(substitute)
    {
      style.fallback_font_family = Some(substitute.clone());
    }
    style.font_family_class = substitution.family_class;
  }

  fn apply_mapped_reserved_font(&self, style: &mut TextStyle, declared_family: Option<&str>) {
    let Some(family) = declared_family.map(str::trim) else {
      return;
    };
    // Some producers use the otherwise non-typeface token "Default" as a
    // document-scoped font-table key. Honor it only when that exact key has
    // authored w:altName/w:family metadata. Without the entry, retain the
    // inherited run face instead of turning "Default" into a global alias.
    if !family.eq_ignore_ascii_case("default")
      || !self
        .font_substitutions
        .contains_key(&family.to_ascii_lowercase())
    {
      return;
    }
    style.font_family = Some(Arc::from(family));
    style.fallback_font_family = None;
    style.font_family_class = None;
  }

  fn style_ref_name_requires_localized_error(&self, style_name: &str) -> bool {
    if !self.simplified_chinese_ui || matches!(style_name.trim().as_bytes(), [b'1'..=b'9']) {
      return false;
    }
    let target = normalized_style_ref_lookup_key(style_name);
    !self.styles.iter().any(|(style_id, entry)| {
      entry.custom_style
        && (normalized_style_ref_lookup_key(style_id) == target
          || entry
            .name
            .as_deref()
            .is_some_and(|name| normalized_style_ref_lookup_key(name) == target))
    })
  }

  fn style_ref_keys(&self, style_id: &str) -> Vec<Arc<str>> {
    let mut keys = Vec::new();
    push_unique_style_ref_key(&mut keys, style_id);
    if let Some(entry) = self.styles.get(style_id) {
      if let Some(name) = &entry.name {
        push_unique_style_ref_key(&mut keys, name);
        if entry.custom_style {
          push_unique_style_ref_key(&mut keys, &format!("{CUSTOM_STYLE_REF_KEY_PREFIX}{name}"));
        }
        if matches!(entry.style_type, Some(w::StyleValues::Paragraph)) {
          push_unique_style_ref_key(&mut keys, &format!("{name} Character"));
        }
      }
      if matches!(entry.style_type, Some(w::StyleValues::Paragraph)) {
        push_unique_style_ref_key(&mut keys, &format!("{style_id}Character"));
      }
    }
    keys
  }

  fn table_style(&self, style_id: Option<&str>) -> TableStyleModel {
    let mut style = TableStyleModel::default();
    let style_id = style_id.or(self.default_table_style_id.as_deref());
    for entry in self.style_chain(style_id) {
      if matches!(entry.style_type, Some(w::StyleValues::Table)) {
        merge_table_style_model(&mut style, &entry.table_style);
      }
    }
    style
  }

  fn style_chain<'a>(&'a self, style_id: Option<&'a str>) -> StyleChain<'a> {
    let mut ids = SmallVec::new();
    let mut current = style_id;
    while let Some(id) = current {
      if ids.contains(&id) {
        break;
      }
      let Some(entry) = self.styles.get(id) else {
        break;
      };
      ids.push(id);
      current = entry.based_on.as_deref();
    }

    StyleChain {
      styles: &self.styles,
      ids,
    }
  }
}

const CUSTOM_STYLE_REF_KEY_PREFIX: &str = "\0custom:";
const WORD_ZH_STYLE_REF_ERROR_LINE_HEIGHT_PER_FONT_SIZE: f32 = 34.0 / 25.0;

fn normalized_style_ref_lookup_key(name: &str) -> String {
  name
    .trim()
    .chars()
    .filter(|character| !character.is_whitespace() && !matches!(character, '-' | '_'))
    .flat_map(char::to_lowercase)
    .collect()
}

fn load_font_substitutions(
  package: &mut WordprocessingDocument,
  main: &MainDocumentPart,
) -> HashMap<String, FontSubstitution> {
  let Some(font_table_part) = main.font_table_part(package) else {
    return HashMap::new();
  };
  let Ok(fonts) = font_table_part.root_element(package) else {
    return HashMap::new();
  };
  fonts
    .xml_children
    .iter()
    .filter_map(|choice| {
      let w::FontsChoice::Font(font) = choice else {
        return None;
      };
      font_substitution_from_table_entry(font)
    })
    .collect()
}

fn font_substitution_from_table_entry(font: &w::Font) -> Option<(String, FontSubstitution)> {
  // ECMA-376 Part 1 §17.8.3.1 defines w:altName as a prioritized,
  // comma-delimited set used when the primary font is unavailable. The
  // current font request model accepts one document-scoped substitute, so
  // preserve the specified priority by selecting the first name.
  let authored_alternate_family = font
    .alt_name
    .as_ref()
    .and_then(|alternate| {
      alternate
        .val
        .as_str()
        .split(',')
        .map(str::trim)
        .find(|name| !name.is_empty())
    })
    .map(Arc::from);
  let unclassified_legacy_font = authored_alternate_family.is_none()
    && font
      .not_true_type
      .as_ref()
      .is_some_and(|value| value.val.is_none_or(|value| value.as_bool()))
    && font
      .font_family
      .as_ref()
      .is_some_and(|family| family.val == w::FontFamilyValues::Auto)
    && font
      .panose1_number
      .as_ref()
      .is_some_and(|panose| panose.val.chars().all(|digit| digit == '0'));
  // Word's fixed-output font mapper treats a non-TrueType font with neither
  // an authored alternate nor usable PANOSE/family classification as an
  // unresolved legacy Latin face. It falls back to Calibri rather than the
  // UI-language East Asian default or an arbitrary system font
  // (testPageref.docx).
  let alternate_family =
    authored_alternate_family.or_else(|| unclassified_legacy_font.then(|| Arc::from("Calibri")));
  let family_class = font
    .font_family
    .as_ref()
    .and_then(|family| match family.val {
      w::FontFamilyValues::Roman => Some(ooxmlsdk_fonts::FontFamilyClass::Serif),
      w::FontFamilyValues::Swiss => Some(ooxmlsdk_fonts::FontFamilyClass::SansSerif),
      w::FontFamilyValues::Modern => Some(ooxmlsdk_fonts::FontFamilyClass::Fixed),
      w::FontFamilyValues::Decorative => Some(ooxmlsdk_fonts::FontFamilyClass::Decorative),
      w::FontFamilyValues::Script => Some(ooxmlsdk_fonts::FontFamilyClass::BrushScript),
      w::FontFamilyValues::Auto => None,
    });
  let family = font.name.as_str().trim();
  (!family.is_empty() && (alternate_family.is_some() || family_class.is_some())).then(|| {
    (
      family.to_ascii_lowercase(),
      FontSubstitution {
        alternate_family,
        family_class,
      },
    )
  })
}

fn office_default_font_family(ui_language: Option<&str>) -> Arc<str> {
  // Microsoft documents DengXian as the Office 2016+ default font for the
  // Simplified Chinese editions of Word, Excel, and PowerPoint. Keep this
  // fallback below explicit document styles and theme fonts.
  if is_simplified_chinese_ui_language(ui_language) {
    Arc::from("DengXian")
  } else {
    Arc::from("Calibri")
  }
}

fn word_doc_default_run_seed(has_default_run_properties: bool) -> TextStyle {
  let mut style = TextStyle::default();
  if has_default_run_properties {
    // StyleSheetTable seeds all three character-height slots to 10pt once an
    // authored w:rPrDefault exists. The Calibri 11pt application recovery is
    // only for documents that omit that context entirely (tdf#108350).
    style.font_size_pt = 10.0;
    style.complex_font_size_pt = Some(10.0);
  }
  style
}

fn is_simplified_chinese_ui_language(ui_language: Option<&str>) -> bool {
  let language = ui_language.unwrap_or_default().to_ascii_lowercase();
  language == "zh-cn"
    || language == "zh-sg"
    || language == "zh-hans"
    || language.starts_with("zh-hans-")
}

struct StyleChain<'a> {
  styles: &'a HashMap<String, StyleEntry>,
  ids: SmallVec<[&'a str; 4]>,
}

impl<'a> IntoIterator for StyleChain<'a> {
  type Item = &'a StyleEntry;
  type IntoIter = StyleChainIter<'a>;

  fn into_iter(self) -> Self::IntoIter {
    StyleChainIter {
      styles: self.styles,
      ids: self.ids,
    }
  }
}

struct StyleChainIter<'a> {
  styles: &'a HashMap<String, StyleEntry>,
  ids: SmallVec<[&'a str; 4]>,
}

impl<'a> Iterator for StyleChainIter<'a> {
  type Item = &'a StyleEntry;

  fn next(&mut self) -> Option<Self::Item> {
    while let Some(id) = self.ids.pop() {
      if let Some(entry) = self.styles.get(id) {
        return Some(entry);
      }
    }
    None
  }
}

impl ThemeData {
  fn load(package: &mut WordprocessingDocument, main: &MainDocumentPart) -> Self {
    let settings = main
      .document_settings_part(package)
      .and_then(|part| part.root_element(package).ok());
    let theme_font_languages = settings
      .as_ref()
      .and_then(|settings| settings.theme_font_languages.as_ref())
      .map(|languages| {
        (
          languages
            .val
            .as_ref()
            .map(|value| Arc::<str>::from(value.to_string())),
          languages
            .east_asia
            .as_ref()
            .map(|value| Arc::<str>::from(value.to_string())),
          languages
            .bidi
            .as_ref()
            .map(|value| Arc::<str>::from(value.to_string())),
        )
      });
    let cjk_punctuation_compression = settings
      .as_ref()
      .and_then(|settings| settings.character_spacing_control.as_ref())
      .is_some_and(|control| {
        matches!(
          control.val,
          w::CharacterSpacingValues::CompressPunctuation
            | w::CharacterSpacingValues::CompressPunctuationAndJapaneseKana
        )
      });
    let Some(theme) = main
      .theme_part(package)
      .and_then(|part| part.root_element(package).ok())
    else {
      return Self {
        cjk_punctuation_compression,
        ..Self::default()
      };
    };
    Self {
      fonts: ThemeFonts::from_theme(theme, theme_font_languages),
      colors: ThemeColors::from_theme(theme),
      lines: ThemeLineStyles::from_theme(theme),
      effects: ThemeEffectStyles::from_theme(theme),
      cjk_punctuation_compression,
    }
  }
}

impl ThemeFonts {
  fn from_theme(theme: &a::Theme, languages: Option<ThemeFontLanguages>) -> Self {
    let scheme = &theme.theme_elements.font_scheme;
    let (latin_language, east_asia_language, bidi_language) = languages.unwrap_or_default();
    Self {
      major_ascii: major_font_family(&scheme.major_font.latin_font.typeface),
      major_high_ansi: major_font_family(&scheme.major_font.latin_font.typeface),
      major_east_asia: major_font_family(&scheme.major_font.east_asian_font.typeface),
      major_bidi: major_font_family(&scheme.major_font.complex_script_font.typeface),
      minor_ascii: major_font_family(&scheme.minor_font.latin_font.typeface),
      minor_high_ansi: major_font_family(&scheme.minor_font.latin_font.typeface),
      minor_east_asia: major_font_family(&scheme.minor_font.east_asian_font.typeface),
      minor_bidi: major_font_family(&scheme.minor_font.complex_script_font.typeface),
      major_supplemental: theme_supplemental_fonts(&scheme.major_font.supplemental_font),
      minor_supplemental: theme_supplemental_fonts(&scheme.minor_font.supplemental_font),
      latin_language,
      east_asia_language,
      bidi_language,
    }
  }

  fn resolve(&self, value: Option<w::ThemeFontValues>) -> Option<Arc<str>> {
    // ECMA-376 Part 1 §17.15.1.88 maps Word's major/minor theme tokens
    // through w:themeFontLang, and §§20.1.4.1.16/L.4.3.2.6 define the
    // script-specific supplemental faces in the DrawingML theme.
    match value? {
      w::ThemeFontValues::MajorAscii => self
        .supplemental(&self.major_supplemental, self.latin_language.as_deref())
        .or_else(|| self.major_ascii.clone()),
      w::ThemeFontValues::MajorHighAnsi => self
        .supplemental(&self.major_supplemental, self.latin_language.as_deref())
        .or_else(|| self.major_high_ansi.clone()),
      w::ThemeFontValues::MajorEastAsia => self
        .supplemental(&self.major_supplemental, self.east_asia_language.as_deref())
        .or_else(|| self.major_east_asia.clone()),
      w::ThemeFontValues::MajorBidi => self
        .supplemental(&self.major_supplemental, self.bidi_language.as_deref())
        .or_else(|| self.major_bidi.clone()),
      w::ThemeFontValues::MinorAscii => self
        .supplemental(&self.minor_supplemental, self.latin_language.as_deref())
        .or_else(|| self.minor_ascii.clone()),
      w::ThemeFontValues::MinorHighAnsi => self
        .supplemental(&self.minor_supplemental, self.latin_language.as_deref())
        .or_else(|| self.minor_high_ansi.clone()),
      w::ThemeFontValues::MinorEastAsia => self
        .supplemental(&self.minor_supplemental, self.east_asia_language.as_deref())
        .or_else(|| self.minor_east_asia.clone()),
      w::ThemeFontValues::MinorBidi => self
        .supplemental(&self.minor_supplemental, self.bidi_language.as_deref())
        .or_else(|| self.minor_bidi.clone()),
    }
  }

  fn resolve_drawingml_typeface(&self, typeface: &str) -> Arc<str> {
    self.resolve_drawingml_typeface_for_language(typeface, None)
  }

  fn resolve_drawingml_typeface_for_language(
    &self,
    typeface: &str,
    fallback_east_asia_language: Option<&str>,
  ) -> Arc<str> {
    match typeface {
      "+mj-lt" | "majorHAnsi" | "majorAscii" => self
        .supplemental(&self.major_supplemental, self.latin_language.as_deref())
        .or_else(|| self.major_high_ansi.clone())
        .or_else(|| self.major_ascii.clone()),
      "+mn-lt" | "minorHAnsi" | "minorAscii" => self
        .supplemental(&self.minor_supplemental, self.latin_language.as_deref())
        .or_else(|| self.minor_high_ansi.clone())
        .or_else(|| self.minor_ascii.clone()),
      "+mj-ea" | "majorEastAsia" => self
        .supplemental(
          &self.major_supplemental,
          self
            .east_asia_language
            .as_deref()
            .or(fallback_east_asia_language),
        )
        .or_else(|| self.major_east_asia.clone()),
      "+mn-ea" | "minorEastAsia" => self
        .supplemental(
          &self.minor_supplemental,
          self
            .east_asia_language
            .as_deref()
            .or(fallback_east_asia_language),
        )
        .or_else(|| self.minor_east_asia.clone()),
      "+mj-cs" | "majorBidi" => self
        .supplemental(&self.major_supplemental, self.bidi_language.as_deref())
        .or_else(|| self.major_bidi.clone()),
      "+mn-cs" | "minorBidi" => self
        .supplemental(&self.minor_supplemental, self.bidi_language.as_deref())
        .or_else(|| self.minor_bidi.clone()),
      _ => None,
    }
    .unwrap_or_else(|| Arc::from(typeface))
  }

  fn supplemental(
    &self,
    fonts: &[(Arc<str>, Arc<str>)],
    language: Option<&str>,
  ) -> Option<Arc<str>> {
    let script = language.and_then(theme_language_script)?;
    fonts
      .iter()
      .find(|(candidate, _)| candidate.eq_ignore_ascii_case(script))
      .map(|(_, typeface)| Arc::clone(typeface))
  }
}

fn theme_supplemental_fonts(fonts: &[a::SupplementalFont]) -> Vec<(Arc<str>, Arc<str>)> {
  fonts
    .iter()
    .filter(|font| !font.script.is_empty() && !font.typeface.is_empty())
    .map(|font| {
      (
        Arc::<str>::from(font.script.as_str()),
        Arc::<str>::from(font.typeface.as_str()),
      )
    })
    .collect()
}

fn theme_language_script(language: &str) -> Option<&'static str> {
  let language = language.to_ascii_lowercase();
  if language == "zh-hant"
    || language.starts_with("zh-hant-")
    || language == "zh-tw"
    || language == "zh-hk"
    || language == "zh-mo"
  {
    Some("Hant")
  } else if language == "zh"
    || language.starts_with("zh-hans")
    || language == "zh-cn"
    || language == "zh-sg"
  {
    Some("Hans")
  } else if language == "ja" || language.starts_with("ja-") {
    Some("Jpan")
  } else if language == "ko" || language.starts_with("ko-") {
    Some("Hang")
  } else if language == "ar" || language.starts_with("ar-") {
    Some("Arab")
  } else if language == "he" || language.starts_with("he-") {
    Some("Hebr")
  } else if language == "th" || language.starts_with("th-") {
    Some("Thai")
  } else {
    None
  }
}

impl ThemeLineStyles {
  fn from_theme(theme: &a::Theme) -> Self {
    Self {
      widths_pt: theme
        .theme_elements
        .format_scheme
        .line_style_list
        .outline
        .iter()
        .filter_map(|line| line.width.map(|width| units::emu_to_points(width as i64)))
        .collect(),
    }
  }
}

impl ThemeEffectStyles {
  fn from_theme(theme: &a::Theme) -> Self {
    Self {
      styles: theme
        .theme_elements
        .format_scheme
        .effect_style_list
        .effect_style
        .clone(),
    }
  }
}

impl ThemeColors {
  fn from_theme(theme: &a::Theme) -> Self {
    let scheme = &theme.theme_elements.color_scheme;
    Self {
      dark1: dark1_color_value(&scheme.dark1_color.dark1_color_choice),
      light1: light1_color_value(&scheme.light1_color.light1_color_choice),
      dark2: dark2_color_value(&scheme.dark2_color.dark2_color_choice),
      light2: light2_color_value(&scheme.light2_color.light2_color_choice),
      accent1: accent1_color_value(&scheme.accent1_color.accent1_color_choice),
      accent2: accent2_color_value(&scheme.accent2_color.accent2_color_choice),
      accent3: accent3_color_value(&scheme.accent3_color.accent3_color_choice),
      accent4: accent4_color_value(&scheme.accent4_color.accent4_color_choice),
      accent5: accent5_color_value(&scheme.accent5_color.accent5_color_choice),
      accent6: accent6_color_value(&scheme.accent6_color.accent6_color_choice),
      hyperlink: hyperlink_color_value(&scheme.hyperlink.hyperlink_choice),
      followed_hyperlink: followed_hyperlink_color_value(
        &scheme
          .followed_hyperlink_color
          .followed_hyperlink_color_choice,
      ),
    }
  }

  fn resolve_wordprocessing(&self, value: w::ThemeColorValues) -> Option<RgbColor> {
    match value {
      w::ThemeColorValues::Dark1 | w::ThemeColorValues::Text1 => self.dark1,
      w::ThemeColorValues::Light1 | w::ThemeColorValues::Background1 => self.light1,
      w::ThemeColorValues::Dark2 | w::ThemeColorValues::Text2 => self.dark2,
      w::ThemeColorValues::Light2 | w::ThemeColorValues::Background2 => self.light2,
      w::ThemeColorValues::Accent1 => self.accent1,
      w::ThemeColorValues::Accent2 => self.accent2,
      w::ThemeColorValues::Accent3 => self.accent3,
      w::ThemeColorValues::Accent4 => self.accent4,
      w::ThemeColorValues::Accent5 => self.accent5,
      w::ThemeColorValues::Accent6 => self.accent6,
      w::ThemeColorValues::Hyperlink => self.hyperlink,
      w::ThemeColorValues::FollowedHyperlink => self.followed_hyperlink,
      w::ThemeColorValues::None => None,
    }
  }

  fn resolve_word2010(&self, value: w14::SchemeColorValues) -> Option<RgbColor> {
    match value {
      w14::SchemeColorValues::BackgroundColor => self.light1,
      w14::SchemeColorValues::TextColor => self.dark1,
      w14::SchemeColorValues::AdditionalBackgroundColor => self.light2,
      w14::SchemeColorValues::AdditionalTextColor => self.dark2,
      w14::SchemeColorValues::ExtraSchemeColor1 => self.accent1,
      w14::SchemeColorValues::ExtraSchemeColor2 => self.accent2,
      w14::SchemeColorValues::ExtraSchemeColor3 => self.accent3,
      w14::SchemeColorValues::ExtraSchemeColor4 => self.accent4,
      w14::SchemeColorValues::ExtraSchemeColor5 => self.accent5,
      w14::SchemeColorValues::ExtraSchemeColor6 => self.accent6,
      w14::SchemeColorValues::HyperlinkColor => self.hyperlink,
      w14::SchemeColorValues::FollowedHyperlinkColor => self.followed_hyperlink,
      w14::SchemeColorValues::MainDarkColor1 => self.dark1,
      w14::SchemeColorValues::MainLightColor1 => self.light1,
      w14::SchemeColorValues::MainDarkColor2 => self.dark2,
      w14::SchemeColorValues::MainLightColor2 => self.light2,
      w14::SchemeColorValues::AutoColor => None,
    }
  }
}

fn major_font_family(value: &Option<String>) -> Option<Arc<str>> {
  value
    .as_deref()
    .map(str::trim)
    .filter(|value| !value.is_empty())
    .map(Arc::<str>::from)
}

macro_rules! theme_color_choice_value {
  ($fn_name:ident, $choice_ty:path, $srgb:path, $sys:path) => {
    fn $fn_name(choice: &Option<$choice_ty>) -> Option<RgbColor> {
      match choice.as_ref()? {
        $srgb(color) => parse_hex_color(color.val.as_str()),
        $sys(color) => color.last_color.as_deref().and_then(parse_hex_color),
        _ => None,
      }
    }
  };
}

theme_color_choice_value!(
  dark1_color_value,
  a::Dark1ColorChoice,
  a::Dark1ColorChoice::RgbColorModelHex,
  a::Dark1ColorChoice::SystemColor
);
theme_color_choice_value!(
  light1_color_value,
  a::Light1ColorChoice,
  a::Light1ColorChoice::RgbColorModelHex,
  a::Light1ColorChoice::SystemColor
);
theme_color_choice_value!(
  dark2_color_value,
  a::Dark2ColorChoice,
  a::Dark2ColorChoice::RgbColorModelHex,
  a::Dark2ColorChoice::SystemColor
);
theme_color_choice_value!(
  light2_color_value,
  a::Light2ColorChoice,
  a::Light2ColorChoice::RgbColorModelHex,
  a::Light2ColorChoice::SystemColor
);
theme_color_choice_value!(
  accent1_color_value,
  a::Accent1ColorChoice,
  a::Accent1ColorChoice::RgbColorModelHex,
  a::Accent1ColorChoice::SystemColor
);
theme_color_choice_value!(
  accent2_color_value,
  a::Accent2ColorChoice,
  a::Accent2ColorChoice::RgbColorModelHex,
  a::Accent2ColorChoice::SystemColor
);
theme_color_choice_value!(
  accent3_color_value,
  a::Accent3ColorChoice,
  a::Accent3ColorChoice::RgbColorModelHex,
  a::Accent3ColorChoice::SystemColor
);
theme_color_choice_value!(
  accent4_color_value,
  a::Accent4ColorChoice,
  a::Accent4ColorChoice::RgbColorModelHex,
  a::Accent4ColorChoice::SystemColor
);
theme_color_choice_value!(
  accent5_color_value,
  a::Accent5ColorChoice,
  a::Accent5ColorChoice::RgbColorModelHex,
  a::Accent5ColorChoice::SystemColor
);
theme_color_choice_value!(
  accent6_color_value,
  a::Accent6ColorChoice,
  a::Accent6ColorChoice::RgbColorModelHex,
  a::Accent6ColorChoice::SystemColor
);
theme_color_choice_value!(
  hyperlink_color_value,
  a::HyperlinkChoice,
  a::HyperlinkChoice::RgbColorModelHex,
  a::HyperlinkChoice::SystemColor
);
theme_color_choice_value!(
  followed_hyperlink_color_value,
  a::FollowedHyperlinkColorChoice,
  a::FollowedHyperlinkColorChoice::RgbColorModelHex,
  a::FollowedHyperlinkColorChoice::SystemColor
);

pub(super) fn resolve_run_color(color: &w::Color, theme_colors: &ThemeColors) -> Option<RgbColor> {
  if color.theme_shade.is_some()
    && let Some(resolved) = color.val.as_deref().and_then(parse_hex_color)
  {
    return Some(resolved);
  }

  let has_theme_transform = color.theme_tint.is_some() || color.theme_shade.is_some();

  if !has_theme_transform && let Some(resolved) = color.val.as_deref().and_then(parse_hex_color) {
    return Some(resolved);
  }

  let mut resolved = color
    .theme_color
    .and_then(|value| theme_colors.resolve_wordprocessing(value))
    .or_else(|| color.val.as_deref().and_then(parse_hex_color))?;

  if let Some(tint) = color.theme_tint.as_deref() {
    resolved = apply_word_tint(resolved, tint);
  }
  if let Some(shade) = color.theme_shade.as_deref() {
    resolved = apply_word_shade(resolved, shade);
  }

  Some(resolved)
}

pub(super) fn resolve_text_fill(
  fill: &w14::FillTextEffect,
  theme_colors: &ThemeColors,
) -> Option<ResolvedColor> {
  match fill.fill_text_effect_choice.as_ref()? {
    w14::FillTextEffectChoice::NoFillEmpty => None,
    w14::FillTextEffectChoice::SolidColorFillProperties(fill) => {
      resolve_solid_text_fill(fill, theme_colors)
    }
    w14::FillTextEffectChoice::GradientFillProperties(_) => None,
  }
}

pub(super) fn resolve_text_outline(
  outline: &w14::TextOutlineEffect,
  theme_colors: &ThemeColors,
) -> Option<ResolvedColor> {
  let resolved = match outline.text_outline_effect_choice1.as_ref()? {
    w14::TextOutlineEffectChoice::NoFillEmpty => return None,
    w14::TextOutlineEffectChoice::SolidColorFillProperties(fill) => {
      resolve_solid_text_fill(fill, theme_colors)?
    }
    w14::TextOutlineEffectChoice::GradientFillProperties(_) => return None,
  };

  Some(ResolvedColor {
    color: resolved.color,
    opacity: resolved.opacity,
  })
}

fn resolve_solid_text_fill(
  fill: &w14::SolidColorFillProperties,
  theme_colors: &ThemeColors,
) -> Option<ResolvedColor> {
  match fill.solid_color_fill_properties_choice.as_ref()? {
    w14::SolidColorFillPropertiesChoice::RgbColorModelHex(color) => Some(ResolvedColor {
      color: parse_hex_color(color.val.as_str())?,
      opacity: opacity_from_w14_rgb_transforms(&color.rgb_color_model_hex_choice),
    }),
    w14::SolidColorFillPropertiesChoice::SchemeColor(color) => {
      let mut resolved = theme_colors.resolve_word2010(color.val)?;
      resolved = apply_w14_scheme_transforms(resolved, &color.scheme_color_choice);
      Some(ResolvedColor {
        color: resolved,
        opacity: opacity_from_w14_scheme_transforms(&color.scheme_color_choice),
      })
    }
  }
}

fn opacity_from_w14_rgb_transforms(transforms: &[w14::RgbColorModelHexChoice]) -> f32 {
  opacity_from_w14_alpha(transforms.iter().find_map(|transform| match transform {
    w14::RgbColorModelHexChoice::Alpha(value) => Some(value.val),
    _ => None,
  }))
}

fn opacity_from_w14_scheme_transforms(transforms: &[w14::SchemeColorChoice]) -> f32 {
  opacity_from_w14_alpha(transforms.iter().find_map(|transform| match transform {
    w14::SchemeColorChoice::Alpha(value) => Some(value.val),
    _ => None,
  }))
}

fn opacity_from_w14_alpha(alpha: Option<i32>) -> f32 {
  let transparency = sdk_units::drawingml_percent_to_ratio(alpha.unwrap_or(0)) as f32;
  (1.0 - transparency).clamp(0.0, 1.0)
}

fn apply_w14_scheme_transforms(color: RgbColor, transforms: &[w14::SchemeColorChoice]) -> RgbColor {
  let mut hsl = hsl_color(color);
  for transform in transforms {
    match transform {
      w14::SchemeColorChoice::Tint(value) => {
        hsl.apply_tint(sdk_units::drawingml_percent_to_ratio(value.val) as f32);
      }
      w14::SchemeColorChoice::Shade(value) => {
        hsl.apply_shade(sdk_units::drawingml_percent_to_ratio(value.val) as f32);
      }
      w14::SchemeColorChoice::LuminanceModulation(value) => {
        hsl.apply_luminance_mod(sdk_units::drawingml_percent_to_ratio(value.val) as f32);
      }
      w14::SchemeColorChoice::LuminanceOffset(value) => {
        hsl.apply_luminance_offset(sdk_units::drawingml_percent_to_ratio(value.val) as f32);
      }
      _ => {}
    }
  }
  rgb_color(hsl)
}

fn apply_word_tint(color: RgbColor, tint: &str) -> RgbColor {
  let Some(tint) = u8::from_str_radix(tint, 16).ok() else {
    return color;
  };
  let mut hsl = hsl_color(color);
  hsl.apply_tint(1.0 - (tint as f32 / units::BYTE_MAX_AS_FLOAT));
  rgb_color(hsl)
}

fn apply_word_shade(color: RgbColor, shade: &str) -> RgbColor {
  let Some(shade) = u8::from_str_radix(shade, 16).ok() else {
    return color;
  };
  let mut hsl = hsl_color(color);
  hsl.apply_shade(shade as f32 / units::BYTE_MAX_AS_FLOAT);
  rgb_color(hsl)
}

fn hsl_color(color: RgbColor) -> color_math::HslColor {
  color_math::HslColor::from_srgb8([color.r, color.g, color.b])
}

fn rgb_color(color: color_math::HslColor) -> RgbColor {
  let [r, g, b] = color.to_srgb8();
  RgbColor { r, g, b }
}

fn table_style_model(
  style: &w::Style,
  theme_fonts: &ThemeFonts,
  theme_colors: &ThemeColors,
  import_settings: ImportSettings,
) -> TableStyleModel {
  let mut model = TableStyleModel::default();
  if let Some(properties) = style.style_table_properties.as_deref() {
    merge_table_level_style(
      &mut model,
      &style_table_level_style(
        properties.table_borders.as_deref(),
        properties.table_cell_margin_default.as_deref(),
        properties.table_cell_spacing.as_ref(),
        properties.table_indentation.as_ref(),
        properties.table_justification.as_ref(),
        properties.table_layout.as_ref(),
        properties.shading.as_ref(),
      ),
    );
  }
  if let Some(properties) = style.style_table_cell_properties.as_deref() {
    model.whole_table = style_table_cell_style(properties);
  }
  if let Some(properties) = style
    .table_style_conditional_formatting_table_row_properties
    .as_ref()
  {
    model.whole_row = style_table_row_style(properties);
  }
  merge_paragraph_format(
    &mut model.whole_table.paragraph_format,
    style
      .style_paragraph_properties
      .as_deref()
      .map(ParagraphProps::Style),
    import_settings,
  );
  properties::merge_run_style(
    &mut model.whole_table.run_style,
    style.style_run_properties.as_deref().map(RunProps::Style),
    theme_fonts,
    theme_colors,
  );
  model.whole_table.run_overrides =
    run_style_overrides(style.style_run_properties.as_deref().map(RunProps::Style));
  normalize_relative_run_style(
    &mut model.whole_table.run_style,
    model.whole_table.run_overrides,
  );
  for conditional in &style.table_style_properties {
    let mut cell_style = TableCellStyle::default();
    merge_paragraph_format(
      &mut cell_style.paragraph_format,
      conditional
        .style_paragraph_properties
        .as_deref()
        .map(ParagraphProps::Style),
      import_settings,
    );
    properties::merge_run_style(
      &mut cell_style.run_style,
      conditional
        .run_properties_base_style
        .as_deref()
        .map(RunProps::BaseStyle),
      theme_fonts,
      theme_colors,
    );
    cell_style.run_overrides = run_style_overrides(
      conditional
        .run_properties_base_style
        .as_deref()
        .map(RunProps::BaseStyle),
    );
    normalize_relative_run_style(&mut cell_style.run_style, cell_style.run_overrides);
    if let Some(properties) = conditional
      .table_style_conditional_formatting_table_properties
      .as_deref()
    {
      if let Some(shading) = properties.shading.as_ref() {
        cell_style.shading = Some(shading_fill(shading));
      }
      merge_table_level_style(&mut model, &conditional_table_level_style(properties));
    }
    if let Some(properties) = conditional
      .table_style_conditional_formatting_table_row_properties
      .as_ref()
    {
      model
        .conditional_rows
        .push((conditional.r#type, style_table_row_style(properties)));
    }
    if let Some(properties) = conditional
      .table_style_conditional_formatting_table_cell_properties
      .as_deref()
    {
      merge_table_cell_style(&mut cell_style, &conditional_table_cell_style(properties));
    }
    model.conditional.push((conditional.r#type, cell_style));
  }
  model
}

fn style_table_cell_style(properties: &w::StyleTableCellProperties) -> TableCellStyle {
  TableCellStyle {
    shading: properties.shading.as_ref().map(shading_fill),
    borders: CellBordersModel::default(),
    margins: properties
      .table_cell_margin
      .as_deref()
      .map(|margins| table_cell_margin(margins, CellMargins::default())),
    vertical_alignment: properties
      .table_cell_vertical_alignment
      .as_ref()
      .map(table_cell_vertical_alignment),
    no_wrap: properties
      .no_wrap
      .as_ref()
      .map(|no_wrap| on_off_only_value(no_wrap.val)),
    ..Default::default()
  }
}

fn conditional_table_cell_style(
  properties: &w::TableStyleConditionalFormattingTableCellProperties,
) -> TableCellStyle {
  TableCellStyle {
    shading: properties.shading.as_ref().map(shading_fill),
    borders: properties
      .table_cell_borders
      .as_deref()
      .map(cell_borders_model)
      .unwrap_or_default(),
    margins: properties
      .table_cell_margin
      .as_deref()
      .map(|margins| table_cell_margin(margins, CellMargins::default())),
    vertical_alignment: properties
      .table_cell_vertical_alignment
      .as_ref()
      .map(table_cell_vertical_alignment),
    no_wrap: properties
      .no_wrap
      .as_ref()
      .map(|no_wrap| on_off_only_value(no_wrap.val)),
    ..Default::default()
  }
}

fn merge_table_style_model(target: &mut TableStyleModel, source: &TableStyleModel) {
  merge_table_level_style(target, source);
  merge_table_row_style(&mut target.whole_row, &source.whole_row);
  target
    .conditional_rows
    .extend(source.conditional_rows.iter().copied());
  merge_table_cell_style(&mut target.whole_table, &source.whole_table);
  target
    .conditional
    .extend(source.conditional.iter().cloned());
}

fn style_table_level_style(
  borders: Option<&w::TableBorders>,
  margins: Option<&w::TableCellMarginDefault>,
  spacing: Option<&w::TableCellSpacing>,
  indentation: Option<&w::TableIndentation>,
  justification: Option<&w::TableJustification>,
  layout: Option<&w::TableLayout>,
  shading: Option<&w::Shading>,
) -> TableStyleModel {
  TableStyleModel {
    table_borders: borders.map(table_borders_model),
    table_shading: shading.map(shading_fill),
    cell_margins: margins.map(table_cell_margin_default),
    cell_spacing_pt: spacing.and_then(table_cell_spacing_to_points),
    indent_left_pt: indentation.and_then(table_indentation_to_points),
    alignment: justification.map(table_alignment),
    layout: layout.map(table_layout_mode),
    ..Default::default()
  }
}

fn conditional_table_level_style(
  properties: &w::TableStyleConditionalFormattingTableProperties,
) -> TableStyleModel {
  style_table_level_style(
    properties.table_borders.as_deref(),
    properties.table_cell_margin_default.as_deref(),
    properties.table_cell_spacing.as_ref(),
    properties.table_indentation.as_ref(),
    properties.table_justification.as_ref(),
    None,
    None,
  )
}

fn merge_table_level_style(target: &mut TableStyleModel, source: &TableStyleModel) {
  if source.table_borders.is_some() {
    target.table_borders = source.table_borders;
  }
  if source.table_shading.is_some() {
    target.table_shading = source.table_shading;
  }
  if source.cell_margins.is_some() {
    target.cell_margins = source.cell_margins;
  }
  if source.cell_spacing_pt.is_some() {
    target.cell_spacing_pt = source.cell_spacing_pt;
  }
  if source.indent_left_pt.is_some() {
    target.indent_left_pt = source.indent_left_pt;
  }
  if source.alignment.is_some() {
    target.alignment = source.alignment;
  }
  if source.layout.is_some() {
    target.layout = source.layout;
  }
}

fn direct_table_row_style(properties: Option<&w::TableRowProperties>) -> TableRowStyle {
  let Some(properties) = properties else {
    return TableRowStyle::default();
  };
  let mut style = TableRowStyle::default();
  for choice in &properties.table_row_properties_choice1 {
    match choice {
      w::TableRowPropertiesChoice::TableRowHeight(height) => {
        apply_table_row_height(&mut style, height);
      }
      w::TableRowPropertiesChoice::TableHeader(header) => {
        style.repeat_header = Some(on_off_only_value(header.val));
      }
      w::TableRowPropertiesChoice::CantSplit(cant_split) => {
        style.cant_split = Some(on_off_only_value(cant_split.val));
      }
      w::TableRowPropertiesChoice::TableCellSpacing(spacing) => {
        style.cell_spacing_pt = table_cell_spacing_to_points(spacing);
      }
      w::TableRowPropertiesChoice::WidthBeforeTableRow(width) => {
        style.width_before_pt = row_width_to_points(width.width.as_ref(), width.r#type);
      }
      w::TableRowPropertiesChoice::WidthAfterTableRow(width) => {
        style.width_after_pt = row_width_to_points(width.width.as_ref(), width.r#type);
      }
      _ => {}
    }
  }
  style
}

fn style_table_row_style(
  properties: &w::TableStyleConditionalFormattingTableRowProperties,
) -> TableRowStyle {
  let mut style = TableRowStyle::default();
  for choice in &properties.table_style_conditional_formatting_table_row_properties_choice {
    match choice {
      w::TableStyleConditionalFormattingTableRowPropertiesChoice::TableHeader(header) => {
        style.repeat_header = Some(on_off_only_value(header.val));
      }
      w::TableStyleConditionalFormattingTableRowPropertiesChoice::CantSplit(cant_split) => {
        style.cant_split = Some(on_off_only_value(cant_split.val));
      }
      w::TableStyleConditionalFormattingTableRowPropertiesChoice::TableCellSpacing(spacing) => {
        style.cell_spacing_pt = table_cell_spacing_to_points(spacing);
      }
      w::TableStyleConditionalFormattingTableRowPropertiesChoice::WidthBeforeTableRow(width) => {
        style.width_before_pt = row_width_to_points(width.width.as_ref(), width.r#type);
      }
      w::TableStyleConditionalFormattingTableRowPropertiesChoice::WidthAfterTableRow(width) => {
        style.width_after_pt = row_width_to_points(width.width.as_ref(), width.r#type);
      }
      _ => {}
    }
  }
  style
}

fn apply_table_row_height(style: &mut TableRowStyle, height: &w::TableRowHeight) {
  style.height_pt = height.val.as_ref().and_then(twips_measure_to_points);
  style.exact_height = Some(matches!(
    height.height_type,
    Some(w::HeightRuleValues::Exact)
  ));
}

fn on_off_only_value(value: Option<ooxmlsdk::simple_type::OnOffValue>) -> bool {
  value.is_none_or(|value| value.as_bool())
}

fn merge_table_row_style(target: &mut TableRowStyle, source: &TableRowStyle) {
  if source.height_pt.is_some() {
    target.height_pt = source.height_pt;
  }
  if source.exact_height.is_some() {
    target.exact_height = source.exact_height;
  }
  if source.repeat_header.is_some() {
    target.repeat_header = source.repeat_header;
  }
  if source.cant_split.is_some() {
    target.cant_split = source.cant_split;
  }
  if source.cell_spacing_pt.is_some() {
    target.cell_spacing_pt = source.cell_spacing_pt;
  }
  if source.width_before_pt.is_some() {
    target.width_before_pt = source.width_before_pt;
  }
  if source.width_after_pt.is_some() {
    target.width_after_pt = source.width_after_pt;
  }
}

fn merge_table_cell_style(target: &mut TableCellStyle, source: &TableCellStyle) {
  if source.shading.is_some() {
    target.shading = source.shading;
  }
  if source.borders != CellBordersModel::default() {
    target.borders = source.borders;
  }
  if source.margins.is_some() {
    target.margins = source.margins;
  }
  if source.vertical_alignment.is_some() {
    target.vertical_alignment = source.vertical_alignment;
  }
  if source.no_wrap.is_some() {
    target.no_wrap = source.no_wrap;
  }
  merge_format_values(&mut target.paragraph_format, &source.paragraph_format);
  merge_style_values(&mut target.run_style, &source.run_style);
  target.run_overrides = merge_run_style_overrides(target.run_overrides, source.run_overrides);
}

fn merge_run_style_overrides(
  mut target: RunStyleOverrides,
  source: RunStyleOverrides,
) -> RunStyleOverrides {
  if source.bold.is_some() {
    target.bold = source.bold;
  }
  if source.font_size_pt.is_some() {
    target.font_size_pt = source.font_size_pt;
  }
  if source.complex_font_size_pt.is_some() {
    target.complex_font_size_pt = source.complex_font_size_pt;
  }
  if source.vertical_alignment.is_some() {
    target.vertical_alignment = source.vertical_alignment;
  }
  if source.italic.is_some() {
    target.italic = source.italic;
  }
  if source.underline.is_some() {
    target.underline = source.underline;
  }
  if source.strikethrough.is_some() {
    target.strikethrough = source.strikethrough;
  }
  if source.uppercase.is_some() {
    target.uppercase = source.uppercase;
  }
  if source.small_caps.is_some() {
    target.small_caps = source.small_caps;
  }
  if source.hidden.is_some() {
    target.hidden = source.hidden;
  }
  if source.legacy_outline.is_some() {
    target.legacy_outline = source.legacy_outline;
  }
  if source.legacy_shadow.is_some() {
    target.legacy_shadow = source.legacy_shadow;
  }
  if source.legacy_emboss.is_some() {
    target.legacy_emboss = source.legacy_emboss;
  }
  if source.legacy_imprint.is_some() {
    target.legacy_imprint = source.legacy_imprint;
  }
  target
}

fn table_look_model(look: &w::TableLook) -> TableLookModel {
  let mut model = TableLookModel::default();
  if let Some(value) = look.first_row {
    model.first_row = value.as_bool();
  }
  if let Some(value) = look.last_row {
    model.last_row = value.as_bool();
  }
  if let Some(value) = look.first_column {
    model.first_column = value.as_bool();
  }
  if let Some(value) = look.last_column {
    model.last_column = value.as_bool();
  }
  if let Some(value) = look.no_horizontal_band {
    model.horizontal_banding = !value.as_bool();
  }
  if let Some(value) = look.no_vertical_band {
    model.vertical_banding = !value.as_bool();
  }
  model
}

fn is_toc_entry_style_name(value: &str) -> bool {
  let lowercase = value.trim().to_ascii_lowercase();
  lowercase
    .strip_prefix("toc")
    .map(str::trim)
    .and_then(|level| level.parse::<u8>().ok())
    .is_some_and(|level| (1..=9).contains(&level))
}

fn push_unique_style_ref_key(keys: &mut Vec<Arc<str>>, key: &str) {
  if key.is_empty() || keys.iter().any(|existing| existing.as_ref() == key) {
    return;
  }
  keys.push(Arc::<str>::from(key));
}

fn merge_builtin_character_style(style: &mut TextStyle, style_id: &str) {
  if style_id.eq_ignore_ascii_case("Hyperlink") {
    style.underline = true;
    style.color = RgbColor {
      r: 0x05,
      g: 0x63,
      b: 0xC1,
    };
  }
}

fn run_style_overrides(properties: Option<RunProps<'_>>) -> RunStyleOverrides {
  let Some(properties) = properties else {
    return RunStyleOverrides::default();
  };

  RunStyleOverrides {
    font_size_pt: properties
      .font_size()
      .map(|value| (value.val.to_points() as f32).max(MIN_ESCAPEMENT_FONT_SIZE_PT)),
    complex_font_size_pt: properties
      .complex_script_font_size()
      .map(|value| (value.val.to_points() as f32).max(MIN_ESCAPEMENT_FONT_SIZE_PT)),
    vertical_alignment: properties.vertical_text_alignment().map(|value| value.val),
    bold: properties
      .bold()
      .and_then(|value| value.val.map(|value| value.as_bool())),
    italic: properties
      .italic()
      .and_then(|value| value.val.map(|value| value.as_bool())),
    underline: properties
      .underline()
      .map(|value| !matches!(value.val, Some(w::UnderlineValues::None))),
    strikethrough: properties
      .double_strike()
      .and_then(|value| value.val.map(|value| value.as_bool()))
      .or_else(|| {
        properties
          .strike()
          .and_then(|value| value.val.map(|value| value.as_bool()))
      }),
    uppercase: properties
      .caps()
      .and_then(|value| value.val.map(|value| value.as_bool())),
    small_caps: properties
      .small_caps()
      .map(|value| value.val.is_none_or(|value| value.as_bool())),
    hidden: properties
      .vanish()
      .and_then(|value| value.val.map(|value| value.as_bool())),
    legacy_outline: properties
      .outline()
      .map(|value| value.val.is_none_or(|value| value.as_bool())),
    legacy_shadow: properties
      .shadow()
      .map(|value| value.val.is_none_or(|value| value.as_bool())),
    legacy_emboss: properties
      .emboss()
      .map(|value| value.val.is_none_or(|value| value.as_bool())),
    legacy_imprint: properties
      .imprint()
      .map(|value| value.val.is_none_or(|value| value.as_bool())),
  }
}

fn normalize_relative_run_style(style: &mut TextStyle, overrides: RunStyleOverrides) {
  if overrides.vertical_alignment.is_none() {
    return;
  }
  // w:vertAlign is relative to the effective size after the complete style
  // cascade. Style entries are cached before their paragraph/run base is
  // known, so retain the authored size overrides separately and defer the
  // automatic escapement transform until the style is applied.
  style.font_size_pt = TextStyle::default().font_size_pt;
  style.complex_font_size_pt = None;
  style.baseline_shift_pt = 0.0;
}

fn apply_run_style_overrides(style: &mut TextStyle, overrides: RunStyleOverrides) {
  if let Some(font_size_pt) = overrides.font_size_pt {
    style.font_size_pt = font_size_pt;
  }
  if let Some(complex_font_size_pt) = overrides.complex_font_size_pt {
    style.complex_font_size_pt = Some(complex_font_size_pt);
  }
  if let Some(bold) = overrides.bold {
    style.bold = bold;
  }
  if let Some(italic) = overrides.italic {
    style.italic = italic;
  }
  if let Some(underline) = overrides.underline {
    style.underline = underline;
  }
  if let Some(strikethrough) = overrides.strikethrough {
    style.strikethrough = strikethrough;
  }
  if let Some(uppercase) = overrides.uppercase {
    style.uppercase = uppercase;
  }
  if let Some(small_caps) = overrides.small_caps {
    style.small_caps = small_caps;
  }
  if let Some(hidden) = overrides.hidden {
    style.hidden = hidden;
  }
  if let Some(outline) = overrides.legacy_outline {
    style.legacy_outline = outline;
  }
  if let Some(shadow) = overrides.legacy_shadow {
    style.legacy_shadow = shadow;
  }
  if let Some(emboss) = overrides.legacy_emboss {
    if emboss {
      style.legacy_relief = LegacyTextRelief::Embossed;
    } else if style.legacy_relief == LegacyTextRelief::Embossed {
      style.legacy_relief = LegacyTextRelief::None;
    }
  }
  if let Some(imprint) = overrides.legacy_imprint {
    if imprint {
      style.legacy_relief = LegacyTextRelief::Engraved;
    } else if style.legacy_relief == LegacyTextRelief::Engraved {
      style.legacy_relief = LegacyTextRelief::None;
    }
  }
}

fn merge_format_values(target: &mut ParagraphFormat, values: &ParagraphFormat) {
  if values.spacing_before_set || values.spacing_before_pt != 0.0 {
    target.spacing_before_pt = values.spacing_before_pt;
    target.spacing_before_lines = values.spacing_before_lines;
    target.spacing_before_set = values.spacing_before_set;
  }
  if values.spacing_before_auto.is_some() {
    target.spacing_before_auto = values.spacing_before_auto;
    target.spacing_before_auto_pt = values.spacing_before_auto_pt;
  }
  if values.spacing_after_set || values.spacing_after_pt != 0.0 {
    target.spacing_after_pt = values.spacing_after_pt;
    target.spacing_after_set = values.spacing_after_set;
  }
  if values.spacing_after_auto.is_some() {
    target.spacing_after_auto = values.spacing_after_auto;
    target.spacing_after_auto_pt = values.spacing_after_auto_pt;
  }
  if values.line_height_pt.is_some() {
    target.line_height_pt = values.line_height_pt;
    target.line_height_rule = values.line_height_rule;
  }
  if values.snap_to_grid.is_some() {
    target.snap_to_grid = values.snap_to_grid;
  }
  if values.line_vertical_alignment.is_some() {
    target.line_vertical_alignment = values.line_vertical_alignment;
  }
  if values.indent_left_set {
    target.indent_left_pt = values.indent_left_pt;
    if values.indent_left_character_units.is_some() {
      target.indent_left_character_units = values.indent_left_character_units;
    }
    target.indent_left_set = true;
  }
  if values.indent_right_set {
    target.indent_right_pt = values.indent_right_pt;
    if values.indent_right_character_units.is_some() {
      target.indent_right_character_units = values.indent_right_character_units;
    }
    target.indent_right_set = true;
  }
  if values.first_line_indent_set {
    target.first_line_indent_pt = values.first_line_indent_pt;
    if values.first_line_indent_character_units.is_some() {
      target.first_line_indent_character_units = values.first_line_indent_character_units;
    }
    target.first_line_indent_set = true;
  }
  if values.tab_stops_set {
    merge_tab_stop_values(target, values);
  }
  if values.justification != ParagraphJustification::default() {
    target.justification = values.justification;
    target.alignment = values.justification.alignment();
  } else if values.alignment != ParagraphAlignment::default() {
    target.alignment = values.alignment;
  }
  if values.shading.is_some() {
    target.shading = values.shading;
  }
  if values.borders != CellBordersModel::default() {
    target.borders = values.borders;
  }
  if values.page_break_before_set {
    target.page_break_before = values.page_break_before;
    target.page_break_before_set = true;
  }
  if values.keep_with_next_set {
    target.keep_with_next = values.keep_with_next;
    target.keep_with_next_set = true;
  }
  if values.keep_lines_set {
    target.keep_lines = values.keep_lines;
    target.keep_lines_set = true;
  }
  if values.widow_control.is_some() {
    target.widow_control = values.widow_control;
  }
  if values.contextual_spacing_set {
    target.contextual_spacing = values.contextual_spacing;
    target.contextual_spacing_set = true;
  }
  if values.suppress_auto_hyphens.is_some() {
    target.suppress_auto_hyphens = values.suppress_auto_hyphens;
  }
  if values.suppress_line_numbers.is_some() {
    target.suppress_line_numbers = values.suppress_line_numbers;
  }
  if values.auto_space_de.is_some() {
    target.auto_space_de = values.auto_space_de;
  }
  if values.auto_space_dn.is_some() {
    target.auto_space_dn = values.auto_space_dn;
  }
  if values.outline_level.is_some() {
    target.outline_level = values.outline_level;
  }
  if values.frame.is_some() {
    target.frame = values.frame;
  }
}

#[derive(Clone, Copy, Debug, Default)]
struct NumberingReference {
  num_id: Option<i32>,
  level_index: Option<i32>,
}

impl NumberingReference {
  fn from_properties(properties: &w::NumberingProperties) -> Option<Self> {
    let mut reference = Self::default();
    reference.merge_properties(properties);
    reference.resolved()
  }

  fn merge_properties(&mut self, properties: &w::NumberingProperties) {
    if let Some(numbering_id) = &properties.numbering_id {
      self.num_id = Some(numbering_id.val);
    }
    if let Some(level) = &properties.numbering_level_reference {
      self.level_index = Some(level.val);
    }
  }

  fn resolved(self) -> Option<Self> {
    self.num_id.map(|_| self)
  }

  fn num_id(self) -> i32 {
    self
      .num_id
      .expect("resolved numbering reference has num_id")
  }

  fn level_index(self) -> i32 {
    self.level_index.unwrap_or(0)
  }
}

fn select_paragraph_numbering(
  direct: Option<NumberingReference>,
  style: Option<NumberingReference>,
) -> (Option<NumberingReference>, bool, bool) {
  if direct.is_some_and(|reference| reference.num_id() == 0) {
    // Word reserves a direct w:numId=0 to stop numbering inherited from the
    // paragraph style. Writer's DomainMapper handles this separately from an
    // ordinary numbering instance, even when numbering.xml contains num 0.
    return (None, false, true);
  }
  let style_applies = direct.is_none() && style.is_some();
  (direct.or(style), style_applies, false)
}

#[derive(Clone, Copy, Debug, Default)]
struct NumberingFormatMergeContext {
  direct_indent_left: bool,
  direct_indent_right: bool,
  direct_first_line_indent: bool,
  direct_tab_stops: bool,
  style_numbering: bool,
  matched_style_indent_left: bool,
  matched_style_indent_right: bool,
  matched_style_first_line_indent: bool,
}

impl NumberingFormatMergeContext {
  fn from_direct_properties(properties: Option<ParagraphProps<'_>>) -> Self {
    let mut context = Self::default();
    let Some(indentation) = properties.and_then(|properties| properties.indentation()) else {
      return context;
    };

    // [MS-OI29500] §2.1.87 (Part 1 §17.3.1.12) says Word ignores a zero
    // character-unit indent together with the related character-unit value
    // inherited earlier in the style hierarchy. Such a value therefore does
    // not protect the corresponding twip indent from the numbering level.
    context.direct_indent_left = indentation.start.is_some()
      || indentation.left.is_some()
      || indentation.start_characters.is_some_and(|value| value != 0)
      || indentation.left_chars.is_some_and(|value| value != 0);
    context.direct_indent_right = indentation.end.is_some()
      || indentation.right.is_some()
      || indentation.end_characters.is_some_and(|value| value != 0)
      || indentation.right_chars.is_some_and(|value| value != 0);
    context.direct_first_line_indent = indentation.first_line.is_some()
      || indentation.hanging.is_some()
      || indentation.first_line_chars.is_some_and(|value| value != 0)
      || indentation.hanging_chars.is_some_and(|value| value != 0);
    context
  }

  fn has_direct_indentation(self) -> bool {
    self.direct_indent_left || self.direct_indent_right || self.direct_first_line_indent
  }
}

fn merge_numbering_format_values(
  target: &mut ParagraphFormat,
  values: &ParagraphFormat,
  context: NumberingFormatMergeContext,
) {
  if values.spacing_before_set || values.spacing_before_pt != 0.0 {
    target.spacing_before_pt = values.spacing_before_pt;
    target.spacing_before_set = values.spacing_before_set;
  }
  if values.spacing_before_auto.is_some() {
    target.spacing_before_auto = values.spacing_before_auto;
    target.spacing_before_auto_pt = values.spacing_before_auto_pt;
  }
  if values.spacing_after_set || values.spacing_after_pt != 0.0 {
    target.spacing_after_pt = values.spacing_after_pt;
    target.spacing_after_set = values.spacing_after_set;
  }
  if values.spacing_after_auto.is_some() {
    target.spacing_after_auto = values.spacing_after_auto;
    target.spacing_after_auto_pt = values.spacing_after_auto_pt;
  }
  if values.line_height_pt.is_some() {
    target.line_height_pt = values.line_height_pt;
    target.line_height_rule = values.line_height_rule;
  }
  if values.snap_to_grid.is_some() {
    target.snap_to_grid = values.snap_to_grid;
  }
  if values.line_vertical_alignment.is_some() {
    target.line_vertical_alignment = values.line_vertical_alignment;
  }
  let protect_indents =
    (context.direct_indent_left || context.style_numbering || context.matched_style_indent_left)
      && target.indent_left_set;
  if values.indent_left_set && !protect_indents {
    target.indent_left_pt = values.indent_left_pt;
    if values.indent_left_character_units.is_some() {
      target.indent_left_character_units = values.indent_left_character_units;
    }
    target.indent_left_set = true;
  }
  let protect_indents =
    (context.direct_indent_right || context.style_numbering || context.matched_style_indent_right)
      && target.indent_right_set;
  if values.indent_right_set && !protect_indents {
    target.indent_right_pt = values.indent_right_pt;
    if values.indent_right_character_units.is_some() {
      target.indent_right_character_units = values.indent_right_character_units;
    }
    target.indent_right_set = true;
  }
  let protect_indents = (context.direct_first_line_indent
    || context.style_numbering
    || context.matched_style_first_line_indent)
    && target.first_line_indent_set;
  if values.first_line_indent_set && !protect_indents {
    target.first_line_indent_pt = values.first_line_indent_pt;
    if values.first_line_indent_character_units.is_some() {
      target.first_line_indent_character_units = values.first_line_indent_character_units;
    }
    target.first_line_indent_set = true;
  }
  if values.tab_stops_set && !(context.direct_tab_stops && target.tab_stops_set) {
    merge_tab_stop_values(target, values);
  }
  if values.justification != ParagraphJustification::default() {
    target.justification = values.justification;
    target.alignment = values.justification.alignment();
  } else if values.alignment != ParagraphAlignment::default() {
    target.alignment = values.alignment;
  }
  if values.shading.is_some() {
    target.shading = values.shading;
  }
  if values.borders != CellBordersModel::default() {
    target.borders = values.borders;
  }
  if values.page_break_before_set {
    target.page_break_before = values.page_break_before;
    target.page_break_before_set = true;
  }
  if values.keep_with_next_set {
    target.keep_with_next = values.keep_with_next;
    target.keep_with_next_set = true;
  }
  if values.keep_lines_set {
    target.keep_lines = values.keep_lines;
    target.keep_lines_set = true;
  }
  if values.widow_control.is_some() {
    target.widow_control = values.widow_control;
  }
  if values.contextual_spacing_set {
    target.contextual_spacing = values.contextual_spacing;
    target.contextual_spacing_set = true;
  }
  if values.suppress_auto_hyphens.is_some() {
    target.suppress_auto_hyphens = values.suppress_auto_hyphens;
  }
  if values.suppress_line_numbers.is_some() {
    target.suppress_line_numbers = values.suppress_line_numbers;
  }
  if values.auto_space_de.is_some() {
    target.auto_space_de = values.auto_space_de;
  }
  if values.auto_space_dn.is_some() {
    target.auto_space_dn = values.auto_space_dn;
  }
  if values.outline_level.is_some() {
    target.outline_level = values.outline_level;
  }
  if values.frame.is_some() {
    target.frame = values.frame;
  }
}

fn merge_style_values(target: &mut TextStyle, values: &TextStyle) {
  if values.font_family.is_some() {
    target.font_family = values.font_family.clone();
  }
  if values.language.is_some() {
    target.language = values.language.clone();
  }
  if values.east_asia_language.is_some() {
    target.east_asia_language = values.east_asia_language.clone();
  }
  if values.bidi_language.is_some() {
    target.bidi_language = values.bidi_language.clone();
  }
  if (values.font_size_pt - TextStyle::default().font_size_pt).abs() > f32::EPSILON {
    target.font_size_pt = values.font_size_pt;
  }
  if values.complex_font_size_pt.is_some() {
    target.complex_font_size_pt = values.complex_font_size_pt;
  }
  if values.complex_script.is_some() {
    target.complex_script = values.complex_script;
  }
  if values.right_to_left.is_some() {
    target.right_to_left = values.right_to_left;
  }
  if values.complex_bold.is_some() {
    target.complex_bold = values.complex_bold;
  }
  if values.complex_italic.is_some() {
    target.complex_italic = values.complex_italic;
  }
  if values.kerning_minimum_size_pt.is_some() {
    target.kerning_minimum_size_pt = values.kerning_minimum_size_pt;
  }
  if values.ligatures.is_some() {
    target.ligatures = values.ligatures;
  }
  if values.horizontal_scale.is_some() {
    target.horizontal_scale = values.horizontal_scale;
  }
  if values.character_spacing_pt.abs() > f32::EPSILON {
    target.character_spacing_pt = values.character_spacing_pt;
  }
  if values.baseline_shift_pt.abs() > f32::EPSILON {
    target.baseline_shift_pt = values.baseline_shift_pt;
  }
  if values.bold {
    target.bold = true;
  }
  if values.italic {
    target.italic = true;
  }
  if values.underline {
    target.underline = true;
  }
  if values.strikethrough {
    target.strikethrough = true;
  }
  if values.uppercase {
    target.uppercase = true;
  }
  if values.small_caps {
    target.small_caps = true;
  }
  if values.hidden {
    target.hidden = true;
  }
  if values.legacy_outline {
    target.legacy_outline = true;
  }
  if values.legacy_shadow {
    target.legacy_shadow = true;
  }
  if values.legacy_relief != LegacyTextRelief::None {
    target.legacy_relief = values.legacy_relief;
  }
  if !values.color_is_automatic || values.color != TextStyle::default().color {
    target.color = values.color;
    target.color_is_automatic = false;
  }
  if values.highlight.is_some() {
    target.highlight = values.highlight;
  }
}

#[derive(Clone, Debug, Default)]
struct NumberingCatalog {
  nums: HashMap<i32, NumberingInstance>,
  abstract_nums: HashMap<i32, AbstractNumbering>,
  picture_bullets: HashMap<i32, InlineImage>,
  counters: HashMap<(i32, i32), i32>,
  initialized_start_overrides: HashSet<(i32, i32)>,
}

#[derive(Clone, Debug)]
struct NumberingInstance {
  abstract_num_id: i32,
  overrides: HashMap<i32, LevelOverride>,
}

#[derive(Clone, Debug)]
struct LevelOverride {
  start: Option<i32>,
  level: Option<NumberingLevel>,
}

#[derive(Clone, Debug, Default)]
struct AbstractNumbering {
  levels: HashMap<i32, NumberingLevel>,
  style_link: Option<String>,
  numbering_style_link: Option<String>,
}

#[derive(Clone, Debug)]
struct NumberingLevel {
  start: i32,
  restart_level: Option<i32>,
  paragraph_style_id: Option<String>,
  format: w::NumberFormatValues,
  custom_format: Option<String>,
  text: String,
  suffix: NumberingSuffix,
  justification: w::LevelJustificationValues,
  list_tab_stop_pt: Option<f32>,
  picture_bullet_id: Option<i32>,
  is_legal: bool,
  format_properties: ParagraphFormat,
  symbol_run_properties: Option<w::NumberingSymbolRunProperties>,
}

#[derive(Clone, Copy, Debug, Default)]
enum NumberingSuffix {
  #[default]
  Tab,
  Space,
  Nothing,
}

#[derive(Clone, Debug)]
struct NumberingLabel {
  text: Option<String>,
  suppressed_non_numerical_text: Option<String>,
  image: Option<InlineImage>,
  style: TextStyle,
  justification: w::LevelJustificationValues,
  list_tab_stop_pt: Option<f32>,
  width_aware_tab: bool,
}

#[derive(Clone, Debug)]
struct NumberingCounterState {
  counters: HashMap<(i32, i32), i32>,
  initialized_start_overrides: HashSet<(i32, i32)>,
}

fn finalize_numbering_symbol_transport_style(
  style: &mut TextStyle,
  inherited_style: &TextStyle,
  format: w::NumberFormatValues,
  symbol_run_properties: Option<&w::NumberingSymbolRunProperties>,
  text: &mut String,
) {
  if !matches!(format, w::NumberFormatValues::Bullet) {
    return;
  }

  let declared_symbol_font = symbol_run_properties
    .and_then(|properties| properties.run_fonts.first())
    .and_then(|fonts| {
      fonts
        .ascii
        .as_deref()
        .filter(|font| symbol_transport_font(font))
        .or_else(|| {
          fonts
            .high_ansi
            .as_deref()
            .filter(|font| symbol_transport_font(font))
        })
    });
  if text
    .chars()
    .any(|character| (0xF000..=0xF0FF).contains(&(character as u32)))
    && let Some(font) = declared_symbol_font
  {
    // ECMA-376 Part 1 §17.9.24 applies the numbering-level rPr specifically
    // to lvlText and keeps it separate from paragraph runs. The normative
    // Numbering Definitions example declares a Symbol bullet through only
    // rFonts@ascii/@hAnsi. Preserve that explicit numbering-only face when
    // paragraph-mark w:rtl/w:cs would otherwise select an inherited complex
    // slot for the legacy U+F0XX symbol transport character.
    let font = Arc::<str>::from(font);
    style.font_family = Some(font.clone());
    style.complex_font_family = Some(font.clone());
    style.symbol_font_family = Some(font);
  }

  if style
    .font_family
    .as_deref()
    .is_some_and(|font| font.eq_ignore_ascii_case("Symbol"))
    && text.contains('\u{f094}')
  {
    // Word's legacy list transport uses F094 for a black square even
    // though Microsoft's Symbol cmap has no U+F094. Let the paragraph font
    // (and normal fallback chain) paint the Unicode square.
    *text = text.replace('\u{f094}', "■");
    style.font_family = inherited_style.font_family.clone();
    style.fallback_font_family = inherited_style.fallback_font_family.clone();
    style.complex_font_family = inherited_style.complex_font_family.clone();
    style.symbol_font_family = None;
  }
}

impl NumberingCatalog {
  fn counter_state(&self) -> NumberingCounterState {
    NumberingCounterState {
      counters: self.counters.clone(),
      initialized_start_overrides: self.initialized_start_overrides.clone(),
    }
  }

  fn restore_counter_state(&mut self, state: NumberingCounterState) {
    self.counters = state.counters;
    self.initialized_start_overrides = state.initialized_start_overrides;
  }

  fn load(
    package: &mut WordprocessingDocument,
    main: &MainDocumentPart,
    import_settings: ImportSettings,
    styles: &StylesCatalog,
  ) -> Result<Self> {
    let Some(numbering_part) = main.numbering_definitions_part(package) else {
      return Ok(Self::default());
    };
    let numbering_images = ImageCatalog::load_from_numbering(package, &numbering_part);
    let numbering = numbering_part.root_element(package)?;
    let mut catalog = Self::default();

    for picture_bullet in &numbering.numbering_picture_bullet {
      if let Some(image) =
        numbering_picture_bullet_image(picture_bullet, &numbering_images, &styles.theme_colors)
      {
        catalog
          .picture_bullets
          .insert(picture_bullet.numbering_picture_bullet_id, image);
      }
    }

    for abstract_num in &numbering.abstract_num {
      let mut entry = AbstractNumbering {
        style_link: abstract_num
          .style_link
          .as_ref()
          .map(|link| link.val.to_string()),
        numbering_style_link: abstract_num
          .numbering_style_link
          .as_ref()
          .map(|link| link.val.to_string()),
        ..Default::default()
      };
      for level in &abstract_num.level {
        entry.levels.insert(
          level.level_index,
          numbering_level_model(level, import_settings),
        );
      }
      catalog
        .abstract_nums
        .insert(abstract_num.abstract_number_id, entry);
    }

    for num in &numbering.numbering_instance {
      let overrides = num
        .level_override
        .iter()
        .map(|level| {
          (
            level.level_index,
            LevelOverride {
              start: level
                .start_override_numbering_value
                .as_ref()
                .map(|value| value.val),
              level: level
                .level
                .as_deref()
                .map(|level| numbering_level_model(level, import_settings)),
            },
          )
        })
        .collect();
      catalog.nums.insert(
        num.number_id,
        NumberingInstance {
          abstract_num_id: num.abstract_num_id.val,
          overrides,
        },
      );
    }

    catalog.resolve_style_linked_abstract_nums(styles);

    Ok(catalog)
  }

  fn resolve_style_linked_abstract_nums(&mut self, styles: &StylesCatalog) {
    let resolved = self
      .nums
      .iter()
      .map(|(&num_id, instance)| {
        (
          num_id,
          self.resolve_abstract_num_id(instance.abstract_num_id, styles),
        )
      })
      .collect::<Vec<_>>();
    for (num_id, abstract_num_id) in resolved {
      if let Some(instance) = self.nums.get_mut(&num_id) {
        instance.abstract_num_id = abstract_num_id;
      }
    }
  }

  fn resolve_abstract_num_id(&self, abstract_num_id: i32, styles: &StylesCatalog) -> i32 {
    let mut current = abstract_num_id;
    let mut visited = HashSet::new();
    while visited.insert(current) {
      let Some(link) = self
        .abstract_nums
        .get(&current)
        .and_then(|abstract_num| abstract_num.numbering_style_link.as_deref())
      else {
        break;
      };
      let from_style = styles
        .numbering_style_num_id(link)
        .and_then(|num_id| self.nums.get(&num_id))
        .map(|instance| instance.abstract_num_id);
      let from_abstract = self
        .abstract_nums
        .iter()
        .filter_map(|(&id, abstract_num)| {
          (abstract_num.style_link.as_deref() == Some(link)).then_some(id)
        })
        .min();
      let Some(next) = from_style.or(from_abstract) else {
        break;
      };
      current = next;
    }
    current
  }

  fn next_label(
    &mut self,
    reference: NumberingReference,
    format: &mut ParagraphFormat,
    styles: &StylesCatalog,
    base_style: TextStyle,
    paragraph_mark_run_properties: Option<&w::ParagraphMarkRunProperties>,
    format_context: NumberingFormatMergeContext,
  ) -> Option<NumberingLabel> {
    let num_id = reference.num_id();
    let level_index = reference.level_index();
    let instance = self.nums.get(&num_id)?;
    let abstract_num_id = instance.abstract_num_id;
    let abstract_num = self.abstract_nums.get(&abstract_num_id)?;
    let level_override = instance.overrides.get(&level_index);
    let level = level_override
      .and_then(|override_| override_.level.as_ref())
      .or_else(|| abstract_num.levels.get(&level_index))?;

    let level_matches_paragraph_style =
      level.paragraph_style_id.as_deref() == format.style_id.as_deref();
    let format_context = NumberingFormatMergeContext {
      // ECMA-376 Part 1 §17.9.23 associates this numbering level with the
      // named paragraph style. When that association matches the effective
      // style, the level's paragraph properties are the style's numbering
      // geometry rather than a lower-priority overlay. Direct paragraph
      // indents remain protected independently.
      style_numbering: format_context.style_numbering && !level_matches_paragraph_style,
      matched_style_indent_left: format_context.matched_style_indent_left
        && level_matches_paragraph_style,
      matched_style_indent_right: format_context.matched_style_indent_right
        && level_matches_paragraph_style,
      matched_style_first_line_indent: format_context.matched_style_first_line_indent
        && level_matches_paragraph_style,
      ..format_context
    };
    merge_numbering_format_values(format, &level.format_properties, format_context);
    let start_override = level_override.and_then(|override_| override_.start);
    let start = start_override.unwrap_or(level.start);

    // A list can begin at a deeper level before any paragraph at its parent
    // levels. Word still counts the implicit parent nodes: a first level-1
    // item formatted as "10.1" consumes level 0's start value 10, so the
    // first later level-0 paragraph is 11 rather than 10. LibreOffice models
    // these missing parents as counted phantom nodes in SwNumberTree.
    for parent_level_index in 0..level_index {
      let parent_override = instance.overrides.get(&parent_level_index);
      let Some(parent_level) = parent_override
        .and_then(|override_| override_.level.as_ref())
        .or_else(|| abstract_num.levels.get(&parent_level_index))
      else {
        continue;
      };
      let parent_start_override = parent_override.and_then(|override_| override_.start);
      let parent_start = parent_start_override.unwrap_or(parent_level.start);
      let parent_key = (abstract_num_id, parent_level_index);
      let initializes_start_override = parent_start_override.is_some()
        && self
          .initialized_start_overrides
          .insert((num_id, parent_level_index));
      let creates_phantom = !self.counters.contains_key(&parent_key);
      if initializes_start_override || creates_phantom {
        self.counters.insert(parent_key, parent_start);
        clear_numbering_counters_restarted_after(
          &mut self.counters,
          instance,
          abstract_num,
          abstract_num_id,
          parent_level_index,
        );
      }
    }

    let counter_key = (abstract_num_id, level_index);
    let initializes_start_override = start_override.is_some()
      && self
        .initialized_start_overrides
        .insert((num_id, level_index));
    let counter = if initializes_start_override {
      self.counters.insert(counter_key, start);
      start
    } else {
      let counter = self.counters.entry(counter_key).or_insert(start - 1);
      *counter += 1;
      *counter
    };
    clear_numbering_counters_restarted_after(
      &mut self.counters,
      instance,
      abstract_num,
      abstract_num_id,
      level_index,
    );

    let mut text = format_numbering_label(
      level,
      abstract_num_id,
      level_index,
      counter,
      abstract_num,
      &instance.overrides,
      &self.counters,
    );
    let suppressed_non_numerical_text = format_numbering_label_suppressing_non_numerical(
      level,
      abstract_num_id,
      level_index,
      counter,
      abstract_num,
      &instance.overrides,
      &self.counters,
    );
    let mut style = base_style;
    let inherited_bullet_style = style.clone();
    // LibreOffice's NewNumberPortion starts ordinary numbering from the
    // paragraph font and clears underline/overline only. Character bullets
    // additionally clear paragraph bold/italic before their explicit
    // numbering-level run properties are applied.
    style.underline = false;
    // Word shapes the synthesized numbering portion independently from the
    // paragraph text. In particular, an inherited w:kern from docDefaults
    // does not kern textual list labels; only an explicit numbering-level or
    // paragraph-mark run property can enable it. This distinction is visible
    // at default-tab boundaries: "Four." and "Nineteen." advance to the next
    // stop in Word while the following body text retains normal kerning.
    style.kerning_minimum_size_pt = Some(f32::INFINITY);
    if matches!(level.format, w::NumberFormatValues::Bullet) {
      style.bold = false;
      style.italic = false;
    }
    properties::merge_run_style(
      &mut style,
      level
        .symbol_run_properties
        .as_ref()
        .map(RunProps::Numbering),
      &styles.theme_fonts,
      &styles.theme_colors,
    );
    if paragraph_mark_run_properties.is_some() {
      style = properties::paragraph_mark_run_style(paragraph_mark_run_properties, style, styles);
      properties::merge_run_style(
        &mut style,
        level
          .symbol_run_properties
          .as_ref()
          .map(RunProps::Numbering),
        &styles.theme_fonts,
        &styles.theme_colors,
      );
    }
    finalize_numbering_symbol_transport_style(
      &mut style,
      &inherited_bullet_style,
      level.format,
      level.symbol_run_properties.as_ref(),
      &mut text,
    );
    let picture_bullet = level.picture_bullet_id.is_some();
    let image = level
      .picture_bullet_id
      .and_then(|id| self.picture_bullets.get(&id).cloned());
    Some(NumberingLabel {
      // w:lvlPicBulletId selects the picture representation even when the
      // referenced VML shape has no usable graphic. Word leaves that marker
      // empty; it does not fall back to the textual w:lvlText bullet.
      text: (!picture_bullet).then_some(text),
      suppressed_non_numerical_text: (!picture_bullet).then_some(suppressed_non_numerical_text),
      image,
      style,
      justification: level.justification,
      list_tab_stop_pt: level.list_tab_stop_pt,
      width_aware_tab: matches!(
        level.format,
        w::NumberFormatValues::Ordinal
          | w::NumberFormatValues::CardinalText
          | w::NumberFormatValues::OrdinalText
      ),
    })
  }
}

fn numbering_level_model(level: &w::Level, import_settings: ImportSettings) -> NumberingLevel {
  let mut format_properties = ParagraphFormat::default();
  merge_paragraph_format(
    &mut format_properties,
    level
      .previous_paragraph_properties
      .as_deref()
      .map(ParagraphProps::Previous),
    import_settings,
  );

  NumberingLevel {
    start: level
      .start_numbering_value
      .as_ref()
      .map(|value| value.val)
      // ECMA-376 Part 1 §17.9.25: an omitted w:start begins at zero.
      .unwrap_or(0),
    restart_level: level.level_restart.as_ref().map(|restart| restart.val),
    paragraph_style_id: level
      .paragraph_style_id_in_level
      .as_ref()
      .map(|style| style.val.to_string()),
    format: level
      .numbering_format
      .as_ref()
      .map(|format| format.val)
      .unwrap_or_default(),
    custom_format: level
      .numbering_format
      .as_ref()
      .and_then(|format| format.format.as_ref())
      .map(ToString::to_string),
    text: level
      .level_text
      .as_ref()
      .and_then(|text| text.val.as_ref())
      .map(ToString::to_string)
      .unwrap_or_else(|| "%1.".to_string()),
    suffix: level
      .level_suffix
      .as_ref()
      .map(|suffix| match suffix.val {
        w::LevelSuffixValues::Tab => NumberingSuffix::Tab,
        w::LevelSuffixValues::Space => NumberingSuffix::Space,
        w::LevelSuffixValues::Nothing => NumberingSuffix::Nothing,
      })
      .unwrap_or_default(),
    justification: level
      .level_justification
      .as_ref()
      .map(|justification| justification.w_val)
      .unwrap_or_default(),
    list_tab_stop_pt: numbering_level_list_tab_stop_pt(level),
    picture_bullet_id: level.level_picture_bullet_id.as_ref().map(|id| id.val),
    is_legal: level.is_legal_numbering_style.is_some(),
    format_properties,
    symbol_run_properties: level.numbering_symbol_run_properties.as_deref().cloned(),
  }
}

fn numbering_level_restarts_after(
  level: &NumberingLevel,
  level_index: i32,
  used_level_index: i32,
) -> bool {
  if used_level_index >= level_index {
    return false;
  }
  match level.restart_level {
    None => true,
    Some(0) => false,
    Some(restart_level) if restart_level > 0 && restart_level <= level_index => {
      used_level_index < restart_level
    }
    // ECMA-376 Part 1 §17.9.10 says an invalid reference to the current or
    // a lower level is ignored, leaving the ordinary previous-level restart.
    Some(_) => true,
  }
}

fn clear_numbering_counters_restarted_after(
  counters: &mut HashMap<(i32, i32), i32>,
  instance: &NumberingInstance,
  abstract_num: &AbstractNumbering,
  abstract_num_id: i32,
  used_level_index: i32,
) {
  for key_level in (used_level_index + 1)..=8 {
    let restart_level = instance
      .overrides
      .get(&key_level)
      .and_then(|override_| override_.level.as_ref())
      .or_else(|| abstract_num.levels.get(&key_level));
    if restart_level
      .is_some_and(|level| !numbering_level_restarts_after(level, key_level, used_level_index))
    {
      continue;
    }
    let key = (abstract_num_id, key_level);
    counters.remove(&key);
  }
}

fn numbering_level_list_tab_stop_pt(level: &w::Level) -> Option<f32> {
  level
    .previous_paragraph_properties
    .as_deref()
    .and_then(|properties| properties.tabs.as_ref())
    .and_then(|tabs| {
      tabs.tab_stop.iter().find_map(|tab| {
        (tab.val == w::TabStopValues::Number)
          .then(|| signed_twips_measure_to_points(&tab.position))
          .flatten()
      })
    })
}

fn numbering_picture_bullet_image(
  picture_bullet: &w::NumberingPictureBullet,
  images: &ImageCatalog,
  theme_colors: &ThemeColors,
) -> Option<InlineImage> {
  match picture_bullet.numbering_picture_bullet_choice.as_ref()? {
    w::NumberingPictureBulletChoice::PictureBulletBase(picture) => {
      picture_bullet_base_image(picture, images).map(normalize_picture_bullet_image_size)
    }
    w::NumberingPictureBulletChoice::Drawing(drawing) => {
      numbering_drawing_image(drawing, images, theme_colors)
        .map(normalize_picture_bullet_image_size)
    }
  }
}

fn numbering_drawing_image(
  drawing: &w::Drawing,
  images: &ImageCatalog,
  theme_colors: &ThemeColors,
) -> Option<InlineImage> {
  if drawing_is_hidden(drawing) {
    return None;
  }
  let (graphic_data, width_pt, height_pt, alt_text) = match drawing.drawing_choice.as_ref()? {
    w::DrawingChoice::Inline(inline) => (
      &inline.graphic.graphic_data,
      units::emu_to_points(inline.extent.cx),
      units::emu_to_points(inline.extent.cy),
      inline.doc_properties.description.clone(),
    ),
    w::DrawingChoice::Anchor(anchor) => (
      &anchor.graphic.as_ref().graphic_data,
      units::emu_to_points(anchor.extent.cx),
      units::emu_to_points(anchor.extent.cy),
      anchor
        .doc_properties
        .as_deref()
        .and_then(|properties| properties.description.clone()),
    ),
  };
  let properties = drawing_image_properties(graphic_data, theme_colors, Some(images))?;
  let resource = images
    .by_relationship_id
    .get(properties.relationship_id.as_deref()?)?;
  let image_data = image_data_with_effects(resource, &properties);
  Some(InlineImage {
    data: image_data.data,
    content_type: image_data.content_type,
    picture_frame: properties.picture_frame,
    effects: properties.shape_effects,
    static3d: properties.static3d,
    width_pt,
    height_pt,
    effect_left_pt: 0.0,
    effect_top_pt: 0.0,
    effect_right_pt: 0.0,
    effect_bottom_pt: 0.0,
    crop: properties.crop,
    rotation_deg: properties.rotation_deg,
    flip_horizontal: properties.flip_horizontal,
    flip_vertical: properties.flip_vertical,
    metafile_background_color: None,
    alt_text,
    hyperlink_url: None,
    semantic_metafile_text: false,
    metafile_native_size: true,
    picture_content_control: false,
    placement: ImagePlacement::Inline,
  })
}

fn picture_bullet_base_image(
  picture: &w::PictureBulletBase,
  images: &ImageCatalog,
) -> Option<InlineImage> {
  picture
    .picture_bullet_base_choice
    .iter()
    .find_map(|choice| match choice {
      w::PictureBulletBaseChoice::Group(group) => group_image(group, images),
      w::PictureBulletBaseChoice::ImageFile(image) => image_file_image(image, images),
      w::PictureBulletBaseChoice::Rectangle(rectangle) => rectangle_image(rectangle, images),
      w::PictureBulletBaseChoice::RoundRectangle(round_rectangle) => {
        round_rectangle_image(round_rectangle, images)
      }
      w::PictureBulletBaseChoice::Shape(shape) => shape_image(shape, images),
      _ => None,
    })
}

fn normalize_picture_bullet_image_size(mut image: InlineImage) -> InlineImage {
  if image.width_pt > 0.0 && image.height_pt > 0.0 {
    let height_pt = 14.0;
    image.width_pt = height_pt * image.width_pt / image.height_pt;
    image.height_pt = height_pt;
  }
  image
}

fn format_numbering_label(
  level: &NumberingLevel,
  counter_id: i32,
  level_index: i32,
  value: i32,
  abstract_num: &AbstractNumbering,
  overrides: &HashMap<i32, LevelOverride>,
  counters: &HashMap<(i32, i32), i32>,
) -> String {
  if matches!(level.format, w::NumberFormatValues::Bullet) {
    return format!("{}{}", level.text, numbering_suffix_text(level.suffix));
  }

  let mut text = level.text.clone();
  for index in 0..=8 {
    let placeholder = format!("%{}", index + 1);
    if !text.contains(&placeholder) {
      continue;
    }
    let referenced_level = overrides
      .get(&index)
      .and_then(|override_| override_.level.as_ref())
      .or_else(|| abstract_num.levels.get(&index));
    let value = if index == level_index {
      value
    } else {
      counters
        .get(&(counter_id, index))
        .copied()
        .unwrap_or_else(|| referenced_level.map(|level| level.start).unwrap_or(1))
    };
    text = text.replace(
      &placeholder,
      &referenced_level.map_or_else(
        || value.to_string(),
        |referenced_level| {
          format_numbering_level_value(
            value,
            referenced_level,
            level.is_legal && legal_numbering_requires_decimal_override(referenced_level.format),
          )
        },
      ),
    );
  }
  format!("{text}{}", numbering_suffix_text(level.suffix))
}

fn format_numbering_label_suppressing_non_numerical(
  level: &NumberingLevel,
  counter_id: i32,
  level_index: i32,
  value: i32,
  abstract_num: &AbstractNumbering,
  overrides: &HashMap<i32, LevelOverride>,
  counters: &HashMap<(i32, i32), i32>,
) -> String {
  if matches!(level.format, w::NumberFormatValues::Bullet) {
    return level
      .text
      .chars()
      .filter(|ch| is_word_numbering_delimiter(*ch))
      .collect();
  }

  let mut output = String::new();
  let mut chars = level.text.chars().peekable();
  while let Some(ch) = chars.next() {
    if ch == '%'
      && let Some(index) = chars.peek().and_then(|ch| ch.to_digit(10))
      && index > 0
    {
      chars.next();
      let referenced_index = i32::try_from(index - 1).unwrap_or_default();
      let referenced_level = overrides
        .get(&referenced_index)
        .and_then(|override_| override_.level.as_ref())
        .or_else(|| abstract_num.levels.get(&referenced_index));
      let referenced_value = if referenced_index == level_index {
        value
      } else {
        counters
          .get(&(counter_id, referenced_index))
          .copied()
          .unwrap_or_else(|| referenced_level.map(|level| level.start).unwrap_or(1))
      };
      output.push_str(&referenced_level.map_or_else(
        || referenced_value.to_string(),
        |referenced_level| {
          format_numbering_level_value(
            referenced_value,
            referenced_level,
            level.is_legal && legal_numbering_requires_decimal_override(referenced_level.format),
          )
        },
      ));
    } else if is_word_numbering_delimiter(ch) {
      output.push(ch);
    }
  }
  output
}

fn legal_numbering_requires_decimal_override(format: w::NumberFormatValues) -> bool {
  !matches!(
    format,
    w::NumberFormatValues::None
      | w::NumberFormatValues::Decimal
      | w::NumberFormatValues::DecimalZero
      | w::NumberFormatValues::DecimalHalfWidth
  )
}

fn is_word_numbering_delimiter(ch: char) -> bool {
  matches!(
    ch,
    '.' | ',' | ':' | ';' | '-' | '(' | ')' | '[' | ']' | '{' | '}' | '/' | '\\' | '|'
  )
}

fn format_numbering_level_value(value: i32, level: &NumberingLevel, force_decimal: bool) -> String {
  if !force_decimal && level.format == w::NumberFormatValues::Custom {
    match level.custom_format.as_deref() {
      Some("001, 002, 003, ...") => return format!("{value:03}"),
      Some("0001, 0002, 0003, ...") => return format!("{value:04}"),
      Some("00001, 00002, 00003, ...") => return format!("{value:05}"),
      Some("α, β, γ, ...") => {
        return alphabetic_sequence_number(
          value,
          &[
            'α', 'β', 'γ', 'δ', 'ε', 'ζ', 'η', 'θ', 'ι', 'κ', 'λ', 'μ', 'ν', 'ξ', 'ο', 'π', 'ρ',
            'σ', 'τ', 'υ', 'φ', 'χ', 'ψ', 'ω',
          ],
        );
      }
      _ => {}
    }
  }
  format_numbering_value(value, level.format, force_decimal)
}

fn numbering_suffix_text(suffix: NumberingSuffix) -> &'static str {
  match suffix {
    NumberingSuffix::Tab => "\t",
    NumberingSuffix::Space => " ",
    NumberingSuffix::Nothing => "",
  }
}

fn format_numbering_value(
  value: i32,
  format: w::NumberFormatValues,
  force_decimal: bool,
) -> String {
  if force_decimal {
    return value.to_string();
  }
  match format {
    w::NumberFormatValues::LowerLetter => alpha_number(value, false),
    w::NumberFormatValues::UpperLetter => alpha_number(value, true),
    w::NumberFormatValues::LowerRoman => roman_number(value).to_lowercase(),
    w::NumberFormatValues::UpperRoman => roman_number(value),
    w::NumberFormatValues::Ordinal => english_ordinal_number(value),
    w::NumberFormatValues::CardinalText => english_cardinal_number(value),
    w::NumberFormatValues::OrdinalText => english_ordinal_text(value),
    w::NumberFormatValues::DecimalZero => format!("{value:02}"),
    w::NumberFormatValues::DecimalEnclosedCircle
    | w::NumberFormatValues::DecimalEnclosedCircleChinese => enclosed_decimal_number(value, 0x2460),
    w::NumberFormatValues::DecimalEnclosedFullstop => enclosed_decimal_number(value, 0x2488),
    w::NumberFormatValues::DecimalEnclosedParen => enclosed_decimal_number(value, 0x2474),
    w::NumberFormatValues::DecimalFullWidth | w::NumberFormatValues::DecimalFullWidth2 => {
      full_width_decimal_number(value)
    }
    w::NumberFormatValues::IdeographTraditional => bounded_sequence_number(
      value,
      &['甲', '乙', '丙', '丁', '戊', '己', '庚', '辛', '壬', '癸'],
    ),
    w::NumberFormatValues::IdeographZodiac | w::NumberFormatValues::IdeographZodiacTraditional => {
      bounded_sequence_number(
        value,
        &[
          '子', '丑', '寅', '卯', '辰', '巳', '午', '未', '申', '酉', '戍', '亥',
        ],
      )
    }
    w::NumberFormatValues::ChineseCounting
    | w::NumberFormatValues::ChineseCountingThousand
    | w::NumberFormatValues::JapaneseCounting => cjk_counting_number(
      value,
      &['〇', '一', '二', '三', '四', '五', '六', '七', '八', '九'],
      &['十', '百', '千'],
      &['万', '亿'],
      true,
    ),
    w::NumberFormatValues::TaiwaneseCounting | w::NumberFormatValues::TaiwaneseCountingThousand => {
      cjk_counting_number(
        value,
        &['〇', '一', '二', '三', '四', '五', '六', '七', '八', '九'],
        &['十', '百', '千'],
        &['萬', '億'],
        true,
      )
    }
    w::NumberFormatValues::ChineseLegalSimplified => cjk_counting_number(
      value,
      &['零', '壹', '贰', '叁', '肆', '伍', '陆', '柒', '捌', '玖'],
      &['拾', '佰', '仟'],
      &['万', '亿'],
      false,
    ),
    w::NumberFormatValues::IdeographLegalTraditional => cjk_counting_number(
      value,
      &['零', '壹', '貳', '參', '肆', '伍', '陸', '柒', '捌', '玖'],
      &['拾', '佰', '仟'],
      &['萬', '億'],
      false,
    ),
    w::NumberFormatValues::IdeographDigital | w::NumberFormatValues::TaiwaneseDigital => {
      cjk_digit_number(
        value,
        &['〇', '一', '二', '三', '四', '五', '六', '七', '八', '九'],
      )
    }
    w::NumberFormatValues::KoreanDigital2 => cjk_digit_number(
      value,
      &['零', '一', '二', '三', '四', '五', '六', '七', '八', '九'],
    ),
    w::NumberFormatValues::DecimalHalfWidth => value.to_string(),
    w::NumberFormatValues::None => String::new(),
    _ => value.to_string(),
  }
}

fn bounded_sequence_number(value: i32, sequence: &[char]) -> String {
  usize::try_from(value - 1)
    .ok()
    .and_then(|index| sequence.get(index))
    .map_or_else(|| value.to_string(), char::to_string)
}

fn alphabetic_sequence_number(mut value: i32, sequence: &[char]) -> String {
  if value <= 0 || sequence.is_empty() {
    return value.to_string();
  }
  let radix = i32::try_from(sequence.len()).expect("numbering alphabet fits i32");
  let mut output = Vec::new();
  while value > 0 {
    value -= 1;
    output.push(sequence[(value % radix) as usize]);
    value /= radix;
  }
  output.iter().rev().collect()
}

fn cjk_digit_number(value: i32, digits: &[char; 10]) -> String {
  value
    .to_string()
    .chars()
    .map(|character| {
      character
        .to_digit(10)
        .map_or(character, |digit| digits[digit as usize])
    })
    .collect()
}

fn cjk_counting_number(
  value: i32,
  digits: &[char; 10],
  small_units: &[char; 3],
  group_units: &[char; 2],
  omit_leading_one_ten: bool,
) -> String {
  if value <= 0 {
    return value.to_string();
  }

  let mut groups = Vec::new();
  let mut remainder = value as u32;
  while remainder > 0 {
    groups.push((remainder % 10_000) as u16);
    remainder /= 10_000;
  }

  let mut output = String::new();
  let mut pending_zero = false;
  for group_index in (0..groups.len()).rev() {
    let group = groups[group_index];
    if group == 0 {
      if !output.is_empty() {
        pending_zero = true;
      }
      continue;
    }
    if !output.is_empty() && (pending_zero || group < 1_000) {
      output.push(digits[0]);
    }
    output.push_str(&cjk_counting_group(
      group,
      digits,
      small_units,
      omit_leading_one_ten && output.is_empty(),
    ));
    if group_index > 0 {
      output.push(group_units[group_index - 1]);
    }
    pending_zero = false;
  }
  output
}

fn cjk_counting_group(
  value: u16,
  digits: &[char; 10],
  units: &[char; 3],
  omit_leading_one_ten: bool,
) -> String {
  debug_assert!((1..10_000).contains(&value));
  let mut output = String::new();
  let mut pending_zero = false;
  for (position, divisor) in [1_000_u16, 100, 10, 1].into_iter().enumerate() {
    let digit = usize::from(value / divisor % 10);
    if digit == 0 {
      if !output.is_empty() && value % divisor != 0 {
        pending_zero = true;
      }
      continue;
    }
    if pending_zero {
      output.push(digits[0]);
      pending_zero = false;
    }
    let is_leading_ten = divisor == 10 && digit == 1 && output.is_empty() && omit_leading_one_ten;
    if !is_leading_ten {
      output.push(digits[digit]);
    }
    if divisor > 1 {
      output.push(units[2 - position]);
    }
  }
  output
}

fn english_ordinal_number(value: i32) -> String {
  if value <= 0 {
    return value.to_string();
  }
  let suffix = match value % 100 {
    11..=13 => "th",
    _ => match value % 10 {
      1 => "st",
      2 => "nd",
      3 => "rd",
      _ => "th",
    },
  };
  format!("{value}{suffix}")
}

fn english_cardinal_number(value: i32) -> String {
  capitalize_ascii_initial(&english_cardinal_lower(value))
}

fn english_ordinal_text(value: i32) -> String {
  capitalize_ascii_initial(&english_ordinal_lower(value))
}

fn english_cardinal_lower(value: i32) -> String {
  if value == 0 {
    return "zero".to_string();
  }
  if value < 0 {
    return format!("minus {}", english_cardinal_lower(value.saturating_abs()));
  }
  let mut remainder = i64::from(value);
  let mut groups = Vec::new();
  for (scale, name) in [
    (1_000_000_000_i64, "billion"),
    (1_000_000_i64, "million"),
    (1_000_i64, "thousand"),
  ] {
    if remainder >= scale {
      groups.push(format!(
        "{} {name}",
        english_cardinal_below_thousand((remainder / scale) as i32)
      ));
      remainder %= scale;
    }
  }
  if remainder > 0 {
    groups.push(english_cardinal_below_thousand(remainder as i32));
  }
  groups.join(" ")
}

fn english_cardinal_below_thousand(value: i32) -> String {
  debug_assert!((1..1000).contains(&value));
  let mut parts = Vec::new();
  let hundreds = value / 100;
  let remainder = value % 100;
  if hundreds > 0 {
    parts.push(format!(
      "{} hundred",
      english_cardinal_below_hundred(hundreds)
    ));
  }
  if remainder > 0 {
    parts.push(english_cardinal_below_hundred(remainder));
  }
  parts.join(" ")
}

fn english_cardinal_below_hundred(value: i32) -> String {
  const SMALL: [&str; 20] = [
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
  ];
  const TENS: [&str; 10] = [
    "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
  ];
  if value < 20 {
    return SMALL[value as usize].to_string();
  }
  let tens = TENS[(value / 10) as usize];
  let ones = value % 10;
  if ones == 0 {
    tens.to_string()
  } else {
    format!("{tens}-{}", SMALL[ones as usize])
  }
}

fn english_ordinal_lower(value: i32) -> String {
  if value <= 0 {
    return value.to_string();
  }
  for (scale, name) in [
    (1_000_000_000, "billionth"),
    (1_000_000, "millionth"),
    (1_000, "thousandth"),
    (100, "hundredth"),
  ] {
    if value % scale == 0 {
      return format!("{} {name}", english_cardinal_lower(value / scale));
    }
    if value > scale {
      return format!(
        "{} {}",
        english_cardinal_lower(value - value % scale),
        english_ordinal_lower(value % scale)
      );
    }
  }
  const SMALL_ORDINALS: [&str; 20] = [
    "zeroth",
    "first",
    "second",
    "third",
    "fourth",
    "fifth",
    "sixth",
    "seventh",
    "eighth",
    "ninth",
    "tenth",
    "eleventh",
    "twelfth",
    "thirteenth",
    "fourteenth",
    "fifteenth",
    "sixteenth",
    "seventeenth",
    "eighteenth",
    "nineteenth",
  ];
  if value < 20 {
    return SMALL_ORDINALS[value as usize].to_string();
  }
  let tens = value / 10;
  let ones = value % 10;
  if ones == 0 {
    return match tens {
      2 => "twentieth",
      3 => "thirtieth",
      4 => "fortieth",
      5 => "fiftieth",
      6 => "sixtieth",
      7 => "seventieth",
      8 => "eightieth",
      9 => "ninetieth",
      _ => unreachable!("value is below one hundred"),
    }
    .to_string();
  }
  format!(
    "{}-{}",
    english_cardinal_below_hundred(tens * 10),
    SMALL_ORDINALS[ones as usize]
  )
}

fn capitalize_ascii_initial(value: &str) -> String {
  let mut output = value.to_string();
  if let Some(first) = output.get_mut(0..1) {
    first.make_ascii_uppercase();
  }
  output
}

fn enclosed_decimal_number(value: i32, first_codepoint: u32) -> String {
  if !(1..=20).contains(&value) {
    return value.to_string();
  }
  char::from_u32(first_codepoint + value as u32 - 1)
    .expect("ECMA-376 enclosed decimal ranges are valid Unicode")
    .to_string()
}

fn full_width_decimal_number(value: i32) -> String {
  value
    .to_string()
    .chars()
    .map(|character| match character {
      '0'..='9' => char::from_u32(0xFF10 + character as u32 - '0' as u32)
        .expect("full-width decimal range is valid Unicode"),
      _ => character,
    })
    .collect()
}

fn alpha_number(mut value: i32, upper: bool) -> String {
  if value <= 0 {
    return value.to_string();
  }
  let mut chars = Vec::new();
  while value > 0 {
    value -= 1;
    let base = if upper { b'A' } else { b'a' };
    chars.push((base + (value % 26) as u8) as char);
    value /= 26;
  }
  chars.iter().rev().collect()
}

fn roman_number(mut value: i32) -> String {
  if !(1..=3999).contains(&value) {
    return value.to_string();
  }
  let mut output = String::new();
  for (arabic, roman) in [
    (1000, "M"),
    (900, "CM"),
    (500, "D"),
    (400, "CD"),
    (100, "C"),
    (90, "XC"),
    (50, "L"),
    (40, "XL"),
    (10, "X"),
    (9, "IX"),
    (5, "V"),
    (4, "IV"),
    (1, "I"),
  ] {
    while value >= arabic {
      output.push_str(roman);
      value -= arabic;
    }
  }
  output
}

#[derive(Clone, Copy)]
enum ParagraphProps<'a> {
  Direct(&'a w::ParagraphProperties),
  Extended(&'a w::ParagraphPropertiesExtended),
  Style(&'a w::StyleParagraphProperties),
  BaseStyle(&'a w::ParagraphPropertiesBaseStyle),
  Previous(&'a w::PreviousParagraphProperties),
}

impl<'a> ParagraphProps<'a> {
  fn page_break_before(&self) -> Option<&'a w::PageBreakBefore> {
    match self {
      Self::Direct(properties) => properties.page_break_before.as_ref(),
      Self::Extended(properties) => properties.page_break_before.as_ref(),
      Self::Style(properties) => properties.page_break_before.as_ref(),
      Self::BaseStyle(properties) => properties.page_break_before.as_ref(),
      Self::Previous(properties) => properties.page_break_before.as_ref(),
    }
  }

  fn keep_next(&self) -> Option<&'a w::KeepNext> {
    match self {
      Self::Direct(properties) => properties.keep_next.as_ref(),
      Self::Extended(properties) => properties.keep_next.as_ref(),
      Self::Style(properties) => properties.keep_next.as_ref(),
      Self::BaseStyle(properties) => properties.keep_next.as_ref(),
      Self::Previous(properties) => properties.keep_next.as_ref(),
    }
  }

  fn keep_lines(&self) -> Option<&'a w::KeepLines> {
    match self {
      Self::Direct(properties) => properties.keep_lines.as_ref(),
      Self::Extended(properties) => properties.keep_lines.as_ref(),
      Self::Style(properties) => properties.keep_lines.as_ref(),
      Self::BaseStyle(properties) => properties.keep_lines.as_ref(),
      Self::Previous(properties) => properties.keep_lines.as_ref(),
    }
  }

  fn widow_control(&self) -> Option<&'a w::WidowControl> {
    match self {
      Self::Direct(properties) => properties.widow_control.as_ref(),
      Self::Extended(properties) => properties.widow_control.as_ref(),
      Self::Style(properties) => properties.widow_control.as_ref(),
      Self::BaseStyle(properties) => properties.widow_control.as_ref(),
      Self::Previous(properties) => properties.widow_control.as_ref(),
    }
  }

  fn contextual_spacing(&self) -> Option<&'a w::ContextualSpacing> {
    match self {
      Self::Direct(properties) => properties.contextual_spacing.as_ref(),
      Self::Extended(properties) => properties.contextual_spacing.as_ref(),
      Self::Style(properties) => properties.contextual_spacing.as_ref(),
      Self::BaseStyle(properties) => properties.contextual_spacing.as_ref(),
      Self::Previous(properties) => properties.contextual_spacing.as_ref(),
    }
  }

  fn suppress_auto_hyphens(&self) -> Option<&'a w::SuppressAutoHyphens> {
    match self {
      Self::Direct(properties) => properties.suppress_auto_hyphens.as_ref(),
      Self::Extended(properties) => properties.suppress_auto_hyphens.as_ref(),
      Self::Style(properties) => properties.suppress_auto_hyphens.as_ref(),
      Self::BaseStyle(properties) => properties.suppress_auto_hyphens.as_ref(),
      Self::Previous(properties) => properties.suppress_auto_hyphens.as_ref(),
    }
  }

  fn suppress_line_numbers(&self) -> Option<&'a w::SuppressLineNumbers> {
    match self {
      Self::Direct(properties) => properties.suppress_line_numbers.as_ref(),
      Self::Extended(properties) => properties.suppress_line_numbers.as_ref(),
      Self::Style(properties) => properties.suppress_line_numbers.as_ref(),
      Self::BaseStyle(properties) => properties.suppress_line_numbers.as_ref(),
      Self::Previous(properties) => properties.suppress_line_numbers.as_ref(),
    }
  }

  fn auto_space_de(&self) -> Option<&'a w::AutoSpaceDe> {
    match self {
      Self::Direct(properties) => properties.auto_space_de.as_ref(),
      Self::Extended(properties) => properties.auto_space_de.as_ref(),
      Self::Style(properties) => properties.auto_space_de.as_ref(),
      Self::BaseStyle(properties) => properties.auto_space_de.as_ref(),
      Self::Previous(properties) => properties.auto_space_de.as_ref(),
    }
  }

  fn auto_space_dn(&self) -> Option<&'a w::AutoSpaceDn> {
    match self {
      Self::Direct(properties) => properties.auto_space_dn.as_ref(),
      Self::Extended(properties) => properties.auto_space_dn.as_ref(),
      Self::Style(properties) => properties.auto_space_dn.as_ref(),
      Self::BaseStyle(properties) => properties.auto_space_dn.as_ref(),
      Self::Previous(properties) => properties.auto_space_dn.as_ref(),
    }
  }

  fn snap_to_grid(&self) -> Option<&'a w::SnapToGrid> {
    match self {
      Self::Direct(properties) => properties.snap_to_grid.as_ref(),
      Self::Extended(properties) => properties.snap_to_grid.as_ref(),
      Self::Style(properties) => properties.snap_to_grid.as_ref(),
      Self::BaseStyle(properties) => properties.snap_to_grid.as_ref(),
      Self::Previous(properties) => properties.snap_to_grid.as_ref(),
    }
  }

  fn text_alignment(&self) -> Option<&'a w::TextAlignment> {
    match self {
      Self::Direct(properties) => properties.text_alignment.as_ref(),
      Self::Extended(properties) => properties.text_alignment.as_ref(),
      Self::Style(properties) => properties.text_alignment.as_ref(),
      Self::BaseStyle(properties) => properties.text_alignment.as_ref(),
      Self::Previous(properties) => properties.text_alignment.as_ref(),
    }
  }

  fn spacing_between_lines(&self) -> Option<&'a w::SpacingBetweenLines> {
    match self {
      Self::Direct(properties) => properties.spacing_between_lines.as_ref(),
      Self::Extended(properties) => properties.spacing_between_lines.as_ref(),
      Self::Style(properties) => properties.spacing_between_lines.as_ref(),
      Self::BaseStyle(properties) => properties.spacing_between_lines.as_ref(),
      Self::Previous(properties) => properties.spacing_between_lines.as_ref(),
    }
  }

  fn indentation(&self) -> Option<&'a w::Indentation> {
    match self {
      Self::Direct(properties) => properties.indentation.as_ref(),
      Self::Extended(properties) => properties.indentation.as_ref(),
      Self::Style(properties) => properties.indentation.as_ref(),
      Self::BaseStyle(properties) => properties.indentation.as_ref(),
      Self::Previous(properties) => properties.indentation.as_ref(),
    }
  }

  fn tabs(&self) -> Option<&'a w::Tabs> {
    match self {
      Self::Direct(properties) => properties.tabs.as_ref(),
      Self::Extended(properties) => properties.tabs.as_ref(),
      Self::Style(properties) => properties.tabs.as_ref(),
      Self::BaseStyle(properties) => properties.tabs.as_ref(),
      Self::Previous(properties) => properties.tabs.as_ref(),
    }
  }

  fn numbering_properties(&self) -> Option<&'a w::NumberingProperties> {
    match self {
      Self::Direct(properties) => properties.numbering_properties.as_deref(),
      Self::Extended(properties) => properties.numbering_properties.as_deref(),
      Self::Style(properties) => properties.numbering_properties.as_deref(),
      Self::BaseStyle(properties) => properties.numbering_properties.as_deref(),
      Self::Previous(properties) => properties.numbering_properties.as_deref(),
    }
  }

  fn justification(&self) -> Option<&'a w::Justification> {
    match self {
      Self::Direct(properties) => properties.justification.as_ref(),
      Self::Extended(properties) => properties.justification.as_ref(),
      Self::Style(properties) => properties.justification.as_ref(),
      Self::BaseStyle(properties) => properties.justification.as_ref(),
      Self::Previous(properties) => properties.justification.as_ref(),
    }
  }

  fn bidi(&self) -> Option<&'a w::BiDi> {
    match self {
      Self::Direct(properties) => properties.bi_di.as_ref(),
      Self::Extended(properties) => properties.bi_di.as_ref(),
      Self::Style(properties) => properties.bi_di.as_ref(),
      Self::BaseStyle(properties) => properties.bi_di.as_ref(),
      Self::Previous(properties) => properties.bi_di.as_ref(),
    }
  }

  fn paragraph_borders(&self) -> Option<&'a w::ParagraphBorders> {
    match self {
      Self::Direct(properties) => properties.paragraph_borders.as_deref(),
      Self::Extended(properties) => properties.paragraph_borders.as_deref(),
      Self::Style(properties) => properties.paragraph_borders.as_deref(),
      Self::BaseStyle(properties) => properties.paragraph_borders.as_deref(),
      Self::Previous(properties) => properties.paragraph_borders.as_deref(),
    }
  }

  fn shading(&self) -> Option<&'a w::Shading> {
    match self {
      Self::Direct(properties) => properties.shading.as_ref(),
      Self::Extended(properties) => properties.shading.as_ref(),
      Self::Style(properties) => properties.shading.as_ref(),
      Self::BaseStyle(properties) => properties.shading.as_ref(),
      Self::Previous(properties) => properties.shading.as_ref(),
    }
  }

  fn outline_level(&self) -> Option<&'a w::OutlineLevel> {
    match self {
      Self::Direct(properties) => properties.outline_level.as_ref(),
      Self::Extended(properties) => properties.outline_level.as_ref(),
      Self::Style(properties) => properties.outline_level.as_ref(),
      Self::BaseStyle(properties) => properties.outline_level.as_ref(),
      Self::Previous(properties) => properties.outline_level.as_ref(),
    }
  }

  fn frame_properties(&self) -> Option<&'a w::FrameProperties> {
    match self {
      Self::Direct(properties) => properties.frame_properties.as_ref(),
      Self::Extended(properties) => properties.frame_properties.as_ref(),
      Self::Style(properties) => properties.frame_properties.as_ref(),
      Self::BaseStyle(properties) => properties.frame_properties.as_ref(),
      Self::Previous(properties) => properties.frame_properties.as_ref(),
    }
  }
}

pub(super) enum RunProps<'a> {
  Direct(&'a w::RunProperties),
  Style(&'a w::StyleRunProperties),
  BaseStyle(&'a w::RunPropertiesBaseStyle),
  Numbering(&'a w::NumberingSymbolRunProperties),
  ParagraphMark(&'a w::ParagraphMarkRunProperties),
}

macro_rules! run_properties_accessor {
  ($name:ident, $variant:ident, $ty:ty) => {
    fn $name(properties: &w::RunProperties) -> Option<&$ty> {
      properties
        .run_properties_choice
        .iter()
        .find_map(|choice| match choice {
          w::RunPropertiesChoice::$variant(value) => Some(value.as_ref()),
          _ => None,
        })
    }
  };
}

macro_rules! paragraph_mark_run_properties_accessor {
  ($name:ident, $variant:ident, $ty:ty) => {
    fn $name(properties: &w::ParagraphMarkRunProperties) -> Option<&$ty> {
      properties
        .paragraph_mark_run_properties_choice2
        .iter()
        .find_map(|choice| match choice {
          w::ParagraphMarkRunPropertiesChoice2::$variant(value) => Some(value.as_ref()),
          _ => None,
        })
    }
  };
}

run_properties_accessor!(run_properties_run_style, RunStyle, w::RunStyle);
run_properties_accessor!(run_properties_run_fonts, RunFonts, w::RunFonts);
run_properties_accessor!(run_properties_bold, Bold, w::Bold);
run_properties_accessor!(
  run_properties_bold_complex_script,
  BoldComplexScript,
  w::BoldComplexScript
);
run_properties_accessor!(run_properties_italic, Italic, w::Italic);
run_properties_accessor!(
  run_properties_italic_complex_script,
  ItalicComplexScript,
  w::ItalicComplexScript
);
run_properties_accessor!(run_properties_font_size, FontSize, w::FontSize);
run_properties_accessor!(
  run_properties_complex_script_font_size,
  FontSizeComplexScript,
  w::FontSizeComplexScript
);
run_properties_accessor!(run_properties_color, Color, w::Color);
run_properties_accessor!(run_properties_shading, Shading, w::Shading);
run_properties_accessor!(run_properties_underline, Underline, w::Underline);
run_properties_accessor!(run_properties_strike, Strike, w::Strike);
run_properties_accessor!(run_properties_double_strike, DoubleStrike, w::DoubleStrike);
run_properties_accessor!(run_properties_outline, Outline, w::Outline);
run_properties_accessor!(run_properties_shadow, Shadow, w::Shadow);
run_properties_accessor!(run_properties_emboss, Emboss, w::Emboss);
run_properties_accessor!(run_properties_imprint, Imprint, w::Imprint);
run_properties_accessor!(run_properties_caps, Caps, w::Caps);
run_properties_accessor!(run_properties_small_caps, SmallCaps, w::SmallCaps);
run_properties_accessor!(run_properties_vanish, Vanish, w::Vanish);
run_properties_accessor!(
  run_properties_vertical_text_alignment,
  VerticalTextAlignment,
  w::VerticalTextAlignment
);
run_properties_accessor!(run_properties_spacing, Spacing, w::Spacing);
run_properties_accessor!(
  run_properties_character_scale,
  CharacterScale,
  w::CharacterScale
);
run_properties_accessor!(run_properties_kern, Kern, w::Kern);
run_properties_accessor!(run_properties_position, Position, w::Position);
run_properties_accessor!(run_properties_highlight, Highlight, w::Highlight);
run_properties_accessor!(
  run_properties_right_to_left_text,
  RightToLeftText,
  w::RightToLeftText
);
run_properties_accessor!(
  run_properties_complex_script,
  ComplexScript,
  w::ComplexScript
);
run_properties_accessor!(run_properties_languages, Languages, w::Languages);

paragraph_mark_run_properties_accessor!(
  paragraph_mark_run_properties_run_style,
  RunStyle,
  w::RunStyle
);
paragraph_mark_run_properties_accessor!(
  paragraph_mark_run_properties_run_fonts,
  RunFonts,
  w::RunFonts
);
paragraph_mark_run_properties_accessor!(paragraph_mark_run_properties_bold, Bold, w::Bold);
paragraph_mark_run_properties_accessor!(
  paragraph_mark_run_properties_bold_complex_script,
  BoldComplexScript,
  w::BoldComplexScript
);
paragraph_mark_run_properties_accessor!(paragraph_mark_run_properties_italic, Italic, w::Italic);
paragraph_mark_run_properties_accessor!(
  paragraph_mark_run_properties_italic_complex_script,
  ItalicComplexScript,
  w::ItalicComplexScript
);
paragraph_mark_run_properties_accessor!(
  paragraph_mark_run_properties_font_size,
  FontSize,
  w::FontSize
);
paragraph_mark_run_properties_accessor!(
  paragraph_mark_run_properties_complex_script_font_size,
  FontSizeComplexScript,
  w::FontSizeComplexScript
);
paragraph_mark_run_properties_accessor!(paragraph_mark_run_properties_color, Color, w::Color);
paragraph_mark_run_properties_accessor!(paragraph_mark_run_properties_shading, Shading, w::Shading);
paragraph_mark_run_properties_accessor!(
  paragraph_mark_run_properties_underline,
  Underline,
  w::Underline
);
paragraph_mark_run_properties_accessor!(paragraph_mark_run_properties_strike, Strike, w::Strike);
paragraph_mark_run_properties_accessor!(
  paragraph_mark_run_properties_double_strike,
  DoubleStrike,
  w::DoubleStrike
);
paragraph_mark_run_properties_accessor!(paragraph_mark_run_properties_outline, Outline, w::Outline);
paragraph_mark_run_properties_accessor!(paragraph_mark_run_properties_shadow, Shadow, w::Shadow);
paragraph_mark_run_properties_accessor!(paragraph_mark_run_properties_emboss, Emboss, w::Emboss);
paragraph_mark_run_properties_accessor!(paragraph_mark_run_properties_imprint, Imprint, w::Imprint);
paragraph_mark_run_properties_accessor!(paragraph_mark_run_properties_caps, Caps, w::Caps);
paragraph_mark_run_properties_accessor!(
  paragraph_mark_run_properties_small_caps,
  SmallCaps,
  w::SmallCaps
);
paragraph_mark_run_properties_accessor!(paragraph_mark_run_properties_vanish, Vanish, w::Vanish);
paragraph_mark_run_properties_accessor!(
  paragraph_mark_run_properties_vertical_text_alignment,
  VerticalTextAlignment,
  w::VerticalTextAlignment
);
paragraph_mark_run_properties_accessor!(paragraph_mark_run_properties_spacing, Spacing, w::Spacing);
paragraph_mark_run_properties_accessor!(
  paragraph_mark_run_properties_character_scale,
  CharacterScale,
  w::CharacterScale
);
paragraph_mark_run_properties_accessor!(paragraph_mark_run_properties_kern, Kern, w::Kern);
paragraph_mark_run_properties_accessor!(
  paragraph_mark_run_properties_position,
  Position,
  w::Position
);
paragraph_mark_run_properties_accessor!(
  paragraph_mark_run_properties_highlight,
  Highlight,
  w::Highlight
);
paragraph_mark_run_properties_accessor!(
  paragraph_mark_run_properties_right_to_left_text,
  RightToLeftText,
  w::RightToLeftText
);
paragraph_mark_run_properties_accessor!(
  paragraph_mark_run_properties_complex_script,
  ComplexScript,
  w::ComplexScript
);
paragraph_mark_run_properties_accessor!(
  paragraph_mark_run_properties_languages,
  Languages,
  w::Languages
);

impl<'a> RunProps<'a> {
  fn run_fonts(&self) -> Option<&'a w::RunFonts> {
    match self {
      Self::Direct(properties) => run_properties_run_fonts(properties),
      Self::Style(properties) => properties.run_fonts.as_ref(),
      Self::BaseStyle(properties) => properties.run_fonts.as_ref(),
      Self::Numbering(properties) => properties.run_fonts.first(),
      Self::ParagraphMark(properties) => paragraph_mark_run_properties_run_fonts(properties),
    }
  }

  fn languages(&self) -> Option<&'a w::Languages> {
    match self {
      Self::Direct(properties) => run_properties_languages(properties),
      Self::Style(properties) => properties.languages.as_ref(),
      Self::BaseStyle(properties) => properties.languages.as_ref(),
      Self::Numbering(properties) => properties.languages.as_ref(),
      Self::ParagraphMark(properties) => paragraph_mark_run_properties_languages(properties),
    }
  }

  fn bold(&self) -> Option<&'a w::Bold> {
    match self {
      Self::Direct(properties) => run_properties_bold(properties),
      Self::Style(properties) => properties.bold.as_ref(),
      Self::BaseStyle(properties) => properties.bold.as_ref(),
      Self::Numbering(properties) => properties.bold.as_ref(),
      Self::ParagraphMark(properties) => paragraph_mark_run_properties_bold(properties),
    }
  }

  fn bold_complex_script(&self) -> Option<&'a w::BoldComplexScript> {
    match self {
      Self::Direct(properties) => run_properties_bold_complex_script(properties),
      Self::Style(properties) => properties.bold_complex_script.as_ref(),
      Self::BaseStyle(properties) => properties.bold_complex_script.as_ref(),
      Self::Numbering(properties) => properties.bold_complex_script.as_ref(),
      Self::ParagraphMark(properties) => {
        paragraph_mark_run_properties_bold_complex_script(properties)
      }
    }
  }

  fn italic(&self) -> Option<&'a w::Italic> {
    match self {
      Self::Direct(properties) => run_properties_italic(properties),
      Self::Style(properties) => properties.italic.as_ref(),
      Self::BaseStyle(properties) => properties.italic.as_ref(),
      Self::Numbering(properties) => properties.italic.as_ref(),
      Self::ParagraphMark(properties) => paragraph_mark_run_properties_italic(properties),
    }
  }

  fn italic_complex_script(&self) -> Option<&'a w::ItalicComplexScript> {
    match self {
      Self::Direct(properties) => run_properties_italic_complex_script(properties),
      Self::Style(properties) => properties.italic_complex_script.as_ref(),
      Self::BaseStyle(properties) => properties.italic_complex_script.as_ref(),
      Self::Numbering(properties) => properties.italic_complex_script.as_ref(),
      Self::ParagraphMark(properties) => {
        paragraph_mark_run_properties_italic_complex_script(properties)
      }
    }
  }

  fn complex_script(&self) -> Option<&'a w::ComplexScript> {
    match self {
      Self::Direct(properties) => run_properties_complex_script(properties),
      Self::Style(_) | Self::BaseStyle(_) => None,
      Self::Numbering(properties) => properties.complex_script.as_ref(),
      Self::ParagraphMark(properties) => paragraph_mark_run_properties_complex_script(properties),
    }
  }

  fn right_to_left_text(&self) -> Option<&'a w::RightToLeftText> {
    match self {
      Self::Direct(properties) => run_properties_right_to_left_text(properties),
      Self::Style(properties) => properties.right_to_left_text.as_ref(),
      Self::BaseStyle(properties) => properties.right_to_left_text.as_ref(),
      Self::Numbering(properties) => properties.right_to_left_text.as_ref(),
      Self::ParagraphMark(properties) => {
        paragraph_mark_run_properties_right_to_left_text(properties)
      }
    }
  }

  fn font_size(&self) -> Option<&'a w::FontSize> {
    match self {
      Self::Direct(properties) => run_properties_font_size(properties),
      Self::Style(properties) => properties.font_size.as_ref(),
      Self::BaseStyle(properties) => properties.font_size.as_ref(),
      Self::Numbering(properties) => properties.font_size.as_ref(),
      Self::ParagraphMark(properties) => paragraph_mark_run_properties_font_size(properties),
    }
  }

  fn complex_script_font_size(&self) -> Option<&'a w::FontSizeComplexScript> {
    match self {
      Self::Direct(properties) => run_properties_complex_script_font_size(properties),
      Self::Style(properties) => properties.font_size_complex_script.as_ref(),
      Self::BaseStyle(properties) => properties.font_size_complex_script.as_ref(),
      Self::Numbering(properties) => properties.font_size_complex_script.as_ref(),
      Self::ParagraphMark(properties) => {
        paragraph_mark_run_properties_complex_script_font_size(properties)
      }
    }
  }

  fn color(&self) -> Option<&'a w::Color> {
    match self {
      Self::Direct(properties) => run_properties_color(properties),
      Self::Style(properties) => properties.color.as_ref(),
      Self::BaseStyle(properties) => properties.color.as_ref(),
      Self::Numbering(properties) => properties.color.as_ref(),
      Self::ParagraphMark(properties) => paragraph_mark_run_properties_color(properties),
    }
  }

  fn shading(&self) -> Option<&'a w::Shading> {
    match self {
      Self::Direct(properties) => run_properties_shading(properties),
      Self::Style(properties) => properties.shading.as_ref(),
      Self::BaseStyle(properties) => properties.shading.as_ref(),
      Self::Numbering(properties) => properties.shading.as_ref(),
      Self::ParagraphMark(properties) => paragraph_mark_run_properties_shading(properties),
    }
  }

  fn underline(&self) -> Option<&'a w::Underline> {
    match self {
      Self::Direct(properties) => run_properties_underline(properties),
      Self::Style(properties) => properties.underline.as_ref(),
      Self::BaseStyle(properties) => properties.underline.as_ref(),
      Self::Numbering(properties) => properties.underline.as_ref(),
      Self::ParagraphMark(properties) => paragraph_mark_run_properties_underline(properties),
    }
  }

  fn strike(&self) -> Option<&'a w::Strike> {
    match self {
      Self::Direct(properties) => run_properties_strike(properties),
      Self::Style(properties) => properties.strike.as_ref(),
      Self::BaseStyle(properties) => properties.strike.as_ref(),
      Self::Numbering(properties) => properties.strike.as_ref(),
      Self::ParagraphMark(properties) => paragraph_mark_run_properties_strike(properties),
    }
  }

  fn double_strike(&self) -> Option<&'a w::DoubleStrike> {
    match self {
      Self::Direct(properties) => run_properties_double_strike(properties),
      Self::Style(properties) => properties.double_strike.as_ref(),
      Self::BaseStyle(properties) => properties.double_strike.as_ref(),
      Self::Numbering(properties) => properties.double_strike.as_ref(),
      Self::ParagraphMark(properties) => paragraph_mark_run_properties_double_strike(properties),
    }
  }

  fn outline(&self) -> Option<&'a w::Outline> {
    match self {
      Self::Direct(properties) => run_properties_outline(properties),
      Self::Style(properties) => properties.outline.as_ref(),
      Self::BaseStyle(properties) => properties.outline.as_ref(),
      Self::Numbering(properties) => properties.outline.as_ref(),
      Self::ParagraphMark(properties) => paragraph_mark_run_properties_outline(properties),
    }
  }

  fn shadow(&self) -> Option<&'a w::Shadow> {
    match self {
      Self::Direct(properties) => run_properties_shadow(properties),
      Self::Style(properties) => properties.shadow.as_ref(),
      Self::BaseStyle(properties) => properties.shadow.as_ref(),
      Self::Numbering(properties) => properties.shadow.as_ref(),
      Self::ParagraphMark(properties) => paragraph_mark_run_properties_shadow(properties),
    }
  }

  fn emboss(&self) -> Option<&'a w::Emboss> {
    match self {
      Self::Direct(properties) => run_properties_emboss(properties),
      Self::Style(properties) => properties.emboss.as_ref(),
      Self::BaseStyle(properties) => properties.emboss.as_ref(),
      Self::Numbering(properties) => properties.emboss.as_ref(),
      Self::ParagraphMark(properties) => paragraph_mark_run_properties_emboss(properties),
    }
  }

  fn imprint(&self) -> Option<&'a w::Imprint> {
    match self {
      Self::Direct(properties) => run_properties_imprint(properties),
      Self::Style(properties) => properties.imprint.as_ref(),
      Self::BaseStyle(properties) => properties.imprint.as_ref(),
      Self::Numbering(properties) => properties.imprint.as_ref(),
      Self::ParagraphMark(properties) => paragraph_mark_run_properties_imprint(properties),
    }
  }

  fn caps(&self) -> Option<&'a w::Caps> {
    match self {
      Self::Direct(properties) => run_properties_caps(properties),
      Self::Style(properties) => properties.caps.as_ref(),
      Self::BaseStyle(properties) => properties.caps.as_ref(),
      Self::Numbering(properties) => properties.caps.as_ref(),
      Self::ParagraphMark(properties) => paragraph_mark_run_properties_caps(properties),
    }
  }

  fn small_caps(&self) -> Option<&'a w::SmallCaps> {
    match self {
      Self::Direct(properties) => run_properties_small_caps(properties),
      Self::Style(properties) => properties.small_caps.as_ref(),
      Self::BaseStyle(properties) => properties.small_caps.as_ref(),
      Self::Numbering(properties) => properties.small_caps.as_ref(),
      Self::ParagraphMark(properties) => paragraph_mark_run_properties_small_caps(properties),
    }
  }

  fn vanish(&self) -> Option<&'a w::Vanish> {
    match self {
      Self::Direct(properties) => run_properties_vanish(properties),
      Self::Style(properties) => properties.vanish.as_ref(),
      Self::BaseStyle(properties) => properties.vanish.as_ref(),
      Self::Numbering(properties) => properties.vanish.as_ref(),
      Self::ParagraphMark(properties) => paragraph_mark_run_properties_vanish(properties),
    }
  }

  fn vertical_text_alignment(&self) -> Option<&'a w::VerticalTextAlignment> {
    match self {
      Self::Direct(properties) => run_properties_vertical_text_alignment(properties),
      Self::Style(properties) => properties.vertical_text_alignment.as_ref(),
      Self::BaseStyle(properties) => properties.vertical_text_alignment.as_ref(),
      Self::Numbering(properties) => properties.vertical_text_alignment.as_ref(),
      Self::ParagraphMark(properties) => {
        paragraph_mark_run_properties_vertical_text_alignment(properties)
      }
    }
  }

  fn spacing(&self) -> Option<&'a w::Spacing> {
    match self {
      Self::Direct(properties) => run_properties_spacing(properties),
      Self::Style(properties) => properties.spacing.as_ref(),
      Self::BaseStyle(properties) => properties.spacing.as_ref(),
      Self::Numbering(properties) => properties.spacing.as_ref(),
      Self::ParagraphMark(properties) => paragraph_mark_run_properties_spacing(properties),
    }
  }

  fn kern(&self) -> Option<&'a w::Kern> {
    match self {
      Self::Direct(properties) => run_properties_kern(properties),
      Self::Style(properties) => properties.kern.as_ref(),
      Self::BaseStyle(properties) => properties.kern.as_ref(),
      Self::Numbering(properties) => properties.kern.as_ref(),
      Self::ParagraphMark(properties) => paragraph_mark_run_properties_kern(properties),
    }
  }

  fn character_scale(&self) -> Option<&'a w::CharacterScale> {
    match self {
      Self::Direct(properties) => run_properties_character_scale(properties),
      Self::Style(properties) => properties.character_scale.as_ref(),
      Self::BaseStyle(properties) => properties.character_scale.as_ref(),
      Self::Numbering(properties) => properties.character_scale.as_ref(),
      Self::ParagraphMark(properties) => paragraph_mark_run_properties_character_scale(properties),
    }
  }

  fn position(&self) -> Option<&'a w::Position> {
    match self {
      Self::Direct(properties) => run_properties_position(properties),
      Self::Style(properties) => properties.position.as_ref(),
      Self::BaseStyle(properties) => properties.position.as_ref(),
      Self::Numbering(properties) => properties.position.as_ref(),
      Self::ParagraphMark(properties) => paragraph_mark_run_properties_position(properties),
    }
  }

  fn ligatures(&self) -> Option<&'a w14::Ligatures> {
    match self {
      Self::Direct(properties) => properties.ligatures.as_ref(),
      Self::Style(properties) => properties.ligatures.as_ref(),
      Self::BaseStyle(properties) => properties.ligatures.as_ref(),
      Self::Numbering(properties) => properties.ligatures.as_ref(),
      Self::ParagraphMark(properties) => properties.ligatures.as_ref(),
    }
  }

  fn text_fill(&self) -> Option<&'a w14::FillTextEffect> {
    match self {
      Self::Direct(properties) => properties.fill_text_effect.as_deref(),
      Self::Style(properties) => properties.fill_text_effect.as_deref(),
      Self::BaseStyle(_) => None,
      Self::Numbering(properties) => properties.fill_text_effect.as_deref(),
      Self::ParagraphMark(properties) => properties.fill_text_effect.as_deref(),
    }
  }

  fn text_outline(&self) -> Option<&'a w14::TextOutlineEffect> {
    match self {
      Self::Direct(properties) => properties.text_outline_effect.as_deref(),
      Self::Style(properties) => properties.text_outline_effect.as_deref(),
      Self::BaseStyle(_) => None,
      Self::Numbering(properties) => properties.text_outline_effect.as_deref(),
      Self::ParagraphMark(properties) => properties.text_outline_effect.as_deref(),
    }
  }

  fn text_glow(&self) -> Option<&'a w14::Glow> {
    match self {
      Self::Direct(properties) => properties.glow.as_deref(),
      Self::Style(properties) => properties.glow.as_deref(),
      Self::BaseStyle(_) => None,
      Self::Numbering(properties) => properties.glow.as_deref(),
      Self::ParagraphMark(properties) => properties.glow.as_deref(),
    }
  }

  fn text_shadow(&self) -> Option<&'a w14::Shadow> {
    match self {
      Self::Direct(properties) => properties.shadow14.as_deref(),
      Self::Style(properties) => properties.shadow14.as_deref(),
      Self::BaseStyle(_) => None,
      Self::Numbering(properties) => properties.shadow14.as_deref(),
      Self::ParagraphMark(properties) => properties.shadow.as_deref(),
    }
  }

  fn text_reflection(&self) -> Option<&'a w14::Reflection> {
    match self {
      Self::Direct(properties) => properties.reflection.as_ref(),
      Self::Style(properties) => properties.reflection.as_ref(),
      Self::BaseStyle(_) => None,
      Self::Numbering(properties) => properties.reflection.as_ref(),
      Self::ParagraphMark(properties) => properties.reflection.as_ref(),
    }
  }

  fn highlight(&self) -> Option<&'a w::Highlight> {
    match self {
      Self::Direct(properties) => run_properties_highlight(properties),
      Self::ParagraphMark(properties) => paragraph_mark_run_properties_highlight(properties),
      Self::Style(_) | Self::BaseStyle(_) | Self::Numbering(_) => None,
    }
  }
}

fn parse_hex_color(value: &str) -> Option<RgbColor> {
  if value.eq_ignore_ascii_case("auto") {
    return None;
  }

  let expanded;
  let hex = if value.len() == 3 {
    expanded = value.chars().flat_map(|ch| [ch, ch]).collect::<String>();
    expanded.as_str()
  } else {
    value
  };

  if hex.len() != 6 {
    return None;
  }

  Some(RgbColor {
    r: u8::from_str_radix(&hex[0..2], 16).ok()?,
    g: u8::from_str_radix(&hex[2..4], 16).ok()?,
    b: u8::from_str_radix(&hex[4..6], 16).ok()?,
  })
}

fn twips_measure_to_twips(value: &TwipsMeasureValue) -> Option<f32> {
  Some(value.to_twips() as f32)
}

fn signed_twips_measure_to_twips(value: &SignedTwipsMeasureValue) -> Option<f32> {
  Some(value.to_twips() as f32)
}

fn twips_measure_to_points(value: &TwipsMeasureValue) -> Option<f32> {
  twips_measure_to_twips(value).map(units::twips_to_points)
}

fn signed_twips_measure_to_points(value: &SignedTwipsMeasureValue) -> Option<f32> {
  signed_twips_measure_to_twips(value).map(units::twips_to_points)
}

fn measurement_or_percent_to_points(value: &MeasurementOrPercentValue) -> Option<f32> {
  measurement_or_percent_to_twips(value).map(units::twips_to_points)
}

fn table_margin_measurement_to_points(value: &MeasurementOrPercentValue) -> Option<f32> {
  let twips = measurement_or_percent_to_twips(value)?;
  (0.0..=MAX_WORD_TABLE_MARGIN_TWIPS)
    .contains(&twips)
    .then(|| units::twips_to_points(twips))
}

fn measurement_or_percent_to_twips(value: &MeasurementOrPercentValue) -> Option<f32> {
  value.to_twips().map(|twips| twips as f32)
}

fn measurement_or_percent_to_percent(value: &MeasurementOrPercentValue) -> Option<f32> {
  value.as_word_ratio().map(|ratio| ratio as f32)
}

fn drawingml_percent_to_ratio(value: &DrawingmlPercentageValue) -> Option<f32> {
  Some(value.as_ratio() as f32)
}

fn page_setup(section: &w::SectionProperties) -> PageSetup {
  // [MS-OI29500] 2.1.220 (ECMA-376 Part 1 §17.6.13) requires Word's
  // w:pgSz width and height to be no greater than 31680 twips (22 inches).
  const WORD_MAX_PAGE_SIZE_PT: f32 = 1_584.0;

  let mut setup = default_word_page_setup();

  if let Some(size) = &section.page_size {
    if let Some(width) = size.width.as_ref().and_then(twips_measure_to_points) {
      setup.width_pt = width.min(WORD_MAX_PAGE_SIZE_PT);
    }
    if let Some(height) = size.height.as_ref().and_then(twips_measure_to_points) {
      setup.height_pt = height.min(WORD_MAX_PAGE_SIZE_PT);
    }
  }

  if let Some(margin) = &section.page_margin {
    if let Some(top) = margin.top.as_ref().and_then(signed_twips_measure_to_twips) {
      setup.top_margin_was_negative = top < 0.0;
      // stores the absolute page margin and uses the sign only to disable
      // dynamic header height / convert header content to a fly frame.
      setup.margin_top_pt = units::twips_to_points(top.abs());
    }
    if let Some(right) = margin.right.as_ref().and_then(twips_measure_to_points) {
      setup.margin_right_pt = right;
    }
    if let Some(bottom) = margin
      .bottom
      .as_ref()
      .and_then(signed_twips_measure_to_twips)
    {
      setup.bottom_margin_was_negative = bottom < 0.0;
      setup.margin_bottom_pt = units::twips_to_points(bottom.abs());
    }
    if let Some(left) = margin.left.as_ref().and_then(twips_measure_to_points) {
      setup.margin_left_pt = left;
    }
    if let Some(gutter) = margin.gutter.as_ref().and_then(twips_measure_to_points) {
      setup.gutter_pt = gutter;
    }
    if let Some(header) = margin.header.as_ref().and_then(twips_measure_to_points) {
      setup.header_distance_pt = header;
    }
    if let Some(footer) = margin.footer.as_ref().and_then(twips_measure_to_points) {
      setup.footer_distance_pt = footer;
    }
  }

  if let Some(borders) = &section.page_borders {
    setup.borders = page_borders_model(borders);
    setup.borders_offset_from_text =
      matches!(borders.offset_from, Some(w::PageBorderOffsetValues::Text));
  }

  setup.line_numbering = section
    .line_number_type
    .as_ref()
    .and_then(line_numbering_model);
  setup.page_number_start = section
    .page_number_type
    .as_ref()
    .and_then(|page_number| page_number.start);
  setup.page_number_format = section
    .page_number_type
    .as_ref()
    .and_then(|page_number| page_number.format)
    .map(page_style_field_number_format)
    .unwrap_or(FieldNumberFormat::Decimal);
  setup.rtl_gutter = section
    .gutter_on_right
    .as_ref()
    .is_some_and(|value| value.val.is_none_or(|value| value.as_bool()));
  setup.doc_grid_line_pitch_pt = section
    .doc_grid
    .as_ref()
    .filter(|grid| {
      matches!(
        grid.r#type,
        Some(
          w::DocGridValues::Lines | w::DocGridValues::LinesAndChars | w::DocGridValues::SnapToChars
        )
      )
    })
    .and_then(|grid| grid.line_pitch)
    .filter(|pitch| *pitch > 0)
    .map(|pitch| units::twips_to_points(pitch as f32));
  setup.doc_grid_character_spacing_pt = section
    .doc_grid
    .as_ref()
    .filter(|grid| {
      matches!(
        grid.r#type,
        Some(w::DocGridValues::LinesAndChars | w::DocGridValues::SnapToChars)
      )
    })
    .and_then(|grid| grid.character_space)
    .and_then(doc_grid_character_spacing_points);

  setup
}

fn page_style_field_number_format(format: w::NumberFormatValues) -> FieldNumberFormat {
  match format {
    w::NumberFormatValues::LowerRoman => FieldNumberFormat::LowerRoman,
    w::NumberFormatValues::UpperRoman => FieldNumberFormat::UpperRoman,
    w::NumberFormatValues::LowerLetter => FieldNumberFormat::LowerLetter,
    w::NumberFormatValues::UpperLetter => FieldNumberFormat::UpperLetter,
    _ => FieldNumberFormat::Decimal,
  }
}

fn doc_grid_character_spacing_points(value: i64) -> Option<f32> {
  let value = i32::try_from(value).ok()?;
  // w:charSpace is Writer's sep.dxtCharSpace: a signed 20.12 fixed-point
  // number whose integral and fractional parts are measured in points.
  // LibreOffice decodes the same value in SectionPropertyMap::CloseSectionGroup.
  let integral = (value & !0x0fff) / 0x1000;
  let fraction = value & 0x0fff;
  Some(integral as f32 + fraction as f32 / 0x0fff as f32)
}

fn default_word_page_setup() -> PageSetup {
  default_word_page_setup_with_size(PageSetup {
    // LibreOffice's OOXML importer initializes an omitted w:pgSz to
    // PAPER_LETTER, matching Microsoft Office fixed output for such sections.
    width_pt: 612.0,
    height_pt: 792.0,
    ..PageSetup::default()
  })
}

fn default_word_page_setup_with_size(mut setup: PageSetup) -> PageSetup {
  // Word's default section properties use 1.25-inch horizontal margins and
  // one-inch vertical margins. Apache POI preserves the same defaults in
  // SEPAbstractType (dxaLeft/dxaRight=1800, dyaTop/dyaBottom=1440).
  setup.margin_left_pt = 90.0;
  setup.margin_right_pt = 90.0;
  setup
}

fn line_numbering_model(properties: &w::LineNumberType) -> Option<LineNumbering> {
  // ECMA-376 Part 1 §17.6.8 disables line numbering when countBy is absent.
  let count_by = properties.count_by?;
  if count_by <= 0 {
    return None;
  }
  Some(LineNumbering {
    count_by,
    // [MS-OI29500] §2.1.215 defines Word's start value as the number of
    // logical line numbers skipped on every restart. Keep that authored skip
    // count here; layout starts at start + 1.
    start: properties.start.unwrap_or(0),
    distance_pt: properties
      .distance
      .as_ref()
      .and_then(twips_measure_to_points)
      .unwrap_or(OFFICE_AUTOMATIC_LINE_NUMBER_DISTANCE_PT),
    restart_each_page: matches!(
      properties.restart,
      None | Some(w::LineNumberRestartValues::NewPage)
    ),
  })
}

#[cfg(test)]
mod tests {
  use super::*;

  fn twips(value: u32) -> TwipsMeasureValue {
    TwipsMeasureValue::Twips(value as u64)
  }

  fn merge_test_paragraph(text: &str) -> Paragraph {
    Paragraph {
      inlines: vec![InlineItem::Text(TextRun {
        text: text.into(),
        style: TextStyle::default(),
        hyperlink_url: None,
        dynamic_field: None,
        style_ref_keys: Vec::new(),
        style_ref_text: None,
        style_ref_numbering_text: None,
        preserve_text_portion: false,
      })],
      field_events: Vec::new(),
      footnote_reference_ids: Vec::new(),
      endnote_reference_ids: Vec::new(),
      starts_after_last_rendered_page_break: false,
      base_style: TextStyle::default(),
      runs: Vec::new(),
      format: Box::new(ParagraphFormat::default()),
      style_ref_keys: Vec::new(),
      style_ref_text: None,
      style_ref_numbering_text: None,
      list_label: None,
      list_label_style: TextStyle::default(),
      list_label_hyperlink_url: None,
      list_label_tab_stop_pt: None,
    }
  }

  #[test]
  fn deleted_paragraph_mark_combines_current_text_with_the_following_paragraph() {
    let mut first = merge_test_paragraph("master.");
    first.format.deleted_separator = true;
    let second = merge_test_paragraph("Basketball");
    let mut blocks = Vec::new();

    push_body_paragraph(&mut blocks, first);
    push_body_paragraph(&mut blocks, second);

    let [Block::Paragraph(paragraph)] = blocks.as_slice() else {
      panic!("one combined paragraph");
    };
    let text = paragraph
      .inlines
      .iter()
      .filter_map(|inline| match inline {
        InlineItem::Text(run) => Some(run.text.as_str()),
        _ => None,
      })
      .collect::<String>();
    assert_eq!(text, "master.Basketball");
    assert!(!paragraph.format.deleted_separator);
    assert_eq!(paragraph.format.outline_text_inlines, None);
  }

  #[test]
  fn zero_note_id_is_normal_only_when_the_authored_type_is_normal() {
    assert!(normal_note_type(None));
    assert!(normal_note_type(Some(w::FootnoteEndnoteValues::Normal)));
    assert!(!normal_note_type(Some(
      w::FootnoteEndnoteValues::ContinuationSeparator
    )));

    let run = w::Run {
      run_choice: vec![
        w::RunChoice::FootnoteReference(w::FootnoteReference {
          id: 0,
          ..Default::default()
        }),
        w::RunChoice::EndnoteReference(w::EndnoteReference {
          id: 0,
          ..Default::default()
        }),
      ],
      ..Default::default()
    };
    let mut footnotes = Vec::new();
    let mut endnotes = Vec::new();
    collect_run_note_reference_ids(&run, &mut footnotes, &mut endnotes);
    assert_eq!(footnotes, [0]);
    assert_eq!(endnotes, [0]);

    let mut inlines = Vec::new();
    push_note_reference(&mut inlines, 0, TextStyle::default(), None);
    assert!(matches!(
      inlines.as_slice(),
      [InlineItem::Text(TextRun { text, .. })] if text == "0"
    ));
  }

  fn scene_with_revolution(revolution: i32) -> a::Scene3DType {
    a::Scene3DType {
      camera: Box::new(a::Camera {
        preset: a::PresetCameraValues::OrthographicFront,
        rotation: Some(a::Rotation {
          latitude: 0,
          longitude: 0,
          revolution,
        }),
        ..a::Camera::default()
      }),
      light_rig: Box::new(a::LightRig::default()),
      ..a::Scene3DType::default()
    }
  }

  #[test]
  fn scene_only_camera_revolution_rotates_the_flat_shape_face() {
    let properties = DrawingMlShapeProperties::Generic(a::ShapeProperties {
      scene3_d_type: Some(Box::new(scene_with_revolution(4_800_000))),
      ..a::ShapeProperties::default()
    });

    assert!((properties.camera_adjusted_rotation_deg(0.0) + 80.0).abs() < 0.001);
  }

  #[test]
  fn shape_3d_keeps_camera_rotation_in_the_static_3d_pipeline() {
    let properties = DrawingMlShapeProperties::Generic(a::ShapeProperties {
      scene3_d_type: Some(Box::new(scene_with_revolution(4_800_000))),
      shape3_d_type: Some(Box::new(a::Shape3DType::default())),
      ..a::ShapeProperties::default()
    });

    assert_eq!(properties.camera_adjusted_rotation_deg(12.0), 12.0);
  }

  #[test]
  fn embedded_vml_metafile_uses_the_host_shape_solid_fill_as_its_background() {
    let object = w::EmbeddedObject::from_bytes(
      br##"<w:object xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"
        xmlns:v="urn:schemas-microsoft-com:vml">
        <v:shapetype id="_x0000_t75" filled="f"/>
        <v:shape type="#_x0000_t75" filled="t" fillcolor="red"/>
      </w:object>"##,
    )
    .expect("embedded VML object");

    assert_eq!(
      embedded_object_metafile_background_color(&object),
      Some([255, 0, 0])
    );
  }

  #[test]
  fn vml_named_colors_match_the_office_basic_palette() {
    for (name, rgb) in [
      ("aqua", [0, 255, 255]),
      ("black", [0, 0, 0]),
      ("blue", [0, 0, 255]),
      ("fuchsia", [255, 0, 255]),
      ("gray", [128, 128, 128]),
      ("green", [0, 128, 0]),
      ("lime", [0, 255, 0]),
      ("maroon", [128, 0, 0]),
      ("navy", [0, 0, 128]),
      ("olive", [128, 128, 0]),
      ("purple", [128, 0, 128]),
      ("red", [255, 0, 0]),
      ("silver", [192, 192, 192]),
      ("teal", [0, 128, 128]),
      ("white", [255, 255, 255]),
      ("yellow", [255, 255, 0]),
    ] {
      let color = parse_vml_color(name).expect("Office VML named color");
      assert_eq!([color.r, color.g, color.b], rgb, "{name}");
    }
  }

  #[test]
  fn vml_rectangle_uses_default_white_fill_and_black_stroke() {
    let rectangle = v::Rectangle::from_bytes(
      br#"<v:rect xmlns:v="urn:schemas-microsoft-com:vml"
          style="width:100pt;height:50pt"/>"#,
    )
    .expect("VML rectangle");
    let shape =
      vml_rectangle_shape(&rectangle, &ImageCatalog::default()).expect("painted rectangle");

    assert_eq!(
      shape.fill_color,
      Some(RgbColor {
        r: u8::MAX,
        g: u8::MAX,
        b: u8::MAX,
      })
    );
    let Some(common::Fill::Solid(fill)) = shape.fill_override.as_deref() else {
      panic!("default solid fill");
    };
    assert_eq!([fill.r, fill.g, fill.b, fill.a], [255, 255, 255, 255]);
    let stroke = shape.stroke_override.as_deref().expect("default stroke");
    assert!((stroke.width.0 - 0.75).abs() < 0.001);
    assert_eq!(
      [
        stroke.color.r,
        stroke.color.g,
        stroke.color.b,
        stroke.color.a
      ],
      [0, 0, 0, 255]
    );

    let unpainted = v::Rectangle::from_bytes(
      br#"<v:rect xmlns:v="urn:schemas-microsoft-com:vml"
          style="width:100pt;height:50pt" filled="f" stroked="f"/>"#,
    )
    .expect("unpainted VML rectangle");
    assert!(vml_rectangle_shape(&unpainted, &ImageCatalog::default()).is_none());
  }

  #[test]
  fn vml_path_resolves_formulas_quadrants_and_path_paint_groups() {
    let formulas = v::Formulas::from_bytes(
      br#"<v:formulas xmlns:v="urn:schemas-microsoft-com:vml"><v:f eqn="sum 33030 0 #0"/><v:f eqn="prod #0 4 3"/><v:f eqn="prod @0 1 3"/><v:f eqn="sum @1 0 @2"/></v:formulas>"#,
    )
    .expect("VML formulas");
    let geometry = vml_path_geometry(
      "m10800,0qx0,10800,10800,21600,21600,10800,10800,0xe\
       m7340,6445qx6215,7570,7340,8695,8465,7570,7340,6445xnfe\
       m4960@0c8853@3,12747@3,16640@0nfe",
      VmlPathGeometryOptions {
        coordinate_origin: Some("0,0"),
        coordinate_size: Some("21600,21600"),
        width_pt: 216.0,
        height_pt: 216.0,
        adjustment: Some("17520"),
        formulas: Some(&formulas),
        allow_fill: true,
        allow_stroke: true,
        allow_extrusion: false,
      },
    )
    .expect("formula-backed VML path");
    let InlineShapeGeometry::Path { paths, .. } = geometry else {
      panic!("expected VML path geometry");
    };

    assert_eq!(paths.len(), 3);
    assert_eq!(paths[0].fill_mode, common::DrawingPathFillMode::Normal);
    assert!(paths[0].stroke);
    assert_eq!(paths[1].fill_mode, common::DrawingPathFillMode::None);
    assert!(paths[1].stroke);
    assert_eq!(paths[2].fill_mode, common::DrawingPathFillMode::None);
    assert!(paths[2].stroke);
    assert!(
      paths
        .iter()
        .flat_map(|path| &path.commands)
        .any(|command| matches!(command, common::PathCommand::CubicTo { .. }))
    );
  }

  #[test]
  fn vml_canonical_line_path_defaults_omitted_moveto_to_origin() {
    let geometry = vml_path_geometry(
      "m,l21600,21600e",
      VmlPathGeometryOptions {
        coordinate_origin: None,
        coordinate_size: Some("21600,21600"),
        width_pt: 100.0,
        height_pt: 0.0,
        adjustment: None,
        formulas: None,
        allow_fill: false,
        allow_stroke: true,
        allow_extrusion: false,
      },
    )
    .expect("ECMA-376 canonical VML line path");
    let InlineShapeGeometry::Path { paths, .. } = geometry else {
      panic!("expected VML path geometry");
    };
    assert!(matches!(
      paths[0].commands.as_slice(),
      [
        common::PathCommand::MoveTo(common::Point {
          x: common::Pt(0.0),
          y: common::Pt(0.0)
        }),
        common::PathCommand::LineTo(common::Point {
          x: common::Pt(100.0),
          y: common::Pt(0.0)
        })
      ]
    ));
  }

  #[test]
  fn vml_path_consecutive_commas_supply_omitted_zero_parameters() {
    let geometry = vml_path_geometry(
      "m0,0c10,10,,,25,13e",
      VmlPathGeometryOptions {
        coordinate_origin: Some("0,0"),
        coordinate_size: Some("25,13"),
        width_pt: 25.0,
        height_pt: 13.0,
        adjustment: None,
        formulas: None,
        allow_fill: true,
        allow_stroke: true,
        allow_extrusion: false,
      },
    )
    .expect("VML path with omitted zero parameters");
    let InlineShapeGeometry::Path { paths, .. } = geometry else {
      panic!("expected VML path geometry");
    };

    assert!(matches!(
      paths[0].commands.as_slice(),
      [
        common::PathCommand::MoveTo(common::Point {
          x: common::Pt(0.0),
          y: common::Pt(0.0)
        }),
        common::PathCommand::CubicTo {
          control1: common::Point {
            x: common::Pt(10.0),
            y: common::Pt(10.0)
          },
          control2: common::Point {
            x: common::Pt(0.0),
            y: common::Pt(0.0)
          },
          end: common::Point {
            x: common::Pt(25.0),
            y: common::Pt(13.0)
          }
        }
      ]
    ));
  }

  #[test]
  fn vml_curve_is_retained_as_cubic_shape_geometry() {
    let curve = v::Curve::from_bytes(
      br##"<v:curve xmlns:v="urn:schemas-microsoft-com:vml"
        style="width:20pt;height:10pt" from="0,0" control1="0,100"
        control2="200,0" to="200,100" fillcolor="#112233" strokecolor="#445566"/>"##,
    )
    .expect("VML curve");
    let shape = vml_special_shape(
      crate::xlsx::object_resources::vml_curve_model(&curve),
      curve.style.as_deref(),
    )
    .expect("VML curve shape");
    let InlineShapeGeometry::Path { paths, .. } = shape.geometry else {
      panic!("curve must lower to a path");
    };
    assert!(matches!(
      paths[0].commands.as_slice(),
      [
        common::PathCommand::MoveTo(_),
        common::PathCommand::CubicTo { .. }
      ]
    ));
  }

  #[test]
  fn vml_path_lowers_clockwise_and_counterclockwise_arcs() {
    let geometry = vml_path_geometry(
      "m0,50wa0,0,100,100,0,50,100,50e\
       m100,50at0,0,100,100,100,50,0,50e",
      VmlPathGeometryOptions {
        coordinate_origin: Some("0,0"),
        coordinate_size: Some("100,100"),
        width_pt: 100.0,
        height_pt: 100.0,
        adjustment: None,
        formulas: None,
        allow_fill: true,
        allow_stroke: true,
        allow_extrusion: false,
      },
    )
    .expect("VML arc paths");
    let InlineShapeGeometry::Path { paths, .. } = geometry else {
      panic!("expected VML path geometry");
    };

    assert_eq!(paths.len(), 2);
    assert!(paths.iter().all(|path| {
      path
        .commands
        .iter()
        .any(|command| matches!(command, common::PathCommand::CubicTo { .. }))
    }));
  }

  #[test]
  fn table_cell_writing_modes_rotate_in_the_declared_flow_direction() {
    let bottom_to_top = w::TableCellProperties::from_bytes(
      br#"<w:tcPr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:textDirection w:val="btLr"/></w:tcPr>"#,
    )
    .expect("bottom-to-top table-cell properties");
    let top_to_bottom = w::TableCellProperties::from_bytes(
      br#"<w:tcPr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:textDirection w:val="tbRl"/></w:tcPr>"#,
    )
    .expect("top-to-bottom table-cell properties");

    assert_eq!(
      table_cell_text_rotation_degrees(&bottom_to_top),
      Some(-90.0)
    );
    assert_eq!(table_cell_text_rotation_degrees(&top_to_bottom), Some(90.0));
  }

  fn signed_twips(value: i64) -> SignedTwipsMeasureValue {
    SignedTwipsMeasureValue::Twips(value)
  }

  fn hps(value: u64) -> ooxmlsdk::simple_type::HpsMeasureValue {
    ooxmlsdk::simple_type::HpsMeasureValue::HalfPoints(value)
  }

  fn measurement(value: i32) -> MeasurementOrPercentValue {
    MeasurementOrPercentValue::DecimalNumberOrPercent(
      ooxmlsdk::simple_type::DecimalNumberOrPercentValue::DecimalNumber(value.into()),
    )
  }

  fn text(value: &str) -> w::Text {
    w::Text(w::TextType {
      space: Some(xml::SpaceProcessingModeValues::Preserve),
      xml_content: Some(value.into()),
    })
  }

  #[test]
  fn drawingml_wrap_through_remains_distinct_from_square_wrap() {
    let through = wp::WrapThrough::from_bytes(
      br#"<wp:wrapThrough xmlns:wp="http://schemas.openxmlformats.org/drawingml/2006/wordprocessingDrawing" wrapText="bothSides"><wp:wrapPolygon edited="0"><wp:start x="0" y="0"/><wp:lineTo x="0" y="0"/></wp:wrapPolygon></wp:wrapThrough>"#,
    )
    .expect("through wrap");
    let square = wp::WrapSquare::from_bytes(
      br#"<wp:wrapSquare xmlns:wp="http://schemas.openxmlformats.org/drawingml/2006/wordprocessingDrawing" wrapText="bothSides"/>"#,
    )
    .expect("square wrap");

    assert_eq!(
      image_wrap_mode(&wp::AnchorChoice::WrapThrough(Box::new(through))),
      ImageWrapMode::Through
    );
    assert_eq!(
      image_wrap_mode(&wp::AnchorChoice::WrapSquare(Box::new(square))),
      ImageWrapMode::Square
    );
  }

  #[test]
  fn word_line_numbering_preserves_skip_count_and_automatic_distance() {
    let automatic = w::LineNumberType::from_bytes(
      br#"<w:lnNumType xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" w:countBy="1"/>"#,
    )
    .expect("automatic line numbering");
    let explicit = w::LineNumberType::from_bytes(
      br#"<w:lnNumType xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" w:countBy="3" w:start="1" w:distance="720"/>"#,
    )
    .expect("explicit line-number distance");
    let disabled = w::LineNumberType::from_bytes(
      br#"<w:lnNumType xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"/>"#,
    )
    .expect("disabled line numbering");

    assert_eq!(
      line_numbering_model(&automatic)
        .expect("line numbering")
        .distance_pt,
      18.0
    );
    let explicit = line_numbering_model(&explicit).expect("line numbering");
    assert_eq!(explicit.count_by, 3);
    assert_eq!(explicit.start, 1);
    assert_eq!(explicit.distance_pt, 36.0);
    assert_eq!(line_numbering_model(&disabled), None);
  }

  #[test]
  fn zero_character_indent_does_not_protect_style_indent_from_numbering() {
    let properties = w::ParagraphProperties::from_bytes(
      br#"<w:pPr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:ind w:firstLineChars="0"/></w:pPr>"#,
    )
    .expect("paragraph properties");
    let context = NumberingFormatMergeContext::from_direct_properties(Some(
      ParagraphProps::Direct(&properties),
    ));
    assert!(!context.has_direct_indentation());

    let mut target = ParagraphFormat {
      first_line_indent_pt: 21.0,
      first_line_indent_set: true,
      ..Default::default()
    };
    let level = ParagraphFormat {
      indent_left_pt: 21.0,
      indent_left_set: true,
      first_line_indent_pt: -21.0,
      first_line_indent_set: true,
      ..Default::default()
    };

    merge_numbering_format_values(&mut target, &level, context);

    assert_eq!(target.indent_left_pt, 21.0);
    assert_eq!(target.first_line_indent_pt, -21.0);
  }

  #[test]
  fn direct_numbering_indent_protection_is_attribute_specific() {
    let properties = w::ParagraphProperties::from_bytes(
      br#"<w:pPr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:ind w:left="720"/></w:pPr>"#,
    )
    .expect("paragraph properties");
    let context = NumberingFormatMergeContext::from_direct_properties(Some(
      ParagraphProps::Direct(&properties),
    ));

    assert!(context.direct_indent_left);
    assert!(!context.direct_indent_right);
    assert!(!context.direct_first_line_indent);
  }

  #[test]
  fn direct_numbering_id_zero_stops_style_numbering() {
    let style = NumberingReference {
      num_id: Some(5),
      level_index: Some(1),
    };

    let (selected, style_applies, cancelled) = select_paragraph_numbering(
      Some(NumberingReference {
        num_id: Some(0),
        level_index: None,
      }),
      Some(style),
    );

    assert_eq!(selected.and_then(|reference| reference.num_id), None);
    assert!(!style_applies);
    assert!(cancelled);

    let styles = StylesCatalog {
      styles: HashMap::from([
        (
          "Parent".to_string(),
          StyleEntry {
            paragraph_format: ParagraphFormat {
              indent_left_pt: 28.35,
              indent_left_set: true,
              first_line_indent_pt: -28.35,
              first_line_indent_set: true,
              ..Default::default()
            },
            ..Default::default()
          },
        ),
        (
          "Child".to_string(),
          StyleEntry {
            based_on: Some("Parent".to_string()),
            paragraph_numbering: Some(Box::default()),
            ..Default::default()
          },
        ),
      ]),
      ..Default::default()
    };

    assert_eq!(
      styles.paragraph_indents_without_numbering(Some("Child")),
      ((0.0, None), (0.0, None))
    );
  }

  #[test]
  fn paragraph_auto_script_spacing_properties_follow_style_overlay_order() {
    let inherited = w::ParagraphProperties::from_bytes(
      br#"<w:pPr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:autoSpaceDE/><w:autoSpaceDN w:val="0"/></w:pPr>"#,
    )
    .expect("inherited paragraph properties");
    let direct = w::ParagraphProperties::from_bytes(
      br#"<w:pPr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:autoSpaceDN/></w:pPr>"#,
    )
    .expect("direct paragraph properties");

    let mut inherited_format = ParagraphFormat::default();
    merge_paragraph_format(
      &mut inherited_format,
      Some(ParagraphProps::Direct(&inherited)),
      ImportSettings::default(),
    );
    assert_eq!(inherited_format.auto_space_de, Some(true));
    assert_eq!(inherited_format.auto_space_dn, Some(false));

    let mut direct_format = ParagraphFormat::default();
    merge_paragraph_format(
      &mut direct_format,
      Some(ParagraphProps::Direct(&direct)),
      ImportSettings::default(),
    );
    merge_format_values(&mut inherited_format, &direct_format);

    assert_eq!(inherited_format.auto_space_de, Some(true));
    assert_eq!(inherited_format.auto_space_dn, Some(true));
  }

  #[test]
  fn character_unit_indents_follow_word_style_hierarchy_rules() {
    let inherited = w::ParagraphProperties::from_bytes(
      br#"<w:pPr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:ind w:leftChars="300" w:rightChars="200" w:firstLineChars="200"/></w:pPr>"#,
    )
    .expect("inherited paragraph properties");
    let physical_overlay = w::ParagraphProperties::from_bytes(
      br#"<w:pPr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:ind w:left="1440" w:right="720" w:hangingChars="100" w:firstLineChars="400"/></w:pPr>"#,
    )
    .expect("physical paragraph properties");
    let zero_overlay = w::ParagraphProperties::from_bytes(
      br#"<w:pPr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:ind w:leftChars="0" w:rightChars="0" w:firstLineChars="0"/></w:pPr>"#,
    )
    .expect("zero paragraph properties");

    let mut inherited_format = ParagraphFormat::default();
    merge_paragraph_format(
      &mut inherited_format,
      Some(ParagraphProps::Direct(&inherited)),
      ImportSettings::default(),
    );
    let mut overlay_format = ParagraphFormat::default();
    merge_paragraph_format(
      &mut overlay_format,
      Some(ParagraphProps::Direct(&physical_overlay)),
      ImportSettings::default(),
    );
    merge_format_values(&mut inherited_format, &overlay_format);

    // [MS-OI29500] §2.1.87 keeps an earlier non-zero character indent
    // ahead of a later physical indent. ECMA-376 §17.3.1.12 gives hanging
    // character indentation precedence when both first-line forms are present.
    assert_eq!(inherited_format.indent_left_pt, 72.0);
    assert_eq!(inherited_format.indent_left_character_units, Some(3.0));
    assert_eq!(inherited_format.indent_right_pt, 36.0);
    assert_eq!(inherited_format.indent_right_character_units, Some(2.0));
    assert_eq!(
      inherited_format.first_line_indent_character_units,
      Some(-1.0)
    );

    let mut zero_format = ParagraphFormat::default();
    merge_paragraph_format(
      &mut zero_format,
      Some(ParagraphProps::Direct(&zero_overlay)),
      ImportSettings::default(),
    );
    merge_format_values(&mut inherited_format, &zero_format);
    assert_eq!(inherited_format.indent_left_character_units, Some(0.0));
    assert_eq!(inherited_format.indent_right_character_units, Some(0.0));
    assert_eq!(
      inherited_format.first_line_indent_character_units,
      Some(0.0)
    );
  }

  #[test]
  fn tab_stop_style_overlays_preserve_parent_positions_and_apply_direct_clears() {
    let parent = w::StyleParagraphProperties::from_bytes(
      br#"<w:pPr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:tabs><w:tab w:val="left" w:pos="567"/><w:tab w:val="left" w:pos="1701"/><w:tab w:val="left" w:pos="2835"/><w:tab w:val="left" w:pos="5669"/></w:tabs></w:pPr>"#,
    )
    .expect("parent paragraph properties");
    let child = w::StyleParagraphProperties::from_bytes(
      br#"<w:pPr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:tabs><w:tab w:val="left" w:pos="144"/><w:tab w:val="left" w:pos="288"/><w:tab w:val="left" w:pos="432"/></w:tabs></w:pPr>"#,
    )
    .expect("child paragraph properties");
    let direct = w::ParagraphProperties::from_bytes(
      br#"<w:pPr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:tabs><w:tab w:val="clear" w:pos="144"/><w:tab w:val="clear" w:pos="288"/><w:tab w:val="clear" w:pos="432"/><w:tab w:val="left" w:pos="1440"/></w:tabs></w:pPr>"#,
    )
    .expect("direct paragraph properties");

    let mut parent_format = ParagraphFormat::default();
    merge_paragraph_format(
      &mut parent_format,
      Some(ParagraphProps::Style(&parent)),
      ImportSettings::default(),
    );
    let mut child_format = ParagraphFormat::default();
    merge_paragraph_format(
      &mut child_format,
      Some(ParagraphProps::Style(&child)),
      ImportSettings::default(),
    );
    let mut resolved = ParagraphFormat::default();
    merge_format_values(&mut resolved, &parent_format);
    merge_format_values(&mut resolved, &child_format);
    merge_paragraph_format(
      &mut resolved,
      Some(ParagraphProps::Direct(&direct)),
      ImportSettings::default(),
    );

    let positions = resolved
      .tab_stops
      .iter()
      .map(|stop| stop.position_pt)
      .collect::<Vec<_>>();
    assert_eq!(positions, vec![28.35, 72.0, 85.05, 141.75, 283.45]);
  }

  #[test]
  fn vml_group_textbox_anchor_includes_parent_absolute_offset() {
    let transform = VmlGroupTransform::new(3_165.0, 3_599.0, 0.05, 0.05);
    let style = transform
      .child_anchor_style(
        Some("position:absolute;margin-left:86.25pt;margin-top:94.9pt"),
        Some("position:absolute;left:8640;top:3599;width:2259;height:705"),
      )
      .expect("transformed child style");
    let parsed = vml_image_style(Some(&style));

    assert!(parsed.absolute_position);
    assert_eq!(
      parsed.horizontal_relative_to,
      HorizontalImageReference::Column
    );
    assert_eq!(
      parsed.vertical_relative_to,
      VerticalImageReference::Paragraph
    );
    assert!((parsed.horizontal_offset_pt - 360.0).abs() < 0.001);
    assert!((parsed.vertical_offset_pt - 94.9).abs() < 0.001);
    let (width, height) = parsed.size_pt.expect("transformed size");
    assert!((width - 112.95).abs() < 0.001);
    assert!((height - 35.25).abs() < 0.001);
  }

  #[test]
  fn inline_vml_group_retains_an_unpainted_flow_frame() {
    let inline = v::Group::from_bytes(
      br#"<v:group xmlns:v="urn:schemas-microsoft-com:vml"
          style="width:453.6pt;height:141.05pt;
                 mso-position-horizontal-relative:char;
                 mso-position-vertical-relative:line"
          coordorigin="1417,1417" coordsize="9072,2821"/>"#,
    )
    .expect("inline VML group");
    let frame = vml_inline_group_frame(&inline).expect("inline group frame");

    assert!((frame.width_pt - 453.6).abs() < 0.001);
    assert!((frame.height_pt - 141.05).abs() < 0.001);
    assert!(matches!(frame.placement, ImagePlacement::Inline));
    assert!(frame.fill_color.is_none());
    assert!(frame.fill_image.is_none());
    assert!(frame.stroke.is_none());
    let child_style = VmlGroupTransform::from_group(&inline)
      .expect("inline group transform")
      .child_anchor_style(
        inline.style.as_deref(),
        Some("position:absolute;left:1798;top:2760;width:2370;height:1333"),
      )
      .expect("inline group child style");
    let child = vml_image_style(Some(&child_style));
    assert_eq!(
      child.horizontal_relative_to,
      HorizontalImageReference::Character
    );
    assert_eq!(
      child.vertical_relative_to,
      VerticalImageReference::Paragraph
    );
    assert!((child.horizontal_offset_pt - 19.05).abs() < 0.001);
    assert!((child.vertical_offset_pt - 67.15).abs() < 0.001);

    let floating = v::Group::from_bytes(
      br#"<v:group xmlns:v="urn:schemas-microsoft-com:vml"
          style="position:absolute;left:12pt;top:18pt;width:100pt;height:50pt"/>"#,
    )
    .expect("floating VML group");
    assert!(vml_inline_group_frame(&floating).is_none());
  }

  #[test]
  fn vml_group_image_uses_the_group_coordinate_transform() {
    let group = v::Group::from_bytes(
      br#"<v:group xmlns:v="urn:schemas-microsoft-com:vml"
          xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
          style="width:100pt;height:50pt" coordorigin="10,20" coordsize="1000,500">
        <v:shape style="position:absolute;left:110;top:70;width:200;height:100">
          <v:imagedata r:id="rId1"/>
        </v:shape>
      </v:group>"#,
    )
    .expect("VML image group");
    let mut images = ImageCatalog::default();
    images.by_relationship_id.insert(
      "rId1".into(),
      package::ImageResource {
        data: vec![1, 2, 3].into(),
        content_type: Some("image/png".into()),
      },
    );

    let image = group_image(&group, &images).expect("transformed group image");
    assert!((image.width_pt - 20.0).abs() < 0.001);
    assert!((image.height_pt - 10.0).abs() < 0.001);
    let ImagePlacement::Floating(placement) = image.placement else {
      panic!("positioned VML group image");
    };
    assert_eq!(
      placement.horizontal_relative_to,
      HorizontalImageReference::Character
    );
    assert_eq!(
      placement.vertical_relative_to,
      VerticalImageReference::Paragraph
    );
    assert!((placement.horizontal_offset_pt - 10.0).abs() < 0.001);
    assert!((placement.vertical_offset_pt - 5.0).abs() < 0.001);
  }

  #[test]
  fn word_text_edge_whitespace_requires_xml_space_preserve() {
    let default_text = w::TextType {
      xml_content: Some(" Page ".into()),
      ..Default::default()
    };
    let preserved_text = w::TextType {
      space: Some(xml::SpaceProcessingModeValues::Preserve),
      xml_content: Some(" Page ".into()),
    };

    assert_eq!(word_text_value(&default_text, false), Some("Page"));
    assert_eq!(word_text_value(&default_text, true), Some(" Page "));
    assert_eq!(word_text_value(&preserved_text, false), Some(" Page "));
  }

  #[test]
  fn word_text_literal_tabs_remain_distinct_from_tab_elements() {
    let default_text = w::TextType {
      xml_content: Some("\tA\t\tline\t".to_string()),
      space: None,
    };
    let preserved_text = w::TextType {
      xml_content: Some("\tA\t\tline\t".to_string()),
      space: Some(xml::SpaceProcessingModeValues::Preserve),
    };
    let mut output = String::new();
    append_word_text(&mut output, &default_text, false);
    assert_eq!(output, "A  line");

    output.clear();
    append_word_text(&mut output, &preserved_text, false);
    assert_eq!(
      output,
      format!("{0}A{0}{0}line{0}", PRESERVED_WORD_TEXT_TAB)
    );
  }

  #[test]
  fn word_text_line_endings_are_imported_as_spaces() {
    let preserved_text = w::TextType {
      xml_content: Some("before\r\nafter\nlast".to_string()),
      space: Some(xml::SpaceProcessingModeValues::Preserve),
    };
    let mut output = String::new();
    append_word_text(&mut output, &preserved_text, false);
    assert_eq!(output, "before after last");
  }

  #[test]
  fn html_alt_chunk_imports_block_text_and_inline_breaks() {
    assert_eq!(
      html_alt_chunk_paragraphs(
        b"<html><body><p>first &amp; second<br/>line</p><div>third</div></body></html>",
        Some("text/html; charset=utf-8"),
      ),
      vec!["first & second line", "third"]
    );
  }

  #[test]
  fn html_alt_chunk_uses_html_entities_and_excludes_head_content() {
    assert_eq!(
      html_alt_chunk_paragraphs(
        b"<html><head><title>hidden</title><style>p { color: red; }</style></head><body><p>Milj&ouml;bilaga&nbsp;X</p></body></html>",
        Some("text/html; charset=utf-8"),
      ),
      vec!["Miljöbilaga X"]
    );
  }

  #[test]
  fn smart_tag_run_preserves_nested_visible_text() {
    let paragraph = w::Paragraph::from_bytes(
      br#"<w:p xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:smartTag w:uri="urn:test" w:element="person"><w:r><w:t>John</w:t></w:r><w:smartTag w:uri="urn:test" w:element="surname"><w:r><w:t xml:space="preserve"> Smith</w:t></w:r></w:smartTag></w:smartTag></w:p>"#,
    )
    .expect("smart-tag paragraph");
    let styles = StylesCatalog::default();
    let images = ImageCatalog::default();
    let hyperlinks = HyperlinkCatalog::default();
    let mut form_widget_ids = FormWidgetIdAllocator::default();
    let inlines = paragraph_inlines(
      &paragraph,
      TextStyle::default(),
      &styles,
      &images,
      &hyperlinks,
      &CustomXmlBindings::default(),
      &mut form_widget_ids,
    );
    let visible_text = inlines
      .iter()
      .filter_map(|inline| match inline {
        InlineItem::Text(run) => Some(run.text.as_str()),
        _ => None,
      })
      .collect::<String>();

    assert_eq!(visible_text, "John Smith");
  }

  #[test]
  fn office_math_paragraph_defaults_to_center_group_alignment() {
    let paragraph = w::Paragraph::from_bytes(
      br#"<w:p xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:m="http://schemas.openxmlformats.org/officeDocument/2006/math"><m:oMathPara><m:oMath><m:r><m:t>x</m:t></m:r></m:oMath></m:oMathPara></w:p>"#,
    )
    .expect("math paragraph");

    assert_eq!(
      math_paragraph_alignment(&paragraph, None),
      Some(ParagraphAlignment::Center)
    );
  }

  #[test]
  fn document_default_frame_properties_do_not_create_text_frames() {
    let defaults = w::ParagraphPropertiesBaseStyle::from_bytes(
      br#"<w:pPr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:framePr w:w="0" w:h="0" w:hRule="exact" w:xAlign="left" w:vAnchor="margin"/></w:pPr>"#,
    )
    .expect("document default paragraph properties");
    let mut format = ParagraphFormat::default();

    merge_paragraph_format(
      &mut format,
      Some(ParagraphProps::BaseStyle(&defaults)),
      ImportSettings::default(),
    );

    assert!(format.frame.is_none());
  }

  #[test]
  fn word_nonzero_frame_height_defaults_to_at_least() {
    fn imported_height_rule(xml: &[u8]) -> FrameHeightRule {
      let properties =
        w::ParagraphProperties::from_bytes(xml).expect("direct paragraph properties");
      let mut format = ParagraphFormat::default();
      merge_paragraph_format(
        &mut format,
        Some(ParagraphProps::Direct(&properties)),
        ImportSettings::default(),
      );
      format.frame.expect("text frame").height_rule
    }

    assert_eq!(
      imported_height_rule(
        br#"<w:pPr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:framePr w:h="1440"/></w:pPr>"#,
      ),
      FrameHeightRule::AtLeast
    );
    assert_eq!(
      imported_height_rule(
        br#"<w:pPr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:framePr w:h="0"/></w:pPr>"#,
      ),
      FrameHeightRule::Auto
    );
    assert_eq!(
      imported_height_rule(
        br#"<w:pPr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:framePr w:h="1440" w:hRule="auto"/></w:pPr>"#,
      ),
      FrameHeightRule::Auto
    );
  }

  #[test]
  fn office_math_only_paragraph_uses_document_display_math_alignment() {
    let paragraph = w::Paragraph::from_bytes(
      br#"<w:p xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:m="http://schemas.openxmlformats.org/officeDocument/2006/math"><m:oMath><m:r><m:t>x</m:t></m:r></m:oMath></w:p>"#,
    )
    .expect("display math");

    assert_eq!(
      math_paragraph_alignment(&paragraph, Some(ParagraphAlignment::Right)),
      Some(ParagraphAlignment::Right)
    );
  }

  #[test]
  fn inline_math_does_not_override_mixed_paragraph_alignment() {
    let paragraph = w::Paragraph::from_bytes(
      br#"<w:p xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:m="http://schemas.openxmlformats.org/officeDocument/2006/math"><w:r><w:t>value </w:t></w:r><m:oMath><m:r><m:t>x</m:t></m:r></m:oMath></w:p>"#,
    )
    .expect("inline math");

    assert_eq!(
      math_paragraph_alignment(&paragraph, Some(ParagraphAlignment::Center)),
      None
    );
  }

  #[test]
  fn empty_word_run_does_not_disable_display_math_alignment() {
    let paragraph = w::Paragraph::from_bytes(
      br#"<w:p xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:m="http://schemas.openxmlformats.org/officeDocument/2006/math"><w:r><w:rPr/></w:r><m:oMath><m:r><m:t>x</m:t></m:r></m:oMath></w:p>"#,
    )
    .expect("display math after empty word run");

    assert_eq!(
      math_paragraph_alignment(&paragraph, Some(ParagraphAlignment::Center)),
      Some(ParagraphAlignment::Center)
    );
  }

  #[test]
  fn empty_word_run_alone_does_not_enable_display_math_alignment() {
    let paragraph = w::Paragraph::from_bytes(
      br#"<w:p xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:r><w:rPr/></w:r></w:p>"#,
    )
    .expect("empty word run");

    assert_eq!(
      math_paragraph_alignment(&paragraph, Some(ParagraphAlignment::Center)),
      None
    );
  }

  #[test]
  fn table_row_exposes_cells_wrapped_in_content_controls() {
    let row = w::TableRow::from_bytes(
      br#"<w:tr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:sdt><w:sdtContent><w:tc><w:p><w:r><w:t>controlled cell</w:t></w:r></w:p></w:tc></w:sdtContent></w:sdt></w:tr>"#,
    )
    .expect("row with cell-level content control");

    assert_eq!(table_row_cells(&row).len(), 1);
  }

  #[test]
  fn cell_level_content_control_refreshes_its_cached_text_from_data_binding() {
    const CORE_PROPERTIES_ID: &str = "{6C3C8BC8-F283-45AE-878A-BAB7291924A1}";
    let table = w::Table::from_bytes(
      format!(
        r#"<w:tbl xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:tblGrid><w:gridCol w:w="4000"/></w:tblGrid><w:tr><w:sdt><w:sdtPr><w:dataBinding w:xpath="/cp:coreProperties[1]/dc:title[1]" w:storeItemID="{CORE_PROPERTIES_ID}"/><w:text/></w:sdtPr><w:sdtContent><w:tc><w:p><w:r><w:t>cached title</w:t></w:r></w:p></w:tc></w:sdtContent></w:sdt></w:tr></w:tbl>"#
      )
      .as_bytes(),
    )
    .expect("table with a data-bound cell content control");
    let bindings = CustomXmlBindings::from_test_xml(
      Some(CORE_PROPERTIES_ID),
      r#"<cp:coreProperties xmlns:cp="urn:core" xmlns:dc="urn:dc"><dc:title>Bound title</dc:title></cp:coreProperties>"#,
    );
    let mut numbering = NumberingCatalog::default();
    let mut form_widget_ids = FormWidgetIdAllocator::default();
    let model = table_model(
      &table,
      &mut TableModelEnv {
        styles: &StylesCatalog::default(),
        numbering: &mut numbering,
        images: &ImageCatalog::default(),
        hyperlinks: &HyperlinkCatalog::default(),
        custom_xml_bindings: &bindings,
        form_widget_ids: &mut form_widget_ids,
      },
      TableModelContext {
        nested_table_level: 1,
        in_header_footer: false,
      },
    );

    let [Block::Paragraph(paragraph)] = model.rows[0].cells[0].blocks.as_slice() else {
      panic!("expected the controlled table cell paragraph");
    };
    assert_eq!(inline_text(&paragraph.inlines), "Bound title");
  }

  #[test]
  fn numbering_style_link_resolves_through_the_numbering_style_instance() {
    let styles = StylesCatalog {
      styles: HashMap::from([(
        "Numbered".to_string(),
        StyleEntry {
          style_type: Some(w::StyleValues::Numbering),
          paragraph_numbering: Some(Box::new(w::NumberingProperties {
            numbering_id: Some(w::NumberingId { val: 1 }),
            ..Default::default()
          })),
          ..Default::default()
        },
      )]),
      ..Default::default()
    };
    let mut catalog = NumberingCatalog {
      abstract_nums: HashMap::from([
        (
          0,
          AbstractNumbering {
            numbering_style_link: Some("Numbered".to_string()),
            ..Default::default()
          },
        ),
        (
          1,
          AbstractNumbering {
            style_link: Some("Numbered".to_string()),
            ..Default::default()
          },
        ),
        (2, AbstractNumbering::default()),
      ]),
      nums: HashMap::from([
        (
          1,
          NumberingInstance {
            abstract_num_id: 2,
            overrides: HashMap::new(),
          },
        ),
        (
          7,
          NumberingInstance {
            abstract_num_id: 0,
            overrides: HashMap::from([(
              0,
              LevelOverride {
                start: Some(4),
                level: None,
              },
            )]),
          },
        ),
      ]),
      ..Default::default()
    };

    catalog.resolve_style_linked_abstract_nums(&styles);

    let instance = &catalog.nums[&7];
    assert_eq!(instance.abstract_num_id, 2);
    assert_eq!(instance.overrides[&0].start, Some(4));
  }

  #[test]
  fn numbering_style_link_falls_back_to_matching_abstract_style_link() {
    let mut catalog = NumberingCatalog {
      abstract_nums: HashMap::from([
        (
          0,
          AbstractNumbering {
            numbering_style_link: Some("Numbered".to_string()),
            ..Default::default()
          },
        ),
        (
          1,
          AbstractNumbering {
            style_link: Some("Numbered".to_string()),
            ..Default::default()
          },
        ),
      ]),
      nums: HashMap::from([(
        7,
        NumberingInstance {
          abstract_num_id: 0,
          overrides: HashMap::new(),
        },
      )]),
      ..Default::default()
    };

    catalog.resolve_style_linked_abstract_nums(&StylesCatalog::default());

    assert_eq!(catalog.nums[&7].abstract_num_id, 1);
  }

  #[test]
  fn numbering_start_override_resets_shared_abstract_sequence() {
    let level = w::Level::from_bytes(
      br#"<w:lvl xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" w:ilvl="0"><w:start w:val="1"/><w:numFmt w:val="decimal"/><w:lvlText w:val="%1."/></w:lvl>"#,
    )
    .expect("decimal numbering level");
    let mut catalog = NumberingCatalog {
      abstract_nums: HashMap::from([(
        4,
        AbstractNumbering {
          levels: HashMap::from([(0, numbering_level_model(&level, ImportSettings::default()))]),
          ..Default::default()
        },
      )]),
      nums: HashMap::from([
        (
          5,
          NumberingInstance {
            abstract_num_id: 4,
            overrides: HashMap::new(),
          },
        ),
        (
          6,
          NumberingInstance {
            abstract_num_id: 4,
            overrides: HashMap::from([(
              0,
              LevelOverride {
                start: Some(1),
                level: None,
              },
            )]),
          },
        ),
      ]),
      ..Default::default()
    };
    let styles = StylesCatalog::default();
    let labels = [5, 5, 6, 5].map(|num_id| {
      catalog
        .next_label(
          NumberingReference {
            num_id: Some(num_id),
            level_index: Some(0),
          },
          &mut ParagraphFormat::default(),
          &styles,
          TextStyle::default(),
          None,
          NumberingFormatMergeContext::default(),
        )
        .expect("numbering label")
        .text
        .expect("text numbering label")
    });

    assert_eq!(labels, ["1.\t", "2.\t", "1.\t", "2.\t"]);
  }

  #[test]
  fn numbering_start_override_initializes_each_instance_only_once() {
    let level = w::Level::from_bytes(
      br#"<w:lvl xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" w:ilvl="0"><w:start w:val="1"/><w:numFmt w:val="decimal"/><w:lvlText w:val="%1"/></w:lvl>"#,
    )
    .expect("decimal numbering level");
    let mut catalog = NumberingCatalog {
      abstract_nums: HashMap::from([(
        4,
        AbstractNumbering {
          levels: HashMap::from([(0, numbering_level_model(&level, ImportSettings::default()))]),
          ..Default::default()
        },
      )]),
      nums: [16, 17, 18]
        .into_iter()
        .map(|num_id| {
          (
            num_id,
            NumberingInstance {
              abstract_num_id: 4,
              overrides: HashMap::from([(
                0,
                LevelOverride {
                  start: Some(1),
                  level: None,
                },
              )]),
            },
          )
        })
        .collect(),
      ..Default::default()
    };
    let styles = StylesCatalog::default();
    let labels = [16, 16, 17, 16, 17, 18, 16].map(|num_id| {
      catalog
        .next_label(
          NumberingReference {
            num_id: Some(num_id),
            level_index: Some(0),
          },
          &mut ParagraphFormat::default(),
          &styles,
          TextStyle::default(),
          None,
          NumberingFormatMergeContext::default(),
        )
        .expect("numbering label")
        .text
        .expect("text numbering label")
    });

    assert_eq!(labels, ["1\t", "2\t", "1\t", "2\t", "3\t", "1\t", "2\t"]);
  }

  #[test]
  fn deeper_numbering_level_consumes_an_implicit_parent_start() {
    let levels = [
      br#"<w:lvl xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" w:ilvl="0"><w:start w:val="10"/><w:numFmt w:val="decimal"/><w:lvlText w:val="%1."/></w:lvl>"#
        .as_slice(),
      br#"<w:lvl xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" w:ilvl="1"><w:start w:val="1"/><w:numFmt w:val="decimal"/><w:lvlText w:val="%1.%2."/></w:lvl>"#
        .as_slice(),
    ]
    .into_iter()
    .map(|xml| {
      let level = w::Level::from_bytes(xml).expect("numbering level");
      (
        level.level_index,
        numbering_level_model(&level, ImportSettings::default()),
      )
    })
    .collect();
    let mut catalog = NumberingCatalog {
      abstract_nums: HashMap::from([(
        4,
        AbstractNumbering {
          levels,
          ..Default::default()
        },
      )]),
      nums: HashMap::from([(
        5,
        NumberingInstance {
          abstract_num_id: 4,
          overrides: HashMap::from([
            (
              0,
              LevelOverride {
                start: Some(10),
                level: None,
              },
            ),
            (
              1,
              LevelOverride {
                start: Some(1),
                level: None,
              },
            ),
          ]),
        },
      )]),
      ..Default::default()
    };
    let styles = StylesCatalog::default();
    let labels = [1, 1, 0, 1, 0, 1].map(|level_index| {
      catalog
        .next_label(
          NumberingReference {
            num_id: Some(5),
            level_index: Some(level_index),
          },
          &mut ParagraphFormat::default(),
          &styles,
          TextStyle::default(),
          None,
          NumberingFormatMergeContext::default(),
        )
        .expect("numbering label")
        .text
        .expect("text numbering label")
    });

    assert_eq!(
      labels,
      ["10.1.\t", "10.2.\t", "11.\t", "11.1.\t", "12.\t", "12.1.\t"]
    );
  }

  #[test]
  fn missing_picture_bullet_does_not_fall_back_to_level_text() {
    let level = w::Level::from_bytes(
      br#"<w:lvl xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" w:ilvl="0"><w:start w:val="1"/><w:numFmt w:val="bullet"/><w:lvlText w:val="&#xF0B7;"/><w:lvlPicBulletId w:val="7"/></w:lvl>"#,
    )
    .expect("picture bullet numbering level");
    let mut catalog = NumberingCatalog {
      abstract_nums: HashMap::from([(
        4,
        AbstractNumbering {
          levels: HashMap::from([(0, numbering_level_model(&level, ImportSettings::default()))]),
          ..Default::default()
        },
      )]),
      nums: HashMap::from([(
        5,
        NumberingInstance {
          abstract_num_id: 4,
          overrides: HashMap::new(),
        },
      )]),
      ..Default::default()
    };

    let label = catalog
      .next_label(
        NumberingReference {
          num_id: Some(5),
          level_index: Some(0),
        },
        &mut ParagraphFormat::default(),
        &StylesCatalog::default(),
        TextStyle::default(),
        None,
        NumberingFormatMergeContext::default(),
      )
      .expect("numbering label");

    assert_eq!(label.text, None);
    assert_eq!(label.suppressed_non_numerical_text, None);
    assert!(label.image.is_none());
  }

  #[test]
  fn synthesized_numbering_label_does_not_inherit_paragraph_kerning() {
    let level = w::Level::from_bytes(
      br#"<w:lvl xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" w:ilvl="0"><w:start w:val="1"/><w:numFmt w:val="cardinalText"/><w:lvlText w:val="%1."/></w:lvl>"#,
    )
    .expect("text numbering level");
    let mut catalog = NumberingCatalog {
      abstract_nums: HashMap::from([(
        4,
        AbstractNumbering {
          levels: HashMap::from([(0, numbering_level_model(&level, ImportSettings::default()))]),
          ..Default::default()
        },
      )]),
      nums: HashMap::from([(
        5,
        NumberingInstance {
          abstract_num_id: 4,
          overrides: HashMap::new(),
        },
      )]),
      ..Default::default()
    };
    let label = catalog
      .next_label(
        NumberingReference {
          num_id: Some(5),
          level_index: Some(0),
        },
        &mut ParagraphFormat::default(),
        &StylesCatalog::default(),
        TextStyle {
          kerning_minimum_size_pt: Some(1.0),
          ..Default::default()
        },
        None,
        NumberingFormatMergeContext::default(),
      )
      .expect("numbering label");

    assert_eq!(label.text.as_deref(), Some("One.\t"));
    assert_eq!(label.style.kerning_minimum_size_pt, Some(f32::INFINITY));
  }

  #[test]
  fn empty_numbering_text_tabs_to_the_level_left_indent() {
    let level = w::Level::from_bytes(
      br#"<w:lvl xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" w:ilvl="0"><w:start w:val="1"/><w:numFmt w:val="decimal"/><w:lvlText w:val=""/><w:pPr><w:ind w:left="1432" w:hanging="1432"/></w:pPr></w:lvl>"#,
    )
    .expect("empty numbering level");
    let mut numbering = NumberingCatalog {
      abstract_nums: HashMap::from([(
        1,
        AbstractNumbering {
          levels: HashMap::from([(0, numbering_level_model(&level, ImportSettings::default()))]),
          ..Default::default()
        },
      )]),
      nums: HashMap::from([(
        1,
        NumberingInstance {
          abstract_num_id: 1,
          overrides: HashMap::new(),
        },
      )]),
      ..Default::default()
    };
    let paragraph = w::Paragraph::from_bytes(
      br#"<w:p xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:pPr><w:numPr><w:numId w:val="1"/></w:numPr></w:pPr><w:r><w:t>Here should be tab before</w:t></w:r></w:p>"#,
    )
    .expect("numbered paragraph");
    let mut form_widget_ids = FormWidgetIdAllocator::default();

    let model = paragraph_model(
      &paragraph,
      &StylesCatalog::default(),
      &mut numbering,
      &ImageCatalog::default(),
      &HyperlinkCatalog::default(),
      &CustomXmlBindings::default(),
      &mut form_widget_ids,
    );

    assert_eq!(model.list_label.as_deref(), Some("\t"));
    assert!((model.format.indent_left_pt - 71.6).abs() < 0.001);
    assert!((model.list_label_tab_stop_pt.unwrap_or_default() - 71.6).abs() < 0.001);
  }

  #[test]
  fn numbering_level_bound_to_paragraph_style_owns_the_style_indent() {
    let level = w::Level::from_bytes(
      br#"<w:lvl xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" w:ilvl="0"><w:start w:val="1"/><w:numFmt w:val="decimal"/><w:pStyle w:val="Style1"/><w:lvlText w:val="%1."/><w:pPr><w:ind w:left="360" w:hanging="360"/></w:pPr></w:lvl>"#,
    )
    .expect("style-bound numbering level");
    let mut catalog = NumberingCatalog {
      abstract_nums: HashMap::from([(
        4,
        AbstractNumbering {
          levels: HashMap::from([(0, numbering_level_model(&level, ImportSettings::default()))]),
          ..Default::default()
        },
      )]),
      nums: HashMap::from([(
        5,
        NumberingInstance {
          abstract_num_id: 4,
          overrides: HashMap::new(),
        },
      )]),
      ..Default::default()
    };
    let mut format = ParagraphFormat {
      style_id: Some(Arc::from("Style1")),
      indent_left_pt: 36.0,
      indent_left_set: true,
      ..Default::default()
    };

    catalog
      .next_label(
        NumberingReference {
          num_id: Some(5),
          level_index: Some(0),
        },
        &mut format,
        &StylesCatalog::default(),
        TextStyle::default(),
        None,
        NumberingFormatMergeContext {
          style_numbering: true,
          ..Default::default()
        },
      )
      .expect("numbering label");

    assert_eq!(format.indent_left_pt, 18.0);
    assert_eq!(format.first_line_indent_pt, -18.0);

    let mut locally_indented_format = ParagraphFormat {
      style_id: Some(Arc::from("Style1")),
      indent_left_pt: 0.0,
      indent_left_set: true,
      first_line_indent_pt: 0.0,
      first_line_indent_set: true,
      ..Default::default()
    };
    catalog
      .next_label(
        NumberingReference {
          num_id: Some(5),
          level_index: Some(0),
        },
        &mut locally_indented_format,
        &StylesCatalog::default(),
        TextStyle::default(),
        None,
        NumberingFormatMergeContext {
          style_numbering: true,
          matched_style_indent_left: true,
          matched_style_first_line_indent: true,
          ..Default::default()
        },
      )
      .expect("numbering label");

    assert_eq!(locally_indented_format.indent_left_pt, 0.0);
    assert_eq!(locally_indented_format.first_line_indent_pt, 0.0);
  }

  #[test]
  fn numbering_level_override_replaces_the_abstract_level_format() {
    let base_level = w::Level::from_bytes(
      br#"<w:lvl xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" w:ilvl="0"><w:start w:val="1"/><w:numFmt w:val="decimal"/><w:lvlText w:val="%1"/></w:lvl>"#,
    )
    .expect("base numbering level");
    let override_definition = w::LevelOverride::from_bytes(
      br#"<w:lvlOverride xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" w:ilvl="0"><w:lvl w:ilvl="0"><w:start w:val="1"/><w:numFmt w:val="upperLetter"/><w:lvlText w:val="%1"/></w:lvl></w:lvlOverride>"#,
    )
    .expect("numbering level override");
    let override_level = override_definition
      .level
      .as_deref()
      .expect("full override level");
    let mut catalog = NumberingCatalog {
      abstract_nums: HashMap::from([(
        9,
        AbstractNumbering {
          levels: HashMap::from([(
            0,
            numbering_level_model(&base_level, ImportSettings::default()),
          )]),
          ..Default::default()
        },
      )]),
      nums: HashMap::from([
        (
          19,
          NumberingInstance {
            abstract_num_id: 9,
            overrides: HashMap::new(),
          },
        ),
        (
          22,
          NumberingInstance {
            abstract_num_id: 9,
            overrides: HashMap::from([(
              0,
              LevelOverride {
                start: None,
                level: Some(numbering_level_model(
                  override_level,
                  ImportSettings::default(),
                )),
              },
            )]),
          },
        ),
      ]),
      ..Default::default()
    };
    let styles = StylesCatalog::default();
    let labels = [19, 22, 22].map(|num_id| {
      catalog
        .next_label(
          NumberingReference {
            num_id: Some(num_id),
            level_index: Some(0),
          },
          &mut ParagraphFormat::default(),
          &styles,
          TextStyle::default(),
          None,
          NumberingFormatMergeContext::default(),
        )
        .expect("numbering label")
        .text
        .expect("text numbering label")
    });

    assert_eq!(labels, ["1\t", "B\t", "C\t"]);
  }

  #[test]
  fn numbering_level_without_start_begins_at_zero() {
    let level = w::Level::from_bytes(
      br#"<w:lvl xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" w:ilvl="1"><w:numFmt w:val="decimal"/><w:lvlText w:val="%2"/></w:lvl>"#,
    )
    .expect("numbering level without w:start");

    assert_eq!(
      numbering_level_model(&level, ImportSettings::default()).start,
      0
    );
  }

  #[test]
  fn numbering_level_restart_uses_the_authored_higher_level_threshold() {
    let levels = [
      br#"<w:lvl xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" w:ilvl="0"><w:start w:val="1"/><w:numFmt w:val="decimal"/><w:lvlText w:val="%1"/></w:lvl>"#
        .as_slice(),
      br#"<w:lvl xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" w:ilvl="1"><w:start w:val="1"/><w:numFmt w:val="decimal"/><w:lvlText w:val="%2"/></w:lvl>"#
        .as_slice(),
      br#"<w:lvl xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" w:ilvl="2"><w:start w:val="1"/><w:numFmt w:val="decimal"/><w:lvlRestart w:val="1"/><w:lvlText w:val="%3"/></w:lvl>"#
        .as_slice(),
    ]
    .into_iter()
    .map(|xml| {
      let level = w::Level::from_bytes(xml).expect("numbering level");
      (
        level.level_index,
        numbering_level_model(&level, ImportSettings::default()),
      )
    })
    .collect();
    let mut catalog = NumberingCatalog {
      abstract_nums: HashMap::from([(
        4,
        AbstractNumbering {
          levels,
          ..Default::default()
        },
      )]),
      nums: HashMap::from([(
        5,
        NumberingInstance {
          abstract_num_id: 4,
          overrides: HashMap::new(),
        },
      )]),
      ..Default::default()
    };
    let styles = StylesCatalog::default();
    let labels = [0, 1, 2, 1, 2, 0, 2].map(|level_index| {
      catalog
        .next_label(
          NumberingReference {
            num_id: Some(5),
            level_index: Some(level_index),
          },
          &mut ParagraphFormat::default(),
          &styles,
          TextStyle::default(),
          None,
          NumberingFormatMergeContext::default(),
        )
        .expect("numbering label")
        .text
        .expect("text numbering label")
    });

    assert_eq!(labels, ["1\t", "1\t", "1\t", "2\t", "2\t", "2\t", "1\t"]);
  }

  #[test]
  fn numbering_uses_ecma_enclosed_and_full_width_decimal_sequences() {
    assert_eq!(
      format_numbering_value(1, w::NumberFormatValues::DecimalEnclosedCircle, false),
      "①"
    );
    assert_eq!(
      format_numbering_value(20, w::NumberFormatValues::DecimalEnclosedFullstop, false),
      "⒛"
    );
    assert_eq!(
      format_numbering_value(21, w::NumberFormatValues::DecimalEnclosedParen, false),
      "21"
    );
    assert_eq!(
      format_numbering_value(120, w::NumberFormatValues::DecimalFullWidth, false),
      "１２０"
    );
  }

  #[test]
  fn numbering_uses_source_backed_cjk_sequences_and_decimal_fallbacks() {
    let heavenly_stems = (1..=11)
      .map(|value| {
        format_numbering_value(value, w::NumberFormatValues::IdeographTraditional, false)
      })
      .collect::<Vec<_>>();
    assert_eq!(
      heavenly_stems,
      [
        "甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸", "11"
      ]
    );

    let earthly_branches = (1..=13)
      .map(|value| format_numbering_value(value, w::NumberFormatValues::IdeographZodiac, false))
      .collect::<Vec<_>>();
    assert_eq!(
      earthly_branches,
      [
        "子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戍", "亥", "13"
      ]
    );

    for (value, expected) in [
      (1, "一"),
      (10, "十"),
      (11, "十一"),
      (20, "二十"),
      (101, "一百〇一"),
      (1_010, "一千〇一十"),
      (10_010, "一萬〇一十"),
    ] {
      assert_eq!(
        format_numbering_value(
          value,
          w::NumberFormatValues::TaiwaneseCountingThousand,
          false,
        ),
        expected
      );
    }
    for (value, expected) in [
      (10, "壹拾"),
      (11, "壹拾壹"),
      (20, "貳拾"),
      (101, "壹佰零壹"),
      (10_010, "壹萬零壹拾"),
    ] {
      assert_eq!(
        format_numbering_value(
          value,
          w::NumberFormatValues::IdeographLegalTraditional,
          false,
        ),
        expected
      );
    }
    assert_eq!(
      format_numbering_value(20, w::NumberFormatValues::KoreanDigital2, false),
      "二零"
    );
  }

  #[test]
  fn numbering_uses_custom_greek_alphabetic_sequence() {
    let level = w::Level::from_bytes(
      br#"<w:lvl xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" w:ilvl="0"><w:numFmt w:val="custom" w:format="&#x3B1;, &#x3B2;, &#x3B3;, ..."/></w:lvl>"#,
    )
    .expect("custom Greek numbering level");
    let level = numbering_level_model(&level, ImportSettings::default());

    assert_eq!(format_numbering_level_value(1, &level, false), "α");
    assert_eq!(format_numbering_level_value(3, &level, false), "γ");
    assert_eq!(format_numbering_level_value(25, &level, false), "αα");
  }

  #[test]
  fn numbering_uses_ms_docx_custom_zero_padded_sequences() {
    for (format, expected) in [
      ("001, 002, 003, ...", "001"),
      ("0001, 0002, 0003, ...", "0001"),
      ("00001, 00002, 00003, ...", "00001"),
    ] {
      let level = w::Level::from_bytes(
        format!(
          r#"<w:lvl xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" w:ilvl="0"><w:numFmt w:val="custom" w:format="{format}"/></w:lvl>"#
        )
        .as_bytes(),
      )
      .expect("custom numbering level");
      let level = numbering_level_model(&level, ImportSettings::default());

      assert_eq!(format_numbering_level_value(1, &level, false), expected);
    }
  }

  #[test]
  fn legal_numbering_decimalizes_non_arabic_levels_and_preserves_decimal_zero() {
    let first = w::Level::from_bytes(
      br#"<w:lvl xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" w:ilvl="0"><w:start w:val="1"/><w:numFmt w:val="upperRoman"/><w:lvlText w:val="CH %1"/></w:lvl>"#,
    )
    .expect("first numbering level");
    let second = w::Level::from_bytes(
      br#"<w:lvl xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" w:ilvl="1"><w:start w:val="1"/><w:numFmt w:val="decimalZero"/><w:isLgl/><w:lvlText w:val="Sect %1.%2"/></w:lvl>"#,
    )
    .expect("legal numbering level");
    let first = numbering_level_model(&first, ImportSettings::default());
    let second = numbering_level_model(&second, ImportSettings::default());
    let abstract_num = AbstractNumbering {
      levels: HashMap::from([(0, first), (1, second.clone())]),
      ..Default::default()
    };
    let counters = HashMap::from([((7, 0), 1), ((7, 1), 1)]);

    assert_eq!(
      format_numbering_label(&second, 7, 1, 1, &abstract_num, &HashMap::new(), &counters,),
      "Sect 1.01\t"
    );
  }

  #[test]
  fn styleref_t_switch_preserves_only_number_values_and_delimiters() {
    let first = w::Level::from_bytes(
      br#"<w:lvl xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" w:ilvl="0"><w:start w:val="1"/><w:numFmt w:val="decimal"/><w:lvlText w:val="Chapter %1.!"/></w:lvl>"#,
    )
    .expect("first numbering level");
    let second = w::Level::from_bytes(
      br#"<w:lvl xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" w:ilvl="1"><w:start w:val="1"/><w:numFmt w:val="lowerLetter"/><w:lvlText w:val=".%1\@123^&amp;~|%2......"/></w:lvl>"#,
    )
    .expect("second numbering level");
    let first = numbering_level_model(&first, ImportSettings::default());
    let second = numbering_level_model(&second, ImportSettings::default());
    let abstract_num = AbstractNumbering {
      levels: HashMap::from([(0, first), (1, second.clone())]),
      ..Default::default()
    };
    let counters = HashMap::from([((7, 0), 2), ((7, 1), 1)]);

    assert_eq!(
      format_numbering_label_suppressing_non_numerical(
        &second,
        7,
        1,
        1,
        &abstract_num,
        &HashMap::new(),
        &counters,
      ),
      ".2\\|a......"
    );
    assert!(matches!(
      style_ref_field_kind(&[
        "Heading 2".to_string(),
        "\\t".to_string(),
        "\\w".to_string(),
      ]),
      Some(DynamicFieldKind::StyleRef {
        numbering_only: true,
        suppress_non_numerical: true,
        ..
      })
    ));
    assert!(matches!(
      style_ref_field_kind(&["Foobar".to_string(), "\\w".to_string()]),
      Some(DynamicFieldKind::StyleRef {
        numbering_only: true,
        suppress_non_numerical: false,
        ..
      })
    ));
  }

  #[test]
  fn style_ref_keys_mark_only_authored_custom_style_names() {
    let styles = StylesCatalog {
      simplified_chinese_ui: true,
      styles: HashMap::from([
        (
          "Foobar".to_string(),
          StyleEntry {
            custom_style: true,
            name: Some("Foobar".to_string()),
            ..StyleEntry::default()
          },
        ),
        (
          "Heading1".to_string(),
          StyleEntry {
            name: Some("heading 1".to_string()),
            ..StyleEntry::default()
          },
        ),
      ]),
      ..StylesCatalog::default()
    };

    assert!(
      styles
        .style_ref_keys("Foobar")
        .iter()
        .any(|key| key.as_ref() == "\0custom:Foobar")
    );
    assert!(
      styles
        .style_ref_keys("Heading1")
        .iter()
        .all(|key| !key.starts_with(CUSTOM_STYLE_REF_KEY_PREFIX))
    );
    assert!(!styles.style_ref_name_requires_localized_error("foobar"));
    assert!(styles.style_ref_name_requires_localized_error("Heading 1"));
    assert!(!styles.style_ref_name_requires_localized_error("1"));
  }

  #[test]
  fn numbering_uses_office_english_text_and_ordinal_sequences() {
    assert_eq!(
      format_numbering_value(21, w::NumberFormatValues::Ordinal, false),
      "21st"
    );
    assert_eq!(
      format_numbering_value(20, w::NumberFormatValues::CardinalText, false),
      "Twenty"
    );
    assert_eq!(
      format_numbering_value(21, w::NumberFormatValues::CardinalText, false),
      "Twenty-one"
    );
    assert_eq!(
      format_numbering_value(12, w::NumberFormatValues::OrdinalText, false),
      "Twelfth"
    );
    assert_eq!(
      format_numbering_value(101, w::NumberFormatValues::OrdinalText, false),
      "One hundred first"
    );
  }

  #[test]
  fn numbering_symbol_font_remains_authoritative_under_paragraph_rtl() {
    let properties = w::NumberingSymbolRunProperties {
      run_fonts: vec![w::RunFonts {
        ascii: Some("Symbol".into()),
        high_ansi: Some("Symbol".into()),
        ..Default::default()
      }],
      ..Default::default()
    };
    let inherited = TextStyle {
      font_family: Some(Arc::from("Calibri")),
      complex_font_family: Some(Arc::from("Times New Roman")),
      right_to_left: Some(true),
      ..Default::default()
    };
    let mut style = TextStyle {
      font_family: Some(Arc::from("Symbol")),
      ..inherited.clone()
    };
    let mut text = "\u{f0b7}".to_string();

    finalize_numbering_symbol_transport_style(
      &mut style,
      &inherited,
      w::NumberFormatValues::Bullet,
      Some(&properties),
      &mut text,
    );

    assert_eq!(style.font_family.as_deref(), Some("Symbol"));
    assert_eq!(style.complex_font_family.as_deref(), Some("Symbol"));
    assert_eq!(style.symbol_font_family.as_deref(), Some("Symbol"));
    assert_eq!(style.right_to_left, Some(true));
    assert_eq!(text, "\u{f0b7}");
  }

  #[test]
  fn numbering_symbol_transport_override_requires_matching_direct_evidence() {
    let symbol_properties = w::NumberingSymbolRunProperties {
      run_fonts: vec![w::RunFonts {
        ascii: Some("Symbol".into()),
        high_ansi: Some("Symbol".into()),
        ..Default::default()
      }],
      ..Default::default()
    };
    let ordinary_properties = w::NumberingSymbolRunProperties {
      run_fonts: vec![w::RunFonts {
        ascii: Some("Calibri".into()),
        ..Default::default()
      }],
      ..Default::default()
    };
    let inherited = TextStyle {
      font_family: Some(Arc::from("Calibri")),
      complex_font_family: Some(Arc::from("Times New Roman")),
      ..Default::default()
    };

    for (format, properties, mut text) in [
      (
        w::NumberFormatValues::Decimal,
        &symbol_properties,
        "\u{f0b7}".to_string(),
      ),
      (
        w::NumberFormatValues::Bullet,
        &symbol_properties,
        "•".to_string(),
      ),
      (
        w::NumberFormatValues::Bullet,
        &ordinary_properties,
        "\u{f0b7}".to_string(),
      ),
    ] {
      let mut style = TextStyle {
        font_family: Some(Arc::from("Symbol")),
        ..inherited.clone()
      };
      finalize_numbering_symbol_transport_style(
        &mut style,
        &inherited,
        format,
        Some(properties),
        &mut text,
      );
      assert_eq!(
        style.complex_font_family.as_deref(),
        Some("Times New Roman")
      );
      assert_eq!(style.symbol_font_family, None);
    }

    let mut style = TextStyle {
      font_family: Some(Arc::from("Symbol")),
      complex_font_family: Some(Arc::from("Times New Roman")),
      ..Default::default()
    };
    let mut text = "\u{f094}".to_string();
    finalize_numbering_symbol_transport_style(
      &mut style,
      &inherited,
      w::NumberFormatValues::Bullet,
      Some(&symbol_properties),
      &mut text,
    );
    assert_eq!(text, "■");
    assert_eq!(style.font_family.as_deref(), Some("Calibri"));
    assert_eq!(
      style.complex_font_family.as_deref(),
      Some("Times New Roman")
    );
    assert_eq!(style.symbol_font_family, None);
  }

  #[test]
  fn office_default_font_follows_simplified_chinese_ui_language() {
    assert_eq!(
      office_default_font_family(Some("zh-CN")).as_ref(),
      "DengXian"
    );
    assert_eq!(
      office_default_font_family(Some("zh-Hans-SG")).as_ref(),
      "DengXian"
    );
    assert_eq!(
      office_default_font_family(Some("zh-TW")).as_ref(),
      "Calibri"
    );
    assert_eq!(
      office_default_font_family(Some("en-US")).as_ref(),
      "Calibri"
    );
  }

  #[test]
  fn unclassified_legacy_latin_font_uses_calibri_as_its_document_fallback() {
    let legacy = w::Font::from_bytes(
      br#"<w:font xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" w:name="Legacy Sans"><w:panose1 w:val="00000000000000000000"/><w:family w:val="auto"/><w:notTrueType/></w:font>"#,
    )
    .expect("legacy font table entry");
    let (_, substitution) = font_substitution_from_table_entry(&legacy).expect("substitution");

    assert_eq!(substitution.alternate_family.as_deref(), Some("Calibri"));
    assert_eq!(substitution.family_class, None);
  }

  #[test]
  fn authored_font_alternate_precedes_legacy_latin_recovery() {
    let legacy = w::Font::from_bytes(
      br#"<w:font xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" w:name="Legacy Sans"><w:altName w:val="Arial, Helvetica"/><w:panose1 w:val="00000000000000000000"/><w:family w:val="auto"/><w:notTrueType/></w:font>"#,
    )
    .expect("legacy font table entry");
    let (_, substitution) = font_substitution_from_table_entry(&legacy).expect("substitution");

    assert_eq!(substitution.alternate_family.as_deref(), Some("Arial"));
  }

  #[test]
  fn authored_run_defaults_seed_word_character_sizes_at_ten_points() {
    let authored = word_doc_default_run_seed(true);
    assert_eq!(authored.font_size_pt, 10.0);
    assert_eq!(authored.complex_font_size_pt, Some(10.0));

    let omitted = word_doc_default_run_seed(false);
    assert_eq!(omitted.font_size_pt, 11.0);
    assert_eq!(omitted.complex_font_size_pt, None);
  }

  #[test]
  fn legacy_word_text_effect_toggles_override_inherited_values() {
    let properties = w::RunProperties::from_bytes(
      br#"<w:rPr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:outline/><w:shadow w:val="0"/><w:emboss/><w:imprint w:val="false"/></w:rPr>"#,
    )
    .expect("legacy text effect run properties");
    let base = TextStyle {
      legacy_shadow: true,
      legacy_relief: LegacyTextRelief::Engraved,
      ..TextStyle::default()
    };

    let style = properties::run_style(Some(&properties), base, &StylesCatalog::default());

    assert!(style.legacy_outline);
    assert!(!style.legacy_shadow);
    assert_eq!(style.legacy_relief, LegacyTextRelief::Embossed);
  }

  #[test]
  fn legacy_word_text_effects_survive_style_chains_and_can_be_cleared() {
    let styles = StylesCatalog {
      styles: HashMap::from([
        (
          "Base".to_string(),
          StyleEntry {
            run_style: TextStyle {
              legacy_shadow: true,
              legacy_relief: LegacyTextRelief::Engraved,
              ..TextStyle::default()
            },
            ..StyleEntry::default()
          },
        ),
        (
          "Inherited".to_string(),
          StyleEntry {
            based_on: Some("Base".to_string()),
            ..StyleEntry::default()
          },
        ),
        (
          "Cleared".to_string(),
          StyleEntry {
            based_on: Some("Inherited".to_string()),
            run_overrides: RunStyleOverrides {
              legacy_outline: Some(true),
              legacy_shadow: Some(false),
              legacy_imprint: Some(false),
              ..RunStyleOverrides::default()
            },
            ..StyleEntry::default()
          },
        ),
      ]),
      ..StylesCatalog::default()
    };

    let inherited = styles.run_style_with_base(
      Some("Inherited"),
      TextStyle::default(),
      RunStyleOverrides::default(),
    );
    assert!(inherited.legacy_shadow);
    assert_eq!(inherited.legacy_relief, LegacyTextRelief::Engraved);

    let cleared = styles.run_style_with_base(
      Some("Cleared"),
      TextStyle::default(),
      RunStyleOverrides::default(),
    );
    assert!(cleared.legacy_outline);
    assert!(!cleared.legacy_shadow);
    assert_eq!(cleared.legacy_relief, LegacyTextRelief::None);
  }

  #[test]
  fn word_font_table_family_class_reaches_the_font_request_style() {
    let styles = StylesCatalog {
      font_substitutions: HashMap::from([(
        "metabook-roman".to_string(),
        FontSubstitution {
          alternate_family: None,
          family_class: Some(ooxmlsdk_fonts::FontFamilyClass::Serif),
        },
      )]),
      ..StylesCatalog::default()
    };
    let mut style = TextStyle {
      font_family: Some(Arc::from("MetaBook-Roman")),
      ..TextStyle::default()
    };

    styles.apply_font_substitution(&mut style);

    assert_eq!(
      style.font_family_class,
      Some(ooxmlsdk_fonts::FontFamilyClass::Serif)
    );
    assert_eq!(style.fallback_font_family, None);
  }

  #[test]
  fn reserved_default_font_uses_only_its_authored_font_table_mapping() {
    let properties = w::RunProperties::from_bytes(
      br#"<w:rPr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:rFonts w:ascii="Default" w:hAnsi="Default"/></w:rPr>"#,
    )
    .expect("run properties");
    let base = TextStyle {
      font_family: Some(Arc::from("Calibri")),
      ..TextStyle::default()
    };
    let mapped = StylesCatalog {
      font_substitutions: HashMap::from([(
        "default".to_string(),
        FontSubstitution {
          alternate_family: Some(Arc::from("Cambria")),
          family_class: Some(ooxmlsdk_fonts::FontFamilyClass::Serif),
        },
      )]),
      ..StylesCatalog::default()
    };

    let mapped_style = properties::run_style(Some(&properties), base.clone(), &mapped);

    assert_eq!(mapped_style.font_family.as_deref(), Some("Default"));
    assert_eq!(
      mapped_style.fallback_font_family.as_deref(),
      Some("Cambria")
    );
    assert_eq!(
      mapped_style.font_family_class,
      Some(ooxmlsdk_fonts::FontFamilyClass::Serif)
    );

    let unmapped_style = properties::run_style(Some(&properties), base, &StylesCatalog::default());
    assert_eq!(unmapped_style.font_family.as_deref(), Some("Calibri"));
    assert_eq!(unmapped_style.fallback_font_family, None);
    assert_eq!(unmapped_style.font_family_class, None);
  }

  #[test]
  fn word_theme_font_language_selects_supplemental_east_asian_face() {
    let fonts = ThemeFonts {
      major_ascii: Some(Arc::from("Cambria")),
      major_supplemental: vec![
        (Arc::from("Hans"), Arc::from("SimSun")),
        (Arc::from("Jpan"), Arc::from("MS Gothic")),
      ],
      latin_language: Some(Arc::from("en-US")),
      east_asia_language: Some(Arc::from("zh-CN")),
      ..ThemeFonts::default()
    };

    assert_eq!(
      fonts.resolve(Some(w::ThemeFontValues::MajorAscii)),
      Some(Arc::from("Cambria"))
    );
    assert_eq!(
      fonts.resolve(Some(w::ThemeFontValues::MajorEastAsia)),
      Some(Arc::from("SimSun"))
    );
  }

  #[test]
  fn unresolved_complex_theme_font_uses_word_application_default() {
    let unresolved = w::RunProperties::from_bytes(
      br#"<w:rPr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:rFonts w:cstheme="minorBidi"/></w:rPr>"#,
    )
    .expect("run properties");
    let inherited = TextStyle {
      font_family: Some(Arc::from("Calibri")),
      complex_font_family: Some(Arc::from("Inherited Complex")),
      ..TextStyle::default()
    };

    let unresolved_style = properties::run_style(
      Some(&unresolved),
      inherited.clone(),
      &StylesCatalog::default(),
    );
    assert_eq!(
      unresolved_style.complex_font_family.as_deref(),
      Some("Times New Roman")
    );
    assert_eq!(unresolved_style.font_family.as_deref(), Some("Calibri"));

    let no_complex_reference = w::RunProperties::from_bytes(
      br#"<w:rPr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:rFonts w:hint="cs"/></w:rPr>"#,
    )
    .expect("run properties");
    let inherited_style = properties::run_style(
      Some(&no_complex_reference),
      inherited,
      &StylesCatalog::default(),
    );
    assert_eq!(
      inherited_style.complex_font_family.as_deref(),
      Some("Inherited Complex")
    );
  }

  #[test]
  fn resolved_complex_theme_font_precedes_word_application_default() {
    let properties = w::RunProperties::from_bytes(
      br#"<w:rPr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:rFonts w:cstheme="minorBidi"/></w:rPr>"#,
    )
    .expect("run properties");
    let styles = StylesCatalog {
      theme_fonts: ThemeFonts {
        minor_bidi: Some(Arc::from("David")),
        ..ThemeFonts::default()
      },
      ..StylesCatalog::default()
    };

    let style = properties::run_style(Some(&properties), TextStyle::default(), &styles);
    assert_eq!(style.complex_font_family.as_deref(), Some("David"));
  }

  #[test]
  fn drawingml_east_asian_theme_font_uses_the_output_ui_language_as_fallback() {
    let fonts = ThemeFonts {
      minor_supplemental: vec![
        (Arc::from("Hans"), Arc::from("DengXian")),
        (Arc::from("Jpan"), Arc::from("Yu Mincho")),
      ],
      ..ThemeFonts::default()
    };

    assert_eq!(
      fonts.resolve_drawingml_typeface_for_language("+mn-ea", Some("zh-CN")),
      Arc::from("DengXian")
    );
  }

  #[test]
  fn chart_east_asian_typeface_populates_the_script_specific_font_slot() {
    let properties = c::TextProperties::from_bytes(
      br#"<c:txPr xmlns:c="http://schemas.openxmlformats.org/drawingml/2006/chart" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"><a:bodyPr/><a:lstStyle/><a:p><a:pPr><a:defRPr><a:ea typeface="+mn-ea"/></a:defRPr></a:pPr></a:p></c:txPr>"#,
    )
    .expect("chart text properties");
    let styles = StylesCatalog {
      simplified_chinese_ui: true,
      theme_fonts: ThemeFonts {
        minor_supplemental: vec![(Arc::from("Hans"), Arc::from("DengXian"))],
        ..ThemeFonts::default()
      },
      ..StylesCatalog::default()
    };
    let mut style = TextStyle::default();

    apply_chart_text_properties(&mut style, &properties, &styles);

    assert_eq!(style.east_asia_font_family, Some(Arc::from("DengXian")));
    assert_eq!(style.fallback_font_family, None);
  }

  #[test]
  fn chart_axis_text_properties_apply_size_and_transformed_theme_color() {
    let properties = c::TextProperties::from_bytes(
      br#"<c:txPr xmlns:c="http://schemas.openxmlformats.org/drawingml/2006/chart" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"><a:bodyPr/><a:lstStyle/><a:p><a:pPr><a:defRPr sz="900"><a:solidFill><a:schemeClr val="tx1"><a:lumMod val="65000"/><a:lumOff val="35000"/></a:schemeClr></a:solidFill></a:defRPr></a:pPr></a:p></c:txPr>"#,
    )
    .expect("chart text properties");
    let mut style = TextStyle::default();

    apply_chart_text_properties(&mut style, &properties, &StylesCatalog::default());

    assert_eq!(style.font_size_pt, 9.0);
    assert_eq!(
      style.color,
      RgbColor {
        r: 89,
        g: 89,
        b: 89
      }
    );
  }

  #[test]
  fn drawing_image_properties_preserve_crop_and_transform() {
    let xml = r#"<pic:pic xmlns:pic="http://schemas.openxmlformats.org/drawingml/2006/picture" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"><pic:nvPicPr><pic:cNvPr id="1" name="Picture 1"/><pic:cNvPicPr/></pic:nvPicPr><pic:blipFill><a:blip r:embed="rId7"/><a:srcRect l="10000" t="20000" r="30000" b="40000"/><a:stretch><a:fillRect/></a:stretch></pic:blipFill><pic:spPr><a:xfrm rot="5400000" flipH="1" flipV="true"/></pic:spPr></pic:pic>"#;

    let picture = pic::Picture::from_bytes(xml.as_bytes()).expect("picture");
    let properties = drawing_picture_image_properties(&picture, &ThemeColors::default(), None)
      .expect("image properties");

    assert_eq!(properties.relationship_id.as_deref(), Some("rId7"));
    assert!((properties.crop.left - 0.1).abs() < 0.001);
    assert!((properties.crop.top - 0.2).abs() < 0.001);
    assert!((properties.crop.right - 0.3).abs() < 0.001);
    assert!((properties.crop.bottom - 0.4).abs() < 0.001);
    assert!((properties.rotation_deg - 90.0).abs() < 0.001);
    assert!(properties.flip_horizontal);
    assert!(properties.flip_vertical);
  }

  #[test]
  fn source_rectangle_crop_rounds_bitmap_pixels_and_keeps_outsets() {
    let mut png = Vec::new();
    PngEncoder::new(&mut png)
      .write_image(
        &vec![0; 484 * 111 * 3],
        484,
        111,
        image::ColorType::Rgb8.into(),
      )
      .expect("source png");
    let image_data = ImportedImageData {
      data: png.into(),
      content_type: Some("image/png".into()),
    };

    let (cropped, residual_crop) = materialize_source_rectangle_crop(
      image_data,
      ImageCrop {
        left: 0.0,
        top: -0.00905,
        right: 0.00555,
        bottom: 0.04073,
      },
      true,
    );

    let cropped = image::load_from_memory(&cropped.data).expect("cropped png");
    assert_eq!((cropped.width(), cropped.height()), (481, 106));
    assert_eq!(
      residual_crop,
      ImageCrop {
        left: 0.0,
        top: -0.00905,
        right: 0.0,
        bottom: 0.0,
      }
    );
  }

  #[test]
  fn drawing_image_properties_preserve_external_link_placeholders() {
    let xml = r#"<pic:pic xmlns:pic="http://schemas.openxmlformats.org/drawingml/2006/picture" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"><pic:nvPicPr><pic:cNvPr id="1" name="Picture 1"/><pic:cNvPicPr/></pic:nvPicPr><pic:blipFill><a:blip r:link="rId5"/><a:stretch><a:fillRect/></a:stretch></pic:blipFill><pic:spPr/></pic:pic>"#;

    let picture = pic::Picture::from_bytes(xml.as_bytes()).expect("picture");
    let properties = drawing_picture_image_properties(&picture, &ThemeColors::default(), None)
      .expect("external image properties");

    assert_eq!(properties.relationship_id.as_deref(), Some("rId5"));
    assert!(properties.external_link);
    let placeholder =
      drawing_image_data(&ImageCatalog::default(), &properties).expect("linked placeholder");
    assert!(placeholder.data.is_empty());
    assert_eq!(placeholder.content_type, None);
  }

  #[test]
  fn drawingml_font_reference_color_requires_an_explicit_color_choice() {
    let without_color = a::FontReference::from_bytes(
      br#"<a:fontRef xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" idx="minor"/>"#,
    )
    .expect("font reference without color");
    assert_eq!(
      drawingml_font_reference_color(&without_color, &ThemeColors::default()),
      None
    );

    let with_color = a::FontReference::from_bytes(
      br#"<a:fontRef xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" idx="minor"><a:srgbClr val="FFFFFF"/></a:fontRef>"#,
    )
    .expect("font reference with color");
    assert_eq!(
      drawingml_font_reference_color(&with_color, &ThemeColors::default()),
      Some(RgbColor {
        r: 255,
        g: 255,
        b: 255,
      })
    );
  }

  #[test]
  fn automatic_shape_text_color_uses_the_higher_contrast_neutral() {
    assert_eq!(
      automatic_text_color_for_background(RgbColor {
        r: 0x0d,
        g: 0x2c,
        b: 0x40,
      }),
      RgbColor {
        r: 255,
        g: 255,
        b: 255,
      }
    );
    for background in [
      RgbColor {
        r: 0xef,
        g: 0x29,
        b: 0x29,
      },
      RgbColor {
        r: 0x72,
        g: 0x9f,
        b: 0xcf,
      },
    ] {
      assert_eq!(
        automatic_text_color_for_background(background),
        RgbColor { r: 0, g: 0, b: 0 }
      );
    }
  }

  #[test]
  fn direct_run_shading_keeps_automatic_or_explicit_text_black() {
    let automatic = w::RunProperties::from_bytes(
      br#"<w:rPr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:shd w:val="clear" w:fill="880088"/></w:rPr>"#,
    )
    .expect("automatically colored shaded run");
    let mut style = TextStyle::default();
    properties::merge_run_style(
      &mut style,
      Some(RunProps::Direct(&automatic)),
      &ThemeFonts::default(),
      &ThemeColors::default(),
    );
    assert_eq!(
      style.highlight,
      Some(RgbColor {
        r: 0x88,
        g: 0,
        b: 0x88,
      })
    );
    assert_eq!(style.color, RgbColor { r: 0, g: 0, b: 0 });
    assert!(style.color_is_automatic);

    let explicit = w::RunProperties::from_bytes(
      br#"<w:rPr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:color w:val="000000"/><w:shd w:val="clear" w:fill="880088"/></w:rPr>"#,
    )
    .expect("explicitly colored shaded run");
    properties::merge_run_style(
      &mut style,
      Some(RunProps::Direct(&explicit)),
      &ThemeFonts::default(),
      &ThemeColors::default(),
    );
    assert_eq!(style.color, RgbColor { r: 0, g: 0, b: 0 });
    assert!(!style.color_is_automatic);
  }

  #[test]
  fn document_defaults_ignore_run_position_but_keep_other_run_properties() {
    let defaults = w::RunPropertiesBaseStyle {
      position: Some(w::Position {
        val: ooxmlsdk::simple_type::SignedHpsMeasureValue::HalfPoints(18),
      }),
      shading: Some(w::Shading {
        fill: Some("880088".into()),
        ..Default::default()
      }),
      ..Default::default()
    };
    let mut style = TextStyle::default();

    properties::merge_doc_default_run_style(
      &mut style,
      Some(&defaults),
      &ThemeFonts::default(),
      &ThemeColors::default(),
    );

    assert_eq!(style.baseline_shift_pt, 0.0);
    assert_eq!(
      style.highlight,
      Some(RgbColor {
        r: 0x88,
        g: 0,
        b: 0x88,
      })
    );
    assert_eq!(
      style.color,
      RgbColor {
        r: 255,
        g: 255,
        b: 255,
      }
    );
  }

  #[test]
  fn wps_textbox_fragment_imports_as_positioned_shape_text() {
    // drawing shape, not as fallback body text.
    let xml = r#"<wps:wsp xmlns:wps="http://schemas.microsoft.com/office/word/2010/wordprocessingShape" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><wps:cNvSpPr txBox="1"/><wps:spPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="857250" cy="742950"/></a:xfrm><a:prstGeom prst="rect"><a:avLst/></a:prstGeom></wps:spPr><wps:txbx><w:txbxContent><w:p><w:r><w:t>inside shape</w:t></w:r></w:p></w:txbxContent></wps:txbx><wps:bodyPr lIns="91440" tIns="45720" rIns="91440" bIns="45720" anchor="t"/></wps:wsp>"#;
    let shape = wps::WordprocessingShape::from_bytes(xml.as_bytes()).expect("wordprocessing shape");
    assert!(wordprocessing_shape_textbox_content(&shape).is_some());
    let graphic_data = a::GraphicData::from_bytes(
      format!(
        r#"<a:graphicData xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" uri="http://schemas.microsoft.com/office/word/2010/wordprocessingShape">{xml}</a:graphicData>"#
      )
      .as_bytes(),
    )
    .expect("graphicData");
    assert!(matches!(
      graphic_data.graphic_data_choice.as_slice(),
      [a::GraphicDataChoice::WordprocessingShape(_)]
    ));

    let styles = StylesCatalog::default();
    let images = ImageCatalog::default();
    let hyperlinks = HyperlinkCatalog::default();
    let frame = wordprocessing_shape_textbox_frame(
      &shape,
      ImagePlacement::Inline,
      DrawingMlGroupTransform::identity(),
      DrawingTextBoxImportContext {
        base_style: TextStyle::default(),
        styles: &styles,
        images: &images,
        hyperlinks: &hyperlinks,
      },
    )
    .expect("wps textbox frame");

    assert!((frame.offset_x_pt - 0.0).abs() < 0.001);
    assert!((frame.offset_y_pt - 0.0).abs() < 0.001);
    assert!((frame.width_pt - 67.5).abs() < 0.001);
    assert!((frame.height_pt - 58.5).abs() < 0.001);
    assert!((frame.text_inset_left_pt - 5.53).abs() < 0.001);
    assert!((frame.text_inset_top_pt - 3.6).abs() < 0.001);
    assert_eq!(frame.text_box_blocks.len(), 1);
  }

  #[test]
  fn fixed_wps_textbox_inline_picture_uses_literal_insets_and_hidden_outline_edge() {
    let xml = r#"<wps:wsp xmlns:wps="http://schemas.microsoft.com/office/word/2010/wordprocessingShape" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:wp="http://schemas.openxmlformats.org/drawingml/2006/wordprocessingDrawing" xmlns:pic="http://schemas.openxmlformats.org/drawingml/2006/picture" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"><wps:cNvSpPr txBox="1"/><wps:spPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="1325880" cy="442595"/></a:xfrm><a:prstGeom prst="rect"><a:avLst/></a:prstGeom><a:ln w="9525"><a:noFill/></a:ln></wps:spPr><wps:txbx><w:txbxContent><w:p><w:r><w:drawing><wp:inline><wp:extent cx="1132377" cy="250750"/><wp:docPr id="1" name="Picture 1"/><wp:cNvGraphicFramePr/><a:graphic><a:graphicData uri="http://schemas.openxmlformats.org/drawingml/2006/picture"><pic:pic><pic:nvPicPr><pic:cNvPr id="1" name="Picture 1"/><pic:cNvPicPr/></pic:nvPicPr><pic:blipFill><a:blip r:embed="rId1"/><a:stretch><a:fillRect/></a:stretch></pic:blipFill><pic:spPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="1132377" cy="250750"/></a:xfrm><a:prstGeom prst="rect"><a:avLst/></a:prstGeom></pic:spPr></pic:pic></a:graphicData></a:graphic></wp:inline></w:drawing></w:r></w:p></w:txbxContent></wps:txbx><wps:bodyPr lIns="91440" tIns="45720" rIns="91440" bIns="45720"><a:noAutofit/></wps:bodyPr></wps:wsp>"#;
    let shape = wps::WordprocessingShape::from_bytes(xml.as_bytes()).expect("WPS picture frame");
    let styles = StylesCatalog::default();
    let mut images = ImageCatalog::default();
    images.by_relationship_id.insert(
      "rId1".into(),
      package::ImageResource {
        data: vec![1, 2, 3].into(),
        content_type: Some("image/png".into()),
      },
    );
    let hyperlinks = HyperlinkCatalog::default();

    let frame = wordprocessing_shape_textbox_frame(
      &shape,
      ImagePlacement::Inline,
      DrawingMlGroupTransform::identity(),
      DrawingTextBoxImportContext {
        base_style: TextStyle::default(),
        styles: &styles,
        images: &images,
        hyperlinks: &hyperlinks,
      },
    )
    .expect("fixed WPS picture frame");

    assert!(text_box_is_single_inline_picture(&frame.text_box_blocks));
    assert!((frame.text_inset_left_pt - 7.575).abs() < 0.001);
    assert!((frame.text_inset_top_pt - 3.975).abs() < 0.001);
    assert!((frame.text_inset_right_pt - 7.575).abs() < 0.001);
    assert!((frame.text_inset_bottom_pt - 3.975).abs() < 0.001);
  }

  #[test]
  fn wps_explicit_no_fill_and_no_line_remains_a_textbox_only_shape() {
    let xml = r#"<wps:wsp xmlns:wps="http://schemas.microsoft.com/office/word/2010/wordprocessingShape" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><wps:cNvSpPr txBox="1"/><wps:spPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="5752465" cy="204470"/></a:xfrm><a:prstGeom prst="rect"><a:avLst/></a:prstGeom><a:noFill/><a:ln w="6350"><a:noFill/></a:ln></wps:spPr><wps:txbx><w:txbxContent><w:p><w:r><w:t>Der …</w:t></w:r></w:p></w:txbxContent></wps:txbx><wps:bodyPr lIns="0" tIns="0" rIns="0" bIns="0" anchor="t"><a:noAutofit/></wps:bodyPr></wps:wsp>"#;
    let source = wps::WordprocessingShape::from_bytes(xml.as_bytes()).expect("WPS shape");
    let styles = StylesCatalog::default();
    let images = ImageCatalog::default();
    let hyperlinks = HyperlinkCatalog::default();

    assert!(
      wordprocessing_shape_shape(
        &source,
        ImagePlacement::Inline,
        DrawingMlGroupTransform::identity(),
        DrawingShapeImportContext {
          effect_extent: DrawingEffectExtent::default(),
          styles: &styles,
          images: &images,
          hyperlinks: &hyperlinks,
          smartart_text_colors_by_model_id: None,
        },
      )
      .is_none(),
      "explicit noFill must not create a visible owning shape"
    );

    let text_box = wordprocessing_shape_textbox_frame(
      &source,
      ImagePlacement::Inline,
      DrawingMlGroupTransform::identity(),
      DrawingTextBoxImportContext {
        base_style: TextStyle::default(),
        styles: &styles,
        images: &images,
        hyperlinks: &hyperlinks,
      },
    )
    .expect("textbox frame");
    assert!(text_box.inline_anchor_after_line);
  }

  #[test]
  fn wps_inline_shape_and_textbox_share_one_layout_object() {
    let xml = r#"<wps:wsp xmlns:wps="http://schemas.microsoft.com/office/word/2010/wordprocessingShape" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><wps:cNvSpPr/><wps:spPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="857250" cy="742950"/></a:xfrm><a:prstGeom prst="rect"><a:avLst/></a:prstGeom><a:solidFill><a:srgbClr val="0D2C40"/></a:solidFill></wps:spPr><wps:txbx><w:txbxContent><w:p><w:r><w:t>inside shape</w:t></w:r></w:p></w:txbxContent></wps:txbx><wps:bodyPr/></wps:wsp>"#;
    let source = wps::WordprocessingShape::from_bytes(xml.as_bytes()).expect("WPS shape");
    let styles = StylesCatalog::default();
    let images = ImageCatalog::default();
    let hyperlinks = HyperlinkCatalog::default();
    let shape = wordprocessing_shape_shape(
      &source,
      ImagePlacement::Inline,
      DrawingMlGroupTransform::identity(),
      DrawingShapeImportContext {
        effect_extent: DrawingEffectExtent::default(),
        styles: &styles,
        images: &images,
        hyperlinks: &hyperlinks,
        smartart_text_colors_by_model_id: None,
      },
    )
    .expect("visual shape");
    let text_box = wordprocessing_shape_textbox_frame(
      &source,
      ImagePlacement::Inline,
      DrawingMlGroupTransform::identity(),
      DrawingTextBoxImportContext {
        base_style: TextStyle::default(),
        styles: &styles,
        images: &images,
        hyperlinks: &hyperlinks,
      },
    )
    .expect("textbox frame");
    let mut inlines = vec![InlineItem::Shape(shape)];

    merge_textbox_frame_into_owning_shape(&mut inlines, text_box).expect("merged textbox");

    assert_eq!(inlines.len(), 1);
    let InlineItem::Shape(shape) = &inlines[0] else {
      panic!("merged item is not a shape");
    };
    assert_eq!(shape.text_box_blocks.len(), 1);
  }

  #[test]
  fn floating_wps_horizontal_autofit_uses_one_resizable_shape() {
    let drawing = w::Drawing::from_bytes(
      br#"<w:drawing xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:wp="http://schemas.openxmlformats.org/drawingml/2006/wordprocessingDrawing" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:wps="http://schemas.microsoft.com/office/word/2010/wordprocessingShape"><wp:anchor distT="0" distB="0" distL="114300" distR="114300" simplePos="0" relativeHeight="2" behindDoc="0" locked="0" layoutInCell="1" allowOverlap="1"><wp:simplePos x="0" y="0"/><wp:positionH relativeFrom="column"><wp:align>center</wp:align></wp:positionH><wp:positionV relativeFrom="paragraph"><wp:posOffset>0</wp:posOffset></wp:positionV><wp:extent cx="2374265" cy="1403985"/><wp:effectExtent l="0" t="0" r="11430" b="15875"/><wp:wrapNone/><wp:docPr id="1" name="Text Box 1"/><wp:cNvGraphicFramePr/><a:graphic><a:graphicData uri="http://schemas.microsoft.com/office/word/2010/wordprocessingShape"><wps:wsp><wps:cNvSpPr txBox="1"/><wps:spPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="2374265" cy="1403985"/></a:xfrm><a:prstGeom prst="rect"><a:avLst/></a:prstGeom><a:solidFill><a:srgbClr val="FFFFFF"/></a:solidFill><a:ln w="9525"><a:solidFill><a:srgbClr val="000000"/></a:solidFill></a:ln></wps:spPr><wps:txbx><w:txbxContent><w:p><w:r><w:t>inside shape</w:t></w:r></w:p></w:txbxContent></wps:txbx><wps:bodyPr wrap="none" lIns="91440" tIns="45720" rIns="91440" bIns="45720"><a:spAutoFit/></wps:bodyPr></wps:wsp></a:graphicData></a:graphic></wp:anchor></w:drawing>"#,
    )
    .expect("floating WPS autofit shape");
    let styles = StylesCatalog::default();
    let images = ImageCatalog::default();
    let hyperlinks = HyperlinkCatalog::default();
    let mut inlines = Vec::new();

    push_drawing_shapes_impl(&drawing, &mut inlines, &styles, &images, &hyperlinks);
    push_drawing_textboxes_impl(
      &drawing,
      &mut inlines,
      TextStyle::default(),
      &styles,
      &images,
      &hyperlinks,
    );

    assert_eq!(inlines.len(), 1);
    let InlineItem::Shape(shape) = &inlines[0] else {
      panic!("merged item is not a shape");
    };
    assert_eq!(shape.text_box_blocks.len(), 1);
    assert!(shape.text_box_auto_fit);
    assert!(shape.text_box_resizes_height_to_fit);
    assert!(!shape.text_box_word_wrap);
    assert!((shape.text_inset_left_pt - 7.2).abs() < 0.001);
    assert!((shape.stroke.expect("authored outline").width_pt - 0.75).abs() < 0.001);
    let ImagePlacement::Floating(placement) = shape.placement else {
      panic!("autofit shape is not floating");
    };
    assert_eq!(placement.wrap, ImageWrapMode::Through);
  }

  #[test]
  fn wps_custom_geometry_line_imports_as_line_shape() {
    let xml = r#"<wps:wsp xmlns:wps="http://schemas.microsoft.com/office/word/2010/wordprocessingShape" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"><wps:spPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="5760720" cy="0"/></a:xfrm><a:custGeom><a:pathLst><a:path w="8504" h="0"><a:moveTo><a:pt x="0" y="0"/></a:moveTo><a:lnTo><a:pt x="8504" y="0"/></a:lnTo></a:path></a:pathLst></a:custGeom><a:noFill/><a:ln w="6480"><a:solidFill><a:srgbClr val="ff0101"/></a:solidFill></a:ln></wps:spPr></wps:wsp>"#;
    let wordprocessing_shape =
      wps::WordprocessingShape::from_bytes(xml.as_bytes()).expect("typed WPS shape");
    let styles = StylesCatalog::default();
    let images = ImageCatalog::default();
    let hyperlinks = HyperlinkCatalog::default();
    let shape = wordprocessing_shape_shape(
      &wordprocessing_shape,
      ImagePlacement::Inline,
      DrawingMlGroupTransform::identity(),
      DrawingShapeImportContext {
        effect_extent: DrawingEffectExtent::default(),
        styles: &styles,
        images: &images,
        hyperlinks: &hyperlinks,
        smartart_text_colors_by_model_id: None,
      },
    )
    .expect("custom geometry shape");

    assert_eq!(shape.geometry, InlineShapeGeometry::Line);
    assert!(shape.fill_color.is_none());
    assert_eq!(shape.stroke.expect("stroke").color.r, 0xff);
  }

  #[test]
  fn wps_zero_width_straight_connector_retains_its_stroked_path() {
    let xml = r#"<wps:wsp xmlns:wps="http://schemas.microsoft.com/office/word/2010/wordprocessingShape" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"><wps:cNvCnPr/><wps:spPr><a:xfrm flipV="1"><a:off x="0" y="0"/><a:ext cx="0" cy="1356910"/></a:xfrm><a:prstGeom prst="straightConnector1"><a:avLst/></a:prstGeom><a:ln><a:tailEnd type="arrow"/></a:ln></wps:spPr><wps:style><a:lnRef idx="2"><a:schemeClr val="accent1"/></a:lnRef><a:fillRef idx="0"><a:schemeClr val="accent1"/></a:fillRef><a:effectRef idx="1"><a:schemeClr val="accent1"/></a:effectRef><a:fontRef idx="minor"><a:schemeClr val="tx1"/></a:fontRef></wps:style></wps:wsp>"#;
    let wordprocessing_shape =
      wps::WordprocessingShape::from_bytes(xml.as_bytes()).expect("typed WPS connector");
    let effect_styles = a::EffectStyleList::from_bytes(
      br#"<a:effectStyleLst xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"><a:effectStyle><a:effectLst><a:outerShdw blurRad="40000" dist="20000" dir="5400000" rotWithShape="0"><a:srgbClr val="000000"><a:alpha val="38000"/></a:srgbClr></a:outerShdw></a:effectLst></a:effectStyle></a:effectStyleLst>"#,
    )
    .expect("typed theme effect styles");
    let styles = StylesCatalog {
      theme_colors: ThemeColors {
        accent1: Some(RgbColor {
          r: 79,
          g: 129,
          b: 189,
        }),
        ..Default::default()
      },
      theme_lines: ThemeLineStyles {
        widths_pt: vec![0.75, 2.0],
      },
      theme_effects: ThemeEffectStyles {
        styles: effect_styles.effect_style,
      },
      ..Default::default()
    };
    let shape = wordprocessing_shape_shape(
      &wordprocessing_shape,
      ImagePlacement::Inline,
      DrawingMlGroupTransform::identity(),
      DrawingShapeImportContext {
        effect_extent: DrawingEffectExtent::default(),
        styles: &styles,
        images: &ImageCatalog::default(),
        hyperlinks: &HyperlinkCatalog::default(),
        smartart_text_colors_by_model_id: None,
      },
    )
    .expect("zero-width straight connector");

    assert_eq!(shape.width_pt, 0.0);
    assert!((shape.height_pt - 106.843).abs() < 0.001);
    assert_eq!(
      shape.stroke,
      Some(BorderStyle {
        width_pt: 2.0,
        color: RgbColor {
          r: 79,
          g: 129,
          b: 189,
        },
        ..Default::default()
      })
    );
    let InlineShapeGeometry::Path { paths, outline } = shape.geometry else {
      panic!("degenerate straight connector must remain a styled path");
    };
    assert!(outline.is_some(), "the authored arrow end must be retained");
    assert!(
      shape.effects.is_some(),
      "the indexed theme shadow must be retained"
    );
    assert_eq!(paths.len(), 1);
    assert_eq!(paths[0].fill_mode, common::DrawingPathFillMode::None);
    assert!(paths[0].stroke);
    assert!(matches!(
      paths[0].commands.as_slice(),
      [
        common::PathCommand::MoveTo(common::Point {
          x: common::Pt(0.0),
          y: common::Pt(0.0)
        }),
        common::PathCommand::LineTo(common::Point {
          x: common::Pt(0.0),
          y: common::Pt(end_y)
        })
      ] if (*end_y - 106.843).abs() < 0.001
    ));
  }

  #[test]
  fn wps_shape_inherits_theme_effect_reference_and_direct_effect_list_clears_it() {
    let effect_styles = a::EffectStyleList::from_bytes(
      br#"<a:effectStyleLst xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"><a:effectStyle><a:effectLst><a:outerShdw blurRad="40000" dist="20000" dir="5400000" rotWithShape="0"><a:schemeClr val="phClr"><a:alpha val="50000"/></a:schemeClr></a:outerShdw></a:effectLst></a:effectStyle></a:effectStyleLst>"#,
    )
    .expect("typed theme effect styles");
    let styles = StylesCatalog {
      theme_colors: ThemeColors {
        accent1: Some(RgbColor {
          r: 79,
          g: 129,
          b: 189,
        }),
        ..Default::default()
      },
      theme_effects: ThemeEffectStyles {
        styles: effect_styles.effect_style,
      },
      ..Default::default()
    };
    let import = |direct_effects: &str| {
      let xml = format!(
        r#"<wps:wsp xmlns:wps="http://schemas.microsoft.com/office/word/2010/wordprocessingShape" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"><wps:spPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="914400" cy="914400"/></a:xfrm><a:prstGeom prst="star5"><a:avLst/></a:prstGeom><a:solidFill><a:srgbClr val="336699"/></a:solidFill>{direct_effects}</wps:spPr><wps:style><a:lnRef idx="0"><a:schemeClr val="accent1"/></a:lnRef><a:fillRef idx="0"><a:schemeClr val="accent1"/></a:fillRef><a:effectRef idx="1"><a:schemeClr val="accent1"/></a:effectRef><a:fontRef idx="minor"><a:schemeClr val="tx1"/></a:fontRef></wps:style></wps:wsp>"#
      );
      let wordprocessing_shape =
        wps::WordprocessingShape::from_bytes(xml.as_bytes()).expect("typed WPS star");
      wordprocessing_shape_shape(
        &wordprocessing_shape,
        ImagePlacement::Inline,
        DrawingMlGroupTransform::identity(),
        DrawingShapeImportContext {
          effect_extent: DrawingEffectExtent::default(),
          styles: &styles,
          images: &ImageCatalog::default(),
          hyperlinks: &HyperlinkCatalog::default(),
          smartart_text_colors_by_model_id: None,
        },
      )
      .expect("WPS star")
    };

    let inherited = import("");
    let inherited_effects = match inherited.effects.as_ref() {
      Some(common::DrawingEffectSource::List {
        resolved: Some(value),
        ..
      }) => value,
      _ => panic!("theme effect list must resolve for an ordinary WPS shape"),
    };
    let shadow_color = inherited_effects
      .effects
      .iter()
      .find_map(|effect| match effect {
        common::drawingml_image_effects::ImageEffect::OuterShadow { color, .. } => Some(*color),
        _ => None,
      })
      .expect("theme outer shadow");
    assert_eq!(
      shadow_color,
      ResolvedEffectColor {
        color: RgbColor {
          r: 79,
          g: 129,
          b: 189,
        },
        alpha: 127,
      },
      "theme phClr must use the effectRef color"
    );

    let cleared = import("<a:effectLst/>");
    let cleared_effects = match cleared.effects.as_ref() {
      Some(common::DrawingEffectSource::List {
        resolved: Some(value),
        ..
      }) => value,
      _ => panic!("direct empty effect list must remain authoritative"),
    };
    assert!(
      cleared_effects.effects.is_empty(),
      "direct shape effects override the referenced theme effect"
    );
  }

  #[test]
  fn wps_custom_geometry_preserves_quadratic_curve_as_path() {
    let xml = r#"<wps:wsp xmlns:wps="http://schemas.microsoft.com/office/word/2010/wordprocessingShape" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"><wps:spPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="914400" cy="914400"/></a:xfrm><a:custGeom><a:pathLst><a:path w="100" h="100"><a:moveTo><a:pt x="0" y="100"/></a:moveTo><a:quadBezTo><a:pt x="50" y="0"/><a:pt x="100" y="100"/></a:quadBezTo><a:close/></a:path></a:pathLst></a:custGeom><a:solidFill><a:srgbClr val="336699"/></a:solidFill></wps:spPr></wps:wsp>"#;
    let wordprocessing_shape =
      wps::WordprocessingShape::from_bytes(xml.as_bytes()).expect("typed WPS shape");
    let shape = wordprocessing_shape_shape(
      &wordprocessing_shape,
      ImagePlacement::Inline,
      DrawingMlGroupTransform::identity(),
      DrawingShapeImportContext {
        effect_extent: DrawingEffectExtent::default(),
        styles: &StylesCatalog::default(),
        images: &ImageCatalog::default(),
        hyperlinks: &HyperlinkCatalog::default(),
        smartart_text_colors_by_model_id: None,
      },
    )
    .expect("custom geometry shape");

    let InlineShapeGeometry::Path { paths, .. } = shape.geometry else {
      panic!("quadratic custom geometry must remain a path");
    };
    let commands = &paths[0].commands;
    assert!(
      commands
        .iter()
        .any(|command| matches!(command, common::PathCommand::CubicTo { .. }))
    );
    assert!(matches!(commands.last(), Some(common::PathCommand::Close)));
  }

  #[test]
  fn symbol_runs_preserve_declared_symbol_font_transport_codes() {
    let mut inlines = Vec::new();
    let run = w::Run {
      run_choice: vec![
        w::RunChoice::SymbolChar(w::SymbolChar {
          font: Some("Symbol".into()),
          char: Some("F0B7".into()),
        }),
        w::RunChoice::SymbolChar(w::SymbolChar {
          font: Some("Wingdings".into()),
          char: Some("F0FC".into()),
        }),
        w::RunChoice::SymbolChar(w::SymbolChar {
          font: Some("Wingdings".into()),
          char: Some("F04C".into()),
        }),
        w::RunChoice::SymbolChar(w::SymbolChar {
          font: None,
          char: Some("00A9".into()),
        }),
      ],
      ..Default::default()
    };

    push_run(
      &run,
      &mut inlines,
      TextStyle::default(),
      &StylesCatalog::default(),
      &ImageCatalog::default(),
      &HyperlinkCatalog::default(),
      None,
    );

    assert_eq!(inline_text(&inlines), "\u{f0b7}\u{f0fc}●©");
    let symbol_fonts = inlines
      .iter()
      .filter_map(|inline| match inline {
        InlineItem::Text(run) => run.style.font_family.as_deref(),
        _ => None,
      })
      .collect::<Vec<_>>();
    assert_eq!(symbol_fonts, ["Symbol", "Wingdings"]);
  }

  #[test]
  fn webdings_run_text_uses_symbol_font_transport_codes() {
    let style = TextStyle {
      font_family: Some(Arc::from("Webdings")),
      ..TextStyle::default()
    };

    assert_eq!(
      run_display_text("add text".into(), style),
      "\u{f061}\u{f064}\u{f064}\u{f020}\u{f074}\u{f065}\u{f078}\u{f074}"
    );
  }

  #[test]
  fn nested_word_runs_preserve_shape_text() {
    let run = w::Run::from_bytes(
      br#"<w:r xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:r><w:t>Test text box</w:t></w:r></w:r>"#,
    )
    .expect("nested run");
    let mut inlines = Vec::new();

    push_run(
      &run,
      &mut inlines,
      TextStyle::default(),
      &StylesCatalog::default(),
      &ImageCatalog::default(),
      &HyperlinkCatalog::default(),
      None,
    );

    assert_eq!(inline_text(&inlines), "Test text box");
  }

  #[test]
  fn paragraph_relative_word_shape_preserves_position_offset() {
    let anchor = wp::Anchor::from_bytes(
      br#"<wp:anchor xmlns:wp="http://schemas.openxmlformats.org/drawingml/2006/wordprocessingDrawing" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" behindDoc="0" distT="0" distB="0" distL="0" distR="0" simplePos="0" locked="0" layoutInCell="1" allowOverlap="1" relativeHeight="2"><wp:simplePos x="0" y="0"/><wp:positionH relativeFrom="column"><wp:posOffset>408305</wp:posOffset></wp:positionH><wp:positionV relativeFrom="paragraph"><wp:posOffset>204470</wp:posOffset></wp:positionV><wp:extent cx="4972050" cy="1152525"/><wp:wrapNone/><wp:docPr id="1" name="Text Frame 1"/><a:graphic><a:graphicData uri="urn:unused"/></a:graphic></wp:anchor>"#,
    )
    .expect("floating anchor");

    let placement = floating_image_placement(&anchor);

    assert_eq!(
      placement.horizontal_relative_to,
      HorizontalImageReference::Column
    );
    assert_eq!(
      placement.vertical_relative_to,
      VerticalImageReference::Paragraph
    );
    assert!((placement.horizontal_offset_pt - 32.15).abs() < 0.001);
    assert!((placement.vertical_offset_pt - 16.1).abs() < 0.001);
  }

  #[test]
  fn paragraph_relative_wps_textbox_keeps_anchor_and_shape_offsets_separate() {
    let drawing_xml = br#"<w:drawing xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:wp="http://schemas.openxmlformats.org/drawingml/2006/wordprocessingDrawing" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:wps="http://schemas.microsoft.com/office/word/2010/wordprocessingShape"><wp:anchor behindDoc="0" distT="0" distB="0" distL="0" distR="0" simplePos="0" locked="0" layoutInCell="1" allowOverlap="1" relativeHeight="2"><wp:simplePos x="0" y="0"/><wp:positionH relativeFrom="column"><wp:posOffset>408305</wp:posOffset></wp:positionH><wp:positionV relativeFrom="paragraph"><wp:posOffset>204470</wp:posOffset></wp:positionV><wp:extent cx="4972050" cy="1152525"/><wp:wrapNone/><wp:docPr id="1" name="Text Frame 1"/><a:graphic><a:graphicData uri="http://schemas.microsoft.com/office/word/2010/wordprocessingShape"><wps:wsp><wps:cNvSpPr txBox="1"/><wps:spPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="4971960" cy="1152360"/></a:xfrm><a:prstGeom prst="rect"><a:avLst/></a:prstGeom><a:noFill/><a:ln w="0"><a:noFill/></a:ln></wps:spPr><wps:txbx><w:txbxContent><w:p><w:r><w:t>Test text box</w:t></w:r></w:p></w:txbxContent></wps:txbx><wps:bodyPr wrap="square" lIns="0" rIns="0" tIns="0" bIns="0" anchor="t"><a:noAutofit/></wps:bodyPr></wps:wsp></a:graphicData></a:graphic></wp:anchor></w:drawing>"#;
    let drawing = w::Drawing::from_bytes(drawing_xml).expect("floating WPS textbox");
    let mut inlines = Vec::new();

    push_drawing_textboxes_impl(
      &drawing,
      &mut inlines,
      TextStyle::default(),
      &StylesCatalog::default(),
      &ImageCatalog::default(),
      &HyperlinkCatalog::default(),
    );

    let InlineItem::Shape(shape) = &inlines[0] else {
      panic!("expected WPS textbox shape");
    };
    let ImagePlacement::Floating(placement) = shape.placement else {
      panic!("expected floating WPS textbox");
    };
    assert!((placement.vertical_offset_pt - 16.1).abs() < 0.001);
    assert!((shape.offset_y_pt - 0.0).abs() < 0.001);
    assert!((shape.text_inset_top_pt - 0.0).abs() < 0.001);
  }

  #[test]
  fn table_cell_margins_use_ecma_default_side_padding() {
    let margins = CellMargins::default();

    assert_eq!(margins.top_pt, 0.0);
    assert_eq!(margins.bottom_pt, 0.0);
    assert!((margins.left_pt - 5.75).abs() < 0.001);
    assert!((margins.right_pt - 5.75).abs() < 0.001);
  }

  #[test]
  fn unstyled_tables_inherit_the_default_table_style() {
    let authored_margins = CellMargins {
      left_pt: 5.4,
      right_pt: 5.4,
      ..CellMargins::default()
    };
    let mut styles = StylesCatalog {
      default_table_style_id: Some("TableNormal".to_string()),
      ..StylesCatalog::default()
    };
    styles.styles.insert(
      "TableNormal".to_string(),
      StyleEntry {
        style_type: Some(w::StyleValues::Table),
        table_style: TableStyleModel {
          cell_margins: Some(authored_margins),
          ..TableStyleModel::default()
        },
        ..StyleEntry::default()
      },
    );

    assert_eq!(
      styles.table_style(None).cell_margins,
      Some(authored_margins)
    );
  }

  #[test]
  fn legacy_word_tables_align_first_cell_content_to_table_indent() {
    let top_level = TableModelContext {
      nested_table_level: 1,
      in_header_footer: false,
    };
    let nested = TableModelContext {
      nested_table_level: 2,
      in_header_footer: false,
    };

    assert!(should_align_leading_cell_content(top_level, 12, true));
    assert!(!should_align_leading_cell_content(top_level, 15, true));
    assert!(should_align_leading_cell_content(top_level, 15, false));
    assert!(!should_align_leading_cell_content(nested, 12, true));
  }

  #[test]
  fn table_indentation_preserves_negative_leading_offsets() {
    let indentation = w::TableIndentation {
      width: Some(measurement(-118)),
      r#type: Some(w::TableWidthUnitValues::Dxa),
    };

    assert_eq!(table_indentation_to_points(&indentation), Some(-5.9));

    let legacy_wrapped = w::TableIndentation {
      width: Some(measurement(65_000)),
      r#type: Some(w::TableWidthUnitValues::Dxa),
    };
    assert_eq!(table_indentation_to_points(&legacy_wrapped), Some(-26.8));
  }

  #[test]
  fn table_layout_defaults_to_autofit_and_preserves_explicit_fixed() {
    assert_eq!(
      table_layout_mode(&w::TableLayout::default()),
      TableLayoutMode::AutoFit
    );
    assert_eq!(
      table_layout_mode(&w::TableLayout {
        r#type: Some(w::TableLayoutValues::Fixed),
      }),
      TableLayoutMode::Fixed
    );
  }

  #[test]
  fn floating_table_overlap_defaults_to_allowed_and_preserves_never() {
    assert!(table_allows_overlap(None));
    assert!(!table_allows_overlap(Some(&w::TableProperties {
      table_overlap: Some(w::TableOverlap {
        val: w::TableOverlapValues::Never,
      }),
      ..Default::default()
    })));
    assert!(table_allows_overlap(Some(&w::TableProperties {
      table_overlap: Some(w::TableOverlap {
        val: w::TableOverlapValues::Overlap,
      }),
      ..Default::default()
    })));
  }

  #[test]
  fn floating_table_position_uses_word_anchor_defaults() {
    let placement = table_position_placement(&w::TablePositionProperties {
      left_from_text: Some(twips(181)),
      table_position_y: Some(signed_twips(1)),
      ..Default::default()
    });

    assert_eq!(placement.horizontal_anchor, FrameHorizontalAnchor::Text);
    assert_eq!(placement.vertical_anchor, FrameVerticalAnchor::Margin);
    assert_eq!(placement.horizontal_offset_pt, 0.0);
    assert!((placement.vertical_offset_pt - 0.05).abs() < 0.001);

    let explicit = table_position_placement(&w::TablePositionProperties {
      left_from_text: Some(twips(181)),
      horizontal_anchor: Some(w::HorizontalAnchorValues::Page),
      table_position_x: Some(signed_twips(5597)),
      ..Default::default()
    });
    assert!((explicit.horizontal_offset_pt - 279.85).abs() < 0.001);
  }

  #[test]
  fn table_cell_no_wrap_cascades_from_style_and_direct_false_overrides_it() {
    let style = style_table_cell_style(&w::StyleTableCellProperties {
      no_wrap: Some(w::NoWrap { val: None }),
      ..Default::default()
    });
    assert_eq!(style.no_wrap, Some(true));

    let direct = w::TableCellProperties {
      no_wrap: Some(w::NoWrap {
        val: Some(false.into()),
      }),
      ..Default::default()
    };
    assert_eq!(
      direct
        .no_wrap
        .as_ref()
        .map(|no_wrap| on_off_only_value(no_wrap.val))
        .or(style.no_wrap),
      Some(false)
    );
  }

  #[test]
  fn table_cell_shading_overrides_row_exception_and_table_shading() {
    let table = RgbColor { r: 1, g: 2, b: 3 };
    let row = RgbColor { r: 4, g: 5, b: 6 };
    let styled_cell = RgbColor { r: 7, g: 8, b: 9 };
    let direct_cell = RgbColor {
      r: 10,
      g: 11,
      b: 12,
    };

    assert_eq!(
      resolved_table_cell_shading(None, None, Some(table)),
      Some(table)
    );
    assert_eq!(
      resolved_table_cell_shading(None, None, Some(row)),
      Some(row)
    );
    assert_eq!(
      resolved_table_cell_shading(None, Some(Some(styled_cell)), Some(row)),
      Some(styled_cell)
    );
    assert_eq!(
      resolved_table_cell_shading(Some(Some(direct_cell)), Some(Some(styled_cell)), Some(row)),
      Some(direct_cell)
    );
    assert_eq!(
      resolved_table_cell_shading(Some(None), Some(Some(styled_cell)), Some(row)),
      None
    );
  }

  #[test]
  fn table_cell_margin_overrides_inherit_unspecified_defaults() {
    let margins = table_cell_margin(
      &w::TableCellMargin {
        left_margin: Some(w::TableCellLeftMargin {
          width: Some(measurement(240)),
          r#type: Some(w::TableWidthUnitValues::Dxa),
        }),
        ..Default::default()
      },
      CellMargins::default(),
    );

    assert_eq!(margins.left_pt, 12.0);
    assert!((margins.right_pt - 5.75).abs() < 0.001);
    assert_eq!(margins.top_pt, 0.0);
    assert_eq!(margins.bottom_pt, 0.0);
  }

  #[test]
  fn table_cell_spacing_uses_dxa_widths() {
    let spacing = w::TableCellSpacing {
      width: Some(measurement(240)),
      r#type: Some(w::TableWidthUnitValues::Dxa),
    };

    assert_eq!(table_cell_spacing_to_points(&spacing), Some(12.0));
  }

  #[test]
  fn table_row_grid_properties_preserve_skipped_grid_columns() {
    let properties = w::TableRowProperties {
      table_row_properties_choice1: vec![
        w::TableRowPropertiesChoice::GridBefore(w::GridBefore { val: 1 }),
        w::TableRowPropertiesChoice::GridAfter(w::GridAfter { val: 2 }),
      ],
      ..Default::default()
    };

    assert_eq!(table_row_grid_properties(Some(&properties)), (1, 2));
  }

  #[test]
  fn table_row_look_uses_the_tbl_pr_ex_exception() {
    let inherited = TableLookModel::default();
    let row = w::TableRow {
      table_property_exceptions: Some(Box::new(w::TablePropertyExceptions {
        table_look: Some(w::TableLook {
          first_row: Some(ooxmlsdk::simple_type::OnOffValue::Off),
          first_column: Some(ooxmlsdk::simple_type::OnOffValue::Off),
          no_horizontal_band: Some(ooxmlsdk::simple_type::OnOffValue::On),
          no_vertical_band: Some(ooxmlsdk::simple_type::OnOffValue::On),
          ..Default::default()
        }),
        ..Default::default()
      })),
      ..Default::default()
    };

    let resolved = table_row_look(&row, inherited);

    assert!(!resolved.first_row);
    assert!(!resolved.first_column);
    assert!(!resolved.horizontal_banding);
    assert!(!resolved.vertical_banding);
  }

  #[test]
  fn bidi_visual_normalizes_the_complete_table_to_physical_order() {
    let table = w::Table::from_bytes(
      br#"<w:tbl xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
        <w:tblPr>
          <w:bidiVisual/>
          <w:jc w:val="left"/>
          <w:tblInd w:w="240" w:type="dxa"/>
          <w:tblBorders>
            <w:left w:val="single" w:sz="4" w:color="000000"/>
            <w:right w:val="single" w:sz="8" w:color="000000"/>
          </w:tblBorders>
        </w:tblPr>
        <w:tblGrid><w:gridCol w:w="200"/><w:gridCol w:w="400"/></w:tblGrid>
        <w:tr>
          <w:trPr><w:gridBefore w:val="1"/><w:gridAfter w:val="2"/></w:trPr>
          <w:tc>
            <w:tcPr>
              <w:tcMar>
                <w:left w:w="100" w:type="dxa"/>
                <w:right w:w="200" w:type="dxa"/>
              </w:tcMar>
            </w:tcPr>
            <w:p><w:r><w:t>A</w:t></w:r></w:p>
          </w:tc>
          <w:tc><w:p><w:r><w:t>B</w:t></w:r></w:p></w:tc>
        </w:tr>
      </w:tbl>"#,
    )
    .expect("right-to-left table");
    let styles = StylesCatalog::default();
    let mut numbering = NumberingCatalog::default();
    let images = ImageCatalog::default();
    let hyperlinks = HyperlinkCatalog::default();
    let bindings = CustomXmlBindings::default();
    let mut form_widget_ids = FormWidgetIdAllocator::default();

    let model = table_model(
      &table,
      &mut TableModelEnv {
        styles: &styles,
        numbering: &mut numbering,
        images: &images,
        hyperlinks: &hyperlinks,
        custom_xml_bindings: &bindings,
        form_widget_ids: &mut form_widget_ids,
      },
      TableModelContext {
        nested_table_level: 1,
        in_header_footer: false,
      },
    );

    let cell_text = |cell: &TableCell| {
      cell
        .blocks
        .iter()
        .filter_map(|block| match block {
          Block::Paragraph(paragraph) => Some(inline_text(&paragraph.inlines)),
          _ => None,
        })
        .collect::<String>()
    };
    assert!(model.right_to_left);
    assert_eq!(model.alignment, TableAlignment::Right);
    assert_eq!(model.indent_left_pt, 12.0);
    assert_eq!(model.column_widths_pt, [20.0, 10.0]);
    assert_eq!(model.rows[0].grid_before, 2);
    assert_eq!(model.rows[0].grid_after, 1);
    assert_eq!(cell_text(&model.rows[0].cells[0]), "B");
    assert_eq!(cell_text(&model.rows[0].cells[1]), "A");
    assert_eq!(model.rows[0].cells[1].margins.left_pt, 10.0);
    assert_eq!(model.rows[0].cells[1].margins.right_pt, 5.0);
    let borders = model.borders.expect("table borders");
    assert_eq!(borders.left.expect("physical left border").width_pt, 1.0);
    assert_eq!(borders.right.expect("physical right border").width_pt, 0.5);
  }

  #[test]
  fn table_style_first_row_overrides_whole_table_cell_style() {
    fn shading(fill: &str) -> w::Shading {
      w::Shading {
        fill: Some(fill.into()),
        ..Default::default()
      }
    }

    let style = table_style_model(
      &w::Style {
        r#type: Some(w::StyleValues::Table),
        style_table_cell_properties: Some(Box::new(w::StyleTableCellProperties {
          shading: Some(shading("EEEEEE")),
          ..Default::default()
        })),
        table_style_properties: vec![w::TableStyleProperties {
          r#type: w::TableStyleOverrideValues::FirstRow,
          style_paragraph_properties: Some(Box::new(w::StyleParagraphProperties {
            justification: Some(w::Justification {
              val: w::JustificationValues::Center,
            }),
            ..Default::default()
          })),
          run_properties_base_style: Some(Box::new(w::RunPropertiesBaseStyle {
            bold: Some(w::Bold { val: None }),
            color: Some(w::Color {
              val: Some("FFFFFF".into()),
              ..Default::default()
            }),
            ..Default::default()
          })),
          table_style_conditional_formatting_table_cell_properties: Some(Box::new(
            w::TableStyleConditionalFormattingTableCellProperties {
              shading: Some(shading("4472C4")),
              ..Default::default()
            },
          )),
          ..Default::default()
        }],
        ..Default::default()
      },
      &ThemeFonts::default(),
      &ThemeColors::default(),
      ImportSettings::default(),
    );

    let first_row = table_cell_style_for(
      &style,
      TableCellStyleContext {
        look: TableLookModel::default(),
        row_index: 0,
        row_count: 2,
        cell_index: 0,
        cell_count: 1,
        row_condition: TableConditionalStyleMask::from_row_position(
          TableLookModel::default(),
          0,
          2,
        ),
        cell_condition: None,
      },
    );
    let body_row = table_cell_style_for(
      &style,
      TableCellStyleContext {
        look: TableLookModel::default(),
        row_index: 1,
        row_count: 2,
        cell_index: 0,
        cell_count: 1,
        row_condition: TableConditionalStyleMask::from_row_position(
          TableLookModel::default(),
          1,
          2,
        ),
        cell_condition: None,
      },
    );

    assert_eq!(
      first_row.shading,
      Some(Some(RgbColor {
        r: 0x44,
        g: 0x72,
        b: 0xC4
      }))
    );
    assert_eq!(
      first_row.paragraph_format.alignment,
      ParagraphAlignment::Center
    );
    assert!(first_row.run_style.bold);
    assert_eq!(
      first_row.run_style.color,
      RgbColor {
        r: 0xFF,
        g: 0xFF,
        b: 0xFF
      }
    );
    assert_eq!(
      body_row.shading,
      Some(Some(RgbColor {
        r: 0xEE,
        g: 0xEE,
        b: 0xEE
      }))
    );
  }

  #[test]
  fn table_style_column_and_corner_conditions_apply_by_cell_position() {
    fn style(fill: &str) -> TableCellStyle {
      TableCellStyle {
        shading: Some(Some(parse_hex_color(fill).unwrap())),
        ..Default::default()
      }
    }

    let table_style = TableStyleModel {
      conditional: vec![
        (w::TableStyleOverrideValues::LastColumn, style("00FF00")),
        (w::TableStyleOverrideValues::NorthEastCell, style("FF0000")),
      ],
      ..Default::default()
    };
    let look = TableLookModel {
      last_column: true,
      ..Default::default()
    };

    let top_right = table_cell_style_for(
      &table_style,
      TableCellStyleContext {
        look,
        row_index: 0,
        row_count: 2,
        cell_index: 2,
        cell_count: 3,
        row_condition: TableConditionalStyleMask::from_row_position(look, 0, 2),
        cell_condition: None,
      },
    );
    let body_right = table_cell_style_for(
      &table_style,
      TableCellStyleContext {
        look,
        row_index: 1,
        row_count: 2,
        cell_index: 2,
        cell_count: 3,
        row_condition: TableConditionalStyleMask::from_row_position(look, 1, 2),
        cell_condition: None,
      },
    );

    assert_eq!(
      top_right.shading,
      Some(Some(RgbColor {
        r: 0xFF,
        g: 0x00,
        b: 0x00
      }))
    );
    assert_eq!(
      body_right.shading,
      Some(Some(RgbColor {
        r: 0x00,
        g: 0xFF,
        b: 0x00
      }))
    );
  }

  #[test]
  fn direct_cell_borders_overlay_style_borders_per_side() {
    fn border(width_pt: f32) -> BorderStyle {
      BorderStyle {
        width_pt,
        ..Default::default()
      }
    }

    let base = CellBordersModel {
      top: Some(border(1.0)),
      right: Some(border(1.5)),
      bottom: Some(border(2.0)),
      left: Some(border(2.5)),
    };
    let direct = w::TableCellBorders {
      top_border: Some(w::TopBorder {
        val: w::BorderValues::None,
        ..Default::default()
      }),
      right_border: Some(w::RightBorder {
        val: w::BorderValues::Single,
        size: Some(24),
        ..Default::default()
      }),
      ..Default::default()
    };
    let merged = direct_cell_borders_model(base, &direct);
    let suppressions = cell_border_suppressions(&direct);

    assert_eq!(merged.top, None);
    assert_eq!(merged.right.unwrap().width_pt, 3.0);
    assert_eq!(merged.bottom, Some(border(2.0)));
    assert_eq!(merged.left, Some(border(2.5)));
    assert!(suppressions.top);
    assert!(!suppressions.right);
  }

  #[test]
  fn word_border_dash_values_are_preserved_as_width_relative_patterns() {
    let cases = [
      (w::BorderValues::Dotted, BorderDashPattern::Dotted),
      (w::BorderValues::Dashed, BorderDashPattern::Dashed),
      (w::BorderValues::DashSmallGap, BorderDashPattern::FineDashed),
      (w::BorderValues::DotDash, BorderDashPattern::DashDot),
      (w::BorderValues::DotDotDash, BorderDashPattern::DashDotDot),
    ];
    for (value, expected) in cases {
      let border = border_style(value, Some(4), None, None, None).unwrap();
      assert_eq!(border.dash_pattern, expected);
    }

    let fine = border_style(w::BorderValues::DashSmallGap, Some(4), None, None, None).unwrap();
    assert_eq!(
      crate::model::common_stroke_from_border(fine, 1.0).dash,
      Some(vec![common::Pt(2.0), common::Pt(0.4)])
    );
  }

  #[test]
  fn direct_table_borders_overlay_style_borders_per_side() {
    fn border(width_pt: f32) -> BorderStyle {
      BorderStyle {
        width_pt,
        ..Default::default()
      }
    }

    let base = TableBordersModel {
      top: Some(border(1.0)),
      right: Some(border(1.5)),
      bottom: Some(border(2.0)),
      left: Some(border(2.5)),
      inside_horizontal: Some(border(3.0)),
      inside_vertical: Some(border(3.5)),
    };
    let merged = direct_table_borders_model(
      Some(base),
      &w::TableBorders {
        left_border: Some(w::LeftBorder {
          val: w::BorderValues::Double,
          size: Some(24),
          ..Default::default()
        }),
        right_border: Some(w::RightBorder {
          val: w::BorderValues::None,
          ..Default::default()
        }),
        ..Default::default()
      },
    );

    assert_eq!(merged.top, Some(border(1.0)));
    assert_eq!(merged.right, None);
    assert_eq!(merged.bottom, Some(border(2.0)));
    assert_eq!(merged.left.unwrap().width_pt, 3.0);
    assert_eq!(merged.inside_horizontal, Some(border(3.0)));
    assert_eq!(merged.inside_vertical, Some(border(3.5)));
  }

  #[test]
  fn table_cell_cnf_style_masks_apply_writer_corner_conditions() {
    fn style(fill: &str) -> TableCellStyle {
      TableCellStyle {
        shading: Some(Some(parse_hex_color(fill).unwrap())),
        ..Default::default()
      }
    }

    let table_style = TableStyleModel {
      conditional: vec![
        (w::TableStyleOverrideValues::FirstRow, style("4472C4")),
        (w::TableStyleOverrideValues::LastColumn, style("00FF00")),
        (w::TableStyleOverrideValues::NorthEastCell, style("FF0000")),
      ],
      ..Default::default()
    };
    let look = TableLookModel {
      first_row: false,
      first_column: false,
      horizontal_banding: false,
      vertical_banding: false,
      ..Default::default()
    };
    let row_condition = TableConditionalStyleMask::from_cnf_style(&w::ConditionalFormatStyle {
      val: Some("100000000000".into()),
      first_row: Some(true.into()),
      ..Default::default()
    });
    let cell_condition = TableConditionalStyleMask::from_cnf_style(&w::ConditionalFormatStyle {
      val: Some("000100000000".into()),
      last_column: Some(true.into()),
      ..Default::default()
    });

    let styled = table_cell_style_for(
      &table_style,
      TableCellStyleContext {
        look,
        row_index: 1,
        row_count: 3,
        cell_index: 0,
        cell_count: 2,
        row_condition,
        cell_condition: Some(cell_condition),
      },
    );

    assert_eq!(
      styled.shading,
      Some(Some(RgbColor {
        r: 0xFF,
        g: 0x00,
        b: 0x00
      }))
    );
  }

  #[test]
  fn table_style_row_properties_apply_and_direct_row_properties_override() {
    let style = table_style_model(
      &w::Style {
        r#type: Some(w::StyleValues::Table),
        table_style_properties: vec![w::TableStyleProperties {
          r#type: w::TableStyleOverrideValues::FirstRow,
          table_style_conditional_formatting_table_row_properties: Some(
            w::TableStyleConditionalFormattingTableRowProperties {
              table_style_conditional_formatting_table_row_properties_choice: vec![
                w::TableStyleConditionalFormattingTableRowPropertiesChoice::TableHeader(
                  w::TableHeader { val: None },
                ),
                w::TableStyleConditionalFormattingTableRowPropertiesChoice::CantSplit(
                  w::CantSplit { val: None },
                ),
                w::TableStyleConditionalFormattingTableRowPropertiesChoice::TableCellSpacing(
                  w::TableCellSpacing {
                    width: Some(measurement(240)),
                    r#type: Some(w::TableWidthUnitValues::Dxa),
                  },
                ),
              ],
            },
          ),
          ..Default::default()
        }],
        ..Default::default()
      },
      &ThemeFonts::default(),
      &ThemeColors::default(),
      ImportSettings::default(),
    );

    let mut first_row = table_row_style_for(
      &style,
      TableLookModel::default(),
      0,
      2,
      TableConditionalStyleMask::from_row_position(TableLookModel::default(), 0, 2),
    );
    let body_row = table_row_style_for(
      &style,
      TableLookModel::default(),
      1,
      2,
      TableConditionalStyleMask::from_row_position(TableLookModel::default(), 1, 2),
    );
    merge_table_row_style(
      &mut first_row,
      &direct_table_row_style(Some(&w::TableRowProperties {
        table_row_properties_choice1: vec![
          w::TableRowPropertiesChoice::TableHeader(w::TableHeader {
            val: Some(ooxmlsdk::simple_type::OnOffValue::Off),
          }),
          w::TableRowPropertiesChoice::TableCellSpacing(w::TableCellSpacing {
            width: Some(measurement(120)),
            r#type: Some(w::TableWidthUnitValues::Dxa),
          }),
        ],
        ..Default::default()
      })),
    );

    assert_eq!(first_row.repeat_header, Some(false));
    assert_eq!(first_row.cant_split, Some(true));
    assert_eq!(first_row.cell_spacing_pt, Some(6.0));
    assert_eq!(body_row.repeat_header, None);
    assert_eq!(body_row.cant_split, None);
    assert_eq!(body_row.cell_spacing_pt, None);
  }

  #[test]
  fn table_style_conditional_table_properties_apply_to_table_level_model() {
    let style = table_style_model(
      &w::Style {
        r#type: Some(w::StyleValues::Table),
        table_style_properties: vec![w::TableStyleProperties {
          r#type: w::TableStyleOverrideValues::WholeTable,
          table_style_conditional_formatting_table_properties: Some(Box::new(
            w::TableStyleConditionalFormattingTableProperties {
              table_justification: Some(w::TableJustification {
                val: w::TableRowAlignmentValues::Center,
              }),
              table_indentation: Some(w::TableIndentation {
                width: Some(measurement(720)),
                r#type: Some(w::TableWidthUnitValues::Dxa),
              }),
              table_cell_spacing: Some(w::TableCellSpacing {
                width: Some(measurement(120)),
                r#type: Some(w::TableWidthUnitValues::Dxa),
              }),
              ..Default::default()
            },
          )),
          ..Default::default()
        }],
        ..Default::default()
      },
      &ThemeFonts::default(),
      &ThemeColors::default(),
      ImportSettings::default(),
    );

    assert_eq!(style.alignment, Some(TableAlignment::Center));
    assert_eq!(style.indent_left_pt, Some(36.0));
    assert_eq!(style.cell_spacing_pt, Some(6.0));
  }

  #[test]
  fn explicit_zero_paragraph_spacing_overrides_doc_default_spacing() {
    // spacing properties into the property map even when the value is zero.
    let mut format = ParagraphFormat {
      spacing_after_pt: 8.0,
      spacing_after_set: true,
      ..Default::default()
    };

    merge_format_values(
      &mut format,
      &ParagraphFormat {
        spacing_after_pt: 0.0,
        spacing_after_set: true,
        ..Default::default()
      },
    );

    assert_eq!(format.spacing_after_pt, 0.0);
    assert!(format.spacing_after_set);
  }

  #[test]
  fn explicit_false_paragraph_booleans_override_inherited_true_values() {
    let mut format = ParagraphFormat {
      page_break_before: true,
      page_break_before_set: true,
      keep_with_next: true,
      keep_with_next_set: true,
      keep_lines: true,
      keep_lines_set: true,
      widow_control: Some(true),
      contextual_spacing: true,
      contextual_spacing_set: true,
      suppress_line_numbers: Some(true),
      ..Default::default()
    };

    merge_format_values(
      &mut format,
      &ParagraphFormat {
        page_break_before_set: true,
        keep_with_next_set: true,
        keep_lines_set: true,
        widow_control: Some(false),
        contextual_spacing_set: true,
        suppress_line_numbers: Some(false),
        ..Default::default()
      },
    );

    assert!(!format.page_break_before);
    assert!(!format.keep_with_next);
    assert!(!format.keep_lines);
    assert_eq!(format.widow_control, Some(false));
    assert!(!format.contextual_spacing);
    assert_eq!(format.suppress_line_numbers, Some(false));
  }

  #[test]
  fn line_number_suppression_preserves_explicit_enable_and_disable() {
    let enabled = w::ParagraphProperties::from_bytes(
      br#"<w:pPr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:suppressLineNumbers/></w:pPr>"#,
    )
    .expect("enabled line-number suppression");
    let disabled = w::ParagraphProperties::from_bytes(
      br#"<w:pPr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:suppressLineNumbers w:val="false"/></w:pPr>"#,
    )
    .expect("disabled line-number suppression");
    let mut format = ParagraphFormat::default();

    merge_paragraph_format(
      &mut format,
      Some(ParagraphProps::Direct(&enabled)),
      ImportSettings::default(),
    );
    assert_eq!(format.suppress_line_numbers, Some(true));

    merge_paragraph_format(
      &mut format,
      Some(ParagraphProps::Direct(&disabled)),
      ImportSettings::default(),
    );
    assert_eq!(format.suppress_line_numbers, Some(false));
  }

  #[test]
  fn missing_styles_recovers_main_story_normal_spacing() {
    let paragraph = w::Paragraph::from_bytes(
      br#"<w:p xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:r><w:t>footer</w:t></w:r></w:p>"#,
    )
    .expect("footer paragraph");
    let styles = StylesCatalog::default();
    let mut numbering = NumberingCatalog::default();
    let mut model = paragraph_model(
      &paragraph,
      &styles,
      &mut numbering,
      &ImageCatalog::default(),
      &HyperlinkCatalog::default(),
      &CustomXmlBindings::default(),
      &mut FormWidgetIdAllocator::default(),
    );

    assert_eq!(model.format.spacing_after_pt, 0.0);
    assert_eq!(model.format.line_height_pt, None);

    apply_recovered_body_paragraph_defaults(&paragraph, &styles, &mut model);

    assert_eq!(model.format.spacing_after_pt, 8.0);
    assert!(model.format.spacing_after_set);
    assert_eq!(model.format.line_height_pt, Some(276.0 / 240.0));
    assert_eq!(model.format.line_height_rule, LineHeightRule::Auto);
  }

  #[test]
  fn missing_styles_keeps_body_normal_spacing_for_simplified_chinese() {
    let paragraph = w::Paragraph::from_bytes(
      br#"<w:p xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:r><w:t>body</w:t></w:r></w:p>"#,
    )
    .expect("body paragraph");
    let styles = StylesCatalog {
      simplified_chinese_ui: true,
      ..Default::default()
    };
    let mut model = paragraph_model(
      &paragraph,
      &styles,
      &mut NumberingCatalog::default(),
      &ImageCatalog::default(),
      &HyperlinkCatalog::default(),
      &CustomXmlBindings::default(),
      &mut FormWidgetIdAllocator::default(),
    );

    apply_recovered_body_paragraph_defaults(&paragraph, &styles, &mut model);

    assert_eq!(model.format.line_height_pt, Some(276.0 / 240.0));
    assert_eq!(model.format.line_height_rule, LineHeightRule::Auto);
  }

  #[test]
  fn default_style_without_paragraph_defaults_recovers_main_story_spacing() {
    let paragraph = w::Paragraph::from_bytes(
      br#"<w:p xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:r><w:t>body</w:t></w:r></w:p>"#,
    )
    .expect("body paragraph");
    let styles = StylesCatalog {
      has_styles_part: true,
      default_paragraph_style_id: Some("Normal".to_string()),
      ..Default::default()
    };
    let mut numbering = NumberingCatalog::default();
    let mut model = paragraph_model(
      &paragraph,
      &styles,
      &mut numbering,
      &ImageCatalog::default(),
      &HyperlinkCatalog::default(),
      &CustomXmlBindings::default(),
      &mut FormWidgetIdAllocator::default(),
    );

    apply_recovered_body_paragraph_defaults(&paragraph, &styles, &mut model);

    assert_eq!(model.format.spacing_after_pt, 8.0);
    assert_eq!(model.format.line_height_pt, Some(276.0 / 240.0));
  }

  #[test]
  fn styles_without_paragraph_defaults_recovers_beneath_explicit_paragraph_style() {
    let paragraph = w::Paragraph::from_bytes(
      br#"<w:p xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:pPr><w:pStyle w:val="AAAA"/></w:pPr><w:r><w:t>body</w:t></w:r></w:p>"#,
    )
    .expect("styled body paragraph");
    let styles = StylesCatalog {
      has_styles_part: true,
      ..Default::default()
    };
    let mut numbering = NumberingCatalog::default();
    let mut model = paragraph_model(
      &paragraph,
      &styles,
      &mut numbering,
      &ImageCatalog::default(),
      &HyperlinkCatalog::default(),
      &CustomXmlBindings::default(),
      &mut FormWidgetIdAllocator::default(),
    );

    apply_recovered_body_paragraph_defaults(&paragraph, &styles, &mut model);

    assert_eq!(model.format.spacing_after_pt, 8.0);
    assert_eq!(model.format.line_height_pt, Some(276.0 / 240.0));
  }

  #[test]
  fn negative_word_line_spacing_uses_positive_writer_compatibility_modes() {
    fn imported_spacing(
      inherited_rule: LineHeightRule,
      line_rule: Option<w::LineSpacingRuleValues>,
    ) -> ParagraphFormat {
      let properties = w::ParagraphProperties {
        spacing_between_lines: Some(w::SpacingBetweenLines {
          line: Some(signed_twips(-240)),
          line_rule,
          ..Default::default()
        }),
        ..Default::default()
      };
      let mut format = ParagraphFormat {
        line_height_rule: inherited_rule,
        line_height_pt: Some(12.0),
        ..Default::default()
      };
      merge_paragraph_format(
        &mut format,
        Some(ParagraphProps::Direct(&properties)),
        ImportSettings::default(),
      );
      format
    }

    let explicit_exact =
      imported_spacing(LineHeightRule::Exact, Some(w::LineSpacingRuleValues::Exact));
    assert_eq!(explicit_exact.line_height_rule, LineHeightRule::AtLeast);
    assert_eq!(explicit_exact.line_height_pt, Some(12.0));

    for (inherited, rule) in [
      (
        LineHeightRule::AtLeast,
        Some(w::LineSpacingRuleValues::AtLeast),
      ),
      (LineHeightRule::Auto, Some(w::LineSpacingRuleValues::Auto)),
      (LineHeightRule::Exact, None),
    ] {
      let format = imported_spacing(inherited, rule);
      assert_eq!(format.line_height_rule, LineHeightRule::Exact);
      assert_eq!(format.line_height_pt, Some(12.0));
    }
  }

  #[test]
  fn automatic_paragraph_spacing_preserves_explicit_enable_and_disable() {
    let enabled = w::ParagraphProperties::from_bytes(
      br#"<w:pPr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:spacing w:before="80" w:beforeAutospacing="on" w:after="160" w:afterAutospacing="on"/></w:pPr>"#,
    )
    .expect("paragraph properties");
    let disabled = w::ParagraphProperties::from_bytes(
      br#"<w:pPr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:spacing w:beforeAutospacing="off" w:afterAutospacing="off"/></w:pPr>"#,
    )
    .expect("paragraph properties");
    let mut format = ParagraphFormat::default();

    merge_paragraph_format(
      &mut format,
      Some(ParagraphProps::Direct(&enabled)),
      ImportSettings::default(),
    );

    assert_eq!(format.spacing_before_pt, 4.0);
    assert_eq!(format.spacing_before_auto, Some(true));
    assert_eq!(
      format.spacing_before_auto_pt,
      Some(OFFICE_AUTOMATIC_PARAGRAPH_SPACING_PT)
    );
    assert!(format.spacing_before_set);
    assert_eq!(format.spacing_after_pt, 8.0);
    assert_eq!(format.spacing_after_auto, Some(true));
    assert_eq!(
      format.spacing_after_auto_pt,
      Some(OFFICE_AUTOMATIC_PARAGRAPH_SPACING_PT)
    );
    assert!(format.spacing_after_set);

    merge_paragraph_format(
      &mut format,
      Some(ParagraphProps::Direct(&disabled)),
      ImportSettings::default(),
    );

    assert_eq!(format.spacing_before_pt, 4.0);
    assert_eq!(format.spacing_before_auto, Some(false));
    assert_eq!(format.spacing_before_auto_pt, None);
    assert_eq!(format.spacing_after_pt, 8.0);
    assert_eq!(format.spacing_after_auto, Some(false));
    assert_eq!(format.spacing_after_auto_pt, None);

    merge_paragraph_format(
      &mut format,
      Some(ParagraphProps::Direct(&enabled)),
      ImportSettings {
        fixed_html_paragraph_auto_spacing: true,
        ..Default::default()
      },
    );

    assert_eq!(
      format.spacing_before_auto_pt,
      Some(OFFICE_FIXED_AUTOMATIC_PARAGRAPH_BEFORE_PT)
    );
    assert_eq!(
      format.spacing_after_auto_pt,
      Some(OFFICE_FIXED_AUTOMATIC_PARAGRAPH_AFTER_PT)
    );
  }

  #[test]
  fn table_style_text_properties_are_base_for_direct_paragraph_and_run_properties() {
    let base_format = ParagraphFormat {
      alignment: ParagraphAlignment::Center,
      ..Default::default()
    };
    let base_run_style = TextStyle {
      color: RgbColor {
        r: 0xFF,
        g: 0xFF,
        b: 0xFF,
      },
      ..Default::default()
    };
    let base_run_overrides = RunStyleOverrides {
      bold: Some(true),
      ..Default::default()
    };

    let paragraph = w::Paragraph {
      paragraph_properties: Some(Box::new(w::ParagraphProperties {
        justification: Some(w::Justification {
          val: w::JustificationValues::Left,
        }),
        ..Default::default()
      })),
      paragraph_choice: vec![w::ParagraphChoice::WRun(Box::new(w::Run {
        run_properties: Some(Box::new(w::RunProperties {
          run_properties_choice: vec![
            w::RunPropertiesChoice::Bold(w::Bold {
              val: Some(false.into()),
            }),
            w::RunPropertiesChoice::Color(Box::new(w::Color {
              val: Some("0000FF".into()),
              ..Default::default()
            })),
          ],
          ..Default::default()
        })),
        run_choice: vec![w::RunChoice::Text(text("Header"))],
        ..Default::default()
      }))],
      ..Default::default()
    };
    let mut numbering = NumberingCatalog::default();

    let paragraph = paragraph_model_with_base(
      &paragraph,
      &StylesCatalog::default(),
      &mut numbering,
      &ImageCatalog::default(),
      &HyperlinkCatalog::default(),
      &mut FormWidgetIdAllocator::default(),
      ParagraphImportBase {
        format: base_format,
        run_style: base_run_style,
        run_overrides: base_run_overrides,
        ..Default::default()
      },
    );

    assert_eq!(paragraph.format.alignment, ParagraphAlignment::Left);
    let InlineItem::Text(run) = &paragraph.inlines[0] else {
      panic!("expected text run");
    };
    assert!(!run.style.bold);
    assert_eq!(
      run.style.color,
      RgbColor {
        r: 0x00,
        g: 0x00,
        b: 0xFF
      }
    );
    assert_eq!(paragraph.runs[0].style, run.style);
  }

  #[test]
  fn paragraph_mark_run_properties_are_base_for_empty_line_height() {
    let paragraph = w::Paragraph {
      paragraph_properties: Some(Box::new(w::ParagraphProperties {
        paragraph_mark_run_properties: Some(Box::new(w::ParagraphMarkRunProperties {
          paragraph_mark_run_properties_choice2: vec![
            w::ParagraphMarkRunPropertiesChoice2::FontSize(w::FontSize { val: hps(96) }),
            w::ParagraphMarkRunPropertiesChoice2::FontSizeComplexScript(w::FontSizeComplexScript {
              val: hps(96),
            }),
          ],
          ..Default::default()
        })),
        ..Default::default()
      })),
      paragraph_choice: vec![w::ParagraphChoice::WRun(Box::new(w::Run {
        run_choice: vec![w::RunChoice::Text(text("visible"))],
        ..Default::default()
      }))],
      ..Default::default()
    };
    let mut numbering = NumberingCatalog::default();

    let paragraph = paragraph_model_with_base(
      &paragraph,
      &StylesCatalog::default(),
      &mut numbering,
      &ImageCatalog::default(),
      &HyperlinkCatalog::default(),
      &mut FormWidgetIdAllocator::default(),
      ParagraphImportBase::default(),
    );

    assert_eq!(paragraph.base_style.font_size_pt, 48.0);
    assert_eq!(paragraph.base_style.complex_font_size_pt, Some(48.0));
    let InlineItem::Text(run) = &paragraph.inlines[0] else {
      panic!("expected text run");
    };
    assert_eq!(run.style.font_size_pt, TextStyle::default().font_size_pt);
  }

  #[test]
  fn table_style_text_properties_apply_to_cell_paragraph_runs() {
    let style = TableCellStyle {
      paragraph_format: ParagraphFormat {
        alignment: ParagraphAlignment::Center,
        ..Default::default()
      },
      run_style: TextStyle {
        color: RgbColor {
          r: 0xFF,
          g: 0xFF,
          b: 0xFF,
        },
        ..Default::default()
      },
      run_overrides: RunStyleOverrides {
        bold: Some(true),
        ..Default::default()
      },
      ..Default::default()
    };

    let cell = w::TableCell {
      table_cell_choice: vec![w::TableCellChoice::Paragraph(Box::new(w::Paragraph {
        paragraph_choice: vec![w::ParagraphChoice::WRun(Box::new(w::Run {
          run_choice: vec![w::RunChoice::Text(text("Header"))],
          ..Default::default()
        }))],
        ..Default::default()
      }))],
      ..Default::default()
    };
    let mut numbering = NumberingCatalog::default();
    let mut form_widget_ids = FormWidgetIdAllocator::default();
    let styles = StylesCatalog::default();
    let images = ImageCatalog::default();
    let hyperlinks = HyperlinkCatalog::default();
    let custom_xml_bindings = CustomXmlBindings::default();
    let mut context = TableImportContext {
      styles: &styles,
      numbering: &mut numbering,
      images: &images,
      hyperlinks: &hyperlinks,
      custom_xml_bindings: &custom_xml_bindings,
      form_widget_ids: &mut form_widget_ids,
      cell_margins: CellMargins::default(),
      direct_cell_margins: false,
      table_shading: None,
      table_borders: None,
      table_style: &TableStyleModel::default(),
      table_look: TableLookModel::default(),
      row_count: 1,
      nested_table_level: 1,
      in_header_footer: false,
    };

    let cell = table_cell_model(&cell, None, &mut context, None, None, style);

    let Block::Paragraph(paragraph) = &cell.blocks[0] else {
      panic!("expected paragraph");
    };
    assert_eq!(paragraph.format.alignment, ParagraphAlignment::Center);
    let InlineItem::Text(inline_run) = &paragraph.inlines[0] else {
      panic!("expected text run");
    };
    assert!(inline_run.style.bold);
    assert_eq!(
      inline_run.style.color,
      RgbColor {
        r: 0xFF,
        g: 0xFF,
        b: 0xFF
      }
    );
    assert!(paragraph.runs[0].style.bold);
    assert_eq!(paragraph.runs[0].style.color, inline_run.style.color);
  }

  #[test]
  fn simple_page_fields_emit_dynamic_markers() {
    let mut inlines = Vec::new();
    let field = w::SimpleField {
      instruction: " PAGE ".into(),
      ..Default::default()
    };
    let styles = StylesCatalog::default();
    let images = ImageCatalog::default();
    let hyperlinks = HyperlinkCatalog::default();
    let custom_xml_bindings = CustomXmlBindings::default();
    let mut form_widget_ids = FormWidgetIdAllocator::default();
    let mut context = InlineImportContext {
      styles: &styles,
      images: &images,
      hyperlinks: &hyperlinks,
      custom_xml_bindings: &custom_xml_bindings,
      form_widget_ids: &mut form_widget_ids,
      suppress_toc_hyperlink_style: false,
    };

    push_simple_field(&field, &mut inlines, TextStyle::default(), &mut context);

    let InlineItem::Text(run) = &inlines[0] else {
      panic!("expected dynamic field text");
    };
    assert_eq!(
      run.dynamic_field,
      Some(DynamicFieldKind::Page {
        number_format: FieldNumberFormat::PageStyle,
      })
    );
  }

  #[test]
  fn simple_page_field_uses_the_cached_result_run_style() {
    let field = w::SimpleField::from_bytes(
      br#"<w:fldSimple xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" w:instr=" PAGE \* MERGEFORMAT "><w:r><w:rPr><w:rFonts w:ascii="Courier New" w:hAnsi="Courier New"/></w:rPr><w:t>2</w:t></w:r></w:fldSimple>"#,
    )
    .expect("simple PAGE field");
    let styles = StylesCatalog::default();
    let images = ImageCatalog::default();
    let hyperlinks = HyperlinkCatalog::default();
    let custom_xml_bindings = CustomXmlBindings::default();
    let mut form_widget_ids = FormWidgetIdAllocator::default();
    let mut context = InlineImportContext {
      styles: &styles,
      images: &images,
      hyperlinks: &hyperlinks,
      custom_xml_bindings: &custom_xml_bindings,
      form_widget_ids: &mut form_widget_ids,
      suppress_toc_hyperlink_style: false,
    };
    let mut inlines = Vec::new();

    push_simple_field(&field, &mut inlines, TextStyle::default(), &mut context);

    let InlineItem::Text(run) = &inlines[0] else {
      panic!("expected dynamic field text");
    };
    assert_eq!(run.style.font_family.as_deref(), Some("Courier New"));
  }

  #[test]
  fn date_field_refresh_is_typed_opt_in_and_respects_field_lock() {
    fn import_field(field: &w::SimpleField, styles: &StylesCatalog) -> TextRun {
      let images = ImageCatalog::default();
      let hyperlinks = HyperlinkCatalog::default();
      let custom_xml_bindings = CustomXmlBindings::default();
      let mut form_widget_ids = FormWidgetIdAllocator::default();
      let mut context = InlineImportContext {
        styles,
        images: &images,
        hyperlinks: &hyperlinks,
        custom_xml_bindings: &custom_xml_bindings,
        form_widget_ids: &mut form_widget_ids,
        suppress_toc_hyperlink_style: false,
      };
      let mut inlines = Vec::new();
      push_simple_field(
        field,
        &mut inlines,
        TextStyle {
          language: Some(Arc::<str>::from("en-US")),
          ..TextStyle::default()
        },
        &mut context,
      );
      let [InlineItem::Text(run)] = inlines.as_slice() else {
        panic!("expected one date field result");
      };
      run.clone()
    }

    let unlocked = w::SimpleField::from_bytes(
      br#"<w:fldSimple xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" w:instr=" DATE \* MERGEFORMAT "><w:r><w:t>2/15/2008</w:t></w:r></w:fldSimple>"#,
    )
    .expect("unlocked DATE field");
    let locked = w::SimpleField::from_bytes(
      br#"<w:fldSimple xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" w:instr=" DATE \* MERGEFORMAT " w:fldLock="true"><w:r><w:t>2/15/2008</w:t></w:r></w:fldSimple>"#,
    )
    .expect("locked DATE field");
    let print_date = w::SimpleField::from_bytes(
      br#"<w:fldSimple xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" w:instr=" PRINTDATE \* MERGEFORMAT "><w:r><w:t>1/23/4567 8:9:10 PM</w:t></w:r></w:fldSimple>"#,
    )
    .expect("unlocked PRINTDATE field");
    let locked_print_date = w::SimpleField::from_bytes(
      br#"<w:fldSimple xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" w:instr=" PRINTDATE \* MERGEFORMAT " w:fldLock="true"><w:r><w:t>1/23/4567 8:9:10 PM</w:t></w:r></w:fldSimple>"#,
    )
    .expect("locked PRINTDATE field");
    let create_date = w::SimpleField::from_bytes(
      br#"<w:fldSimple xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" w:instr=" CREATEDATE \* MERGEFORMAT "><w:r><w:t>7/7/2020 10:11:00 AM</w:t></w:r></w:fldSimple>"#,
    )
    .expect("CREATEDATE field");
    let updating_styles = StylesCatalog {
      import_settings: ImportSettings {
        field_update_datetime: Some(FieldUpdateDateTime {
          year: 2026,
          month: 7,
          day: 12,
          hour: 15,
          minute: 21,
          second: 43,
        }),
        ..Default::default()
      },
      ..Default::default()
    };

    assert_eq!(import_field(&unlocked, &updating_styles).text, "7/12/2026");
    assert_eq!(import_field(&locked, &updating_styles).text, "2/15/2008");
    assert_eq!(
      import_field(&print_date, &updating_styles).text,
      "7/12/2026 3:21:00 PM"
    );
    assert_eq!(
      import_field(&locked_print_date, &updating_styles).text,
      "1/23/4567 8:9:10 PM"
    );
    assert_eq!(
      import_field(&create_date, &updating_styles).text,
      "7/7/2020 10:11:00 AM"
    );
    assert_eq!(
      import_field(&unlocked, &StylesCatalog::default()).text,
      "2/15/2008"
    );
  }

  #[test]
  fn current_date_formtext_refreshes_the_outer_result_once() {
    let paragraph = w::Paragraph::from_bytes(
      br#"<w:p xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
        <w:r><w:fldChar w:fldCharType="begin"><w:ffData><w:textInput><w:type w:val="currentDate"/><w:format w:val="M/d/yyyy h:mm:ss am/pm"/></w:textInput></w:ffData></w:fldChar></w:r>
        <w:r><w:instrText xml:space="preserve"> FORMTEXT </w:instrText></w:r>
        <w:r><w:fldChar w:fldCharType="begin"/></w:r>
        <w:r><w:instrText xml:space="preserve"> TIME \@ "M/d/yyyy h:mm:ss am/pm" </w:instrText></w:r>
        <w:r><w:fldChar w:fldCharType="separate"/></w:r>
        <w:r><w:instrText>10/23/2018 11:12:43 AM</w:instrText></w:r>
        <w:r><w:fldChar w:fldCharType="end"/></w:r>
        <w:r><w:fldChar w:fldCharType="separate"/></w:r>
        <w:r><w:t>10/22/2018 5:19:27 PM</w:t></w:r>
        <w:r><w:fldChar w:fldCharType="end"/></w:r>
      </w:p>"#,
    )
    .expect("nested current-date FORMTEXT");
    let styles = StylesCatalog {
      import_settings: ImportSettings {
        field_update_datetime: Some(FieldUpdateDateTime {
          year: 2026,
          month: 7,
          day: 12,
          hour: 20,
          minute: 19,
          second: 54,
        }),
        ..Default::default()
      },
      ..Default::default()
    };
    let mut form_widget_ids = FormWidgetIdAllocator::default();
    let inlines = paragraph_inlines(
      &paragraph,
      TextStyle {
        language: Some(Arc::<str>::from("en-GB")),
        ..TextStyle::default()
      },
      &styles,
      &ImageCatalog::default(),
      &HyperlinkCatalog::default(),
      &CustomXmlBindings::default(),
      &mut form_widget_ids,
    );

    assert_eq!(
      field_result_text(&inlines).as_deref(),
      Some("7/12/2026 8:19:54 PM")
    );
  }

  #[test]
  fn page_field_instruction_preserves_numeric_format_switch() {
    assert_eq!(
      dynamic_field_kind(r" PAGE \* roman "),
      Some(DynamicFieldKind::Page {
        number_format: FieldNumberFormat::LowerRoman,
      })
    );
    assert_eq!(
      dynamic_field_kind(r" NUMPAGES \* ALPHABETIC "),
      Some(DynamicFieldKind::NumPages {
        number_format: FieldNumberFormat::UpperLetter,
      })
    );
  }

  #[test]
  fn symbol_field_without_cached_result_uses_declared_symbol_font_encoding() {
    let run = symbol_field_run(
      r#"\SYMBOL 94 \f "Symbol""#,
      TextStyle {
        font_family: Some(Arc::from("Times New Roman")),
        font_size_pt: 12.0,
        ..TextStyle::default()
      },
      None,
    )
    .expect("supported symbol field");

    assert_eq!(run.text, "\u{f05e}");
    assert_eq!(run.style.font_family.as_deref(), Some("Symbol"));
    assert_eq!(run.style.symbol_font_family.as_deref(), Some("Symbol"));
    assert_eq!(run.style.font_size_pt, 12.0);
  }

  #[test]
  fn symbol_field_unicode_and_size_switches_override_direct_formatting() {
    let run = symbol_field_run(r"SYMBOL 0x20ac \u \s 18", TextStyle::default(), None)
      .expect("supported Unicode symbol field");

    assert_eq!(run.text, "€");
    assert_eq!(run.style.font_size_pt, 18.0);
    assert_eq!(run.style.complex_font_size_pt, Some(18.0));
    assert!(symbol_field_run(r"SYMBOL 65 \h", TextStyle::default(), None).is_none());
  }

  #[test]
  fn pageref_field_instruction_emits_bookmark_page_marker() {
    assert_eq!(
      dynamic_field_kind(r#" PAGEREF "_Toc123" \h "#),
      Some(DynamicFieldKind::PageRef {
        bookmark_name: Arc::<str>::from("_Toc123"),
        number_format: FieldNumberFormat::PageStyle,
        relative_position: false,
      })
    );
  }

  #[test]
  fn pageref_field_instruction_preserves_relative_and_number_format_switches() {
    assert_eq!(
      dynamic_field_kind(r#" PAGEREF "_Toc123" \p \* ROMAN "#),
      Some(DynamicFieldKind::PageRef {
        bookmark_name: Arc::<str>::from("_Toc123"),
        number_format: FieldNumberFormat::UpperRoman,
        relative_position: true,
      })
    );
    assert_eq!(
      dynamic_field_kind(r#" PAGEREF \* alphabetic "_Toc456" \h "#),
      Some(DynamicFieldKind::PageRef {
        bookmark_name: Arc::<str>::from("_Toc456"),
        number_format: FieldNumberFormat::LowerLetter,
        relative_position: false,
      })
    );
  }

  #[test]
  fn internal_word_hyperlink_uses_the_pdf_bookmark_target_namespace() {
    let hyperlink = w::Hyperlink {
      anchor: Some("_Toc123".into()),
      ..Default::default()
    };
    assert_eq!(
      hyperlink_url(&hyperlink, &HyperlinkCatalog::default()).as_deref(),
      Some("ooxmlsdk-pdf:bookmark:_Toc123")
    );
  }

  #[test]
  fn cached_toc_hyperlink_ignores_hyperlink_character_style_but_keeps_link_target() {
    fn imported_run(xml: &[u8], styles: &StylesCatalog) -> TextRun {
      let paragraph = w::Paragraph::from_bytes(xml).expect("hyperlink paragraph");
      let mut form_widget_ids = FormWidgetIdAllocator::default();
      let inlines = paragraph_inlines(
        &paragraph,
        TextStyle::default(),
        styles,
        &ImageCatalog::default(),
        &HyperlinkCatalog::default(),
        &CustomXmlBindings::default(),
        &mut form_widget_ids,
      );
      let [InlineItem::Text(run)] = inlines.as_slice() else {
        panic!("expected one hyperlink text run");
      };
      run.clone()
    }

    let styles = StylesCatalog {
      styles: HashMap::from([
        (
          "a3".to_string(),
          StyleEntry {
            style_type: Some(w::StyleValues::Character),
            name: Some("Hyperlink".to_string()),
            run_style: TextStyle {
              underline: true,
              color: RgbColor {
                r: 0x05,
                g: 0x63,
                b: 0xC1,
              },
              color_is_automatic: false,
              ..TextStyle::default()
            },
            ..StyleEntry::default()
          },
        ),
        (
          "10".to_string(),
          StyleEntry {
            style_type: Some(w::StyleValues::Paragraph),
            name: Some("toc 1".to_string()),
            ..StyleEntry::default()
          },
        ),
      ]),
      ..StylesCatalog::default()
    };
    let toc = imported_run(
      br#"<w:p xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:r><w:fldChar w:fldCharType="begin"/></w:r><w:r><w:instrText> TOC \o "1-3" \h </w:instrText></w:r><w:r><w:fldChar w:fldCharType="separate"/></w:r><w:hyperlink w:anchor="_Toc123"><w:r><w:rPr><w:rStyle w:val="a3"/><w:b/></w:rPr><w:t>Entry</w:t></w:r></w:hyperlink><w:r><w:fldChar w:fldCharType="end"/></w:r></w:p>"#,
      &styles,
    );
    assert_eq!(toc.text, "Entry");
    assert_eq!(
      toc.hyperlink_url.as_deref(),
      Some("ooxmlsdk-pdf:bookmark:_Toc123")
    );
    assert!(toc.style.bold);
    assert!(!toc.style.underline);
    assert_eq!(toc.style.color, RgbColor { r: 0, g: 0, b: 0 });

    let ordinary = imported_run(
      br#"<w:p xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:hyperlink w:anchor="_Toc123"><w:r><w:rPr><w:rStyle w:val="a3"/></w:rPr><w:t>Entry</w:t></w:r></w:hyperlink></w:p>"#,
      &styles,
    );
    assert!(ordinary.style.underline);
    assert_eq!(
      ordinary.style.color,
      RgbColor {
        r: 0x05,
        g: 0x63,
        b: 0xC1,
      }
    );

    let continuation = w::Paragraph::from_bytes(
      br#"<w:p xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:pPr><w:pStyle w:val="10"/></w:pPr><w:hyperlink w:anchor="_Toc124"><w:r><w:rPr><w:rStyle w:val="a3"/></w:rPr><w:t>Next entry</w:t></w:r></w:hyperlink></w:p>"#,
    )
    .expect("continued TOC cache paragraph");
    let mut numbering = NumberingCatalog::default();
    let mut form_widget_ids = FormWidgetIdAllocator::default();
    let model = paragraph_model(
      &continuation,
      &styles,
      &mut numbering,
      &ImageCatalog::default(),
      &HyperlinkCatalog::default(),
      &CustomXmlBindings::default(),
      &mut form_widget_ids,
    );
    let [InlineItem::Text(continuation)] = model.inlines.as_slice() else {
      panic!("expected one continued TOC entry run");
    };
    assert!(!continuation.style.underline);
    assert_eq!(continuation.style.color, RgbColor { r: 0, g: 0, b: 0 });
  }

  #[test]
  fn locked_pageref_keeps_its_persisted_result() {
    let paragraph = w::Paragraph::from_bytes(
      br#"<w:p xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:r><w:fldChar w:fldCharType="begin" w:fldLock="1"/></w:r><w:r><w:instrText>PAGEREF "_Toc123"</w:instrText></w:r><w:r><w:fldChar w:fldCharType="separate"/></w:r><w:r><w:t>27</w:t></w:r><w:r><w:fldChar w:fldCharType="end"/></w:r></w:p>"#,
    )
    .expect("locked complex field paragraph");
    let mut form_widget_ids = FormWidgetIdAllocator::default();
    let inlines = paragraph_inlines(
      &paragraph,
      TextStyle::default(),
      &StylesCatalog::default(),
      &ImageCatalog::default(),
      &HyperlinkCatalog::default(),
      &CustomXmlBindings::default(),
      &mut form_widget_ids,
    );
    let [InlineItem::Text(run)] = inlines.as_slice() else {
      panic!("expected the persisted field result");
    };
    assert_eq!(run.text, "27");
    assert_eq!(run.dynamic_field, None);
  }

  #[test]
  fn complex_pageref_field_uses_cached_result_for_layout_text() {
    let styles = StylesCatalog::default();
    let images = ImageCatalog::default();
    let hyperlinks = HyperlinkCatalog::default();
    let mut inlines = Vec::new();
    let mut complex_fields = Vec::new();
    let runs = [
      w::Run {
        run_choice: vec![w::RunChoice::FieldChar(Box::new(w::FieldChar {
          field_char_type: w::FieldCharValues::Begin,
          ..Default::default()
        }))],
        ..Default::default()
      },
      w::Run {
        run_choice: vec![w::RunChoice::FieldCode(w::FieldCode(w::TextType {
          xml_content: Some(r#" PAGEREF "_Toc123" \h "#.into()),
          ..Default::default()
        }))],
        ..Default::default()
      },
      w::Run {
        run_choice: vec![w::RunChoice::FieldChar(Box::new(w::FieldChar {
          field_char_type: w::FieldCharValues::Separate,
          ..Default::default()
        }))],
        ..Default::default()
      },
      w::Run {
        run_choice: vec![w::RunChoice::Text(text("27"))],
        ..Default::default()
      },
      w::Run {
        run_choice: vec![w::RunChoice::FieldChar(Box::new(w::FieldChar {
          field_char_type: w::FieldCharValues::End,
          ..Default::default()
        }))],
        ..Default::default()
      },
    ];

    for run in &runs {
      push_run_or_complex_field(
        run,
        &mut inlines,
        TextStyle::default(),
        RunImportContext {
          styles: &styles,
          images: &images,
          hyperlinks: &hyperlinks,
          suppress_toc_hyperlink_style: false,
        },
        None,
        &mut complex_fields,
      );
    }

    let InlineItem::Text(run) = &inlines[0] else {
      panic!("expected dynamic field text");
    };
    assert_eq!(run.text, "27");
    assert_eq!(
      run.dynamic_field,
      Some(DynamicFieldKind::PageRef {
        bookmark_name: Arc::<str>::from("_Toc123"),
        number_format: FieldNumberFormat::PageStyle,
        relative_position: false,
      })
    );
  }

  #[test]
  fn complex_page_mergeformat_preserves_cached_result_style() {
    let paragraph = w::Paragraph::from_bytes(
      br#"<w:p xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:r><w:fldChar w:fldCharType="begin"/></w:r><w:r><w:instrText> PAGE \* MERGEFORMAT </w:instrText></w:r><w:r><w:fldChar w:fldCharType="separate"/></w:r><w:r><w:rPr><w:b/></w:rPr><w:t>7</w:t></w:r><w:r><w:fldChar w:fldCharType="end"/></w:r></w:p>"#,
    )
    .expect("PAGE field with MERGEFORMAT");
    let mut form_widget_ids = FormWidgetIdAllocator::default();
    let inlines = paragraph_inlines(
      &paragraph,
      TextStyle::default(),
      &StylesCatalog::default(),
      &ImageCatalog::default(),
      &HyperlinkCatalog::default(),
      &CustomXmlBindings::default(),
      &mut form_widget_ids,
    );

    let [InlineItem::Text(run)] = inlines.as_slice() else {
      panic!("expected one dynamic PAGE result");
    };
    assert_eq!(run.text, "7");
    assert!(run.style.bold);
    assert_eq!(
      run.dynamic_field,
      Some(DynamicFieldKind::Page {
        number_format: FieldNumberFormat::PageStyle,
      })
    );
    assert!(field_uses_merge_format(r" PAGE \* mergeformat "));
    assert!(!field_uses_merge_format(" PAGE "));
  }

  #[test]
  fn positional_tab_retains_its_absolute_positioning_contract() {
    let paragraph = w::Paragraph::from_bytes(
      br#"<w:p xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:r><w:t>left</w:t><w:ptab w:alignment="right" w:relativeTo="margin" w:leader="dot"/><w:t>right</w:t></w:r></w:p>"#,
    )
    .expect("paragraph with positional tab");
    let mut form_widget_ids = FormWidgetIdAllocator::default();
    let inlines = paragraph_inlines(
      &paragraph,
      TextStyle::default(),
      &StylesCatalog::default(),
      &ImageCatalog::default(),
      &HyperlinkCatalog::default(),
      &CustomXmlBindings::default(),
      &mut form_widget_ids,
    );

    let [
      InlineItem::Text(left),
      InlineItem::PositionalTab(tab),
      InlineItem::Text(right),
    ] = inlines.as_slice()
    else {
      panic!("expected text, positional tab, text");
    };
    assert_eq!(left.text, "left");
    assert_eq!(right.text, "right");
    assert_eq!(tab.alignment, TabStopAlignment::Right);
    assert_eq!(tab.relative_to, PositionalTabBase::Margin);
    assert_eq!(tab.leader, TabLeader::Dot);
  }

  #[test]
  fn closed_field_without_instruction_drops_result_but_unclosed_field_keeps_it() {
    fn imported_text(xml: &[u8]) -> String {
      let paragraph = w::Paragraph::from_bytes(xml).expect("complex field paragraph");
      let mut form_widget_ids = FormWidgetIdAllocator::default();
      inline_text(&paragraph_inlines(
        &paragraph,
        TextStyle::default(),
        &StylesCatalog::default(),
        &ImageCatalog::default(),
        &HyperlinkCatalog::default(),
        &CustomXmlBindings::default(),
        &mut form_widget_ids,
      ))
    }

    let closed = br#"<w:p xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:r><w:fldChar w:fldCharType="begin"/></w:r><w:r><w:fldChar w:fldCharType="separate"/></w:r><w:r><w:t>stale result</w:t></w:r><w:r><w:fldChar w:fldCharType="end"/></w:r></w:p>"#;
    let unclosed = br#"<w:p xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:r><w:fldChar w:fldCharType="begin"/></w:r><w:r><w:fldChar w:fldCharType="separate"/></w:r><w:r><w:t>literal result</w:t></w:r></w:p>"#;

    assert_eq!(imported_text(closed), "");
    assert_eq!(imported_text(unclosed), "literal result");
  }

  #[test]
  fn complex_field_instruction_inside_inserted_revision_keeps_its_result() {
    let paragraph = w::Paragraph::from_bytes(
      br#"<w:p xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:r><w:fldChar w:fldCharType="begin"/></w:r><w:ins w:id="1" w:author="editor"><w:r><w:instrText> HYPERLINK "https://www.libreoffice.org/" </w:instrText></w:r></w:ins><w:r><w:fldChar w:fldCharType="separate"/></w:r><w:r><w:t>Libreoffice</w:t></w:r><w:r><w:fldChar w:fldCharType="end"/></w:r></w:p>"#,
    )
    .expect("complex field with inserted instruction");
    let mut form_widget_ids = FormWidgetIdAllocator::default();
    let inlines = paragraph_inlines(
      &paragraph,
      TextStyle::default(),
      &StylesCatalog::default(),
      &ImageCatalog::default(),
      &HyperlinkCatalog::default(),
      &CustomXmlBindings::default(),
      &mut form_widget_ids,
    );

    assert_eq!(inline_text(&inlines), "Libreoffice");
    let [InlineItem::Text(run)] = inlines.as_slice() else {
      panic!("expected hyperlink field result");
    };
    assert_eq!(
      run.hyperlink_url.as_deref(),
      Some("https://www.libreoffice.org/")
    );
  }

  #[test]
  fn closed_set_field_has_no_visible_value() {
    let paragraph = w::Paragraph::from_bytes(
      br#"<w:p xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:r><w:fldChar w:fldCharType="begin"/></w:r><w:r><w:instrText> SET TEST_VAR 99 </w:instrText></w:r><w:r><w:fldChar w:fldCharType="separate"/></w:r><w:r><w:t>99</w:t></w:r><w:r><w:fldChar w:fldCharType="end"/></w:r></w:p>"#,
    )
    .expect("SET field paragraph");
    let mut form_widget_ids = FormWidgetIdAllocator::default();
    let inlines = paragraph_inlines(
      &paragraph,
      TextStyle::default(),
      &StylesCatalog::default(),
      &ImageCatalog::default(),
      &HyperlinkCatalog::default(),
      &CustomXmlBindings::default(),
      &mut form_widget_ids,
    );

    assert_eq!(inline_text(&inlines), "");
  }

  #[test]
  fn empty_form_drop_down_field_uses_selected_list_entry() {
    let styles = StylesCatalog::default();
    let images = ImageCatalog::default();
    let hyperlinks = HyperlinkCatalog::default();
    let mut inlines = Vec::new();
    let mut complex_fields = Vec::new();
    let runs = [
      w::Run::from_bytes(
        br#"<w:r xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:fldChar w:fldCharType="begin"><w:ffData><w:ddList><w:default w:val="1"/><w:result w:val="2"/><w:listEntry w:val="1000"/><w:listEntry w:val="2000"/><w:listEntry w:val="3000"/></w:ddList></w:ffData></w:fldChar></w:r>"#,
      )
      .expect("form drop-down begin"),
      w::Run {
        run_choice: vec![w::RunChoice::FieldCode(w::FieldCode(w::TextType {
          xml_content: Some(" FORMDROPDOWN ".into()),
          ..Default::default()
        }))],
        ..Default::default()
      },
      w::Run {
        run_choice: vec![w::RunChoice::FieldChar(Box::new(w::FieldChar {
          field_char_type: w::FieldCharValues::Separate,
          ..Default::default()
        }))],
        ..Default::default()
      },
      w::Run {
        run_choice: vec![w::RunChoice::FieldChar(Box::new(w::FieldChar {
          field_char_type: w::FieldCharValues::End,
          ..Default::default()
        }))],
        ..Default::default()
      },
    ];

    for run in &runs {
      push_run_or_complex_field(
        run,
        &mut inlines,
        TextStyle::default(),
        RunImportContext {
          styles: &styles,
          images: &images,
          hyperlinks: &hyperlinks,
          suppress_toc_hyperlink_style: false,
        },
        None,
        &mut complex_fields,
      );
    }

    assert_eq!(field_result_text(&inlines).as_deref(), Some("3000"));

    let field = w::FieldChar::from_bytes(
      br#"<w:fldChar xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" w:fldCharType="begin"><w:ffData><w:ddList><w:listEntry w:val="first"/><w:listEntry w:val="second"/></w:ddList></w:ffData></w:fldChar>"#,
    )
    .expect("form drop-down without explicit selection");
    assert_eq!(form_drop_down_value(&field).as_deref(), Some("first"));
  }

  #[test]
  fn bound_date_uses_sdt_display_mask() {
    for (format, expected) in [
      ("M/d/yyyy", "4/26/2012"),
      ("dd/MM/yyyy", "26/04/2012"),
      ("d. M. yyyy", "26. 4. 2012"),
      ("dd-MM-yyyy", "26-04-2012"),
    ] {
      let properties = w::SdtProperties::from_bytes(
        format!(
          r#"<w:sdtPr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:date><w:dateFormat w:val="{format}"/></w:date></w:sdtPr>"#
        )
        .as_bytes(),
      )
      .expect("date content control properties");

      assert_eq!(
        sdt_bound_display_text(&properties, "2012-04-26T00:00:00".to_owned()),
        expected
      );
    }
  }

  #[test]
  fn showing_placeholder_uses_common_boolean_semantics() {
    for (attribute, expected) in [
      ("", true),
      (r#" w:val="1""#, true),
      (r#" w:val="true""#, true),
      (r#" w:val="on""#, true),
      (r#" w:val="0""#, false),
      (r#" w:val="false""#, false),
      (r#" w:val="off""#, false),
    ] {
      let properties = w::SdtProperties::from_bytes(
        format!(
          r#"<w:sdtPr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:showingPlcHdr{attribute}/></w:sdtPr>"#
        )
        .as_bytes(),
      )
      .expect("placeholder content control properties");

      assert_eq!(
        sdt_showing_placeholder(&properties),
        expected,
        "{attribute}"
      );
      assert_eq!(
        sdt_form_widget(&properties).is_some(),
        expected,
        "{attribute}"
      );
    }

    assert!(!sdt_showing_placeholder(&w::SdtProperties::default()));
  }

  #[test]
  fn empty_bound_value_keeps_cached_text_only_for_a_showing_placeholder() {
    for (attribute, expected) in [
      ("", None),
      (r#" w:val="true""#, None),
      (r#" w:val="false""#, Some("")),
    ] {
      let properties = w::SdtProperties::from_bytes(
        format!(
          r#"<w:sdtPr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:showingPlcHdr{attribute}/><w:text/></w:sdtPr>"#
        )
        .as_bytes(),
      )
      .expect("bound placeholder content control properties");

      assert_eq!(
        sdt_bound_replacement_text(&properties, String::new()).as_deref(),
        expected,
        "{attribute}"
      );
    }

    assert_eq!(
      sdt_bound_replacement_text(&w::SdtProperties::default(), String::new()).as_deref(),
      Some("")
    );
  }

  #[test]
  fn empty_data_binding_uses_its_named_simple_glossary_placeholder() {
    const CORE_PROPERTIES_ID: &str = "{6C3C8BC8-F283-45AE-878A-BAB7291924A1}";
    let properties = w::SdtProperties::from_bytes(
      format!(
        r#"<w:sdtPr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:placeholder><w:docPart w:val="TitlePlaceholder"/></w:placeholder><w:dataBinding w:xpath="/cp:coreProperties[1]/dc:title[1]" w:storeItemID="{CORE_PROPERTIES_ID}"/><w:text/></w:sdtPr>"#
      )
      .as_bytes(),
    )
    .expect("data-bound content control with a named placeholder");
    let placeholder = w::DocPartBody::from_bytes(
      br#"<w:docPartBody xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:p><w:r><w:t>[Type the document title]</w:t></w:r></w:p></w:docPartBody>"#,
    )
    .expect("simple glossary placeholder");
    let bindings = CustomXmlBindings::from_test_xml(
      Some(CORE_PROPERTIES_ID),
      r#"<cp:coreProperties xmlns:cp="urn:core" xmlns:dc="urn:dc"><dc:title/></cp:coreProperties>"#,
    )
    .with_test_placeholder("TitlePlaceholder", placeholder);

    assert_eq!(
      sdt_bound_replacement(&bindings, &properties).as_deref(),
      Some("[Type the document title]")
    );

    let showing_only = w::SdtProperties::from_bytes(
      br#"<w:sdtPr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:placeholder><w:docPart w:val="TitlePlaceholder"/></w:placeholder><w:showingPlcHdr/><w:text/></w:sdtPr>"#,
    )
    .expect("showing-placeholder content control without a data binding");
    assert_eq!(sdt_bound_replacement(&bindings, &showing_only), None);
  }

  #[test]
  fn nested_pageref_inside_unsupported_field_preserves_outer_result_text() {
    let styles = StylesCatalog::default();
    let images = ImageCatalog::default();
    let hyperlinks = HyperlinkCatalog::default();
    let mut inlines = Vec::new();
    let mut complex_fields = Vec::new();
    let runs = [
      w::Run {
        run_choice: vec![w::RunChoice::FieldChar(Box::new(w::FieldChar {
          field_char_type: w::FieldCharValues::Begin,
          ..Default::default()
        }))],
        ..Default::default()
      },
      w::Run {
        run_choice: vec![w::RunChoice::FieldCode(w::FieldCode(w::TextType {
          xml_content: Some(r#" HYPERLINK \l md_intro "#.into()),
          ..Default::default()
        }))],
        ..Default::default()
      },
      w::Run {
        run_choice: vec![w::RunChoice::FieldChar(Box::new(w::FieldChar {
          field_char_type: w::FieldCharValues::Separate,
          ..Default::default()
        }))],
        ..Default::default()
      },
      w::Run {
        run_choice: vec![w::RunChoice::Text(text("前言"))],
        ..Default::default()
      },
      w::Run {
        run_choice: vec![w::RunChoice::TabChar],
        ..Default::default()
      },
      w::Run {
        run_choice: vec![w::RunChoice::FieldChar(Box::new(w::FieldChar {
          field_char_type: w::FieldCharValues::Begin,
          ..Default::default()
        }))],
        ..Default::default()
      },
      w::Run {
        run_choice: vec![w::RunChoice::FieldCode(w::FieldCode(w::TextType {
          xml_content: Some(r#" PAGEREF md_intro \h "#.into()),
          ..Default::default()
        }))],
        ..Default::default()
      },
      w::Run {
        run_choice: vec![w::RunChoice::FieldChar(Box::new(w::FieldChar {
          field_char_type: w::FieldCharValues::Separate,
          ..Default::default()
        }))],
        ..Default::default()
      },
      w::Run {
        run_choice: vec![w::RunChoice::Text(text("4"))],
        ..Default::default()
      },
      w::Run {
        run_choice: vec![w::RunChoice::FieldChar(Box::new(w::FieldChar {
          field_char_type: w::FieldCharValues::End,
          ..Default::default()
        }))],
        ..Default::default()
      },
      w::Run {
        run_choice: vec![w::RunChoice::FieldChar(Box::new(w::FieldChar {
          field_char_type: w::FieldCharValues::End,
          ..Default::default()
        }))],
        ..Default::default()
      },
    ];

    for run in &runs {
      push_run_or_complex_field(
        run,
        &mut inlines,
        TextStyle::default(),
        RunImportContext {
          styles: &styles,
          images: &images,
          hyperlinks: &hyperlinks,
          suppress_toc_hyperlink_style: false,
        },
        None,
        &mut complex_fields,
      );
    }

    assert_eq!(field_result_text(&inlines).as_deref(), Some("前言\t4"));
    assert!(
      inlines
        .iter()
        .filter_map(|inline| match inline {
          InlineItem::Text(run) => Some(run),
          _ => None,
        })
        .all(|run| run.hyperlink_url.as_deref() == Some("ooxmlsdk-pdf:bookmark:md_intro"))
    );
  }

  #[test]
  fn pgnum_runs_emit_dynamic_page_marker() {
    let mut inlines = Vec::new();
    let run = w::Run {
      run_choice: vec![w::RunChoice::PageNumber],
      ..Default::default()
    };

    push_run(
      &run,
      &mut inlines,
      TextStyle::default(),
      &StylesCatalog::default(),
      &ImageCatalog::default(),
      &HyperlinkCatalog::default(),
      None,
    );

    let InlineItem::Text(run) = &inlines[0] else {
      panic!("expected dynamic page number text");
    };
    assert_eq!(
      run.dynamic_field,
      Some(DynamicFieldKind::Page {
        number_format: FieldNumberFormat::PageStyle,
      })
    );
  }

  #[test]
  fn ruby_runs_import_base_and_phonetic_guide() {
    let mut inlines = Vec::new();
    let ruby = w::Ruby {
      ruby_properties: Box::new(w::RubyProperties {
        ruby_align: w::RubyAlign {
          val: w::RubyAlignValues::DistributeSpace,
        },
        phonetic_guide_text_font_size: w::PhoneticGuideTextFontSize { val: hps(11) },
        phonetic_guide_raise: w::PhoneticGuideRaise { val: 20 },
        ..Default::default()
      }),
      ruby_content: w::RubyContent {
        ruby_content_choice: vec![w::RubyContentChoice::WRun(Box::new(w::Run {
          run_choice: vec![w::RunChoice::Text(text("かん"))],
          ..Default::default()
        }))],
      },
      ruby_base: w::RubyBase {
        ruby_base_choice: vec![w::RubyBaseChoice::WRun(Box::new(w::Run {
          run_choice: vec![w::RunChoice::Text(text("漢"))],
          ..Default::default()
        }))],
      },
    };
    let run = w::Run {
      run_choice: vec![
        w::RunChoice::Text(text("Before ")),
        w::RunChoice::Ruby(Box::new(ruby)),
        w::RunChoice::Text(text(" after")),
      ],
      ..Default::default()
    };

    push_run(
      &run,
      &mut inlines,
      TextStyle::default(),
      &StylesCatalog::default(),
      &ImageCatalog::default(),
      &HyperlinkCatalog::default(),
      None,
    );

    assert_eq!(inline_text(&inlines), "Before 漢 after");
    let InlineItem::Ruby(ruby) = &inlines[1] else {
      panic!("expected compound ruby inline");
    };
    assert_eq!(ruby.base[0].text, "漢");
    assert_eq!(ruby.guide[0].text, "かん");
    assert_eq!(ruby.alignment, RubyAlignment::DistributeSpace);
    assert_eq!(ruby.raise_pt, 10.0);
    assert_eq!(ruby.guide[0].style.font_size_pt, 5.5);
  }

  #[test]
  fn undeclared_vml_picture_frame_emits_only_the_image() {
    let mut catalog = ImageCatalog::default();
    catalog.by_relationship_id.insert(
      "rId1".into(),
      package::ImageResource {
        data: vec![1, 2, 3].into(),
        content_type: Some("image/png".into()),
      },
    );
    let run = w::Run {
      run_choice: vec![w::RunChoice::Picture(Box::new(w::Picture {
        picture_choice: vec![w::PictureChoice::Shape(Box::new(v::Shape {
          r#type: Some("#_x0000_t75".into()),
          style: Some("width:1in;height:24pt;rotation:90;flip:x y".into()),
          alternate: Some("VML image".into()),
          shape_choice: vec![v::ShapeChoice::ImageData(Box::new(v::ImageData {
            relationship_id: Some("rId1".into()),
            crop_left: Some("10%".into()),
            crop_top: Some("13107f".into()),
            crop_right: Some("0.3".into()),
            crop_bottom: Some("-1".into()),
            ..Default::default()
          }))],
          ..Default::default()
        }))],
        ..Default::default()
      }))],
      ..Default::default()
    };
    let mut inlines = Vec::new();

    push_run(
      &run,
      &mut inlines,
      TextStyle::default(),
      &StylesCatalog::default(),
      &catalog,
      &HyperlinkCatalog::default(),
      None,
    );

    let image = inlines
      .iter()
      .find_map(|item| match item {
        InlineItem::Image(image) => Some(image),
        InlineItem::Text(_)
        | InlineItem::PositionalTab(_)
        | InlineItem::Ruby(_)
        | InlineItem::Shape(_)
        | InlineItem::BookmarkStart(_)
        | InlineItem::FormWidgetStart(_)
        | InlineItem::FormWidgetEnd(_)
        | InlineItem::DrawingGroupStart(_)
        | InlineItem::DrawingGroupEnd
        | InlineItem::LastRenderedPageBreak
        | InlineItem::PageBreak
        | InlineItem::ColumnBreak => None,
      })
      .expect("VML image");
    assert_eq!(image.content_type.as_deref(), Some("image/png"));
    assert_eq!(image.width_pt, 72.0);
    assert_eq!(image.height_pt, 24.0);
    assert!((image.crop.left - 0.1).abs() < 0.001);
    assert!((image.crop.top - 0.2).abs() < 0.001);
    assert!((image.crop.right - 0.3).abs() < 0.001);
    assert_eq!(image.crop.bottom, 0.0);
    assert!((image.rotation_deg + 90.0).abs() < 0.001);
    assert!(image.flip_horizontal);
    assert!(image.flip_vertical);
    assert_eq!(image.alt_text.as_deref(), Some("VML image"));
    assert!(
      inlines
        .iter()
        .all(|item| !matches!(item, InlineItem::Shape(_))),
      "the built-in picture frame must not add a default white fill or black stroke"
    );
  }

  #[test]
  fn undeclared_vml_picture_frame_preserves_explicit_fill() {
    let shape = v::Shape {
      r#type: Some("#_x0000_t75".into()),
      style: Some("width:72pt;height:24pt".into()),
      filled: Some(true.into()),
      fill_color: Some("red".into()),
      stroked: Some(false.into()),
      ..Default::default()
    };

    let inline =
      vml_shape_shape(&shape, &ImageCatalog::default(), &[]).expect("explicit picture-frame fill");

    assert_eq!(inline.fill_color, Some(RgbColor { r: 255, g: 0, b: 0 }));
    assert!(inline.stroke.is_none());
  }

  #[test]
  fn vml_style_rotation_accepts_fixed_degrees() {
    let style = vml_image_style(Some("width:20pt;height:10pt;rotation:5898240fd;flip:x"));

    assert_eq!(style.size_pt, Some((20.0, 10.0)));
    assert!((style.rotation_deg + 90.0).abs() < 0.001);
    assert!(style.flip_horizontal);
    assert!(!style.flip_vertical);
  }

  #[test]
  fn vml_absolute_style_maps_to_floating_placement() {
    let style = vml_image_style(Some(
      "position:absolute;margin-left:12pt;margin-top:18pt;z-index:-2;\
       mso-position-horizontal:left;\
       mso-position-horizontal-relative:page;mso-position-vertical-relative:margin;\
       mso-wrap-style:square;mso-wrap-distance-left:0x0001BE7C",
    ));

    let ImagePlacement::Floating(placement) = style.placement() else {
      panic!("floating placement");
    };
    assert_eq!(
      placement.horizontal_relative_to,
      HorizontalImageReference::Page
    );
    assert_eq!(
      placement.vertical_relative_to,
      VerticalImageReference::Margin
    );
    assert_eq!(
      placement.horizontal_alignment,
      Some(HorizontalImageAlignment::Left)
    );
    assert_eq!(placement.wrap, ImageWrapMode::Square);
    assert!(placement.behind_text);
    assert!((placement.horizontal_offset_pt - 12.0).abs() < 0.001);
    assert!((placement.vertical_offset_pt - 18.0).abs() < 0.001);
    assert!((placement.margin_left_pt - 9.0).abs() < 0.001);
  }

  #[test]
  fn vml_negative_z_index_without_wrap_style_does_not_exclude_body_text() {
    let style = vml_image_style(Some(
      "position:absolute;margin-left:71.75pt;margin-top:13.45pt;\
       width:124.8pt;height:12.95pt;z-index:-251945472",
    ));

    let ImagePlacement::Floating(placement) = style.placement() else {
      panic!("floating placement");
    };
    assert!(placement.behind_text);
    assert_eq!(placement.wrap, ImageWrapMode::Through);
  }

  #[test]
  fn vml_absolute_shape_without_wrap_style_uses_through_default() {
    let style = vml_image_style(Some(
      "position:absolute;margin-left:-36.9pt;margin-top:-25.2pt;\
       width:400pt;height:21.6pt;z-index:251664384",
    ));

    let ImagePlacement::Floating(placement) = style.placement() else {
      panic!("floating placement");
    };
    assert!(!placement.behind_text);
    assert_eq!(placement.wrap, ImageWrapMode::Through);
  }

  #[test]
  fn vml_textboxes_emit_text_content() {
    let run = w::Run {
      run_choice: vec![w::RunChoice::Picture(Box::new(w::Picture {
        picture_choice: vec![w::PictureChoice::Shape(Box::new(v::Shape {
          shape_choice: vec![v::ShapeChoice::TextBox(Box::new(v::TextBox {
            text_box_choice: Some(v::TextBoxChoice::TextBoxContent(w::TextBoxContent {
              text_box_content_choice: vec![w::TextBoxContentChoice::Paragraph(Box::new(
                w::Paragraph {
                  paragraph_choice: vec![w::ParagraphChoice::WRun(Box::new(w::Run {
                    run_choice: vec![w::RunChoice::Text(text("Text inside VML box"))],
                    ..Default::default()
                  }))],
                  ..Default::default()
                },
              ))],
            })),
            ..Default::default()
          }))],
          ..Default::default()
        }))],
        ..Default::default()
      }))],
      ..Default::default()
    };
    let mut inlines = Vec::new();

    push_run(
      &run,
      &mut inlines,
      TextStyle::default(),
      &StylesCatalog::default(),
      &ImageCatalog::default(),
      &HyperlinkCatalog::default(),
      None,
    );

    assert!(inline_text(&inlines).contains("Text inside VML box"));
  }

  #[test]
  fn drawingml_wpg_group_maps_child_coordinates_to_points() {
    let xml = r#"
      <wpg:wgp xmlns:wpg="http://schemas.microsoft.com/office/word/2010/wordprocessingGroup"
               xmlns:wps="http://schemas.microsoft.com/office/word/2010/wordprocessingShape"
               xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
               xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
        <wpg:cNvGrpSpPr/>
        <wpg:grpSpPr>
          <a:xfrm>
            <a:off x="0" y="0"/>
            <a:ext cx="6994525" cy="4023360"/>
            <a:chOff x="613" y="8712"/>
            <a:chExt cx="11015" cy="6336"/>
          </a:xfrm>
        </wpg:grpSpPr>
        <wps:wsp>
          <wps:spPr><a:xfrm><a:off x="4897" y="8714"/><a:ext cx="6731" cy="6334"/></a:xfrm></wps:spPr>
          <wps:txbx><w:txbxContent><w:p><w:r><w:t>Right</w:t></w:r></w:p></w:txbxContent></wps:txbx>
        </wps:wsp>
      </wpg:wgp>
    "#;

    let group = wpg::WordprocessingGroup::from_bytes(xml.as_bytes()).expect("typed WPG group");
    let styles = StylesCatalog::default();
    let images = ImageCatalog::default();
    let hyperlinks = HyperlinkCatalog::default();
    let frames = wordprocessing_group_textbox_frames(
      &group,
      ImagePlacement::Inline,
      DrawingMlGroupTransform::identity(),
      DrawingTextBoxImportContext {
        base_style: TextStyle::default(),
        styles: &styles,
        images: &images,
        hyperlinks: &hyperlinks,
      },
    );

    assert_eq!(frames.len(), 1);
    assert!((frames[0].offset_x_pt - 214.2).abs() < 0.5);
    assert!((frames[0].width_pt - 336.4).abs() < 0.5);
  }

  #[test]
  fn drawingml_locked_canvas_imports_nested_generic_line_shapes() {
    let xml = r#"
      <lc:lockedCanvas xmlns:lc="http://schemas.openxmlformats.org/drawingml/2006/lockedCanvas"
                       xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
        <a:nvGrpSpPr><a:cNvPr id="0" name=""/><a:cNvGrpSpPr/></a:nvGrpSpPr>
        <a:grpSpPr>
          <a:xfrm>
            <a:off x="0" y="0"/><a:ext cx="103188" cy="2459038"/>
            <a:chOff x="381000" y="533400"/><a:chExt cx="103188" cy="2459038"/>
          </a:xfrm>
        </a:grpSpPr>
        <a:grpSp>
          <a:nvGrpSpPr><a:cNvPr id="1" name="Group"/><a:cNvGrpSpPr/></a:nvGrpSpPr>
          <a:grpSpPr>
            <a:xfrm>
              <a:off x="381000" y="533400"/><a:ext cx="103188" cy="2459038"/>
              <a:chOff x="1430" y="1412"/><a:chExt cx="163" cy="3873"/>
            </a:xfrm>
          </a:grpSpPr>
          <a:sp>
            <a:nvSpPr><a:cNvPr id="2" name="Outer line 1"/><a:cNvSpPr/></a:nvSpPr>
            <a:spPr>
              <a:xfrm><a:off x="1592" y="1540"/><a:ext cx="1" cy="3582"/></a:xfrm>
              <a:prstGeom prst="line"><a:avLst/></a:prstGeom>
              <a:noFill/>
              <a:ln w="0"><a:solidFill><a:srgbClr val="000000"/></a:solidFill></a:ln>
            </a:spPr>
          </a:sp>
          <a:sp>
            <a:nvSpPr><a:cNvPr id="3" name="Outer line 2"/><a:cNvSpPr/></a:nvSpPr>
            <a:spPr>
              <a:xfrm flipV="1"><a:off x="1430" y="1540"/><a:ext cx="1" cy="3582"/></a:xfrm>
              <a:prstGeom prst="line"><a:avLst/></a:prstGeom>
              <a:noFill/>
              <a:ln w="0"><a:solidFill><a:srgbClr val="000000"/></a:solidFill></a:ln>
            </a:spPr>
          </a:sp>
          <a:sp>
            <a:nvSpPr><a:cNvPr id="4" name="Line"/><a:cNvSpPr/></a:nvSpPr>
            <a:spPr>
              <a:xfrm flipV="1"><a:off x="1512" y="1800"/><a:ext cx="0" cy="3088"/></a:xfrm>
              <a:prstGeom prst="line"><a:avLst/></a:prstGeom>
              <a:noFill/>
              <a:ln w="9525"><a:solidFill><a:srgbClr val="FF0000"/></a:solidFill></a:ln>
            </a:spPr>
          </a:sp>
        </a:grpSp>
      </lc:lockedCanvas>
    "#;
    let canvas = lc::LockedCanvas::from_bytes(xml.as_bytes()).expect("typed locked canvas");
    let styles = StylesCatalog::default();
    let images = ImageCatalog::default();
    let hyperlinks = HyperlinkCatalog::default();
    let items = drawingml_locked_canvas_shapes(
      &canvas,
      ImagePlacement::Inline,
      DrawingMlGroupTransform::identity().with_fallback_size(Some((8.25, 193.5))),
      DrawingShapeImportContext {
        effect_extent: DrawingEffectExtent::default(),
        styles: &styles,
        images: &images,
        hyperlinks: &hyperlinks,
        smartart_text_colors_by_model_id: None,
      },
    );

    assert_eq!(items.len(), 3);
    let shape = items
      .iter()
      .filter_map(|item| match item {
        InlineItem::Shape(shape) => Some(shape),
        _ => None,
      })
      .find(|shape| {
        shape
          .stroke
          .as_ref()
          .is_some_and(|stroke| stroke.color == RgbColor { r: 255, g: 0, b: 0 })
      })
      .expect("red imported generic line shape");
    assert_eq!(shape.geometry, InlineShapeGeometry::Line);
    assert!(shape.width_pt.abs() < 0.01);
    assert!((shape.height_pt - 166.81).abs() < 0.5);
    assert!((shape.offset_x_pt - 4.15).abs() < 0.2);
    assert!((shape.offset_y_pt - 14.05).abs() < 0.5);
    assert_eq!(
      shape.stroke.as_ref().map(|stroke| stroke.color),
      Some(RgbColor { r: 255, g: 0, b: 0 })
    );
  }

  #[test]
  fn drawingml_wpg_group_effects_wrap_children_once_and_skip_empty_lists() {
    fn group(effect_xml: &str) -> wpg::WordprocessingGroup {
      let xml = format!(
        r#"
        <wpg:wgp xmlns:wpg="http://schemas.microsoft.com/office/word/2010/wordprocessingGroup"
                 xmlns:wps="http://schemas.microsoft.com/office/word/2010/wordprocessingShape"
                 xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
          <wpg:cNvGrpSpPr/>
          <wpg:grpSpPr>
            <a:xfrm>
              <a:off x="0" y="0"/><a:ext cx="127000" cy="127000"/>
              <a:chOff x="0" y="0"/><a:chExt cx="127000" cy="127000"/>
            </a:xfrm>
            {effect_xml}
          </wpg:grpSpPr>
          <wps:wsp>
            <wps:cNvSpPr/>
            <wps:spPr>
              <a:xfrm><a:off x="0" y="0"/><a:ext cx="127000" cy="127000"/></a:xfrm>
              <a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
              <a:solidFill><a:srgbClr val="336699"/></a:solidFill>
            </wps:spPr>
            <wps:bodyPr/>
          </wps:wsp>
        </wpg:wgp>
        "#
      );
      wpg::WordprocessingGroup::from_bytes(xml.as_bytes()).expect("typed WPG group")
    }

    let styles = StylesCatalog::default();
    let images = ImageCatalog::default();
    let hyperlinks = HyperlinkCatalog::default();
    let import = |group: &wpg::WordprocessingGroup| {
      wordprocessing_group_shapes(
        group,
        ImagePlacement::Inline,
        DrawingMlGroupTransform::identity(),
        DrawingShapeImportContext {
          effect_extent: DrawingEffectExtent::default(),
          styles: &styles,
          images: &images,
          hyperlinks: &hyperlinks,
          smartart_text_colors_by_model_id: None,
        },
      )
    };

    let with_glow =
      group(r#"<a:effectLst><a:glow rad="12700"><a:srgbClr val="FF0000"/></a:glow></a:effectLst>"#);
    let items = import(&with_glow);
    assert!(matches!(
      items.as_slice(),
      [
        InlineItem::DrawingGroupStart(_),
        InlineItem::Shape(_),
        InlineItem::DrawingGroupEnd
      ]
    ));
    let InlineItem::DrawingGroupStart(group_effect) = &items[0] else {
      unreachable!("group effects must wrap the child shape");
    };
    let resolved = match &group_effect.effects {
      common::DrawingEffectSource::List {
        resolved: Some(value),
        ..
      } => value,
      _ => panic!("effect list must resolve for WPG group rendering"),
    };
    fn glow_profile(
      container: &common::drawingml_image_effects::ImageEffectContainer,
    ) -> Option<(
      f32,
      common::drawingml_image_effects::GlowSpreadKernel,
      common::drawingml_image_effects::GlowBlurKernel,
    )> {
      container.effects.iter().find_map(|effect| match effect {
        common::drawingml_image_effects::ImageEffect::Glow {
          spread_ratio,
          spread_kernel,
          blur_kernel,
          ..
        } => Some((*spread_ratio, *spread_kernel, *blur_kernel)),
        common::drawingml_image_effects::ImageEffect::AlphaModulate(container)
        | common::drawingml_image_effects::ImageEffect::Container(container)
        | common::drawingml_image_effects::ImageEffect::Blend { container, .. } => {
          glow_profile(container)
        }
        _ => None,
      })
    }
    let (spread_ratio, spread_kernel, blur_kernel) =
      glow_profile(resolved).expect("resolved WPG glow");
    assert!((spread_ratio - 0.4).abs() < f32::EPSILON);
    assert_eq!(
      spread_kernel,
      common::drawingml_image_effects::GlowSpreadKernel::Square
    );
    assert_eq!(
      blur_kernel,
      common::drawingml_image_effects::GlowBlurKernel::Gaussian
    );

    let empty = group("<a:effectLst/>");
    let items = import(&empty);
    assert!(matches!(items.as_slice(), [InlineItem::Shape(_)]));
  }

  #[test]
  fn drawingml_group_rotation_remains_an_oriented_child_frame() {
    let transform = DrawingMlGroupTransform::identity().child(DrawingMlGroupXfrm {
      rotation_deg: 90.0,
      width_pt: 200.0,
      height_pt: 100.0,
      child_width: 200.0,
      child_height: 100.0,
      ..DrawingMlGroupXfrm::default()
    });
    let mapped = transform.map_rect((0.0, 0.0, 200.0, 100.0), (0.0, false, false));

    assert!((mapped.rotation_deg - 90.0).abs() < 0.001);
    assert!((mapped.width_pt - 200.0).abs() < 0.001);
    assert!((mapped.height_pt - 100.0).abs() < 0.001);
    assert!((mapped.x_pt - 0.0).abs() < 0.001);
    assert!((mapped.y_pt - 0.0).abs() < 0.001);
  }

  #[test]
  fn drawingml_textbox_rotation_honors_upright_body_property() {
    let rotated = wps::WordprocessingShape::from_bytes(
      br#"<wps:wsp xmlns:wps="http://schemas.microsoft.com/office/word/2010/wordprocessingShape"><wps:bodyPr rot="5400000"/></wps:wsp>"#,
    )
    .expect("rotated WPS shape");
    assert_eq!(
      wordprocessing_shape_textbox_text_rotation(&rotated),
      Some(90.0)
    );

    let upright = wps::WordprocessingShape::from_bytes(
      br#"<wps:wsp xmlns:wps="http://schemas.microsoft.com/office/word/2010/wordprocessingShape"><wps:bodyPr rot="5400000" upright="1" vert="vert"/></wps:wsp>"#,
    )
    .expect("upright WPS shape");
    assert_eq!(
      wordprocessing_shape_textbox_text_rotation(&upright),
      Some(90.0)
    );
  }

  #[test]
  fn drawingml_textbox_vertical_flow_uses_drawingml_clockwise_angles() {
    let vertical = wps::WordprocessingShape::from_bytes(
      br#"<wps:wsp xmlns:wps="http://schemas.microsoft.com/office/word/2010/wordprocessingShape"><wps:bodyPr vert="vert"/></wps:wsp>"#,
    )
    .expect("vertical WPS shape");
    assert_eq!(
      wordprocessing_shape_textbox_text_rotation(&vertical),
      Some(90.0)
    );

    let vertical_270 = wps::WordprocessingShape::from_bytes(
      br#"<wps:wsp xmlns:wps="http://schemas.microsoft.com/office/word/2010/wordprocessingShape"><wps:bodyPr vert="vert270"/></wps:wsp>"#,
    )
    .expect("vertical-270 WPS shape");
    assert_eq!(
      wordprocessing_shape_textbox_text_rotation(&vertical_270),
      Some(-90.0)
    );
  }

  #[test]
  fn right_arrow_text_rectangle_is_biased_toward_the_shaft() {
    let preset = a::PresetGeometry::from_bytes(
      br#"<a:prstGeom xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" prst="rightArrow"><a:avLst/></a:prstGeom>"#,
    )
    .expect("right-arrow preset geometry");
    let insets = drawingml_preset_text_rectangle_insets(&preset, 141.3, 92.4)
      .expect("right-arrow text rectangle");

    assert!((insets[0] - 0.0).abs() < 0.001);
    assert!((insets[1] - 23.1).abs() < 0.001);
    assert!((insets[2] - 23.1).abs() < 0.001);
    assert!((insets[3] - 23.1).abs() < 0.001);
    assert!(rotations_cancel(-90.0, 90.0));
    assert!(rotations_cancel(270.0, 90.0));
    assert!(!rotations_cancel(90.0, 90.0));
  }

  #[test]
  fn right_triangle_text_rectangle_stays_inside_the_face() {
    let preset = a::PresetGeometry::from_bytes(
      br#"<a:prstGeom xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" prst="rtTriangle"><a:avLst/></a:prstGeom>"#,
    )
    .expect("right-triangle preset geometry");
    let insets = drawingml_preset_text_rectangle_insets(&preset, 240.0, 120.0)
      .expect("right-triangle text rectangle");

    assert_eq!(insets, [20.0, 70.0, 100.0, 10.0]);
  }

  #[test]
  fn drawing_textboxes_extract_cached_text() {
    let xml = r#"<wps:wsp xmlns:wps="http://schemas.microsoft.com/office/word/2010/wordprocessingShape" xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <wps:txbx>
    <w:txbxContent>
      <w:p><w:r><w:t>Modern text box</w:t></w:r></w:p>
      <w:p><w:r><w:t>Second line</w:t></w:r></w:p>
    </w:txbxContent>
  </wps:txbx>
</wps:wsp>"#;

    let shape = wps::WordprocessingShape::from_bytes(xml.as_bytes()).expect("typed WPS shape");
    let content = wordprocessing_shape_textbox_content(&shape).expect("typed textbox content");
    let blocks = textbox_blocks(
      content,
      &StylesCatalog::default(),
      &ImageCatalog::default(),
      &HyperlinkCatalog::default(),
    );
    let text: Vec<_> = blocks
      .iter()
      .filter_map(|block| match block {
        Block::Paragraph(paragraph) => Some(inline_text(&paragraph.inlines)),
        _ => None,
      })
      .collect();

    assert_eq!(text, ["Modern text box", "Second line"]);
  }

  #[test]
  fn drawing_textboxes_import_block_content_controls() {
    let xml = r#"<wps:wsp xmlns:wps="http://schemas.microsoft.com/office/word/2010/wordprocessingShape" xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <wps:txbx>
    <w:txbxContent>
      <w:sdt>
        <w:sdtPr><w:date/></w:sdtPr>
        <w:sdtContent>
          <w:p><w:r><w:t>Click here to enter a date.</w:t></w:r></w:p>
        </w:sdtContent>
      </w:sdt>
    </w:txbxContent>
  </wps:txbx>
</wps:wsp>"#;

    let shape = wps::WordprocessingShape::from_bytes(xml.as_bytes()).expect("typed WPS shape");
    let content = wordprocessing_shape_textbox_content(&shape).expect("typed textbox content");
    let blocks = textbox_blocks_with_base(
      content,
      TextStyle {
        font_size_pt: 17.0,
        ..Default::default()
      },
      &StylesCatalog::default(),
      &ImageCatalog::default(),
      &HyperlinkCatalog::default(),
    );
    let Block::Paragraph(paragraph) = &blocks[0] else {
      panic!("content-control paragraph");
    };

    assert_eq!(
      inline_text(&paragraph.inlines),
      "Click here to enter a date."
    );
    let InlineItem::Text(run) = &paragraph.inlines[0] else {
      panic!("content-control text run");
    };
    assert_eq!(run.style.font_size_pt, 17.0);
  }

  #[test]
  fn style_chain_preserves_explicit_false_run_properties() {
    let mut catalog = StylesCatalog::default();
    catalog.styles.insert(
      "Base".into(),
      StyleEntry {
        style_type: Some(w::StyleValues::Paragraph),
        run_style: TextStyle {
          bold: true,
          italic: true,
          underline: true,
          ..Default::default()
        },
        ..Default::default()
      },
    );
    catalog.styles.insert(
      "Derived".into(),
      StyleEntry {
        style_type: Some(w::StyleValues::Paragraph),
        based_on: Some("Base".into()),
        run_overrides: RunStyleOverrides {
          bold: Some(false),
          underline: Some(false),
          ..Default::default()
        },
        ..Default::default()
      },
    );

    let style = catalog.run_style_with_base(
      Some("Derived"),
      TextStyle::default(),
      RunStyleOverrides::default(),
    );

    assert!(!style.bold);
    assert!(style.italic);
    assert!(!style.underline);
  }

  #[test]
  fn style_chain_preserves_an_explicit_default_sized_font() {
    let mut catalog = StylesCatalog::default();
    catalog.styles.insert(
      "Base".into(),
      StyleEntry {
        style_type: Some(w::StyleValues::Paragraph),
        run_style: TextStyle {
          font_size_pt: 12.0,
          complex_font_size_pt: Some(12.0),
          ..Default::default()
        },
        ..Default::default()
      },
    );
    catalog.styles.insert(
      "Derived".into(),
      StyleEntry {
        style_type: Some(w::StyleValues::Paragraph),
        based_on: Some("Base".into()),
        run_overrides: RunStyleOverrides {
          font_size_pt: Some(11.0),
          complex_font_size_pt: Some(11.0),
          ..Default::default()
        },
        ..Default::default()
      },
    );

    let style = catalog.run_style_with_base(
      Some("Derived"),
      TextStyle::default(),
      RunStyleOverrides::default(),
    );

    assert_eq!(style.font_size_pt, 11.0);
    assert_eq!(style.complex_font_size_pt, Some(11.0));
  }

  #[test]
  fn character_style_superscript_resolves_after_the_effective_base_size() {
    let mut cached_style = TextStyle {
      // Style loading initially sees the document-default 11pt base.
      font_size_pt: 11.0 * WORD_DEFAULT_ESCAPEMENT_HEIGHT_SCALE,
      baseline_shift_pt: 11.0 * LO_SUPERSCRIPT_BASELINE_SHIFT_SCALE,
      ..Default::default()
    };
    let overrides = RunStyleOverrides {
      vertical_alignment: Some(w::VerticalPositionValues::Superscript),
      ..Default::default()
    };
    normalize_relative_run_style(&mut cached_style, overrides);

    let mut catalog = StylesCatalog::default();
    catalog.styles.insert(
      "FootnoteReference".into(),
      StyleEntry {
        style_type: Some(w::StyleValues::Character),
        run_style: cached_style,
        run_overrides: overrides,
        ..Default::default()
      },
    );

    let style = catalog.character_run_style(
      Some("FootnoteReference"),
      TextStyle {
        font_size_pt: 10.0,
        ..Default::default()
      },
    );

    assert_eq!(style.font_size_pt, 6.5);
    assert!((style.baseline_shift_pt - 3.3).abs() < 0.001);
  }

  #[test]
  fn direct_run_preserves_complex_script_formatting_state() {
    let properties = w::RunProperties::from_bytes(
      br#"<w:rPr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:rFonts w:ascii="Latin Face" w:cs="Complex Face"/><w:b/><w:bCs w:val="0"/><w:i w:val="0"/><w:iCs/><w:sz w:val="20"/><w:szCs w:val="40"/><w:cs/><w:rtl/></w:rPr>"#,
    )
    .expect("run properties");

    let style = properties::run_style(
      Some(&properties),
      TextStyle::default(),
      &StylesCatalog::default(),
    );

    assert_eq!(style.font_family.as_deref(), Some("Latin Face"));
    assert_eq!(style.complex_font_family.as_deref(), Some("Complex Face"));
    assert_eq!(style.font_size_pt, 10.0);
    assert_eq!(style.complex_font_size_pt, Some(20.0));
    assert_eq!(style.complex_script, Some(true));
    assert_eq!(style.right_to_left, Some(true));
    assert!(style.bold);
    assert_eq!(style.complex_bold, Some(false));
    assert!(!style.italic);
    assert_eq!(style.complex_italic, Some(true));
  }

  #[test]
  fn automatic_superscript_uses_word_fixed_output_scale() {
    let properties = w::RunProperties::from_bytes(
      br#"<w:rPr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:sz w:val="20"/><w:szCs w:val="40"/><w:vertAlign w:val="superscript"/></w:rPr>"#,
    )
    .expect("run properties");

    let style = properties::run_style(
      Some(&properties),
      TextStyle::default(),
      &StylesCatalog::default(),
    );

    assert_eq!(style.font_size_pt, 6.5);
    assert_eq!(style.complex_font_size_pt, Some(13.0));
    assert!((style.baseline_shift_pt - 3.3).abs() < 0.001);
  }

  #[test]
  fn paragraph_mark_preserves_explicit_non_complex_override() {
    let paragraph = w::Paragraph::from_bytes(
      br#"<w:p xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:pPr><w:rPr><w:sz w:val="20"/><w:szCs w:val="40"/><w:cs w:val="0"/><w:rtl w:val="0"/></w:rPr></w:pPr></w:p>"#,
    )
    .expect("paragraph");
    let model = paragraph_model(
      &paragraph,
      &StylesCatalog::default(),
      &mut NumberingCatalog::default(),
      &ImageCatalog::default(),
      &HyperlinkCatalog::default(),
      &CustomXmlBindings::default(),
      &mut FormWidgetIdAllocator::default(),
    );

    assert_eq!(model.base_style.font_size_pt, 10.0);
    assert_eq!(model.base_style.complex_font_size_pt, Some(20.0));
    assert_eq!(model.base_style.complex_script, Some(false));
    assert_eq!(model.base_style.right_to_left, Some(false));
  }

  #[test]
  fn run_style_imports_the_wordprocessingml_kerning_threshold() {
    let properties = w::RunPropertiesBaseStyle {
      kern: Some(w::Kern { val: 24 }),
      ..Default::default()
    };
    let mut style = TextStyle::default();

    properties::merge_run_style(
      &mut style,
      Some(RunProps::BaseStyle(&properties)),
      &ThemeFonts::default(),
      &ThemeColors::default(),
    );

    assert_eq!(style.kerning_minimum_size_pt, Some(12.0));
  }

  #[test]
  fn run_style_imports_the_wordprocessingml_ligature_categories() {
    let properties = w::RunPropertiesBaseStyle {
      ligatures: Some(w14::Ligatures {
        val: w14::LigaturesValues::StandardContextualHistorical,
      }),
      ..Default::default()
    };
    let mut style = TextStyle::default();

    properties::merge_run_style(
      &mut style,
      Some(RunProps::BaseStyle(&properties)),
      &ThemeFonts::default(),
      &ThemeColors::default(),
    );

    assert_eq!(
      style.ligatures,
      Some(common::OpenTypeLigatures {
        standard: true,
        contextual: true,
        historical: true,
        discretionary: false,
      })
    );
  }

  #[test]
  fn run_style_imports_the_wordprocessingml_baseline_position() {
    let properties = w::RunPropertiesBaseStyle {
      position: Some(w::Position {
        val: ooxmlsdk::simple_type::SignedHpsMeasureValue::HalfPoints(-12),
      }),
      ..Default::default()
    };
    let mut style = TextStyle::default();

    properties::merge_run_style(
      &mut style,
      Some(RunProps::BaseStyle(&properties)),
      &ThemeFonts::default(),
      &ThemeColors::default(),
    );

    assert_eq!(style.baseline_shift_pt, -6.0);
    assert_eq!(style.font_size_pt, TextStyle::default().font_size_pt);
  }

  #[test]
  fn run_style_imports_wordprocessingml_character_scale() {
    let properties = w::RunPropertiesBaseStyle {
      character_scale: Some(w::CharacterScale { val: Some(33) }),
      ..Default::default()
    };
    let mut style = TextStyle::default();

    properties::merge_run_style(
      &mut style,
      Some(RunProps::BaseStyle(&properties)),
      &ThemeFonts::default(),
      &ThemeColors::default(),
    );

    assert_eq!(style.horizontal_scale, Some(0.33));
  }

  #[test]
  fn run_style_resets_out_of_range_wordprocessingml_character_scale() {
    for percentage in [0, 601] {
      let properties = w::RunPropertiesBaseStyle {
        character_scale: Some(w::CharacterScale {
          val: Some(percentage),
        }),
        ..Default::default()
      };
      let mut style = TextStyle::default();

      properties::merge_run_style(
        &mut style,
        Some(RunProps::BaseStyle(&properties)),
        &ThemeFonts::default(),
        &ThemeColors::default(),
      );

      assert_eq!(style.horizontal_scale, Some(1.0));
    }
  }

  #[test]
  fn body_sections_split_paragraph_and_body_section_properties() {
    let body = w::Body {
      body_choice: vec![
        w::BodyChoice::Paragraph(Box::new(paragraph())),
        w::BodyChoice::Paragraph(Box::new(paragraph_with_section(section(
          12240,
          15840,
          w::PageOrientationValues::Portrait,
          None,
        )))),
        w::BodyChoice::Paragraph(Box::new(paragraph())),
      ],
      section_properties: Some(Box::new(section(
        15840,
        12240,
        w::PageOrientationValues::Landscape,
        Some(w::SectionMarkValues::Continuous),
      ))),
    };
    let mut numbering = NumberingCatalog::default();

    let sections = body_sections(
      &body,
      BodySectionEnv {
        styles: &StylesCatalog::default(),
        numbering: &mut numbering,
        images: &ImageCatalog::default(),
        alt_chunks: &AltChunkCatalog::default(),
        hyperlinks: &HyperlinkCatalog::default(),
        custom_xml_bindings: &CustomXmlBindings::default(),
        form_widget_ids: &mut FormWidgetIdAllocator::default(),
        no_column_balance: false,
      },
    );

    assert_eq!(sections.len(), 2);
    assert_eq!(sections[0].blocks.len(), 1);
    assert_eq!(sections[0].break_kind, SectionBreakKind::NextPage);
    assert_eq!(sections[0].page.width_pt, 612.0);
    assert_eq!(sections[0].page.height_pt, 792.0);
    assert_eq!(sections[1].blocks.len(), 1);
    assert_eq!(sections[1].break_kind, SectionBreakKind::NextPage);
    assert_eq!(sections[1].page.width_pt, 792.0);
    assert_eq!(sections[1].page.height_pt, 612.0);
  }

  #[test]
  fn empty_numbered_section_paragraph_does_not_consume_a_list_number() {
    let section_paragraph = w::Paragraph::from_bytes(
      br#"<w:p xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:pPr><w:numPr><w:ilvl w:val="0"/><w:numId w:val="1"/></w:numPr><w:sectPr/></w:pPr></w:p>"#,
    )
    .expect("numbered section paragraph");
    let visible_paragraph = w::Paragraph::from_bytes(
      br#"<w:p xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:pPr><w:numPr><w:ilvl w:val="0"/><w:numId w:val="1"/></w:numPr></w:pPr><w:r><w:t>foo</w:t></w:r></w:p>"#,
    )
    .expect("visible numbered paragraph");
    let level = w::Level::from_bytes(
      br#"<w:lvl xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" w:ilvl="0"><w:start w:val="1"/><w:numFmt w:val="decimal"/><w:lvlText w:val="%1."/></w:lvl>"#,
    )
    .expect("numbering level");
    let mut numbering = NumberingCatalog {
      abstract_nums: HashMap::from([(
        1,
        AbstractNumbering {
          levels: HashMap::from([(0, numbering_level_model(&level, ImportSettings::default()))]),
          ..Default::default()
        },
      )]),
      nums: HashMap::from([(
        1,
        NumberingInstance {
          abstract_num_id: 1,
          overrides: HashMap::new(),
        },
      )]),
      ..Default::default()
    };
    let body = w::Body {
      body_choice: vec![
        w::BodyChoice::Paragraph(Box::new(section_paragraph)),
        w::BodyChoice::Paragraph(Box::new(visible_paragraph)),
      ],
      section_properties: Some(Box::new(w::SectionProperties::default())),
    };

    let sections = body_sections(
      &body,
      BodySectionEnv {
        styles: &StylesCatalog::default(),
        numbering: &mut numbering,
        images: &ImageCatalog::default(),
        alt_chunks: &AltChunkCatalog::default(),
        hyperlinks: &HyperlinkCatalog::default(),
        custom_xml_bindings: &CustomXmlBindings::default(),
        form_widget_ids: &mut FormWidgetIdAllocator::default(),
        no_column_balance: false,
      },
    );

    assert!(sections[0].blocks.is_empty());
    let Block::Paragraph(paragraph) = &sections[1].blocks[0] else {
      panic!("second section should start with a paragraph");
    };
    assert_eq!(paragraph.list_label.as_deref(), Some("1.\t"));
  }

  #[test]
  fn moved_from_numbered_paragraph_is_absent_from_the_current_story() {
    let visible = |text: &str| {
      w::Paragraph::from_bytes(
        format!(
          r#"<w:p xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:pPr><w:numPr><w:ilvl w:val="0"/><w:numId w:val="1"/></w:numPr></w:pPr><w:r><w:t>{text}</w:t></w:r></w:p>"#
        )
        .as_bytes(),
      )
      .expect("visible numbered paragraph")
    };
    let moved_from = w::Paragraph::from_bytes(
      br#"<w:p xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:pPr><w:numPr><w:ilvl w:val="0"/><w:numId w:val="1"/></w:numPr><w:rPr><w:moveFrom w:id="1" w:author="Author" w:date="2022-01-01T00:00:00Z"/></w:rPr></w:pPr><w:moveFrom w:id="2" w:author="Author" w:date="2022-01-01T00:00:00Z"><w:r><w:t>deleted</w:t></w:r></w:moveFrom></w:p>"#,
    )
    .expect("moved-from numbered paragraph");
    let level = w::Level::from_bytes(
      br#"<w:lvl xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" w:ilvl="0"><w:start w:val="1"/><w:numFmt w:val="decimal"/><w:lvlText w:val="%1."/></w:lvl>"#,
    )
    .expect("numbering level");
    let mut numbering = NumberingCatalog {
      abstract_nums: HashMap::from([(
        1,
        AbstractNumbering {
          levels: HashMap::from([(0, numbering_level_model(&level, ImportSettings::default()))]),
          ..Default::default()
        },
      )]),
      nums: HashMap::from([(
        1,
        NumberingInstance {
          abstract_num_id: 1,
          overrides: HashMap::new(),
        },
      )]),
      ..Default::default()
    };
    let body = w::Body {
      body_choice: vec![
        w::BodyChoice::Paragraph(Box::new(visible("first"))),
        w::BodyChoice::Paragraph(Box::new(moved_from)),
        w::BodyChoice::Paragraph(Box::new(visible("second"))),
      ],
      section_properties: Some(Box::new(w::SectionProperties::default())),
    };

    let sections = body_sections(
      &body,
      BodySectionEnv {
        styles: &StylesCatalog::default(),
        numbering: &mut numbering,
        images: &ImageCatalog::default(),
        alt_chunks: &AltChunkCatalog::default(),
        hyperlinks: &HyperlinkCatalog::default(),
        custom_xml_bindings: &CustomXmlBindings::default(),
        form_widget_ids: &mut FormWidgetIdAllocator::default(),
        no_column_balance: false,
      },
    );

    assert_eq!(sections[0].blocks.len(), 2);
    let labels = sections[0]
      .blocks
      .iter()
      .map(|block| match block {
        Block::Paragraph(paragraph) => paragraph.list_label.as_deref(),
        _ => None,
      })
      .collect::<Vec<_>>();
    assert_eq!(labels, [Some("1.\t"), Some("2.\t")]);
  }

  #[test]
  fn empty_style_numbered_continuous_section_keeps_flow_height_without_consuming_a_number() {
    let section_paragraph = w::Paragraph::from_bytes(
      br#"<w:p xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:pPr><w:pStyle w:val="NumberedHeading"/><w:sectPr><w:type w:val="continuous"/></w:sectPr></w:pPr></w:p>"#,
    )
    .expect("style-numbered section paragraph");
    let visible_paragraph = w::Paragraph::from_bytes(
      br#"<w:p xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:pPr><w:pStyle w:val="NumberedHeading"/></w:pPr><w:r><w:t>foo</w:t></w:r></w:p>"#,
    )
    .expect("visible style-numbered paragraph");
    let style_numbering = w::NumberingProperties::from_bytes(
      br#"<w:numPr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:ilvl w:val="0"/><w:numId w:val="1"/></w:numPr>"#,
    )
    .expect("style numbering");
    let styles = StylesCatalog {
      styles: HashMap::from([(
        "NumberedHeading".to_string(),
        StyleEntry {
          paragraph_numbering: Some(Box::new(style_numbering)),
          ..Default::default()
        },
      )]),
      ..Default::default()
    };
    let level = w::Level::from_bytes(
      br#"<w:lvl xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" w:ilvl="0"><w:start w:val="1"/><w:numFmt w:val="decimal"/><w:lvlText w:val="%1."/></w:lvl>"#,
    )
    .expect("numbering level");
    let mut numbering = NumberingCatalog {
      abstract_nums: HashMap::from([(
        1,
        AbstractNumbering {
          levels: HashMap::from([(0, numbering_level_model(&level, ImportSettings::default()))]),
          ..Default::default()
        },
      )]),
      nums: HashMap::from([(
        1,
        NumberingInstance {
          abstract_num_id: 1,
          overrides: HashMap::new(),
        },
      )]),
      ..Default::default()
    };
    let body = w::Body {
      body_choice: vec![
        w::BodyChoice::Paragraph(Box::new(section_paragraph)),
        w::BodyChoice::Paragraph(Box::new(visible_paragraph)),
      ],
      section_properties: Some(Box::new(w::SectionProperties::default())),
    };

    let sections = body_sections(
      &body,
      BodySectionEnv {
        styles: &styles,
        numbering: &mut numbering,
        images: &ImageCatalog::default(),
        alt_chunks: &AltChunkCatalog::default(),
        hyperlinks: &HyperlinkCatalog::default(),
        custom_xml_bindings: &CustomXmlBindings::default(),
        form_widget_ids: &mut FormWidgetIdAllocator::default(),
        no_column_balance: false,
      },
    );

    let [Block::Paragraph(section_carrier)] = sections[0].blocks.as_slice() else {
      panic!("continuous section should retain one flow paragraph");
    };
    assert!(section_carrier.inlines.is_empty());
    assert!(section_carrier.list_label.is_none());
    let Block::Paragraph(paragraph) = &sections[1].blocks[0] else {
      panic!("second section should start with a paragraph");
    };
    assert_eq!(paragraph.list_label.as_deref(), Some("1.\t"));
  }

  #[test]
  fn empty_style_numbered_page_section_is_only_suppressed_by_a_direct_override() {
    let inherited = w::Paragraph::from_bytes(
      br#"<w:p xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:pPr><w:pStyle w:val="NumberedHeading"/><w:sectPr/></w:pPr></w:p>"#,
    )
    .expect("inherited style-numbered section paragraph");
    let directly_indented = w::Paragraph::from_bytes(
      br#"<w:p xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:pPr><w:pStyle w:val="NumberedHeading"/><w:ind w:left="0" w:firstLine="0"/><w:sectPr/></w:pPr></w:p>"#,
    )
    .expect("directly indented style-numbered section paragraph");

    assert!(!empty_section_carrier_suppresses_numbering(
      &inherited,
      SectionBreakKind::NextPage
    ));
    assert!(empty_section_carrier_suppresses_numbering(
      &directly_indented,
      SectionBreakKind::NextPage
    ));
    assert!(empty_section_carrier_suppresses_numbering(
      &inherited,
      SectionBreakKind::Continuous
    ));
  }

  #[test]
  fn page_setup_preserves_custom_twip_dimensions() {
    let setup = page_setup(&section(
      5000,
      8000,
      w::PageOrientationValues::Portrait,
      None,
    ));

    assert_eq!(setup.width_pt, 250.0);
    assert_eq!(setup.height_pt, 400.0);
  }

  #[test]
  fn document_grid_character_spacing_decodes_signed_fixed_point() {
    assert_eq!(doc_grid_character_spacing_points(49_152), Some(12.0));
    assert!((doc_grid_character_spacing_points(2_048).unwrap() - 0.5).abs() < 0.001);
    assert!((doc_grid_character_spacing_points(-2_048).unwrap() + 0.5).abs() < 0.001);
  }

  #[test]
  fn word_page_size_is_limited_to_twenty_two_inches() {
    let setup = page_setup(&section(
      65_534,
      65_534,
      w::PageOrientationValues::Portrait,
      None,
    ));

    assert_eq!(setup.width_pt, 1_584.0);
    assert_eq!(setup.height_pt, 1_584.0);
  }

  #[test]
  fn default_page_setup_uses_reference_a4_paper() {
    let setup = PageSetup::default();

    assert!((setup.width_pt - units::millimeters_to_points(210.0)).abs() < 0.001);
    assert!((setup.height_pt - units::millimeters_to_points(297.0)).abs() < 0.001);
  }

  #[test]
  fn word_section_without_page_size_uses_letter_paper() {
    let setup = page_setup(&w::SectionProperties::default());

    assert_eq!(setup.width_pt, 612.0);
    assert_eq!(setup.height_pt, 792.0);
    assert_eq!(setup.margin_left_pt, 90.0);
    assert_eq!(setup.margin_right_pt, 90.0);
    assert_eq!(setup.margin_top_pt, 72.0);
    assert_eq!(setup.margin_bottom_pt, 72.0);

    let document_default = default_section(Vec::new()).page;
    assert!((document_default.width_pt - units::millimeters_to_points(210.0)).abs() < 0.001);
    assert_eq!(document_default.margin_left_pt, 90.0);
    assert_eq!(document_default.margin_right_pt, 90.0);
  }

  #[test]
  fn continuous_section_keeps_continuous_when_orientation_matches() {
    let previous = section(
      12240,
      15840,
      w::PageOrientationValues::Portrait,
      Some(w::SectionMarkValues::NextPage),
    );
    let current = section(
      12240,
      15840,
      w::PageOrientationValues::Portrait,
      Some(w::SectionMarkValues::Continuous),
    );

    assert_eq!(
      normalized_section_break(Some(&current), Some(&previous)),
      SectionBreakKind::Continuous
    );
  }

  #[test]
  fn next_column_section_normalizes_to_next_page_without_matching_columns() {
    let previous = section_with_columns(w::SectionMarkValues::NextPage, 2);
    let current = section_with_columns(w::SectionMarkValues::NextColumn, 1);

    assert_eq!(
      normalized_section_break(Some(&current), Some(&previous)),
      SectionBreakKind::NextPage
    );
  }

  #[test]
  fn next_column_section_uses_explicit_column_list_count() {
    let previous = explicit_columns_section(w::SectionMarkValues::NextPage);
    let current = explicit_columns_section(w::SectionMarkValues::NextColumn);

    assert_eq!(
      normalized_section_break(Some(&current), Some(&previous)),
      SectionBreakKind::NextColumn
    );
  }

  fn paragraph() -> w::Paragraph {
    w::Paragraph::default()
  }

  fn paragraph_with_section(section_properties: w::SectionProperties) -> w::Paragraph {
    w::Paragraph {
      paragraph_properties: Some(Box::new(w::ParagraphProperties {
        section_properties: Some(Box::new(section_properties)),
        ..Default::default()
      })),
      ..Default::default()
    }
  }

  fn section(
    width: u32,
    height: u32,
    orient: w::PageOrientationValues,
    break_type: Option<w::SectionMarkValues>,
  ) -> w::SectionProperties {
    w::SectionProperties {
      section_type: break_type.map(|val| w::SectionType { val }),
      page_size: Some(w::PageSize {
        width: Some(twips(width)),
        height: Some(twips(height)),
        orient: Some(orient),
        ..Default::default()
      }),
      ..Default::default()
    }
  }

  fn section_with_columns(
    break_type: w::SectionMarkValues,
    column_count: i16,
  ) -> w::SectionProperties {
    w::SectionProperties {
      section_type: Some(w::SectionType { val: break_type }),
      columns: Some(w::Columns {
        column_count: Some(column_count),
        ..Default::default()
      }),
      ..Default::default()
    }
  }

  fn explicit_columns_section(break_type: w::SectionMarkValues) -> w::SectionProperties {
    w::SectionProperties {
      section_type: Some(w::SectionType { val: break_type }),
      columns: Some(w::Columns {
        equal_width: Some(false.into()),
        column: vec![
          w::Column {
            width: Some(signed_twips(1440)),
            space: Some(signed_twips(720)),
          },
          w::Column {
            width: Some(signed_twips(2880)),
            ..Default::default()
          },
        ],
        ..Default::default()
      }),
      ..Default::default()
    }
  }

  fn inline_text(inlines: &[InlineItem]) -> String {
    let mut text = String::new();
    for item in inlines {
      match item {
        InlineItem::Text(run) => text.push_str(&run.text),
        InlineItem::PositionalTab(_) => text.push('\t'),
        InlineItem::Ruby(ruby) => {
          for run in &ruby.base {
            text.push_str(&run.text);
          }
        }
        InlineItem::Image(_)
        | InlineItem::Shape(_)
        | InlineItem::BookmarkStart(_)
        | InlineItem::FormWidgetStart(_)
        | InlineItem::FormWidgetEnd(_)
        | InlineItem::DrawingGroupStart(_)
        | InlineItem::DrawingGroupEnd
        | InlineItem::LastRenderedPageBreak
        | InlineItem::PageBreak
        | InlineItem::ColumnBreak => {}
      }
    }
    text
  }

  #[test]
  fn out_of_place_paragraph_break_is_imported_as_page_break() {
    let paragraph = w::Paragraph::from_bytes(
      br#"<w:p xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:br w:type="page"/></w:p>"#,
    )
    .expect("paragraph with recovered break");
    let styles = StylesCatalog::default();
    let mut form_widget_ids = FormWidgetIdAllocator::default();

    let inlines = paragraph_inlines(
      &paragraph,
      TextStyle::default(),
      &styles,
      &ImageCatalog::default(),
      &HyperlinkCatalog::default(),
      &CustomXmlBindings::default(),
      &mut form_widget_ids,
    );

    assert!(matches!(inlines.as_slice(), [InlineItem::PageBreak]));
  }

  #[test]
  fn table_cell_preserves_out_of_place_breaks_before_paragraph() {
    let table = w::Table::from_bytes(
      br#"<w:tbl xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:tr><w:tc><w:br w:type="textWrapping"/><w:br w:type="textWrapping"/><w:p><w:br w:type="textWrapping"/><w:r><w:t>cell text</w:t></w:r></w:p></w:tc></w:tr></w:tbl>"#,
    )
    .expect("table with recovered cell breaks");
    let row = table
      .table_choice2
      .iter()
      .find_map(|choice| match choice {
        w::TableChoice2::TableRow(row) => Some(row.as_ref()),
        _ => None,
      })
      .expect("table row");
    let cell = table_row_cells(row)
      .into_iter()
      .next()
      .expect("table cell")
      .cell;

    assert_eq!(
      cell
        .table_cell_choice
        .iter()
        .filter(|choice| matches!(choice, w::TableCellChoice::Break(_)))
        .count(),
      2
    );
    assert!(
      cell
        .table_cell_choice
        .iter()
        .any(|choice| matches!(choice, w::TableCellChoice::Paragraph(_)))
    );

    let styles = StylesCatalog {
      simplified_chinese_ui: true,
      ..Default::default()
    };
    let mut numbering = NumberingCatalog::default();
    let images = ImageCatalog::default();
    let hyperlinks = HyperlinkCatalog::default();
    let bindings = CustomXmlBindings::default();
    let mut form_widget_ids = FormWidgetIdAllocator::default();
    let model = table_model(
      &table,
      &mut TableModelEnv {
        styles: &styles,
        numbering: &mut numbering,
        images: &images,
        hyperlinks: &hyperlinks,
        custom_xml_bindings: &bindings,
        form_widget_ids: &mut form_widget_ids,
      },
      TableModelContext {
        nested_table_level: 1,
        in_header_footer: false,
      },
    );
    let rendered_text = model.rows[0].cells[0]
      .blocks
      .iter()
      .filter_map(|block| match block {
        Block::Paragraph(paragraph) => Some(inline_text(&paragraph.inlines)),
        _ => None,
      })
      .collect::<String>();

    assert_eq!(rendered_text, "\n\n\ncell text");
    let paragraph = model.rows[0].cells[0]
      .blocks
      .iter()
      .find_map(|block| match block {
        Block::Paragraph(paragraph) => Some(paragraph),
        _ => None,
      })
      .expect("cell paragraph");
    assert_eq!(paragraph.format.line_height_pt, Some(360.0 / 240.0));
  }

  #[test]
  fn table_cell_moves_out_of_place_paragraph_content_into_nested_table() {
    let table = w::Table::from_bytes(
      br#"<w:tbl xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:tr><w:tc><w:p><w:r><w:t>[outer:A2]</w:t><w:br/></w:r><w:tbl><w:tr><w:tc><w:p><w:r><w:t>[inner:A1]</w:t></w:r></w:p></w:tc></w:tr></w:tbl></w:p></w:tc></w:tr></w:tbl>"#,
    )
    .expect("table with out-of-place nested table");
    let styles = StylesCatalog::default();
    let mut numbering = NumberingCatalog::default();
    let images = ImageCatalog::default();
    let hyperlinks = HyperlinkCatalog::default();
    let bindings = CustomXmlBindings::default();
    let mut form_widget_ids = FormWidgetIdAllocator::default();
    let model = table_model(
      &table,
      &mut TableModelEnv {
        styles: &styles,
        numbering: &mut numbering,
        images: &images,
        hyperlinks: &hyperlinks,
        custom_xml_bindings: &bindings,
        form_widget_ids: &mut form_widget_ids,
      },
      TableModelContext {
        nested_table_level: 1,
        in_header_footer: false,
      },
    );

    let nested = match model.rows[0].cells[0].blocks.as_slice() {
      [Block::Table(table)] => table,
      blocks => panic!("expected repaired nested table, got {blocks:?}"),
    };
    let paragraph = nested.rows[0].cells[0]
      .blocks
      .iter()
      .find_map(|block| match block {
        Block::Paragraph(paragraph) => Some(paragraph),
        _ => None,
      })
      .expect("nested first paragraph");
    assert_eq!(inline_text(&paragraph.inlines), "[outer:A2]\n[inner:A1]");
  }

  #[test]
  fn body_break_is_postponed_into_first_populated_table_cell() {
    let document = w::Document::from_bytes(
      br#"<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:body><w:br w:type="textWrapping"/><w:tbl><w:tr><w:tc><w:br w:type="textWrapping"/><w:br w:type="textWrapping"/><w:p><w:br w:type="textWrapping"/><w:r><w:t>cell text</w:t></w:r></w:p></w:tc></w:tr></w:tbl><w:br w:type="page"/></w:body></w:document>"#,
    )
    .expect("document with recovered body breaks");
    let styles = StylesCatalog::default();
    let mut numbering = NumberingCatalog::default();
    let images = ImageCatalog::default();
    let alt_chunks = AltChunkCatalog::default();
    let hyperlinks = HyperlinkCatalog::default();
    let bindings = CustomXmlBindings::default();
    let mut form_widget_ids = FormWidgetIdAllocator::default();
    let sections = body_sections(
      document.body.as_deref().expect("document body"),
      BodySectionEnv {
        styles: &styles,
        numbering: &mut numbering,
        images: &images,
        alt_chunks: &alt_chunks,
        hyperlinks: &hyperlinks,
        custom_xml_bindings: &bindings,
        form_widget_ids: &mut form_widget_ids,
        no_column_balance: false,
      },
    );
    let table = sections[0]
      .blocks
      .iter()
      .find_map(|block| match block {
        Block::Table(table) => Some(table),
        _ => None,
      })
      .expect("imported table block");

    assert_eq!(table.rows.len(), 1);
    assert_eq!(table.rows[0].cells.len(), 1);
    let rendered_text = table.rows[0].cells[0]
      .blocks
      .iter()
      .filter_map(|block| match block {
        Block::Paragraph(paragraph) => Some(inline_text(&paragraph.inlines)),
        _ => None,
      })
      .collect::<String>();
    assert_eq!(rendered_text, "\n\n\n\ncell text");
  }

  #[test]
  fn body_page_break_skips_empty_paragraph_and_prefixes_next_text() {
    let document = w::Document::from_bytes(
      br#"<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:body><w:br w:type="page"/><w:p/><w:p><w:r><w:t>next page</w:t></w:r></w:p></w:body></w:document>"#,
    )
    .expect("document with postponed page break");
    let styles = StylesCatalog::default();
    let mut numbering = NumberingCatalog::default();
    let images = ImageCatalog::default();
    let alt_chunks = AltChunkCatalog::default();
    let hyperlinks = HyperlinkCatalog::default();
    let bindings = CustomXmlBindings::default();
    let mut form_widget_ids = FormWidgetIdAllocator::default();
    let sections = body_sections(
      document.body.as_deref().expect("document body"),
      BodySectionEnv {
        styles: &styles,
        numbering: &mut numbering,
        images: &images,
        alt_chunks: &alt_chunks,
        hyperlinks: &hyperlinks,
        custom_xml_bindings: &bindings,
        form_widget_ids: &mut form_widget_ids,
        no_column_balance: false,
      },
    );
    let paragraphs = sections[0]
      .blocks
      .iter()
      .filter_map(|block| match block {
        Block::Paragraph(paragraph) => Some(paragraph),
        _ => None,
      })
      .collect::<Vec<_>>();

    assert_eq!(paragraphs.len(), 2);
    assert!(paragraphs[0].inlines.is_empty());
    assert!(matches!(
      paragraphs[1].inlines.first(),
      Some(InlineItem::PageBreak)
    ));
    assert_eq!(inline_text(&paragraphs[1].inlines), "next page");
  }

  #[test]
  fn missing_theme_uses_current_office_scheme_colors() {
    let colors = ThemeColors::default();

    assert_eq!(
      colors.resolve_wordprocessing(w::ThemeColorValues::Hyperlink),
      Some(RgbColor {
        r: 0x46,
        g: 0x78,
        b: 0x86,
      })
    );
    assert_eq!(
      colors.resolve_wordprocessing(w::ThemeColorValues::Accent1),
      Some(RgbColor {
        r: 0x15,
        g: 0x60,
        b: 0x82,
      })
    );
  }

  #[test]
  fn wordart_outline_fragment_resolves_expected_color_and_opacity() {
    let fragment = r#"<w14:textOutline xmlns:w14="http://schemas.microsoft.com/office/word/2010/wordml" w14:w="228600" w14:cap="rnd" w14:cmpd="sng" w14:algn="ctr"><w14:solidFill><w14:schemeClr w14:val="accent2"><w14:alpha w14:val="20000"/><w14:lumMod w14:val="75000"/></w14:schemeClr></w14:solidFill><w14:prstDash w14:val="sysDot"/><w14:bevel/></w14:textOutline>"#;
    let outline = w14::TextOutlineEffect::from_bytes(fragment.as_bytes()).unwrap();
    let theme_colors = ThemeColors {
      accent2: Some(RgbColor {
        r: 0xC0,
        g: 0x50,
        b: 0x4D,
      }),
      ..Default::default()
    };

    let resolved = resolve_text_outline(&outline, &theme_colors).unwrap();

    assert_eq!(
      resolved.color,
      RgbColor {
        r: 0x95,
        g: 0x37,
        b: 0x35,
      }
    );
    assert!((resolved.opacity - 0.8).abs() < 0.001);
  }

  #[test]
  fn drawingml_gradient_preserves_percentage_hsl_and_rotation_semantics() {
    let fill = a::GradientFill::from_bytes(
      br#"<a:gradFill xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" rotWithShape="0"><a:gsLst><a:gs pos="0"><a:scrgbClr r="100000" g="0" b="0"><a:alpha val="50000"/></a:scrgbClr></a:gs><a:gs pos="100000"><a:hslClr hue="14400000" sat="100000" lum="50000"/></a:gs></a:gsLst><a:lin ang="0"/></a:gradFill>"#,
    )
    .expect("DrawingML gradient");

    let common::Fill::Gradient(gradient) =
      drawingml_gradient_fill(&fill, &ThemeColors::default()).expect("resolved gradient")
    else {
      panic!("expected gradient fill");
    };

    assert_eq!(gradient.stops.len(), 2);
    assert!(gradient.stops[0].color.r > gradient.stops[0].color.b);
    assert!((120..=135).contains(&gradient.stops[0].color.a));
    assert!(gradient.stops[1].color.b > gradient.stops[1].color.r);
    assert_eq!(gradient.rotate_with_shape, Some(false));
  }

  #[test]
  fn drawingml_shape_tile_fill_is_not_flattened_to_stretch() {
    let fill = a::BlipFill::from_bytes(
      br#"<a:blipFill xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"><a:blip/><a:tile sx="75000" sy="50000" flip="xy" algn="br"/></a:blipFill>"#,
    )
    .expect("DrawingML blip fill");

    let InlineShapeImageFillMode::DrawingMlTile(tile) = drawingml_image_fill_mode(&fill) else {
      panic!("expected DrawingML tile");
    };
    assert_eq!(
      tile.alignment,
      Some(a::RectangleAlignmentValues::BottomRight)
    );
    assert_eq!(tile.flip, Some(a::TileFlipValues::HorizontalAndVertical));
  }

  #[test]
  fn office_treats_diagram_group_fill_as_no_fill() {
    let properties = dsp::ShapeProperties::from_bytes(
      br#"<dsp:spPr xmlns:dsp="http://schemas.microsoft.com/office/drawing/2008/diagram" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"><a:grpFill/></dsp:spPr>"#,
    )
    .expect("diagram shape properties");

    assert!(drawingml_shape_properties_has_no_fill(
      &DrawingMlShapeProperties::Diagram(properties)
    ));
  }

  #[test]
  fn text_effect_overrides_apply_to_style_from_run_properties_fragment() {
    let fragment = r#"<w:rPr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:w14="http://schemas.microsoft.com/office/word/2010/wordml"><w:color w:val="D6E3BC" w:themeColor="accent3" w:themeTint="66"/><w14:textOutline w14:w="228600" w14:cap="rnd" w14:cmpd="sng" w14:algn="ctr"><w14:solidFill><w14:schemeClr w14:val="accent2"><w14:alpha w14:val="20000"/><w14:lumMod w14:val="75000"/></w14:schemeClr></w14:solidFill><w14:prstDash w14:val="sysDot"/><w14:bevel/></w14:textOutline></w:rPr>"#;
    let styles = StylesCatalog {
      theme_colors: ThemeColors {
        accent2: Some(RgbColor {
          r: 0xC0,
          g: 0x50,
          b: 0x4D,
        }),
        accent3: Some(RgbColor {
          r: 0x9B,
          g: 0xBB,
          b: 0x59,
        }),
        ..Default::default()
      },
      ..Default::default()
    };
    let mut style = TextStyle::default();

    let properties = w::RunProperties::from_bytes(fragment.as_bytes()).expect("run properties");
    style = properties::run_style(Some(&properties), style, &styles);

    assert_eq!(
      style.color,
      RgbColor {
        r: 0xD7,
        g: 0xE4,
        b: 0xBD,
      }
    );
    assert_eq!(
      style.outline_color,
      Some(RgbColor {
        r: 0x95,
        g: 0x37,
        b: 0x35,
      })
    );
    assert!((style.outline_opacity - 0.8).abs() < 0.001);
  }
}
