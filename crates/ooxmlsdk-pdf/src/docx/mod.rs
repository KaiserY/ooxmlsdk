mod drawing;
mod model;
mod package;
mod properties;
mod table;
mod text;

use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;

use ooxmlsdk::parts::{
  main_document_part::MainDocumentPart, wordprocessing_document::WordprocessingDocument,
};
use ooxmlsdk::schemas::{
  schemas_microsoft_com_office_word_2010_wordml as w14,
  schemas_microsoft_com_office_word_2010_wordprocessing_drawing as wp14,
  schemas_microsoft_com_vml as v, schemas_openxmlformats_org_drawingml_2006_main as a,
  schemas_openxmlformats_org_drawingml_2006_wordprocessing_drawing as wp,
  schemas_openxmlformats_org_wordprocessingml_2006_main as w,
};
use ooxmlsdk::simple_type::{
  MeasurementOrPercentValue, SignedTwipsMeasureValue, TwipsMeasureValue,
};
use quick_xml::Reader;
use quick_xml::Writer;
use quick_xml::events::Event;

use crate::error::Result;
use crate::options::PdfOptions;
use crate::units;

pub(crate) use model::*;
use package::{HyperlinkCatalog, ImageCatalog};
use table::{TableConditionalStyleMask, TableLookModel};
use text::{ParagraphImportBase, paragraph_model, paragraph_model_with_base};

const DEFAULT_TAB_STOP_PT: f32 = 36.0;
// Source: LibreOffice sw/source/writerfilter/dmapper/SectionColumnHandler.cxx
// initializes w:cols/@space to 720 twips.
const DEFAULT_SECTION_COLUMN_GAP_PT: f32 = 720.0 / units::TWIPS_PER_POINT;
const DEFAULT_TEXTBOX_MIN_WIDTH_PT: f32 = 11.0;
const DEFAULT_TEXTBOX_MIN_HEIGHT_PT: f32 = 14.0;
// LibreOffice oox/source/shape/WpsContext.cxx uses the OOXML spec defaults:
// left/right 91440 EMU, top/bottom 45720 EMU.
const DEFAULT_TEXTBOX_LEFT_RIGHT_INSET_PT: f32 = 91_440.0 / units::EMUS_PER_POINT;
const DEFAULT_TEXTBOX_TOP_BOTTOM_INSET_PT: f32 = 45_720.0 / units::EMUS_PER_POINT;
const WML_DEFAULT_BORDER_WIDTH_PT: f32 = 0.5;
const WML_MIN_BORDER_WIDTH_PT: f32 = 0.25;
const DRAWINGML_DEFAULT_LINE_WIDTH_EMU: i64 = 0;
const VML_DEFAULT_STROKE_WEIGHT_EMU: i64 = 1;
// Source: LibreOffice include/editeng/escapementitem.hxx and
// sw/source/writerfilter/dmapper/DomainMapper.cxx. DOCX vertAlign maps to
// DFLT_ESC_PROP with DFLT_ESC_SUPER / DFLT_ESC_SUB.
const LO_DEFAULT_ESCAPEMENT_HEIGHT_SCALE: f32 = 0.58;
const LO_SUPERSCRIPT_BASELINE_SHIFT_SCALE: f32 = 0.33;
const LO_SUBSCRIPT_BASELINE_SHIFT_SCALE: f32 = -0.08;
const MIN_ESCAPEMENT_FONT_SIZE_PT: f32 = 1.0;
const MIN_IMPORTED_LINE_HEIGHT_PT: f32 = 0.1;
const TAB_STOP_DEDUP_EPSILON_PT: f32 = 0.1;
const COMMENT_REFERENCE_FONT_SCALE: f32 = 0.75;
// Source: LibreOffice include/editeng/svxfont.hxx SMALL_CAPS_PERCENTAGE.
const LO_SMALL_CAPS_FONT_SCALE: f32 = 0.80;

pub(crate) fn extract(
  package: &mut WordprocessingDocument,
  _options: &PdfOptions,
) -> Result<DocxDocument> {
  let main = package.main_document_part()?;
  let styles = StylesCatalog::load(package, &main)?;
  let mut numbering = NumberingCatalog::load(package, &main)?;
  let images = ImageCatalog::load(package, &main);
  let hyperlinks = HyperlinkCatalog::load(package, &main);
  let mut form_widget_ids = FormWidgetIdAllocator::default();
  let default_tab_stop_pt = default_tab_stop_pt(package, &main);
  let even_and_odd_headers = even_and_odd_headers(package, &main);
  let mirror_margins = mirror_margins(package, &main);
  let document = main.root_element(package)?;
  let page_background = document
    .document_background
    .as_deref()
    .and_then(document_background_color);
  let mut sections = document
    .body
    .as_deref()
    .map(|body| {
      body_sections(
        body,
        &styles,
        &mut numbering,
        &images,
        &hyperlinks,
        &mut form_widget_ids,
      )
    })
    .unwrap_or_else(|| vec![default_section(Vec::new())]);
  let supplemental_graphic_blocks = supplemental_graphic_blocks(package, &main, &styles);
  if let Some(first_section) = sections.first_mut() {
    first_section
      .blocks
      .extend(supplemental_graphic_blocks.iter().cloned());
  }
  for section in &mut sections {
    section.page.background = page_background;
    section.page.mirror_margins = mirror_margins;
  }
  resolve_section_repeating_blocks(package, &main, &styles, &mut sections, &mut form_widget_ids);
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
  let footnotes = footnotes(package, &main, &styles, &mut form_widget_ids)?;
  let footnote_blocks = flatten_note_blocks(&footnotes);
  let endnotes = endnotes(package, &main, &styles, &mut form_widget_ids)?;
  let endnote_blocks = flatten_note_blocks(&endnotes);
  let comment_blocks = comment_blocks(package, &main, &styles, &mut form_widget_ids)?;
  let title_page = sections
    .first()
    .map(|section| section.title_page)
    .unwrap_or(false);
  let form_widgets = form_widget_ids.widgets().to_vec();

  Ok(DocxDocument {
    page,
    default_tab_stop_pt,
    even_and_odd_headers,
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
    comment_blocks,
    title_page,
    blocks,
  })
}

fn default_tab_stop_pt(package: &mut WordprocessingDocument, main: &MainDocumentPart) -> f32 {
  main
    .document_settings_part(package)
    .and_then(|part| part.root_element(package).ok())
    .and_then(|settings| {
      settings
        .w_default_tab_stop
        .as_ref()
        .and_then(|stop| twips_measure_to_points(&stop.val))
    })
    .filter(|value| value.is_finite() && *value > 0.0)
    .unwrap_or(DEFAULT_TAB_STOP_PT)
}

fn supplemental_graphic_blocks(
  package: &mut WordprocessingDocument,
  main: &MainDocumentPart,
  styles: &StylesCatalog,
) -> Vec<Block> {
  let mut blocks = chart_text_blocks(package, main, styles);
  blocks.extend(diagram_text_blocks(package, main, styles));
  blocks
}

fn chart_text_blocks(
  package: &WordprocessingDocument,
  main: &MainDocumentPart,
  styles: &StylesCatalog,
) -> Vec<Block> {
  let mut blocks = Vec::new();
  for chart_part in main.chart_parts(package) {
    let Some(xml) = chart_part
      .data_to_vec(package)
      .and_then(|data| String::from_utf8(data).ok())
    else {
      continue;
    };
    let Some(color) = chart_label_color(&xml, &styles.theme_colors) else {
      continue;
    };
    for text in chart_label_texts(&xml) {
      blocks.push(simple_text_block(
        text,
        text_style_with_color(styles, color),
      ));
    }
  }
  blocks
}

fn diagram_text_blocks(
  package: &WordprocessingDocument,
  main: &MainDocumentPart,
  styles: &StylesCatalog,
) -> Vec<Block> {
  let diagram_colors = main
    .diagram_colors_parts(package)
    .filter_map(|part| {
      part
        .data_to_vec(package)
        .and_then(|data| String::from_utf8(data).ok())
    })
    .collect::<Vec<_>>();
  let mut color_index = 0usize;
  let mut blocks = Vec::new();
  for drawing_part in main.diagram_persist_layout_parts(package) {
    let Some(xml) = drawing_part
      .data_to_vec(package)
      .and_then(|data| String::from_utf8(data).ok())
    else {
      continue;
    };
    let color = diagram_colors
      .get(color_index)
      .and_then(|colors_xml| diagram_text_color(colors_xml, &styles.theme_colors))
      .or(styles.theme_colors.dark2)
      .unwrap_or(RgbColor {
        r: 0x1F,
        g: 0x49,
        b: 0x7D,
      });
    color_index += 1;
    for text in diagram_shape_texts(&xml) {
      blocks.push(simple_text_block(
        text,
        text_style_with_color(styles, color),
      ));
    }
  }
  blocks
}

fn chart_label_color(xml: &str, theme_colors: &ThemeColors) -> Option<RgbColor> {
  let mut reader = Reader::from_str(xml);
  reader.config_mut().trim_text(false);

  let mut in_dlbls = false;
  let mut in_txpr = false;
  let mut in_defrpr = false;

  loop {
    match reader.read_event().ok()? {
      Event::Start(event) if qname_ends_with(event.name().as_ref(), b"dLbls") => in_dlbls = true,
      Event::End(event) if qname_ends_with(event.name().as_ref(), b"dLbls") => in_dlbls = false,
      Event::Start(event) if in_dlbls && qname_ends_with(event.name().as_ref(), b"txPr") => {
        in_txpr = true;
      }
      Event::End(event) if qname_ends_with(event.name().as_ref(), b"txPr") => in_txpr = false,
      Event::Start(event) if in_txpr && qname_ends_with(event.name().as_ref(), b"defRPr") => {
        in_defrpr = true;
      }
      Event::End(event) if qname_ends_with(event.name().as_ref(), b"defRPr") => {
        in_defrpr = false;
      }
      Event::Start(event) if in_defrpr && qname_ends_with(event.name().as_ref(), b"schemeClr") => {
        return resolve_scheme_color_from_reader(&mut reader, event, theme_colors)
          .map(|it| it.color);
      }
      Event::Empty(event) if in_defrpr && qname_ends_with(event.name().as_ref(), b"schemeClr") => {
        return resolve_empty_scheme_color(&event, theme_colors).map(|it| it.color);
      }
      Event::Start(event) if in_defrpr && qname_ends_with(event.name().as_ref(), b"srgbClr") => {
        return color_attr(&event, b"val");
      }
      Event::Empty(event) if in_defrpr && qname_ends_with(event.name().as_ref(), b"srgbClr") => {
        return color_attr(&event, b"val");
      }
      Event::Eof => return None,
      _ => {}
    }
  }
}

fn chart_label_texts(xml: &str) -> Vec<String> {
  let mut reader = Reader::from_str(xml);
  reader.config_mut().trim_text(false);

  let mut in_dlbl = false;
  let mut in_text = false;
  let mut current = String::new();
  let mut texts = Vec::new();

  loop {
    match reader.read_event() {
      Ok(Event::Start(event)) if qname_ends_with(event.name().as_ref(), b"dLbl") => {
        in_dlbl = true;
        current.clear();
      }
      Ok(Event::End(event)) if qname_ends_with(event.name().as_ref(), b"dLbl") => {
        if !current.is_empty() {
          texts.push(current.clone());
        }
        current.clear();
        in_dlbl = false;
      }
      Ok(Event::Start(event)) if in_dlbl && qname_ends_with(event.name().as_ref(), b"t") => {
        in_text = true;
      }
      Ok(Event::End(event)) if qname_ends_with(event.name().as_ref(), b"t") => {
        in_text = false;
      }
      Ok(Event::Text(event)) if in_dlbl && in_text => {
        if let Ok(value) = event.xml10_content() {
          current.push_str(value.as_ref());
        }
      }
      Ok(Event::CData(event)) if in_dlbl && in_text => {
        if let Ok(value) = event.xml10_content() {
          current.push_str(value.as_ref());
        }
      }
      Ok(Event::Eof) => break,
      Ok(_) => {}
      Err(_) => break,
    }
  }

  texts
}

fn diagram_text_color(xml: &str, theme_colors: &ThemeColors) -> Option<RgbColor> {
  let mut reader = Reader::from_str(xml);
  reader.config_mut().trim_text(false);

  let mut in_tx_fill = false;
  loop {
    match reader.read_event().ok()? {
      Event::Start(event) if qname_ends_with(event.name().as_ref(), b"txFillClrLst") => {
        in_tx_fill = true;
      }
      Event::End(event) if qname_ends_with(event.name().as_ref(), b"txFillClrLst") => {
        in_tx_fill = false;
      }
      Event::Start(event) if in_tx_fill && qname_ends_with(event.name().as_ref(), b"schemeClr") => {
        return resolve_scheme_color_from_reader(&mut reader, event, theme_colors)
          .map(|it| it.color);
      }
      Event::Empty(event) if in_tx_fill && qname_ends_with(event.name().as_ref(), b"schemeClr") => {
        return resolve_empty_scheme_color(&event, theme_colors).map(|it| it.color);
      }
      Event::Start(event) if in_tx_fill && qname_ends_with(event.name().as_ref(), b"srgbClr") => {
        return color_attr(&event, b"val");
      }
      Event::Empty(event) if in_tx_fill && qname_ends_with(event.name().as_ref(), b"srgbClr") => {
        return color_attr(&event, b"val");
      }
      Event::Eof => return None,
      _ => {}
    }
  }
}

fn diagram_shape_texts(xml: &str) -> Vec<String> {
  let mut reader = Reader::from_str(xml);
  reader.config_mut().trim_text(false);

  let mut in_shape = false;
  let mut in_text = false;
  let mut current = String::new();
  let mut texts = Vec::new();

  loop {
    match reader.read_event() {
      Ok(Event::Start(event)) if qname_ends_with(event.name().as_ref(), b"sp") => {
        in_shape = true;
        current.clear();
      }
      Ok(Event::End(event)) if qname_ends_with(event.name().as_ref(), b"sp") => {
        if !current.is_empty() {
          texts.push(current.clone());
        }
        current.clear();
        in_shape = false;
      }
      Ok(Event::Start(event)) if in_shape && qname_ends_with(event.name().as_ref(), b"t") => {
        in_text = true;
      }
      Ok(Event::End(event)) if qname_ends_with(event.name().as_ref(), b"t") => {
        in_text = false;
      }
      Ok(Event::Text(event)) if in_shape && in_text => {
        if let Ok(value) = event.xml10_content() {
          current.push_str(value.as_ref());
        }
      }
      Ok(Event::CData(event)) if in_shape && in_text => {
        if let Ok(value) = event.xml10_content() {
          current.push_str(value.as_ref());
        }
      }
      Ok(Event::Eof) => break,
      Ok(_) => {}
      Err(_) => break,
    }
  }

  texts
}

fn simple_text_block(text: String, style: TextStyle) -> Block {
  Block::Paragraph(Paragraph {
    inlines: vec![InlineItem::Text(TextRun {
      text,
      style: style.clone(),
      hyperlink_url: None,
      dynamic_field: None,
    })],
    footnote_reference_ids: Vec::new(),
    endnote_reference_ids: Vec::new(),
    #[cfg(test)]
    runs: Vec::new(),
    format: ParagraphFormat::default(),
    list_label: None,
    list_label_style: TextStyle::default(),
    list_label_hyperlink_url: None,
  })
}

fn text_style_with_color(styles: &StylesCatalog, color: RgbColor) -> TextStyle {
  let mut style = styles.doc_default_run.clone();
  style.color = color;
  style
}

fn resolve_empty_scheme_color(
  event: &quick_xml::events::BytesStart<'_>,
  theme_colors: &ThemeColors,
) -> Option<ResolvedColor> {
  let value = attr_value(event, b"val")?;
  Some(ResolvedColor {
    color: resolve_drawingml_scheme_color_name(value.as_ref(), theme_colors)?,
    opacity: 1.0,
  })
}

fn resolve_scheme_color_from_reader(
  reader: &mut Reader<&[u8]>,
  start: quick_xml::events::BytesStart<'_>,
  theme_colors: &ThemeColors,
) -> Option<ResolvedColor> {
  let value = attr_value(&start, b"val")?;
  let mut color = resolve_drawingml_scheme_color_name(value.as_ref(), theme_colors)?;
  let mut opacity = 1.0f32;
  let target_name = start.name().as_ref().to_vec();
  let mut depth = 1usize;

  while depth > 0 {
    match reader.read_event().ok()? {
      Event::Start(event) if event.name().as_ref() == target_name.as_slice() => depth += 1,
      Event::End(event) if event.name().as_ref() == target_name.as_slice() => {
        depth = depth.saturating_sub(1);
      }
      Event::Start(event) | Event::Empty(event) => {
        if qname_ends_with(event.name().as_ref(), b"tint") {
          if let Some(value) = percent_attr(&event, b"val") {
            let mut hsl = HslColor::from_rgb(color);
            hsl.apply_tint(value);
            color = hsl.to_rgb();
          }
        } else if qname_ends_with(event.name().as_ref(), b"shade") {
          if let Some(value) = percent_attr(&event, b"val") {
            let mut hsl = HslColor::from_rgb(color);
            hsl.apply_shade(value);
            color = hsl.to_rgb();
          }
        } else if qname_ends_with(event.name().as_ref(), b"lumMod") {
          if let Some(value) = percent_attr(&event, b"val") {
            let mut hsl = HslColor::from_rgb(color);
            hsl.apply_luminance_mod(value);
            color = hsl.to_rgb();
          }
        } else if qname_ends_with(event.name().as_ref(), b"lumOff") {
          if let Some(value) = percent_attr(&event, b"val") {
            let mut hsl = HslColor::from_rgb(color);
            hsl.apply_luminance_offset(value);
            color = hsl.to_rgb();
          }
        } else if qname_ends_with(event.name().as_ref(), b"alpha") {
          opacity = alpha_percent_attr(&event, b"val");
        }
      }
      Event::Eof => return None,
      _ => {}
    }
  }

  Some(ResolvedColor { color, opacity })
}

fn resolve_drawingml_scheme_color_name(
  value: &str,
  theme_colors: &ThemeColors,
) -> Option<RgbColor> {
  match value {
    "dk1" | "tx1" => theme_colors.dark1,
    "lt1" | "bg1" => theme_colors.light1,
    "dk2" | "tx2" => theme_colors.dark2,
    "lt2" | "bg2" => theme_colors.light2,
    "accent1" => theme_colors.accent1,
    "accent2" => theme_colors.accent2,
    "accent3" => theme_colors.accent3,
    "accent4" => theme_colors.accent4,
    "accent5" => theme_colors.accent5,
    "accent6" => theme_colors.accent6,
    "hlink" => theme_colors.hyperlink,
    "folHlink" => theme_colors.followed_hyperlink,
    _ => None,
  }
}

fn color_attr(event: &quick_xml::events::BytesStart<'_>, key: &[u8]) -> Option<RgbColor> {
  parse_hex_color(attr_value(event, key)?.as_ref())
}

fn attr_value(event: &quick_xml::events::BytesStart<'_>, key: &[u8]) -> Option<Box<str>> {
  event
    .attributes()
    .flatten()
    .find(|attribute| attribute.key.as_ref() == key)
    .and_then(|attribute| String::from_utf8(attribute.value.into_owned()).ok())
    .map(String::into_boxed_str)
}

fn percent_attr(event: &quick_xml::events::BytesStart<'_>, key: &[u8]) -> Option<f32> {
  attr_value(event, key)?
    .parse::<f32>()
    .ok()
    .map(|value| (value / units::DRAWINGML_PERCENT_SCALE).clamp(0.0, 1.0))
}

fn alpha_percent_attr(event: &quick_xml::events::BytesStart<'_>, key: &[u8]) -> f32 {
  percent_attr(event, key).unwrap_or(1.0)
}

fn even_and_odd_headers(package: &mut WordprocessingDocument, main: &MainDocumentPart) -> bool {
  main
    .document_settings_part(package)
    .and_then(|part| part.root_element(package).ok())
    .and_then(|settings| {
      settings
        .w_even_and_odd_headers
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
      form_widget_ids,
    )
    .unwrap_or_else(|| previous_default_header.clone());
    section.footer_blocks = referenced_footer_blocks(
      package,
      main,
      section_properties,
      styles,
      w::HeaderFooterValues::Default,
      form_widget_ids,
    )
    .unwrap_or_else(|| previous_default_footer.clone());
    section.first_header_blocks = referenced_header_blocks(
      package,
      main,
      section_properties,
      styles,
      w::HeaderFooterValues::First,
      form_widget_ids,
    )
    .unwrap_or_else(|| previous_first_header.clone());
    section.first_footer_blocks = referenced_footer_blocks(
      package,
      main,
      section_properties,
      styles,
      w::HeaderFooterValues::First,
      form_widget_ids,
    )
    .unwrap_or_else(|| previous_first_footer.clone());
    section.even_header_blocks = referenced_header_blocks(
      package,
      main,
      section_properties,
      styles,
      w::HeaderFooterValues::Even,
      form_widget_ids,
    )
    .unwrap_or_else(|| previous_even_header.clone());
    section.even_footer_blocks = referenced_footer_blocks(
      package,
      main,
      section_properties,
      styles,
      w::HeaderFooterValues::Even,
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

fn body_sections(
  body: &w::Body,
  styles: &StylesCatalog,
  numbering: &mut NumberingCatalog,
  images: &ImageCatalog,
  hyperlinks: &HyperlinkCatalog,
  form_widget_ids: &mut FormWidgetIdAllocator,
) -> Vec<ImportedSection> {
  let mut sections = Vec::new();
  let mut current_blocks = Vec::new();
  let mut previous_properties = None;

  for choice in &body.body_choice {
    match choice {
      w::BodyChoice::WP(paragraph) => {
        let mut model = paragraph_model(
          paragraph,
          styles,
          numbering,
          images,
          hyperlinks,
          form_widget_ids,
        );
        model.format.hidden_separator = paragraph_mark_is_hidden(paragraph);
        if paragraph_is_effectively_empty(&model)
          && (current_blocks
            .last()
            .is_some_and(|block| matches!(block, Block::Table(_)))
            || paragraph_empty_font_size_pt(&model) <= 4.5)
        {
          continue;
        }
        push_body_paragraph(&mut current_blocks, model);
        if let Some(section_properties) = paragraph
          .paragraph_properties
          .as_deref()
          .and_then(|properties| properties.section_properties.as_deref())
          .cloned()
        {
          close_section(
            &mut sections,
            &mut current_blocks,
            Some(section_properties),
            &mut previous_properties,
          );
        }
      }
      w::BodyChoice::WTbl(table) => current_blocks.push(Block::Table(table_model(
        table,
        styles,
        numbering,
        images,
        hyperlinks,
        form_widget_ids,
      ))),
      w::BodyChoice::WSdt(sdt) => {
        current_blocks.extend(sdt_block_blocks(
          sdt,
          styles,
          numbering,
          images,
          hyperlinks,
          form_widget_ids,
        ));
      }
      _ => {}
    }
  }

  if body.w_sect_pr.is_some() || sections.is_empty() || !current_blocks.is_empty() {
    close_section(
      &mut sections,
      &mut current_blocks,
      body.w_sect_pr.as_deref().cloned(),
      &mut previous_properties,
    );
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
    blocks.push(Block::Frame(FloatingFrame {
      blocks: vec![Block::Paragraph(paragraph.clone())],
      width_pt: frame.width_pt,
      height_pt: frame.height_pt,
      height_rule: frame.height_rule,
      placement: frame.placement,
      fill_color: paragraph.format.shading,
      borders: paragraph.format.borders,
    }));
    return;
  }
  blocks.push(Block::Paragraph(paragraph));
}

fn paragraph_mark_is_hidden(paragraph: &w::Paragraph) -> bool {
  paragraph
    .paragraph_properties
    .as_deref()
    .and_then(|properties| properties.paragraph_mark_run_properties.as_deref())
    .and_then(|properties| properties.w_vanish.as_ref())
    .is_some_and(|vanish| vanish.val.is_none_or(|value| value.as_bool()))
}

fn paragraph_is_effectively_empty(paragraph: &Paragraph) -> bool {
  paragraph.list_label.is_none()
    && paragraph.footnote_reference_ids.is_empty()
    && paragraph.endnote_reference_ids.is_empty()
    && paragraph.inlines.iter().all(|inline| match inline {
      InlineItem::Text(run) => run.text.trim().is_empty(),
      InlineItem::Image(_) | InlineItem::Shape(_) => false,
      InlineItem::FormWidgetStart(_) | InlineItem::FormWidgetEnd(_) => true,
      InlineItem::PageBreak | InlineItem::ColumnBreak => false,
    })
}

fn paragraph_empty_font_size_pt(paragraph: &Paragraph) -> f32 {
  paragraph
    .inlines
    .iter()
    .find_map(|inline| match inline {
      InlineItem::Text(run) => Some(run.style.font_size_pt),
      InlineItem::Image(_)
      | InlineItem::Shape(_)
      | InlineItem::FormWidgetStart(_)
      | InlineItem::FormWidgetEnd(_)
      | InlineItem::PageBreak
      | InlineItem::ColumnBreak => None,
    })
    .unwrap_or(TextStyle::default().font_size_pt)
}

fn close_section(
  sections: &mut Vec<ImportedSection>,
  current_blocks: &mut Vec<Block>,
  section_properties: Option<w::SectionProperties>,
  previous_properties: &mut Option<w::SectionProperties>,
) {
  let break_kind =
    normalized_section_break(section_properties.as_ref(), previous_properties.as_ref());
  let page = section_properties
    .as_ref()
    .map(page_setup)
    .unwrap_or_default();
  let columns = section_properties
    .as_ref()
    .map(section_columns)
    .unwrap_or_default();
  let title_page = section_properties
    .as_ref()
    .and_then(|section| section.w_title_pg.as_ref())
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
    page: PageSetup::default(),
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
    .w_type
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
    .w_pg_sz
    .as_ref()
    .and_then(|size| size.orient)
    .or_else(|| {
      let size = section.w_pg_sz.as_ref()?;
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

fn section_column_count(section: &w::SectionProperties) -> i16 {
  let Some(columns) = section.w_cols.as_ref() else {
    return 1;
  };
  if !columns.equal_width.is_none_or(|value| value.as_bool()) && !columns.w_col.is_empty() {
    return (columns.w_col.len() as i16).max(1);
  }
  columns.column_count.unwrap_or(1).max(1)
}

fn section_columns(section: &w::SectionProperties) -> SectionColumns {
  let Some(columns) = section.w_cols.as_ref() else {
    return SectionColumns::default();
  };
  let equal_width = columns.equal_width.is_none_or(|value| value.as_bool());
  let gap_pt = columns
    .space
    .as_ref()
    .and_then(twips_measure_to_points)
    .filter(|gap| gap.is_finite() && *gap >= 0.0)
    .unwrap_or(DEFAULT_SECTION_COLUMN_GAP_PT);
  if !equal_width && !columns.w_col.is_empty() {
    let explicit_widths_pt = columns
      .w_col
      .iter()
      .filter_map(|column| {
        column
          .width
          .as_ref()
          .and_then(twips_measure_to_points)
          .filter(|width| width.is_finite() && *width > 0.0)
      })
      .collect::<Vec<_>>();
    if explicit_widths_pt.len() == columns.w_col.len() {
      let explicit_gaps_pt = columns
        .w_col
        .iter()
        .take(columns.w_col.len().saturating_sub(1))
        .map(|column| {
          column
            .space
            .as_ref()
            .and_then(twips_measure_to_points)
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
  form_widget_ids: &mut FormWidgetIdAllocator,
) -> Vec<Block> {
  let Some(content) = sdt.sdt_content_block.as_ref() else {
    return Vec::new();
  };

  content
    .sdt_content_block_choice
    .iter()
    .filter_map(|choice| match choice {
      w::SdtContentBlockChoice::WP(paragraph) => Some(vec![Block::Paragraph(paragraph_model(
        paragraph.as_ref(),
        styles,
        numbering,
        images,
        hyperlinks,
        form_widget_ids,
      ))]),
      w::SdtContentBlockChoice::WTbl(table) => Some(vec![Block::Table(table_model(
        table.as_ref(),
        styles,
        numbering,
        images,
        hyperlinks,
        form_widget_ids,
      ))]),
      w::SdtContentBlockChoice::WSdt(sdt) => Some(sdt_block_blocks(
        sdt.as_ref(),
        styles,
        numbering,
        images,
        hyperlinks,
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
  form_widget_ids: &mut FormWidgetIdAllocator,
) -> Option<Vec<Block>> {
  let relationship_id =
    section
      .section_properties_choice
      .iter()
      .find_map(|choice| match choice {
        w::SectionPropertiesChoice::WHeaderReference(reference)
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
        w::HeaderChoice::WP(paragraph) => Some(Block::Paragraph(paragraph_model(
          paragraph,
          styles,
          &mut numbering,
          &images,
          &hyperlinks,
          form_widget_ids,
        ))),
        w::HeaderChoice::WTbl(table) => Some(Block::Table(table_model(
          table,
          styles,
          &mut numbering,
          &images,
          &hyperlinks,
          form_widget_ids,
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
  form_widget_ids: &mut FormWidgetIdAllocator,
) -> Option<Vec<Block>> {
  header_blocks(package, main, section, styles, header_type, form_widget_ids)
}

fn footer_blocks(
  package: &mut WordprocessingDocument,
  main: &MainDocumentPart,
  section: &w::SectionProperties,
  styles: &StylesCatalog,
  footer_type: w::HeaderFooterValues,
  form_widget_ids: &mut FormWidgetIdAllocator,
) -> Option<Vec<Block>> {
  let relationship_id =
    section
      .section_properties_choice
      .iter()
      .find_map(|choice| match choice {
        w::SectionPropertiesChoice::WFooterReference(reference)
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
        w::FooterChoice::WP(paragraph) => Some(Block::Paragraph(paragraph_model(
          paragraph,
          styles,
          &mut numbering,
          &images,
          &hyperlinks,
          form_widget_ids,
        ))),
        w::FooterChoice::WTbl(table) => Some(Block::Table(table_model(
          table,
          styles,
          &mut numbering,
          &images,
          &hyperlinks,
          form_widget_ids,
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
  form_widget_ids: &mut FormWidgetIdAllocator,
) -> Option<Vec<Block>> {
  footer_blocks(package, main, section, styles, footer_type, form_widget_ids)
}

fn footnotes(
  package: &mut WordprocessingDocument,
  main: &MainDocumentPart,
  styles: &StylesCatalog,
  form_widget_ids: &mut FormWidgetIdAllocator,
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
    form_widget_ids,
  };
  let mut notes = BTreeMap::new();

  for footnote in &footnotes.w_footnote {
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
        format!("{} ", footnote.id),
        Some(note_backlink_url("footnote", footnote.id)),
      ),
      footnote
        .footnote_choice
        .iter()
        .filter_map(|choice| match choice {
          w::FootnoteChoice::WP(paragraph) => Some(paragraph.as_ref()),
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
  form_widget_ids: &mut FormWidgetIdAllocator,
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
    form_widget_ids,
  };
  let mut notes = BTreeMap::new();

  for endnote in &endnotes.w_endnote {
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
        format!("{} ", endnote.id),
        Some(note_backlink_url("endnote", endnote.id)),
      ),
      endnote
        .endnote_choice
        .iter()
        .filter_map(|choice| match choice {
          w::EndnoteChoice::WP(paragraph) => Some(paragraph.as_ref()),
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

fn comment_blocks(
  package: &mut WordprocessingDocument,
  main: &MainDocumentPart,
  styles: &StylesCatalog,
  form_widget_ids: &mut FormWidgetIdAllocator,
) -> Result<Vec<Block>> {
  let Some(part) = main.wordprocessing_comments_part(package) else {
    return Ok(Vec::new());
  };
  let images = ImageCatalog::load_from_comments(package, &part);
  let hyperlinks = HyperlinkCatalog::load(package, &part);
  let comments = part.root_element(package)?;
  let mut numbering = NumberingCatalog::default();
  let mut context = NoteImportContext {
    styles,
    numbering: &mut numbering,
    images: &images,
    hyperlinks: &hyperlinks,
    form_widget_ids,
  };
  let mut blocks = Vec::new();

  for comment in &comments.w_comment {
    append_note_blocks(
      &mut blocks,
      NoteLabel::new(format!("[{}] ", comment.id), None),
      comment
        .comment_choice
        .iter()
        .filter_map(|choice| match choice {
          w::CommentChoice::WP(paragraph) => Some(paragraph.as_ref()),
          _ => None,
        }),
      &mut context,
    );
  }

  Ok(blocks)
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
      context.form_widget_ids,
    );
    if is_first_paragraph {
      prepend_note_marker(&mut model, &label);
      is_first_paragraph = false;
    }
    blocks.push(Block::Paragraph(model));
  }
}

fn prepend_note_marker(paragraph: &mut Paragraph, label: &NoteLabel) {
  let base_style = paragraph
    .inlines
    .iter()
    .find_map(|inline| match inline {
      InlineItem::Text(run) => Some(run.style.clone()),
      _ => None,
    })
    .unwrap_or_default();
  paragraph.inlines.insert(
    0,
    InlineItem::Text(TextRun {
      text: label.text.clone(),
      style: note_reference_style(&base_style),
      hyperlink_url: label.hyperlink_url.clone(),
      dynamic_field: None,
    }),
  );
}

fn table_model(
  table: &w::Table,
  styles: &StylesCatalog,
  numbering: &mut NumberingCatalog,
  images: &ImageCatalog,
  hyperlinks: &HyperlinkCatalog,
  form_widget_ids: &mut FormWidgetIdAllocator,
) -> Table {
  let properties = table.w_tbl_pr.as_deref();
  let table_style_id = properties
    .and_then(|properties| properties.table_style.as_ref())
    .map(|style| style.val.as_str());
  let table_style = styles.table_style(table_style_id);
  let table_look = properties
    .and_then(|properties| properties.table_look.as_ref())
    .map(table_look_model)
    .unwrap_or_default();
  let cell_margins = properties
    .and_then(|properties| properties.table_cell_margin_default.as_deref())
    .map(table_cell_margin_default)
    .or(table_style.cell_margins)
    .unwrap_or_default();
  let rows = table
    .table_choice2
    .iter()
    .filter_map(|choice| match choice {
      w::TableChoice2::WTr(row) => Some(row.as_ref()),
      _ => None,
    })
    .collect::<Vec<_>>();
  let row_count = rows.len();
  Table {
    column_widths_pt: table
      .w_tbl_grid
      .as_deref()
      .map(|grid| {
        grid
          .w_grid_col
          .iter()
          .filter_map(|column| column.width.as_ref().and_then(twips_measure_to_points))
          .collect()
      })
      .unwrap_or_default(),
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
    placement: properties
      .and_then(|properties| properties.table_position_properties.as_ref())
      .map(table_position_placement),
    borders: properties
      .and_then(|properties| properties.table_borders.as_deref())
      .map(|borders| direct_table_borders_model(table_style.table_borders, borders))
      .or(table_style.table_borders),
    cell_spacing_pt: properties
      .and_then(|properties| properties.table_cell_spacing.as_ref())
      .and_then(table_cell_spacing_to_points)
      .or(table_style.cell_spacing_pt)
      .unwrap_or(0.0),
    rows: {
      let mut context = TableImportContext {
        styles,
        numbering,
        images,
        hyperlinks,
        form_widget_ids,
        cell_margins,
        table_style: &table_style,
        table_look,
        row_count,
      };
      rows
        .iter()
        .enumerate()
        .map(|(row_index, row)| table_row_model(row, &mut context, row_index))
        .collect()
    },
  }
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
  TableRow {
    height_pt: row_style.height_pt,
    exact_height: row_style.exact_height.unwrap_or(false),
    repeat_header: row_style.repeat_header.unwrap_or(false),
    cant_split: row_style.cant_split.unwrap_or(false),
    cell_spacing_pt: row_style.cell_spacing_pt,
    grid_before,
    grid_after,
    cells: {
      let cells = row
        .table_row_choice
        .iter()
        .filter_map(|choice| match choice {
          w::TableRowChoice::WTc(cell) => Some(cell.as_ref()),
          _ => None,
        })
        .collect::<Vec<_>>();
      let cell_count = cells.len();
      cells
        .iter()
        .enumerate()
        .map(|(cell_index, cell)| {
          table_cell_model(
            cell,
            context,
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
        .collect()
    },
  }
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
  style: TableCellStyle,
) -> TableCell {
  let properties = cell.table_cell_properties.as_deref();
  let blocks = cell
    .table_cell_choice
    .iter()
    .filter_map(|choice| match choice {
      w::TableCellChoice::WP(paragraph) => Some(Block::Paragraph(paragraph_model_with_base(
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
        },
      ))),
      w::TableCellChoice::WTbl(table) => Some(Block::Table(table_model(
        table,
        context.styles,
        context.numbering,
        context.images,
        context.hyperlinks,
        context.form_widget_ids,
      ))),
      _ => None,
    })
    .collect();
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
      .map(|margins| table_cell_margin(margins, context.cell_margins))
      .or(style.margins)
      .unwrap_or(context.cell_margins),
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

fn table_row_grid_properties(properties: Option<&w::TableRowProperties>) -> (usize, usize) {
  let Some(properties) = properties else {
    return (0, 0);
  };
  let mut grid_before = 0;
  let mut grid_after = 0;
  for choice in &properties.table_row_properties_choice1 {
    match choice {
      w::TableRowPropertiesChoice::WGridBefore(before) => {
        grid_before = before.val.max(0) as usize;
      }
      w::TableRowPropertiesChoice::WGridAfter(after) => {
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
        if let w::TableRowPropertiesChoice::WCnfStyle(style) = choice {
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
  let mut model = CellMargins::default();
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
  width.and_then(measurement_or_percent_to_points)
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
    .and_then(measurement_or_percent_to_points)
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
  background.color.as_deref().and_then(parse_hex_color)
}

fn merge_paragraph_format(format: &mut ParagraphFormat, properties: Option<ParagraphProps<'_>>) {
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

  if let Some(spacing) = properties.spacing_between_lines() {
    format.spacing_before_set = spacing.before.is_some();
    format.spacing_before_pt = spacing
      .before
      .as_ref()
      .and_then(twips_measure_to_points)
      .unwrap_or(0.0);
    format.spacing_after_set = spacing.after.is_some();
    format.spacing_after_pt = spacing
      .after
      .as_ref()
      .and_then(twips_measure_to_points)
      .unwrap_or(0.0);
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
    format.indent_left_pt = indentation
      .start
      .as_ref()
      .or(indentation.left.as_ref())
      .and_then(signed_twips_measure_to_points)
      .unwrap_or(0.0);
    format.indent_right_pt = indentation
      .end
      .as_ref()
      .or(indentation.right.as_ref())
      .and_then(signed_twips_measure_to_points)
      .unwrap_or(0.0);
    let first_line = indentation
      .first_line
      .as_ref()
      .and_then(twips_measure_to_points)
      .unwrap_or(0.0);
    let hanging = indentation
      .hanging
      .as_ref()
      .and_then(twips_measure_to_points)
      .unwrap_or(0.0);
    format.first_line_indent_pt = first_line - hanging;
  }

  if let Some(tabs) = properties.tabs() {
    format.tab_stops = tab_stops(tabs);
  }

  if let Some(justification) = properties.justification() {
    format.alignment = match justification.val {
      w::JustificationValues::Center => ParagraphAlignment::Center,
      w::JustificationValues::Right | w::JustificationValues::End => ParagraphAlignment::Right,
      w::JustificationValues::Both
      | w::JustificationValues::Distribute
      | w::JustificationValues::MediumKashida
      | w::JustificationValues::HighKashida
      | w::JustificationValues::LowKashida
      | w::JustificationValues::ThaiDistribute => ParagraphAlignment::Justify,
      w::JustificationValues::Left
      | w::JustificationValues::Start
      | w::JustificationValues::NumTab => ParagraphAlignment::Left,
    };
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
    format.frame = Some(paragraph_frame_properties(frame));
  }
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
      wrap: frame_wrap_mode(frame.wrap),
      margin_top_pt: vertical_space,
      margin_right_pt: horizontal_space,
      margin_bottom_pt: vertical_space,
      margin_left_pt: horizontal_space,
    },
  }
}

fn tab_stops(tabs: &w::Tabs) -> Vec<TabStop> {
  let mut stops = tabs
    .w_tab
    .iter()
    .filter_map(|tab| {
      let alignment = match tab.val {
        w::TabStopValues::Left | w::TabStopValues::Start | w::TabStopValues::Decimal => {
          TabStopAlignment::Left
        }
        w::TabStopValues::Center => TabStopAlignment::Center,
        w::TabStopValues::Right | w::TabStopValues::End | w::TabStopValues::Number => {
          TabStopAlignment::Right
        }
        w::TabStopValues::Clear | w::TabStopValues::Bar => return None,
      };
      Some(TabStop {
        position_pt: signed_twips_measure_to_points(&tab.position)?,
        alignment,
      })
    })
    .filter(|stop| stop.position_pt.is_finite() && stop.position_pt >= 0.0)
    .collect::<Vec<_>>();
  stops.sort_by(|a, b| a.position_pt.total_cmp(&b.position_pt));
  stops.dedup_by(|a, b| (a.position_pt - b.position_pt).abs() < TAB_STOP_DEDUP_EPSILON_PT);
  stops
}

fn paragraph_inlines(
  paragraph: &w::Paragraph,
  base_style: TextStyle,
  styles: &StylesCatalog,
  images: &ImageCatalog,
  hyperlinks: &HyperlinkCatalog,
  form_widget_ids: &mut FormWidgetIdAllocator,
) -> Vec<InlineItem> {
  let mut inlines = Vec::new();
  let mut inline_context = InlineImportContext {
    styles,
    images,
    hyperlinks,
    form_widget_ids,
  };
  let mut complex_field = None;

  for choice in &paragraph.paragraph_choice {
    match choice {
      w::ParagraphChoice::WR(run) => {
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
          &mut complex_field,
        );
      }
      w::ParagraphChoice::WFldSimple(field) => {
        push_simple_field(
          field,
          &mut inlines,
          base_style.clone(),
          inline_context.styles,
          inline_context.images,
          inline_context.hyperlinks,
          inline_context.form_widget_ids,
        );
      }
      w::ParagraphChoice::WHyperlink(hyperlink) => {
        push_hyperlink_content(
          hyperlink,
          &mut inlines,
          base_style.clone(),
          None,
          &mut inline_context,
          &mut complex_field,
        );
      }
      w::ParagraphChoice::Choice(choice) => match choice.as_ref() {
        w::ParagraphChoice2::WIns(inserted) => {
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
        w::ParagraphChoice2::WDel(deleted) => {
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
        w::ParagraphChoice2::WMoveFrom(moved) => {
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
        w::ParagraphChoice2::WMoveTo(moved) => {
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
        _ => {}
      },
      w::ParagraphChoice::WSdt(sdt) => push_sdt_run(
        sdt,
        &mut inlines,
        base_style.clone(),
        None,
        &mut inline_context,
      ),
      _ => {}
    }
  }
  flush_unclosed_complex_field(&mut inlines, &mut complex_field);

  inlines
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
  form_widget_ids: &'a mut FormWidgetIdAllocator,
}

fn push_run_or_complex_field(
  run: &w::Run,
  inlines: &mut Vec<InlineItem>,
  base_style: TextStyle,
  context: RunImportContext<'_>,
  hyperlink_url: Option<&str>,
  state: &mut Option<ComplexFieldState>,
) {
  if state.is_none() && !run_starts_complex_field(run) {
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
      w::RunChoice::WFldChar(field_char)
        if field_char.field_char_type == w::FieldCharValues::Begin =>
      {
        *state = Some(ComplexFieldState {
          instr: String::new(),
          result: Vec::new(),
          in_result: false,
          style: style.clone(),
          hyperlink_url: hyperlink_url.map(ToString::to_string),
        });
      }
      w::RunChoice::WFldChar(field_char)
        if field_char.field_char_type == w::FieldCharValues::Separate =>
      {
        if let Some(state) = state {
          state.in_result = true;
        }
      }
      w::RunChoice::WFldChar(field_char)
        if field_char.field_char_type == w::FieldCharValues::End =>
      {
        flush_closed_complex_field(inlines, state);
      }
      w::RunChoice::WInstrText(code) => {
        if let Some(state) = state
          && !state.in_result
          && let Some(content) = &code.xml_content
        {
          state.instr.push_str(content);
        }
      }
      _ => {
        if let Some(state) = state
          && state.in_result
        {
          push_run(
            run,
            &mut state.result,
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
      w::RunChoice::WFldChar(field_char)
        if field_char.field_char_type == w::FieldCharValues::Begin
    )
  })
}

fn flush_closed_complex_field(
  inlines: &mut Vec<InlineItem>,
  state: &mut Option<ComplexFieldState>,
) {
  let Some(state) = state.take() else {
    return;
  };
  if let Some(kind) = dynamic_field_kind(&state.instr) {
    push_dynamic_field(inlines, kind, state.style, state.hyperlink_url.as_deref());
  } else {
    inlines.extend(state.result);
  }
}

fn flush_unclosed_complex_field(
  inlines: &mut Vec<InlineItem>,
  state: &mut Option<ComplexFieldState>,
) {
  if state.is_some() {
    flush_closed_complex_field(inlines, state);
  }
}

fn dynamic_field_kind(instr: &str) -> Option<DynamicFieldKind> {
  let name = instr.split_whitespace().next()?.to_ascii_uppercase();
  match name.as_str() {
    "PAGE" => Some(DynamicFieldKind::Page),
    "NUMPAGES" => Some(DynamicFieldKind::NumPages),
    _ => None,
  }
}

fn push_dynamic_field(
  inlines: &mut Vec<InlineItem>,
  kind: DynamicFieldKind,
  style: TextStyle,
  hyperlink_url: Option<&str>,
) {
  inlines.push(InlineItem::Text(TextRun {
    text: "1".to_string(),
    style,
    hyperlink_url: hyperlink_url.map(ToString::to_string),
    dynamic_field: Some(kind),
  }));
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
  complex_field: &mut Option<ComplexFieldState>,
) {
  let hyperlink_url = self::hyperlink_url(hyperlink, context.hyperlinks)
    .or_else(|| inherited_url.map(ToString::to_string));
  for item in &hyperlink.hyperlink_choice {
    match item {
      w::HyperlinkChoice::WR(run) => push_run_or_complex_field(
        run,
        inlines,
        base_style.clone(),
        RunImportContext {
          styles: context.styles,
          images: context.images,
          hyperlinks: context.hyperlinks,
        },
        hyperlink_url.as_deref(),
        complex_field,
      ),
      w::HyperlinkChoice::WFldSimple(field) => push_simple_field(
        field,
        inlines,
        base_style.clone(),
        context.styles,
        context.images,
        context.hyperlinks,
        context.form_widget_ids,
      ),
      w::HyperlinkChoice::WHyperlink(nested) => push_hyperlink_content(
        nested,
        inlines,
        base_style.clone(),
        hyperlink_url.as_deref(),
        context,
        complex_field,
      ),
      w::HyperlinkChoice::WSdt(sdt) => push_sdt_run(
        sdt,
        inlines,
        base_style.clone(),
        hyperlink_url.as_deref(),
        context,
      ),
      w::HyperlinkChoice::WIns(inserted) => push_inserted_run(
        inserted,
        inlines,
        base_style.clone(),
        context.styles,
        context.images,
        context.hyperlinks,
        hyperlink_url.as_deref(),
      ),
      w::HyperlinkChoice::WDel(deleted) => push_deleted_run(
        deleted,
        inlines,
        base_style.clone(),
        context.styles,
        context.images,
        context.hyperlinks,
        hyperlink_url.as_deref(),
      ),
      w::HyperlinkChoice::WMoveFrom(moved) => push_move_from_run(
        moved,
        inlines,
        base_style.clone(),
        context.styles,
        context.images,
        context.hyperlinks,
        hyperlink_url.as_deref(),
      ),
      w::HyperlinkChoice::WMoveTo(moved) => push_move_to_run(
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

fn paragraph_note_reference_ids(paragraph: &w::Paragraph) -> (Vec<i64>, Vec<i64>) {
  let mut footnotes = Vec::new();
  let mut endnotes = Vec::new();
  for choice in &paragraph.paragraph_choice {
    match choice {
      w::ParagraphChoice::WR(run) => {
        collect_run_note_reference_ids(run, &mut footnotes, &mut endnotes)
      }
      w::ParagraphChoice::WFldSimple(field) => {
        collect_simple_field_note_reference_ids(field, &mut footnotes, &mut endnotes);
      }
      w::ParagraphChoice::WHyperlink(hyperlink) => {
        collect_hyperlink_note_reference_ids(hyperlink, &mut footnotes, &mut endnotes);
      }
      w::ParagraphChoice::Choice(choice) => match choice.as_ref() {
        w::ParagraphChoice2::WIns(inserted) => {
          collect_inserted_run_note_reference_ids(inserted, &mut footnotes, &mut endnotes);
        }
        w::ParagraphChoice2::WDel(deleted) => {
          collect_deleted_run_note_reference_ids(deleted, &mut footnotes, &mut endnotes);
        }
        w::ParagraphChoice2::WMoveFrom(moved) => {
          collect_move_from_run_note_reference_ids(moved, &mut footnotes, &mut endnotes);
        }
        w::ParagraphChoice2::WMoveTo(moved) => {
          collect_move_to_run_note_reference_ids(moved, &mut footnotes, &mut endnotes);
        }
        _ => {}
      },
      w::ParagraphChoice::WSdt(sdt) => {
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
      w::RunChoice::WFootnoteReference(reference) if reference.id > 0 => {
        footnotes.push(reference.id);
      }
      w::RunChoice::WEndnoteReference(reference) if reference.id > 0 => {
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
      w::SimpleFieldChoice::WR(run) => collect_run_note_reference_ids(run, footnotes, endnotes),
      w::SimpleFieldChoice::WFldSimple(field) => {
        collect_simple_field_note_reference_ids(field, footnotes, endnotes);
      }
      w::SimpleFieldChoice::WHyperlink(hyperlink) => {
        collect_hyperlink_note_reference_ids(hyperlink, footnotes, endnotes);
      }
      w::SimpleFieldChoice::WSdt(sdt) => {
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
      w::HyperlinkChoice::WR(run) => collect_run_note_reference_ids(run, footnotes, endnotes),
      w::HyperlinkChoice::WFldSimple(field) => {
        collect_simple_field_note_reference_ids(field, footnotes, endnotes);
      }
      w::HyperlinkChoice::WHyperlink(hyperlink) => {
        collect_hyperlink_note_reference_ids(hyperlink, footnotes, endnotes);
      }
      w::HyperlinkChoice::WSdt(sdt) => {
        collect_sdt_run_note_reference_ids(sdt, footnotes, endnotes);
      }
      w::HyperlinkChoice::WIns(inserted) => {
        collect_inserted_run_note_reference_ids(inserted, footnotes, endnotes);
      }
      w::HyperlinkChoice::WDel(deleted) => {
        collect_deleted_run_note_reference_ids(deleted, footnotes, endnotes);
      }
      w::HyperlinkChoice::WMoveFrom(moved) => {
        collect_move_from_run_note_reference_ids(moved, footnotes, endnotes);
      }
      w::HyperlinkChoice::WMoveTo(moved) => {
        collect_move_to_run_note_reference_ids(moved, footnotes, endnotes);
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
      w::SdtContentRunChoice::WR(run) => collect_run_note_reference_ids(run, footnotes, endnotes),
      w::SdtContentRunChoice::WFldSimple(field) => {
        collect_simple_field_note_reference_ids(field, footnotes, endnotes);
      }
      w::SdtContentRunChoice::WHyperlink(hyperlink) => {
        collect_hyperlink_note_reference_ids(hyperlink, footnotes, endnotes);
      }
      w::SdtContentRunChoice::WSdt(sdt) => {
        collect_sdt_run_note_reference_ids(sdt, footnotes, endnotes);
      }
      w::SdtContentRunChoice::WIns(inserted) => {
        collect_inserted_run_note_reference_ids(inserted, footnotes, endnotes);
      }
      w::SdtContentRunChoice::WDel(deleted) => {
        collect_deleted_run_note_reference_ids(deleted, footnotes, endnotes);
      }
      w::SdtContentRunChoice::WMoveFrom(moved) => {
        collect_move_from_run_note_reference_ids(moved, footnotes, endnotes);
      }
      w::SdtContentRunChoice::WMoveTo(moved) => {
        collect_move_to_run_note_reference_ids(moved, footnotes, endnotes);
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
      w::InsertedRunChoice::WR(run) => collect_run_note_reference_ids(run, footnotes, endnotes),
      w::InsertedRunChoice::Choice(choice) => match choice.as_ref() {
        w::InsertedRunChoice2::WIns(inserted) => {
          collect_inserted_run_note_reference_ids(inserted, footnotes, endnotes);
        }
        w::InsertedRunChoice2::WDel(deleted) => {
          collect_deleted_run_note_reference_ids(deleted, footnotes, endnotes);
        }
        w::InsertedRunChoice2::WMoveFrom(moved) => {
          collect_move_from_run_note_reference_ids(moved, footnotes, endnotes);
        }
        w::InsertedRunChoice2::WMoveTo(moved) => {
          collect_move_to_run_note_reference_ids(moved, footnotes, endnotes);
        }
        _ => {}
      },
      _ => {}
    }
  }
}

fn collect_deleted_run_note_reference_ids(
  deleted: &w::DeletedRun,
  footnotes: &mut Vec<i64>,
  endnotes: &mut Vec<i64>,
) {
  for choice in &deleted.deleted_run_choice {
    match choice {
      w::DeletedRunChoice::WR(run) => collect_run_note_reference_ids(run, footnotes, endnotes),
      w::DeletedRunChoice::Choice(choice) => match choice.as_ref() {
        w::DeletedRunChoice2::WIns(inserted) => {
          collect_inserted_run_note_reference_ids(inserted, footnotes, endnotes);
        }
        w::DeletedRunChoice2::WDel(deleted) => {
          collect_deleted_run_note_reference_ids(deleted, footnotes, endnotes);
        }
        w::DeletedRunChoice2::WMoveFrom(moved) => {
          collect_move_from_run_note_reference_ids(moved, footnotes, endnotes);
        }
        w::DeletedRunChoice2::WMoveTo(moved) => {
          collect_move_to_run_note_reference_ids(moved, footnotes, endnotes);
        }
        _ => {}
      },
      _ => {}
    }
  }
}

fn collect_move_from_run_note_reference_ids(
  moved: &w::MoveFromRun,
  footnotes: &mut Vec<i64>,
  endnotes: &mut Vec<i64>,
) {
  for choice in &moved.move_from_run_choice {
    match choice {
      w::MoveFromRunChoice::WR(run) => collect_run_note_reference_ids(run, footnotes, endnotes),
      w::MoveFromRunChoice::Choice(choice) => match choice.as_ref() {
        w::MoveFromRunChoice2::WIns(inserted) => {
          collect_inserted_run_note_reference_ids(inserted, footnotes, endnotes);
        }
        w::MoveFromRunChoice2::WDel(deleted) => {
          collect_deleted_run_note_reference_ids(deleted, footnotes, endnotes);
        }
        w::MoveFromRunChoice2::WMoveFrom(moved) => {
          collect_move_from_run_note_reference_ids(moved, footnotes, endnotes);
        }
        w::MoveFromRunChoice2::WMoveTo(moved) => {
          collect_move_to_run_note_reference_ids(moved, footnotes, endnotes);
        }
        _ => {}
      },
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
      w::MoveToRunChoice::WR(run) => collect_run_note_reference_ids(run, footnotes, endnotes),
      w::MoveToRunChoice::Choice(choice) => match choice.as_ref() {
        w::MoveToRunChoice2::WIns(inserted) => {
          collect_inserted_run_note_reference_ids(inserted, footnotes, endnotes);
        }
        w::MoveToRunChoice2::WDel(deleted) => {
          collect_deleted_run_note_reference_ids(deleted, footnotes, endnotes);
        }
        w::MoveToRunChoice2::WMoveFrom(moved) => {
          collect_move_from_run_note_reference_ids(moved, footnotes, endnotes);
        }
        w::MoveToRunChoice2::WMoveTo(moved) => {
          collect_move_to_run_note_reference_ids(moved, footnotes, endnotes);
        }
        _ => {}
      },
      _ => {}
    }
  }
}

fn push_simple_field(
  field: &w::SimpleField,
  inlines: &mut Vec<InlineItem>,
  base_style: TextStyle,
  styles: &StylesCatalog,
  images: &ImageCatalog,
  hyperlinks: &HyperlinkCatalog,
  form_widget_ids: &mut FormWidgetIdAllocator,
) {
  if let Some(kind) = dynamic_field_kind(&field.instruction) {
    push_dynamic_field(inlines, kind, base_style, None);
    return;
  }

  let mut inline_context = InlineImportContext {
    styles,
    images,
    hyperlinks,
    form_widget_ids,
  };
  for choice in &field.simple_field_choice {
    match choice {
      w::SimpleFieldChoice::WR(run) => push_run(
        run,
        inlines,
        base_style.clone(),
        styles,
        images,
        hyperlinks,
        None,
      ),
      w::SimpleFieldChoice::WHyperlink(hyperlink) => {
        let mut complex_field = None;
        push_hyperlink_content(
          hyperlink,
          inlines,
          base_style.clone(),
          None,
          &mut inline_context,
          &mut complex_field,
        );
        flush_unclosed_complex_field(inlines, &mut complex_field);
      }
      w::SimpleFieldChoice::WFldSimple(field) => {
        push_simple_field(
          field,
          inlines,
          base_style.clone(),
          inline_context.styles,
          inline_context.images,
          inline_context.hyperlinks,
          inline_context.form_widget_ids,
        );
      }
      w::SimpleFieldChoice::WSdt(sdt) => {
        push_sdt_run(sdt, inlines, base_style.clone(), None, &mut inline_context)
      }
      _ => {}
    }
  }
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
  if style.hidden {
    return;
  }
  let mut text = String::new();

  for choice in &run.run_choice {
    match choice {
      w::RunChoice::WT(text_node) => {
        if let Some(content) = &text_node.xml_content {
          text.push_str(content);
        }
      }
      w::RunChoice::WDelText(text_node) => {
        if let Some(content) = &text_node.xml_content {
          text.push_str(content);
        }
      }
      w::RunChoice::WTab => text.push('\t'),
      w::RunChoice::WCr => text.push('\n'),
      w::RunChoice::WBr(br) => match br.r#type {
        Some(w::BreakValues::Page) => {
          flush_run_text(inlines, &mut text, style.clone(), hyperlink_url);
          inlines.push(InlineItem::PageBreak);
        }
        Some(w::BreakValues::Column) => {
          flush_run_text(inlines, &mut text, style.clone(), hyperlink_url);
          inlines.push(InlineItem::ColumnBreak);
        }
        Some(w::BreakValues::TextWrapping) | None => text.push('\n'),
      },
      // This is a cached layout artifact from Word, not an author-authored break.
      w::RunChoice::WLastRenderedPageBreak => {}
      w::RunChoice::WSym(symbol) => {
        if let Some(symbol) = symbol_text(symbol) {
          text.push(symbol);
        }
      }
      w::RunChoice::WPgNum => {
        flush_run_text(inlines, &mut text, style.clone(), hyperlink_url);
        push_dynamic_field(
          inlines,
          DynamicFieldKind::Page,
          style.clone(),
          hyperlink_url,
        );
      }
      w::RunChoice::WNoBreakHyphen => text.push('\u{2011}'),
      w::RunChoice::WSoftHyphen => text.push('\u{00ad}'),
      w::RunChoice::WFootnoteReference(reference) => {
        flush_run_text(inlines, &mut text, style.clone(), hyperlink_url);
        push_note_reference(
          inlines,
          reference.id,
          style.clone(),
          Some(note_reference_url("footnote", reference.id)),
        );
      }
      w::RunChoice::WEndnoteReference(reference) => {
        flush_run_text(inlines, &mut text, style.clone(), hyperlink_url);
        push_note_reference(
          inlines,
          reference.id,
          style.clone(),
          Some(note_reference_url("endnote", reference.id)),
        );
      }
      w::RunChoice::WCommentReference(reference) => {
        flush_run_text(inlines, &mut text, style.clone(), hyperlink_url);
        push_comment_reference(inlines, &reference.id, style.clone());
      }
      w::RunChoice::WDrawing(drawing) => {
        flush_run_text(inlines, &mut text, style.clone(), hyperlink_url);
        if let Some(image) = drawing::inline_image(drawing, images, hyperlinks) {
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
      w::RunChoice::WPict(picture) => {
        flush_run_text(inlines, &mut text, style.clone(), hyperlink_url);
        if let Some(image) = drawing::pict_image(picture, images) {
          inlines.push(InlineItem::Image(image));
        }
        drawing::push_pict_shapes(picture, inlines);
        drawing::push_pict_textboxes(
          picture,
          inlines,
          base_style.clone(),
          styles,
          images,
          hyperlinks,
        );
      }
      w::RunChoice::WPtab(_) => text.push('\t'),
      w::RunChoice::XmlAny(xml) => {
        flush_run_text(inlines, &mut text, style.clone(), hyperlink_url);
        push_run_xml_any(
          xml,
          inlines,
          base_style.clone(),
          style.clone(),
          styles,
          images,
          hyperlinks,
        );
      }
      w::RunChoice::WRuby(ruby) => {
        flush_run_text(inlines, &mut text, style.clone(), hyperlink_url);
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

  flush_run_text(inlines, &mut text, style, hyperlink_url);
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
      w::RubyBaseChoice::WR(run) => push_run(
        run,
        inlines,
        base_style.clone(),
        styles,
        images,
        hyperlinks,
        hyperlink_url,
      ),
      w::RubyBaseChoice::WIns(inserted) => {
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
      w::RubyBaseChoice::WDel(deleted) => {
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
      w::RubyBaseChoice::WMoveFrom(moved) => {
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
      w::RubyBaseChoice::WMoveTo(moved) => {
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
  let widget_id = sdt
    .sdt_properties
    .as_ref()
    .and_then(sdt_form_widget)
    .map(|(kind, entries)| context.form_widget_ids.next_widget(kind, entries));
  if let Some(widget_id) = widget_id {
    inlines.push(InlineItem::FormWidgetStart(widget_id));
  }

  for choice in &content.sdt_content_run_choice {
    match choice {
      w::SdtContentRunChoice::WR(run) => push_run(
        run.as_ref(),
        inlines,
        base_style.clone(),
        context.styles,
        context.images,
        context.hyperlinks,
        hyperlink_url,
      ),
      w::SdtContentRunChoice::WFldSimple(field) => {
        push_simple_field(
          field.as_ref(),
          inlines,
          base_style.clone(),
          context.styles,
          context.images,
          context.hyperlinks,
          context.form_widget_ids,
        );
      }
      w::SdtContentRunChoice::WHyperlink(hyperlink) => {
        let mut complex_field = None;
        push_hyperlink_content(
          hyperlink,
          inlines,
          base_style.clone(),
          hyperlink_url,
          context,
          &mut complex_field,
        );
        flush_unclosed_complex_field(inlines, &mut complex_field);
      }
      w::SdtContentRunChoice::WSdt(sdt) => push_sdt_run(
        sdt.as_ref(),
        inlines,
        base_style.clone(),
        hyperlink_url,
        context,
      ),
      w::SdtContentRunChoice::WIns(inserted) => {
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
      w::SdtContentRunChoice::WDel(deleted) => {
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
      w::SdtContentRunChoice::WMoveFrom(moved) => {
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
      w::SdtContentRunChoice::WMoveTo(moved) => {
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
  if let Some(widget_id) = widget_id {
    inlines.push(InlineItem::FormWidgetEnd(widget_id));
  }
}

fn sdt_form_widget(properties: &w::SdtProperties) -> Option<(FormWidgetKind, Vec<String>)> {
  let mut kind = FormWidgetKind::Text;
  let mut entries = Vec::new();
  for choice in &properties.sdt_properties_choice {
    match choice {
      w::SdtPropertiesChoice::WComboBox(combo_box) => {
        kind = FormWidgetKind::ComboBox;
        entries = sdt_list_item_display_texts(&combo_box.w_list_item);
      }
      w::SdtPropertiesChoice::WDropDownList(drop_down) => {
        kind = FormWidgetKind::DropDownList;
        entries = sdt_list_item_display_texts(&drop_down.w_list_item);
      }
      w::SdtPropertiesChoice::WDate(_) => {
        kind = FormWidgetKind::Text;
      }
      w::SdtPropertiesChoice::WRichText | w::SdtPropertiesChoice::WText(_) => {
        kind = FormWidgetKind::Text;
      }
      _ => {}
    }
  }
  Some((kind, entries))
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

fn push_run_xml_any(
  xml: &str,
  inlines: &mut Vec<InlineItem>,
  base_style: TextStyle,
  style: TextStyle,
  styles: &StylesCatalog,
  images: &ImageCatalog,
  hyperlinks: &HyperlinkCatalog,
) {
  if let Ok(drawing) = w::Drawing::from_bytes(xml.as_bytes()) {
    if let Some(image) = drawing::inline_image(&drawing, images, hyperlinks) {
      inlines.push(InlineItem::Image(image));
    }
    drawing::push_drawing_shapes(&drawing, inlines, styles, images, hyperlinks);
    drawing::push_drawing_textboxes(&drawing, inlines, style, styles, images, hyperlinks);
    return;
  }

  if let Ok(picture) = w::Picture::from_bytes(xml.as_bytes()) {
    if let Some(image) = drawing::pict_image(&picture, images) {
      inlines.push(InlineItem::Image(image));
    }
    drawing::push_pict_shapes(&picture, inlines);
    drawing::push_pict_textboxes(&picture, inlines, base_style, styles, images, hyperlinks);
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
  let mut redline_style = base_style;
  redline_style.color = redline_author_color();
  for choice in &inserted.inserted_run_choice {
    match choice {
      w::InsertedRunChoice::WR(run) => push_run(
        run,
        inlines,
        redline_style.clone(),
        styles,
        images,
        hyperlinks,
        hyperlink_url,
      ),
      w::InsertedRunChoice::Choice(choice) => match choice.as_ref() {
        w::InsertedRunChoice2::WIns(nested) => {
          push_inserted_run(
            nested,
            inlines,
            redline_style.clone(),
            styles,
            images,
            hyperlinks,
            hyperlink_url,
          );
        }
        w::InsertedRunChoice2::WDel(deleted) => {
          push_deleted_run(
            deleted,
            inlines,
            redline_style.clone(),
            styles,
            images,
            hyperlinks,
            hyperlink_url,
          );
        }
        w::InsertedRunChoice2::WMoveFrom(moved) => {
          push_move_from_run(
            moved,
            inlines,
            redline_style.clone(),
            styles,
            images,
            hyperlinks,
            hyperlink_url,
          );
        }
        w::InsertedRunChoice2::WMoveTo(moved) => {
          push_move_to_run(
            moved,
            inlines,
            redline_style.clone(),
            styles,
            images,
            hyperlinks,
            hyperlink_url,
          );
        }
        _ => {}
      },
      _ => {}
    }
  }
}

fn push_deleted_run(
  deleted: &w::DeletedRun,
  inlines: &mut Vec<InlineItem>,
  mut base_style: TextStyle,
  styles: &StylesCatalog,
  images: &ImageCatalog,
  hyperlinks: &HyperlinkCatalog,
  hyperlink_url: Option<&str>,
) {
  base_style.color = redline_author_color();
  for choice in &deleted.deleted_run_choice {
    match choice {
      w::DeletedRunChoice::WR(run) => push_run(
        run,
        inlines,
        base_style.clone(),
        styles,
        images,
        hyperlinks,
        hyperlink_url,
      ),
      w::DeletedRunChoice::Choice(choice) => match choice.as_ref() {
        w::DeletedRunChoice2::WIns(inserted) => push_inserted_run(
          inserted,
          inlines,
          base_style.clone(),
          styles,
          images,
          hyperlinks,
          hyperlink_url,
        ),
        w::DeletedRunChoice2::WDel(deleted) => push_deleted_run(
          deleted,
          inlines,
          base_style.clone(),
          styles,
          images,
          hyperlinks,
          hyperlink_url,
        ),
        w::DeletedRunChoice2::WMoveFrom(moved) => push_move_from_run(
          moved,
          inlines,
          base_style.clone(),
          styles,
          images,
          hyperlinks,
          hyperlink_url,
        ),
        w::DeletedRunChoice2::WMoveTo(moved) => push_move_to_run(
          moved,
          inlines,
          base_style.clone(),
          styles,
          images,
          hyperlinks,
          hyperlink_url,
        ),
        _ => {}
      },
      _ => {}
    }
  }
}

fn push_move_from_run(
  moved: &w::MoveFromRun,
  inlines: &mut Vec<InlineItem>,
  mut base_style: TextStyle,
  styles: &StylesCatalog,
  images: &ImageCatalog,
  hyperlinks: &HyperlinkCatalog,
  hyperlink_url: Option<&str>,
) {
  base_style.color = redline_author_color();
  base_style.strikethrough = true;
  for choice in &moved.move_from_run_choice {
    match choice {
      w::MoveFromRunChoice::WR(run) => push_run(
        run,
        inlines,
        base_style.clone(),
        styles,
        images,
        hyperlinks,
        hyperlink_url,
      ),
      w::MoveFromRunChoice::Choice(choice) => match choice.as_ref() {
        w::MoveFromRunChoice2::WIns(inserted) => push_inserted_run(
          inserted,
          inlines,
          base_style.clone(),
          styles,
          images,
          hyperlinks,
          hyperlink_url,
        ),
        w::MoveFromRunChoice2::WDel(deleted) => push_deleted_run(
          deleted,
          inlines,
          base_style.clone(),
          styles,
          images,
          hyperlinks,
          hyperlink_url,
        ),
        w::MoveFromRunChoice2::WMoveFrom(moved) => push_move_from_run(
          moved,
          inlines,
          base_style.clone(),
          styles,
          images,
          hyperlinks,
          hyperlink_url,
        ),
        w::MoveFromRunChoice2::WMoveTo(moved) => push_move_to_run(
          moved,
          inlines,
          base_style.clone(),
          styles,
          images,
          hyperlinks,
          hyperlink_url,
        ),
        _ => {}
      },
      _ => {}
    }
  }
}

fn push_move_to_run(
  moved: &w::MoveToRun,
  inlines: &mut Vec<InlineItem>,
  mut base_style: TextStyle,
  styles: &StylesCatalog,
  images: &ImageCatalog,
  hyperlinks: &HyperlinkCatalog,
  hyperlink_url: Option<&str>,
) {
  base_style.color = moved_redline_color();
  for choice in &moved.move_to_run_choice {
    match choice {
      w::MoveToRunChoice::WR(run) => push_run(
        run,
        inlines,
        base_style.clone(),
        styles,
        images,
        hyperlinks,
        hyperlink_url,
      ),
      w::MoveToRunChoice::Choice(choice) => match choice.as_ref() {
        w::MoveToRunChoice2::WIns(inserted) => push_inserted_run(
          inserted,
          inlines,
          base_style.clone(),
          styles,
          images,
          hyperlinks,
          hyperlink_url,
        ),
        w::MoveToRunChoice2::WDel(deleted) => push_deleted_run(
          deleted,
          inlines,
          base_style.clone(),
          styles,
          images,
          hyperlinks,
          hyperlink_url,
        ),
        w::MoveToRunChoice2::WMoveFrom(moved) => push_move_from_run(
          moved,
          inlines,
          base_style.clone(),
          styles,
          images,
          hyperlinks,
          hyperlink_url,
        ),
        w::MoveToRunChoice2::WMoveTo(moved) => push_move_to_run(
          moved,
          inlines,
          base_style.clone(),
          styles,
          images,
          hyperlinks,
          hyperlink_url,
        ),
        _ => {}
      },
      _ => {}
    }
  }
}

pub(super) fn redline_author_color() -> RgbColor {
  RgbColor {
    r: 0xff,
    g: 0x00,
    b: 0x00,
  }
}

fn moved_redline_color() -> RgbColor {
  RgbColor {
    r: 0x00,
    g: 0x80,
    b: 0x00,
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

fn push_comment_reference(inlines: &mut Vec<InlineItem>, id: &str, style: TextStyle) {
  inlines.push(InlineItem::Text(TextRun {
    text: format!("[{id}]"),
    style: TextStyle {
      font_size_pt: (style.font_size_pt * COMMENT_REFERENCE_FONT_SCALE)
        .max(MIN_ESCAPEMENT_FONT_SIZE_PT),
      color: RgbColor {
        r: 0x80,
        g: 0x40,
        b: 0x00,
      },
      ..style
    },
    hyperlink_url: None,
    dynamic_field: None,
  }));
}

fn flush_run_text(
  inlines: &mut Vec<InlineItem>,
  text: &mut String,
  style: TextStyle,
  hyperlink_url: Option<&str>,
) {
  if !text.is_empty() {
    inlines.push(InlineItem::Text(TextRun {
      text: run_display_text(std::mem::take(text), style.clone()),
      style,
      hyperlink_url: hyperlink_url.map(ToString::to_string),
      dynamic_field: None,
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
  let low_byte = code & 0xFF;
  let font = symbol.font.as_deref().unwrap_or("").to_ascii_lowercase();

  if font.contains("wingdings") {
    return wingdings_symbol(low_byte).or_else(|| char::from_u32(code));
  }
  if font == "symbol" || font.ends_with(" symbol") {
    return symbol_font_symbol(low_byte).or_else(|| char::from_u32(code));
  }

  char::from_u32(code).or_else(|| {
    if (0xF000..=0xF0FF).contains(&code) {
      char::from_u32(low_byte)
    } else {
      None
    }
  })
}

fn symbol_font_symbol(code: u32) -> Option<char> {
  Some(match code {
    0x41 => 'Α',
    0x42 => 'Β',
    0x43 => 'Χ',
    0x44 => 'Δ',
    0x45 => 'Ε',
    0x46 => 'Φ',
    0x47 => 'Γ',
    0x48 => 'Η',
    0x49 => 'Ι',
    0x4A => 'ϑ',
    0x4B => 'Κ',
    0x4C => 'Λ',
    0x4D => 'Μ',
    0x4E => 'Ν',
    0x4F => 'Ο',
    0x50 => 'Π',
    0x51 => 'Θ',
    0x52 => 'Ρ',
    0x53 => 'Σ',
    0x54 => 'Τ',
    0x55 => 'Υ',
    0x56 => 'ς',
    0x57 => 'Ω',
    0x58 => 'Ξ',
    0x59 => 'Ψ',
    0x5A => 'Ζ',
    0x61 => 'α',
    0x62 => 'β',
    0x63 => 'χ',
    0x64 => 'δ',
    0x65 => 'ε',
    0x66 => 'φ',
    0x67 => 'γ',
    0x68 => 'η',
    0x69 => 'ι',
    0x6A => 'ϕ',
    0x6B => 'κ',
    0x6C => 'λ',
    0x6D => 'μ',
    0x6E => 'ν',
    0x6F => 'ο',
    0x70 => 'π',
    0x71 => 'θ',
    0x72 => 'ρ',
    0x73 => 'σ',
    0x74 => 'τ',
    0x75 => 'υ',
    0x76 => 'ϖ',
    0x77 => 'ω',
    0x78 => 'ξ',
    0x79 => 'ψ',
    0x7A => 'ζ',
    0xA2 => '′',
    0xA3 => '≤',
    0xA5 => '∞',
    0xA7 => '♣',
    0xA8 => '♦',
    0xA9 => '♥',
    0xAA => '♠',
    0xB1 => '±',
    0xB4 => '×',
    0xB5 => '∝',
    0xB6 => '∂',
    0xB7 => '•',
    0xB8 => '÷',
    0xB9 => '≠',
    0xBA => '≡',
    0xBB => '≈',
    0xBC => '…',
    0xBD => '⏐',
    0xBE => '⎯',
    0xBF => '↵',
    0xC0 => 'ℵ',
    0xC1 => 'ℑ',
    0xC2 => 'ℜ',
    0xC3 => '℘',
    0xC4 => '⊗',
    0xC5 => '⊕',
    0xC6 => '∅',
    0xC7 => '∩',
    0xC8 => '∪',
    0xC9 => '⊃',
    0xCA => '⊇',
    0xCB => '⊄',
    0xCC => '⊂',
    0xCD => '⊆',
    0xCE => '∈',
    0xCF => '∉',
    0xD0 => '∠',
    0xD1 => '∇',
    0xD2 => '®',
    0xD3 => '©',
    0xD4 => '™',
    0xD5 => '∏',
    0xD6 => '√',
    0xD7 => '⋅',
    0xD8 => '¬',
    0xD9 => '∧',
    0xDA => '∨',
    0xDB => '⇔',
    0xDC => '⇐',
    0xDD => '⇑',
    0xDE => '⇒',
    0xDF => '⇓',
    0xE0 => '◊',
    0xE1 => '〈',
    0xE2 => '®',
    0xE3 => '©',
    0xE4 => '™',
    0xE5 => '∑',
    0xE6 => '⎛',
    0xE7 => '⎜',
    0xE8 => '⎝',
    0xE9 => '⎡',
    0xEA => '⎢',
    0xEB => '⎣',
    0xEC => '⎧',
    0xED => '⎨',
    0xEE => '⎩',
    0xEF => '⎪',
    0xF1 => '〉',
    0xF2 => '∫',
    0xF3 => '⌠',
    0xF4 => '⎮',
    0xF5 => '⌡',
    0xF6 => '⎞',
    0xF7 => '⎟',
    0xF8 => '⎠',
    0xF9 => '⎤',
    0xFA => '⎥',
    0xFB => '⎦',
    0xFC => '⎫',
    0xFD => '⎬',
    0xFE => '⎭',
    _ => return char::from_u32(code),
  })
}

fn wingdings_symbol(code: u32) -> Option<char> {
  Some(match code {
    0x4A => '☺',
    0x4C => '●',
    0x6C => '●',
    0x6D => '■',
    0x6E => '□',
    0x71 => '❑',
    0x72 => '❒',
    0x73 => '⬧',
    0x74 => '◆',
    0x75 => '❖',
    0x76 => '⬥',
    0x77 => '⌧',
    0x78 => '⌦',
    0x9F => '•',
    0xA8 => '◻',
    0xF0 => '➔',
    0xFC => '✓',
    0xFD => '☒',
    0xFE => '☑',
    _ => return None,
  })
}

fn inline_image_impl(
  drawing: &w::Drawing,
  images: &ImageCatalog,
  hyperlinks: &HyperlinkCatalog,
) -> Option<InlineImage> {
  if drawing_is_hidden(drawing) {
    return None;
  }

  match drawing.drawing_choice.as_ref()? {
    w::DrawingChoice::WpInline(inline) => {
      let properties = drawing_image_properties(&inline.graphic.graphic_data)?;
      let relationship_id = properties.relationship_id?;
      let resource = images.by_relationship_id.get(&relationship_id)?;
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
        data: resource.data.clone(),
        content_type: resource.content_type.clone(),
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
    w::DrawingChoice::WpAnchor(anchor) => {
      let graphic = anchor.a_graphic.as_ref()?;
      let extent = anchor.extent.as_ref()?;
      let properties = drawing_image_properties(&graphic.graphic_data)?;
      let relationship_id = properties.relationship_id?;
      let resource = images.by_relationship_id.get(&relationship_id)?;
      let hyperlink_url = anchor
        .wp_doc_pr
        .as_ref()
        .and_then(|doc_pr| doc_pr.hyperlink_on_click.as_deref())
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
        data: resource.data.clone(),
        content_type: resource.content_type.clone(),
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
          .wp_doc_pr
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
    .map(|extent| units::emu_to_points(extent.left_edge))
    .unwrap_or(0.0)
}

fn effect_extent_top(extent: Option<&wp::EffectExtent>) -> f32 {
  extent
    .map(|extent| units::emu_to_points(extent.top_edge))
    .unwrap_or(0.0)
}

fn effect_extent_right(extent: Option<&wp::EffectExtent>) -> f32 {
  extent
    .map(|extent| units::emu_to_points(extent.right_edge))
    .unwrap_or(0.0)
}

fn effect_extent_bottom(extent: Option<&wp::EffectExtent>) -> f32 {
  extent
    .map(|extent| units::emu_to_points(extent.bottom_edge))
    .unwrap_or(0.0)
}

fn floating_image_placement(anchor: &wp::Anchor) -> FloatingImagePlacement {
  let margins = floating_wrap_margins(anchor);
  let simple_position = anchor
    .simple_pos
    .as_ref()
    .is_some_and(|value| value.as_bool())
    .then_some(anchor.simple_position.as_ref())
    .flatten();
  FloatingImagePlacement {
    horizontal_relative_to: simple_position
      .map(|_| HorizontalImageReference::Page)
      .or_else(|| {
        anchor
          .horizontal_position
          .as_deref()
          .map(horizontal_image_reference)
      })
      .unwrap_or_default(),
    vertical_relative_to: simple_position
      .map(|_| VerticalImageReference::Page)
      .or_else(|| {
        anchor
          .vertical_position
          .as_deref()
          .map(vertical_image_reference)
      })
      .unwrap_or_default(),
    horizontal_alignment: simple_position.map(|_| None).unwrap_or_else(|| {
      anchor
        .horizontal_position
        .as_deref()
        .and_then(horizontal_position_alignment)
    }),
    vertical_alignment: simple_position.map(|_| None).unwrap_or_else(|| {
      anchor
        .vertical_position
        .as_deref()
        .and_then(vertical_position_alignment)
    }),
    horizontal_offset_pt: simple_position
      .map(|position| units::emu_to_points(position.x))
      .or_else(|| {
        anchor
          .horizontal_position
          .as_deref()
          .and_then(horizontal_position_offset)
      })
      .unwrap_or(0.0),
    vertical_offset_pt: simple_position
      .map(|position| units::emu_to_points(position.y))
      .or_else(|| {
        anchor
          .vertical_position
          .as_deref()
          .and_then(vertical_position_offset)
      })
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
    layout_in_cell: anchor.layout_in_cell.as_bool(),
    allow_overlap: anchor.allow_overlap.as_bool(),
    relative_height: anchor.relative_height,
    relative_width_to: anchor
      .wp14_size_rel_h
      .as_ref()
      .map(|relative| relative_width_reference(relative.object_id)),
    relative_width_pct: anchor
      .wp14_size_rel_h
      .as_ref()
      .map(|relative| relative.percentage_width as f32 / units::DRAWINGML_PERCENT_SCALE),
    relative_height_to: anchor
      .wp14_size_rel_v
      .as_ref()
      .map(|relative| relative_height_reference(relative.relative_from)),
    relative_height_pct: anchor
      .wp14_size_rel_v
      .as_ref()
      .map(|relative| relative.percentage_height as f32 / units::DRAWINGML_PERCENT_SCALE),
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
    Some(wp::AnchorChoice::WpWrapNone)
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
    Some(wp::AnchorChoice::WpWrapSquare(square)) => {
      margins.top_pt = optional_emu_to_points(square.distance_from_top).max(margins.top_pt);
      margins.right_pt = optional_emu_to_points(square.distance_from_right).max(margins.right_pt);
      margins.bottom_pt =
        optional_emu_to_points(square.distance_from_bottom).max(margins.bottom_pt);
      margins.left_pt = optional_emu_to_points(square.distance_from_left).max(margins.left_pt);
    }
    Some(wp::AnchorChoice::WpWrapTight(tight)) => {
      margins.right_pt = optional_emu_to_points(tight.distance_from_right).max(margins.right_pt);
      margins.left_pt = optional_emu_to_points(tight.distance_from_left).max(margins.left_pt);
    }
    Some(wp::AnchorChoice::WpWrapThrough(through)) => {
      margins.right_pt = optional_emu_to_points(through.distance_from_right).max(margins.right_pt);
      margins.left_pt = optional_emu_to_points(through.distance_from_left).max(margins.left_pt);
    }
    Some(wp::AnchorChoice::WpWrapTopAndBottom(top_bottom)) => {
      margins.top_pt = optional_emu_to_points(top_bottom.distance_from_top).max(margins.top_pt);
      margins.bottom_pt =
        optional_emu_to_points(top_bottom.distance_from_bottom).max(margins.bottom_pt);
    }
    Some(wp::AnchorChoice::WpWrapNone) | None => {}
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
    wp::HorizontalPositionChoice::WpPosOffset(offset) => Some(units::emu_to_points(*offset as i64)),
    wp::HorizontalPositionChoice::WpAlign(_)
    | wp::HorizontalPositionChoice::Wp14PctPosHOffset(_) => None,
  }
}

fn horizontal_position_alignment(
  position: &wp::HorizontalPosition,
) -> Option<HorizontalImageAlignment> {
  match position.horizontal_position_choice.as_ref()? {
    wp::HorizontalPositionChoice::WpAlign(alignment) => match alignment {
      wp::HorizontalAlignmentValues::Left => Some(HorizontalImageAlignment::Left),
      wp::HorizontalAlignmentValues::Center => Some(HorizontalImageAlignment::Center),
      wp::HorizontalAlignmentValues::Right => Some(HorizontalImageAlignment::Right),
      wp::HorizontalAlignmentValues::Inside => Some(HorizontalImageAlignment::Inside),
      wp::HorizontalAlignmentValues::Outside => Some(HorizontalImageAlignment::Outside),
    },
    wp::HorizontalPositionChoice::WpPosOffset(_)
    | wp::HorizontalPositionChoice::Wp14PctPosHOffset(_) => None,
  }
}

fn vertical_position_offset(position: &wp::VerticalPosition) -> Option<f32> {
  match position.vertical_position_choice.as_ref()? {
    wp::VerticalPositionChoice::WpPosOffset(offset) => Some(units::emu_to_points(*offset as i64)),
    wp::VerticalPositionChoice::WpAlign(_) | wp::VerticalPositionChoice::Wp14PctPosVOffset(_) => {
      None
    }
  }
}

fn vertical_position_alignment(position: &wp::VerticalPosition) -> Option<VerticalImageAlignment> {
  match position.vertical_position_choice.as_ref()? {
    wp::VerticalPositionChoice::WpAlign(alignment) => match alignment {
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
    wp::VerticalPositionChoice::WpPosOffset(_)
    | wp::VerticalPositionChoice::Wp14PctPosVOffset(_) => None,
  }
}

fn image_wrap_mode(choice: &wp::AnchorChoice) -> ImageWrapMode {
  match choice {
    wp::AnchorChoice::WpWrapNone => ImageWrapMode::Through,
    wp::AnchorChoice::WpWrapSquare(_) => ImageWrapMode::Square,
    wp::AnchorChoice::WpWrapTight(_) => ImageWrapMode::Tight,
    wp::AnchorChoice::WpWrapThrough(_) => ImageWrapMode::Square,
    wp::AnchorChoice::WpWrapTopAndBottom(_) => ImageWrapMode::TopBottom,
  }
}

fn image_wrap_side(choice: &wp::AnchorChoice) -> ImageWrapSide {
  match choice {
    wp::AnchorChoice::WpWrapSquare(square) => wrap_text_side(square.wrap_text),
    wp::AnchorChoice::WpWrapTight(tight) => wrap_text_side(tight.wrap_text),
    wp::AnchorChoice::WpWrapThrough(through) => wrap_text_side(through.wrap_text),
    wp::AnchorChoice::WpWrapNone | wp::AnchorChoice::WpWrapTopAndBottom(_) => {
      ImageWrapSide::BothSides
    }
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

  let placement = match drawing.drawing_choice.as_ref() {
    Some(w::DrawingChoice::WpInline(_)) => ImagePlacement::Inline,
    Some(w::DrawingChoice::WpAnchor(anchor)) => {
      ImagePlacement::Floating(floating_image_placement(anchor))
    }
    None => return,
  };

  for child in &graphic_data.xml_children {
    let text_box_frames = drawingml_textbox_frames_from_xml(
      child,
      placement,
      DrawingMlGroupTransform::identity(),
      styles,
      images,
      hyperlinks,
      false,
    );
    if !text_box_frames.is_empty() {
      inlines.extend(text_box_frames.into_iter().map(InlineItem::Shape));
      continue;
    }
    if let Some(content) = drawing_textbox_content(child) {
      push_textbox_content(
        &content,
        inlines,
        base_style.clone(),
        styles,
        images,
        hyperlinks,
      );
    } else if let Some(text) = drawing_textbox_text(child) {
      inlines.push(InlineItem::Text(TextRun {
        text,
        style: base_style.clone(),
        hyperlink_url: None,
        dynamic_field: None,
      }));
    }
  }
}

fn drawingml_textbox_frames_from_xml(
  xml: &str,
  placement: ImagePlacement,
  transform: DrawingMlGroupTransform,
  styles: &StylesCatalog,
  images: &ImageCatalog,
  hyperlinks: &HyperlinkCatalog,
  skip_container_root: bool,
) -> Vec<InlineShape> {
  let mut frames = Vec::new();
  let mut reader = Reader::from_str(xml);
  reader.config_mut().trim_text(false);
  let mut skipped_container_root = false;

  loop {
    match reader.read_event() {
      Ok(Event::Start(event)) if skip_container_root && !skipped_container_root => {
        skipped_container_root = true;
        if qname_ends_with(event.name().as_ref(), b"wgp") {
          continue;
        }
      }
      Ok(Event::Start(event)) if qname_ends_with(event.name().as_ref(), b"wgp") => {
        if let Some(fragment) = read_outer_xml_fragment(&mut reader, event) {
          let child_transform = drawingml_group_transform_from_fragment(&fragment)
            .map(|xfrm| transform.child(xfrm))
            .unwrap_or(transform);
          frames.extend(drawingml_textbox_frames_from_xml(
            &fragment,
            placement,
            child_transform,
            styles,
            images,
            hyperlinks,
            true,
          ));
        }
      }
      Ok(Event::Start(event)) if qname_ends_with(event.name().as_ref(), b"wsp") => {
        if let Some(fragment) = read_outer_xml_fragment(&mut reader, event)
          && let Some(frame) = drawingml_textbox_frame_from_fragment(
            &fragment, placement, transform, styles, images, hyperlinks,
          )
        {
          frames.push(frame);
        }
      }
      Ok(Event::Empty(event)) if qname_ends_with(event.name().as_ref(), b"wsp") => {
        let mut writer = Writer::new(Vec::new());
        if writer.write_event(Event::Empty(event.into_owned())).is_ok()
          && let Ok(fragment) = String::from_utf8(writer.into_inner())
          && let Some(frame) = drawingml_textbox_frame_from_fragment(
            &fragment, placement, transform, styles, images, hyperlinks,
          )
        {
          frames.push(frame);
        }
      }
      Ok(Event::Eof) | Err(_) => break,
      _ => {}
    }
  }

  frames
}

fn drawingml_textbox_frame_from_fragment(
  xml: &str,
  placement: ImagePlacement,
  transform: DrawingMlGroupTransform,
  styles: &StylesCatalog,
  images: &ImageCatalog,
  hyperlinks: &HyperlinkCatalog,
) -> Option<InlineShape> {
  let content = drawing_textbox_content(xml)?;
  let text_box = text_box_frame_from_drawingml(xml, &content, styles, images, hyperlinks);
  let (offset_x_pt, offset_y_pt, shape_width_pt, shape_height_pt) = drawingml_shape_geometry(xml)?;
  let (offset_x_pt, offset_y_pt, shape_width_pt, shape_height_pt) =
    transform.rect(offset_x_pt, offset_y_pt, shape_width_pt, shape_height_pt);
  let width_pt =
    (shape_width_pt - text_box.left_pt - text_box.right_pt).max(DEFAULT_TEXTBOX_MIN_WIDTH_PT);
  let height_pt =
    (shape_height_pt - text_box.top_pt - text_box.bottom_pt).max(DEFAULT_TEXTBOX_MIN_HEIGHT_PT);

  Some(InlineShape {
    width_pt,
    height_pt,
    geometry: InlineShapeGeometry::Rectangle,
    offset_x_pt: offset_x_pt + text_box.left_pt,
    offset_y_pt: offset_y_pt + text_box.top_pt,
    fill_color: None,
    stroke: None,
    placement,
    text_box_blocks: text_box.blocks,
    text_inset_left_pt: 0.0,
    text_inset_top_pt: 0.0,
    text_inset_right_pt: 0.0,
    text_inset_bottom_pt: 0.0,
    text_vertical_alignment: text_box.vertical_alignment,
  })
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

fn text_box_frame_from_drawingml(
  xml: &str,
  content: &w::TextBoxContent,
  styles: &StylesCatalog,
  images: &ImageCatalog,
  hyperlinks: &HyperlinkCatalog,
) -> TextBoxFrameContent {
  let mut frame = TextBoxFrameContent::new(textbox_blocks(content, styles, images, hyperlinks));
  if let Some(body_pr) = first_named_xml_fragment(xml, b"bodyPr") {
    apply_drawingml_textbox_body_properties(&body_pr, &mut frame);
  }
  frame
}

fn apply_drawingml_textbox_body_properties(xml: &str, frame: &mut TextBoxFrameContent) {
  let mut reader = Reader::from_str(xml);
  reader.config_mut().trim_text(false);

  loop {
    match reader.read_event() {
      Ok(Event::Start(event)) | Ok(Event::Empty(event))
        if qname_ends_with(event.name().as_ref(), b"bodyPr") =>
      {
        frame.left_pt = attr_value(&event, b"lIns")
          .and_then(|value| value.parse::<i64>().ok())
          .map(units::emu_to_points)
          .unwrap_or(frame.left_pt);
        frame.top_pt = attr_value(&event, b"tIns")
          .and_then(|value| value.parse::<i64>().ok())
          .map(units::emu_to_points)
          .unwrap_or(frame.top_pt);
        frame.right_pt = attr_value(&event, b"rIns")
          .and_then(|value| value.parse::<i64>().ok())
          .map(units::emu_to_points)
          .unwrap_or(frame.right_pt);
        frame.bottom_pt = attr_value(&event, b"bIns")
          .and_then(|value| value.parse::<i64>().ok())
          .map(units::emu_to_points)
          .unwrap_or(frame.bottom_pt);
        frame.vertical_alignment = match attr_value(&event, b"anchor").as_deref() {
          Some("ctr") => TextBoxVerticalAlignment::Center,
          Some("b") | Some("bottom") => TextBoxVerticalAlignment::Bottom,
          _ => frame.vertical_alignment,
        };
        break;
      }
      Ok(Event::Eof) | Err(_) => break,
      _ => {}
    }
  }
}

#[cfg(test)]
fn apply_text_effect_overrides_from_fragment(
  style: &mut TextStyle,
  fragment: &str,
  styles: &StylesCatalog,
) {
  if let Some(fill_fragment) = first_named_xml_fragment(fragment, b"textFill") {
    let fill_fragment = textbox_fragment_with_namespaces(fill_fragment);
    if let Ok(fill) = w14::FillTextEffect::from_bytes(fill_fragment.as_bytes())
      && let Some(resolved) = resolve_text_fill(&fill, &styles.theme_colors)
    {
      style.color = resolved.color;
      style.opacity = resolved.opacity;
    }
  }

  if let Some(outline_fragment) = first_named_xml_fragment(fragment, b"textOutline") {
    let outline_fragment = textbox_fragment_with_namespaces(outline_fragment);
    if let Ok(outline) = w14::TextOutlineEffect::from_bytes(outline_fragment.as_bytes())
      && let Some(resolved) = resolve_text_outline(&outline, &styles.theme_colors)
    {
      style.outline_color = Some(resolved.color);
      style.outline_opacity = resolved.opacity;
      style.outline_width_pt = outline
        .line_width
        .map(|width| units::emu_to_points(width as i64))
        .unwrap_or(style.outline_width_pt);
    }
  }
}

fn read_outer_xml_fragment(
  reader: &mut Reader<&[u8]>,
  start: quick_xml::events::BytesStart<'_>,
) -> Option<String> {
  let target_name = start.name().as_ref().to_vec();
  let mut writer = Writer::new(Vec::new());
  writer.write_event(Event::Start(start.into_owned())).ok()?;
  let mut depth = 1usize;

  while depth > 0 {
    let event = reader.read_event().ok()?;
    match &event {
      Event::Start(event) if event.name().as_ref() == target_name.as_slice() => depth += 1,
      Event::End(event) if event.name().as_ref() == target_name.as_slice() => {
        depth = depth.saturating_sub(1);
      }
      Event::Eof => return None,
      _ => {}
    }
    writer.write_event(event.into_owned()).ok()?;
  }

  String::from_utf8(writer.into_inner()).ok()
}

fn first_named_xml_fragment(xml: &str, local_name: &[u8]) -> Option<String> {
  let mut reader = Reader::from_str(xml);
  reader.config_mut().trim_text(false);

  loop {
    match reader.read_event().ok()? {
      Event::Start(event) if qname_ends_with(event.name().as_ref(), local_name) => {
        return read_outer_xml_fragment(&mut reader, event);
      }
      Event::Empty(event) if qname_ends_with(event.name().as_ref(), local_name) => {
        let mut writer = Writer::new(Vec::new());
        writer.write_event(Event::Empty(event.into_owned())).ok()?;
        return String::from_utf8(writer.into_inner()).ok();
      }
      Event::Eof => return None,
      _ => {}
    }
  }
}

fn drawing_graphic_data(drawing: &w::Drawing) -> Option<&ooxmlsdk::schemas::a::GraphicData> {
  match drawing.drawing_choice.as_ref()? {
    w::DrawingChoice::WpInline(inline) => Some(&inline.graphic.graphic_data),
    w::DrawingChoice::WpAnchor(anchor) => Some(&anchor.a_graphic.as_ref()?.graphic_data),
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

  let placement = match drawing.drawing_choice.as_ref() {
    Some(w::DrawingChoice::WpInline(_)) => ImagePlacement::Inline,
    Some(w::DrawingChoice::WpAnchor(anchor)) => {
      ImagePlacement::Floating(floating_image_placement(anchor))
    }
    None => return,
  };

  for xml in &graphic_data.xml_children {
    inlines.extend(drawingml_shapes_from_xml(
      xml,
      placement,
      DrawingMlGroupTransform::identity(),
      styles,
      images,
      hyperlinks,
      false,
    ));
  }
}

#[derive(Clone, Copy, Debug)]
struct DrawingMlGroupTransform {
  scale_x: f32,
  scale_y: f32,
  translate_x_pt: f32,
  translate_y_pt: f32,
}

impl DrawingMlGroupTransform {
  fn identity() -> Self {
    Self {
      scale_x: 1.0,
      scale_y: 1.0,
      translate_x_pt: 0.0,
      translate_y_pt: 0.0,
    }
  }

  fn child(self, xfrm: DrawingMlGroupXfrm) -> Self {
    let scale_x = if xfrm.child_width_pt != 0.0 {
      xfrm.width_pt / xfrm.child_width_pt
    } else {
      1.0
    };
    let scale_y = if xfrm.child_height_pt != 0.0 {
      xfrm.height_pt / xfrm.child_height_pt
    } else {
      1.0
    };
    Self {
      scale_x: self.scale_x * scale_x,
      scale_y: self.scale_y * scale_y,
      translate_x_pt: self.translate_x_pt
        + self.scale_x * (xfrm.offset_x_pt - xfrm.child_offset_x_pt * scale_x),
      translate_y_pt: self.translate_y_pt
        + self.scale_y * (xfrm.offset_y_pt - xfrm.child_offset_y_pt * scale_y),
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
  child_offset_x_pt: f32,
  child_offset_y_pt: f32,
  child_width_pt: f32,
  child_height_pt: f32,
}

fn drawingml_shapes_from_xml(
  xml: &str,
  placement: ImagePlacement,
  transform: DrawingMlGroupTransform,
  styles: &StylesCatalog,
  images: &ImageCatalog,
  hyperlinks: &HyperlinkCatalog,
  skip_container_root: bool,
) -> Vec<InlineItem> {
  let mut shapes = Vec::new();
  let mut reader = Reader::from_str(xml);
  reader.config_mut().trim_text(false);
  let mut skipped_container_root = false;

  loop {
    match reader.read_event() {
      Ok(Event::Start(event)) if skip_container_root && !skipped_container_root => {
        skipped_container_root = true;
        if qname_ends_with(event.name().as_ref(), b"wgp") {
          continue;
        }
      }
      Ok(Event::Start(event)) if qname_ends_with(event.name().as_ref(), b"wgp") => {
        if let Some(fragment) = read_outer_xml_fragment(&mut reader, event) {
          let child_transform = drawingml_group_transform_from_fragment(&fragment)
            .map(|xfrm| transform.child(xfrm))
            .unwrap_or(transform);
          shapes.extend(drawingml_shapes_from_xml(
            &fragment,
            placement,
            child_transform,
            styles,
            images,
            hyperlinks,
            true,
          ));
        }
      }
      Ok(Event::Start(event)) if qname_ends_with(event.name().as_ref(), b"wsp") => {
        if let Some(fragment) = read_outer_xml_fragment(&mut reader, event)
          && let Some(shape) =
            drawingml_shape_from_fragment(&fragment, placement, transform, styles)
        {
          shapes.push(InlineItem::Shape(shape));
        }
      }
      Ok(Event::Start(event)) if qname_ends_with(event.name().as_ref(), b"pic") => {
        if let Some(fragment) = read_outer_xml_fragment(&mut reader, event) {
          if let Some(image) = drawingml_picture_image_from_fragment(
            &fragment, placement, transform, images, hyperlinks,
          ) {
            shapes.push(InlineItem::Image(image));
          }
          if let Some(shape) =
            drawingml_picture_frame_from_fragment(&fragment, placement, transform)
          {
            shapes.push(InlineItem::Shape(shape));
          }
        }
      }
      Ok(Event::Empty(event)) if qname_ends_with(event.name().as_ref(), b"wsp") => {
        let mut writer = Writer::new(Vec::new());
        if writer.write_event(Event::Empty(event.into_owned())).is_ok()
          && let Ok(fragment) = String::from_utf8(writer.into_inner())
          && let Some(shape) =
            drawingml_shape_from_fragment(&fragment, placement, transform, styles)
        {
          shapes.push(InlineItem::Shape(shape));
        }
      }
      Ok(Event::Eof) | Err(_) => break,
      _ => {}
    }
  }

  shapes
}

fn drawing_is_hidden(drawing: &w::Drawing) -> bool {
  match drawing.drawing_choice.as_ref() {
    Some(w::DrawingChoice::WpInline(inline)) => inline
      .doc_properties
      .hidden
      .as_ref()
      .is_some_and(|hidden| hidden.as_bool()),
    Some(w::DrawingChoice::WpAnchor(anchor)) => {
      anchor
        .hidden
        .as_ref()
        .is_some_and(|hidden| hidden.as_bool())
        || anchor
          .wp_doc_pr
          .as_ref()
          .and_then(|properties| properties.hidden.as_ref())
          .is_some_and(|hidden| hidden.as_bool())
    }
    None => false,
  }
}

fn drawingml_shape_from_fragment(
  xml: &str,
  placement: ImagePlacement,
  transform: DrawingMlGroupTransform,
  styles: &StylesCatalog,
) -> Option<InlineShape> {
  let sp_pr = first_named_xml_fragment(xml, b"spPr")?;
  let fill_color = drawingml_shape_fill_color(&sp_pr, &styles.theme_colors)
    .or_else(|| drawingml_shape_style_color(xml, b"fillRef", &styles.theme_colors));
  let stroke = drawingml_shape_stroke(&sp_pr, &styles.theme_colors)
    .or_else(|| drawingml_shape_style_stroke(xml, &styles.theme_colors, &styles.theme_lines));
  if fill_color.is_none() && stroke.is_none() {
    return None;
  }

  let geometry = drawingml_shape_geometry_kind(&sp_pr);
  let (offset_x_pt, offset_y_pt, width_pt, height_pt) =
    drawingml_geometry_from_sp_pr(&sp_pr, geometry)?;
  let (offset_x_pt, offset_y_pt, width_pt, height_pt) =
    transform.rect(offset_x_pt, offset_y_pt, width_pt, height_pt);

  Some(InlineShape {
    width_pt,
    height_pt,
    geometry,
    offset_x_pt,
    offset_y_pt,
    fill_color,
    stroke,
    placement,
    text_box_blocks: Vec::new(),
    text_inset_left_pt: 0.0,
    text_inset_top_pt: 0.0,
    text_inset_right_pt: 0.0,
    text_inset_bottom_pt: 0.0,
    text_vertical_alignment: TextBoxVerticalAlignment::Top,
  })
}

fn drawingml_shape_geometry(xml: &str) -> Option<(f32, f32, f32, f32)> {
  let sp_pr = first_named_xml_fragment(xml, b"spPr")?;
  drawingml_geometry_from_sp_pr(&sp_pr, drawingml_shape_geometry_kind(&sp_pr))
}

fn drawingml_group_transform_from_fragment(xml: &str) -> Option<DrawingMlGroupXfrm> {
  let grp_sp_pr = first_named_xml_fragment(xml, b"grpSpPr")?;
  let xfrm = first_named_xml_fragment(&grp_sp_pr, b"xfrm")?;
  let mut reader = Reader::from_str(&xfrm);
  reader.config_mut().trim_text(false);
  let mut group = DrawingMlGroupXfrm::default();

  loop {
    match reader.read_event().ok()? {
      Event::Empty(event) if qname_ends_with(event.name().as_ref(), b"off") => {
        group.offset_x_pt = attr_value(&event, b"x")
          .and_then(|value| value.parse::<i64>().ok())
          .map(units::emu_to_points)
          .unwrap_or(group.offset_x_pt);
        group.offset_y_pt = attr_value(&event, b"y")
          .and_then(|value| value.parse::<i64>().ok())
          .map(units::emu_to_points)
          .unwrap_or(group.offset_y_pt);
      }
      Event::Empty(event) if qname_ends_with(event.name().as_ref(), b"ext") => {
        group.width_pt = attr_value(&event, b"cx")
          .and_then(|value| value.parse::<i64>().ok())
          .map(units::emu_to_points)
          .unwrap_or(group.width_pt);
        group.height_pt = attr_value(&event, b"cy")
          .and_then(|value| value.parse::<i64>().ok())
          .map(units::emu_to_points)
          .unwrap_or(group.height_pt);
      }
      Event::Empty(event) if qname_ends_with(event.name().as_ref(), b"chOff") => {
        group.child_offset_x_pt = attr_value(&event, b"x")
          .and_then(|value| value.parse::<i64>().ok())
          .map(units::emu_to_points)
          .unwrap_or(group.child_offset_x_pt);
        group.child_offset_y_pt = attr_value(&event, b"y")
          .and_then(|value| value.parse::<i64>().ok())
          .map(units::emu_to_points)
          .unwrap_or(group.child_offset_y_pt);
      }
      Event::Empty(event) if qname_ends_with(event.name().as_ref(), b"chExt") => {
        group.child_width_pt = attr_value(&event, b"cx")
          .and_then(|value| value.parse::<i64>().ok())
          .map(units::emu_to_points)
          .unwrap_or(group.child_width_pt);
        group.child_height_pt = attr_value(&event, b"cy")
          .and_then(|value| value.parse::<i64>().ok())
          .map(units::emu_to_points)
          .unwrap_or(group.child_height_pt);
      }
      Event::Eof => return Some(group),
      _ => {}
    }
  }
}

fn drawingml_shape_geometry_kind(sp_pr: &str) -> InlineShapeGeometry {
  let mut reader = Reader::from_str(sp_pr);
  reader.config_mut().trim_text(false);

  loop {
    match reader.read_event() {
      Ok(Event::Start(event)) | Ok(Event::Empty(event))
        if qname_ends_with(event.name().as_ref(), b"prstGeom") =>
      {
        if attr_value(&event, b"prst")
          .as_deref()
          .is_some_and(|value| value.eq_ignore_ascii_case("line"))
        {
          return InlineShapeGeometry::Line;
        }
      }
      Ok(Event::Eof) | Err(_) => break,
      _ => {}
    }
  }

  InlineShapeGeometry::Rectangle
}

fn drawingml_geometry_from_sp_pr(
  sp_pr: &str,
  geometry: InlineShapeGeometry,
) -> Option<(f32, f32, f32, f32)> {
  let mut reader = Reader::from_str(sp_pr);
  reader.config_mut().trim_text(false);
  let mut offset_x_pt = 0.0f32;
  let mut offset_y_pt = 0.0f32;
  let mut width_pt = 0.0f32;
  let mut height_pt = 0.0f32;

  loop {
    match reader.read_event() {
      Ok(Event::Empty(event)) if qname_ends_with(event.name().as_ref(), b"off") => {
        offset_x_pt = attr_value(&event, b"x")
          .and_then(|value| value.parse::<i64>().ok())
          .map(units::emu_to_points)
          .unwrap_or(offset_x_pt);
        offset_y_pt = attr_value(&event, b"y")
          .and_then(|value| value.parse::<i64>().ok())
          .map(units::emu_to_points)
          .unwrap_or(offset_y_pt);
      }
      Ok(Event::Empty(event)) if qname_ends_with(event.name().as_ref(), b"ext") => {
        width_pt = attr_value(&event, b"cx")
          .and_then(|value| value.parse::<i64>().ok())
          .map(units::emu_to_points)
          .unwrap_or(width_pt);
        height_pt = attr_value(&event, b"cy")
          .and_then(|value| value.parse::<i64>().ok())
          .map(units::emu_to_points)
          .unwrap_or(height_pt);
      }
      Ok(Event::Eof) | Err(_) => break,
      _ => {}
    }
  }

  match geometry {
    InlineShapeGeometry::Rectangle if width_pt <= 0.0 || height_pt <= 0.0 => return None,
    InlineShapeGeometry::Line if width_pt <= 0.0 && height_pt <= 0.0 => return None,
    InlineShapeGeometry::Rectangle | InlineShapeGeometry::Line => {}
  }

  Some((offset_x_pt, offset_y_pt, width_pt, height_pt))
}

fn drawingml_picture_frame_from_fragment(
  xml: &str,
  placement: ImagePlacement,
  transform: DrawingMlGroupTransform,
) -> Option<InlineShape> {
  let (offset_x_pt, offset_y_pt, width_pt, height_pt) = drawingml_shape_geometry(xml)?;
  let (offset_x_pt, offset_y_pt, width_pt, height_pt) =
    transform.rect(offset_x_pt, offset_y_pt, width_pt, height_pt);

  Some(InlineShape {
    width_pt,
    height_pt,
    geometry: InlineShapeGeometry::Rectangle,
    offset_x_pt,
    offset_y_pt,
    fill_color: None,
    stroke: None,
    placement,
    text_box_blocks: Vec::new(),
    text_inset_left_pt: 0.0,
    text_inset_top_pt: 0.0,
    text_inset_right_pt: 0.0,
    text_inset_bottom_pt: 0.0,
    text_vertical_alignment: TextBoxVerticalAlignment::Top,
  })
}

fn drawingml_picture_image_from_fragment(
  xml: &str,
  placement: ImagePlacement,
  transform: DrawingMlGroupTransform,
  images: &ImageCatalog,
  hyperlinks: &HyperlinkCatalog,
) -> Option<InlineImage> {
  let properties = drawing_image_properties_from_xml(xml)?;
  let relationship_id = properties.relationship_id?;
  let resource = images.by_relationship_id.get(&relationship_id)?;
  let (offset_x_pt, offset_y_pt, width_pt, height_pt) = drawingml_shape_geometry(xml)?;
  let (offset_x_pt, offset_y_pt, width_pt, height_pt) =
    transform.rect(offset_x_pt, offset_y_pt, width_pt, height_pt);
  let hyperlink_url = properties
    .hyperlink_relationship_id
    .as_deref()
    .and_then(|relationship_id| hyperlinks.target(relationship_id))
    .map(ToString::to_string);
  Some(InlineImage {
    data: resource.data.clone(),
    content_type: resource.content_type.clone(),
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
    alt_text: drawingml_picture_alt_text(xml),
    hyperlink_url,
    placement: drawingml_child_placement(placement, offset_x_pt, offset_y_pt),
  })
}

fn drawingml_picture_alt_text(xml: &str) -> Option<String> {
  let mut reader = Reader::from_str(xml);
  reader.config_mut().trim_text(false);
  loop {
    match reader.read_event().ok()? {
      Event::Start(event) | Event::Empty(event)
        if qname_ends_with(event.name().as_ref(), b"cNvPr") =>
      {
        if let Some(description) = attr_value(&event, b"descr") {
          return Some(description.to_string());
        }
        if let Some(name) = attr_value(&event, b"name") {
          return Some(name.to_string());
        }
      }
      Event::Eof => return None,
      _ => {}
    }
  }
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

fn drawingml_shape_fill_color(xml: &str, theme_colors: &ThemeColors) -> Option<RgbColor> {
  drawingml_color_from_named_fragment(xml, b"solidFill", theme_colors).map(|color| color.color)
}

fn drawingml_shape_style_color(
  xml: &str,
  local_name: &[u8],
  theme_colors: &ThemeColors,
) -> Option<RgbColor> {
  drawingml_color_from_named_fragment(xml, local_name, theme_colors).map(|color| color.color)
}

fn drawingml_shape_style_stroke(
  xml: &str,
  theme_colors: &ThemeColors,
  theme_lines: &ThemeLineStyles,
) -> Option<BorderStyle> {
  let fragment = first_named_xml_fragment(xml, b"lnRef")?;
  let index = drawingml_style_ref_index(&fragment)?;
  let width_pt = theme_lines.width_pt(index)?;
  let color = drawingml_color_from_named_fragment(&fragment, b"schemeClr", theme_colors)?.color;
  Some(BorderStyle {
    width_pt,
    spacing_pt: 0.0,
    color,
    compound: false,
  })
}

fn drawingml_style_ref_index(xml: &str) -> Option<usize> {
  let mut reader = Reader::from_str(xml);
  loop {
    match reader.read_event().ok()? {
      Event::Start(event) | Event::Empty(event)
        if qname_ends_with(event.name().as_ref(), b"lnRef") =>
      {
        return attr_value(&event, b"idx")?.parse::<usize>().ok();
      }
      Event::Eof => return None,
      _ => {}
    }
  }
}

fn drawingml_shape_stroke(xml: &str, theme_colors: &ThemeColors) -> Option<BorderStyle> {
  let line_fragment = first_named_xml_fragment(xml, b"ln")?;
  let mut width_pt = units::emu_to_points(DRAWINGML_DEFAULT_LINE_WIDTH_EMU);
  let mut reader = Reader::from_str(&line_fragment);
  reader.config_mut().trim_text(false);

  loop {
    match reader.read_event() {
      Ok(Event::Start(event)) | Ok(Event::Empty(event))
        if qname_ends_with(event.name().as_ref(), b"ln") =>
      {
        width_pt = attr_value(&event, b"w")
          .and_then(|value| value.parse::<i64>().ok())
          .map(units::emu_to_points)
          .unwrap_or(width_pt);
        break;
      }
      Ok(Event::Eof) | Err(_) => break,
      _ => {}
    }
  }

  Some(BorderStyle {
    width_pt,
    spacing_pt: 0.0,
    color: drawingml_color_from_named_fragment(&line_fragment, b"solidFill", theme_colors)?.color,
    compound: false,
  })
}

fn drawingml_color_from_named_fragment(
  xml: &str,
  local_name: &[u8],
  theme_colors: &ThemeColors,
) -> Option<ResolvedColor> {
  let fragment = first_named_xml_fragment(xml, local_name)?;
  let mut reader = Reader::from_str(&fragment);
  reader.config_mut().trim_text(false);

  loop {
    match reader.read_event().ok()? {
      Event::Start(event) if qname_ends_with(event.name().as_ref(), b"schemeClr") => {
        return resolve_scheme_color_from_reader(&mut reader, event, theme_colors);
      }
      Event::Empty(event) if qname_ends_with(event.name().as_ref(), b"schemeClr") => {
        return resolve_empty_scheme_color(&event, theme_colors);
      }
      Event::Start(event) if qname_ends_with(event.name().as_ref(), b"srgbClr") => {
        let color = color_attr(&event, b"val")?;
        return Some(ResolvedColor {
          color,
          opacity: 1.0,
        });
      }
      Event::Empty(event) if qname_ends_with(event.name().as_ref(), b"srgbClr") => {
        let color = color_attr(&event, b"val")?;
        return Some(ResolvedColor {
          color,
          opacity: 1.0,
        });
      }
      Event::Start(event) | Event::Empty(event)
        if qname_ends_with(event.name().as_ref(), b"sysClr") =>
      {
        let color = color_attr(&event, b"lastClr").or_else(|| color_attr(&event, b"val"))?;
        return Some(ResolvedColor {
          color,
          opacity: 1.0,
        });
      }
      Event::Eof => return None,
      _ => {}
    }
  }
}

fn push_pict_shapes_impl(picture: &w::Picture, inlines: &mut Vec<InlineItem>) {
  for choice in &picture.picture_choice {
    push_picture_choice_shapes(choice, inlines);
  }
}

fn push_picture_choice_shapes(choice: &w::PictureChoice, inlines: &mut Vec<InlineItem>) {
  match choice {
    w::PictureChoice::VGroup(group) => push_group_shapes(group, inlines),
    w::PictureChoice::VRect(rectangle) => {
      if let Some(shape) = vml_rectangle_shape(rectangle) {
        inlines.push(InlineItem::Shape(shape));
      }
    }
    w::PictureChoice::VShape(shape) => {
      if let Some(shape) = vml_shape_shape(shape) {
        inlines.push(InlineItem::Shape(shape));
      }
    }
    _ => {}
  }
}

fn push_group_shapes(group: &v::Group, inlines: &mut Vec<InlineItem>) {
  for choice in &group.group_choice {
    match choice {
      v::GroupChoice::VGroup(group) => push_group_shapes(group, inlines),
      v::GroupChoice::VRect(rectangle) => {
        if let Some(shape) = vml_rectangle_shape(rectangle) {
          inlines.push(InlineItem::Shape(shape));
        }
      }
      v::GroupChoice::VShape(shape) => {
        if let Some(shape) = vml_shape_shape(shape) {
          inlines.push(InlineItem::Shape(shape));
        }
      }
      _ => {}
    }
  }
}

fn vml_rectangle_shape(rectangle: &v::Rectangle) -> Option<InlineShape> {
  vml_inline_shape(
    rectangle.style.as_deref(),
    rectangle.fill_color.as_deref(),
    rectangle.stroke_color.as_deref(),
    rectangle.stroke_weight.as_deref(),
  )
}

fn vml_shape_shape(shape: &v::Shape) -> Option<InlineShape> {
  vml_inline_shape(
    shape.style.as_deref(),
    shape.fill_color.as_deref(),
    shape.stroke_color.as_deref(),
    shape.stroke_weight.as_deref(),
  )
}

fn vml_inline_shape(
  style: Option<&str>,
  fill_color: Option<&str>,
  stroke_color: Option<&str>,
  stroke_weight: Option<&str>,
) -> Option<InlineShape> {
  if vml_style_is_hidden(style) {
    return None;
  }

  let style = vml_image_style(style);
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
  if fill_color.is_none() && stroke.is_none() {
    return None;
  }

  Some(InlineShape {
    width_pt,
    height_pt,
    geometry: InlineShapeGeometry::Rectangle,
    offset_x_pt: 0.0,
    offset_y_pt: 0.0,
    fill_color,
    stroke,
    placement: style.placement(),
    text_box_blocks: Vec::new(),
    text_inset_left_pt: 0.0,
    text_inset_top_pt: 0.0,
    text_inset_right_pt: 0.0,
    text_inset_bottom_pt: 0.0,
    text_vertical_alignment: TextBoxVerticalAlignment::Top,
  })
}

fn vml_textbox_frame(
  shape_style: Option<&str>,
  textbox: &v::TextBox,
  styles: &StylesCatalog,
  images: &ImageCatalog,
  hyperlinks: &HyperlinkCatalog,
) -> Option<InlineShape> {
  if vml_style_is_hidden(shape_style) {
    return None;
  }

  let Some(v::TextBoxChoice::WTxbxContent(content)) = textbox.text_box_choice.as_ref() else {
    return None;
  };
  let style = vml_image_style(shape_style);
  let (shape_width_pt, shape_height_pt) = style.size_pt?;
  let mut frame = TextBoxFrameContent::new(textbox_blocks(content, styles, images, hyperlinks));
  apply_vml_textbox_properties(textbox, &mut frame);
  let width_pt =
    (shape_width_pt - frame.left_pt - frame.right_pt).max(DEFAULT_TEXTBOX_MIN_WIDTH_PT);
  let height_pt =
    (shape_height_pt - frame.top_pt - frame.bottom_pt).max(DEFAULT_TEXTBOX_MIN_HEIGHT_PT);

  Some(InlineShape {
    width_pt,
    height_pt,
    geometry: InlineShapeGeometry::Rectangle,
    offset_x_pt: frame.left_pt,
    offset_y_pt: frame.top_pt,
    fill_color: None,
    stroke: None,
    placement: style.placement(),
    text_box_blocks: frame.blocks,
    text_inset_left_pt: 0.0,
    text_inset_top_pt: 0.0,
    text_inset_right_pt: 0.0,
    text_inset_bottom_pt: 0.0,
    text_vertical_alignment: frame.vertical_alignment,
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
    parse_hex_color(base)
  }
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
    w::PictureChoice::VGroup(group) => group_image(group, images),
    w::PictureChoice::VImage(image) => image_file_image(image, images),
    w::PictureChoice::VRect(rectangle) => rectangle_image(rectangle, images),
    w::PictureChoice::VShape(shape) => shape_image(shape, images),
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
    w::PictureChoice::VGroup(group) => {
      push_group_textboxes(group, inlines, base_style, styles, images, hyperlinks);
    }
    w::PictureChoice::VImage(image) => {
      push_image_file_textboxes(image, inlines, base_style, styles, images, hyperlinks);
    }
    w::PictureChoice::VRect(rectangle) => {
      push_rectangle_textboxes(rectangle, inlines, base_style, styles, images, hyperlinks);
    }
    w::PictureChoice::VShape(shape) => {
      push_shape_textboxes(shape, inlines, base_style, styles, images, hyperlinks);
    }
    _ => {}
  }
}

fn group_image(group: &v::Group, images: &ImageCatalog) -> Option<InlineImage> {
  group.group_choice.iter().find_map(|choice| match choice {
    v::GroupChoice::VGroup(group) => group_image(group, images),
    v::GroupChoice::VImage(image) => image_file_image(image, images),
    v::GroupChoice::VRect(rectangle) => rectangle_image(rectangle, images),
    v::GroupChoice::VShape(shape) => shape_image(shape, images),
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
  for choice in &group.group_choice {
    match choice {
      v::GroupChoice::VGroup(group) => {
        push_group_textboxes(
          group,
          inlines,
          base_style.clone(),
          styles,
          images,
          hyperlinks,
        );
      }
      v::GroupChoice::VImage(image) => {
        push_image_file_textboxes(
          image,
          inlines,
          base_style.clone(),
          styles,
          images,
          hyperlinks,
        );
      }
      v::GroupChoice::VRect(rectangle) => {
        push_rectangle_textboxes(
          rectangle,
          inlines,
          base_style.clone(),
          styles,
          images,
          hyperlinks,
        );
      }
      v::GroupChoice::VShape(shape) => {
        push_shape_textboxes(
          shape,
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
      v::ImageFileChoice::VImagedata(data) => vml_image_data(
        data,
        image.style.as_deref(),
        image.alternate.clone(),
        images,
      ),
      _ => None,
    })
}

fn push_image_file_textboxes(
  image: &v::ImageFile,
  inlines: &mut Vec<InlineItem>,
  base_style: TextStyle,
  styles: &StylesCatalog,
  images: &ImageCatalog,
  hyperlinks: &HyperlinkCatalog,
) {
  if vml_style_is_hidden(image.style.as_deref()) {
    return;
  }

  for choice in &image.image_file_choice {
    if let v::ImageFileChoice::VTextbox(textbox) = choice {
      if let Some(frame) =
        vml_textbox_frame(image.style.as_deref(), textbox, styles, images, hyperlinks)
      {
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
      v::RectangleChoice::VImagedata(data) => vml_image_data(
        data,
        rectangle.style.as_deref(),
        rectangle.alternate.clone(),
        images,
      ),
      _ => None,
    })
}

fn push_rectangle_textboxes(
  rectangle: &v::Rectangle,
  inlines: &mut Vec<InlineItem>,
  base_style: TextStyle,
  styles: &StylesCatalog,
  images: &ImageCatalog,
  hyperlinks: &HyperlinkCatalog,
) {
  if vml_style_is_hidden(rectangle.style.as_deref()) {
    return;
  }

  for choice in &rectangle.rectangle_choice {
    if let v::RectangleChoice::VTextbox(textbox) = choice {
      if let Some(frame) = vml_textbox_frame(
        rectangle.style.as_deref(),
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
    v::ShapeChoice::VImagedata(data) => vml_image_data(
      data,
      shape.style.as_deref(),
      shape.alternate.clone(),
      images,
    ),
    _ => None,
  })
}

fn push_shape_textboxes(
  shape: &v::Shape,
  inlines: &mut Vec<InlineItem>,
  base_style: TextStyle,
  styles: &StylesCatalog,
  images: &ImageCatalog,
  hyperlinks: &HyperlinkCatalog,
) {
  if vml_style_is_hidden(shape.style.as_deref()) {
    return;
  }

  for choice in &shape.shape_choice {
    if let v::ShapeChoice::VTextbox(textbox) = choice {
      if let Some(frame) =
        vml_textbox_frame(shape.style.as_deref(), textbox, styles, images, hyperlinks)
      {
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
  let Some(v::TextBoxChoice::WTxbxContent(content)) = textbox.text_box_choice.as_ref() else {
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
  let mut blocks = Vec::new();
  let mut numbering = NumberingCatalog::default();
  let mut form_widget_ids = FormWidgetIdAllocator::default();
  for choice in &content.text_box_content_choice {
    match choice {
      w::TextBoxContentChoice::WP(paragraph) => {
        let paragraph = paragraph_model(
          paragraph,
          styles,
          &mut numbering,
          images,
          hyperlinks,
          &mut form_widget_ids,
        );
        blocks.push(Block::Paragraph(paragraph));
      }
      w::TextBoxContentChoice::WTbl(table) => {
        let table = table_model(
          table,
          styles,
          &mut numbering,
          images,
          hyperlinks,
          &mut form_widget_ids,
        );
        blocks.push(Block::Table(table));
      }
      _ => {}
    }
  }
  blocks
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
    }));
  }
}

fn vml_image_data(
  data: &v::ImageData,
  style: Option<&str>,
  alt_text: Option<String>,
  images: &ImageCatalog,
) -> Option<InlineImage> {
  let relationship_id = data.relationship_id.as_ref().or(data.rel_id.as_ref())?;
  let resource = images.by_relationship_id.get(relationship_id)?;
  let style = vml_image_style(style);
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
  horizontal_offset_pt: f32,
  vertical_offset_pt: f32,
  wrap: ImageWrapMode,
  behind_text: bool,
  margin_top_pt: f32,
  margin_right_pt: f32,
  margin_bottom_pt: f32,
  margin_left_pt: f32,
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
      horizontal_offset_pt: 0.0,
      vertical_offset_pt: 0.0,
      wrap: ImageWrapMode::Square,
      behind_text: false,
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
        vertical_alignment: None,
        horizontal_offset_pt: self.horizontal_offset_pt,
        vertical_offset_pt: self.vertical_offset_pt,
        wrap: self.wrap,
        wrap_side: ImageWrapSide::BothSides,
        behind_text: self.behind_text,
        layout_in_cell: true,
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
      .map(|value| value / units::VML_PERCENT_SCALE)
  } else if let Some(fixed) = value.strip_suffix('f') {
    fixed
      .trim()
      .parse::<f32>()
      .ok()
      .map(|value| value / units::VML_FIXED_SCALE)
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
    "line" => VerticalImageReference::Line,
    _ => VerticalImageReference::Paragraph,
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
      .parse::<f32>()
      .ok()
      .map(|value| value / units::VML_FIXED_SCALE)
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
  rotation_deg: f32,
  flip_horizontal: bool,
  flip_vertical: bool,
}

fn drawing_image_properties(
  graphic_data: &ooxmlsdk::schemas::a::GraphicData,
) -> Option<DrawingImageProperties> {
  if graphic_data.uri != "http://schemas.openxmlformats.org/drawingml/2006/picture" {
    return None;
  }
  graphic_data
    .xml_children
    .iter()
    .find_map(|child| drawing_image_properties_from_xml(child))
}

fn drawing_image_properties_from_xml(xml: &str) -> Option<DrawingImageProperties> {
  let mut reader = Reader::from_str(xml);
  reader.config_mut().trim_text(true);
  let mut properties = DrawingImageProperties::default();
  loop {
    match reader.read_event().ok()? {
      Event::Empty(event) | Event::Start(event) if event.name().as_ref().ends_with(b":blip") => {
        for attr in event.attributes().with_checks(false).flatten() {
          if attr.key.as_ref().ends_with(b":embed") || attr.key.as_ref() == b"embed" {
            properties.relationship_id = attr
              .decode_and_unescape_value(reader.decoder())
              .ok()
              .map(|value| value.into_owned());
          }
        }
      }
      Event::Empty(event) | Event::Start(event)
        if qname_ends_with(event.name().as_ref(), b"hlinkClick") =>
      {
        for attr in event.attributes().with_checks(false).flatten() {
          if attr.key.as_ref().ends_with(b":id") || attr.key.as_ref() == b"id" {
            properties.hyperlink_relationship_id = attr
              .decode_and_unescape_value(reader.decoder())
              .ok()
              .map(|value| value.into_owned());
          }
        }
      }
      Event::Empty(event) | Event::Start(event)
        if qname_ends_with(event.name().as_ref(), b"srcRect") =>
      {
        properties.crop = image_crop_from_src_rect(&event, reader.decoder());
      }
      Event::Empty(event) | Event::Start(event)
        if qname_ends_with(event.name().as_ref(), b"xfrm") =>
      {
        apply_image_transform_attrs(&mut properties, &event, reader.decoder());
      }
      Event::Eof => return properties.relationship_id.is_some().then_some(properties),
      _ => {}
    }
  }
}

fn image_crop_from_src_rect(
  event: &quick_xml::events::BytesStart<'_>,
  decoder: quick_xml::Decoder,
) -> ImageCrop {
  let mut crop = ImageCrop::default();
  for attr in event.attributes().with_checks(false).flatten() {
    let value = attr
      .decode_and_unescape_value(decoder)
      .ok()
      .and_then(|value| value.parse::<i32>().ok())
      .map(relative_rect_attr_to_fraction)
      .unwrap_or(0.0);
    match attr.key.as_ref() {
      b"l" => crop.left = value,
      b"t" => crop.top = value,
      b"r" => crop.right = value,
      b"b" => crop.bottom = value,
      _ => {}
    }
  }
  crop
}

fn apply_image_transform_attrs(
  properties: &mut DrawingImageProperties,
  event: &quick_xml::events::BytesStart<'_>,
  decoder: quick_xml::Decoder,
) {
  for attr in event.attributes().with_checks(false).flatten() {
    let value = attr.decode_and_unescape_value(decoder).ok();
    match attr.key.as_ref() {
      b"rot" => {
        properties.rotation_deg = value
          .as_deref()
          .and_then(|value| value.parse::<i32>().ok())
          .map(|value| value as f32 / units::DRAWINGML_ANGLE_UNITS_PER_DEGREE)
          .unwrap_or(0.0);
      }
      b"flipH" => properties.flip_horizontal = value.as_deref().is_some_and(xml_bool),
      b"flipV" => properties.flip_vertical = value.as_deref().is_some_and(xml_bool),
      _ => {}
    }
  }
}

fn relative_rect_attr_to_fraction(value: i32) -> f32 {
  (value as f32 / units::DRAWINGML_PERCENT_SCALE)
    .clamp(0.0, units::DRAWINGML_MAX_FRACTION_BELOW_ONE)
}

fn xml_bool(value: &str) -> bool {
  matches!(value, "1" | "true" | "t" | "on")
}

fn drawing_textbox_content(xml: &str) -> Option<w::TextBoxContent> {
  if !xml.contains("txbxContent") {
    return None;
  }

  let mut reader = Reader::from_str(xml);
  reader.config_mut().trim_text(false);

  loop {
    match reader.read_event().ok()? {
      Event::Start(event) if qname_ends_with(event.name().as_ref(), b"txbxContent") => {
        let mut writer = Writer::new(Vec::new());
        writer.write_event(Event::Start(event.into_owned())).ok()?;
        let mut depth = 1usize;

        while depth > 0 {
          let event = reader.read_event().ok()?;
          match &event {
            Event::Start(_) => depth += 1,
            Event::End(_) => depth = depth.saturating_sub(1),
            Event::Empty(_) => {}
            Event::Eof => return None,
            _ => {}
          }
          writer.write_event(event.into_owned()).ok()?;
        }

        let xml = textbox_fragment_with_namespaces(String::from_utf8(writer.into_inner()).ok()?);
        return w::TextBoxContent::from_bytes(xml.as_bytes()).ok();
      }
      Event::Empty(event) if qname_ends_with(event.name().as_ref(), b"txbxContent") => {
        let mut writer = Writer::new(Vec::new());
        writer.write_event(Event::Empty(event.into_owned())).ok()?;
        let xml = textbox_fragment_with_namespaces(String::from_utf8(writer.into_inner()).ok()?);
        return w::TextBoxContent::from_bytes(xml.as_bytes()).ok();
      }
      Event::Eof => return None,
      _ => {}
    }
  }
}

fn textbox_fragment_with_namespaces(mut xml: String) -> String {
  if xml.contains("xmlns:w=") {
    return xml;
  }

  let namespaces = concat!(
    " xmlns:w=\"http://schemas.openxmlformats.org/wordprocessingml/2006/main\"",
    " xmlns:r=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships\"",
    " xmlns:w14=\"http://schemas.microsoft.com/office/word/2010/wordml\"",
    " xmlns:mc=\"http://schemas.openxmlformats.org/markup-compatibility/2006\""
  );

  if let Some(index) = xml.find('>') {
    if xml.as_bytes().get(index.saturating_sub(1)) == Some(&b'/') {
      xml.insert_str(index.saturating_sub(1), namespaces);
    } else {
      xml.insert_str(index, namespaces);
    }
  }

  xml
}

fn drawing_textbox_text(xml: &str) -> Option<String> {
  if !xml.contains("txbxContent") {
    return None;
  }

  let mut reader = Reader::from_str(xml);
  reader.config_mut().trim_text(true);
  let mut textbox_depth = 0usize;
  let mut paragraph_depth = 0usize;
  let mut in_text = false;
  let mut output = String::new();

  loop {
    match reader.read_event().ok()? {
      Event::Start(event) => {
        if qname_ends_with(event.name().as_ref(), b"txbxContent") {
          textbox_depth += 1;
        } else if textbox_depth > 0 && qname_ends_with(event.name().as_ref(), b"p") {
          paragraph_depth += 1;
        } else if textbox_depth > 0 && qname_ends_with(event.name().as_ref(), b"t") {
          in_text = true;
        }
      }
      Event::End(event) => {
        if qname_ends_with(event.name().as_ref(), b"t") {
          in_text = false;
        } else if textbox_depth > 0 && qname_ends_with(event.name().as_ref(), b"p") {
          paragraph_depth = paragraph_depth.saturating_sub(1);
          output.push('\n');
        } else if qname_ends_with(event.name().as_ref(), b"txbxContent") {
          textbox_depth = textbox_depth.saturating_sub(1);
        }
      }
      Event::Text(event) if textbox_depth > 0 && in_text => {
        output.push_str(event.xml10_content().ok()?.as_ref());
      }
      Event::CData(event) if textbox_depth > 0 && in_text => {
        output.push_str(event.xml10_content().ok()?.as_ref());
      }
      Event::Eof => break,
      _ => {}
    }
  }

  if paragraph_depth > 0 {
    output.push('\n');
  }
  (!output.is_empty()).then_some(output)
}

fn qname_ends_with(qname: &[u8], local_name: &[u8]) -> bool {
  qname == local_name
    || qname
      .strip_suffix(local_name)
      .is_some_and(|prefix| prefix.ends_with(b":"))
}

#[derive(Clone, Debug, Default)]
struct StylesCatalog {
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
  form_widget_ids: &'a mut FormWidgetIdAllocator,
  cell_margins: CellMargins,
  table_style: &'a TableStyleModel,
  table_look: TableLookModel,
  row_count: usize,
}

impl StylesCatalog {
  fn load(package: &mut WordprocessingDocument, main: &MainDocumentPart) -> Result<Self> {
    let theme = ThemeData::load(package, main);
    let Some(styles_part) = main.style_definitions_part(package) else {
      let mut catalog = Self {
        theme_fonts: theme.fonts,
        theme_colors: theme.colors,
        theme_lines: theme.lines,
        ..Self::default()
      };
      if catalog.doc_default_run.font_family.is_none() {
        catalog.doc_default_run.font_family = Some(Arc::<str>::from("Calibri"));
      }
      return Ok(catalog);
    };
    let styles = styles_part.root_element(package)?;
    let mut catalog = Self {
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

    for style in &styles.w_style {
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
      entry.table_style = table_style_model(style, &catalog.theme_fonts, &catalog.theme_colors);
      catalog.styles.insert(style_id.to_string(), entry);
    }

    if catalog.doc_default_run.font_family.is_none() {
      catalog.doc_default_run.font_family = catalog
        .theme_fonts
        .minor_high_ansi
        .clone()
        .or_else(|| catalog.theme_fonts.minor_ascii.clone())
        .or_else(|| Some(Arc::<str>::from("Calibri")));
    }

    Ok(catalog)
  }

  fn paragraph_format_with_base(
    &self,
    style_id: Option<&str>,
    base_format: ParagraphFormat,
  ) -> ParagraphFormat {
    let mut format = self.doc_default_paragraph.clone();
    merge_format_values(&mut format, base_format);
    let style_id = style_id
      .map(str::to_string)
      .or_else(|| self.default_paragraph_style_id.clone());
    for entry in self.style_chain(style_id.as_deref()) {
      merge_format_values(&mut format, entry.paragraph_format.clone());
    }
    format
  }

  fn paragraph_numbering_properties(
    &self,
    style_id: Option<&str>,
  ) -> Option<w::NumberingProperties> {
    let mut merged = None;
    for properties in self
      .style_chain(style_id)
      .into_iter()
      .filter_map(|entry| entry.paragraph_numbering.as_deref())
    {
      let target = merged.get_or_insert_with(w::NumberingProperties::default);
      merge_numbering_properties(target, properties);
    }
    merged
  }

  fn run_style_with_base(
    &self,
    style_id: Option<&str>,
    base_style: TextStyle,
    base_overrides: RunStyleOverrides,
  ) -> TextStyle {
    let mut style = self.doc_default_run.clone();
    merge_style_values(&mut style, base_style);
    apply_run_style_overrides(&mut style, base_overrides);
    let style_id = style_id
      .map(str::to_string)
      .or_else(|| self.default_paragraph_style_id.clone());
    for entry in self.style_chain(style_id.as_deref()) {
      merge_style_values(&mut style, entry.run_style.clone());
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
        merge_style_values(&mut style, entry.run_style.clone());
        apply_run_style_overrides(&mut style, entry.run_overrides);
      }
    }
    if !matched {
      merge_builtin_character_style(&mut style, style_id);
    }
    style
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

  fn style_chain(&self, style_id: Option<&str>) -> Vec<&StyleEntry> {
    let mut ids = Vec::new();
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

    ids
      .into_iter()
      .rev()
      .filter_map(|id| self.styles.get(id))
      .collect()
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
        .a_ln
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
  a::Dark1ColorChoice::ASrgbClr,
  a::Dark1ColorChoice::ASysClr
);
theme_color_choice_value!(
  light1_color_value,
  a::Light1ColorChoice,
  a::Light1ColorChoice::ASrgbClr,
  a::Light1ColorChoice::ASysClr
);
theme_color_choice_value!(
  dark2_color_value,
  a::Dark2ColorChoice,
  a::Dark2ColorChoice::ASrgbClr,
  a::Dark2ColorChoice::ASysClr
);
theme_color_choice_value!(
  light2_color_value,
  a::Light2ColorChoice,
  a::Light2ColorChoice::ASrgbClr,
  a::Light2ColorChoice::ASysClr
);
theme_color_choice_value!(
  accent1_color_value,
  a::Accent1ColorChoice,
  a::Accent1ColorChoice::ASrgbClr,
  a::Accent1ColorChoice::ASysClr
);
theme_color_choice_value!(
  accent2_color_value,
  a::Accent2ColorChoice,
  a::Accent2ColorChoice::ASrgbClr,
  a::Accent2ColorChoice::ASysClr
);
theme_color_choice_value!(
  accent3_color_value,
  a::Accent3ColorChoice,
  a::Accent3ColorChoice::ASrgbClr,
  a::Accent3ColorChoice::ASysClr
);
theme_color_choice_value!(
  accent4_color_value,
  a::Accent4ColorChoice,
  a::Accent4ColorChoice::ASrgbClr,
  a::Accent4ColorChoice::ASysClr
);
theme_color_choice_value!(
  accent5_color_value,
  a::Accent5ColorChoice,
  a::Accent5ColorChoice::ASrgbClr,
  a::Accent5ColorChoice::ASysClr
);
theme_color_choice_value!(
  accent6_color_value,
  a::Accent6ColorChoice,
  a::Accent6ColorChoice::ASrgbClr,
  a::Accent6ColorChoice::ASysClr
);
theme_color_choice_value!(
  hyperlink_color_value,
  a::HyperlinkChoice,
  a::HyperlinkChoice::ASrgbClr,
  a::HyperlinkChoice::ASysClr
);
theme_color_choice_value!(
  followed_hyperlink_color_value,
  a::FollowedHyperlinkColorChoice,
  a::FollowedHyperlinkColorChoice::ASrgbClr,
  a::FollowedHyperlinkColorChoice::ASysClr
);

pub(super) fn resolve_run_color(color: &w::Color, theme_colors: &ThemeColors) -> Option<RgbColor> {
  let has_theme_transform = color.theme_tint.is_some() || color.theme_shade.is_some();

  if !has_theme_transform && let Some(resolved) = parse_hex_color(&color.val) {
    return Some(resolved);
  }

  let mut resolved = color
    .theme_color
    .and_then(|value| theme_colors.resolve_wordprocessing(value))
    .or_else(|| parse_hex_color(&color.val))?;

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
    w14::FillTextEffectChoice::W14NoFill => None,
    w14::FillTextEffectChoice::W14SolidFill(fill) => resolve_solid_text_fill(fill, theme_colors),
    w14::FillTextEffectChoice::W14GradFill(_) => None,
  }
}

pub(super) fn resolve_text_outline(
  outline: &w14::TextOutlineEffect,
  theme_colors: &ThemeColors,
) -> Option<ResolvedColor> {
  let resolved = match outline.text_outline_effect_choice1.as_ref()? {
    w14::TextOutlineEffectChoice::W14NoFill => return None,
    w14::TextOutlineEffectChoice::W14SolidFill(fill) => {
      resolve_solid_text_fill(fill, theme_colors)?
    }
    w14::TextOutlineEffectChoice::W14GradFill(_) => return None,
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
    w14::SolidColorFillPropertiesChoice::W14SrgbClr(color) => Some(ResolvedColor {
      color: parse_hex_color(color.val.as_str())?,
      opacity: opacity_from_w14_rgb_transforms(&color.rgb_color_model_hex_choice),
    }),
    w14::SolidColorFillPropertiesChoice::W14SchemeClr(color) => {
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
    w14::RgbColorModelHexChoice::W14Alpha(value) => Some(value.val),
    _ => None,
  }))
}

fn opacity_from_w14_scheme_transforms(transforms: &[w14::SchemeColorChoice]) -> f32 {
  opacity_from_w14_alpha(transforms.iter().find_map(|transform| match transform {
    w14::SchemeColorChoice::W14Alpha(value) => Some(value.val),
    _ => None,
  }))
}

fn opacity_from_w14_alpha(alpha: Option<i32>) -> f32 {
  let transparency = alpha.unwrap_or(0) as f32 / units::DRAWINGML_PERCENT_SCALE;
  (1.0 - transparency).clamp(0.0, 1.0)
}

fn apply_w14_scheme_transforms(color: RgbColor, transforms: &[w14::SchemeColorChoice]) -> RgbColor {
  let mut hsl = HslColor::from_rgb(color);
  for transform in transforms {
    match transform {
      w14::SchemeColorChoice::W14Tint(value) => {
        hsl.apply_tint(value.val as f32 / units::DRAWINGML_PERCENT_SCALE);
      }
      w14::SchemeColorChoice::W14Shade(value) => {
        hsl.apply_shade(value.val as f32 / units::DRAWINGML_PERCENT_SCALE);
      }
      w14::SchemeColorChoice::W14LumMod(value) => {
        hsl.apply_luminance_mod(value.val as f32 / units::DRAWINGML_PERCENT_SCALE);
      }
      w14::SchemeColorChoice::W14LumOff(value) => {
        hsl.apply_luminance_offset(value.val as f32 / units::DRAWINGML_PERCENT_SCALE);
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
  );
  properties::merge_run_style(
    &mut model.whole_table.run_style,
    style.style_run_properties.as_deref().map(RunProps::Style),
    theme_fonts,
    theme_colors,
  );
  model.whole_table.run_overrides =
    run_style_overrides(style.style_run_properties.as_deref().map(RunProps::Style));
  for conditional in &style.w_tbl_style_pr {
    let mut cell_style = TableCellStyle::default();
    merge_paragraph_format(
      &mut cell_style.paragraph_format,
      conditional
        .style_paragraph_properties
        .as_deref()
        .map(ParagraphProps::Style),
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
      w::TableRowPropertiesChoice::WTrHeight(height) => {
        apply_table_row_height(&mut style, height);
      }
      w::TableRowPropertiesChoice::WTblHeader(header) => {
        style.repeat_header = Some(on_off_only_value(header.val));
      }
      w::TableRowPropertiesChoice::WCantSplit(cant_split) => {
        style.cant_split = Some(on_off_only_value(cant_split.val));
      }
      w::TableRowPropertiesChoice::WTblCellSpacing(spacing) => {
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
      w::TableStyleConditionalFormattingTableRowPropertiesChoice::WTblHeader(header) => {
        style.repeat_header = Some(on_off_only_value(header.val));
      }
      w::TableStyleConditionalFormattingTableRowPropertiesChoice::WCantSplit(cant_split) => {
        style.cant_split = Some(on_off_only_value(cant_split.val));
      }
      w::TableStyleConditionalFormattingTableRowPropertiesChoice::WTblCellSpacing(spacing) => {
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
  merge_format_values(
    &mut target.paragraph_format,
    source.paragraph_format.clone(),
  );
  merge_style_values(&mut target.run_style, source.run_style.clone());
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
  if let Some(hidden) = overrides.hidden {
    style.hidden = hidden;
  }
}

fn merge_format_values(target: &mut ParagraphFormat, values: ParagraphFormat) {
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
  if values.indent_left_pt != 0.0 {
    target.indent_left_pt = values.indent_left_pt;
  }
  if values.indent_right_pt != 0.0 {
    target.indent_right_pt = values.indent_right_pt;
  }
  if values.first_line_indent_pt != 0.0 {
    target.first_line_indent_pt = values.first_line_indent_pt;
  }
  if !values.tab_stops.is_empty() {
    target.tab_stops = values.tab_stops;
  }
  if values.alignment != ParagraphAlignment::default() {
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
}

fn merge_numbering_properties(
  target: &mut w::NumberingProperties,
  source: &w::NumberingProperties,
) {
  if source.numbering_level_reference.is_some() {
    target.numbering_level_reference = source.numbering_level_reference.clone();
  }
  if source.numbering_id.is_some() {
    target.numbering_id = source.numbering_id.clone();
  }
  if source.numbering_change.is_some() {
    target.numbering_change = source.numbering_change.clone();
  }
  if source.inserted.is_some() {
    target.inserted = source.inserted.clone();
  }
}

fn merge_style_values(target: &mut TextStyle, values: TextStyle) {
  if values.font_family.is_some() {
    target.font_family = values.font_family.clone();
  }
  if (values.font_size_pt - TextStyle::default().font_size_pt).abs() > f32::EPSILON {
    target.font_size_pt = values.font_size_pt;
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
  is_legal: bool,
  format_properties: ParagraphFormat,
  symbol_run_properties: Option<w::NumberingSymbolRunProperties>,
}

impl NumberingCatalog {
  fn load(package: &mut WordprocessingDocument, main: &MainDocumentPart) -> Result<Self> {
    let Some(numbering_part) = main.numbering_definitions_part(package) else {
      return Ok(Self::default());
    };
    let numbering = numbering_part.root_element(package)?;
    let mut catalog = Self::default();

    for abstract_num in &numbering.w_abstract_num {
      let mut entry = AbstractNumbering::default();
      for level in &abstract_num.w_lvl {
        entry
          .levels
          .insert(level.level_index, numbering_level_model(level));
      }
      catalog
        .abstract_nums
        .insert(abstract_num.abstract_number_id, entry);
    }

    for num in &numbering.w_num {
      let overrides = num
        .w_lvl_override
        .iter()
        .map(|level| {
          (
            level.level_index,
            LevelOverride {
              start: level
                .start_override_numbering_value
                .as_ref()
                .map(|value| value.val),
              level: level.level.as_deref().map(numbering_level_model),
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
    properties: &w::NumberingProperties,
    format: &mut ParagraphFormat,
    styles: &StylesCatalog,
    base_style: TextStyle,
  ) -> Option<(String, TextStyle)> {
    let num_id = properties.numbering_id.as_ref()?.val;
    let level_index = properties
      .numbering_level_reference
      .as_ref()
      .map(|level| level.val)
      .unwrap_or(0);
    let instance = self.nums.get(&num_id)?;
    let abstract_num = self.abstract_nums.get(&instance.abstract_num_id)?;
    let level_override = instance.overrides.get(&level_index);
    let level = level_override
      .and_then(|override_| override_.level.as_ref())
      .or_else(|| abstract_num.levels.get(&level_index))?;

    merge_format_values(format, level.format_properties.clone());
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

    let label = format_numbering_label(
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
    Some((label, style))
  }
}

fn numbering_level_model(level: &w::Level) -> NumberingLevel {
  let mut format_properties = ParagraphFormat::default();
  merge_paragraph_format(
    &mut format_properties,
    level
      .previous_paragraph_properties
      .as_deref()
      .map(ParagraphProps::Previous),
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
    is_legal: level.is_legal_numbering_style.is_some(),
    format_properties,
    symbol_run_properties: level.numbering_symbol_run_properties.as_deref().cloned(),
  }
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
    return format!("{} ", level.text);
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
  format!("{text} ")
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
    _ => value.to_string(),
  }
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

enum ParagraphProps<'a> {
  Direct(&'a w::ParagraphProperties),
  Style(&'a w::StyleParagraphProperties),
  BaseStyle(&'a w::ParagraphPropertiesBaseStyle),
  Previous(&'a w::PreviousParagraphProperties),
}

impl<'a> ParagraphProps<'a> {
  fn page_break_before(&self) -> Option<&'a w::PageBreakBefore> {
    match self {
      Self::Direct(properties) => properties.page_break_before.as_ref(),
      Self::Style(properties) => properties.page_break_before.as_ref(),
      Self::BaseStyle(properties) => properties.page_break_before.as_ref(),
      Self::Previous(properties) => properties.page_break_before.as_ref(),
    }
  }

  fn keep_next(&self) -> Option<&'a w::KeepNext> {
    match self {
      Self::Direct(properties) => properties.keep_next.as_ref(),
      Self::Style(properties) => properties.keep_next.as_ref(),
      Self::BaseStyle(properties) => properties.keep_next.as_ref(),
      Self::Previous(properties) => properties.keep_next.as_ref(),
    }
  }

  fn keep_lines(&self) -> Option<&'a w::KeepLines> {
    match self {
      Self::Direct(properties) => properties.keep_lines.as_ref(),
      Self::Style(properties) => properties.keep_lines.as_ref(),
      Self::BaseStyle(properties) => properties.keep_lines.as_ref(),
      Self::Previous(properties) => properties.keep_lines.as_ref(),
    }
  }

  fn contextual_spacing(&self) -> Option<&'a w::ContextualSpacing> {
    match self {
      Self::Direct(properties) => properties.contextual_spacing.as_ref(),
      Self::Style(properties) => properties.contextual_spacing.as_ref(),
      Self::BaseStyle(properties) => properties.contextual_spacing.as_ref(),
      Self::Previous(properties) => properties.contextual_spacing.as_ref(),
    }
  }

  fn spacing_between_lines(&self) -> Option<&'a w::SpacingBetweenLines> {
    match self {
      Self::Direct(properties) => properties.spacing_between_lines.as_ref(),
      Self::Style(properties) => properties.spacing_between_lines.as_ref(),
      Self::BaseStyle(properties) => properties.spacing_between_lines.as_ref(),
      Self::Previous(properties) => properties.spacing_between_lines.as_ref(),
    }
  }

  fn indentation(&self) -> Option<&'a w::Indentation> {
    match self {
      Self::Direct(properties) => properties.indentation.as_ref(),
      Self::Style(properties) => properties.indentation.as_ref(),
      Self::BaseStyle(properties) => properties.indentation.as_ref(),
      Self::Previous(properties) => properties.indentation.as_ref(),
    }
  }

  fn tabs(&self) -> Option<&'a w::Tabs> {
    match self {
      Self::Direct(properties) => properties.tabs.as_ref(),
      Self::Style(properties) => properties.tabs.as_ref(),
      Self::BaseStyle(properties) => properties.tabs.as_ref(),
      Self::Previous(properties) => properties.tabs.as_ref(),
    }
  }

  fn justification(&self) -> Option<&'a w::Justification> {
    match self {
      Self::Direct(properties) => properties.justification.as_ref(),
      Self::Style(properties) => properties.justification.as_ref(),
      Self::BaseStyle(properties) => properties.justification.as_ref(),
      Self::Previous(properties) => properties.justification.as_ref(),
    }
  }

  fn bidi(&self) -> Option<&'a w::BiDi> {
    match self {
      Self::Direct(properties) => properties.bi_di.as_ref(),
      Self::Style(properties) => properties.bi_di.as_ref(),
      Self::BaseStyle(properties) => properties.bi_di.as_ref(),
      Self::Previous(properties) => properties.bi_di.as_ref(),
    }
  }

  fn paragraph_borders(&self) -> Option<&'a w::ParagraphBorders> {
    match self {
      Self::Direct(properties) => properties.paragraph_borders.as_deref(),
      Self::Style(properties) => properties.paragraph_borders.as_deref(),
      Self::BaseStyle(properties) => properties.paragraph_borders.as_deref(),
      Self::Previous(properties) => properties.paragraph_borders.as_deref(),
    }
  }

  fn shading(&self) -> Option<&'a w::Shading> {
    match self {
      Self::Direct(properties) => properties.shading.as_ref(),
      Self::Style(properties) => properties.shading.as_ref(),
      Self::BaseStyle(properties) => properties.shading.as_ref(),
      Self::Previous(properties) => properties.shading.as_ref(),
    }
  }

  fn outline_level(&self) -> Option<&'a w::OutlineLevel> {
    match self {
      Self::Direct(properties) => properties.outline_level.as_ref(),
      Self::Style(properties) => properties.outline_level.as_ref(),
      Self::BaseStyle(properties) => properties.outline_level.as_ref(),
      Self::Previous(properties) => properties.outline_level.as_ref(),
    }
  }

  fn frame_properties(&self) -> Option<&'a w::FrameProperties> {
    match self {
      Self::Direct(properties) => properties.frame_properties.as_ref(),
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
}

impl<'a> RunProps<'a> {
  fn run_fonts(&self) -> Option<&'a w::RunFonts> {
    match self {
      Self::Direct(properties) => properties.run_fonts.as_ref(),
      Self::Style(properties) => properties.run_fonts.as_ref(),
      Self::BaseStyle(properties) => properties.run_fonts.as_ref(),
      Self::Numbering(properties) => properties.run_fonts.as_ref(),
    }
  }

  fn bold(&self) -> Option<&'a w::Bold> {
    match self {
      Self::Direct(properties) => properties.bold.as_ref(),
      Self::Style(properties) => properties.bold.as_ref(),
      Self::BaseStyle(properties) => properties.bold.as_ref(),
      Self::Numbering(properties) => properties.bold.as_ref(),
    }
  }

  fn italic(&self) -> Option<&'a w::Italic> {
    match self {
      Self::Direct(properties) => properties.italic.as_ref(),
      Self::Style(properties) => properties.italic.as_ref(),
      Self::BaseStyle(properties) => properties.italic.as_ref(),
      Self::Numbering(properties) => properties.italic.as_ref(),
    }
  }

  fn font_size(&self) -> Option<&'a w::FontSize> {
    match self {
      Self::Direct(properties) => properties.font_size.as_ref(),
      Self::Style(properties) => properties.font_size.as_ref(),
      Self::BaseStyle(properties) => properties.font_size.as_ref(),
      Self::Numbering(properties) => properties.font_size.as_ref(),
    }
  }

  fn color(&self) -> Option<&'a w::Color> {
    match self {
      Self::Direct(properties) => properties.color.as_ref(),
      Self::Style(properties) => properties.color.as_ref(),
      Self::BaseStyle(properties) => properties.color.as_ref(),
      Self::Numbering(properties) => properties.color.as_ref(),
    }
  }

  fn underline(&self) -> Option<&'a w::Underline> {
    match self {
      Self::Direct(properties) => properties.underline.as_ref(),
      Self::Style(properties) => properties.underline.as_ref(),
      Self::BaseStyle(properties) => properties.underline.as_ref(),
      Self::Numbering(properties) => properties.underline.as_ref(),
    }
  }

  fn strike(&self) -> Option<&'a w::Strike> {
    match self {
      Self::Direct(properties) => properties.strike.as_ref(),
      Self::Style(properties) => properties.strike.as_ref(),
      Self::BaseStyle(properties) => properties.strike.as_ref(),
      Self::Numbering(properties) => properties.strike.as_ref(),
    }
  }

  fn double_strike(&self) -> Option<&'a w::DoubleStrike> {
    match self {
      Self::Direct(properties) => properties.double_strike.as_ref(),
      Self::Style(properties) => properties.double_strike.as_ref(),
      Self::BaseStyle(properties) => properties.double_strike.as_ref(),
      Self::Numbering(properties) => properties.double_strike.as_ref(),
    }
  }

  fn caps(&self) -> Option<&'a w::Caps> {
    match self {
      Self::Direct(properties) => properties.caps.as_ref(),
      Self::Style(properties) => properties.caps.as_ref(),
      Self::BaseStyle(properties) => properties.caps.as_ref(),
      Self::Numbering(properties) => properties.caps.as_ref(),
    }
  }

  fn small_caps(&self) -> Option<&'a w::SmallCaps> {
    match self {
      Self::Direct(properties) => properties.small_caps.as_ref(),
      Self::Style(properties) => properties.small_caps.as_ref(),
      Self::BaseStyle(properties) => properties.small_caps.as_ref(),
      Self::Numbering(properties) => properties.small_caps.as_ref(),
    }
  }

  fn vanish(&self) -> Option<&'a w::Vanish> {
    match self {
      Self::Direct(properties) => properties.vanish.as_ref(),
      Self::Style(properties) => properties.vanish.as_ref(),
      Self::BaseStyle(properties) => properties.vanish.as_ref(),
      Self::Numbering(properties) => properties.vanish.as_ref(),
    }
  }

  fn vertical_text_alignment(&self) -> Option<&'a w::VerticalTextAlignment> {
    match self {
      Self::Direct(properties) => properties.vertical_text_alignment.as_ref(),
      Self::Style(properties) => properties.vertical_text_alignment.as_ref(),
      Self::BaseStyle(properties) => properties.vertical_text_alignment.as_ref(),
      Self::Numbering(properties) => properties.vertical_text_alignment.as_ref(),
    }
  }

  fn spacing(&self) -> Option<&'a w::Spacing> {
    match self {
      Self::Direct(properties) => properties.spacing.as_ref(),
      Self::Style(properties) => properties.spacing.as_ref(),
      Self::BaseStyle(properties) => properties.spacing.as_ref(),
      Self::Numbering(properties) => properties.spacing.as_ref(),
    }
  }

  fn text_fill(&self) -> Option<&'a w14::FillTextEffect> {
    match self {
      Self::Direct(properties) => properties.fill_text_effect.as_deref(),
      Self::Style(_) | Self::BaseStyle(_) | Self::Numbering(_) => None,
    }
  }

  fn text_outline(&self) -> Option<&'a w14::TextOutlineEffect> {
    match self {
      Self::Direct(properties) => properties.text_outline_effect.as_deref(),
      Self::Style(_) | Self::BaseStyle(_) | Self::Numbering(_) => None,
    }
  }

  fn highlight(&self) -> Option<&'a w::Highlight> {
    match self {
      Self::Direct(properties) => properties.highlight.as_ref(),
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
  match value {
    TwipsMeasureValue::UnsignedDecimalNumber(value) => Some(*value as f32),
    TwipsMeasureValue::PositiveUniversalMeasure(value) => {
      universal_measure_to_points(value).map(units::points_to_twips)
    }
  }
}

fn signed_twips_measure_to_twips(value: &SignedTwipsMeasureValue) -> Option<f32> {
  match value {
    SignedTwipsMeasureValue::Integer(value) => Some(*value as f32),
    SignedTwipsMeasureValue::UniversalMeasure(value) => {
      universal_measure_to_points(value).map(units::points_to_twips)
    }
  }
}

fn twips_measure_to_points(value: &TwipsMeasureValue) -> Option<f32> {
  twips_measure_to_twips(value).map(units::twips_to_points)
}

fn signed_twips_measure_to_points(value: &SignedTwipsMeasureValue) -> Option<f32> {
  signed_twips_measure_to_twips(value).map(units::twips_to_points)
}

fn measurement_or_percent_to_points(value: &MeasurementOrPercentValue) -> Option<f32> {
  match value {
    MeasurementOrPercentValue::DecimalNumberOrPercent(
      ooxmlsdk::simple_type::DecimalNumberOrPercentValue::DecimalNumber(value),
    ) => Some(units::twips_to_points(*value as f32)),
    MeasurementOrPercentValue::DecimalNumberOrPercent(
      ooxmlsdk::simple_type::DecimalNumberOrPercentValue::Percent(_),
    ) => None,
    MeasurementOrPercentValue::UniversalMeasure(value) => universal_measure_to_points(value),
  }
}

fn measurement_or_percent_to_percent(value: &MeasurementOrPercentValue) -> Option<f32> {
  match value {
    MeasurementOrPercentValue::DecimalNumberOrPercent(
      ooxmlsdk::simple_type::DecimalNumberOrPercentValue::DecimalNumber(value),
    ) => Some(*value as f32 / units::WORD_PERCENT_MEASURE_SCALE),
    MeasurementOrPercentValue::DecimalNumberOrPercent(
      ooxmlsdk::simple_type::DecimalNumberOrPercentValue::Percent(value),
    ) => value
      .strip_suffix('%')
      .and_then(|value| value.parse::<f32>().ok())
      .map(|value| value / units::VML_PERCENT_SCALE),
    MeasurementOrPercentValue::UniversalMeasure(_) => None,
  }
}

fn universal_measure_to_points(value: &str) -> Option<f32> {
  let (number, unit) = value
    .strip_suffix("mm")
    .map(|number| (number, "mm"))
    .or_else(|| value.strip_suffix("cm").map(|number| (number, "cm")))
    .or_else(|| value.strip_suffix("in").map(|number| (number, "in")))
    .or_else(|| value.strip_suffix("pt").map(|number| (number, "pt")))
    .or_else(|| value.strip_suffix("pc").map(|number| (number, "pc")))
    .or_else(|| value.strip_suffix("pi").map(|number| (number, "pi")))?;
  let number = number.parse::<f32>().ok()?;
  Some(match unit {
    "mm" => units::millimeters_to_points(number),
    "cm" => units::centimeters_to_points(number),
    "in" => units::inches_to_points(number),
    "pt" => number,
    "pc" | "pi" => number * units::POINTS_PER_PICA,
    _ => return None,
  })
}

fn page_setup(section: &w::SectionProperties) -> PageSetup {
  let mut setup = PageSetup::default();

  if let Some(size) = &section.w_pg_sz {
    if let Some(width) = size.width.as_ref().and_then(twips_measure_to_points) {
      setup.width_pt = width;
    }
    if let Some(height) = size.height.as_ref().and_then(twips_measure_to_points) {
      setup.height_pt = height;
    }
  }

  if let Some(margin) = &section.w_pg_mar {
    if let Some(top) = margin.top.as_ref().and_then(signed_twips_measure_to_twips) {
      setup.top_margin_was_negative = top < 0.0;
      setup.margin_top_pt = units::twips_to_points(top.max(0.0));
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
      setup.margin_bottom_pt = units::twips_to_points(bottom.max(0.0));
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

  if let Some(borders) = &section.w_pg_borders {
    setup.borders = page_borders_model(borders);
    setup.borders_offset_from_text =
      matches!(borders.offset_from, Some(w::PageBorderOffsetValues::Text));
  }

  setup
}

#[cfg(test)]
mod tests {
  use super::*;

  fn twips(value: u32) -> TwipsMeasureValue {
    TwipsMeasureValue::UnsignedDecimalNumber(value)
  }

  fn measurement(value: i32) -> MeasurementOrPercentValue {
    MeasurementOrPercentValue::DecimalNumberOrPercent(
      ooxmlsdk::simple_type::DecimalNumberOrPercentValue::DecimalNumber(value),
    )
  }

  #[test]
  fn drawing_image_properties_preserve_crop_and_transform() {
    let xml = r#"<pic:pic xmlns:pic="http://schemas.openxmlformats.org/drawingml/2006/picture" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"><pic:blipFill><a:blip r:embed="rId7"/><a:srcRect l="10000" t="20000" r="30000" b="40000"/></pic:blipFill><pic:spPr><a:xfrm rot="5400000" flipH="1" flipV="true"/></pic:spPr></pic:pic>"#;

    let properties = drawing_image_properties_from_xml(xml).expect("image properties");

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
    // Source: LibreOffice imports <wps:txbx> through WpsContext as text on the
    // drawing shape, not as fallback body text.
    let xml = r#"<wps:wsp xmlns:wps="http://schemas.microsoft.com/office/word/2010/wordprocessingShape" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><wps:cNvSpPr txBox="1"/><wps:spPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="857250" cy="742950"/></a:xfrm><a:prstGeom prst="rect"><a:avLst/></a:prstGeom></wps:spPr><wps:txbx><w:txbxContent><w:p><w:r><w:t>inside shape</w:t></w:r></w:p></w:txbxContent></wps:txbx><wps:bodyPr lIns="91440" tIns="45720" rIns="91440" bIns="45720" anchor="t"/></wps:wsp>"#;
    assert!(drawing_textbox_content(xml).is_some());
    assert!(drawingml_shape_geometry(xml).is_some());

    let frame = drawingml_textbox_frame_from_fragment(
      xml,
      ImagePlacement::Inline,
      DrawingMlGroupTransform::identity(),
      &StylesCatalog::default(),
      &ImageCatalog::default(),
      &HyperlinkCatalog::default(),
    )
    .expect("wps textbox frame");

    assert!((frame.offset_x_pt - 7.2).abs() < 0.001);
    assert!((frame.offset_y_pt - 3.6).abs() < 0.001);
    assert!((frame.width_pt - 53.1).abs() < 0.001);
    assert!((frame.height_pt - 51.3).abs() < 0.001);
    assert_eq!(frame.text_box_blocks.len(), 1);
  }

  #[test]
  fn symbol_runs_emit_unicode_text() {
    let mut inlines = Vec::new();
    let run = w::Run {
      run_choice: vec![
        w::RunChoice::WSym(Box::new(w::SymbolChar {
          font: Some("Symbol".into()),
          char: Some("F0B7".into()),
        })),
        w::RunChoice::WSym(Box::new(w::SymbolChar {
          font: Some("Wingdings".into()),
          char: Some("F0FC".into()),
        })),
        w::RunChoice::WSym(Box::new(w::SymbolChar {
          font: None,
          char: Some("00A9".into()),
        })),
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
        w::TableRowPropertiesChoice::WGridBefore(Box::new(w::GridBefore { val: 1 })),
        w::TableRowPropertiesChoice::WGridAfter(Box::new(w::GridAfter { val: 2 })),
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
        w_tbl_style_pr: vec![w::TableStyleProperties {
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
              val: "FFFFFF".into(),
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
      val: "100000000000".into(),
      first_row: Some(true.into()),
      ..Default::default()
    });
    let cell_condition = TableConditionalStyleMask::from_cnf_style(&w::ConditionalFormatStyle {
      val: "000100000000".into(),
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
        w_tbl_style_pr: vec![w::TableStyleProperties {
          r#type: w::TableStyleOverrideValues::FirstRow,
          table_style_conditional_formatting_table_row_properties: Some(
            w::TableStyleConditionalFormattingTableRowProperties {
              table_style_conditional_formatting_table_row_properties_choice: vec![
                w::TableStyleConditionalFormattingTableRowPropertiesChoice::WTblHeader(Box::new(
                  w::TableHeader { val: None },
                )),
                w::TableStyleConditionalFormattingTableRowPropertiesChoice::WCantSplit(Box::new(
                  w::CantSplit { val: None },
                )),
                w::TableStyleConditionalFormattingTableRowPropertiesChoice::WTblCellSpacing(
                  Box::new(w::TableCellSpacing {
                    width: Some(measurement(240)),
                    r#type: Some(w::TableWidthUnitValues::Dxa),
                  }),
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
          w::TableRowPropertiesChoice::WTblHeader(Box::new(w::TableHeader {
            val: Some(ooxmlsdk::simple_type::OnOffValue::Off),
          })),
          w::TableRowPropertiesChoice::WTblCellSpacing(Box::new(w::TableCellSpacing {
            width: Some(measurement(120)),
            r#type: Some(w::TableWidthUnitValues::Dxa),
          })),
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
        w_tbl_style_pr: vec![w::TableStyleProperties {
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
    );

    assert_eq!(style.alignment, Some(TableAlignment::Center));
    assert_eq!(style.indent_left_pt, Some(36.0));
    assert_eq!(style.cell_spacing_pt, Some(6.0));
  }

  #[test]
  fn explicit_zero_paragraph_spacing_overrides_doc_default_spacing() {
    // Source: LibreOffice writerfilter/dmapper imports explicit paragraph
    // spacing properties into the property map even when the value is zero.
    let mut format = ParagraphFormat {
      spacing_after_pt: 8.0,
      spacing_after_set: true,
      ..Default::default()
    };

    merge_format_values(
      &mut format,
      ParagraphFormat {
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
      paragraph_choice: vec![w::ParagraphChoice::WR(Box::new(w::Run {
        run_properties: Some(Box::new(w::RunProperties {
          bold: Some(w::Bold {
            val: Some(false.into()),
          }),
          color: Some(w::Color {
            val: "0000FF".into(),
            ..Default::default()
          }),
          ..Default::default()
        })),
        run_choice: vec![w::RunChoice::WT(Box::new(w::Text {
          xml_content: Some("Header".into()),
          ..Default::default()
        }))],
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
      table_cell_choice: vec![w::TableCellChoice::WP(Box::new(w::Paragraph {
        paragraph_choice: vec![w::ParagraphChoice::WR(Box::new(w::Run {
          run_choice: vec![w::RunChoice::WT(Box::new(w::Text {
            xml_content: Some("Header".into()),
            ..Default::default()
          }))],
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
    let mut context = TableImportContext {
      styles: &styles,
      numbering: &mut numbering,
      images: &images,
      hyperlinks: &hyperlinks,
      form_widget_ids: &mut form_widget_ids,
      cell_margins: CellMargins::default(),
      table_style: &TableStyleModel::default(),
      table_look: TableLookModel::default(),
      row_count: 1,
    };

    let cell = table_cell_model(&cell, &mut context, style);

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

    push_simple_field(
      &field,
      &mut inlines,
      TextStyle::default(),
      &StylesCatalog::default(),
      &ImageCatalog::default(),
      &HyperlinkCatalog::default(),
      &mut FormWidgetIdAllocator::default(),
    );

    let InlineItem::Text(run) = &inlines[0] else {
      panic!("expected dynamic field text");
    };
    assert_eq!(run.dynamic_field, Some(DynamicFieldKind::Page));
  }

  #[test]
  fn pgnum_runs_emit_dynamic_page_marker() {
    let mut inlines = Vec::new();
    let run = w::Run {
      run_choice: vec![w::RunChoice::WPgNum],
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
      ruby_base: Box::new(w::RubyBase {
        ruby_base_choice: vec![w::RubyBaseChoice::WR(Box::new(w::Run {
          run_choice: vec![w::RunChoice::WT(Box::new(w::Text {
            xml_content: Some("漢".into()),
            ..Default::default()
          }))],
          ..Default::default()
        }))],
        ..Default::default()
      }),
      ..Default::default()
    };
    let run = w::Run {
      run_choice: vec![
        w::RunChoice::WT(Box::new(w::Text {
          xml_content: Some("Before ".into()),
          ..Default::default()
        })),
        w::RunChoice::WRuby(Box::new(ruby)),
        w::RunChoice::WT(Box::new(w::Text {
          xml_content: Some(" after".into()),
          ..Default::default()
        })),
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
        data: vec![1, 2, 3],
        content_type: Some("image/png".into()),
      },
    );
    let run = w::Run {
      run_choice: vec![w::RunChoice::WPict(Box::new(w::Picture {
        picture_choice: vec![w::PictureChoice::VShape(Box::new(v::Shape {
          style: Some("width:1in;height:24pt;rotation:90;flip:x y".into()),
          alternate: Some("VML image".into()),
          shape_choice: vec![v::ShapeChoice::VImagedata(Box::new(v::ImageData {
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
        | InlineItem::FormWidgetStart(_)
        | InlineItem::FormWidgetEnd(_)
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
      run_choice: vec![w::RunChoice::WPict(Box::new(w::Picture {
        picture_choice: vec![w::PictureChoice::VShape(Box::new(v::Shape {
          shape_choice: vec![v::ShapeChoice::VTextbox(Box::new(v::TextBox {
            text_box_choice: Some(v::TextBoxChoice::WTxbxContent(Box::new(
              w::TextBoxContent {
                text_box_content_choice: vec![w::TextBoxContentChoice::WP(Box::new(
                  w::Paragraph {
                    paragraph_choice: vec![w::ParagraphChoice::WR(Box::new(w::Run {
                      run_choice: vec![w::RunChoice::WT(Box::new(w::Text {
                        xml_content: Some("Text inside VML box".into()),
                        ..Default::default()
                      }))],
                      ..Default::default()
                    }))],
                    ..Default::default()
                  },
                ))],
                ..Default::default()
              },
            ))),
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
  fn drawing_textboxes_extract_cached_text() {
    let xml = r#"<wps:wsp xmlns:wps="http://schemas.microsoft.com/office/word/2010/wordprocessingShape" xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <wps:txbx>
    <w:txbxContent>
      <w:p><w:r><w:t>Modern text box</w:t></w:r></w:p>
      <w:p><w:r><w:t>Second line</w:t></w:r></w:p>
    </w:txbxContent>
  </wps:txbx>
</wps:wsp>"#;

    assert_eq!(
      drawing_textbox_text(xml).as_deref(),
      Some("Modern text box\nSecond line\n")
    );
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
        w::BodyChoice::WP(Box::new(paragraph())),
        w::BodyChoice::WP(Box::new(paragraph_with_section(section(
          12240,
          15840,
          w::PageOrientationValues::Portrait,
          None,
        )))),
        w::BodyChoice::WP(Box::new(paragraph())),
      ],
      w_sect_pr: Some(Box::new(section(
        15840,
        12240,
        w::PageOrientationValues::Landscape,
        Some(w::SectionMarkValues::Continuous),
      ))),
      ..Default::default()
    };
    let mut numbering = NumberingCatalog::default();

    let sections = body_sections(
      &body,
      &StylesCatalog::default(),
      &mut numbering,
      &ImageCatalog::default(),
      &HyperlinkCatalog::default(),
      &mut FormWidgetIdAllocator::default(),
    );

    assert_eq!(sections.len(), 2);
    assert_eq!(sections[0].blocks.len(), 2);
    assert_eq!(sections[0].break_kind, SectionBreakKind::NextPage);
    assert_eq!(sections[0].page.width_pt, 612.0);
    assert_eq!(sections[0].page.height_pt, 792.0);
    assert_eq!(sections[1].blocks.len(), 1);
    assert_eq!(sections[1].break_kind, SectionBreakKind::NextPage);
    assert_eq!(sections[1].page.width_pt, 792.0);
    assert_eq!(sections[1].page.height_pt, 612.0);
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
      w_type: break_type.map(|val| w::SectionType { val }),
      w_pg_sz: Some(w::PageSize {
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
      w_type: Some(w::SectionType { val: break_type }),
      w_cols: Some(w::Columns {
        column_count: Some(column_count),
        ..Default::default()
      }),
      ..Default::default()
    }
  }

  fn explicit_columns_section(break_type: w::SectionMarkValues) -> w::SectionProperties {
    w::SectionProperties {
      w_type: Some(w::SectionType { val: break_type }),
      w_cols: Some(w::Columns {
        equal_width: Some(false.into()),
        w_col: vec![
          w::Column {
            width: Some(twips(1440)),
            space: Some(twips(720)),
          },
          w::Column {
            width: Some(twips(2880)),
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
        | InlineItem::FormWidgetStart(_)
        | InlineItem::FormWidgetEnd(_)
        | InlineItem::PageBreak
        | InlineItem::ColumnBreak => None,
      })
      .collect()
  }

  #[test]
  fn wordart_outline_fragment_resolves_expected_color_and_opacity() {
    let fragment = textbox_fragment_with_namespaces(
      r#"<w14:textOutline w14:w="228600" w14:cap="rnd" w14:cmpd="sng" w14:algn="ctr"><w14:solidFill><w14:schemeClr w14:val="accent2"><w14:alpha w14:val="20000"/><w14:lumMod w14:val="75000"/></w14:schemeClr></w14:solidFill><w14:prstDash w14:val="sysDot"/><w14:bevel/></w14:textOutline>"#.to_string(),
    );
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
    let fragment = textbox_fragment_with_namespaces(
      r#"<w:rPr><w:color w:val="D6E3BC" w:themeColor="accent3" w:themeTint="66"/><w14:textOutline w14:w="228600" w14:cap="rnd" w14:cmpd="sng" w14:algn="ctr"><w14:solidFill><w14:schemeClr w14:val="accent2"><w14:alpha w14:val="20000"/><w14:lumMod w14:val="75000"/></w14:schemeClr></w14:solidFill><w14:prstDash w14:val="sysDot"/><w14:bevel/></w14:textOutline></w:rPr>"#.to_string(),
    );
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

    if let Ok(properties) = w::RunProperties::from_bytes(fragment.as_bytes()) {
      style = properties::run_style(Some(&properties), style, &styles);
    }
    apply_text_effect_overrides_from_fragment(&mut style, &fragment, &styles);

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
