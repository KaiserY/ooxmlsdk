use ooxmlsdk::parts::presentation_document::PresentationDocument;
use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;
use ooxmlsdk::schemas::schemas_openxmlformats_org_presentationml_2006_main as p;

use crate::error::Result;
use crate::layout::{self, LayoutDocument};
use crate::options::PdfOptions;
use crate::units;

// Source: LibreOffice sd/source/filter/eppt/pptx-epptbase.cxx falls back to
// a 28000 x 21000 mm100 master page when exporting a presentation with no page
// property value.
const LO_DEFAULT_SLIDE_WIDTH_MM100: f32 = 28_000.0;
const LO_DEFAULT_SLIDE_HEIGHT_MM100: f32 = 21_000.0;
const DEFAULT_PRESENTATION_MARGIN_PT: f32 = 0.0;

pub(crate) fn layout(
  package: &mut PresentationDocument,
  _options: &PdfOptions,
) -> Result<LayoutDocument> {
  let presentation_part = package.presentation_part()?;
  let setup = {
    let presentation = presentation_part.root_element(package)?;
    presentation
      .slide_size
      .as_ref()
      .map(|size| crate::docx::PageSetup {
        width_pt: units::emu_to_points(i64::from(size.cx)),
        height_pt: units::emu_to_points(i64::from(size.cy)),
        margin_top_pt: DEFAULT_PRESENTATION_MARGIN_PT,
        margin_right_pt: DEFAULT_PRESENTATION_MARGIN_PT,
        margin_bottom_pt: DEFAULT_PRESENTATION_MARGIN_PT,
        margin_left_pt: DEFAULT_PRESENTATION_MARGIN_PT,
        ..crate::docx::PageSetup::default()
      })
      .unwrap_or(crate::docx::PageSetup {
        width_pt: units::millimeters_to_points(
          LO_DEFAULT_SLIDE_WIDTH_MM100 / units::MM100_PER_MILLIMETER,
        ),
        height_pt: units::millimeters_to_points(
          LO_DEFAULT_SLIDE_HEIGHT_MM100 / units::MM100_PER_MILLIMETER,
        ),
        margin_top_pt: DEFAULT_PRESENTATION_MARGIN_PT,
        margin_right_pt: DEFAULT_PRESENTATION_MARGIN_PT,
        margin_bottom_pt: DEFAULT_PRESENTATION_MARGIN_PT,
        margin_left_pt: DEFAULT_PRESENTATION_MARGIN_PT,
        ..crate::docx::PageSetup::default()
      })
  };

  let mut pages = Vec::new();
  let slide_parts = presentation_part.slide_parts(package).collect::<Vec<_>>();
  for (index, slide_part) in slide_parts.iter().enumerate() {
    let slide = slide_part.root_element(package)?;
    let mut lines = vec![format!("Slide {}", index + 1)];
    collect_shape_tree_text(&slide.common_slide_data.shape_tree, &mut lines);
    pages.push((setup, lines));
  }

  Ok(layout::text_pages(pages))
}

fn collect_shape_tree_text(tree: &p::ShapeTree, lines: &mut Vec<String>) {
  for choice in &tree.shape_tree_choice {
    match choice {
      p::ShapeTreeChoice::PSp(shape) => collect_shape_text(shape, lines),
      p::ShapeTreeChoice::PGrpSp(group) => collect_group_text(group, lines),
      _ => {}
    }
  }
}

fn collect_group_text(group: &p::GroupShape, lines: &mut Vec<String>) {
  for choice in &group.group_shape_choice {
    match choice {
      p::GroupShapeChoice::PSp(shape) => collect_shape_text(shape, lines),
      p::GroupShapeChoice::PGrpSp(group) => collect_group_text(group, lines),
      _ => {}
    }
  }
}

fn collect_shape_text(shape: &p::Shape, lines: &mut Vec<String>) {
  let Some(text_body) = &shape.text_body else {
    return;
  };

  for paragraph in &text_body.a_p {
    let text = paragraph_text(paragraph);
    if !text.is_empty() {
      lines.push(text);
    }
  }
}

fn paragraph_text(paragraph: &a::Paragraph) -> String {
  let mut text = String::new();
  for choice in &paragraph.paragraph_choice {
    match choice {
      a::ParagraphChoice::AR(run) => text.push_str(&run.text),
      a::ParagraphChoice::ABr(_) => text.push('\n'),
      a::ParagraphChoice::AFld(field) => {
        if let Some(value) = &field.text {
          text.push_str(value);
        }
      }
      a::ParagraphChoice::A14M(_) => {}
    }
  }
  text
}
