mod chart;
mod custom_xml;
mod drawing;
mod layout;
mod model;
mod package;
mod properties;
mod settings;
mod table;
mod text;

use std::collections::{BTreeMap, HashMap};
use std::io::Cursor;
use std::sync::Arc;

use image::codecs::png::PngEncoder;
use image::{ColorType, ImageEncoder};
use ooxmlsdk::parts::{
  main_document_part::MainDocumentPart, wordprocessing_document::WordprocessingDocument,
};
use ooxmlsdk::schemas::{
  schemas_microsoft_com_office_drawing_2008_diagram as dsp,
  schemas_microsoft_com_office_word_2010_wordml as w14,
  schemas_microsoft_com_office_word_2010_wordprocessing_canvas as wpc,
  schemas_microsoft_com_office_word_2010_wordprocessing_drawing as wp14,
  schemas_microsoft_com_office_word_2010_wordprocessing_group as wpg,
  schemas_microsoft_com_office_word_2010_wordprocessing_shape as wps,
  schemas_microsoft_com_vml as v, schemas_openxmlformats_org_drawingml_2006_chart as c,
  schemas_openxmlformats_org_drawingml_2006_diagram as dgm,
  schemas_openxmlformats_org_drawingml_2006_main as a,
  schemas_openxmlformats_org_drawingml_2006_picture as pic,
  schemas_openxmlformats_org_drawingml_2006_wordprocessing_drawing as wp,
  schemas_openxmlformats_org_markup_compatibility_2006 as mc,
  schemas_openxmlformats_org_wordprocessingml_2006_main as w,
};
use ooxmlsdk::sdk::SdkType;
use ooxmlsdk::simple_type::{
  DrawingmlPercentageValue, MeasurementOrPercentValue, SignedTwipsMeasureValue, TwipsMeasureValue,
};
use ooxmlsdk::units as sdk_units;
use smallvec::SmallVec;

use crate::error::Result;
use crate::options::{LayoutActionOptions, LayoutDiagnosticsOptions, LayoutOptions};
use crate::render::chart as shared_chart;
use crate::render::math as shared_math;
use crate::render::symbol as shared_symbol;
use crate::units;

use chart::supplemental_graphic_blocks;
pub(crate) use custom_xml::CustomXmlBindings;
pub(crate) use model::*;
use package::{HyperlinkCatalog, ImageCatalog};
use settings::{
  compatibility_mode, default_tab_stop_pt, no_column_balance, split_page_break_and_paragraph_mark,
};
use table::{TableConditionalStyleMask, TableLookModel};
use text::{ParagraphImportBase, paragraph_model, paragraph_model_with_base};

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
// sw/source/writerfilter/dmapper/DomainMapper.cxx. DOCX vertAlign maps to
// DFLT_ESC_PROP with DFLT_ESC_SUPER / DFLT_ESC_SUB.
const LO_DEFAULT_ESCAPEMENT_HEIGHT_SCALE: f32 = 0.58;
const LO_SUPERSCRIPT_BASELINE_SHIFT_SCALE: f32 = 0.33;
const LO_SUBSCRIPT_BASELINE_SHIFT_SCALE: f32 = -0.08;
const MIN_ESCAPEMENT_FONT_SIZE_PT: f32 = 1.0;
const MIN_IMPORTED_LINE_HEIGHT_PT: f32 = 0.1;
const TAB_STOP_DEDUP_EPSILON_PT: f32 = 0.1;
const MAX_WORD_TABLE_MARGIN_TWIPS: f32 = 31_680.0;

#[derive(Clone, Copy, Debug, Default)]
struct ImportSettings {
  justify_lines_with_shrinking: bool,
  exchange_left_right: bool,
  use_literal_direction: bool,
}

pub(crate) fn extract(
  package: &mut WordprocessingDocument,
  options: &LayoutOptions,
) -> Result<DocxDocument> {
  let main = package.main_document_part()?;
  let compatibility_mode = compatibility_mode(package, &main);
  let import_settings = ImportSettings {
    justify_lines_with_shrinking: compatibility_mode >= 15,
    ..Default::default()
  };
  let styles = StylesCatalog::load(
    package,
    &main,
    import_settings,
    options.ui_language.as_deref(),
  )?;
  let mut numbering = NumberingCatalog::load(package, &main, import_settings)?;
  let images = ImageCatalog::load(package, &main);
  let hyperlinks = HyperlinkCatalog::load(package, &main);
  let custom_xml_bindings = CustomXmlBindings::load(package, &main);
  let mut form_widget_ids = FormWidgetIdAllocator::default();
  let default_tab_stop_pt = default_tab_stop_pt(package, &main);
  let even_and_odd_headers = even_and_odd_headers(package, &main);
  let no_column_balance = no_column_balance(package, &main);
  let split_page_break_and_paragraph_mark = split_page_break_and_paragraph_mark(package, &main);
  let mirror_margins = mirror_margins(package, &main);
  let document = main.root_element(package)?;
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
          styles: &styles,
          numbering: &mut numbering,
          images: &images,
          hyperlinks: &hyperlinks,
          custom_xml_bindings: &custom_xml_bindings,
          form_widget_ids: &mut form_widget_ids,
          no_column_balance,
        },
      )
    })
    .unwrap_or_else(|| vec![default_section(Vec::new())]);
  let supplemental_graphic_blocks = supplemental_graphic_blocks(package, &main, &styles);
  if let Some(first_section) = sections.first_mut() {
    if let Some(image) = page_background_image {
      first_section
        .blocks
        .insert(0, page_background_image_block(image, first_section.page));
    }
    first_section
      .blocks
      .extend(supplemental_graphic_blocks.iter().cloned());
  }
  for section in &mut sections {
    section.page.background = page_background;
    section.page.mirror_margins = mirror_margins;
  }
  resolve_section_repeating_blocks(
    package,
    &main,
    &styles,
    &custom_xml_bindings,
    &mut sections,
    &mut form_widget_ids,
  );
  let (footnote_labels, endnote_labels) = note_reference_labels(package, &main, &sections);
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
    default_tab_stop_pt,
    compatibility_mode,
    justify_lines_with_shrinking: import_settings.justify_lines_with_shrinking,
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
    endnote_blocks,
    endnotes,
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
      preserve_text_portion: false,
    })],
    footnote_reference_ids: Vec::new(),
    endnote_reference_ids: Vec::new(),
    starts_after_last_rendered_page_break: false,
    base_style: style.clone(),
    #[cfg(test)]
    runs: Vec::new(),
    format: Box::new(ParagraphFormat::default()),
    style_ref_keys: Vec::new(),
    style_ref_text: None,
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
      alt_text: None,
      hyperlink_url: None,
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
        relative_height: 0,
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
    footnote_reference_ids: Vec::new(),
    endnote_reference_ids: Vec::new(),
    starts_after_last_rendered_page_break: false,
    base_style: TextStyle::default(),
    #[cfg(test)]
    runs: Vec::new(),
    format: Box::new(ParagraphFormat::default()),
    style_ref_keys: Vec::new(),
    style_ref_text: None,
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
  let component = i32::from(value) * sdk_units::DRAWINGML_PERCENT_SCALE / 255;
  drawingml_gamma(component, 2.3)
}

fn drawingml_crgb_component_to_rgb(value: i32) -> u8 {
  let component = drawingml_gamma(value, 1.0 / 2.3);
  (component * 255 / sdk_units::DRAWINGML_PERCENT_SCALE).clamp(0, 255) as u8
}

fn drawingml_gamma(value: i32, gamma: f64) -> i32 {
  let scale = f64::from(sdk_units::DRAWINGML_PERCENT_SCALE);
  ((f64::from(value) / scale).powf(gamma) * scale + 0.5) as i32
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
  let BodySectionEnv {
    styles,
    numbering,
    images,
    hyperlinks,
    custom_xml_bindings,
    form_widget_ids,
    no_column_balance,
  } = env;

  for choice in &body.body_choice {
    match choice {
      w::BodyChoice::Paragraph(paragraph) => {
        let mut model = paragraph_model(
          paragraph,
          styles,
          numbering,
          images,
          hyperlinks,
          custom_xml_bindings,
          form_widget_ids,
        );
        model.format.hidden_separator = paragraph_mark_is_hidden(paragraph);
        if paragraph_has_drop_cap_frame(&model) {
          pending_drop_cap_text = paragraph_drop_cap_text(&model);
          continue;
        }
        if let Some(text) = pending_drop_cap_text.take() {
          prepend_drop_cap_text(&mut model, text);
        }
        let section_properties = paragraph
          .paragraph_properties
          .as_deref()
          .and_then(|properties| properties.section_properties.as_deref())
          .cloned();
        if paragraph_is_effectively_empty(&model)
          && section_properties.is_none()
          && current_blocks
            .last()
            .is_some_and(|block| matches!(block, Block::Table(table) if table.placement.is_none()))
        {
          continue;
        }
        if let Some(section_properties) = section_properties {
          if !paragraph_is_effectively_empty(&model) {
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
      w::BodyChoice::Table(table) => current_blocks.push(Block::Table(table_model(
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
      ))),
      w::BodyChoice::SdtBlock(sdt) => {
        current_blocks.extend(sdt_block_blocks(
          sdt,
          styles,
          numbering,
          images,
          hyperlinks,
          custom_xml_bindings,
          form_widget_ids,
        ));
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

fn push_body_paragraph(blocks: &mut Vec<Block>, mut paragraph: Paragraph) {
  if let Some(Block::Paragraph(previous)) = blocks.last_mut()
    && previous.format.hidden_separator
  {
    previous
      .format
      .outline_text_inlines
      .get_or_insert(previous.inlines.len());
    previous.format.hidden_separator = paragraph.format.hidden_separator;
    previous
      .footnote_reference_ids
      .append(&mut paragraph.footnote_reference_ids);
    previous
      .endnote_reference_ids
      .append(&mut paragraph.endnote_reference_ids);
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

fn paragraph_is_effectively_empty(paragraph: &Paragraph) -> bool {
  paragraph.list_label.is_none()
    && paragraph.footnote_reference_ids.is_empty()
    && paragraph.endnote_reference_ids.is_empty()
    && paragraph.inlines.iter().all(|inline| match inline {
      InlineItem::Text(run) => run.text.trim().is_empty(),
      InlineItem::Image(_) | InlineItem::Shape(_) => false,
      InlineItem::BookmarkStart(_) => true,
      InlineItem::FormWidgetStart(_) | InlineItem::FormWidgetEnd(_) => true,
      InlineItem::LastRenderedPageBreak => true,
      InlineItem::PageBreak | InlineItem::ColumnBreak => false,
    })
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
      InlineItem::Image(_)
      | InlineItem::Shape(_)
      | InlineItem::BookmarkStart(_)
      | InlineItem::FormWidgetStart(_)
      | InlineItem::FormWidgetEnd(_)
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
    | w::TextDirectionValues::TopToBottomRightToLeftRotated2010 => Some(-90.0),
    w::TextDirectionValues::BottomToTopLeftToRight
    | w::TextDirectionValues::BottomToTopLeftToRight2010 => Some(90.0),
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

fn sdt_block_blocks(
  sdt: &w::SdtBlock,
  styles: &StylesCatalog,
  numbering: &mut NumberingCatalog,
  images: &ImageCatalog,
  hyperlinks: &HyperlinkCatalog,
  custom_xml_bindings: &CustomXmlBindings,
  form_widget_ids: &mut FormWidgetIdAllocator,
) -> Vec<Block> {
  let Some(content) = sdt.sdt_content_block.as_ref() else {
    return Vec::new();
  };

  content
    .sdt_content_block_choice
    .iter()
    .filter_map(|choice| match choice {
      w::SdtContentBlockChoice::Paragraph(paragraph) => {
        Some(vec![Block::paragraph(paragraph_model(
          paragraph.as_ref(),
          styles,
          numbering,
          images,
          hyperlinks,
          custom_xml_bindings,
          form_widget_ids,
        ))])
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
          in_header_footer: false,
        },
      ))]),
      w::SdtContentBlockChoice::SdtBlock(sdt) => Some(sdt_block_blocks(
        sdt.as_ref(),
        styles,
        numbering,
        images,
        hyperlinks,
        custom_xml_bindings,
        form_widget_ids,
      )),
      _ => None,
    })
    .flatten()
    .collect()
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
        w::HeaderChoice::Paragraph(paragraph) => Some(Block::paragraph(paragraph_model(
          paragraph,
          styles,
          &mut numbering,
          &images,
          &hyperlinks,
          custom_xml_bindings,
          form_widget_ids,
        ))),
        w::HeaderChoice::Table(table) => Some(Block::Table(table_model(
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
        ))),
        _ => None,
      })
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
        w::FooterChoice::Paragraph(paragraph) => Some(Block::paragraph(paragraph_model(
          paragraph,
          styles,
          &mut numbering,
          &images,
          &hyperlinks,
          custom_xml_bindings,
          form_widget_ids,
        ))),
        w::FooterChoice::Table(table) => Some(Block::Table(table_model(
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
        ))),
        _ => None,
      })
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

#[derive(Clone, Copy)]
struct NoteNumberingSpec {
  format: w::NumberFormatValues,
  start: i32,
  restart: w::RestartNumberValues,
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

fn note_reference_labels(
  package: &mut WordprocessingDocument,
  main: &MainDocumentPart,
  sections: &[ImportedSection],
) -> (HashMap<i64, String>, HashMap<i64, String>) {
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

  (
    note_labels_for_sections(sections, NoteKind::Footnote, footnote_default),
    note_labels_for_sections(sections, NoteKind::Endnote, endnote_default),
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
  document_spec: NoteNumberingSpec,
) -> HashMap<i64, String> {
  let mut labels = HashMap::new();
  let mut value = document_spec.start;
  for (section_index, section) in sections.iter().enumerate() {
    let spec = section_note_numbering_spec(section, kind, document_spec);
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
    if footnote.id < 1
      || !matches!(
        footnote.r#type,
        None | Some(w::FootnoteEndnoteValues::Normal)
      )
    {
      continue;
    }
    let mut blocks = Vec::new();
    append_note_blocks(
      &mut blocks,
      NoteLabel::new(
        format!(
          "{} ",
          labels
            .get(&footnote.id)
            .map_or_else(|| footnote.id.to_string(), Clone::clone)
        ),
        Some(note_backlink_url("footnote", footnote.id)),
      ),
      footnote
        .footnote_choice
        .iter()
        .filter_map(|choice| match choice {
          w::FootnoteChoice::Paragraph(paragraph) => Some(paragraph.as_ref()),
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
    if endnote.id < 1
      || !matches!(
        endnote.r#type,
        None | Some(w::FootnoteEndnoteValues::Normal)
      )
    {
      continue;
    }
    let mut blocks = Vec::new();
    append_note_blocks(
      &mut blocks,
      NoteLabel::new(
        format!(
          "{} ",
          labels
            .get(&endnote.id)
            .map_or_else(|| endnote.id.to_string(), Clone::clone)
        ),
        Some(note_backlink_url("endnote", endnote.id)),
      ),
      endnote
        .endnote_choice
        .iter()
        .filter_map(|choice| match choice {
          w::EndnoteChoice::Paragraph(paragraph) => Some(paragraph.as_ref()),
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

fn append_note_blocks<'a>(
  blocks: &mut Vec<Block>,
  label: NoteLabel,
  paragraphs: impl Iterator<Item = &'a w::Paragraph>,
  context: &mut NoteImportContext<'_>,
) {
  let mut is_first_paragraph = true;
  for paragraph in paragraphs {
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

fn table_model(
  table: &w::Table,
  env: &mut TableModelEnv<'_>,
  model_context: TableModelContext,
) -> Table {
  let properties = table.table_properties.as_deref();
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
      table_style: &table_style,
      table_look,
      row_count,
      nested_table_level: model_context.nested_table_level,
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
  let split_allowed = placement.is_some();
  let following_text_flow = placement.is_some()
    && (model_context.nested_table_level >= 2 || model_context.in_header_footer)
    && !(model_context.in_header_footer
      && placement
        .is_some_and(|placement| matches!(placement.vertical_anchor, FrameVerticalAnchor::Page)));
  Table {
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
    placement,
    split_allowed,
    following_text_flow,
    explicit_no_repeat_header,
    starts_after_last_rendered_page_break,
    borders: properties
      .and_then(|properties| properties.table_borders.as_deref())
      .map(|borders| direct_table_borders_model(table_style.table_borders, borders))
      .or(table_style.table_borders),
    cell_spacing_pt: properties
      .and_then(|properties| properties.table_cell_spacing.as_ref())
      .and_then(table_cell_spacing_to_points)
      .or(table_style.cell_spacing_pt)
      .unwrap_or(0.0),
    rows,
  }
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
      InlineItem::Image(_) | InlineItem::Shape(_) => return saw_last_rendered_page_break,
      InlineItem::PageBreak | InlineItem::ColumnBreak => return false,
      InlineItem::Text(_)
      | InlineItem::BookmarkStart(_)
      | InlineItem::FormWidgetStart(_)
      | InlineItem::FormWidgetEnd(_) => {}
    }
  }
  false
}

fn table_position_placement(properties: &w::TablePositionProperties) -> FloatingFramePlacement {
  FloatingFramePlacement {
    horizontal_anchor: frame_horizontal_anchor(properties.horizontal_anchor),
    vertical_anchor: frame_vertical_anchor(properties.vertical_anchor),
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
    margin_left_pt: properties
      .left_from_text
      .as_ref()
      .and_then(twips_measure_to_points)
      .unwrap_or(0.0),
  }
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

fn frame_height_rule(value: Option<w::HeightRuleValues>) -> FrameHeightRule {
  match value.unwrap_or_default() {
    w::HeightRuleValues::Auto => FrameHeightRule::Auto,
    w::HeightRuleValues::AtLeast => FrameHeightRule::AtLeast,
    w::HeightRuleValues::Exact => FrameHeightRule::Exact,
  }
}

fn table_row_model(
  row: &w::TableRow,
  context: &mut TableImportContext<'_>,
  row_index: usize,
) -> TableRow {
  let (grid_before, grid_after) = table_row_grid_properties(row.table_row_properties.as_deref());
  let row_condition = table_row_conditional_style(row.table_row_properties.as_deref())
    .unwrap_or_else(|| {
      TableConditionalStyleMask::from_row_position(context.table_look, row_index, context.row_count)
    });
  let mut row_style = table_row_style_for(
    context.table_style,
    context.table_look,
    row_index,
    context.row_count,
    row_condition,
  );
  merge_table_row_style(
    &mut row_style,
    &direct_table_row_style(row.table_row_properties.as_deref()),
  );
  let cells = table_row_cells(row);
  let cell_count = cells.len();
  let cells = cells
    .iter()
    .enumerate()
    .map(|(cell_index, cell)| {
      table_cell_model(
        cell,
        context,
        row.table_property_exceptions.as_deref(),
        table_cell_style_for(
          context.table_style,
          TableCellStyleContext {
            look: context.table_look,
            row_index,
            row_count: context.row_count,
            cell_index,
            cell_count,
            row_condition,
            cell_condition: cell
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
    redline_color: None,
    cells,
  }
}

fn table_row_cells(row: &w::TableRow) -> Vec<&w::TableCell> {
  let mut cells = Vec::new();
  for choice in &row.table_row_choice {
    match choice {
      w::TableRowChoice::TableCell(cell) => cells.push(cell.as_ref()),
      w::TableRowChoice::SdtCell(sdt) => collect_sdt_cells(sdt, &mut cells),
      _ => {}
    }
  }
  cells
}

fn collect_sdt_cells<'a>(sdt: &'a w::SdtCell, cells: &mut Vec<&'a w::TableCell>) {
  let Some(content) = sdt.sdt_content_cell.as_ref() else {
    return;
  };
  for choice in &content.sdt_content_cell_choice {
    match choice {
      w::SdtContentCellChoice::TableCell(cell) => cells.push(cell.as_ref()),
      w::SdtContentCellChoice::SdtCell(nested) => collect_sdt_cells(nested, cells),
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
  context: &mut TableImportContext<'_>,
  row_table_exceptions: Option<&w::TablePropertyExceptions>,
  style: TableCellStyle,
) -> TableCell {
  let properties = cell.table_cell_properties.as_deref();
  let base_margins = if context.direct_cell_margins {
    context.cell_margins
  } else {
    style.margins.unwrap_or(context.cell_margins)
  };
  let row_cell_margins = row_table_exceptions
    .and_then(|exceptions| exceptions.table_cell_margin_default.as_deref())
    .map(|margins| table_cell_margin_default_with_base(margins, base_margins))
    .unwrap_or(base_margins);
  let mut blocks = cell
    .table_cell_choice
    .iter()
    .fold(Vec::new(), |mut blocks, choice| {
      match choice {
        w::TableCellChoice::Paragraph(paragraph) => {
          let paragraph = paragraph_model_with_base(
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
          push_cell_paragraph(&mut blocks, paragraph);
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
            in_header_footer: false,
          },
        ))),
        w::TableCellChoice::SdtBlock(sdt) => blocks.extend(sdt_block_blocks(
          sdt,
          context.styles,
          context.numbering,
          context.images,
          context.hyperlinks,
          context.custom_xml_bindings,
          context.form_widget_ids,
        )),
        _ => {}
      }
      blocks
    });
  if let Some(rotation_deg) = properties.and_then(table_cell_text_rotation_degrees) {
    rotate_blocks_text(&mut blocks, rotation_deg);
  }
  TableCell {
    blocks,
    shading: properties
      .and_then(|properties| properties.shading.as_ref())
      .and_then(shading_fill)
      .or(style.shading),
    borders: properties
      .and_then(|properties| properties.table_cell_borders.as_deref())
      .map(|borders| direct_cell_borders_model(style.borders, borders))
      .unwrap_or(style.borders),
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
    vertical_merge_continue: properties
      .and_then(|properties| properties.vertical_merge.as_ref())
      .map(|merge| matches!(merge.val, None | Some(w::MergedCellValues::Continue)))
      .unwrap_or(false),
    vertical_alignment: properties
      .and_then(|properties| properties.table_cell_vertical_alignment.as_ref())
      .map(table_cell_vertical_alignment)
      .or(style.vertical_alignment)
      .unwrap_or_default(),
  }
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
    .and_then(table_margin_measurement_to_points)
}

fn table_alignment(justification: &w::TableJustification) -> TableAlignment {
  match justification.val {
    w::TableRowAlignmentValues::Center => TableAlignment::Center,
    w::TableRowAlignmentValues::Right | w::TableRowAlignmentValues::End => TableAlignment::Right,
    w::TableRowAlignmentValues::Left | w::TableRowAlignmentValues::Start => TableAlignment::Left,
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
  })
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
  }
  if let Some(keep_next) = properties.keep_next() {
    format.keep_with_next = keep_next.val.is_none_or(|value| value.as_bool());
  }
  if let Some(keep_lines) = properties.keep_lines() {
    format.keep_lines = keep_lines.val.is_none_or(|value| value.as_bool());
  }
  if let Some(contextual_spacing) = properties.contextual_spacing() {
    format.contextual_spacing = contextual_spacing.val.is_none_or(|value| value.as_bool());
  }
  if let Some(snap_to_grid) = properties.snap_to_grid() {
    format.snap_to_grid = Some(snap_to_grid.val.is_none_or(|value| value.as_bool()));
  }

  if let Some(spacing) = properties.spacing_between_lines() {
    if let Some(before) = spacing.before.as_ref() {
      format.spacing_before_set = true;
      format.spacing_before_pt = signed_twips_measure_to_points(before).unwrap_or(0.0);
    }
    if let Some(after) = spacing.after.as_ref() {
      format.spacing_after_set = true;
      format.spacing_after_pt = signed_twips_measure_to_points(after).unwrap_or(0.0);
    }
    if let Some(line) = spacing.line.as_ref() {
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

  if let Some(indentation) = properties.indentation() {
    if indentation.start.is_some() || indentation.left.is_some() {
      format.indent_left_set = true;
      format.indent_left_pt = indentation
        .start
        .as_ref()
        .or(indentation.left.as_ref())
        .and_then(signed_twips_measure_to_points)
        .unwrap_or(0.0);
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
    if indentation.first_line.is_some() || indentation.hanging.is_some() {
      format.first_line_indent_set = true;
      let first_line = indentation
        .first_line
        .as_ref()
        .and_then(twips_measure_to_points)
        .unwrap_or(0.0);
      let hanging = indentation
        .hanging
        .as_ref()
        .and_then(signed_twips_measure_to_points)
        .unwrap_or(0.0);
      format.first_line_indent_pt = first_line - hanging;
    }
  }

  if let Some(tabs) = properties.tabs() {
    // LN_CT_PPrBase_tabs initializes the current tab-stop vector from the
    // paragraph style, then DomainMapper_Impl::IncorporateTabStop() applies
    // each direct tab. A w:val="clear" entry removes an inherited tab at the
    // same position instead of being ignored.
    apply_tab_stops(&mut format.tab_stops, tabs);
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

  if let Some(frame) = properties.frame_properties() {
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
    merged.height_rule = frame_height_rule(frame.height_type);
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
    height_pt: frame.height.as_ref().and_then(twips_measure_to_points),
    height_rule: frame_height_rule(frame.height_type),
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

fn apply_tab_stops(stops: &mut Vec<TabStop>, tabs: &w::Tabs) {
  for tab in &tabs.tab_stop {
    let Some(position_pt) = signed_twips_measure_to_points(&tab.position)
      .filter(|position| position.is_finite() && *position >= 0.0)
    else {
      continue;
    };
    if matches!(tab.val, w::TabStopValues::Clear) {
      stops.retain(|stop| (stop.position_pt - position_pt).abs() >= TAB_STOP_DEDUP_EPSILON_PT);
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
    if let Some(existing) = stops
      .iter_mut()
      .find(|stop| (stop.position_pt - position_pt).abs() < TAB_STOP_DEDUP_EPSILON_PT)
    {
      existing.alignment = alignment;
      existing.leader = tab.leader.map(tab_leader).unwrap_or_default();
    } else {
      stops.push(TabStop {
        position_pt,
        alignment,
        leader: tab.leader.map(tab_leader).unwrap_or_default(),
      });
    }
  }
  stops.sort_by(|a, b| a.position_pt.total_cmp(&b.position_pt));
  stops.dedup_by(|a, b| (a.position_pt - b.position_pt).abs() < TAB_STOP_DEDUP_EPSILON_PT);
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

fn paragraph_inlines(
  paragraph: &w::Paragraph,
  base_style: TextStyle,
  styles: &StylesCatalog,
  images: &ImageCatalog,
  hyperlinks: &HyperlinkCatalog,
  custom_xml_bindings: &CustomXmlBindings,
  form_widget_ids: &mut FormWidgetIdAllocator,
) -> Vec<InlineItem> {
  let mut inlines = Vec::new();
  let mut inline_context = InlineImportContext {
    styles,
    images,
    hyperlinks,
    custom_xml_bindings,
    form_widget_ids,
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
        push_inserted_run(
          inserted,
          &mut inlines,
          base_style.clone(),
          styles,
          images,
          hyperlinks,
          None,
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
        push_move_to_run(
          moved,
          &mut inlines,
          base_style.clone(),
          styles,
          images,
          hyperlinks,
          None,
        );
      }
      w::ParagraphChoice::SdtRun(sdt) => push_sdt_run(
        sdt,
        &mut inlines,
        base_style.clone(),
        None,
        &mut inline_context,
      ),
      w::ParagraphChoice::AlternateContent(alternate_content) => {
        push_alternate_content_inlines(
          alternate_content,
          &mut inlines,
          AlternateContentImportContext {
            base_style: base_style.clone(),
            style: base_style.clone(),
            styles,
            images,
            hyperlinks,
            hyperlink_url: None,
          },
        );
      }
      choice => {
        if let Some(text) = shared_math::wordprocessing_math_text(choice)
          && !text.is_empty()
        {
          let mut style = base_style.clone();
          style.font_family = Some(Arc::from("Cambria Math"));
          inlines.push(InlineItem::Text(TextRun {
            text,
            style,
            hyperlink_url: None,
            dynamic_field: None,
            style_ref_keys: Vec::new(),
            style_ref_text: None,
            preserve_text_portion: false,
          }));
        }
      }
    }
  }
  flush_unclosed_complex_fields(&mut inlines, &mut complex_fields);

  inlines
}

fn math_paragraph_alignment(paragraph: &w::Paragraph) -> Option<ParagraphAlignment> {
  paragraph.paragraph_choice.iter().find_map(|choice| {
    let w::ParagraphChoice::Paragraph(math_paragraph) = choice else {
      return None;
    };
    let justification = math_paragraph
      .paragraph_properties
      .as_deref()
      .and_then(|properties| properties.justification.as_ref())
      .map(|justification| justification.val)
      .unwrap_or(ooxmlsdk::schemas::m::JustificationValues::CenterGroup);
    Some(match justification {
      ooxmlsdk::schemas::m::JustificationValues::Left => ParagraphAlignment::Left,
      ooxmlsdk::schemas::m::JustificationValues::Right => ParagraphAlignment::Right,
      ooxmlsdk::schemas::m::JustificationValues::Center
      | ooxmlsdk::schemas::m::JustificationValues::CenterGroup => ParagraphAlignment::Center,
    })
  })
}

#[derive(Clone, Debug)]
struct ComplexFieldState {
  instr: String,
  result: Vec<InlineItem>,
  in_result: bool,
  style: TextStyle,
  hyperlink_url: Option<String>,
}

#[derive(Clone, Copy)]
struct RunImportContext<'a> {
  styles: &'a StylesCatalog,
  images: &'a ImageCatalog,
  hyperlinks: &'a HyperlinkCatalog,
}

struct InlineImportContext<'a> {
  styles: &'a StylesCatalog,
  images: &'a ImageCatalog,
  hyperlinks: &'a HyperlinkCatalog,
  custom_xml_bindings: &'a CustomXmlBindings,
  form_widget_ids: &'a mut FormWidgetIdAllocator,
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
    push_run(
      run,
      inlines,
      base_style,
      context.styles,
      context.images,
      context.hyperlinks,
      hyperlink_url,
    );
    return;
  }

  let style = properties::run_style(
    run.run_properties.as_deref(),
    base_style.clone(),
    context.styles,
  );
  for choice in &run.run_choice {
    match choice {
      w::RunChoice::FieldChar(field_char)
        if field_char.field_char_type == w::FieldCharValues::Begin =>
      {
        fields.push(ComplexFieldState {
          instr: String::new(),
          result: Vec::new(),
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
        flush_closed_complex_field(inlines, fields);
      }
      w::RunChoice::FieldCode(code) => {
        if let Some(field) = fields.last_mut()
          && !field.in_result
          && let Some(content) = &code.xml_content
        {
          field.instr.push_str(content);
        }
      }
      _ => {
        if let Some(field) = fields.last_mut()
          && field.in_result
        {
          push_run(
            run,
            &mut field.result,
            base_style.clone(),
            context.styles,
            context.images,
            context.hyperlinks,
            hyperlink_url,
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

fn flush_closed_complex_field(inlines: &mut Vec<InlineItem>, fields: &mut Vec<ComplexFieldState>) {
  let Some(state) = fields.pop() else {
    return;
  };
  let mut resolved = Vec::new();
  if let Some(kind) = dynamic_field_kind(&state.instr) {
    push_dynamic_field(
      &mut resolved,
      kind,
      state.style,
      state.hyperlink_url.as_deref(),
      field_result_text(&state.result),
    );
  } else {
    resolved = state.result;
  }
  if let Some(parent) = fields.last_mut()
    && parent.in_result
  {
    parent.result.extend(resolved);
  } else {
    inlines.extend(resolved);
  }
}

fn flush_unclosed_complex_fields(
  inlines: &mut Vec<InlineItem>,
  fields: &mut Vec<ComplexFieldState>,
) {
  while !fields.is_empty() {
    flush_closed_complex_field(inlines, fields);
  }
}

fn dynamic_field_kind(instr: &str) -> Option<DynamicFieldKind> {
  let tokens = field_instruction_tokens(instr);
  let name = tokens.first()?.to_ascii_uppercase();
  match name.as_str() {
    "PAGE" => Some(DynamicFieldKind::Page),
    "NUMPAGES" => Some(DynamicFieldKind::NumPages),
    "PAGEREF" => page_ref_field_kind(&tokens[1..]),
    "STYLEREF" => style_ref_field_kind(&tokens[1..]),
    _ => None,
  }
}

fn page_ref_field_kind(tokens: &[String]) -> Option<DynamicFieldKind> {
  for token in tokens {
    if token.starts_with('\\') {
      continue;
    }
    return Some(DynamicFieldKind::PageRef {
      bookmark_name: Arc::<str>::from(token.as_str()),
    });
  }
  None
}

fn style_ref_field_kind(tokens: &[String]) -> Option<DynamicFieldKind> {
  let mut style_name = None;
  let mut from_bottom = false;
  let mut skip_switch_arg = false;
  for token in tokens {
    if skip_switch_arg {
      skip_switch_arg = false;
      continue;
    }
    if let Some(switch) = token.strip_prefix('\\') {
      if switch.eq_ignore_ascii_case("l") {
        from_bottom = true;
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
    preserve_text_portion: false,
  }));
}

fn field_result_text(result: &[InlineItem]) -> Option<String> {
  let mut text = String::new();
  for item in result {
    match item {
      InlineItem::Text(run) => text.push_str(&run.text),
      InlineItem::PageBreak | InlineItem::ColumnBreak | InlineItem::LastRenderedPageBreak => {}
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
  hyperlink
    .id
    .as_deref()
    .and_then(|relationship_id| hyperlinks.target(relationship_id))
    .map(ToString::to_string)
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
      w::HyperlinkChoice::InsertedRun(inserted) => push_inserted_run(
        inserted,
        inlines,
        base_style.clone(),
        context.styles,
        context.images,
        context.hyperlinks,
        hyperlink_url.as_deref(),
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
      w::HyperlinkChoice::MoveToRun(moved) => push_move_to_run(
        moved,
        inlines,
        base_style.clone(),
        context.styles,
        context.images,
        context.hyperlinks,
        hyperlink_url.as_deref(),
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
      w::CustomXmlRunChoice::InsertedRun(inserted) => push_inserted_run(
        inserted,
        inlines,
        base_style.clone(),
        context.styles,
        context.images,
        context.hyperlinks,
        hyperlink_url,
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
      w::CustomXmlRunChoice::MoveToRun(moved) => push_move_to_run(
        moved,
        inlines,
        base_style.clone(),
        context.styles,
        context.images,
        context.hyperlinks,
        hyperlink_url,
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
      w::RunChoice::FootnoteReference(reference) if reference.id > 0 => {
        footnotes.push(reference.id);
      }
      w::RunChoice::EndnoteReference(reference) if reference.id > 0 => {
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
  if let Some(kind) = dynamic_field_kind(&field.instruction) {
    push_dynamic_field(
      inlines,
      kind,
      base_style.clone(),
      None,
      simple_field_result_text(field, base_style, context),
    );
    return;
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
        flush_unclosed_complex_fields(inlines, &mut complex_fields);
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

fn simple_field_result_text(
  field: &w::SimpleField,
  base_style: TextStyle,
  context: &mut InlineImportContext<'_>,
) -> Option<String> {
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
        flush_unclosed_complex_fields(&mut result, &mut complex_fields);
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
  field_result_text(&result)
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
  let style = properties::run_style(run.run_properties.as_deref(), base_style.clone(), styles);
  let style_ref_keys = run
    .run_properties
    .as_deref()
    .and_then(run_properties_style_id)
    .map(|style_id| styles.style_ref_keys(style_id))
    .unwrap_or_default();
  if style.hidden {
    push_hidden_style_ref_run(run, inlines, style, hyperlink_url, &style_ref_keys);
    return;
  }
  let mut text = String::new();

  for choice in &run.run_choice {
    match choice {
      w::RunChoice::Text(text_node) => {
        if let Some(content) = &text_node.xml_content {
          text.push_str(content);
        }
      }
      w::RunChoice::DeletedText(text_node) => {
        if let Some(content) = &text_node.xml_content {
          text.push_str(content);
        }
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
        if let Some(symbol) = symbol_text(symbol) {
          text.push(symbol);
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
          DynamicFieldKind::Page,
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
      w::RunChoice::PositionalTab(_) => text.push('\t'),
      w::RunChoice::AlternateContent(alternate_content) => {
        flush_run_text(
          inlines,
          &mut text,
          style.clone(),
          hyperlink_url,
          &style_ref_keys,
        );
        push_alternate_content_inlines(
          alternate_content,
          inlines,
          AlternateContentImportContext {
            base_style: base_style.clone(),
            style: style.clone(),
            styles,
            images,
            hyperlinks,
            hyperlink_url: hyperlink_url.map(str::to_owned),
          },
        );
      }
      w::RunChoice::Ruby(ruby) => {
        flush_run_text(
          inlines,
          &mut text,
          style.clone(),
          hyperlink_url,
          &style_ref_keys,
        );
        push_ruby_base(
          ruby,
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

  flush_run_text(inlines, &mut text, style, hyperlink_url, &style_ref_keys);
}

#[derive(Clone)]
struct AlternateContentImportContext<'a> {
  base_style: TextStyle,
  style: TextStyle,
  styles: &'a StylesCatalog,
  images: &'a ImageCatalog,
  hyperlinks: &'a HyperlinkCatalog,
  hyperlink_url: Option<String>,
}

fn push_alternate_content_inlines(
  alternate_content: &mc::AlternateContent,
  inlines: &mut Vec<InlineItem>,
  context: AlternateContentImportContext<'_>,
) {
  let fallback =
    alternate_content
      .alternate_content_choice
      .iter()
      .find_map(|branch| match branch {
        mc::AlternateContentChoice::Fallback(fallback) => Some(fallback.as_ref()),
        _ => None,
      });

  for choice in alternate_content
    .alternate_content_choice
    .iter()
    .filter_map(|branch| match branch {
      mc::AlternateContentChoice::Choice(choice) => Some(choice.as_ref()),
      _ => None,
    })
  {
    let mut choice_inlines = Vec::new();
    for child in &choice.xml_children {
      push_alternate_content_child(child, &mut choice_inlines, &context, true);
    }
    if !choice_inlines.is_empty() {
      inlines.extend(choice_inlines);
      return;
    }
  }

  if let Some(fallback) = fallback {
    for child in &fallback.xml_children {
      push_alternate_content_child(child, inlines, &context, false);
    }
  }
}

fn push_alternate_content_child(
  xml: &[u8],
  inlines: &mut Vec<InlineItem>,
  context: &AlternateContentImportContext<'_>,
  choice_branch: bool,
) {
  let Ok(xml_text) = std::str::from_utf8(xml) else {
    return;
  };
  match root_qname(xml_text) {
    Some("w:r") => {
      if let Ok(run) = w::Run::from_bytes(xml) {
        push_run(
          &run,
          inlines,
          context.base_style.clone(),
          context.styles,
          context.images,
          context.hyperlinks,
          context.hyperlink_url.as_deref(),
        );
      }
    }
    Some("w:drawing") => {
      let Ok(drawing) = w::Drawing::from_bytes(xml) else {
        return;
      };
      if choice_branch
        && drawing_graphic_data(&drawing).is_some_and(|graphic_data| {
          graphic_data.uri
            == "http://schemas.microsoft.com/office/word/2008/6/28/wordprocessingShape"
        })
      {
        return;
      }
      if let Some(image) =
        drawing::inline_image(&drawing, context.styles, context.images, context.hyperlinks)
      {
        inlines.push(InlineItem::Image(image));
      }
      drawing::push_drawing_shapes(
        &drawing,
        inlines,
        context.styles,
        context.images,
        context.hyperlinks,
      );
      drawing::push_drawing_textboxes(
        &drawing,
        inlines,
        context.style.clone(),
        context.styles,
        context.images,
        context.hyperlinks,
      );
    }
    Some("w:pict") => {
      let Ok(picture) = w::Picture::from_bytes(xml) else {
        return;
      };
      if let Some(image) = drawing::pict_image(&picture, context.images) {
        inlines.push(InlineItem::Image(image));
      }
      drawing::push_pict_shapes(&picture, inlines, context.images);
      drawing::push_pict_textboxes(
        &picture,
        inlines,
        context.base_style.clone(),
        context.styles,
        context.images,
        context.hyperlinks,
      );
    }
    _ => {}
  }
}

fn push_hidden_style_ref_run(
  run: &w::Run,
  inlines: &mut Vec<InlineItem>,
  style: TextStyle,
  hyperlink_url: Option<&str>,
  style_ref_keys: &[Arc<str>],
) {
  if style_ref_keys.is_empty() {
    return;
  }
  let text = hidden_run_text(run);
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
    preserve_text_portion: false,
  }));
}

fn hidden_run_text(run: &w::Run) -> String {
  let mut text = String::new();
  for choice in &run.run_choice {
    match choice {
      w::RunChoice::Text(text_node) => {
        if let Some(content) = &text_node.xml_content {
          text.push_str(content);
        }
      }
      w::RunChoice::DeletedText(text_node) => {
        if let Some(content) = &text_node.xml_content {
          text.push_str(content);
        }
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

fn run_properties_style_id(properties: &w::RunProperties) -> Option<&str> {
  run_properties_run_style(properties).map(|run_style| run_style.val.as_str())
}

fn push_ruby_base(
  ruby: &w::Ruby,
  inlines: &mut Vec<InlineItem>,
  base_style: TextStyle,
  styles: &StylesCatalog,
  images: &ImageCatalog,
  hyperlinks: &HyperlinkCatalog,
  hyperlink_url: Option<&str>,
) {
  for choice in &ruby.ruby_base.ruby_base_choice {
    match choice {
      w::RubyBaseChoice::WRun(run) => push_run(
        run,
        inlines,
        base_style.clone(),
        styles,
        images,
        hyperlinks,
        hyperlink_url,
      ),
      w::RubyBaseChoice::InsertedRun(inserted) => {
        push_inserted_run(
          inserted,
          inlines,
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
          inlines,
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
          inlines,
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
    .and_then(|properties| context.custom_xml_bindings.value_for_sdt(properties))
  {
    inlines.push(InlineItem::Text(TextRun {
      text: format!("*{value}*"),
      style: base_style,
      hyperlink_url: hyperlink_url.map(str::to_owned),
      dynamic_field: None,
      style_ref_keys: Vec::new(),
      style_ref_text: None,
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
        push_inserted_run(
          inserted.as_ref(),
          inlines,
          base_style.clone(),
          context.styles,
          context.images,
          context.hyperlinks,
          hyperlink_url,
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
        push_move_to_run(
          moved.as_ref(),
          inlines,
          base_style.clone(),
          context.styles,
          context.images,
          context.hyperlinks,
          hyperlink_url,
        );
      }
      _ => {}
    }
  }
  flush_unclosed_complex_fields(inlines, &mut complex_fields);
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

fn sdt_showing_placeholder(properties: &w::SdtProperties) -> bool {
  properties
    .sdt_properties_choice
    .iter()
    .any(|choice| matches!(choice, w::SdtPropertiesChoice::ShowingPlaceholder(_)))
}

fn sdt_form_widget(properties: &w::SdtProperties) -> Option<(FormWidgetKind, Vec<String>)> {
  let mut kind = None;
  let mut entries = Vec::new();
  let mut showing_placeholder = false;
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
      w::SdtPropertiesChoice::ShowingPlaceholder(_) => {
        showing_placeholder = true;
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
  if id < 1 {
    return;
  }
  inlines.push(InlineItem::Text(TextRun {
    text: id.to_string(),
    style: note_reference_style(&style),
    hyperlink_url,
    dynamic_field: None,
    style_ref_keys: Vec::new(),
    style_ref_text: None,
    preserve_text_portion: false,
  }));
}

fn note_reference_style(style: &TextStyle) -> TextStyle {
  if style.baseline_shift_pt.abs() > f32::EPSILON {
    return style.clone();
  }
  let mut reference_style = style.clone();
  reference_style.baseline_shift_pt = style.font_size_pt * LO_SUPERSCRIPT_BASELINE_SHIFT_SCALE;
  reference_style.font_size_pt =
    (style.font_size_pt * LO_DEFAULT_ESCAPEMENT_HEIGHT_SCALE).max(MIN_ESCAPEMENT_FONT_SIZE_PT);
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
      preserve_text_portion: false,
    }));
  }
}

fn run_display_text(text: String, style: TextStyle) -> String {
  if style.uppercase {
    text.to_uppercase()
  } else {
    text
  }
}

fn symbol_text(symbol: &w::SymbolChar) -> Option<char> {
  let code = u32::from_str_radix(symbol.char.as_deref()?, 16).ok()?;
  shared_symbol::font_symbol_code(symbol.font.as_deref(), code)
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
      let properties =
        drawing_image_properties(&inline.graphic.graphic_data, &styles.theme_colors)?;
      let relationship_id = properties.relationship_id.as_deref()?;
      let resource = images.by_relationship_id.get(relationship_id)?;
      let image_data = image_data_with_effects(resource, &properties);
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
        width_pt: units::emu_to_points(inline.extent.cx),
        height_pt: units::emu_to_points(inline.extent.cy),
        effect_left_pt: effect_extent_left(inline.effect_extent.as_ref()),
        effect_top_pt: effect_extent_top(inline.effect_extent.as_ref()),
        effect_right_pt: effect_extent_right(inline.effect_extent.as_ref()),
        effect_bottom_pt: effect_extent_bottom(inline.effect_extent.as_ref()),
        crop: properties.crop,
        rotation_deg: properties.rotation_deg,
        flip_horizontal: properties.flip_horizontal,
        flip_vertical: properties.flip_vertical,
        alt_text: inline.doc_properties.description.clone(),
        hyperlink_url,
        placement: ImagePlacement::Inline,
      })
    }
    w::DrawingChoice::Anchor(anchor) => {
      let graphic = anchor.graphic.as_ref();
      let extent = &anchor.extent;
      let properties = drawing_image_properties(&graphic.graphic_data, &styles.theme_colors)?;
      let relationship_id = properties.relationship_id.as_deref()?;
      let resource = images.by_relationship_id.get(relationship_id)?;
      let image_data = image_data_with_effects(resource, &properties);
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
      Some(InlineImage {
        data: image_data.data,
        content_type: image_data.content_type,
        width_pt: units::emu_to_points(extent.cx),
        height_pt: units::emu_to_points(extent.cy),
        effect_left_pt: effect_extent_left(anchor.effect_extent.as_ref()),
        effect_top_pt: effect_extent_top(anchor.effect_extent.as_ref()),
        effect_right_pt: effect_extent_right(anchor.effect_extent.as_ref()),
        effect_bottom_pt: effect_extent_bottom(anchor.effect_extent.as_ref()),
        crop: properties.crop,
        rotation_deg: properties.rotation_deg,
        flip_horizontal: properties.flip_horizontal,
        flip_vertical: properties.flip_vertical,
        alt_text: anchor
          .doc_properties
          .as_ref()
          .and_then(|properties| properties.description.clone()),
        hyperlink_url,
        placement: ImagePlacement::Floating(floating_image_placement(anchor)),
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
    relative_height: anchor.relative_height.unwrap_or_default(),
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
    wp::AnchorChoice::WrapThrough(_) => ImageWrapMode::Square,
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
  if drawing_image_properties(graphic_data, &styles.theme_colors).is_some() {
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
      inlines.extend(text_box_frames.into_iter().map(InlineItem::Shape));
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

fn wordprocessing_shape_textbox_has_fontwork_warp(shape: &wps::WordprocessingShape) -> bool {
  shape
    .text_body_properties
    .as_deref()
    .and_then(|properties| properties.preset_text_warp.as_ref())
    .is_some_and(|warp| {
      !matches!(
        warp.preset,
        a::TextShapeValues::TextPlain | a::TextShapeValues::TextNoShape
      )
    })
}

fn fontwork_warp_geometry() -> InlineShapeGeometry {
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
  if wordprocessing_shape_textbox_uses_auto_light_text(shape, styles) {
    base_style.color = RgbColor {
      r: 255,
      g: 255,
      b: 255,
    };
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
  apply_drawingml_textbox_layout_adjustments(&mut frame);
  frame
}

fn wordprocessing_shape_textbox_text_rotation(shape: &wps::WordprocessingShape) -> Option<f32> {
  match shape
    .text_body_properties
    .as_ref()
    .and_then(|properties| properties.vertical)
  {
    Some(a::TextVerticalValues::Vertical)
    | Some(a::TextVerticalValues::WordArtVertical)
    | Some(a::TextVerticalValues::EastAsianVetical) => Some(-90.0),
    Some(a::TextVerticalValues::Vertical270) | Some(a::TextVerticalValues::WordArtLeftToRight) => {
      Some(90.0)
    }
    _ => None,
  }
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

fn apply_drawingml_textbox_layout_adjustments(frame: &mut TextBoxFrameContent) {
  frame.left_pt = (frame.left_pt - 1.67).max(0.0);
  if frame.top_pt.abs() < f32::EPSILON {
    frame.top_pt = -14.5;
  }
}

fn wordprocessing_shape_textbox_uses_auto_light_text(
  shape: &wps::WordprocessingShape,
  styles: &StylesCatalog,
) -> bool {
  let fill_color = wordprocessing_shape_fill_color(shape, &styles.theme_colors).or_else(|| {
    shape
      .shape_style
      .as_ref()
      .and_then(|style| drawingml_fill_reference_color(&style.fill_reference, &styles.theme_colors))
  });
  fill_color.is_some_and(libreoffice_color_is_dark)
}

fn libreoffice_color_is_dark(color: RgbColor) -> bool {
  color_wcag_luminance(color) <= 87
}

fn color_wcag_luminance(color: RgbColor) -> u8 {
  let red = normalized_linear_rgb(color.r);
  let green = normalized_linear_rgb(color.g);
  let blue = normalized_linear_rgb(color.b);
  (255.0 * (red * 0.2126 + green * 0.7152 + blue * 0.0722)).round() as u8
}

fn normalized_linear_rgb(component: u8) -> f32 {
  let component = f32::from(component) / 255.0;
  if component <= 0.04045 {
    component / 12.92
  } else {
    ((component + 0.055) / 1.055).powf(2.4)
  }
}

fn apply_wordprocessing_shape_textbox_body_properties(
  properties: &wps::TextBodyProperties,
  frame: &mut TextBoxFrameContent,
) {
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
  let child_transform = drawingml_group_transform_from_properties(&group.group_shape_properties)
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
  let child_transform = drawingml_group_transform_from_properties(&group.group_shape_properties)
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
  let text_box = text_box_frame_from_wordprocessing_shape(
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
  let (offset_x_pt, offset_y_pt, shape_width_pt, shape_height_pt) =
    transform.rect(offset_x_pt, offset_y_pt, shape_width_pt, shape_height_pt);
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
  let has_fontwork_warp = wordprocessing_shape_textbox_has_fontwork_warp(shape);
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
  let geometry = if has_fontwork_warp {
    fontwork_warp_geometry()
  } else {
    InlineShapeGeometry::Rectangle
  };
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
    fill_color,
    additional_fill_colors,
    fill_image: None,
    stroke: frame_stroke.or_else(|| expands_auto_fit.then_some(BorderStyle::default())),
    suppress_zero_relative_background: false,
    allow_outside_page: false,
    inline_anchor_after_line: matches!(placement, ImagePlacement::Inline),
    placement,
    text_box_blocks: text_box.blocks,
    text_inset_left_pt: text_box.left_pt,
    text_inset_top_pt: text_box.top_pt,
    text_inset_right_pt: text_box.right_pt,
    text_inset_bottom_pt: text_box.bottom_pt,
    text_box_auto_fit: auto_fit,
    text_vertical_alignment: text_box.vertical_alignment,
  })
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
  let is_top_level_picture = drawing_image_properties(graphic_data, &styles.theme_colors).is_some();

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
  for choice in &graphic_data.graphic_data_choice {
    match choice {
      a::GraphicDataChoice::ChartReference(reference) => {
        if let Some(chart_shapes) =
          drawing_chart_shapes(drawing, reference, &images.charts_by_relationship_id)
        {
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
  let child_transform = drawingml_group_transform_from_properties(&group.group_shape_properties)
    .map(|xfrm| transform.child(xfrm))
    .unwrap_or(transform);
  let child_context = DrawingShapeImportContext {
    effect_extent: DrawingEffectExtent::default(),
    ..context
  };
  group
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
    .collect()
}

fn wordprocessing_group_shape_shapes(
  group: &wpg::GroupShape,
  placement: ImagePlacement,
  transform: DrawingMlGroupTransform,
  context: DrawingShapeImportContext<'_>,
) -> Vec<InlineItem> {
  let child_transform = drawingml_group_transform_from_properties(&group.group_shape_properties)
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
      wordprocessing_group_shape_choice_shapes(
        choice,
        drawingml_group_child_placement(placement),
        child_transform,
        child_context,
      )
    })
    .collect()
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
  let explicit_fill_color =
    drawingml_shape_properties_fill_color(&properties, &context.styles.theme_colors);
  let fill_color = if drawingml_shape_properties_has_no_fill(&properties) {
    None
  } else {
    explicit_fill_color.or_else(|| {
      shape.shape_style.as_ref().and_then(|style| {
        drawingml_fill_reference_color(&style.fill_reference, &context.styles.theme_colors)
      })
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
  if fill_color.is_none() && fill_image.is_none() && stroke.is_none() {
    return None;
  }

  let mut geometry = properties
    .geometry_kind()
    .unwrap_or(InlineShapeGeometry::Rectangle);
  let has_custom_geometry = properties.custom_geometry().is_some();
  if geometry == InlineShapeGeometry::Rectangle && has_custom_geometry {
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
  if has_custom_geometry
    && let Some(custom_geometry) =
      drawingml_custom_geometry_from_properties(&properties, width_pt, height_pt)
  {
    geometry = custom_geometry;
  }
  let (offset_x_pt, offset_y_pt, width_pt, height_pt) =
    transform.rect(offset_x_pt, offset_y_pt, width_pt, height_pt);

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
    fill_color,
    additional_fill_colors: Vec::new(),
    fill_image,
    stroke,
    suppress_zero_relative_background: explicit_fill_color.is_some(),
    allow_outside_page: false,
    inline_anchor_after_line: matches!(placement, ImagePlacement::Inline)
      && wordprocessing_shape_textbox_content(shape).is_some(),
    placement,
    text_box_blocks: Vec::new(),
    text_inset_left_pt: 0.0,
    text_inset_top_pt: 0.0,
    text_inset_right_pt: 0.0,
    text_inset_bottom_pt: 0.0,
    text_box_auto_fit: false,
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
  if let Some(shape) = drawingml_picture_frame(picture, placement, transform) {
    items.push(InlineItem::Shape(shape));
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
  let child_transform =
    drawingml_group_transform_from_diagram_properties(&drawing.shape_tree.group_shape_properties)
      .map(|xfrm| transform.child(xfrm))
      .unwrap_or(transform);
  drawing
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
    .collect()
}

fn drawingml_diagram_group_shapes(
  group: &dsp::GroupShape,
  placement: ImagePlacement,
  transform: DrawingMlGroupTransform,
  context: DrawingShapeImportContext<'_>,
) -> Vec<InlineItem> {
  let child_transform =
    drawingml_group_transform_from_diagram_properties(&group.group_shape_properties)
      .map(|xfrm| transform.child(xfrm))
      .unwrap_or(transform);
  group
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
    .collect()
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
  let explicit_fill_color =
    drawingml_shape_properties_fill_color(&properties, &context.styles.theme_colors);
  let fill_color = if drawingml_shape_properties_has_no_fill(&properties) {
    None
  } else {
    explicit_fill_color.or_else(|| {
      shape.shape_style.as_ref().and_then(|style| {
        drawingml_fill_reference_color(&style.fill_reference, &context.styles.theme_colors)
      })
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
  let smartart_text_color = context
    .smartart_text_colors_by_model_id
    .and_then(|colors| colors.get(shape.model_id.as_str()).copied());
  let mut text_box = drawingml_diagram_shape_text_box(shape, context.styles, smartart_text_color);
  if fill_color.is_none() && fill_image.is_none() && stroke.is_none() && text_box.is_none() {
    return None;
  }

  let mut geometry = properties
    .geometry_kind()
    .unwrap_or(InlineShapeGeometry::Rectangle);
  let has_custom_geometry = properties.custom_geometry().is_some();
  if geometry == InlineShapeGeometry::Rectangle && has_custom_geometry {
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
  if has_custom_geometry
    && let Some(custom_geometry) =
      drawingml_custom_geometry_from_properties(&properties, width_pt, height_pt)
  {
    geometry = custom_geometry;
  }
  let (offset_x_pt, offset_y_pt, width_pt, height_pt) =
    transform.rect(offset_x_pt, offset_y_pt, width_pt, height_pt);
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
    fill_color,
    additional_fill_colors: Vec::new(),
    fill_image,
    stroke,
    suppress_zero_relative_background: explicit_fill_color.is_some(),
    allow_outside_page: false,
    inline_anchor_after_line: matches!(placement, ImagePlacement::Inline)
      && shape.text_body.is_some(),
    placement,
    text_box_blocks: Vec::new(),
    text_inset_left_pt: 0.0,
    text_inset_top_pt: 0.0,
    text_inset_right_pt: 0.0,
    text_inset_bottom_pt: 0.0,
    text_box_auto_fit: false,
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
  apply_drawingml_textbox_layout_adjustments(&mut frame);
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
) -> Option<Vec<InlineShape>> {
  let chart_space = charts_by_relationship_id.get(reference.id.as_str())?;
  let (width_pt, height_pt, placement) = drawing_chart_extent_and_placement(drawing)?;
  let stroke = Some(BorderStyle::default());

  Some(match shared_chart::kind(chart_space) {
    shared_chart::ChartKind::Pie => {
      let diameter_pt = (width_pt.min(height_pt) * 0.911_706_3)
        .min(width_pt)
        .min(height_pt);
      vec![
        chart_shape(diameter_pt, diameter_pt, 0.0, placement, stroke),
        chart_shape(diameter_pt, diameter_pt, 0.0, placement, stroke),
      ]
    }
    shared_chart::ChartKind::Bar => vec![chart_shape(
      width_pt / 3.0,
      height_pt * 0.55,
      0.0,
      placement,
      stroke,
    )],
    shared_chart::ChartKind::Area => {
      let mut shapes = vec![chart_shape(
        width_pt,
        height_pt,
        height_pt * 1.055,
        placement,
        stroke,
      )];
      if chart_has_values(chart_space, &["37261", "23"]) {
        shapes.push(chart_text_shape(
          "23",
          30.0,
          12.0,
          -height_pt * 0.365,
          placement,
        ));
      }
      shapes
    }
    shared_chart::ChartKind::Other => {
      vec![chart_shape(width_pt, height_pt, 0.0, placement, stroke)]
    }
  })
}

fn chart_has_values(chart_space: &c::ChartSpace, expected: &[&str]) -> bool {
  shared_chart::has_values(chart_space, expected)
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
    fill_color: None,
    additional_fill_colors: Vec::new(),
    fill_image: None,
    stroke,
    suppress_zero_relative_background: false,
    allow_outside_page: false,
    inline_anchor_after_line: false,
    placement,
    text_box_blocks: Vec::new(),
    text_inset_left_pt: 0.0,
    text_inset_top_pt: 0.0,
    text_inset_right_pt: 0.0,
    text_inset_bottom_pt: 0.0,
    text_box_auto_fit: false,
    text_vertical_alignment: TextBoxVerticalAlignment::Top,
  }
}

fn chart_text_shape(
  text: &str,
  width_pt: f32,
  height_pt: f32,
  offset_y_pt: f32,
  placement: ImagePlacement,
) -> InlineShape {
  let mut shape = chart_shape(width_pt, height_pt, offset_y_pt, placement, None);
  shape.text_box_blocks = vec![simple_text_block(text.to_string(), TextStyle::default())];
  shape
}

#[derive(Clone, Copy, Debug)]
struct DrawingMlGroupTransform {
  scale_x: f32,
  scale_y: f32,
  translate_x_pt: f32,
  translate_y_pt: f32,
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
      scale_x: 1.0,
      scale_y: 1.0,
      translate_x_pt: 0.0,
      translate_y_pt: 0.0,
      raw_coordinates: false,
      fallback_size: None,
    }
  }

  fn with_fallback_size(mut self, fallback_size: Option<(f32, f32)>) -> Self {
    self.fallback_size = fallback_size;
    self
  }

  fn child(self, xfrm: DrawingMlGroupXfrm) -> Self {
    let scale_x = if xfrm.child_width != 0.0 {
      xfrm.width_pt / xfrm.child_width
    } else {
      1.0
    };
    let scale_y = if xfrm.child_height != 0.0 {
      xfrm.height_pt / xfrm.child_height
    } else {
      1.0
    };
    let (offset_x_pt, offset_y_pt, _, _) = self.rect(
      xfrm.offset_x_pt,
      xfrm.offset_y_pt,
      xfrm.width_pt,
      xfrm.height_pt,
    );
    Self {
      scale_x: self.scale_x * scale_x,
      scale_y: self.scale_y * scale_y,
      translate_x_pt: offset_x_pt - self.scale_x * xfrm.child_offset_x * scale_x,
      translate_y_pt: offset_y_pt - self.scale_y * xfrm.child_offset_y * scale_y,
      raw_coordinates: true,
      fallback_size: None,
    }
  }

  fn rect(self, x_pt: f32, y_pt: f32, width_pt: f32, height_pt: f32) -> (f32, f32, f32, f32) {
    (
      self.translate_x_pt + x_pt * self.scale_x,
      self.translate_y_pt + y_pt * self.scale_y,
      width_pt * self.scale_x.abs(),
      height_pt * self.scale_y.abs(),
    )
  }
}

#[derive(Clone, Copy, Debug, Default)]
struct DrawingMlGroupXfrm {
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
  Wordprocessing(wps::ShapeProperties),
  Picture(pic::ShapeProperties),
}

enum DrawingMlFillProperties<'a> {
  NoFill,
  Solid(&'a a::SolidFill),
  Gradient(&'a a::GradientFill),
}

impl DrawingMlShapeProperties {
  fn transform2_d(&self) -> Option<&a::Transform2D> {
    match self {
      Self::Diagram(properties) => properties.transform2_d.as_deref(),
      Self::Wordprocessing(properties) => properties.transform2_d.as_deref(),
      Self::Picture(properties) => properties.transform2_d.as_deref(),
    }
  }

  fn geometry_kind(&self) -> Option<InlineShapeGeometry> {
    let is_line = match self {
      Self::Diagram(properties) => matches!(
        properties.shape_properties_choice1.as_ref(),
        Some(dsp::ShapePropertiesChoice::PresetGeometry(geometry))
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

  fn fill(&self) -> Option<DrawingMlFillProperties<'_>> {
    match self {
      Self::Diagram(properties) => match properties.shape_properties_choice2.as_ref()? {
        dsp::ShapePropertiesChoice2::NoFill(_) => Some(DrawingMlFillProperties::NoFill),
        dsp::ShapePropertiesChoice2::SolidFill(fill) => {
          Some(DrawingMlFillProperties::Solid(fill.as_ref()))
        }
        dsp::ShapePropertiesChoice2::GradientFill(fill) => {
          Some(DrawingMlFillProperties::Gradient(fill.as_ref()))
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
        _ => None,
      },
    }
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
    fill_color: None,
    additional_fill_colors: Vec::new(),
    fill_image: None,
    stroke: None,
    suppress_zero_relative_background: false,
    allow_outside_page: false,
    inline_anchor_after_line: false,
    placement,
    text_box_blocks: Vec::new(),
    text_inset_left_pt: 0.0,
    text_inset_top_pt: 0.0,
    text_inset_right_pt: 0.0,
    text_inset_bottom_pt: 0.0,
    text_box_auto_fit: false,
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
) -> Option<DrawingMlGroupXfrm> {
  properties
    .transform_group
    .as_deref()
    .map(drawingml_group_transform_from_model)
}

fn drawingml_group_transform_from_diagram_properties(
  properties: &dsp::GroupShapeProperties,
) -> Option<DrawingMlGroupXfrm> {
  properties
    .transform_group
    .as_deref()
    .map(drawingml_group_transform_from_model)
}

fn drawingml_group_transform_from_model(transform: &a::TransformGroup) -> DrawingMlGroupXfrm {
  let mut group = DrawingMlGroupXfrm::default();

  if let Some(offset) = &transform.offset {
    group.offset_x_pt = units::emu_to_points(offset.x.to_emu());
    group.offset_y_pt = units::emu_to_points(offset.y.to_emu());
  }
  if let Some(extents) = &transform.extents {
    group.width_pt = units::emu_to_points(extents.cx.to_emu());
    group.height_pt = units::emu_to_points(extents.cy.to_emu());
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

fn drawingml_custom_geometry_from_properties(
  properties: &DrawingMlShapeProperties,
  width_pt: f32,
  height_pt: f32,
) -> Option<InlineShapeGeometry> {
  let geometry = properties.custom_geometry()?;
  let path = geometry.path_list.path.first()?;
  let path_width = path
    .width
    .map(|value| value.to_emu() as f32)
    .unwrap_or(width_pt);
  let path_height = path
    .height
    .map(|value| value.to_emu() as f32)
    .unwrap_or(height_pt);
  let mut points = Vec::new();
  let mut closed = false;

  for choice in &path.path_choice {
    match choice {
      a::PathChoice::CloseShapePath => closed = true,
      a::PathChoice::MoveTo(point) => {
        let x = point.point.x.parse::<f32>().ok()?;
        let y = point.point.y.parse::<f32>().ok()?;
        points.push(drawingml_custom_geometry_point(
          x,
          y,
          path_width,
          path_height,
          width_pt,
          height_pt,
        ));
      }
      a::PathChoice::LineTo(point) => {
        let x = point.point.x.parse::<f32>().ok()?;
        let y = point.point.y.parse::<f32>().ok()?;
        points.push(drawingml_custom_geometry_point(
          x,
          y,
          path_width,
          path_height,
          width_pt,
          height_pt,
        ));
      }
      _ => {}
    }
  }

  if points.len() == 2 && !closed {
    return Some(InlineShapeGeometry::Line);
  }

  (points.len() >= 2).then_some(InlineShapeGeometry::Polyline { points, closed })
}

fn drawingml_custom_geometry_point(
  x: f32,
  y: f32,
  path_width: f32,
  path_height: f32,
  width_pt: f32,
  height_pt: f32,
) -> (f32, f32) {
  let scaled_x = if path_width == 0.0 {
    0.0
  } else {
    x / path_width * width_pt
  };
  let scaled_y = if path_height == 0.0 {
    0.0
  } else {
    y / path_height * height_pt
  };
  (scaled_x, scaled_y)
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
) -> Option<InlineShape> {
  let properties = DrawingMlShapeProperties::Picture(
    picture
      .shape_properties
      .as_deref()
      .cloned()
      .unwrap_or_default(),
  );
  let geometry = properties
    .geometry_kind()
    .unwrap_or(InlineShapeGeometry::Rectangle);
  let (offset_x_pt, offset_y_pt, width_pt, height_pt) = drawingml_geometry_from_shape_properties(
    Some(&properties),
    &geometry,
    transform.raw_coordinates,
    None,
  )?;
  let (offset_x_pt, offset_y_pt, width_pt, height_pt) =
    transform.rect(offset_x_pt, offset_y_pt, width_pt, height_pt);

  Some(InlineShape {
    width_pt,
    height_pt,
    effect_left_pt: 0.0,
    effect_top_pt: 0.0,
    effect_right_pt: 0.0,
    effect_bottom_pt: 0.0,
    geometry: InlineShapeGeometry::Rectangle,
    offset_x_pt,
    offset_y_pt,
    fill_color: None,
    additional_fill_colors: Vec::new(),
    fill_image: None,
    stroke: None,
    suppress_zero_relative_background: false,
    allow_outside_page: false,
    inline_anchor_after_line: false,
    placement,
    text_box_blocks: Vec::new(),
    text_inset_left_pt: 0.0,
    text_inset_top_pt: 0.0,
    text_inset_right_pt: 0.0,
    text_inset_bottom_pt: 0.0,
    text_box_auto_fit: false,
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
  let properties = drawing_picture_image_properties(picture, &styles.theme_colors)?;
  let relationship_id = properties.relationship_id.as_deref()?;
  let resource = images.by_relationship_id.get(relationship_id)?;
  let image_data = image_data_with_effects(resource, &properties);
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
  let (offset_x_pt, offset_y_pt, width_pt, height_pt) =
    transform.rect(offset_x_pt, offset_y_pt, width_pt, height_pt);
  let hyperlink_url = properties
    .hyperlink_relationship_id
    .as_deref()
    .and_then(|relationship_id| hyperlinks.target(relationship_id))
    .map(ToString::to_string);
  Some(InlineImage {
    data: image_data.data,
    content_type: image_data.content_type,
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
    alt_text: drawingml_picture_alt_text(picture),
    hyperlink_url,
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
  let image_properties = drawing_blip_fill_image_properties(blip_fill, &ThemeColors::default())?;
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
  let image_properties = drawing_blip_fill_image_properties(blip_fill, &ThemeColors::default())?;
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
  })
}

fn resolve_drawingml_solid_fill(
  fill: &a::SolidFill,
  theme_colors: &ThemeColors,
) -> Option<ResolvedColor> {
  match fill.solid_fill_choice.as_ref()? {
    a::SolidFillChoice::RgbColorModelHex(color) => Some(ResolvedColor {
      color: parse_hex_color(color.val.as_str())?,
      opacity: opacity_from_drawingml_rgb_transforms(&color.rgb_color_model_hex_choice),
    }),
    a::SolidFillChoice::SystemColor(color) => Some(ResolvedColor {
      color: color.last_color.as_deref().and_then(parse_hex_color)?,
      opacity: opacity_from_drawingml_system_transforms(&color.system_color_choice),
    }),
    a::SolidFillChoice::SchemeColor(color) => Some(ResolvedColor {
      color: resolve_drawingml_scheme_color(color, theme_colors)?,
      opacity: opacity_from_drawingml_scheme_transforms(&color.scheme_color_choice),
    }),
    a::SolidFillChoice::PresetColor(color) => Some(ResolvedColor {
      color: drawingml_preset_color_value(color.val)?,
      opacity: 1.0,
    }),
    _ => None,
  }
}

fn drawingml_first_gradient_fill_color(
  fill: &a::GradientFill,
  theme_colors: &ThemeColors,
) -> Option<RgbColor> {
  let stop = fill.gradient_stop_list.as_ref()?.gradient_stop.first()?;
  match stop.gradient_stop_choice.as_ref()? {
    a::GradientStopChoice::RgbColorModelHex(color) => parse_hex_color(color.val.as_str()),
    a::GradientStopChoice::SystemColor(color) => {
      color.last_color.as_deref().and_then(parse_hex_color)
    }
    a::GradientStopChoice::SchemeColor(color) => {
      resolve_drawingml_scheme_color(color, theme_colors)
    }
    a::GradientStopChoice::PresetColor(color) => drawingml_preset_color_value(color.val),
    _ => None,
  }
}

fn opacity_from_drawingml_rgb_transforms(transforms: &[a::RgbColorModelHexChoice]) -> f32 {
  transforms
    .iter()
    .find_map(|transform| match transform {
      a::RgbColorModelHexChoice::Alpha(value) => drawingml_percent_to_ratio(&value.val),
      _ => None,
    })
    .unwrap_or(1.0)
}

fn opacity_from_drawingml_system_transforms(transforms: &[a::SystemColorChoice]) -> f32 {
  transforms
    .iter()
    .find_map(|transform| match transform {
      a::SystemColorChoice::Alpha(value) => drawingml_percent_to_ratio(&value.val),
      _ => None,
    })
    .unwrap_or(1.0)
}

fn opacity_from_drawingml_scheme_transforms(transforms: &[a::SchemeColorChoice]) -> f32 {
  transforms
    .iter()
    .find_map(|transform| match transform {
      a::SchemeColorChoice::Alpha(value) => drawingml_percent_to_ratio(&value.val),
      _ => None,
    })
    .unwrap_or(1.0)
}

struct ImportedImageData {
  data: Arc<[u8]>,
  content_type: Option<String>,
}

fn image_data_with_effects(
  resource: &package::ImageResource,
  properties: &DrawingImageProperties,
) -> ImportedImageData {
  if properties.effects == ImageEffects::default() {
    return ImportedImageData {
      data: resource.data.clone(),
      content_type: resource.content_type.clone(),
    };
  }

  let Some(data) = apply_image_effects(&resource.data, properties.effects) else {
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

fn apply_image_effects(data: &[u8], effects: ImageEffects) -> Option<Vec<u8>> {
  let mut image = image::load_from_memory(data).ok()?.to_rgba8();

  for pixel in image.pixels_mut() {
    let [mut r, mut g, mut b, a] = pixel.0;
    if effects.grayscale {
      let luminance = image_luminance(r, g, b);
      r = luminance;
      g = luminance;
      b = luminance;
    }
    if let Some((color1, color2)) = effects.duotone {
      let luminance = image_luminance(r, g, b) as u16;
      r = duotone_component(luminance, color1.r, color2.r);
      g = duotone_component(luminance, color1.g, color2.g);
      b = duotone_component(luminance, color1.b, color2.b);
    }
    if effects.brightness.is_some() || effects.contrast.is_some() {
      let brightness = effects.brightness.unwrap_or(0);
      let contrast = effects.contrast.unwrap_or(0);
      r = mso_brightness_contrast_component(r, brightness, contrast);
      g = mso_brightness_contrast_component(g, brightness, contrast);
      b = mso_brightness_contrast_component(b, brightness, contrast);
    }
    pixel.0 = [r, g, b, a];
  }

  let mut output = Vec::new();
  let encoder = PngEncoder::new(Cursor::new(&mut output));
  encoder
    .write_image(
      image.as_raw(),
      image.width(),
      image.height(),
      ColorType::Rgba8.into(),
    )
    .ok()?;
  Some(output)
}

fn image_luminance(r: u8, g: u8, b: u8) -> u8 {
  ((u16::from(b) * 29 + u16::from(g) * 151 + u16::from(r) * 76) >> 8) as u8
}

fn duotone_component(luminance: u16, color1: u8, color2: u8) -> u8 {
  let light = u16::from(color2) * luminance / 255;
  let dark = u16::from(color1) * (255 - luminance) / 255;
  (light + dark) as u8
}

fn mso_brightness_contrast_component(value: u8, brightness: i32, contrast: i32) -> u8 {
  let contrast = contrast.clamp(-100, 100) as f32;
  let slope = if contrast >= 0.0 {
    128.0 / (128.0 - 1.27 * contrast)
  } else {
    (128.0 + 1.27 * contrast) / 128.0
  };
  let offset = brightness.clamp(-100, 100) as f32 * 2.55;
  ((f32::from(value) + offset / 2.0 - 128.0) * slope + 128.0 + offset / 2.0)
    .round()
    .clamp(0.0, 255.0) as u8
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
  }
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
  })
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
    a::OutlineChoice::PatternFill(_) => return None,
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
  })
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
    a::OutlineChoice::PatternFill(_) => return None,
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
  for choice in &picture.picture_choice {
    push_picture_choice_shapes(choice, inlines, images);
  }
}

fn push_picture_choice_shapes(
  choice: &w::PictureChoice,
  inlines: &mut Vec<InlineItem>,
  images: &ImageCatalog,
) {
  match choice {
    w::PictureChoice::Group(group) => push_group_shapes(group, inlines, images),
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
      if let Some(shape) = vml_shape_shape(shape, images) {
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

fn push_group_shapes(group: &v::Group, inlines: &mut Vec<InlineItem>, images: &ImageCatalog) {
  let transform = VmlGroupTransform::from_group(group);
  for choice in &group.group_choice {
    match choice {
      v::GroupChoice::Group(group) => push_group_shapes(group, inlines, images),
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
        if let Some(shape) = vml_shape_shape_with_style(shape, style.as_deref(), images) {
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

fn vml_rectangle_shape(rectangle: &v::Rectangle, images: &ImageCatalog) -> Option<InlineShape> {
  vml_rectangle_shape_with_style(rectangle, rectangle.style.as_deref(), images)
}

fn vml_rectangle_shape_with_style(
  rectangle: &v::Rectangle,
  style: Option<&str>,
  images: &ImageCatalog,
) -> Option<InlineShape> {
  vml_inline_shape(
    style,
    vml_allow_in_cell(rectangle.allow_in_cell),
    rectangle.fill_color.as_deref(),
    vml_rectangle_fill_image(rectangle, images),
    rectangle.stroke_color.as_deref(),
    rectangle.stroke_weight.as_deref(),
    None,
  )
}

fn vml_round_rectangle_shape(round_rectangle: &v::RoundRectangle) -> Option<InlineShape> {
  vml_round_rectangle_shape_with_style(round_rectangle, round_rectangle.style.as_deref())
}

fn vml_round_rectangle_shape_with_style(
  round_rectangle: &v::RoundRectangle,
  style: Option<&str>,
) -> Option<InlineShape> {
  let filled = round_rectangle.filled.is_none_or(|value| value.as_bool());
  let stroked = round_rectangle.stroked.is_none_or(|value| value.as_bool());
  vml_inline_shape(
    style,
    vml_allow_in_cell(round_rectangle.allow_in_cell),
    filled
      .then_some(round_rectangle.fill_color.as_deref())
      .flatten(),
    None,
    stroked
      .then_some(round_rectangle.stroke_color.as_deref())
      .flatten(),
    round_rectangle.stroke_weight.as_deref(),
    None,
  )
}

fn vml_shape_shape(shape: &v::Shape, images: &ImageCatalog) -> Option<InlineShape> {
  vml_shape_shape_with_style(shape, shape.style.as_deref(), images)
}

fn vml_shape_shape_with_style(
  shape: &v::Shape,
  style: Option<&str>,
  images: &ImageCatalog,
) -> Option<InlineShape> {
  let has_path = shape
    .edge_path
    .as_deref()
    .is_some_and(|path| !path.trim().is_empty());
  let stroked = shape.stroked.is_none_or(|value| value.as_bool());
  vml_inline_shape(
    style,
    vml_allow_in_cell(shape.allow_in_cell),
    shape.fill_color.as_deref(),
    vml_shape_fill_image(shape, images),
    shape
      .stroke_color
      .as_deref()
      .or_else(|| (has_path && stroked).then_some("black"))
      .or_else(|| vml_shape_has_textbox(shape).then_some("black")),
    shape.stroke_weight.as_deref(),
    vml_fontwork_shape_geometry(shape.r#type.as_deref(), shape.id.as_deref()),
  )
}

fn vml_shape_has_textbox(shape: &v::Shape) -> bool {
  shape
    .shape_choice
    .iter()
    .any(|choice| matches!(choice, v::ShapeChoice::TextBox(_)))
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
  is_legacy_fontwork.then(fontwork_warp_geometry)
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
  let closed = points
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
    })
  } else {
    None
  };
  if fill_color.is_none() && stroke.is_none() {
    return None;
  }
  let mut style = vml_image_style(polyline.style.as_deref());
  style.layout_in_cell = vml_allow_in_cell(polyline.allow_in_cell);
  Some(InlineShape {
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
    fill_color,
    additional_fill_colors: Vec::new(),
    fill_image: None,
    stroke,
    suppress_zero_relative_background: false,
    allow_outside_page: style.absolute_position,
    inline_anchor_after_line: false,
    placement: style.placement(),
    text_box_blocks: Vec::new(),
    text_inset_left_pt: 0.0,
    text_inset_top_pt: 0.0,
    text_inset_right_pt: 0.0,
    text_inset_bottom_pt: 0.0,
    text_box_auto_fit: false,
    text_vertical_alignment: TextBoxVerticalAlignment::Top,
  })
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

fn vml_shape_fill_image(shape: &v::Shape, images: &ImageCatalog) -> Option<InlineShapeImageFill> {
  shape.shape_choice.iter().find_map(|choice| match choice {
    v::ShapeChoice::Fill(fill) => vml_fill_image(fill, shape.style.as_deref(), images),
    _ => None,
  })
}

fn vml_fill_image(
  fill: &v::Fill,
  shape_style: Option<&str>,
  images: &ImageCatalog,
) -> Option<InlineShapeImageFill> {
  let relationship_id = fill.relationship_id.as_ref().or(fill.id.as_ref())?;
  let resource = images.by_relationship_id.get(relationship_id)?;
  let style = vml_image_style(shape_style);

  Some(InlineShapeImageFill {
    data: resource.data.clone(),
    content_type: resource.content_type.clone(),
    crop: ImageCrop::default(),
    rotation_deg: style.rotation_deg,
    flip_horizontal: style.flip_horizontal,
    flip_vertical: style.flip_vertical,
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
  if vml_style_is_hidden(style) {
    return None;
  }

  let mut style = vml_image_style(style);
  style.layout_in_cell = layout_in_cell;
  let (width_pt, height_pt) = style.size_pt?;
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
    });
  if fill_color.is_none() && fill_image.is_none() && stroke.is_none() {
    return None;
  }

  Some(InlineShape {
    width_pt,
    height_pt,
    effect_left_pt: 0.0,
    effect_top_pt: 0.0,
    effect_right_pt: 0.0,
    effect_bottom_pt: 0.0,
    geometry: geometry_override.unwrap_or(InlineShapeGeometry::Rectangle),
    offset_x_pt: 0.0,
    offset_y_pt: 0.0,
    fill_color,
    additional_fill_colors: Vec::new(),
    fill_image,
    stroke,
    suppress_zero_relative_background: false,
    allow_outside_page: style.absolute_position,
    inline_anchor_after_line: false,
    placement: style.placement(),
    text_box_blocks: Vec::new(),
    text_inset_left_pt: 0.0,
    text_inset_top_pt: 0.0,
    text_inset_right_pt: 0.0,
    text_inset_bottom_pt: 0.0,
    text_box_auto_fit: false,
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
    fill_color: None,
    additional_fill_colors: Vec::new(),
    fill_image: None,
    stroke: None,
    suppress_zero_relative_background: false,
    allow_outside_page: style.absolute_position,
    inline_anchor_after_line: false,
    placement: style.placement(),
    text_box_blocks: frame.blocks,
    text_inset_left_pt: 0.0,
    text_inset_top_pt: 0.0,
    text_inset_right_pt: 0.0,
    text_inset_bottom_pt: 0.0,
    text_box_auto_fit: auto_fit,
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

fn parse_vml_color(value: &str) -> Option<RgbColor> {
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
    "black" => Some(RgbColor { r: 0, g: 0, b: 0 }),
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
  let &(first_x, first_y) = points.first()?;
  let mut min_x = first_x;
  let mut min_y = first_y;
  let mut max_x = first_x;
  let mut max_y = first_y;
  for &(x, y) in points {
    min_x = min_x.min(x);
    min_y = min_y.min(y);
    max_x = max_x.max(x);
    max_y = max_y.max(y);
  }
  Some((min_x, min_y, max_x, max_y))
}

fn pict_image_impl(picture: &w::Picture, images: &ImageCatalog) -> Option<InlineImage> {
  picture
    .picture_choice
    .iter()
    .find_map(|choice| picture_choice_image(choice, images))
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
    w::PictureChoice::RoundRectangle(_) => None,
    w::PictureChoice::Shape(shape) => shape_image(shape, images),
    _ => None,
  }
}

fn embedded_object_image(object: &w::EmbeddedObject, images: &ImageCatalog) -> Option<InlineImage> {
  object
    .embedded_object_choice1
    .iter()
    .find_map(|choice| match choice {
      w::EmbeddedObjectChoice::Group(group) => group_image(group, images),
      w::EmbeddedObjectChoice::ImageFile(image) => image_file_image(image, images),
      w::EmbeddedObjectChoice::Rectangle(rectangle) => rectangle_image(rectangle, images),
      w::EmbeddedObjectChoice::Shape(shape) => shape_image(shape, images),
      _ => None,
    })
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
  group.group_choice.iter().find_map(|choice| match choice {
    v::GroupChoice::Group(group) => group_image(group, images),
    v::GroupChoice::ImageFile(image) => image_file_image(image, images),
    v::GroupChoice::Rectangle(rectangle) => rectangle_image(rectangle, images),
    v::GroupChoice::RoundRectangle(_) => None,
    v::GroupChoice::Shape(shape) => shape_image(shape, images),
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
  let transform = VmlGroupTransform::from_group(group);
  for choice in &group.group_choice {
    match choice {
      v::GroupChoice::Group(group) => {
        push_group_textboxes(
          group,
          inlines,
          base_style.clone(),
          styles,
          images,
          hyperlinks,
        );
      }
      v::GroupChoice::ImageFile(image) => {
        let style = transform.and_then(|transform| transform.child_style(image.style.as_deref()));
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
        let style =
          transform.and_then(|transform| transform.child_style(rectangle.style.as_deref()));
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
        let style =
          transform.and_then(|transform| transform.child_style(round_rectangle.style.as_deref()));
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
        let style = transform.and_then(|transform| transform.child_style(shape.style.as_deref()));
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
  if vml_style_is_hidden(image.style.as_deref()) {
    return None;
  }

  image
    .image_file_choice
    .iter()
    .find_map(|choice| match choice {
      v::ImageFileChoice::ImageData(data) => vml_image_data(
        data,
        image.style.as_deref(),
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
  if vml_style_is_hidden(rectangle.style.as_deref()) {
    return None;
  }

  rectangle
    .rectangle_choice
    .iter()
    .find_map(|choice| match choice {
      v::RectangleChoice::ImageData(data) => vml_image_data(
        data,
        rectangle.style.as_deref(),
        vml_allow_in_cell(rectangle.allow_in_cell),
        rectangle.alternate.clone(),
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
  if vml_style_is_hidden(shape.style.as_deref()) {
    return None;
  }

  shape.shape_choice.iter().find_map(|choice| match choice {
    v::ShapeChoice::ImageData(data) => vml_image_data(
      data,
      shape.style.as_deref(),
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
    alt_text: alt_text.or_else(|| data.title.clone()),
    hyperlink_url: None,
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
  vertical_relative_to: VerticalImageReference,
  vertical_alignment: Option<VerticalImageAlignment>,
  horizontal_offset_pt: f32,
  vertical_offset_pt: f32,
  wrap: ImageWrapMode,
  behind_text: bool,
  layout_in_cell: bool,
  margin_top_pt: f32,
  margin_right_pt: f32,
  margin_bottom_pt: f32,
  margin_left_pt: f32,
}

#[derive(Clone, Copy, Debug)]
struct VmlGroupTransform {
  origin_x: f32,
  origin_y: f32,
  scale_x: f32,
  scale_y: f32,
}

impl VmlGroupTransform {
  fn from_group(group: &v::Group) -> Option<Self> {
    let style = vml_image_style(group.style.as_deref());
    let (width_pt, height_pt) = style.size_pt?;
    let (coord_width, coord_height) = vml_coordinate_pair(group.coordinate_size.as_deref())?;
    if coord_width.abs() <= f32::EPSILON || coord_height.abs() <= f32::EPSILON {
      return None;
    }
    let (origin_x, origin_y) =
      vml_coordinate_pair(group.coordinate_origin.as_deref()).unwrap_or((0.0, 0.0));

    Some(Self {
      origin_x,
      origin_y,
      scale_x: width_pt / coord_width,
      scale_y: height_pt / coord_height,
    })
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
        "left" => vml_raw_coordinate(value).map(|coord| (coord - self.origin_x) * self.scale_x),
        "top" => vml_raw_coordinate(value).map(|coord| (coord - self.origin_y) * self.scale_y),
        "width" => vml_raw_coordinate(value).map(|coord| coord * self.scale_x),
        "height" => vml_raw_coordinate(value).map(|coord| coord * self.scale_y),
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
    if !parent.absolute_position {
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
    output.push(vml_horizontal_reference_style(parent.horizontal_relative_to).to_string());
    output.push(vml_vertical_reference_style(parent.vertical_relative_to).to_string());
    if parent.behind_text {
      output.push("z-index:-1".to_string());
    }
    Some(output.join(";"))
  }
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
      vertical_relative_to: VerticalImageReference::Paragraph,
      vertical_alignment: None,
      horizontal_offset_pt: 0.0,
      vertical_offset_pt: 0.0,
      wrap: ImageWrapMode::Square,
      behind_text: false,
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
        horizontal_alignment: None,
        vertical_alignment: self.vertical_alignment,
        horizontal_offset_pt: self.horizontal_offset_pt,
        vertical_offset_pt: self.vertical_offset_pt,
        wrap: self.wrap,
        wrap_side: ImageWrapSide::BothSides,
        behind_text: self.behind_text,
        layout_in_cell: self.layout_in_cell,
        allow_overlap: true,
        relative_height: 0,
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
        output.behind_text = value.trim().parse::<i32>().is_ok_and(|value| value < 0);
        output.absolute_position = true;
      }
      "mso-position-horizontal-relative" => {
        output.horizontal_relative_to = vml_horizontal_reference(value);
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
      "mso-wrap-style" => output.wrap = vml_wrap_mode(value),
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

fn vml_measure_to_points(value: &str) -> Option<f32> {
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
  hyperlink_relationship_id: Option<String>,
  crop: ImageCrop,
  effects: ImageEffects,
  rotation_deg: f32,
  flip_horizontal: bool,
  flip_vertical: bool,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct ImageEffects {
  grayscale: bool,
  brightness: Option<i32>,
  contrast: Option<i32>,
  duotone: Option<(RgbColor, RgbColor)>,
}

fn drawing_image_properties(
  graphic_data: &ooxmlsdk::schemas::a::GraphicData,
  theme_colors: &ThemeColors,
) -> Option<DrawingImageProperties> {
  if graphic_data.uri != "http://schemas.openxmlformats.org/drawingml/2006/picture" {
    return None;
  }
  graphic_data.graphic_data_choice.iter().find_map(|choice| {
    if let a::GraphicDataChoice::Picture(picture) = choice {
      drawing_picture_image_properties(picture, theme_colors)
    } else {
      None
    }
  })
}

fn drawing_picture_image_properties(
  picture: &pic::Picture,
  theme_colors: &ThemeColors,
) -> Option<DrawingImageProperties> {
  let blip_fill = picture.blip_fill.as_deref()?;
  let mut properties = DrawingImageProperties {
    relationship_id: blip_fill.blip.as_ref()?.embed.clone(),
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
    ..DrawingImageProperties::default()
  };

  if let Some(crop) = blip_fill
    .blip_fill_choice
    .as_ref()
    .and_then(|choice| match choice {
      pic::BlipFillChoice::Stretch(stretch) => stretch.fill_rectangle.as_ref(),
      pic::BlipFillChoice::Tile(_) => None,
    })
    .map(image_crop_from_fill_rectangle)
    .or_else(|| {
      blip_fill
        .source_rectangle
        .as_ref()
        .map(image_crop_from_source_rectangle)
    })
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
    apply_image_effects_from_blip(&mut properties, blip, theme_colors);
  }

  Some(properties)
}

fn drawing_blip_fill_image_properties(
  blip_fill: &a::BlipFill,
  theme_colors: &ThemeColors,
) -> Option<DrawingImageProperties> {
  let mut properties = DrawingImageProperties {
    relationship_id: blip_fill.blip.as_ref()?.embed.clone(),
    ..DrawingImageProperties::default()
  };

  if let Some(crop) = blip_fill
    .blip_fill_choice
    .as_ref()
    .and_then(|choice| match choice {
      a::BlipFillChoice::Stretch(stretch) => stretch.fill_rectangle.as_ref(),
      a::BlipFillChoice::Tile(_) => None,
    })
    .map(image_crop_from_fill_rectangle)
    .or_else(|| {
      blip_fill
        .source_rectangle
        .as_ref()
        .map(image_crop_from_source_rectangle)
    })
  {
    properties.crop = crop;
  }

  if let Some(blip) = blip_fill.blip.as_ref() {
    apply_image_effects_from_blip(&mut properties, blip, theme_colors);
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
) {
  for choice in &blip.blip_choice {
    match choice {
      a::BlipChoice::Grayscale => properties.effects.grayscale = true,
      a::BlipChoice::LuminanceEffect(luminance) => {
        properties.effects.brightness = luminance
          .brightness
          .as_ref()
          .and_then(drawingml_percent_to_ratio)
          .map(drawingml_ratio_to_effect_percent);
        properties.effects.contrast = luminance
          .contrast
          .as_ref()
          .and_then(drawingml_percent_to_ratio)
          .map(drawingml_ratio_to_effect_percent);
      }
      a::BlipChoice::Duotone(duotone) => {
        if let Some(colors) = image_duotone_colors_from_model(duotone, theme_colors) {
          properties.effects.duotone = Some(colors);
        }
      }
      _ => {}
    }
  }
}

fn drawingml_ratio_to_effect_percent(value: f32) -> i32 {
  (value * 100.0).round() as i32
}

fn image_duotone_colors_from_model(
  duotone: &a::Duotone,
  theme_colors: &ThemeColors,
) -> Option<(RgbColor, RgbColor)> {
  let colors = duotone
    .duotone_choice
    .iter()
    .filter_map(|choice| drawingml_duotone_color(choice, theme_colors))
    .collect::<Vec<_>>();
  Some((colors.first().copied()?, colors.get(1).copied()?))
}

fn drawingml_duotone_color(
  choice: &a::DuotoneChoice,
  theme_colors: &ThemeColors,
) -> Option<RgbColor> {
  match choice {
    a::DuotoneChoice::RgbColorModelHex(color) => parse_hex_color(color.val.as_str()),
    a::DuotoneChoice::SchemeColor(color) => resolve_drawingml_scheme_color(color, theme_colors),
    a::DuotoneChoice::PresetColor(color) => drawingml_preset_color_value(color.val),
    _ => None,
  }
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
          let mut hsl = HslColor::from_rgb(resolved);
          hsl.apply_saturation_mod(amount);
          resolved = hsl.to_rgb();
        }
      }
      a::SchemeColorChoice::LuminanceModulation(value) => {
        if let Some(amount) = drawingml_percent_to_ratio(&value.val) {
          let mut hsl = HslColor::from_rgb(resolved);
          hsl.apply_luminance_mod(amount);
          resolved = hsl.to_rgb();
        }
      }
      a::SchemeColorChoice::LuminanceOffset(value) => {
        if let Some(amount) = drawingml_percent_to_ratio(&value.val) {
          let mut hsl = HslColor::from_rgb(resolved);
          hsl.apply_luminance_offset(amount);
          resolved = hsl.to_rgb();
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
  doc_default_paragraph: ParagraphFormat,
  doc_default_run: TextStyle,
  default_paragraph_style_id: Option<String>,
  theme_fonts: ThemeFonts,
  theme_colors: ThemeColors,
  theme_lines: ThemeLineStyles,
  styles: HashMap<String, StyleEntry>,
}

#[derive(Clone, Debug, Default)]
struct ThemeData {
  fonts: ThemeFonts,
  colors: ThemeColors,
  lines: ThemeLineStyles,
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
}

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

#[derive(Clone, Copy, Debug)]
pub(super) struct ResolvedColor {
  pub color: RgbColor,
  pub opacity: f32,
}

#[derive(Clone, Debug, Default)]
struct StyleEntry {
  style_type: Option<w::StyleValues>,
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
  bold: Option<bool>,
  italic: Option<bool>,
  underline: Option<bool>,
  strikethrough: Option<bool>,
  uppercase: Option<bool>,
  small_caps: Option<bool>,
  hidden: Option<bool>,
}

#[derive(Clone, Debug, Default)]
struct TableStyleModel {
  table_borders: Option<TableBordersModel>,
  cell_margins: Option<CellMargins>,
  cell_spacing_pt: Option<f32>,
  indent_left_pt: Option<f32>,
  alignment: Option<TableAlignment>,
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
}

#[derive(Clone, Debug, Default)]
struct TableCellStyle {
  shading: Option<RgbColor>,
  borders: CellBordersModel,
  margins: Option<CellMargins>,
  vertical_alignment: Option<TableCellVerticalAlignment>,
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
  table_style: &'a TableStyleModel,
  table_look: TableLookModel,
  row_count: usize,
  nested_table_level: usize,
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
    let Some(styles_part) = main.style_definitions_part(package) else {
      let mut catalog = Self {
        import_settings,
        theme_fonts: theme.fonts,
        theme_colors: theme.colors,
        theme_lines: theme.lines,
        ..Self::default()
      };
      if catalog.doc_default_run.font_family.is_none() {
        catalog.doc_default_run.font_family = Some(office_default_font_family(ui_language));
      }
      return Ok(catalog);
    };
    let styles = styles_part.root_element(package)?;
    let mut catalog = Self {
      import_settings,
      theme_fonts: theme.fonts,
      theme_colors: theme.colors,
      theme_lines: theme.lines,
      ..Self::default()
    };

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
      properties::merge_run_style(
        &mut catalog.doc_default_run,
        defaults
          .run_properties_default
          .as_deref()
          .and_then(|default| default.run_properties_base_style.as_deref())
          .map(RunProps::BaseStyle),
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
      let mut entry = StyleEntry {
        style_type: style.r#type,
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
        .or_else(|| Some(office_default_font_family(ui_language)));
    }

    Ok(catalog)
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

  fn run_style_with_base(
    &self,
    style_id: Option<&str>,
    base_style: TextStyle,
    base_overrides: RunStyleOverrides,
  ) -> TextStyle {
    let mut style = self.doc_default_run.clone();
    merge_style_values(&mut style, &base_style);
    apply_run_style_overrides(&mut style, base_overrides);
    let style_id = style_id.or(self.default_paragraph_style_id.as_deref());
    for entry in self.style_chain(style_id) {
      merge_style_values(&mut style, &entry.run_style);
      apply_run_style_overrides(&mut style, entry.run_overrides);
    }
    style
  }

  fn character_run_style(&self, style_id: Option<&str>, base_style: TextStyle) -> TextStyle {
    let Some(style_id) = style_id else {
      return base_style;
    };
    let mut style = base_style;
    let mut matched = false;
    for entry in self.style_chain(Some(style_id)) {
      if matches!(entry.style_type, Some(w::StyleValues::Character)) {
        matched = true;
        merge_style_values(&mut style, &entry.run_style);
        apply_run_style_overrides(&mut style, entry.run_overrides);
      }
    }
    if !matched {
      merge_builtin_character_style(&mut style, style_id);
    }
    style
  }

  fn style_ref_keys(&self, style_id: &str) -> Vec<Arc<str>> {
    let mut keys = Vec::new();
    push_unique_style_ref_key(&mut keys, style_id);
    if let Some(entry) = self.styles.get(style_id) {
      if let Some(name) = &entry.name {
        push_unique_style_ref_key(&mut keys, name);
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

fn office_default_font_family(ui_language: Option<&str>) -> Arc<str> {
  let language = ui_language.unwrap_or_default().to_ascii_lowercase();
  // Microsoft documents DengXian as the Office 2016+ default font for the
  // Simplified Chinese editions of Word, Excel, and PowerPoint. Keep this
  // fallback below explicit document styles and theme fonts.
  if language == "zh-cn"
    || language == "zh-sg"
    || language == "zh-hans"
    || language.starts_with("zh-hans-")
  {
    Arc::from("DengXian")
  } else {
    Arc::from("Calibri")
  }
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
    let Some(theme_part) = main.theme_part(package) else {
      return Self::default();
    };
    let Ok(theme) = theme_part.root_element(package) else {
      return Self::default();
    };
    Self {
      fonts: ThemeFonts::from_theme(theme),
      colors: ThemeColors::from_theme(theme),
      lines: ThemeLineStyles::from_theme(theme),
    }
  }
}

impl ThemeFonts {
  fn from_theme(theme: &a::Theme) -> Self {
    let scheme = &theme.theme_elements.font_scheme;
    Self {
      major_ascii: major_font_family(&scheme.major_font.latin_font.typeface),
      major_high_ansi: major_font_family(&scheme.major_font.latin_font.typeface),
      major_east_asia: major_font_family(&scheme.major_font.east_asian_font.typeface),
      major_bidi: major_font_family(&scheme.major_font.complex_script_font.typeface),
      minor_ascii: major_font_family(&scheme.minor_font.latin_font.typeface),
      minor_high_ansi: major_font_family(&scheme.minor_font.latin_font.typeface),
      minor_east_asia: major_font_family(&scheme.minor_font.east_asian_font.typeface),
      minor_bidi: major_font_family(&scheme.minor_font.complex_script_font.typeface),
    }
  }

  fn resolve(&self, value: Option<w::ThemeFontValues>) -> Option<Arc<str>> {
    match value? {
      w::ThemeFontValues::MajorAscii => self.major_ascii.clone(),
      w::ThemeFontValues::MajorHighAnsi => self.major_high_ansi.clone(),
      w::ThemeFontValues::MajorEastAsia => self.major_east_asia.clone(),
      w::ThemeFontValues::MajorBidi => self.major_bidi.clone(),
      w::ThemeFontValues::MinorAscii => self.minor_ascii.clone(),
      w::ThemeFontValues::MinorHighAnsi => self.minor_high_ansi.clone(),
      w::ThemeFontValues::MinorEastAsia => self.minor_east_asia.clone(),
      w::ThemeFontValues::MinorBidi => self.minor_bidi.clone(),
    }
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
  let mut hsl = HslColor::from_rgb(color);
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
  hsl.to_rgb()
}

fn apply_word_tint(color: RgbColor, tint: &str) -> RgbColor {
  let Some(tint) = u8::from_str_radix(tint, 16).ok() else {
    return color;
  };
  let mut hsl = HslColor::from_rgb(color);
  hsl.apply_tint(1.0 - (tint as f32 / units::BYTE_MAX_AS_FLOAT));
  hsl.to_rgb()
}

fn apply_word_shade(color: RgbColor, shade: &str) -> RgbColor {
  let Some(shade) = u8::from_str_radix(shade, 16).ok() else {
    return color;
  };
  let mut hsl = HslColor::from_rgb(color);
  hsl.apply_shade(shade as f32 / units::BYTE_MAX_AS_FLOAT);
  hsl.to_rgb()
}

#[derive(Clone, Copy, Debug)]
struct HslColor {
  hue: f32,
  saturation: f32,
  luminance: f32,
}

impl HslColor {
  fn from_rgb(color: RgbColor) -> Self {
    let red = color.r as f32 / units::BYTE_MAX_AS_FLOAT;
    let green = color.g as f32 / units::BYTE_MAX_AS_FLOAT;
    let blue = color.b as f32 / units::BYTE_MAX_AS_FLOAT;
    let max = red.max(green.max(blue));
    let min = red.min(green.min(blue));
    let luminance = (max + min) / 2.0;

    if (max - min).abs() < f32::EPSILON {
      return Self {
        hue: 0.0,
        saturation: 0.0,
        luminance,
      };
    }

    let delta = max - min;
    let saturation = if luminance > 0.5 {
      delta / (2.0 - max - min)
    } else {
      delta / (max + min)
    };
    let hue = if (max - red).abs() < f32::EPSILON {
      ((green - blue) / delta).rem_euclid(6.0)
    } else if (max - green).abs() < f32::EPSILON {
      ((blue - red) / delta) + 2.0
    } else {
      ((red - green) / delta) + 4.0
    } / 6.0;

    Self {
      hue,
      saturation,
      luminance,
    }
  }

  fn to_rgb(self) -> RgbColor {
    if self.saturation <= f32::EPSILON {
      let value = (self.luminance * units::BYTE_MAX_AS_FLOAT).round() as u8;
      return RgbColor {
        r: value,
        g: value,
        b: value,
      };
    }

    let q = if self.luminance < 0.5 {
      self.luminance * (1.0 + self.saturation)
    } else {
      self.luminance + self.saturation - self.luminance * self.saturation
    };
    let p = 2.0 * self.luminance - q;

    RgbColor {
      r: hue_to_rgb(p, q, self.hue + (1.0 / 3.0)),
      g: hue_to_rgb(p, q, self.hue),
      b: hue_to_rgb(p, q, self.hue - (1.0 / 3.0)),
    }
  }

  fn apply_tint(&mut self, amount: f32) {
    self.luminance = (self.luminance * (1.0 - amount) + amount).clamp(0.0, 1.0);
  }

  fn apply_shade(&mut self, amount: f32) {
    self.luminance = (self.luminance * amount).clamp(0.0, 1.0);
  }

  fn apply_saturation_mod(&mut self, amount: f32) {
    self.saturation = (self.saturation * amount).clamp(0.0, 1.0);
  }

  fn apply_luminance_mod(&mut self, amount: f32) {
    self.luminance = (self.luminance * amount).clamp(0.0, 1.0);
  }

  fn apply_luminance_offset(&mut self, amount: f32) {
    self.luminance = (self.luminance + amount).clamp(0.0, 1.0);
  }
}

fn hue_to_rgb(p: f32, q: f32, mut hue: f32) -> u8 {
  if hue < 0.0 {
    hue += 1.0;
  } else if hue > 1.0 {
    hue -= 1.0;
  }

  let value = if hue < (1.0 / 6.0) {
    p + (q - p) * 6.0 * hue
  } else if hue < 0.5 {
    q
  } else if hue < (2.0 / 3.0) {
    p + (q - p) * ((2.0 / 3.0) - hue) * 6.0
  } else {
    p
  };

  (value.clamp(0.0, 1.0) * units::BYTE_MAX_AS_FLOAT).round() as u8
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
    if let Some(properties) = conditional
      .table_style_conditional_formatting_table_properties
      .as_deref()
    {
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
    shading: properties.shading.as_ref().and_then(shading_fill),
    borders: CellBordersModel::default(),
    margins: properties
      .table_cell_margin
      .as_deref()
      .map(|margins| table_cell_margin(margins, CellMargins::default())),
    vertical_alignment: properties
      .table_cell_vertical_alignment
      .as_ref()
      .map(table_cell_vertical_alignment),
    ..Default::default()
  }
}

fn conditional_table_cell_style(
  properties: &w::TableStyleConditionalFormattingTableCellProperties,
) -> TableCellStyle {
  TableCellStyle {
    shading: properties.shading.as_ref().and_then(shading_fill),
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
) -> TableStyleModel {
  TableStyleModel {
    table_borders: borders.map(table_borders_model),
    cell_margins: margins.map(table_cell_margin_default),
    cell_spacing_pt: spacing.and_then(table_cell_spacing_to_points),
    indent_left_pt: indentation.and_then(table_indentation_to_points),
    alignment: justification.map(table_alignment),
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
  )
}

fn merge_table_level_style(target: &mut TableStyleModel, source: &TableStyleModel) {
  if source.table_borders.is_some() {
    target.table_borders = source.table_borders;
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
  }
}

fn apply_run_style_overrides(style: &mut TextStyle, overrides: RunStyleOverrides) {
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
}

fn merge_format_values(target: &mut ParagraphFormat, values: &ParagraphFormat) {
  if values.spacing_before_set || values.spacing_before_pt != 0.0 {
    target.spacing_before_pt = values.spacing_before_pt;
    target.spacing_before_set = values.spacing_before_set;
  }
  if values.spacing_after_set || values.spacing_after_pt != 0.0 {
    target.spacing_after_pt = values.spacing_after_pt;
    target.spacing_after_set = values.spacing_after_set;
  }
  if values.line_height_pt.is_some() {
    target.line_height_pt = values.line_height_pt;
    target.line_height_rule = values.line_height_rule;
  }
  if values.snap_to_grid.is_some() {
    target.snap_to_grid = values.snap_to_grid;
  }
  if values.indent_left_set {
    target.indent_left_pt = values.indent_left_pt;
    target.indent_left_set = true;
  }
  if values.indent_right_set {
    target.indent_right_pt = values.indent_right_pt;
    target.indent_right_set = true;
  }
  if values.first_line_indent_set {
    target.first_line_indent_pt = values.first_line_indent_pt;
    target.first_line_indent_set = true;
  }
  if values.tab_stops_set {
    target.tab_stops.clone_from(&values.tab_stops);
    target.tab_stops_set = true;
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
  if values.page_break_before {
    target.page_break_before = true;
  }
  if values.keep_with_next {
    target.keep_with_next = true;
  }
  if values.keep_lines {
    target.keep_lines = true;
  }
  if values.contextual_spacing {
    target.contextual_spacing = true;
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

#[derive(Clone, Copy, Debug, Default)]
struct NumberingFormatMergeContext {
  direct_indentation: bool,
  direct_tab_stops: bool,
  style_numbering: bool,
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
  if values.spacing_after_set || values.spacing_after_pt != 0.0 {
    target.spacing_after_pt = values.spacing_after_pt;
    target.spacing_after_set = values.spacing_after_set;
  }
  if values.line_height_pt.is_some() {
    target.line_height_pt = values.line_height_pt;
    target.line_height_rule = values.line_height_rule;
  }
  if values.snap_to_grid.is_some() {
    target.snap_to_grid = values.snap_to_grid;
  }
  let protect_indents =
    (context.direct_indentation || context.style_numbering) && target.indent_left_set;
  if values.indent_left_set && !protect_indents {
    target.indent_left_pt = values.indent_left_pt;
    target.indent_left_set = true;
  }
  let protect_indents =
    (context.direct_indentation || context.style_numbering) && target.indent_right_set;
  if values.indent_right_set && !protect_indents {
    target.indent_right_pt = values.indent_right_pt;
    target.indent_right_set = true;
  }
  let protect_indents =
    (context.direct_indentation || context.style_numbering) && target.first_line_indent_set;
  if values.first_line_indent_set && !protect_indents {
    target.first_line_indent_pt = values.first_line_indent_pt;
    target.first_line_indent_set = true;
  }
  if values.tab_stops_set && !(context.direct_tab_stops && target.tab_stops_set) {
    target.tab_stops.clone_from(&values.tab_stops);
    target.tab_stops_set = true;
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
  if values.page_break_before {
    target.page_break_before = true;
  }
  if values.keep_with_next {
    target.keep_with_next = true;
  }
  if values.keep_lines {
    target.keep_lines = true;
  }
  if values.contextual_spacing {
    target.contextual_spacing = true;
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
  if (values.font_size_pt - TextStyle::default().font_size_pt).abs() > f32::EPSILON {
    target.font_size_pt = values.font_size_pt;
  }
  if values.complex_font_size_pt.is_some() {
    target.complex_font_size_pt = values.complex_font_size_pt;
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
  if values.color != TextStyle::default().color {
    target.color = values.color;
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
}

#[derive(Clone, Debug)]
struct NumberingLevel {
  start: i32,
  format: w::NumberFormatValues,
  text: String,
  suffix: NumberingSuffix,
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
  image: Option<InlineImage>,
  style: TextStyle,
  list_tab_stop_pt: Option<f32>,
}

impl NumberingCatalog {
  fn load(
    package: &mut WordprocessingDocument,
    main: &MainDocumentPart,
    import_settings: ImportSettings,
  ) -> Result<Self> {
    let Some(numbering_part) = main.numbering_definitions_part(package) else {
      return Ok(Self::default());
    };
    let numbering_images = ImageCatalog::load_from_numbering(package, &numbering_part);
    let numbering = numbering_part.root_element(package)?;
    let mut catalog = Self::default();

    for picture_bullet in &numbering.numbering_picture_bullet {
      if let Some(image) = numbering_picture_bullet_image(picture_bullet, &numbering_images) {
        catalog
          .picture_bullets
          .insert(picture_bullet.numbering_picture_bullet_id, image);
      }
    }

    for abstract_num in &numbering.abstract_num {
      let mut entry = AbstractNumbering::default();
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

    Ok(catalog)
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
    let abstract_num = self.abstract_nums.get(&instance.abstract_num_id)?;
    let level_override = instance.overrides.get(&level_index);
    let level = level_override
      .and_then(|override_| override_.level.as_ref())
      .or_else(|| abstract_num.levels.get(&level_index))?;

    merge_numbering_format_values(format, &level.format_properties, format_context);
    let start = level_override
      .and_then(|override_| override_.start)
      .unwrap_or(level.start);
    let counter = {
      let counter = self
        .counters
        .entry((num_id, level_index))
        .or_insert(start - 1);
      *counter += 1;
      *counter
    };
    for key_level in (level_index + 1)..=8 {
      self.counters.remove(&(num_id, key_level));
    }

    let text = format_numbering_label(
      level,
      num_id,
      level_index,
      counter,
      abstract_num,
      &self.counters,
    );
    let mut style = base_style;
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
    let image = level
      .picture_bullet_id
      .and_then(|id| self.picture_bullets.get(&id).cloned());
    Some(NumberingLabel {
      text: if image.is_some() { None } else { Some(text) },
      image,
      style,
      list_tab_stop_pt: level.list_tab_stop_pt,
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
      .unwrap_or(1),
    format: level
      .numbering_format
      .as_ref()
      .map(|format| format.val)
      .unwrap_or_default(),
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
    list_tab_stop_pt: numbering_level_list_tab_stop_pt(level),
    picture_bullet_id: level.level_picture_bullet_id.as_ref().map(|id| id.val),
    is_legal: level.is_legal_numbering_style.is_some(),
    format_properties,
    symbol_run_properties: level.numbering_symbol_run_properties.as_deref().cloned(),
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
) -> Option<InlineImage> {
  match picture_bullet.numbering_picture_bullet_choice.as_ref()? {
    w::NumberingPictureBulletChoice::PictureBulletBase(picture) => {
      picture_bullet_base_image(picture, images).map(normalize_picture_bullet_image_size)
    }
    w::NumberingPictureBulletChoice::Drawing(_) => None,
  }
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
  num_id: i32,
  level_index: i32,
  value: i32,
  abstract_num: &AbstractNumbering,
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
    let value = if index == level_index {
      value
    } else {
      counters.get(&(num_id, index)).copied().unwrap_or_else(|| {
        abstract_num
          .levels
          .get(&index)
          .map(|level| level.start)
          .unwrap_or(1)
      })
    };
    let format = abstract_num
      .levels
      .get(&index)
      .map(|level| level.format)
      .unwrap_or_default();
    text = text.replace(
      &placeholder,
      &format_numbering_value(value, format, level.is_legal && index < level_index),
    );
  }
  format!("{text}{}", numbering_suffix_text(level.suffix))
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
    w::NumberFormatValues::DecimalZero => format!("{value:02}"),
    w::NumberFormatValues::DecimalEnclosedCircle
    | w::NumberFormatValues::DecimalEnclosedCircleChinese => enclosed_decimal_number(value, 0x2460),
    w::NumberFormatValues::DecimalEnclosedFullstop => enclosed_decimal_number(value, 0x2488),
    w::NumberFormatValues::DecimalEnclosedParen => enclosed_decimal_number(value, 0x2474),
    w::NumberFormatValues::DecimalFullWidth | w::NumberFormatValues::DecimalFullWidth2 => {
      full_width_decimal_number(value)
    }
    w::NumberFormatValues::DecimalHalfWidth => value.to_string(),
    w::NumberFormatValues::None => String::new(),
    _ => value.to_string(),
  }
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

  fn contextual_spacing(&self) -> Option<&'a w::ContextualSpacing> {
    match self {
      Self::Direct(properties) => properties.contextual_spacing.as_ref(),
      Self::Extended(properties) => properties.contextual_spacing.as_ref(),
      Self::Style(properties) => properties.contextual_spacing.as_ref(),
      Self::BaseStyle(properties) => properties.contextual_spacing.as_ref(),
      Self::Previous(properties) => properties.contextual_spacing.as_ref(),
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
run_properties_accessor!(run_properties_italic, Italic, w::Italic);
run_properties_accessor!(run_properties_font_size, FontSize, w::FontSize);
run_properties_accessor!(
  run_properties_complex_script_font_size,
  FontSizeComplexScript,
  w::FontSizeComplexScript
);
run_properties_accessor!(run_properties_color, Color, w::Color);
run_properties_accessor!(run_properties_underline, Underline, w::Underline);
run_properties_accessor!(run_properties_strike, Strike, w::Strike);
run_properties_accessor!(run_properties_double_strike, DoubleStrike, w::DoubleStrike);
run_properties_accessor!(run_properties_caps, Caps, w::Caps);
run_properties_accessor!(run_properties_small_caps, SmallCaps, w::SmallCaps);
run_properties_accessor!(run_properties_vanish, Vanish, w::Vanish);
run_properties_accessor!(
  run_properties_vertical_text_alignment,
  VerticalTextAlignment,
  w::VerticalTextAlignment
);
run_properties_accessor!(run_properties_spacing, Spacing, w::Spacing);
run_properties_accessor!(run_properties_highlight, Highlight, w::Highlight);

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
paragraph_mark_run_properties_accessor!(paragraph_mark_run_properties_italic, Italic, w::Italic);
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
  paragraph_mark_run_properties_highlight,
  Highlight,
  w::Highlight
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

  fn bold(&self) -> Option<&'a w::Bold> {
    match self {
      Self::Direct(properties) => run_properties_bold(properties),
      Self::Style(properties) => properties.bold.as_ref(),
      Self::BaseStyle(properties) => properties.bold.as_ref(),
      Self::Numbering(properties) => properties.bold.as_ref(),
      Self::ParagraphMark(properties) => paragraph_mark_run_properties_bold(properties),
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

  fn text_fill(&self) -> Option<&'a w14::FillTextEffect> {
    match self {
      Self::Direct(properties) => properties.fill_text_effect.as_deref(),
      Self::ParagraphMark(properties) => properties.fill_text_effect.as_deref(),
      Self::Style(_) | Self::BaseStyle(_) | Self::Numbering(_) => None,
    }
  }

  fn text_outline(&self) -> Option<&'a w14::TextOutlineEffect> {
    match self {
      Self::Direct(properties) => properties.text_outline_effect.as_deref(),
      Self::ParagraphMark(properties) => properties.text_outline_effect.as_deref(),
      Self::Style(_) | Self::BaseStyle(_) | Self::Numbering(_) => None,
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
  let mut setup = default_word_page_setup();

  if let Some(size) = &section.page_size {
    if let Some(width) = size.width.as_ref().and_then(twips_measure_to_points) {
      setup.width_pt = width;
    }
    if let Some(height) = size.height.as_ref().and_then(twips_measure_to_points) {
      setup.height_pt = height;
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

  setup
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
  let count_by = properties.count_by.unwrap_or(1);
  if count_by <= 0 {
    return None;
  }
  Some(LineNumbering {
    count_by,
    start: properties.start.unwrap_or(1),
    distance_pt: properties
      .distance
      .as_ref()
      .and_then(twips_measure_to_points)
      .unwrap_or(14.0),
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
      xml_content: Some(value.into()),
      ..Default::default()
    })
  }

  #[test]
  fn smart_tag_run_preserves_nested_visible_text() {
    let paragraph = w::Paragraph::from_bytes(
      br#"<w:p xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:smartTag w:uri="urn:test" w:element="person"><w:r><w:t>John</w:t></w:r><w:smartTag w:uri="urn:test" w:element="surname"><w:r><w:t> Smith</w:t></w:r></w:smartTag></w:smartTag></w:p>"#,
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
      math_paragraph_alignment(&paragraph),
      Some(ParagraphAlignment::Center)
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
  fn drawing_image_properties_preserve_crop_and_transform() {
    let xml = r#"<pic:pic xmlns:pic="http://schemas.openxmlformats.org/drawingml/2006/picture" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"><pic:nvPicPr><pic:cNvPr id="1" name="Picture 1"/><pic:cNvPicPr/></pic:nvPicPr><pic:blipFill><a:blip r:embed="rId7"/><a:srcRect l="10000" t="20000" r="30000" b="40000"/></pic:blipFill><pic:spPr><a:xfrm rot="5400000" flipH="1" flipV="true"/></pic:spPr></pic:pic>"#;

    let picture = pic::Picture::from_bytes(xml.as_bytes()).expect("picture");
    let properties = drawing_picture_image_properties(&picture, &ThemeColors::default())
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
  fn alternate_content_drawing_imports_choice_shape() {
    let xml = r#"<mc:AlternateContent xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006" xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:wp="http://schemas.openxmlformats.org/drawingml/2006/wordprocessingDrawing" xmlns:wps="http://schemas.microsoft.com/office/word/2010/wordprocessingShape" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"><mc:Choice Requires="wps"><w:drawing><wp:anchor behindDoc="1" distT="0" distB="0" distL="114300" distR="114300" simplePos="0" locked="0" layoutInCell="1" allowOverlap="1" relativeHeight="2"><wp:simplePos x="0" y="0"/><wp:positionH relativeFrom="page"><wp:posOffset>1080135</wp:posOffset></wp:positionH><wp:positionV relativeFrom="page"><wp:posOffset>1260475</wp:posOffset></wp:positionV><wp:extent cx="5760720" cy="0"/><wp:effectExtent l="0" t="0" r="0" b="0"/><wp:wrapNone/><wp:docPr id="1" name="Freeform 2"/><a:graphic><a:graphicData uri="http://schemas.microsoft.com/office/word/2010/wordprocessingShape"><wps:wsp><wps:cNvSpPr/><wps:spPr><a:custGeom><a:pathLst><a:path w="8504" h="0"><a:moveTo><a:pt x="0" y="0"/></a:moveTo><a:lnTo><a:pt x="8504" y="0"/></a:lnTo></a:path></a:pathLst></a:custGeom><a:noFill/><a:ln w="6480"><a:solidFill><a:srgbClr val="ff0101"/></a:solidFill></a:ln></wps:spPr><wps:bodyPr/></wps:wsp></a:graphicData></a:graphic></wp:anchor></w:drawing></mc:Choice><mc:Fallback><w:pict/></mc:Fallback></mc:AlternateContent>"#;
    let alternate_content = mc::AlternateContent::from_bytes(xml.as_bytes()).expect("MCE");
    let mut inlines = Vec::new();
    let styles = StylesCatalog::default();
    let images = ImageCatalog::default();
    let hyperlinks = HyperlinkCatalog::default();
    push_alternate_content_inlines(
      &alternate_content,
      &mut inlines,
      AlternateContentImportContext {
        base_style: TextStyle::default(),
        style: TextStyle::default(),
        styles: &styles,
        images: &images,
        hyperlinks: &hyperlinks,
        hyperlink_url: None,
      },
    );

    assert!(inlines
      .iter()
      .any(|inline| matches!(inline, InlineItem::Shape(shape) if shape.geometry == InlineShapeGeometry::Line)));
  }

  #[test]
  fn symbol_runs_emit_unicode_text() {
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

    assert_eq!(inline_text(&inlines), "•✓©");
  }

  #[test]
  fn table_cell_margins_default_to_word_side_padding() {
    let margins = CellMargins::default();

    assert_eq!(margins.top_pt, 0.0);
    assert_eq!(margins.bottom_pt, 0.0);
    assert!((margins.left_pt - 5.4).abs() < 0.001);
    assert!((margins.right_pt - 5.4).abs() < 0.001);
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
    assert!((margins.right_pt - 5.4).abs() < 0.001);
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
      Some(RgbColor {
        r: 0x44,
        g: 0x72,
        b: 0xC4
      })
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
      Some(RgbColor {
        r: 0xEE,
        g: 0xEE,
        b: 0xEE
      })
    );
  }

  #[test]
  fn table_style_column_and_corner_conditions_apply_by_cell_position() {
    fn style(fill: &str) -> TableCellStyle {
      TableCellStyle {
        shading: Some(parse_hex_color(fill).unwrap()),
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
      Some(RgbColor {
        r: 0xFF,
        g: 0x00,
        b: 0x00
      })
    );
    assert_eq!(
      body_right.shading,
      Some(RgbColor {
        r: 0x00,
        g: 0xFF,
        b: 0x00
      })
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
    let merged = direct_cell_borders_model(
      base,
      &w::TableCellBorders {
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
      },
    );

    assert_eq!(merged.top, None);
    assert_eq!(merged.right.unwrap().width_pt, 3.0);
    assert_eq!(merged.bottom, Some(border(2.0)));
    assert_eq!(merged.left, Some(border(2.5)));
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
        shading: Some(parse_hex_color(fill).unwrap()),
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
      Some(RgbColor {
        r: 0xFF,
        g: 0x00,
        b: 0x00
      })
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
      table_style: &TableStyleModel::default(),
      table_look: TableLookModel::default(),
      row_count: 1,
      nested_table_level: 1,
    };

    let cell = table_cell_model(&cell, &mut context, None, style);

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
    };

    push_simple_field(&field, &mut inlines, TextStyle::default(), &mut context);

    let InlineItem::Text(run) = &inlines[0] else {
      panic!("expected dynamic field text");
    };
    assert_eq!(run.dynamic_field, Some(DynamicFieldKind::Page));
  }

  #[test]
  fn pageref_field_instruction_emits_bookmark_page_marker() {
    assert_eq!(
      dynamic_field_kind(r#" PAGEREF "_Toc123" \h "#),
      Some(DynamicFieldKind::PageRef {
        bookmark_name: Arc::<str>::from("_Toc123"),
      })
    );
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
      })
    );
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
        },
        None,
        &mut complex_fields,
      );
    }

    assert_eq!(field_result_text(&inlines).as_deref(), Some("前言\t4"));
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
    assert_eq!(run.dynamic_field, Some(DynamicFieldKind::Page));
  }

  #[test]
  fn ruby_runs_emit_base_text() {
    let mut inlines = Vec::new();
    let ruby = w::Ruby {
      ruby_base: w::RubyBase {
        ruby_base_choice: vec![w::RubyBaseChoice::WRun(Box::new(w::Run {
          run_choice: vec![w::RunChoice::Text(text("漢"))],
          ..Default::default()
        }))],
      },
      ..Default::default()
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
  }

  #[test]
  fn vml_pict_runs_emit_images() {
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
        | InlineItem::Shape(_)
        | InlineItem::BookmarkStart(_)
        | InlineItem::FormWidgetStart(_)
        | InlineItem::FormWidgetEnd(_)
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
    assert_eq!(placement.wrap, ImageWrapMode::Square);
    assert!(placement.behind_text);
    assert!((placement.horizontal_offset_pt - 12.0).abs() < 0.001);
    assert!((placement.vertical_offset_pt - 18.0).abs() < 0.001);
    assert!((placement.margin_left_pt - 9.0).abs() < 0.001);
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
    inlines
      .iter()
      .filter_map(|item| match item {
        InlineItem::Text(run) => Some(run.text.as_str()),
        InlineItem::Image(_)
        | InlineItem::Shape(_)
        | InlineItem::BookmarkStart(_)
        | InlineItem::FormWidgetStart(_)
        | InlineItem::FormWidgetEnd(_)
        | InlineItem::LastRenderedPageBreak
        | InlineItem::PageBreak
        | InlineItem::ColumnBreak => None,
      })
      .collect()
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
