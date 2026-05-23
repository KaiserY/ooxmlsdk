use std::collections::HashMap;
use std::io::Cursor;
use std::sync::Arc;

use crate::docx::{BorderStyle, ImageCrop, RgbColor, TextStyle};
use crate::layout::{
  self, ImageItem, LineItem, LineItemKind, LinkAreaItem, PageItem, PdfTextSegmentation, RectItem,
  TextItem,
};
use crate::render::chart as shared_chart;
use crate::render::diagram as shared_diagram;
use crate::render::emf_wmf;
use crate::text_metrics::measure_text;
use crate::units;
use image::codecs::png::PngEncoder;
use image::{ColorType, ImageEncoder};
use ooxmlsdk::schemas::schemas_microsoft_com_office_drawing_2008_diagram as dsp;
use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_diagram as dgm;
use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;
use ooxmlsdk::units::DrawingmlPercentageValue;

use super::drawingml::color::Color;
use super::drawingml::fill::{FillKind, FillProperties};
use super::drawingml::line::{LineFill, LineProperties};
use super::drawingml::shape::{
  FontStyleReference, GraphicDataKind, GraphicDataRecord, Shape, ShapeService,
};
use super::drawingml::table::{
  TableCell, TableCellBorders, TableProperties, TableStyle, TableStyleBorders, TableStylePart,
  TableStyleTextProperties, predefined_table_style,
};
use super::drawingml::text_body::{
  TextAutoFit, TextBody, TextBodyDisplayProperties, TextParagraph, TextRun, TextRunKind,
  has_noninherited_body_properties,
};
use super::drawingml::text_list_style::{
  TextListLevelParagraphProperties, TextListParagraphStyle, TextListStyle,
};
use super::import::{PowerPointImport, ThemeFragmentRecord};
use super::slide::{
  BackgroundKind, BackgroundProperties, ChartResource, ImageResource, SlidePersist,
};

const DEFAULT_TEXT_FONT_SIZE_PT: f32 = 18.0;
const DEFAULT_TEXT_LINE_HEIGHT_SCALE: f32 = 1.2;
const DEFAULT_TEXT_INSET_EMU: i64 = 91_440;
const DEFAULT_TABLE_BORDER_PT: f32 = 0.75;

pub(crate) fn lower_to_layout_document(import: &PowerPointImport) -> layout::LayoutDocument {
  let pages = import
    .draw_pages
    .iter()
    .map(|slide| (slide.size.to_page_setup(), lower_slide_items(import, slide)))
    .collect();
  layout::fixed_pages_with_items(pages)
}

fn lower_slide_items(import: &PowerPointImport, slide: &SlidePersist) -> Vec<PageItem> {
  let mut items = Vec::new();
  let _has_structured_comment_identity = slide.comments.iter().any(|comment| comment.has_payload())
    || slide
      .comment_authors
      .iter()
      .any(|author| author.has_payload());
  let _has_header_footer_identity = slide.header_footer.has_visible_slot();
  let master_page = slide
    .master_page_index
    .and_then(|master_page_index| import.master_pages.get(master_page_index));
  if let Some(background) = slide
    .background_properties
    .as_ref()
    .or_else(|| master_page.and_then(|master_page| master_page.background_properties.as_ref()))
  {
    lower_background(import, slide, background, &mut items);
  }
  lower_shapes(import, slide, &slide.shapes, &mut items);
  items
}

fn lower_background(
  import: &PowerPointImport,
  slide: &SlidePersist,
  background: &BackgroundProperties,
  items: &mut Vec<PageItem>,
) {
  let fill_properties = match &background.kind {
    BackgroundKind::Properties(fill_properties) => Some(
      fill_properties
        .clone()
        .with_placeholder_color(slide.background_color.clone()),
    ),
    BackgroundKind::StyleReference {
      style_index,
      placeholder_color,
    } => import.get_theme_fill_style(*style_index).map(|fill| {
      fill.with_placeholder_color(
        placeholder_color
          .clone()
          .or_else(|| slide.background_color.clone()),
      )
    }),
  };
  let Some(fill_properties) = fill_properties else {
    return;
  };
  if let Some(fill_paint) = background_fill_paint(import, slide, &fill_properties) {
    if is_default_white_page_background(fill_paint) {
      return;
    }
    items.push(PageItem::Rect(RectItem {
      x_pt: 0.0,
      y_pt: 0.0,
      width_pt: slide.size.width_pt,
      height_pt: slide.size.height_pt,
      fill_color: Some(fill_paint.color),
      fill_opacity: fill_paint.opacity,
      stroke: None,
      stroke_opacity: 1.0,
    }));
  } else {
    items.extend(
      blip_fill_image_items(
        import,
        slide,
        &fill_properties,
        0.0,
        0.0,
        slide.size.width_pt,
        slide.size.height_pt,
        0.0,
        false,
        false,
        false,
        None,
        None,
      )
      .into_iter()
      .map(PageItem::Image),
    );
  }
}

fn is_default_white_page_background(paint: DisplayPaint) -> bool {
  paint.opacity >= 1.0 && paint.color.r == 255 && paint.color.g == 255 && paint.color.b == 255
}

fn background_fill_paint(
  import: &PowerPointImport,
  slide: &SlidePersist,
  fill: &FillProperties,
) -> Option<DisplayPaint> {
  match &fill.kind {
    FillKind::Solid(color) => color.as_ref().and_then(|color| {
      display_paint_for_slide(import, slide, color, fill.placeholder_color.as_ref())
    }),
    FillKind::None
    | FillKind::Group
    | FillKind::Gradient(_)
    | FillKind::Blip(_)
    | FillKind::Pattern(_) => None,
  }
}

fn lower_shapes(
  import: &PowerPointImport,
  slide: &SlidePersist,
  shapes: &[Shape],
  items: &mut Vec<PageItem>,
) {
  for shape in shapes {
    lower_shape(import, slide, shape, DisplayOffset::default(), items);
  }
}

#[derive(Clone, Copy, Debug, Default)]
struct DisplayOffset {
  x_emu: i64,
  y_emu: i64,
}

fn lower_shape(
  import: &PowerPointImport,
  slide: &SlidePersist,
  shape: &Shape,
  offset: DisplayOffset,
  items: &mut Vec<PageItem>,
) {
  if shape.hidden
    || shape.hidden_master_shape
    || shape.referenced
    || is_uninstantiated_placeholder(shape)
  {
    return;
  }

  lower_shape_bounds(import, slide, shape, offset, items);
  lower_picture(import, slide, shape, offset, items);
  lower_shape_hyperlink(shape, offset, items);
  let _has_structured_media_identity = shape.media.as_ref().is_some_and(|media| {
    !matches!(media.kind, super::drawingml::shape::MediaKind::Unknown)
      || media.embed_relationship_id.is_some()
      || media.link_relationship_id.is_some()
      || media
        .resource
        .as_ref()
        .is_some_and(|resource| resource.has_payload())
  });
  let _has_structured_content_part_identity =
    shape.content_part.as_ref().is_some_and(|content_part| {
      !content_part.relationship_id.is_empty()
        || content_part
          .resource
          .as_ref()
          .is_some_and(|resource| resource.has_payload())
    });
  let _has_structured_graphic_identity = shape
    .graphic_data
    .as_ref()
    .is_some_and(graphic_data_has_structured_identity);

  if let Some(table) = &shape.table_properties
    && shape.service_name == ShapeService::TableShape
  {
    lower_table(import, shape, offset, table, items);
  }

  if shape.service_name == ShapeService::ChartShape
    && let Some(record) = &shape.graphic_data
  {
    lower_chart(import, slide, shape, offset, record, items);
  }

  if shape.frame_type == super::drawingml::shape::FrameType::Diagram
    && let Some(record) = &shape.graphic_data
  {
    lower_diagram(import, slide, shape, offset, record, items);
  }

  if let Some(text_body) = &shape.text_body {
    lower_text_body(import, shape, offset, text_body, items);
  }

  let child_offset = child_display_offset(shape, offset);
  for child in &shape.children {
    lower_shape(import, slide, child, child_offset, items);
  }
}

fn is_uninstantiated_placeholder(shape: &Shape) -> bool {
  shape.sub_type.is_some()
    && shape
      .shape_location
      .is_some_and(|location| location != super::slide::ShapeLocation::Slide)
}

fn graphic_data_has_structured_identity(record: &GraphicDataRecord) -> bool {
  !record.uri.is_empty()
    || !matches!(record.kind, GraphicDataKind::Unsupported)
    || record.chart_relationship_id.is_some()
    || record
      .chart_resource
      .as_ref()
      .is_some_and(|resource| resource.has_payload())
    || record
      .extended_chart_resource
      .as_ref()
      .is_some_and(|resource| resource.has_payload())
    || record.has_inline_chart_space
    || record.diagram_relationship_ids.as_ref().is_some_and(|ids| {
      !ids.data_part.is_empty()
        || !ids.layout_part.is_empty()
        || !ids.style_part.is_empty()
        || !ids.color_part.is_empty()
    })
    || record
      .diagram_data_resource
      .as_ref()
      .is_some_and(|resource| resource.has_payload())
    || record
      .diagram_layout_resource
      .as_ref()
      .is_some_and(|resource| resource.has_payload())
    || record
      .diagram_style_resource
      .as_ref()
      .is_some_and(|resource| resource.has_payload())
    || record
      .diagram_color_resource
      .as_ref()
      .is_some_and(|resource| resource.has_payload())
    || record.ole_object.as_ref().is_some_and(|ole| {
      ole.relationship_id.is_some()
        || ole.name.is_some()
        || ole.prog_id.is_some()
        || ole.show_as_icon
    })
    || record
      .ole_binary_resource
      .as_ref()
      .is_some_and(|resource| resource.has_payload())
    || record
      .embedded_package_resource
      .as_ref()
      .is_some_and(|resource| resource.has_payload())
}

fn lower_chart(
  import: &PowerPointImport,
  slide: &SlidePersist,
  shape: &Shape,
  offset: DisplayOffset,
  record: &GraphicDataRecord,
  items: &mut Vec<PageItem>,
) {
  if shape.size.cx <= 0 || shape.size.cy <= 0 {
    return;
  }
  let Some(chart_resource) = &record.chart_resource else {
    return;
  };
  let paints = chart_data_point_paints(import, slide, chart_resource);
  let texts = shared_chart::visible_texts(&chart_resource.chart_space);
  if paints.is_empty() && texts.is_empty() {
    return;
  }

  let x = units::emu_to_points(offset.x_emu + shape.position.x);
  let y = units::emu_to_points(offset.y_emu + shape.position.y);
  let width = units::emu_to_points(shape.size.cx);
  let height = units::emu_to_points(shape.size.cy);
  let plot_x = x + width * 0.12;
  let plot_y = y + height * 0.12;
  let plot_width = width * 0.76;
  let plot_height = height * 0.76;
  if !paints.is_empty() {
    let point_count = paints.len().max(1) as f32;
    let gap = (plot_width * 0.02).min(6.0);
    let item_width = ((plot_width - gap * (point_count - 1.0)) / point_count).max(1.0);

    // Source: LibreOffice sd/qa/unit/import-tests4.cxx:testTdf153012
    // Chart data point fills resolve against c:clrMapOvr, so bg1 maps through
    // the chart color map instead of the slide p:clrMapOvr.
    for (index, paint) in paints.iter().enumerate() {
      items.push(PageItem::Rect(RectItem {
        x_pt: plot_x + index as f32 * (item_width + gap),
        y_pt: plot_y,
        width_pt: item_width,
        height_pt: plot_height,
        fill_color: Some(paint.color),
        fill_opacity: paint.opacity,
        stroke: None,
        stroke_opacity: 1.0,
      }));
    }
  }

  lower_chart_texts(x, y, width, height, texts, items);
}

fn lower_chart_texts(
  x: f32,
  y: f32,
  width: f32,
  height: f32,
  texts: Vec<String>,
  items: &mut Vec<PageItem>,
) {
  let mut style = TextStyle::default();
  style.font_size_pt = 11.0;
  let line_step = 13.2;
  let start_x = x + width * 0.12;
  let mut text_y = y + height * 0.12;
  let max_y = y + height * 0.9;

  for text in texts {
    if text_y > max_y {
      break;
    }
    push_text_item(items, start_x, text_y, text, style.clone(), None, 1.0);
    text_y += line_step;
  }
}

fn chart_data_point_paints(
  import: &PowerPointImport,
  slide: &SlidePersist,
  chart_resource: &ChartResource,
) -> Vec<DisplayPaint> {
  shared_chart::data_point_solid_fills(&chart_resource.chart_space)
    .into_iter()
    .filter_map(|fill| display_paint_for_chart(import, slide, chart_resource, fill.fill))
    .collect()
}

fn display_paint_for_chart(
  import: &PowerPointImport,
  slide: &SlidePersist,
  chart_resource: &ChartResource,
  fill: &a::SolidFill,
) -> Option<DisplayPaint> {
  let theme = chart_theme(import, slide)?;
  let color_map = chart_resource.chart_space.color_map_override.as_deref();
  let color = fill
    .solid_fill_choice
    .as_ref()
    .and_then(Color::from_solid_fill_choice)?;
  let mut scheme_resolver = |token| {
    let mapped = shared_chart::scheme_color_token(color_map, token)?;
    theme.color_scheme.get_color(mapped).cloned()
  };
  let color = color.resolve_rgb(&mut scheme_resolver, None)?;
  Some(DisplayPaint {
    color: RgbColor {
      r: color.r,
      g: color.g,
      b: color.b,
    },
    opacity: color_opacity(color.alpha),
  })
}

fn chart_theme<'a>(
  import: &'a PowerPointImport,
  slide: &SlidePersist,
) -> Option<&'a ThemeFragmentRecord> {
  slide
    .theme_path
    .as_deref()
    .and_then(|path| import.get_theme(path))
    .or_else(|| import.get_current_theme_ptr())
}

fn lower_diagram(
  import: &PowerPointImport,
  slide: &SlidePersist,
  shape: &Shape,
  offset: DisplayOffset,
  record: &GraphicDataRecord,
  items: &mut Vec<PageItem>,
) {
  let Some(data_resource) = record.diagram_data_resource.as_ref() else {
    return;
  };
  let x_pt = units::emu_to_points(offset.x_emu + shape.position.x);
  let y_pt = units::emu_to_points(offset.y_emu + shape.position.y);
  let width_pt = units::emu_to_points(shape.size.cx);
  let height_pt = units::emu_to_points(shape.size.cy);
  if let Some(background_fill) = diagram_background_fill(import, slide, &data_resource.model) {
    items.push(PageItem::Rect(RectItem {
      x_pt,
      y_pt,
      width_pt,
      height_pt,
      fill_color: Some(background_fill),
      fill_opacity: 1.0,
      stroke: None,
      stroke_opacity: 1.0,
    }));
  }
  if let Some(drawing) = diagram_drawing_resource(slide, &data_resource.model)
    && lower_diagram_drawing(
      import,
      slide,
      drawing,
      &data_resource.model,
      x_pt,
      y_pt,
      width_pt,
      height_pt,
      items,
    )
  {
    return;
  }
  let fill = diagram_accent_fill(import, slide);
  let styles = diagram_styles(record);
  let colors = diagram_style_colors(import, slide, record);
  let shapes = shared_diagram::layout_shapes(
    &data_resource.model,
    record
      .diagram_layout_resource
      .as_ref()
      .map(|resource| &resource.layout),
    styles.as_ref(),
    colors.as_ref(),
    shared_diagram::DiagramBounds {
      x: x_pt,
      y: y_pt,
      width: width_pt,
      height: height_pt,
    },
    fill,
  );
  let mut drawing_items = Vec::new();
  let mut text_items = Vec::new();
  let mut pending_text_items = Vec::new();
  let mut font_sync_scales: HashMap<String, f32> = HashMap::new();
  for diagram_shape in shapes {
    let fill_images = diagram_shape
      .shape_properties
      .as_deref()
      .map(|properties| {
        diagram_model_shape_blip_fill_image_items(
          import,
          slide,
          data_resource,
          properties,
          shared_diagram::DiagramBounds {
            x: diagram_shape.x,
            y: diagram_shape.y,
            width: diagram_shape.width,
            height: diagram_shape.height,
          },
        )
      })
      .unwrap_or_default();
    if fill_images.is_empty()
      && diagram_shape.is_blip_placeholder
      && let Some(item) = diagram_blip_placeholder_image_item(shared_diagram::DiagramBounds {
        x: diagram_shape.x,
        y: diagram_shape.y,
        width: diagram_shape.width,
        height: diagram_shape.height,
      })
    {
      drawing_items.push(PageItem::Image(item));
    }
    drawing_items.extend(fill_images.into_iter().map(PageItem::Image));
    let fill_color = diagram_shape
      .shape_properties
      .as_deref()
      .and_then(|properties| diagram_model_shape_fill_color(import, slide, properties))
      .unwrap_or(diagram_shape.fill);
    let suppress_fill = diagram_shape
      .shape_properties
      .as_deref()
      .is_some_and(diagram_model_shape_suppresses_fill);
    if diagram_shape.is_connector {
      drawing_items.push(PageItem::Line(diagram_connector_line_item(&diagram_shape)));
    } else {
      drawing_items.push(PageItem::Rect(RectItem {
        x_pt: diagram_shape.x,
        y_pt: diagram_shape.y,
        width_pt: diagram_shape.width,
        height_pt: diagram_shape.height,
        fill_color: diagram_shape
          .shape_properties
          .as_deref()
          .is_none_or(|properties| !diagram_model_shape_has_blip_fill(properties))
          .then_some(fill_color)
          .filter(|_| !suppress_fill),
        fill_opacity: 1.0,
        stroke: diagram_shape
          .shape_properties
          .as_deref()
          .and_then(|properties| diagram_model_shape_outline(import, slide, properties))
          .or_else(|| Some(BorderStyle::default()).filter(|_| !suppress_fill)),
        stroke_opacity: 1.0,
      }));
    }
    if !diagram_shape.text_body.is_empty() {
      let mut text_body = diagram_text_body(&diagram_shape.text_body);
      if diagram_shape.text_rotation_deg != 0.0 {
        let rotation = (diagram_shape.text_rotation_deg * 60_000.0).round() as i32;
        text_body.display_properties.text_area_rotation = Some(
          text_body
            .display_properties
            .text_area_rotation
            .unwrap_or_default()
            + rotation,
        );
      }
      let text_frame = text_body_frame(
        diagram_shape.x,
        diagram_shape.y,
        diagram_shape.width,
        diagram_shape.height,
        &text_body,
      );
      let font_reference = diagram_shape
        .style
        .as_deref()
        .map(|style| diagram_font_style_reference(&style.font_reference, diagram_shape.text_fill));
      let options = TextLoweringOptions::from_text_body(&text_body);
      let base_style = text_base_style(
        import,
        &text_body,
        font_reference.as_ref(),
        None,
        diagram_shape.font_size_pt,
      );
      let font_scale =
        text_auto_fit_font_scale(import, text_frame, &text_body, &base_style, &options);
      if let Some(group) = diagram_shape.font_sync_group.as_deref() {
        font_sync_scales
          .entry(group.to_string())
          .and_modify(|scale| *scale = scale.min(font_scale))
          .or_insert(font_scale);
      }
      pending_text_items.push(PendingDiagramTextItem {
        order: diagram_shape.order,
        frame: text_frame,
        text_body,
        font_reference,
        base_font_size_pt: diagram_shape.font_size_pt,
        font_sync_group: diagram_shape.font_sync_group,
        font_scale,
      });
    }
  }
  for pending in pending_text_items {
    let mut lowered_text_items = Vec::new();
    let font_scale = pending
      .font_sync_group
      .as_deref()
      .and_then(|group| font_sync_scales.get(group).copied())
      .unwrap_or(pending.font_scale);
    lower_text_body_at_with_style_and_scale(
      import,
      pending.frame,
      &pending.text_body,
      pending.font_reference.as_ref(),
      None,
      None,
      pending.base_font_size_pt,
      Some(font_scale),
      &mut lowered_text_items,
    );
    for mut item in lowered_text_items {
      if let PageItem::Text(text_item) = &mut item {
        text_item.preserve_text_portion = true;
      }
      text_items.push(DiagramDrawingTextItem {
        order: pending.order,
        item,
      });
    }
  }
  text_items.sort_by_key(|text_item| text_item.order);
  items.extend(drawing_items);
  items.extend(text_items.into_iter().map(|text_item| text_item.item));
}

fn diagram_drawing_resource<'a>(
  slide: &'a SlidePersist,
  data: &dgm::DataModelRoot,
) -> Option<&'a super::slide::DiagramDrawingResource> {
  let extensions = data.data_model_extension_list.as_ref()?;
  for extension in &extensions.data_model_extension {
    if let Some(a::DataModelExtensionChoice::DataModelExtensionBlock(block)) =
      extension.data_model_extension_choice.as_ref()
      && let Some(rel_id) = block.rel_id.as_deref()
      && let Some(resource) = slide.diagram_drawing_resources.get(rel_id)
    {
      return Some(resource);
    }
  }
  None
}

fn lower_diagram_drawing(
  import: &PowerPointImport,
  slide: &SlidePersist,
  drawing_resource: &super::slide::DiagramDrawingResource,
  data: &dgm::DataModelRoot,
  x_pt: f32,
  y_pt: f32,
  width_pt: f32,
  height_pt: f32,
  items: &mut Vec<PageItem>,
) -> bool {
  // Source: LibreOffice oox/source/drawingml/diagram/diagram.cxx loadDiagram()
  // imports persisted diagramDrawing extDrawing before falling back to layout
  // atom shape generation.
  let Some(extent) = diagram_drawing_extent(&drawing_resource.drawing.shape_tree) else {
    return false;
  };
  let scale_x = if extent.width > 0 {
    width_pt / units::emu_to_points(extent.width)
  } else {
    1.0
  };
  let scale_y = if extent.height > 0 {
    height_pt / units::emu_to_points(extent.height)
  } else {
    1.0
  };
  let text_orders = shared_diagram::presentation_point_list_orders(data);
  let mut drawing_items = Vec::new();
  let mut text_items = Vec::new();
  for choice in &drawing_resource.drawing.shape_tree.shape_tree_choice {
    match choice {
      dsp::ShapeTreeChoice::Shape(shape) => lower_diagram_drawing_shape(
        import,
        slide,
        drawing_resource,
        shape,
        &text_orders,
        extent.x,
        extent.y,
        x_pt,
        y_pt,
        height_pt,
        scale_x,
        scale_y,
        &mut drawing_items,
        &mut text_items,
      ),
      dsp::ShapeTreeChoice::GroupShape(group) => lower_diagram_drawing_group(
        import,
        slide,
        drawing_resource,
        group,
        &text_orders,
        extent.x,
        extent.y,
        x_pt,
        y_pt,
        height_pt,
        scale_x,
        scale_y,
        &mut drawing_items,
        &mut text_items,
      ),
    }
  }
  if drawing_items.is_empty() && text_items.is_empty() {
    return false;
  }
  text_items.sort_by_key(|text_item| text_item.order);
  items.extend(drawing_items);
  items.extend(text_items.into_iter().map(|text_item| text_item.item));
  true
}

fn lower_diagram_drawing_group(
  import: &PowerPointImport,
  slide: &SlidePersist,
  drawing_resource: &super::slide::DiagramDrawingResource,
  group: &dsp::GroupShape,
  text_orders: &HashMap<String, usize>,
  origin_x: i64,
  origin_y: i64,
  x_pt: f32,
  y_pt: f32,
  container_height_pt: f32,
  scale_x: f32,
  scale_y: f32,
  items: &mut Vec<PageItem>,
  text_items: &mut Vec<DiagramDrawingTextItem>,
) {
  for choice in &group.group_shape_choice {
    match choice {
      dsp::GroupShapeChoice::Shape(shape) => lower_diagram_drawing_shape(
        import,
        slide,
        drawing_resource,
        shape,
        text_orders,
        origin_x,
        origin_y,
        x_pt,
        y_pt,
        container_height_pt,
        scale_x,
        scale_y,
        items,
        text_items,
      ),
      dsp::GroupShapeChoice::GroupShape(group) => lower_diagram_drawing_group(
        import,
        slide,
        drawing_resource,
        group,
        text_orders,
        origin_x,
        origin_y,
        x_pt,
        y_pt,
        container_height_pt,
        scale_x,
        scale_y,
        items,
        text_items,
      ),
    }
  }
}

fn lower_diagram_drawing_shape(
  import: &PowerPointImport,
  slide: &SlidePersist,
  drawing_resource: &super::slide::DiagramDrawingResource,
  shape: &dsp::Shape,
  text_orders: &HashMap<String, usize>,
  origin_x: i64,
  origin_y: i64,
  x_pt: f32,
  y_pt: f32,
  container_height_pt: f32,
  scale_x: f32,
  scale_y: f32,
  items: &mut Vec<PageItem>,
  text_items: &mut Vec<DiagramDrawingTextItem>,
) {
  let Some(bounds) = diagram_shape_bounds(
    &shape.shape_properties,
    origin_x,
    origin_y,
    x_pt,
    y_pt,
    container_height_pt,
    scale_x,
    scale_y,
  ) else {
    return;
  };
  let fill_color =
    diagram_shape_fill_color(import, slide, &shape.shape_properties).unwrap_or(RgbColor {
      r: 255,
      g: 255,
      b: 255,
    });
  let fill_images = diagram_shape_blip_fill_image_items(
    import,
    slide,
    drawing_resource,
    &shape.shape_properties,
    bounds,
  );
  items.extend(fill_images.into_iter().map(PageItem::Image));
  items.push(PageItem::Rect(RectItem {
    x_pt: bounds.x,
    y_pt: bounds.y,
    width_pt: bounds.width,
    height_pt: bounds.height,
    fill_color: (!diagram_shape_has_blip_fill(&shape.shape_properties)).then_some(fill_color),
    fill_opacity: 1.0,
    stroke: Some(BorderStyle::default()),
    stroke_opacity: 1.0,
  }));
  let Some(text_body) = shape.text_body.as_deref() else {
    return;
  };
  let text_body = TextBody::from_diagram_drawing(text_body);
  if text_body
    .paragraphs
    .iter()
    .flat_map(|paragraph| &paragraph.runs)
    .all(|run| run.text.trim().is_empty())
  {
    return;
  }
  let text_bounds = shape
    .transform2_d
    .as_deref()
    .and_then(|transform| {
      diagram_text_transform_bounds(
        transform,
        origin_x,
        origin_y,
        x_pt,
        y_pt,
        container_height_pt,
        scale_x,
        scale_y,
      )
    })
    .unwrap_or(bounds);
  let mut lowered_text_items = Vec::new();
  lower_text_body_at_with_style(
    import,
    TextFrame {
      x_pt: text_bounds.x + 4.0,
      y_pt: text_bounds.y,
      width_pt: (text_bounds.width - 8.0).max(1.0),
      height_pt: text_bounds.height,
    },
    &text_body,
    None,
    None,
    None,
    None,
    &mut lowered_text_items,
  );
  for item in &mut lowered_text_items {
    if let PageItem::Text(text) = item {
      text.preserve_text_portion = true;
    }
  }
  let order = text_orders
    .get(shape.model_id.as_str())
    .copied()
    .unwrap_or(usize::MAX);
  text_items.extend(
    lowered_text_items
      .into_iter()
      .map(|item| DiagramDrawingTextItem { order, item }),
  );
}

struct DiagramDrawingTextItem {
  order: usize,
  item: PageItem,
}

struct PendingDiagramTextItem {
  order: usize,
  frame: TextFrame,
  text_body: TextBody,
  font_reference: Option<FontStyleReference>,
  base_font_size_pt: Option<f32>,
  font_sync_group: Option<String>,
  font_scale: f32,
}

fn diagram_connector_line_item(diagram_shape: &shared_diagram::DiagramShape) -> LineItem {
  let center_x = diagram_shape.x + diagram_shape.width / 2.0;
  let center_y = diagram_shape.y + diagram_shape.height / 2.0;
  let length = diagram_shape.width.max(diagram_shape.height).max(1.0);
  let radians = diagram_shape.connector_angle_deg.to_radians();
  let dx = radians.cos() * length / 2.0;
  let dy = radians.sin() * length / 2.0;
  LineItem {
    x1_pt: center_x - dx,
    y1_pt: center_y - dy,
    x2_pt: center_x + dx,
    y2_pt: center_y + dy,
    width_pt: 1.0,
    color: RgbColor { r: 0, g: 0, b: 0 },
    kind: LineItemKind::Stroke,
  }
}

fn diagram_blip_placeholder_image_item(bounds: shared_diagram::DiagramBounds) -> Option<ImageItem> {
  if bounds.width <= f32::EPSILON || bounds.height <= f32::EPSILON {
    return None;
  }
  Some(ImageItem {
    x_pt: bounds.x,
    y_pt: bounds.y,
    width_pt: bounds.width,
    height_pt: bounds.height,
    crop: ImageCrop::default(),
    rotation_deg: 0.0,
    flip_horizontal: false,
    flip_vertical: false,
    data: transparent_png_1x1()?,
    content_type: Some("image/png".to_string()),
    alt_text: None,
    hyperlink_url: None,
    floating: false,
    behind_text: false,
  })
}

fn diagram_text_body(source: &shared_diagram::DiagramTextBody) -> TextBody {
  let mut display_properties = source
    .body_properties
    .as_deref()
    .map(TextBodyDisplayProperties::from_body_properties)
    .unwrap_or_default();
  if source.auto_fit {
    display_properties.auto_fit = TextAutoFit::Shape;
  }
  TextBody {
    has_body_properties: source.body_properties.is_some(),
    has_noninherited_body_properties: source
      .body_properties
      .as_deref()
      .is_some_and(has_noninherited_body_properties),
    body_properties: source.body_properties.clone(),
    display_properties,
    has_list_style: source.list_style.is_some(),
    list_style: source
      .list_style
      .as_deref()
      .map(TextListStyle::from_dml_list_style),
    paragraphs: source
      .paragraphs
      .iter()
      .map(|paragraph| TextParagraph {
        level: paragraph.level,
        paragraph_properties: paragraph.paragraph_properties.clone(),
        end_paragraph_run_properties: paragraph.end_paragraph_run_properties.clone(),
        master_paragraph_style: None,
        text_paragraph_style: None,
        runs: paragraph
          .runs
          .iter()
          .map(|run| TextRun {
            text: run.text.clone(),
            kind: match run.kind {
              shared_diagram::DiagramTextRunKind::Run => TextRunKind::Run,
              shared_diagram::DiagramTextRunKind::Break => TextRunKind::Break,
              shared_diagram::DiagramTextRunKind::Field => TextRunKind::Field,
              shared_diagram::DiagramTextRunKind::Math => TextRunKind::Math,
            },
            hyperlink_url: None,
            field_type: run.field_type.clone(),
            run_properties: run.run_properties.clone(),
            field_paragraph_properties: run.field_paragraph_properties.clone(),
          })
          .collect(),
      })
      .collect(),
  }
}

fn diagram_model_shape_fill_color(
  import: &PowerPointImport,
  slide: &SlidePersist,
  properties: &dgm::ShapeProperties,
) -> Option<RgbColor> {
  let fill = match properties.shape_properties_choice2.as_ref()? {
    dgm::ShapePropertiesChoice2::SolidFill(fill) => fill,
    _ => return None,
  };
  let color = Color::from_solid_fill_choice(fill.solid_fill_choice.as_ref()?)?;
  let resolved = import.resolve_color_for_slide(slide, &color, None)?;
  Some(RgbColor {
    r: resolved.r,
    g: resolved.g,
    b: resolved.b,
  })
}

fn diagram_model_shape_has_blip_fill(properties: &dgm::ShapeProperties) -> bool {
  matches!(
    properties.shape_properties_choice2.as_ref(),
    Some(dgm::ShapePropertiesChoice2::BlipFill(_))
  )
}

fn diagram_model_shape_suppresses_fill(properties: &dgm::ShapeProperties) -> bool {
  matches!(
    properties.shape_properties_choice2.as_ref(),
    Some(dgm::ShapePropertiesChoice2::NoFill(_) | dgm::ShapePropertiesChoice2::BlipFill(_))
  )
}

fn diagram_model_shape_outline(
  import: &PowerPointImport,
  slide: &SlidePersist,
  properties: &dgm::ShapeProperties,
) -> Option<BorderStyle> {
  let outline = properties.outline.as_deref()?;
  let color = match outline.outline_choice1.as_ref()? {
    a::OutlineChoice::NoFill(_) => return None,
    a::OutlineChoice::SolidFill(fill) => {
      let color = Color::from_solid_fill_choice(fill.solid_fill_choice.as_ref()?)?;
      import.resolve_color_for_slide(slide, &color, None)?
    }
    _ => return None,
  };
  Some(BorderStyle {
    width_pt: outline
      .width
      .map(|width| units::emu_to_points(i64::from(width)))
      .unwrap_or(0.5),
    spacing_pt: 0.0,
    color: RgbColor {
      r: color.r,
      g: color.g,
      b: color.b,
    },
    compound: false,
  })
}

fn diagram_model_shape_blip_fill_image_items(
  import: &PowerPointImport,
  slide: &SlidePersist,
  data_resource: &super::slide::DiagramDataResource,
  properties: &dgm::ShapeProperties,
  bounds: shared_diagram::DiagramBounds,
) -> Vec<ImageItem> {
  let Some(dgm::ShapePropertiesChoice2::BlipFill(blip_fill)) =
    properties.shape_properties_choice2.as_ref()
  else {
    return Vec::new();
  };
  let Some(blip) = blip_fill.blip.as_ref() else {
    return Vec::new();
  };
  let Some(relationship_id) = blip.embed.as_deref() else {
    return Vec::new();
  };
  let Some(resource) = data_resource.image_resources.get(relationship_id) else {
    return Vec::new();
  };
  let rotation_deg = properties
    .transform2_d
    .as_deref()
    .and_then(|transform| transform.rotation)
    .map(|rotation| rotation as f32 / 60000.0)
    .unwrap_or_default();
  let flip_horizontal = properties
    .transform2_d
    .as_deref()
    .and_then(|transform| transform.horizontal_flip)
    .map(bool::from)
    .unwrap_or(false);
  let flip_vertical = properties
    .transform2_d
    .as_deref()
    .and_then(|transform| transform.vertical_flip)
    .map(bool::from)
    .unwrap_or(false);
  blip_fill_image_items_from_resource(
    import,
    slide,
    blip_fill,
    blip,
    resource,
    bounds.x,
    bounds.y,
    bounds.width,
    bounds.height,
    rotation_deg,
    flip_horizontal,
    flip_vertical,
    false,
    None,
    None,
  )
}

#[derive(Clone, Copy, Debug)]
struct DiagramDrawingExtent {
  x: i64,
  y: i64,
  width: i64,
  height: i64,
}

fn diagram_drawing_extent(tree: &dsp::ShapeTree) -> Option<DiagramDrawingExtent> {
  let mut extent: Option<DiagramDrawingExtent> = None;
  for choice in &tree.shape_tree_choice {
    match choice {
      dsp::ShapeTreeChoice::Shape(shape) => merge_diagram_shape_extent(&mut extent, shape),
      dsp::ShapeTreeChoice::GroupShape(group) => merge_diagram_group_extent(&mut extent, group),
    }
  }
  extent
}

fn merge_diagram_group_extent(extent: &mut Option<DiagramDrawingExtent>, group: &dsp::GroupShape) {
  for choice in &group.group_shape_choice {
    match choice {
      dsp::GroupShapeChoice::Shape(shape) => merge_diagram_shape_extent(extent, shape),
      dsp::GroupShapeChoice::GroupShape(group) => merge_diagram_group_extent(extent, group),
    }
  }
}

fn merge_diagram_shape_extent(extent: &mut Option<DiagramDrawingExtent>, shape: &dsp::Shape) {
  let Some(transform) = shape.shape_properties.transform2_d.as_deref() else {
    return;
  };
  let Some(offset) = transform.offset.as_ref() else {
    return;
  };
  let Some(extents) = transform.extents.as_ref() else {
    return;
  };
  let x = offset.x.to_emu();
  let y = offset.y.to_emu();
  let width = extents.cx.to_emu();
  let height = extents.cy.to_emu();
  let right = x + width;
  let bottom = y + height;
  *extent = Some(match *extent {
    Some(current) => {
      let left = current.x.min(x);
      let top = current.y.min(y);
      DiagramDrawingExtent {
        x: left,
        y: top,
        width: (current.x + current.width).max(right) - left,
        height: (current.y + current.height).max(bottom) - top,
      }
    }
    None => DiagramDrawingExtent {
      x,
      y,
      width,
      height,
    },
  });
}

fn diagram_shape_bounds(
  properties: &dsp::ShapeProperties,
  origin_x: i64,
  origin_y: i64,
  x_pt: f32,
  y_pt: f32,
  container_height_pt: f32,
  scale_x: f32,
  scale_y: f32,
) -> Option<shared_diagram::DiagramBounds> {
  let transform = properties.transform2_d.as_deref()?;
  let offset = transform.offset.as_ref()?;
  let extents = transform.extents.as_ref()?;
  let x = offset.x.to_emu();
  let y = offset.y.to_emu();
  let width = units::emu_to_points(extents.cx.to_emu()) * scale_x;
  let height = units::emu_to_points(extents.cy.to_emu()) * scale_y;
  let local_y = units::emu_to_points(y - origin_y) * scale_y;
  Some(shared_diagram::DiagramBounds {
    x: x_pt + units::emu_to_points(x - origin_x) * scale_x,
    y: y_pt + container_height_pt - local_y - height,
    width,
    height,
  })
}

fn diagram_text_transform_bounds(
  transform: &dsp::Transform2D,
  origin_x: i64,
  origin_y: i64,
  x_pt: f32,
  y_pt: f32,
  container_height_pt: f32,
  scale_x: f32,
  scale_y: f32,
) -> Option<shared_diagram::DiagramBounds> {
  let offset = transform.offset.as_ref()?;
  let extents = transform.extents.as_ref()?;
  let x = offset.x.to_emu();
  let y = offset.y.to_emu();
  let width = units::emu_to_points(extents.cx.to_emu()) * scale_x;
  let height = units::emu_to_points(extents.cy.to_emu()) * scale_y;
  let local_y = units::emu_to_points(y - origin_y) * scale_y;
  Some(shared_diagram::DiagramBounds {
    x: x_pt + units::emu_to_points(x - origin_x) * scale_x,
    y: y_pt + container_height_pt - local_y - height,
    width,
    height,
  })
}

fn diagram_shape_fill_color(
  import: &PowerPointImport,
  slide: &SlidePersist,
  properties: &dsp::ShapeProperties,
) -> Option<RgbColor> {
  let fill = match properties.shape_properties_choice2.as_ref()? {
    dsp::ShapePropertiesChoice2::SolidFill(fill) => fill,
    _ => return None,
  };
  let color = Color::from_solid_fill_choice(fill.solid_fill_choice.as_ref()?)?;
  let resolved = import.resolve_color_for_slide(slide, &color, None)?;
  Some(RgbColor {
    r: resolved.r,
    g: resolved.g,
    b: resolved.b,
  })
}

fn diagram_shape_has_blip_fill(properties: &dsp::ShapeProperties) -> bool {
  matches!(
    properties.shape_properties_choice2.as_ref(),
    Some(dsp::ShapePropertiesChoice2::BlipFill(_))
  )
}

fn diagram_shape_blip_fill_image_items(
  import: &PowerPointImport,
  slide: &SlidePersist,
  drawing_resource: &super::slide::DiagramDrawingResource,
  properties: &dsp::ShapeProperties,
  bounds: shared_diagram::DiagramBounds,
) -> Vec<ImageItem> {
  let Some(dsp::ShapePropertiesChoice2::BlipFill(blip_fill)) =
    properties.shape_properties_choice2.as_ref()
  else {
    return Vec::new();
  };
  let Some(blip) = blip_fill.blip.as_ref() else {
    return Vec::new();
  };
  let Some(relationship_id) = blip.embed.as_deref() else {
    return Vec::new();
  };
  let Some(resource) = drawing_resource
    .image_resources
    .get(relationship_id)
    .or_else(|| slide.image_resources.get(relationship_id))
  else {
    return Vec::new();
  };
  let rotation_deg = properties
    .transform2_d
    .as_deref()
    .and_then(|transform| transform.rotation)
    .map(|rotation| rotation as f32 / 60000.0)
    .unwrap_or_default();
  let flip_horizontal = properties
    .transform2_d
    .as_deref()
    .and_then(|transform| transform.horizontal_flip)
    .map(bool::from)
    .unwrap_or(false);
  let flip_vertical = properties
    .transform2_d
    .as_deref()
    .and_then(|transform| transform.vertical_flip)
    .map(bool::from)
    .unwrap_or(false);
  blip_fill_image_items_from_resource(
    import,
    slide,
    blip_fill,
    blip,
    resource,
    bounds.x,
    bounds.y,
    bounds.width,
    bounds.height,
    rotation_deg,
    flip_horizontal,
    flip_vertical,
    false,
    None,
    None,
  )
}

fn diagram_background_fill(
  import: &PowerPointImport,
  slide: &SlidePersist,
  data: &dgm::DataModelRoot,
) -> Option<RgbColor> {
  let fill = match data.background.as_deref()?.background_choice1.as_ref()? {
    dgm::BackgroundChoice::SolidFill(fill) => fill,
    _ => return None,
  };
  let color = Color::from_solid_fill_choice(fill.solid_fill_choice.as_ref()?)?;
  let resolved = import.resolve_color_for_slide(slide, &color, None)?;
  Some(RgbColor {
    r: resolved.r,
    g: resolved.g,
    b: resolved.b,
  })
}

fn diagram_styles(record: &GraphicDataRecord) -> Option<shared_diagram::DiagramStyles> {
  let style_resource = record.diagram_style_resource.as_ref()?;
  let style_by_label: HashMap<String, Box<dgm::Style>> = style_resource
    .style
    .style_label
    .iter()
    .filter_map(|label| Some((label.name.clone(), label.style.clone()?)))
    .collect();
  (!style_by_label.is_empty()).then_some(shared_diagram::DiagramStyles { style_by_label })
}

fn diagram_style_colors(
  import: &PowerPointImport,
  slide: &SlidePersist,
  record: &GraphicDataRecord,
) -> Option<shared_diagram::DiagramStyleColors> {
  let color_resource = record.diagram_color_resource.as_ref()?;
  let mut fill_by_label = HashMap::new();
  let mut text_fill_by_label = HashMap::new();
  for label in &color_resource.colors.color_transform_style_label {
    if let Some(fill_list) = label.fill_color_list.as_ref() {
      let fills: Vec<RgbColor> = fill_list
        .fill_color_list_choice
        .iter()
        .filter_map(Color::from_diagram_fill_color_choice)
        .filter_map(|color| import.resolve_color_for_slide(slide, &color, None))
        .map(|color| RgbColor {
          r: color.r,
          g: color.g,
          b: color.b,
        })
        .collect();
      if !fills.is_empty() {
        fill_by_label.insert(label.name.clone(), fills);
      }
    }
    if let Some(text_fill_list) = label.text_fill_color_list.as_ref() {
      let fills: Vec<RgbColor> = text_fill_list
        .text_fill_color_list_choice
        .iter()
        .filter_map(Color::from_diagram_text_fill_color_choice)
        .filter_map(|color| import.resolve_color_for_slide(slide, &color, None))
        .map(|color| RgbColor {
          r: color.r,
          g: color.g,
          b: color.b,
        })
        .collect();
      if !fills.is_empty() {
        text_fill_by_label.insert(label.name.clone(), fills);
      }
    }
  }
  (!fill_by_label.is_empty() || !text_fill_by_label.is_empty()).then_some(
    shared_diagram::DiagramStyleColors {
      fill_by_label,
      text_fill_by_label,
    },
  )
}

fn diagram_font_style_reference(
  reference: &a::FontReference,
  text_fill: Option<RgbColor>,
) -> FontStyleReference {
  FontStyleReference {
    index: reference.index,
    placeholder_color: text_fill
      .map(|color| {
        Color::RgbHex(super::drawingml::color::RgbHexColor {
          value: format!("{:02X}{:02X}{:02X}", color.r, color.g, color.b),
          transformations: Vec::new(),
        })
      })
      .or_else(|| {
        reference
          .font_reference_choice
          .as_ref()
          .and_then(Color::from_font_reference_choice)
      }),
  }
}

fn diagram_accent_fill(import: &PowerPointImport, slide: &SlidePersist) -> RgbColor {
  chart_theme(import, slide)
    .and_then(|theme| {
      theme
        .color_scheme
        .get_color(a::ColorSchemeIndexValues::Accent1)
    })
    .and_then(|color| import.resolve_color_for_slide(slide, color, None))
    .map(|color| RgbColor {
      r: color.r,
      g: color.g,
      b: color.b,
    })
    .unwrap_or(RgbColor {
      r: 0x4f,
      g: 0x81,
      b: 0xbd,
    })
}

fn lower_picture(
  import: &PowerPointImport,
  slide: &SlidePersist,
  shape: &Shape,
  offset: DisplayOffset,
  items: &mut Vec<PageItem>,
) {
  let Some(picture) = &shape.picture else {
    return;
  };
  if shape.size.cx <= 0 || shape.size.cy <= 0 {
    return;
  }
  let _embed_relationship_id = picture.embed_relationship_id.as_deref();
  let _link_relationship_id = picture.link_relationship_id.as_deref();
  let Some(resource) = picture.image_resource.as_ref() else {
    return;
  };
  let image_data = image_data_with_blip_effects(
    import,
    slide,
    &resource.data,
    resource.content_type.as_deref(),
    &picture.blip_choices,
  );
  let custom_geometry = shape.custom_shape_properties.geometry.is_some();
  let (data, content_type, crop, flip_horizontal, flip_vertical) =
    if custom_geometry && (picture.crop != ImageCrop::default() || shape.flip_h || shape.flip_v) {
      transform_image_data_to_png(&image_data.data, picture.crop, shape.flip_h, shape.flip_v)
        .map(|data| {
          (
            data,
            Some("image/png".into()),
            ImageCrop::default(),
            false,
            false,
          )
        })
        .unwrap_or_else(|| {
          (
            image_data.data,
            image_data
              .content_type
              .or_else(|| resource.content_type.clone()),
            picture.crop,
            shape.flip_h,
            shape.flip_v,
          )
        })
    } else {
      (
        image_data.data,
        image_data
          .content_type
          .or_else(|| resource.content_type.clone()),
        picture.crop,
        shape.flip_h,
        shape.flip_v,
      )
    };
  items.push(PageItem::Image(ImageItem {
    x_pt: units::emu_to_points(offset.x_emu + shape.position.x),
    y_pt: units::emu_to_points(offset.y_emu + shape.position.y),
    width_pt: units::emu_to_points(shape.size.cx),
    height_pt: units::emu_to_points(shape.size.cy),
    crop,
    rotation_deg: shape.rotation,
    flip_horizontal,
    flip_vertical,
    data,
    content_type,
    alt_text: shape
      .description
      .clone()
      .or_else(|| shape.title.clone())
      .or_else(|| shape.name.clone()),
    hyperlink_url: shape.hyperlink_url.clone(),
    floating: false,
    behind_text: false,
  }));
}

fn lower_shape_hyperlink(shape: &Shape, offset: DisplayOffset, items: &mut Vec<PageItem>) {
  let Some(hyperlink_url) = &shape.hyperlink_url else {
    return;
  };
  if shape.service_name == ShapeService::GroupShape || shape.size.cx <= 0 || shape.size.cy <= 0 {
    return;
  }
  items.push(PageItem::LinkArea(LinkAreaItem {
    x_pt: units::emu_to_points(offset.x_emu + shape.position.x),
    y_pt: units::emu_to_points(offset.y_emu + shape.position.y),
    width_pt: units::emu_to_points(shape.size.cx),
    height_pt: units::emu_to_points(shape.size.cy),
    hyperlink_url: hyperlink_url.clone(),
  }));
}

fn lower_table(
  import: &PowerPointImport,
  shape: &Shape,
  offset: DisplayOffset,
  table: &TableProperties,
  items: &mut Vec<PageItem>,
) {
  // Source: LibreOffice oox/source/drawingml/shape.cxx uses the DrawingML
  // table grid and row heights as the visible TableShape size.
  let x0 = units::emu_to_points(offset.x_emu + shape.position.x);
  let y0 = units::emu_to_points(offset.y_emu + shape.position.y);
  let table_width = units::emu_to_points(table.grid.iter().copied().sum::<i64>());
  let row_height_sum = table.rows.iter().map(|row| row.height).sum::<i64>();
  let table_height = units::emu_to_points(row_height_sum.max(shape.size.cy));
  if table_width <= 0.0 || table_height <= 0.0 {
    return;
  }

  let package_table_style = table
    .inline_style
    .as_ref()
    .or_else(|| import.get_table_style(table.style_id.as_deref()));
  let predefined_table_style = if package_table_style.is_none() {
    predefined_table_style(table.style_id.as_deref())
  } else {
    None
  };
  let table_style = package_table_style.or(predefined_table_style.as_ref());
  let table_background = table_style.and_then(|style| {
    table_style_part_fill(import, &style.table_background)
      .and_then(|fill| fill_paint(import, &fill))
  });
  let border_color = RgbColor { r: 0, g: 0, b: 0 };
  let draw_fallback_grid = table_style.is_none() && !table_has_visible_direct_borders(table);
  if draw_fallback_grid {
    push_table_line(items, x0, y0, x0 + table_width, y0, border_color);
    push_table_line(items, x0, y0, x0, y0 + table_height, border_color);
  } else if let Some(style) = table_style
    && !table_has_visible_direct_borders(table)
  {
    lower_table_style_outer_borders(import, style, x0, y0, table_width, table_height, items);
  }

  let mut y = y0;
  let max_row = table.rows.len().saturating_sub(1);
  let max_column = table.grid.len().saturating_sub(1);
  for (row_index, row) in table.rows.iter().enumerate() {
    let row_height = table_row_display_height(row.height, row_height_sum, shape.size.cy);
    let mut x = x0;
    let mut grid_index = 0usize;
    for cell in &row.cells {
      let span = table_cell_grid_span(cell);
      let width_emu = if grid_index < table.grid.len() {
        table.grid[grid_index..table.grid.len().min(grid_index + span)]
          .iter()
          .copied()
          .sum::<i64>()
      } else {
        0
      };
      let cell_width = units::emu_to_points(width_emu);
      if !cell.horizontal_merge && !cell.vertical_merge {
        let style_part = table_style.map(|style| {
          table_cell_style_part(
            import, table, style, grid_index, max_column, row_index, max_row,
          )
        });
        lower_table_cell(
          import,
          cell,
          style_part.as_ref(),
          table_background,
          x,
          y,
          cell_width,
          row_height,
          items,
        );
      }
      x += cell_width;
      grid_index = grid_index.saturating_add(span);
    }
    y += row_height;
    if draw_fallback_grid {
      push_table_line(items, x0, y, x0 + table_width, y, border_color);
    }
  }

  if draw_fallback_grid {
    let mut x = x0;
    for width in &table.grid {
      x += units::emu_to_points(*width);
      push_table_line(items, x, y0, x, y0 + table_height, border_color);
    }
  }
}

fn lower_table_style_outer_borders(
  import: &PowerPointImport,
  style: &TableStyle,
  x: f32,
  y: f32,
  width: f32,
  height: f32,
  items: &mut Vec<PageItem>,
) {
  let borders = &style.whole_table.borders;
  push_table_border_line(
    import,
    &table_style_border_line(import, &borders.top, &borders.top_reference),
    x,
    y,
    x + width,
    y,
    items,
  );
  push_table_border_line(
    import,
    &table_style_border_line(import, &borders.bottom, &borders.bottom_reference),
    x,
    y + height,
    x + width,
    y + height,
    items,
  );
  push_table_border_line(
    import,
    &table_style_border_line(import, &borders.left, &borders.left_reference),
    x,
    y,
    x,
    y + height,
    items,
  );
  push_table_border_line(
    import,
    &table_style_border_line(import, &borders.right, &borders.right_reference),
    x + width,
    y,
    x + width,
    y + height,
    items,
  );
}

fn table_row_display_height(row_height: i64, row_height_sum: i64, shape_height: i64) -> f32 {
  let row_height = units::emu_to_points(row_height);
  if row_height_sum <= 0 || shape_height <= row_height_sum {
    return row_height;
  }
  row_height * shape_height as f32 / row_height_sum as f32
}

fn table_cell_style_part(
  import: &PowerPointImport,
  table: &TableProperties,
  style: &TableStyle,
  column: usize,
  max_column: usize,
  row: usize,
  max_row: usize,
) -> TableStylePart {
  // Source: LibreOffice tablecell.cxx applies table style parts in a fixed
  // order: whole table, first/last row/column, horizontal banding, corners,
  // then vertical banding. Direct tcPr is merged afterwards by the caller.
  let mut result = TableStylePart::default();
  merge_style_part(
    import,
    &mut result,
    &style.whole_table,
    true,
    column,
    max_column,
    row,
    max_row,
  );
  if table.first_row && row == 0 {
    merge_style_part(
      import,
      &mut result,
      &style.first_row,
      false,
      column,
      max_column,
      row,
      max_row,
    );
  }
  if table.last_row && row == max_row {
    merge_style_part(
      import,
      &mut result,
      &style.last_row,
      false,
      column,
      max_column,
      row,
      max_row,
    );
  }
  if table.first_column && column == 0 {
    merge_style_part(
      import,
      &mut result,
      &style.first_column,
      false,
      column,
      max_column,
      row,
      max_row,
    );
  }
  if table.last_column && column == max_column {
    merge_style_part(
      import,
      &mut result,
      &style.last_column,
      false,
      column,
      max_column,
      row,
      max_row,
    );
  }
  if table.band_row
    && (!table.first_row || row != 0)
    && (!table.last_row || row != max_row)
    && (!table.first_column || column != 0 || !table_style_part_has_fill(&style.first_column))
    && (!table.last_column
      || column != max_column
      || !table_style_part_has_fill(&style.last_column))
  {
    let band = row + usize::from(table.first_row);
    let part = if band & 1 == 1 {
      &style.band2_horizontal
    } else {
      &style.band1_horizontal
    };
    merge_style_part(
      import,
      &mut result,
      part,
      false,
      column,
      max_column,
      row,
      max_row,
    );
  }
  if row == 0 && column == 0 {
    merge_style_part(
      import,
      &mut result,
      &style.northwest_cell,
      false,
      column,
      max_column,
      row,
      max_row,
    );
  }
  if row == max_row && column == 0 {
    merge_style_part(
      import,
      &mut result,
      &style.southwest_cell,
      false,
      column,
      max_column,
      row,
      max_row,
    );
  }
  if row == 0 && column == max_column {
    merge_style_part(
      import,
      &mut result,
      &style.northeast_cell,
      false,
      column,
      max_column,
      row,
      max_row,
    );
  }
  if row == max_row && column == max_column {
    merge_style_part(
      import,
      &mut result,
      &style.southeast_cell,
      false,
      column,
      max_column,
      row,
      max_row,
    );
  }
  if table.band_column
    && (!table.first_row || row != 0)
    && (!table.last_row || row != max_row)
    && (!table.first_column || column != 0)
    && (!table.last_column || column != max_column)
  {
    let band = column + usize::from(table.first_column);
    let part = if band & 1 == 1 {
      &style.band2_vertical
    } else {
      &style.band1_vertical
    };
    merge_style_part(
      import,
      &mut result,
      part,
      false,
      column,
      max_column,
      row,
      max_row,
    );
  }
  result
}

fn merge_style_part(
  import: &PowerPointImport,
  target: &mut TableStylePart,
  source: &TableStylePart,
  whole_table: bool,
  column: usize,
  max_column: usize,
  row: usize,
  max_row: usize,
) {
  if let Some(fill) = table_style_part_fill(import, source) {
    target.fill_properties = Some(fill);
  }
  let mut borders = TableCellBorders::default();
  merge_style_borders(
    import,
    &mut borders,
    &source.borders,
    whole_table,
    column,
    max_column,
    row,
    max_row,
  );
  merge_cell_borders_from_style(&mut target.borders, &borders);
  target.text.merge_from(&source.text);
}

fn table_style_part_has_fill(part: &TableStylePart) -> bool {
  part.fill_properties.is_some() || part.fill_reference.is_some()
}

fn table_style_part_fill(
  import: &PowerPointImport,
  part: &TableStylePart,
) -> Option<FillProperties> {
  part.fill_properties.clone().or_else(|| {
    part.fill_reference.as_ref().and_then(|reference| {
      import
        .get_theme_fill_style(reference.index)
        .map(|fill| fill.with_placeholder_color(reference.placeholder_color.clone()))
    })
  })
}

fn merge_style_borders(
  import: &PowerPointImport,
  target: &mut TableCellBorders,
  source: &TableStyleBorders,
  whole_table: bool,
  column: usize,
  max_column: usize,
  row: usize,
  max_row: usize,
) {
  if (!whole_table || column == 0)
    && let Some(line) = table_style_border_line(import, &source.left, &source.left_reference)
  {
    target.left = Some(line);
  }
  if (!whole_table || column >= max_column)
    && let Some(line) = table_style_border_line(import, &source.right, &source.right_reference)
  {
    target.right = Some(line);
  }
  if (!whole_table || row == 0)
    && let Some(line) = table_style_border_line(import, &source.top, &source.top_reference)
  {
    target.top = Some(line);
  }
  if (!whole_table || row >= max_row)
    && let Some(line) = table_style_border_line(import, &source.bottom, &source.bottom_reference)
  {
    target.bottom = Some(line);
  }
  if let Some(line) = table_style_border_line(
    import,
    &source.inside_horizontal,
    &source.inside_horizontal_reference,
  ) {
    if row != 0 {
      target.top = Some(line.clone());
    }
    if row != max_row {
      target.bottom = Some(line);
    }
  }
  if let Some(line) = table_style_border_line(
    import,
    &source.inside_vertical,
    &source.inside_vertical_reference,
  ) {
    if column != 0 {
      target.left = Some(line.clone());
    }
    if column != max_column {
      target.right = Some(line);
    }
  }
  if let Some(line) = table_style_border_line(
    import,
    &source.top_left_to_bottom_right,
    &source.top_left_to_bottom_right_reference,
  ) {
    target.top_left_to_bottom_right = Some(line);
  }
  if let Some(line) = table_style_border_line(
    import,
    &source.bottom_left_to_top_right,
    &source.bottom_left_to_top_right_reference,
  ) {
    target.bottom_left_to_top_right = Some(line);
  }
}

fn table_style_border_line(
  import: &PowerPointImport,
  direct: &Option<LineProperties>,
  reference: &Option<super::drawingml::shape::ShapeStyleReference>,
) -> Option<LineProperties> {
  direct.clone().or_else(|| {
    reference.as_ref().and_then(|reference| {
      import
        .get_theme_line_style(reference.index)
        .map(|line| line.with_placeholder_color(reference.placeholder_color.clone()))
    })
  })
}

fn merge_cell_borders_from_style(target: &mut TableStyleBorders, source: &TableCellBorders) {
  if source.left.is_some() {
    target.left = source.left.clone();
  }
  if source.right.is_some() {
    target.right = source.right.clone();
  }
  if source.top.is_some() {
    target.top = source.top.clone();
  }
  if source.bottom.is_some() {
    target.bottom = source.bottom.clone();
  }
  if source.top_left_to_bottom_right.is_some() {
    target.top_left_to_bottom_right = source.top_left_to_bottom_right.clone();
  }
  if source.bottom_left_to_top_right.is_some() {
    target.bottom_left_to_top_right = source.bottom_left_to_top_right.clone();
  }
}

fn table_has_visible_direct_borders(table: &TableProperties) -> bool {
  table.rows.iter().any(|row| {
    row.cells.iter().any(|cell| {
      table_border_line_is_visible(&cell.borders.left)
        || table_border_line_is_visible(&cell.borders.right)
        || table_border_line_is_visible(&cell.borders.top)
        || table_border_line_is_visible(&cell.borders.bottom)
        || table_border_line_is_visible(&cell.borders.top_left_to_bottom_right)
        || table_border_line_is_visible(&cell.borders.bottom_left_to_top_right)
    })
  })
}

fn table_border_line_is_visible(line: &Option<LineProperties>) -> bool {
  matches!(
    line.as_ref().map(|line| &line.fill),
    Some(LineFill::Solid(_) | LineFill::Gradient(_) | LineFill::Pattern(_))
  )
}

fn lower_table_cell(
  import: &PowerPointImport,
  cell: &TableCell,
  style_part: Option<&TableStylePart>,
  table_background: Option<DisplayPaint>,
  x: f32,
  y: f32,
  width: f32,
  height: f32,
  items: &mut Vec<PageItem>,
) {
  if width <= 0.0 || height <= 0.0 {
    return;
  }
  let fill = table_cell_fill_paint(import, cell, style_part, table_background);
  if let Some(fill) = fill {
    items.push(PageItem::Rect(RectItem {
      x_pt: x,
      y_pt: y,
      width_pt: width,
      height_pt: height,
      fill_color: Some(fill.color),
      fill_opacity: fill.opacity,
      stroke: None,
      stroke_opacity: 1.0,
    }));
  }
  let borders = table_cell_effective_borders(cell, style_part);
  lower_table_cell_borders(import, &borders, x, y, width, height, items);

  if let Some(text_body) = &cell.text_body {
    let mut text_body = text_body.clone();
    text_body.display_properties.vertical = cell.vertical;
    text_body.display_properties.anchor = cell.anchor;
    text_body.display_properties.anchor_center = cell.anchor_center;
    text_body.display_properties.horizontal_overflow = Some(cell.horizontal_overflow);
    let x = x + units::emu_to_points(i64::from(cell.margins.left));
    let y = y + units::emu_to_points(i64::from(cell.margins.top));
    lower_text_body_at_with_table_style(
      import,
      TextFrame {
        x_pt: x,
        y_pt: y,
        width_pt: (width - units::emu_to_points(i64::from(cell.margins.left + cell.margins.right)))
          .max(0.0),
        height_pt: (height
          - units::emu_to_points(i64::from(cell.margins.top + cell.margins.bottom)))
        .max(0.0),
      },
      &text_body,
      style_part.map(|style| &style.text),
      items,
    );
  }
}

fn table_cell_fill_paint(
  import: &PowerPointImport,
  cell: &TableCell,
  style_part: Option<&TableStylePart>,
  table_background: Option<DisplayPaint>,
) -> Option<DisplayPaint> {
  let cell_fill = cell
    .fill_properties
    .as_ref()
    .map(|fill| fill_paint(import, fill))
    .unwrap_or_else(|| {
      style_part
        .and_then(|style| style.fill_properties.as_ref())
        .and_then(|fill| fill_paint(import, fill))
    });
  match (table_background, cell_fill) {
    (Some(background), Some(cell)) => Some(blend_table_cell_fill(background, cell)),
    (Some(background), None) => Some(background),
    (None, Some(cell)) => Some(cell),
    (None, None) => None,
  }
}

fn blend_table_cell_fill(background: DisplayPaint, cell: DisplayPaint) -> DisplayPaint {
  // Source: LibreOffice tablecell.cxx blends table background and cell fill
  // through basegfx::interpolate(bg, cell, 1 - cellTransparency).
  let cell_weight = cell.opacity.clamp(0.0, 1.0);
  let background_weight = 1.0 - cell_weight;
  DisplayPaint {
    color: RgbColor {
      r: blend_channel(
        background.color.r,
        cell.color.r,
        background_weight,
        cell_weight,
      ),
      g: blend_channel(
        background.color.g,
        cell.color.g,
        background_weight,
        cell_weight,
      ),
      b: blend_channel(
        background.color.b,
        cell.color.b,
        background_weight,
        cell_weight,
      ),
    },
    opacity: background.opacity.max(cell.opacity).clamp(0.0, 1.0),
  }
}

fn blend_channel(background: u8, cell: u8, background_weight: f32, cell_weight: f32) -> u8 {
  (f32::from(background) * background_weight + f32::from(cell) * cell_weight)
    .round()
    .clamp(0.0, 255.0) as u8
}

fn lower_table_cell_borders(
  import: &PowerPointImport,
  borders: &TableCellBorders,
  x: f32,
  y: f32,
  width: f32,
  height: f32,
  items: &mut Vec<PageItem>,
) {
  push_table_border_line(import, &borders.top, x, y, x + width, y, items);
  push_table_border_line(
    import,
    &borders.bottom,
    x,
    y + height,
    x + width,
    y + height,
    items,
  );
  push_table_border_line(import, &borders.left, x, y, x, y + height, items);
  push_table_border_line(
    import,
    &borders.right,
    x + width,
    y,
    x + width,
    y + height,
    items,
  );
  push_table_border_line(
    import,
    &borders.top_left_to_bottom_right,
    x,
    y,
    x + width,
    y + height,
    items,
  );
  push_table_border_line(
    import,
    &borders.bottom_left_to_top_right,
    x,
    y + height,
    x + width,
    y,
    items,
  );
}

fn table_cell_effective_borders(
  cell: &TableCell,
  style_part: Option<&TableStylePart>,
) -> TableCellBorders {
  let mut borders = TableCellBorders::default();
  if let Some(style_part) = style_part {
    borders.left = style_part.borders.left.clone();
    borders.right = style_part.borders.right.clone();
    borders.top = style_part.borders.top.clone();
    borders.bottom = style_part.borders.bottom.clone();
    borders.top_left_to_bottom_right = style_part.borders.top_left_to_bottom_right.clone();
    borders.bottom_left_to_top_right = style_part.borders.bottom_left_to_top_right.clone();
  }
  if cell.borders.left.is_some() {
    borders.left = cell.borders.left.clone();
  }
  if cell.borders.right.is_some() {
    borders.right = cell.borders.right.clone();
  }
  if cell.borders.top.is_some() {
    borders.top = cell.borders.top.clone();
  }
  if cell.borders.bottom.is_some() {
    borders.bottom = cell.borders.bottom.clone();
  }
  if cell.borders.top_left_to_bottom_right.is_some() {
    borders.top_left_to_bottom_right = cell.borders.top_left_to_bottom_right.clone();
  }
  if cell.borders.bottom_left_to_top_right.is_some() {
    borders.bottom_left_to_top_right = cell.borders.bottom_left_to_top_right.clone();
  }
  borders
}

fn push_table_border_line(
  import: &PowerPointImport,
  line: &Option<LineProperties>,
  x1_pt: f32,
  y1_pt: f32,
  x2_pt: f32,
  y2_pt: f32,
  items: &mut Vec<PageItem>,
) {
  let Some(stroke) = line.as_ref().and_then(|line| line_stroke(import, line)) else {
    return;
  };
  items.push(PageItem::Line(LineItem {
    x1_pt,
    y1_pt,
    x2_pt,
    y2_pt,
    width_pt: stroke.style.width_pt,
    color: stroke.style.color,
    kind: LineItemKind::Stroke,
  }));
}

fn table_cell_grid_span(cell: &TableCell) -> usize {
  cell
    .grid_span
    .and_then(|span| usize::try_from(span).ok())
    .filter(|span| *span > 0)
    .unwrap_or(1)
}

fn push_table_line(
  items: &mut Vec<PageItem>,
  x1_pt: f32,
  y1_pt: f32,
  x2_pt: f32,
  y2_pt: f32,
  color: RgbColor,
) {
  items.push(PageItem::Line(LineItem {
    x1_pt,
    y1_pt,
    x2_pt,
    y2_pt,
    width_pt: DEFAULT_TABLE_BORDER_PT,
    color,
    kind: LineItemKind::Stroke,
  }));
}

fn lower_shape_bounds(
  import: &PowerPointImport,
  slide: &SlidePersist,
  shape: &Shape,
  offset: DisplayOffset,
  items: &mut Vec<PageItem>,
) {
  if shape.service_name == ShapeService::GroupShape || shape.size.cx <= 0 || shape.size.cy <= 0 {
    return;
  }

  let fill_paint = shape
    .actual_fill_properties
    .as_ref()
    .and_then(|fill| fill_paint(import, fill));
  let x_pt = units::emu_to_points(offset.x_emu + shape.position.x);
  let y_pt = units::emu_to_points(offset.y_emu + shape.position.y);
  let width_pt = units::emu_to_points(shape.size.cx);
  let height_pt = units::emu_to_points(shape.size.cy);
  let fill_images = shape
    .actual_fill_properties
    .as_ref()
    .map(|fill| {
      blip_fill_image_items(
        import,
        slide,
        fill,
        x_pt,
        y_pt,
        width_pt,
        height_pt,
        shape.rotation,
        shape.flip_h,
        shape.flip_v,
        shape.custom_shape_properties.geometry.is_some(),
        shape
          .description
          .clone()
          .or_else(|| shape.title.clone())
          .or_else(|| shape.name.clone()),
        shape.hyperlink_url.clone(),
      )
    })
    .unwrap_or_default();
  let line = shape
    .actual_line_properties
    .as_ref()
    .and_then(|line| line_stroke(import, line));
  let has_fill_image = !fill_images.is_empty();
  if fill_paint.is_none() && !has_fill_image && line.is_none() {
    return;
  }
  items.extend(fill_images.into_iter().map(PageItem::Image));
  let (stroke, stroke_opacity) = line
    .map(|line| (Some(line.style), line.opacity))
    .unwrap_or((None, 1.0));
  if fill_paint.is_none() && stroke.is_none() {
    return;
  }

  items.push(PageItem::Rect(RectItem {
    x_pt,
    y_pt,
    width_pt,
    height_pt,
    fill_color: fill_paint.map(|fill| fill.color),
    fill_opacity: fill_paint.map(|fill| fill.opacity).unwrap_or(1.0),
    stroke,
    stroke_opacity,
  }));
}

fn child_display_offset(shape: &Shape, offset: DisplayOffset) -> DisplayOffset {
  DisplayOffset {
    x_emu: offset.x_emu + shape.position.x - shape.child_position.x,
    y_emu: offset.y_emu + shape.position.y - shape.child_position.y,
  }
}

fn blip_fill_image_items(
  import: &PowerPointImport,
  slide: &SlidePersist,
  fill: &FillProperties,
  x_pt: f32,
  y_pt: f32,
  width_pt: f32,
  height_pt: f32,
  rotation_deg: f32,
  flip_horizontal: bool,
  flip_vertical: bool,
  crop_bitmap: bool,
  alt_text: Option<String>,
  hyperlink_url: Option<String>,
) -> Vec<ImageItem> {
  let FillKind::Blip(blip_fill) = &fill.kind else {
    return Vec::new();
  };
  let Some(blip) = blip_fill.blip.as_ref() else {
    return Vec::new();
  };
  let Some(relationship_id) = blip.embed.as_deref() else {
    return Vec::new();
  };
  let Some(resource) = slide.image_resources.get(relationship_id) else {
    return Vec::new();
  };
  blip_fill_image_items_from_resource(
    import,
    slide,
    blip_fill,
    blip,
    resource,
    x_pt,
    y_pt,
    width_pt,
    height_pt,
    rotation_deg,
    flip_horizontal,
    flip_vertical,
    crop_bitmap,
    alt_text,
    hyperlink_url,
  )
}

fn blip_fill_image_items_from_resource(
  import: &PowerPointImport,
  slide: &SlidePersist,
  blip_fill: &a::BlipFill,
  blip: &a::Blip,
  resource: &ImageResource,
  x_pt: f32,
  y_pt: f32,
  width_pt: f32,
  height_pt: f32,
  rotation_deg: f32,
  flip_horizontal: bool,
  flip_vertical: bool,
  crop_bitmap: bool,
  alt_text: Option<String>,
  hyperlink_url: Option<String>,
) -> Vec<ImageItem> {
  let image_data = image_data_with_blip_effects(
    import,
    slide,
    &resource.data,
    resource.content_type.as_deref(),
    &blip.blip_choice,
  );
  let content_type = image_data
    .content_type
    .clone()
    .or_else(|| resource.content_type.clone());

  let crop = blip_fill_image_crop(blip_fill);
  if let Some(a::BlipFillChoice::Tile(tile)) = blip_fill.blip_fill_choice.as_ref() {
    return tiled_blip_fill_image_items(
      &image_data.data,
      content_type,
      tile,
      x_pt,
      y_pt,
      width_pt,
      height_pt,
      rotation_deg,
      flip_horizontal,
      flip_vertical,
      alt_text,
      hyperlink_url,
    );
  }

  // Source: LibreOffice oox/source/drawingml/fillproperties.cxx
  // lclGetBitmapMode() defaults missing bitmap mode to XML_tile for MSO.
  if blip_fill.blip_fill_choice.is_none() {
    return tiled_blip_fill_image_items(
      &image_data.data,
      content_type,
      &a::Tile::default(),
      x_pt,
      y_pt,
      width_pt,
      height_pt,
      rotation_deg,
      flip_horizontal,
      flip_vertical,
      alt_text,
      hyperlink_url,
    );
  }

  let (data, content_type, crop, flip_horizontal, flip_vertical) = if crop_bitmap
    && ((blip_fill.source_rectangle.is_some() && crop != ImageCrop::default())
      || flip_horizontal
      || flip_vertical)
  {
    transform_image_data_to_png(&image_data.data, crop, flip_horizontal, flip_vertical)
      .map(|data| {
        (
          data,
          Some("image/png".into()),
          ImageCrop::default(),
          false,
          false,
        )
      })
      .unwrap_or((
        image_data.data,
        content_type,
        crop,
        flip_horizontal,
        flip_vertical,
      ))
  } else {
    (
      image_data.data,
      content_type,
      crop,
      flip_horizontal,
      flip_vertical,
    )
  };

  vec![ImageItem {
    x_pt,
    y_pt,
    width_pt,
    height_pt,
    crop,
    rotation_deg,
    flip_horizontal,
    flip_vertical,
    data,
    content_type,
    alt_text,
    hyperlink_url,
    floating: false,
    behind_text: false,
  }]
}

fn tiled_blip_fill_image_items(
  data: &[u8],
  content_type: Option<String>,
  tile: &a::Tile,
  x_pt: f32,
  y_pt: f32,
  width_pt: f32,
  height_pt: f32,
  rotation_deg: f32,
  flip_horizontal: bool,
  flip_vertical: bool,
  alt_text: Option<String>,
  hyperlink_url: Option<String>,
) -> Vec<ImageItem> {
  let (mut tile_width_pt, mut tile_height_pt) =
    image_tile_size_pt(data).unwrap_or((width_pt, height_pt));
  let scale_x = tile
    .horizontal_ratio
    .as_ref()
    .map(|value| value.as_ratio() as f32)
    .unwrap_or(1.0);
  let scale_y = tile
    .vertical_ratio
    .as_ref()
    .map(|value| value.as_ratio() as f32)
    .unwrap_or(1.0);
  tile_width_pt = (tile_width_pt * scale_x).max(1.0).min(width_pt.max(1.0));
  tile_height_pt = (tile_height_pt * scale_y).max(1.0).min(height_pt.max(1.0));
  let offset_x_pt = tile
    .horizontal_offset
    .map(|value| units::emu_to_points(value.to_emu()))
    .unwrap_or(0.0);
  let offset_y_pt = tile
    .vertical_offset
    .map(|value| units::emu_to_points(value.to_emu()))
    .unwrap_or(0.0);

  let mut start_x = x_pt + offset_x_pt % tile_width_pt;
  while start_x > x_pt {
    start_x -= tile_width_pt;
  }
  let mut start_y = y_pt + offset_y_pt % tile_height_pt;
  while start_y > y_pt {
    start_y -= tile_height_pt;
  }

  let mut images = Vec::new();
  let mut y = start_y;
  let mut row = 0usize;
  while y < y_pt + height_pt && images.len() < 256 {
    let mut x = start_x;
    let mut column = 0usize;
    while x < x_pt + width_pt && images.len() < 256 {
      let item_x = x.max(x_pt);
      let item_y = y.max(y_pt);
      let item_right = (x + tile_width_pt).min(x_pt + width_pt);
      let item_bottom = (y + tile_height_pt).min(y_pt + height_pt);
      if item_right > item_x && item_bottom > item_y {
        let crop = ImageCrop {
          left: ((item_x - x) / tile_width_pt).max(0.0),
          top: ((item_y - y) / tile_height_pt).max(0.0),
          right: ((x + tile_width_pt - item_right) / tile_width_pt).max(0.0),
          bottom: ((y + tile_height_pt - item_bottom) / tile_height_pt).max(0.0),
        };
        let (tile_flip_h, tile_flip_v) = tile_flip(tile.flip, row, column);
        images.push(ImageItem {
          x_pt: item_x,
          y_pt: item_y,
          width_pt: item_right - item_x,
          height_pt: item_bottom - item_y,
          crop,
          rotation_deg,
          flip_horizontal: flip_horizontal ^ tile_flip_h,
          flip_vertical: flip_vertical ^ tile_flip_v,
          data: data.to_vec(),
          content_type: content_type.clone(),
          alt_text: alt_text.clone(),
          hyperlink_url: hyperlink_url.clone(),
          floating: false,
          behind_text: false,
        });
      }
      x += tile_width_pt;
      column += 1;
    }
    y += tile_height_pt;
    row += 1;
  }
  images
}

fn image_tile_size_pt(data: &[u8]) -> Option<(f32, f32)> {
  let image = image::load_from_memory(data).ok()?;
  Some((
    image.width() as f32 * units::POINTS_PER_CSS_PIXEL,
    image.height() as f32 * units::POINTS_PER_CSS_PIXEL,
  ))
}

fn tile_flip(flip: Option<a::TileFlipValues>, row: usize, column: usize) -> (bool, bool) {
  match flip.unwrap_or_default() {
    a::TileFlipValues::None => (false, false),
    a::TileFlipValues::Horizontal => (column % 2 == 1, false),
    a::TileFlipValues::Vertical => (false, row % 2 == 1),
    a::TileFlipValues::HorizontalAndVertical => (column % 2 == 1, row % 2 == 1),
  }
}

struct ImportedImageData {
  data: Vec<u8>,
  content_type: Option<String>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct ImageEffects {
  color_change: Option<ColorChangeEffect>,
  grayscale: bool,
  brightness: Option<i32>,
  contrast: Option<i32>,
  bilevel_threshold: Option<u8>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct ColorChangeEffect {
  from: RgbColor,
  to: RgbColor,
  alpha: u8,
  tolerance: u8,
}

fn image_data_with_blip_effects(
  import: &PowerPointImport,
  slide: &SlidePersist,
  data: &[u8],
  content_type: Option<&str>,
  blip_choices: &[a::BlipChoice],
) -> ImportedImageData {
  let effects = image_effects_from_blip(import, slide, blip_choices, content_type);
  if effects == ImageEffects::default() {
    return ImportedImageData {
      data: data.to_vec(),
      content_type: content_type.map(str::to_string),
    };
  }
  let Some(data) = apply_image_effects(data, content_type, effects) else {
    return ImportedImageData {
      data: data.to_vec(),
      content_type: content_type.map(str::to_string),
    };
  };
  ImportedImageData {
    data,
    content_type: Some("image/png".into()),
  }
}

fn image_effects_from_blip(
  import: &PowerPointImport,
  slide: &SlidePersist,
  blip_choices: &[a::BlipChoice],
  content_type: Option<&str>,
) -> ImageEffects {
  let mut effects = ImageEffects::default();
  for choice in blip_choices {
    match choice {
      a::BlipChoice::ColorChange(change) => {
        let Some(from) = change
          .color_from
          .color_from_choice
          .as_ref()
          .and_then(|choice| resolve_color_from_choice(import, slide, choice))
        else {
          continue;
        };
        let Some(to) = change
          .color_to
          .color_to_choice
          .as_ref()
          .and_then(|choice| resolve_color_to_choice(import, slide, choice))
        else {
          continue;
        };
        if from.r != to.r || from.g != to.g || from.b != to.b || to.alpha < 100_000 {
          effects.color_change = Some(ColorChangeEffect {
            from: RgbColor {
              r: from.r,
              g: from.g,
              b: from.b,
            },
            to: RgbColor {
              r: to.r,
              g: to.g,
              b: to.b,
            },
            alpha: ((to.alpha.clamp(0, 100_000) as u32 * 255 + 50_000) / 100_000) as u8,
            tolerance: color_change_tolerance(content_type),
          });
        }
      }
      a::BlipChoice::Grayscale => effects.grayscale = true,
      a::BlipChoice::LuminanceEffect(luminance) => {
        effects.brightness = luminance
          .brightness
          .as_ref()
          .map(|value| (value.as_ratio() * 100.0).round() as i32);
        effects.contrast = luminance
          .contrast
          .as_ref()
          .map(|value| (value.as_ratio() * 100.0).round() as i32);
      }
      a::BlipChoice::BiLevel(bilevel) => {
        // Source: LibreOffice oox/source/drawingml/fillproperties.cxx
        // lclApplyBlackWhiteEffect maps DrawingML's 0..100000 threshold to
        // BitmapMonochromeFilter's 0..255 luminance threshold.
        effects.bilevel_threshold = Some(
          (bilevel.threshold.as_ratio() * 255.0)
            .round()
            .clamp(0.0, 255.0) as u8,
        );
      }
      _ => {}
    }
  }
  effects
}

fn resolve_color_from_choice(
  import: &PowerPointImport,
  slide: &SlidePersist,
  choice: &a::ColorFromChoice,
) -> Option<super::drawingml::color::ResolvedColor> {
  let color = Color::from_color_from_choice(choice)?;
  import.resolve_color_for_slide(slide, &color, None)
}

fn resolve_color_to_choice(
  import: &PowerPointImport,
  slide: &SlidePersist,
  choice: &a::ColorToChoice,
) -> Option<super::drawingml::color::ResolvedColor> {
  let color = Color::from_color_to_choice(choice)?;
  import.resolve_color_for_slide(slide, &color, None)
}

fn color_change_tolerance(content_type: Option<&str>) -> u8 {
  // Source: LibreOffice oox/source/drawingml/fillproperties.cxx
  // lclCheckAndApplyChangeColorTransform uses 9 by default, native JPEG 15,
  // native PNG/TIFF 1, and native BMP 0.
  match content_type {
    Some("image/jpeg" | "image/jpg") => 15,
    Some("image/png" | "image/tiff" | "image/tif") => 1,
    Some("image/bmp" | "image/x-bmp") => 0,
    _ => 9,
  }
}

fn apply_image_effects(
  data: &[u8],
  content_type: Option<&str>,
  effects: ImageEffects,
) -> Option<Vec<u8>> {
  let raster_data = emf_wmf::decode_metafile_as_raster(data, content_type)
    .ok()
    .flatten()
    .map(|raster| raster.data)
    .unwrap_or_else(|| data.to_vec());
  let mut image = image::load_from_memory(&raster_data).ok()?.to_rgba8();
  for pixel in image.pixels_mut() {
    let [mut r, mut g, mut b, mut a] = pixel.0;
    if let Some(effect) = effects.color_change {
      if channel_within_tolerance(r, effect.from.r, effect.tolerance)
        && channel_within_tolerance(g, effect.from.g, effect.tolerance)
        && channel_within_tolerance(b, effect.from.b, effect.tolerance)
      {
        r = effect.to.r;
        g = effect.to.g;
        b = effect.to.b;
        a = effect.alpha;
      }
    }
    if effects.grayscale {
      let luminance = image_luminance(r, g, b);
      r = luminance;
      g = luminance;
      b = luminance;
    }
    if effects.brightness.is_some() || effects.contrast.is_some() {
      let brightness = effects.brightness.unwrap_or(0);
      let contrast = effects.contrast.unwrap_or(0);
      r = mso_brightness_contrast_component(r, brightness, contrast);
      g = mso_brightness_contrast_component(g, brightness, contrast);
      b = mso_brightness_contrast_component(b, brightness, contrast);
    }
    if let Some(threshold) = effects.bilevel_threshold {
      let value = if image_luminance(r, g, b) >= threshold {
        255
      } else {
        0
      };
      r = value;
      g = value;
      b = value;
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

fn transform_image_data_to_png(
  data: &[u8],
  crop: ImageCrop,
  flip_horizontal: bool,
  flip_vertical: bool,
) -> Option<Vec<u8>> {
  let mut image = image::load_from_memory(data).ok()?.to_rgba8();
  let width = image.width();
  let height = image.height();
  if width == 0 || height == 0 {
    return None;
  }

  // Source: LibreOffice oox/source/drawingml/fillproperties.cxx
  // lclCropGraphic rounds the srcRect-derived quotients against bitmap pixels
  // and creates a cropped bitmap before assigning it as custom-shape fill.
  let left = ((width as f32) * crop.left)
    .round()
    .clamp(0.0, width as f32) as u32;
  let top = ((height as f32) * crop.top)
    .round()
    .clamp(0.0, height as f32) as u32;
  let right = ((width as f32) * crop.right)
    .round()
    .clamp(0.0, width as f32) as u32;
  let bottom = ((height as f32) * crop.bottom)
    .round()
    .clamp(0.0, height as f32) as u32;
  if left + right >= width || top + bottom >= height {
    return None;
  }

  image = image::imageops::crop_imm(
    &image,
    left,
    top,
    width - left - right,
    height - top - bottom,
  )
  .to_image();
  // Source: LibreOffice oox/source/drawingml/fillproperties.cxx
  // lclMirrorGraphic mirrors custom-shape fill bitmaps directly instead of
  // relying on a shape-level bitmap flip.
  if flip_horizontal {
    image = image::imageops::flip_horizontal(&image);
  }
  if flip_vertical {
    image = image::imageops::flip_vertical(&image);
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

fn channel_within_tolerance(actual: u8, expected: u8, tolerance: u8) -> bool {
  actual.abs_diff(expected) <= tolerance
}

fn image_luminance(r: u8, g: u8, b: u8) -> u8 {
  ((u16::from(b) * 29 + u16::from(g) * 151 + u16::from(r) * 76) >> 8) as u8
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

fn blip_fill_image_crop(blip_fill: &a::BlipFill) -> ImageCrop {
  blip_fill
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
    .unwrap_or_default()
}

fn image_crop_from_source_rectangle(rect: &a::SourceRectangle) -> ImageCrop {
  // Source: LibreOffice oox/source/drawingml/fillproperties.cxx
  // CropQuotientsFromSrcRect clamps negative srcRect edges to zero before
  // deriving crop quotients.
  let left = drawingml_percent_ratio(rect.left.as_ref()).max(0.0);
  let top = drawingml_percent_ratio(rect.top.as_ref()).max(0.0);
  let right = drawingml_percent_ratio(rect.right.as_ref()).max(0.0);
  let bottom = drawingml_percent_ratio(rect.bottom.as_ref()).max(0.0);
  if left + right >= 1.0 || top + bottom >= 1.0 {
    return ImageCrop::default();
  }
  ImageCrop {
    left,
    top,
    right,
    bottom,
  }
}

fn image_crop_from_fill_rectangle(rect: &a::FillRectangle) -> ImageCrop {
  // Source: LibreOffice oox/source/drawingml/fillproperties.cxx
  // CropQuotientsFromFillRect ignores positive fillRect edges and computes
  // crop quotients from the negative growth denominator.
  let left = drawingml_percent_ratio(rect.left.as_ref()).min(0.0);
  let top = drawingml_percent_ratio(rect.top.as_ref()).min(0.0);
  let right = drawingml_percent_ratio(rect.right.as_ref()).min(0.0);
  let bottom = drawingml_percent_ratio(rect.bottom.as_ref()).min(0.0);
  let horizontal_divisor = -1.0 + left + right;
  let vertical_divisor = -1.0 + top + bottom;
  ImageCrop {
    left: left / horizontal_divisor,
    top: top / vertical_divisor,
    right: right / horizontal_divisor,
    bottom: bottom / vertical_divisor,
  }
}

fn drawingml_percent_ratio(value: Option<&DrawingmlPercentageValue>) -> f32 {
  value.map(|value| value.as_ratio() as f32).unwrap_or(0.0)
}

fn lower_text_body(
  import: &PowerPointImport,
  shape: &Shape,
  offset: DisplayOffset,
  text_body: &TextBody,
  items: &mut Vec<PageItem>,
) {
  let text_box = text_box_metrics(shape, offset, text_body);
  lower_text_body_at_with_font_ref(
    import,
    text_box,
    text_body,
    shape
      .shape_style_refs
      .as_ref()
      .map(|style| &style.font_reference),
    shape.hyperlink_url.as_deref(),
    items,
  );
}

fn lower_text_body_at_with_table_style(
  import: &PowerPointImport,
  frame: TextFrame,
  text_body: &TextBody,
  table_text_style: Option<&TableStyleTextProperties>,
  items: &mut Vec<PageItem>,
) {
  lower_text_body_at_with_style(
    import,
    frame,
    text_body,
    None,
    table_text_style,
    None,
    None,
    items,
  );
}

fn lower_text_body_at_with_font_ref(
  import: &PowerPointImport,
  frame: TextFrame,
  text_body: &TextBody,
  font_reference: Option<&FontStyleReference>,
  shape_hyperlink_url: Option<&str>,
  items: &mut Vec<PageItem>,
) {
  lower_text_body_at_with_style(
    import,
    frame,
    text_body,
    font_reference,
    None,
    shape_hyperlink_url,
    None,
    items,
  );
}

fn lower_text_body_at_with_style(
  import: &PowerPointImport,
  frame: TextFrame,
  text_body: &TextBody,
  font_reference: Option<&FontStyleReference>,
  table_text_style: Option<&TableStyleTextProperties>,
  shape_hyperlink_url: Option<&str>,
  base_font_size_pt: Option<f32>,
  items: &mut Vec<PageItem>,
) {
  lower_text_body_at_with_style_and_scale(
    import,
    frame,
    text_body,
    font_reference,
    table_text_style,
    shape_hyperlink_url,
    base_font_size_pt,
    None,
    items,
  );
}

fn lower_text_body_at_with_style_and_scale(
  import: &PowerPointImport,
  frame: TextFrame,
  text_body: &TextBody,
  font_reference: Option<&FontStyleReference>,
  table_text_style: Option<&TableStyleTextProperties>,
  shape_hyperlink_url: Option<&str>,
  base_font_size_pt: Option<f32>,
  auto_fit_font_scale: Option<f32>,
  items: &mut Vec<PageItem>,
) {
  let mut options = TextLoweringOptions::from_text_body(text_body);
  let base_style = text_base_style(
    import,
    text_body,
    font_reference,
    table_text_style,
    base_font_size_pt,
  );
  options.font_scale = auto_fit_font_scale
    .unwrap_or_else(|| text_auto_fit_font_scale(import, frame, text_body, &base_style, &options));

  let mut scaled_base_style = base_style.clone();
  apply_text_scale(&mut scaled_base_style, &options);
  let estimated_height =
    estimate_text_body_height(text_body, &scaled_base_style, options.line_scale);
  let y_pt = match text_body.display_properties.anchor {
    a::TextAnchoringTypeValues::Center => {
      frame.y_pt + ((frame.height_pt - estimated_height) / 2.0).max(0.0)
    }
    a::TextAnchoringTypeValues::Bottom => {
      frame.y_pt + (frame.height_pt - estimated_height).max(0.0)
    }
    a::TextAnchoringTypeValues::Top => frame.y_pt,
  };

  let mut cursor = TextCursor {
    x_pt: frame.x_pt,
    y_pt,
    column_index: 0,
  };
  for paragraph in &text_body.paragraphs {
    lower_paragraph(
      import,
      paragraph,
      &base_style,
      &options,
      frame,
      shape_hyperlink_url,
      &mut cursor,
      items,
    );
  }
}

fn text_base_style(
  import: &PowerPointImport,
  text_body: &TextBody,
  font_reference: Option<&FontStyleReference>,
  table_text_style: Option<&TableStyleTextProperties>,
  base_font_size_pt: Option<f32>,
) -> TextStyle {
  let options = TextLoweringOptions::from_text_body(text_body);
  let mut base_style = TextStyle {
    font_family: Some(Arc::from("Liberation Sans")),
    font_size_pt: base_font_size_pt.unwrap_or(DEFAULT_TEXT_FONT_SIZE_PT),
    rotation_deg: options.rotation_deg,
    ..TextStyle::default()
  };
  if let Some(font_reference) = font_reference {
    apply_font_reference_text_style(import, font_reference, &mut base_style);
  }
  if let Some(table_text_style) = table_text_style {
    apply_table_text_style(import, table_text_style, &mut base_style);
  }
  base_style
}

fn text_auto_fit_font_scale(
  import: &PowerPointImport,
  frame: TextFrame,
  text_body: &TextBody,
  base_style: &TextStyle,
  options: &TextLoweringOptions,
) -> f32 {
  if text_body.display_properties.auto_fit != TextAutoFit::Shape {
    return options.font_scale;
  }
  let text_width = estimate_text_body_width(import, text_body, base_style, options);
  let text_height = estimate_text_body_height(text_body, base_style, options.line_scale);
  let width_scale = if text_width > 0.0 {
    frame.width_pt / text_width
  } else {
    1.0
  };
  let height_scale = if text_height > 0.0 {
    frame.height_pt / text_height
  } else {
    1.0
  };
  options.font_scale * width_scale.min(height_scale).clamp(0.01, 1.0)
}

fn apply_font_reference_text_style(
  import: &PowerPointImport,
  reference: &FontStyleReference,
  style: &mut TextStyle,
) {
  if let Some(typeface) = import.get_theme_latin_font(reference.index) {
    style.font_family = Some(Arc::from(typeface));
  }
  if let Some(paint) = reference
    .placeholder_color
    .as_ref()
    .and_then(|color| display_paint(import, color, None))
  {
    style.color = paint.color;
    style.opacity = paint.opacity;
  }
}

fn apply_table_text_style(
  import: &PowerPointImport,
  properties: &TableStyleTextProperties,
  style: &mut TextStyle,
) {
  if let Some(font_reference) = &properties.font_reference {
    apply_font_reference_text_style(import, font_reference, style);
  }
  if let Some(typeface) = properties.fonts.latin.as_deref() {
    style.font_family = Some(Arc::from(typeface));
  }
  if let Some(bold) = properties.bold.and_then(boolean_style_value) {
    style.bold = bold;
  }
  if let Some(italic) = properties.italic.and_then(boolean_style_value) {
    style.italic = italic;
  }
  if let Some(paint) = properties
    .color
    .as_ref()
    .and_then(|color| display_paint(import, color, None))
  {
    style.color = paint.color;
    style.opacity = paint.opacity;
  }
}

fn boolean_style_value(value: a::BooleanStyleValues) -> Option<bool> {
  match value {
    a::BooleanStyleValues::On => Some(true),
    a::BooleanStyleValues::Off => Some(false),
    a::BooleanStyleValues::Default => None,
  }
}

#[derive(Clone, Copy, Debug)]
struct TextFrame {
  x_pt: f32,
  y_pt: f32,
  width_pt: f32,
  height_pt: f32,
}

#[derive(Clone, Copy, Debug)]
struct TextCursor {
  x_pt: f32,
  y_pt: f32,
  column_index: usize,
}

#[derive(Clone, Copy, Debug)]
struct TextLoweringOptions {
  font_scale: f32,
  line_scale: f32,
  rotation_deg: f32,
  column_count: usize,
  column_spacing_pt: f32,
  clip_vertical_overflow: bool,
}

impl TextLoweringOptions {
  fn from_text_body(text_body: &TextBody) -> Self {
    Self {
      font_scale: text_body.display_properties.font_scale(),
      line_scale: text_body.display_properties.line_height_scale(),
      rotation_deg: text_body.display_properties.rotation_degrees(),
      column_count: text_body.display_properties.column_count.max(1),
      column_spacing_pt: units::emu_to_points(text_body.display_properties.column_spacing_emu),
      clip_vertical_overflow: text_body.display_properties.clip_vertical_overflow,
    }
  }

  fn column_width(self, frame: TextFrame) -> f32 {
    if self.column_count <= 1 {
      frame.width_pt
    } else {
      let total_spacing = self.column_spacing_pt * (self.column_count - 1) as f32;
      ((frame.width_pt - total_spacing) / self.column_count as f32).max(0.0)
    }
  }
}

fn text_box_metrics(shape: &Shape, offset: DisplayOffset, text_body: &TextBody) -> TextFrame {
  text_body_frame(
    units::emu_to_points(offset.x_emu + shape.position.x),
    units::emu_to_points(offset.y_emu + shape.position.y),
    units::emu_to_points(shape.size.cx),
    units::emu_to_points(shape.size.cy),
    text_body,
  )
}

fn text_body_frame(
  x_pt: f32,
  y_pt: f32,
  width_pt: f32,
  height_pt: f32,
  text_body: &TextBody,
) -> TextFrame {
  let body_properties = text_body.body_properties.as_deref();
  let left_inset = body_properties
    .and_then(|properties| properties.left_inset)
    .map(|value| units::emu_to_points(value.to_emu()))
    .unwrap_or_else(|| units::emu_to_points(DEFAULT_TEXT_INSET_EMU));
  let top_inset = body_properties
    .and_then(|properties| properties.top_inset)
    .map(|value| units::emu_to_points(value.to_emu()))
    .unwrap_or_else(|| units::emu_to_points(DEFAULT_TEXT_INSET_EMU));
  let right_inset = body_properties
    .and_then(|properties| properties.right_inset)
    .map(|value| units::emu_to_points(value.to_emu()))
    .unwrap_or_else(|| units::emu_to_points(DEFAULT_TEXT_INSET_EMU));
  let bottom_inset = body_properties
    .and_then(|properties| properties.bottom_inset)
    .map(|value| units::emu_to_points(value.to_emu()))
    .unwrap_or_else(|| units::emu_to_points(DEFAULT_TEXT_INSET_EMU));

  TextFrame {
    x_pt: x_pt + left_inset,
    y_pt: y_pt + top_inset,
    width_pt: (width_pt - left_inset - right_inset).max(0.0),
    height_pt: (height_pt - top_inset - bottom_inset).max(0.0),
  }
}

fn lower_paragraph(
  import: &PowerPointImport,
  paragraph: &TextParagraph,
  base_style: &TextStyle,
  options: &TextLoweringOptions,
  frame: TextFrame,
  shape_hyperlink_url: Option<&str>,
  cursor: &mut TextCursor,
  items: &mut Vec<PageItem>,
) {
  let paragraph_style = ParagraphDisplayStyle::from_paragraph(paragraph);
  let column_width = options.column_width(frame);
  let column_x =
    frame.x_pt + cursor.column_index as f32 * (column_width + options.column_spacing_pt);
  cursor.x_pt = column_x;
  if options.clip_vertical_overflow && cursor.y_pt > frame.y_pt + frame.height_pt {
    return;
  }
  let bullet_label = paragraph_style.bullet_label(paragraph);
  let paragraph_x = cursor.x_pt
    + paragraph_style.left_margin_pt
    + if bullet_label.is_some() {
      0.0
    } else {
      paragraph_style.indent_pt
    };
  let mut segment_start = 0usize;
  let mut is_first_segment = true;

  loop {
    let segment_end = paragraph.runs[segment_start..]
      .iter()
      .position(|run| run.kind == TextRunKind::Break)
      .map(|offset| segment_start + offset)
      .unwrap_or(paragraph.runs.len());
    let line_width = paragraph_run_width(
      import,
      &paragraph_style,
      base_style,
      options,
      &paragraph.runs[segment_start..segment_end],
    );
    let mut run_x = aligned_paragraph_x(
      paragraph_x,
      column_width,
      line_width,
      paragraph_style.alignment,
    );
    let mut base_line_style = base_style.clone();
    apply_text_scale(&mut base_line_style, options);
    let mut max_line_height = line_height(&base_line_style, options.line_scale);

    if is_first_segment && let Some(label) = bullet_label.clone() {
      let mut bullet_style = base_style.clone();
      paragraph_style.apply_default_run_style(import, &mut bullet_style);
      apply_text_scale(&mut bullet_style, options);
      max_line_height = max_line_height.max(line_height(&bullet_style, options.line_scale));
      push_text_item(
        items,
        run_x + paragraph_style.indent_pt,
        cursor.y_pt,
        label,
        bullet_style,
        shape_hyperlink_url.map(ToString::to_string),
        options.line_scale,
      );
    }

    for run in &paragraph.runs[segment_start..segment_end] {
      if !matches!(
        run.kind,
        TextRunKind::Run | TextRunKind::Field | TextRunKind::Math
      ) || run.text.is_empty()
      {
        continue;
      }
      let mut style = base_style.clone();
      paragraph_style.apply_default_run_style(import, &mut style);
      apply_run_properties(import, run, &mut style);
      apply_text_scale(&mut style, options);
      max_line_height = max_line_height.max(line_height(&style, options.line_scale));
      let text = run_text(run, &style);
      if run.kind == TextRunKind::Math {
        push_math_ole_preview_item(
          items,
          run_x,
          cursor.y_pt,
          measure_text(&text, &style),
          line_height(&style, options.line_scale),
        );
      }
      push_text_item(
        items,
        run_x,
        cursor.y_pt,
        text.clone(),
        style.clone(),
        run
          .hyperlink_url
          .clone()
          .or_else(|| shape_hyperlink_url.map(ToString::to_string)),
        options.line_scale,
      );
      run_x += measure_text(&text, &style);
    }

    cursor.y_pt += max_line_height;
    advance_text_column_if_needed(cursor, frame, *options);
    if segment_end == paragraph.runs.len() {
      break;
    }
    segment_start = segment_end + 1;
    is_first_segment = false;
  }
}

fn paragraph_run_width(
  import: &PowerPointImport,
  paragraph_style: &ParagraphDisplayStyle,
  base_style: &TextStyle,
  options: &TextLoweringOptions,
  runs: &[TextRun],
) -> f32 {
  runs
    .iter()
    .filter(|run| {
      matches!(
        run.kind,
        TextRunKind::Run | TextRunKind::Field | TextRunKind::Math
      ) && !run.text.is_empty()
    })
    .map(|run| {
      let mut style = base_style.clone();
      paragraph_style.apply_default_run_style(import, &mut style);
      apply_run_properties(import, run, &mut style);
      apply_text_scale(&mut style, options);
      measure_text(&run_text(run, &style), &style)
    })
    .sum()
}

fn apply_text_scale(style: &mut TextStyle, options: &TextLoweringOptions) {
  style.font_size_pt *= options.font_scale;
  style.character_spacing_pt *= options.font_scale;
  style.baseline_shift_pt *= options.font_scale;
}

fn aligned_paragraph_x(
  paragraph_x: f32,
  column_width: f32,
  line_width: f32,
  alignment: a::TextAlignmentTypeValues,
) -> f32 {
  match alignment {
    a::TextAlignmentTypeValues::Center => {
      paragraph_x + ((column_width - line_width) / 2.0).max(0.0)
    }
    a::TextAlignmentTypeValues::Right => paragraph_x + (column_width - line_width).max(0.0),
    a::TextAlignmentTypeValues::Left
    | a::TextAlignmentTypeValues::Justified
    | a::TextAlignmentTypeValues::JustifiedLow
    | a::TextAlignmentTypeValues::Distributed
    | a::TextAlignmentTypeValues::ThaiDistributed => paragraph_x,
  }
}

fn push_text_item(
  items: &mut Vec<PageItem>,
  x_pt: f32,
  y_pt: f32,
  text: String,
  style: TextStyle,
  hyperlink_url: Option<String>,
  line_scale: f32,
) {
  items.push(PageItem::Text(TextItem {
    x_pt,
    y_pt,
    line_height_pt: line_height(&style, line_scale),
    text,
    style,
    hyperlink_url,
    dynamic_field: None,
    style_ref_keys: Vec::new(),
    style_ref_text: None,
    form_widget_id: None,
    paragraph_bidi: false,
    preserve_text_portion: false,
    decoration_span_start_x_pt: None,
    pdf_text_segmentation: PdfTextSegmentation::Line,
  }));
}

fn line_height(style: &TextStyle, line_scale: f32) -> f32 {
  style.font_size_pt * DEFAULT_TEXT_LINE_HEIGHT_SCALE * line_scale
}

fn estimate_text_body_height(text_body: &TextBody, base_style: &TextStyle, line_scale: f32) -> f32 {
  let mut lines = 0usize;
  for paragraph in &text_body.paragraphs {
    lines += paragraph
      .runs
      .iter()
      .filter(|run| run.kind == TextRunKind::Break)
      .count()
      + 1;
  }
  lines as f32 * line_height(base_style, line_scale)
}

fn estimate_text_body_width(
  import: &PowerPointImport,
  text_body: &TextBody,
  base_style: &TextStyle,
  options: &TextLoweringOptions,
) -> f32 {
  text_body
    .paragraphs
    .iter()
    .map(|paragraph| {
      let paragraph_style = ParagraphDisplayStyle::from_paragraph(paragraph);
      paragraph
        .runs
        .split(|run| run.kind == TextRunKind::Break)
        .map(|runs| paragraph_run_width(import, &paragraph_style, base_style, options, runs))
        .fold(0.0, f32::max)
    })
    .fold(0.0, f32::max)
}

fn advance_text_column_if_needed(
  cursor: &mut TextCursor,
  frame: TextFrame,
  options: TextLoweringOptions,
) {
  if options.column_count <= 1 || cursor.y_pt <= frame.y_pt + frame.height_pt {
    return;
  }
  if cursor.column_index + 1 >= options.column_count {
    return;
  }
  cursor.column_index += 1;
  cursor.y_pt = frame.y_pt;
}

fn run_text(run: &TextRun, style: &TextStyle) -> String {
  if style.uppercase {
    run.text.to_uppercase()
  } else {
    run.text.clone()
  }
}

fn push_math_ole_preview_item(
  items: &mut Vec<PageItem>,
  x_pt: f32,
  y_pt: f32,
  width_pt: f32,
  height_pt: f32,
) {
  if width_pt <= f32::EPSILON || height_pt <= f32::EPSILON {
    return;
  }
  let Some(data) = transparent_png_1x1() else {
    return;
  };
  items.push(PageItem::Image(ImageItem {
    x_pt,
    y_pt,
    width_pt,
    height_pt,
    crop: ImageCrop::default(),
    rotation_deg: 0.0,
    flip_horizontal: false,
    flip_vertical: false,
    data,
    content_type: Some("image/png".to_string()),
    alt_text: None,
    hyperlink_url: None,
    floating: false,
    behind_text: false,
  }));
}

fn transparent_png_1x1() -> Option<Vec<u8>> {
  let mut output = Vec::new();
  let encoder = PngEncoder::new(Cursor::new(&mut output));
  encoder
    .write_image(&[0, 0, 0, 0], 1, 1, ColorType::Rgba8.into())
    .ok()?;
  Some(output)
}

#[derive(Clone, Debug)]
struct ParagraphDisplayStyle {
  left_margin_pt: f32,
  indent_pt: f32,
  alignment: a::TextAlignmentTypeValues,
  bullet: Option<String>,
  default_run_properties: Option<Box<a::DefaultRunProperties>>,
}

impl Default for ParagraphDisplayStyle {
  fn default() -> Self {
    Self {
      left_margin_pt: 0.0,
      indent_pt: 0.0,
      alignment: a::TextAlignmentTypeValues::Left,
      bullet: None,
      default_run_properties: None,
    }
  }
}

impl ParagraphDisplayStyle {
  fn from_paragraph(paragraph: &TextParagraph) -> Self {
    let mut style = Self::default();
    if let Some(master_style) = &paragraph.master_paragraph_style {
      style.apply_text_list_style(master_style);
    }
    if let Some(text_style) = &paragraph.text_paragraph_style {
      style.apply_text_list_style(text_style);
    }
    if let Some(properties) = paragraph.paragraph_properties.as_deref() {
      if let Some(left_margin) = properties.left_margin {
        style.left_margin_pt = units::emu_to_points(i64::from(left_margin));
      }
      if let Some(indent) = properties.indent {
        style.indent_pt = units::emu_to_points(i64::from(indent));
      }
      if let Some(default_run_properties) = &properties.default_run_properties {
        style.default_run_properties = Some(default_run_properties.clone());
      }
      if let Some(alignment) = properties.alignment {
        style.alignment = alignment;
      }
      style.bullet = paragraph_properties_bullet(&properties.paragraph_properties_choice4);
    }
    style
  }

  fn apply_text_list_style(&mut self, style: &TextListParagraphStyle) {
    match style {
      TextListParagraphStyle::Default(properties) => {
        self.left_margin_pt = properties
          .left_margin
          .map(|value| units::emu_to_points(i64::from(value)))
          .unwrap_or(self.left_margin_pt);
        self.indent_pt = properties
          .indent
          .map(|value| units::emu_to_points(i64::from(value)))
          .unwrap_or(self.indent_pt);
        self.alignment = properties.alignment.unwrap_or(self.alignment);
        self.default_run_properties = properties.default_run_properties.clone();
        self.bullet =
          default_paragraph_properties_bullet(&properties.default_paragraph_properties_choice4);
      }
      TextListParagraphStyle::Level(level) => {
        self.apply_level_style(&level.paragraph_properties);
      }
    }
  }

  fn apply_level_style(&mut self, properties: &TextListLevelParagraphProperties) {
    macro_rules! apply_level {
      ($properties:expr, $bullet_fn:ident, $choice:ident) => {{
        self.left_margin_pt = $properties
          .left_margin
          .map(|value| units::emu_to_points(i64::from(value)))
          .unwrap_or(self.left_margin_pt);
        self.indent_pt = $properties
          .indent
          .map(|value| units::emu_to_points(i64::from(value)))
          .unwrap_or(self.indent_pt);
        self.alignment = $properties.alignment.unwrap_or(self.alignment);
        self.default_run_properties = $properties.default_run_properties.clone();
        self.bullet = $bullet_fn(&$properties.$choice);
      }};
    }

    match properties {
      TextListLevelParagraphProperties::Level1(properties) => {
        apply_level!(
          properties,
          level1_paragraph_properties_bullet,
          level1_paragraph_properties_choice4
        )
      }
      TextListLevelParagraphProperties::Level2(properties) => {
        apply_level!(
          properties,
          level2_paragraph_properties_bullet,
          level2_paragraph_properties_choice4
        )
      }
      TextListLevelParagraphProperties::Level3(properties) => {
        apply_level!(
          properties,
          level3_paragraph_properties_bullet,
          level3_paragraph_properties_choice4
        )
      }
      TextListLevelParagraphProperties::Level4(properties) => {
        apply_level!(
          properties,
          level4_paragraph_properties_bullet,
          level4_paragraph_properties_choice4
        )
      }
      TextListLevelParagraphProperties::Level5(properties) => {
        apply_level!(
          properties,
          level5_paragraph_properties_bullet,
          level5_paragraph_properties_choice4
        )
      }
      TextListLevelParagraphProperties::Level6(properties) => {
        apply_level!(
          properties,
          level6_paragraph_properties_bullet,
          level6_paragraph_properties_choice4
        )
      }
      TextListLevelParagraphProperties::Level7(properties) => {
        apply_level!(
          properties,
          level7_paragraph_properties_bullet,
          level7_paragraph_properties_choice4
        )
      }
      TextListLevelParagraphProperties::Level8(properties) => {
        apply_level!(
          properties,
          level8_paragraph_properties_bullet,
          level8_paragraph_properties_choice4
        )
      }
      TextListLevelParagraphProperties::Level9(properties) => {
        apply_level!(
          properties,
          level9_paragraph_properties_bullet,
          level9_paragraph_properties_choice4
        )
      }
    }
  }

  fn bullet_label(&self, paragraph: &TextParagraph) -> Option<String> {
    if paragraph
      .paragraph_properties
      .as_deref()
      .and_then(|properties| properties.paragraph_properties_choice4.as_ref())
      .is_some_and(|choice| matches!(choice, a::ParagraphPropertiesChoice4::NoBullet))
    {
      return None;
    }
    self.bullet.clone().or_else(|| {
      paragraph
        .level
        .filter(|level| *level > 0)
        .map(|_| "\u{2022}".to_string())
    })
  }

  fn apply_default_run_style(&self, import: &PowerPointImport, style: &mut TextStyle) {
    if let Some(properties) = &self.default_run_properties {
      apply_default_run_properties(import, properties, style);
    }
  }
}

fn paragraph_properties_bullet(choice: &Option<a::ParagraphPropertiesChoice4>) -> Option<String> {
  match choice {
    Some(a::ParagraphPropertiesChoice4::NoBullet) => None,
    Some(a::ParagraphPropertiesChoice4::CharacterBullet(bullet)) => Some(bullet.char.clone()),
    Some(a::ParagraphPropertiesChoice4::AutoNumberedBullet(_)) => Some("1.".to_string()),
    Some(a::ParagraphPropertiesChoice4::PictureBullet(_)) => Some("\u{2022}".to_string()),
    None => None,
  }
}

macro_rules! bullet_fn {
  ($name:ident, $choice_ty:ty) => {
    fn $name(choice: &Option<$choice_ty>) -> Option<String> {
      match choice {
        Some(choice) => level_bullet_label(choice),
        None => None,
      }
    }
  };
}

fn default_paragraph_properties_bullet(
  choice: &Option<a::DefaultParagraphPropertiesChoice4>,
) -> Option<String> {
  match choice {
    Some(a::DefaultParagraphPropertiesChoice4::NoBullet) => None,
    Some(a::DefaultParagraphPropertiesChoice4::CharacterBullet(bullet)) => {
      Some(bullet.char.clone())
    }
    Some(a::DefaultParagraphPropertiesChoice4::AutoNumberedBullet(_)) => Some("1.".to_string()),
    Some(a::DefaultParagraphPropertiesChoice4::PictureBullet(_)) => Some("\u{2022}".to_string()),
    None => None,
  }
}

bullet_fn!(
  level1_paragraph_properties_bullet,
  a::Level1ParagraphPropertiesChoice4
);
bullet_fn!(
  level2_paragraph_properties_bullet,
  a::Level2ParagraphPropertiesChoice4
);
bullet_fn!(
  level3_paragraph_properties_bullet,
  a::Level3ParagraphPropertiesChoice4
);
bullet_fn!(
  level4_paragraph_properties_bullet,
  a::Level4ParagraphPropertiesChoice4
);
bullet_fn!(
  level5_paragraph_properties_bullet,
  a::Level5ParagraphPropertiesChoice4
);
bullet_fn!(
  level6_paragraph_properties_bullet,
  a::Level6ParagraphPropertiesChoice4
);
bullet_fn!(
  level7_paragraph_properties_bullet,
  a::Level7ParagraphPropertiesChoice4
);
bullet_fn!(
  level8_paragraph_properties_bullet,
  a::Level8ParagraphPropertiesChoice4
);
bullet_fn!(
  level9_paragraph_properties_bullet,
  a::Level9ParagraphPropertiesChoice4
);

trait BulletChoice {
  fn no_bullet(&self) -> bool;
  fn character(&self) -> Option<String>;
  fn auto_numbered(&self) -> bool;
  fn picture(&self) -> bool;
}

macro_rules! impl_bullet_choice {
  ($ty:ty) => {
    impl BulletChoice for $ty {
      fn no_bullet(&self) -> bool {
        matches!(self, Self::NoBullet)
      }

      fn character(&self) -> Option<String> {
        match self {
          Self::CharacterBullet(bullet) => Some(bullet.char.clone()),
          _ => None,
        }
      }

      fn auto_numbered(&self) -> bool {
        matches!(self, Self::AutoNumberedBullet(_))
      }

      fn picture(&self) -> bool {
        matches!(self, Self::PictureBullet(_))
      }
    }
  };
}

impl_bullet_choice!(a::Level1ParagraphPropertiesChoice4);
impl_bullet_choice!(a::Level2ParagraphPropertiesChoice4);
impl_bullet_choice!(a::Level3ParagraphPropertiesChoice4);
impl_bullet_choice!(a::Level4ParagraphPropertiesChoice4);
impl_bullet_choice!(a::Level5ParagraphPropertiesChoice4);
impl_bullet_choice!(a::Level6ParagraphPropertiesChoice4);
impl_bullet_choice!(a::Level7ParagraphPropertiesChoice4);
impl_bullet_choice!(a::Level8ParagraphPropertiesChoice4);
impl_bullet_choice!(a::Level9ParagraphPropertiesChoice4);

fn level_bullet_label(choice: &impl BulletChoice) -> Option<String> {
  if choice.no_bullet() {
    None
  } else if let Some(character) = choice.character() {
    Some(character)
  } else if choice.auto_numbered() {
    Some("1.".to_string())
  } else if choice.picture() {
    Some("\u{2022}".to_string())
  } else {
    None
  }
}

fn apply_run_properties(import: &PowerPointImport, run: &TextRun, style: &mut TextStyle) {
  if run.kind == TextRunKind::Math {
    // Source: LibreOffice oox/source/drawingml/shape.cxx imports a14:m as a
    // Math OLE object. Use the Office math face for text extraction/rendering
    // of the flattened math text instead of inheriting the surrounding
    // DrawingML paragraph font.
    style.font_family = Some(Arc::from("Cambria Math"));
  }
  let Some(properties) = run.run_properties.as_deref() else {
    return;
  };
  apply_run_common(
    RunCommon {
      font_size: properties.font_size,
      bold: properties.bold.as_ref().map(|value| value.as_bool()),
      italic: properties.italic.as_ref().map(|value| value.as_bool()),
      underline: properties.underline,
      strike: properties.strike,
      capital: properties.capital,
      spacing: properties.spacing,
      baseline: properties.baseline,
      latin_font: properties.latin_font.as_ref(),
    },
    style,
  );
  if let Some(fill) = properties.run_properties_choice1.as_ref() {
    apply_text_fill(import, fill, style);
  }
}

fn apply_default_run_properties(
  import: &PowerPointImport,
  properties: &a::DefaultRunProperties,
  style: &mut TextStyle,
) {
  apply_run_common(
    RunCommon {
      font_size: properties.font_size,
      bold: properties.bold.as_ref().map(|value| value.as_bool()),
      italic: properties.italic.as_ref().map(|value| value.as_bool()),
      underline: properties.underline,
      strike: properties.strike,
      capital: properties.capital,
      spacing: properties.spacing,
      baseline: properties.baseline,
      latin_font: properties.latin_font.as_ref(),
    },
    style,
  );
  if let Some(fill) = properties.default_run_properties_choice1.as_ref() {
    apply_default_text_fill(import, fill, style);
  }
}

struct RunCommon<'a> {
  font_size: Option<i32>,
  bold: Option<bool>,
  italic: Option<bool>,
  underline: Option<a::TextUnderlineValues>,
  strike: Option<a::TextStrikeValues>,
  capital: Option<a::TextCapsValues>,
  spacing: Option<ooxmlsdk::simple_type::TextPointValue>,
  baseline: Option<ooxmlsdk::simple_type::DrawingmlPercentageValue>,
  latin_font: Option<&'a a::LatinFont>,
}

fn apply_run_common(properties: RunCommon<'_>, style: &mut TextStyle) {
  if let Some(font_size) = properties.font_size {
    style.font_size_pt = ooxmlsdk::units::drawingml_text_size_to_points(font_size) as f32;
  }
  if let Some(bold) = properties.bold {
    style.bold = bold;
  }
  if let Some(italic) = properties.italic {
    style.italic = italic;
  }
  if let Some(underline) = properties.underline {
    style.underline = underline != a::TextUnderlineValues::None;
  }
  if let Some(strike) = properties.strike {
    style.strikethrough = strike != a::TextStrikeValues::NoStrike;
  }
  if let Some(capital) = properties.capital {
    style.uppercase = capital == a::TextCapsValues::All;
    style.small_caps = capital == a::TextCapsValues::Small;
  }
  if let Some(spacing) = properties.spacing {
    style.character_spacing_pt = spacing.to_points() as f32;
  }
  if let Some(baseline) = properties.baseline {
    style.baseline_shift_pt =
      style.font_size_pt * baseline.as_drawingml_percent() as f32 / 100_000.0;
  }
  if let Some(typeface) = properties
    .latin_font
    .and_then(|font| font.typeface.as_ref())
    .filter(|typeface| !typeface.is_empty())
  {
    style.font_family = Some(Arc::from(typeface.as_str()));
  }
}

fn apply_text_fill(
  import: &PowerPointImport,
  fill: &a::RunPropertiesChoice,
  style: &mut TextStyle,
) {
  match fill {
    a::RunPropertiesChoice::NoFill(_) => {
      // Source: LibreOffice sd/qa/unit/import-tests2.cxx:testTdf118776
      // imports text noFill as 99% CharTransparence, not as a dropped run.
      style.color = RgbColor { r: 0, g: 0, b: 0 };
      style.opacity = 0.01;
    }
    a::RunPropertiesChoice::SolidFill(fill) => {
      if let Some(color) = fill
        .solid_fill_choice
        .as_ref()
        .and_then(Color::from_solid_fill_choice)
        .and_then(|color| display_paint(import, &color, None))
      {
        style.color = color.color;
        style.opacity = color.opacity;
      }
    }
    a::RunPropertiesChoice::GradientFill(_)
    | a::RunPropertiesChoice::BlipFill(_)
    | a::RunPropertiesChoice::PatternFill(_)
    | a::RunPropertiesChoice::GroupFill => {}
  }
}

fn apply_default_text_fill(
  import: &PowerPointImport,
  fill: &a::DefaultRunPropertiesChoice,
  style: &mut TextStyle,
) {
  match fill {
    a::DefaultRunPropertiesChoice::NoFill(_) => {
      // Source: LibreOffice sd/qa/unit/import-tests2.cxx:testTdf118776
      // imports text noFill as 99% CharTransparence, not as a dropped run.
      style.color = RgbColor { r: 0, g: 0, b: 0 };
      style.opacity = 0.01;
    }
    a::DefaultRunPropertiesChoice::SolidFill(fill) => {
      if let Some(color) = fill
        .solid_fill_choice
        .as_ref()
        .and_then(Color::from_solid_fill_choice)
        .and_then(|color| display_paint(import, &color, None))
      {
        style.color = color.color;
        style.opacity = color.opacity;
      }
    }
    a::DefaultRunPropertiesChoice::GradientFill(_)
    | a::DefaultRunPropertiesChoice::BlipFill(_)
    | a::DefaultRunPropertiesChoice::PatternFill(_)
    | a::DefaultRunPropertiesChoice::GroupFill => {}
  }
}

#[derive(Clone, Copy, Debug)]
struct DisplayPaint {
  color: RgbColor,
  opacity: f32,
}

#[derive(Clone, Copy, Debug)]
struct DisplayStroke {
  style: BorderStyle,
  opacity: f32,
}

fn fill_paint(import: &PowerPointImport, fill: &FillProperties) -> Option<DisplayPaint> {
  match &fill.kind {
    FillKind::Solid(color) => color
      .as_ref()
      .and_then(|color| display_paint(import, color, fill.placeholder_color.as_ref())),
    FillKind::None
    | FillKind::Group
    | FillKind::Gradient(_)
    | FillKind::Blip(_)
    | FillKind::Pattern(_) => None,
  }
}

fn line_stroke(import: &PowerPointImport, line: &LineProperties) -> Option<DisplayStroke> {
  let LineFill::Solid(color) = &line.fill else {
    return None;
  };
  let paint = color
    .as_ref()
    .and_then(|color| display_paint(import, color, line.placeholder_color.as_ref()))?;
  Some(DisplayStroke {
    style: BorderStyle {
      width_pt: line.width_emu.map(units::emu_to_points).unwrap_or(0.75),
      spacing_pt: 0.0,
      color: paint.color,
      compound: false,
    },
    opacity: paint.opacity,
  })
}

fn display_paint(
  import: &PowerPointImport,
  color: &Color,
  placeholder_color: Option<&Color>,
) -> Option<DisplayPaint> {
  let color = import.resolve_color(color, placeholder_color)?;
  Some(DisplayPaint {
    color: RgbColor {
      r: color.r,
      g: color.g,
      b: color.b,
    },
    opacity: color_opacity(color.alpha),
  })
}

fn display_paint_for_slide(
  import: &PowerPointImport,
  slide: &SlidePersist,
  color: &Color,
  placeholder_color: Option<&Color>,
) -> Option<DisplayPaint> {
  let color = import.resolve_color_for_slide(slide, color, placeholder_color)?;
  Some(DisplayPaint {
    color: RgbColor {
      r: color.r,
      g: color.g,
      b: color.b,
    },
    opacity: color_opacity(color.alpha),
  })
}

fn color_opacity(alpha: i32) -> f32 {
  alpha.clamp(0, 100_000) as f32 / 100_000.0
}
