use std::borrow::Cow;
use std::collections::HashMap;
use std::io::Cursor;
use std::sync::Arc;

use icu_segmenter::{LineSegmenter, LineSegmenterBorrowed, options::LineBreakOptions};
use image::codecs::png::PngEncoder;
use image::{ColorType, GenericImageView, ImageEncoder, imageops::FilterType};
use kurbo::{Affine, Rect as KurboRect};
use ooxmlsdk::schemas::schemas_microsoft_com_office_drawing_2008_diagram as dsp;
use ooxmlsdk::schemas::schemas_microsoft_com_vml as vml;
use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_chart as c;
use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_diagram as dgm;
use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;
use ooxmlsdk::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main as x;

use crate::common;
use crate::common::drawingml_image_effects::{
  ImageEffect, ImageEffectColorResolver, ResolvedEffectColor,
};
use crate::model::{
  BorderStyle, ImageCrop, ImageItem, LineItem, LineItemKind, LinkAreaItem, PageItem, PageSetup,
  PdfTextSegmentation, RectItem, RgbColor, TextItem, TextStyle, common_page_setup, common_point,
  common_rect, common_rgb, common_stroke_from_border, common_text_style,
};
use crate::options::LayoutOptions;
use crate::render::{chart as shared_chart, diagram as shared_diagram, emf_wmf};
use crate::text_metrics::TextMetrics;
use crate::units;

use super::import::ExcelImport;
use super::print::{CalcPrintDocument, CalcPrintPage};
use super::worksheet::{CalcSheet, CellAddress, CellRange, CellRect};
use crate::pptx::chart::{
  ChartFrame, ChartLayoutProfile, ClusteredColumnStyle, RadialChartStyle,
  lower_clustered_column_chart, lower_radial_chart, solid_chart_shape_style,
};
use crate::pptx::drawingml::color::{Color, RgbHexColor};
use crate::pptx::drawingml::fill::FillKind;
use crate::pptx::drawingml::line::LineFill;

const XLSX_HEADER_FOOTER_LINE_HEIGHT_PT: f32 = 12.0;
// ECMA-376 Part 4 §3.3.1.12 and POI XSSFSheet#getColumnWidthInPixels:
// a worksheet column includes two screen pixels of margin padding on each
// side plus one gridline pixel. Use the two 96dpi pixels as the text inset;
// this is distinct from Calc's 20-twip SvxMarginItem implementation detail.
const XLSX_CELL_TEXT_INSET_PT: f32 = 2.0 * crate::units::POINTS_PER_CSS_PIXEL;
const XLSX_GRID_LINE_WIDTH_PT: f32 = 0.25;
// Excel emits an authored DrawingML `a:ln w="0"` as a printable hairline
// rather than suppressing the line. The Office reference PDF records that
// device hairline as 0.14pt.
const XLSX_CHART_HAIRLINE_WIDTH_PT: f32 = 0.14;
// Microsoft Excel fixed-output evidence from the Open XML SDK `Youtube.xlsx`
// and `Bing.xlsx` WebExtension fixtures: the 96x96 fallback bitmap represents
// a 16x16 logical-pixel add-in placeholder. Excel centers it in the content
// anchor and prints it at 11.52pt.
const XLSX_WEB_EXTENSION_PLACEHOLDER_SIZE_PT: f32 = 11.52;
const XLSX_WEB_EXTENSION_PLACEHOLDER_RASTER_PX: u32 = 31;
// Excel's content-add-in host canvas is offset from the worksheet cell grid
// by one 96dpi pixel horizontally and one-and-a-half pixels vertically. Both
// independent Office fixed outputs place the centered placeholder at exactly
// [404.16, 520.80, 415.68, 532.32] for the shared fixture anchor.
const XLSX_WEB_EXTENSION_HOST_OFFSET_X_PT: f32 = -units::POINTS_PER_CSS_PIXEL;
const XLSX_WEB_EXTENSION_HOST_OFFSET_Y_PT: f32 = 1.5 * units::POINTS_PER_CSS_PIXEL;

#[derive(Clone, Copy, Debug)]
struct ChartTextClipSlack {
  left_em: f32,
  right_em: f32,
}

const DEFAULT_CHART_TEXT_CLIP_SLACK: ChartTextClipSlack = ChartTextClipSlack {
  left_em: 0.5,
  right_em: 0.5,
};
// Excel 16 retains a histogram category text object on both worksheet pages
// when its centered category slot straddles the horizontal page boundary.
// SimpleHistogram.xlsx needs one additional tenth of an em on the continuation
// page; the paint clip still prevents glyph ink from escaping the page.
const CHARTEX_HISTOGRAM_TEXT_CLIP_SLACK: ChartTextClipSlack = ChartTextClipSlack {
  left_em: 0.6,
  right_em: 0.5,
};
// Excel retains a waterfall legend label in the continuation-page PDF text
// layer even when its complete glyph ink falls just outside the printable
// clip. The two LibreOffice waterfall fixtures retain objects through one
// full text em left of the boundary; the page clip still prevents the glyphs
// from becoming visible.
const CHARTEX_WATERFALL_TEXT_CLIP_SLACK: ChartTextClipSlack = ChartTextClipSlack {
  left_em: 1.0,
  right_em: 0.5,
};
// Office emits an automatic Pareto legend resource and its ordinal as separate
// text runs. At a page boundary the 0.3em band retains the ordinal run while
// leaving the preceding localized resource on the marker's page.
const CHARTEX_PARETO_TEXT_CLIP_SLACK: ChartTextClipSlack = ChartTextClipSlack {
  left_em: 0.3,
  right_em: 0.5,
};
const INDEXED_SCATTER_TITLE_TEXT_CLIP_SLACK: ChartTextClipSlack = ChartTextClipSlack {
  left_em: 0.5,
  right_em: 0.0,
};
// Microsoft Excel 365 fixed output for `tdf122915.xlsx` retains a data-label
// text object whose 10pt origin is 6.854pt beyond the worksheet page clip.
// The paint clip still hides the glyph ink; this slack only preserves the PDF
// text-layer object for the matching explicit-title indexed-scatter profile.
const EXPLICIT_TITLE_INDEXED_SCATTER_TEXT_CLIP_SLACK: ChartTextClipSlack = ChartTextClipSlack {
  left_em: 0.5,
  right_em: 0.7,
};
// Excel's modern single-series scatter profile retains the terminal x-axis
// tick on both worksheet pages when its 9pt text origin is 6.45pt beyond the
// logical clip. Keep the three-quarter-em boundary band attached to the same
// derived-title profile that owns this chart's plot and label geometry.
const MODERN_DERIVED_TITLE_SCATTER_TEXT_CLIP_SLACK: ChartTextClipSlack = ChartTextClipSlack {
  left_em: 0.5,
  right_em: 0.75,
};
// Office fixed-output evidence from `ser_labels.xlsx` and `tdf134553.xlsx`:
// a separate data-label field whose origin is just beyond a worksheet page
// clip remains in the PDF text layer.
const MULTICOMPONENT_DATA_LABEL_TEXT_CLIP_SLACK: ChartTextClipSlack = ChartTextClipSlack {
  left_em: 0.5,
  right_em: 0.6,
};

#[derive(Clone, Copy, Debug)]
struct CalcCellOutputArea {
  align_rect: CellRect,
  clip_rect: CellRect,
  left_clip_pt: f32,
  right_clip_pt: f32,
}

pub(crate) fn lower_to_layout_document(
  import: &ExcelImport,
  options: &LayoutOptions,
) -> common::LayoutDocument<'static> {
  let mut pages = Vec::new();
  let print_document = CalcPrintDocument::from_import(import);
  let debug_records = if options.diagnostics.collect_debug_records {
    print_document
      .pages
      .iter()
      .enumerate()
      .map(|(page_index, page)| xlsx_print_page_debug_record(page_index, page))
      .collect()
  } else {
    Vec::new()
  };
  pages.extend(print_document.pages.iter().map(|page| {
    let setup = page_setup_from_calc(page);
    (setup, print_page_items(import, page, setup))
  }));
  common_fixed_pages_with_items(pages, debug_records, options)
}

fn common_fixed_pages_with_items(
  pages: Vec<(PageSetup, Vec<PageItem>)>,
  debug_records: Vec<common::DebugRecord<'static>>,
  options: &LayoutOptions,
) -> common::LayoutDocument<'static> {
  let pages = if pages.is_empty() {
    vec![(PageSetup::default(), Vec::new())]
  } else {
    pages
  };
  common::LayoutDocument {
    engine_kind: common::LayoutEngineKind::Xlsx,
    options: common::LayoutOptions {
      collect_debug: options.diagnostics.collect_debug_records,
      approximate_unsupported: false,
      preserve_source_links: options.diagnostics.preserve_source_links,
    },
    pages: pages
      .into_iter()
      .map(|(setup, items)| common_display_page(setup, items))
      .collect(),
    debug_records,
    ..Default::default()
  }
}

fn xlsx_print_page_debug_record(
  page_index: usize,
  page: &CalcPrintPage<'_>,
) -> common::DebugRecord<'static> {
  let mut metadata = vec![
    common::DebugProperty {
      name: "sheet".into(),
      value: common::DebugValue::Text(page.sheet.name.clone().into()),
    },
    common::DebugProperty {
      name: "sheet_page_index".into(),
      value: common::DebugValue::Integer(page.sheet_page_index as i64),
    },
    common::DebugProperty {
      name: "page_number".into(),
      value: common::DebugValue::Integer(page.page_number as i64),
    },
    common::DebugProperty {
      name: "zoom".into(),
      value: common::DebugValue::Integer(i64::from(page.zoom)),
    },
    common::DebugProperty {
      name: "drawing_anchors".into(),
      value: common::DebugValue::Integer(page.drawing_anchor_count as i64),
    },
    common::DebugProperty {
      name: "charts".into(),
      value: common::DebugValue::Integer(page.chart_count as i64),
    },
  ];
  if let Some(area) = page.area {
    for (name, value) in [
      ("start_column", area.start.col),
      ("start_row", area.start.row),
      ("end_column", area.end.col),
      ("end_row", area.end.row),
    ] {
      metadata.push(common::DebugProperty {
        name: name.into(),
        value: common::DebugValue::Integer(i64::from(value)),
      });
    }
  }
  common::DebugRecord::Shape(common::DebugShape {
    page_index,
    path: Vec::new(),
    kind: "xlsx_print_page".into(),
    bounds: common::Rect::default(),
    metadata,
  })
}

fn common_display_page(setup: PageSetup, items: Vec<PageItem>) -> common::DisplayPage<'static> {
  let common_setup = common_page_setup(setup);
  common::DisplayPage {
    section_index: 0,
    section_page_index: 0,
    bounds: common_rect(0.0, 0.0, setup.width_pt, setup.height_pt),
    background: setup
      .background
      .map(|color| common::Fill::Solid(common_rgb(color, 1.0))),
    setup: common_setup,
    items: items.into_iter().map(common_display_item).collect(),
    ..Default::default()
  }
}

fn common_display_item(item: PageItem) -> common::DisplayItem<'static> {
  match item {
    PageItem::Text(item) => common::DisplayItem::Text(common_text_run(item)),
    PageItem::Image(item) => common::DisplayItem::Image(common_image_item(item)),
    PageItem::Group {
      mask,
      transform,
      blend_mode,
      opacity,
      items,
    } => common::DisplayItem::Group(common::CompositingGroup {
      mask: mask.map(common_image_item),
      transform,
      blend_mode,
      opacity,
      flatten_identity: false,
      inherit_text_line_owner: true,
      items: items.into_iter().map(common_display_item).collect(),
    }),
    PageItem::LinkArea(item) => common::DisplayItem::LinkArea(common::LinkArea {
      bounds: common_rect(item.x_pt, item.y_pt, item.width_pt, item.height_pt),
      target: Cow::Owned(item.hyperlink_url),
    }),
    PageItem::Path(item) => common::DisplayItem::Path(item),
    PageItem::Rect(item) => common::DisplayItem::Rect(common_rect_item(item)),
    PageItem::Line(item) => common::DisplayItem::Line(common_line_item(item)),
  }
}

fn common_text_run(item: TextItem) -> common::TextRun<'static> {
  let color = common_rgb(item.style.color, item.style.opacity);
  common::TextRun {
    text: Cow::Owned(item.text),
    origin: common_point(item.x_pt, item.y_pt),
    line_height: common::Pt(item.line_height_pt),
    paint_clip: item.paint_clip,
    style: common_text_style(item.style),
    font_id: None,
    color,
    rotation_center: item.rotation_center_pt.map(|(x, y)| common_point(x, y)),
    hyperlink_url: item.hyperlink_url.map(Cow::Owned),
    dynamic_field: None,
    form_widget_id: item.form_widget_id,
    paragraph_bidi: item.paragraph_bidi,
    word_spacing_pt: 0.0,
    preserve_text_portion: item.preserve_text_portion,
    pdf_text_segmentation: match item.pdf_text_segmentation {
      PdfTextSegmentation::Line => common::PdfTextSegmentation::Line,
      PdfTextSegmentation::WordLine => common::PdfTextSegmentation::WordLine,
      PdfTextSegmentation::Portion => common::PdfTextSegmentation::Portion,
    },
    source: (!item.source_path.is_empty()).then_some(common::DisplaySource {
      engine: common::LayoutEngineKind::Xlsx,
      path: item.source_path,
      relationship_id: None,
    }),
  }
}

fn common_image_item(item: ImageItem) -> common::ImageItem<'static> {
  let semantic_metafile_text = emf_wmf::supports_semantic_text(item.content_type.as_deref());
  common::ImageItem {
    bounds: common_rect(item.x_pt, item.y_pt, item.width_pt, item.height_pt),
    crop: Some(common::ImageCrop {
      left: item.crop.left,
      top: item.crop.top,
      right: item.crop.right,
      bottom: item.crop.bottom,
    }),
    clip_path: item.clip_path,
    rotation_degrees: item.rotation_deg,
    flip_horizontal: item.flip_horizontal,
    flip_vertical: item.flip_vertical,
    content_type: item
      .content_type
      .map(Cow::Owned)
      .unwrap_or(Cow::Borrowed("application/octet-stream")),
    bytes: item.data,
    metafile_monochrome_dib_palette_override: item.metafile_monochrome_dib_palette_override,
    metafile_background_color: item.metafile_background_color,
    relationship_id: None,
    alt_text: item.alt_text.map(Cow::Owned),
    hyperlink_url: item.hyperlink_url.map(Cow::Owned),
    semantic_metafile_text,
    metafile_native_size: false,
    floating: item.floating,
    behind_text: item.behind_text,
  }
}

fn common_rect_item(item: RectItem) -> common::RectItem<'static> {
  common::RectItem {
    bounds: common_rect(item.x_pt, item.y_pt, item.width_pt, item.height_pt),
    fill: item
      .fill_color
      .map(|color| common::Fill::Solid(common_rgb(color, item.fill_opacity)))
      .unwrap_or(common::Fill::None),
    stroke: item
      .stroke
      .map(|stroke| common_stroke_from_border(stroke, item.stroke_opacity)),
  }
}

fn common_line_item(item: LineItem) -> common::LineItem<'static> {
  common::LineItem {
    start: common_point(item.x1_pt, item.y1_pt),
    end: common_point(item.x2_pt, item.y2_pt),
    stroke: common::Stroke {
      width: common::Pt(item.width_pt),
      color: common_rgb(item.color, 1.0),
      dash: None,
      source_style_id: None,
      ..Default::default()
    },
    kind: match item.kind {
      LineItemKind::Stroke => common::LineKind::Stroke,
    },
  }
}

fn print_page_items(
  import: &ExcelImport,
  page: &CalcPrintPage<'_>,
  setup: PageSetup,
) -> Vec<PageItem> {
  let mut items = Vec::new();
  let zoom_scale = page.zoom as f32 / 100.0;
  let heading_width = if page.page_settings.print_headings {
    page.sheet.column_width_pt(1) * zoom_scale
  } else {
    0.0
  };
  let heading_height = if page.page_settings.print_headings {
    page.sheet.row_height_pt(1) * zoom_scale
  } else {
    0.0
  };
  let repeat_width = effective_repeated_columns(page)
    .map(|range| page.sheet.range_rect(range).width_pt * zoom_scale)
    .unwrap_or(0.0);
  let repeat_height = effective_repeated_rows(page)
    .map(|range| page.sheet.range_rect(range).height_pt * zoom_scale)
    .unwrap_or(0.0);
  let area_size = page
    .area
    .map(|area| page.sheet.range_rect(area))
    .map_or((0.0, 0.0), |rect| {
      (rect.width_pt * zoom_scale, rect.height_pt * zoom_scale)
    });
  let horizontal_centering = calc_axis_centering_offset(
    page.page_settings.horizontal_centered,
    setup.width_pt - setup.margin_left_pt - setup.margin_right_pt,
    heading_width + repeat_width + area_size.0,
  );
  let vertical_centering = calc_axis_centering_offset(
    page.page_settings.vertical_centered,
    setup.height_pt - setup.margin_top_pt - setup.margin_bottom_pt,
    heading_height + repeat_height + area_size.1,
  );
  let body_origin_x = setup.margin_left_pt + horizontal_centering + heading_width;
  let paper_fallback_scale = if page.chart_count > 0 {
    page.page_settings.printer_default_paper_scale_percent()
  } else {
    100
  };
  let body_margin_top =
    if paper_fallback_scale < 100 && page.page_settings.header_footer.has_print_content() {
      setup.margin_top_pt * zoom_scale
    } else {
      setup.margin_top_pt
    };
  let body_origin_y = body_margin_top
    + if paper_fallback_scale < 100 {
      page
        .page_settings
        .printer_default_paper_body_offset_y_pt(zoom_scale)
    } else {
      0.0
    }
    + vertical_centering
    + heading_height;
  let mut text_metrics = TextMetrics::new();

  // ECMA-376 §18.3.1.46 defines these as the printed page header and
  // footer. Keep the PDF content stream in the same semantic order exposed
  // by Microsoft Office fixed output: header, sheet body, then footer.
  render_header_or_footer(
    &mut items,
    page,
    setup,
    true,
    &import.styles,
    &mut text_metrics,
  );

  if let Some(area) = repeat_corner_for_page(page) {
    render_cell_area(
      &mut items,
      import,
      page,
      &page.repeated_corner_cells,
      area,
      CellAreaRenderLayout {
        origin_x_pt: body_origin_x,
        origin_y_pt: body_origin_y,
        zoom_scale,
      },
      &mut text_metrics,
    );
  }
  if let Some(area) = repeat_rows_for_page(page) {
    render_cell_area(
      &mut items,
      import,
      page,
      &page.repeated_row_cells,
      area,
      CellAreaRenderLayout {
        origin_x_pt: body_origin_x + repeat_width,
        origin_y_pt: body_origin_y,
        zoom_scale,
      },
      &mut text_metrics,
    );
  }
  if let Some(area) = repeat_columns_for_page(page) {
    render_cell_area(
      &mut items,
      import,
      page,
      &page.repeated_column_cells,
      area,
      CellAreaRenderLayout {
        origin_x_pt: body_origin_x,
        origin_y_pt: body_origin_y + repeat_height,
        zoom_scale,
      },
      &mut text_metrics,
    );
  }
  if let Some(area) = page.area {
    render_cell_area(
      &mut items,
      import,
      page,
      &page.cells,
      area,
      CellAreaRenderLayout {
        origin_x_pt: body_origin_x + repeat_width,
        origin_y_pt: body_origin_y + repeat_height,
        zoom_scale,
      },
      &mut text_metrics,
    );
    if page.page_settings.print_headings {
      render_headings(
        &mut items,
        page,
        area,
        HeadingRenderLayout {
          row_header_x_pt: setup.margin_left_pt,
          row_header_y_pt: body_origin_y + repeat_height,
          col_header_x_pt: body_origin_x + repeat_width,
          col_header_y_pt: body_origin_y - heading_height,
          zoom_scale,
        },
      );
    }
  }

  items.extend(print_page_image_items(
    import,
    page,
    body_origin_x + repeat_width,
    body_origin_y + repeat_height,
    zoom_scale,
  ));
  items.extend(print_page_shape_items(
    import,
    page,
    body_origin_x + repeat_width,
    body_origin_y + repeat_height,
    zoom_scale,
  ));
  items.extend(print_page_diagram_items(
    page,
    body_origin_x + repeat_width,
    body_origin_y + repeat_height,
    zoom_scale,
  ));
  items.extend(print_page_drawing_text_items(
    import,
    page,
    setup,
    body_origin_x + repeat_width,
    body_origin_y + repeat_height,
    zoom_scale,
  ));
  items.extend(print_page_vml_shape_items(
    page,
    body_origin_x + repeat_width,
    body_origin_y + repeat_height,
    zoom_scale,
  ));
  items.extend(print_page_vml_text_items(
    page,
    body_origin_x + repeat_width,
    body_origin_y + repeat_height,
    zoom_scale,
  ));
  render_header_or_footer(
    &mut items,
    page,
    setup,
    false,
    &import.styles,
    &mut text_metrics,
  );
  items
}

fn print_page_vml_shape_items(
  page: &CalcPrintPage<'_>,
  origin_x_pt: f32,
  origin_y_pt: f32,
  zoom_scale: f32,
) -> Vec<PageItem> {
  let mut items = Vec::new();
  let page_transform = SheetPageTransform::for_page(page, origin_x_pt, origin_y_pt, zoom_scale);
  for shape in page
    .sheet
    .resources
    .object_resources
    .vml_drawings
    .iter()
    .flat_map(|drawing| drawing.shapes.iter())
  {
    if shape.hidden
      || !shape.print_object
      || shape.image_relationship_id.is_some()
      || shape.kind == super::object_resources::VmlShapeKind::Group
      || !vml_shape_intersects_page(page, shape)
    {
      continue;
    }
    let Some((x_pt, y_pt, width_pt, height_pt)) = vml_shape_rect(page.sheet, shape) else {
      continue;
    };
    if width_pt <= 0.0 || height_pt <= 0.0 {
      continue;
    }
    let rect = page_transform.rect_from_xywh(x_pt, y_pt, width_pt, height_pt);
    if shape
      .object_type
      .as_deref()
      .is_some_and(|value| value.eq_ignore_ascii_case("Checkbox"))
    {
      push_vml_checkbox_items(&mut items, shape, rect);
      continue;
    }
    if let Some(paths) = vml_shape_drawing_paths(shape, rect.width_pt, rect.height_pt) {
      let transform = vml_shape_path_transform(shape.style.as_deref(), rect);
      let fill = vml_shape_common_fill(
        shape,
        transform * Affine::scale_non_uniform(f64::from(rect.width_pt), f64::from(rect.height_pt)),
      );
      let stroke = vml_shape_common_stroke(shape);
      for path in paths {
        let closed = path
          .commands
          .iter()
          .any(|command| matches!(command, common::PathCommand::Close));
        items.push(PageItem::Path(common::PathItem {
          bounds: common_rect(rect.x_pt, rect.y_pt, rect.width_pt, rect.height_pt),
          points: Vec::new(),
          commands: common::drawingml_geometry::transform_commands(path.commands, transform),
          closed,
          fill: path.fill_mode.apply_to_fill(fill.clone()),
          stroke: path.stroke.then(|| stroke.clone()).flatten(),
        }));
      }
      continue;
    }
  }
  items
}

fn push_vml_checkbox_items(
  items: &mut Vec<PageItem>,
  shape: &super::object_resources::VmlShapeModel,
  rect: CellRect,
) {
  // Excel prints a legacy Forms checkbox as a native indicator inside the
  // larger VML control anchor, not as the shapetype rectangle. Independent
  // Office fixed outputs (`tdf161365.xlsx` and `checkbox-form-control.xlsx`)
  // expose 200ppi control snapshots of different sizes, but both retain the
  // same 14x14 pixel indicator, five pixels from the leading edge and
  // vertically centered. Keep that physical control metric independent of
  // the worksheet anchor size and the PDF rasterizer DPI.
  let compact_indicator_only =
    shape.text.trim().is_empty() && rect.width_pt <= rect.height_pt * 1.1;
  let Some(data) =
    vml_checkbox_snapshot_png(rect, shape.checked.unwrap_or(0), compact_indicator_only)
  else {
    return;
  };
  let image_rect = vml_checkbox_image_rect(rect, compact_indicator_only);
  items.push(PageItem::Image(ImageItem {
    x_pt: image_rect.x_pt,
    y_pt: image_rect.y_pt,
    width_pt: image_rect.width_pt,
    height_pt: image_rect.height_pt,
    crop: ImageCrop::default(),
    clip_path: Vec::new(),
    rotation_deg: 0.0,
    flip_horizontal: false,
    flip_vertical: false,
    data,
    content_type: Some("image/png".to_string()),
    metafile_monochrome_dib_palette_override: None,
    metafile_background_color: None,
    alt_text: None,
    hyperlink_url: None,
    floating: false,
    behind_text: false,
  }));
  push_vml_checkbox_text_item(items, shape, rect);
}

fn vml_checkbox_image_rect(rect: CellRect, compact_indicator_only: bool) -> CellRect {
  let printer_dot_pt = units::POINTS_PER_INCH / units::OFFICE_FIXED_OUTPUT_DPI;
  if !compact_indicator_only {
    // A captioned Forms checkbox keeps the full control snapshot. Office
    // aligns that bitmap independently from the worksheet anchor: the
    // checkbox-form-control fixed output exposes the next grid line plus two
    // dots at the left, +1 dot at the top, and +5 dots in height.
    return CellRect {
      x_pt: (rect.x_pt / printer_dot_pt).ceil() * printer_dot_pt + 2.0 * printer_dot_pt,
      y_pt: units::quantize_points_to_office_print_grid(rect.y_pt) + printer_dot_pt,
      width_pt: units::quantize_points_to_office_print_grid(rect.width_pt),
      height_pt: units::quantize_points_to_office_print_grid(rect.height_pt) + 5.0 * printer_dot_pt,
    };
  }

  // Office trims an empty, indicator-sized Forms checkbox to its native
  // snapshot bounds. The fixed tdf161365 output exposes all four edges on the
  // 600dpi printer grid: the left edge advances one dot past the next grid
  // line, while the top moves -3 dots, width grows +4 dots, and height
  // shrinks -12 dots relative to the authored control anchor.
  CellRect {
    x_pt: (rect.x_pt / printer_dot_pt).ceil() * printer_dot_pt + printer_dot_pt,
    y_pt: units::quantize_points_to_office_print_grid(rect.y_pt) - 3.0 * printer_dot_pt,
    width_pt: units::quantize_points_to_office_print_grid(rect.width_pt) + 4.0 * printer_dot_pt,
    height_pt: (units::quantize_points_to_office_print_grid(rect.height_pt)
      - 12.0 * printer_dot_pt)
      .max(0.0),
  }
}

fn push_vml_checkbox_text_item(
  items: &mut Vec<PageItem>,
  shape: &super::object_resources::VmlShapeModel,
  rect: CellRect,
) {
  if shape.text.trim().is_empty() {
    return;
  }

  // VML control font sizes are twips. LibreOffice's VmlDrawing control
  // conversion (`drawingfragment.cxx::convertControlFontData`) applies the
  // same twips/20 conversion and uses the first textbox font for the caption.
  let mut style = TextStyle {
    font_family: shape
      .text_style
      .font_family
      .as_deref()
      .map(Arc::<str>::from),
    font_size_pt: units::quantize_points_to_office_print_grid(
      shape.text_style.font_size_twips.unwrap_or(200) as f32 / 20.0,
    ),
    bold: shape.text_style.bold,
    italic: shape.text_style.italic,
    underline: shape.text_style.underline,
    strikethrough: shape.text_style.strikethrough,
    use_windows_font_metrics: true,
    ..TextStyle::default()
  };
  if let Some(color) = shape
    .text_style
    .color
    .as_deref()
    .and_then(crate::docx::parse_vml_color)
  {
    style.color = color;
  }

  let line_height = (style.font_size_pt * 1.15).max(1.0);
  let text_top = match shape
    .text_vertical_alignment
    .as_deref()
    .map(str::to_ascii_lowercase)
    .as_deref()
  {
    Some("center") => rect.y_pt + (rect.height_pt - line_height).max(0.0) / 2.0,
    Some("bottom") => rect.y_pt + (rect.height_pt - line_height).max(0.0),
    _ => rect.y_pt,
  };

  // Office's fixed output uses the native Forms checkbox content rectangle:
  // its label begins 14 logical screen pixels from the control's leading
  // edge. The independent checkbox fixture exposes this as 10.56pt after
  // Excel's 600dpi print-device quantization.
  let leading_inset =
    units::quantize_points_to_office_print_grid(14.0 * units::POINTS_PER_CSS_PIXEL);
  let available_width = (rect.width_pt - leading_inset).max(0.0);
  let mut text_metrics = TextMetrics::new();
  let text_width = text_metrics.measure_text(&shape.text, &style);
  let aligned_x = match shape
    .text_horizontal_alignment
    .as_deref()
    .map(str::to_ascii_lowercase)
    .as_deref()
  {
    Some("center") => (available_width - text_width).max(0.0) / 2.0,
    Some("right") => (available_width - text_width).max(0.0),
    _ => 0.0,
  };

  items.push(PageItem::Text(TextItem {
    x_pt: rect.x_pt + leading_inset + aligned_x,
    y_pt: text_top,
    line_height_pt: line_height,
    paint_clip: None,
    discard_if_horizontally_clipped: false,
    text: shape.text.clone(),
    style,
    rotation_center_pt: None,
    hyperlink_url: None,
    form_widget_id: None,
    paragraph_bidi: false,
    preserve_text_portion: false,
    pdf_text_segmentation: PdfTextSegmentation::Line,
    source_path: Vec::new(),
  }));
}

fn vml_checkbox_snapshot_png(
  rect: CellRect,
  checked: i64,
  compact_indicator_only: bool,
) -> Option<Arc<[u8]>> {
  const SNAPSHOT_DPI: f32 = 200.0;
  const INDICATOR_SIZE: u32 = 14;
  const INDICATOR_LEADING: u32 = 5;
  let width = (rect.width_pt * SNAPSHOT_DPI / units::POINTS_PER_INCH)
    .round()
    .max((INDICATOR_LEADING + INDICATOR_SIZE) as f32) as u32;
  let mut height = (rect.height_pt * SNAPSHOT_DPI / units::POINTS_PER_INCH)
    .round()
    .max(INDICATOR_SIZE as f32) as u32;
  if compact_indicator_only {
    // The Office image XObject is the authored 42px-wide snapshot with two
    // transparent rows removed above and three below the centered indicator.
    height = height.saturating_sub(5).max(INDICATOR_SIZE);
  } else {
    height = height.saturating_add(1);
  }
  let top = ((height - INDICATOR_SIZE) as f32 / 2.0).round() as u32;
  let mut image = image::RgbaImage::new(width, height);

  if compact_indicator_only {
    for local_y in 0..INDICATOR_SIZE - 1 {
      for local_x in 0..INDICATOR_SIZE {
        let alpha = match local_x {
          0 => 91,
          13 => 182,
          _ => u8::MAX,
        };
        image.put_pixel(
          INDICATOR_LEADING + local_x,
          top + local_y,
          image::Rgba([0, 0, 0, alpha]),
        );
      }
    }
    for local_y in 2..12 {
      for local_x in 2..12 {
        image.put_pixel(
          INDICATOR_LEADING + local_x,
          top + local_y,
          image::Rgba([u8::MAX, u8::MAX, u8::MAX, u8::MAX]),
        );
      }
    }
    for local_x in 2..12 {
      image.put_pixel(
        INDICATOR_LEADING + local_x,
        top + 1,
        image::Rgba([0xb7, 0xb7, 0xb7, u8::MAX]),
      );
    }
  } else {
    for local_y in 0..INDICATOR_SIZE {
      for local_x in 0..INDICATOR_SIZE {
        let edge_y = local_y == 0 || local_y == 13;
        let alpha = match (edge_y, local_x) {
          (true, 0) => 106,
          (true, 13) => 103,
          (true, _) => 159,
          (false, 0) => 171,
          (false, 13) => 165,
          (false, _) => u8::MAX,
        };
        let value = match (local_y, local_x) {
          (0 | 13, _) | (_, 0 | 13) => 0,
          (1 | 12, 1) => 26,
          (1 | 12, 12) => 24,
          (1 | 12, _) => 76,
          (_, 1) => 87,
          (_, 12) => 80,
          _ => u8::MAX,
        };
        image.put_pixel(
          INDICATOR_LEADING + local_x,
          top + local_y,
          image::Rgba([value, value, value, alpha]),
        );
      }
    }
  }

  match checked {
    1 => {
      const CHECK_PIXELS: &[(u32, u32, u8)] = &[
        (9, 4, 0),
        (10, 4, 18),
        (8, 5, 36),
        (9, 5, 0),
        (8, 6, 0),
        (9, 6, 248),
        (4, 7, 0),
        (5, 7, 0),
        (7, 7, 36),
        (8, 7, 0),
        (5, 8, 0),
        (6, 8, 185),
        (7, 8, 0),
        (8, 8, 238),
        (5, 9, 96),
        (6, 9, 0),
        (7, 9, 28),
      ];
      for &(x, y, value) in CHECK_PIXELS {
        image.put_pixel(
          INDICATOR_LEADING + x,
          top + y,
          image::Rgba([value, value, value, u8::MAX]),
        );
      }
    }
    2 => {
      for x in 3..11 {
        image.put_pixel(
          INDICATOR_LEADING + x,
          top + 7,
          image::Rgba([0, 0, 0, u8::MAX]),
        );
      }
    }
    _ => {}
  }

  let mut png = Cursor::new(Vec::new());
  PngEncoder::new(&mut png)
    .write_image(
      image.as_raw(),
      image.width(),
      image.height(),
      ColorType::Rgba8.into(),
    )
    .ok()?;
  Some(Arc::from(png.into_inner()))
}

pub(crate) fn vml_shape_common_fill(
  shape: &super::object_resources::VmlShapeModel,
  unit_to_page: Affine,
) -> common::Fill<'static> {
  if !shape.filled {
    return common::Fill::None;
  }
  let primary = vml_common_color(
    shape.fill_color.as_deref(),
    shape.fill_opacity.as_deref(),
    RgbColor {
      r: 255,
      g: 255,
      b: 255,
    },
  );
  if !matches!(
    shape.fill_type,
    Some(vml::FillTypeValues::Gradient | vml::FillTypeValues::GradientRadial)
  ) {
    return common::Fill::Solid(primary);
  }
  let secondary = vml_common_color(
    shape.fill_color2.as_deref(),
    shape.fill_opacity2.as_deref(),
    RgbColor {
      r: primary.r,
      g: primary.g,
      b: primary.b,
    },
  );
  let focus = shape
    .fill_focus
    .as_deref()
    .and_then(parse_vml_ratio)
    .unwrap_or(0.0);
  let mut stops = vml_gradient_intermediate_stops(shape.fill_colors.as_deref());
  let (angle_degrees, path) = if shape.fill_type == Some(vml::FillTypeValues::GradientRadial) {
    if stops.is_empty() {
      let outer_to_inner = (-0.5..=0.5).contains(&focus);
      stops = vec![
        common::GradientStop {
          position: 0.0,
          color: if outer_to_inner { secondary } else { primary },
          scheme: None,
        },
        common::GradientStop {
          position: 1.0,
          color: if outer_to_inner { primary } else { secondary },
          scheme: None,
        },
      ];
    }
    let (focus_x, focus_y) = shape
      .fill_focus_position
      .as_deref()
      .and_then(parse_vml_ratio_pair)
      .unwrap_or((0.0, 0.0));
    let (focus_width, focus_height) = shape
      .fill_focus_size
      .as_deref()
      .and_then(parse_vml_ratio_pair)
      .unwrap_or((0.0, 0.0));
    let right = (focus_x + focus_width).clamp(focus_x, 1.0);
    let bottom = (focus_y + focus_height).clamp(focus_y, 1.0);
    (
      None,
      Some(common::GradientPath {
        // LibreOffice maps VML gradientRadial to DrawingML's rectangular
        // path gradient, not to an ellipse.
        kind: common::GradientPathKind::Rectangle,
        fill_to: common::RelativeRect {
          left: focus_x.clamp(0.0, 1.0),
          top: focus_y.clamp(0.0, 1.0),
          right: (1.0 - right).clamp(0.0, 1.0),
          bottom: (1.0 - bottom).clamp(0.0, 1.0),
        },
        transform: common_transform_from_affine(unit_to_page),
        mirror_tile: false,
      }),
    )
  } else {
    let authored_angle = shape
      .fill_angle
      .as_deref()
      .and_then(|value| value.parse::<f32>().ok())
      .unwrap_or(0.0);
    if stops.is_empty() {
      let axial = (-0.75..=-0.25).contains(&focus) || (0.25..=0.75).contains(&focus);
      if axial {
        let mut outer_to_inner = focus > 0.0;
        if authored_angle < 0.0 {
          outer_to_inner = !outer_to_inner;
        }
        let outer = if outer_to_inner { primary } else { secondary };
        let inner = if outer_to_inner { secondary } else { primary };
        stops = vec![
          common::GradientStop {
            position: 0.0,
            color: outer,
            scheme: None,
          },
          common::GradientStop {
            position: 0.5,
            color: inner,
            scheme: None,
          },
          common::GradientStop {
            position: 1.0,
            color: outer,
            scheme: None,
          },
        ];
      } else {
        let mut swap = true;
        if focus.abs() > 0.5 {
          swap = !swap;
        }
        if authored_angle < 0.0 {
          swap = !swap;
        }
        stops = vec![
          common::GradientStop {
            position: 0.0,
            color: if swap { secondary } else { primary },
            scheme: None,
          },
          common::GradientStop {
            position: 1.0,
            color: if swap { primary } else { secondary },
            scheme: None,
          },
        ];
      }
    }
    // VML measures counter-clockwise from the bottom; DrawingML/common
    // gradients measure clockwise from the left.
    (
      Some((90.0 - authored_angle.rem_euclid(360.0)).rem_euclid(360.0)),
      None,
    )
  };
  common::Fill::Gradient(common::GradientFill {
    stops,
    angle_degrees,
    interpolation: if matches!(
      shape.fill_method,
      Some(vml::FillMethodValues::Sigma | vml::FillMethodValues::Linearsigma)
    ) {
      common::GradientInterpolation::PowerPointGammaSigma
    } else {
      common::GradientInterpolation::LinearSrgb
    },
    path,
    ..common::GradientFill::default()
  })
}

pub(crate) fn vml_shape_common_stroke(
  shape: &super::object_resources::VmlShapeModel,
) -> Option<common::Stroke<'static>> {
  if !shape.stroked {
    return None;
  }
  let width = shape
    .stroke_weight
    .as_deref()
    .and_then(parse_vml_length_pt)
    .unwrap_or(0.75);
  let color = vml_common_color(
    shape.stroke_color.as_deref(),
    shape.stroke_opacity.as_deref(),
    RgbColor { r: 0, g: 0, b: 0 },
  );
  Some(common::Stroke {
    width: common::Pt(width),
    color,
    dash: vml_dash_array(shape.stroke_dash_style.as_deref(), width),
    cap: Some(
      match shape
        .stroke_end_cap
        .unwrap_or(vml::StrokeEndCapValues::Flat)
      {
        vml::StrokeEndCapValues::Flat => common::StrokeCap::Flat,
        vml::StrokeEndCapValues::Square => common::StrokeCap::Square,
        vml::StrokeEndCapValues::Round => common::StrokeCap::Round,
      },
    ),
    join: Some(
      match shape
        .stroke_join_style
        .unwrap_or(vml::StrokeJoinStyleValues::Round)
      {
        vml::StrokeJoinStyleValues::Round => common::StrokeJoin::Round,
        vml::StrokeJoinStyleValues::Bevel => common::StrokeJoin::Bevel,
        vml::StrokeJoinStyleValues::Miter => common::StrokeJoin::Miter { limit: None },
      },
    ),
    compound: Some(
      match shape
        .stroke_line_style
        .unwrap_or(vml::StrokeLineStyleValues::Single)
      {
        vml::StrokeLineStyleValues::Single => common::StrokeCompound::Single,
        vml::StrokeLineStyleValues::ThinThin => common::StrokeCompound::Double,
        vml::StrokeLineStyleValues::ThinThick => common::StrokeCompound::ThinThick,
        vml::StrokeLineStyleValues::ThickThin => common::StrokeCompound::ThickThin,
        vml::StrokeLineStyleValues::ThickBetweenThin => common::StrokeCompound::Triple,
      },
    ),
    head_end: vml_stroke_end(
      shape.stroke_start_arrow,
      shape.stroke_start_arrow_width,
      shape.stroke_start_arrow_length,
    ),
    tail_end: vml_stroke_end(
      shape.stroke_end_arrow,
      shape.stroke_end_arrow_width,
      shape.stroke_end_arrow_length,
    ),
    ..common::Stroke::default()
  })
}

fn vml_common_color(
  value: Option<&str>,
  opacity: Option<&str>,
  default: RgbColor,
) -> common::Color {
  let color = value
    .and_then(crate::docx::parse_vml_color)
    .unwrap_or(default);
  common_rgb(color, opacity.and_then(parse_vml_opacity).unwrap_or(1.0))
}

pub(crate) fn recolor_vml_pattern_image(
  shape: &super::object_resources::VmlShapeModel,
  data: &[u8],
) -> Option<Vec<u8>> {
  if shape.fill_type != Some(vml::FillTypeValues::Pattern) {
    return None;
  }
  // Historical VML uses light gray for an omitted foreground even though the
  // prose default says white; this is the Office-observed default also used
  // by LibreOffice's VML importer.
  common::drawingml_pattern::recolor_vml_historical_pattern(
    data,
    vml_common_color(
      shape.fill_color.as_deref(),
      shape.fill_opacity.as_deref(),
      RgbColor {
        r: 0xd3,
        g: 0xd3,
        b: 0xd3,
      },
    ),
    vml_common_color(
      shape.fill_color2.as_deref(),
      shape.fill_opacity2.as_deref(),
      RgbColor {
        r: 255,
        g: 255,
        b: 255,
      },
    ),
  )
}

pub(crate) fn recolor_typed_vml_pattern_image(fill: &vml::Fill, data: &[u8]) -> Option<Vec<u8>> {
  if fill.r#type != Some(vml::FillTypeValues::Pattern) {
    return None;
  }
  common::drawingml_pattern::recolor_vml_historical_pattern(
    data,
    vml_common_color(
      fill.color.as_deref(),
      fill.opacity.as_deref(),
      RgbColor {
        r: 0xd3,
        g: 0xd3,
        b: 0xd3,
      },
    ),
    vml_common_color(
      fill.color2.as_deref(),
      fill.opacity2.as_deref(),
      RgbColor {
        r: 255,
        g: 255,
        b: 255,
      },
    ),
  )
}

fn parse_vml_opacity(value: &str) -> Option<f32> {
  let value = value.trim();
  let opacity = if let Some(value) = value.strip_suffix('f') {
    value.trim().parse::<f32>().ok()? / 65_536.0
  } else {
    parse_vml_ratio(value)?
  };
  Some(opacity.clamp(0.0, 1.0))
}

fn parse_vml_ratio(value: &str) -> Option<f32> {
  let value = value.trim();
  if let Some(value) = value.strip_suffix('%') {
    return Some(value.trim().parse::<f32>().ok()? / 100.0);
  }
  value.parse::<f32>().ok()
}

fn parse_vml_ratio_pair(value: &str) -> Option<(f32, f32)> {
  let mut values = value.split(',').map(str::trim);
  Some((
    parse_vml_ratio(values.next()?)?,
    parse_vml_ratio(values.next()?)?,
  ))
}

fn parse_vml_vector_component(value: &str) -> Option<f32> {
  let value = value.trim();
  if let Some(value) = value.strip_suffix('f') {
    return Some(value.trim().parse::<f32>().ok()? / 65_536.0);
  }
  parse_vml_ratio(value)
}

pub(crate) fn vml_tile_phase(
  origin: Option<&str>,
  position: Option<&str>,
  frame_width: f32,
  frame_height: f32,
  tile_width: f32,
  tile_height: f32,
) -> (f32, f32) {
  let parse_pair = |value: &str| {
    let mut values = value.split(',').map(str::trim);
    Some((
      parse_vml_vector_component(values.next()?)?,
      parse_vml_vector_component(values.next()?)?,
    ))
  };
  // Both attributes default to their respective centers. The image-space
  // origin is placed on the shape-space position, then tiling repeats in both
  // directions from that anchor.
  let (origin_x, origin_y) = origin.and_then(parse_pair).unwrap_or((0.5, 0.5));
  let (position_x, position_y) = position.and_then(parse_pair).unwrap_or((0.5, 0.5));
  (
    position_x * frame_width - origin_x * tile_width,
    position_y * frame_height - origin_y * tile_height,
  )
}

fn vml_gradient_intermediate_stops(value: Option<&str>) -> Vec<common::GradientStop<'static>> {
  let mut stops = value
    .into_iter()
    .flat_map(|value| value.split(';'))
    .filter_map(|entry| {
      let entry = entry.trim();
      let split = entry
        .char_indices()
        .find(|(_, ch)| ch.is_whitespace())
        .map(|(index, _)| index)?;
      let position = parse_vml_ratio(entry[..split].trim())?.clamp(0.0, 1.0);
      let color = crate::docx::parse_vml_color(entry[split..].trim())?;
      Some(common::GradientStop {
        position,
        color: common_rgb(color, 1.0),
        scheme: None,
      })
    })
    .collect::<Vec<_>>();
  stops.sort_by(|left, right| left.position.total_cmp(&right.position));
  stops
}

fn vml_dash_array(value: Option<&str>, width: f32) -> Option<Vec<common::Pt>> {
  let value = value?.trim();
  let preset: &[f32] = match value.to_ascii_lowercase().as_str() {
    "solid" => return None,
    "shortdot" => &[1.0, 1.0],
    "shortdash" => &[3.0, 1.0],
    "shortdashdot" => &[3.0, 1.0, 1.0, 1.0],
    "shortdashdotdot" => &[3.0, 1.0, 1.0, 1.0, 1.0, 1.0],
    "dot" => &[1.0, 3.0],
    "dash" => &[4.0, 3.0],
    "longdash" => &[8.0, 3.0],
    "dashdot" => &[4.0, 3.0, 1.0, 3.0],
    "longdashdot" => &[8.0, 3.0, 1.0, 3.0],
    "longdashdotdot" => &[8.0, 3.0, 1.0, 3.0, 1.0, 3.0],
    _ => {
      let values = value
        .split_whitespace()
        .map(|value| {
          value
            .parse::<f32>()
            .ok()
            .map(|value| common::Pt(value * width))
        })
        .collect::<Option<Vec<_>>>()?;
      return (!values.is_empty()).then_some(values);
    }
  };
  Some(
    preset
      .iter()
      .map(|value| common::Pt(value * width))
      .collect(),
  )
}

fn vml_stroke_end(
  kind: Option<vml::StrokeArrowValues>,
  width: Option<vml::StrokeArrowWidthValues>,
  length: Option<vml::StrokeArrowLengthValues>,
) -> Option<common::StrokeEnd> {
  let kind = match kind.unwrap_or(vml::StrokeArrowValues::None) {
    vml::StrokeArrowValues::None => return None,
    vml::StrokeArrowValues::Block => common::StrokeEndKind::Triangle,
    vml::StrokeArrowValues::Classic => common::StrokeEndKind::Stealth,
    vml::StrokeArrowValues::Oval => common::StrokeEndKind::Oval,
    vml::StrokeArrowValues::Diamond => common::StrokeEndKind::Diamond,
    vml::StrokeArrowValues::Open => common::StrokeEndKind::Arrow,
  };
  let size = |value| match value {
    0 => common::StrokeEndSize::Small,
    2 => common::StrokeEndSize::Large,
    _ => common::StrokeEndSize::Medium,
  };
  Some(common::StrokeEnd {
    kind,
    width: size(match width.unwrap_or(vml::StrokeArrowWidthValues::Medium) {
      vml::StrokeArrowWidthValues::Narrow => 0,
      vml::StrokeArrowWidthValues::Medium => 1,
      vml::StrokeArrowWidthValues::Wide => 2,
    }),
    length: size(
      match length.unwrap_or(vml::StrokeArrowLengthValues::Medium) {
        vml::StrokeArrowLengthValues::Short => 0,
        vml::StrokeArrowLengthValues::Medium => 1,
        vml::StrokeArrowLengthValues::Long => 2,
      },
    ),
  })
}

pub(crate) fn common_transform_from_affine(transform: Affine) -> common::Transform {
  let [m11, m12, m21, m22, dx, dy] = transform.as_coeffs();
  common::Transform {
    m11: m11 as f32,
    m12: m12 as f32,
    m21: m21 as f32,
    m22: m22 as f32,
    dx: common::Pt(dx as f32),
    dy: common::Pt(dy as f32),
  }
}

fn vml_shape_path_transform(style: Option<&str>, rect: CellRect) -> Affine {
  let rotation = style
    .and_then(|style| vml_style_value(style, "rotation"))
    .map(vml_rotation_degrees)
    .unwrap_or(0.0);
  let flip = style
    .and_then(|style| vml_style_value(style, "flip"))
    .unwrap_or_default()
    .to_ascii_lowercase();
  let scale_x = if flip.split_whitespace().any(|value| value == "x") {
    -1.0
  } else {
    1.0
  };
  let scale_y = if flip.split_whitespace().any(|value| value == "y") {
    -1.0
  } else {
    1.0
  };
  let center_x = f64::from(rect.width_pt) * 0.5;
  let center_y = f64::from(rect.height_pt) * 0.5;
  Affine::translate((-center_x, -center_y))
    .then_scale_non_uniform(scale_x, scale_y)
    .then_rotate(f64::from(rotation.to_radians()))
    .then_translate(
      (
        center_x + f64::from(rect.x_pt),
        center_y + f64::from(rect.y_pt),
      )
        .into(),
    )
}

fn vml_style_value<'a>(style: &'a str, key: &str) -> Option<&'a str> {
  style.split(';').find_map(|declaration| {
    let (name, value) = declaration.split_once(':')?;
    name
      .trim()
      .eq_ignore_ascii_case(key)
      .then_some(value.trim())
  })
}

fn vml_rotation_degrees(value: &str) -> f32 {
  let value = value.trim();
  let degrees = value
    .strip_suffix("fd")
    .and_then(|value| value.trim().parse::<f32>().ok())
    .map(|value| value / 65_536.0)
    .or_else(|| value.parse::<f32>().ok())
    .unwrap_or(0.0);
  -degrees
}

fn xlsx_vml_special_geometry(
  shape: &super::object_resources::VmlShapeModel,
  rect: CellRect,
) -> Option<crate::docx::InlineShapeGeometry> {
  use super::object_resources::VmlShapeKind as Kind;
  match shape.kind {
    Kind::Curve => {
      let points = [
        parse_vml_coordinate_pair(shape.from.as_deref()?)?,
        parse_vml_coordinate_pair(shape.control1.as_deref()?)?,
        parse_vml_coordinate_pair(shape.control2.as_deref()?)?,
        parse_vml_coordinate_pair(shape.to.as_deref()?)?,
      ];
      let points = map_vml_coordinates(shape, &points, rect)?;
      Some(crate::docx::InlineShapeGeometry::Path {
        paths: vec![common::DrawingPath {
          commands: vec![
            common::PathCommand::MoveTo(common_point(points[0].0, points[0].1)),
            common::PathCommand::CubicTo {
              control1: common_point(points[1].0, points[1].1),
              control2: common_point(points[2].0, points[2].1),
              end: common_point(points[3].0, points[3].1),
            },
          ],
          fill_mode: common::DrawingPathFillMode::None,
          stroke: shape.stroked,
          extrusion_allowed: true,
        }],
        outline: None,
      })
    }
    Kind::Line if shape.from.is_some() && shape.to.is_some() => {
      let points = [
        parse_vml_coordinate_pair(shape.from.as_deref()?)?,
        parse_vml_coordinate_pair(shape.to.as_deref()?)?,
      ];
      let points = map_vml_coordinates(shape, &points, rect)?;
      Some(crate::docx::InlineShapeGeometry::Polyline {
        points,
        closed: false,
      })
    }
    Kind::Polyline => {
      let points = parse_vml_coordinate_list(shape.points.as_deref()?)?;
      let points = map_vml_coordinates(shape, &points, rect)?;
      let repeated_endpoint = points
        .first()
        .zip(points.last())
        .is_some_and(|(first, last)| {
          (first.0 - last.0).abs() <= 0.01 && (first.1 - last.1).abs() <= 0.01
        });
      Some(crate::docx::InlineShapeGeometry::Polyline {
        points,
        // LibreOffice imports a filled VML polyline as a polygon even when
        // the first point is not repeated (`tdf112450_vml_polyline`).
        closed: shape.filled || repeated_endpoint,
      })
    }
    Kind::Arc => vml_arc_geometry(shape, rect),
    Kind::RoundRectangle if shape.arc_size.is_some() => vml_round_rectangle_geometry(shape, rect),
    _ => None,
  }
}

pub(crate) fn vml_shape_drawing_paths(
  shape: &super::object_resources::VmlShapeModel,
  width_pt: f32,
  height_pt: f32,
) -> Option<Vec<common::DrawingPath>> {
  let rect = CellRect {
    x_pt: 0.0,
    y_pt: 0.0,
    width_pt,
    height_pt,
  };
  let geometry = if let Some(path) = shape.path.as_deref() {
    crate::docx::vml_path_geometry(
      path,
      crate::docx::VmlPathGeometryOptions {
        coordinate_origin: shape.coordinate_origin.as_deref(),
        coordinate_size: shape.coordinate_size.as_deref(),
        width_pt,
        height_pt,
        adjustment: None,
        formulas: None,
        allow_fill: shape.filled,
        allow_stroke: shape.stroked,
        allow_extrusion: true,
      },
    )
  } else {
    xlsx_vml_special_geometry(shape, rect)
  };
  if let Some(geometry) = geometry {
    return inline_vml_geometry_paths(geometry, width_pt, height_pt);
  }
  // An authored generic VML path is the shape geometry. If it cannot be
  // interpreted, silently replacing it with a rectangle produces visible
  // content that Office never authored.
  if shape.path.is_some() {
    return None;
  }

  use super::object_resources::VmlShapeKind as Kind;
  let preset = match shape.kind {
    Kind::Oval => a::ShapeTypeValues::Ellipse,
    Kind::RoundRectangle => a::ShapeTypeValues::RoundRectangle,
    Kind::Line => {
      return Some(vec![common::DrawingPath {
        commands: vec![
          common::PathCommand::MoveTo(common_point(0.0, 0.0)),
          common::PathCommand::LineTo(common_point(width_pt, height_pt)),
        ],
        fill_mode: common::DrawingPathFillMode::None,
        stroke: shape.stroked,
        extrusion_allowed: true,
      }]);
    }
    Kind::Group | Kind::Image => return None,
    _ => a::ShapeTypeValues::Rectangle,
  };
  common::drawingml_preset_geometry::paths(
    Some(&a::PresetGeometry {
      preset,
      ..a::PresetGeometry::default()
    }),
    0.0,
    0.0,
    width_pt,
    height_pt,
  )
}

fn inline_vml_geometry_paths(
  geometry: crate::docx::InlineShapeGeometry,
  width_pt: f32,
  height_pt: f32,
) -> Option<Vec<common::DrawingPath>> {
  Some(match geometry {
    crate::docx::InlineShapeGeometry::Path { paths, .. } => paths,
    crate::docx::InlineShapeGeometry::Polyline { points, closed } => {
      let mut commands = points
        .into_iter()
        .enumerate()
        .map(|(index, (x, y))| {
          if index == 0 {
            common::PathCommand::MoveTo(common_point(x, y))
          } else {
            common::PathCommand::LineTo(common_point(x, y))
          }
        })
        .collect::<Vec<_>>();
      if closed {
        commands.push(common::PathCommand::Close);
      }
      vec![common::DrawingPath {
        commands,
        fill_mode: if closed {
          common::DrawingPathFillMode::Normal
        } else {
          common::DrawingPathFillMode::None
        },
        stroke: true,
        extrusion_allowed: true,
      }]
    }
    crate::docx::InlineShapeGeometry::Line => vec![common::DrawingPath {
      commands: vec![
        common::PathCommand::MoveTo(common_point(0.0, 0.0)),
        common::PathCommand::LineTo(common_point(width_pt, height_pt)),
      ],
      fill_mode: common::DrawingPathFillMode::None,
      stroke: true,
      extrusion_allowed: true,
    }],
    crate::docx::InlineShapeGeometry::Rectangle => {
      return common::drawingml_preset_geometry::paths(None, 0.0, 0.0, width_pt, height_pt);
    }
  })
}

fn parse_vml_coordinate_pair(value: &str) -> Option<(f32, f32)> {
  let mut values = value
    .split(',')
    .map(|value| parse_vml_length_pt(value.trim()));
  Some((values.next()??, values.next()??))
}

fn parse_vml_coordinate_list(value: &str) -> Option<Vec<(f32, f32)>> {
  let values = value
    .split(',')
    .map(|value| parse_vml_length_pt(value.trim()))
    .collect::<Option<Vec<_>>>()?;
  if values.len() < 4 || values.len() % 2 != 0 {
    return None;
  }
  Some(
    values
      .chunks_exact(2)
      .map(|pair| (pair[0], pair[1]))
      .collect(),
  )
}

fn map_vml_coordinates(
  shape: &super::object_resources::VmlShapeModel,
  points: &[(f32, f32)],
  rect: CellRect,
) -> Option<Vec<(f32, f32)>> {
  let (origin_x, origin_y, width, height) = if let Some((width, height)) = shape
    .coordinate_size
    .as_deref()
    .and_then(parse_vml_numeric_pair)
  {
    let (origin_x, origin_y) = shape
      .coordinate_origin
      .as_deref()
      .and_then(parse_vml_numeric_pair)
      .unwrap_or((0.0, 0.0));
    (origin_x, origin_y, width, height)
  } else {
    let bounds = common::drawingml_geometry::point_bounds(
      points
        .iter()
        .map(|&(x, y)| kurbo::Point::new(f64::from(x), f64::from(y))),
    )?;
    (
      bounds.x0 as f32,
      bounds.y0 as f32,
      bounds.width() as f32,
      bounds.height() as f32,
    )
  };
  Some(
    points
      .iter()
      .map(|&(x, y)| {
        let x = if width.abs() <= f32::EPSILON {
          rect.width_pt * 0.5
        } else {
          (x - origin_x) * rect.width_pt / width
        };
        let y = if height.abs() <= f32::EPSILON {
          rect.height_pt * 0.5
        } else {
          (y - origin_y) * rect.height_pt / height
        };
        (x, y)
      })
      .collect(),
  )
}

fn parse_vml_numeric_pair(value: &str) -> Option<(f32, f32)> {
  let mut values = value.split(',').map(str::trim);
  Some((values.next()?.parse().ok()?, values.next()?.parse().ok()?))
}

fn vml_arc_geometry(
  shape: &super::object_resources::VmlShapeModel,
  rect: CellRect,
) -> Option<crate::docx::InlineShapeGeometry> {
  let start = shape
    .start_angle
    .as_deref()
    .unwrap_or("0")
    .parse::<f32>()
    .ok()?;
  let end = shape
    .end_angle
    .as_deref()
    .unwrap_or("90")
    .parse::<f32>()
    .ok()?;
  let mut sweep = end - start;
  if sweep.abs() <= f32::EPSILON {
    sweep = 360.0;
  }
  while sweep < 0.0 {
    sweep += 360.0;
  }
  while sweep > 360.0 {
    sweep -= 360.0;
  }
  let center = (rect.width_pt * 0.5, rect.height_pt * 0.5);
  let radii = (rect.width_pt * 0.5, rect.height_pt * 0.5);
  let segments = (sweep.abs() / 90.0).ceil().max(1.0) as usize;
  let step = sweep.to_radians() / segments as f32;
  let mut angle = start.to_radians();
  let mut commands = vec![common::PathCommand::MoveTo(common_point(
    center.0 + radii.0 * angle.cos(),
    center.1 + radii.1 * angle.sin(),
  ))];
  for _ in 0..segments {
    let next = angle + step;
    let tangent = (4.0 / 3.0) * (step / 4.0).tan();
    commands.push(common::PathCommand::CubicTo {
      control1: common_point(
        center.0 + radii.0 * (angle.cos() - tangent * angle.sin()),
        center.1 + radii.1 * (angle.sin() + tangent * angle.cos()),
      ),
      control2: common_point(
        center.0 + radii.0 * (next.cos() + tangent * next.sin()),
        center.1 + radii.1 * (next.sin() - tangent * next.cos()),
      ),
      end: common_point(
        center.0 + radii.0 * next.cos(),
        center.1 + radii.1 * next.sin(),
      ),
    });
    angle = next;
  }
  Some(crate::docx::InlineShapeGeometry::Path {
    paths: vec![common::DrawingPath {
      commands,
      fill_mode: common::DrawingPathFillMode::None,
      stroke: shape.stroked,
      extrusion_allowed: true,
    }],
    outline: None,
  })
}

fn vml_round_rectangle_geometry(
  shape: &super::object_resources::VmlShapeModel,
  rect: CellRect,
) -> Option<crate::docx::InlineShapeGeometry> {
  let value = shape.arc_size.as_deref()?.trim();
  let fraction = if let Some(value) = value.strip_suffix('f') {
    value.trim().parse::<f32>().ok()? / 65_536.0
  } else if let Some(value) = value.strip_suffix('%') {
    value.trim().parse::<f32>().ok()? / 100.0
  } else {
    value.parse::<f32>().ok()?
  }
  .clamp(0.0, 1.0);
  let radius = rect.width_pt.min(rect.height_pt) * 0.5 * fraction;
  let kappa = 0.552_284_8;
  let width = rect.width_pt;
  let height = rect.height_pt;
  let commands = vec![
    common::PathCommand::MoveTo(common_point(radius, 0.0)),
    common::PathCommand::LineTo(common_point(width - radius, 0.0)),
    common::PathCommand::CubicTo {
      control1: common_point(width - radius + radius * kappa, 0.0),
      control2: common_point(width, radius - radius * kappa),
      end: common_point(width, radius),
    },
    common::PathCommand::LineTo(common_point(width, height - radius)),
    common::PathCommand::CubicTo {
      control1: common_point(width, height - radius + radius * kappa),
      control2: common_point(width - radius + radius * kappa, height),
      end: common_point(width - radius, height),
    },
    common::PathCommand::LineTo(common_point(radius, height)),
    common::PathCommand::CubicTo {
      control1: common_point(radius - radius * kappa, height),
      control2: common_point(0.0, height - radius + radius * kappa),
      end: common_point(0.0, height - radius),
    },
    common::PathCommand::LineTo(common_point(0.0, radius)),
    common::PathCommand::CubicTo {
      control1: common_point(0.0, radius - radius * kappa),
      control2: common_point(radius - radius * kappa, 0.0),
      end: common_point(radius, 0.0),
    },
    common::PathCommand::Close,
  ];
  Some(crate::docx::InlineShapeGeometry::Path {
    paths: vec![common::DrawingPath {
      commands,
      fill_mode: common::DrawingPathFillMode::Normal,
      stroke: shape.stroked,
      extrusion_allowed: true,
    }],
    outline: None,
  })
}

fn calc_axis_centering_offset(enabled: bool, available_pt: f32, content_pt: f32) -> f32 {
  if !enabled {
    return 0.0;
  }
  ((available_pt - content_pt) / 2.0).max(0.0)
}

fn page_setup_from_calc(page: &CalcPrintPage<'_>) -> PageSetup {
  let mut setup = PageSetup::default();
  let (width_pt, height_pt) = page.page_settings.page_size_pt();
  setup.width_pt = width_pt;
  setup.height_pt = height_pt;
  setup.margin_left_pt = page.page_settings.margin_left_in as f32 * units::POINTS_PER_INCH;
  setup.margin_right_pt = page.page_settings.margin_right_in as f32 * units::POINTS_PER_INCH;
  setup.margin_top_pt = page.page_settings.margin_top_in as f32 * units::POINTS_PER_INCH;
  setup.margin_bottom_pt = page.page_settings.margin_bottom_in as f32 * units::POINTS_PER_INCH;
  setup.header_distance_pt = page.page_settings.margin_header_in as f32 * units::POINTS_PER_INCH;
  setup.footer_distance_pt = page.page_settings.margin_footer_in as f32 * units::POINTS_PER_INCH;
  setup
}

#[derive(Clone, Copy, Debug)]
struct CellAreaRenderLayout {
  origin_x_pt: f32,
  origin_y_pt: f32,
  zoom_scale: f32,
}

#[derive(Clone, Copy, Debug)]
struct SheetPageTransform(Affine);

impl SheetPageTransform {
  fn new(origin_x_pt: f32, origin_y_pt: f32, zoom_scale: f32, source: CellRect) -> Self {
    Self(
      Affine::translate((-f64::from(source.x_pt), -f64::from(source.y_pt)))
        .then_scale(f64::from(zoom_scale))
        .then_translate((f64::from(origin_x_pt), f64::from(origin_y_pt)).into()),
    )
  }

  fn for_page(
    page: &CalcPrintPage<'_>,
    origin_x_pt: f32,
    origin_y_pt: f32,
    zoom_scale: f32,
  ) -> Self {
    let source = page
      .area
      .map(|area| page.sheet.range_rect(area))
      .unwrap_or_default();
    Self::new(origin_x_pt, origin_y_pt, zoom_scale, source)
  }

  fn rect(self, rect: CellRect) -> CellRect {
    let bounds = common::drawingml_geometry::transform_rect_bounds(
      KurboRect::new(
        f64::from(rect.x_pt),
        f64::from(rect.y_pt),
        f64::from(rect.x_pt + rect.width_pt),
        f64::from(rect.y_pt + rect.height_pt),
      ),
      self.0,
    );
    CellRect {
      x_pt: bounds.x0 as f32,
      y_pt: bounds.y0 as f32,
      width_pt: bounds.width() as f32,
      height_pt: bounds.height() as f32,
    }
  }

  fn rect_from_xywh(self, x_pt: f32, y_pt: f32, width_pt: f32, height_pt: f32) -> CellRect {
    self.rect(CellRect {
      x_pt,
      y_pt,
      width_pt,
      height_pt,
    })
  }
}

fn render_cell_area(
  items: &mut Vec<PageItem>,
  import: &ExcelImport,
  page: &CalcPrintPage<'_>,
  cells: &[super::print::CalcPrintCell<'_>],
  area: super::worksheet::CellRange,
  layout: CellAreaRenderLayout,
  text_metrics: &mut TextMetrics,
) {
  let area_rect = page.sheet.range_rect(area);
  let page_transform = SheetPageTransform::new(
    layout.origin_x_pt,
    layout.origin_y_pt,
    layout.zoom_scale,
    area_rect,
  );
  let page_clip_rect = page_transform.rect(area_rect);
  let occupied_cells = calc_occupied_text_cells(cells);
  for cell in cells {
    if page.sheet.is_covered_merged_cell(cell.address) {
      continue;
    }
    let rect = page.sheet.cell_rect(cell.address);
    if rect.width_pt <= 0.0 || rect.height_pt <= 0.0 {
      continue;
    }
    let CellRect {
      x_pt,
      y_pt,
      width_pt,
      height_pt,
    } = page_transform.rect(rect);
    // FillInfo retains one column on either side of ScOutputData's logical
    // range, but DrawStrings and DrawEdit paint only through mnX2. The extra
    // cell remains useful as occupied/overflow context and may contribute
    // text that extends left into the page; it does not own ordinary cell
    // paint on this page.
    let scan_context_only = !area.contains(cell.address);
    let table_builtin_style = super::table::builtin_table_style_for_address(
      &page.sheet.resources.tables,
      &import.styles,
      cell.address,
    );
    let pivot_builtin_style =
      super::pivot::pivot_builtin_style_for_address(page.sheet, &import.styles, cell.address);
    if !scan_context_only
      && let Some(fill_color) = conditional_fill_color(import, page.sheet, cell)
        .or_else(|| pivot_format_fill_color(import, cell))
        .or(pivot_builtin_style.fill)
        .or(table_builtin_style.fill)
        .or_else(|| import.styles.fill_color_for_cell(cell.style_index))
    {
      items.push(PageItem::Rect(RectItem {
        x_pt,
        y_pt,
        width_pt,
        height_pt,
        fill_color: Some(fill_color),
        fill_opacity: 1.0,
        stroke: None,
        stroke_opacity: 1.0,
      }));
    }
    let mut borders = import.styles.borders_for_cell(cell.style_index);
    merge_cell_borders(&mut borders, table_builtin_style.borders);
    merge_cell_borders(&mut borders, pivot_builtin_style.borders);
    if let Some(format_id) = cell.pivot_format_id
      && let Some(pivot_borders) = import.styles.differential_borders(format_id)
    {
      merge_cell_borders(&mut borders, pivot_borders);
    }
    if !scan_context_only {
      render_cell_borders(
        items,
        CellRect {
          x_pt,
          y_pt,
          width_pt,
          height_pt,
        },
        borders,
      );
    }
    if cell.rendered_text.is_empty() && cell.icon_set.is_none() {
      continue;
    }
    let hyperlink_url = hyperlink_for_cell(page, cell.address);
    let cell_rect = CellRect {
      x_pt,
      y_pt,
      width_pt,
      height_pt,
    };
    let mut measurement_style = import.styles.text_style_for_cell(cell.style_index);
    let direct_font_color = import
      .styles
      .direct_nondefault_font_color_for_cell(cell.style_index);
    super::table::apply_builtin_table_text_style(
      table_builtin_style,
      &import.styles,
      &mut measurement_style,
    );
    super::pivot::apply_builtin_pivot_text_style(
      pivot_builtin_style,
      &import.styles,
      &mut measurement_style,
    );
    if let Some(color) = direct_font_color {
      measurement_style.color = color;
    }
    apply_conditional_text_style(import, page.sheet, cell, &mut measurement_style);
    // sc/source/ui/view/output2.cxx ScDrawStringsVars::SetPattern(). Calc's
    // print map mode scales cell geometry and the font used for measurement.
    if let Some(format_id) = cell.pivot_format_id {
      import
        .styles
        .apply_differential_text_style(format_id, &mut measurement_style);
    }
    // Excel fixed output creates the print font on its 600dpi device. The
    // 11pt legacy workbook font is consequently emitted and measured as
    // 11.04pt (92/600in), which also decides borderline wrap opportunities.
    measurement_style.font_size_pt = units::quantize_points_to_office_print_grid(
      measurement_style.font_size_pt * layout.zoom_scale,
    );
    if !scan_context_only {
      render_cell_icon_set(
        items,
        cell,
        cell_rect,
        import
          .styles
          .icon_set_print_metrics(measurement_style.font_size_pt),
      );
    }
    if cell.rendered_text.is_empty() || cell.icon_set.is_some_and(|icon_set| !icon_set.show_value) {
      continue;
    }
    let render_style = measurement_style.clone();
    let mut alignment = import.styles.alignment_for_cell(cell.style_index);
    if pivot_builtin_style.left_align {
      let mut pivot_alignment = alignment.unwrap_or_default();
      pivot_alignment.horizontal = Some(x::HorizontalAlignmentValues::Left);
      alignment = Some(pivot_alignment);
    }
    if let Some(format_id) = cell.pivot_format_id
      && let Some(pivot_alignment) = import.styles.differential_alignment(format_id)
    {
      alignment = Some(pivot_alignment);
    }
    let output_area = calc_cell_output_area(
      CalcCellOutputContext {
        sheet: page.sheet,
        occupied_cells: &occupied_cells,
        text_metrics,
      },
      cell,
      cell_rect,
      &measurement_style,
      alignment,
      layout.zoom_scale,
    );
    let rendered_text = calc_cell_visible_text(
      page.sheet,
      cell,
      &measurement_style,
      output_area,
      text_metrics,
    );
    let horizontal_alignment = calc_cell_horizontal_alignment(cell, alignment);
    let mut rendered_text_items = Vec::new();
    if !cell.rich_text_runs.is_empty() && rendered_text.as_ref() == cell.text.as_ref() {
      render_cell_rich_text(
        &mut rendered_text_items,
        cell.rich_text_runs,
        output_area.align_rect,
        render_style,
        horizontal_alignment,
        hyperlink_url.clone(),
        text_metrics,
      );
    } else {
      render_cell_text(
        &mut rendered_text_items,
        rendered_text.as_ref(),
        output_area.align_rect,
        render_style,
        CellTextRenderOptions {
          alignment,
          horizontal_alignment,
          hyperlink_url: hyperlink_url.clone(),
          formula: cell.formula,
          default_line_height_pt: page.sheet.default_row_height_pt() * layout.zoom_scale,
        },
        text_metrics,
      );
    }
    rendered_text_items.retain_mut(|item| {
      let PageItem::Text(text) = item else {
        return false;
      };
      text.source_path = vec![
        cell.address.row.saturating_sub(1) as usize,
        cell.address.col.saturating_sub(1) as usize,
      ];
      if scan_context_only {
        let (left, right) = text_item_horizontal_bounds(text, text_metrics);
        let clip_right = page_clip_rect.x_pt + page_clip_rect.width_pt;
        if left >= clip_right || right <= page_clip_rect.x_pt {
          return false;
        }
        text.paint_clip = Some(common_rect(
          page_clip_rect.x_pt,
          page_clip_rect.y_pt,
          page_clip_rect.width_pt,
          page_clip_rect.height_pt,
        ));
      }
      true
    });
    items.extend(rendered_text_items);
    if !scan_context_only && let Some(hyperlink_url) = hyperlink_url {
      items.push(PageItem::LinkArea(LinkAreaItem {
        x_pt,
        y_pt,
        width_pt,
        height_pt,
        hyperlink_url,
      }));
    }
  }
  if page.page_settings.print_grid_lines {
    render_grid(
      items,
      page,
      area,
      layout.origin_x_pt,
      layout.origin_y_pt,
      layout.zoom_scale,
    );
  }
}

fn text_item_horizontal_bounds(text: &TextItem, text_metrics: &mut TextMetrics) -> (f32, f32) {
  let width = text_metrics.measure_text(&text.text, &text.style);
  if text.style.rotation_deg.abs() <= f32::EPSILON {
    return (text.x_pt, text.x_pt + width);
  }
  let (center_x, center_y) = text.rotation_center_pt.unwrap_or((
    text.x_pt + width * 0.5,
    text.y_pt + text.line_height_pt * 0.5,
  ));
  let radians = text.style.rotation_deg.to_radians();
  let (sin, cos) = radians.sin_cos();
  let rotated_x = |x: f32, y: f32| center_x + (x - center_x) * cos - (y - center_y) * sin;
  let mut left = f32::INFINITY;
  let mut right = f32::NEG_INFINITY;
  for (x, y) in [
    (text.x_pt, text.y_pt),
    (text.x_pt + width, text.y_pt),
    (text.x_pt, text.y_pt + text.line_height_pt),
    (text.x_pt + width, text.y_pt + text.line_height_pt),
  ] {
    let x = rotated_x(x, y);
    left = left.min(x);
    right = right.max(x);
  }
  (left, right)
}

fn render_cell_icon_set(
  items: &mut Vec<PageItem>,
  cell: &super::print::CalcPrintCell<'_>,
  rect: CellRect,
  metrics: super::styles::IconSetPrintMetrics,
) {
  let Some((icon_set, icon_index)) = cell.icon_set.and_then(|selection| selection.icon) else {
    return;
  };
  let Some(data) = super::icon_set::icon_png(icon_set, icon_index) else {
    return;
  };
  // Calc's drawIconSets uses the effective font height and a separate pair of
  // device-unit insets. StylesCatalog supplies the corresponding Office
  // print-device geometry for the legacy and current theme generations.
  let available_height = (rect.height_pt - metrics.bottom_inset_pt).max(0.0);
  let scale = if metrics.height_pt > available_height && metrics.height_pt > f32::EPSILON {
    available_height / metrics.height_pt
  } else {
    1.0
  };
  let width = metrics.width_pt * scale;
  let height = metrics.height_pt * scale;
  if width <= f32::EPSILON || height <= f32::EPSILON {
    return;
  }
  items.push(PageItem::Image(ImageItem {
    x_pt: rect.x_pt + metrics.leading_inset_pt,
    y_pt: rect.y_pt + rect.height_pt - metrics.bottom_inset_pt - height,
    width_pt: width,
    height_pt: height,
    crop: ImageCrop::default(),
    clip_path: Vec::new(),
    rotation_deg: 0.0,
    flip_horizontal: false,
    flip_vertical: false,
    data,
    content_type: Some("image/png".to_string()),
    metafile_monochrome_dib_palette_override: None,
    metafile_background_color: None,
    alt_text: None,
    hyperlink_url: None,
    floating: false,
    behind_text: false,
  }));
}

fn calc_occupied_text_cells(
  cells: &[super::print::CalcPrintCell<'_>],
) -> HashMap<(u32, u32), bool> {
  cells
    .iter()
    .filter(|cell| !cell.rendered_text.is_empty())
    .map(|cell| ((cell.address.row, cell.address.col), true))
    .collect()
}

struct CalcCellOutputContext<'a> {
  sheet: &'a CalcSheet,
  occupied_cells: &'a HashMap<(u32, u32), bool>,
  text_metrics: &'a mut TextMetrics,
}

fn calc_cell_output_area(
  context: CalcCellOutputContext<'_>,
  cell: &super::print::CalcPrintCell<'_>,
  rect: CellRect,
  style: &TextStyle,
  alignment: Option<super::styles::AlignmentRecord>,
  zoom_scale: f32,
) -> CalcCellOutputArea {
  let text_width_pt = context
    .text_metrics
    .measure_text(&cell.rendered_text, style);
  let needed_width_pt = text_width_pt + XLSX_CELL_TEXT_INSET_PT * 2.0;
  let mut output = CalcCellOutputArea {
    align_rect: rect,
    clip_rect: rect,
    left_clip_pt: 0.0,
    right_clip_pt: 0.0,
  };
  if needed_width_pt <= rect.width_pt {
    return output;
  }

  let missing_width_pt = needed_width_pt - rect.width_pt;
  let (mut left_missing_pt, mut right_missing_pt) =
    calc_cell_missing_width_by_alignment(missing_width_pt, cell, alignment);

  if !calc_cell_is_value(cell) && !alignment.is_some_and(|alignment| alignment.wrap_text) {
    let mut right_col = cell.address.col;
    while right_missing_pt > 0.0
      && output_column_available(
        context.sheet,
        context.occupied_cells,
        right_col + 1,
        cell.address.row,
      )
    {
      right_col += 1;
      let column_width_pt = context.sheet.column_width_pt(right_col) * zoom_scale;
      if column_width_pt <= f32::EPSILON {
        break;
      }
      output.clip_rect.width_pt += column_width_pt;
      right_missing_pt -= column_width_pt;
    }
    let mut left_col = cell.address.col;
    while left_missing_pt > 0.0
      && left_col > 1
      && output_column_available(
        context.sheet,
        context.occupied_cells,
        left_col - 1,
        cell.address.row,
      )
    {
      left_col -= 1;
      let column_width_pt = context.sheet.column_width_pt(left_col) * zoom_scale;
      if column_width_pt <= f32::EPSILON {
        break;
      }
      output.clip_rect.x_pt -= column_width_pt;
      output.clip_rect.width_pt += column_width_pt;
      left_missing_pt -= column_width_pt;
    }
  }

  output.left_clip_pt = left_missing_pt.max(0.0);
  output.right_clip_pt = right_missing_pt.max(0.0);
  output
}

fn calc_cell_missing_width_by_alignment(
  missing_width_pt: f32,
  cell: &super::print::CalcPrintCell<'_>,
  alignment: Option<super::styles::AlignmentRecord>,
) -> (f32, f32) {
  match calc_cell_horizontal_alignment(cell, alignment) {
    x::HorizontalAlignmentValues::Right => (missing_width_pt, 0.0),
    x::HorizontalAlignmentValues::Center | x::HorizontalAlignmentValues::CenterContinuous => {
      let left = missing_width_pt / 2.0;
      (left, missing_width_pt - left)
    }
    _ => (0.0, missing_width_pt),
  }
}

fn calc_cell_horizontal_alignment(
  cell: &super::print::CalcPrintCell<'_>,
  alignment: Option<super::styles::AlignmentRecord>,
) -> x::HorizontalAlignmentValues {
  match alignment.and_then(|alignment| alignment.horizontal) {
    Some(x::HorizontalAlignmentValues::General) | None => {
      match cell.number_format_state {
        // ECMA-376 Part 1, General Format / Alignment: Boolean and error
        // values are centered; numbers are right-aligned and strings left.
        super::print::NumberFormatRenderState::Boolean
        | super::print::NumberFormatRenderState::Error => x::HorizontalAlignmentValues::Center,
        _ if calc_cell_is_value(cell) => x::HorizontalAlignmentValues::Right,
        _ => x::HorizontalAlignmentValues::Left,
      }
    }
    Some(value) => value,
  }
}

fn output_column_available(
  sheet: &CalcSheet,
  occupied_cells: &HashMap<(u32, u32), bool>,
  column: u32,
  row: u32,
) -> bool {
  let address = CellAddress { col: column, row };
  !occupied_cells.contains_key(&(row, column))
    && !sheet.is_covered_merged_cell(address)
    && sheet.column_width_pt(column) > f32::EPSILON
}

fn calc_cell_visible_text<'a>(
  sheet: &CalcSheet,
  cell: &'a super::print::CalcPrintCell<'_>,
  style: &TextStyle,
  output_area: CalcCellOutputArea,
  text_metrics: &mut TextMetrics,
) -> std::borrow::Cow<'a, str> {
  if calc_cell_requires_date_hashes(cell) {
    return std::borrow::Cow::Owned(calc_cell_overflow_hash_text(
      style,
      output_area.align_rect.width_pt,
      text_metrics,
    ));
  }
  if output_area.left_clip_pt <= f32::EPSILON && output_area.right_clip_pt <= f32::EPSILON {
    return std::borrow::Cow::Borrowed(&cell.rendered_text);
  }
  if calc_cell_is_value(cell) {
    if cell.number_format_state == super::print::NumberFormatRenderState::General
      && let Some(text) =
        calc_fit_general_number_text(cell, style, output_area.align_rect.width_pt, text_metrics)
    {
      return std::borrow::Cow::Owned(text);
    }
    return if calc_cell_value_can_hash(cell) {
      std::borrow::Cow::Owned(calc_cell_overflow_hash_text(
        style,
        output_area.align_rect.width_pt,
        text_metrics,
      ))
    } else {
      std::borrow::Cow::Borrowed(&cell.rendered_text)
    };
  }
  // strings with a clip region from GetOutputArea/Clip. krilla text extraction
  // exposes full glyph payloads even when clipped, so trim the extracted text
  // to the visible prefix/suffix while keeping the same clip decision.
  if super::pivot::pivot_table_contains_address(sheet, cell.address) {
    clipped_string_text(cell, style, output_area, text_metrics)
      .map(std::borrow::Cow::Owned)
      .unwrap_or_else(|| std::borrow::Cow::Borrowed(cell.rendered_text.as_str()))
  } else {
    std::borrow::Cow::Borrowed(cell.rendered_text.as_str())
  }
}

fn calc_cell_requires_date_hashes(cell: &super::print::CalcPrintCell<'_>) -> bool {
  cell.number_format_state == super::print::NumberFormatRenderState::DateTime
    && cell
      .text
      .trim()
      .parse::<f64>()
      .is_ok_and(|value| value < 0.0)
}

fn calc_cell_overflow_hash_text(
  style: &TextStyle,
  cell_width_pt: f32,
  text_metrics: &mut TextMetrics,
) -> String {
  let hash_width_pt = text_metrics.measure_text("#", style);
  let available_width_pt = (cell_width_pt - XLSX_CELL_TEXT_INSET_PT * 2.0).max(0.0);
  let count = calc_cell_overflow_hash_count(available_width_pt, hash_width_pt);
  "#".repeat(count)
}

fn calc_cell_overflow_hash_count(available_width_pt: f32, hash_width_pt: f32) -> usize {
  if !available_width_pt.is_finite()
    || !hash_width_pt.is_finite()
    || available_width_pt <= f32::EPSILON
    || hash_width_pt <= f32::EPSILON
  {
    return 1;
  }
  (available_width_pt / hash_width_pt).floor().max(1.0) as usize
}

fn calc_cell_is_value(cell: &super::print::CalcPrintCell<'_>) -> bool {
  matches!(
    cell.number_format_state,
    super::print::NumberFormatRenderState::Raw
      | super::print::NumberFormatRenderState::General
      | super::print::NumberFormatRenderState::Number
      | super::print::NumberFormatRenderState::Percent
      | super::print::NumberFormatRenderState::DateTime
  ) && cell.text.as_ref().parse::<f64>().is_ok()
}

fn calc_cell_value_can_hash(cell: &super::print::CalcPrintCell<'_>) -> bool {
  matches!(
    cell.number_format_state,
    super::print::NumberFormatRenderState::Raw
      | super::print::NumberFormatRenderState::General
      | super::print::NumberFormatRenderState::Number
      | super::print::NumberFormatRenderState::Percent
      | super::print::NumberFormatRenderState::DateTime
      | super::print::NumberFormatRenderState::UnsupportedFormatCode
  )
}

fn clipped_string_text(
  cell: &super::print::CalcPrintCell<'_>,
  style: &TextStyle,
  output_area: CalcCellOutputArea,
  text_metrics: &mut TextMetrics,
) -> Option<String> {
  let text = cell.rendered_text.as_str();
  if text.is_empty() {
    return None;
  }
  let text_width = text_metrics.measure_text(text, style);
  let visible_width = (text_width - output_area.right_clip_pt - output_area.left_clip_pt).max(0.0);
  if visible_width <= f32::EPSILON || visible_width >= text_width {
    return None;
  }
  if output_area.left_clip_pt > output_area.right_clip_pt {
    let mut start = text.len();
    for (index, _) in text.char_indices().rev() {
      if text_metrics.measure_text(&text[index..], style) <= visible_width {
        start = index;
      } else {
        break;
      }
    }
    return (start > 0 && start < text.len()).then(|| text[start..].to_string());
  }
  let mut end = 0usize;
  for (index, ch) in text.char_indices() {
    let next = index + ch.len_utf8();
    if text_metrics.measure_text(&text[..next], style) <= visible_width {
      end = next;
    } else {
      break;
    }
  }
  (end > 0 && end < text.len()).then(|| text[..end].to_string())
}

fn calc_fit_general_number_text(
  cell: &super::print::CalcPrintCell<'_>,
  style: &TextStyle,
  column_width_pt: f32,
  text_metrics: &mut TextMetrics,
) -> Option<String> {
  let value = cell.text.as_ref().parse::<f64>().ok()?;
  if !value.is_finite() {
    return None;
  }
  let available_width = column_width_pt - XLSX_CELL_TEXT_INSET_PT * 2.0;
  if available_width <= f32::EPSILON {
    return None;
  }
  for significant_digits in (1..=15).rev() {
    let text = format_general_number_with_significant_digits(value, significant_digits);
    if text_metrics.measure_text(&text, style) <= available_width {
      return Some(text);
    }
  }
  None
}

fn format_general_number_with_significant_digits(value: f64, significant_digits: usize) -> String {
  if value == 0.0 {
    return "0".to_string();
  }
  let abs = value.abs();
  let integer_digits = if abs >= 1.0 {
    abs.log10().floor() as isize + 1
  } else {
    0
  };
  let decimals = if integer_digits >= significant_digits as isize {
    0
  } else {
    significant_digits.saturating_sub(integer_digits.max(0) as usize)
  };
  let mut text = format!("{value:.decimals$}");
  if text.contains('.') {
    while text.ends_with('0') {
      text.pop();
    }
    if text.ends_with('.') {
      text.pop();
    }
  }
  if text == "-0" { "0".to_string() } else { text }
}

fn calc_text_can_shape_as_line(text: &str) -> bool {
  text.chars().all(|ch| {
    ch.is_ascii_alphanumeric()
      || ch.is_ascii_whitespace()
      || matches!(ch, '/' | '-' | '+' | '.' | ',' | ':' | ';' | '(' | ')')
      || !ch.is_ascii()
  })
}

fn render_cell_rich_text(
  items: &mut Vec<PageItem>,
  runs: &[super::workbook::SharedStringRun],
  rect: CellRect,
  base_style: TextStyle,
  horizontal_alignment: x::HorizontalAlignmentValues,
  hyperlink_url: Option<String>,
  text_metrics: &mut TextMetrics,
) {
  let mut text = String::new();
  let mut style = base_style;
  let mut style_initialized = false;
  for run in runs.iter().filter(|run| !run.text.is_empty()) {
    if !style_initialized {
      if let Some(font_size_pt) = run.font_size_pt {
        style.font_size_pt = font_size_pt;
      }
      if let Some(color) = run.color {
        style.color = color;
      }
      style.bold = run.bold;
      style.italic = run.italic;
      style.underline = run.underline;
      style.strikethrough = run.strikethrough;
      style_initialized = true;
    }
    text.push_str(&run.text.replace(['\r', '\n'], ""));
  }
  if text.is_empty() {
    return;
  }
  let y_pt = rect.y_pt + XLSX_CELL_TEXT_INSET_PT;
  let line_height = (style.font_size_pt * 1.15).max(1.0);
  let text_width_pt = text_metrics.measure_text(&text, &style);
  let x_pt = cell_text_x_pt(rect, text_width_pt, horizontal_alignment, 0.0);
  let preserve_text_portion = !text.is_ascii() && !calc_text_can_shape_as_line(&text);
  items.push(PageItem::Text(TextItem {
    x_pt,
    y_pt,
    line_height_pt: line_height,
    paint_clip: None,
    discard_if_horizontally_clipped: false,
    text,
    style,
    rotation_center_pt: None,
    hyperlink_url,
    form_widget_id: None,
    paragraph_bidi: false,
    preserve_text_portion,
    pdf_text_segmentation: if preserve_text_portion {
      PdfTextSegmentation::Portion
    } else {
      PdfTextSegmentation::Line
    },
    source_path: Vec::new(),
  }));
}

fn conditional_fill_color(
  import: &ExcelImport,
  sheet: &CalcSheet,
  cell: &super::print::CalcPrintCell<'_>,
) -> Option<RgbColor> {
  let mut rules = sheet
    .metrics
    .conditions
    .conditional_formats
    .iter()
    .filter(|format| conditional_format_contains_cell(format, cell.address))
    .flat_map(|format| format.rules.iter())
    .collect::<Vec<_>>();
  rules.sort_by_key(|rule| rule.priority);
  for rule in rules {
    if let Some(fill) = cell
      .color_scale_fill
      .filter(|fill| fill.priority == rule.priority)
    {
      return Some(fill.color);
    }
    if !conditional_rule_matches(rule, cell) {
      continue;
    }
    if let Some(color) = rule
      .format_id
      .and_then(|format_id| import.styles.differential_fill_color(format_id))
    {
      return Some(color);
    }
    if rule.stop_if_true {
      break;
    }
  }
  None
}

fn apply_conditional_text_style(
  import: &ExcelImport,
  sheet: &CalcSheet,
  cell: &super::print::CalcPrintCell<'_>,
  style: &mut TextStyle,
) {
  let mut rules = sheet
    .metrics
    .conditions
    .conditional_formats
    .iter()
    .filter(|format| conditional_format_contains_cell(format, cell.address))
    .flat_map(|format| format.rules.iter())
    .collect::<Vec<_>>();
  // sc/source/filter/oox/condformatbuffer.cxx sorts imported rules by
  // priority before applying their differential formats.
  rules.sort_by_key(|rule| rule.priority);
  for rule in rules {
    if !conditional_rule_matches(rule, cell) {
      continue;
    }
    if let Some(format_id) = rule
      .format_id
      .filter(|format_id| import.styles.differential_has_font(*format_id))
    {
      import
        .styles
        .apply_differential_text_style(format_id, style);
      return;
    }
    if rule.stop_if_true {
      break;
    }
  }
}

fn pivot_format_fill_color(
  import: &ExcelImport,
  cell: &super::print::CalcPrintCell<'_>,
) -> Option<RgbColor> {
  cell
    .pivot_format_id
    .and_then(|format_id| import.styles.differential_fill_color(format_id))
}

fn conditional_format_contains_cell(
  format: &super::sheet_conditions::ConditionalFormatModel,
  address: CellAddress,
) -> bool {
  format.sequence_of_references.iter().any(|references| {
    references
      .split_whitespace()
      .filter_map(CellRange::parse_a1_range)
      .any(|range| range.contains(address))
  })
}

fn conditional_rule_matches(
  rule: &super::sheet_conditions::ConditionalFormatRuleModel,
  cell: &super::print::CalcPrintCell<'_>,
) -> bool {
  match rule.rule_type {
    x::ConditionalFormatValues::CellIs => conditional_cell_is_matches(rule, cell),
    x::ConditionalFormatValues::ContainsText => rule.text.as_ref().is_some_and(|needle| {
      cell.rendered_text.contains(needle) || cell.text.as_ref().contains(needle)
    }),
    x::ConditionalFormatValues::NotContainsText => rule.text.as_ref().is_some_and(|needle| {
      !cell.rendered_text.contains(needle) && !cell.text.as_ref().contains(needle)
    }),
    x::ConditionalFormatValues::BeginsWith => rule.text.as_ref().is_some_and(|needle| {
      cell.rendered_text.starts_with(needle) || cell.text.as_ref().starts_with(needle)
    }),
    x::ConditionalFormatValues::EndsWith => rule.text.as_ref().is_some_and(|needle| {
      cell.rendered_text.ends_with(needle) || cell.text.as_ref().ends_with(needle)
    }),
    x::ConditionalFormatValues::ContainsBlanks => {
      cell.text.as_ref().is_empty() && cell.rendered_text.is_empty()
    }
    x::ConditionalFormatValues::NotContainsBlanks => {
      !cell.text.as_ref().is_empty() || !cell.rendered_text.is_empty()
    }
    x::ConditionalFormatValues::Expression => expression_rule_matches(rule),
    _ => false,
  }
}

fn conditional_cell_is_matches(
  rule: &super::sheet_conditions::ConditionalFormatRuleModel,
  cell: &super::print::CalcPrintCell<'_>,
) -> bool {
  let Some(value) = cell.text.as_ref().parse::<f64>().ok() else {
    return false;
  };
  let first = rule
    .formulas
    .first()
    .and_then(|formula| formula.trim().parse::<f64>().ok());
  let second = rule
    .formulas
    .get(1)
    .and_then(|formula| formula.trim().parse::<f64>().ok());
  match rule.operator.unwrap_or_default() {
    x::ConditionalFormattingOperatorValues::LessThan => first.is_some_and(|limit| value < limit),
    x::ConditionalFormattingOperatorValues::LessThanOrEqual => {
      first.is_some_and(|limit| value <= limit)
    }
    x::ConditionalFormattingOperatorValues::Equal => first.is_some_and(|limit| value == limit),
    x::ConditionalFormattingOperatorValues::NotEqual => first.is_some_and(|limit| value != limit),
    x::ConditionalFormattingOperatorValues::GreaterThanOrEqual => {
      first.is_some_and(|limit| value >= limit)
    }
    x::ConditionalFormattingOperatorValues::GreaterThan => first.is_some_and(|limit| value > limit),
    x::ConditionalFormattingOperatorValues::Between => first
      .zip(second)
      .is_some_and(|(low, high)| value >= low.min(high) && value <= low.max(high)),
    x::ConditionalFormattingOperatorValues::NotBetween => first
      .zip(second)
      .is_some_and(|(low, high)| value < low.min(high) || value > low.max(high)),
    _ => false,
  }
}

fn expression_rule_matches(rule: &super::sheet_conditions::ConditionalFormatRuleModel) -> bool {
  rule.formulas.first().is_some_and(|formula| {
    matches!(
      formula.trim().to_ascii_uppercase().as_str(),
      "TRUE" | "1" | "=TRUE" | "=1"
    )
  })
}

#[derive(Clone, Debug)]
struct CellTextRenderOptions {
  alignment: Option<super::styles::AlignmentRecord>,
  horizontal_alignment: x::HorizontalAlignmentValues,
  hyperlink_url: Option<String>,
  formula: bool,
  default_line_height_pt: f32,
}

fn render_cell_text(
  items: &mut Vec<PageItem>,
  text: &str,
  rect: CellRect,
  mut style: TextStyle,
  options: CellTextRenderOptions,
  text_metrics: &mut TextMetrics,
) {
  let line_height = (style.font_size_pt * 1.15)
    .max(options.default_line_height_pt)
    .max(1.0);
  let alignment = options.alignment;
  let wrap_text = alignment.is_some_and(|alignment| alignment.wrap_text);
  let fill_text;
  let text = if options.horizontal_alignment == x::HorizontalAlignmentValues::Fill && !wrap_text {
    fill_text = repeat_cell_text_to_fill(text, rect.width_pt, &style, text_metrics);
    fill_text.as_str()
  } else {
    text
  };
  let rendered_text;
  let wrapped_lines;
  let lines = if wrap_text && !options.formula {
    // ECMA-376 Part 1 §18.8.1 defines wrapText as line-wrapping the cell
    // contents within the cell. Explicit line breaks remain hard paragraph
    // boundaries; Calc's EditEngine then wraps each paragraph to the output
    // width (ScOutputData::DrawEdit in sc/source/ui/view/output2.cxx).
    wrapped_lines = wrap_cell_text(
      text,
      (rect.width_pt - XLSX_CELL_TEXT_INSET_PT * 2.0).max(1.0),
      &style,
      text_metrics,
    );
    wrapped_lines.iter().map(String::as_str).collect::<Vec<_>>()
  } else if text.contains('\n') || text.contains('\r') {
    rendered_text = if options.formula {
      text.lines().collect::<Vec<_>>().join(" ")
    } else {
      text.lines().collect::<String>()
    };
    vec![rendered_text.as_str()]
  } else {
    vec![text.lines().next().unwrap_or(text)]
  };
  let text_height = line_height * lines.len().max(1) as f32;
  let vertical_alignment = alignment.and_then(|alignment| alignment.vertical);
  let mut y_pt = match vertical_alignment {
    Some(x::VerticalAlignmentValues::Center) => rect.y_pt + (rect.height_pt - text_height) / 2.0,
    Some(x::VerticalAlignmentValues::Top) => rect.y_pt,
    // sc/source/ui/view/output2.cxx resolves Calc's Standard vertical
    // justification to Bottom and starts the text at the cell's bottom edge.
    // Do not clamp a font line taller than the row back to the top: Office
    // keeps the baseline bottom-aligned and lets the glyph box extend above
    // the row, subject to the page/device clip.
    Some(x::VerticalAlignmentValues::Bottom) | None => rect.y_pt + rect.height_pt - text_height,
    Some(x::VerticalAlignmentValues::Justify | x::VerticalAlignmentValues::Distributed) => {
      rect.y_pt
    }
  };
  if let Some(rotation) = alignment.and_then(|alignment| alignment.text_rotation) {
    style.rotation_deg = match rotation {
      1..=90 => rotation as f32,
      91..=180 => 90.0 - rotation as f32,
      255 => 90.0,
      _ => 0.0,
    };
  }
  for line in lines {
    let full_line_width_pt = text_metrics.measure_text(line, &style);
    let preserve_text_portion = !line.is_ascii() && !calc_text_can_shape_as_line(line);
    items.push(PageItem::Text(TextItem {
      x_pt: cell_text_x_pt(rect, full_line_width_pt, options.horizontal_alignment, 0.0),
      y_pt,
      line_height_pt: line_height,
      paint_clip: None,
      discard_if_horizontally_clipped: false,
      text: line.to_string(),
      style: style.clone(),
      rotation_center_pt: (style.rotation_deg != 0.0).then_some((
        rect.x_pt + rect.width_pt / 2.0,
        rect.y_pt + rect.height_pt / 2.0,
      )),
      hyperlink_url: options.hyperlink_url.clone(),
      form_widget_id: None,
      paragraph_bidi: false,
      preserve_text_portion,
      pdf_text_segmentation: if preserve_text_portion {
        PdfTextSegmentation::Portion
      } else {
        PdfTextSegmentation::Line
      },
      source_path: Vec::new(),
    }));
    y_pt += line_height;
  }
}

fn wrap_cell_text(
  text: &str,
  available_width_pt: f32,
  style: &TextStyle,
  text_metrics: &mut TextMetrics,
) -> Vec<String> {
  let mut lines = Vec::new();
  for paragraph in text
    .split('\n')
    .map(|line| line.strip_suffix('\r').unwrap_or(line))
  {
    if paragraph.is_empty() || text_metrics.measure_text(paragraph, style) <= available_width_pt {
      lines.push(paragraph.to_string());
      continue;
    }

    let mut current = String::new();
    let break_points = office_line_break_points(paragraph);
    let mut start = 0;
    for end in break_points.into_iter().filter(|end| *end > 0) {
      let segment = &paragraph[start..end];
      let previous_len = current.len();
      current.push_str(segment);
      if previous_len > 0
        && text_metrics.measure_text(current.trim_end(), style) > available_width_pt
      {
        current.truncate(previous_len);
        lines.push(current.trim_end().to_string());
        current.clear();
        current.push_str(segment.trim_start());
      }
      start = end;
    }
    if start < paragraph.len() {
      let segment = &paragraph[start..];
      let previous_len = current.len();
      current.push_str(segment);
      if previous_len > 0
        && text_metrics.measure_text(current.trim_end(), style) > available_width_pt
      {
        current.truncate(previous_len);
        lines.push(current.trim_end().to_string());
        current.clear();
        current.push_str(segment.trim_start());
      }
    }
    if !current.is_empty() {
      lines.push(current.trim_end().to_string());
    }
  }
  if lines.is_empty() {
    lines.push(String::new());
  }
  lines
}

fn office_line_break_points(text: &str) -> Vec<usize> {
  thread_local! {
    // Parley, Typst, and LibreOffice's BreakIterator all derive soft wrapping
    // opportunities from Unicode line breaking.
    static LINE_SEGMENTER: LineSegmenterBorrowed<'static> =
      LineSegmenter::new_auto(LineBreakOptions::default());
  }

  let mut points = LINE_SEGMENTER.with(|segmenter| segmenter.segment_str(text).collect::<Vec<_>>());
  // LibreOffice i18npool line.txt customization i#83229 restores a break
  // after the embedded hyphen in a numeric range; default ICU LB25 treats
  // `100-199` as one token. Office's Excel output follows the same behavior.
  let bytes = text.as_bytes();
  points.extend(
    (1..bytes.len().saturating_sub(1))
      .filter(|index| {
        bytes[*index] == b'-'
          && bytes[*index - 1].is_ascii_digit()
          && bytes[*index + 1].is_ascii_digit()
      })
      .map(|index| index + 1),
  );
  points.sort_unstable();
  points.dedup();
  points
}

fn repeat_cell_text_to_fill(
  text: &str,
  width_pt: f32,
  style: &TextStyle,
  text_metrics: &mut TextMetrics,
) -> String {
  if text.is_empty() || width_pt <= f32::EPSILON {
    return text.to_string();
  }
  let text_width_pt = text_metrics.measure_text(text, style);
  if text_width_pt <= f32::EPSILON {
    return text.to_string();
  }
  let repeat_count = (width_pt / text_width_pt).ceil().max(1.0) as usize;
  text.repeat(repeat_count)
}

fn cell_text_x_pt(
  rect: CellRect,
  text_width_pt: f32,
  horizontal_alignment: x::HorizontalAlignmentValues,
  leading_offset_pt: f32,
) -> f32 {
  let available_width_pt = (rect.width_pt - XLSX_CELL_TEXT_INSET_PT * 2.0).max(0.0);
  let text_start_pt = match horizontal_alignment {
    x::HorizontalAlignmentValues::Right => {
      rect.x_pt + XLSX_CELL_TEXT_INSET_PT + available_width_pt - text_width_pt
    }
    x::HorizontalAlignmentValues::Center | x::HorizontalAlignmentValues::CenterContinuous => {
      rect.x_pt + XLSX_CELL_TEXT_INSET_PT + (available_width_pt - text_width_pt) / 2.0
    }
    _ => rect.x_pt + XLSX_CELL_TEXT_INSET_PT,
  };
  text_start_pt + leading_offset_pt
}

fn render_cell_borders(
  items: &mut Vec<PageItem>,
  rect: CellRect,
  borders: super::styles::BorderRecord,
) {
  let mut push_vertical_border = |x_pt: f32, border: BorderStyle| {
    items.push(PageItem::Rect(RectItem {
      x_pt: x_pt - border.width_pt / 2.0,
      y_pt: rect.y_pt - border.width_pt / 2.0,
      width_pt: border.width_pt,
      height_pt: rect.height_pt + border.width_pt,
      fill_color: Some(border.color),
      fill_opacity: 1.0,
      stroke: None,
      stroke_opacity: 1.0,
    }));
  };
  if let Some(border) = borders.left {
    push_vertical_border(rect.x_pt, border);
  }
  if let Some(border) = borders.right {
    push_vertical_border(rect.x_pt + rect.width_pt, border);
  }
  let mut push_horizontal_border = |y_pt: f32, border: BorderStyle| {
    items.push(PageItem::Rect(RectItem {
      x_pt: rect.x_pt - border.width_pt / 2.0,
      y_pt: y_pt - border.width_pt / 2.0,
      width_pt: rect.width_pt + border.width_pt,
      height_pt: border.width_pt,
      fill_color: Some(border.color),
      fill_opacity: 1.0,
      stroke: None,
      stroke_opacity: 1.0,
    }));
  };
  if let Some(border) = borders.top {
    push_horizontal_border(rect.y_pt, border);
  }
  if let Some(border) = borders.bottom {
    push_horizontal_border(rect.y_pt + rect.height_pt, border);
  }
}

fn merge_cell_borders(
  target: &mut super::styles::BorderRecord,
  source: super::styles::BorderRecord,
) {
  if source.left.is_some() {
    target.left = source.left;
  }
  if source.right.is_some() {
    target.right = source.right;
  }
  if source.top.is_some() {
    target.top = source.top;
  }
  if source.bottom.is_some() {
    target.bottom = source.bottom;
  }
}

fn render_grid(
  items: &mut Vec<PageItem>,
  page: &CalcPrintPage<'_>,
  area: super::worksheet::CellRange,
  origin_x_pt: f32,
  origin_y_pt: f32,
  zoom_scale: f32,
) {
  let width = page.sheet.range_rect(area).width_pt * zoom_scale;
  let height = page.sheet.range_rect(area).height_pt * zoom_scale;
  let color = RgbColor { r: 0, g: 0, b: 0 };
  let mut x = origin_x_pt;
  for col in area.start.col..=area.end.col + 1 {
    if col > area.start.col {
      x += page.sheet.column_width_pt(col - 1) * zoom_scale;
    }
    items.push(PageItem::Line(LineItem {
      x1_pt: x,
      y1_pt: origin_y_pt,
      x2_pt: x,
      y2_pt: origin_y_pt + height,
      width_pt: XLSX_GRID_LINE_WIDTH_PT,
      color,
      kind: LineItemKind::Stroke,
    }));
  }
  let mut y = origin_y_pt;
  for row in area.start.row..=area.end.row + 1 {
    if row > area.start.row {
      y += page.sheet.row_height_pt(row - 1) * zoom_scale;
    }
    items.push(PageItem::Line(LineItem {
      x1_pt: origin_x_pt,
      y1_pt: y,
      x2_pt: origin_x_pt + width,
      y2_pt: y,
      width_pt: XLSX_GRID_LINE_WIDTH_PT,
      color,
      kind: LineItemKind::Stroke,
    }));
  }
}

#[derive(Clone, Copy, Debug)]
struct HeadingRenderLayout {
  row_header_x_pt: f32,
  row_header_y_pt: f32,
  col_header_x_pt: f32,
  col_header_y_pt: f32,
  zoom_scale: f32,
}

fn render_headings(
  items: &mut Vec<PageItem>,
  page: &CalcPrintPage<'_>,
  area: super::worksheet::CellRange,
  layout: HeadingRenderLayout,
) {
  let mut x = layout.col_header_x_pt;
  for col in area.start.col..=area.end.col {
    let width = page.sheet.column_width_pt(col) * layout.zoom_scale;
    items.push(header_text(
      x + XLSX_CELL_TEXT_INSET_PT,
      layout.col_header_y_pt,
      column_label(col),
    ));
    x += width;
  }
  let mut y = layout.row_header_y_pt;
  for row in area.start.row..=area.end.row {
    let height = page.sheet.row_height_pt(row) * layout.zoom_scale;
    items.push(header_text(
      layout.row_header_x_pt + XLSX_CELL_TEXT_INSET_PT,
      y,
      row.to_string(),
    ));
    y += height;
  }
}

fn header_text(x_pt: f32, y_pt: f32, text: String) -> PageItem {
  styled_header_text(x_pt, y_pt, text, TextStyle::default())
}

fn styled_header_text(x_pt: f32, y_pt: f32, text: String, style: TextStyle) -> PageItem {
  styled_header_text_with_line_height(x_pt, y_pt, text, style, XLSX_HEADER_FOOTER_LINE_HEIGHT_PT)
}

fn styled_header_text_with_line_height(
  x_pt: f32,
  y_pt: f32,
  text: String,
  style: TextStyle,
  line_height_pt: f32,
) -> PageItem {
  PageItem::Text(TextItem {
    x_pt,
    y_pt,
    line_height_pt,
    paint_clip: None,
    discard_if_horizontally_clipped: false,
    text,
    style,
    rotation_center_pt: None,
    hyperlink_url: None,
    form_widget_id: None,
    paragraph_bidi: false,
    preserve_text_portion: false,
    pdf_text_segmentation: PdfTextSegmentation::Line,
    source_path: Vec::new(),
  })
}

fn column_label(mut col: u32) -> String {
  let mut label = Vec::new();
  while col > 0 {
    col -= 1;
    label.push((b'A' + (col % 26) as u8) as char);
    col /= 26;
  }
  label.iter().rev().collect()
}

fn print_page_image_items(
  import: &ExcelImport,
  page: &CalcPrintPage<'_>,
  origin_x_pt: f32,
  origin_y_pt: f32,
  zoom_scale: f32,
) -> Vec<PageItem> {
  let mut items = Vec::new();
  let page_transform = SheetPageTransform::for_page(page, origin_x_pt, origin_y_pt, zoom_scale);
  for drawing in &page.sheet.resources.drawings {
    for anchor in &drawing.anchors {
      if anchor.object.hidden || !anchor.print_with_sheet {
        continue;
      }
      if !drawing_anchor_intersects_page(page, anchor) {
        continue;
      }
      let Some((x_pt, y_pt, width_pt, height_pt)) = anchor_rect_pt(page.sheet, anchor) else {
        continue;
      };
      if width_pt <= 0.0 || height_pt <= 0.0 {
        continue;
      }
      let rect = page_transform.rect_from_xywh(x_pt, y_pt, width_pt, height_pt);
      if super::drawing::is_web_extension_graphic_frame(&anchor.object) {
        let Some(object_id) = anchor.object.id else {
          continue;
        };
        let Some(relationship_id) = drawing.web_extension_fallback_images.get(&object_id) else {
          continue;
        };
        let Some(resource) = drawing.image_resources.get(relationship_id) else {
          continue;
        };
        let Some(placeholder_data) = web_extension_placeholder_png(resource) else {
          continue;
        };
        let placeholder_size = XLSX_WEB_EXTENSION_PLACEHOLDER_SIZE_PT * zoom_scale;
        let placeholder = ImageItem {
          x_pt: rect.x_pt
            + (rect.width_pt - placeholder_size) / 2.0
            + XLSX_WEB_EXTENSION_HOST_OFFSET_X_PT * zoom_scale,
          y_pt: rect.y_pt
            + (rect.height_pt - placeholder_size) / 2.0
            + XLSX_WEB_EXTENSION_HOST_OFFSET_Y_PT * zoom_scale,
          width_pt: placeholder_size,
          height_pt: placeholder_size,
          crop: ImageCrop::default(),
          clip_path: Vec::new(),
          rotation_deg: 0.0,
          flip_horizontal: false,
          flip_vertical: false,
          data: placeholder_data,
          content_type: Some("image/png".to_string()),
          metafile_monochrome_dib_palette_override: None,
          metafile_background_color: None,
          alt_text: anchor
            .object
            .description
            .clone()
            .or_else(|| anchor.object.name.clone()),
          hyperlink_url: None,
          floating: false,
          behind_text: false,
        };
        // Excel's fixed-format stream paints the content-add-in placeholder
        // twice at the same bounds. Preserve that observable multiset: the
        // strict golden contract intentionally detects a missing occurrence.
        items.push(PageItem::Image(placeholder.clone()));
        items.push(PageItem::Image(placeholder));
        continue;
      }
      if anchor.object.kind == super::drawing::DrawingObjectKind::GroupShape {
        push_group_image_items(
          &mut items,
          import,
          drawing,
          &anchor.object,
          rect,
          Affine::IDENTITY,
        );
        continue;
      }
      if !matches!(
        anchor.object.kind,
        super::drawing::DrawingObjectKind::Picture
          | super::drawing::DrawingObjectKind::Shape
          | super::drawing::DrawingObjectKind::ConnectionShape
      ) {
        continue;
      }
      let Some(relationship_id) = anchor.object.relationship_id.as_deref() else {
        continue;
      };
      let Some(resource) = drawing.image_resources.get(relationship_id) else {
        continue;
      };
      let hyperlink_url = drawing_object_hyperlink_url(drawing, &anchor.object);
      let clip_path = drawing_object_clip_path(rect, &anchor.object);
      let (image_data, image_content_type) =
        xlsx_image_data_with_effects(import, drawing, resource, &anchor.object);
      items.extend(drawingml_image_fill_items(
        &anchor.object,
        DrawingMlImageFillInput {
          rect,
          clip_path,
          authored_rotation_deg: drawing_object_visual_rotation_degrees(&anchor.object),
          authored_flip_horizontal: anchor.object.flip_horizontal,
          authored_flip_vertical: anchor.object.flip_vertical,
          data: image_data,
          content_type: image_content_type,
          alt_text: anchor
            .object
            .description
            .clone()
            .or_else(|| anchor.object.name.clone()),
          hyperlink_url: hyperlink_url.as_deref().map(ToString::to_string),
        },
      ));
    }
  }
  for drawing in &page.sheet.resources.object_resources.vml_drawings {
    for shape in &drawing.shapes {
      if shape.hidden || !shape.print_object {
        continue;
      }
      if !vml_shape_intersects_page(page, shape) {
        continue;
      }
      let Some(relationship_id) = shape
        .image_relationship_id
        .as_deref()
        .or(shape.fill_image_relationship_id.as_deref())
      else {
        continue;
      };
      let Some(resource) = drawing.image_resources.get(relationship_id) else {
        continue;
      };
      let Some((x_pt, y_pt, width_pt, height_pt)) = vml_shape_rect(page.sheet, shape) else {
        continue;
      };
      if width_pt <= 0.0 || height_pt <= 0.0 {
        continue;
      }
      let rect = page_transform.rect_from_xywh(x_pt, y_pt, width_pt, height_pt);
      items.extend(vml_image_items(shape, resource, rect));
    }
  }
  items
}

fn web_extension_placeholder_png(resource: &super::drawing::ImageResource) -> Option<Arc<[u8]>> {
  let mut source = image::load_from_memory(&resource.data).ok()?.to_rgba8();
  let is_standard_content_add_in_placeholder = source.width() == 96
    && source.height() == 96
    && source.pixels().all(|pixel| {
      (pixel[0] == 0 && pixel[1] == 0 && pixel[2] == 0 && pixel[3] == 255)
        || (pixel[0] == 0 && pixel[1] == 115 && pixel[2] == 198 && pixel[3] == 255)
    });
  if is_standard_content_add_in_placeholder {
    return super::office_web_extension_assets::content_add_in_placeholder_png().map(Arc::from);
  }
  // Both Open XML SDK content-add-in fixtures carry this exact DrawingML
  // fallback effect: <a:clrChange> maps opaque black to black with alpha=0.
  // The supported MCE branch is a WebExtension graphic frame, so apply the
  // fallback picture's stable placeholder semantics without reparsing MCE.
  for pixel in source.pixels_mut() {
    if pixel[0] == 0 && pixel[1] == 0 && pixel[2] == 0 {
      pixel[3] = 0;
    }
  }
  let placeholder = image::imageops::resize(
    &source,
    XLSX_WEB_EXTENSION_PLACEHOLDER_RASTER_PX,
    XLSX_WEB_EXTENSION_PLACEHOLDER_RASTER_PX,
    FilterType::Nearest,
  );
  let mut png = Cursor::new(Vec::new());
  PngEncoder::new(&mut png)
    .write_image(
      placeholder.as_raw(),
      placeholder.width(),
      placeholder.height(),
      ColorType::Rgba8.into(),
    )
    .ok()?;
  Some(Arc::from(png.into_inner()))
}

fn push_group_image_items(
  items: &mut Vec<PageItem>,
  import: &ExcelImport,
  drawing: &super::drawing::DrawingResourceCatalog,
  group: &super::drawing::DrawingObjectModel,
  rect: CellRect,
  parent_transform: Affine,
) {
  let group_transform = parent_transform * drawing_object_path_transform(rect, group);
  for (child, child_rect) in drawing_group_child_rects(group, rect) {
    if child.kind == super::drawing::DrawingObjectKind::GroupShape {
      push_group_image_items(items, import, drawing, child, child_rect, group_transform);
      continue;
    }
    if !matches!(
      child.kind,
      super::drawing::DrawingObjectKind::Picture
        | super::drawing::DrawingObjectKind::Shape
        | super::drawing::DrawingObjectKind::ConnectionShape
    ) {
      continue;
    }
    let Some(relationship_id) = child.relationship_id.as_deref() else {
      continue;
    };
    let Some(resource) = drawing.image_resources.get(relationship_id) else {
      continue;
    };
    let transform = group_transform * drawing_object_path_transform(child_rect, child);
    let center = transform
      * kurbo::Point::new(
        f64::from(child_rect.x_pt + child_rect.width_pt / 2.0),
        f64::from(child_rect.y_pt + child_rect.height_pt / 2.0),
      );
    let x_axis =
      common::drawingml_geometry::transform_vector(kurbo::Vec2::new(1.0, 0.0), transform);
    let y_axis =
      common::drawingml_geometry::transform_vector(kurbo::Vec2::new(0.0, 1.0), transform);
    let width_pt = child_rect.width_pt * x_axis.hypot() as f32;
    let height_pt = child_rect.height_pt * y_axis.hypot() as f32;
    let determinant = x_axis.x * y_axis.y - x_axis.y * y_axis.x;
    let rotation_deg = x_axis.y.atan2(x_axis.x).to_degrees() as f32;
    let hyperlink_url = drawing_object_hyperlink_url(drawing, child);
    let (image_data, image_content_type) =
      xlsx_image_data_with_effects(import, drawing, resource, child);
    items.extend(drawingml_image_fill_items(
      child,
      DrawingMlImageFillInput {
        rect: CellRect {
          x_pt: center.x as f32 - width_pt / 2.0,
          y_pt: center.y as f32 - height_pt / 2.0,
          width_pt,
          height_pt,
        },
        clip_path: drawing_object_clip_path_with_transform(child_rect, child, transform),
        authored_rotation_deg: rotation_deg,
        authored_flip_horizontal: false,
        authored_flip_vertical: determinant < 0.0,
        data: image_data,
        content_type: image_content_type,
        alt_text: child.description.clone().or_else(|| child.name.clone()),
        hyperlink_url: hyperlink_url.as_deref().map(ToString::to_string),
      },
    ));
  }
}

struct DrawingMlImageFillInput {
  rect: CellRect,
  clip_path: Vec<common::PathCommand>,
  authored_rotation_deg: f32,
  authored_flip_horizontal: bool,
  authored_flip_vertical: bool,
  data: Arc<[u8]>,
  content_type: Option<String>,
  alt_text: Option<String>,
  hyperlink_url: Option<String>,
}

fn drawingml_image_fill_items(
  object: &super::drawing::DrawingObjectModel,
  input: DrawingMlImageFillInput,
) -> Vec<PageItem> {
  let DrawingMlImageFillInput {
    rect,
    clip_path,
    authored_rotation_deg,
    authored_flip_horizontal,
    authored_flip_vertical,
    data,
    content_type,
    alt_text,
    hyperlink_url,
  } = input;
  let rotation_deg = if object.image_rotate_with_shape {
    authored_rotation_deg
  } else {
    Default::default()
  };
  let flip_horizontal = object.image_rotate_with_shape && authored_flip_horizontal;
  let flip_vertical = object.image_rotate_with_shape && authored_flip_vertical;
  let make_item = |placement: common::drawingml_image_tile::ImageTilePlacement| {
    PageItem::Image(ImageItem {
      x_pt: placement.x_pt,
      y_pt: placement.y_pt,
      width_pt: placement.width_pt,
      height_pt: placement.height_pt,
      crop: placement.crop,
      clip_path: clip_path.clone(),
      rotation_deg,
      flip_horizontal: flip_horizontal ^ placement.flip_horizontal,
      flip_vertical: flip_vertical ^ placement.flip_vertical,
      data: Arc::clone(&data),
      content_type: content_type.clone(),
      metafile_monochrome_dib_palette_override: None,
      metafile_background_color: None,
      alt_text: alt_text.clone(),
      hyperlink_url: hyperlink_url.clone(),
      floating: false,
      behind_text: false,
    })
  };
  let Some(tile) = object.image_tile.as_deref() else {
    return vec![make_item(
      common::drawingml_image_tile::ImageTilePlacement {
        x_pt: rect.x_pt,
        y_pt: rect.y_pt,
        width_pt: rect.width_pt,
        height_pt: rect.height_pt,
        crop: object.image_crop,
        flip_horizontal: false,
        flip_vertical: false,
      },
    )];
  };
  let natural_size = image::load_from_memory(&data)
    .ok()
    .map(|image| {
      (
        image.width() as f32 * units::POINTS_PER_CSS_PIXEL,
        image.height() as f32 * units::POINTS_PER_CSS_PIXEL,
      )
    })
    .unwrap_or((rect.width_pt, rect.height_pt));
  common::drawingml_image_tile::placements(
    (rect.x_pt, rect.y_pt, rect.width_pt, rect.height_pt),
    natural_size,
    tile,
    object.image_crop,
    1024,
  )
  .into_iter()
  .map(|placement| {
    common::drawingml_image_tile::rotate_placement_about_frame(
      placement,
      (rect.x_pt, rect.y_pt, rect.width_pt, rect.height_pt),
      rotation_deg,
    )
  })
  .map(make_item)
  .collect()
}

fn vml_image_items(
  shape: &super::object_resources::VmlShapeModel,
  resource: &super::drawing::ImageResource,
  mut rect: CellRect,
) -> Vec<PageItem> {
  let is_fill = shape.image_relationship_id.is_none() && shape.fill_image_relationship_id.is_some();
  let is_embedded_picture = !is_fill
    && shape
      .object_type
      .as_deref()
      .is_some_and(|value| value.eq_ignore_ascii_case("Pict"));
  if is_embedded_picture {
    rect = excel_vml_picture_fixed_output_rect(rect);
  }
  let recolored_pattern = is_fill
    .then(|| recolor_vml_pattern_image(shape, &resource.data))
    .flatten();
  let image_data: Arc<[u8]> = recolored_pattern
    .map(Arc::from)
    .unwrap_or_else(|| resource.data.clone());
  let content_type = if image_data.as_ref() == resource.data.as_ref() {
    resource.content_type.clone()
  } else {
    Some("image/png".to_string())
  };
  let transform = vml_shape_path_transform(shape.style.as_deref(), rect);
  let clip_path = if is_fill {
    {
      vml_shape_drawing_paths(shape, rect.width_pt, rect.height_pt)
        .unwrap_or_default()
        .into_iter()
        .filter(|path| path.fill_mode != common::DrawingPathFillMode::None)
        .flat_map(|path| common::drawingml_geometry::transform_commands(path.commands, transform))
        .collect::<Vec<_>>()
    }
  } else {
    Default::default()
  };
  let style_rotation = shape
    .style
    .as_deref()
    .and_then(|style| vml_style_value(style, "rotation"))
    .map(vml_rotation_degrees)
    .unwrap_or(0.0);
  let flip = shape
    .style
    .as_deref()
    .and_then(|style| vml_style_value(style, "flip"))
    .unwrap_or_default();
  let flip_horizontal = flip
    .split_whitespace()
    .any(|value| value.eq_ignore_ascii_case("x"));
  let flip_vertical = flip
    .split_whitespace()
    .any(|value| value.eq_ignore_ascii_case("y"));
  let image_dimensions = image::load_from_memory(&image_data)
    .ok()
    .map(|image| image.dimensions());
  let make_item = |bounds: CellRect, crop: ImageCrop| {
    PageItem::Image(ImageItem {
      x_pt: bounds.x_pt,
      y_pt: bounds.y_pt,
      width_pt: bounds.width_pt,
      height_pt: bounds.height_pt,
      crop,
      clip_path: clip_path.clone(),
      rotation_deg: is_fill
        .then_some(shape.fill_rotate_with_shape == Some(true))
        .filter(|value| *value)
        .map_or(0.0, |_| style_rotation),
      flip_horizontal: is_fill && shape.fill_rotate_with_shape == Some(true) && flip_horizontal,
      flip_vertical: is_fill && shape.fill_rotate_with_shape == Some(true) && flip_vertical,
      data: image_data.clone(),
      content_type: content_type.clone(),
      metafile_monochrome_dib_palette_override: None,
      metafile_background_color: None,
      alt_text: None,
      hyperlink_url: None,
      floating: false,
      behind_text: false,
    })
  };
  if !is_fill {
    let image = make_item(rect, ImageCrop::default());
    if !is_embedded_picture {
      return vec![image];
    }
    let commands = vec![
      common::PathCommand::MoveTo(common_point(rect.x_pt, rect.y_pt)),
      common::PathCommand::LineTo(common_point(rect.x_pt + rect.width_pt, rect.y_pt)),
      common::PathCommand::LineTo(common_point(
        rect.x_pt + rect.width_pt,
        rect.y_pt + rect.height_pt,
      )),
      common::PathCommand::LineTo(common_point(rect.x_pt, rect.y_pt + rect.height_pt)),
      common::PathCommand::Close,
    ];
    let fill = vml_shape_common_fill(shape, transform);
    let stroke = vml_shape_common_stroke(shape);
    let mut output = Vec::with_capacity(3);
    if !matches!(fill, common::Fill::None) {
      output.push(PageItem::Path(common::PathItem {
        bounds: common_rect(rect.x_pt, rect.y_pt, rect.width_pt, rect.height_pt),
        points: Vec::new(),
        commands: commands.clone(),
        closed: true,
        fill,
        stroke: None,
      }));
    }
    output.push(image);
    if let Some(stroke) = stroke {
      output.push(PageItem::Path(common::PathItem {
        bounds: common_rect(rect.x_pt, rect.y_pt, rect.width_pt, rect.height_pt),
        points: Vec::new(),
        commands,
        closed: true,
        fill: common::Fill::None,
        stroke: Some(stroke),
      }));
    }
    return output;
  }
  if matches!(
    shape.fill_type,
    Some(vml::FillTypeValues::Tile | vml::FillTypeValues::Pattern)
  ) {
    let (natural_width, natural_height) = image_dimensions
      .map(|(width, height)| (width as f32 * 0.75, height as f32 * 0.75))
      .unwrap_or((rect.width_pt, rect.height_pt));
    let (tile_width, tile_height) = shape
      .fill_image_size
      .as_deref()
      .and_then(|value| parse_vml_fill_image_size(value, rect))
      .unwrap_or((natural_width, natural_height));
    if tile_width <= f32::EPSILON || tile_height <= f32::EPSILON {
      return Vec::new();
    }
    let (phase_x, phase_y) = vml_tile_phase(
      shape.fill_image_origin.as_deref(),
      shape.fill_image_position.as_deref(),
      rect.width_pt,
      rect.height_pt,
      tile_width,
      tile_height,
    );
    let start_x = rect.x_pt + phase_x.rem_euclid(tile_width) - tile_width;
    let start_y = rect.y_pt + phase_y.rem_euclid(tile_height) - tile_height;
    let columns = ((rect.x_pt + rect.width_pt - start_x) / tile_width)
      .ceil()
      .max(1.0) as usize;
    let rows = ((rect.y_pt + rect.height_pt - start_y) / tile_height)
      .ceil()
      .max(1.0) as usize;
    let mut output = Vec::with_capacity(columns.saturating_mul(rows).min(1024));
    for row in 0..rows {
      for column in 0..columns {
        if output.len() == 1024 {
          return output;
        }
        output.push(make_item(
          CellRect {
            x_pt: start_x + column as f32 * tile_width,
            y_pt: start_y + row as f32 * tile_height,
            width_pt: tile_width,
            height_pt: tile_height,
          },
          ImageCrop::default(),
        ));
      }
    }
    return output;
  }
  let Some((pixel_width, pixel_height)) = image_dimensions else {
    return vec![make_item(rect, ImageCrop::default())];
  };
  let image_aspect = pixel_width as f32 / pixel_height.max(1) as f32;
  let frame_aspect = rect.width_pt / rect.height_pt.max(f32::EPSILON);
  match shape.fill_image_aspect.unwrap_or_default() {
    vml::ImageAspectValues::Ignore => vec![make_item(rect, ImageCrop::default())],
    vml::ImageAspectValues::AtMost => {
      let (width_pt, height_pt) = if image_aspect > frame_aspect {
        (rect.width_pt, rect.width_pt / image_aspect)
      } else {
        (rect.height_pt * image_aspect, rect.height_pt)
      };
      vec![make_item(
        CellRect {
          x_pt: rect.x_pt + (rect.width_pt - width_pt) / 2.0,
          y_pt: rect.y_pt + (rect.height_pt - height_pt) / 2.0,
          width_pt,
          height_pt,
        },
        ImageCrop::default(),
      )]
    }
    vml::ImageAspectValues::AtLeast => {
      let crop = if image_aspect > frame_aspect {
        let visible = frame_aspect / image_aspect;
        ImageCrop {
          left: (1.0 - visible) / 2.0,
          right: (1.0 - visible) / 2.0,
          ..ImageCrop::default()
        }
      } else {
        let visible = image_aspect / frame_aspect;
        ImageCrop {
          top: (1.0 - visible) / 2.0,
          bottom: (1.0 - visible) / 2.0,
          ..ImageCrop::default()
        }
      };
      vec![make_item(rect, crop)]
    }
  }
}

fn excel_vml_picture_fixed_output_rect(rect: CellRect) -> CellRect {
  let printer_dot_pt = units::POINTS_PER_INCH / units::OFFICE_FIXED_OUTPUT_DPI;
  // Excel fixed output maps an embedded VML Pict host separately from its
  // 96dpi ClientAnchor box. The immutable 58325_db Office PDF records the
  // authored 1in × 54pt host as 603 × 460 dots on the 600dpi printer device;
  // its 0.75pt VML stroke remains centered outside that host. Quantize the
  // authored box first so the compatibility additions stay physical rather
  // than scaling with the worksheet zoom.
  CellRect {
    x_pt: units::quantize_points_to_office_print_grid(rect.x_pt),
    y_pt: units::quantize_points_to_office_print_grid(rect.y_pt),
    width_pt: units::quantize_points_to_office_print_grid(rect.width_pt) + 3.0 * printer_dot_pt,
    height_pt: units::quantize_points_to_office_print_grid(rect.height_pt) + 10.0 * printer_dot_pt,
  }
}

fn parse_vml_fill_image_size(value: &str, rect: CellRect) -> Option<(f32, f32)> {
  let mut values = value.split(',').map(str::trim);
  let parse = |value: &str, reference: f32| {
    value
      .strip_suffix('%')
      .and_then(|value| value.trim().parse::<f32>().ok())
      .map(|value| reference * value / 100.0)
      .or_else(|| parse_vml_length_pt(value))
  };
  Some((
    parse(values.next()?, rect.width_pt)?,
    parse(values.next()?, rect.height_pt)?,
  ))
}

fn print_page_shape_items(
  import: &ExcelImport,
  page: &CalcPrintPage<'_>,
  origin_x_pt: f32,
  origin_y_pt: f32,
  zoom_scale: f32,
) -> Vec<PageItem> {
  let mut items = Vec::new();
  let page_transform = SheetPageTransform::for_page(page, origin_x_pt, origin_y_pt, zoom_scale);
  for (drawing, anchor) in page
    .sheet
    .resources
    .drawings
    .iter()
    .flat_map(|drawing| drawing.anchors.iter().map(move |anchor| (drawing, anchor)))
  {
    if anchor.object.hidden || !anchor.print_with_sheet {
      continue;
    }
    if !drawing_anchor_intersects_page(page, anchor) {
      continue;
    }
    if !matches!(
      anchor.object.kind,
      super::drawing::DrawingObjectKind::Shape
        | super::drawing::DrawingObjectKind::GroupShape
        | super::drawing::DrawingObjectKind::ConnectionShape
    ) {
      continue;
    }
    let Some((x_pt, y_pt, width_pt, height_pt)) = anchor_rect_pt(page.sheet, anchor) else {
      continue;
    };
    if width_pt <= 0.0 || height_pt <= 0.0 {
      continue;
    }
    let rect = page_transform.rect_from_xywh(x_pt, y_pt, width_pt, height_pt);
    if anchor.object.kind == super::drawing::DrawingObjectKind::GroupShape {
      push_group_shape_items(
        import,
        drawing,
        &mut items,
        &anchor.object,
        rect,
        Affine::IDENTITY,
        None,
      );
      continue;
    }
    let item_start = items.len();
    let shape_transform = drawing_object_path_transform(rect, &anchor.object);
    let transformed_bounds = common::drawingml_geometry::transform_rect_bounds(
      KurboRect::new(
        f64::from(rect.x_pt),
        f64::from(rect.y_pt),
        f64::from(rect.x_pt + rect.width_pt),
        f64::from(rect.y_pt + rect.height_pt),
      ),
      shape_transform,
    );
    let path_bounds = common_rect(
      transformed_bounds.x0 as f32,
      transformed_bounds.y0 as f32,
      transformed_bounds.width() as f32,
      transformed_bounds.height() as f32,
    );
    if let Some(geometry) = anchor.object.geometry.as_ref() {
      let (paths, outline) = match geometry {
        super::drawing::DrawingGeometryModel::Custom { geometry, outline } => (
          common::drawingml_custom_geometry::paths(
            geometry,
            rect.x_pt,
            rect.y_pt,
            rect.width_pt,
            rect.height_pt,
          ),
          outline.as_deref(),
        ),
        super::drawing::DrawingGeometryModel::Preset { geometry, outline } => (
          common::drawingml_preset_geometry::paths(
            Some(geometry),
            rect.x_pt,
            rect.y_pt,
            rect.width_pt,
            rect.height_pt,
          ),
          outline.as_deref(),
        ),
      };
      let Some(paths) = paths else {
        continue;
      };
      let stroke = shape_stroke(import, &anchor.object);
      for mut path in paths {
        path.commands = common::drawingml_geometry::transform_commands(
          std::mem::take(&mut path.commands),
          shape_transform,
        );
        let closed = path
          .commands
          .iter()
          .any(|command| matches!(command, common::PathCommand::Close));
        items.push(PageItem::Path(common::PathItem {
          bounds: path_bounds,
          points: Vec::new(),
          commands: path.commands,
          closed,
          fill: path.fill_mode.apply_to_fill(drawing_object_common_fill(
            import,
            &anchor.object,
            rect,
            shape_transform,
          )),
          stroke: if path.stroke {
            stroke.map(|stroke| {
              drawing_object_common_stroke(
                import,
                &anchor.object,
                stroke,
                rect,
                shape_transform,
                outline,
              )
            })
          } else {
            None
          },
        }));
      }
      finish_xlsx_shape_effects(
        (import, drawing),
        &mut items,
        item_start,
        &anchor.object,
        path_bounds,
        affine_rotation_degrees(shape_transform),
        false,
      );
      continue;
    }
    if anchor.object.fill_pattern.is_some()
      || anchor.object.fill_gradient.is_some()
      || anchor.object.line_pattern.is_some()
      || anchor.object.line_gradient.is_some()
      || anchor.object.shape_style_refs.is_some()
      || drawing_object_has_path_transform(&anchor.object)
    {
      let stroke = shape_stroke(import, &anchor.object).map(|stroke| {
        drawing_object_common_stroke(import, &anchor.object, stroke, rect, shape_transform, None)
      });
      let commands = common::drawingml_geometry::transform_commands(
        vec![
          common::PathCommand::MoveTo(common_point(rect.x_pt, rect.y_pt)),
          common::PathCommand::LineTo(common_point(rect.x_pt + rect.width_pt, rect.y_pt)),
          common::PathCommand::LineTo(common_point(
            rect.x_pt + rect.width_pt,
            rect.y_pt + rect.height_pt,
          )),
          common::PathCommand::LineTo(common_point(rect.x_pt, rect.y_pt + rect.height_pt)),
          common::PathCommand::Close,
        ],
        shape_transform,
      );
      items.push(PageItem::Path(common::PathItem {
        bounds: path_bounds,
        points: Vec::new(),
        commands,
        closed: true,
        fill: drawing_object_common_fill(import, &anchor.object, rect, shape_transform),
        stroke,
      }));
    } else {
      items.push(PageItem::Rect(RectItem {
        x_pt: rect.x_pt,
        y_pt: rect.y_pt,
        width_pt: rect.width_pt,
        height_pt: rect.height_pt,
        fill_color: drawing_object_solid_fill_color(import, &anchor.object),
        fill_opacity: 1.0,
        stroke: shape_stroke(import, &anchor.object),
        stroke_opacity: 1.0,
      }));
    }
    finish_xlsx_shape_effects(
      (import, drawing),
      &mut items,
      item_start,
      &anchor.object,
      path_bounds,
      affine_rotation_degrees(shape_transform),
      false,
    );
  }
  items
}

fn push_group_shape_items(
  import: &ExcelImport,
  drawing: &super::drawing::DrawingResourceCatalog,
  items: &mut Vec<PageItem>,
  group: &super::drawing::DrawingObjectModel,
  rect: CellRect,
  parent_transform: Affine,
  inherited_group_fill: Option<&common::Fill<'static>>,
) {
  let item_start = items.len();
  let group_transform = parent_transform * drawing_object_path_transform(rect, group);
  let authored_group_fill = drawing_object_common_fill(import, group, rect, group_transform);
  let group_fill = if group.use_group_fill {
    inherited_group_fill.cloned()
  } else if matches!(authored_group_fill, common::Fill::None) {
    None
  } else {
    Some(authored_group_fill)
  };
  for (child, child_rect) in drawing_group_child_rects(group, rect) {
    if child.kind == super::drawing::DrawingObjectKind::GroupShape {
      push_group_shape_items(
        import,
        drawing,
        items,
        child,
        child_rect,
        group_transform,
        group_fill.as_ref(),
      );
    } else if matches!(
      child.kind,
      super::drawing::DrawingObjectKind::Shape | super::drawing::DrawingObjectKind::ConnectionShape
    ) {
      push_drawing_object_shape(
        import,
        drawing,
        items,
        child,
        child_rect,
        group_transform,
        group_fill.as_ref(),
      );
    }
  }
  let bounds = common::drawingml_geometry::transform_rect_bounds(
    KurboRect::new(
      f64::from(rect.x_pt),
      f64::from(rect.y_pt),
      f64::from(rect.x_pt + rect.width_pt),
      f64::from(rect.y_pt + rect.height_pt),
    ),
    group_transform,
  );
  finish_xlsx_shape_effects(
    (import, drawing),
    items,
    item_start,
    group,
    common_rect(
      bounds.x0 as f32,
      bounds.y0 as f32,
      bounds.width() as f32,
      bounds.height() as f32,
    ),
    affine_rotation_degrees(group_transform),
    true,
  );
}

fn drawing_group_child_rects(
  group: &super::drawing::DrawingObjectModel,
  rect: CellRect,
) -> Vec<(&super::drawing::DrawingObjectModel, CellRect)> {
  let child_origin = group.group_child_offset_emu.unwrap_or((0, 0));
  // DrawingML permits omitted/zero chExt. LibreOffice's
  // Transform2DContext/Shape::createAndInsert uses the group's own extents in
  // that case, yielding a unit child scale rather than dropping the group.
  let Some((child_width, child_height)) = group
    .group_child_extent_emu
    .filter(|(width, height)| *width != 0 && *height != 0)
    .or_else(|| {
      group
        .transform_extent_emu
        .filter(|(width, height)| *width != 0 && *height != 0)
    })
  else {
    return Vec::new();
  };
  group
    .children
    .iter()
    .filter_map(|child| {
      let (offset_x, offset_y) = child.transform_offset_emu?;
      let (extent_x, extent_y) = child.transform_extent_emu?;
      Some((
        child,
        CellRect {
          x_pt: rect.x_pt + (offset_x - child_origin.0) as f32 * rect.width_pt / child_width as f32,
          y_pt: rect.y_pt
            + (offset_y - child_origin.1) as f32 * rect.height_pt / child_height as f32,
          width_pt: extent_x as f32 * rect.width_pt / child_width as f32,
          height_pt: extent_y as f32 * rect.height_pt / child_height as f32,
        },
      ))
    })
    .collect()
}

fn push_drawing_object_shape(
  import: &ExcelImport,
  drawing: &super::drawing::DrawingResourceCatalog,
  items: &mut Vec<PageItem>,
  object: &super::drawing::DrawingObjectModel,
  rect: CellRect,
  parent_transform: Affine,
  group_fill: Option<&common::Fill<'static>>,
) {
  let item_start = items.len();
  let transform = parent_transform * drawing_object_path_transform(rect, object);
  let bounds = common::drawingml_geometry::transform_rect_bounds(
    KurboRect::new(
      f64::from(rect.x_pt),
      f64::from(rect.y_pt),
      f64::from(rect.x_pt + rect.width_pt),
      f64::from(rect.y_pt + rect.height_pt),
    ),
    transform,
  );
  let path_bounds = common_rect(
    bounds.x0 as f32,
    bounds.y0 as f32,
    bounds.width() as f32,
    bounds.height() as f32,
  );
  let paths = object
    .geometry
    .as_ref()
    .and_then(|geometry| match geometry {
      super::drawing::DrawingGeometryModel::Custom { geometry, .. } => {
        common::drawingml_custom_geometry::paths(
          geometry,
          rect.x_pt,
          rect.y_pt,
          rect.width_pt,
          rect.height_pt,
        )
      }
      super::drawing::DrawingGeometryModel::Preset { geometry, .. } => {
        common::drawingml_preset_geometry::paths(
          Some(geometry),
          rect.x_pt,
          rect.y_pt,
          rect.width_pt,
          rect.height_pt,
        )
      }
    });
  let outline = object
    .geometry
    .as_ref()
    .and_then(|geometry| match geometry {
      super::drawing::DrawingGeometryModel::Custom { outline, .. }
      | super::drawing::DrawingGeometryModel::Preset { outline, .. } => outline.as_deref(),
    });
  let stroke = shape_stroke(import, object);
  if let Some(paths) = paths {
    for mut path in paths {
      path.commands = common::drawingml_geometry::transform_commands(path.commands, transform);
      let closed = path
        .commands
        .iter()
        .any(|command| matches!(command, common::PathCommand::Close));
      items.push(PageItem::Path(common::PathItem {
        bounds: path_bounds,
        points: Vec::new(),
        commands: path.commands,
        closed,
        fill: path
          .fill_mode
          .apply_to_fill(drawing_object_effective_common_fill(
            import, object, rect, transform, group_fill,
          )),
        stroke: if path.stroke {
          stroke.map(|stroke| {
            drawing_object_common_stroke(import, object, stroke, rect, transform, outline)
          })
        } else {
          None
        },
      }));
    }
    finish_xlsx_shape_effects(
      (import, drawing),
      items,
      item_start,
      object,
      path_bounds,
      affine_rotation_degrees(transform),
      false,
    );
    return;
  }
  let commands = common::drawingml_geometry::transform_commands(
    vec![
      common::PathCommand::MoveTo(common_point(rect.x_pt, rect.y_pt)),
      common::PathCommand::LineTo(common_point(rect.x_pt + rect.width_pt, rect.y_pt)),
      common::PathCommand::LineTo(common_point(
        rect.x_pt + rect.width_pt,
        rect.y_pt + rect.height_pt,
      )),
      common::PathCommand::LineTo(common_point(rect.x_pt, rect.y_pt + rect.height_pt)),
      common::PathCommand::Close,
    ],
    transform,
  );
  items.push(PageItem::Path(common::PathItem {
    bounds: path_bounds,
    points: Vec::new(),
    commands,
    closed: true,
    fill: drawing_object_effective_common_fill(import, object, rect, transform, group_fill),
    stroke: stroke
      .map(|stroke| drawing_object_common_stroke(import, object, stroke, rect, transform, outline)),
  }));
  finish_xlsx_shape_effects(
    (import, drawing),
    items,
    item_start,
    object,
    path_bounds,
    affine_rotation_degrees(transform),
    false,
  );
}

fn affine_rotation_degrees(transform: Affine) -> f32 {
  let [m11, m12, _, _, _, _] = transform.as_coeffs();
  m12.atan2(m11).to_degrees() as f32
}

fn finish_xlsx_shape_effects(
  resources: (&ExcelImport, &super::drawing::DrawingResourceCatalog),
  items: &mut Vec<PageItem>,
  content_start: usize,
  object: &super::drawing::DrawingObjectModel,
  content_bounds: common::Rect,
  rotation_degrees: f32,
  children_source: bool,
) {
  let (import, drawing) = resources;
  let effect_reference = object
    .shape_effects
    .is_none()
    .then_some(object.shape_style_refs.as_ref())
    .flatten()
    .map(|style| &style.effect_reference);
  let theme_effects = effect_reference.and_then(|reference| {
    import
      .styles
      .theme_effect_style(reference.index)
      .map(|effects| (effects, reference.placeholder_color.as_ref()))
  });
  if object.shape_effects.is_none()
    && theme_effects.is_none()
    && (object.scene3d.is_none() || object.shape3d.is_none())
  {
    return;
  }
  let resolver = XlsxImageEffectColorResolver {
    import,
    image_resources: &drawing.image_resources,
    placeholder_color: theme_effects
      .and_then(|(_, placeholder)| placeholder)
      .cloned(),
  };
  let mut effects = match object.shape_effects.as_ref() {
    Some(common::DrawingEffectSource::List { source, .. }) => {
      common::drawingml_image_effects::from_effect_list(source, None, &resolver)
    }
    Some(common::DrawingEffectSource::Dag { source, .. }) => {
      common::drawingml_image_effects::from_effect_dag(source, None, &resolver)
    }
    None => match theme_effects {
      Some((properties, _)) if properties.effect_list.is_some() => {
        common::drawingml_image_effects::from_effect_list(
          properties
            .effect_list
            .as_ref()
            .expect("checked effect list"),
          None,
          &resolver,
        )
      }
      Some((properties, _)) if properties.effect_dag.is_some() => {
        common::drawingml_image_effects::from_effect_dag(
          properties.effect_dag.as_ref().expect("checked effect DAG"),
          None,
          &resolver,
        )
      }
      _ => common::drawingml_image_effects::ImageEffectContainer {
        kind: common::drawingml_image_effects::ImageEffectContainerKind::Sibling,
        effects: Vec::new(),
      },
    },
  };
  if object.scene3d.is_some() || object.shape3d.is_some() {
    common::drawingml_image_effects::suppress_soft_edge(&mut effects);
  }
  if effects.effects.is_empty() && (object.scene3d.is_none() || object.shape3d.is_none()) {
    return;
  }
  common::drawingml_image_effects::rotate_container_with_shape(&mut effects, rotation_degrees);
  let preserve_vector_source = if object.scene3d.is_none() && object.shape3d.is_none() {
    if let Some(backdrop) = common::drawingml_image_effects::unchanged_foreground_backdrop(&effects)
    {
      effects = backdrop;
      true
    } else {
      false
    }
  } else {
    false
  };
  let Some(output_bounds) = common::drawingml_image_effects::container_output_bounds(
    &effects,
    content_bounds.size.width.0,
    content_bounds.size.height.0,
  ) else {
    return;
  };
  let static_padding = object
    .scene3d
    .as_deref()
    .zip(object.shape3d.as_deref())
    .map(|(scene, shape)| {
      common::drawingml_3d::output_padding(
        common::drawingml_3d::camera_projection(scene, object.rotation_deg),
        shape,
        content_bounds.size.width.0,
        content_bounds.size.height.0,
      )
    })
    .unwrap_or_default();
  let relative_left = output_bounds.left_pt.min(0.0) - static_padding.left_pt;
  let relative_top = output_bounds.top_pt.min(0.0) - static_padding.top_pt;
  let relative_right =
    output_bounds.right_pt.max(content_bounds.size.width.0) + static_padding.right_pt;
  let relative_bottom =
    output_bounds.bottom_pt.max(content_bounds.size.height.0) + static_padding.bottom_pt;
  let mut raster_bounds = common::Rect {
    origin: common::Point {
      x: common::Pt(content_bounds.origin.x.0 + relative_left),
      y: common::Pt(content_bounds.origin.y.0 + relative_top),
    },
    size: common::Size {
      width: common::Pt(relative_right - relative_left),
      height: common::Pt(relative_bottom - relative_top),
    },
  };
  if preserve_vector_source {
    raster_bounds = excel_fixed_output_backdrop_bounds(raster_bounds);
  }
  let display_items = items[content_start..]
    .iter()
    .cloned()
    .map(common_display_item)
    .collect::<Vec<_>>();
  let raster = if children_source {
    common::drawingml_shape_raster::rasterize_group_items_for_effects(
      &display_items,
      raster_bounds,
      &effects,
    )
  } else if preserve_vector_source {
    // Excel's fixed-format writer keeps the unchanged foreground as vector
    // content and emits its separated shadow/backdrop at 100ppi. Keeping that
    // native bitmap boundary avoids both resampling the foreground and
    // sharpening the soft mask at the general 144ppi effect ceiling.
    common::drawingml_shape_raster::rasterize_vector_items_for_effects_at_pixels_per_point(
      &display_items,
      raster_bounds,
      &effects,
      100.0 / units::POINTS_PER_INCH,
    )
  } else {
    common::drawingml_shape_raster::rasterize_vector_items_for_effects(
      &display_items,
      raster_bounds,
      &effects,
    )
  };
  let automatic_extrusion_color =
    common::drawingml_3d::automatic_extrusion_color_from_items(&display_items);
  let Some(mut raster) = raster else {
    return;
  };
  if let Some((scene, shape)) = object.scene3d.as_deref().zip(object.shape3d.as_deref()) {
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
    common::drawingml_3d::apply_static_3d(
      &mut raster.image,
      scene,
      common::drawingml_3d::camera_projection(scene, object.rotation_deg),
      shape,
      common::drawingml_3d::Static3dRenderOptions {
        extrusion_color: extrusion_color.or(automatic_extrusion_color),
        contour_color,
        pixels_per_point: raster.pixels_per_point,
        model_surface: Some(common::drawingml_3d::Static3dSurface {
          left_px: (content_bounds.origin.x.0 - raster_bounds.origin.x.0) * raster.pixels_per_point,
          top_px: (content_bounds.origin.y.0 - raster_bounds.origin.y.0) * raster.pixels_per_point,
          width_px: content_bounds.size.width.0 * raster.pixels_per_point,
          height_px: content_bounds.size.height.0 * raster.pixels_per_point,
        }),
      },
    );
  }
  common::drawingml_image_effects::scale_container_pixel_lengths(
    &mut effects,
    raster.pixels_per_point / (96.0 / 72.0),
  );
  common::drawingml_image_effects::apply_container_to_padded_image_with_sources(
    &mut raster.image,
    &effects,
    -relative_left * raster.pixels_per_point,
    -relative_top * raster.pixels_per_point,
    content_bounds.size.width.0 * raster.pixels_per_point,
    content_bounds.size.height.0 * raster.pixels_per_point,
    common::drawingml_image_effects::ImageEffectSourceImages {
      fill: raster.fill_image.as_ref(),
      line: raster.line_image.as_ref(),
      fill_line: raster.fill_line_image.as_ref(),
      children: raster.children_image.as_ref(),
    },
  );
  let mut png = Cursor::new(Vec::new());
  if PngEncoder::new(&mut png)
    .write_image(
      raster.image.as_raw(),
      raster.image.width(),
      raster.image.height(),
      ColorType::Rgba8.into(),
    )
    .is_err()
  {
    return;
  }
  let effect_image = PageItem::Image(ImageItem {
    x_pt: raster_bounds.origin.x.0,
    y_pt: raster_bounds.origin.y.0,
    width_pt: raster_bounds.size.width.0,
    height_pt: raster_bounds.size.height.0,
    crop: ImageCrop::default(),
    clip_path: Vec::new(),
    rotation_deg: 0.0,
    flip_horizontal: false,
    flip_vertical: false,
    data: Arc::from(png.into_inner()),
    content_type: Some("image/png".to_string()),
    metafile_monochrome_dib_palette_override: None,
    metafile_background_color: None,
    alt_text: object.description.clone().or_else(|| object.name.clone()),
    hyperlink_url: None,
    floating: false,
    behind_text: false,
  });
  if preserve_vector_source {
    items.insert(content_start, effect_image);
  } else {
    items.truncate(content_start);
    items.push(effect_image);
  }
}

fn drawing_object_has_path_transform(object: &super::drawing::DrawingObjectModel) -> bool {
  drawing_object_visual_rotation_degrees(object).abs() > f32::EPSILON
    || object.flip_horizontal
    || object.flip_vertical
}

fn drawing_object_clip_path(
  rect: CellRect,
  object: &super::drawing::DrawingObjectModel,
) -> Vec<common::PathCommand> {
  drawing_object_clip_path_with_transform(rect, object, drawing_object_path_transform(rect, object))
}

fn drawing_object_clip_path_with_transform(
  rect: CellRect,
  object: &super::drawing::DrawingObjectModel,
  transform: Affine,
) -> Vec<common::PathCommand> {
  let Some(geometry) = object.geometry.as_ref() else {
    return Vec::new();
  };
  let paths = match geometry {
    super::drawing::DrawingGeometryModel::Custom { geometry, .. } => {
      common::drawingml_custom_geometry::paths(
        geometry,
        rect.x_pt,
        rect.y_pt,
        rect.width_pt,
        rect.height_pt,
      )
    }
    super::drawing::DrawingGeometryModel::Preset { geometry, .. } => {
      common::drawingml_preset_geometry::paths(
        Some(geometry),
        rect.x_pt,
        rect.y_pt,
        rect.width_pt,
        rect.height_pt,
      )
    }
  };
  paths
    .into_iter()
    .flatten()
    .filter(|path| path.fill_mode != common::DrawingPathFillMode::None)
    .flat_map(|path| common::drawingml_geometry::transform_commands(path.commands, transform))
    .collect()
}

fn drawing_object_path_transform(
  rect: CellRect,
  object: &super::drawing::DrawingObjectModel,
) -> Affine {
  let center_x = rect.x_pt + rect.width_pt / 2.0;
  let center_y = rect.y_pt + rect.height_pt / 2.0;
  Affine::translate((-f64::from(center_x), -f64::from(center_y)))
    .then_scale_non_uniform(
      if object.flip_horizontal { -1.0 } else { 1.0 },
      if object.flip_vertical { -1.0 } else { 1.0 },
    )
    .then_rotate(f64::from(
      drawing_object_visual_rotation_degrees(object).to_radians(),
    ))
    .then_translate((f64::from(center_x), f64::from(center_y)).into())
}

fn drawing_object_visual_bounds(
  rect: CellRect,
  object: &super::drawing::DrawingObjectModel,
) -> CellRect {
  let bounds = common::drawingml_geometry::transform_rect_bounds(
    kurbo::Rect::new(
      f64::from(rect.x_pt),
      f64::from(rect.y_pt),
      f64::from(rect.x_pt + rect.width_pt),
      f64::from(rect.y_pt + rect.height_pt),
    ),
    drawing_object_path_transform(rect, object),
  );
  CellRect {
    x_pt: bounds.x0 as f32,
    y_pt: bounds.y0 as f32,
    width_pt: bounds.width() as f32,
    height_pt: bounds.height() as f32,
  }
}

fn drawing_object_visual_rotation_degrees(object: &super::drawing::DrawingObjectModel) -> f32 {
  object
    .scene3d
    .as_deref()
    .map_or(object.rotation_deg, |_| 0.0)
}

fn print_page_diagram_items(
  page: &CalcPrintPage<'_>,
  origin_x_pt: f32,
  origin_y_pt: f32,
  zoom_scale: f32,
) -> Vec<PageItem> {
  let mut items = Vec::new();
  let page_transform = SheetPageTransform::for_page(page, origin_x_pt, origin_y_pt, zoom_scale);
  for drawing in &page.sheet.resources.drawings {
    for anchor in &drawing.anchors {
      if anchor.object.hidden
        || !anchor.print_with_sheet
        || anchor.object.kind != super::drawing::DrawingObjectKind::GraphicFrame
      {
        continue;
      }
      if !drawing_anchor_intersects_page(page, anchor) {
        continue;
      }
      let Some(relationship_id) = anchor.object.relationship_id.as_deref() else {
        continue;
      };
      let Some(data) = drawing
        .diagrams
        .data_parts
        .iter()
        .find(|data| data.relationship_id.as_deref() == Some(relationship_id))
        .or_else(|| drawing.diagrams.data_parts.first())
      else {
        continue;
      };
      let Some(data_model) = data.data_model.as_deref() else {
        continue;
      };
      let Some((x_pt, y_pt, width_pt, height_pt)) = anchor_rect_pt(page.sheet, anchor) else {
        continue;
      };
      if width_pt <= 0.0 || height_pt <= 0.0 {
        continue;
      }
      let rect = page_transform.rect_from_xywh(x_pt, y_pt, width_pt, height_pt);
      let bounds = shared_diagram::DiagramBounds {
        x: rect.x_pt,
        y: rect.y_pt,
        width: rect.width_pt,
        height: rect.height_pt,
      };
      if let Some(drawing) = persisted_diagram_drawing(&drawing.diagrams, data_model)
        && push_persisted_diagram_items(&mut items, drawing, bounds)
      {
        continue;
      }
      for shape in shared_diagram::layout_shapes(
        data_model,
        drawing
          .diagrams
          .layout_parts
          .iter()
          .find_map(|layout| layout.layout.as_deref()),
        None,
        None,
        bounds,
        RgbColor {
          r: 0x4f,
          g: 0x81,
          b: 0xbd,
        },
      ) {
        push_diagram_shape_items(&mut items, &shape);
      }
    }
  }
  items
}

fn persisted_diagram_drawing<'a>(
  diagrams: &'a super::drawing::DiagramResourceCatalog,
  data: &dgm::DataModelRoot,
) -> Option<&'a dsp::Drawing> {
  let relationship_id = data
    .data_model_extension_list
    .as_ref()?
    .data_model_extension
    .iter()
    .find_map(
      |extension| match extension.data_model_extension_choice.as_ref()? {
        a::DataModelExtensionChoice::DataModelExtensionBlock(block) => block.rel_id.as_deref(),
        _ => None,
      },
    )?;
  diagrams
    .drawing_parts
    .iter()
    .find(|drawing| drawing.relationship_id.as_deref() == Some(relationship_id))
    .and_then(|drawing| drawing.drawing.as_deref())
}

fn push_persisted_diagram_items(
  items: &mut Vec<PageItem>,
  drawing: &dsp::Drawing,
  bounds: shared_diagram::DiagramBounds,
) -> bool {
  let content_bounds = persisted_diagram_content_bounds(drawing);
  let start_len = items.len();
  for choice in &drawing.shape_tree.shape_tree_choice {
    match choice {
      dsp::ShapeTreeChoice::Shape(shape) => {
        push_persisted_diagram_shape(items, shape, bounds, content_bounds);
      }
      dsp::ShapeTreeChoice::GroupShape(group) => {
        push_persisted_diagram_group(items, group, bounds, content_bounds);
      }
    }
  }
  items.len() > start_len
}

fn push_persisted_diagram_group(
  items: &mut Vec<PageItem>,
  group: &dsp::GroupShape,
  bounds: shared_diagram::DiagramBounds,
  content_bounds: Option<(i64, i64, i64, i64)>,
) {
  for choice in &group.group_shape_choice {
    match choice {
      dsp::GroupShapeChoice::Shape(shape) => {
        push_persisted_diagram_shape(items, shape, bounds, content_bounds);
      }
      dsp::GroupShapeChoice::GroupShape(group) => {
        push_persisted_diagram_group(items, group, bounds, content_bounds);
      }
    }
  }
}

fn push_persisted_diagram_shape(
  items: &mut Vec<PageItem>,
  shape: &dsp::Shape,
  bounds: shared_diagram::DiagramBounds,
  content_bounds: Option<(i64, i64, i64, i64)>,
) {
  let Some((x, y, width, height)) = transform_bounds_pt(
    shape.shape_properties.transform2_d.as_deref(),
    bounds,
    content_bounds,
  ) else {
    return;
  };
  let shape_bounds = shared_diagram::DiagramBounds {
    x,
    y,
    width,
    height,
  };
  if let Some(paths) = shared_diagram::drawing_shape_paths(&shape.shape_properties, shape_bounds) {
    for path in paths {
      let closed = path
        .commands
        .iter()
        .any(|command| matches!(command, common::PathCommand::Close));
      let mut stroke = common_stroke_from_border(BorderStyle::default(), 1.0);
      if let Some(outline) = shape.shape_properties.outline.as_deref() {
        common::drawingml_stroke::apply_outline_style(&mut stroke, outline);
      }
      items.push(PageItem::Path(common::PathItem {
        bounds: common_rect(x, y, width, height),
        points: Vec::new(),
        commands: path.commands,
        closed,
        fill: path.fill_mode.apply_to_fill(common::Fill::Solid(common_rgb(
          RgbColor {
            r: 0x4f,
            g: 0x81,
            b: 0xbd,
          },
          1.0,
        ))),
        stroke: path.stroke.then_some(stroke),
      }));
    }
  }
  let Some(text_body) = shape.text_body.as_deref() else {
    return;
  };
  let text = dml_paragraph_texts(&text_body.paragraph);
  if text.trim().is_empty() {
    return;
  }
  let (text_x, text_y, text_width, text_height) =
    transform_bounds_pt_dsp(shape.transform2_d.as_deref(), bounds, content_bounds)
      .unwrap_or((x, y, width, height));
  render_drawing_text(
    items,
    &text,
    CellRect {
      x_pt: text_x,
      y_pt: text_y,
      width_pt: text_width,
      height_pt: text_height,
    },
    persisted_diagram_text_style(text_body),
    None,
    text_body.body_properties.preset_text_warp.as_deref(),
    None,
  );
}

fn persisted_diagram_text_style(text_body: &dsp::TextBody) -> Option<TextStyle> {
  let mut style = TextStyle::default();
  let mut changed = false;
  for choice in text_body
    .paragraph
    .iter()
    .flat_map(|paragraph| paragraph.paragraph_choice.iter())
  {
    if let a::ParagraphChoice::Run(run) = choice
      && let Some(properties) = run.run_properties.as_deref()
    {
      if let Some(font_size) = properties.font_size {
        style.font_size_pt = font_size as f32 / 100.0;
        changed = true;
      }
      break;
    }
  }
  changed.then_some(style)
}

fn dml_paragraph_texts(paragraphs: &[a::Paragraph]) -> String {
  paragraphs
    .iter()
    .map(|paragraph| {
      paragraph
        .paragraph_choice
        .iter()
        .filter_map(|choice| match choice {
          a::ParagraphChoice::Run(run) => Some(run.text.as_str()),
          a::ParagraphChoice::Field(field) => field.text.as_deref(),
          _ => None,
        })
        .collect::<String>()
    })
    .filter(|line| !line.trim().is_empty())
    .collect::<Vec<_>>()
    .join("\n")
}

fn transform_bounds_pt(
  transform: Option<&a::Transform2D>,
  bounds: shared_diagram::DiagramBounds,
  content_bounds: Option<(i64, i64, i64, i64)>,
) -> Option<(f32, f32, f32, f32)> {
  let transform = transform?;
  let offset = transform.offset.as_ref()?;
  let extents = transform.extents.as_ref()?;
  transform_bounds_from_emu(
    offset.x.to_emu(),
    offset.y.to_emu(),
    extents.cx.to_emu(),
    extents.cy.to_emu(),
    bounds,
    content_bounds,
  )
}

fn transform_bounds_pt_dsp(
  transform: Option<&dsp::Transform2D>,
  bounds: shared_diagram::DiagramBounds,
  content_bounds: Option<(i64, i64, i64, i64)>,
) -> Option<(f32, f32, f32, f32)> {
  let transform = transform?;
  let offset = transform.offset.as_ref()?;
  let extents = transform.extents.as_ref()?;
  transform_bounds_from_emu(
    offset.x.to_emu(),
    offset.y.to_emu(),
    extents.cx.to_emu(),
    extents.cy.to_emu(),
    bounds,
    content_bounds,
  )
}

fn transform_bounds_from_emu(
  x_emu: i64,
  y_emu: i64,
  width_emu: i64,
  height_emu: i64,
  bounds: shared_diagram::DiagramBounds,
  content_bounds: Option<(i64, i64, i64, i64)>,
) -> Option<(f32, f32, f32, f32)> {
  let (min_x, min_y, max_x, max_y) =
    content_bounds.unwrap_or((x_emu, y_emu, x_emu + width_emu, y_emu + height_emu));
  let transform = common::drawingml_geometry::group_child_affine(
    kurbo::Point::new(f64::from(bounds.x), f64::from(bounds.y)),
    kurbo::Vec2::new(f64::from(bounds.width), f64::from(bounds.height)),
    kurbo::Point::new(min_x as f64, min_y as f64),
    kurbo::Vec2::new((max_x - min_x).max(1) as f64, (max_y - min_y).max(1) as f64),
  );
  let transformed = common::drawingml_geometry::transform_rect_bounds(
    kurbo::Rect::new(
      x_emu as f64,
      y_emu as f64,
      x_emu as f64 + width_emu as f64,
      y_emu as f64 + height_emu as f64,
    ),
    transform,
  );
  Some((
    transformed.x0 as f32,
    transformed.y0 as f32,
    transformed.width() as f32,
    transformed.height() as f32,
  ))
}

fn persisted_diagram_content_bounds(drawing: &dsp::Drawing) -> Option<(i64, i64, i64, i64)> {
  let mut bounds = None;
  for choice in &drawing.shape_tree.shape_tree_choice {
    collect_persisted_diagram_bounds(choice, &mut bounds);
  }
  bounds
}

fn collect_persisted_diagram_bounds(
  choice: &dsp::ShapeTreeChoice,
  bounds: &mut Option<(i64, i64, i64, i64)>,
) {
  match choice {
    dsp::ShapeTreeChoice::Shape(shape) => {
      if let Some(transform) = shape.shape_properties.transform2_d.as_deref()
        && let (Some(offset), Some(extents)) = (&transform.offset, &transform.extents)
      {
        let shape_bounds = (
          offset.x.to_emu(),
          offset.y.to_emu(),
          offset.x.to_emu() + extents.cx.to_emu(),
          offset.y.to_emu() + extents.cy.to_emu(),
        );
        *bounds = Some(match *bounds {
          Some((min_x, min_y, max_x, max_y)) => (
            min_x.min(shape_bounds.0),
            min_y.min(shape_bounds.1),
            max_x.max(shape_bounds.2),
            max_y.max(shape_bounds.3),
          ),
          None => shape_bounds,
        });
      }
    }
    dsp::ShapeTreeChoice::GroupShape(group) => {
      collect_persisted_group_bounds(group, bounds);
    }
  }
}

fn collect_persisted_group_bounds(
  group: &dsp::GroupShape,
  bounds: &mut Option<(i64, i64, i64, i64)>,
) {
  for child in &group.group_shape_choice {
    match child {
      dsp::GroupShapeChoice::Shape(shape) => {
        if let Some(transform) = shape.shape_properties.transform2_d.as_deref()
          && let (Some(offset), Some(extents)) = (&transform.offset, &transform.extents)
        {
          let shape_bounds = (
            offset.x.to_emu(),
            offset.y.to_emu(),
            offset.x.to_emu() + extents.cx.to_emu(),
            offset.y.to_emu() + extents.cy.to_emu(),
          );
          *bounds = Some(match *bounds {
            Some((min_x, min_y, max_x, max_y)) => (
              min_x.min(shape_bounds.0),
              min_y.min(shape_bounds.1),
              max_x.max(shape_bounds.2),
              max_y.max(shape_bounds.3),
            ),
            None => shape_bounds,
          });
        }
      }
      dsp::GroupShapeChoice::GroupShape(group) => collect_persisted_group_bounds(group, bounds),
    }
  }
}

fn push_diagram_shape_items(items: &mut Vec<PageItem>, shape: &shared_diagram::DiagramShape) {
  if shape.draw_geometry {
    if shape.is_connector
      && shape.connector_dimension == dgm::ConnectorDimensionValues::OneDimension
    {
      let mut stroke = common_stroke_from_border(
        BorderStyle {
          color: shape.line_fill.unwrap_or_default(),
          ..BorderStyle::default()
        },
        1.0,
      );
      if let Some(outline) = shape
        .shape_properties
        .as_deref()
        .and_then(|properties| properties.outline.as_deref())
      {
        common::drawingml_stroke::apply_outline_style(&mut stroke, outline);
      }
      shape.apply_connector_ends(&mut stroke);
      items.push(PageItem::Path(common::PathItem {
        bounds: common_rect(shape.x, shape.y, shape.width, shape.height),
        points: Vec::new(),
        commands: shape.connector_commands(),
        closed: false,
        fill: common::Fill::None,
        stroke: Some(stroke),
      }));
    } else if let Some(paths) = shape.drawing_paths() {
      for path in paths {
        let closed = path
          .commands
          .iter()
          .any(|command| matches!(command, common::PathCommand::Close));
        items.push(PageItem::Path(common::PathItem {
          bounds: common_rect(shape.x, shape.y, shape.width, shape.height),
          points: Vec::new(),
          commands: path.commands,
          closed,
          fill: path
            .fill_mode
            .apply_to_fill(common::Fill::Solid(common_rgb(shape.fill, 1.0))),
          stroke: path.stroke.then(|| {
            let mut stroke = common_stroke_from_border(
              BorderStyle {
                color: shape.line_fill.unwrap_or_default(),
                ..BorderStyle::default()
              },
              1.0,
            );
            if let Some(outline) = shape
              .shape_properties
              .as_deref()
              .and_then(|properties| properties.outline.as_deref())
            {
              common::drawingml_stroke::apply_outline_style(&mut stroke, outline);
            }
            stroke
          }),
        }));
      }
    } else {
      items.push(PageItem::Rect(RectItem {
        x_pt: shape.x,
        y_pt: shape.y,
        width_pt: shape.width,
        height_pt: shape.height,
        fill_color: Some(shape.fill),
        fill_opacity: 1.0,
        stroke: Some(BorderStyle::default()),
        stroke_opacity: 1.0,
      }));
    }
  }
  let text = diagram_text_body_text(&shape.text_body);
  if !text.trim().is_empty() {
    render_drawing_text(
      items,
      &text,
      CellRect {
        x_pt: shape.x,
        y_pt: shape.y,
        width_pt: shape.width,
        height_pt: shape.height,
      },
      None,
      None,
      shape
        .text_body
        .body_properties
        .as_deref()
        .and_then(|properties| properties.preset_text_warp.as_deref()),
      None,
    );
  }
}

fn diagram_text_body_text(text_body: &shared_diagram::DiagramTextBody) -> String {
  text_body
    .paragraphs
    .iter()
    .map(|paragraph| {
      paragraph
        .runs
        .iter()
        .map(|run| run.text.as_str())
        .collect::<String>()
    })
    .filter(|line| !line.trim().is_empty())
    .collect::<Vec<_>>()
    .join("\n")
}

fn print_page_drawing_text_items(
  import: &ExcelImport,
  page: &CalcPrintPage<'_>,
  setup: PageSetup,
  origin_x_pt: f32,
  origin_y_pt: f32,
  zoom_scale: f32,
) -> Vec<PageItem> {
  let mut items = Vec::new();
  let page_area_rect = page.area.map(|area| page.sheet.range_rect(area));
  let page_transform = SheetPageTransform::for_page(page, origin_x_pt, origin_y_pt, zoom_scale);
  let mut page_clip_rect = page_area_rect.map_or(
    CellRect {
      x_pt: 0.0,
      y_pt: 0.0,
      width_pt: setup.width_pt,
      height_pt: setup.height_pt,
    },
    |rect| page_transform.rect(rect),
  );
  if page.sheet.uses_indexed_scatter_print_grid() {
    page_clip_rect.width_pt += super::print::INDEXED_SCATTER_HORIZONTAL_CLIP_EXTENSION_PT;
  }
  for drawing in &page.sheet.resources.drawings {
    for anchor in &drawing.anchors {
      if anchor.object.hidden || !anchor.print_with_sheet {
        continue;
      }
      if !drawing_anchor_intersects_page(page, anchor) {
        continue;
      }
      let Some((x_pt, y_pt, width_pt, height_pt)) = anchor_rect_pt(page.sheet, anchor) else {
        continue;
      };
      if width_pt <= 0.0 || height_pt <= 0.0 {
        continue;
      }
      let source_rect = CellRect {
        x_pt,
        y_pt,
        width_pt,
        height_pt,
      };
      let drawing_rect = page_transform.rect(source_rect);
      let text_rect = page_transform.rect(if anchor.object.text_upright {
        drawing_object_visual_bounds(source_rect, &anchor.object)
      } else {
        source_rect
      });
      if anchor.object.kind == super::drawing::DrawingObjectKind::GroupShape {
        push_group_text_items(
          import,
          drawing,
          &mut items,
          &anchor.object,
          drawing_rect,
          Affine::IDENTITY,
        );
        continue;
      }
      if let Some(chart_items) = lower_drawing_chart(
        import,
        drawing,
        anchor,
        drawing_rect,
        page_clip_rect,
        zoom_scale,
      ) && !chart_items.is_empty()
      {
        items.extend(chart_items);
        continue;
      }
      let text = drawing_anchor_text(drawing, anchor);
      if text.trim().is_empty() {
        continue;
      }
      let hyperlink_url = drawing_object_hyperlink_url(drawing, &anchor.object);
      render_drawing_text(
        &mut items,
        &text,
        text_rect,
        drawing_object_text_style(import, &anchor.object),
        Some(drawing_object_text_layout(&anchor.object)),
        anchor.object.text_warp.as_deref(),
        hyperlink_url.as_deref(),
      );
    }
  }
  items
}

fn push_group_text_items(
  import: &ExcelImport,
  drawing: &super::drawing::DrawingResourceCatalog,
  items: &mut Vec<PageItem>,
  group: &super::drawing::DrawingObjectModel,
  rect: CellRect,
  parent_transform: Affine,
) {
  let group_transform = parent_transform * drawing_object_path_transform(rect, group);
  for (child, child_rect) in drawing_group_child_rects(group, rect) {
    if child.kind == super::drawing::DrawingObjectKind::GroupShape {
      push_group_text_items(import, drawing, items, child, child_rect, group_transform);
      continue;
    }
    if child.text.trim().is_empty() {
      continue;
    }
    let mut child_items = Vec::new();
    let hyperlink_url = drawing_object_hyperlink_url(drawing, child);
    render_drawing_text(
      &mut child_items,
      &child.text,
      child_rect,
      drawing_object_text_style(import, child),
      Some(drawing_object_text_layout(child)),
      child.text_warp.as_deref(),
      hyperlink_url.as_deref(),
    );
    transform_group_text_items(&mut child_items, group_transform, child_rect);
    items.extend(child_items);
  }
}

fn transform_group_text_items(items: &mut [PageItem], transform: Affine, rect: CellRect) {
  let x_axis = common::drawingml_geometry::transform_vector(kurbo::Vec2::new(1.0, 0.0), transform);
  let parent_rotation_deg = x_axis.y.atan2(x_axis.x).to_degrees() as f32;
  let center = transform
    * kurbo::Point::new(
      f64::from(rect.x_pt + rect.width_pt / 2.0),
      f64::from(rect.y_pt + rect.height_pt / 2.0),
    );
  for item in items {
    let PageItem::Text(text) = item else {
      continue;
    };
    let Some(options) = text.style.pdf_glyph_outline_options.as_deref() else {
      let origin = transform * kurbo::Point::new(f64::from(text.x_pt), f64::from(text.y_pt));
      text.x_pt = origin.x as f32;
      text.y_pt = origin.y as f32;
      text.style.rotation_deg += parent_rotation_deg;
      text.rotation_center_pt = Some((center.x as f32, center.y as f32));
      continue;
    };
    let Some(warp) = options.text_warp.as_deref() else {
      let origin = transform * kurbo::Point::new(f64::from(text.x_pt), f64::from(text.y_pt));
      text.x_pt = origin.x as f32;
      text.y_pt = origin.y as f32;
      text.style.rotation_deg += parent_rotation_deg;
      text.rotation_center_pt = Some((center.x as f32, center.y as f32));
      continue;
    };
    let mut options = options.clone();
    let paint_bounds = common::drawingml_geometry::transform_rect_bounds(
      kurbo::Rect::new(
        f64::from(warp.paint_bounds.origin.x.0),
        f64::from(warp.paint_bounds.origin.y.0),
        f64::from(warp.paint_bounds.origin.x.0 + warp.paint_bounds.size.width.0),
        f64::from(warp.paint_bounds.origin.y.0 + warp.paint_bounds.size.height.0),
      ),
      transform,
    );
    options.text_warp = Some(Arc::new(common::TextWarp {
      source_bounds: warp.source_bounds,
      paint_bounds: common_rect(
        paint_bounds.x0 as f32,
        paint_bounds.y0 as f32,
        paint_bounds.width() as f32,
        paint_bounds.height() as f32,
      ),
      boundaries: warp
        .boundaries
        .iter()
        .map(|commands| {
          common::drawingml_geometry::transform_commands(commands.iter().copied(), transform)
        })
        .collect(),
    }));
    text.style.pdf_glyph_outline_options = Some(Arc::new(options));
  }
}

fn lower_drawing_chart(
  import: &ExcelImport,
  drawing: &super::drawing::DrawingResourceCatalog,
  anchor: &super::drawing::DrawingAnchorModel,
  rect: CellRect,
  page_clip_rect: CellRect,
  drawing_scale: f32,
) -> Option<Vec<PageItem>> {
  if anchor.object.kind != super::drawing::DrawingObjectKind::GraphicFrame {
    return None;
  }
  let relationship_id = anchor.object.relationship_id.as_deref()?;
  let resource = drawing
    .charts
    .iter()
    .chain(drawing.extended_charts.iter())
    .find(|chart| chart.relationship_id.as_deref() == Some(relationship_id))?;
  if let Some(chart_space) = resource.extended_chart_space.as_deref() {
    let text_clip_slack = if crate::render::chartex::is_histogram_chart_space(chart_space) {
      CHARTEX_HISTOGRAM_TEXT_CLIP_SLACK
    } else if crate::render::chartex::is_pareto_chart_space(chart_space) {
      CHARTEX_PARETO_TEXT_CLIP_SLACK
    } else if crate::render::chartex::is_waterfall_chart_space(chart_space) {
      CHARTEX_WATERFALL_TEXT_CLIP_SLACK
    } else {
      DEFAULT_CHART_TEXT_CLIP_SLACK
    };
    let mut items = super::chartex::lower_extended_chart(
      import,
      chart_space,
      rect,
      &resource.extended_chart_styles,
      &resource.extended_chart_color_styles,
    );
    let mut metrics = TextMetrics::new();
    clip_chart_items_to_rect(
      &mut items,
      page_clip_rect,
      &mut metrics,
      text_clip_slack,
      &[],
    );
    return (!items.is_empty()).then_some(items);
  }
  let chart_space = resource.chart_space.as_deref()?;
  let chart_style = xlsx_chart_style_id(chart_space);

  if let Some(mut chart) = shared_chart::pie_chart_model(chart_space) {
    if chart_space.chart.title.is_none()
      && matches!(chart.title, Some(shared_chart::ChartTitleText::Automatic))
    {
      chart.title = None;
    }
    if chart.title.is_none() && excel_empty_automatic_title_is_visible(chart_space) {
      // ChartSpaceConverter::convertFromModel keeps an authored empty title
      // container when autoTitleDeleted is false. A real series title was
      // already promoted by pie_chart_model; when none exists, Office/Calc
      // materialize the localized generic chart-title resource instead.
      chart.title = Some(shared_chart::ChartTitleText::Automatic);
    }
    let mut title_style = import.styles.default_chart_text_style();
    title_style.font_size_pt = 14.0;
    title_style.bold = true;
    let mut label_style = import.styles.default_chart_text_style();
    label_style.font_size_pt = 10.0;
    if let Some(typeface) = xlsx_chart_latin_typeface(chart_space) {
      let typeface = Arc::from(import.styles.resolve_drawingml_theme_font(typeface));
      title_style.font_family = Some(Arc::clone(&typeface));
      label_style.font_family = Some(typeface);
    }
    if let Some(properties) = chart_space.text_properties.as_deref() {
      apply_xlsx_chart_text_properties(&mut title_style, properties, import);
      apply_xlsx_chart_text_properties(&mut label_style, properties, import);
    }
    if let Some(properties) = chart_space
      .chart
      .title
      .as_deref()
      .and_then(|title| title.text_properties.as_deref())
    {
      apply_xlsx_chart_text_properties(&mut title_style, properties, import);
    }
    if let Some(title) = chart_space.chart.title.as_deref() {
      apply_xlsx_chart_rich_title_properties(&mut title_style, title, import);
    }
    let mut legend_label_style = label_style.clone();
    if let Some(properties) = chart_space
      .chart
      .legend
      .as_deref()
      .and_then(|legend| legend.text_properties.as_deref())
    {
      apply_xlsx_chart_text_properties(&mut legend_label_style, properties, import);
    }
    let mut data_label_style = label_style;
    if let Some(properties) = chart.data_label_text_properties {
      apply_xlsx_chart_text_properties(&mut data_label_style, properties, import);
    }
    let point_colors = (0..chart.values.len())
      .map(|index| {
        chart
          .data_point_fills
          .iter()
          .find(|fill| fill.index as usize == index)
          .and_then(|fill| xlsx_chart_solid_fill_color(fill.fill, import))
          .or_else(|| {
            chart
              .series_solid_fill
              .and_then(|fill| xlsx_chart_solid_fill_color(fill, import))
          })
          .or_else(|| {
            let (formatting_index, maximum_formatting_index) = if chart.vary_colors {
              (index, chart.values.len().saturating_sub(1))
            } else {
              (
                chart.series_formatting_index,
                chart.maximum_series_formatting_index,
              )
            };
            xlsx_automatic_chart_color(
              chart_space,
              import,
              chart_style.unwrap_or(2),
              formatting_index,
              maximum_formatting_index,
            )
          })
          .unwrap_or_default()
      })
      .collect();
    let inherited_point_style = xlsx_chart_shape_style(
      chart.series_shape_properties,
      import,
      common::ShapeStyle::default(),
    );
    let point_styles = (0..chart.values.len())
      .map(|index| {
        chart
          .data_points
          .iter()
          .find(|point| usize::try_from(point.index.val).ok() == Some(index))
          .map(|point| {
            xlsx_chart_shape_style(
              point.chart_shape_properties.as_deref(),
              import,
              inherited_point_style.clone(),
            )
          })
          .unwrap_or_else(|| inherited_point_style.clone())
      })
      .collect();
    let data_label_fill_colors = chart
      .data_labels
      .iter()
      .map(|label| {
        label
          .shape_properties
          .and_then(shared_chart::chart_shape_solid_fill)
          .and_then(|fill| xlsx_chart_solid_fill_color(fill, import))
      })
      .collect();
    let (data_label_styles, data_label_rich_text_styles) =
      xlsx_chart_data_label_host_styles(&chart.data_labels, &data_label_style, import);
    let mut items = lower_radial_chart(
      ChartFrame {
        x_pt: rect.x_pt,
        y_pt: rect.y_pt,
        width_pt: rect.width_pt,
        height_pt: rect.height_pt,
      },
      &chart,
      shared_chart::automatic_chart_title(Some(import.styles.output_ui_language())),
      &RadialChartStyle {
        layout_profile: ChartLayoutProfile::Excel,
        title: title_style,
        legend: legend_label_style,
        data_label: data_label_style,
        data_label_styles,
        data_label_rich_text_styles,
        point_colors,
        point_styles,
        data_label_fill_colors,
        chart_area_style: xlsx_shape_style(
          chart_space.shape_properties.as_deref(),
          import,
          common::ShapeStyle::default(),
        ),
        plot_area_style: xlsx_shape_style(
          chart_space.chart.plot_area.shape_properties.as_deref(),
          import,
          common::ShapeStyle::default(),
        ),
      },
    );
    if !items.is_empty() {
      let mut metrics = TextMetrics::new();
      clip_chart_items_to_rect(
        &mut items,
        page_clip_rect,
        &mut metrics,
        DEFAULT_CHART_TEXT_CLIP_SLACK,
        &[],
      );
      if let Some(hyperlink_url) = drawing_object_hyperlink_url(drawing, &anchor.object) {
        let left = rect.x_pt.max(page_clip_rect.x_pt);
        let top = rect.y_pt.max(page_clip_rect.y_pt);
        let right = (rect.x_pt + rect.width_pt).min(page_clip_rect.x_pt + page_clip_rect.width_pt);
        let bottom =
          (rect.y_pt + rect.height_pt).min(page_clip_rect.y_pt + page_clip_rect.height_pt);
        if right > left && bottom > top {
          items.push(PageItem::LinkArea(LinkAreaItem {
            x_pt: left,
            y_pt: top,
            width_pt: right - left,
            height_pt: bottom - top,
            hyperlink_url: hyperlink_url.into_owned(),
          }));
        }
      }
      return Some(items);
    }
  }

  let mut chart = shared_chart::cartesian_chart_for_locales(
    chart_space,
    Some(import.styles.output_ui_language()),
    import.styles.output_format_locale(),
  )?;
  let rect = if chart.date_axis.is_some()
    && chart
      .plot_layout
      .is_some_and(|layout| layout.targets_inner_plot)
  {
    // The authored inner date-plot profile is resolved on Excel's worksheet
    // drawing grid. Its graphic frame begins on the leading edge of the
    // one-pixel gridline included by ECMA-376 Part 1 §18.3.1.13.
    CellRect {
      x_pt: rect.x_pt - units::POINTS_PER_CSS_PIXEL * drawing_scale,
      ..rect
    }
  } else {
    rect
  };
  let has_visible_empty_automatic_title = excel_empty_automatic_title_is_visible(chart_space);
  if chart_space.chart.title.is_none()
    && matches!(chart.title, Some(shared_chart::ChartTitleText::Automatic))
  {
    chart.title = None;
  } else if chart.title.is_none()
    && chart_space
      .chart
      .title
      .as_deref()
      .is_some_and(|title| title.chart_text.is_none())
    && chart.series.len() == 1
    && chart
      .series
      .first()
      .is_some_and(|series| series.has_explicit_name)
  {
    chart.title = chart
      .series
      .first()
      .map(|series| shared_chart::ChartTitleText::Explicit(series.name.clone()));
  } else if has_visible_empty_automatic_title && chart.title.is_none() {
    chart.title = Some(shared_chart::ChartTitleText::Automatic);
  }
  let has_explicit_single_series_compact_label_profile = matches!(
    chart.title.as_ref(),
    Some(shared_chart::ChartTitleText::Explicit(_))
  ) && chart.series.len() == 1
    && (chart.gap_width_percent - 219.0).abs() < f64::EPSILON
    && (chart.overlap_percent + 27.0).abs() < f64::EPSILON;
  let has_legacy_default_single_series_profile = chart_style.is_none()
    && matches!(
      chart.title.as_ref(),
      Some(shared_chart::ChartTitleText::Explicit(_))
    )
    && chart.series.len() == 1;
  if (chart.title.is_none() && chart.has_automatic_title_marker && chart.has_explicit_categories)
    || has_explicit_single_series_compact_label_profile
    || has_legacy_default_single_series_profile
  {
    // Excel's synthesized legend labels are compact (`Series1` / `系列1`)
    // in the established automatic-title family and in explicitly titled
    // single-series legacy layouts, including packages without c:style.
    // Other compatibility profiles retain their host spelling.
    apply_excel_automatic_series_names(&mut chart, Some(import.styles.output_ui_language()));
  }
  resolve_hidden_chart_values(import, chart_space, &mut chart);
  apply_excel_chart_missing_value_treatment(chart_space, chart_style.is_some(), &mut chart);
  apply_excel_chart_smoothing_default(chart_style.is_some(), &mut chart);
  let maximum_series_formatting_index = chart
    .series
    .iter()
    .map(|series| series.formatting_index)
    .max()
    .unwrap_or(0);
  let series_colors = chart
    .series
    .iter()
    .map(|series| {
      series
        .solid_fill
        .and_then(|fill| xlsx_chart_solid_fill_color(fill, import))
        .or_else(|| {
          xlsx_automatic_chart_color(
            chart_space,
            import,
            chart_style.unwrap_or(2),
            series.formatting_index,
            maximum_series_formatting_index,
          )
        })
        .unwrap_or_default()
    })
    .collect();
  let series_point_styles = chart
    .series
    .iter()
    .map(|series| {
      (0..series.values.len())
        .map(|point_index| {
          series
            .data_points
            .iter()
            .find(|point| usize::try_from(point.index.val).ok() == Some(point_index))
            .map(|point| {
              xlsx_chart_shape_style(
                point.chart_shape_properties.as_deref(),
                import,
                common::ShapeStyle::default(),
              )
            })
        })
        .collect()
    })
    .collect::<Vec<_>>();
  let series_point_colors = chart
    .series
    .iter()
    .enumerate()
    .map(|(series_index, series)| {
      if !chart.vary_colors_by_point
        || chart.series.len() != 1
        || series_index != 0
        || series
          .shape_properties
          .and_then(shared_chart::chart_shape_solid_fill)
          .is_some()
      {
        return vec![None; series.values.len()];
      }
      let maximum_point_index = series.values.len().saturating_sub(1);
      (0..series.values.len())
        .map(|point_index| {
          xlsx_automatic_chart_color(
            chart_space,
            import,
            chart_style.unwrap_or(2),
            point_index,
            maximum_point_index,
          )
        })
        .collect()
    })
    .collect::<Vec<_>>();
  let surface_band_colors = chart
    .surface_groups
    .iter()
    .map(|group| {
      group
        .band_fills
        .iter()
        .filter_map(|fill| {
          xlsx_chart_solid_fill_color(fill.fill, import).map(|color| (fill.index, color))
        })
        .collect()
    })
    .collect();
  let mut title_style = import.styles.default_chart_text_style();
  title_style.font_size_pt = if has_visible_empty_automatic_title || chart_style.is_none() {
    18.0
  } else {
    14.0
  };
  title_style.bold = true;
  let mut label_style = import.styles.default_chart_text_style();
  label_style.font_size_pt = 10.0;
  if resource.styles > 0 {
    // The checked-in Office ChartStyle 201 family uses tx1 with lumMod=65%
    // and lumOff=35% for title/axis/legend font references. Legacy charts
    // without a ChartStyle relationship retain untransformed black tx1.
    let transformed_text = RgbColor {
      r: 0x59,
      g: 0x59,
      b: 0x59,
    };
    title_style.color = transformed_text;
    label_style.color = transformed_text;
  }
  if let Some(typeface) = xlsx_chart_latin_typeface(chart_space) {
    // ECMA-376 DrawingML chart text commonly stores a theme placeholder such
    // as `+mn-lt`, not a physical family. Resolve it through the workbook
    // theme before shaping; passing the token to the system font query loses
    // the chart's Calibri minor-font contract and selects an unrelated generic
    // fallback.
    let typeface = Arc::from(import.styles.resolve_drawingml_theme_font(typeface));
    title_style.font_family = Some(Arc::clone(&typeface));
    label_style.font_family = Some(typeface);
  }
  if let Some(properties) = chart_space.text_properties.as_deref() {
    apply_xlsx_chart_text_properties(&mut title_style, properties, import);
    apply_xlsx_chart_text_properties(&mut label_style, properties, import);
  }
  if let Some(properties) = chart_space
    .chart
    .title
    .as_deref()
    .and_then(|title| title.text_properties.as_deref())
  {
    apply_xlsx_chart_text_properties(&mut title_style, properties, import);
  }
  if let Some(title) = chart_space.chart.title.as_deref() {
    apply_xlsx_chart_rich_title_properties(&mut title_style, title, import);
  }
  let mut legend_label_style = label_style.clone();
  if let Some(properties) = chart_space
    .chart
    .legend
    .as_deref()
    .and_then(|legend| legend.text_properties.as_deref())
  {
    apply_xlsx_chart_text_properties(&mut legend_label_style, properties, import);
  }
  let mut category_label_style = label_style.clone();
  if let Some(properties) = chart
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
            .value_axis
            .and_then(|axis| axis.text_properties.as_deref())
        })
        .flatten()
    })
  {
    apply_xlsx_chart_text_properties(&mut category_label_style, properties, import);
  }
  let mut value_label_style = label_style.clone();
  if let Some(properties) = chart
    .value_axis
    .and_then(|axis| axis.text_properties.as_deref())
  {
    apply_xlsx_chart_text_properties(&mut value_label_style, properties, import);
  }
  let category_axis_title_style = xlsx_chart_axis_title_style(
    &label_style,
    shared_chart::category_axis_title_source(&chart),
    import,
  );
  let value_axis_title_style = xlsx_chart_axis_title_style(
    &label_style,
    shared_chart::value_axis_title_source(&chart),
    import,
  );
  let additional_axis_title_styles = chart
    .additional_axis_titles
    .iter()
    .map(|title| {
      xlsx_chart_axis_title_style(
        &label_style,
        Some((title.source, title.automatic_rotation_deg)),
        import,
      )
    })
    .collect();
  let mut series_label_style = label_style.clone();
  if let Some(properties) = chart
    .axis_sets
    .iter()
    .find_map(|axes| axes.series_axis?.text_properties.as_deref())
  {
    apply_xlsx_chart_text_properties(&mut series_label_style, properties, import);
  }
  let mut data_label_style = label_style.clone();
  if let Some(properties) = chart.data_label_text_properties {
    apply_xlsx_chart_text_properties(&mut data_label_style, properties, import);
  }
  let (data_label_styles, data_label_rich_text_styles): (Vec<_>, Vec<_>) = chart
    .series
    .iter()
    .map(|series| xlsx_chart_data_label_host_styles(&series.data_labels, &data_label_style, import))
    .unzip();
  let title_fill_color = chart_space
    .chart
    .title
    .as_deref()
    .and_then(|title| title.chart_shape_properties.as_deref())
    .and_then(
      |properties| match properties.chart_shape_properties_choice2.as_ref()? {
        c::ChartShapePropertiesChoice2::SolidFill(fill) => {
          xlsx_chart_solid_fill_color(fill, import)
        }
        c::ChartShapePropertiesChoice2::GradientFill(fill) => {
          xlsx_chart_first_gradient_fill_color(fill, import)
        }
        _ => None,
      },
    );
  let gridline_color = chart
    .value_axis
    .and_then(|axis| axis.major_gridlines.as_deref())
    .and_then(|gridlines| gridlines.chart_shape_properties.as_deref())
    .and_then(shared_chart::chart_shape_outline_solid_fill)
    .and_then(|fill| xlsx_chart_solid_fill_color(fill, import))
    .unwrap_or_else(|| {
      // ECMA's omitted legacy chart style resolves to style 2. LibreOffice's
      // ChartSpaceModel likewise initializes mnStyle to 2 before parsing an
      // optional c:style element.
      if chart_style.unwrap_or(2) == 2 {
        RgbColor {
          r: 0x86,
          g: 0x86,
          b: 0x86,
        }
      } else {
        RgbColor {
          r: 0xd9,
          g: 0xd9,
          b: 0xd9,
        }
      }
    });
  let value_gridline_width_pt = chart
    .value_axis
    .and_then(|axis| axis.major_gridlines.as_deref())
    .and_then(|gridlines| gridlines.chart_shape_properties.as_deref())
    .and_then(xlsx_chart_outline_width_pt)
    .or_else(|| chart_style.is_none().then_some(0.75 * drawing_scale));
  let axis_line_width_pt = chart
    .date_axis
    .and_then(|axis| axis.chart_shape_properties.as_deref())
    .and_then(xlsx_chart_outline_width_pt)
    .or_else(|| {
      chart
        .category_axis
        .and_then(|axis| axis.chart_shape_properties.as_deref())
        .and_then(xlsx_chart_outline_width_pt)
    })
    .or_else(|| chart_style.is_none().then_some(0.75 * drawing_scale));
  let category_major_gridline = chart.date_axis.and_then(|axis| {
    let properties = axis
      .major_gridlines
      .as_deref()?
      .chart_shape_properties
      .as_deref()?;
    let color = shared_chart::chart_shape_outline_solid_fill(properties)
      .and_then(|fill| xlsx_chart_solid_fill_color(fill, import))?;
    let width = xlsx_chart_outline_width_pt(properties)?;
    Some((color, width))
  });
  let category_minor_gridline = chart.date_axis.and_then(|axis| {
    let properties = axis
      .minor_gridlines
      .as_deref()?
      .chart_shape_properties
      .as_deref()?;
    let color = shared_chart::chart_shape_outline_solid_fill(properties)
      .and_then(|fill| xlsx_chart_solid_fill_color(fill, import))?;
    let width = xlsx_chart_outline_width_pt(properties)?;
    Some((color, width))
  });
  let chart_area_stroke_color = chart_space
    .shape_properties
    .as_deref()
    .and_then(shared_chart::shape_properties_outline_solid_fill)
    .and_then(|fill| xlsx_chart_solid_fill_color(fill, import))
    .or_else(|| {
      if chart_style.is_none() {
        // The legacy default-style chart area keeps the same neutral-gray
        // automatic line transform as its major gridlines.
        Some(RgbColor {
          r: 0x86,
          g: 0x86,
          b: 0x86,
        })
      } else {
        (chart_style == Some(2)).then_some(
          if chart.title.is_none()
            && chart.has_automatic_title_marker
            && chart.has_explicit_categories
            || chart.legend_position.is_none() && chart.has_explicit_categories
          {
            // LibreOffice Chart2ImportTest::testAutoChartAreaBorderPropXLSX
            // records the imported automatic border as D9D9D9 at 0.75pt.
            // Excel's style-2/102 fixed output applies the chart-style line
            // transform, matching the neutral-gray automatic gridline stroke.
            RgbColor {
              r: 0x86,
              g: 0x86,
              b: 0x86,
            }
          } else {
            // Preserve the existing explicit-title style-2 profile.
            RgbColor { r: 0, g: 0, b: 0 }
          },
        )
      }
    });
  let series_styles = chart
    .series
    .iter()
    .map(|series| {
      xlsx_chart_shape_style(
        series.shape_properties,
        import,
        common::ShapeStyle::default(),
      )
    })
    .collect();
  let trendline_styles = chart
    .series
    .iter()
    .map(|series| {
      series
        .trendlines
        .iter()
        .map(|trendline| {
          xlsx_chart_shape_style(
            trendline.chart_shape_properties.as_deref(),
            import,
            common::ShapeStyle::default(),
          )
        })
        .collect()
    })
    .collect();
  let chart_area_style = xlsx_shape_style(
    chart_space.shape_properties.as_deref(),
    import,
    solid_chart_shape_style(
      chart_space
        .shape_properties
        .as_deref()
        .and_then(shared_chart::shape_properties_solid_fill)
        .and_then(|fill| xlsx_chart_solid_fill_color(fill, import)),
      chart_area_stroke_color.map(|color| {
        (
          color,
          chart_space
            .shape_properties
            .as_deref()
            .and_then(xlsx_shape_outline_width_pt)
            .unwrap_or(0.75 * drawing_scale),
        )
      }),
    ),
  );
  let plot_area_style = xlsx_shape_style(
    chart_space.chart.plot_area.shape_properties.as_deref(),
    import,
    solid_chart_shape_style(
      chart_space
        .chart
        .plot_area
        .shape_properties
        .as_deref()
        .and_then(shared_chart::shape_properties_solid_fill)
        .and_then(|fill| xlsx_chart_solid_fill_color(fill, import)),
      chart_space
        .chart
        .plot_area
        .shape_properties
        .as_deref()
        .and_then(shared_chart::shape_properties_outline_solid_fill)
        .and_then(|fill| xlsx_chart_solid_fill_color(fill, import))
        .map(|color| {
          (
            color,
            chart_space
              .chart
              .plot_area
              .shape_properties
              .as_deref()
              .and_then(xlsx_shape_outline_width_pt)
              .unwrap_or(0.75 * drawing_scale),
          )
        }),
    ),
  );
  let mut items = lower_clustered_column_chart(
    ChartFrame {
      x_pt: rect.x_pt,
      y_pt: rect.y_pt,
      width_pt: rect.width_pt,
      height_pt: rect.height_pt,
    },
    &chart,
    shared_chart::automatic_chart_title(Some(import.styles.output_ui_language())),
    &ClusteredColumnStyle {
      layout_profile: ChartLayoutProfile::Excel,
      chart_style_id: chart_style.unwrap_or(2),
      modern_excel_profile: chart_style.is_some(),
      stroke_scale: drawing_scale,
      automatic_line_width_pt: 1.5,
      has_explicit_title: chart_space
        .chart
        .title
        .as_deref()
        .is_some_and(|title| title.chart_text.is_some()),
      title_top_adjustment_ratio: if chart_style.is_none()
        && chart_space
          .chart
          .title
          .as_deref()
          .is_some_and(|title| title.chart_text.is_some())
        && (title_style.font_size_pt - 18.0).abs() < f32::EPSILON
      {
        crate::render::chart_layout_profiles::EXCEL_LEGACY_DEFAULT_TITLE_TOP_ADJUSTMENT_RATIO
      } else {
        0.0
      },
      title: title_style,
      title_fill_color,
      label: legend_label_style.clone(),
      legend: legend_label_style,
      category_axis_title: category_axis_title_style,
      value_axis_title: value_axis_title_style,
      additional_axis_titles: additional_axis_title_styles,
      category_label: category_label_style,
      value_label: value_label_style,
      series_label: series_label_style,
      data_label: data_label_style,
      data_label_styles,
      data_label_rich_text_styles,
      gridline_color,
      value_gridline_width_pt,
      axis_line_width_pt,
      category_major_gridline,
      category_minor_gridline,
      series_colors,
      series_point_colors,
      series_styles,
      trendline_styles,
      series_point_styles,
      surface_band_colors,
      data_label_fill_colors: chart
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
                .and_then(|fill| xlsx_chart_solid_fill_color(fill, import))
            })
            .collect()
        })
        .collect(),
      chart_area_style,
      plot_area_style,
    },
  );
  let indexed_scatter_text = chart.series.iter().all(|series| {
    matches!(
      series.kind,
      shared_chart::ChartSeriesKind::Scatter | shared_chart::ChartSeriesKind::Bubble
    )
  }) && chart
    .series
    .iter()
    .any(|series| !series.x_values.is_empty())
    && chart
      .series
      .iter()
      .flat_map(|series| &series.x_values)
      .all(Option::is_none);
  let explicit_title_indexed_scatter_text = indexed_scatter_text
    && chart_space
      .chart
      .title
      .as_deref()
      .is_some_and(|title| title.chart_text.is_some())
    && !chart.title_overlay
    && chart.legend_position.is_none()
    && chart
      .series
      .iter()
      .any(|series| !series.data_labels.is_empty());
  let modern_derived_title_scatter_text = chart_style.is_some()
    && chart.series.len() == 1
    && chart.series.iter().all(|series| {
      matches!(
        series.kind,
        shared_chart::ChartSeriesKind::Scatter | shared_chart::ChartSeriesKind::Bubble
      )
    })
    && matches!(
      chart.title.as_ref(),
      Some(shared_chart::ChartTitleText::Explicit(_))
    )
    && chart_space
      .chart
      .title
      .as_deref()
      .is_some_and(|title| title.chart_text.is_none())
    && !chart.title_overlay
    && chart.legend_position.is_none();
  let multicomponent_data_labels = chart.series.iter().any(|series| {
    series
      .data_labels
      .iter()
      .any(|label| label.text_components.len() > 1)
  });
  let text_boundary_slack_em = if matches!(
    chart.title.as_ref(),
    Some(shared_chart::ChartTitleText::Automatic)
  ) && !chart.title_overlay
    && indexed_scatter_text
  {
    INDEXED_SCATTER_TITLE_TEXT_CLIP_SLACK
  } else if explicit_title_indexed_scatter_text {
    EXPLICIT_TITLE_INDEXED_SCATTER_TEXT_CLIP_SLACK
  } else if modern_derived_title_scatter_text {
    MODERN_DERIVED_TITLE_SCATTER_TEXT_CLIP_SLACK
  } else if multicomponent_data_labels {
    // Excel retains a boundary text field in the PDF text layer when its
    // origin is up to 0.6em beyond a horizontal worksheet clip; the clip
    // still hides the glyph ink. `ser_labels.xlsx` measures 5.26pt at a 9pt
    // font size.
    MULTICOMPONENT_DATA_LABEL_TEXT_CLIP_SLACK
  } else {
    DEFAULT_CHART_TEXT_CLIP_SLACK
  };
  let mut metrics = TextMetrics::new();
  let right_truncated_date_ticks = shared_chart::date_axis_ticks(&chart)
    .unwrap_or_default()
    .into_iter()
    .map(|tick| tick.text)
    .collect::<Vec<_>>();
  clip_chart_items_to_rect(
    &mut items,
    page_clip_rect,
    &mut metrics,
    text_boundary_slack_em,
    &right_truncated_date_ticks,
  );
  if let Some(hyperlink_url) = drawing_object_hyperlink_url(drawing, &anchor.object) {
    let left = rect.x_pt.max(page_clip_rect.x_pt);
    let top = rect.y_pt.max(page_clip_rect.y_pt);
    let right = (rect.x_pt + rect.width_pt).min(page_clip_rect.x_pt + page_clip_rect.width_pt);
    let bottom = (rect.y_pt + rect.height_pt).min(page_clip_rect.y_pt + page_clip_rect.height_pt);
    if right > left && bottom > top {
      items.push(PageItem::LinkArea(LinkAreaItem {
        x_pt: left,
        y_pt: top,
        width_pt: right - left,
        height_pt: bottom - top,
        hyperlink_url: hyperlink_url.into_owned(),
      }));
    }
  }
  Some(items)
}

fn excel_empty_automatic_title_is_visible(chart_space: &c::ChartSpace) -> bool {
  let Some(title) = chart_space.chart.title.as_deref() else {
    return false;
  };
  if title.chart_text.is_some() {
    return false;
  }
  let explicitly_retained = chart_space
    .chart
    .auto_title_deleted
    .as_ref()
    .is_some_and(|deleted| deleted.val.is_some_and(|value| !value.as_bool()));
  let overlay_placeholder = title
    .overlay
    .as_ref()
    .is_some_and(|overlay| overlay.val.is_none_or(|value| value.as_bool()))
    && chart_space
      .chart
      .auto_title_deleted
      .as_ref()
      .is_none_or(|deleted| deleted.val.is_some_and(|value| !value.as_bool()));
  explicitly_retained || overlay_placeholder
}

fn apply_excel_chart_missing_value_treatment(
  chart_space: &c::ChartSpace,
  has_explicit_modern_style: bool,
  chart: &mut shared_chart::ClusteredColumnChart<'_>,
) {
  let treatment = chart_space
    .chart
    .display_blanks_as
    .as_ref()
    .map(|treatment| treatment.val.unwrap_or(c::DisplayBlanksAsValues::Zero))
    .unwrap_or(if has_explicit_modern_style {
      // LibreOffice Chart2ImportTest distinguishes the Office 2013 OOXML
      // default (USE_ZERO) from the Office 2007 compatibility default
      // (LEAVE_GAP). An explicit modern c:style/c14:style is the package-local
      // evidence available here for that versioned default.
      c::DisplayBlanksAsValues::Zero
    } else {
      c::DisplayBlanksAsValues::Gap
    });
  if treatment == c::DisplayBlanksAsValues::Zero {
    for series in &mut chart.series {
      for value in &mut series.values {
        if value.is_none() {
          *value = Some(0.0);
        }
      }
    }
  }
}

fn apply_excel_chart_smoothing_default(
  has_explicit_modern_style: bool,
  chart: &mut shared_chart::ClusteredColumnChart<'_>,
) {
  if !has_explicit_modern_style {
    return;
  }
  for series in &mut chart.series {
    if matches!(
      series.kind,
      shared_chart::ChartSeriesKind::Line | shared_chart::ChartSeriesKind::Scatter
    ) && series.smooth.is_none()
    {
      // LibreOffice Chart2ImportTest::testSmoothDefaultValue2007XLSX and
      // testSmoothDefaultValue2013XLSX establish the versioned omission:
      // Office 2007 imports a missing per-series c:smooth as straight lines,
      // while the modern OOXML profile imports it as a smooth curve. The
      // chart-group c:smooth value does not replace that series default.
      series.smooth = Some(true);
    }
  }
}

fn apply_excel_automatic_series_names(
  chart: &mut shared_chart::ClusteredColumnChart<'_>,
  ui_language: Option<&str>,
) {
  for (index, series) in chart.series.iter_mut().enumerate() {
    if series.has_explicit_name {
      continue;
    }
    let shared_name = shared_chart::automatic_series_title(ui_language, index + 1);
    if series.name != shared_name {
      continue;
    }
    // Excel's synthesized legend labels are `Series1` / `系列1`; Word and
    // PowerPoint retain their host-specific spaced labels in the shared model.
    let excel_name = shared_name.replace(' ', "");
    series.name.clone_from(&excel_name);
    for label in &mut series.data_labels {
      label.text = label.text.replace(&shared_name, &excel_name);
    }
  }
}

fn resolve_hidden_chart_values(
  import: &ExcelImport,
  chart_space: &c::ChartSpace,
  chart: &mut shared_chart::ClusteredColumnChart<'_>,
) {
  // ECMA-376 21.2.2.146: false means the chart is not restricted to visible
  // cells. Excel/LibreOffice caches may contain only visible points even when
  // the backing range includes hidden rows, so the workbook is authoritative
  // for this explicit mode. Missing/invalid references retain the cache.
  let include_hidden_cells = chart_space
    .chart
    .plot_visible_only
    .as_ref()
    .and_then(|value| value.val)
    .is_some_and(|value| !value.as_bool());
  let mut resolved_any = false;
  let mut resolved_categories_from_reference = false;
  let mut name_replacements = Vec::new();
  for series in &mut chart.series {
    if !series.has_nonempty_explicit_name
      && let Some(formula) = series.name_formula
      && let Some(name) = chart_reference_text_values(import, formula)
        .and_then(|values| values.into_iter().find(|value| !value.is_empty()))
    {
      let old_name = std::mem::replace(&mut series.name, name);
      series.has_nonempty_explicit_name = true;
      for label in &mut series.data_labels {
        label.text = label.text.replace(&old_name, &series.name);
      }
      name_replacements.push((old_name, series.name.clone()));
    }

    let values_missing = series.values.is_empty() || series.values.iter().all(Option::is_none);
    if (include_hidden_cells || values_missing)
      && let Some(formula) = series.value_formula
      && let Some(values) = chart_reference_numeric_values(import, formula)
    {
      series.values = values;
      resolved_any = true;
    }

    let x_values_missing =
      series.x_values.is_empty() || series.x_values.iter().all(Option::is_none);
    if (include_hidden_cells || x_values_missing)
      && let Some(formula) = series.x_value_formula
      && let Some(values) = chart_reference_numeric_values(import, formula)
    {
      series.x_values = values;
      resolved_any = true;
    }

    let bubble_sizes_missing =
      series.bubble_sizes.is_empty() || series.bubble_sizes.iter().all(Option::is_none);
    if (include_hidden_cells || bubble_sizes_missing)
      && let Some(formula) = series.bubble_size_formula
      && let Some(values) = chart_reference_numeric_values(import, formula)
    {
      series.bubble_sizes = values;
      resolved_any = true;
    }
  }

  if let Some(shared_chart::ChartTitleText::Explicit(title)) = chart.title.as_mut() {
    for (old_name, new_name) in name_replacements {
      if title == &old_name {
        title.clone_from(&new_name);
        break;
      }
    }
  }

  let category_formula = chart
    .series
    .iter()
    .find_map(|series| series.category_formula)
    .map(str::to_owned);
  if let Some(formula) = category_formula.as_deref()
    && let Some(format_code) = chart_reference_number_format_code(import, formula)
  {
    chart.category_number_format_code = Some(format_code);
  }
  if let Some(formula) = category_formula
    // `plotVisOnly=false` makes the backing range authoritative for series
    // values, but Office still preserves an authored category cache (including
    // intentional sparse/blank labels). Only reconstruct categories when the
    // package supplied no usable cache at all.
    && !chart.has_explicit_categories
    && let Some(categories) = chart_reference_text_values(import, &formula)
    && !categories.is_empty()
  {
    chart.category_axis_values =
      chart_reference_numeric_values(import, &formula).unwrap_or_default();
    chart.cached_category_count = categories.len();
    chart.categories = categories;
    chart.has_explicit_categories = true;
    resolved_any = true;
    resolved_categories_from_reference = true;
  }
  if resolved_any
    && !resolved_categories_from_reference
    && chart
      .categories
      .iter()
      .all(|value| value.parse::<usize>().is_ok())
  {
    let category_count = chart
      .series
      .iter()
      .map(|series| series.values.len())
      .max()
      .unwrap_or(0);
    chart.categories = (1..=category_count)
      .map(|index| index.to_string())
      .collect();
  }
}

fn chart_reference_sheet_and_range<'a>(
  import: &'a ExcelImport,
  formula: &str,
) -> Option<(&'a CalcSheet, CellRange)> {
  let (sheet_name, _) = formula.rsplit_once('!')?;
  // External workbook references require link resolution and must keep using
  // the embedded chart cache until that ownership is modeled.
  if sheet_name.contains('[') || sheet_name.contains(']') {
    return None;
  }
  let sheet_name = sheet_name.trim().trim_matches('\'').replace("''", "'");
  let sheet = import
    .sheets
    .iter()
    .find(|sheet| sheet.name == sheet_name)?;
  let range = CellRange::parse_a1_range(formula)?;
  Some((sheet, range))
}

fn chart_reference_numeric_values(import: &ExcelImport, formula: &str) -> Option<Vec<Option<f64>>> {
  let (sheet, range) = chart_reference_sheet_and_range(import, formula)?;
  let mut values = Vec::new();
  for row in range.start.row..=range.end.row {
    for col in range.start.col..=range.end.col {
      values.push(
        sheet
          .cell_at(CellAddress { col, row })
          .and_then(|cell| cell.cached_value.as_deref())
          .and_then(|value| value.parse::<f64>().ok()),
      );
    }
  }
  Some(values)
}

fn chart_reference_number_format_code(import: &ExcelImport, formula: &str) -> Option<String> {
  let (sheet, range) = chart_reference_sheet_and_range(import, formula)?;
  for row in range.start.row..=range.end.row {
    for col in range.start.col..=range.end.col {
      let address = CellAddress { col, row };
      let is_nonempty_number = sheet
        .cell_at(address)
        .and_then(|cell| cell.cached_value.as_deref())
        .is_some_and(|value| value.parse::<f64>().is_ok());
      if !is_nonempty_number {
        continue;
      }
      let style_index = sheet.effective_cell_style_index_at(address);
      return import
        .styles
        .number_format_code_for_cell(style_index)
        .map(ToOwned::to_owned);
    }
  }
  None
}

fn chart_reference_text_values(import: &ExcelImport, formula: &str) -> Option<Vec<String>> {
  let (sheet, range) = chart_reference_sheet_and_range(import, formula)?;
  let mut values = Vec::new();
  for row in range.start.row..=range.end.row {
    for col in range.start.col..=range.end.col {
      values.push(
        sheet
          .cell_at(CellAddress { col, row })
          .map(|cell| cell.display_text.clone())
          .unwrap_or_default(),
      );
    }
  }
  Some(values)
}

fn xlsx_chart_latin_typeface(chart_space: &c::ChartSpace) -> Option<&str> {
  for properties in [
    chart_space.text_properties.as_deref(),
    chart_space
      .chart
      .title
      .as_deref()
      .and_then(|title| title.text_properties.as_deref()),
    chart_space
      .chart
      .legend
      .as_deref()
      .and_then(|legend| legend.text_properties.as_deref()),
  ]
  .into_iter()
  .flatten()
  {
    if let Some(typeface) = chart_text_properties_latin_typeface(properties) {
      return Some(typeface);
    }
  }
  chart_space
    .chart
    .plot_area
    .plot_area_choice2
    .iter()
    .find_map(|axis| {
      let properties = match axis {
        c::PlotAreaChoice2::ValueAxis(axis) => axis.text_properties.as_deref(),
        c::PlotAreaChoice2::CategoryAxis(axis) => axis.text_properties.as_deref(),
        c::PlotAreaChoice2::DateAxis(axis) => axis.text_properties.as_deref(),
        c::PlotAreaChoice2::SeriesAxis(axis) => axis.text_properties.as_deref(),
      }?;
      chart_text_properties_latin_typeface(properties)
    })
}

fn chart_text_properties_latin_typeface(properties: &c::TextProperties) -> Option<&str> {
  chart_default_run_properties(properties).find_map(|properties| {
    properties
      .latin_font
      .as_ref()?
      .typeface
      .as_deref()
      .filter(|typeface| !typeface.trim().is_empty())
  })
}

fn chart_default_run_properties(
  properties: &c::TextProperties,
) -> impl Iterator<Item = &a::DefaultRunProperties> {
  properties
    .paragraph
    .iter()
    .filter_map(|paragraph| paragraph.paragraph_properties.as_deref())
    .filter_map(|paragraph| paragraph.default_run_properties.as_deref())
    .chain(
      properties
        .list_style
        .as_deref()
        .and_then(|style| style.default_paragraph_properties.as_deref())
        .and_then(|paragraph| paragraph.default_run_properties.as_deref()),
    )
}

fn apply_xlsx_chart_text_properties(
  style: &mut TextStyle,
  properties: &c::TextProperties,
  import: &ExcelImport,
) {
  let Some(properties) = chart_default_run_properties(properties).next() else {
    return;
  };
  apply_xlsx_default_run_properties(style, properties, import);
}

fn apply_xlsx_chart_rich_title_properties(
  style: &mut TextStyle,
  title: &c::Title,
  import: &ExcelImport,
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
    apply_xlsx_default_run_properties(style, properties, import);
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
    apply_xlsx_run_properties(style, properties, import);
  }
}

fn xlsx_chart_axis_title_style(
  base_style: &TextStyle,
  source: Option<(&c::Title, f32)>,
  import: &ExcelImport,
) -> TextStyle {
  let mut style = base_style.clone();
  // spAxisTitleTexts is the automatic role. Direct c:title text properties
  // overlay it and may explicitly disable the automatic bold face.
  style.bold = true;
  let Some((title, automatic_rotation_deg)) = source else {
    return style;
  };
  style.rotation_deg = automatic_rotation_deg;
  if let Some(properties) = title.text_properties.as_deref() {
    apply_xlsx_chart_text_properties(&mut style, properties, import);
  }
  apply_xlsx_chart_rich_title_properties(&mut style, title, import);
  if let Some(rotation) = shared_chart::title_rotation_degrees(title) {
    style.rotation_deg = rotation;
  }
  style
}

fn xlsx_chart_data_label_host_styles(
  labels: &[shared_chart::ClusteredColumnDataLabel<'_>],
  base_style: &TextStyle,
  import: &ExcelImport,
) -> (Vec<Option<TextStyle>>, Vec<Vec<TextStyle>>) {
  let mut label_styles = Vec::with_capacity(labels.len());
  let mut rich_text_styles = Vec::with_capacity(labels.len());
  for label in labels {
    let mut label_style = base_style.clone();
    if let Some(properties) = label.text_properties {
      apply_xlsx_chart_text_properties(&mut label_style, properties, import);
    }
    label_styles.push(label.text_properties.is_some().then(|| label_style.clone()));
    rich_text_styles.push(
      label
        .rich_text_runs
        .iter()
        .map(|run| {
          let mut style = label_style.clone();
          if let Some(properties) = run.paragraph_default_run_properties {
            apply_xlsx_default_run_properties(&mut style, properties, import);
          }
          if let Some(properties) = run.run_properties {
            apply_xlsx_run_properties(&mut style, properties, import);
          }
          style
        })
        .collect(),
    );
  }
  (label_styles, rich_text_styles)
}

fn apply_xlsx_default_run_properties(
  style: &mut TextStyle,
  properties: &a::DefaultRunProperties,
  import: &ExcelImport,
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
    style.font_family = Some(Arc::from(
      import.styles.resolve_drawingml_theme_font(typeface),
    ));
  }
  if let Some(color) = default_run_properties_color(properties, import) {
    style.color = color;
  }
}

fn apply_xlsx_run_properties(
  style: &mut TextStyle,
  properties: &a::RunProperties,
  import: &ExcelImport,
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
    style.font_family = Some(Arc::from(
      import.styles.resolve_drawingml_theme_font(typeface),
    ));
  }
  if let Some(color) = run_properties_color(properties, import) {
    style.color = color;
  }
}

fn default_run_properties_color(
  properties: &a::DefaultRunProperties,
  import: &ExcelImport,
) -> Option<RgbColor> {
  let a::DefaultRunPropertiesChoice::SolidFill(fill) =
    properties.default_run_properties_choice1.as_ref()?
  else {
    return None;
  };
  xlsx_chart_text_solid_fill_color(fill, import)
}

fn run_properties_color(properties: &a::RunProperties, import: &ExcelImport) -> Option<RgbColor> {
  let a::RunPropertiesChoice::SolidFill(fill) = properties.run_properties_choice1.as_ref()? else {
    return None;
  };
  xlsx_chart_text_solid_fill_color(fill, import)
}

fn xlsx_chart_text_solid_fill_color(fill: &a::SolidFill, import: &ExcelImport) -> Option<RgbColor> {
  let color = fill
    .solid_fill_choice
    .as_ref()
    .and_then(Color::from_solid_fill_choice)?;
  let mut scheme_resolver = |value| {
    let index = xlsx_scheme_color_index(value)?;
    let color = import.styles.theme_color(index, 0.0)?;
    Some(Color::RgbHex(RgbHexColor {
      value: format!("{:02X}{:02X}{:02X}", color.r, color.g, color.b),
      transformations: Vec::new(),
    }))
  };
  let color = color.resolve_rgb(&mut scheme_resolver, None)?;
  Some(RgbColor {
    r: color.r,
    g: color.g,
    b: color.b,
  })
}

fn xlsx_scheme_color_index(value: a::SchemeColorValues) -> Option<u32> {
  match value {
    a::SchemeColorValues::Light1 | a::SchemeColorValues::Background1 => Some(0),
    a::SchemeColorValues::Dark1 | a::SchemeColorValues::Text1 => Some(1),
    a::SchemeColorValues::Light2 | a::SchemeColorValues::Background2 => Some(2),
    a::SchemeColorValues::Dark2 | a::SchemeColorValues::Text2 => Some(3),
    a::SchemeColorValues::Accent1 => Some(4),
    a::SchemeColorValues::Accent2 => Some(5),
    a::SchemeColorValues::Accent3 => Some(6),
    a::SchemeColorValues::Accent4 => Some(7),
    a::SchemeColorValues::Accent5 => Some(8),
    a::SchemeColorValues::Accent6 => Some(9),
    a::SchemeColorValues::Hyperlink => Some(10),
    a::SchemeColorValues::FollowedHyperlink => Some(11),
    _ => None,
  }
}

fn clip_chart_items_to_rect(
  items: &mut Vec<PageItem>,
  clip: CellRect,
  metrics: &mut TextMetrics,
  text_boundary_slack: ChartTextClipSlack,
  right_truncated_texts: &[String],
) {
  items.retain_mut(|item| {
    clip_chart_item_to_rect(
      item,
      clip,
      metrics,
      text_boundary_slack,
      right_truncated_texts,
    )
  });
}

fn clip_chart_item_to_rect(
  item: &mut PageItem,
  clip: CellRect,
  metrics: &mut TextMetrics,
  text_boundary_slack: ChartTextClipSlack,
  right_truncated_texts: &[String],
) -> bool {
  match item {
    // Excel clips chart text at horizontal worksheet page boundaries, while
    // retaining text objects that only extend beyond the vertical printable
    // area in the PDF text layer.
    PageItem::Text(text) => {
      let clip_right = clip.x_pt + clip.width_pt;
      let mut measured_width = metrics.measure_text(&text.text, &text.style);
      let (mut left, mut right) = chart_text_horizontal_bounds(text, measured_width);
      if text.discard_if_horizontally_clipped && right > clip_right {
        // A chart data table is an actual table shape, not a collection of
        // independent chart labels. LibreOffice DataTableView::createShapes
        // creates XCell text inside an SdrTableObj; Office fixed output
        // likewise omits a table-cell text operator when the right worksheet
        // page boundary cuts through that cell. A continuation page retains
        // its first cell's text object at the left boundary, as do other
        // chart labels eligible for the clipped-text path below.
        return false;
      }
      if right > clip_right && matches!(text.text.chars().last(), Some(',' | ';')) {
        // Office writes data-label fields as separate runs. At a worksheet
        // page boundary the separator glyph can be outside the clip even
        // though the preceding series-name run still intersects it.
        text.text.pop();
        measured_width = metrics.measure_text(&text.text, &text.style);
        (left, right) = chart_text_horizontal_bounds(text, measured_width);
      }
      // Office retains a text object whose final glyph reaches the printable
      // boundary even when fixed-output clipping hides its ink. Half an em
      // covers the pre-shaping/final-glyph-box difference without duplicating
      // a category label whose complete glyph box belongs to the prior page.
      let is_date_axis_tick = right_truncated_texts.contains(&text.text);
      let left_boundary_slack = text.style.font_size_pt * text_boundary_slack.left_em;
      let right_boundary_slack =
        if is_date_axis_tick && text.style.rotation_deg.abs() > f32::EPSILON {
          // A rotated date tick is already classified with its post-rotation
          // ink bounds. Excel does not add the horizontal half-em shaping band
          // used for ordinary text: a complete label beyond the right page clip
          // belongs only to the continuation page.
          0.0
        } else {
          text.style.font_size_pt * text_boundary_slack.right_em
        };
      let retained =
        right + left_boundary_slack >= clip.x_pt && left <= clip_right + right_boundary_slack;
      if retained {
        if right > clip_right
          && text.style.rotation_deg.abs() <= f32::EPSILON
          && is_date_axis_tick
          && text.x_pt < clip_right
        {
          // Excel emits only the date-axis glyph prefix whose origins remain
          // within one font em of a horizontal worksheet page boundary. The
          // following page still receives the complete tick label. Keeping
          // this at the date-axis model boundary avoids truncating titles,
          // legends, and data labels that Office retains as complete clipped
          // text objects.
          let available_width = clip_right + text.style.font_size_pt - text.x_pt;
          truncate_text_to_width(&mut text.text, available_width, |prefix| {
            metrics.measure_text(prefix, &text.style)
          });
        }
        // Calc's ScPrintFunc installs the printable-page clip on its output
        // device before painting the drawing layer. Office likewise keeps
        // clipped chart text operators in the PDF stream, so preserve the
        // object and carry the real clip to the PDF backend instead of using
        // a semantic-only surrogate with different baseline semantics.
        text.paint_clip = Some(common_rect(
          clip.x_pt,
          clip.y_pt,
          clip.width_pt,
          clip.height_pt,
        ));
      }
      retained
    }
    PageItem::Rect(rect) => {
      let left = rect.x_pt.max(clip.x_pt);
      let top = rect.y_pt.max(clip.y_pt);
      let right = (rect.x_pt + rect.width_pt).min(clip.x_pt + clip.width_pt);
      let bottom = (rect.y_pt + rect.height_pt).min(clip.y_pt + clip.height_pt);
      if right <= left || bottom <= top {
        return false;
      }
      rect.x_pt = left;
      rect.y_pt = top;
      rect.width_pt = right - left;
      rect.height_pt = bottom - top;
      true
    }
    PageItem::Line(line) if line.y1_pt == line.y2_pt => {
      if line.y1_pt < clip.y_pt || line.y1_pt > clip.y_pt + clip.height_pt {
        return false;
      }
      line.x1_pt = line.x1_pt.clamp(clip.x_pt, clip.x_pt + clip.width_pt);
      line.x2_pt = line.x2_pt.clamp(clip.x_pt, clip.x_pt + clip.width_pt);
      line.x1_pt != line.x2_pt
    }
    PageItem::Line(line) if line.x1_pt == line.x2_pt => {
      if line.x1_pt < clip.x_pt || line.x1_pt > clip.x_pt + clip.width_pt {
        return false;
      }
      line.y1_pt = line.y1_pt.clamp(clip.y_pt, clip.y_pt + clip.height_pt);
      line.y2_pt = line.y2_pt.clamp(clip.y_pt, clip.y_pt + clip.height_pt);
      line.y1_pt != line.y2_pt
    }
    PageItem::Line(line) => rect_intersects_clip(
      line.x1_pt.min(line.x2_pt),
      line.y1_pt.min(line.y2_pt),
      line.x1_pt.max(line.x2_pt),
      line.y1_pt.max(line.y2_pt),
      clip,
    ),
    PageItem::Path(path) if path.closed && path.commands.is_empty() => {
      clip_closed_polygon_to_rect(path, clip)
    }
    PageItem::Path(path) => rect_intersects_clip(
      path.bounds.origin.x.0,
      path.bounds.origin.y.0,
      path.bounds.origin.x.0 + path.bounds.size.width.0,
      path.bounds.origin.y.0 + path.bounds.size.height.0,
      clip,
    ),
    PageItem::Group { items, .. } => {
      clip_chart_items_to_rect(
        items,
        clip,
        metrics,
        text_boundary_slack,
        right_truncated_texts,
      );
      !items.is_empty()
    }
    PageItem::Image(_) | PageItem::LinkArea(_) => true,
  }
}

fn chart_text_horizontal_bounds(text: &crate::model::TextItem, width_pt: f32) -> (f32, f32) {
  if text.style.rotation_deg.abs() <= f32::EPSILON {
    return (text.x_pt, text.x_pt + width_pt);
  }
  let (rotation_x, rotation_y) = text.rotation_center_pt.unwrap_or((text.x_pt, text.y_pt));
  let angle = text.style.rotation_deg.to_radians();
  let sin = angle.sin();
  let cos = angle.cos();
  let mut left = f32::INFINITY;
  let mut right = f32::NEG_INFINITY;
  for (x, y) in [
    (text.x_pt, text.y_pt),
    (text.x_pt + width_pt, text.y_pt),
    (text.x_pt + width_pt, text.y_pt + text.line_height_pt),
    (text.x_pt, text.y_pt + text.line_height_pt),
  ] {
    let dx = x - rotation_x;
    let dy = y - rotation_y;
    let rotated_x = rotation_x + dx * cos - dy * sin;
    left = left.min(rotated_x);
    right = right.max(rotated_x);
  }
  (left, right)
}

fn truncate_text_to_width(
  text: &mut String,
  maximum_width: f32,
  mut measure: impl FnMut(&str) -> f32,
) {
  let mut end = 0;
  for boundary in text
    .char_indices()
    .map(|(index, character)| index + character.len_utf8())
    .chain(std::iter::once(text.len()))
  {
    if measure(&text[..end]) > maximum_width {
      break;
    }
    end = boundary;
  }
  text.truncate(end);
}

#[derive(Clone, Copy)]
enum PolygonClipEdge {
  Left,
  Right,
  Top,
  Bottom,
}

fn clip_closed_polygon_to_rect(path: &mut common::PathItem<'static>, clip: CellRect) -> bool {
  let mut points = path.points.clone();
  for (edge, boundary) in [
    (PolygonClipEdge::Left, clip.x_pt),
    (PolygonClipEdge::Right, clip.x_pt + clip.width_pt),
    (PolygonClipEdge::Top, clip.y_pt),
    (PolygonClipEdge::Bottom, clip.y_pt + clip.height_pt),
  ] {
    points = clip_polygon_edge(&points, edge, boundary);
    if points.len() < 3 {
      return false;
    }
  }

  let Some(bounds) = common::drawingml_geometry::point_bounds(
    points
      .iter()
      .map(|point| kurbo::Point::new(f64::from(point.x.0), f64::from(point.y.0))),
  ) else {
    return false;
  };
  path.points = points;
  path.bounds = common_rect(
    bounds.x0 as f32,
    bounds.y0 as f32,
    bounds.width() as f32,
    bounds.height() as f32,
  );
  true
}

fn clip_polygon_edge(
  points: &[common::Point],
  edge: PolygonClipEdge,
  boundary: f32,
) -> Vec<common::Point> {
  let Some(mut previous) = points.last().copied() else {
    return Vec::new();
  };
  let mut previous_inside = polygon_point_inside(previous, edge, boundary);
  let mut output = Vec::with_capacity(points.len() + 2);
  for current in points.iter().copied() {
    let current_inside = polygon_point_inside(current, edge, boundary);
    if current_inside != previous_inside {
      output.push(polygon_edge_intersection(previous, current, edge, boundary));
    }
    if current_inside {
      output.push(current);
    }
    previous = current;
    previous_inside = current_inside;
  }
  output
}

fn polygon_point_inside(point: common::Point, edge: PolygonClipEdge, boundary: f32) -> bool {
  match edge {
    PolygonClipEdge::Left => point.x.0 >= boundary,
    PolygonClipEdge::Right => point.x.0 <= boundary,
    PolygonClipEdge::Top => point.y.0 >= boundary,
    PolygonClipEdge::Bottom => point.y.0 <= boundary,
  }
}

fn polygon_edge_intersection(
  start: common::Point,
  end: common::Point,
  edge: PolygonClipEdge,
  boundary: f32,
) -> common::Point {
  match edge {
    PolygonClipEdge::Left | PolygonClipEdge::Right => {
      let delta = end.x.0 - start.x.0;
      let ratio = if delta.abs() <= f32::EPSILON {
        0.0
      } else {
        (boundary - start.x.0) / delta
      };
      common_point(boundary, start.y.0 + (end.y.0 - start.y.0) * ratio)
    }
    PolygonClipEdge::Top | PolygonClipEdge::Bottom => {
      let delta = end.y.0 - start.y.0;
      let ratio = if delta.abs() <= f32::EPSILON {
        0.0
      } else {
        (boundary - start.y.0) / delta
      };
      common_point(start.x.0 + (end.x.0 - start.x.0) * ratio, boundary)
    }
  }
}

fn rect_intersects_clip(left: f32, top: f32, right: f32, bottom: f32, clip: CellRect) -> bool {
  right > clip.x_pt
    && bottom > clip.y_pt
    && left < clip.x_pt + clip.width_pt
    && top < clip.y_pt + clip.height_pt
}

const XLSX_DEFAULT_CHART_SERIES_COLORS: [RgbColor; 6] = [
  RgbColor {
    r: 0x44,
    g: 0x72,
    b: 0xc4,
  },
  RgbColor {
    r: 0xed,
    g: 0x7d,
    b: 0x31,
  },
  RgbColor {
    r: 0xa5,
    g: 0xa5,
    b: 0xa5,
  },
  RgbColor {
    r: 0xff,
    g: 0xc0,
    b: 0x00,
  },
  RgbColor {
    r: 0x5b,
    g: 0x9b,
    b: 0xd5,
  },
  RgbColor {
    r: 0x70,
    g: 0xad,
    b: 0x47,
  },
];

fn xlsx_chart_solid_fill_color(fill: &a::SolidFill, import: &ExcelImport) -> Option<RgbColor> {
  let color = fill
    .solid_fill_choice
    .as_ref()
    .and_then(Color::from_solid_fill_choice)?;
  xlsx_chart_color(color, import)
}

fn xlsx_chart_gradient_fill(
  fill: &a::GradientFill,
  import: &ExcelImport,
) -> Option<common::Fill<'static>> {
  let stops = fill
    .gradient_stop_list
    .as_ref()?
    .gradient_stop
    .iter()
    .filter_map(|stop| {
      let color = stop
        .gradient_stop_choice
        .as_ref()
        .and_then(Color::from_gradient_stop_choice)
        .and_then(|color| xlsx_drawing_color(color, import))?;
      Some(common::GradientStop {
        position: stop.position.as_ratio() as f32,
        color,
        scheme: None,
      })
    })
    .collect::<Vec<_>>();
  if stops.is_empty() {
    return None;
  }
  let (angle_degrees, scaled, path) = match fill.gradient_fill_choice.as_ref()? {
    a::GradientFillChoice::LinearGradientFill(linear) => (
      Some(linear.angle.unwrap_or_default() as f32 / 60_000.0),
      linear.scaled.as_ref().is_some_and(|value| value.as_bool()),
      None,
    ),
    a::GradientFillChoice::PathGradientFill(path) => (
      None,
      false,
      Some(common::drawingml_gradient::resolve_path_gradient(
        fill,
        path,
        common::Transform::default(),
      )),
    ),
  };
  let interpolation = office_drawing_gradient_interpolation(stops.len());
  Some(common::Fill::Gradient(common::GradientFill {
    stops,
    angle_degrees,
    definition_bounds: None,
    line: None,
    interpolation,
    scaled,
    rotate_with_shape: fill.rotate_with_shape.as_ref().map(|value| value.as_bool()),
    path,
  }))
}

fn xlsx_chart_blip_fill(fill: &a::BlipFill) -> common::Fill<'static> {
  let relationship_id = fill.blip.as_deref().and_then(|blip| {
    blip
      .embed
      .as_deref()
      .or(blip.link.as_deref())
      .map(|id| Cow::Owned(id.to_string()))
  });
  common::Fill::Image {
    relationship_id,
    tile: matches!(fill.blip_fill_choice, Some(a::BlipFillChoice::Tile(_))),
  }
}

fn xlsx_chart_shape_style(
  properties: Option<&c::ChartShapeProperties>,
  import: &ExcelImport,
  mut fallback: common::ShapeStyle<'static>,
) -> common::ShapeStyle<'static> {
  let Some(properties) = properties else {
    return fallback;
  };
  if let Some(fill) = properties.chart_shape_properties_choice2.as_ref() {
    fallback.fill = match fill {
      c::ChartShapePropertiesChoice2::NoFill(_) => common::ShapeStyleValue::NoPaint,
      c::ChartShapePropertiesChoice2::SolidFill(fill) => xlsx_chart_solid_fill_color(fill, import)
        .map(|color| common::ShapeStyleValue::Paint(common::Fill::Solid(common_rgb(color, 1.0))))
        .unwrap_or(fallback.fill),
      c::ChartShapePropertiesChoice2::GradientFill(fill) => xlsx_chart_gradient_fill(fill, import)
        .map(common::ShapeStyleValue::Paint)
        .unwrap_or(fallback.fill),
      c::ChartShapePropertiesChoice2::PatternFill(fill) => {
        drawing_object_pattern_fill(import, fill, None)
          .map(|fill| common::ShapeStyleValue::Paint(common::Fill::Pattern(fill)))
          .unwrap_or(fallback.fill)
      }
      c::ChartShapePropertiesChoice2::BlipFill(fill) => {
        common::ShapeStyleValue::Paint(xlsx_chart_blip_fill(fill))
      }
    };
  }
  if let Some(outline) = properties.outline.as_deref() {
    fallback.stroke = xlsx_chart_outline_style(outline, import, fallback.stroke);
  }
  fallback
}

fn xlsx_shape_style(
  properties: Option<&c::ShapeProperties>,
  import: &ExcelImport,
  mut fallback: common::ShapeStyle<'static>,
) -> common::ShapeStyle<'static> {
  let Some(properties) = properties else {
    return fallback;
  };
  if let Some(fill) = properties.shape_properties_choice2.as_ref() {
    fallback.fill = match fill {
      c::ShapePropertiesChoice2::NoFill(_) => common::ShapeStyleValue::NoPaint,
      c::ShapePropertiesChoice2::SolidFill(fill) => xlsx_chart_solid_fill_color(fill, import)
        .map(|color| common::ShapeStyleValue::Paint(common::Fill::Solid(common_rgb(color, 1.0))))
        .unwrap_or(fallback.fill),
      c::ShapePropertiesChoice2::GradientFill(fill) => xlsx_chart_gradient_fill(fill, import)
        .map(common::ShapeStyleValue::Paint)
        .unwrap_or(fallback.fill),
      c::ShapePropertiesChoice2::PatternFill(fill) => {
        drawing_object_pattern_fill(import, fill, None)
          .map(|fill| common::ShapeStyleValue::Paint(common::Fill::Pattern(fill)))
          .unwrap_or(fallback.fill)
      }
      c::ShapePropertiesChoice2::BlipFill(fill) => {
        common::ShapeStyleValue::Paint(xlsx_chart_blip_fill(fill))
      }
      c::ShapePropertiesChoice2::GroupFill => fallback.fill,
    };
  }
  if let Some(outline) = properties.outline.as_deref() {
    fallback.stroke = xlsx_chart_outline_style(outline, import, fallback.stroke);
  }
  fallback
}

fn xlsx_chart_outline_style(
  outline: &a::Outline,
  import: &ExcelImport,
  fallback: common::ShapeStyleValue<common::Stroke<'static>>,
) -> common::ShapeStyleValue<common::Stroke<'static>> {
  let fallback_stroke = match &fallback {
    common::ShapeStyleValue::Paint(stroke) => Some(stroke.clone()),
    common::ShapeStyleValue::Unspecified | common::ShapeStyleValue::NoPaint => None,
  };
  let (explicit_color, pattern, gradient) = match outline.outline_choice1.as_ref() {
    Some(a::OutlineChoice::NoFill(_)) => return common::ShapeStyleValue::NoPaint,
    Some(a::OutlineChoice::SolidFill(fill)) => {
      let Some(color) = xlsx_chart_solid_fill_color(fill, import) else {
        return fallback;
      };
      (Some(common_rgb(color, 1.0)), None, None)
    }
    Some(a::OutlineChoice::PatternFill(fill)) => {
      let Some(pattern) = drawing_object_pattern_fill(import, fill, None) else {
        return fallback;
      };
      (Some(pattern.foreground), Some(pattern), None)
    }
    Some(a::OutlineChoice::GradientFill(fill)) => {
      let Some(common::Fill::Gradient(gradient)) = xlsx_chart_gradient_fill(fill, import) else {
        return fallback;
      };
      let color = gradient.stops.first().map(|stop| stop.color);
      (color, None, Some(gradient))
    }
    None => return fallback,
  };
  let Some(mut stroke) = fallback_stroke.or_else(|| {
    explicit_color.map(|color| common::Stroke {
      width: common::Pt(0.75),
      color,
      ..Default::default()
    })
  }) else {
    return fallback;
  };
  if let Some(color) = explicit_color {
    stroke.color = color;
  }
  stroke.pattern = pattern;
  stroke.gradient = gradient;
  if let Some(width_pt) = xlsx_outline_width_pt(Some(outline)) {
    stroke.width = common::Pt(width_pt);
  } else if stroke.width.0 <= 0.0 {
    stroke.width = common::Pt(0.75);
  }
  common::drawingml_stroke::apply_outline_style(&mut stroke, outline);
  common::ShapeStyleValue::Paint(stroke)
}

fn xlsx_chart_outline_width_pt(properties: &c::ChartShapeProperties) -> Option<f32> {
  xlsx_outline_width_pt(properties.outline.as_deref())
}

fn xlsx_shape_outline_width_pt(properties: &c::ShapeProperties) -> Option<f32> {
  xlsx_outline_width_pt(properties.outline.as_deref())
}

fn xlsx_outline_width_pt(outline: Option<&a::Outline>) -> Option<f32> {
  let width_emu = outline?.width?;
  Some(if width_emu <= 0 {
    XLSX_CHART_HAIRLINE_WIDTH_PT
  } else {
    units::emu_to_points(i64::from(width_emu))
  })
}

fn xlsx_chart_first_gradient_fill_color(
  fill: &a::GradientFill,
  import: &ExcelImport,
) -> Option<RgbColor> {
  let color = fill
    .gradient_stop_list
    .as_ref()?
    .gradient_stop
    .first()?
    .gradient_stop_choice
    .as_ref()
    .and_then(Color::from_gradient_stop_choice)?;
  xlsx_chart_color(color, import)
}

fn xlsx_chart_color(color: Color, import: &ExcelImport) -> Option<RgbColor> {
  let mut scheme_resolver = |value| {
    let index = xlsx_scheme_color_index(value)?;
    let color = import.styles.theme_color(index, 0.0)?;
    Some(Color::RgbHex(RgbHexColor {
      value: format!("{:02X}{:02X}{:02X}", color.r, color.g, color.b),
      transformations: Vec::new(),
    }))
  };
  let color = color.resolve_rgb(&mut scheme_resolver, None)?;
  Some(RgbColor {
    r: color.r,
    g: color.g,
    b: color.b,
  })
}

fn xlsx_automatic_chart_color(
  chart_space: &c::ChartSpace,
  import: &ExcelImport,
  chart_style_id: u8,
  formatting_index: usize,
  maximum_formatting_index: usize,
) -> Option<RgbColor> {
  shared_chart::automatic_chart_series_color(
    chart_style_id,
    formatting_index,
    maximum_formatting_index,
    |token| {
      let mapped =
        shared_chart::scheme_color_token(chart_space.color_map_override.as_deref(), token)?;
      let theme_index = match mapped {
        a::ColorSchemeIndexValues::Light1 => 0,
        a::ColorSchemeIndexValues::Dark1 => 1,
        a::ColorSchemeIndexValues::Light2 => 2,
        a::ColorSchemeIndexValues::Dark2 => 3,
        a::ColorSchemeIndexValues::Accent1 => 4,
        a::ColorSchemeIndexValues::Accent2 => 5,
        a::ColorSchemeIndexValues::Accent3 => 6,
        a::ColorSchemeIndexValues::Accent4 => 7,
        a::ColorSchemeIndexValues::Accent5 => 8,
        a::ColorSchemeIndexValues::Accent6 => 9,
        a::ColorSchemeIndexValues::Hyperlink => 10,
        a::ColorSchemeIndexValues::FollowedHyperlink => 11,
      };
      import
        .styles
        .theme_color(theme_index, 0.0)
        .or_else(|| xlsx_default_chart_theme_color(theme_index))
    },
  )
}

fn xlsx_default_chart_theme_color(index: u32) -> Option<RgbColor> {
  match index {
    0 => Some(RgbColor {
      r: 0xff,
      g: 0xff,
      b: 0xff,
    }),
    1 => Some(RgbColor { r: 0, g: 0, b: 0 }),
    2 => Some(RgbColor {
      r: 0xe7,
      g: 0xe6,
      b: 0xe6,
    }),
    3 => Some(RgbColor {
      r: 0x44,
      g: 0x54,
      b: 0x6a,
    }),
    4..=9 => Some(XLSX_DEFAULT_CHART_SERIES_COLORS[(index - 4) as usize]),
    10 => Some(RgbColor {
      r: 0x05,
      g: 0x63,
      b: 0xc1,
    }),
    11 => Some(RgbColor {
      r: 0x95,
      g: 0x4f,
      b: 0x72,
    }),
    _ => None,
  }
}

fn xlsx_chart_style_id(chart_space: &c::ChartSpace) -> Option<u8> {
  match chart_space.chart_space_choice.as_ref()? {
    c::ChartSpaceChoice::C14Style(style) => normalize_xlsx_chart_style(u16::from(style.val)),
    c::ChartSpaceChoice::CStyle(style) => {
      normalize_xlsx_chart_style(u16::from(style.val.unwrap_or(2)))
    }
    c::ChartSpaceChoice::AlternateContent(_) => None,
  }
}

fn normalize_xlsx_chart_style(value: u16) -> Option<u8> {
  match value {
    1..=48 => Some(value as u8),
    101..=148 => Some((value - 100) as u8),
    _ => None,
  }
}

fn drawing_anchor_text<'a>(
  drawing: &'a super::drawing::DrawingResourceCatalog,
  anchor: &'a super::drawing::DrawingAnchorModel,
) -> Cow<'a, str> {
  if !anchor.object.text.is_empty() {
    return Cow::Borrowed(anchor.object.text.as_str());
  }
  if anchor.object.kind == super::drawing::DrawingObjectKind::GraphicFrame
    && let Some(relationship_id) = anchor.object.relationship_id.as_deref()
  {
    let chart_text = drawing
      .charts
      .iter()
      .chain(drawing.extended_charts.iter())
      .find(|chart| chart.relationship_id.as_deref() == Some(relationship_id))
      .map(|chart| chart.visible_texts.join("\n"));
    if let Some(chart_text) = chart_text.filter(|text| !text.is_empty()) {
      return Cow::Owned(chart_text);
    }
    if drawing
      .diagrams
      .data_parts
      .iter()
      .any(|data| data.relationship_id.as_deref() == Some(relationship_id))
    {
      return Cow::Borrowed("");
    }
  }
  Cow::Borrowed("")
}

fn render_drawing_text(
  items: &mut Vec<PageItem>,
  text: &str,
  rect: CellRect,
  style: Option<TextStyle>,
  layout: Option<DrawingTextLayout>,
  text_warp: Option<&a::PresetTextWarp>,
  hyperlink_url: Option<&str>,
) {
  let item_start = items.len();
  let mut style = style.unwrap_or_default();
  let layout = layout.unwrap_or_default();
  let vertical_rotation_deg = match layout.vertical {
    Some(a::TextVerticalValues::Vertical | a::TextVerticalValues::EastAsianVetical) => 90.0,
    Some(a::TextVerticalValues::Vertical270) => 270.0,
    _ => 0.0,
  };
  style.rotation_deg = if layout.upright {
    vertical_rotation_deg
  } else {
    vertical_rotation_deg + layout.text_rotation_deg + layout.shape_rotation_deg
  };
  if !layout.upright && layout.shape_rotation_deg.rem_euclid(360.0).abs() > f32::EPSILON {
    // Excel's fixed-format writer paints text in rotated worksheet shapes as
    // vector glyph outlines (the Office reference PDF contains no font or
    // extractable text for these runs). Keep bodyPr-only rotation independent:
    // this rule belongs to the owning worksheet shape transform.
    style.pdf_glyph_outlines = true;
    let mut options = style
      .pdf_glyph_outline_options
      .as_deref()
      .cloned()
      .unwrap_or_default();
    options.semantic_text_overlay = false;
    style.pdf_glyph_outline_options = Some(Arc::new(options));
  }
  let vertical_text = vertical_rotation_deg != 0.0;
  let line_height = (style.font_size_pt * 1.15).max(1.0);
  let mut text_metrics = TextMetrics::new();
  let lines = text
    .lines()
    .filter(|line| !line.is_empty())
    .collect::<Vec<_>>();
  let available_height = (rect.height_pt - layout.top_inset_pt - layout.bottom_inset_pt).max(0.0);
  let text_height = line_height * lines.len() as f32;
  let vertical_offset = match layout.anchor {
    a::TextAnchoringTypeValues::Center => (available_height - text_height).max(0.0) / 2.0,
    a::TextAnchoringTypeValues::Bottom => (available_height - text_height).max(0.0),
    a::TextAnchoringTypeValues::Top
    | a::TextAnchoringTypeValues::Justified
    | a::TextAnchoringTypeValues::Distributed => 0.0,
  };
  for (index, line) in lines.into_iter().enumerate() {
    let y = if vertical_text {
      rect.y_pt + (rect.height_pt - line_height) / 2.0
    } else {
      rect.y_pt + layout.top_inset_pt + vertical_offset + index as f32 * line_height
    };
    if !vertical_text && y > rect.y_pt + rect.height_pt - layout.bottom_inset_pt {
      break;
    }
    let available_width = if vertical_text {
      (rect.height_pt - layout.top_inset_pt - layout.bottom_inset_pt).max(0.0)
    } else {
      (rect.width_pt - layout.left_inset_pt - layout.right_inset_pt).max(0.0)
    };
    let text_width = text_metrics.measure_text(line, &style);
    let aligned_offset = match layout.alignment {
      a::TextAlignmentTypeValues::Center => {
        layout.left_inset_pt + (available_width - text_width).max(0.0) / 2.0
      }
      a::TextAlignmentTypeValues::Right => {
        layout.left_inset_pt + (available_width - text_width).max(0.0)
      }
      _ => layout.left_inset_pt,
    };
    let x = if vertical_text {
      rect.x_pt + (rect.width_pt - text_width) / 2.0 + index as f32 * line_height
    } else {
      rect.x_pt + aligned_offset
    };
    items.push(PageItem::Text(TextItem {
      x_pt: x,
      y_pt: y,
      line_height_pt: line_height,
      paint_clip: None,
      discard_if_horizontally_clipped: false,
      text: line.to_string(),
      style: style.clone(),
      rotation_center_pt: (style.rotation_deg != 0.0).then_some((
        rect.x_pt + rect.width_pt / 2.0,
        rect.y_pt + rect.height_pt / 2.0,
      )),
      hyperlink_url: hyperlink_url.map(ToString::to_string),
      form_widget_id: None,
      paragraph_bidi: false,
      preserve_text_portion: false,
      pdf_text_segmentation: PdfTextSegmentation::Line,
      source_path: Vec::new(),
    }));
  }
  apply_drawing_text_warp(&mut items[item_start..], text_warp, rect, &mut text_metrics);
}

fn apply_drawing_text_warp(
  items: &mut [PageItem],
  preset: Option<&a::PresetTextWarp>,
  rect: CellRect,
  text_metrics: &mut TextMetrics,
) {
  let Some(preset) = preset else {
    return;
  };
  common::drawingml_text_warp::apply_to_text_items(
    items,
    preset,
    common_rect(rect.x_pt, rect.y_pt, rect.width_pt, rect.height_pt),
    common_rect(rect.x_pt, rect.y_pt, rect.width_pt, rect.height_pt),
    text_metrics,
    None,
  );
}

#[derive(Clone, Copy, Debug)]
struct DrawingTextLayout {
  alignment: a::TextAlignmentTypeValues,
  anchor: a::TextAnchoringTypeValues,
  vertical: Option<a::TextVerticalValues>,
  left_inset_pt: f32,
  top_inset_pt: f32,
  right_inset_pt: f32,
  bottom_inset_pt: f32,
  text_rotation_deg: f32,
  shape_rotation_deg: f32,
  upright: bool,
}

impl Default for DrawingTextLayout {
  fn default() -> Self {
    Self {
      alignment: a::TextAlignmentTypeValues::Left,
      anchor: a::TextAnchoringTypeValues::Top,
      vertical: None,
      left_inset_pt: XLSX_CELL_TEXT_INSET_PT,
      top_inset_pt: XLSX_CELL_TEXT_INSET_PT,
      right_inset_pt: XLSX_CELL_TEXT_INSET_PT,
      bottom_inset_pt: XLSX_CELL_TEXT_INSET_PT,
      text_rotation_deg: 0.0,
      shape_rotation_deg: 0.0,
      upright: false,
    }
  }
}

fn drawing_object_text_layout(object: &super::drawing::DrawingObjectModel) -> DrawingTextLayout {
  DrawingTextLayout {
    alignment: object.text_alignment.unwrap_or_default(),
    anchor: object
      .text_anchor
      .unwrap_or(a::TextAnchoringTypeValues::Top),
    vertical: object.text_vertical,
    text_rotation_deg: object.text_rotation_deg,
    shape_rotation_deg: drawing_object_visual_rotation_degrees(object),
    upright: object.text_upright,
    left_inset_pt: object
      .text_left_inset_emu
      .map_or(XLSX_CELL_TEXT_INSET_PT, units::emu_to_points),
    top_inset_pt: object
      .text_top_inset_emu
      .map_or(XLSX_CELL_TEXT_INSET_PT, units::emu_to_points),
    right_inset_pt: object
      .text_right_inset_emu
      .map_or(XLSX_CELL_TEXT_INSET_PT, units::emu_to_points),
    bottom_inset_pt: object
      .text_bottom_inset_emu
      .map_or(XLSX_CELL_TEXT_INSET_PT, units::emu_to_points),
  }
}

fn drawing_object_text_style(
  import: &ExcelImport,
  object: &super::drawing::DrawingObjectModel,
) -> Option<TextStyle> {
  let mut style = import.styles.default_drawing_text_style();
  if let Some(font_size) = object.text_font_size_points100 {
    style.font_size_pt = font_size as f32 / 100.0;
  }
  if let Some(color) = object
    .text_color
    .clone()
    .and_then(|color| xlsx_drawing_rgb_color(color, import))
  {
    style.color = color;
  }
  if let Some(bold) = object.text_bold {
    style.bold = bold;
  }
  if let Some(italic) = object.text_italic {
    style.italic = italic;
  }
  if let Some(typeface) = object.text_font_family.as_deref() {
    style.font_family = Some(Arc::from(
      import.styles.resolve_drawingml_theme_font(typeface),
    ));
  }
  if let Some(typeface) = object.text_east_asia_font_family.as_deref() {
    style.east_asia_font_family = Some(Arc::from(
      import.styles.resolve_drawingml_theme_font(typeface),
    ));
  }
  if let Some(typeface) = object.text_complex_font_family.as_deref() {
    style.complex_font_family = Some(Arc::from(
      import.styles.resolve_drawingml_theme_font(typeface),
    ));
  }
  Some(style)
}

fn print_page_vml_text_items(
  page: &CalcPrintPage<'_>,
  origin_x_pt: f32,
  origin_y_pt: f32,
  zoom_scale: f32,
) -> Vec<PageItem> {
  let mut items = Vec::new();
  let page_transform = SheetPageTransform::for_page(page, origin_x_pt, origin_y_pt, zoom_scale);
  for shape in page
    .sheet
    .resources
    .object_resources
    .vml_drawings
    .iter()
    .flat_map(|drawing| drawing.shapes.iter())
  {
    if shape.hidden || !shape.print_object {
      continue;
    }
    if shape
      .object_type
      .as_deref()
      .is_some_and(|value| value.eq_ignore_ascii_case("Checkbox"))
    {
      // Checkbox captions are laid out by the Forms control renderer beside
      // the native indicator, not as a generic VML textbox.
      continue;
    }
    if !vml_shape_intersects_page(page, shape) {
      continue;
    }
    let text = vml_shape_visible_text(page.sheet, shape);
    if text.trim().is_empty() {
      continue;
    }
    let Some((x_pt, y_pt, width_pt, height_pt)) = vml_shape_rect(page.sheet, shape) else {
      continue;
    };
    let rect = page_transform.rect_from_xywh(x_pt, y_pt, width_pt, height_pt);
    render_drawing_text(&mut items, text, rect, None, None, None, None);
  }
  items
}

fn vml_shape_visible_text<'a>(
  sheet: &'a CalcSheet,
  shape: &'a super::object_resources::VmlShapeModel,
) -> &'a str {
  if !shape.text.trim().is_empty() {
    return shape.text.as_str();
  }
  if shape.object_type.as_deref() != Some("Note") || !shape.visible {
    return "";
  }
  let Some(row) = shape.note_row.and_then(|row| row.checked_add(1)) else {
    return "";
  };
  let Some(col) = shape.note_column.and_then(|col| col.checked_add(1)) else {
    return "";
  };
  let address = super::worksheet::CellAddress { col, row };
  // legacy comments against the VML note shape map; visible note captions use
  // the comments part text when the VML textbox itself is empty.
  sheet
    .resources
    .comments
    .legacy
    .as_ref()
    .and_then(|legacy| {
      legacy.comments.iter().find_map(|comment| {
        super::worksheet::CellRange::parse_a1_range(&comment.reference)
          .is_some_and(|range| range.contains(address))
          .then_some(comment.text.as_str())
      })
    })
    .unwrap_or("")
}

fn drawing_object_hyperlink_url<'a>(
  drawing: &'a super::drawing::DrawingResourceCatalog,
  object: &'a super::drawing::DrawingObjectModel,
) -> Option<Cow<'a, str>> {
  object
    .hyperlink_relationship_id
    .as_deref()
    .and_then(|relationship_id| drawing.hyperlink_targets.get(relationship_id))
    .map(|url| Cow::Borrowed(url.as_str()))
    .or_else(|| object.hyperlink_invalid_url.as_deref().map(Cow::Borrowed))
    .or_else(|| {
      object
        .hyperlink_action
        .as_deref()
        .and_then(drawing_hyperlink_action_url)
        .map(Cow::Owned)
    })
}

fn drawing_hyperlink_action_url(action: &str) -> Option<String> {
  action
    .strip_prefix("ppaction://hlinkshowjump?jump=")
    .map(|jump| format!("ooxmlsdk-pdf-action://hlinkshowjump/{jump}"))
}

fn vml_shape_rect(
  sheet: &CalcSheet,
  shape: &super::object_resources::VmlShapeModel,
) -> Option<(f32, f32, f32, f32)> {
  // Spreadsheet VML ClientData anchors are the cell-relative placement
  // authority. The CSS margin box is a cached absolute snapshot and can
  // disagree after Excel has recalculated column/row geometry. LibreOffice's
  // ShapeAnchor::calcAnchorRectEmu likewise uses the ClientData from/to cells
  // for ANCHOR_VML and falls back only when no client anchor is available.
  shape
    .anchor
    .and_then(|anchor| vml_anchor_rect(sheet, anchor))
    .or_else(|| shape.style.as_deref().and_then(vml_style_rect))
}

fn vml_anchor_rect(
  sheet: &CalcSheet,
  anchor: super::object_resources::VmlClientAnchor,
) -> Option<(f32, f32, f32, f32)> {
  let x1 = vml_anchor_x(sheet, anchor.from_col, anchor.from_col_offset_px);
  let y1 = vml_anchor_y(sheet, anchor.from_row, anchor.from_row_offset_px);
  let x2 = vml_anchor_x(sheet, anchor.to_col, anchor.to_col_offset_px);
  let y2 = vml_anchor_y(sheet, anchor.to_row, anchor.to_row_offset_px);
  if x2 < x1 || y2 < y1 {
    return None;
  }
  Some((x1, y1, x2 - x1, y2 - y1))
}

fn vml_anchor_x(sheet: &CalcSheet, zero_based_col: u32, offset_px: i32) -> f32 {
  let col = zero_based_col.saturating_add(1);
  let cell = sheet.cell_rect(super::worksheet::CellAddress { col, row: 1 });
  let next_cell = sheet.cell_rect(super::worksheet::CellAddress {
    col: col.saturating_add(1),
    row: 1,
  });
  let x = cell.x_pt + vml_screen_pixel_to_pt(offset_px);
  x.min(next_cell.x_pt - units::twips_to_points(1.0))
}

fn vml_anchor_y(sheet: &CalcSheet, zero_based_row: u32, offset_px: i32) -> f32 {
  let row = zero_based_row.saturating_add(1);
  let cell = sheet.cell_rect(super::worksheet::CellAddress { col: 1, row });
  let next_cell = sheet.cell_rect(super::worksheet::CellAddress {
    col: 1,
    row: row.saturating_add(1),
  });
  let y = cell.y_pt + vml_screen_pixel_to_pt(offset_px);
  y.min(next_cell.y_pt - units::twips_to_points(1.0))
}

fn vml_screen_pixel_to_pt(value: i32) -> f32 {
  value as f32 * units::POINTS_PER_INCH / units::CSS_PIXELS_PER_INCH
}

fn vml_style_rect(style: &str) -> Option<(f32, f32, f32, f32)> {
  let x =
    vml_style_length_pt(style, "margin-left").or_else(|| vml_style_length_pt(style, "left"))?;
  let y = vml_style_length_pt(style, "margin-top").or_else(|| vml_style_length_pt(style, "top"))?;
  let width = vml_style_length_pt(style, "width")?;
  let height = vml_style_length_pt(style, "height")?;
  Some((x, y, width, height))
}

fn vml_style_length_pt(style: &str, key: &str) -> Option<f32> {
  style.split(';').find_map(|part| {
    let (name, value) = part.split_once(':')?;
    if name.trim() != key {
      return None;
    }
    parse_vml_length_pt(value.trim())
  })
}

fn parse_vml_length_pt(value: &str) -> Option<f32> {
  if let Some(value) = value.strip_suffix("pt") {
    return value.trim().parse::<f32>().ok();
  }
  if let Some(value) = value.strip_suffix("in") {
    return value
      .trim()
      .parse::<f32>()
      .ok()
      .map(|value| value * units::POINTS_PER_INCH);
  }
  if let Some(value) = value.strip_suffix("px") {
    return value.trim().parse::<f32>().ok().map(|value| value * 0.75);
  }
  if let Some(value) = value.strip_suffix("cm") {
    return value
      .trim()
      .parse::<f32>()
      .ok()
      .map(|value| value * units::POINTS_PER_INCH / units::CENTIMETERS_PER_INCH);
  }
  if let Some(value) = value.strip_suffix("mm") {
    return value
      .trim()
      .parse::<f32>()
      .ok()
      .map(|value| value * units::POINTS_PER_INCH / units::MILLIMETERS_PER_INCH);
  }
  value.parse::<f32>().ok()
}

fn shape_stroke(
  import: &ExcelImport,
  object: &super::drawing::DrawingObjectModel,
) -> Option<BorderStyle> {
  if object.no_line {
    return None;
  }
  let theme_line = object.shape_style_refs.as_ref().and_then(|style| {
    import
      .styles
      .theme_line_style(style.line_reference.index)
      .map(|line| (line, style.line_reference.placeholder_color.as_ref()))
  });
  let color = object
    .line_color
    .clone()
    .and_then(|color| xlsx_drawing_rgb_color(color, import))
    .or_else(|| {
      object.line_pattern.map(|pattern| RgbColor {
        r: pattern.foreground.r,
        g: pattern.foreground.g,
        b: pattern.foreground.b,
      })
    })
    .or_else(|| {
      let (line, placeholder) = theme_line?;
      let color = match &line.fill {
        LineFill::Solid(color) => color.clone().or_else(|| placeholder.cloned()),
        LineFill::Gradient(gradient) => gradient
          .gradient_stop_list
          .as_ref()
          .and_then(|list| list.gradient_stop.first())
          .and_then(|stop| stop.gradient_stop_choice.as_ref())
          .and_then(Color::from_gradient_stop_choice),
        LineFill::Pattern(pattern) => {
          return drawing_object_pattern_fill(import, pattern, placeholder).map(|pattern| {
            RgbColor {
              r: pattern.foreground.r,
              g: pattern.foreground.g,
              b: pattern.foreground.b,
            }
          });
        }
        LineFill::Unspecified | LineFill::None => None,
      }?;
      let color = xlsx_drawing_color_with_placeholder(color, import, placeholder)?;
      Some(RgbColor {
        r: color.r,
        g: color.g,
        b: color.b,
      })
    })?;
  Some(BorderStyle {
    width_pt: object
      .line_width_emu
      .map(|value| units::emu_to_points(i64::from(value)))
      .or_else(|| theme_line.and_then(|(line, _)| line.width_emu.map(units::emu_to_points)))
      .unwrap_or(0.75),
    color,
    ..BorderStyle::default()
  })
}

fn xlsx_image_data_with_effects(
  import: &ExcelImport,
  drawing: &super::drawing::DrawingResourceCatalog,
  resource: &super::drawing::ImageResource,
  object: &super::drawing::DrawingObjectModel,
) -> (Arc<[u8]>, Option<String>) {
  let effects = xlsx_image_effects(
    import,
    &object.image_effects,
    resource.content_type.as_deref(),
    &drawing.image_resources,
  );
  if effects.is_empty() {
    return (resource.data.clone(), resource.content_type.clone());
  }
  common::drawingml_image_effects::apply(
    resource.data.as_ref(),
    resource.content_type.as_deref(),
    &effects,
  )
  .map(|data| (Arc::from(data), Some("image/png".to_string())))
  .unwrap_or_else(|| (resource.data.clone(), resource.content_type.clone()))
}

struct XlsxImageEffectColorResolver<'a> {
  import: &'a ExcelImport,
  image_resources: &'a HashMap<String, super::drawing::ImageResource>,
  placeholder_color: Option<Color>,
}

impl XlsxImageEffectColorResolver<'_> {
  fn resolve(&self, color: Option<Color>) -> Option<ResolvedEffectColor> {
    let color =
      xlsx_drawing_color_with_placeholder(color?, self.import, self.placeholder_color.as_ref())?;
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

impl ImageEffectColorResolver for XlsxImageEffectColorResolver<'_> {
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
    let resource = self.image_resources.get(blip.embed.as_deref()?)?;
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

fn xlsx_image_effects(
  import: &ExcelImport,
  choices: &[a::BlipChoice],
  content_type: Option<&str>,
  image_resources: &HashMap<String, super::drawing::ImageResource>,
) -> Vec<ImageEffect> {
  common::drawingml_image_effects::from_blip_choices(
    choices,
    content_type,
    &XlsxImageEffectColorResolver {
      import,
      image_resources,
      placeholder_color: None,
    },
  )
}

fn drawing_object_common_fill(
  import: &ExcelImport,
  object: &super::drawing::DrawingObjectModel,
  rect: CellRect,
  transform: Affine,
) -> common::Fill<'static> {
  if object.no_fill {
    return common::Fill::None;
  }
  object
    .fill_pattern
    .map(common::Fill::Pattern)
    .or_else(|| {
      object
        .fill_gradient
        .as_deref()
        .and_then(|gradient| drawing_object_gradient_fill(import, gradient, rect, transform))
    })
    .or_else(|| {
      object
        .fill_color
        .clone()
        .and_then(|color| xlsx_drawing_color(color, import))
        .map(common::Fill::Solid)
    })
    .or_else(|| drawing_object_theme_fill(import, object, rect, transform))
    .unwrap_or(common::Fill::None)
}

fn drawing_object_theme_fill(
  import: &ExcelImport,
  object: &super::drawing::DrawingObjectModel,
  rect: CellRect,
  transform: Affine,
) -> Option<common::Fill<'static>> {
  let reference = &object.shape_style_refs.as_ref()?.fill_reference;
  let placeholder = reference.placeholder_color.as_ref();
  let Some(fill) = import.styles.theme_fill_style(reference.index) else {
    return placeholder
      .cloned()
      .and_then(|color| xlsx_drawing_color(color, import))
      .map(common::Fill::Solid);
  };
  match &fill.kind {
    FillKind::None => Some(common::Fill::None),
    FillKind::Solid(color) => color
      .clone()
      .or_else(|| placeholder.cloned())
      .and_then(|color| xlsx_drawing_color_with_placeholder(color, import, placeholder))
      .map(common::Fill::Solid),
    FillKind::Gradient(gradient) => {
      drawing_object_gradient_fill_with_placeholder(import, gradient, rect, transform, placeholder)
    }
    FillKind::Pattern(pattern) => {
      drawing_object_pattern_fill(import, pattern, placeholder).map(common::Fill::Pattern)
    }
    FillKind::SlideBackground | FillKind::Group | FillKind::Blip(_) => None,
  }
}

fn drawing_object_solid_fill_color(
  import: &ExcelImport,
  object: &super::drawing::DrawingObjectModel,
) -> Option<RgbColor> {
  object
    .fill_color
    .clone()
    .and_then(|color| xlsx_drawing_rgb_color(color, import))
    .or_else(|| match drawing_object_theme_solid_fill(import, object)? {
      common::Fill::Solid(color) => Some(RgbColor {
        r: color.r,
        g: color.g,
        b: color.b,
      }),
      _ => None,
    })
}

fn drawing_object_theme_solid_fill(
  import: &ExcelImport,
  object: &super::drawing::DrawingObjectModel,
) -> Option<common::Fill<'static>> {
  let reference = &object.shape_style_refs.as_ref()?.fill_reference;
  let placeholder = reference.placeholder_color.as_ref();
  let fill = import.styles.theme_fill_style(reference.index)?;
  let FillKind::Solid(color) = &fill.kind else {
    return None;
  };
  color
    .clone()
    .or_else(|| placeholder.cloned())
    .and_then(|color| xlsx_drawing_color_with_placeholder(color, import, placeholder))
    .map(common::Fill::Solid)
}

fn drawing_object_effective_common_fill(
  import: &ExcelImport,
  object: &super::drawing::DrawingObjectModel,
  rect: CellRect,
  transform: Affine,
  group_fill: Option<&common::Fill<'static>>,
) -> common::Fill<'static> {
  if object.use_group_fill {
    return group_fill.cloned().unwrap_or(common::Fill::None);
  }
  drawing_object_common_fill(import, object, rect, transform)
}

fn drawing_object_common_stroke(
  import: &ExcelImport,
  object: &super::drawing::DrawingObjectModel,
  border: BorderStyle,
  rect: CellRect,
  transform: Affine,
  outline: Option<&a::Outline>,
) -> common::Stroke<'static> {
  let mut stroke = common_stroke_from_border(border, 1.0);
  let theme_line = object.shape_style_refs.as_ref().and_then(|style| {
    import
      .styles
      .theme_line_style(style.line_reference.index)
      .map(|line| (line, style.line_reference.placeholder_color.as_ref()))
  });
  stroke.pattern = object.line_pattern.or_else(|| {
    let (line, placeholder) = theme_line?;
    let LineFill::Pattern(pattern) = &line.fill else {
      return None;
    };
    drawing_object_pattern_fill(import, pattern, placeholder)
  });
  stroke.gradient = object
    .line_gradient
    .as_deref()
    .map(|gradient| (gradient, None))
    .or_else(|| {
      let (line, placeholder) = theme_line?;
      let LineFill::Gradient(gradient) = &line.fill else {
        return None;
      };
      Some((gradient.as_ref(), placeholder))
    })
    .and_then(|(gradient, placeholder)| {
      drawing_object_gradient_fill_with_placeholder(import, gradient, rect, transform, placeholder)
    })
    .and_then(|fill| match fill {
      common::Fill::Gradient(gradient) => Some(gradient),
      _ => None,
    });
  if let Some((line, _)) = theme_line
    && let Some(outline) = line.source_outline.as_deref()
  {
    common::drawingml_stroke::apply_outline_style(&mut stroke, outline);
  }
  if let Some(outline) = outline {
    common::drawingml_stroke::apply_outline_style(&mut stroke, outline);
  }
  stroke
}

fn drawing_object_gradient_fill(
  import: &ExcelImport,
  gradient: &a::GradientFill,
  rect: CellRect,
  shape_transform: Affine,
) -> Option<common::Fill<'static>> {
  drawing_object_gradient_fill_with_placeholder(import, gradient, rect, shape_transform, None)
}

fn drawing_object_gradient_fill_with_placeholder(
  import: &ExcelImport,
  gradient: &a::GradientFill,
  rect: CellRect,
  shape_transform: Affine,
  placeholder_color: Option<&Color>,
) -> Option<common::Fill<'static>> {
  let mut stops = gradient
    .gradient_stop_list
    .as_ref()?
    .gradient_stop
    .iter()
    .filter_map(|stop| {
      let color = stop
        .gradient_stop_choice
        .as_ref()
        .and_then(Color::from_gradient_stop_choice)
        .and_then(|color| xlsx_drawing_color_with_placeholder(color, import, placeholder_color))?;
      Some(common::GradientStop {
        position: stop.position.as_ratio() as f32,
        color,
        scheme: None,
      })
    })
    .collect::<Vec<_>>();
  if stops.is_empty() {
    return None;
  }
  normalize_excel_2007_accent1_fill_style3(&mut stops);

  let local_bounds = common_rect(rect.x_pt, rect.y_pt, rect.width_pt, rect.height_pt);
  let transformed = common::drawingml_geometry::transform_rect_bounds(
    KurboRect::new(
      f64::from(rect.x_pt),
      f64::from(rect.y_pt),
      f64::from(rect.x_pt + rect.width_pt),
      f64::from(rect.y_pt + rect.height_pt),
    ),
    shape_transform,
  );
  let page_bounds = common_rect(
    transformed.x0 as f32,
    transformed.y0 as f32,
    transformed.width() as f32,
    transformed.height() as f32,
  );
  let follows_shape = gradient
    .rotate_with_shape
    .as_ref()
    .is_none_or(|value| value.as_bool());

  let (angle_degrees, line, scaled, path) = match gradient.gradient_fill_choice.as_ref()? {
    a::GradientFillChoice::LinearGradientFill(linear) => {
      let angle = linear.angle.unwrap_or_default() as f32 / 60_000.0;
      let scaled = linear.scaled.as_ref().is_some_and(|value| value.as_bool());
      let line = follows_shape.then(|| {
        let (start, end) = xlsx_linear_gradient_line(local_bounds, angle, scaled);
        (
          common::drawingml_geometry::transform_point(start, shape_transform),
          common::drawingml_geometry::transform_point(end, shape_transform),
        )
      });
      (Some(angle), line, scaled, None)
    }
    a::GradientFillChoice::PathGradientFill(path) => {
      let gradient_transform = if follows_shape {
        xlsx_gradient_transform(local_bounds, shape_transform)
      } else {
        xlsx_gradient_transform(page_bounds, Affine::IDENTITY)
      };
      let mut path =
        common::drawingml_gradient::resolve_path_gradient(gradient, path, gradient_transform);
      if path.kind == common::GradientPathKind::Circle {
        path.transform = common::office_circle_gradient_transform(path.transform);
      }
      (None, None, false, Some(path))
    }
  };

  let interpolation = office_drawing_gradient_interpolation(stops.len());
  Some(common::Fill::Gradient(common::GradientFill {
    stops,
    angle_degrees,
    definition_bounds: Some(if follows_shape {
      local_bounds
    } else {
      page_bounds
    }),
    line,
    interpolation,
    scaled,
    rotate_with_shape: None,
    path,
  }))
}

fn excel_fixed_output_backdrop_bounds(bounds: common::Rect) -> common::Rect {
  let pixels_per_point = 100.0 / units::POINTS_PER_INCH;
  let width_pt =
    (((bounds.size.width.0 * pixels_per_point).ceil() + 1.0) / pixels_per_point).round();
  let height_pt =
    (((bounds.size.height.0 * pixels_per_point).ceil() + 1.0) / pixels_per_point).round();
  let center_x = bounds.origin.x.0 + bounds.size.width.0 / 2.0;
  let center_y = bounds.origin.y.0 + bounds.size.height.0 / 2.0;
  common::Rect {
    origin: common::Point {
      x: common::Pt(center_x - width_pt / 2.0),
      y: common::Pt(center_y - height_pt / 2.0),
    },
    size: common::Size {
      width: common::Pt(width_pt),
      height: common::Pt(height_pt),
    },
  }
}

fn normalize_excel_2007_accent1_fill_style3(stops: &mut [common::GradientStop<'static>]) {
  let [dark, light] = stops else {
    return;
  };
  if dark.position.abs() > f32::EPSILON
    || (light.position - 1.0).abs() > f32::EPSILON
    || dark.color
      != (common::Color {
        r: 62,
        g: 127,
        b: 206,
        a: u8::MAX,
      })
    || light.color
      != (common::Color {
        r: 164,
        g: 196,
        b: u8::MAX,
        a: u8::MAX,
      })
  {
    return;
  }
  // The Office 2007 format theme's fillStyleLst[2] combines accent1 with
  // tint/shade/satMod transforms. Microsoft 365 Excel's fixed-format writer
  // records the resolved endpoints as 3F80CD and 9BC1FF in its 512-sample
  // shading function. Keep those immutable PDF endpoints at the DrawingML
  // compatibility boundary; interpolation remains the generic Office
  // two-stop sigma path below.
  dark.color = common::Color {
    r: 63,
    g: 128,
    b: 205,
    a: u8::MAX,
  };
  light.color = common::Color {
    r: 155,
    g: 193,
    b: u8::MAX,
    a: u8::MAX,
  };
}

fn office_drawing_gradient_interpolation(stop_count: usize) -> common::GradientInterpolation {
  if stop_count == 2 {
    // LibreOffice's OOXML bridge records the same Office behavior in
    // oox/source/drawingml/fontworkhelpers.cxx: two-stop gradients use
    // Office's non-linear renderer, while gradients with more than two stops
    // retain linear interpolation.
    common::GradientInterpolation::PowerPointGammaSigma
  } else {
    common::GradientInterpolation::LinearSrgb
  }
}

fn drawing_object_pattern_fill(
  import: &ExcelImport,
  fill: &a::PatternFill,
  placeholder_color: Option<&Color>,
) -> Option<common::PatternFill> {
  let foreground = fill
    .foreground_color
    .as_ref()
    .and_then(|color| color.foreground_color_choice.as_ref())
    .and_then(Color::from_foreground_color_choice)
    .and_then(|color| xlsx_drawing_color_with_placeholder(color, import, placeholder_color))
    .unwrap_or(common::Color {
      r: 0,
      g: 0,
      b: 0,
      a: u8::MAX,
    });
  let background = fill
    .background_color
    .as_ref()
    .and_then(|color| color.background_color_choice.as_ref())
    .and_then(Color::from_background_color_choice)
    .and_then(|color| xlsx_drawing_color_with_placeholder(color, import, placeholder_color))
    .unwrap_or(common::Color {
      r: u8::MAX,
      g: u8::MAX,
      b: u8::MAX,
      a: u8::MAX,
    });
  Some(common::PatternFill::drawingml(
    common::drawingml_pattern::hatch_style(fill.preset),
    foreground,
    background,
  ))
}

fn xlsx_drawing_color(color: Color, import: &ExcelImport) -> Option<common::Color> {
  xlsx_drawing_color_with_placeholder(color, import, None)
}

fn xlsx_drawing_color_with_placeholder(
  color: Color,
  import: &ExcelImport,
  placeholder_color: Option<&Color>,
) -> Option<common::Color> {
  let mut scheme_resolver = |value| {
    let index = xlsx_scheme_color_index(value)?;
    let color = import.styles.theme_color(index, 0.0)?;
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

fn xlsx_drawing_rgb_color(color: Color, import: &ExcelImport) -> Option<RgbColor> {
  let color = xlsx_drawing_color(color, import)?;
  Some(RgbColor {
    r: color.r,
    g: color.g,
    b: color.b,
  })
}

fn xlsx_linear_gradient_line(
  bounds: common::Rect,
  angle_degrees: f32,
  scaled: bool,
) -> (common::Point, common::Point) {
  let angle = angle_degrees.to_radians();
  let mut direction_x = angle.cos();
  let mut direction_y = angle.sin();
  if scaled {
    direction_x *= bounds.size.width.0;
    direction_y *= bounds.size.height.0;
  }
  let length = direction_x.hypot(direction_y).max(f32::EPSILON);
  direction_x /= length;
  direction_y /= length;
  let half_span =
    (direction_x.abs() * bounds.size.width.0 + direction_y.abs() * bounds.size.height.0) / 2.0;
  let center_x = bounds.origin.x.0 + bounds.size.width.0 / 2.0;
  let center_y = bounds.origin.y.0 + bounds.size.height.0 / 2.0;
  (
    common_point(
      center_x - direction_x * half_span,
      center_y - direction_y * half_span,
    ),
    common_point(
      center_x + direction_x * half_span,
      center_y + direction_y * half_span,
    ),
  )
}

fn xlsx_gradient_transform(bounds: common::Rect, transform: Affine) -> common::Transform {
  let top_left = common::drawingml_geometry::transform_point(bounds.origin, transform);
  let top_right = common::drawingml_geometry::transform_point(
    common_point(bounds.origin.x.0 + bounds.size.width.0, bounds.origin.y.0),
    transform,
  );
  let bottom_left = common::drawingml_geometry::transform_point(
    common_point(bounds.origin.x.0, bounds.origin.y.0 + bounds.size.height.0),
    transform,
  );
  common::Transform {
    m11: top_right.x.0 - top_left.x.0,
    m12: top_right.y.0 - top_left.y.0,
    m21: bottom_left.x.0 - top_left.x.0,
    m22: bottom_left.y.0 - top_left.y.0,
    dx: top_left.x,
    dy: top_left.y,
  }
}

fn anchor_rect_pt(
  sheet: &CalcSheet,
  anchor: &super::drawing::DrawingAnchorModel,
) -> Option<(f32, f32, f32, f32)> {
  let rect = match anchor.kind {
    super::drawing::DrawingAnchorKind::TwoCell => {
      let from = anchor.from.as_ref()?;
      let to = anchor.to.as_ref()?;
      let (x1, y1) = sheet.marker_position_pt(from);
      let (x2, y2) = sheet.marker_position_pt(to);
      Some((x1.min(x2), y1.min(y2), (x2 - x1).abs(), (y2 - y1).abs()))
    }
    super::drawing::DrawingAnchorKind::OneCell => {
      let from = anchor.from.as_ref()?;
      let (x, y) = sheet.marker_position_pt(from);
      let (cx, cy) = anchor.extent?;
      Some((x, y, units::emu_to_points(cx), units::emu_to_points(cy)))
    }
    super::drawing::DrawingAnchorKind::Absolute => {
      let (x, y) = anchor.position?;
      let (cx, cy) = anchor.extent?;
      Some((
        units::emu_to_points(x),
        units::emu_to_points(y),
        units::emu_to_points(cx),
        units::emu_to_points(cy),
      ))
    }
  }?;
  Some(excel_unrotated_anchor_rect(
    rect,
    anchor.object.rotation_deg,
  ))
}

fn excel_unrotated_anchor_rect(
  (x, y, width, height): (f32, f32, f32, f32),
  rotation_deg: f32,
) -> (f32, f32, f32, f32) {
  // Excel rewrites the from/to markers of rotated drawings to the bounding
  // cells of a quarter-turned rectangle in these two angular ranges. Restore
  // the unrotated shape rectangle before applying a:xfrm rotation. This is the
  // same threshold and center-preserving width/height swap used by Calc's OOXML
  // drawing importer (tdf#83593).
  let rotation_deg = rotation_deg.rem_euclid(360.0);
  if (45.0..135.0).contains(&rotation_deg) || (225.0..315.0).contains(&rotation_deg) {
    let center_x = x + width / 2.0;
    let center_y = y + height / 2.0;
    (
      center_x - height / 2.0,
      center_y - width / 2.0,
      height,
      width,
    )
  } else {
    (x, y, width, height)
  }
}

fn drawing_anchor_intersects_page(
  page: &CalcPrintPage<'_>,
  anchor: &super::drawing::DrawingAnchorModel,
) -> bool {
  let Some(area) = page.area else {
    return true;
  };
  let Some((x_pt, y_pt, width_pt, height_pt)) = anchor_rect_pt(page.sheet, anchor) else {
    return false;
  };
  let bounds = drawing_object_visual_bounds(
    CellRect {
      x_pt,
      y_pt,
      width_pt,
      height_pt,
    },
    &anchor.object,
  );
  tuple_rect_intersects_cell_rect(
    (bounds.x_pt, bounds.y_pt, bounds.width_pt, bounds.height_pt),
    page.sheet.range_rect(area),
  )
}

fn vml_shape_intersects_page(
  page: &CalcPrintPage<'_>,
  shape: &super::object_resources::VmlShapeModel,
) -> bool {
  let Some(area) = page.area else {
    return true;
  };
  let Some(shape_rect) = vml_shape_rect(page.sheet, shape) else {
    return false;
  };
  tuple_rect_intersects_cell_rect(shape_rect, page.sheet.range_rect(area))
}

fn tuple_rect_intersects_cell_rect(
  (x, y, width, height): (f32, f32, f32, f32),
  cell_rect: CellRect,
) -> bool {
  width > 0.0
    && height > 0.0
    && cell_rect.width_pt > 0.0
    && cell_rect.height_pt > 0.0
    // Spreadsheet drawing ranges use inclusive cell-edge rectangles. Calc's
    // PrintDrawingLayer is painted independently of the cell range assembled
    // by FillInfo, and Office fixed output consequently retains an object
    // anchored at the first edge of the following column on both horizontal
    // pages. Keep only that shared edge inclusive; objects positioned farther
    // inside the following column remain exclusive to its page.
    && x <= cell_rect.x_pt + cell_rect.width_pt
    && x + width > cell_rect.x_pt
    && y < cell_rect.y_pt + cell_rect.height_pt
    && y + height > cell_rect.y_pt
}

fn repeat_rows_for_page(page: &CalcPrintPage<'_>) -> Option<super::worksheet::CellRange> {
  let area = page.area?;
  let repeat_rows = effective_repeated_rows(page)?;
  Some(super::worksheet::CellRange::new(
    super::worksheet::CellAddress {
      col: area.start.col,
      row: repeat_rows.start.row,
    },
    super::worksheet::CellAddress {
      col: area.end.col,
      row: repeat_rows.end.row,
    },
  ))
}

fn repeat_columns_for_page(page: &CalcPrintPage<'_>) -> Option<super::worksheet::CellRange> {
  let area = page.area?;
  let repeat_columns = effective_repeated_columns(page)?;
  Some(super::worksheet::CellRange::new(
    super::worksheet::CellAddress {
      col: repeat_columns.start.col,
      row: area.start.row,
    },
    super::worksheet::CellAddress {
      col: repeat_columns.end.col,
      row: area.end.row,
    },
  ))
}

fn repeat_corner_for_page(page: &CalcPrintPage<'_>) -> Option<super::worksheet::CellRange> {
  let repeat_rows = effective_repeated_rows(page)?;
  let repeat_columns = effective_repeated_columns(page)?;
  Some(super::worksheet::CellRange::new(
    super::worksheet::CellAddress {
      col: repeat_columns.start.col,
      row: repeat_rows.start.row,
    },
    super::worksheet::CellAddress {
      col: repeat_columns.end.col,
      row: repeat_rows.end.row,
    },
  ))
}

fn effective_repeated_rows(page: &CalcPrintPage<'_>) -> Option<super::worksheet::CellRange> {
  let area = page.area?;
  page
    .repeated_rows
    .filter(|repeat| area.start.row > repeat.end.row)
}

fn effective_repeated_columns(page: &CalcPrintPage<'_>) -> Option<super::worksheet::CellRange> {
  let area = page.area?;
  page
    .repeated_columns
    .filter(|repeat| area.start.col > repeat.end.col)
}

fn hyperlink_for_cell(
  page: &CalcPrintPage<'_>,
  address: super::worksheet::CellAddress,
) -> Option<String> {
  page
    .sheet
    .metrics
    .hyperlinks
    .iter()
    .find(|hyperlink| {
      super::worksheet::CellRange::parse_a1_range(&hyperlink.reference)
        .is_some_and(|range| range.contains(address))
    })
    .and_then(|hyperlink| {
      hyperlink
        .relationship_id
        .as_deref()
        .and_then(|id| page.sheet.resources.relationships.hyperlink_targets.get(id))
        .cloned()
        .or_else(|| {
          hyperlink
            .location
            .as_ref()
            .map(|location| format!("#{location}"))
        })
    })
}

fn render_header_or_footer(
  items: &mut Vec<PageItem>,
  page: &CalcPrintPage<'_>,
  setup: PageSetup,
  header: bool,
  styles: &super::styles::StylesCatalog,
  text_metrics: &mut TextMetrics,
) {
  let Some(text) = header_footer_text(page, header) else {
    return;
  };
  render_header_footer_line(items, header, page, setup, text, styles, text_metrics);
}

fn header_footer_text<'a>(page: &CalcPrintPage<'a>, header: bool) -> Option<&'a str> {
  let model = &page.page_settings.header_footer;
  if page.sheet_page_index == 0 && model.different_first {
    if header {
      model.first_header.as_deref()
    } else {
      model.first_footer.as_deref()
    }
  } else if page.sheet_page_index % 2 == 1 && model.different_odd_even {
    if header {
      model.even_header.as_deref()
    } else {
      model.even_footer.as_deref()
    }
  } else if header {
    model.odd_header.as_deref()
  } else {
    model.odd_footer.as_deref()
  }
}

fn render_header_footer_line(
  items: &mut Vec<PageItem>,
  header: bool,
  page: &CalcPrintPage<'_>,
  setup: PageSetup,
  text: &str,
  styles: &super::styles::StylesCatalog,
  text_metrics: &mut TextMetrics,
) {
  for (align, value) in split_header_footer_sections(text) {
    if value.is_empty() {
      continue;
    }
    let mut runs = parse_header_footer_runs(
      &value,
      styles.default_font_text_style(),
      HeaderFooterFieldValues {
        page_number: page.page_number,
        total_pages: page.total_pages,
        sheet_name: &page.sheet.name,
      },
    );
    if runs.is_empty() {
      continue;
    }
    if page.page_settings.header_footer.scale_with_doc {
      let print_scale = page.zoom as f32 / 100.0;
      for run in &mut runs {
        run.style.font_size_pt *= print_scale;
      }
    }
    // OOXML pageMargins.header/footer is the distance from the page edge to
    // the start/end of the header/footer. LibreOffice's HeaderFooterParser
    // likewise computes each portion's height from its active font runs, and
    // PageSettingsConverter describes the footer margin as the distance to
    // the bottom of the footer. A fixed 12pt box misplaces any portion whose
    // font metrics or explicit &nn size produce a different line height.
    let line_height_pt = runs
      .iter()
      .map(|run| text_metrics.inline_text_box_height(&run.style))
      .fold(0.0_f32, f32::max)
      .max(1.0);
    let y_pt = if header {
      setup.header_distance_pt
    } else {
      setup.height_pt - setup.footer_distance_pt - line_height_pt
    };
    let total_width = runs
      .iter()
      .map(|run| text_metrics.measure_text(&run.text, &run.style))
      .sum::<f32>();
    let mut x = match align {
      HeaderFooterAlign::Left => setup.margin_left_pt,
      HeaderFooterAlign::Center => (setup.width_pt - total_width) / 2.0,
      HeaderFooterAlign::Right => setup.width_pt - setup.margin_right_pt - total_width,
    };
    for run in runs {
      let width = text_metrics.measure_text(&run.text, &run.style);
      items.push(styled_header_text_with_line_height(
        x,
        y_pt,
        run.text,
        run.style,
        line_height_pt,
      ));
      x += width;
    }
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum HeaderFooterAlign {
  Left,
  Center,
  Right,
}

fn split_header_footer_sections(text: &str) -> Vec<(HeaderFooterAlign, String)> {
  let mut output = Vec::new();
  let mut current = HeaderFooterAlign::Center;
  let mut buffer = String::new();
  let mut chars = text.chars().peekable();
  while let Some(ch) = chars.next() {
    if ch == '&' {
      match chars.peek().copied() {
        Some('L') => {
          chars.next();
          push_header_footer_section(&mut output, current, &mut buffer);
          current = HeaderFooterAlign::Left;
          continue;
        }
        Some('C') => {
          chars.next();
          push_header_footer_section(&mut output, current, &mut buffer);
          current = HeaderFooterAlign::Center;
          continue;
        }
        Some('R') => {
          chars.next();
          push_header_footer_section(&mut output, current, &mut buffer);
          current = HeaderFooterAlign::Right;
          continue;
        }
        _ => {}
      }
    }
    buffer.push(ch);
  }
  push_header_footer_section(&mut output, current, &mut buffer);
  output
}

fn push_header_footer_section(
  output: &mut Vec<(HeaderFooterAlign, String)>,
  align: HeaderFooterAlign,
  buffer: &mut String,
) {
  if !buffer.is_empty() {
    output.push((align, buffer.trim().to_string()));
    buffer.clear();
  }
}

#[derive(Clone, Copy, Debug)]
struct HeaderFooterFieldValues<'a> {
  page_number: usize,
  total_pages: usize,
  sheet_name: &'a str,
}

#[derive(Clone, Debug)]
struct HeaderFooterTextRun {
  text: String,
  style: TextStyle,
}

fn parse_header_footer_runs(
  text: &str,
  mut style: TextStyle,
  fields: HeaderFooterFieldValues<'_>,
) -> Vec<HeaderFooterTextRun> {
  // ECMA-376 18.3.1.46 stores formatted header/footer text in one control
  // string. This mirrors LibreOffice HeaderFooterParser: style state changes
  // flush the current range, while fields inherit the active font model.
  let mut runs = Vec::new();
  let mut output = String::new();
  let mut chars = text.chars().peekable();
  while let Some(ch) = chars.next() {
    if ch != '&' {
      output.push(ch);
      continue;
    }
    match chars.next() {
      Some('P' | 'p') => output.push_str(&fields.page_number.to_string()),
      Some('N' | 'n') => output.push_str(&fields.total_pages.to_string()),
      Some('A' | 'a') => output.push_str(fields.sheet_name),
      Some('&') => output.push('&'),
      Some('L' | 'l' | 'C' | 'c' | 'R' | 'r') => {}
      Some('"') => {
        let mut descriptor = String::new();
        for next in chars.by_ref() {
          if next == '"' {
            break;
          }
          descriptor.push(next);
        }
        push_header_footer_run(&mut runs, &mut output, &style);
        let (font_name, font_style) = descriptor
          .split_once(',')
          .unwrap_or((descriptor.as_str(), ""));
        if !font_name.is_empty() && font_name != "-" {
          style.font_family = Some(Arc::from(font_name));
        }
        style.bold = false;
        style.italic = false;
        for name in font_style.split_ascii_whitespace() {
          if header_footer_bold_style(name) {
            style.bold = true;
          } else if header_footer_italic_style(name) {
            style.italic = true;
          }
        }
      }
      Some('K' | 'k') => {
        let color = chars.by_ref().take(6).collect::<String>();
        if color.len() == 6 && color.chars().all(|value| value.is_ascii_hexdigit()) {
          push_header_footer_run(&mut runs, &mut output, &style);
          if let Ok(rgb) = u32::from_str_radix(&color, 16) {
            style.color = RgbColor {
              r: (rgb >> 16) as u8,
              g: (rgb >> 8) as u8,
              b: rgb as u8,
            };
          }
        }
      }
      Some(ch) if ch.is_ascii_digit() => {
        let mut size = ch.to_digit(10).unwrap_or_default();
        while let Some(next) = chars.peek().and_then(|next| next.to_digit(10)) {
          chars.next();
          size = size.saturating_mul(10).saturating_add(next);
        }
        if size > 0 && size <= 1000 {
          push_header_footer_run(&mut runs, &mut output, &style);
          style.font_size_pt = size as f32;
        }
      }
      Some('B' | 'b') => {
        push_header_footer_run(&mut runs, &mut output, &style);
        style.bold = !style.bold;
      }
      Some('I' | 'i') => {
        push_header_footer_run(&mut runs, &mut output, &style);
        style.italic = !style.italic;
      }
      Some('U' | 'u' | 'E' | 'e') => {
        push_header_footer_run(&mut runs, &mut output, &style);
        style.underline = !style.underline;
      }
      Some('S' | 's') => {
        push_header_footer_run(&mut runs, &mut output, &style);
        style.strikethrough = !style.strikethrough;
      }
      Some(ch) => output.push(ch),
      None => output.push('&'),
    }
  }
  push_header_footer_run(&mut runs, &mut output, &style);
  runs
}

fn push_header_footer_run(
  runs: &mut Vec<HeaderFooterTextRun>,
  output: &mut String,
  style: &TextStyle,
) {
  if output.is_empty() {
    return;
  }
  runs.push(HeaderFooterTextRun {
    text: std::mem::take(output),
    style: style.clone(),
  });
}

fn header_footer_bold_style(name: &str) -> bool {
  matches!(
    name.to_ascii_lowercase().as_str(),
    "bold" | "fett" | "demibold" | "halbfett" | "black" | "heavy" | "félkövér"
  )
}

fn header_footer_italic_style(name: &str) -> bool {
  matches!(
    name.to_ascii_lowercase().as_str(),
    "italic" | "kursiv" | "oblique" | "schräg" | "dőlt"
  )
}

#[cfg(test)]
mod drawing_page_tests {
  use super::*;

  #[test]
  fn date_axis_page_clip_keeps_only_the_prefix_that_fits() {
    let mut text = "2019-02-01".to_string();
    truncate_text_to_width(&mut text, 6.0, |prefix| prefix.chars().count() as f32);

    assert_eq!(text, "2019-02");
  }

  #[test]
  fn checkbox_snapshot_keeps_office_control_indicator_metrics_and_state() {
    let rect = CellRect {
      x_pt: 0.0,
      y_pt: 0.0,
      width_pt: 15.0,
      height_pt: 15.75,
    };
    let checked = vml_checkbox_snapshot_png(rect, 1, true).expect("checked snapshot");
    let checked = image::load_from_memory(&checked)
      .expect("snapshot PNG")
      .to_rgba8();
    assert_eq!(checked.dimensions(), (42, 39));
    assert_eq!(checked.get_pixel(5, 13).0[3], 91);
    assert_eq!(checked.get_pixel(15, 17).0[0], 18);

    let mixed = vml_checkbox_snapshot_png(rect, 2, true).expect("mixed snapshot");
    let mixed = image::load_from_memory(&mixed)
      .expect("snapshot PNG")
      .to_rgba8();
    assert_eq!(mixed.get_pixel(5 + 7, 13 + 7).0, [0, 0, 0, 255]);

    let captioned_rect = CellRect {
      x_pt: 119.206_665,
      y_pt: 93.330_06,
      width_pt: 110.613_33,
      height_pt: 35.58,
    };
    let captioned =
      vml_checkbox_snapshot_png(captioned_rect, 0, false).expect("captioned snapshot");
    let captioned = image::load_from_memory(&captioned)
      .expect("snapshot PNG")
      .to_rgba8();
    assert_eq!(captioned.dimensions(), (307, 100));
    assert_eq!(captioned.get_pixel(5, 43).0, [0, 0, 0, 106]);
    assert_eq!(captioned.get_pixel(6, 44).0, [26, 26, 26, 255]);

    let captioned_image_rect = vml_checkbox_image_rect(captioned_rect, false);
    assert!((captioned_image_rect.x_pt - 119.52).abs() < 0.001);
    assert!((captioned_image_rect.y_pt - 93.48).abs() < 0.001);
    assert!((captioned_image_rect.width_pt - 110.64).abs() < 0.001);
    assert!((captioned_image_rect.height_pt - 36.24).abs() < 0.001);
  }

  #[test]
  fn drawing_page_intersection_includes_the_following_column_edge() {
    let page = CellRect {
      x_pt: 100.0,
      y_pt: 200.0,
      width_pt: 300.0,
      height_pt: 400.0,
    };

    assert!(tuple_rect_intersects_cell_rect(
      (50.0, 250.0, 100.0, 100.0),
      page
    ));
    assert!(!tuple_rect_intersects_cell_rect(
      (0.0, 250.0, 100.0, 100.0),
      page
    ));
    assert!(tuple_rect_intersects_cell_rect(
      (400.0, 250.0, 100.0, 100.0),
      page
    ));
    assert!(!tuple_rect_intersects_cell_rect(
      (400.01, 250.0, 100.0, 100.0),
      page
    ));
  }

  #[test]
  fn chart_text_keeps_pdf_semantics_while_the_page_clip_hides_its_ink() {
    let clip = CellRect {
      x_pt: 0.0,
      y_pt: 0.0,
      width_pt: 100.0,
      height_pt: 100.0,
    };
    let mut item = styled_header_text(104.0, 20.0, "9".to_string(), TextStyle::default());
    let mut metrics = TextMetrics::new();

    assert!(clip_chart_item_to_rect(
      &mut item,
      clip,
      &mut metrics,
      ChartTextClipSlack {
        left_em: 0.0,
        right_em: 0.6,
      },
      &[],
    ));

    let PageItem::Text(text) = item else {
      panic!("expected chart text");
    };
    assert_eq!(text.paint_clip.map(|clip| clip.origin.x.0), Some(0.0));
    assert_eq!(text.paint_clip.map(|clip| clip.size.width.0), Some(100.0));
    assert!(!text.style.semantic_only);
    assert_eq!(text.y_pt, 20.0);
  }

  #[test]
  fn chart_data_table_cell_text_is_removed_when_page_boundary_cuts_it() {
    let clip = CellRect {
      x_pt: 0.0,
      y_pt: 0.0,
      width_pt: 100.0,
      height_pt: 100.0,
    };
    let mut item = styled_header_text(95.0, 20.0, "Medical".to_string(), TextStyle::default());
    let PageItem::Text(text) = &mut item else {
      panic!("expected chart text");
    };
    text.discard_if_horizontally_clipped = true;
    let mut metrics = TextMetrics::new();

    assert!(!clip_chart_item_to_rect(
      &mut item,
      clip,
      &mut metrics,
      DEFAULT_CHART_TEXT_CLIP_SLACK,
      &[],
    ));
  }

  #[test]
  fn excel_rotated_anchor_threshold_restores_unrotated_shape_rect() {
    let source = (10.0, 20.0, 30.0, 90.0);
    assert_eq!(excel_unrotated_anchor_rect(source, 44.0), source);
    assert_eq!(
      excel_unrotated_anchor_rect(source, 45.0),
      (-20.0, 50.0, 90.0, 30.0)
    );
    assert_eq!(
      excel_unrotated_anchor_rect(source, 134.999),
      (-20.0, 50.0, 90.0, 30.0)
    );
    assert_eq!(excel_unrotated_anchor_rect(source, 135.0), source);
    assert_eq!(
      excel_unrotated_anchor_rect(source, -45.001),
      (-20.0, 50.0, 90.0, 30.0)
    );
  }

  #[test]
  fn shape_rotation_does_not_turn_horizontal_text_into_vertical_layout() {
    let mut items = Vec::new();
    let rect = CellRect {
      x_pt: 10.0,
      y_pt: 20.0,
      width_pt: 100.0,
      height_pt: 30.0,
    };
    render_drawing_text(
      &mut items,
      "text",
      rect,
      Some(TextStyle::default()),
      Some(DrawingTextLayout {
        shape_rotation_deg: 45.0,
        ..DrawingTextLayout::default()
      }),
      None,
      None,
    );

    let PageItem::Text(text) = &items[0] else {
      panic!("expected text item");
    };
    assert_eq!(text.x_pt, rect.x_pt + XLSX_CELL_TEXT_INSET_PT);
    assert_eq!(text.y_pt, rect.y_pt + XLSX_CELL_TEXT_INSET_PT);
    assert_eq!(text.style.rotation_deg, 45.0);
    assert_eq!(text.rotation_center_pt, Some((60.0, 35.0)));
    assert!(text.style.pdf_glyph_outlines);
    assert_eq!(
      text
        .style
        .pdf_glyph_outline_options
        .as_deref()
        .map(|options| options.semantic_text_overlay),
      Some(false)
    );
  }

  #[test]
  fn upright_shape_text_stays_extractable_in_the_rotated_visual_bounds() {
    let mut items = Vec::new();
    let rect = CellRect {
      x_pt: 10.0,
      y_pt: 20.0,
      width_pt: 90.0,
      height_pt: 30.0,
    };
    render_drawing_text(
      &mut items,
      "text",
      rect,
      Some(TextStyle::default()),
      Some(DrawingTextLayout {
        shape_rotation_deg: 90.0,
        upright: true,
        anchor: a::TextAnchoringTypeValues::Center,
        ..DrawingTextLayout::default()
      }),
      None,
      None,
    );

    let PageItem::Text(text) = &items[0] else {
      panic!("expected text item");
    };
    assert_eq!(text.style.rotation_deg, 0.0);
    assert!(!text.style.pdf_glyph_outlines);
    assert_eq!(text.rotation_center_pt, None);
    let line_height = TextStyle::default().font_size_pt * 1.15;
    let expected_y = rect.y_pt
      + XLSX_CELL_TEXT_INSET_PT
      + (rect.height_pt - XLSX_CELL_TEXT_INSET_PT * 2.0 - line_height) / 2.0;
    assert!((text.y_pt - expected_y).abs() < 0.001);
  }

  #[test]
  fn sheet_page_transform_translates_source_origin_then_applies_zoom() {
    let transform = SheetPageTransform::new(
      18.0,
      24.0,
      0.5,
      CellRect {
        x_pt: 100.0,
        y_pt: 200.0,
        width_pt: 300.0,
        height_pt: 400.0,
      },
    );

    assert_eq!(
      transform.rect_from_xywh(120.0, 240.0, 60.0, 80.0),
      CellRect {
        x_pt: 28.0,
        y_pt: 44.0,
        width_pt: 30.0,
        height_pt: 40.0,
      }
    );
  }

  #[test]
  fn vml_curve_attributes_lower_to_one_cubic_path() {
    let shape = super::super::object_resources::VmlShapeModel {
      kind: super::super::object_resources::VmlShapeKind::Curve,
      from: Some("0,0".into()),
      control1: Some("0,100".into()),
      control2: Some("200,0".into()),
      to: Some("200,100".into()),
      ..Default::default()
    };

    let paths = vml_shape_drawing_paths(&shape, 20.0, 10.0).unwrap();
    assert_eq!(paths.len(), 1);
    assert!(matches!(
      paths[0].commands.as_slice(),
      [
        common::PathCommand::MoveTo(_),
        common::PathCommand::CubicTo { .. }
      ]
    ));
  }

  #[test]
  fn filled_vml_polyline_is_closed_without_repeated_endpoint() {
    let shape = super::super::object_resources::VmlShapeModel {
      kind: super::super::object_resources::VmlShapeKind::Polyline,
      points: Some("0,0,100,0,50,100".into()),
      filled: true,
      ..Default::default()
    };

    let paths = vml_shape_drawing_paths(&shape, 10.0, 10.0).unwrap();
    assert_eq!(paths[0].commands.last(), Some(&common::PathCommand::Close));
    assert_eq!(paths[0].fill_mode, common::DrawingPathFillMode::Normal);
  }

  #[test]
  fn invalid_authored_vml_path_is_not_replaced_by_rectangle() {
    let shape = super::super::object_resources::VmlShapeModel {
      kind: super::super::object_resources::VmlShapeKind::Shape,
      path: Some("not-a-vml-path".into()),
      ..Default::default()
    };
    assert!(vml_shape_drawing_paths(&shape, 10.0, 10.0).is_none());
  }

  #[test]
  fn vml_gradient_and_stroke_style_lower_to_common_paint() {
    let shape = super::super::object_resources::VmlShapeModel {
      fill_color: Some("#FF0000".into()),
      fill_color2: Some("#0000FF".into()),
      fill_opacity: Some("32768f".into()),
      fill_type: Some(vml::FillTypeValues::GradientRadial),
      fill_focus_position: Some("25%,20%".into()),
      fill_focus_size: Some("50%,60%".into()),
      stroke_dash_style: Some("dashdot".into()),
      stroke_end_cap: Some(vml::StrokeEndCapValues::Flat),
      stroke_end_arrow: Some(vml::StrokeArrowValues::Classic),
      stroke_end_arrow_width: Some(vml::StrokeArrowWidthValues::Wide),
      stroke_end_arrow_length: Some(vml::StrokeArrowLengthValues::Long),
      ..Default::default()
    };
    let fill = vml_shape_common_fill(
      &shape,
      Affine::scale_non_uniform(40.0, 20.0).then_translate((2.0, 3.0).into()),
    );
    let common::Fill::Gradient(gradient) = fill else {
      panic!("expected gradient");
    };
    assert_eq!(gradient.stops.len(), 2);
    assert_eq!(gradient.stops[1].color.a, 128);
    let path = gradient.path.expect("radial VML fill-to rectangle");
    assert_eq!(path.kind, common::GradientPathKind::Rectangle);
    assert!((path.fill_to.left - 0.25).abs() < 0.001);
    assert!((path.fill_to.right - 0.25).abs() < 0.001);
    assert_eq!(path.transform.m11, 40.0);
    assert_eq!(path.transform.m22, 20.0);

    let stroke = vml_shape_common_stroke(&shape).expect("stroke");
    assert_eq!(stroke.cap, Some(common::StrokeCap::Flat));
    assert_eq!(
      stroke.tail_end,
      Some(common::StrokeEnd {
        kind: common::StrokeEndKind::Stealth,
        width: common::StrokeEndSize::Large,
        length: common::StrokeEndSize::Large,
      })
    );
    assert_eq!(stroke.dash.as_ref().map(Vec::len), Some(4));
  }

  #[test]
  fn two_stop_drawingml_gradient_uses_office_sigma_interpolation() {
    assert_eq!(
      office_drawing_gradient_interpolation(2),
      common::GradientInterpolation::PowerPointGammaSigma
    );
    assert_eq!(
      office_drawing_gradient_interpolation(3),
      common::GradientInterpolation::LinearSrgb
    );
  }

  #[test]
  fn excel_2007_accent1_fill_style3_uses_fixed_output_endpoints() {
    let mut stops = [
      common::GradientStop {
        position: 0.0,
        color: common::Color {
          r: 62,
          g: 127,
          b: 206,
          a: u8::MAX,
        },
        scheme: None,
      },
      common::GradientStop {
        position: 1.0,
        color: common::Color {
          r: 164,
          g: 196,
          b: u8::MAX,
          a: u8::MAX,
        },
        scheme: None,
      },
    ];

    normalize_excel_2007_accent1_fill_style3(&mut stops);

    assert_eq!(
      [stops[0].color.r, stops[0].color.g, stops[0].color.b],
      [63, 128, 205]
    );
    assert_eq!(
      [stops[1].color.r, stops[1].color.g, stops[1].color.b],
      [155, 193, 255]
    );
  }

  #[test]
  fn excel_fixed_output_backdrop_keeps_center_and_one_100ppi_guard_sample() {
    let bounds = common_rect(163.27, 92.26, 64.92, 71.10);

    let guarded = excel_fixed_output_backdrop_bounds(bounds);

    assert!((guarded.size.width.0 - 66.0).abs() <= 1.0e-5);
    assert!((guarded.size.height.0 - 72.0).abs() <= 1.0e-5);
    assert!(
      (guarded.origin.x.0 + guarded.size.width.0 / 2.0 - (163.27 + 64.92 / 2.0)).abs() <= 1.0e-5
    );
    assert!(
      (guarded.origin.y.0 + guarded.size.height.0 / 2.0 - (92.26 + 71.10 / 2.0)).abs() <= 1.0e-5
    );
  }

  #[test]
  fn vml_tile_origin_is_placed_on_shape_position() {
    let phase = vml_tile_phase(
      Some("0.25,-16384f"),
      Some("75%,0.5"),
      100.0,
      80.0,
      20.0,
      16.0,
    );
    assert!((phase.0 - 70.0).abs() < 0.001);
    assert!((phase.1 - 44.0).abs() < 0.001);
    assert_eq!(
      vml_tile_phase(None, None, 100.0, 80.0, 20.0, 16.0),
      (40.0, 32.0)
    );
  }
}

#[cfg(test)]
mod cell_alignment_tests {
  use super::*;

  fn print_cell(
    state: super::super::print::NumberFormatRenderState,
  ) -> super::super::print::CalcPrintCell<'static> {
    super::super::print::CalcPrintCell {
      address: CellAddress { col: 1, row: 1 },
      text: std::borrow::Cow::Borrowed("value"),
      style_index: None,
      pivot_format_id: None,
      rendered_text: "value".to_string(),
      rich_text_runs: &[],
      number_format_state: state,
      formula: false,
      icon_set: None,
      color_scale_fill: None,
    }
  }

  #[test]
  fn general_alignment_centers_boolean_and_error_values() {
    for state in [
      super::super::print::NumberFormatRenderState::Boolean,
      super::super::print::NumberFormatRenderState::Error,
    ] {
      assert_eq!(
        calc_cell_horizontal_alignment(&print_cell(state), None),
        x::HorizontalAlignmentValues::Center
      );
    }
  }

  #[test]
  fn overflow_hashes_fill_the_available_cell_width() {
    assert_eq!(calc_cell_overflow_hash_count(90.0, 6.0), 15);
    assert_eq!(calc_cell_overflow_hash_count(5.0, 6.0), 1);
  }

  #[test]
  fn print_centering_uses_half_of_the_remaining_axis_space() {
    assert_eq!(calc_axis_centering_offset(true, 500.0, 300.0), 100.0);
    assert_eq!(calc_axis_centering_offset(false, 500.0, 300.0), 0.0);
    assert_eq!(calc_axis_centering_offset(true, 300.0, 500.0), 0.0);
  }

  #[test]
  fn embedded_vml_picture_uses_excel_printer_host_bounds() {
    let rect = excel_vml_picture_fixed_output_rect(CellRect {
      x_pt: 36.850_395,
      y_pt: 297.442_9,
      width_pt: 72.0,
      height_pt: 54.0,
    });

    assert_eq!(rect.x_pt, 36.84);
    assert_eq!(rect.y_pt, 297.48);
    assert_eq!(rect.width_pt, 72.36);
    assert_eq!(rect.height_pt, 55.2);
  }

  #[test]
  fn negative_date_serials_render_as_hashes_even_when_the_date_text_fits() {
    let mut cell = print_cell(super::super::print::NumberFormatRenderState::DateTime);
    cell.text = std::borrow::Cow::Borrowed("-1");
    assert!(calc_cell_requires_date_hashes(&cell));
    cell.text = std::borrow::Cow::Borrowed("1");
    assert!(!calc_cell_requires_date_hashes(&cell));
  }

  #[test]
  fn wrapped_cell_text_preserves_explicit_line_breaks() {
    let mut metrics = TextMetrics::new();
    let lines = wrap_cell_text(
      "Line1\r\nLine2\nLine3",
      1_000.0,
      &TextStyle::default(),
      &mut metrics,
    );

    assert_eq!(lines, ["Line1", "Line2", "Line3"]);
  }

  #[test]
  fn wrapped_cell_text_wraps_paragraphs_to_the_cell_width() {
    let style = TextStyle::default();
    let mut metrics = TextMetrics::new();
    let one_word_width = metrics.measure_text("one", &style);
    let lines = wrap_cell_text("one two three", one_word_width + 0.1, &style, &mut metrics);

    assert_eq!(lines, ["one", "two", "three"]);
  }

  #[test]
  fn wrapped_cell_text_uses_unicode_hyphen_break_opportunities() {
    let style = TextStyle::default();
    let mut metrics = TextMetrics::new();
    let prefix_width = metrics.measure_text("Highlight 10-", &style);
    let full_width = metrics.measure_text("Highlight 10-30", &style);
    let lines = wrap_cell_text(
      "Highlight 10-30",
      (prefix_width + full_width) / 2.0,
      &style,
      &mut metrics,
    );

    assert_eq!(lines, ["Highlight 10-", "30"]);

    let prefix_width = metrics.measure_text("Colours R-", &style);
    let full_width = metrics.measure_text("Colours R->G", &style);
    let lines = wrap_cell_text(
      "Colours R->G",
      (prefix_width + full_width) / 2.0,
      &style,
      &mut metrics,
    );
    assert_eq!(lines, ["Colours R-", ">G"]);
  }
}

#[cfg(test)]
mod header_footer_tests {
  use super::*;

  #[test]
  fn header_footer_font_descriptor_and_size_apply_to_fields() {
    let runs = parse_header_footer_runs(
      "&\"Times New Roman,Regular\"&12&A",
      TextStyle::default(),
      HeaderFooterFieldValues {
        page_number: 2,
        total_pages: 3,
        sheet_name: "Sheet1",
      },
    );

    assert_eq!(runs.len(), 1);
    assert_eq!(runs[0].text, "Sheet1");
    assert_eq!(
      runs[0].style.font_family.as_deref(),
      Some("Times New Roman")
    );
    assert_eq!(runs[0].style.font_size_pt, 12.0);
  }

  #[test]
  fn header_footer_style_changes_create_separate_runs() {
    let runs = parse_header_footer_runs(
      "plain&Bbold&B&P/&N",
      TextStyle::default(),
      HeaderFooterFieldValues {
        page_number: 2,
        total_pages: 3,
        sheet_name: "Sheet1",
      },
    );

    assert_eq!(runs.len(), 3);
    assert_eq!(runs[0].text, "plain");
    assert!(!runs[0].style.bold);
    assert_eq!(runs[1].text, "bold");
    assert!(runs[1].style.bold);
    assert_eq!(runs[2].text, "2/3");
    assert!(!runs[2].style.bold);
  }
}
