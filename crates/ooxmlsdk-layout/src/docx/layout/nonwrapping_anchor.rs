use super::*;
use crate::docx::{ImagePlacement, Paragraph};

#[derive(Clone, Copy, Default)]
pub(super) struct Offset {
  x_pt: f32,
  y_pt: f32,
  first_line_y_pt: f32,
}

fn applies(shape: &InlineShape, placement: FloatingImagePlacement, flow: FlowContext) -> bool {
  flow.compatibility_mode < 15
    && flow.text_segmentation == TextSegmentation::Body
    && !flow.inside_paragraph_frame
    && shape.wordprocessing_shape_host
    && placement.paint_order.is_drawing_ml()
    && placement.alignment_extent.is_none()
    && placement.wrap == ImageWrapMode::None
    && (placement.horizontal_relative_to == HorizontalImageReference::Column
      || matches!(
        placement.vertical_relative_to,
        VerticalImageReference::Paragraph | VerticalImageReference::Line
      ))
}

impl Offset {
  pub(super) fn position(
    self,
    shape: &InlineShape,
    placement: FloatingImagePlacement,
    flow: FlowContext,
    anchor_y: f32,
  ) -> (FlowContext, f32) {
    if !applies(shape, placement, flow) {
      return (flow, anchor_y);
    }
    let flow = if placement.horizontal_relative_to == HorizontalImageReference::Column {
      FlowContext {
        content_left_pt: flow.content_left_pt + self.x_pt,
        ..flow
      }
    } else {
      flow
    };
    let anchor_y = match placement.vertical_relative_to {
      VerticalImageReference::Paragraph => anchor_y + self.y_pt,
      // A resolved anchor line can already include the first-line clearance.
      // Preserve later physical lines instead of adding the dummy span twice.
      VerticalImageReference::Line if self.x_pt > LAYOUT_EPSILON_PT => {
        anchor_y.max(self.first_line_y_pt)
      }
      _ => anchor_y,
    };
    (flow, anchor_y)
  }
}

pub(super) fn paragraph_offset(
  paragraph: &Paragraph,
  flow: FlowContext,
  frame: TextFrame,
  line_y: f32,
  prior_exclusions: &[WrapExclusion],
) -> Offset {
  if flow.compatibility_mode >= 15
    || flow.text_segmentation != TextSegmentation::Body
    || flow.inside_paragraph_frame
    || !paragraph.inlines.iter().any(|item| {
      matches!(item, InlineItem::Shape(shape)
      if matches!(shape.placement, ImagePlacement::Floating(placement)
        if applies(shape, placement, flow)))
    })
  {
    return Offset::default();
  }
  // Word <=2010 adds the first real line's fly base to no-wrap WPS anchors.
  // Wrapping siblings still use their original base, avoiding self feedback.
  // See Writer CalcBaseOfstForFly/GetTopForObjPos and native mode11/12/14/15
  // controls. Reuse the complete pre-registered wraps once per paragraph.
  let exclusions = prior_exclusions
    .iter()
    .filter(|exclusion| exclusion.owner != WrapExclusionOwner::DisplacedHeaderDrawing)
    .copied()
    .collect::<Vec<_>>();
  if exclusions.is_empty() {
    return Offset::default();
  }
  // The fly base uses the column frame, not the paragraph's indented lane.
  // Native36/72pt indents retain the same offset even when the paragraph
  // mark is already to the right of a wrapping sibling.
  let indents = frame.paragraph_indents();
  let base_left = flow.content_left_pt;
  let base_right = base_left + flow.content_width + indents.left_pt + indents.right_pt;
  let y = dodge_text_wrap_exclusions(
    line_y,
    frame.base_line_height,
    base_left,
    base_right,
    &exclusions,
  );
  let (left, _) = line_bounds_for_band(
    base_left,
    base_right,
    WrapLineBand::full(y, frame.base_line_height),
    &exclusions,
  );
  // A purely vertical dummy-line clearance does not change the fly base.
  // Native full-width-only controls keep both offsets zero. An ordinary
  // paragraph indent alone likewise is not an offset caused by wrapping.
  if left <= base_left + LAYOUT_EPSILON_PT {
    return Offset::default();
  }
  Offset {
    x_pt: left - flow.content_left_pt,
    y_pt: y - line_y,
    first_line_y_pt: y,
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use ooxmlsdk::parts::wordprocessing_document::WordprocessingDocument;
  use ooxmlsdk::sdk::WordprocessingDocumentType;

  fn native_control(
    mode: u16,
    mask: usize,
    wrap: &str,
    horizontal: &str,
    vertical: &str,
    context: &str,
  ) -> (f32, f32, Option<f32>) {
    let mut package = WordprocessingDocument::create(WordprocessingDocumentType::Document);
    let main = package.add_main_document_part().unwrap();
    let mut xml = String::from(
      r#"<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:wp="http://schemas.openxmlformats.org/drawingml/2006/wordprocessingDrawing" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:wps="http://schemas.microsoft.com/office/word/2010/wordprocessingShape"><w:body><w:p>"#,
    );
    if let Some(indent) = match context {
      "indent36" => Some(720),
      "indent72" => Some(1440),
      _ => None,
    } {
      xml.push_str(&format!(r#"<w:pPr><w:ind w:left="{indent}"/></w:pPr>"#));
    }
    if context.starts_with("prefix") {
      xml.push_str(r#"<w:r><w:rPr><w:rFonts w:ascii="Calibri" w:hAnsi="Calibri"/><w:sz w:val="22"/></w:rPr><w:t>PREFIX</w:t>"#);
      if context == "prefix2" {
        xml.push_str("<w:br/><w:t>SECOND</w:t>");
      }
      xml.push_str("</w:r>");
    }
    for (number, x, y, cx, cy, right, bottom) in [
      (3, 360045, 360045, 1155600, 1242000, 26035, 15875),
      (2, -725372, 540385, 1274400, 1209600, 21590, 10160),
      (1, -725805, -540385, 7325360, 1029335, 27940, 18415),
    ] {
      let enabled = mask & if number == 1 { 1 } else { 2 } != 0;
      let wrap = if number == 3 {
        wrap
      } else if enabled {
        "Square"
      } else {
        "None"
      };
      let wrap = if wrap == "Square" {
        r#"<wp:wrapSquare wrapText="bothSides"/>"#
      } else {
        "<wp:wrapNone/>"
      };
      let h = if number == 3 { horizontal } else { "column" };
      let v = if number == 3 { vertical } else { "paragraph" };
      xml.push_str(&format!(r#"<w:r><w:drawing><wp:anchor distT="0" distB="0" distL="114300" distR="114300" simplePos="0" relativeHeight="{}" behindDoc="0" locked="0" layoutInCell="1" allowOverlap="1"><wp:simplePos x="0" y="0"/><wp:positionH relativeFrom="{h}"><wp:posOffset>{x}</wp:posOffset></wp:positionH><wp:positionV relativeFrom="{v}"><wp:posOffset>{y}</wp:posOffset></wp:positionV><wp:extent cx="{cx}" cy="{cy}"/><wp:effectExtent l="0" t="0" r="{right}" b="{bottom}"/>{wrap}<wp:docPr id="{number}" name="Rectangle {number}"/><a:graphic><a:graphicData uri="http://schemas.microsoft.com/office/word/2010/wordprocessingShape"><wps:wsp><wps:cNvSpPr/><wps:spPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="{cx}" cy="{cy}"/></a:xfrm><a:prstGeom prst="rect"><a:avLst/></a:prstGeom><a:solidFill><a:srgbClr val="4F81BD"/></a:solidFill><a:ln w="25400"><a:solidFill><a:srgbClr val="385D8A"/></a:solidFill></a:ln></wps:spPr><wps:bodyPr/></wps:wsp></a:graphicData></a:graphic></wp:anchor></w:drawing></w:r>"#,251658240+number*1024));
    }
    xml.push_str(r#"</w:p><w:sectPr><w:pgSz w:w="12240" w:h="15840"/><w:pgMar w:top="1417" w:right="1417" w:bottom="1417" w:left="1417"/></w:sectPr></w:body></w:document>"#);
    main.set_data(&mut package, xml.into_bytes()).unwrap();
    let options = LayoutOptions::default();
    let mut source = crate::docx::extract(&package, &options).unwrap();
    source.compatibility_mode = mode;
    let placed = layout(&source, &options).unwrap();
    assert_eq!(placed.pages.len(), 1);
    fn target(items: &[PageItem]) -> Option<(f32, f32)> {
      items.iter().find_map(|item| match item {
        PageItem::Rect(rect) if (rect.width_pt - 90.95).abs() < 0.1 => Some((rect.x_pt, rect.y_pt)),
        PageItem::Path(path) if (path.bounds.size.width.0 - 90.95).abs() < 0.1 => {
          Some((path.bounds.origin.x.0, path.bounds.origin.y.0))
        }
        PageItem::Group(items)
        | PageItem::InlineObjectGroup(items)
        | PageItem::IndependentTextFrame(items)
        | PageItem::CompositingGroup { items, .. }
        | PageItem::FloatingDrawing { items, .. } => target(items),
        _ => None,
      })
    }
    fn text_y(items: &[PageItem], wanted: &str) -> Option<f32> {
      items.iter().find_map(|item| match item {
        PageItem::Text(text) if text.text == wanted => Some(text.y_pt),
        PageItem::Group(items)
        | PageItem::InlineObjectGroup(items)
        | PageItem::IndependentTextFrame(items)
        | PageItem::CompositingGroup { items, .. }
        | PageItem::FloatingDrawing { items, .. } => text_y(items, wanted),
        _ => None,
      })
    }
    let items = &placed.pages[0].items;
    let target = target(items).expect("painted target rectangle retained");
    let advance = text_y(items, "SECOND")
      .zip(text_y(items, "PREFIX"))
      .map(|(second, first)| second - first);
    (target.0, target.1, advance)
  }

  #[test]
  fn no_wrap_base_requires_legacy_mode_and_a_side_wrap_after_dummy_lines() {
    // Complete native mask/mode/wrap opposites, not just the failing document.
    for mode in [11, 12, 14, 15] {
      for mask in 0..4 {
        for wrap in ["None", "Square"] {
          let expected = if mode < 15 && mask == 3 && wrap == "None" {
            (153.1, 139.15)
          } else {
            (99.2, 99.2)
          };
          let actual = native_control(mode, mask, wrap, "column", "paragraph", "");
          assert!(
            (actual.0 - expected.0).abs() < 0.02 && (actual.1 - expected.1).abs() < 0.02,
            "mode{mode} mask{mask} {wrap}: {actual:?}"
          );
        }
      }
    }
  }

  #[test]
  fn no_wrap_offsets_respect_the_two_independent_reference_axes() {
    for (h, x) in [("column", 153.1), ("margin", 99.2), ("page", 28.35)] {
      for (v, y) in [("paragraph", 139.15), ("line", 139.15), ("page", 28.35)] {
        let actual = native_control(14, 3, "None", h, v, "");
        assert!(
          (actual.0 - x).abs() < 0.02 && (actual.1 - y).abs() < 0.02,
          "{h}/{v}: {actual:?}"
        );
      }
    }
  }

  #[test]
  fn no_wrap_base_keeps_the_anchor_line_and_indent_owners_separate() {
    for (context, vertical, y) in [
      ("prefix1", "line", 139.15),
      ("prefix1", "paragraph", 139.15),
      ("prefix2", "paragraph", 139.15),
      ("indent36", "paragraph", 139.15),
      ("indent72", "paragraph", 139.15),
    ] {
      let actual = native_control(14, 3, "None", "column", vertical, context);
      assert!(
        (actual.0 - 153.1).abs() < 0.02 && (actual.1 - y).abs() < 0.02,
        "{context}/{vertical}: {actual:?}"
      );
    }
    // This synthetic package has no native theme/default paragraph-mark
    // style. Check the line-relative anchor against its actual physical-line
    // advance; the complete native documents independently pin15.44pt.
    let actual = native_control(14, 3, "None", "column", "line", "prefix2");
    let advance = actual.2.expect("two physical source lines retained");
    assert!((actual.0 - 153.1).abs() < 0.02);
    assert!((actual.1 - 139.15 - advance).abs() < 0.02, "{actual:?}");
  }
}
