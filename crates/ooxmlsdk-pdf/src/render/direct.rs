use std::borrow::Cow;
use std::io::Write;
use std::num::NonZeroU32;
use std::sync::{Arc, OnceLock};

use flate2::Compression;
use flate2::write::ZlibEncoder;
use pdf_writer::types::{
  Direction, LineCapStyle, LineJoinStyle, NumberingStyle, PageLayout, PageMode, TabOrder,
  TextRenderingMode,
};
use pdf_writer::{Content, Filter, Finish, Name, Null, Pdf, Rect, Ref, Settings, Str, TextStr};
use rustc_hash::FxHashMap as HashMap;

use super::direct_conformance::{ConformanceObjects, DirectConformance};
use super::direct_font::{DirectFontSet, RegisteredGlyph};
use super::direct_form::DirectFormSet;
use super::direct_glyph::{GlyphOutlinePlacement, build_glyph_outline_path};
use super::direct_gradient::{
  DirectGradientSet, GradientOpacity, RegisteredGradient, RegisteredGradientPattern,
};
use super::direct_image::DirectImageSet;
use super::direct_link::DirectPageLinks;
use super::direct_outline::DirectOutline;
use super::direct_path_gradient::{PathGradientProfile, PathGradientRasterSampling};
use super::direct_pattern::DirectPatternSet;
use super::direct_tag::{DirectPageTags, DirectTaggedPageSource, DirectTagging};
use super::image::{
  DirectRasterColorSpace, DirectRasterEncoding, DirectRasterImage, ImageSet, PdfRasterPixels,
  PreparedRasterImage,
};
use super::link::{InternalLinkTargets, LinkRect};
use super::marker::{
  StrokeMarkerGeometry, shortened_straight_polyline_points, stroke_marker_geometries,
};
use super::page::{PageLabelSpec, PageSelection, page_label_spec, pdf_page_dimension};
use crate::error::{PdfError, Result};
use crate::options::{
  PdfImageOptimizationPolicy, PdfOptions, PdfPageLayout, PdfStandard, PdfViewerMagnification,
  PdfViewerPageMode,
};
use crate::{
  PdfConversionDiagnostics, PdfConversionOutput, PdfFontAudit, PdfFontAuditOutput,
  PdfPageDiagnostics,
};
use ooxmlsdk_layout::common;

/// Transitional direct-serialization backend.
///
/// Each supported feature is lowered directly into `pdf-writer` objects. A
/// layout feature that has not been migrated must fail here instead of being
/// silently omitted or delegated to the legacy backend.
pub(super) fn render(
  document: &common::LayoutDocument<'static>,
  options: &PdfOptions,
) -> Result<Vec<u8>> {
  render_inner(document, options, RenderObservation::None).map(|output| output.pdf)
}

pub(super) fn render_with_diagnostics(
  document: &common::LayoutDocument<'static>,
  options: &PdfOptions,
) -> Result<PdfConversionOutput> {
  let output = render_inner(document, options, RenderObservation::Diagnostics)?;
  Ok(PdfConversionOutput {
    pdf: output.pdf,
    diagnostics: output.diagnostics,
  })
}

pub(super) fn render_with_font_audit(
  document: &common::LayoutDocument<'static>,
  options: &PdfOptions,
) -> Result<PdfFontAuditOutput> {
  let output = render_inner(document, options, RenderObservation::FontAudit)?;
  Ok(PdfFontAuditOutput {
    pdf: output.pdf,
    audit: output.font_audit,
  })
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RenderObservation {
  None,
  Diagnostics,
  FontAudit,
}

struct RenderOutput {
  pdf: Vec<u8>,
  diagnostics: PdfConversionDiagnostics,
  font_audit: PdfFontAudit,
}

fn render_inner(
  document: &common::LayoutDocument<'static>,
  options: &PdfOptions,
  observation: RenderObservation,
) -> Result<RenderOutput> {
  let options = pdf_ua_render_options(document, options)?;
  let options = options.as_ref();
  let conformance = DirectConformance::from_options(options)?;
  let selection =
    PageSelection::from_range(document.pages.len(), options.general.page_range.as_deref())?;
  ensure_page_subset_is_supported(document, options, &selection)?;

  let needs_preparation = selection
    .source_indices
    .iter()
    .any(|&source_index| display_items_need_preparation(&document.pages[source_index].items));
  let paint = needs_preparation.then(|| {
    let all_paint = super::paint::prepare_for_direct(document, options);
    super::paint::PaintDocument {
      pages: selection
        .source_indices
        .iter()
        .map(|&source_index| all_paint.pages[source_index].clone())
        .collect(),
    }
  });
  let diagnostics = if observation == RenderObservation::Diagnostics {
    paint.as_ref().map_or_else(
      || empty_page_diagnostics(document, &selection),
      super::paint::conversion_diagnostics,
    )
  } else {
    PdfConversionDiagnostics::default()
  };
  let font_audit = if observation == RenderObservation::FontAudit {
    paint.as_ref().map_or_else(PdfFontAudit::default, |paint| {
      super::paint::conversion_font_audit_for_direct(paint)
    })
  } else {
    PdfFontAudit::default()
  };
  let pdf = write_page_document(document, options, &selection, paint.as_ref(), conformance)?;
  Ok(RenderOutput {
    pdf,
    diagnostics,
    font_audit,
  })
}

fn pdf_ua_render_options<'a>(
  document: &common::LayoutDocument<'static>,
  options: &'a PdfOptions,
) -> Result<Cow<'a, PdfOptions>> {
  let requests_pdf_ua = options.general.pdf_ua_compliance
    || options
      .standards
      .iter()
      .any(|standard| matches!(standard, PdfStandard::PdfUa1));
  if !requests_pdf_ua {
    return Ok(Cow::Borrowed(options));
  }
  if options
    .metadata
    .title
    .as_deref()
    .is_some_and(|title| !title.trim().is_empty())
  {
    return Ok(Cow::Borrowed(options));
  }

  let title = options
    .source_file_name
    .as_deref()
    .map(str::trim)
    .filter(|title| !title.is_empty())
    .or_else(|| {
      document
        .outline_entries
        .iter()
        .map(|entry| entry.text.trim())
        .find(|title| !title.is_empty())
    })
    .ok_or_else(|| {
      PdfError::Options(
        "PDF/UA requires a non-empty metadata title, source file name, or outline title"
          .to_string(),
      )
    })?;
  let mut effective = options.clone();
  effective.metadata.title = Some(title.to_string());
  Ok(Cow::Owned(effective))
}

fn ensure_page_subset_is_supported(
  document: &common::LayoutDocument<'static>,
  options: &PdfOptions,
  selection: &PageSelection,
) -> Result<()> {
  for &source_index in &selection.source_indices {
    for item in &document.pages[source_index].items {
      ensure_display_item_supported(item)?;
    }
  }
  // Office's built-in fixed-format exporters produce a static PDF view of
  // document controls even when the source contains editable content
  // controls. Keep `form_widget_id` as layout metadata and paint its resolved
  // value normally; interactive AcroForm export is a separate capability.
  if options.watermark.is_some() {
    return Err(PdfError::DirectWriterUnsupported {
      feature: "watermarks",
    });
  }
  Ok(())
}

fn ensure_display_item_supported(item: &common::DisplayItem<'static>) -> Result<()> {
  match item {
    common::DisplayItem::Rect(rect) => {
      ensure_fill_supported(&rect.fill, rect.bounds)?;
      if let Some(stroke) = &rect.stroke {
        ensure_stroke_supported(stroke, StrokeUse::Rectangle)?;
      }
      validate_rect(rect.bounds)
    }
    common::DisplayItem::Line(line) => {
      ensure_stroke_supported(&line.stroke, StrokeUse::Line)?;
      validate_point(line.start)?;
      validate_point(line.end)
    }
    common::DisplayItem::Path(path) => {
      ensure_fill_supported(&path.fill, path.bounds)?;
      if let Some(stroke) = &path.stroke {
        ensure_path_stroke_supported(stroke, path.closed)?;
        if let Some(gradient) = &stroke.gradient {
          DirectGradientSet::validate(gradient, path.bounds)?;
        }
      }
      validate_rect(path.bounds)?;
      for point in &path.points {
        validate_point(*point)?;
      }
      for command in &path.commands {
        validate_path_command(*command)?;
      }
      Ok(())
    }
    common::DisplayItem::Text(text) => {
      validate_point(text.origin)?;
      if !text.line_height.0.is_finite() || text.line_height.0 < 0.0 {
        return Err(PdfError::Writer(format!(
          "invalid text line height {} pt",
          text.line_height.0
        )));
      }
      Ok(())
    }
    common::DisplayItem::Glyphs(_) => unsupported("explicit glyph-run painting"),
    common::DisplayItem::Image(image) => {
      validate_rect(image.bounds)?;
      if !image.rotation_degrees.is_finite() {
        return Err(PdfError::Writer(
          "non-finite image rotation angle".to_string(),
        ));
      }
      for command in &image.clip_path {
        validate_path_command(*command)?;
      }
      Ok(())
    }
    common::DisplayItem::Group(group) => ensure_group_supported(group),
    // Link rectangles use the legacy annotation boundary: malformed or
    // degenerate rectangles are omitted instead of failing visible content.
    common::DisplayItem::LinkArea(_) => Ok(()),
    common::DisplayItem::AnnotationHint(_) => unsupported("annotation hints"),
    common::DisplayItem::Clip(_) => unsupported("display-list clipping operations"),
    common::DisplayItem::Transform(_) => unsupported("display-list transform operations"),
  }
}

fn ensure_group_supported(group: &common::CompositingGroup<'static>) -> Result<()> {
  if group.mask.is_some() {
    return unsupported("group alpha masks");
  }
  if group.transform.is_some() {
    return unsupported("transformed compositing groups");
  }
  if group.blend_mode != common::BlendMode::Normal {
    return unsupported("group blend modes");
  }
  opacity_alpha(group.opacity)?;
  if let Some(clip) = group.clip {
    validate_rect(clip)?;
  }
  for item in &group.items {
    ensure_display_item_supported(item)?;
  }
  Ok(())
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum StrokeUse {
  Rectangle,
  Line,
  Path,
}

fn ensure_fill_supported(fill: &common::Fill<'static>, bounds: common::Rect) -> Result<()> {
  match fill {
    common::Fill::None => Ok(()),
    common::Fill::Solid(_) => Ok(()),
    common::Fill::Gradient(gradient) => DirectGradientSet::validate(gradient, bounds),
    common::Fill::Pattern(pattern) => DirectPatternSet::validate(*pattern),
    common::Fill::Theme(_) => unsupported("unresolved theme fills"),
    common::Fill::Image { .. } => unsupported("image fills"),
  }
}

fn ensure_stroke_supported(stroke: &common::Stroke<'static>, usage: StrokeUse) -> Result<()> {
  if !stroke.width.0.is_finite() || stroke.width.0 < 0.0 {
    return Err(PdfError::Writer(format!(
      "invalid stroke width {} pt",
      stroke.width.0
    )));
  }
  if stroke.pattern.is_some() {
    return unsupported("tiling-pattern strokes");
  }
  if !matches!(stroke.compound, None | Some(common::StrokeCompound::Single)) {
    return unsupported("compound strokes");
  }
  if stroke.alignment == Some(common::StrokeAlignment::Inside) {
    return unsupported("inside-aligned strokes");
  }
  if stroke.gradient.is_some() {
    if usage != StrokeUse::Path {
      return unsupported("gradient strokes on non-path display items");
    }
    if stroke.width.0 <= f32::EPSILON {
      return unsupported("zero-width gradient strokes");
    }
    if stroke_has_visible_endpoint_markers(stroke) {
      return unsupported("gradient stroke endpoint markers");
    }
  }
  if usage != StrokeUse::Path && stroke_has_visible_endpoint_markers(stroke) {
    return unsupported("stroke endpoint markers");
  }
  if usage == StrokeUse::Rectangle
    && (stroke.dash.is_some()
      || stroke.preset_dash.is_some()
      || stroke.dash_offset.0 != 0.0
      || stroke.cap.is_some()
      || stroke.join.is_some())
  {
    return unsupported("styled rectangle outlines");
  }
  if !stroke.dash_offset.0.is_finite() {
    return Err(PdfError::Writer(
      "non-finite stroke dash offset".to_string(),
    ));
  }
  if let Some(values) = stroke.resolved_dash()
    && values
      .iter()
      .any(|value| !value.0.is_finite() || value.0 < 0.0)
  {
    return Err(PdfError::Writer(
      "stroke dash array contains an invalid length".to_string(),
    ));
  }
  if let Some(common::StrokeJoin::Miter { limit: Some(limit) }) = stroke.join
    && (!limit.is_finite() || limit < 1.0)
  {
    return Err(PdfError::Writer(format!(
      "invalid stroke miter limit {limit}"
    )));
  }
  Ok(())
}

fn ensure_path_stroke_supported(stroke: &common::Stroke<'static>, closed: bool) -> Result<()> {
  if !stroke_is_compound(stroke) {
    return ensure_stroke_supported(stroke, StrokeUse::Path);
  }
  if !stroke.width.0.is_finite() || stroke.width.0 < 0.0 {
    return Err(PdfError::Writer(format!(
      "invalid stroke width {} pt",
      stroke.width.0
    )));
  }
  if stroke.pattern.is_some() {
    return unsupported("tiling-pattern strokes");
  }
  if stroke.alignment == Some(common::StrokeAlignment::Inside) {
    return unsupported("inside-aligned strokes");
  }
  if stroke.width.0 <= f32::EPSILON {
    return unsupported("zero-width compound strokes");
  }
  if stroke.gradient.is_some() {
    return unsupported("compound gradient strokes");
  }
  if stroke.resolved_dash().is_some() {
    return unsupported("dashed compound strokes");
  }
  if stroke_has_visible_endpoint_markers(stroke) {
    return unsupported("compound stroke endpoint markers");
  }
  if !closed {
    return unsupported("open compound strokes");
  }
  Ok(())
}

fn stroke_is_compound(stroke: &common::Stroke<'static>) -> bool {
  !matches!(stroke.compound, None | Some(common::StrokeCompound::Single))
}

fn stroke_has_visible_endpoint_markers(stroke: &common::Stroke<'static>) -> bool {
  stroke
    .head_end
    .is_some_and(|end| end.kind != common::StrokeEndKind::None)
    || stroke
      .tail_end
      .is_some_and(|end| end.kind != common::StrokeEndKind::None)
}

fn validate_rect(rect: common::Rect) -> Result<()> {
  validate_point(rect.origin)?;
  if !rect.size.width.0.is_finite() || !rect.size.height.0.is_finite() {
    return Err(PdfError::Writer("non-finite rectangle size".to_string()));
  }
  Ok(())
}

fn validate_point(point: common::Point) -> Result<()> {
  if !point.x.0.is_finite() || !point.y.0.is_finite() {
    return Err(PdfError::Writer("non-finite path coordinate".to_string()));
  }
  Ok(())
}

fn validate_path_command(command: common::PathCommand) -> Result<()> {
  match command {
    common::PathCommand::MoveTo(point) | common::PathCommand::LineTo(point) => {
      validate_point(point)
    }
    common::PathCommand::CubicTo {
      control1,
      control2,
      end,
    } => {
      validate_point(control1)?;
      validate_point(control2)?;
      validate_point(end)
    }
    common::PathCommand::Close => Ok(()),
  }
}

fn unsupported(feature: &'static str) -> Result<()> {
  Err(PdfError::DirectWriterUnsupported { feature })
}

fn empty_page_diagnostics(
  document: &common::LayoutDocument<'static>,
  selection: &PageSelection,
) -> PdfConversionDiagnostics {
  PdfConversionDiagnostics {
    fonts: Vec::new(),
    pages: selection
      .source_indices
      .iter()
      .enumerate()
      .map(|(page_index, &source_index)| {
        let source_page = &document.pages[source_index];
        PdfPageDiagnostics {
          page_index,
          width_pt: pdf_page_dimension(document.engine_kind, source_page.setup.size.width.0),
          height_pt: pdf_page_dimension(document.engine_kind, source_page.setup.size.height.0),
          text_runs: Vec::new(),
        }
      })
      .collect(),
  }
}

fn write_page_document(
  document: &common::LayoutDocument<'static>,
  options: &PdfOptions,
  selection: &PageSelection,
  paint: Option<&super::paint::PaintDocument<'_>>,
  conformance: DirectConformance,
) -> Result<Vec<u8>> {
  let settings = Settings { pretty: false };
  let version = actual_text_version_floor(
    ordinary_pdf_version(options)?,
    paint.is_some_and(paint_document_requires_actual_text),
  );
  let mut refs = RefAllocator::default();
  let catalog_id = refs.alloc()?;
  let page_tree_id = refs.alloc()?;
  let page_objects = selection
    .source_indices
    .iter()
    .map(|_| Ok((refs.alloc()?, refs.alloc()?)))
    .collect::<Result<Vec<_>>>()?;
  let page_heights_pt = selection
    .source_indices
    .iter()
    .map(|&source_index| {
      pdf_page_dimension(
        document.engine_kind,
        document.pages[source_index].setup.size.height.0,
      )
    })
    .collect::<Vec<_>>();
  let outline = DirectOutline::allocate(document, options, selection, &mut refs)?;
  let page_label_entries = page_label_entries(document, selection)
    .into_iter()
    .map(|(output_index, label)| Ok((output_index, label, refs.alloc()?)))
    .collect::<Result<Vec<_>>>()?;
  let named_destinations = collect_named_destinations(document, options, selection, &mut refs)?;
  let attachment_objects =
    super::direct_attachment::AttachmentObjects::allocate(options, || refs.alloc())?;
  let metadata_objects =
    super::direct_metadata::MetadataObjects::allocate(options, version, || refs.alloc())?;
  let conformance_objects = conformance.allocate(&mut refs)?;
  let supports_associated_files = conformance.supports_associated_files(version);
  let mut tagging = super::settings::requests_tagging(options)
    .then(|| DirectTagging::allocate(&mut refs))
    .transpose()?;
  let mut fonts = DirectFontSet::default();
  let mut image_policy = ImageSet::default();
  let mut images = DirectImageSet::default();
  let mut patterns = DirectPatternSet::new(
    options.compress_content_streams,
    document.engine_kind,
    options.optimize_for,
  );
  let mut forms = DirectFormSet::default();
  let internal_links = paint
    .map(|paint| InternalLinkTargets::from_layout(paint, document, selection))
    .unwrap_or_default();

  let mut pdf = Pdf::with_settings(settings);
  let (major, minor) = version;
  pdf.set_version(major, minor);
  pdf
    .pages(page_tree_id)
    .kids(page_objects.iter().map(|&(page_id, _)| page_id))
    .count(
      i32::try_from(page_objects.len())
        .map_err(|_| PdfError::Writer("page count exceeds the PDF integer range".to_string()))?,
    );

  for (output_index, (&source_index, &(page_id, content_id))) in selection
    .source_indices
    .iter()
    .zip(&page_objects)
    .enumerate()
  {
    let source_page = &document.pages[source_index];
    let width_pt = pdf_page_dimension(document.engine_kind, source_page.setup.size.width.0);
    let height_pt = pdf_page_dimension(document.engine_kind, source_page.setup.size.height.0);
    validate_page_size(width_pt, height_pt)?;

    let mut resources = PageResources::new(width_pt, height_pt, path_gradient_profile(options));
    let mut page_links = DirectPageLinks::default();
    let mut content = Content::with_settings(settings);
    content.save_state().transform([
      // Layout uses a top-left origin while PDF page space uses a bottom-left
      // origin. Freeze the coordinate-system boundary in the root stream so
      // every display-list lowering shares the same transform.
      1.0, 0.0, 0.0, -1.0, 0.0, height_pt,
    ]);
    let paint_page = if display_items_need_preparation(&source_page.items) {
      Some(
        paint
          .and_then(|paint| paint.pages.get(output_index))
          .ok_or_else(|| {
            PdfError::Writer(format!(
              "prepared PDF paint page {output_index} for source page {source_index} is missing"
            ))
          })?,
      )
    } else {
      None
    };
    let mut page_tags = tagging.as_ref().map(|_| DirectPageTags::default());
    if let Some(paint_page) = paint_page {
      let mut writer = PreparedPageWriter {
        options,
        resources: &mut resources,
        fonts: &mut fonts,
        image_policy: &mut image_policy,
        images: &mut images,
        patterns: &mut patterns,
        forms: &mut forms,
        refs: &mut refs,
        internal_links: &internal_links,
        page_links: &mut page_links,
        requires_codepoint_mappings: conformance.requires_codepoint_mappings(),
        forbids_private_use_mappings: conformance.forbids_private_use_mappings(),
        page_width_pt: width_pt,
        page_height_pt: height_pt,
      };
      // The prepared display list can retain table cells and drawing objects
      // outside the physical page. PDF page boxes suppress their ink but not
      // necessarily their extractable text, so preserve the source-backed
      // fixed-output culling boundary before assigning tags or annotations.
      for (item_index, item) in paint_page
        .items
        .iter()
        .enumerate()
        .filter(|(_, item)| super::paint::paint_item_intersects_page(item, width_pt, height_pt))
      {
        let annotation_start = writer.page_links.len();
        let marked = page_tags
          .as_mut()
          .map(|tags| tags.begin_prepared(&mut content, item_index, item))
          .transpose()?
          .unwrap_or(false);
        write_prepared_item(&mut content, item, &mut writer)?;
        if marked {
          content.end_marked_content();
        }
        if let Some(tags) = &mut page_tags {
          tags.finish_item(item_index, annotation_start, writer.page_links.len())?;
        }
      }
    } else {
      for (item_index, item) in source_page.items.iter().enumerate() {
        let marked = page_tags
          .as_mut()
          .map(|tags| tags.begin_display(&mut content, item_index, item))
          .transpose()?
          .unwrap_or(false);
        write_display_item(
          &mut content,
          item,
          &mut DisplayPageWriter {
            engine_kind: document.engine_kind,
            resources: &mut resources,
            patterns: &mut patterns,
            images: &mut images,
            refs: &mut refs,
            page_height_pt: height_pt,
          },
        )?;
        if marked {
          content.end_marked_content();
        }
      }
    }
    content.restore_state();
    let content = content.finish();
    write_content_stream(
      &mut pdf,
      content_id,
      content.as_slice(),
      options.compress_content_streams,
    )?;
    resources.write_objects(&mut pdf);

    let struct_parent_key = match (&mut tagging, page_tags) {
      (Some(tagging), Some(page_tags)) => tagging.finish_page(
        DirectTaggedPageSource {
          document,
          page_index: source_index,
          page_id,
        },
        paint_page,
        page_tags,
        &mut page_links,
        &mut refs,
      )?,
      (None, None) => None,
      _ => {
        return Err(PdfError::Writer(
          "direct tagged-PDF page state is inconsistent".to_string(),
        ));
      }
    };
    page_links.write_objects(&mut pdf, height_pt, &page_objects, &page_heights_pt)?;

    let mut page = pdf.page(page_id);
    page
      .parent(page_tree_id)
      .media_box(Rect::new(0.0, 0.0, width_pt, height_pt))
      .contents(content_id);
    {
      let mut dictionary = page.resources();
      resources.write_dictionary(&mut dictionary);
    }
    if let Some(key) = struct_parent_key {
      page.struct_parents(key);
    }
    if conformance.requires_structure_tab_order() {
      page.tab_order(TabOrder::StructureOrder);
    }
    if !page_links.is_empty() {
      page.annotations(page_links.ids());
    }
    page.finish();
  }

  fonts.write_objects(&mut pdf)?;
  patterns.write_objects(&mut pdf)?;
  images.write_objects(&mut pdf)?;
  forms.write_objects(&mut pdf)?;
  if let Some(tagging) = &tagging {
    tagging.write_objects(&mut pdf);
  }
  if let Some(outline) = &outline {
    outline.write_objects(&mut pdf, document, selection, &page_objects)?;
  }

  write_page_label_objects(&mut pdf, &page_label_entries)?;
  write_named_destination_objects(
    &mut pdf,
    document,
    selection,
    &page_objects,
    &named_destinations,
  );
  attachment_objects.write_objects(&mut pdf, version, supports_associated_files)?;
  conformance.write_objects(&mut pdf, conformance_objects)?;
  super::direct_metadata::write(
    &mut pdf,
    metadata_objects,
    options,
    version,
    page_objects.len(),
    conformance,
  )?;
  write_catalog(
    &mut pdf,
    CatalogObjects {
      catalog_id,
      page_tree_id,
      page_objects: &page_objects,
      page_label_entries: &page_label_entries,
      named_destinations: &named_destinations,
      attachments: &attachment_objects,
      metadata_id: metadata_objects.xmp_id(),
      tagging: tagging.as_ref(),
      outline: outline.as_ref(),
      supports_associated_files,
      conformance,
      conformance_objects,
    },
    options,
  )?;

  Ok(pdf.finish())
}

struct CatalogObjects<'a, 'attachment> {
  catalog_id: Ref,
  page_tree_id: Ref,
  page_objects: &'a [(Ref, Ref)],
  page_label_entries: &'a [(usize, Option<PageLabelSpec>, Ref)],
  named_destinations: &'a [NamedDestinationEntry],
  attachments: &'a super::direct_attachment::AttachmentObjects<'attachment>,
  metadata_id: Ref,
  tagging: Option<&'a DirectTagging>,
  outline: Option<&'a DirectOutline>,
  supports_associated_files: bool,
  conformance: DirectConformance,
  conformance_objects: ConformanceObjects,
}

fn write_catalog(
  pdf: &mut Pdf,
  objects: CatalogObjects<'_, '_>,
  options: &PdfOptions,
) -> Result<()> {
  let CatalogObjects {
    catalog_id,
    page_tree_id,
    page_objects,
    page_label_entries,
    named_destinations,
    attachments,
    metadata_id,
    tagging,
    outline,
    supports_associated_files,
    conformance,
    conformance_objects,
  } = objects;
  let mut catalog = pdf.catalog(catalog_id);
  catalog.pages(page_tree_id);
  catalog.metadata(metadata_id);

  if let Some(tagging) = tagging {
    catalog.pair(Name(b"StructTreeRoot"), tagging.root_id());
    let mut mark_info = catalog.mark_info();
    mark_info.marked(true);
    if conformance.requests_pdf_ua() {
      mark_info.suspects(false);
    }
    mark_info.finish();
  }
  if let Some(outline) = outline {
    catalog.outlines(outline.root_id());
  }

  if let Some(language) = options.canonical_document_language() {
    catalog.lang(TextStr(&language));
  }

  if !page_label_entries.is_empty() {
    let mut tree = catalog.page_labels();
    let mut numbers = tree.nums();
    for &(output_index, _, label_id) in page_label_entries {
      let output_index = i32::try_from(output_index).map_err(|_| {
        PdfError::Writer("page-label index exceeds the PDF integer range".to_string())
      })?;
      numbers.insert(output_index, label_id);
    }
    numbers.finish();
    tree.finish();
  }

  if !named_destinations.is_empty() || !attachments.is_empty() {
    let mut names = catalog.names();
    if !named_destinations.is_empty() {
      let mut tree = names.destinations();
      let mut entries = tree.names();
      for destination in named_destinations {
        entries.insert(Str(destination.name.as_bytes()), destination.id);
      }
      entries.finish();
      tree.finish();
    }
    if !attachments.is_empty() {
      attachments.write_name_tree(&mut names);
    }
    names.finish();
  }

  attachments.write_catalog_associations(&mut catalog, supports_associated_files);
  conformance.write_catalog(&mut catalog, conformance_objects);

  match options.viewer.page_layout {
    PdfPageLayout::Default => {}
    PdfPageLayout::SinglePage => {
      catalog.page_layout(PageLayout::SinglePage);
    }
    PdfPageLayout::Continuous => {
      catalog.page_layout(PageLayout::OneColumn);
    }
    PdfPageLayout::ContinuousFacing => {
      catalog.page_layout(PageLayout::TwoColumnRight);
    }
  }

  let requested_page_mode = viewer_page_mode(options.viewer.page_mode);
  if options.viewer.full_screen {
    catalog.page_mode(PageMode::FullScreen);
  } else if options.viewer.page_mode != PdfViewerPageMode::Default {
    catalog.page_mode(requested_page_mode);
  }

  let display_document_title = options.metadata.title.as_deref().is_some_and(|title| {
    !title.is_empty() && (options.viewer.display_document_title || conformance.requests_pdf_ua())
  });
  let needs_preferences = options.viewer.hide_toolbar
    || options.viewer.hide_menubar
    || options.viewer.hide_window_controls
    || options.viewer.fit_window
    || options.viewer.center_window
    || options.viewer.first_page_left
    || options.viewer.full_screen
    || display_document_title;
  if needs_preferences {
    let mut preferences = catalog.viewer_preferences();
    if options.viewer.hide_toolbar {
      preferences.hide_toolbar(true);
    }
    if options.viewer.hide_menubar {
      preferences.hide_menubar(true);
    }
    if options.viewer.hide_window_controls {
      preferences.pair(Name(b"HideWindowUI"), true);
    }
    if options.viewer.fit_window {
      preferences.fit_window(true);
    }
    if options.viewer.center_window {
      preferences.center_window(true);
    }
    if display_document_title {
      preferences.display_doc_title(true);
    }
    if options.viewer.first_page_left {
      preferences.direction(Direction::R2L);
    }
    if options.viewer.full_screen {
      preferences.non_full_screen_page_mode(requested_page_mode);
    }
    preferences.finish();
  }

  let needs_open_action = options.viewer.initial_page != 1
    || options.viewer.magnification != PdfViewerMagnification::Default;
  if needs_open_action {
    let page_index = usize::try_from(options.viewer.initial_page)
      .ok()
      .and_then(|page| page.checked_sub(1))
      .filter(|&page| page < page_objects.len())
      .ok_or_else(|| {
        PdfError::Options(format!(
          "viewer initial page {} exceeds the generated PDF page count {}",
          options.viewer.initial_page,
          page_objects.len()
        ))
      })?;
    let page_id = page_objects[page_index].0;
    let mut action = catalog.insert(Name(b"OpenAction")).array();
    action.item(page_id);
    match options.viewer.magnification {
      PdfViewerMagnification::Default => {
        action.item(Name(b"XYZ"));
        action.item(Null);
        action.item(Null);
        action.item(0);
      }
      PdfViewerMagnification::FitInWindow => {
        action.item(Name(b"Fit"));
      }
      PdfViewerMagnification::FitWidth => {
        action.item(Name(b"FitH"));
        action.item(Null);
      }
      PdfViewerMagnification::FitVisible => {
        action.item(Name(b"FitBH"));
        action.item(Null);
      }
      PdfViewerMagnification::Zoom(zoom) => {
        action.item(Name(b"XYZ"));
        action.item(Null);
        action.item(Null);
        action.item(zoom);
      }
    }
    action.finish();
  }
  catalog.finish();
  Ok(())
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct NamedDestinationEntry {
  name: String,
  output_page_index: usize,
  id: Ref,
}

fn collect_named_destinations(
  document: &common::LayoutDocument<'static>,
  options: &PdfOptions,
  selection: &PageSelection,
  refs: &mut RefAllocator,
) -> Result<Vec<NamedDestinationEntry>> {
  if !options.links.export_bookmarks_to_pdf_destinations {
    return Ok(Vec::new());
  }

  let mut destinations = document
    .anchor_pages
    .iter()
    .filter_map(|anchor| {
      let output_page_index = selection.output_index(anchor.page_index)?;
      (!anchor.name.is_empty() && anchor.page_index < document.pages.len())
        .then(|| (anchor.name.to_string(), output_page_index))
    })
    .collect::<Vec<_>>();
  destinations.sort_by(|left, right| left.0.as_bytes().cmp(right.0.as_bytes()));
  if let Some(duplicate) = destinations
    .windows(2)
    .find(|pair| pair[0].0.as_bytes() == pair[1].0.as_bytes())
  {
    return Err(PdfError::Options(format!(
      "bookmark '{}' resolves to more than one PDF destination",
      duplicate[0].0
    )));
  }

  destinations
    .into_iter()
    .map(|(name, output_page_index)| {
      Ok(NamedDestinationEntry {
        name,
        output_page_index,
        id: refs.alloc()?,
      })
    })
    .collect()
}

fn write_named_destination_objects(
  pdf: &mut Pdf,
  document: &common::LayoutDocument<'static>,
  selection: &PageSelection,
  page_objects: &[(Ref, Ref)],
  destinations: &[NamedDestinationEntry],
) {
  for destination in destinations {
    let source_page_index = selection.source_indices[destination.output_page_index];
    let page_height = pdf_page_dimension(
      document.engine_kind,
      document.pages[source_page_index].setup.size.height.0,
    );
    pdf
      .destination(destination.id)
      .page(page_objects[destination.output_page_index].0)
      .xyz(0.0, page_height, None);
  }
}

fn page_label_entries(
  document: &common::LayoutDocument<'static>,
  selection: &PageSelection,
) -> Vec<(usize, Option<PageLabelSpec>)> {
  let labels = selection
    .source_indices
    .iter()
    .map(|&source_index| page_label_spec(document, source_index))
    .collect::<Vec<_>>();
  if labels.iter().all(Option::is_none) {
    return Vec::new();
  }

  let mut entries = Vec::new();
  let mut previous = None;
  for (output_index, label) in labels.into_iter().enumerate() {
    let continues = previous.is_some_and(|previous| page_label_continues(previous, label));
    if output_index == 0 || !continues {
      entries.push((output_index, label));
    }
    previous = Some(label);
  }
  entries
}

fn page_label_continues(previous: Option<PageLabelSpec>, current: Option<PageLabelSpec>) -> bool {
  match (previous, current) {
    (None, None) => true,
    (Some(previous), Some(current)) => {
      let previous = previous.start.map_or(1, NonZeroU32::get);
      let current = current.start.map_or(1, NonZeroU32::get);
      previous.checked_add(1) == Some(current)
    }
    _ => false,
  }
}

fn write_page_label_objects(
  pdf: &mut Pdf,
  entries: &[(usize, Option<PageLabelSpec>, Ref)],
) -> Result<()> {
  for &(_, label, label_id) in entries {
    let mut object = pdf
      .indirect(label_id)
      .start::<pdf_writer::writers::PageLabel>();
    if let Some(label) = label {
      object.style(NumberingStyle::Arabic);
      if let Some(start) = label.start {
        let start = i32::try_from(start.get()).map_err(|_| {
          PdfError::Writer("page-label start exceeds the PDF integer range".to_string())
        })?;
        object.offset(start);
      }
    }
    object.finish();
  }
  Ok(())
}

fn viewer_page_mode(mode: PdfViewerPageMode) -> PageMode {
  match mode {
    PdfViewerPageMode::Default => PageMode::UseNone,
    PdfViewerPageMode::UseOutlines => PageMode::UseOutlines,
    PdfViewerPageMode::UseThumbs => PageMode::UseThumbs,
  }
}

fn display_items_need_preparation(items: &[common::DisplayItem<'static>]) -> bool {
  items.iter().any(|item| match item {
    common::DisplayItem::Text(_)
    | common::DisplayItem::Image(_)
    | common::DisplayItem::LinkArea(_) => true,
    common::DisplayItem::Path(path) => path
      .stroke
      .as_ref()
      .is_some_and(|stroke| stroke.head_end.is_some() || stroke.tail_end.is_some()),
    common::DisplayItem::Group(group) => {
      !group.flatten_identity
        || (group.opacity - 1.0).abs() > f32::EPSILON
        || display_items_need_preparation(&group.items)
    }
    common::DisplayItem::Glyphs(_)
    | common::DisplayItem::AnnotationHint(_)
    | common::DisplayItem::Clip(_)
    | common::DisplayItem::Transform(_)
    | common::DisplayItem::Rect(_)
    | common::DisplayItem::Line(_) => false,
  })
}

struct PreparedPageWriter<'a> {
  options: &'a PdfOptions,
  resources: &'a mut PageResources,
  fonts: &'a mut DirectFontSet,
  image_policy: &'a mut ImageSet,
  images: &'a mut DirectImageSet,
  patterns: &'a mut DirectPatternSet,
  forms: &'a mut DirectFormSet,
  refs: &'a mut RefAllocator,
  internal_links: &'a InternalLinkTargets,
  page_links: &'a mut DirectPageLinks,
  requires_codepoint_mappings: bool,
  forbids_private_use_mappings: bool,
  page_width_pt: f32,
  page_height_pt: f32,
}

fn write_prepared_item(
  content: &mut Content,
  item: &super::paint::PaintItem<'_>,
  writer: &mut PreparedPageWriter<'_>,
) -> Result<()> {
  match item {
    super::paint::PaintItem::Text(text) => write_prepared_text(content, text, writer),
    super::paint::PaintItem::Rect(rect) => write_prepared_rect(content, rect, writer),
    super::paint::PaintItem::Line(line) => {
      write_prepared_line(content, line, writer.resources, writer.refs)
    }
    super::paint::PaintItem::Polyline(path) => write_prepared_polyline(content, path, writer),
    super::paint::PaintItem::Group {
      mask,
      clip,
      transform,
      blend_mode,
      opacity,
      flatten_identity,
      items,
    } => {
      if mask.is_some() {
        return unsupported("group alpha masks");
      }
      if transform.is_some() {
        return unsupported("transformed compositing groups");
      }
      if *blend_mode != common::BlendMode::Normal {
        return unsupported("group blend modes");
      }
      let alpha = opacity_alpha(*opacity)?;
      let pushed_clip = clip.is_some_and(|clip| {
        if clip.width_pt <= 0.0 || clip.height_pt <= 0.0 {
          return false;
        }
        content
          .save_state()
          .rect(clip.x_pt, clip.y_pt, clip.width_pt, clip.height_pt)
          .clip_nonzero()
          .end_path();
        true
      });
      if *flatten_identity && alpha == u8::MAX {
        for child in items {
          write_prepared_item(content, child, writer)?;
        }
      } else {
        // Group alpha belongs to the Do operation, not its children. The
        // transparency Form initializes their alpha to one (PDF 1.5 §7.5.5).
        if alpha != u8::MAX {
          content.save_state();
          set_alpha(content, writer.resources, writer.refs, None, Some(alpha))?;
        }
        write_isolated_prepared_group(content, items, writer)?;
        if alpha != u8::MAX {
          content.restore_state();
        }
      }
      if pushed_clip {
        content.restore_state();
      }
      Ok(())
    }
    super::paint::PaintItem::Image(image) => {
      write_prepared_image(content, image, writer)?;
      if let Some(url) = image.hyperlink_url.as_deref() {
        writer.page_links.resolve_and_push(
          LinkRect {
            x_pt: image.x_pt,
            y_pt: image.y_pt,
            width_pt: image.width_pt,
            height_pt: image.height_pt,
          },
          image.alt_text.as_deref(),
          url,
          writer.internal_links,
          writer.options,
          writer.refs,
        )?;
      }
      Ok(())
    }
    super::paint::PaintItem::LinkArea(link) => writer.page_links.resolve_and_push(
      LinkRect {
        x_pt: link.x_pt,
        y_pt: link.y_pt,
        width_pt: link.width_pt,
        height_pt: link.height_pt,
      },
      None,
      &link.hyperlink_url,
      writer.internal_links,
      writer.options,
      writer.refs,
    ),
  }
}

fn write_isolated_prepared_group(
  content: &mut Content,
  items: &[super::paint::PaintItem<'_>],
  writer: &mut PreparedPageWriter<'_>,
) -> Result<()> {
  let (name, id) = writer.forms.allocate(writer.refs)?;
  let mut resources = PageResources::new(
    writer.page_width_pt,
    writer.page_height_pt,
    path_gradient_profile(writer.options),
  );
  let mut group_content = Content::with_settings(Settings { pretty: false });
  group_content.save_state();
  {
    let mut group_writer = PreparedPageWriter {
      options: writer.options,
      resources: &mut resources,
      fonts: &mut *writer.fonts,
      image_policy: &mut *writer.image_policy,
      images: &mut *writer.images,
      patterns: &mut *writer.patterns,
      forms: &mut *writer.forms,
      refs: &mut *writer.refs,
      internal_links: writer.internal_links,
      page_links: &mut *writer.page_links,
      requires_codepoint_mappings: writer.requires_codepoint_mappings,
      forbids_private_use_mappings: writer.forbids_private_use_mappings,
      page_width_pt: writer.page_width_pt,
      page_height_pt: writer.page_height_pt,
    };
    for item in items {
      write_prepared_item(&mut group_content, item, &mut group_writer)?;
    }
  }
  group_content.restore_state();
  writer.forms.register(
    id,
    writer.page_width_pt,
    writer.page_height_pt,
    group_content.finish().into_vec(),
    resources,
    writer.options.compress_content_streams,
  );
  writer.resources.register_x_object(&name, id);
  content.x_object(Name(&name));
  Ok(())
}

fn write_prepared_image(
  content: &mut Content,
  image: &super::paint::ImageItem<'_>,
  writer: &mut PreparedPageWriter<'_>,
) -> Result<()> {
  ensure_image_supported(image)?;
  if image.data.is_empty() {
    return write_missing_linked_image(content, image, writer);
  }
  if is_office_math_svg(image) {
    return write_prepared_office_math_svg(content, image, writer);
  }
  if is_svg_image(image) {
    return write_prepared_svg(content, image, writer);
  }
  if super::metafile::is_metafile_image(image) {
    return write_prepared_metafile(content, image, writer);
  }

  write_prepared_raster_image(
    content,
    image,
    writer,
    None,
    image.width_pt,
    image.height_pt,
  )
}

fn write_missing_linked_image(
  content: &mut Content,
  image: &super::paint::ImageItem<'_>,
  writer: &mut PreparedPageWriter<'_>,
) -> Result<()> {
  const FRAME_WIDTH_PT: f32 = 0.14;
  const FRAME_INSET_PT: f32 = FRAME_WIDTH_PT / 2.0;
  const ICON_X_INSET_PT: f32 = 0.84;
  const ICON_Y_INSET_PT: f32 = 0.84;
  const ICON_WIDTH_PT: f32 = 1.68;
  const ICON_HEIGHT_PT: f32 = 1.92;

  if image.width_pt <= f32::EPSILON || image.height_pt <= f32::EPSILON {
    return Ok(());
  }

  let frame_width = (image.width_pt - FRAME_WIDTH_PT).max(0.0);
  let frame_height = (image.height_pt - FRAME_WIDTH_PT).max(0.0);
  if frame_width > f32::EPSILON && frame_height > f32::EPSILON {
    content
      .save_state()
      .set_stroke_rgb(0.0, 0.0, 0.0)
      .set_line_width(FRAME_WIDTH_PT)
      .set_line_cap(LineCapStyle::ButtCap)
      .set_line_join(LineJoinStyle::MiterJoin)
      .set_dash_pattern(std::iter::empty(), 0.0)
      .rect(
        image.x_pt + FRAME_INSET_PT,
        image.y_pt + FRAME_INSET_PT,
        frame_width,
        frame_height,
      )
      .stroke()
      .restore_state();
  }

  let icon = writer
    .images
    .register(missing_linked_image_icon(), || writer.refs.alloc())?;
  writer.resources.register_image(&icon.name, icon.id);
  content
    .save_state()
    .transform([
      ICON_WIDTH_PT,
      0.0,
      0.0,
      -ICON_HEIGHT_PT,
      image.x_pt + ICON_X_INSET_PT,
      image.y_pt + ICON_Y_INSET_PT + ICON_HEIGHT_PT,
    ])
    .x_object(Name(&icon.name))
    .restore_state();
  Ok(())
}

fn missing_linked_image_icon() -> PreparedRasterImage {
  static ICON: OnceLock<PreparedRasterImage> = OnceLock::new();
  ICON
    .get_or_init(|| {
      const PIXELS: [u8; 60] = [
        128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, // row 1
        128, 128, 128, 255, 255, 255, 255, 255, 255, 255, 255, 255, // row 2
        128, 128, 128, 255, 255, 255, 255, 0, 0, 255, 255, 255, // row 3
        128, 128, 128, 255, 204, 204, 255, 255, 255, 255, 255, 255, // row 4
        128, 128, 128, 255, 255, 255, 255, 255, 255, 255, 255, 255, // row 5
      ];
      PreparedRasterImage::new(DirectRasterImage {
        width: 4,
        height: 5,
        color_space: DirectRasterColorSpace::Rgb,
        bits_per_component: 8,
        encoding: DirectRasterEncoding::Sampled {
          pixels: Arc::new(PdfRasterPixels {
            width: 4,
            height: 5,
            rgb: PIXELS.to_vec(),
            alpha: None,
            icc_profile: None,
          }),
        },
        interpolate: false,
        soft_mask_interpolate: false,
        matte: None,
      })
    })
    .clone()
}

fn is_office_math_svg(image: &super::paint::ImageItem<'_>) -> bool {
  image.content_type.as_deref().is_some_and(|content_type| {
    content_type.eq_ignore_ascii_case("application/vnd.ooxmlsdk.office-math+xml")
  })
}

fn is_svg_image(image: &super::paint::ImageItem<'_>) -> bool {
  image
    .content_type
    .as_deref()
    .is_some_and(|content_type| content_type.eq_ignore_ascii_case("image/svg+xml"))
    || std::str::from_utf8(&image.data)
      .ok()
      .is_some_and(|text| text.trim_start().starts_with("<svg"))
}

fn write_prepared_svg(
  content: &mut Content,
  image: &super::paint::ImageItem<'_>,
  writer: &mut PreparedPageWriter<'_>,
) -> Result<()> {
  let scene = writer.image_policy.svg(&image.data)?;
  if !super::direct_svg::svg_scene_is_directly_writable(&scene) {
    let prepared = writer.image_policy.rasterized_svg(
      &image.data,
      &scene,
      writer.options,
      image.width_pt,
      image.height_pt,
    )?;
    return write_registered_raster_image(
      content,
      image,
      writer,
      prepared,
      image.width_pt,
      image.height_pt,
    );
  }

  let tree = scene.tree();
  let tree_width = tree.size().width();
  let tree_height = tree.size().height();
  if !tree_width.is_finite() || !tree_height.is_finite() || tree_width <= 0.0 || tree_height <= 0.0
  {
    return Err(PdfError::Image(
      "SVG image has an invalid intrinsic size".to_string(),
    ));
  }
  let resources = &mut *writer.resources;
  let refs = &mut *writer.refs;
  write_transformed_image_content(
    content,
    image,
    image.width_pt,
    image.height_pt,
    image.crop,
    |content, draw_width, draw_height, local_transform| {
      if local_transform != IDENTITY_TRANSFORM {
        content.transform(local_transform);
      }
      content
        .transform(scale(draw_width / tree_width, draw_height / tree_height))
        .rect(0.0, 0.0, tree_width, tree_height)
        .clip_nonzero()
        .end_path();
      super::direct_svg::write_svg_scene(
        content,
        &scene,
        &mut super::direct_svg::SvgWriteContext { resources, refs },
      )
    },
  )
}

fn write_prepared_office_math_svg(
  content: &mut Content,
  image: &super::paint::ImageItem<'_>,
  writer: &mut PreparedPageWriter<'_>,
) -> Result<()> {
  let scene = writer.image_policy.office_math_svg(&image.data)?;
  let tree = scene.tree();
  let tree_width = tree.size().width();
  let tree_height = tree.size().height();
  if !tree_width.is_finite() || !tree_height.is_finite() || tree_width <= 0.0 || tree_height <= 0.0
  {
    return Err(PdfError::Image(
      "OfficeMath SVG has an invalid intrinsic size".to_string(),
    ));
  }
  let resources = &mut *writer.resources;
  let fonts = &mut *writer.fonts;
  let refs = &mut *writer.refs;
  let requires_codepoint_mappings = writer.requires_codepoint_mappings;
  let forbids_private_use_mappings = writer.forbids_private_use_mappings;
  write_transformed_image_content(
    content,
    image,
    image.width_pt,
    image.height_pt,
    image.crop,
    |content, draw_width, draw_height, local_transform| {
      if local_transform != IDENTITY_TRANSFORM {
        content.transform(local_transform);
      }
      content
        .transform(scale(draw_width / tree_width, draw_height / tree_height))
        .rect(0.0, 0.0, tree_width, tree_height)
        .clip_nonzero()
        .end_path();
      super::direct_svg::write_office_math_scene(
        content,
        &scene,
        &mut super::direct_svg::OfficeMathWriteContext {
          resources,
          fonts,
          refs,
          requires_codepoint_mappings,
          forbids_private_use_mappings,
        },
      )
    },
  )
}

fn write_prepared_metafile(
  content: &mut Content,
  image: &super::paint::ImageItem<'_>,
  writer: &mut PreparedPageWriter<'_>,
) -> Result<()> {
  if let Some(color) = image.metafile_background_color {
    write_metafile_host_background(content, image, color)?;
  }

  let render_options = super::metafile::render_options_for_image(image, writer.options);
  if let Ok(Some(scene)) =
    ooxmlsdk_layout::render::emf_wmf::extract_metafile_vector_scene_with_options(
      &image.data,
      image.content_type.as_deref(),
      render_options,
    )
  {
    return write_metafile_vector_scene(content, image, &scene);
  }

  let (paint_width_pt, paint_height_pt) =
    super::metafile::native_paint_size(image).unwrap_or((image.width_pt, image.height_pt));
  write_prepared_raster_image(
    content,
    image,
    writer,
    Some(render_options),
    paint_width_pt,
    paint_height_pt,
  )
}

fn write_prepared_raster_image(
  content: &mut Content,
  image: &super::paint::ImageItem<'_>,
  writer: &mut PreparedPageWriter<'_>,
  metafile_render_options: Option<ooxmlsdk_layout::render::emf_wmf::RenderOptions>,
  paint_width_pt: f32,
  paint_height_pt: f32,
) -> Result<()> {
  if paint_width_pt <= f32::EPSILON || paint_height_pt <= f32::EPSILON {
    return Ok(());
  }

  let prepared = writer.image_policy.raster_direct(
    &image.data,
    image.content_type.as_deref(),
    writer.options,
    metafile_render_options,
    image.width_pt,
    image.height_pt,
  )?;
  write_registered_raster_image(
    content,
    image,
    writer,
    prepared,
    paint_width_pt,
    paint_height_pt,
  )
}

fn write_registered_raster_image(
  content: &mut Content,
  image: &super::paint::ImageItem<'_>,
  writer: &mut PreparedPageWriter<'_>,
  prepared: PreparedRasterImage,
  paint_width_pt: f32,
  paint_height_pt: f32,
) -> Result<()> {
  let registered = writer.images.register(prepared, || writer.refs.alloc())?;
  writer
    .resources
    .register_image(&registered.name, registered.id);

  write_transformed_image_content(
    content,
    image,
    paint_width_pt,
    paint_height_pt,
    image.crop,
    |content, draw_width, draw_height, local_transform| {
      content
        .transform(concat_transform(
          local_transform,
          [draw_width, 0.0, 0.0, -draw_height, 0.0, draw_height],
        ))
        .x_object(Name(&registered.name));
      Ok(())
    },
  )
}

fn write_metafile_host_background(
  content: &mut Content,
  image: &super::paint::ImageItem<'_>,
  color: [u8; 3],
) -> Result<()> {
  write_transformed_image_content(
    content,
    image,
    image.width_pt,
    image.height_pt,
    super::paint::ImageCrop::default(),
    |content, draw_width, draw_height, local_transform| {
      if local_transform != IDENTITY_TRANSFORM {
        content.transform(local_transform);
      }
      content
        .set_fill_rgb(
          f32::from(color[0]) / 255.0,
          f32::from(color[1]) / 255.0,
          f32::from(color[2]) / 255.0,
        )
        .rect(0.0, 0.0, draw_width, draw_height)
        .fill_nonzero();
      Ok(())
    },
  )
}

fn write_metafile_vector_scene(
  content: &mut Content,
  image: &super::paint::ImageItem<'_>,
  scene: &ooxmlsdk_layout::render::emf_wmf::MetafileVectorScene,
) -> Result<()> {
  for fill in &scene.fills {
    for subpath in &fill.subpaths {
      for point in subpath {
        if !point.x.is_finite() || !point.y.is_finite() {
          return Err(PdfError::Writer(
            "metafile vector scene contains a non-finite point".to_string(),
          ));
        }
      }
    }
  }

  let (paint_width_pt, paint_height_pt) =
    super::metafile::native_paint_size(image).unwrap_or((image.width_pt, image.height_pt));
  write_transformed_image_content(
    content,
    image,
    paint_width_pt,
    paint_height_pt,
    image.crop,
    |content, draw_width, draw_height, local_transform| {
      if local_transform != IDENTITY_TRANSFORM {
        content.transform(local_transform);
      }
      for fill in &scene.fills {
        let mut has_path = false;
        for subpath in &fill.subpaths {
          let Some(first) = subpath.first() else {
            continue;
          };
          content.move_to(first.x * draw_width, first.y * draw_height);
          for point in &subpath[1..] {
            content.line_to(point.x * draw_width, point.y * draw_height);
          }
          content.close_path();
          has_path = true;
        }
        if !has_path {
          continue;
        }
        content.set_fill_rgb(
          f32::from(fill.color[0]) / 255.0,
          f32::from(fill.color[1]) / 255.0,
          f32::from(fill.color[2]) / 255.0,
        );
        match fill.fill_rule {
          ooxmlsdk_layout::render::emf_wmf::MetafileVectorFillRule::Alternate => {
            content.fill_even_odd();
          }
          ooxmlsdk_layout::render::emf_wmf::MetafileVectorFillRule::Winding => {
            content.fill_nonzero();
          }
        }
      }
      Ok(())
    },
  )
}

fn write_transformed_image_content(
  content: &mut Content,
  image: &super::paint::ImageItem<'_>,
  width_pt: f32,
  height_pt: f32,
  crop: super::paint::ImageCrop,
  draw: impl FnOnce(&mut Content, f32, f32, PdfTransform) -> Result<()>,
) -> Result<()> {
  if !width_pt.is_finite() || !height_pt.is_finite() {
    return Err(PdfError::Writer(
      "image paint size contains a non-finite value".to_string(),
    ));
  }
  if width_pt <= f32::EPSILON || height_pt <= f32::EPSILON {
    return Ok(());
  }
  let visible_width = 1.0 - crop.left - crop.right;
  let visible_height = 1.0 - crop.top - crop.bottom;
  if visible_width <= f32::EPSILON || visible_height <= f32::EPSILON {
    return Ok(());
  }

  content.save_state();
  if !image.clip_path.is_empty() {
    append_path_commands(content, image.clip_path);
    content.clip_even_odd().end_path();
  }

  let mut frame = translate(image.x_pt, image.y_pt);
  if image.rotation_deg.abs() > f32::EPSILON {
    frame = concat_transform(
      frame,
      rotate_at(image.rotation_deg, width_pt / 2.0, height_pt / 2.0),
    );
  }
  content.transform(frame);

  if crop != super::paint::ImageCrop::default() {
    content
      .rect(0.0, 0.0, width_pt, height_pt)
      .clip_nonzero()
      .end_path();
  }

  let mut local_transform = IDENTITY_TRANSFORM;
  if image.flip_horizontal {
    local_transform = concat_transform(local_transform, translate(width_pt, 0.0));
    local_transform = concat_transform(local_transform, scale(-1.0, 1.0));
  }
  if image.flip_vertical {
    local_transform = concat_transform(local_transform, translate(0.0, height_pt));
    local_transform = concat_transform(local_transform, scale(1.0, -1.0));
  }
  let draw_width = width_pt / visible_width;
  let draw_height = height_pt / visible_height;
  local_transform = concat_transform(
    local_transform,
    translate(-crop.left * draw_width, -crop.top * draw_height),
  );
  let result = draw(content, draw_width, draw_height, local_transform);
  content.restore_state();
  result
}

fn ensure_image_supported(image: &super::paint::ImageItem<'_>) -> Result<()> {
  for value in [
    image.x_pt,
    image.y_pt,
    image.width_pt,
    image.height_pt,
    image.rotation_deg,
    image.crop.left,
    image.crop.top,
    image.crop.right,
    image.crop.bottom,
  ] {
    if !value.is_finite() {
      return Err(PdfError::Writer(
        "image geometry contains a non-finite value".to_string(),
      ));
    }
  }
  for command in image.clip_path {
    validate_path_command(*command)?;
  }
  if let Some(signature_line) = image.signature_line.as_ref() {
    match signature_line.state {
      common::SignatureLineState::Unsigned => {
        return unsupported("unsigned signature-line image semantics");
      }
      common::SignatureLineState::SignedValid | common::SignatureLineState::SignedInvalid
        if image.data.is_empty() =>
      {
        return unsupported("signed signature-line image without provider data");
      }
      common::SignatureLineState::SignedValid | common::SignatureLineState::SignedInvalid => {}
    }
  }
  if image.data.is_empty() {
    return Ok(());
  }
  Ok(())
}

type PdfTransform = [f32; 6];

const IDENTITY_TRANSFORM: PdfTransform = [1.0, 0.0, 0.0, 1.0, 0.0, 0.0];

fn translate(x: f32, y: f32) -> PdfTransform {
  [1.0, 0.0, 0.0, 1.0, x, y]
}

fn scale(x: f32, y: f32) -> PdfTransform {
  [x, 0.0, 0.0, y, 0.0, 0.0]
}

fn rotate_at(degrees: f32, x: f32, y: f32) -> PdfTransform {
  let radians = degrees.to_radians();
  let (sin, cos) = radians.sin_cos();
  concat_transform(
    concat_transform(translate(x, y), [cos, sin, -sin, cos, 0.0, 0.0]),
    translate(-x, -y),
  )
}

fn prepared_text_rotation_transform(
  degrees: f32,
  center_x: f32,
  center_y: f32,
) -> Result<Option<PdfTransform>> {
  if !degrees.is_finite() {
    return Err(PdfError::Writer(
      "text rotation angle must be finite".to_string(),
    ));
  }

  // Normalize before converting to radians so very large but finite authored
  // angles cannot overflow the trigonometric input. Keep an exact full turn
  // as the identity; its center is then intentionally inert.
  let normalized = f64::from(degrees).rem_euclid(360.0);
  let signed_degrees = if normalized > 180.0 {
    normalized - 360.0
  } else {
    normalized
  };
  if signed_degrees.abs() <= f64::from(f32::EPSILON) {
    return Ok(None);
  }
  if !center_x.is_finite() || !center_y.is_finite() {
    return Err(PdfError::Writer(
      "text rotation center must be finite".to_string(),
    ));
  }

  let (sin, cos) = signed_degrees.to_radians().sin_cos();
  let center_x = f64::from(center_x);
  let center_y = f64::from(center_y);
  let transform = [
    cos as f32,
    sin as f32,
    (-sin) as f32,
    cos as f32,
    (center_x - cos * center_x + sin * center_y) as f32,
    (center_y - sin * center_x - cos * center_y) as f32,
  ];
  if !transform.into_iter().all(f32::is_finite) {
    return Err(PdfError::Writer(
      "text rotation transform is not finite".to_string(),
    ));
  }
  Ok(Some(transform))
}

fn begin_prepared_text_rotation(content: &mut Content, transform: Option<PdfTransform>) -> bool {
  let Some(transform) = transform else {
    return false;
  };
  content.save_state().transform(transform);
  true
}

/// Matrix concatenation in the same row-vector order used by tiny-skia and
/// Krilla's `GraphicsState::concat_transform`.
fn concat_transform(left: PdfTransform, right: PdfTransform) -> PdfTransform {
  let [la, lb, lc, ld, le, lf] = left;
  let [ra, rb, rc, rd, re, rf] = right;
  [
    la * ra + lc * rb,
    lb * ra + ld * rb,
    la * rc + lc * rd,
    lb * rc + ld * rd,
    la * re + lc * rf + le,
    lb * re + ld * rf + lf,
  ]
}

fn append_path_commands(content: &mut Content, commands: &[common::PathCommand]) {
  for command in commands {
    match *command {
      common::PathCommand::MoveTo(point) => {
        content.move_to(point.x.0, point.y.0);
      }
      common::PathCommand::LineTo(point) => {
        content.line_to(point.x.0, point.y.0);
      }
      common::PathCommand::CubicTo {
        control1,
        control2,
        end,
      } => {
        content.cubic_to(
          control1.x.0,
          control1.y.0,
          control2.x.0,
          control2.y.0,
          end.x.0,
          end.y.0,
        );
      }
      common::PathCommand::Close => {
        content.close_path();
      }
    }
  }
}

fn write_prepared_text(
  content: &mut Content,
  text: &super::paint::PaintText<'_>,
  writer: &mut PreparedPageWriter<'_>,
) -> Result<()> {
  ensure_ordinary_text_supported(text)?;
  let portion_rotations = text
    .portions
    .iter()
    .map(|portion| {
      let (center_x, center_y) = text
        .item
        .rotation_center_pt
        .unwrap_or((portion.x_pt, portion.baseline_y));
      prepared_text_rotation_transform(text.item.style.rotation_deg, center_x, center_y)
    })
    .collect::<Result<Vec<_>>>()?;
  let horizontal_scale = text.item.style.horizontal_scale.unwrap_or(1.0);
  let outlined = direct_text_requires_glyph_outlines(&text.item.style);
  let paints_glyphs = super::paint::text_has_visible_glyph_paint(&text.item.style);
  if text.item.text.is_empty() {
    return Ok(());
  }

  let small_caps_semantic_text =
    super::paint::word_small_caps_semantic_text(&text.item.text, text.item.style.small_caps);
  let glyph_semantic_text = super::paint::symbol_font_semantic_text(
    small_caps_semantic_text.as_ref(),
    text.item.style.pdf_font_family(),
  );
  let glyph_semantic_text =
    super::paint::word_no_break_hyphen_semantic_text(glyph_semantic_text.as_ref());

  let color = text.item.style.color;
  if paints_glyphs {
    content.set_fill_rgb(
      f32::from(color.r) / f32::from(u8::MAX),
      f32::from(color.g) / f32::from(u8::MAX),
      f32::from(color.b) / f32::from(u8::MAX),
    );
  }
  for (portion, rotation) in text.portions.iter().zip(portion_rotations) {
    let paints_portion = (paints_glyphs
      && !matches!(portion.kind, super::paint::PaintTextPortionKind::Tab))
      || portion.highlight.is_some()
      || portion.underline.is_some()
      || portion.strikethrough.is_some();
    let semantic_clipped =
      paints_portion && begin_semantic_only_text_clip(content, text.item.style.semantic_only);
    let clipped = paints_portion && begin_prepared_text_clip(content, portion.clip.as_ref())?;
    // Paint clips are resolved in page space by layout ownership. Establish
    // them before concatenating the text-local rotation; clipping paths are
    // frozen in device space when `W` executes and must not rotate a second
    // time with the glyphs, highlights, or decorations.
    let rotated = paints_portion && begin_prepared_text_rotation(content, rotation);
    if paints_portion && let Some(highlight) = portion.highlight.as_ref() {
      write_prepared_text_highlight(content, highlight)?;
    }
    // A tab's measured advance is already reflected in the following
    // portion's page-space origin. Its shaped glyph is only a layout sentinel
    // and must not become a PDF glyph, a .notdef CID, or a ToUnicode entry.
    if paints_glyphs && !matches!(portion.kind, super::paint::PaintTextPortionKind::Tab) {
      let glyph_runs = portion
        .glyphs
        .as_ref()
        .ok_or(PdfError::DirectWriterUnsupported {
          feature: "text without resolved glyphs",
        })?;
      let variation_glyph_runs =
        super::paint::merge_variation_selector_font_runs(glyph_runs, &text.item.text);
      for run in variation_glyph_runs.as_ref() {
        let remapped_glyphs = super::paint::remap_glyph_text_ranges(
          &run.glyphs,
          &text.item.text,
          glyph_semantic_text.as_ref(),
        );
        let (run_semantic_text, run_glyphs) = match &remapped_glyphs {
          Some(glyphs) => (glyph_semantic_text.as_ref(), glyphs.as_ref()),
          None => (text.item.text.as_ref(), run.glyphs.as_slice()),
        };
        if outlined {
          write_prepared_glyph_outline_run(content, text, portion, run, writer)?;
          if text
            .item
            .style
            .pdf_glyph_outline_options
            .as_ref()
            .is_some_and(|options| options.semantic_text_overlay)
          {
            write_prepared_font_run_as_text(
              content,
              PreparedTextRun {
                portion,
                run,
                semantic_text: run_semantic_text,
                glyphs: run_glyphs,
                horizontal_scale,
                color: super::paint::RgbColor { r: 0, g: 0, b: 0 },
                kind: PreparedTextRunKind::TransparentSemanticOverlay,
              },
              writer,
            )?;
          }
        } else {
          write_prepared_font_run_as_text(
            content,
            PreparedTextRun {
              portion,
              run,
              semantic_text: run_semantic_text,
              glyphs: run_glyphs,
              horizontal_scale,
              color,
              kind: PreparedTextRunKind::Ordinary,
            },
            writer,
          )?;
        }
      }
    }
    if paints_portion {
      if let Some(underline) = portion.underline.as_ref() {
        write_prepared_text_decoration(content, underline)?;
      }
      if let Some(strikethrough) = portion.strikethrough.as_ref() {
        write_prepared_text_decoration(content, strikethrough)?;
      }
    }
    match (&portion.link, text.item.hyperlink_url.as_deref()) {
      (Some(link), Some(url)) => writer.page_links.resolve_and_push(
        LinkRect {
          x_pt: link.x_pt,
          y_pt: link.y_pt,
          width_pt: link.width_pt,
          height_pt: link.height_pt,
        },
        text.item.text.get(portion.text_range.clone()),
        url,
        writer.internal_links,
        writer.options,
        writer.refs,
      )?,
      (Some(_), None) => {
        return Err(PdfError::Writer(
          "prepared text link is missing its hyperlink target".to_string(),
        ));
      }
      (None, _) => {}
    }
    if rotated {
      content.restore_state();
    }
    if clipped {
      content.restore_state();
    }
    if semantic_clipped {
      content.restore_state();
    }
  }
  Ok(())
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum PreparedTextRunKind {
  Ordinary,
  TransparentSemanticOverlay,
}

struct PreparedTextRun<'a> {
  portion: &'a super::paint::PaintTextPortion,
  run: &'a super::paint::PaintGlyphFontRun,
  semantic_text: &'a str,
  glyphs: &'a [super::paint::PaintGlyph],
  horizontal_scale: f32,
  color: super::paint::RgbColor,
  kind: PreparedTextRunKind,
}

fn write_prepared_font_run_as_text(
  content: &mut Content,
  text_run: PreparedTextRun<'_>,
  writer: &mut PreparedPageWriter<'_>,
) -> Result<()> {
  let PreparedTextRun {
    portion,
    run,
    semantic_text,
    glyphs,
    horizontal_scale,
    color,
    kind,
  } = text_run;
  let handle = writer
    .fonts
    .register_face(&run.font_face, || writer.refs.alloc())?;
  let (resource_name, font_id) = writer.fonts.resource(handle)?;
  let resource_name = resource_name.to_vec();
  writer.resources.register_font(&resource_name, font_id);
  let synthesis = DirectTextSynthesis {
    // Explicit vector outlines already contain the authored visible stroke.
    // Their separate Office search layer is transparent and must not add an
    // artificial-bold stroke, while synthetic italic still determines glyph
    // placement and selection geometry.
    bold: kind == PreparedTextRunKind::Ordinary && run.font_face.synthetic_bold,
    italic: run.font_face.synthetic_italic,
    color,
    font_size_pt: run.font_size_pt,
    baseline_y: portion.baseline_y,
  };
  if kind == PreparedTextRunKind::TransparentSemanticOverlay {
    content.save_state();
    set_alpha(content, writer.resources, writer.refs, None, Some(0))?;
    // Word fixed output carries outlined DrawingML text in a separate,
    // transparent black text object. Its visible vector paint remains wholly
    // independent; preserving black here keeps extracted text-style state
    // stable across differently coloured outlines.
    set_prepared_fill_rgb(content, color);
  }
  synthesis.begin(content);
  let glyph_context = GlyphWriteContext {
    source: semantic_text,
    baseline_y: portion.baseline_y,
    font_size_pt: run.font_size_pt,
    horizontal_scale,
    resource_name: &resource_name,
    handle,
    rendering_mode: synthesis.rendering_mode(),
  };
  let mut segment_start = 0usize;
  let mut consumed_advance_em = 0.0f32;
  while segment_start < glyphs.len() {
    let segment = next_glyph_segment(glyphs, segment_start);
    let actual_text = segment
      .actual_text_range
      .as_ref()
      .map(|range| {
        semantic_text
          .get(range.clone())
          .ok_or_else(|| PdfError::Writer("ActualText cluster has an invalid range".to_string()))
      })
      .transpose()?;
    consumed_advance_em += write_positioned_glyph_segment(
      content,
      PositionedGlyphSegment {
        glyphs: &glyphs[segment_start..segment.end],
        actual_text,
        requires_codepoint_mappings: writer.requires_codepoint_mappings,
        forbids_private_use_mappings: writer.forbids_private_use_mappings,
        x_pt: scaled_text_x(
          portion.x_pt,
          run.x_offset_pt + consumed_advance_em * run.font_size_pt,
          horizontal_scale,
        )?,
      },
      &glyph_context,
      writer.fonts,
    )?;
    segment_start = segment.end;
  }
  synthesis.end(content);
  if kind == PreparedTextRunKind::TransparentSemanticOverlay {
    content.restore_state();
  }
  Ok(())
}

fn write_prepared_glyph_outline_run(
  content: &mut Content,
  text: &super::paint::PaintText<'_>,
  portion: &super::paint::PaintTextPortion,
  run: &super::paint::PaintGlyphFontRun,
  writer: &mut PreparedPageWriter<'_>,
) -> Result<()> {
  let options = text.item.style.pdf_glyph_outline_options.as_ref();
  let path = build_glyph_outline_path(
    run,
    GlyphOutlinePlacement {
      anchor_x_pt: portion.x_pt,
      run_x_offset_pt: run.x_offset_pt,
      baseline_y_pt: portion.baseline_y,
      horizontal_scale: text.item.style.horizontal_scale.unwrap_or(1.0),
      vertical_scale: text.item.style.direct_vertical_scale(),
    },
    options.and_then(|options| options.transform),
    options.and_then(|options| options.text_warp.as_deref()),
  )?;
  if path.is_empty() {
    return Ok(());
  }

  let fill = options
    .and_then(|options| options.fill.clone())
    .unwrap_or_else(|| fallback_glyph_fill(&text.item.style));
  let paint_context = GlyphOutlinePaintContext {
    text,
    portion,
    font_size_pt: run.font_size_pt,
    options,
  };
  write_glyph_outline_fill(content, paint_context, &path, &fill, writer)?;

  if let Some(stroke) = resolved_glyph_outline_stroke(
    &text.item.style,
    run.font_size_pt,
    run.font_face.synthetic_bold,
  )? {
    write_glyph_outline_stroke(content, paint_context, &path, &stroke, writer)?;
  }
  Ok(())
}

#[derive(Clone, Copy)]
struct GlyphOutlinePaintContext<'a, 'text> {
  text: &'a super::paint::PaintText<'text>,
  portion: &'a super::paint::PaintTextPortion,
  font_size_pt: f32,
  options: Option<&'a common::PdfGlyphOutlineOptions>,
}

fn resolved_glyph_outline_gradient(
  gradient: &common::GradientFill<'static>,
  bounds: common::Rect,
) -> common::GradientFill<'static> {
  let mut gradient = gradient.clone();
  let unresolved = gradient.definition_bounds.is_none();
  gradient.definition_bounds.get_or_insert(bounds);
  if let Some(path) = &mut gradient.path
    && unresolved
  {
    path.transform = common::bind_path_transform_to_bounds(path.transform, bounds);
    if path.kind == common::GradientPathKind::Circle {
      *path = common::office_circle_gradient_path(*path);
    }
  }
  gradient
}

fn write_glyph_outline_fill(
  content: &mut Content,
  context: GlyphOutlinePaintContext<'_, '_>,
  path: &super::direct_glyph::DirectOutlinePath,
  fill: &common::Fill<'static>,
  writer: &mut PreparedPageWriter<'_>,
) -> Result<()> {
  let GlyphOutlinePaintContext {
    text,
    portion,
    font_size_pt,
    options,
  } = context;
  match fill {
    common::Fill::None => Ok(()),
    common::Fill::Solid(color) => paint_solid_glyph_fill(content, path, *color, writer, true),
    common::Fill::Gradient(gradient) => {
      let bounds = outlined_glyph_paint_bounds(text, portion, font_size_pt, options)?;
      let gradient = resolved_glyph_outline_gradient(gradient, bounds);
      match writer.resources.gradients.register_pattern(
        &gradient,
        bounds,
        bounds,
        Some(path.commands()),
        writer.refs,
      )? {
        RegisteredGradientPattern::Solid(color) => {
          paint_solid_glyph_fill(content, path, color, writer, true)
        }
        RegisteredGradientPattern::Pattern { name, opacity } => {
          content.save_state();
          apply_gradient_opacity(content, writer.resources, writer.refs, opacity)?;
          content
            .set_fill_color_space(Name(b"Pattern"))
            .set_fill_pattern(std::iter::empty(), Name(&name));
          append_path_commands(content, path.commands());
          content.fill_even_odd().restore_state();
          Ok(())
        }
        RegisteredGradientPattern::Raster(image) => write_raster_gradient_pattern(
          content,
          image,
          bounds,
          GradientPatternWriter {
            resources: &mut *writer.resources,
            images: &mut *writer.images,
            refs: &mut *writer.refs,
          },
          path.commands(),
          true,
        ),
      }
    }
    common::Fill::Pattern(pattern) => {
      let bounds = outlined_glyph_paint_bounds(text, portion, font_size_pt, options)?;
      write_tiling_pattern_fill(
        content,
        *pattern,
        bounds.origin,
        TilingPatternWriter {
          page_height_pt: writer.page_height_pt,
          resources: &mut *writer.resources,
          patterns: &mut *writer.patterns,
          images: &mut *writer.images,
          refs: &mut *writer.refs,
        },
        |content| append_path_commands(content, path.commands()),
        true,
      )
    }
    common::Fill::Theme(_) => unsupported("unresolved theme outlined glyph fills"),
    common::Fill::Image { .. } => unsupported("image outlined glyph fills"),
  }
}

fn paint_solid_glyph_fill(
  content: &mut Content,
  path: &super::direct_glyph::DirectOutlinePath,
  color: common::Color,
  writer: &mut PreparedPageWriter<'_>,
  even_odd: bool,
) -> Result<()> {
  content.save_state();
  set_alpha(content, writer.resources, writer.refs, None, Some(color.a))?;
  set_fill_color(content, color);
  append_path_commands(content, path.commands());
  if even_odd {
    content.fill_even_odd();
  } else {
    content.fill_nonzero();
  }
  content.restore_state();
  Ok(())
}

fn write_glyph_outline_stroke(
  content: &mut Content,
  context: GlyphOutlinePaintContext<'_, '_>,
  path: &super::direct_glyph::DirectOutlinePath,
  stroke: &ResolvedGlyphOutlineStroke,
  writer: &mut PreparedPageWriter<'_>,
) -> Result<()> {
  let GlyphOutlinePaintContext {
    text,
    portion,
    font_size_pt,
    options,
  } = context;
  match &stroke.paint {
    common::Fill::None => Ok(()),
    common::Fill::Solid(color) => {
      if glyph_stroke_uses_filled_geometry(&stroke.style) {
        let expanded = path.expanded_stroke(&stroke.style)?;
        if expanded.is_empty() {
          return Ok(());
        }
        paint_solid_glyph_fill(content, &expanded, *color, writer, false)
      } else {
        let mut style = stroke.style.clone();
        style.color = *color;
        content.save_state();
        set_alpha(content, writer.resources, writer.refs, Some(color.a), None)?;
        set_stroke(content, &style);
        append_path_commands(content, path.commands());
        content.stroke().restore_state();
        Ok(())
      }
    }
    common::Fill::Gradient(gradient) => {
      let bounds = outlined_glyph_paint_bounds(text, portion, font_size_pt, options)?;
      let gradient = resolved_glyph_outline_gradient(gradient, bounds);
      let expanded = path.expanded_stroke(&stroke.style)?;
      if expanded.is_empty() {
        return Ok(());
      }
      match writer.resources.gradients.register_pattern(
        &gradient,
        bounds,
        bounds,
        Some(expanded.commands()),
        writer.refs,
      )? {
        RegisteredGradientPattern::Solid(color) => {
          if glyph_stroke_uses_filled_geometry(&stroke.style) {
            paint_solid_glyph_fill(content, &expanded, color, writer, false)
          } else {
            let mut style = stroke.style.clone();
            style.color = color;
            content.save_state();
            set_alpha(content, writer.resources, writer.refs, Some(color.a), None)?;
            set_stroke(content, &style);
            append_path_commands(content, path.commands());
            content.stroke().restore_state();
            Ok(())
          }
        }
        RegisteredGradientPattern::Pattern { name, opacity } => {
          content.save_state();
          apply_gradient_opacity(content, writer.resources, writer.refs, opacity)?;
          content
            .set_fill_color_space(Name(b"Pattern"))
            .set_fill_pattern(std::iter::empty(), Name(&name));
          append_path_commands(content, expanded.commands());
          content.fill_nonzero().restore_state();
          Ok(())
        }
        RegisteredGradientPattern::Raster(image) => write_raster_gradient_pattern(
          content,
          image,
          bounds,
          GradientPatternWriter {
            resources: &mut *writer.resources,
            images: &mut *writer.images,
            refs: &mut *writer.refs,
          },
          expanded.commands(),
          false,
        ),
      }
    }
    common::Fill::Pattern(_) => unsupported("pattern outlined glyph strokes"),
    common::Fill::Theme(_) => unsupported("unresolved theme outlined glyph strokes"),
    common::Fill::Image { .. } => unsupported("image outlined glyph strokes"),
  }
}

fn glyph_stroke_uses_filled_geometry(stroke: &common::Stroke<'static>) -> bool {
  !matches!(stroke.compound, None | Some(common::StrokeCompound::Single))
}

fn outlined_glyph_paint_bounds(
  text: &super::paint::PaintText<'_>,
  portion: &super::paint::PaintTextPortion,
  font_size_pt: f32,
  options: Option<&common::PdfGlyphOutlineOptions>,
) -> Result<common::Rect> {
  let definition_width =
    options.map_or(common::Pt(portion.width_pt.max(font_size_pt)), |options| {
      options.unresolved_definition_width(common::Pt(portion.width_pt), common::Pt(font_size_pt))
    });
  let bounds = options
    .and_then(|options| options.text_warp.as_deref())
    .map(|warp| warp.paint_bounds)
    .unwrap_or(common::Rect {
      origin: common::Point {
        x: common::Pt(portion.x_pt),
        y: common::Pt(text.item.y_pt),
      },
      size: common::Size {
        width: definition_width,
        height: common::Pt(text.item.line_height_pt.max(font_size_pt)),
      },
    });
  if ![
    bounds.origin.x.0,
    bounds.origin.y.0,
    bounds.size.width.0,
    bounds.size.height.0,
  ]
  .into_iter()
  .all(f32::is_finite)
    || bounds.size.width.0 <= f32::EPSILON
    || bounds.size.height.0 <= f32::EPSILON
  {
    return Err(PdfError::Writer(
      "outlined glyph paint bounds must be finite and positive".to_string(),
    ));
  }
  Ok(bounds)
}

fn fallback_glyph_fill(style: &super::paint::TextStyle<'_>) -> common::Fill<'static> {
  common::Fill::Solid(common::Color {
    r: style.color.r,
    g: style.color.g,
    b: style.color.b,
    a: normalized_opacity_u8(style.opacity),
  })
}

struct ResolvedGlyphOutlineStroke {
  style: common::Stroke<'static>,
  paint: common::Fill<'static>,
}

fn resolved_glyph_outline_stroke(
  style: &super::paint::TextStyle<'_>,
  font_size_pt: f32,
  synthetic_bold: bool,
) -> Result<Option<ResolvedGlyphOutlineStroke>> {
  let options = style.pdf_glyph_outline_options.as_ref();
  let mut stroke = options
    .and_then(|options| options.outline_stroke.clone())
    .or_else(|| {
      let color = style.outline_color?;
      (style.outline_width_pt > f32::EPSILON && style.outline_opacity > f32::EPSILON).then(|| {
        common::Stroke {
          width: common::Pt(style.outline_width_pt),
          color: common::Color {
            r: color.r,
            g: color.g,
            b: color.b,
            a: normalized_opacity_u8(style.outline_opacity),
          },
          ..Default::default()
        }
      })
    })
    .or_else(|| {
      synthetic_bold.then(|| common::Stroke {
        width: common::Pt(font_size_pt / 30.0),
        color: common::Color {
          r: style.color.r,
          g: style.color.g,
          b: style.color.b,
          a: normalized_opacity_u8(style.opacity),
        },
        ..Default::default()
      })
    });
  let Some(mut stroke) = stroke.take() else {
    return Ok(None);
  };
  if !stroke.width.0.is_finite() || stroke.width.0 <= f32::EPSILON {
    return Ok(None);
  }
  if !matches!(
    stroke.alignment,
    None | Some(common::StrokeAlignment::Center)
  ) {
    return Err(PdfError::DirectWriterUnsupported {
      feature: "inside-aligned outlined glyph strokes",
    });
  }
  let paint = options
    .and_then(|options| options.outline_fill.clone())
    .or_else(|| stroke.gradient.clone().map(common::Fill::Gradient))
    .or_else(|| stroke.pattern.map(common::Fill::Pattern))
    .unwrap_or(common::Fill::Solid(stroke.color));
  stroke.gradient = None;
  stroke.pattern = None;
  if matches!(paint, common::Fill::None)
    || matches!(paint, common::Fill::Solid(color) if color.a == 0)
  {
    return Ok(None);
  }
  Ok(Some(ResolvedGlyphOutlineStroke {
    style: stroke,
    paint,
  }))
}

fn normalized_opacity_u8(opacity: f32) -> u8 {
  (opacity.clamp(0.0, 1.0) * f32::from(u8::MAX)).round() as u8
}

fn begin_semantic_only_text_clip(content: &mut Content, semantic_only: bool) -> bool {
  if !semantic_only {
    return false;
  }

  // A semantic carrier must retain its real font, glyph positioning, colour,
  // ToUnicode map, and structure ownership without contributing page ink.
  // Keep that state observable and suppress only its effective painted shape,
  // matching the source-backed legacy lowering. The clip is isolated because
  // clipping paths can only be intersected, not enlarged, within one graphics
  // state.
  content
    .save_state()
    .rect(-10_000.0, -10_000.0, 0.001, 0.001)
    .clip_nonzero()
    .end_path();
  true
}

fn begin_prepared_text_clip(
  content: &mut Content,
  clip: Option<&super::paint::PaintClipRect>,
) -> Result<bool> {
  let Some(clip) = clip else {
    return Ok(false);
  };
  if ![clip.x_pt, clip.y_pt, clip.width_pt, clip.height_pt]
    .into_iter()
    .all(f32::is_finite)
  {
    return Err(PdfError::Writer(
      "text clip rectangle contains a non-finite coordinate".to_string(),
    ));
  }
  if clip.width_pt <= 0.0 || clip.height_pt <= 0.0 {
    return Ok(false);
  }
  content
    .save_state()
    .rect(clip.x_pt, clip.y_pt, clip.width_pt, clip.height_pt)
    .clip_nonzero()
    .end_path();
  Ok(true)
}

fn write_prepared_text_highlight(
  content: &mut Content,
  rect: &super::paint::PaintRect,
) -> Result<()> {
  if ![rect.x_pt, rect.y_pt, rect.width_pt, rect.height_pt]
    .into_iter()
    .all(f32::is_finite)
  {
    return Err(PdfError::Writer(
      "text highlight rectangle contains a non-finite coordinate".to_string(),
    ));
  }
  if rect.width_pt <= 0.0 || rect.height_pt <= 0.0 {
    return Ok(());
  }
  content.save_state();
  set_prepared_fill_rgb(content, rect.color);
  content
    .rect(rect.x_pt, rect.y_pt, rect.width_pt, rect.height_pt)
    .fill_nonzero();
  content.restore_state();
  Ok(())
}

fn write_prepared_text_decoration(
  content: &mut Content,
  line: &super::paint::PaintStrokeLine,
) -> Result<()> {
  if ![
    line.x1_pt,
    line.y1_pt,
    line.x2_pt,
    line.y2_pt,
    line.width_pt,
  ]
  .into_iter()
  .all(f32::is_finite)
    || line.width_pt <= 0.0
  {
    return Err(PdfError::Writer(
      "text decoration contains invalid line geometry".to_string(),
    ));
  }
  content.save_state();
  set_prepared_stroke_rgb(content, line.color);
  content
    .set_line_width(line.width_pt)
    .set_line_cap(LineCapStyle::ButtCap)
    .set_line_join(LineJoinStyle::MiterJoin)
    .set_dash_pattern(std::iter::empty(), 0.0)
    .move_to(line.x1_pt, line.y1_pt)
    .line_to(line.x2_pt, line.y2_pt)
    .stroke()
    .restore_state();
  Ok(())
}

#[derive(Clone, Copy, Debug)]
struct DirectTextSynthesis {
  bold: bool,
  italic: bool,
  color: super::paint::RgbColor,
  font_size_pt: f32,
  baseline_y: f32,
}

impl DirectTextSynthesis {
  fn begin(self, content: &mut Content) {
    if !self.is_active() {
      return;
    }
    content.save_state();
    if self.bold {
      set_prepared_stroke_rgb(content, self.color);
      content.set_line_width(self.font_size_pt / 30.0);
    }
    if self.italic {
      content.transform([
        1.0,
        0.0,
        -SYNTHETIC_ITALIC_SHEAR,
        1.0,
        self.baseline_y * SYNTHETIC_ITALIC_SHEAR,
        0.0,
      ]);
    }
  }

  fn end(self, content: &mut Content) {
    if self.is_active() {
      content.restore_state();
    }
  }

  fn is_active(self) -> bool {
    self.bold || self.italic
  }

  fn rendering_mode(self) -> TextRenderingMode {
    if self.bold {
      TextRenderingMode::FillStroke
    } else {
      TextRenderingMode::Fill
    }
  }
}

const SYNTHETIC_ITALIC_SHEAR: f32 = 1.0 / 3.0;

struct GlyphSegment {
  end: usize,
  actual_text_range: Option<std::ops::Range<usize>>,
}

fn next_glyph_segment(glyphs: &[super::paint::PaintGlyph], start: usize) -> GlyphSegment {
  debug_assert!(start < glyphs.len());
  if start + 1 < glyphs.len() && glyphs[start].text_range == glyphs[start + 1].text_range {
    let mut end = start + 2;
    while end < glyphs.len() && glyphs[end].text_range == glyphs[start].text_range {
      end += 1;
    }
    return GlyphSegment {
      end,
      actual_text_range: Some(glyphs[start].text_range.clone()),
    };
  }

  let mut end = start + 1;
  while end < glyphs.len() {
    if end + 1 < glyphs.len() && glyphs[end].text_range == glyphs[end + 1].text_range {
      break;
    }
    end += 1;
  }
  GlyphSegment {
    end,
    actual_text_range: None,
  }
}

fn begin_actual_text(content: &mut Content, text: &str) {
  let mut marked = content.begin_marked_content_with_properties(Name(b"Span"));
  let mut properties = marked.properties();
  properties.actual_text(TextStr(text));
  properties.finish();
  marked.finish();
}

struct GlyphWriteContext<'a> {
  source: &'a str,
  baseline_y: f32,
  font_size_pt: f32,
  horizontal_scale: f32,
  resource_name: &'a [u8],
  handle: super::direct_font::FontHandle,
  rendering_mode: TextRenderingMode,
}

struct PositionedGlyphSegment<'glyphs, 'text> {
  glyphs: &'glyphs [super::paint::PaintGlyph],
  actual_text: Option<&'text str>,
  requires_codepoint_mappings: bool,
  forbids_private_use_mappings: bool,
  x_pt: f32,
}

fn write_positioned_glyph_segment(
  content: &mut Content,
  segment: PositionedGlyphSegment<'_, '_>,
  context: &GlyphWriteContext<'_>,
  fonts: &mut DirectFontSet,
) -> Result<f32> {
  let PositionedGlyphSegment {
    glyphs,
    actual_text,
    requires_codepoint_mappings,
    forbids_private_use_mappings,
    x_pt,
  } = segment;
  if let Some(actual_text) = actual_text {
    let mut prepared = Vec::with_capacity(glyphs.len());
    for (index, glyph) in glyphs.iter().enumerate() {
      let semantic = glyph_semantic_text(context.source, glyph)?;
      ensure_glyph_semantic_mapping_supported(
        semantic,
        requires_codepoint_mappings,
        forbids_private_use_mappings,
      )?;
      let registered = fonts.register_glyph(
        context.handle,
        glyph.glyph_id,
        (index == 0 || requires_codepoint_mappings).then_some(semantic),
      )?;
      prepared.push(PreparedGlyph { glyph, registered });
    }
    begin_actual_text(content, actual_text);
    let advance = write_registered_glyph_segment(content, &prepared, x_pt, context);
    content.end_marked_content();
    return Ok(advance);
  }

  let mut compatible = Vec::new();
  let mut current_x_pt = x_pt;
  let mut total_advance_em = 0.0;
  for glyph in glyphs {
    let semantic = glyph_semantic_text(context.source, glyph)?;
    ensure_glyph_semantic_mapping_supported(
      semantic,
      requires_codepoint_mappings,
      forbids_private_use_mappings,
    )?;
    let registered = fonts.register_glyph(context.handle, glyph.glyph_id, Some(semantic))?;
    let prepared = PreparedGlyph { glyph, registered };
    if registered.semantic_conflict {
      let advance = write_registered_glyph_segment(content, &compatible, current_x_pt, context);
      compatible.clear();
      current_x_pt += advance * context.font_size_pt * context.horizontal_scale;
      total_advance_em += advance;

      begin_actual_text(content, semantic);
      let advance = write_registered_glyph_segment(
        content,
        std::slice::from_ref(&prepared),
        current_x_pt,
        context,
      );
      content.end_marked_content();
      current_x_pt += advance * context.font_size_pt * context.horizontal_scale;
      total_advance_em += advance;
    } else {
      compatible.push(prepared);
    }
  }
  total_advance_em += write_registered_glyph_segment(content, &compatible, current_x_pt, context);
  Ok(total_advance_em)
}

fn glyph_semantic_text<'a>(source: &'a str, glyph: &super::paint::PaintGlyph) -> Result<&'a str> {
  source
    .get(glyph.text_range.clone())
    .ok_or_else(|| PdfError::Writer("shaped glyph has an invalid text range".to_string()))
}

pub(super) fn ensure_glyph_semantic_mapping_supported(
  semantic: &str,
  requires_codepoint_mappings: bool,
  forbids_private_use_mappings: bool,
) -> Result<()> {
  if requires_codepoint_mappings
    && (semantic.is_empty()
      || semantic
        .chars()
        .any(|character| matches!(character as u32, 0x0000 | 0xfeff | 0xfffe)))
  {
    return unsupported("invalid glyph semantics in a Unicode-mapped PDF profile");
  }
  if forbids_private_use_mappings && semantic.chars().any(super::paint::is_unicode_private_use) {
    return unsupported("private-use glyph semantics in PDF/A-3a");
  }
  Ok(())
}

#[derive(Clone, Copy)]
struct PreparedGlyph<'a> {
  glyph: &'a super::paint::PaintGlyph,
  registered: RegisteredGlyph,
}

fn write_registered_glyph_segment(
  content: &mut Content,
  glyphs: &[PreparedGlyph<'_>],
  x_pt: f32,
  context: &GlyphWriteContext<'_>,
) -> f32 {
  if glyphs.is_empty() {
    return 0.0;
  }
  let mut positioned = Vec::new();
  let mut encoded = Vec::with_capacity(glyphs.len() * 2);
  let mut adjustment_em = 0.0f32;
  let mut total_advance_em = 0.0;
  for prepared in glyphs {
    let glyph = prepared.glyph;
    let registered = prepared.registered;
    adjustment_em += glyph.x_offset;
    if adjustment_em.abs() > 1.0e-7 {
      if !encoded.is_empty() {
        positioned.push(PositionedTextItem::Show(std::mem::take(&mut encoded)));
      }
      positioned.push(PositionedTextItem::Adjust(
        -adjustment_em * PDF_TEXT_UNITS_PER_EM,
      ));
      adjustment_em = 0.0;
    }
    encoded.push((registered.cid >> 8) as u8);
    encoded.push(registered.cid as u8);
    adjustment_em += glyph.x_advance - registered.natural_advance_pdf_units / PDF_TEXT_UNITS_PER_EM;
    adjustment_em -= glyph.x_offset;
    total_advance_em += glyph.x_advance;
  }
  if !encoded.is_empty() {
    positioned.push(PositionedTextItem::Show(encoded));
  }

  content
    .begin_text()
    .set_text_rendering_mode(context.rendering_mode)
    .set_font(Name(context.resource_name), context.font_size_pt)
    .set_text_matrix([
      context.horizontal_scale,
      0.0,
      0.0,
      -1.0,
      x_pt,
      context.baseline_y,
    ]);
  let mut show = content.show_positioned();
  let mut items = show.items();
  for item in &positioned {
    match item {
      PositionedTextItem::Show(bytes) => {
        items.show(Str(bytes));
      }
      PositionedTextItem::Adjust(amount) => {
        items.adjust(*amount);
      }
    }
  }
  items.finish();
  show.finish();
  content.end_text();
  total_advance_em
}

const PDF_TEXT_UNITS_PER_EM: f32 = 1000.0;

enum PositionedTextItem {
  Show(Vec<u8>),
  Adjust(f32),
}

fn direct_text_requires_glyph_outlines(style: &super::paint::TextStyle<'_>) -> bool {
  super::paint::text_requires_glyph_outlines(style)
}

fn ensure_ordinary_text_supported(text: &super::paint::PaintText<'_>) -> Result<()> {
  let style = &text.item.style;
  let outlined = direct_text_requires_glyph_outlines(style);
  let paints_glyphs = super::paint::text_has_visible_glyph_paint(style);
  if style.hidden {
    return unsupported("hidden text painting");
  }
  if !outlined
    && style.outline_color.is_some()
    && style.outline_width_pt > f32::EPSILON
    && style.outline_opacity > f32::EPSILON
  {
    return unsupported("text outline strokes");
  }
  let horizontal_scale = style.horizontal_scale.unwrap_or(1.0);
  if !horizontal_scale.is_finite() || horizontal_scale <= 0.0 {
    return Err(PdfError::Writer(
      "text horizontal scale must be finite and positive".to_string(),
    ));
  }
  if paints_glyphs && !outlined && (style.direct_vertical_scale() - 1.0).abs() > f32::EPSILON {
    return unsupported("vertically scaled text painting");
  }
  let source = text.item.text.as_ref();
  for portion in &text.portions {
    if !valid_text_range(source, &portion.text_range) {
      return Err(PdfError::Writer(format!(
        "text portion range {:?} is invalid for {} UTF-8 bytes",
        portion.text_range,
        source.len()
      )));
    }
    match portion.kind {
      super::paint::PaintTextPortionKind::Tab => {
        if source.get(portion.text_range.clone()) != Some("\t") {
          return Err(PdfError::Writer(
            "tab text portion must cover exactly one tab character".to_string(),
          ));
        }
        continue;
      }
      super::paint::PaintTextPortionKind::Text
      | super::paint::PaintTextPortionKind::Field
      | super::paint::PaintTextPortionKind::Link => {}
    }
    if !paints_glyphs {
      continue;
    }
    let Some(runs) = &portion.glyphs else {
      return unsupported("text without resolved glyphs");
    };
    for run in runs {
      if !run.font_size_pt.is_finite() || run.font_size_pt <= 0.0 || !run.x_offset_pt.is_finite() {
        return Err(PdfError::Writer(
          "text run has invalid font metrics".to_string(),
        ));
      }
      for glyph in &run.glyphs {
        if glyph.glyph_id == 0 {
          return unsupported("missing-glyph text painting");
        }
        if !glyph.x_advance.is_finite()
          || !glyph.x_offset.is_finite()
          || !glyph.y_offset.is_finite()
          || !glyph.y_advance.is_finite()
        {
          return Err(PdfError::Writer(
            "text glyph has non-finite metrics".to_string(),
          ));
        }
        if glyph.y_offset.abs() > f32::EPSILON || glyph.y_advance.abs() > f32::EPSILON {
          return unsupported("vertically positioned text glyphs");
        }
        if !valid_text_range(source, &glyph.text_range) || glyph.text_range.is_empty() {
          return Err(PdfError::Writer(format!(
            "glyph text range {:?} is invalid for {} UTF-8 bytes",
            glyph.text_range,
            source.len()
          )));
        }
      }
    }
  }
  Ok(())
}

fn scaled_text_x(anchor_x_pt: f32, unscaled_offset_pt: f32, horizontal_scale: f32) -> Result<f32> {
  let x_pt = horizontal_scale.mul_add(unscaled_offset_pt, anchor_x_pt);
  if !x_pt.is_finite() {
    return Err(PdfError::Writer(
      "scaled text origin is not finite".to_string(),
    ));
  }
  Ok(x_pt)
}

fn valid_text_range(text: &str, range: &std::ops::Range<usize>) -> bool {
  range.start <= range.end
    && range.end <= text.len()
    && text.is_char_boundary(range.start)
    && text.is_char_boundary(range.end)
}

fn write_prepared_rect(
  content: &mut Content,
  rect: &super::paint::RectItem<'_>,
  writer: &mut PreparedPageWriter<'_>,
) -> Result<()> {
  if let Some(fill) = rect.fill {
    match fill {
      super::paint::RectFill::Solid { color, opacity } => {
        content.save_state();
        set_alpha(
          content,
          writer.resources,
          writer.refs,
          None,
          Some(opacity_alpha(opacity)?),
        )?;
        set_prepared_fill_rgb(content, color);
        content
          .rect(rect.x_pt, rect.y_pt, rect.width_pt, rect.height_pt)
          .fill_even_odd();
        content.restore_state();
      }
      super::paint::RectFill::Gradient(gradient) => write_gradient_fill(
        content,
        gradient,
        common::Rect {
          origin: common::Point {
            x: common::Pt(rect.x_pt),
            y: common::Pt(rect.y_pt),
          },
          size: common::Size {
            width: common::Pt(rect.width_pt),
            height: common::Pt(rect.height_pt),
          },
        },
        None,
        GradientPatternWriter {
          resources: &mut *writer.resources,
          images: &mut *writer.images,
          refs: &mut *writer.refs,
        },
        |content| {
          content.rect(rect.x_pt, rect.y_pt, rect.width_pt, rect.height_pt);
        },
      )?,
      super::paint::RectFill::Pattern(pattern) => write_tiling_pattern_fill(
        content,
        *pattern,
        common::Point {
          x: common::Pt(rect.x_pt),
          y: common::Pt(rect.y_pt),
        },
        TilingPatternWriter {
          page_height_pt: writer.page_height_pt,
          resources: &mut *writer.resources,
          patterns: &mut *writer.patterns,
          images: &mut *writer.images,
          refs: &mut *writer.refs,
        },
        |content| {
          content.rect(rect.x_pt, rect.y_pt, rect.width_pt, rect.height_pt);
        },
        true,
      )?,
    }
  }
  if let Some(stroke) = rect.stroke {
    content.save_state();
    set_alpha(
      content,
      writer.resources,
      writer.refs,
      Some(opacity_alpha(rect.stroke_opacity)?),
      None,
    )?;
    set_prepared_stroke_rgb(content, stroke.color);
    content
      .set_line_width(stroke.width_pt)
      .set_line_cap(LineCapStyle::ButtCap)
      .set_line_join(LineJoinStyle::MiterJoin)
      .set_dash_pattern(std::iter::empty(), 0.0)
      .rect(rect.x_pt, rect.y_pt, rect.width_pt, rect.height_pt)
      .stroke();
    content.restore_state();
  }
  Ok(())
}

struct GradientPatternWriter<'a> {
  resources: &'a mut PageResources,
  images: &'a mut DirectImageSet,
  refs: &'a mut RefAllocator,
}

fn write_gradient_fill(
  content: &mut Content,
  gradient: &common::GradientFill<'static>,
  bounds: common::Rect,
  commands: Option<&[common::PathCommand]>,
  writer: GradientPatternWriter<'_>,
  append_clip_path: impl FnOnce(&mut Content),
) -> Result<()> {
  let GradientPatternWriter {
    resources,
    images,
    refs,
  } = writer;
  let registered = resources
    .gradients
    .register(gradient, bounds, commands, refs)?;
  content.save_state();
  match registered {
    RegisteredGradient::Solid(color) => {
      set_alpha(content, resources, refs, None, Some(color.a))?;
      set_fill_color(content, color);
      append_clip_path(content);
      content.fill_even_odd();
    }
    RegisteredGradient::Shading { name, opacity } => {
      apply_gradient_opacity(content, resources, refs, opacity)?;
      append_clip_path(content);
      content.clip_even_odd().end_path().shading(Name(&name));
    }
    RegisteredGradient::Pattern { name, opacity } => {
      apply_gradient_opacity(content, resources, refs, opacity)?;
      content
        .set_fill_color_space(Name(b"Pattern"))
        .set_fill_pattern(std::iter::empty(), Name(&name));
      append_clip_path(content);
      content.fill_even_odd();
    }
    RegisteredGradient::Raster(image) => {
      let registered = images.register(image, || refs.alloc())?;
      resources.register_image(&registered.name, registered.id);
      append_clip_path(content);
      content
        .clip_even_odd()
        .end_path()
        .transform([
          bounds.size.width.0,
          0.0,
          0.0,
          -bounds.size.height.0,
          bounds.origin.x.0,
          bounds.origin.y.0 + bounds.size.height.0,
        ])
        .x_object(Name(&registered.name));
    }
  }
  content.restore_state();
  Ok(())
}

fn write_gradient_stroke(
  content: &mut Content,
  gradient: &common::GradientFill<'static>,
  definition_bounds: common::Rect,
  commands: &[common::PathCommand],
  stroke: &common::Stroke<'static>,
  writer: GradientPatternWriter<'_>,
) -> Result<()> {
  let GradientPatternWriter {
    resources,
    images,
    refs,
  } = writer;
  if gradient.stops.iter().all(|stop| stop.color.a == 0) {
    return Ok(());
  }

  // Office fixed output does not use a stroking Pattern color space for a
  // DrawingML gradient outline. It widens the pen into closed band geometry
  // and fills that geometry with a non-stroking gradient pattern. Reuse the
  // same path-space expansion as outlined glyphs so caps, joins, miter limits,
  // and authored dash lengths remain geometry rather than PDF viewer policy.
  let expanded =
    super::direct_glyph::DirectOutlinePath::from_commands(commands).expanded_stroke(stroke)?;
  if expanded.is_empty() {
    return Ok(());
  }
  let raster_bounds = expanded.paint_bounds().ok_or_else(|| {
    PdfError::Writer("expanded gradient stroke has no finite paint bounds".to_string())
  })?;

  match resources.gradients.register_pattern(
    gradient,
    definition_bounds,
    raster_bounds,
    Some(expanded.commands()),
    refs,
  )? {
    RegisteredGradientPattern::Solid(color) => {
      content.save_state();
      set_alpha(content, resources, refs, None, Some(color.a))?;
      set_fill_color(content, color);
      append_path_commands(content, expanded.commands());
      content.fill_nonzero().restore_state();
      Ok(())
    }
    RegisteredGradientPattern::Pattern { name, opacity } => {
      content.save_state();
      apply_gradient_opacity(content, resources, refs, opacity)?;
      content
        .set_fill_color_space(Name(b"Pattern"))
        .set_fill_pattern(std::iter::empty(), Name(&name));
      append_path_commands(content, expanded.commands());
      content.fill_nonzero().restore_state();
      Ok(())
    }
    RegisteredGradientPattern::Raster(image) => write_raster_gradient_pattern(
      content,
      image,
      raster_bounds,
      GradientPatternWriter {
        resources,
        images,
        refs,
      },
      expanded.commands(),
      false,
    ),
  }
}

fn write_solid_compound_stroke(
  content: &mut Content,
  commands: &[common::PathCommand],
  stroke: &common::Stroke<'static>,
  resources: &mut PageResources,
  refs: &mut RefAllocator,
) -> Result<()> {
  if stroke.color.a == 0 {
    return Ok(());
  }
  let expanded =
    super::direct_glyph::DirectOutlinePath::from_commands(commands).expanded_stroke(stroke)?;
  if expanded.is_empty() {
    return Ok(());
  }

  content.save_state();
  set_alpha(content, resources, refs, None, Some(stroke.color.a))?;
  set_fill_color(content, stroke.color);
  append_path_commands(content, expanded.commands());
  content.fill_nonzero().restore_state();
  Ok(())
}

fn write_raster_gradient_pattern(
  content: &mut Content,
  image: super::image::PreparedRasterImage,
  bounds: common::Rect,
  writer: GradientPatternWriter<'_>,
  commands: &[common::PathCommand],
  even_odd: bool,
) -> Result<()> {
  let GradientPatternWriter {
    resources,
    images,
    refs,
  } = writer;
  let registered = images.register(image, || refs.alloc())?;
  resources.register_image(&registered.name, registered.id);
  content.save_state();
  append_path_commands(content, commands);
  if even_odd {
    content.clip_even_odd();
  } else {
    content.clip_nonzero();
  }
  content
    .end_path()
    .transform([
      bounds.size.width.0,
      0.0,
      0.0,
      -bounds.size.height.0,
      bounds.origin.x.0,
      bounds.origin.y.0 + bounds.size.height.0,
    ])
    .x_object(Name(&registered.name))
    .restore_state();
  Ok(())
}

struct TilingPatternWriter<'a> {
  page_height_pt: f32,
  resources: &'a mut PageResources,
  patterns: &'a mut DirectPatternSet,
  images: &'a mut DirectImageSet,
  refs: &'a mut RefAllocator,
}

fn write_tiling_pattern_fill(
  content: &mut Content,
  pattern: common::PatternFill,
  origin: common::Point,
  writer: TilingPatternWriter<'_>,
  append_path: impl FnOnce(&mut Content),
  even_odd: bool,
) -> Result<()> {
  let registered = writer.patterns.register(
    pattern,
    origin.x.0,
    origin.y.0,
    writer.page_height_pt,
    writer.images,
    writer.refs,
  )?;
  writer
    .resources
    .register_tiling_pattern(&registered.name, registered.id);
  content
    .save_state()
    .set_fill_color_space(Name(b"Pattern"))
    .set_fill_pattern(std::iter::empty(), Name(&registered.name));
  append_path(content);
  if even_odd {
    content.fill_even_odd();
  } else {
    content.fill_nonzero();
  }
  content.restore_state();
  Ok(())
}

fn apply_gradient_opacity(
  content: &mut Content,
  resources: &mut PageResources,
  refs: &mut RefAllocator,
  opacity: GradientOpacity,
) -> Result<()> {
  match opacity {
    GradientOpacity::Opaque => Ok(()),
    GradientOpacity::Constant(alpha) => set_alpha(content, resources, refs, None, Some(alpha)),
    GradientOpacity::SoftMask { name } => {
      content.set_parameters(Name(&name));
      Ok(())
    }
  }
}

fn write_prepared_line(
  content: &mut Content,
  line: &super::paint::LineItem,
  resources: &mut PageResources,
  refs: &mut RefAllocator,
) -> Result<()> {
  content.save_state();
  let alpha = opacity_alpha(line.opacity)?;
  let fill_alpha = (line.kind == super::paint::LineItemKind::FilledRect).then_some(alpha);
  set_alpha(content, resources, refs, Some(alpha), fill_alpha)?;
  set_prepared_stroke_rgb(content, line.color);
  content
    .set_line_width(line.width_pt)
    .set_line_cap(match line.line_cap {
      super::paint::PaintLineCap::Butt => LineCapStyle::ButtCap,
      super::paint::PaintLineCap::Round => LineCapStyle::RoundCap,
      super::paint::PaintLineCap::Square => LineCapStyle::ProjectingSquareCap,
    })
    .set_line_join(LineJoinStyle::MiterJoin);
  if let Some(dash) = &line.dash {
    content.set_dash_pattern(dash.iter().copied(), line.dash_offset);
  } else {
    content.set_dash_pattern(std::iter::empty(), 0.0);
  }
  match line.kind {
    super::paint::LineItemKind::Stroke => {
      content
        .move_to(line.x1_pt, line.y1_pt)
        .line_to(line.x2_pt, line.y2_pt)
        .stroke();
    }
    super::paint::LineItemKind::FilledRect => {
      set_prepared_fill_rgb(content, line.color);
      content
        .move_to(line.x1_pt, line.y2_pt)
        .line_to(line.x1_pt, line.y1_pt)
        .line_to(line.x2_pt, line.y1_pt)
        .line_to(line.x2_pt, line.y2_pt)
        .close_path()
        .fill_even_odd_and_stroke();
    }
  }
  content.restore_state();
  Ok(())
}

fn prepared_polyline_bounds(path: &super::paint::PolylineItem<'_>) -> common::Rect {
  common::Rect {
    origin: common::Point {
      x: common::Pt(path.x_pt),
      y: common::Pt(path.y_pt),
    },
    size: common::Size {
      width: common::Pt(path.width_pt),
      height: common::Pt(path.height_pt),
    },
  }
}

fn write_prepared_polyline_fill_only(
  content: &mut Content,
  path: &super::paint::PolylineItem<'_>,
  writer: &mut PreparedPageWriter<'_>,
) -> Result<()> {
  match path.fill {
    common::Fill::None => Ok(()),
    common::Fill::Solid(color) => {
      content.save_state();
      set_alpha(content, writer.resources, writer.refs, None, Some(color.a))?;
      set_fill_color(content, *color);
      append_prepared_polyline(content, path);
      content.fill_even_odd().restore_state();
      Ok(())
    }
    common::Fill::Gradient(gradient) => {
      let commands = path_commands_for_paint(path.commands, path.points, path.closed);
      write_gradient_fill(
        content,
        gradient,
        prepared_polyline_bounds(path),
        Some(commands.as_ref()),
        GradientPatternWriter {
          resources: &mut *writer.resources,
          images: &mut *writer.images,
          refs: &mut *writer.refs,
        },
        |content| append_prepared_polyline(content, path),
      )
    }
    common::Fill::Pattern(pattern) => write_tiling_pattern_fill(
      content,
      *pattern,
      common::Point {
        x: common::Pt(path.x_pt),
        y: common::Pt(path.y_pt),
      },
      TilingPatternWriter {
        page_height_pt: writer.page_height_pt,
        resources: &mut *writer.resources,
        patterns: &mut *writer.patterns,
        images: &mut *writer.images,
        refs: &mut *writer.refs,
      },
      |content| append_prepared_polyline(content, path),
      true,
    ),
    common::Fill::Theme(_) => unsupported("unresolved theme path fills"),
    common::Fill::Image { .. } => unsupported("image path fills"),
  }
}

fn write_prepared_polyline(
  content: &mut Content,
  path: &super::paint::PolylineItem<'_>,
  writer: &mut PreparedPageWriter<'_>,
) -> Result<()> {
  if path.commands.is_empty() && path.points.len() < 2 {
    return Ok(());
  }
  if let Some(stroke) = path.stroke
    && stroke_is_compound(stroke)
  {
    write_prepared_polyline_fill_only(content, path, writer)?;
    let commands = path_commands_for_paint(path.commands, path.points, path.closed);
    write_solid_compound_stroke(
      content,
      commands.as_ref(),
      stroke,
      writer.resources,
      writer.refs,
    )?;
    return Ok(());
  }
  if let Some(stroke) = path.stroke
    && let Some(gradient) = stroke.gradient.as_ref()
  {
    write_prepared_polyline_fill_only(content, path, writer)?;
    let commands = path_commands_for_paint(path.commands, path.points, path.closed);
    write_gradient_stroke(
      content,
      gradient,
      prepared_polyline_bounds(path),
      commands.as_ref(),
      stroke,
      GradientPatternWriter {
        resources: &mut *writer.resources,
        images: &mut *writer.images,
        refs: &mut *writer.refs,
      },
    )?;
    return Ok(());
  }
  if let common::Fill::Gradient(gradient) = path.fill {
    let gradient_commands = path_commands_for_paint(path.commands, path.points, path.closed);
    write_gradient_fill(
      content,
      gradient,
      prepared_polyline_bounds(path),
      Some(gradient_commands.as_ref()),
      GradientPatternWriter {
        resources: &mut *writer.resources,
        images: &mut *writer.images,
        refs: &mut *writer.refs,
      },
      |content| append_prepared_polyline(content, path),
    )?;
    if let Some(stroke) = path.stroke {
      content.save_state();
      set_alpha(
        content,
        writer.resources,
        writer.refs,
        Some(stroke.color.a),
        None,
      )?;
      set_stroke(content, stroke);
      append_prepared_polyline(content, path);
      content.stroke();
      content.restore_state();
      write_prepared_stroke_markers(content, path, stroke, writer.resources, writer.refs)?;
    }
    return Ok(());
  }
  if let common::Fill::Pattern(pattern) = path.fill {
    write_tiling_pattern_fill(
      content,
      *pattern,
      common::Point {
        x: common::Pt(path.x_pt),
        y: common::Pt(path.y_pt),
      },
      TilingPatternWriter {
        page_height_pt: writer.page_height_pt,
        resources: &mut *writer.resources,
        patterns: &mut *writer.patterns,
        images: &mut *writer.images,
        refs: &mut *writer.refs,
      },
      |content| append_prepared_polyline(content, path),
      true,
    )?;
    if let Some(stroke) = path.stroke {
      content.save_state();
      set_alpha(
        content,
        writer.resources,
        writer.refs,
        Some(stroke.color.a),
        None,
      )?;
      set_stroke(content, stroke);
      append_prepared_polyline(content, path);
      content.stroke();
      content.restore_state();
      write_prepared_stroke_markers(content, path, stroke, writer.resources, writer.refs)?;
    }
    return Ok(());
  }
  let fill = match path.fill {
    common::Fill::Solid(color) => Some(*color),
    common::Fill::None => None,
    _ => return unsupported("non-solid path fills"),
  };
  let stroke = path.stroke;
  match (fill, stroke) {
    (Some(fill), Some(stroke)) if path.separate_fill_and_stroke => {
      content.save_state();
      set_alpha(content, writer.resources, writer.refs, None, Some(fill.a))?;
      set_fill_color(content, fill);
      append_prepared_polyline(content, path);
      content.fill_even_odd();
      content.restore_state();

      content.save_state();
      set_alpha(
        content,
        writer.resources,
        writer.refs,
        Some(stroke.color.a),
        None,
      )?;
      set_stroke(content, stroke);
      append_prepared_polyline(content, path);
      content.stroke();
      content.restore_state();
    }
    (Some(fill), Some(stroke)) => {
      content.save_state();
      set_alpha(
        content,
        writer.resources,
        writer.refs,
        Some(stroke.color.a),
        Some(fill.a),
      )?;
      set_fill_color(content, fill);
      set_stroke(content, stroke);
      append_prepared_polyline(content, path);
      content.fill_even_odd_and_stroke();
      content.restore_state();
    }
    (Some(fill), None) => {
      content.save_state();
      set_alpha(content, writer.resources, writer.refs, None, Some(fill.a))?;
      set_fill_color(content, fill);
      append_prepared_polyline(content, path);
      content.fill_even_odd();
      content.restore_state();
    }
    (None, Some(stroke)) => {
      content.save_state();
      set_alpha(
        content,
        writer.resources,
        writer.refs,
        Some(stroke.color.a),
        None,
      )?;
      set_stroke(content, stroke);
      append_prepared_polyline(content, path);
      content.stroke();
      content.restore_state();
    }
    (None, None) => {}
  }
  if let Some(stroke) = stroke {
    write_prepared_stroke_markers(content, path, stroke, writer.resources, writer.refs)?;
  }
  Ok(())
}

fn append_prepared_polyline(content: &mut Content, path: &super::paint::PolylineItem<'_>) {
  if let Some((start, end)) = shortened_straight_polyline_points(path) {
    content.move_to(start.0, start.1).line_to(end.0, end.1);
    return;
  }
  if path.commands.is_empty() {
    let first = path.points[0];
    content.move_to(first.x.0, first.y.0);
    for point in &path.points[1..] {
      content.line_to(point.x.0, point.y.0);
    }
    if path.closed {
      content.close_path();
    }
    return;
  }
  for command in path.commands {
    match *command {
      common::PathCommand::MoveTo(point) => {
        content.move_to(point.x.0, point.y.0);
      }
      common::PathCommand::LineTo(point) => {
        content.line_to(point.x.0, point.y.0);
      }
      common::PathCommand::CubicTo {
        control1,
        control2,
        end,
      } => {
        content.cubic_to(
          control1.x.0,
          control1.y.0,
          control2.x.0,
          control2.y.0,
          end.x.0,
          end.y.0,
        );
      }
      common::PathCommand::Close => {
        content.close_path();
      }
    }
  }
}

fn path_commands_for_paint<'a>(
  commands: &'a [common::PathCommand],
  points: &'a [common::Point],
  closed: bool,
) -> Cow<'a, [common::PathCommand]> {
  if !commands.is_empty() {
    return Cow::Borrowed(commands);
  }

  let mut commands = Vec::with_capacity(points.len() + usize::from(closed));
  if let Some((first, rest)) = points.split_first() {
    commands.push(common::PathCommand::MoveTo(*first));
    commands.extend(rest.iter().copied().map(common::PathCommand::LineTo));
    if closed {
      commands.push(common::PathCommand::Close);
    }
  }
  Cow::Owned(commands)
}

fn write_prepared_stroke_markers(
  content: &mut Content,
  path: &super::paint::PolylineItem<'_>,
  stroke: &common::Stroke<'static>,
  resources: &mut PageResources,
  refs: &mut RefAllocator,
) -> Result<()> {
  for geometry in stroke_marker_geometries(path, stroke).into_iter().flatten() {
    match geometry {
      StrokeMarkerGeometry::Filled { points } => {
        let Some((&first, rest)) = points.split_first() else {
          continue;
        };
        if !points.iter().flat_map(|&(x, y)| [x, y]).all(f32::is_finite) {
          return Err(PdfError::Writer(
            "filled stroke marker has non-finite geometry".to_string(),
          ));
        }
        content.save_state();
        set_alpha(content, resources, refs, None, Some(stroke.color.a))?;
        set_fill_color(content, stroke.color);
        content.move_to(first.0, first.1);
        for point in rest {
          content.line_to(point.0, point.1);
        }
        content.close_path().fill_nonzero();
      }
      StrokeMarkerGeometry::StrokedOpen { points, width_pt } => {
        if !points
          .into_iter()
          .flat_map(|(x, y)| [x, y])
          .chain(std::iter::once(width_pt))
          .all(f32::is_finite)
          || width_pt < 0.0
        {
          return Err(PdfError::Writer(
            "open stroke marker has invalid geometry".to_string(),
          ));
        }
        content.save_state();
        set_alpha(content, resources, refs, Some(stroke.color.a), None)?;
        let [first, middle, last] = points;
        let [r, g, b] = normalized_rgb(stroke.color);
        content
          .set_stroke_rgb(r, g, b)
          .set_line_width(width_pt)
          .set_line_cap(LineCapStyle::RoundCap)
          .set_line_join(LineJoinStyle::MiterJoin)
          .set_dash_pattern(std::iter::empty(), 0.0)
          .move_to(first.0, first.1)
          .line_to(middle.0, middle.1)
          .line_to(last.0, last.1)
          .stroke();
      }
    }
    content.restore_state();
  }
  Ok(())
}

fn set_prepared_fill_rgb(content: &mut Content, color: super::paint::RgbColor) {
  content.set_fill_rgb(
    f32::from(color.r) / f32::from(u8::MAX),
    f32::from(color.g) / f32::from(u8::MAX),
    f32::from(color.b) / f32::from(u8::MAX),
  );
}

fn set_prepared_stroke_rgb(content: &mut Content, color: super::paint::RgbColor) {
  content.set_stroke_rgb(
    f32::from(color.r) / f32::from(u8::MAX),
    f32::from(color.g) / f32::from(u8::MAX),
    f32::from(color.b) / f32::from(u8::MAX),
  );
}

fn opacity_alpha(opacity: f32) -> Result<u8> {
  if !opacity.is_finite() || !(0.0..=1.0).contains(&opacity) {
    return Err(PdfError::Writer(format!("invalid paint opacity {opacity}")));
  }
  Ok((opacity * f32::from(u8::MAX)).round() as u8)
}

struct DisplayPageWriter<'a> {
  engine_kind: common::LayoutEngineKind,
  resources: &'a mut PageResources,
  patterns: &'a mut DirectPatternSet,
  images: &'a mut DirectImageSet,
  refs: &'a mut RefAllocator,
  page_height_pt: f32,
}

fn write_display_item(
  content: &mut Content,
  item: &common::DisplayItem<'static>,
  writer: &mut DisplayPageWriter<'_>,
) -> Result<()> {
  match item {
    common::DisplayItem::Rect(rect) => write_rect_item(content, rect, writer),
    common::DisplayItem::Line(line) => {
      write_line_item(content, line, writer.resources, writer.refs)
    }
    common::DisplayItem::Path(path) => write_path_item(content, path, writer),
    common::DisplayItem::Text(_) => unsupported("text painting"),
    common::DisplayItem::Glyphs(_) => unsupported("explicit glyph-run painting"),
    common::DisplayItem::Image(_) => unsupported("image painting"),
    common::DisplayItem::Group(group) => write_flattened_group(content, group, writer),
    common::DisplayItem::LinkArea(_) => unsupported("link annotations"),
    common::DisplayItem::AnnotationHint(_) => unsupported("annotation hints"),
    common::DisplayItem::Clip(_) => unsupported("display-list clipping operations"),
    common::DisplayItem::Transform(_) => unsupported("display-list transform operations"),
  }
}

fn write_flattened_group(
  content: &mut Content,
  group: &common::CompositingGroup<'static>,
  writer: &mut DisplayPageWriter<'_>,
) -> Result<()> {
  let pushed_clip = group.clip.is_some_and(|clip| {
    if clip.size.width.0 <= 0.0 || clip.size.height.0 <= 0.0 {
      return false;
    }
    content
      .save_state()
      .rect(
        clip.origin.x.0,
        clip.origin.y.0,
        clip.size.width.0,
        clip.size.height.0,
      )
      .clip_nonzero()
      .end_path();
    true
  });
  for item in &group.items {
    write_display_item(content, item, writer)?;
  }
  if pushed_clip {
    content.restore_state();
  }
  Ok(())
}

fn write_rect_item(
  content: &mut Content,
  rect: &common::RectItem<'static>,
  writer: &mut DisplayPageWriter<'_>,
) -> Result<()> {
  let bounds = rect.bounds;
  match &rect.fill {
    common::Fill::Solid(color) => {
      content.save_state();
      set_alpha(content, writer.resources, writer.refs, None, Some(color.a))?;
      set_fill_color(content, *color);
      content
        .rect(
          bounds.origin.x.0,
          bounds.origin.y.0,
          bounds.size.width.0,
          bounds.size.height.0,
        )
        .fill_even_odd();
      content.restore_state();
    }
    common::Fill::Gradient(gradient) => write_gradient_fill(
      content,
      gradient,
      bounds,
      None,
      GradientPatternWriter {
        resources: &mut *writer.resources,
        images: &mut *writer.images,
        refs: &mut *writer.refs,
      },
      |content| {
        content.rect(
          bounds.origin.x.0,
          bounds.origin.y.0,
          bounds.size.width.0,
          bounds.size.height.0,
        );
      },
    )?,
    common::Fill::Pattern(pattern) => write_tiling_pattern_fill(
      content,
      *pattern,
      bounds.origin,
      TilingPatternWriter {
        page_height_pt: writer.page_height_pt,
        resources: &mut *writer.resources,
        patterns: &mut *writer.patterns,
        images: &mut *writer.images,
        refs: &mut *writer.refs,
      },
      |content| {
        content.rect(
          bounds.origin.x.0,
          bounds.origin.y.0,
          bounds.size.width.0,
          bounds.size.height.0,
        );
      },
      true,
    )?,
    common::Fill::None => {}
    common::Fill::Theme(_) | common::Fill::Image { .. } => {
      return unsupported("non-solid rectangle fills");
    }
  }
  if let Some(stroke) = &rect.stroke {
    content.save_state();
    set_alpha(
      content,
      writer.resources,
      writer.refs,
      Some(stroke.color.a),
      None,
    )?;
    set_stroke(content, stroke);
    content
      .rect(
        bounds.origin.x.0,
        bounds.origin.y.0,
        bounds.size.width.0,
        bounds.size.height.0,
      )
      .stroke();
    content.restore_state();
  }
  Ok(())
}

fn write_line_item(
  content: &mut Content,
  line: &common::LineItem<'static>,
  resources: &mut PageResources,
  refs: &mut RefAllocator,
) -> Result<()> {
  content.save_state();
  let non_stroking_alpha =
    (line.kind == common::LineKind::FilledRect).then_some(line.stroke.color.a);
  set_alpha(
    content,
    resources,
    refs,
    Some(line.stroke.color.a),
    non_stroking_alpha,
  )?;
  set_stroke(content, &line.stroke);
  match line.kind {
    common::LineKind::Stroke => {
      content
        .move_to(line.start.x.0, line.start.y.0)
        .line_to(line.end.x.0, line.end.y.0)
        .stroke();
    }
    common::LineKind::FilledRect => {
      set_fill_color(content, line.stroke.color);
      content
        .move_to(line.start.x.0, line.end.y.0)
        .line_to(line.start.x.0, line.start.y.0)
        .line_to(line.end.x.0, line.start.y.0)
        .line_to(line.end.x.0, line.end.y.0)
        .close_path()
        .fill_even_odd_and_stroke();
    }
  }
  content.restore_state();
  Ok(())
}

fn write_path_fill_only(
  content: &mut Content,
  path: &common::PathItem<'static>,
  writer: &mut DisplayPageWriter<'_>,
) -> Result<()> {
  match &path.fill {
    common::Fill::None => Ok(()),
    common::Fill::Solid(color) => {
      content.save_state();
      set_alpha(content, writer.resources, writer.refs, None, Some(color.a))?;
      set_fill_color(content, *color);
      append_path(content, path);
      content.fill_even_odd().restore_state();
      Ok(())
    }
    common::Fill::Gradient(gradient) => {
      let commands = path_commands_for_paint(&path.commands, &path.points, path.closed);
      write_gradient_fill(
        content,
        gradient,
        path.bounds,
        Some(commands.as_ref()),
        GradientPatternWriter {
          resources: &mut *writer.resources,
          images: &mut *writer.images,
          refs: &mut *writer.refs,
        },
        |content| append_path(content, path),
      )
    }
    common::Fill::Pattern(pattern) => write_tiling_pattern_fill(
      content,
      *pattern,
      path.bounds.origin,
      TilingPatternWriter {
        page_height_pt: writer.page_height_pt,
        resources: &mut *writer.resources,
        patterns: &mut *writer.patterns,
        images: &mut *writer.images,
        refs: &mut *writer.refs,
      },
      |content| append_path(content, path),
      true,
    ),
    common::Fill::Theme(_) => unsupported("unresolved theme path fills"),
    common::Fill::Image { .. } => unsupported("image path fills"),
  }
}

fn write_path_item(
  content: &mut Content,
  path: &common::PathItem<'static>,
  writer: &mut DisplayPageWriter<'_>,
) -> Result<()> {
  if path.commands.is_empty() && path.points.len() < 2 {
    return Ok(());
  }
  if let Some(stroke) = &path.stroke
    && stroke_is_compound(stroke)
  {
    write_path_fill_only(content, path, writer)?;
    let commands = path_commands_for_paint(&path.commands, &path.points, path.closed);
    write_solid_compound_stroke(
      content,
      commands.as_ref(),
      stroke,
      writer.resources,
      writer.refs,
    )?;
    return Ok(());
  }
  if let Some(stroke) = &path.stroke
    && let Some(gradient) = stroke.gradient.as_ref()
  {
    write_path_fill_only(content, path, writer)?;
    let commands = path_commands_for_paint(&path.commands, &path.points, path.closed);
    write_gradient_stroke(
      content,
      gradient,
      path.bounds,
      commands.as_ref(),
      stroke,
      GradientPatternWriter {
        resources: &mut *writer.resources,
        images: &mut *writer.images,
        refs: &mut *writer.refs,
      },
    )?;
    return Ok(());
  }
  if let common::Fill::Gradient(gradient) = &path.fill {
    let gradient_commands = path_commands_for_paint(&path.commands, &path.points, path.closed);
    write_gradient_fill(
      content,
      gradient,
      path.bounds,
      Some(gradient_commands.as_ref()),
      GradientPatternWriter {
        resources: &mut *writer.resources,
        images: &mut *writer.images,
        refs: &mut *writer.refs,
      },
      |content| {
        append_path(content, path);
      },
    )?;
    if let Some(stroke) = &path.stroke {
      content.save_state();
      set_alpha(
        content,
        writer.resources,
        writer.refs,
        Some(stroke.color.a),
        None,
      )?;
      set_stroke(content, stroke);
      append_path(content, path);
      content.stroke();
      content.restore_state();
    }
    return Ok(());
  }
  if let common::Fill::Pattern(pattern) = &path.fill {
    write_tiling_pattern_fill(
      content,
      *pattern,
      path.bounds.origin,
      TilingPatternWriter {
        page_height_pt: writer.page_height_pt,
        resources: &mut *writer.resources,
        patterns: &mut *writer.patterns,
        images: &mut *writer.images,
        refs: &mut *writer.refs,
      },
      |content| append_path(content, path),
      true,
    )?;
    if let Some(stroke) = &path.stroke {
      content.save_state();
      set_alpha(
        content,
        writer.resources,
        writer.refs,
        Some(stroke.color.a),
        None,
      )?;
      set_stroke(content, stroke);
      append_path(content, path);
      content.stroke();
      content.restore_state();
    }
    return Ok(());
  }
  let fill = match &path.fill {
    common::Fill::Solid(color) => Some(*color),
    common::Fill::None => None,
    _ => return unsupported("non-solid path fills"),
  };
  let stroke = path.stroke.as_ref();

  match (fill, stroke) {
    (Some(fill), Some(stroke)) if writer.engine_kind == common::LayoutEngineKind::Pptx => {
      // The PowerPoint reference path submits fill and outline independently;
      // a PDF paint operator consumes its current path, so construct it once
      // for `f*` and once for `S`.
      content.save_state();
      set_alpha(content, writer.resources, writer.refs, None, Some(fill.a))?;
      set_fill_color(content, fill);
      append_path(content, path);
      content.fill_even_odd();
      content.restore_state();

      content.save_state();
      set_alpha(
        content,
        writer.resources,
        writer.refs,
        Some(stroke.color.a),
        None,
      )?;
      set_stroke(content, stroke);
      append_path(content, path);
      content.stroke();
      content.restore_state();
    }
    (Some(fill), Some(stroke)) => {
      content.save_state();
      set_alpha(
        content,
        writer.resources,
        writer.refs,
        Some(stroke.color.a),
        Some(fill.a),
      )?;
      set_fill_color(content, fill);
      set_stroke(content, stroke);
      append_path(content, path);
      content.fill_even_odd_and_stroke();
      content.restore_state();
    }
    (Some(fill), None) => {
      content.save_state();
      set_alpha(content, writer.resources, writer.refs, None, Some(fill.a))?;
      set_fill_color(content, fill);
      append_path(content, path);
      content.fill_even_odd();
      content.restore_state();
    }
    (None, Some(stroke)) => {
      content.save_state();
      set_alpha(
        content,
        writer.resources,
        writer.refs,
        Some(stroke.color.a),
        None,
      )?;
      set_stroke(content, stroke);
      append_path(content, path);
      content.stroke();
      content.restore_state();
    }
    (None, None) => {}
  }
  Ok(())
}

fn append_path(content: &mut Content, path: &common::PathItem<'static>) {
  if path.commands.is_empty() {
    let first = path.points[0];
    content.move_to(first.x.0, first.y.0);
    for point in &path.points[1..] {
      content.line_to(point.x.0, point.y.0);
    }
    if path.closed {
      content.close_path();
    }
    return;
  }

  for command in &path.commands {
    match *command {
      common::PathCommand::MoveTo(point) => {
        content.move_to(point.x.0, point.y.0);
      }
      common::PathCommand::LineTo(point) => {
        content.line_to(point.x.0, point.y.0);
      }
      common::PathCommand::CubicTo {
        control1,
        control2,
        end,
      } => {
        content.cubic_to(
          control1.x.0,
          control1.y.0,
          control2.x.0,
          control2.y.0,
          end.x.0,
          end.y.0,
        );
      }
      common::PathCommand::Close => {
        content.close_path();
      }
    }
  }
}

fn set_fill_color(content: &mut Content, color: common::Color) {
  let [r, g, b] = normalized_rgb(color);
  content.set_fill_rgb(r, g, b);
}

fn set_stroke(content: &mut Content, stroke: &common::Stroke<'static>) {
  let [r, g, b] = normalized_rgb(stroke.color);
  content
    .set_stroke_rgb(r, g, b)
    .set_line_width(stroke.width.0)
    .set_line_cap(match stroke.cap {
      Some(common::StrokeCap::Round) => LineCapStyle::RoundCap,
      Some(common::StrokeCap::Square) => LineCapStyle::ProjectingSquareCap,
      Some(common::StrokeCap::Flat) | None => LineCapStyle::ButtCap,
    })
    .set_line_join(match stroke.join {
      Some(common::StrokeJoin::Round) => LineJoinStyle::RoundJoin,
      Some(common::StrokeJoin::Bevel) => LineJoinStyle::BevelJoin,
      Some(common::StrokeJoin::Miter { .. }) | None => LineJoinStyle::MiterJoin,
    });
  if let Some(common::StrokeJoin::Miter { limit: Some(limit) }) = stroke.join {
    content.set_miter_limit(limit);
  }
  if let Some(dash) = stroke.resolved_dash() {
    content.set_dash_pattern(dash.into_iter().map(|value| value.0), stroke.dash_offset.0);
  } else {
    content.set_dash_pattern(std::iter::empty(), 0.0);
  }
}

fn normalized_rgb(color: common::Color) -> [f32; 3] {
  const CHANNEL_MAX: f32 = u8::MAX as f32;
  [
    f32::from(color.r) / CHANNEL_MAX,
    f32::from(color.g) / CHANNEL_MAX,
    f32::from(color.b) / CHANNEL_MAX,
  ]
}

pub(super) fn set_alpha(
  content: &mut Content,
  resources: &mut PageResources,
  refs: &mut RefAllocator,
  stroking: Option<u8>,
  non_stroking: Option<u8>,
) -> Result<()> {
  if let Some(name) = resources.register_alpha(refs, stroking, non_stroking)? {
    content.set_parameters(Name(name));
  }
  Ok(())
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct AlphaState {
  stroking: Option<u8>,
  non_stroking: Option<u8>,
}

impl AlphaState {
  fn new(stroking: Option<u8>, non_stroking: Option<u8>) -> Option<Self> {
    let stroking = stroking.filter(|alpha| *alpha != u8::MAX);
    let non_stroking = non_stroking.filter(|alpha| *alpha != u8::MAX);
    (stroking.is_some() || non_stroking.is_some()).then_some(Self {
      stroking,
      non_stroking,
    })
  }
}

#[derive(Debug)]
struct ExtGraphicsResource {
  name: Vec<u8>,
  id: Ref,
  state: AlphaState,
}

#[derive(Debug)]
struct FontResource {
  name: Vec<u8>,
  id: Ref,
}

#[derive(Debug)]
struct XObjectResource {
  name: Vec<u8>,
  id: Ref,
}

#[derive(Debug)]
struct TilingPatternResource {
  name: Vec<u8>,
  id: Ref,
}

#[derive(Debug)]
pub(super) struct PageResources {
  ext_graphics: Vec<ExtGraphicsResource>,
  fonts: Vec<FontResource>,
  gradients: DirectGradientSet,
  tiling_patterns: Vec<TilingPatternResource>,
  x_objects: Vec<XObjectResource>,
}

impl PageResources {
  fn new(
    page_width_pt: f32,
    page_height_pt: f32,
    path_gradient_profile: PathGradientProfile,
  ) -> Self {
    Self {
      ext_graphics: Vec::new(),
      fonts: Vec::new(),
      gradients: DirectGradientSet::new(page_width_pt, page_height_pt, path_gradient_profile),
      tiling_patterns: Vec::new(),
      x_objects: Vec::new(),
    }
  }

  pub(super) fn register_font(&mut self, name: &[u8], id: Ref) {
    if self.fonts.iter().any(|font| font.id == id) {
      return;
    }
    self.fonts.push(FontResource {
      name: name.to_vec(),
      id,
    });
  }

  fn register_image(&mut self, name: &[u8], id: Ref) {
    self.register_x_object(name, id);
  }

  fn register_x_object(&mut self, name: &[u8], id: Ref) {
    if self.x_objects.iter().any(|resource| resource.id == id) {
      return;
    }
    self.x_objects.push(XObjectResource {
      name: name.to_vec(),
      id,
    });
  }

  fn register_tiling_pattern(&mut self, name: &[u8], id: Ref) {
    if self
      .tiling_patterns
      .iter()
      .any(|resource| resource.id == id)
    {
      return;
    }
    self.tiling_patterns.push(TilingPatternResource {
      name: name.to_vec(),
      id,
    });
  }

  fn register_alpha<'a>(
    &'a mut self,
    refs: &mut RefAllocator,
    stroking: Option<u8>,
    non_stroking: Option<u8>,
  ) -> Result<Option<&'a [u8]>> {
    let Some(state) = AlphaState::new(stroking, non_stroking) else {
      return Ok(None);
    };
    let index = if let Some(index) = self
      .ext_graphics
      .iter()
      .position(|resource| resource.state == state)
    {
      index
    } else {
      let index = self.ext_graphics.len();
      self.ext_graphics.push(ExtGraphicsResource {
        name: format!("GS{index}").into_bytes(),
        id: refs.alloc()?,
        state,
      });
      index
    };
    Ok(Some(self.ext_graphics[index].name.as_slice()))
  }

  pub(super) fn write_objects(&self, pdf: &mut Pdf) {
    for resource in &self.ext_graphics {
      let mut state = pdf.ext_graphics(resource.id);
      if let Some(alpha) = resource.state.stroking {
        state.stroking_alpha(normalized_alpha(alpha));
      }
      if let Some(alpha) = resource.state.non_stroking {
        state.non_stroking_alpha(normalized_alpha(alpha));
      }
      state.finish();
    }
    self.gradients.write_objects(pdf);
  }

  pub(super) fn write_dictionary(&self, resources: &mut pdf_writer::writers::Resources<'_>) {
    if !self.x_objects.is_empty() {
      resources.x_objects().pairs(
        self
          .x_objects
          .iter()
          .map(|resource| (Name(resource.name.as_slice()), resource.id)),
      );
    }
    if !self.fonts.is_empty() {
      resources.fonts().pairs(
        self
          .fonts
          .iter()
          .map(|resource| (Name(resource.name.as_slice()), resource.id)),
      );
    }
    if !self.ext_graphics.is_empty() || self.gradients.has_soft_masks() {
      resources.ext_g_states().pairs(
        self
          .ext_graphics
          .iter()
          .map(|resource| (Name(resource.name.as_slice()), resource.id))
          .chain(self.gradients.soft_mask_dictionary_entries()),
      );
    }
    if self.gradients.has_shadings() {
      resources
        .shadings()
        .pairs(self.gradients.dictionary_entries());
    }
    if self.gradients.has_patterns() || !self.tiling_patterns.is_empty() {
      resources.patterns().pairs(
        self.gradients.pattern_dictionary_entries().chain(
          self
            .tiling_patterns
            .iter()
            .map(|resource| (Name(resource.name.as_slice()), resource.id)),
        ),
      );
    }
  }
}

fn path_gradient_profile(options: &PdfOptions) -> PathGradientProfile {
  let office_fixed_output = matches!(
    options.images.optimization_policy,
    PdfImageOptimizationPolicy::MicrosoftOfficeFixedOutput(_)
  );
  PathGradientProfile {
    office_fixed_output,
    // Office's Word-print and Excel-screen rectangle-path controls both use
    // the shared 96-DPI, twip-quantized inclusive-endpoint surface. The
    // generic profile retains the former bounded 144-DPI sampler ceiling.
    raster_sampling: if office_fixed_output {
      PathGradientRasterSampling::OfficeFixedOutputDpi(96)
    } else {
      PathGradientRasterSampling::BoundedPixelsPerPoint(2.0)
    },
  }
}

fn normalized_alpha(alpha: u8) -> f32 {
  f32::from(alpha) / f32::from(u8::MAX)
}

fn ordinary_pdf_version(options: &PdfOptions) -> Result<(u8, u8)> {
  let mut resolved = None;
  for standard in &options.standards {
    let version = match standard {
      PdfStandard::Pdf14 => Some((1, 4)),
      PdfStandard::Pdf15 => Some((1, 5)),
      PdfStandard::Pdf16 => Some((1, 6)),
      PdfStandard::Pdf17 => Some((1, 7)),
      PdfStandard::Pdf20 => Some((2, 0)),
      _ => None,
    };
    if let Some(version) = version {
      if resolved.is_some_and(|previous| previous != version) {
        return Err(PdfError::Writer(
          "resolved options contain more than one PDF version".to_string(),
        ));
      }
      resolved = Some(version);
    }
  }
  Ok(resolved.unwrap_or((1, 7)))
}

fn actual_text_version_floor(version: (u8, u8), requires_actual_text: bool) -> (u8, u8) {
  if requires_actual_text && version < (1, 5) {
    (1, 5)
  } else {
    version
  }
}

fn paint_document_requires_actual_text(paint: &super::paint::PaintDocument<'_>) -> bool {
  let mut mappings = HashMap::default();
  paint
    .pages
    .iter()
    .any(|page| paint_items_require_actual_text(&page.items, &mut mappings))
}

fn paint_items_require_actual_text(
  items: &[super::paint::PaintItem<'_>],
  mappings: &mut HashMap<ooxmlsdk_layout::fonts::FontFaceCacheKey, HashMap<u32, String>>,
) -> bool {
  items.iter().any(|item| match item {
    super::paint::PaintItem::Text(text) => {
      let small_caps_semantic_text =
        super::paint::word_small_caps_semantic_text(&text.item.text, text.item.style.small_caps);
      let glyph_semantic_text = super::paint::symbol_font_semantic_text(
        small_caps_semantic_text.as_ref(),
        text.item.style.pdf_font_family(),
      );
      let glyph_semantic_text =
        super::paint::word_no_break_hyphen_semantic_text(glyph_semantic_text.as_ref());
      for portion in &text.portions {
        if matches!(portion.kind, super::paint::PaintTextPortionKind::Tab) {
          continue;
        }
        let Some(glyph_runs) = &portion.glyphs else {
          continue;
        };
        let variation_glyph_runs =
          super::paint::merge_variation_selector_font_runs(glyph_runs, &text.item.text);
        for run in variation_glyph_runs.as_ref() {
          let remapped_glyphs = super::paint::remap_glyph_text_ranges(
            &run.glyphs,
            &text.item.text,
            glyph_semantic_text.as_ref(),
          );
          let (run_semantic_text, run_glyphs) = match &remapped_glyphs {
            Some(glyphs) => (glyph_semantic_text.as_ref(), glyphs.as_ref()),
            None => (text.item.text.as_ref(), run.glyphs.as_slice()),
          };
          if run_glyphs
            .windows(2)
            .any(|pair| pair[0].text_range == pair[1].text_range)
          {
            return true;
          }
          let font_mappings = mappings.entry(run.font_face.cache_key()).or_default();
          for glyph in run_glyphs {
            let Some(semantic) = run_semantic_text.get(glyph.text_range.clone()) else {
              continue;
            };
            match font_mappings.get(&glyph.glyph_id) {
              Some(existing) if existing != semantic => return true,
              Some(_) => {}
              None => {
                font_mappings.insert(glyph.glyph_id, semantic.to_string());
              }
            }
          }
        }
      }
      false
    }
    super::paint::PaintItem::Group { items, .. } => {
      paint_items_require_actual_text(items, mappings)
    }
    super::paint::PaintItem::Image(_)
    | super::paint::PaintItem::LinkArea(_)
    | super::paint::PaintItem::Rect(_)
    | super::paint::PaintItem::Line(_)
    | super::paint::PaintItem::Polyline(_) => false,
  })
}

fn validate_page_size(width_pt: f32, height_pt: f32) -> Result<()> {
  if !width_pt.is_finite() || !height_pt.is_finite() || width_pt < 3.0 || height_pt < 3.0 {
    return Err(PdfError::Writer(format!(
      "invalid page size {width_pt} x {height_pt} pt"
    )));
  }
  Ok(())
}

fn write_content_stream(pdf: &mut Pdf, id: Ref, content: &[u8], compress: bool) -> Result<()> {
  if compress {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder
      .write_all(content)
      .map_err(|error| PdfError::Writer(format!("content compression failed: {error}")))?;
    let compressed = encoder
      .finish()
      .map_err(|error| PdfError::Writer(format!("content compression failed: {error}")))?;
    pdf.stream(id, &compressed).filter(Filter::FlateDecode);
  } else {
    pdf.stream(id, content);
  }
  Ok(())
}

#[derive(Debug)]
pub(super) struct RefAllocator {
  next: i32,
}

impl Default for RefAllocator {
  fn default() -> Self {
    Self { next: 1 }
  }
}

impl RefAllocator {
  pub(super) fn alloc(&mut self) -> Result<Ref> {
    let id = self.next;
    self.next = self
      .next
      .checked_add(1)
      .ok_or_else(|| PdfError::Writer("indirect object ID space exhausted".to_string()))?;
    Ok(Ref::new(id))
  }
}

#[cfg(test)]
mod tests {
  use std::borrow::Cow;
  use std::sync::Arc;

  use super::*;
  use ooxmlsdk_layout::common::{DisplayPage, Pt, Size};

  fn blank_document(sizes: &[(f32, f32)]) -> common::LayoutDocument<'static> {
    common::LayoutDocument {
      pages: sizes
        .iter()
        .map(|&(width, height)| DisplayPage {
          setup: common::PageSetup {
            size: Size {
              width: Pt(width),
              height: Pt(height),
            },
            ..Default::default()
          },
          ..Default::default()
        })
        .collect(),
      ..Default::default()
    }
  }

  fn uncompressed_options() -> PdfOptions {
    PdfOptions {
      compress_content_streams: false,
      ..PdfOptions::default()
    }
  }

  fn color(r: u8, g: u8, b: u8, a: u8) -> common::Color {
    common::Color { r, g, b, a }
  }

  fn text_document(
    text: &str,
    style: common::TextStyle<'static>,
  ) -> common::LayoutDocument<'static> {
    let mut document = blank_document(&[(612.0, 792.0)]);
    document.pages[0]
      .items
      .push(common::DisplayItem::Text(common::TextRun {
        text: text.to_string().into(),
        origin: common::Point {
          x: Pt(72.0),
          y: Pt(72.0),
        },
        line_height: Pt(14.0),
        line_metrics_participant: true,
        paint_clip: None,
        style,
        font_id: None,
        color: color(0, 0, 0, u8::MAX),
        rotation_center: None,
        hyperlink_url: None,
        dynamic_field: None,
        form_widget_id: None,
        paragraph_bidi: false,
        word_spacing_pt: 0.0,
        preserve_text_portion: false,
        pdf_text_segmentation: common::PdfTextSegmentation::default(),
        source: None,
      }));
    document
  }

  fn opaque_test_png() -> Vec<u8> {
    let mut encoded = Vec::new();
    {
      let mut encoder = png::Encoder::new(&mut encoded, 2, 2);
      encoder.set_color(png::ColorType::Rgb);
      encoder.set_depth(png::BitDepth::Eight);
      let mut writer = encoder.write_header().unwrap();
      writer
        .write_image_data(&[255, 0, 0, 0, 255, 0, 0, 0, 255, 255, 255, 255])
        .unwrap();
    }
    encoded
  }

  fn test_image_item(bytes: Vec<u8>) -> common::ImageItem<'static> {
    common::ImageItem {
      bounds: common::Rect {
        origin: common::Point {
          x: Pt(10.0),
          y: Pt(20.0),
        },
        size: Size {
          width: Pt(30.0),
          height: Pt(40.0),
        },
      },
      crop: None,
      clip_path: Vec::new(),
      rotation_degrees: 0.0,
      flip_horizontal: false,
      flip_vertical: false,
      content_type: "image/png".into(),
      bytes: bytes.into(),
      metafile_monochrome_dib_palette_override: None,
      metafile_background_color: None,
      metafile_external_header: None,
      metafile_fixed_output_profile: common::MetafileFixedOutputProfile::Default,
      relationship_id: None,
      alt_text: None,
      hyperlink_url: None,
      semantic_metafile_text: false,
      metafile_semantic_text_includes_raster_backdrop: false,
      signature_line: None,
      metafile_native_size: false,
      floating: false,
      behind_text: false,
    }
  }

  fn image_document(image: common::ImageItem<'static>) -> common::LayoutDocument<'static> {
    let mut document = blank_document(&[(100.0, 100.0)]);
    document.pages[0]
      .items
      .push(common::DisplayItem::Image(image));
    document
  }

  fn paint_metafile_item() -> super::super::paint::ImageItem<'static> {
    super::super::paint::ImageItem {
      x_pt: 10.0,
      y_pt: 20.0,
      width_pt: 30.0,
      height_pt: 40.0,
      crop: super::super::paint::ImageCrop::default(),
      clip_path: &[],
      rotation_deg: 0.0,
      flip_horizontal: false,
      flip_vertical: false,
      data: Cow::Borrowed(&[]),
      content_type: Some(Cow::Borrowed("image/x-emf")),
      metafile_monochrome_dib_palette_override: None,
      metafile_background_color: None,
      metafile_external_header: None,
      metafile_fixed_output_profile: common::MetafileFixedOutputProfile::Default,
      alt_text: None,
      hyperlink_url: None,
      semantic_metafile_text: false,
      metafile_semantic_text_includes_raster_backdrop: false,
      signature_line: None,
      metafile_native_size: false,
    }
  }

  fn trailer_identifiers(pdf: &str) -> (uuid::Uuid, uuid::Uuid) {
    let trailer = &pdf[pdf.rfind("trailer").unwrap()..];
    let identifiers = &trailer[trailer.find("/ID").unwrap()..];
    let first_start = identifiers.find('<').unwrap() + 1;
    let first_end = identifiers[first_start..].find('>').unwrap() + first_start;
    let second_start = identifiers[first_end + 1..].find('<').unwrap() + first_end + 2;
    let second_end = identifiers[second_start..].find('>').unwrap() + second_start;
    (
      uuid::Uuid::parse_str(&identifiers[first_start..first_end]).unwrap(),
      uuid::Uuid::parse_str(&identifiers[second_start..second_end]).unwrap(),
    )
  }

  fn object_body(pdf: &str, id: i32) -> &str {
    let marker = format!("{id} 0 obj");
    let start = pdf.find(&marker).unwrap() + marker.len();
    let object = &pdf[start..];
    &object[..object.find("endobj").unwrap()]
  }

  fn content_matrices(pdf: &str) -> Vec<[f32; 6]> {
    let tokens = pdf.split_ascii_whitespace().collect::<Vec<_>>();
    tokens
      .windows(7)
      .filter_map(|window| {
        if window[6] != "cm" && !window[6].starts_with("cm/") {
          return None;
        }
        let mut matrix = [0.0; 6];
        for (index, token) in window[..6].iter().enumerate() {
          matrix[index] = token.parse().ok()?;
        }
        Some(matrix)
      })
      .collect()
  }

  fn assert_matrix_near(actual: [f32; 6], expected: [f32; 6]) {
    for (actual, expected) in actual.into_iter().zip(expected) {
      assert!(
        (actual - expected).abs() <= 1.0e-5,
        "matrix component {actual} differs from {expected}"
      );
    }
  }

  fn attachment(
    path: &str,
    association: crate::PdfAttachmentAssociation,
    data: &[u8],
    compress: Option<bool>,
  ) -> crate::PdfAttachment {
    crate::PdfAttachment {
      path: path.to_string(),
      mime_type: "text/plain".to_string(),
      description: format!("Description for {path}"),
      association,
      data: Arc::from(data),
      modification_date: None,
      compress,
    }
  }

  #[test]
  fn direct_writer_serializes_selected_page_tree_geometry_and_root_transform() {
    let document = blank_document(&[(612.0, 792.0), (720.0, 540.0)]);
    let mut options = uncompressed_options();
    options.general.page_range = Some("2,1".to_string());

    let bytes = render(&document, &options).unwrap();
    let pdf = String::from_utf8_lossy(&bytes);

    assert!(pdf.starts_with("%PDF-1.7"));
    assert!(pdf.contains("/Count 2"));
    let landscape = pdf.find("/MediaBox[0 0 720 540]").unwrap();
    let portrait = pdf.find("/MediaBox[0 0 612 792]").unwrap();
    assert!(landscape < portrait);
    assert!(pdf.contains("1 0 0 -1 0 540 cm"));
    assert!(pdf.contains("1 0 0 -1 0 792 cm"));
  }

  #[test]
  fn direct_writer_emits_viewer_catalog_preferences_and_selected_open_action() {
    let document = blank_document(&[(612.0, 792.0), (612.0, 792.0)]);
    let mut options = uncompressed_options();
    options.viewer.page_mode = PdfViewerPageMode::UseThumbs;
    options.viewer.page_layout = PdfPageLayout::ContinuousFacing;
    options.viewer.magnification = PdfViewerMagnification::FitVisible;
    options.viewer.initial_page = 2;
    options.viewer.hide_toolbar = true;
    options.viewer.hide_window_controls = true;
    options.viewer.fit_window = true;
    options.viewer.first_page_left = true;

    let bytes = render(&document, &options).unwrap();
    let pdf = String::from_utf8_lossy(&bytes);

    assert!(pdf.contains("/PageMode/UseThumbs"));
    assert!(pdf.contains("/PageLayout/TwoColumnRight"));
    assert!(pdf.contains("/HideToolbar true"));
    assert!(pdf.contains("/HideWindowUI true"));
    assert!(pdf.contains("/FitWindow true"));
    assert!(pdf.contains("/Direction/R2L"));
    assert!(pdf.contains("/OpenAction[5 0 R/FitBH null]"));
  }

  #[test]
  fn direct_writer_outlines_filter_selected_pages_and_close_the_object_graph() {
    let mut document = blank_document(&[(612.0, 792.0), (612.0, 792.0)]);
    document.outline_entries = vec![
      common::OutlineEntry {
        level: 0,
        text: "Selected parent".into(),
        page_index: 1,
        target: common::Point {
          x: Pt(87.0),
          y: Pt(72.0),
        },
        merged_hidden_separator: false,
      },
      common::OutlineEntry {
        level: 1,
        text: "Selected child".into(),
        page_index: 1,
        target: common::Point {
          x: Pt(90.0),
          y: Pt(119.0),
        },
        merged_hidden_separator: false,
      },
      common::OutlineEntry {
        level: 0,
        text: "Excluded page".into(),
        page_index: 0,
        target: common::Point::default(),
        merged_hidden_separator: false,
      },
    ];
    let mut options = uncompressed_options();
    options.general.export_bookmarks = true;
    options.general.open_bookmark_levels = Some(0);
    options.general.page_range = Some("2".to_string());

    let bytes = render(&document, &options).unwrap();
    let pdf = String::from_utf8_lossy(&bytes);
    assert!(pdf.contains("/Outlines"));
    assert!(pdf.contains("/Title(Selected parent)"));
    assert!(pdf.contains("/Title(Selected child)"));
    assert!(!pdf.contains("Excluded page"));
    assert!(pdf.contains("/XYZ 87 720 0"));
    assert!(pdf.contains("/XYZ 90 673 0"));
    assert!(pdf.contains("/Count 1"));

    options.general.page_range = Some("1".to_string());
    let excluded = String::from_utf8_lossy(&render(&document, &options).unwrap()).into_owned();
    assert!(excluded.contains("/Title(Excluded page)"));
    assert!(!excluded.contains("Selected parent"));
  }

  #[test]
  fn direct_writer_omits_outlines_when_export_is_disabled_or_selection_is_empty() {
    let mut document = blank_document(&[(612.0, 792.0), (612.0, 792.0)]);
    document.outline_entries.push(common::OutlineEntry {
      level: 0,
      text: "Only second page".into(),
      page_index: 1,
      target: common::Point::default(),
      merged_hidden_separator: false,
    });

    let mut disabled = uncompressed_options();
    disabled.general.export_bookmarks = false;
    let pdf = String::from_utf8_lossy(&render(&document, &disabled).unwrap()).into_owned();
    assert!(!pdf.contains("/Outlines"));

    let mut selected = uncompressed_options();
    selected.general.export_bookmarks = true;
    selected.general.page_range = Some("1".to_string());
    let pdf = String::from_utf8_lossy(&render(&document, &selected).unwrap()).into_owned();
    assert!(!pdf.contains("/Outlines"));
  }

  #[test]
  fn direct_writer_omits_default_viewer_entries_and_rejects_an_invalid_initial_page() {
    let document = blank_document(&[(612.0, 792.0)]);
    let default_pdf =
      String::from_utf8_lossy(&render(&document, &uncompressed_options()).unwrap()).into_owned();
    assert!(!default_pdf.contains("/ViewerPreferences"));
    assert!(!default_pdf.contains("/PageMode"));
    assert!(!default_pdf.contains("/PageLayout"));
    assert!(!default_pdf.contains("/OpenAction"));

    let mut invalid = uncompressed_options();
    invalid.viewer.initial_page = 2;
    assert!(matches!(
      render(&document, &invalid),
      Err(PdfError::Options(message)) if message.contains("page count 1")
    ));
  }

  #[test]
  fn direct_writer_emits_only_the_resolved_catalog_document_language() {
    let document = blank_document(&[(612.0, 792.0)]);

    let mut explicit = uncompressed_options();
    explicit.ui_language = Some("zh-CN".to_string());
    explicit.default_document_language = Some("ja_jp".to_string());
    let explicit_pdf = String::from_utf8_lossy(&render(&document, &explicit).unwrap()).into_owned();
    assert!(explicit_pdf.contains("/Lang(ja-JP)"));
    assert!(!explicit_pdf.contains("/Lang(zh-CN)"));

    let mut fallback = uncompressed_options();
    fallback.ui_language = Some("zh_hant_tw".to_string());
    let fallback_pdf = String::from_utf8_lossy(&render(&document, &fallback).unwrap()).into_owned();
    assert!(fallback_pdf.contains("/Lang(zh-Hant-TW)"));

    let absent_pdf =
      String::from_utf8_lossy(&render(&document, &uncompressed_options()).unwrap()).into_owned();
    assert!(!absent_pdf.contains("/Lang"));
  }

  #[test]
  fn direct_writer_always_emits_deterministic_xmp_and_matching_initial_file_ids() {
    let document = blank_document(&[(612.0, 792.0)]);
    let first = render(&document, &uncompressed_options()).unwrap();
    let second = render(&document, &uncompressed_options()).unwrap();
    assert_eq!(first, second);

    let pdf = String::from_utf8_lossy(&first);
    assert!(pdf.contains("/Metadata"));
    assert!(pdf.contains("/Type/Metadata/Subtype/XML"));
    assert!(pdf.contains("<dc:format>application/pdf</dc:format>"));
    assert!(pdf.contains("<xmpTPg:NPages>1</xmpTPg:NPages>"));
    assert!(pdf.contains("<pdf:PDFVersion>1.7</pdf:PDFVersion>"));
    assert!(!pdf.contains("/Info "));

    let (document_id, instance_id) = trailer_identifiers(&pdf);
    assert_eq!(document_id, instance_id);
    assert_eq!(document_id.get_version(), Some(uuid::Version::Sha1));
    let xmp_id = format!("uuid:{document_id}");
    assert!(pdf.contains(&format!("<xmpMM:DocumentID>{xmp_id}</xmpMM:DocumentID>")));
    assert!(pdf.contains(&format!("<xmpMM:InstanceID>{xmp_id}</xmpMM:InstanceID>")));

    let changed = render(&blank_document(&[(613.0, 792.0)]), &uncompressed_options()).unwrap();
    let changed = String::from_utf8_lossy(&changed);
    assert_ne!(trailer_identifiers(&changed).0, document_id);
  }

  #[test]
  fn direct_writer_synchronizes_unicode_info_xmp_keywords_dates_and_title_display() {
    let document = blank_document(&[(612.0, 792.0)]);
    let mut options = uncompressed_options();
    options.default_document_language = Some("ja_jp".to_string());
    options.viewer.display_document_title = true;
    options.metadata.title = Some("标题 & <proof>".to_string());
    options.metadata.author = Some("作者".to_string());
    options.metadata.subject = Some("主题".to_string());
    options.metadata.keywords = Some("alpha, beta; ; gamma".to_string());
    options.metadata.creator = Some("creator".to_string());
    options.metadata.producer = Some("producer".to_string());
    options.metadata.creation_date = Some(crate::PdfDateTime {
      year: 2026,
      month: Some(8),
      day: Some(17),
      hour: Some(12),
      minute: Some(30),
      second: Some(45),
      utc_offset_hour: Some(8),
      utc_offset_minute: Some(30),
    });

    let bytes = render(&document, &options).unwrap();
    let pdf = String::from_utf8_lossy(&bytes);
    assert!(pdf.contains("/Info 5 0 R"));
    let info = pdf
      .split("5 0 obj")
      .nth(1)
      .unwrap()
      .split("endobj")
      .next()
      .unwrap();
    assert!(info.contains("/Title<FEFF"));
    assert!(info.contains("/Author<FEFF4F5C8005>"));
    assert!(info.contains("/Subject<FEFF4E3B9898>"));
    assert!(info.contains("/Keywords(alpha, beta, gamma)"));
    assert!(info.contains("/Creator(creator)"));
    assert!(info.contains("/Producer(producer)"));
    assert!(info.contains("/CreationDate(D:20260817123045+08'30')"));
    assert!(info.contains("/ModDate(D:20260817123045+08'30')"));

    assert!(pdf.contains("标题 &amp; &lt;proof&gt;"));
    assert!(pdf.contains("<rdf:li>作者</rdf:li>"));
    assert!(pdf.contains("<pdf:Keywords>alpha, beta, gamma</pdf:Keywords>"));
    assert!(pdf.contains("<xmp:CreatorTool>creator</xmp:CreatorTool>"));
    assert!(pdf.contains("<pdf:Producer>producer</pdf:Producer>"));
    assert!(pdf.contains("<rdf:li>ja-JP</rdf:li>"));
    assert!(pdf.contains("<xmp:CreateDate>2026-08-17T12:30:45+08:30</xmp:CreateDate>"));
    assert!(pdf.contains("<xmp:ModifyDate>2026-08-17T12:30:45+08:30</xmp:ModifyDate>"));
    assert!(pdf.contains("/DisplayDocTitle true"));
  }

  #[test]
  fn direct_writer_omits_empty_info_values_and_pdf20_keeps_text_only_in_xmp() {
    let document = blank_document(&[(612.0, 792.0)]);
    let mut empty = uncompressed_options();
    empty.metadata.title = Some(String::new());
    empty.metadata.author = Some(String::new());
    empty.metadata.subject = Some(String::new());
    empty.metadata.keywords = Some(" , ; ".to_string());
    empty.metadata.creator = Some(String::new());
    empty.metadata.producer = Some(String::new());
    let empty_pdf = String::from_utf8_lossy(&render(&document, &empty).unwrap()).into_owned();
    assert!(!empty_pdf.contains("/Info "));
    assert!(!empty_pdf.contains("<dc:title>"));
    assert!(!empty_pdf.contains("<dc:creator>"));
    assert!(!empty_pdf.contains("<pdf:Keywords>"));

    let mut pdf20 = uncompressed_options();
    pdf20.standards = vec![PdfStandard::Pdf20];
    pdf20.metadata.title = Some("PDF 2 title".to_string());
    pdf20.metadata.author = Some("PDF 2 author".to_string());
    let text_only = String::from_utf8_lossy(&render(&document, &pdf20).unwrap()).into_owned();
    assert!(!text_only.contains("/Info "));
    assert!(text_only.contains("PDF 2 title"));
    assert!(text_only.contains("PDF 2 author"));

    pdf20.metadata.creation_date = Some(crate::PdfDateTime {
      year: 2026,
      month: None,
      day: None,
      hour: None,
      minute: None,
      second: None,
      utc_offset_hour: None,
      utc_offset_minute: None,
    });
    let dated = String::from_utf8_lossy(&render(&document, &pdf20).unwrap()).into_owned();
    assert!(dated.contains("/Info 5 0 R"));
    let info = dated
      .split("5 0 obj")
      .nth(1)
      .unwrap()
      .split("endobj")
      .next()
      .unwrap();
    assert!(!info.contains("/Title"));
    assert!(!info.contains("/Author"));
    assert!(info.contains("/CreationDate(D:20260101000000Z)"));
    assert!(dated.contains("<xmp:CreateDate>2026-01-01T00:00:00Z</xmp:CreateDate>"));
  }

  #[test]
  fn direct_writer_embeds_sorted_files_and_preserves_stream_parameters() {
    let document = blank_document(&[(612.0, 792.0)]);
    let mut options = uncompressed_options();
    let mut z_file = attachment(
      "z.txt",
      crate::PdfAttachmentAssociation::Supplement,
      b"uncompressed attachment",
      Some(false),
    );
    z_file.modification_date = Some(crate::PdfDateTime {
      year: 2026,
      month: Some(8),
      day: Some(25),
      hour: Some(9),
      minute: Some(30),
      second: Some(45),
      utc_offset_hour: Some(8),
      utc_offset_minute: Some(30),
    });
    options.attachments = vec![
      z_file,
      attachment(
        "a.txt",
        crate::PdfAttachmentAssociation::Source,
        &[b'a'; 128],
        Some(true),
      ),
    ];

    let first = render(&document, &options).unwrap();
    options.attachments.reverse();
    let second = render(&document, &options).unwrap();
    assert_eq!(first, second);

    let pdf = String::from_utf8_lossy(&first);
    assert!(pdf.contains("/Names<</EmbeddedFiles<</Names[(a.txt)5 0 R(z.txt)7 0 R]>>>>"));
    let a_stream = object_body(&pdf, 6);
    assert!(a_stream.contains("/Type/EmbeddedFile"));
    assert!(a_stream.contains("/Subtype/text#2Fplain"));
    assert!(a_stream.contains("/Filter/FlateDecode"));
    assert!(a_stream.contains("/Params<</Size 128>>"));
    assert!(!a_stream.contains("/CheckSum"));

    let z_stream = object_body(&pdf, 8);
    assert!(!z_stream.contains("/Filter"));
    assert!(z_stream.contains("/Size 23"));
    assert!(z_stream.contains("/ModDate(D:20260825093045+08'30)"));
    assert!(!z_stream.contains("+08'30')"));
    assert!(z_stream.contains("uncompressed attachment"));

    let a_spec = object_body(&pdf, 5);
    assert!(a_spec.contains("/Type/Filespec/F(a.txt)/UF(a.txt)"));
    assert!(a_spec.contains("/EF<</F 6 0 R/UF 6 0 R>>"));
    assert!(a_spec.contains("/Desc(Description for a.txt)"));
    assert!(!a_spec.contains("/AFRelationship"));
    assert!(!pdf.contains("/AF["));
  }

  #[test]
  fn direct_writer_attachment_unicode_and_association_keys_follow_pdf_version() {
    let document = blank_document(&[(612.0, 792.0)]);
    for (standard, has_unicode, has_association) in [
      (PdfStandard::Pdf14, false, false),
      (PdfStandard::Pdf16, false, false),
      (PdfStandard::Pdf17, true, false),
      (PdfStandard::Pdf20, true, true),
    ] {
      let mut options = uncompressed_options();
      options.standards = vec![standard];
      options.attachments.push(attachment(
        "数据.txt",
        crate::PdfAttachmentAssociation::Data,
        b"data",
        Some(false),
      ));

      let bytes = render(&document, &options).unwrap();
      let pdf = String::from_utf8_lossy(&bytes);
      let file_spec = object_body(&pdf, 5);
      assert_eq!(
        file_spec.contains("/UF<FEFF6570636E002E007400780074>"),
        has_unicode
      );
      assert_eq!(file_spec.contains("/EF<</F 6 0 R/UF 6 0 R>>"), has_unicode);
      assert_eq!(file_spec.contains("/AFRelationship/Data"), has_association);
      assert_eq!(pdf.contains("/AF[5 0 R]"), has_association);
      assert!(pdf.contains("/EmbeddedFiles"));
    }
  }

  #[test]
  fn direct_writer_maps_every_pdf20_attachment_relationship_and_merges_name_trees() {
    let mut document = blank_document(&[(612.0, 792.0)]);
    document.anchor_pages.push(common::AnchorPage {
      name: "destination".into(),
      page_index: 0,
      section_index: 0,
      section_page_index: 0,
      physical_page_number: 1,
      virtual_page_number: 1,
    });
    let mut options = uncompressed_options();
    options.standards = vec![PdfStandard::Pdf20];
    options.links.export_bookmarks_to_pdf_destinations = true;
    for (path, association) in [
      (
        "alternative.txt",
        crate::PdfAttachmentAssociation::Alternative,
      ),
      ("data.txt", crate::PdfAttachmentAssociation::Data),
      ("source.txt", crate::PdfAttachmentAssociation::Source),
      (
        "supplement.txt",
        crate::PdfAttachmentAssociation::Supplement,
      ),
      (
        "unspecified.txt",
        crate::PdfAttachmentAssociation::Unspecified,
      ),
    ] {
      options
        .attachments
        .push(attachment(path, association, b"data", Some(false)));
    }

    let bytes = render(&document, &options).unwrap();
    let pdf = String::from_utf8_lossy(&bytes);
    assert!(pdf.contains("/Names<</Dests<</Names[(destination)5 0 R]>>/EmbeddedFiles<<"));
    for relationship in ["Alternative", "Data", "Source", "Supplement", "Unspecified"] {
      assert!(pdf.contains(&format!("/AFRelationship/{relationship}")));
    }
    assert!(pdf.contains("/AF[6 0 R 8 0 R 10 0 R 12 0 R 14 0 R]"));
  }

  #[test]
  fn direct_writer_page_labels_follow_selected_output_order_and_compress_forward_runs() {
    let mut document = blank_document(&[(612.0, 792.0); 3]);
    for (index, page) in document.pages.iter_mut().enumerate() {
      page.section_page_index = index;
      page.setup.page_number_start = Some(7);
    }
    let selection = PageSelection::from_range(3, Some("3,1,2")).unwrap();

    let entries = page_label_entries(&document, &selection);
    assert_eq!(
      entries,
      vec![
        (
          0,
          Some(PageLabelSpec {
            start: NonZeroU32::new(9),
          }),
        ),
        (
          1,
          Some(PageLabelSpec {
            start: NonZeroU32::new(7),
          }),
        ),
      ]
    );

    let mut options = uncompressed_options();
    options.general.page_range = Some("3,1,2".to_string());
    let pdf = String::from_utf8_lossy(&render(&document, &options).unwrap()).into_owned();
    assert!(pdf.contains("/PageLabels<</Nums[0 9 0 R 1 10 0 R]>>"));
    let first_run = pdf
      .split("9 0 obj")
      .nth(1)
      .unwrap()
      .split("endobj")
      .next()
      .unwrap();
    let second_run = pdf
      .split("10 0 obj")
      .nth(1)
      .unwrap()
      .split("endobj")
      .next()
      .unwrap();
    assert!(first_run.contains("/Type/PageLabel/S/D/St 9"));
    assert!(second_run.contains("/Type/PageLabel/S/D/St 7"));
  }

  #[test]
  fn direct_writer_page_labels_reset_unlabelled_runs_and_reject_integer_overflow() {
    let mut document = blank_document(&[(612.0, 792.0); 3]);
    document.pages[1].setup.page_number_start = Some(7);
    let selection = PageSelection::from_range(3, None).unwrap();
    let entries = page_label_entries(&document, &selection);
    assert_eq!(entries.len(), 3);
    assert_eq!(entries[0].0, 0);
    assert_eq!(entries[0].1, None);
    assert_eq!(entries[1].1.unwrap().start, NonZeroU32::new(7));
    assert_eq!(entries[2].1, None);

    let pdf =
      String::from_utf8_lossy(&render(&document, &uncompressed_options()).unwrap()).into_owned();
    assert!(pdf.contains("/PageLabels"));
    assert_eq!(pdf.matches("/Type/PageLabel").count(), 3);

    let absent = blank_document(&[(612.0, 792.0)]);
    let absent_pdf =
      String::from_utf8_lossy(&render(&absent, &uncompressed_options()).unwrap()).into_owned();
    assert!(!absent_pdf.contains("/PageLabels"));

    let mut overflow = blank_document(&[(612.0, 792.0)]);
    overflow.pages[0].setup.page_number_start = Some(i32::MAX);
    overflow.pages[0].section_page_index = 1;
    assert!(matches!(
      render(&overflow, &uncompressed_options()),
      Err(PdfError::Writer(message)) if message.contains("page-label start")
    ));
  }

  #[test]
  fn direct_writer_named_destinations_sort_and_remap_selected_pages() {
    let mut document = blank_document(&[(200.0, 696.0), (240.0, 708.0), (280.0, 720.0)]);
    document.anchor_pages = vec![
      common::AnchorPage {
        name: "z-third".into(),
        page_index: 2,
        section_index: 0,
        section_page_index: 2,
        physical_page_number: 3,
        virtual_page_number: 3,
      },
      common::AnchorPage {
        name: "a-first".into(),
        page_index: 0,
        section_index: 0,
        section_page_index: 0,
        physical_page_number: 1,
        virtual_page_number: 1,
      },
      common::AnchorPage {
        name: "m-unselected".into(),
        page_index: 1,
        section_index: 0,
        section_page_index: 1,
        physical_page_number: 2,
        virtual_page_number: 2,
      },
    ];
    let mut options = uncompressed_options();
    options.general.page_range = Some("3,1".to_string());
    options.links.export_bookmarks_to_pdf_destinations = true;

    let pdf = String::from_utf8_lossy(&render(&document, &options).unwrap()).into_owned();
    assert!(pdf.contains("/Names<</Dests<</Names[(a-first)7 0 R(z-third)8 0 R]>>>>"));
    let first = pdf
      .split("7 0 obj")
      .nth(1)
      .unwrap()
      .split("endobj")
      .next()
      .unwrap();
    let third = pdf
      .split("8 0 obj")
      .nth(1)
      .unwrap()
      .split("endobj")
      .next()
      .unwrap();
    assert!(first.contains("[5 0 R/XYZ 0 696 0]"));
    assert!(third.contains("[3 0 R/XYZ 0 720 0]"));
    assert!(!pdf.contains("m-unselected"));
  }

  #[test]
  fn direct_writer_named_destinations_are_opt_in_and_reject_selected_duplicates() {
    let mut document = blank_document(&[(612.0, 792.0), (612.0, 792.0)]);
    document.anchor_pages = vec![
      common::AnchorPage {
        name: "same".into(),
        page_index: 0,
        section_index: 0,
        section_page_index: 0,
        physical_page_number: 1,
        virtual_page_number: 1,
      },
      common::AnchorPage {
        name: "same".into(),
        page_index: 1,
        section_index: 0,
        section_page_index: 1,
        physical_page_number: 2,
        virtual_page_number: 2,
      },
    ];

    let disabled =
      String::from_utf8_lossy(&render(&document, &uncompressed_options()).unwrap()).into_owned();
    assert!(!disabled.contains("/Names"));

    let mut duplicate = uncompressed_options();
    duplicate.links.export_bookmarks_to_pdf_destinations = true;
    assert!(matches!(
      render(&document, &duplicate),
      Err(PdfError::Options(message)) if message.contains("same")
    ));

    duplicate.general.page_range = Some("1".to_string());
    let selected = String::from_utf8_lossy(&render(&document, &duplicate).unwrap()).into_owned();
    assert!(selected.contains("/Names<</Dests<</Names[(same)5 0 R]>>>>"));
  }

  #[test]
  fn direct_writer_uses_flate_only_when_requested() {
    let document = blank_document(&[(612.0, 792.0)]);
    let compressed = render(&document, &PdfOptions::default()).unwrap();
    let plain = render(&document, &uncompressed_options()).unwrap();

    assert!(
      compressed
        .windows(b"/Filter/FlateDecode".len())
        .any(|window| { window == b"/Filter/FlateDecode" })
    );
    assert!(
      !plain
        .windows(b"/Filter/FlateDecode".len())
        .any(|window| { window == b"/Filter/FlateDecode" })
    );
  }

  #[test]
  fn direct_writer_diagnostics_follow_selected_output_page_order() {
    let style = common::TextStyle {
      font_family: Some("Liberation Serif".into()),
      font_size: Pt(12.0),
      color: color(0, 0, 0, u8::MAX),
      ..Default::default()
    };
    let mut first = text_document("first", style.clone());
    let mut second = text_document("second", style);
    let mut document = blank_document(&[(612.0, 792.0), (720.0, 540.0)]);
    document.pages[0].items = std::mem::take(&mut first.pages[0].items);
    document.pages[1].items = std::mem::take(&mut second.pages[0].items);
    let mut options = uncompressed_options();
    options.general.page_range = Some("2,1".to_string());

    let output = render_with_diagnostics(&document, &options).unwrap();

    assert_eq!(output.diagnostics.pages.len(), 2);
    assert_eq!(output.diagnostics.pages[0].page_index, 0);
    assert_eq!(output.diagnostics.pages[0].width_pt, 720.0);
    assert_eq!(output.diagnostics.pages[0].height_pt, 540.0);
    assert_eq!(output.diagnostics.pages[0].text_runs[0].text, "second");
    assert_eq!(output.diagnostics.pages[1].page_index, 1);
    assert_eq!(output.diagnostics.pages[1].width_pt, 612.0);
    assert_eq!(output.diagnostics.pages[1].height_pt, 792.0);
    assert_eq!(output.diagnostics.pages[1].text_runs[0].text, "first");
  }

  #[test]
  fn direct_writer_emits_solid_rect_fill_and_outline_as_independent_paths() {
    let mut document = blank_document(&[(612.0, 792.0)]);
    document.pages[0]
      .items
      .push(common::DisplayItem::Rect(common::RectItem {
        bounds: common::Rect {
          origin: common::Point {
            x: Pt(10.0),
            y: Pt(20.0),
          },
          size: Size {
            width: Pt(30.0),
            height: Pt(40.0),
          },
        },
        fill: common::Fill::Solid(color(255, 0, 0, 255)),
        stroke: Some(common::Stroke {
          width: Pt(2.0),
          color: color(0, 0, 255, 255),
          ..Default::default()
        }),
      }));

    let bytes = render(&document, &uncompressed_options()).unwrap();
    let pdf = String::from_utf8_lossy(&bytes);

    let fill = pdf.find("1 0 0 rg").unwrap();
    let fill_path = pdf[fill..].find("10 20 30 40 re").unwrap() + fill;
    let fill_operator = pdf[fill_path..].find("f*").unwrap() + fill_path;
    let stroke = pdf[fill_operator..].find("0 0 1 RG").unwrap() + fill_operator;
    let stroke_path = pdf[stroke..].find("10 20 30 40 re").unwrap() + stroke;
    let stroke_operator = pdf[stroke_path..].find("S").unwrap() + stroke_path;
    assert!(fill < fill_path && fill_path < fill_operator);
    assert!(fill_operator < stroke && stroke < stroke_path && stroke_path < stroke_operator);
    assert!(!pdf.contains("/ExtGState"));
  }

  #[test]
  fn direct_writer_emits_one_clipped_axial_shading_for_prepared_multistop_gradient() {
    let gradient = common::GradientFill {
      stops: vec![
        common::GradientStop {
          position: 0.0,
          color: color(255, 0, 0, 128),
          scheme: None,
        },
        common::GradientStop {
          position: 0.25,
          color: color(0, 255, 0, 128),
          scheme: None,
        },
        common::GradientStop {
          position: 1.0,
          color: color(0, 0, 255, 128),
          scheme: None,
        },
      ],
      line: Some((
        common::Point {
          x: Pt(10.0),
          y: Pt(20.0),
        },
        common::Point {
          x: Pt(110.0),
          y: Pt(70.0),
        },
      )),
      ..Default::default()
    };
    let mut document = text_document(
      "",
      common::TextStyle {
        font_size: Pt(12.0),
        color: color(0, 0, 0, 255),
        ..Default::default()
      },
    );
    document.pages[0].items.insert(
      0,
      common::DisplayItem::Rect(common::RectItem {
        bounds: common::Rect {
          origin: common::Point {
            x: Pt(10.0),
            y: Pt(20.0),
          },
          size: Size {
            width: Pt(100.0),
            height: Pt(50.0),
          },
        },
        fill: common::Fill::Gradient(gradient),
        stroke: Some(common::Stroke {
          width: Pt(1.0),
          color: color(0, 0, 0, 255),
          ..Default::default()
        }),
      }),
    );

    let bytes = render(&document, &uncompressed_options()).unwrap();
    let pdf = String::from_utf8_lossy(&bytes);
    assert_eq!(pdf.matches("/ShadingType 2").count(), 1);
    assert_eq!(pdf.matches("/FunctionType 3").count(), 1);
    assert_eq!(pdf.matches("/FunctionType 2").count(), 2);
    assert!(pdf.contains("/Bounds[0.25]"));
    assert!(pdf.contains("/Coords[10 20 110 70]"));
    assert!(pdf.contains("/ca 0.5019608"));
    let clip_path = pdf.find("10 20 100 50 re").unwrap();
    let clip = pdf[clip_path..].find("W*").unwrap() + clip_path;
    let end_path = pdf[clip..].find("n").unwrap() + clip;
    let shading = pdf[end_path..].find("/Sh0 sh").unwrap() + end_path;
    let stroke_path = pdf[shading..].find("10 20 100 50 re").unwrap() + shading;
    let stroke = pdf[stroke_path..].find("S").unwrap() + stroke_path;
    assert!(clip_path < clip && clip < end_path && end_path < shading);
    assert!(shading < stroke_path && stroke_path < stroke);

    document.pages[0].items.pop();
    let bytes = render(&document, &uncompressed_options()).unwrap();
    let pdf = String::from_utf8_lossy(&bytes);
    assert_eq!(pdf.matches("/ShadingType 2").count(), 1);
    let clip_path = pdf.find("10 20 100 50 re").unwrap();
    let clip = pdf[clip_path..].find("W*").unwrap() + clip_path;
    let end_path = pdf[clip..].find("n").unwrap() + clip;
    let shading = pdf[end_path..].find("/Sh0 sh").unwrap() + end_path;
    assert!(clip_path < clip && clip < end_path && end_path < shading);
  }

  #[test]
  fn direct_writer_selects_opaque_constant_and_variable_gradient_alpha_paths() {
    let render_gradient = |alphas: [u8; 3]| {
      let gradient = common::GradientFill {
        stops: vec![
          common::GradientStop {
            position: 0.0,
            color: color(255, 0, 0, alphas[0]),
            scheme: None,
          },
          common::GradientStop {
            position: 0.25,
            color: color(0, 255, 0, alphas[1]),
            scheme: None,
          },
          common::GradientStop {
            position: 1.0,
            color: color(0, 0, 255, alphas[2]),
            scheme: None,
          },
        ],
        line: Some((
          common::Point {
            x: Pt(10.0),
            y: Pt(20.0),
          },
          common::Point {
            x: Pt(110.0),
            y: Pt(70.0),
          },
        )),
        ..Default::default()
      };
      let mut document = blank_document(&[(612.0, 792.0)]);
      document.pages[0]
        .items
        .push(common::DisplayItem::Rect(common::RectItem {
          bounds: common::Rect {
            origin: common::Point {
              x: Pt(10.0),
              y: Pt(20.0),
            },
            size: Size {
              width: Pt(100.0),
              height: Pt(50.0),
            },
          },
          fill: common::Fill::Gradient(gradient),
          stroke: None,
        }));
      String::from_utf8_lossy(&render(&document, &uncompressed_options()).unwrap()).into_owned()
    };

    let opaque = render_gradient([u8::MAX; 3]);
    assert_eq!(opaque.matches("/ShadingType 2").count(), 1, "{opaque}");
    assert!(!opaque.contains("/ExtGState"), "{opaque}");
    assert!(!opaque.contains("/SMask"), "{opaque}");

    let constant = render_gradient([128; 3]);
    assert_eq!(constant.matches("/ShadingType 2").count(), 1, "{constant}");
    assert!(constant.contains("/ca 0.5019608"), "{constant}");
    assert!(!constant.contains("/SMask"), "{constant}");

    let variable = render_gradient([u8::MAX, 128, 0]);
    assert_eq!(variable.matches("/ShadingType 2").count(), 2, "{variable}");
    assert_eq!(variable.matches("/FunctionType 3").count(), 2, "{variable}");
    assert_eq!(variable.matches("/FunctionType 2").count(), 4, "{variable}");
    assert_eq!(variable.matches("/GSM0 gs").count(), 1, "{variable}");
    assert_eq!(variable.matches("/SMask").count(), 1, "{variable}");
    assert!(variable.contains("/S/Luminosity"), "{variable}");
    assert!(variable.contains("/CS/DeviceGray"), "{variable}");
    assert!(variable.contains("/ColorSpace/DeviceGray"), "{variable}");
    assert!(variable.contains("/BC[0]"), "{variable}");
    assert!(variable.contains("/BBox[0 0 612 792]"), "{variable}");
    assert!(variable.contains("/ExtGState<</GSM0"), "{variable}");
    assert!(!variable.contains("/ca "), "{variable}");
  }

  #[test]
  fn direct_writer_keeps_unresolved_gradient_boundaries_typed() {
    let bounds = common::Rect {
      origin: common::Point {
        x: Pt(0.0),
        y: Pt(0.0),
      },
      size: Size {
        width: Pt(100.0),
        height: Pt(100.0),
      },
    };
    let stops = vec![
      common::GradientStop {
        position: 0.0,
        color: color(255, 0, 0, 255),
        scheme: None,
      },
      common::GradientStop {
        position: 1.0,
        color: color(0, 0, 255, 255),
        scheme: None,
      },
    ];
    let mut path_gradient = common::GradientFill {
      stops: stops.clone(),
      path: Some(common::GradientPath {
        kind: common::GradientPathKind::Circle,
        context: common::GradientPathContext::DrawingObject,
        fill_to: common::RelativeRect::default(),
        transform: common::Transform::default(),
        mirror_tile: false,
      }),
      ..Default::default()
    };
    DirectGradientSet::validate(&path_gradient, bounds).unwrap();

    path_gradient.path = None;
    path_gradient.stops[1].color.a = 127;
    DirectGradientSet::validate(&path_gradient, bounds).unwrap();

    path_gradient.stops = vec![
      stops[0].clone(),
      common::GradientStop {
        position: 0.0,
        color: color(0, 255, 0, 255),
        scheme: None,
      },
      stops[1].clone(),
    ];
    assert!(matches!(
      DirectGradientSet::validate(&path_gradient, bounds),
      Err(PdfError::DirectWriterUnsupported {
        feature: "coincident hard-edge gradient stops"
      })
    ));
  }

  fn glyph_path_gradient(
    kind: common::GradientPathKind,
    transform: common::Transform,
    definition_bounds: Option<common::Rect>,
  ) -> common::GradientFill<'static> {
    common::GradientFill {
      definition_bounds,
      path: Some(common::GradientPath {
        kind,
        context: common::GradientPathContext::WordprocessingText,
        fill_to: common::RelativeRect::default(),
        transform,
        mirror_tile: false,
      }),
      ..Default::default()
    }
  }

  fn glyph_gradient_bounds() -> common::Rect {
    common::Rect {
      origin: common::Point {
        x: Pt(20.0),
        y: Pt(30.0),
      },
      size: Size {
        width: Pt(100.0),
        height: Pt(50.0),
      },
    }
  }

  #[test]
  fn direct_writer_binds_unresolved_glyph_circle_gradient_to_outline_bounds() {
    let bounds = glyph_gradient_bounds();
    let mut gradient = glyph_path_gradient(
      common::GradientPathKind::Circle,
      common::Transform::default(),
      None,
    );
    gradient.path.as_mut().unwrap().fill_to = common::RelativeRect {
      left: 0.5,
      top: 1.3,
      right: 0.5,
      bottom: -0.3,
    };
    let resolved = resolved_glyph_outline_gradient(&gradient, bounds);
    let path = resolved.path.expect("bound circle gradient path");
    let diameter = 2.0 * 65.0_f32.hypot(50.0);

    assert_eq!(resolved.definition_bounds, Some(bounds));
    assert_eq!(
      path.fill_to,
      common::RelativeRect {
        left: 0.5,
        top: 0.5,
        right: 0.5,
        bottom: 0.5,
      }
    );
    assert!((path.transform.m11 - diameter).abs() < 1.0e-4);
    assert!((path.transform.m22 - diameter).abs() < 1.0e-4);
    assert!((path.transform.dx.0 + path.transform.m11 * 0.5 - 70.0).abs() < 1.0e-4);
    assert!((path.transform.dy.0 + path.transform.m22 * 0.5 - 95.0).abs() < 1.0e-4);
  }

  #[test]
  fn direct_writer_does_not_rebind_resolved_glyph_path_gradient() {
    let bounds = glyph_gradient_bounds();
    let gradient = glyph_path_gradient(
      common::GradientPathKind::Circle,
      common::Transform {
        m11: 120.0,
        m12: 0.0,
        m21: 0.0,
        m22: 120.0,
        dx: Pt(10.0),
        dy: Pt(20.0),
      },
      Some(bounds),
    );

    assert_eq!(resolved_glyph_outline_gradient(&gradient, bounds), gradient);
  }

  #[test]
  fn direct_writer_binds_unresolved_glyph_raster_gradient_to_outline_bounds() {
    let bounds = glyph_gradient_bounds();
    let resolved = resolved_glyph_outline_gradient(
      &glyph_path_gradient(
        common::GradientPathKind::Rectangle,
        common::Transform::default(),
        None,
      ),
      bounds,
    );
    let path = resolved.path.expect("bound rectangle gradient path");

    assert_eq!(resolved.definition_bounds, Some(bounds));
    assert_eq!(
      path.transform,
      common::Transform {
        m11: 100.0,
        m12: 0.0,
        m21: 0.0,
        m22: 50.0,
        dx: Pt(20.0),
        dy: Pt(30.0),
      }
    );
  }

  #[test]
  fn direct_writer_keeps_office_and_requested_path_gradient_owners_distinct() {
    let bounds = common::Rect {
      origin: common::Point {
        x: Pt(20.0),
        y: Pt(30.0),
      },
      size: Size {
        width: Pt(100.0),
        height: Pt(100.0),
      },
    };
    let gradient = common::GradientFill {
      stops: vec![
        common::GradientStop {
          position: 0.0,
          color: color(255, 255, 0, 255),
          scheme: None,
        },
        common::GradientStop {
          position: 1.0,
          color: color(0, 176, 80, 255),
          scheme: None,
        },
      ],
      path: Some(common::GradientPath {
        kind: common::GradientPathKind::Circle,
        context: common::GradientPathContext::DrawingObject,
        fill_to: common::RelativeRect {
          left: 0.2,
          top: 0.5,
          right: 0.8,
          bottom: 0.5,
        },
        transform: common::Transform {
          m11: 100.0,
          m12: 0.0,
          m21: 0.0,
          m22: 100.0,
          dx: Pt(20.0),
          dy: Pt(30.0),
        },
        mirror_tile: false,
      }),
      ..Default::default()
    };
    let document = |gradient: common::GradientFill<'static>, bounds| {
      let mut document = blank_document(&[(200.0, 200.0)]);
      document.engine_kind = common::LayoutEngineKind::Pptx;
      document.pages[0]
        .items
        .push(common::DisplayItem::Rect(common::RectItem {
          bounds,
          fill: common::Fill::Gradient(gradient),
          stroke: None,
        }));
      document
    };

    let requested = String::from_utf8_lossy(
      &render(&document(gradient.clone(), bounds), &uncompressed_options()).unwrap(),
    )
    .into_owned();
    assert!(requested.contains("/ShadingType 3"), "{requested}");
    assert!(
      requested.contains("/Coords[40 80 0 70 80 50]"),
      "{requested}"
    );
    assert!(
      requested.contains("/Matrix[1 0 0 -1 0 200.04]"),
      "{requested}"
    );

    let mut office_options = uncompressed_options();
    office_options.images.optimization_policy =
      PdfImageOptimizationPolicy::MicrosoftOfficeFixedOutput(crate::PdfDocumentKind::Pptx);
    let office =
      String::from_utf8_lossy(&render(&document(gradient, bounds), &office_options).unwrap())
        .into_owned();
    assert!(office.contains("/ShadingType 3"), "{office}");
    assert!(office.contains("/Coords[70 80 0 70 80 50]"), "{office}");
    assert_eq!(office.matches("/PatternType 2").count(), 1, "{office}");

    let rectangle = common::GradientFill {
      stops: vec![
        common::GradientStop {
          position: 0.0,
          color: color(255, 255, 255, 255),
          scheme: None,
        },
        common::GradientStop {
          position: 1.0,
          color: color(0, 0, 255, 255),
          scheme: None,
        },
      ],
      path: Some(common::GradientPath {
        kind: common::GradientPathKind::Rectangle,
        context: common::GradientPathContext::DrawingObject,
        fill_to: common::RelativeRect {
          left: 0.5,
          top: 0.5,
          right: 0.5,
          bottom: 0.5,
        },
        transform: common::Transform {
          m11: 100.0,
          m12: 0.0,
          m21: 0.0,
          m22: 50.0,
          dx: Pt(20.0),
          dy: Pt(30.0),
        },
        mirror_tile: false,
      }),
      ..Default::default()
    };
    let rectangle_bounds = common::Rect {
      size: Size {
        width: Pt(100.0),
        height: Pt(50.0),
      },
      ..bounds
    };
    let office_rectangle = String::from_utf8_lossy(
      &render(&document(rectangle, rectangle_bounds), &office_options).unwrap(),
    )
    .into_owned();
    assert!(
      !office_rectangle.contains("/ShadingType"),
      "{office_rectangle}"
    );
    assert!(
      office_rectangle.contains("/Subtype/Image"),
      "{office_rectangle}"
    );
    assert!(
      office_rectangle.contains("/Width 133"),
      "{office_rectangle}"
    );
    assert!(
      office_rectangle.contains("/Height 66"),
      "{office_rectangle}"
    );
    assert!(
      !office_rectangle.contains("/Interpolate"),
      "{office_rectangle}"
    );
  }

  #[test]
  fn direct_writer_preserves_powerpoint_separate_fill_then_stroke_operators() {
    let mut document = blank_document(&[(720.0, 540.0)]);
    document.engine_kind = common::LayoutEngineKind::Pptx;
    document.pages[0]
      .items
      .push(common::DisplayItem::Path(common::PathItem {
        commands: vec![
          common::PathCommand::MoveTo(common::Point {
            x: Pt(10.0),
            y: Pt(10.0),
          }),
          common::PathCommand::LineTo(common::Point {
            x: Pt(40.0),
            y: Pt(10.0),
          }),
          common::PathCommand::LineTo(common::Point {
            x: Pt(25.0),
            y: Pt(35.0),
          }),
          common::PathCommand::Close,
        ],
        fill: common::Fill::Solid(color(255, 0, 0, 255)),
        stroke: Some(common::Stroke {
          width: Pt(1.0),
          color: color(0, 0, 255, 255),
          ..Default::default()
        }),
        ..Default::default()
      }));

    let bytes = render(&document, &uncompressed_options()).unwrap();
    let pdf = String::from_utf8_lossy(&bytes);
    let fill = pdf.find("f*").unwrap();
    let stroke = pdf[fill..].find("S").unwrap() + fill;

    assert!(fill < stroke);
    assert!(!pdf.contains("B*"));
  }

  #[test]
  fn direct_writer_embeds_and_reuses_one_ordinary_opaque_raster() {
    let mut image = test_image_item(opaque_test_png());
    // This flag is inert for a proven PNG. The legacy renderer consults it
    // only after metafile parsing succeeds.
    image.metafile_native_size = true;
    let mut document = blank_document(&[(100.0, 100.0), (100.0, 100.0)]);
    document.pages[0]
      .items
      .push(common::DisplayItem::Image(image.clone()));
    document.pages[1]
      .items
      .push(common::DisplayItem::Image(image));

    let bytes = render(&document, &uncompressed_options()).unwrap();
    let pdf = String::from_utf8_lossy(&bytes);
    assert_eq!(pdf.matches("/Subtype/Image").count(), 1);
    assert_eq!(pdf.matches("/XObject<</Im0").count(), 2);
    assert_eq!(pdf.matches("/Im0 Do").count(), 2);
    assert!(
      pdf.contains("/Width 2/Height 2/BitsPerComponent 8/ColorSpace/DeviceRGB"),
      "{pdf}"
    );
    assert!(pdf.contains("1 0 0 1 10 20 cm 30 0 0 -40 0 40 cm/Im0 Do"));
    assert!(!pdf.contains("/SMask"));
    assert!(!pdf.contains("/Interpolate"));
  }

  #[test]
  fn direct_writer_paints_the_office_missing_linked_image_frame_and_icon() {
    let mut image = test_image_item(Vec::new());
    image.content_type = "application/octet-stream".into();
    image.relationship_id = Some("rId5".into());
    let pdf =
      String::from_utf8_lossy(&render(&image_document(image), &uncompressed_options()).unwrap())
        .into_owned();

    assert_eq!(pdf.matches("/Subtype/Image").count(), 1, "{pdf}");
    assert!(
      pdf.contains("/Width 4/Height 5/BitsPerComponent 8/ColorSpace/DeviceRGB"),
      "{pdf}"
    );
    assert!(!pdf.contains("/Interpolate"), "{pdf}");
    assert!(
      pdf.contains("0 0 0 RG 0.14 w 0 J 0 j[]0 d 10.07 20.07 29.86 39.86 re\nS"),
      "{pdf}"
    );
    assert!(
      pdf.contains("1.68 0 0 -1.92 10.84 22.76 cm/Im0 Do"),
      "{pdf}"
    );
  }

  #[test]
  fn missing_linked_image_icon_is_the_fixed_office_rgb_sample_plane() {
    const EXPECTED: [u8; 60] = [
      128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, // row 1
      128, 128, 128, 255, 255, 255, 255, 255, 255, 255, 255, 255, // row 2
      128, 128, 128, 255, 255, 255, 255, 0, 0, 255, 255, 255, // row 3
      128, 128, 128, 255, 204, 204, 255, 255, 255, 255, 255, 255, // row 4
      128, 128, 128, 255, 255, 255, 255, 255, 255, 255, 255, 255, // row 5
    ];
    let icon = missing_linked_image_icon();
    let direct = icon.direct();

    assert_eq!((direct.width, direct.height), (4, 5));
    assert_eq!(direct.color_space, DirectRasterColorSpace::Rgb);
    assert_eq!(direct.bits_per_component, 8);
    assert!(!direct.interpolate);
    assert!(direct.matte.is_none());
    let DirectRasterEncoding::Sampled { pixels } = &direct.encoding else {
      panic!("missing-image icon must use the unencoded sampled transport");
    };
    assert_eq!(pixels.rgb, EXPECTED);
    assert!(pixels.alpha.is_none());
    assert!(pixels.icc_profile.is_none());
  }

  #[test]
  fn image_support_keeps_signature_svg_and_invalid_raster_states_independent() {
    let mut signature = paint_metafile_item();
    signature.data = Cow::Borrowed(b"signature preview");
    signature.signature_line = Some(common::SignatureLineProperties::default());
    assert!(
      matches!(
        ensure_image_supported(&signature),
        Err(PdfError::DirectWriterUnsupported {
          feature: "unsigned signature-line image semantics"
        })
      ),
      "an unsigned signature line that escaped paint expansion remains unsupported"
    );

    for state in [
      common::SignatureLineState::SignedValid,
      common::SignatureLineState::SignedInvalid,
    ] {
      signature.signature_line.as_mut().unwrap().state = state;
      assert!(
        ensure_image_supported(&signature).is_ok(),
        "a signed provider image is ordinary image content after validity selection: {state:?}"
      );
    }

    signature.data = Cow::Borrowed(&[]);
    assert!(matches!(
      ensure_image_supported(&signature),
      Err(PdfError::DirectWriterUnsupported {
        feature: "signed signature-line image without provider data"
      })
    ));

    let mut svg_paint = paint_metafile_item();
    svg_paint.data = Cow::Borrowed(b"<svg xmlns='http://www.w3.org/2000/svg'/>");
    svg_paint.content_type = Some(Cow::Borrowed("image/svg+xml"));
    assert!(ensure_image_supported(&svg_paint).is_ok());

    let mut svg = test_image_item(b"<svg xmlns='http://www.w3.org/2000/svg'/>".to_vec());
    svg.content_type = "image/svg+xml".into();
    assert!(render(&image_document(svg), &uncompressed_options()).is_ok());

    let malformed = test_image_item(b"not a PNG".to_vec());
    assert!(matches!(
      render(&image_document(malformed), &uncompressed_options()),
      Err(PdfError::Image(_))
    ));
  }

  #[test]
  fn direct_writer_source_crop_changes_only_clip_and_image_matrix() {
    let mut image = test_image_item(opaque_test_png());
    image.crop = Some(common::ImageCrop {
      left: 0.25,
      top: 0.0,
      right: 0.25,
      bottom: 0.0,
    });

    let pdf =
      String::from_utf8_lossy(&render(&image_document(image), &uncompressed_options()).unwrap())
        .into_owned();
    assert!(pdf.contains("0 0 30 40 re\nW\nn"), "{pdf}");
    assert!(pdf.contains("60 0 0 -40 -15 40 cm/Im0 Do"));
  }

  #[test]
  fn direct_writer_horizontal_and_vertical_image_flips_are_independent() {
    let mut horizontal = test_image_item(opaque_test_png());
    horizontal.flip_horizontal = true;
    let horizontal_pdf = String::from_utf8_lossy(
      &render(&image_document(horizontal), &uncompressed_options()).unwrap(),
    )
    .into_owned();
    assert!(horizontal_pdf.contains("-30 0 0 -40 30 40 cm/Im0 Do"));

    let mut vertical = test_image_item(opaque_test_png());
    vertical.flip_vertical = true;
    let vertical_pdf =
      String::from_utf8_lossy(&render(&image_document(vertical), &uncompressed_options()).unwrap())
        .into_owned();
    assert!(vertical_pdf.contains("30 0 0 40 0 0 cm/Im0 Do"));
  }

  #[test]
  fn direct_writer_rotates_an_image_about_its_frame_center() {
    let mut image = test_image_item(opaque_test_png());
    image.rotation_degrees = 90.0;
    let pdf =
      String::from_utf8_lossy(&render(&image_document(image), &uncompressed_options()).unwrap())
        .into_owned();
    let matrices = content_matrices(&pdf);
    assert_eq!(matrices.len(), 3, "{pdf}");
    assert_matrix_near(matrices[1], [0.0, 1.0, -1.0, 0.0, 45.0, 25.0]);
    assert_eq!(matrices[2], [30.0, 0.0, 0.0, -40.0, 0.0, 40.0]);
  }

  #[test]
  fn direct_writer_vectorizes_plain_svg_and_sniffs_metafile_bytes_before_fallback() {
    let svg = br##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 10 10">
      <g transform="translate(1 2)"><path d="M0 0L8 0L4 7Z" fill="#123456"/></g>
    </svg>"##;
    let mut svg = test_image_item(svg.to_vec());
    svg.content_type = "application/octet-stream".into();
    let pdf =
      String::from_utf8_lossy(&render(&image_document(svg), &uncompressed_options()).unwrap())
        .into_owned();
    assert!(!pdf.contains("/Subtype/Image"), "{pdf}");
    assert!(pdf.contains("0.07058824 0.20392157 0.3372549 rg"), "{pdf}");
    assert!(pdf.contains("0 0 m 8 0 l 4 7 l"), "{pdf}");
    assert!(pdf.contains("f\n"), "{pdf}");

    let mut metafile = test_image_item(opaque_test_png());
    metafile.content_type = "image/x-emf".into();
    let pdf =
      String::from_utf8_lossy(&render(&image_document(metafile), &uncompressed_options()).unwrap())
        .into_owned();
    assert!(pdf.contains("/Subtype/Image"), "{pdf}");
    assert!(pdf.contains("/Im0 Do"), "{pdf}");
  }

  #[test]
  fn direct_writer_rasterizes_svg_compositing_at_the_fixed_output_density() {
    let svg = br##"<svg xmlns="http://www.w3.org/2000/svg" width="30" height="40">
      <defs><linearGradient id="g"><stop offset="0" stop-color="#102030"/><stop offset="1" stop-color="#f0e0d0"/></linearGradient></defs>
      <rect width="30" height="40" fill="url(#g)"/>
    </svg>"##;
    let mut image = test_image_item(svg.to_vec());
    image.content_type = "image/svg+xml".into();
    let pdf =
      String::from_utf8_lossy(&render(&image_document(image), &uncompressed_options()).unwrap())
        .into_owned();

    assert_eq!(pdf.matches("/Subtype/Image").count(), 1, "{pdf}");
    assert!(pdf.contains("/Width 84/Height 112"), "{pdf}");
    assert!(pdf.contains("/Interpolate true"), "{pdf}");
    assert!(pdf.contains("/Im0 Do"), "{pdf}");

    let mut archival_image = test_image_item(svg.to_vec());
    archival_image.content_type = "image/svg+xml".into();
    let mut archival_options = uncompressed_options();
    archival_options.standards.push(PdfStandard::PdfA3a);
    archival_options.default_document_language = Some("en-US".to_string());
    archival_options.metadata.creation_date = Some(crate::PdfDateTime {
      year: 2026,
      month: Some(8),
      day: Some(18),
      hour: Some(12),
      minute: Some(0),
      second: Some(0),
      utc_offset_hour: Some(8),
      utc_offset_minute: Some(0),
    });
    let archival_pdf =
      String::from_utf8_lossy(&render(&image_document(archival_image), &archival_options).unwrap())
        .into_owned();
    assert!(
      !archival_pdf.contains("/Interpolate true"),
      "{archival_pdf}"
    );
  }

  #[test]
  fn direct_writer_lowers_only_the_closed_office_math_svg_scene() {
    let office_math = br##"<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 20 20">
      <defs><clipPath id="math-semantic-clip" clipPathUnits="userSpaceOnUse"><path d="M0 0L0 0"/></clipPath></defs>
      <rect x="1" y="2" width="5" height="4" fill="#123456" fill-opacity="0.5"/>
      <path d="M2 10L8 10L5 4Z" transform="matrix(1 0 0 -1 0 14)" fill="#abcdef" stroke="#102030" stroke-width="0.5" stroke-linejoin="round" paint-order="fill stroke"/>
      <text id="ooxmlsdk-math-visible-0" visibility="hidden" x="0" y="0" transform="translate(2 15) scale(0.9 1)" font-family="DejaVu Sans" font-size="12" fill="#00000a">x</text>
      <g clip-path="url(#math-semantic-clip)"><text id="ooxmlsdk-math-semantic-1" visibility="hidden" x="0" y="0" transform="translate(2 15) scale(0.9 1)" font-family="DejaVu Sans" font-size="12" font-weight="normal" font-style="normal" fill="#00000a" fill-opacity="1" stroke="none" xml:space="preserve">x</text></g>
      <g clip-path="url(#math-semantic-clip)"><text id="ooxmlsdk-math-semantic-2-gid-3" visibility="hidden" x="0" y="0" transform="translate(8 15) scale(1 1)" font-family="DejaVu Sans" font-size="12" font-weight="normal" font-style="normal" fill="#00000a" fill-opacity="1" stroke="none" xml:space="preserve"> </text></g>
      <line x1="1" y1="18" x2="19" y2="18" stroke="#445566" stroke-width="0.2" stroke-linecap="butt"/>
    </svg>"##;
    let mut image = test_image_item(office_math.to_vec());
    image.content_type = "application/vnd.ooxmlsdk.office-math+xml".into();
    let pdf =
      String::from_utf8_lossy(&render(&image_document(image), &uncompressed_options()).unwrap())
        .into_owned();
    assert!(pdf.contains("/Subtype/Type0"), "{pdf}");
    assert!(!pdf.contains("/Subtype/Image"), "{pdf}");
    assert!(pdf.contains("0 0 20 20 re\nW\nn"), "{pdf}");
    assert!(pdf.contains("0 0 m 0 0 l\nW\nn"), "{pdf}");
    assert!(pdf.contains("/F0 12 Tf"), "{pdf}");
    assert!(pdf.contains(")Tj"), "{pdf}");
    assert!(pdf.contains("(\\000\\002)Tj"), "{pdf}");

    let whitespace_without_exact_gid = br##"<svg xmlns="http://www.w3.org/2000/svg" width="10" height="10" viewBox="0 0 10 10">
      <defs><clipPath id="math-semantic-clip" clipPathUnits="userSpaceOnUse"><path d="M0 0L0 0"/></clipPath></defs>
      <g clip-path="url(#math-semantic-clip)"><text id="ooxmlsdk-math-semantic-0" visibility="hidden" x="0" y="0" transform="translate(1 8) scale(1 1)" font-family="DejaVu Sans" font-size="8" font-weight="normal" font-style="normal" fill="#000000" fill-opacity="1" stroke="none" xml:space="preserve"> </text></g>
    </svg>"##;
    let mut whitespace_without_exact_gid_image =
      test_image_item(whitespace_without_exact_gid.to_vec());
    whitespace_without_exact_gid_image.content_type =
      "application/vnd.ooxmlsdk.office-math+xml".into();
    assert!(matches!(
      render(
        &image_document(whitespace_without_exact_gid_image),
        &uncompressed_options()
      ),
      Err(PdfError::DirectWriterUnsupported {
        feature: "OfficeMath SVG dropped semantic carrier without exact GID"
      })
    ));

    let unsupported_svg = br##"<svg xmlns="http://www.w3.org/2000/svg" width="10" height="10">
      <defs><linearGradient id="g"><stop offset="0" stop-color="#000"/><stop offset="1" stop-color="#fff"/></linearGradient></defs>
      <rect width="10" height="10" fill="url(#g)"/>
    </svg>"##;
    let mut unsupported_image = test_image_item(unsupported_svg.to_vec());
    unsupported_image.content_type = "application/vnd.ooxmlsdk.office-math+xml".into();
    assert!(matches!(
      render(&image_document(unsupported_image), &uncompressed_options()),
      Err(PdfError::DirectWriterUnsupported {
        feature: "OfficeMath SVG paint servers"
      })
    ));
  }

  #[test]
  fn direct_writer_serializes_only_a_complete_metafile_vector_scene() {
    use ooxmlsdk_layout::render::emf_wmf::{
      MetafileVectorFill, MetafileVectorFillRule, MetafileVectorPoint, MetafileVectorScene,
    };

    let scene = MetafileVectorScene {
      fills: vec![
        MetafileVectorFill {
          subpaths: vec![
            vec![
              MetafileVectorPoint { x: 0.0, y: 0.0 },
              MetafileVectorPoint { x: 1.0, y: 0.0 },
              MetafileVectorPoint { x: 1.0, y: 1.0 },
              MetafileVectorPoint { x: 0.0, y: 1.0 },
            ],
            vec![
              MetafileVectorPoint { x: 0.25, y: 0.25 },
              MetafileVectorPoint { x: 0.75, y: 0.25 },
              MetafileVectorPoint { x: 0.75, y: 0.75 },
              MetafileVectorPoint { x: 0.25, y: 0.75 },
            ],
          ],
          color: [255, 0, 0],
          fill_rule: MetafileVectorFillRule::Alternate,
        },
        MetafileVectorFill {
          subpaths: vec![vec![
            MetafileVectorPoint { x: 0.0, y: 0.0 },
            MetafileVectorPoint { x: 0.5, y: 1.0 },
            MetafileVectorPoint { x: 1.0, y: 0.0 },
          ]],
          color: [0, 0, 255],
          fill_rule: MetafileVectorFillRule::Winding,
        },
      ],
    };
    let mut content = Content::with_settings(Settings { pretty: false });
    write_metafile_vector_scene(&mut content, &paint_metafile_item(), &scene).unwrap();
    let stream = String::from_utf8(content.finish().into_vec()).unwrap();
    let tokens = stream
      .split_ascii_whitespace()
      .collect::<Vec<_>>()
      .join(" ");

    assert!(tokens.contains("1 0 0 1 10 20 cm"), "{stream}");
    assert!(tokens.contains("0 0 m 30 0 l 30 40 l 0 40 l h"), "{stream}");
    assert!(tokens.contains("7.5 10 m 22.5 10 l 22.5 30 l"), "{stream}");
    assert!(tokens.contains("1 0 0 rg f*"), "{stream}");
    assert!(tokens.contains("0 0 1 rg f"), "{stream}");
    assert!(!tokens.contains(" Do"), "{stream}");

    let invalid = MetafileVectorScene {
      fills: vec![MetafileVectorFill {
        subpaths: vec![vec![MetafileVectorPoint {
          x: f32::NAN,
          y: 0.0,
        }]],
        color: [0, 0, 0],
        fill_rule: MetafileVectorFillRule::Winding,
      }],
    };
    let mut untouched = Content::with_settings(Settings { pretty: false });
    assert!(matches!(
      write_metafile_vector_scene(&mut untouched, &paint_metafile_item(), &invalid),
      Err(PdfError::Writer(message))
        if message == "metafile vector scene contains a non-finite point"
    ));
    assert!(untouched.finish().into_vec().is_empty());
  }

  #[test]
  fn direct_writer_tags_image_links_and_removes_the_external_counterexample() {
    let target = "https://example.test/image";
    let mut linked = test_image_item(opaque_test_png());
    linked.hyperlink_url = Some(target.into());
    let document = image_document(linked);
    let mut options = uncompressed_options();
    options.general.tagged_pdf = true;

    let pdf = String::from_utf8_lossy(&render(&document, &options).unwrap()).into_owned();
    assert!(pdf.contains("/Subtype/Link"), "{pdf}");
    assert!(pdf.contains("/Rect[10 39.96 40 79.96]"), "{pdf}");
    assert!(pdf.contains(&format!("/Contents({target})")), "{pdf}");
    assert!(pdf.contains("/StructParent 0"), "{pdf}");
    assert!(pdf.contains("/S/Link"), "{pdf}");
    assert!(pdf.contains("/Type/OBJR"), "{pdf}");
    assert!(pdf.contains("/ParentTreeNextKey 1"), "{pdf}");
    assert!(!pdf.contains("/StructParents"), "{pdf}");

    options.links.default_action = crate::PdfLinkDefaultAction::RemoveExternalLinks;
    let removed = String::from_utf8_lossy(&render(&document, &options).unwrap()).into_owned();
    assert!(!removed.contains("/Subtype/Link"), "{removed}");
    assert!(!removed.contains("/StructParent"), "{removed}");
    assert!(!removed.contains("/Type/OBJR"), "{removed}");
    assert!(!removed.contains("/ParentTree"), "{removed}");
  }

  #[test]
  fn direct_writer_embeds_ordinary_text_as_positioned_type0_cid_glyphs() {
    let document = text_document(
      "aaaa",
      common::TextStyle {
        font_family: Some("Liberation Serif".into()),
        font_size: Pt(12.0),
        color: color(12, 34, 56, u8::MAX),
        ..Default::default()
      },
    );

    let bytes = render(&document, &uncompressed_options()).unwrap();
    let pdf = String::from_utf8_lossy(&bytes);
    assert!(pdf.contains("/Subtype/Type0"));
    assert!(pdf.contains("/Encoding/Identity-H"));
    assert!(pdf.contains("/Subtype/CIDFontType2"));
    assert!(pdf.contains("/CIDToGIDMap/Identity"));
    assert!(pdf.contains("/ToUnicode"));
    assert!(pdf.contains("/FontFile2"));
    let expected_color = format!(
      "{} {} {} rg",
      f32::from(12_u8) / f32::from(u8::MAX),
      f32::from(34_u8) / f32::from(u8::MAX),
      f32::from(56_u8) / f32::from(u8::MAX),
    );
    assert!(pdf.contains(&expected_color), "{pdf}");
    assert!(pdf.contains("BT"));
    assert!(pdf.contains("1 0 0 -1 72 "));
    assert!(pdf.contains(" Tm"));
    assert!(pdf.contains("TJ"));
  }

  #[test]
  fn direct_writer_culls_fully_off_page_prepared_text_but_keeps_edge_intersections() {
    let style = common::TextStyle {
      font_family: Some("Liberation Serif".into()),
      font_size: Pt(12.0),
      color: color(0, 0, 0, u8::MAX),
      ..Default::default()
    };
    let mut outside = text_document("outside", style.clone());
    let common::DisplayItem::Text(text) = &mut outside.pages[0].items[0] else {
      unreachable!();
    };
    text.origin.x = Pt(621.0);
    let outside_pdf =
      String::from_utf8_lossy(&render(&outside, &uncompressed_options()).unwrap()).into_owned();
    assert!(!outside_pdf.contains("/Subtype/Type0"), "{outside_pdf}");
    assert!(!outside_pdf.contains("\nBT "), "{outside_pdf}");

    let mut crossing = text_document("crossing", style);
    let common::DisplayItem::Text(text) = &mut crossing.pages[0].items[0] else {
      unreachable!();
    };
    text.origin.x = Pt(600.0);
    let crossing_pdf =
      String::from_utf8_lossy(&render(&crossing, &uncompressed_options()).unwrap()).into_owned();
    assert!(crossing_pdf.contains("/Subtype/Type0"), "{crossing_pdf}");
    assert_eq!(crossing_pdf.matches("\nBT ").count(), 1, "{crossing_pdf}");
  }

  #[test]
  fn direct_writer_keeps_tab_advance_and_decorations_without_painting_a_tab_glyph() {
    let style = common::TextStyle {
      font_family: Some("Liberation Serif".into()),
      font_size: Pt(12.0),
      color: color(0, 0, 0, u8::MAX),
      ..Default::default()
    };
    let document = text_document("A\tB", style.clone());
    let options = uncompressed_options();
    let paint = super::super::paint::prepare_for_direct(&document, &options);
    let super::super::paint::PaintItem::Text(text) = &paint.pages[0].items[0] else {
      unreachable!();
    };
    assert_eq!(text.portions.len(), 3);
    assert_eq!(
      text
        .portions
        .iter()
        .map(|portion| portion.kind)
        .collect::<Vec<_>>(),
      [
        super::super::paint::PaintTextPortionKind::Text,
        super::super::paint::PaintTextPortionKind::Tab,
        super::super::paint::PaintTextPortionKind::Text,
      ]
    );
    let tab = &text.portions[1];
    assert_eq!(text.item.text.get(tab.text_range.clone()), Some("\t"));
    assert!(tab.width_pt > 0.0);
    assert!(
      (text.portions[2].x_pt - (tab.x_pt + tab.width_pt)).abs() <= 1.0e-4,
      "following text origin {} does not include tab advance {} + {}",
      text.portions[2].x_pt,
      tab.x_pt,
      tab.width_pt
    );

    let mut tab_without_glyphs = text.as_ref().clone();
    tab_without_glyphs.portions[1].glyphs = None;
    assert!(ensure_ordinary_text_supported(&tab_without_glyphs).is_ok());

    let mut invalid_tab = text.as_ref().clone();
    invalid_tab.portions[1].text_range = 0..1;
    assert!(matches!(
      ensure_ordinary_text_supported(&invalid_tab),
      Err(PdfError::Writer(message))
        if message == "tab text portion must cover exactly one tab character"
    ));

    let pdf = String::from_utf8_lossy(&render(&document, &options).unwrap()).into_owned();
    assert_eq!(pdf.matches("\nBT ").count(), 2, "{pdf}");

    let decorated = text_document(
      "\t",
      common::TextStyle {
        underline: true,
        ..style
      },
    );
    let decorated_pdf =
      String::from_utf8_lossy(&render(&decorated, &options).unwrap()).into_owned();
    assert!(!decorated_pdf.contains("/Subtype/Type0"), "{decorated_pdf}");
    assert!(!decorated_pdf.contains("\nBT "), "{decorated_pdf}");
    assert!(decorated_pdf.contains("\nS\n"), "{decorated_pdf}");
  }

  #[test]
  fn direct_writer_paints_solid_glyph_paths_and_keeps_only_the_requested_semantic_layer() {
    let outlined_style = |semantic_text_overlay| common::TextStyle {
      font_family: Some("Liberation Serif".into()),
      font_size: Pt(12.0),
      color: color(235, 233, 233, u8::MAX),
      outline_color: Some(color(57, 82, 116, u8::MAX)),
      outline_width: Pt(0.5),
      pdf_glyph_outlines: true,
      pdf_glyph_outline_options: Some(Arc::new(common::PdfGlyphOutlineOptions {
        semantic_text_overlay,
        outline_stroke: Some(common::Stroke {
          width: Pt(0.5),
          color: color(57, 82, 116, u8::MAX),
          cap: Some(common::StrokeCap::Flat),
          join: Some(common::StrokeJoin::Round),
          compound: Some(common::StrokeCompound::Single),
          alignment: Some(common::StrokeAlignment::Center),
          ..Default::default()
        }),
        ..Default::default()
      })),
      ..Default::default()
    };

    let semantic = String::from_utf8_lossy(
      &render(
        &text_document("Mills", outlined_style(true)),
        &uncompressed_options(),
      )
      .unwrap(),
    )
    .into_owned();
    assert!(semantic.contains("/Subtype/Type0"), "{semantic}");
    assert!(semantic.contains("/ToUnicode"), "{semantic}");
    assert!(semantic.contains("/ca 0"), "{semantic}");
    let semantic_content = object_body(&semantic, 4);
    let semantic_tokens = semantic_content
      .split_ascii_whitespace()
      .collect::<Vec<_>>();
    assert!(semantic_content.contains("0 0 0 rg"), "{semantic_content}");
    assert!(semantic_content.contains("0.5 w"), "{semantic_content}");
    assert!(semantic_content.contains("1 j"), "{semantic_content}");
    assert!(semantic_tokens.contains(&"f*"), "{semantic_content}");
    assert!(semantic_tokens.contains(&"S"), "{semantic_content}");

    let paths_only = String::from_utf8_lossy(
      &render(
        &text_document("Mills", outlined_style(false)),
        &uncompressed_options(),
      )
      .unwrap(),
    )
    .into_owned();
    assert!(!paths_only.contains("/Subtype/Type0"), "{paths_only}");
    assert!(!paths_only.contains("/ToUnicode"), "{paths_only}");
    let paths_only_content = object_body(&paths_only, 4);
    let paths_only_tokens = paths_only_content
      .split_ascii_whitespace()
      .collect::<Vec<_>>();
    assert!(!paths_only_content.contains("BT"), "{paths_only_content}");
    assert!(!paths_only.contains("/ca 0"), "{paths_only}");
    assert!(paths_only_content.contains("0.5 w"), "{paths_only_content}");
    assert!(paths_only_tokens.contains(&"f*"), "{paths_only_content}");
    assert!(paths_only_tokens.contains(&"S"), "{paths_only_content}");
  }

  #[test]
  fn direct_writer_uses_one_shading_pattern_for_glyph_fill_and_expanded_gradient_stroke() {
    let gradient = common::GradientFill {
      stops: vec![
        common::GradientStop {
          position: 0.0,
          color: color(251, 251, 251, 128),
          scheme: None,
        },
        common::GradientStop {
          position: 0.86,
          color: color(68, 114, 196, 128),
          scheme: None,
        },
      ],
      angle_degrees: Some(90.0),
      ..Default::default()
    };
    let outlined_style = |gradient: common::GradientFill<'static>| common::TextStyle {
      font_family: Some("Liberation Serif".into()),
      font_size: Pt(12.0),
      color: color(235, 233, 233, u8::MAX),
      pdf_glyph_outlines: true,
      pdf_glyph_outline_options: Some(Arc::new(common::PdfGlyphOutlineOptions {
        semantic_text_overlay: false,
        fill: Some(common::Fill::Gradient(gradient.clone())),
        outline_fill: Some(common::Fill::Gradient(gradient)),
        outline_stroke: Some(common::Stroke {
          width: Pt(2.0),
          cap: Some(common::StrokeCap::Flat),
          join: Some(common::StrokeJoin::Round),
          compound: Some(common::StrokeCompound::Single),
          alignment: Some(common::StrokeAlignment::Center),
          ..Default::default()
        }),
        ..Default::default()
      })),
      ..Default::default()
    };

    let pdf = String::from_utf8_lossy(
      &render(
        &text_document("Gradient", outlined_style(gradient.clone())),
        &uncompressed_options(),
      )
      .unwrap(),
    )
    .into_owned();
    assert_eq!(pdf.matches("/ShadingType 2").count(), 1, "{pdf}");
    assert_eq!(pdf.matches("/PatternType 2").count(), 1, "{pdf}");
    assert_eq!(pdf.matches("/Pattern cs/P0 scn").count(), 2, "{pdf}");
    assert!(pdf.contains("/ca 0.5019608"), "{pdf}");
    assert!(!pdf.contains("/SMask"), "{pdf}");
    assert!(!pdf.contains("/Sh0 sh"), "{pdf}");
    let content = object_body(&pdf, 4);
    let tokens = content.split_ascii_whitespace().collect::<Vec<_>>();
    assert!(tokens.contains(&"f*"), "{content}");
    assert!(tokens.contains(&"f"), "{content}");
    assert!(!tokens.contains(&"S"), "{content}");

    let mut variable_alpha = gradient;
    variable_alpha.stops[1].color.a = 64;
    let pdf = String::from_utf8_lossy(
      &render(
        &text_document("Gradient", outlined_style(variable_alpha)),
        &uncompressed_options(),
      )
      .unwrap(),
    )
    .into_owned();
    assert_eq!(pdf.matches("/ShadingType 2").count(), 2, "{pdf}");
    assert_eq!(pdf.matches("/PatternType 2").count(), 1, "{pdf}");
    assert_eq!(pdf.matches("/GSM0 gs").count(), 2, "{pdf}");
    assert_eq!(pdf.matches("/SMask").count(), 1, "{pdf}");
    assert!(pdf.contains("/S/Luminosity"), "{pdf}");
    assert!(pdf.contains("/CS/DeviceGray"), "{pdf}");
    assert!(pdf.contains("/ColorSpace/DeviceGray"), "{pdf}");
    assert!(pdf.contains("/BC[0]"), "{pdf}");
    assert!(pdf.contains("/BBox[0 0 612 792]"), "{pdf}");
    assert!(pdf.contains("/ExtGState<</GSM0"), "{pdf}");
    assert!(!pdf.contains("/ca 0.5019608"), "{pdf}");
  }

  #[test]
  fn direct_writer_expands_shape_gradient_strokes_for_raw_and_prepared_pages() {
    let gradient = common::GradientFill {
      stops: vec![
        common::GradientStop {
          position: 0.0,
          color: color(31, 73, 125, 255),
          scheme: None,
        },
        common::GradientStop {
          position: 0.5,
          color: color(99, 37, 35, 255),
          scheme: None,
        },
        common::GradientStop {
          position: 1.0,
          color: color(247, 150, 70, 255),
          scheme: None,
        },
      ],
      angle_degrees: Some(90.0),
      definition_bounds: Some(common::Rect {
        origin: common::Point {
          x: Pt(100.0),
          y: Pt(100.0),
        },
        size: Size {
          width: Pt(120.0),
          height: Pt(60.0),
        },
      }),
      ..Default::default()
    };
    let gradient_path = || {
      common::DisplayItem::Path(common::PathItem {
        bounds: common::Rect {
          origin: common::Point {
            x: Pt(100.0),
            y: Pt(100.0),
          },
          size: Size {
            width: Pt(120.0),
            height: Pt(60.0),
          },
        },
        commands: vec![
          common::PathCommand::MoveTo(common::Point {
            x: Pt(100.0),
            y: Pt(100.0),
          }),
          common::PathCommand::LineTo(common::Point {
            x: Pt(220.0),
            y: Pt(100.0),
          }),
          common::PathCommand::LineTo(common::Point {
            x: Pt(220.0),
            y: Pt(160.0),
          }),
          common::PathCommand::LineTo(common::Point {
            x: Pt(100.0),
            y: Pt(160.0),
          }),
          common::PathCommand::Close,
        ],
        closed: true,
        fill: common::Fill::None,
        stroke: Some(common::Stroke {
          width: Pt(10.0),
          preset_dash: Some(common::StrokeDashPreset::Solid),
          cap: Some(common::StrokeCap::Flat),
          join: Some(common::StrokeJoin::Round),
          compound: Some(common::StrokeCompound::Single),
          alignment: Some(common::StrokeAlignment::Center),
          gradient: Some(gradient.clone()),
          ..Default::default()
        }),
        ..Default::default()
      })
    };

    let mut raw = blank_document(&[(320.0, 260.0)]);
    raw.engine_kind = common::LayoutEngineKind::Pptx;
    raw.pages[0].items.push(gradient_path());

    let mut prepared = raw.clone();
    prepared.pages[0]
      .items
      .push(common::DisplayItem::LinkArea(common::LinkArea {
        bounds: common::Rect {
          origin: common::Point {
            x: Pt(1.0),
            y: Pt(1.0),
          },
          size: Size {
            width: Pt(1.0),
            height: Pt(1.0),
          },
        },
        target: "https://example.test/".into(),
      }));

    for document in [&raw, &prepared] {
      let pdf =
        String::from_utf8_lossy(&render(document, &uncompressed_options()).unwrap()).into_owned();
      assert_eq!(pdf.matches("/ShadingType 2").count(), 1, "{pdf}");
      assert_eq!(pdf.matches("/PatternType 2").count(), 1, "{pdf}");
      assert!(pdf.contains("/Pattern cs/P0 scn"), "{pdf}");
      assert!(!pdf.contains("/Pattern CS"), "{pdf}");
      let content = object_body(&pdf, 4);
      let tokens = content.split_ascii_whitespace().collect::<Vec<_>>();
      assert!(tokens.contains(&"f"), "{content}");
      assert!(!tokens.contains(&"S"), "{content}");
      assert!(content.contains("95"), "{content}");
      assert!(content.contains("225"), "{content}");
    }
  }

  #[test]
  fn direct_writer_shape_gradient_stroke_freezes_alpha_and_typed_boundaries() {
    let gradient = |alphas: [u8; 2]| common::GradientFill {
      stops: vec![
        common::GradientStop {
          position: 0.0,
          color: color(255, 0, 0, alphas[0]),
          scheme: None,
        },
        common::GradientStop {
          position: 1.0,
          color: color(0, 0, 255, alphas[1]),
          scheme: None,
        },
      ],
      line: Some((
        common::Point {
          x: Pt(20.0),
          y: Pt(50.0),
        },
        common::Point {
          x: Pt(180.0),
          y: Pt(50.0),
        },
      )),
      ..Default::default()
    };
    let path_document = |gradient| {
      let mut document = blank_document(&[(200.0, 100.0)]);
      document.pages[0]
        .items
        .push(common::DisplayItem::Path(common::PathItem {
          bounds: common::Rect {
            origin: common::Point {
              x: Pt(20.0),
              y: Pt(50.0),
            },
            size: Size {
              width: Pt(160.0),
              height: Pt(0.0),
            },
          },
          points: vec![
            common::Point {
              x: Pt(20.0),
              y: Pt(50.0),
            },
            common::Point {
              x: Pt(180.0),
              y: Pt(50.0),
            },
          ],
          stroke: Some(common::Stroke {
            width: Pt(4.0),
            cap: Some(common::StrokeCap::Flat),
            join: Some(common::StrokeJoin::Round),
            gradient: Some(gradient),
            ..Default::default()
          }),
          ..Default::default()
        }));
      document
    };

    let variable = String::from_utf8_lossy(
      &render(
        &path_document(gradient([u8::MAX, 64])),
        &uncompressed_options(),
      )
      .unwrap(),
    )
    .into_owned();
    assert_eq!(variable.matches("/ShadingType 2").count(), 2, "{variable}");
    assert!(variable.contains("/SMask"), "{variable}");
    assert!(variable.contains("/Pattern cs/P0 scn"), "{variable}");

    let transparent = String::from_utf8_lossy(
      &render(&path_document(gradient([0, 0])), &uncompressed_options()).unwrap(),
    )
    .into_owned();
    assert!(!transparent.contains("/PatternType"), "{transparent}");
    assert!(!transparent.contains("/ShadingType"), "{transparent}");
    assert!(!transparent.contains("/SMask"), "{transparent}");

    let base = common::Stroke {
      width: Pt(2.0),
      gradient: Some(gradient([255, 255])),
      ..Default::default()
    };
    assert!(ensure_stroke_supported(&base, StrokeUse::Path).is_ok());

    let mut compound = base.clone();
    compound.compound = Some(common::StrokeCompound::ThickThin);
    assert!(matches!(
      ensure_path_stroke_supported(&compound, true),
      Err(PdfError::DirectWriterUnsupported {
        feature: "compound gradient strokes"
      })
    ));

    let mut marker = base.clone();
    marker.tail_end = Some(common::StrokeEnd {
      kind: common::StrokeEndKind::Triangle,
      width: common::StrokeEndSize::Medium,
      length: common::StrokeEndSize::Medium,
    });
    assert!(matches!(
      ensure_stroke_supported(&marker, StrokeUse::Path),
      Err(PdfError::DirectWriterUnsupported {
        feature: "gradient stroke endpoint markers"
      })
    ));

    let mut hairline = base.clone();
    hairline.width = Pt(0.0);
    assert!(matches!(
      ensure_stroke_supported(&hairline, StrokeUse::Path),
      Err(PdfError::DirectWriterUnsupported {
        feature: "zero-width gradient strokes"
      })
    ));

    assert!(matches!(
      ensure_stroke_supported(&base, StrokeUse::Line),
      Err(PdfError::DirectWriterUnsupported {
        feature: "gradient strokes on non-path display items"
      })
    ));
  }

  #[test]
  fn direct_writer_expands_closed_solid_compound_paths_for_raw_and_prepared_pages() {
    let compound_path = |compound, commands: bool, alpha| {
      let points = [
        common::Point {
          x: Pt(100.0),
          y: Pt(100.0),
        },
        common::Point {
          x: Pt(220.0),
          y: Pt(100.0),
        },
        common::Point {
          x: Pt(220.0),
          y: Pt(160.0),
        },
        common::Point {
          x: Pt(100.0),
          y: Pt(160.0),
        },
      ];
      common::DisplayItem::Path(common::PathItem {
        bounds: common::Rect {
          origin: points[0],
          size: Size {
            width: Pt(120.0),
            height: Pt(60.0),
          },
        },
        points: if commands {
          Vec::new()
        } else {
          points.to_vec()
        },
        commands: if commands {
          vec![
            common::PathCommand::MoveTo(points[0]),
            common::PathCommand::LineTo(points[1]),
            common::PathCommand::LineTo(points[2]),
            common::PathCommand::LineTo(points[3]),
            common::PathCommand::Close,
          ]
        } else {
          Vec::new()
        },
        closed: true,
        fill: common::Fill::None,
        stroke: Some(common::Stroke {
          width: Pt(6.0),
          color: color(57, 82, 116, alpha),
          preset_dash: Some(common::StrokeDashPreset::Solid),
          cap: Some(common::StrokeCap::Flat),
          join: Some(common::StrokeJoin::Round),
          compound: Some(compound),
          alignment: Some(common::StrokeAlignment::Center),
          ..Default::default()
        }),
      })
    };

    for (compound, commands, boundary_count) in [
      (common::StrokeCompound::Double, true, 4),
      (common::StrokeCompound::ThickThin, false, 4),
      (common::StrokeCompound::ThinThick, true, 4),
      (common::StrokeCompound::Triple, false, 6),
    ] {
      let mut raw = blank_document(&[(320.0, 260.0)]);
      raw.pages[0]
        .items
        .push(compound_path(compound, commands, 192));

      let mut prepared = raw.clone();
      prepared.pages[0]
        .items
        .push(common::DisplayItem::LinkArea(common::LinkArea {
          bounds: common::Rect {
            origin: common::Point {
              x: Pt(1.0),
              y: Pt(1.0),
            },
            size: Size {
              width: Pt(1.0),
              height: Pt(1.0),
            },
          },
          target: "https://example.test/".into(),
        }));

      for document in [&raw, &prepared] {
        let pdf =
          String::from_utf8_lossy(&render(document, &uncompressed_options()).unwrap()).into_owned();
        let content = object_body(&pdf, 4);
        let tokens = content.split_ascii_whitespace().collect::<Vec<_>>();
        assert_eq!(
          tokens.iter().filter(|&&token| token == "h").count(),
          boundary_count
        );
        assert_eq!(tokens.iter().filter(|&&token| token == "f").count(), 1);
        assert!(!tokens.contains(&"S"), "{compound:?}: {content}");
        assert!(!tokens.contains(&"B"), "{compound:?}: {content}");
        assert!(pdf.contains("/ca 0.7529412"), "{compound:?}: {pdf}");
      }
    }

    let mut transparent = blank_document(&[(320.0, 260.0)]);
    transparent.pages[0]
      .items
      .push(compound_path(common::StrokeCompound::Double, true, 0));
    let pdf =
      String::from_utf8_lossy(&render(&transparent, &uncompressed_options()).unwrap()).into_owned();
    assert!(
      !object_body(&pdf, 4)
        .split_ascii_whitespace()
        .any(|token| token == "f")
    );
    assert!(!pdf.contains("/ca 0"), "{pdf}");
  }

  #[test]
  fn direct_writer_keeps_unsettled_compound_path_states_typed() {
    let base = common::Stroke {
      width: Pt(6.0),
      compound: Some(common::StrokeCompound::Double),
      alignment: Some(common::StrokeAlignment::Center),
      ..Default::default()
    };
    assert!(ensure_path_stroke_supported(&base, true).is_ok());

    let mut gradient = base.clone();
    gradient.gradient = Some(common::GradientFill {
      stops: vec![
        common::GradientStop {
          position: 0.0,
          color: color(255, 0, 0, 255),
          scheme: None,
        },
        common::GradientStop {
          position: 1.0,
          color: color(0, 0, 255, 255),
          scheme: None,
        },
      ],
      ..Default::default()
    });
    assert!(matches!(
      ensure_path_stroke_supported(&gradient, true),
      Err(PdfError::DirectWriterUnsupported {
        feature: "compound gradient strokes"
      })
    ));

    let mut dashed = base.clone();
    dashed.preset_dash = Some(common::StrokeDashPreset::Dash);
    assert!(matches!(
      ensure_path_stroke_supported(&dashed, true),
      Err(PdfError::DirectWriterUnsupported {
        feature: "dashed compound strokes"
      })
    ));
    assert!(matches!(
      ensure_path_stroke_supported(&base, false),
      Err(PdfError::DirectWriterUnsupported {
        feature: "open compound strokes"
      })
    ));

    let mut marker = base.clone();
    marker.tail_end = Some(common::StrokeEnd {
      kind: common::StrokeEndKind::Triangle,
      width: common::StrokeEndSize::Medium,
      length: common::StrokeEndSize::Medium,
    });
    assert!(matches!(
      ensure_path_stroke_supported(&marker, true),
      Err(PdfError::DirectWriterUnsupported {
        feature: "compound stroke endpoint markers"
      })
    ));

    let mut hairline = base.clone();
    hairline.width = Pt(0.0);
    assert!(matches!(
      ensure_path_stroke_supported(&hairline, true),
      Err(PdfError::DirectWriterUnsupported {
        feature: "zero-width compound strokes"
      })
    ));

    let mut inside = base.clone();
    inside.alignment = Some(common::StrokeAlignment::Inside);
    assert!(matches!(
      ensure_path_stroke_supported(&inside, true),
      Err(PdfError::DirectWriterUnsupported {
        feature: "inside-aligned strokes"
      })
    ));
    assert!(matches!(
      ensure_stroke_supported(&base, StrokeUse::Rectangle),
      Err(PdfError::DirectWriterUnsupported {
        feature: "compound strokes"
      })
    ));
  }

  #[test]
  fn direct_writer_expands_compound_glyph_strokes_as_filled_band_geometry() {
    let outlined_style = |compound, preset_dash| common::TextStyle {
      font_family: Some("Liberation Serif".into()),
      font_size: Pt(12.0),
      color: color(0, 0, 0, u8::MAX),
      pdf_glyph_outlines: true,
      pdf_glyph_outline_options: Some(Arc::new(common::PdfGlyphOutlineOptions {
        semantic_text_overlay: false,
        fill: Some(common::Fill::None),
        outline_fill: Some(common::Fill::Solid(color(57, 82, 116, 192))),
        outline_stroke: Some(common::Stroke {
          width: Pt(2.0),
          cap: Some(common::StrokeCap::Round),
          join: Some(common::StrokeJoin::Miter { limit: Some(0.0) }),
          compound: Some(compound),
          alignment: Some(common::StrokeAlignment::Center),
          preset_dash,
          ..Default::default()
        }),
        ..Default::default()
      })),
      ..Default::default()
    };

    for compound in [
      common::StrokeCompound::Double,
      common::StrokeCompound::ThickThin,
      common::StrokeCompound::ThinThick,
      common::StrokeCompound::Triple,
    ] {
      let pdf = String::from_utf8_lossy(
        &render(
          &text_document("Band", outlined_style(compound, None)),
          &uncompressed_options(),
        )
        .unwrap(),
      )
      .into_owned();
      let content = object_body(&pdf, 4);
      let tokens = content.split_ascii_whitespace().collect::<Vec<_>>();
      assert!(tokens.contains(&"f"), "{compound:?}: {content}");
      assert!(!tokens.contains(&"S"), "{compound:?}: {content}");
      assert!(!tokens.contains(&"B"), "{compound:?}: {content}");
      assert!(pdf.contains("/ca 0.7529412"), "{compound:?}: {pdf}");
    }

    for compound in [
      common::StrokeCompound::Double,
      common::StrokeCompound::Triple,
    ] {
      let pdf = String::from_utf8_lossy(
        &render(
          &text_document(
            "Dash",
            outlined_style(compound, Some(common::StrokeDashPreset::Dash)),
          ),
          &uncompressed_options(),
        )
        .unwrap(),
      )
      .into_owned();
      let tokens = object_body(&pdf, 4)
        .split_ascii_whitespace()
        .collect::<Vec<_>>();
      assert!(tokens.contains(&"f"), "{compound:?}: {pdf}");
      assert!(!tokens.contains(&"S"), "{compound:?}: {pdf}");
    }

    for compound in [
      common::StrokeCompound::ThickThin,
      common::StrokeCompound::ThinThick,
    ] {
      assert!(matches!(
        render(
          &text_document(
            "Dash",
            outlined_style(compound, Some(common::StrokeDashPreset::Dash)),
          ),
          &uncompressed_options(),
        ),
        Err(PdfError::DirectWriterUnsupported {
          feature: "dashed asymmetric compound outlined glyph strokes"
        })
      ));
    }
  }

  #[test]
  fn direct_writer_keeps_semantic_only_text_under_an_isolated_nonpainting_clip() {
    let semantic_style = common::TextStyle {
      font_family: Some("Liberation Serif".into()),
      font_size: Pt(12.0),
      semantic_only: true,
      color: color(9, 23, 47, 0),
      ..Default::default()
    };
    let semantic = String::from_utf8_lossy(
      &render(
        &text_document("searchable carrier", semantic_style.clone()),
        &uncompressed_options(),
      )
      .unwrap(),
    )
    .into_owned();
    let tokens = semantic.split_ascii_whitespace().collect::<Vec<_>>();
    let clip = tokens
      .windows(7)
      .position(|window| {
        window[0] == "-10000"
          && window[1] == "-10000"
          && window[2].parse::<f32>().ok() == Some(0.001)
          && window[3].parse::<f32>().ok() == Some(0.001)
          && window[4..] == ["re", "W", "n"]
      })
      .expect("semantic-only nonpainting clip");
    let text = tokens[clip + 7..]
      .iter()
      .position(|token| *token == "BT")
      .map(|index| index + clip + 7)
      .expect("semantic-only text object");
    let restore = tokens[text + 1..]
      .iter()
      .position(|token| *token == "Q")
      .map(|index| index + text + 1)
      .expect("semantic-only clip restore");

    assert!(clip < text && text < restore, "{semantic}");
    assert!(semantic.contains("/Subtype/Type0"), "{semantic}");
    assert!(semantic.contains("/ToUnicode"), "{semantic}");
    assert!(!tokens.windows(2).any(|window| window == ["3", "Tr"]));
    assert!(
      semantic.contains(&format!(
        "{} {} {} rg",
        f32::from(9_u8) / f32::from(u8::MAX),
        f32::from(23_u8) / f32::from(u8::MAX),
        f32::from(47_u8) / f32::from(u8::MAX),
      )),
      "{semantic}"
    );

    let ordinary = String::from_utf8_lossy(
      &render(
        &text_document(
          "ordinary counterexample",
          common::TextStyle {
            semantic_only: false,
            color: color(9, 23, 47, u8::MAX),
            ..semantic_style.clone()
          },
        ),
        &uncompressed_options(),
      )
      .unwrap(),
    )
    .into_owned();
    assert!(!ordinary.contains("-10000 -10000"), "{ordinary}");

    let rotated = String::from_utf8_lossy(
      &render(
        &text_document(
          "independent rotation",
          common::TextStyle {
            rotation_degrees: 15.0,
            ..semantic_style.clone()
          },
        ),
        &uncompressed_options(),
      )
      .unwrap(),
    )
    .into_owned();
    let rotated_tokens = rotated.split_ascii_whitespace().collect::<Vec<_>>();
    let semantic_clip = rotated_tokens
      .windows(7)
      .position(|window| window[0] == "-10000" && window[4..] == ["re", "W", "n"])
      .expect("semantic-only nonpainting clip");
    let rotation = rotated_tokens
      .iter()
      .enumerate()
      .filter(|(_, token)| **token == "cm")
      .nth(1)
      .map(|(index, _)| index)
      .expect("text rotation matrix");
    let text = rotated_tokens
      .iter()
      .position(|token| *token == "BT")
      .expect("semantic text object");
    assert!(semantic_clip < rotation && rotation < text, "{rotated}");
    let matrices = content_matrices(&rotated);
    assert_eq!(matrices.len(), 2, "{rotated}");
    let angle = 15.0_f32.to_radians();
    assert_matrix_near(
      [
        matrices[1][0],
        matrices[1][1],
        matrices[1][2],
        matrices[1][3],
        0.0,
        0.0,
      ],
      [
        angle.cos(),
        angle.sin(),
        -angle.sin(),
        angle.cos(),
        0.0,
        0.0,
      ],
    );

    let invisible_ordinary = text_document(
      "independent opacity",
      common::TextStyle {
        semantic_only: false,
        ..semantic_style
      },
    );
    let invisible =
      String::from_utf8_lossy(&render(&invisible_ordinary, &uncompressed_options()).unwrap())
        .into_owned();
    assert!(!invisible.contains("BT"), "{invisible}");
    assert!(!invisible.contains("/Subtype/Type0"), "{invisible}");
    assert!(!invisible.contains("-10000 -10000"), "{invisible}");
  }

  #[test]
  fn direct_writer_scales_horizontal_text_space_about_the_run_anchor() {
    for horizontal_scale in [0.33, 0.75, 1.2, 1.5] {
      let document = text_document(
        "scaled text",
        common::TextStyle {
          font_family: Some("Liberation Serif".into()),
          font_size: Pt(12.0),
          horizontal_scale: Some(horizontal_scale),
          color: color(0, 0, 0, u8::MAX),
          ..Default::default()
        },
      );

      let bytes = render(&document, &uncompressed_options()).unwrap();
      let pdf = String::from_utf8_lossy(&bytes);
      let tokens = pdf.split_ascii_whitespace().collect::<Vec<_>>();
      let matrix = tokens
        .windows(7)
        .find_map(|window| {
          window[6].starts_with("Tm").then(|| {
            let mut matrix = [0.0; 6];
            for (index, token) in window[..6].iter().enumerate() {
              matrix[index] = token.parse::<f32>().unwrap();
            }
            matrix
          })
        })
        .expect("scaled text matrix");

      assert_matrix_near(matrix, [horizontal_scale, 0.0, 0.0, -1.0, 72.0, matrix[5]]);
      assert!(!tokens.contains(&"Tz"), "{pdf}");
    }

    assert!((scaled_text_x(72.0, 10.0, 0.33).unwrap() - 75.3).abs() <= 1.0e-5);
    assert!((scaled_text_x(72.0, 10.0, 1.5).unwrap() - 87.0).abs() <= 1.0e-5);
  }

  #[test]
  fn direct_writer_restores_scaled_visible_portion_and_definition_widths() {
    let measure = |horizontal_scale: f32, character_spacing_pt: f32| {
      let document = text_document(
        "abcdef",
        common::TextStyle {
          font_family: Some("Liberation Serif".into()),
          font_size: Pt(12.0),
          horizontal_scale: Some(horizontal_scale),
          character_spacing: Pt(character_spacing_pt),
          pdf_glyph_outlines: true,
          pdf_glyph_outline_options: Some(Arc::new(common::PdfGlyphOutlineOptions {
            definition_trailing_advance: Pt(9.0),
            ..Default::default()
          })),
          color: color(0, 0, 0, u8::MAX),
          ..Default::default()
        },
      );
      let options = uncompressed_options();
      let paint = super::super::paint::prepare_for_direct(&document, &options);
      let super::super::paint::PaintItem::Text(text) = &paint.pages[0].items[0] else {
        unreachable!();
      };
      assert_eq!(text.portions.len(), 1);
      let portion = &text.portions[0];
      let bounds = outlined_glyph_paint_bounds(
        text,
        portion,
        12.0,
        text.item.style.pdf_glyph_outline_options.as_ref(),
      )
      .unwrap();
      (portion.width_pt, bounds.size.width.0)
    };

    let (natural_width, natural_definition_width) = measure(1.0, 0.0);
    assert!((natural_definition_width - natural_width - 9.0).abs() <= 1.0e-4);
    for horizontal_scale in [0.75, 1.25, 1.5, 2.0, 3.0, 6.0] {
      let (visible_width, definition_width) = measure(horizontal_scale, 0.0);
      assert!((visible_width - natural_width * horizontal_scale).abs() <= 1.0e-4);
      assert!((definition_width - visible_width - 9.0).abs() <= 1.0e-4);
    }

    let (scaled_spaced_width, scaled_spaced_definition_width) = measure(1.5, 2.0);
    assert!((scaled_spaced_width - (natural_width * 1.5 + 12.0)).abs() <= 1.0e-4);
    assert!((scaled_spaced_definition_width - scaled_spaced_width - 9.0).abs() <= 1.0e-4);
  }

  #[test]
  fn direct_writer_scopes_word_text_advance_definition_width_without_one_em_floor() {
    let measure = |definition_width_basis| {
      let document = text_document(
        "a",
        common::TextStyle {
          font_family: Some("Liberation Serif".into()),
          font_size: Pt(48.0),
          pdf_glyph_outlines: true,
          pdf_glyph_outline_options: Some(Arc::new(common::PdfGlyphOutlineOptions {
            definition_width_basis,
            definition_trailing_advance: Pt(9.0),
            ..Default::default()
          })),
          color: color(0, 0, 0, u8::MAX),
          ..Default::default()
        },
      );
      let options = uncompressed_options();
      let paint = super::super::paint::prepare_for_direct(&document, &options);
      let super::super::paint::PaintItem::Text(text) = &paint.pages[0].items[0] else {
        unreachable!();
      };
      let portion = &text.portions[0];
      let bounds = outlined_glyph_paint_bounds(
        text,
        portion,
        48.0,
        text.item.style.pdf_glyph_outline_options.as_ref(),
      )
      .unwrap();
      (portion.width_pt, bounds.size.width.0)
    };

    let (text_advance, word_width) = measure(common::PdfGlyphDefinitionWidthBasis::TextAdvance);
    assert!(text_advance < 48.0);
    assert!((word_width - text_advance - 9.0).abs() <= 1.0e-4);

    let (_, drawingml_width) = measure(common::PdfGlyphDefinitionWidthBasis::AtLeastFontSize);
    assert!((drawingml_width - 57.0).abs() <= 1.0e-4);
  }

  #[test]
  fn direct_writer_rejects_invalid_horizontal_and_independent_vertical_text_scaling() {
    for horizontal_scale in [0.0, -1.0, f32::NAN, f32::INFINITY] {
      let document = text_document(
        "invalid scale",
        common::TextStyle {
          font_family: Some("Liberation Serif".into()),
          font_size: Pt(12.0),
          horizontal_scale: Some(horizontal_scale),
          color: color(0, 0, 0, u8::MAX),
          ..Default::default()
        },
      );
      assert!(matches!(
        render(&document, &uncompressed_options()),
        Err(PdfError::Writer(message))
          if message == "text horizontal scale must be finite and positive"
      ));
    }

    let legacy_vertical = text_document(
      "independent vertical scale",
      common::TextStyle {
        font_family: Some("Arial".into()),
        font_size: Pt(11.0),
        bold: true,
        color: color(0, 0, 0, u8::MAX),
        ..Default::default()
      },
    );
    assert!(matches!(
      render(&legacy_vertical, &uncompressed_options()),
      Err(PdfError::DirectWriterUnsupported {
        feature: "vertically scaled text painting"
      })
    ));
  }

  #[test]
  fn direct_writer_matches_office_fixed_output_by_flattening_form_values() {
    let mut document = text_document(
      "12",
      common::TextStyle {
        font_family: Some("Liberation Serif".into()),
        font_size: Pt(11.0),
        color: color(0, 0, 0, u8::MAX),
        ..Default::default()
      },
    );
    let common::DisplayItem::Text(text) = &mut document.pages[0].items[0] else {
      unreachable!();
    };
    text.dynamic_field = Some(common::DynamicField::Page {
      number_format: common::FieldNumberFormat::Decimal,
    });
    text.form_widget_id = Some(7);
    document.form_widgets.push(common::FormWidget {
      id: 7,
      kind: common::FormWidgetKind::Text,
      entries: Vec::new(),
    });

    let flattened = render(&document, &uncompressed_options()).unwrap();
    let flattened = String::from_utf8_lossy(&flattened);
    assert!(flattened.contains("/Subtype/Type0"));
    assert!(!flattened.contains("/Subtype/Widget"));

    let mut widgets = uncompressed_options();
    widgets.forms.export_form_fields = true;
    let flattened = render(&document, &widgets).unwrap();
    let flattened = String::from_utf8_lossy(&flattened);
    assert!(flattened.contains("/Subtype/Type0"));
    assert!(!flattened.contains("/Subtype/Widget"));
    assert!(!flattened.contains("/AcroForm"));
  }

  #[test]
  fn direct_writer_splits_actual_text_at_exact_multi_glyph_cluster_boundaries() {
    let glyph = |text_range| super::super::paint::PaintGlyph {
      glyph_id: 1,
      text_range,
      x_advance: 1.0,
      x_offset: 0.0,
      y_offset: 0.0,
      y_advance: 0.0,
      bounds_em: None,
    };
    let glyphs = vec![
      glyph(0..1),
      glyph(1..4),
      glyph(1..4),
      glyph(4..5),
      glyph(5..6),
      glyph(5..6),
      glyph(5..6),
    ];

    let first = next_glyph_segment(&glyphs, 0);
    assert_eq!(first.end, 1);
    assert_eq!(first.actual_text_range, None);
    let second = next_glyph_segment(&glyphs, first.end);
    assert_eq!(second.end, 3);
    assert_eq!(second.actual_text_range, Some(1..4));
    let third = next_glyph_segment(&glyphs, second.end);
    assert_eq!(third.end, 4);
    assert_eq!(third.actual_text_range, None);
    let fourth = next_glyph_segment(&glyphs, third.end);
    assert_eq!(fourth.end, glyphs.len());
    assert_eq!(fourth.actual_text_range, Some(5..6));

    let mut content = Content::with_settings(Settings { pretty: false });
    begin_actual_text(&mut content, "ffi");
    content.end_marked_content();
    let content = String::from_utf8(content.finish().into_vec()).unwrap();
    assert_eq!(content.matches("/ActualText").count(), 1);
    assert!(content.starts_with("/Span<</ActualText(ffi)>>BDC"));
    assert!(content.ends_with("EMC"));
  }

  #[test]
  fn direct_writer_uses_actual_text_only_for_contextual_cid_mapping_conflicts() {
    fn forced_conflict(source: &str) -> String {
      let document = text_document(
        source,
        common::TextStyle {
          font_family: Some("Liberation Serif".into()),
          font_size: Pt(12.0),
          color: color(0, 0, 0, u8::MAX),
          ..Default::default()
        },
      );
      let mut options = uncompressed_options();
      options.standards = vec![PdfStandard::Pdf14];
      let mut paint = super::super::paint::prepare_for_direct(&document, &options);
      let super::super::paint::PaintItem::Text(text) = &mut paint.pages[0].items[0] else {
        unreachable!();
      };
      let glyphs = &mut text.portions[0].glyphs.as_mut().unwrap()[0].glyphs;
      assert_eq!(glyphs.len(), 2);
      glyphs[1].glyph_id = glyphs[0].glyph_id;

      let selection = PageSelection::from_range(document.pages.len(), None).unwrap();
      let conformance = DirectConformance::from_options(&options).unwrap();
      String::from_utf8_lossy(
        &write_page_document(&document, &options, &selection, Some(&paint), conformance).unwrap(),
      )
      .into_owned()
    }

    let first_then_second = forced_conflict("ab");
    assert!(first_then_second.starts_with("%PDF-1.5"));
    assert_eq!(first_then_second.matches("/ActualText").count(), 1);
    assert!(
      first_then_second.contains("/ActualText(b)"),
      "{first_then_second}"
    );

    let second_then_first = forced_conflict("ba");
    assert!(second_then_first.starts_with("%PDF-1.5"));
    assert_eq!(second_then_first.matches("/ActualText").count(), 1);
    assert!(
      second_then_first.contains("/ActualText(a)"),
      "{second_then_first}"
    );

    let mut compatible_options = uncompressed_options();
    compatible_options.standards = vec![PdfStandard::Pdf14];
    let compatible = String::from_utf8_lossy(
      &render(
        &text_document(
          "aa",
          common::TextStyle {
            font_family: Some("Liberation Serif".into()),
            font_size: Pt(12.0),
            color: color(0, 0, 0, u8::MAX),
            ..Default::default()
          },
        ),
        &compatible_options,
      )
      .unwrap(),
    )
    .into_owned();
    assert!(compatible.starts_with("%PDF-1.4"));
    assert!(!compatible.contains("/ActualText"), "{compatible}");
  }

  #[test]
  fn direct_writer_accepts_the_source_backed_pdf_semantic_remapping_families() {
    let base_style = common::TextStyle {
      font_family: Some("Liberation Serif".into()),
      font_size: Pt(12.0),
      color: color(0, 0, 0, u8::MAX),
      ..Default::default()
    };
    let mut small_caps = base_style.clone();
    small_caps.small_caps = true;
    let mut uppercase = base_style.clone();
    uppercase.uppercase = true;
    let mut symbol = base_style.clone();
    symbol.font_family = Some("Symbol".into());
    symbol.explicit_symbol_character = true;

    for (source, style) in [
      ("small caps", small_caps),
      ("uppercase", uppercase),
      ("A\u{2011}B", base_style.clone()),
      ("☀\u{fe0e}", base_style),
      ("\u{f0b7}", symbol),
    ] {
      let pdf = render(&text_document(source, style), &uncompressed_options()).unwrap();
      assert!(String::from_utf8_lossy(&pdf).contains("/ToUnicode"));
    }
  }

  #[test]
  fn direct_writer_applies_profile_semantic_rules_after_legacy_symbol_remapping() {
    assert!(ensure_glyph_semantic_mapping_supported("\u{2022}", true, true).is_ok());
    assert!(ensure_glyph_semantic_mapping_supported("\u{f0b7}", false, false).is_ok());
    assert!(ensure_glyph_semantic_mapping_supported("\u{f0b7}", true, false).is_ok());
    assert!(matches!(
      ensure_glyph_semantic_mapping_supported("\u{f0b7}", true, true),
      Err(PdfError::DirectWriterUnsupported {
        feature: "private-use glyph semantics in PDF/A-3a"
      })
    ));
    assert!(ensure_glyph_semantic_mapping_supported("\0", false, false).is_ok());
    assert!(matches!(
      ensure_glyph_semantic_mapping_supported("\0", true, false),
      Err(PdfError::DirectWriterUnsupported {
        feature: "invalid glyph semantics in a Unicode-mapped PDF profile"
      })
    ));
  }

  #[test]
  fn direct_writer_raises_only_actual_text_pdf14_to_the_pdf15_feature_floor() {
    assert_eq!(actual_text_version_floor((1, 4), false), (1, 4));
    assert_eq!(actual_text_version_floor((1, 4), true), (1, 5));
    assert_eq!(actual_text_version_floor((1, 5), true), (1, 5));
    assert_eq!(actual_text_version_floor((2, 0), true), (2, 0));
  }

  #[test]
  fn direct_writer_isolates_source_backed_synthetic_bold_and_italic_state() {
    let synthesis = DirectTextSynthesis {
      bold: true,
      italic: true,
      color: super::super::paint::RgbColor {
        r: 12,
        g: 34,
        b: 56,
      },
      font_size_pt: 18.0,
      baseline_y: 96.0,
    };
    let mut content = Content::with_settings(Settings { pretty: false });
    synthesis.begin(&mut content);
    content
      .begin_text()
      .set_text_rendering_mode(synthesis.rendering_mode());
    content.end_text();
    synthesis.end(&mut content);
    let content = String::from_utf8(content.finish().into_vec()).unwrap();

    assert!(content.starts_with("q"), "{content}");
    assert!(
      content.contains("0.047058824 0.13333334 0.21960784 RG"),
      "{content}"
    );
    assert!(content.contains("0.6 w"), "{content}");
    assert!(content.contains("1 0 -0.33333334 1 32 0 cm"), "{content}");
    assert!(content.contains("2 Tr"), "{content}");
    assert!(content.ends_with('Q'), "{content}");

    let ordinary = DirectTextSynthesis {
      bold: false,
      italic: false,
      color: super::super::paint::RgbColor {
        r: 12,
        g: 34,
        b: 56,
      },
      font_size_pt: 18.0,
      baseline_y: 96.0,
    };
    let mut ordinary_content = Content::with_settings(Settings { pretty: false });
    ordinary.begin(&mut ordinary_content);
    ordinary.end(&mut ordinary_content);
    assert!(ordinary_content.finish().into_vec().is_empty());
    assert_eq!(ordinary.rendering_mode(), TextRenderingMode::Fill);
  }

  #[test]
  fn direct_writer_applies_only_finite_positive_resolved_text_clips() {
    let text_style = || common::TextStyle {
      font_family: Some("Liberation Serif".into()),
      font_size: Pt(12.0),
      color: color(0, 0, 0, u8::MAX),
      ..Default::default()
    };
    let set_clip = |document: &mut common::LayoutDocument<'static>, clip| {
      let common::DisplayItem::Text(text) = &mut document.pages[0].items[0] else {
        unreachable!();
      };
      text.paint_clip = Some(clip);
    };

    let mut clipped = text_document("clipped text", text_style());
    set_clip(
      &mut clipped,
      common::Rect {
        origin: common::Point {
          x: Pt(80.0),
          y: Pt(60.0),
        },
        size: Size {
          width: Pt(10.0),
          height: Pt(20.0),
        },
      },
    );
    let clipped =
      String::from_utf8_lossy(&render(&clipped, &uncompressed_options()).unwrap()).into_owned();
    let clip = clipped.find("80 60 10 20 re\nW\nn").unwrap();
    let text = clipped[clip..].find("BT").unwrap() + clip;
    let restore = clipped[text..].find("\nQ").unwrap() + text;
    assert!(clip < text && text < restore, "{clipped}");

    let mut empty = text_document("not clipped", text_style());
    set_clip(
      &mut empty,
      common::Rect {
        origin: common::Point {
          x: Pt(80.0),
          y: Pt(60.0),
        },
        size: Size {
          width: Pt(0.0),
          height: Pt(20.0),
        },
      },
    );
    let empty =
      String::from_utf8_lossy(&render(&empty, &uncompressed_options()).unwrap()).into_owned();
    assert!(!empty.contains("80 60 0 20 re"), "{empty}");
    assert!(empty.contains("BT"), "{empty}");

    let mut invalid = text_document("invalid clip", text_style());
    set_clip(
      &mut invalid,
      common::Rect {
        origin: common::Point {
          x: Pt(f32::NAN),
          y: Pt(60.0),
        },
        size: Size {
          width: Pt(10.0),
          height: Pt(20.0),
        },
      },
    );
    assert!(matches!(
      render(&invalid, &uncompressed_options()),
      Err(PdfError::Writer(message)) if message.contains("text clip rectangle")
    ));
  }

  #[test]
  fn direct_writer_paints_resolved_underline_and_strikethrough_after_glyphs() {
    let document = text_document(
      "decorated",
      common::TextStyle {
        font_family: Some("Liberation Serif".into()),
        font_size: Pt(12.0),
        color: color(0, 0, 0, u8::MAX),
        underline: true,
        strikethrough: true,
        underline_color: Some(color(255, 0, 0, u8::MAX)),
        ..Default::default()
      },
    );

    let pdf =
      String::from_utf8_lossy(&render(&document, &uncompressed_options()).unwrap()).into_owned();
    let glyphs = pdf.find("ET").unwrap();
    let underline = pdf[glyphs..].find("1 0 0 RG").unwrap() + glyphs;
    let strikethrough = pdf[underline..].find("0 0 0 RG").unwrap() + underline;
    assert!(glyphs < underline && underline < strikethrough, "{pdf}");
    assert_eq!(pdf[glyphs..].matches("\nS").count(), 2, "{pdf}");

    let mut invalid_content = Content::with_settings(Settings { pretty: false });
    let invalid = super::super::paint::PaintStrokeLine {
      x1_pt: f32::NAN,
      y1_pt: 10.0,
      x2_pt: 20.0,
      y2_pt: 10.0,
      width_pt: 1.0,
      color: super::super::paint::RgbColor { r: 0, g: 0, b: 0 },
    };
    assert!(matches!(
      write_prepared_text_decoration(&mut invalid_content, &invalid),
      Err(PdfError::Writer(message)) if message.contains("text decoration")
    ));
    assert!(invalid_content.finish().into_vec().is_empty());
  }

  #[test]
  fn direct_writer_serializes_uri_link_annotations_from_text_and_link_areas() {
    let mut area_document = blank_document(&[(100.0, 100.0)]);
    area_document.pages[0]
      .items
      .push(common::DisplayItem::LinkArea(common::LinkArea {
        bounds: common::Rect {
          origin: common::Point {
            x: Pt(10.0),
            y: Pt(20.0),
          },
          size: Size {
            width: Pt(30.0),
            height: Pt(40.0),
          },
        },
        target: "https://example.test/report.docx?view=1".into(),
      }));
    let mut options = uncompressed_options();
    options.links.convert_office_targets_to_pdf_targets = true;
    let area_pdf = String::from_utf8_lossy(&render(&area_document, &options).unwrap()).into_owned();
    assert!(area_pdf.contains("/Subtype/Link"), "{area_pdf}");
    assert!(area_pdf.contains("/Rect[10 39.96 40 79.96]"), "{area_pdf}");
    assert!(area_pdf.contains("/Border[0 0 0]"), "{area_pdf}");
    assert!(area_pdf.contains("/F 4"), "{area_pdf}");
    assert!(area_pdf.contains("/Contents(https://example.test/report.docx?view=1)"));
    assert!(
      area_pdf.contains("/S/URI/URI(https://example.test/report.pdf?view=1)"),
      "{area_pdf}"
    );
    assert!(area_pdf.contains("/Annots["), "{area_pdf}");

    let mut text_document = text_document(
      "Linked text",
      common::TextStyle {
        font_family: Some("Liberation Serif".into()),
        font_size: Pt(12.0),
        color: color(0, 0, 0, u8::MAX),
        ..Default::default()
      },
    );
    let common::DisplayItem::Text(text) = &mut text_document.pages[0].items[0] else {
      unreachable!();
    };
    text.hyperlink_url = Some("https://example.test/text".into());
    let text_pdf =
      String::from_utf8_lossy(&render(&text_document, &uncompressed_options()).unwrap())
        .into_owned();
    assert!(text_pdf.contains("/Subtype/Link"), "{text_pdf}");
    assert!(text_pdf.contains("/Contents(Linked text)"), "{text_pdf}");
    assert!(
      text_pdf.contains("/S/URI/URI(https://example.test/text)"),
      "{text_pdf}"
    );
  }

  #[test]
  fn direct_writer_omits_removed_missing_and_invalid_link_annotations() {
    let link_area = |width| {
      common::DisplayItem::LinkArea(common::LinkArea {
        bounds: common::Rect {
          origin: common::Point {
            x: Pt(10.0),
            y: Pt(20.0),
          },
          size: Size {
            width: Pt(width),
            height: Pt(40.0),
          },
        },
        target: "https://example.test/report".into(),
      })
    };

    let mut removed = blank_document(&[(100.0, 100.0)]);
    removed.pages[0].items.push(link_area(30.0));
    let mut options = uncompressed_options();
    options.links.default_action = crate::PdfLinkDefaultAction::RemoveExternalLinks;
    let removed = String::from_utf8_lossy(&render(&removed, &options).unwrap()).into_owned();
    assert!(!removed.contains("/Subtype/Link"), "{removed}");
    assert!(!removed.contains("/Annots["), "{removed}");

    let mut invalid = blank_document(&[(100.0, 100.0)]);
    invalid.pages[0].items.push(link_area(0.0));
    invalid.pages[0].items.push(link_area(f32::NAN));
    let invalid =
      String::from_utf8_lossy(&render(&invalid, &uncompressed_options()).unwrap()).into_owned();
    assert!(!invalid.contains("/Subtype/Link"), "{invalid}");
    assert!(!invalid.contains("/Annots["), "{invalid}");
  }

  #[test]
  fn direct_writer_internal_note_links_follow_selected_output_pages() {
    let style = common::TextStyle {
      font_family: Some("Liberation Serif".into()),
      font_size: Pt(12.0),
      color: color(0, 0, 0, u8::MAX),
      ..Default::default()
    };
    let mut reference = text_document("reference", style.clone()).pages[0]
      .items
      .remove(0);
    let common::DisplayItem::Text(reference_text) = &mut reference else {
      unreachable!();
    };
    reference_text.hyperlink_url = Some("ooxmlsdk-pdf:footnote-reference:7".into());
    let mut backlink = text_document("backlink", style).pages[0].items.remove(0);
    let common::DisplayItem::Text(backlink_text) = &mut backlink else {
      unreachable!();
    };
    backlink_text.origin = common::Point {
      x: Pt(96.0),
      y: Pt(120.0),
    };
    backlink_text.hyperlink_url = Some("ooxmlsdk-pdf:footnote-backlink:7".into());

    let mut document = blank_document(&[(612.0, 792.0), (612.0, 792.0)]);
    document.pages[0].items.push(reference);
    document.pages[1].items.push(backlink);
    let mut options = uncompressed_options();
    options.general.page_range = Some("2,1".to_string());
    let selected = String::from_utf8_lossy(&render(&document, &options).unwrap()).into_owned();
    assert_eq!(selected.matches("/Subtype/Link").count(), 2, "{selected}");
    assert!(selected.contains("/Dest[3 0 R/XYZ"), "{selected}");
    assert!(selected.contains("/Dest[5 0 R/XYZ"), "{selected}");
    assert!(!selected.contains("/S/URI"), "{selected}");

    options.general.page_range = Some("1".to_string());
    let missing = String::from_utf8_lossy(&render(&document, &options).unwrap()).into_owned();
    assert!(!missing.contains("/Subtype/Link"), "{missing}");
    assert!(!missing.contains("/Annots["), "{missing}");
  }

  #[test]
  fn direct_writer_paints_filled_and_stroked_endpoint_markers_after_a_shortened_shaft() {
    let marker = |kind| common::StrokeEnd {
      kind,
      width: common::StrokeEndSize::Medium,
      length: common::StrokeEndSize::Medium,
    };
    let stroke = common::Stroke {
      width: Pt(2.0),
      color: color(255, 0, 0, 128),
      head_end: Some(marker(common::StrokeEndKind::Triangle)),
      tail_end: Some(marker(common::StrokeEndKind::Arrow)),
      ..Default::default()
    };
    let mut document = blank_document(&[(100.0, 100.0)]);
    document.pages[0]
      .items
      .push(common::DisplayItem::Path(common::PathItem {
        bounds: common::Rect {
          origin: common::Point {
            x: Pt(10.0),
            y: Pt(50.0),
          },
          size: Size {
            width: Pt(80.0),
            height: Pt(0.0),
          },
        },
        points: vec![
          common::Point {
            x: Pt(10.0),
            y: Pt(50.0),
          },
          common::Point {
            x: Pt(90.0),
            y: Pt(50.0),
          },
        ],
        commands: Vec::new(),
        closed: false,
        fill: common::Fill::None,
        stroke: Some(stroke.clone()),
      }));

    let pdf =
      String::from_utf8_lossy(&render(&document, &uncompressed_options()).unwrap()).into_owned();
    assert!(pdf.contains("10 50 m 88 50 l\nS"), "{pdf}");
    assert!(pdf.contains("1 J"), "{pdf}");
    assert!(pdf.contains("2 w"), "{pdf}");
    assert!(pdf.contains("\nh\nf"), "{pdf}");
    assert!(pdf.contains("/CA 0.5019608"), "{pdf}");
    assert!(pdf.contains("/ca 0.5019608"), "{pdf}");

    let mut unresolved_line = blank_document(&[(100.0, 100.0)]);
    unresolved_line.pages[0]
      .items
      .push(common::DisplayItem::Line(common::LineItem {
        start: common::Point {
          x: Pt(10.0),
          y: Pt(50.0),
        },
        end: common::Point {
          x: Pt(90.0),
          y: Pt(50.0),
        },
        stroke,
        kind: common::LineKind::Stroke,
      }));
    assert!(matches!(
      render(&unresolved_line, &uncompressed_options()),
      Err(PdfError::DirectWriterUnsupported {
        feature: "stroke endpoint markers"
      })
    ));
  }

  #[test]
  fn direct_writer_reports_the_exact_prepared_glyphs_used_for_serialization() {
    let document = text_document(
      "aaaa",
      common::TextStyle {
        font_family: Some("Liberation Serif".into()),
        font_size: Pt(12.0),
        color: color(0, 0, 0, u8::MAX),
        ..Default::default()
      },
    );

    let output = render_with_diagnostics(&document, &uncompressed_options()).unwrap();
    assert_eq!(output.diagnostics.fonts.len(), 1);
    assert_eq!(output.diagnostics.pages.len(), 1);
    let text = &output.diagnostics.pages[0].text_runs[0];
    assert_eq!(text.text, "aaaa");
    assert_eq!(text.portions.len(), 1);
    assert!(text.portions[0].has_explicit_glyphs);
    assert_eq!(text.portions[0].glyph_runs.len(), 1);
    assert_eq!(text.portions[0].glyph_runs[0].glyphs.len(), 4);
    assert!(String::from_utf8_lossy(&output.pdf).contains("/Subtype/Type0"));
  }

  #[test]
  fn direct_writer_keeps_opaque_text_with_paint_metadata_as_pdf_text() {
    for outlined in [false, true] {
      let document = text_document(
        "first second",
        common::TextStyle {
          font_family: Some("Liberation Serif".into()),
          font_size: Pt(12.0),
          color: color(0, 0, 0, u8::MAX),
          pdf_glyph_outlines: outlined,
          pdf_glyph_outline_options: Some(Arc::new(common::PdfGlyphOutlineOptions {
            fill: Some(common::Fill::Solid(color(0, 0, 0, u8::MAX))),
            outline_fill: Some(common::Fill::None),
            ..Default::default()
          })),
          ..Default::default()
        },
      );
      let output = render_with_font_audit(&document, &uncompressed_options()).unwrap();
      let pdf = String::from_utf8_lossy(&output.pdf);
      assert_eq!(pdf.contains("/ToUnicode"), !outlined);
      assert_eq!(pdf.contains("/Subtype/Type0"), !outlined);
      assert_eq!(output.audit.painted_text_portion_count > 0, !outlined);
    }
  }

  #[test]
  fn direct_writer_font_audit_counts_serialized_text_instead_of_returning_an_empty_audit() {
    let document = text_document(
      "aaaa",
      common::TextStyle {
        font_family: Some("Liberation Serif".into()),
        font_size: Pt(12.0),
        color: color(0, 0, 0, u8::MAX),
        ..Default::default()
      },
    );

    let output = render_with_font_audit(&document, &uncompressed_options()).unwrap();
    assert_eq!(output.audit.fonts.len(), 1);
    assert_eq!(output.audit.text_portion_count, 1);
    assert_eq!(output.audit.painted_text_portion_count, 1);
    assert_eq!(output.audit.explicit_glyph_portion_count, 1);
    assert_eq!(output.audit.glyph_run_count, 1);
    assert_eq!(output.audit.glyph_count, 4);
    assert_eq!(output.audit.actual_text_cluster_count, 0);
    assert!(output.audit.issues.is_empty());
    assert!(String::from_utf8_lossy(&output.pdf).contains("/ToUnicode"));
  }

  #[test]
  fn direct_writer_tagged_text_closes_the_structure_and_parent_tree_indexes() {
    let document = text_document(
      "Tagged paragraph",
      common::TextStyle {
        font_family: Some("Liberation Serif".into()),
        font_size: Pt(12.0),
        color: color(0, 0, 0, u8::MAX),
        ..Default::default()
      },
    );
    let mut options = uncompressed_options();
    options.general.tagged_pdf = true;
    options.ui_language = Some("en-US".to_string());

    let bytes = render(&document, &options).unwrap();
    let pdf = String::from_utf8_lossy(&bytes);

    assert!(pdf.contains("/MarkInfo<</Marked true>>"), "{pdf}");
    assert!(pdf.contains("/StructTreeRoot"), "{pdf}");
    assert!(pdf.contains("/Type/StructTreeRoot"), "{pdf}");
    assert!(pdf.contains("/S/Document"), "{pdf}");
    assert!(pdf.contains("/S/Part"), "{pdf}");
    assert!(pdf.contains("/S/P"), "{pdf}");
    assert!(pdf.contains("/Span<</MCID 0>>BDC"), "{pdf}");
    assert!(pdf.contains("/StructParents 0"), "{pdf}");
    assert!(pdf.contains("/ParentTree"), "{pdf}");
    assert!(pdf.contains("/ParentTreeNextKey 1"), "{pdf}");
    assert_eq!(pdf.matches("/MCID 0").count(), 1);
  }

  #[test]
  fn direct_writer_tagged_no_fill_text_has_no_empty_mcid_or_font_resource() {
    let document = text_document(
      "Invisible due to no fill",
      common::TextStyle {
        font_family: Some("Liberation Serif".into()),
        font_size: Pt(12.0),
        color: color(255, 255, 255, 0),
        ..Default::default()
      },
    );
    let mut options = uncompressed_options();
    options.general.tagged_pdf = true;
    options.ui_language = Some("en-US".to_string());

    let bytes = render(&document, &options).unwrap();
    let pdf = String::from_utf8_lossy(&bytes);

    assert!(pdf.contains("/MarkInfo<</Marked true>>"), "{pdf}");
    assert!(pdf.contains("/S/Document"), "{pdf}");
    assert!(pdf.contains("/S/Part"), "{pdf}");
    assert!(!pdf.contains("/MCID"), "{pdf}");
    assert!(!pdf.contains("/StructParents"), "{pdf}");
    assert!(!pdf.contains("/ParentTree"), "{pdf}");
    assert!(!pdf.contains("/Subtype/Type0"), "{pdf}");
    assert!(!pdf.contains("BT"), "{pdf}");
  }

  #[test]
  fn direct_writer_tagged_layout_artifact_has_no_semantic_mcid_or_parent_index() {
    let mut document = blank_document(&[(612.0, 792.0)]);
    document.pages[0]
      .items
      .push(common::DisplayItem::Rect(common::RectItem {
        bounds: common::Rect {
          origin: common::Point {
            x: Pt(10.0),
            y: Pt(20.0),
          },
          size: Size {
            width: Pt(30.0),
            height: Pt(40.0),
          },
        },
        fill: common::Fill::Solid(color(255, 0, 0, u8::MAX)),
        stroke: None,
      }));
    let mut options = uncompressed_options();
    options.general.tagged_pdf = true;

    let bytes = render(&document, &options).unwrap();
    let pdf = String::from_utf8_lossy(&bytes);

    assert!(pdf.contains("/Artifact<</Type/Layout>>BDC"), "{pdf}");
    assert!(pdf.contains("/MarkInfo<</Marked true>>"), "{pdf}");
    assert!(pdf.contains("/S/Document"), "{pdf}");
    assert!(pdf.contains("/S/Part"), "{pdf}");
    assert!(!pdf.contains("/MCID"), "{pdf}");
    assert!(!pdf.contains("/StructParents"), "{pdf}");
    assert!(!pdf.contains("/ParentTree"), "{pdf}");
  }

  #[test]
  fn direct_writer_tagged_mcid_scope_resets_per_page_while_parent_keys_do_not() {
    let mut document = text_document(
      "Page one",
      common::TextStyle {
        font_family: Some("Liberation Serif".into()),
        font_size: Pt(12.0),
        color: color(0, 0, 0, u8::MAX),
        ..Default::default()
      },
    );
    let mut second_page = document.pages[0].clone();
    let common::DisplayItem::Text(text) = &mut second_page.items[0] else {
      unreachable!();
    };
    text.text = "Page two".into();
    document.pages.push(second_page);
    let mut options = uncompressed_options();
    options.general.tagged_pdf = true;

    let bytes = render(&document, &options).unwrap();
    let pdf = String::from_utf8_lossy(&bytes);

    assert_eq!(pdf.matches("/Span<</MCID 0>>BDC").count(), 2, "{pdf}");
    assert!(!pdf.contains("/MCID 1"), "{pdf}");
    assert!(pdf.contains("/StructParents 0"), "{pdf}");
    assert!(pdf.contains("/StructParents 1"), "{pdf}");
    assert!(pdf.contains("/ParentTreeNextKey 2"), "{pdf}");
  }

  #[test]
  fn direct_writer_keeps_mixed_shape_and_text_z_order_in_the_shared_paint_list() {
    let mut document = text_document(
      "aaaa",
      common::TextStyle {
        font_family: Some("Liberation Serif".into()),
        font_size: Pt(12.0),
        color: color(0, 0, 0, u8::MAX),
        ..Default::default()
      },
    );
    document.pages[0].items.insert(
      0,
      common::DisplayItem::Rect(common::RectItem {
        bounds: common::Rect {
          origin: common::Point {
            x: Pt(10.0),
            y: Pt(20.0),
          },
          size: Size {
            width: Pt(30.0),
            height: Pt(40.0),
          },
        },
        fill: common::Fill::Solid(color(255, 0, 0, u8::MAX)),
        stroke: None,
      }),
    );
    document.pages[0]
      .items
      .push(common::DisplayItem::Line(common::LineItem {
        start: common::Point {
          x: Pt(100.0),
          y: Pt(100.0),
        },
        end: common::Point {
          x: Pt(120.0),
          y: Pt(100.0),
        },
        stroke: common::Stroke {
          width: Pt(1.0),
          color: color(0, 0, 255, u8::MAX),
          ..Default::default()
        },
        kind: common::LineKind::Stroke,
      }));

    let bytes = render(&document, &uncompressed_options()).unwrap();
    let pdf = String::from_utf8_lossy(&bytes);
    let rect = pdf.find("10 20 30 40 re").unwrap();
    let text = pdf.find("BT").unwrap();
    let line_start = pdf.find("100 100 m").unwrap();
    let line_end = pdf[line_start..].find("120 100 l").unwrap() + line_start;
    let line_stroke = pdf[line_end..].find("\nS").unwrap() + line_end;
    assert!(rect < text && text < line_start && line_start < line_end);
    assert!(line_end < line_stroke);
  }

  #[test]
  fn direct_writer_freezes_ordinary_text_boundary_with_independent_examples() {
    let mut translucent = common::TextStyle {
      font_family: Some("Liberation Serif".into()),
      font_size: Pt(12.0),
      ..Default::default()
    };
    translucent.color.a = 128;
    let mut invisible = translucent.clone();
    invisible.color.a = 0;
    for alpha in [1, 128, 254] {
      let mut style = translucent.clone();
      style.color.a = alpha;
      let translucent_pdf = String::from_utf8_lossy(
        &render(&text_document("aaaa", style), &uncompressed_options()).unwrap(),
      )
      .into_owned();
      assert!(
        translucent_pdf.contains("/Type/ExtGState"),
        "{translucent_pdf}"
      );
      assert!(translucent_pdf.contains("f*"), "{translucent_pdf}");
      assert!(!translucent_pdf.contains("BT"), "{translucent_pdf}");
      if alpha == 128 {
        assert!(
          translucent_pdf.contains("/ca 0.5019608"),
          "{translucent_pdf}"
        );
      }
    }

    let invisible_pdf = String::from_utf8_lossy(
      &render(
        &text_document("aaaa", invisible.clone()),
        &uncompressed_options(),
      )
      .unwrap(),
    )
    .into_owned();
    assert!(!invisible_pdf.contains("BT"), "{invisible_pdf}");
    assert!(!invisible_pdf.contains("/Subtype/Type0"), "{invisible_pdf}");
    assert!(
      !invisible_pdf.contains("/Type/ExtGState"),
      "{invisible_pdf}"
    );
    assert!(!invisible_pdf.contains(" rg"), "{invisible_pdf}");

    let mut invisible_highlight = invisible.clone();
    invisible_highlight.highlight = Some(color(255, 255, 0, u8::MAX));
    let invisible_highlight_pdf = String::from_utf8_lossy(
      &render(
        &text_document("aaaa", invisible_highlight),
        &uncompressed_options(),
      )
      .unwrap(),
    )
    .into_owned();
    assert!(invisible_highlight_pdf.contains("1 1 0 rg"));
    assert!(invisible_highlight_pdf.contains(" re\nf"));
    assert!(!invisible_highlight_pdf.contains("BT"));

    let outline_color = color(57, 82, 116, u8::MAX);
    let outline_only = common::TextStyle {
      font_family: Some("Liberation Serif".into()),
      font_size: Pt(12.0),
      color: color(235, 233, 233, 0),
      outline_color: Some(outline_color),
      outline_width: Pt(0.5),
      pdf_glyph_outlines: true,
      pdf_glyph_outline_options: Some(Arc::new(common::PdfGlyphOutlineOptions {
        outline_stroke: Some(common::Stroke {
          width: Pt(0.5),
          color: outline_color,
          ..Default::default()
        }),
        ..Default::default()
      })),
      ..Default::default()
    };
    let outline_only_pdf = String::from_utf8_lossy(
      &render(
        &text_document("aaaa", outline_only),
        &uncompressed_options(),
      )
      .unwrap(),
    )
    .into_owned();
    assert!(outline_only_pdf.contains("\nS\n"), "{outline_only_pdf}");
    assert!(!outline_only_pdf.contains("BT"), "{outline_only_pdf}");

    let highlighted = common::TextStyle {
      font_family: Some("Liberation Serif".into()),
      font_size: Pt(12.0),
      color: color(0, 0, 0, u8::MAX),
      highlight: Some(color(255, 255, 0, u8::MAX)),
      ..Default::default()
    };
    let highlighted_pdf = String::from_utf8_lossy(
      &render(&text_document("aaaa", highlighted), &uncompressed_options()).unwrap(),
    )
    .into_owned();
    let highlight_color = highlighted_pdf.find("1 1 0 rg").unwrap();
    let highlight_rect =
      highlighted_pdf[highlight_color..].find(" re\nf").unwrap() + highlight_color;
    let text = highlighted_pdf[highlight_rect..].find("BT").unwrap() + highlight_rect;
    assert!(highlight_color < highlight_rect && highlight_rect < text);

    let rotated = common::TextStyle {
      font_family: Some("Liberation Serif".into()),
      font_size: Pt(12.0),
      color: color(0, 0, 0, u8::MAX),
      rotation_degrees: 30.0,
      ..Default::default()
    };
    let rotated_pdf = String::from_utf8_lossy(
      &render(&text_document("aaaa", rotated), &uncompressed_options()).unwrap(),
    )
    .into_owned();
    assert_eq!(content_matrices(&rotated_pdf).len(), 2, "{rotated_pdf}");
  }

  #[test]
  fn direct_writer_rotates_the_complete_text_paint_about_one_frozen_center() {
    let style = common::TextStyle {
      font_family: Some("Liberation Serif".into()),
      font_size: Pt(12.0),
      color: color(0, 0, 0, u8::MAX),
      highlight: Some(color(255, 255, 0, u8::MAX)),
      underline: true,
      rotation_degrees: 30.0,
      ..Default::default()
    };
    let mut document = text_document("rotation", style.clone());
    let common::DisplayItem::Text(text) = &mut document.pages[0].items[0] else {
      unreachable!();
    };
    text.rotation_center = Some(common::Point {
      x: Pt(100.0),
      y: Pt(200.0),
    });
    text.paint_clip = Some(common::Rect {
      origin: common::Point {
        x: Pt(80.0),
        y: Pt(60.0),
      },
      size: Size {
        width: Pt(10.0),
        height: Pt(20.0),
      },
    });

    let ordinary =
      String::from_utf8_lossy(&render(&document, &uncompressed_options()).unwrap()).into_owned();
    let matrices = content_matrices(&ordinary);
    assert_eq!(matrices.len(), 2, "{ordinary}");
    assert_matrix_near(
      matrices[1],
      [0.866_025_4, 0.5, -0.5, 0.866_025_4, 113.397_46, -23.205_08],
    );

    let tokens = ordinary.split_ascii_whitespace().collect::<Vec<_>>();
    let rotation = tokens
      .iter()
      .enumerate()
      .filter(|(_, token)| **token == "cm")
      .nth(1)
      .map(|(index, _)| index)
      .expect("text rotation matrix");
    let page_space_clip = tokens
      .iter()
      .position(|token| *token == "W")
      .expect("resolved text clip");
    assert!(page_space_clip < rotation, "{ordinary}");
    let mut depth = 1_u32;
    let mut saw_highlight = false;
    let mut saw_text = false;
    let mut saw_decoration = false;
    for token in &tokens[rotation + 1..] {
      match *token {
        "q" => depth += 1,
        "Q" => {
          depth -= 1;
          if depth == 0 {
            break;
          }
        }
        "rg" if depth > 0 => saw_highlight = true,
        "BT" if depth > 0 => saw_text = true,
        "S" if depth > 0 => saw_decoration = true,
        _ => {}
      }
    }
    assert!(saw_highlight && saw_text && saw_decoration, "{ordinary}");

    let mut outlined_document = text_document(
      "rotation",
      common::TextStyle {
        pdf_glyph_outlines: true,
        highlight: None,
        underline: false,
        ..style
      },
    );
    let common::DisplayItem::Text(text) = &mut outlined_document.pages[0].items[0] else {
      unreachable!();
    };
    text.rotation_center = Some(common::Point {
      x: Pt(100.0),
      y: Pt(200.0),
    });
    let outlined =
      String::from_utf8_lossy(&render(&outlined_document, &uncompressed_options()).unwrap())
        .into_owned();
    let outlined_matrices = content_matrices(&outlined);
    assert_eq!(outlined_matrices.len(), 2, "{outlined}");
    assert_matrix_near(outlined_matrices[1], matrices[1]);
    assert!(!outlined.contains("BT"), "{outlined}");
    assert!(outlined.contains("f*"), "{outlined}");
  }

  #[test]
  fn direct_writer_freezes_text_rotation_identity_fallback_and_invalid_inputs() {
    assert!(
      prepared_text_rotation_transform(360.0, f32::NAN, f32::INFINITY)
        .unwrap()
        .is_none()
    );
    assert!(matches!(
      prepared_text_rotation_transform(f32::NAN, 0.0, 0.0),
      Err(PdfError::Writer(message)) if message == "text rotation angle must be finite"
    ));
    assert!(matches!(
      prepared_text_rotation_transform(30.0, f32::NAN, 0.0),
      Err(PdfError::Writer(message)) if message == "text rotation center must be finite"
    ));

    let fallback = String::from_utf8_lossy(
      &render(
        &text_document(
          "fallback",
          common::TextStyle {
            font_family: Some("Liberation Serif".into()),
            font_size: Pt(12.0),
            color: color(0, 0, 0, u8::MAX),
            rotation_degrees: 90.0,
            ..Default::default()
          },
        ),
        &uncompressed_options(),
      )
      .unwrap(),
    )
    .into_owned();
    let matrices = content_matrices(&fallback);
    assert_eq!(matrices.len(), 2, "{fallback}");
    assert_matrix_near(
      [
        matrices[1][0],
        matrices[1][1],
        matrices[1][2],
        matrices[1][3],
        0.0,
        0.0,
      ],
      [0.0, 1.0, -1.0, 0.0, 0.0, 0.0],
    );
    let recovered_x = (matrices[1][4] - matrices[1][5]) * 0.5;
    let recovered_baseline = (matrices[1][4] + matrices[1][5]) * 0.5;
    assert!((recovered_x - 72.0).abs() <= 1.0e-5, "{fallback}");
    assert!(recovered_baseline > 72.0, "{fallback}");
  }

  #[test]
  fn direct_writer_flattens_only_source_backed_identity_group_with_parent_clip() {
    let mut document = blank_document(&[(612.0, 792.0)]);
    document.pages[0]
      .items
      .push(common::DisplayItem::Group(common::CompositingGroup {
        mask: None,
        clip: Some(common::Rect {
          origin: common::Point {
            x: Pt(5.0),
            y: Pt(6.0),
          },
          size: Size {
            width: Pt(20.0),
            height: Pt(30.0),
          },
        }),
        transform: None,
        blend_mode: common::BlendMode::Normal,
        opacity: 1.0,
        flatten_identity: true,
        inherit_text_line_owner: false,
        items: vec![common::DisplayItem::Rect(common::RectItem {
          bounds: common::Rect {
            origin: common::Point {
              x: Pt(0.0),
              y: Pt(0.0),
            },
            size: Size {
              width: Pt(50.0),
              height: Pt(50.0),
            },
          },
          fill: common::Fill::Solid(color(255, 0, 0, 255)),
          stroke: None,
        })],
      }));

    let bytes = render(&document, &uncompressed_options()).unwrap();
    let pdf = String::from_utf8_lossy(&bytes);
    let clip = pdf.find("5 6 20 30 re").unwrap();
    let clip_operator = pdf[clip..].find("W").unwrap() + clip;
    let end_path = pdf[clip_operator..].find("n").unwrap() + clip_operator;
    let fill = pdf[end_path..].find("1 0 0 rg").unwrap() + end_path;

    assert!(clip < clip_operator && clip_operator < end_path && end_path < fill);
  }

  #[test]
  fn direct_writer_writes_an_authored_isolation_boundary_as_a_form_group() {
    let mut document = blank_document(&[(612.0, 792.0)]);
    document.pages[0]
      .items
      .push(common::DisplayItem::Group(common::CompositingGroup {
        mask: None,
        clip: None,
        transform: None,
        blend_mode: common::BlendMode::Normal,
        opacity: 1.0,
        flatten_identity: false,
        inherit_text_line_owner: false,
        items: vec![
          common::DisplayItem::Rect(common::RectItem {
            bounds: common::Rect {
              origin: common::Point {
                x: Pt(10.0),
                y: Pt(20.0),
              },
              size: Size {
                width: Pt(40.0),
                height: Pt(30.0),
              },
            },
            fill: common::Fill::Solid(color(255, 0, 0, 128)),
            stroke: None,
          }),
          common::DisplayItem::Rect(common::RectItem {
            bounds: common::Rect {
              origin: common::Point {
                x: Pt(30.0),
                y: Pt(20.0),
              },
              size: Size {
                width: Pt(40.0),
                height: Pt(30.0),
              },
            },
            fill: common::Fill::Solid(color(0, 0, 255, 128)),
            stroke: None,
          }),
        ],
      }));

    let bytes = render(&document, &uncompressed_options()).unwrap();
    let pdf = String::from_utf8_lossy(&bytes);
    assert_eq!(pdf.matches("/Subtype/Form").count(), 1);
    assert!(pdf.contains("/BBox[0 0 612 792]"), "{pdf}");
    assert!(pdf.contains("/S/Transparency"), "{pdf}");
    assert!(pdf.contains("/I true"), "{pdf}");
    assert!(pdf.contains("/CS/DeviceRGB"), "{pdf}");
    assert!(pdf.contains("/XObject<</Fm0"), "{pdf}");
    assert!(pdf.contains("/Fm0 Do"), "{pdf}");
    assert!(pdf.contains("/ExtGState<</GS0"), "{pdf}");
  }

  #[test]
  fn direct_writer_applies_group_alpha_once_even_with_flatten_identity_requested() {
    for alpha in [0_u8, 64, 128, 254] {
      let mut document = blank_document(&[(612.0, 792.0)]);
      document.pages[0]
        .items
        .push(common::DisplayItem::Group(common::CompositingGroup {
          mask: None,
          clip: None,
          transform: None,
          blend_mode: common::BlendMode::Normal,
          opacity: f32::from(alpha) / 255.0,
          flatten_identity: true,
          inherit_text_line_owner: true,
          items: vec![common::DisplayItem::Rect(common::RectItem {
            bounds: common::Rect {
              origin: common::Point {
                x: Pt(10.0),
                y: Pt(20.0),
              },
              size: Size {
                width: Pt(40.0),
                height: Pt(30.0),
              },
            },
            fill: common::Fill::Solid(color(0, 0, 0, 255)),
            stroke: Some(common::Stroke {
              width: Pt(4.0),
              color: color(0, 0, 0, 255),
              ..Default::default()
            }),
          })],
        }));
      assert!(display_items_need_preparation(&document.pages[0].items));
      let bytes = render(&document, &uncompressed_options()).unwrap();
      let pdf = String::from_utf8_lossy(&bytes);
      assert_eq!(pdf.matches("/Subtype/Form").count(), 1, "{pdf}");
      assert!(pdf.contains("/I true"), "{pdf}");
      assert_eq!(pdf.matches("/ca ").count(), 1, "{pdf}");
      assert_eq!(pdf.matches("/CA ").count(), 0, "{pdf}");
      assert!(pdf.contains("/GS0 gs/Fm0 Do"), "{pdf}");
    }
  }

  #[test]
  fn direct_writer_keeps_other_group_compositing_states_independent() {
    let base = common::CompositingGroup {
      mask: None,
      clip: None,
      transform: None,
      blend_mode: common::BlendMode::Normal,
      opacity: 1.0,
      flatten_identity: false,
      inherit_text_line_owner: false,
      items: Vec::new(),
    };

    let mut masked = base.clone();
    masked.mask = Some(test_image_item(opaque_test_png()));
    assert!(matches!(
      ensure_group_supported(&masked),
      Err(PdfError::DirectWriterUnsupported {
        feature: "group alpha masks"
      })
    ));

    let mut transformed = base.clone();
    transformed.transform = Some(common::Transform::default());
    assert!(matches!(
      ensure_group_supported(&transformed),
      Err(PdfError::DirectWriterUnsupported {
        feature: "transformed compositing groups"
      })
    ));

    let mut blended = base.clone();
    blended.blend_mode = common::BlendMode::Multiply;
    assert!(matches!(
      ensure_group_supported(&blended),
      Err(PdfError::DirectWriterUnsupported {
        feature: "group blend modes"
      })
    ));

    let mut translucent = base;
    translucent.opacity = 0.5;
    assert!(ensure_group_supported(&translucent).is_ok());
    for invalid in [f32::NAN, f32::INFINITY, -0.01, 1.01] {
      translucent.opacity = invalid;
      assert!(matches!(
        ensure_group_supported(&translucent),
        Err(PdfError::Writer(_))
      ));
    }
  }

  #[test]
  fn direct_writer_serializes_and_deduplicates_stroking_and_nonstroking_alpha() {
    let mut document = blank_document(&[(612.0, 792.0)]);
    for x in [10.0, 50.0] {
      document.pages[0]
        .items
        .push(common::DisplayItem::Rect(common::RectItem {
          bounds: common::Rect {
            origin: common::Point {
              x: Pt(x),
              y: Pt(20.0),
            },
            size: Size {
              width: Pt(30.0),
              height: Pt(40.0),
            },
          },
          fill: common::Fill::Solid(color(255, 0, 0, 128)),
          stroke: Some(common::Stroke {
            width: Pt(1.0),
            color: color(0, 0, 255, 64),
            ..Default::default()
          }),
        }));
    }

    let bytes = render(&document, &uncompressed_options()).unwrap();
    let pdf = String::from_utf8_lossy(&bytes);

    assert_eq!(pdf.matches("/Type/ExtGState").count(), 2);
    assert!(pdf.contains("/ca 0.5019608"));
    assert!(pdf.contains("/CA 0.2509804"));
    assert_eq!(pdf.matches("/GS0 gs").count(), 2);
    assert_eq!(pdf.matches("/GS1 gs").count(), 2);
  }

  #[test]
  fn direct_writer_sets_explicit_pdf_version_but_rejects_unimplemented_pdf_a_profiles() {
    let document = blank_document(&[(612.0, 792.0)]);
    let mut pdf_20 = uncompressed_options();
    pdf_20.standards.push(PdfStandard::Pdf20);
    assert!(render(&document, &pdf_20).unwrap().starts_with(b"%PDF-2.0"));

    let mut pdf_a = uncompressed_options();
    pdf_a.standards.push(PdfStandard::PdfA1b);
    assert!(matches!(
      render(&document, &pdf_a),
      Err(PdfError::DirectWriterUnsupported {
        feature: "PDF/A profiles outside the implemented PDF/A-3 family"
      })
    ));
  }

  #[test]
  fn direct_writer_pdf_a3a_closes_the_output_intent_and_metadata_chain() {
    let document = blank_document(&[(612.0, 792.0)]);
    let creation_date = crate::PdfDateTime {
      year: 2026,
      month: Some(8),
      day: Some(18),
      hour: Some(12),
      minute: Some(0),
      second: Some(0),
      utc_offset_hour: Some(8),
      utc_offset_minute: Some(0),
    };
    let mut options = uncompressed_options();
    options.standards.push(PdfStandard::PdfA3a);
    options.default_document_language = Some("en-US".to_string());
    options.metadata.creation_date = Some(creation_date);
    options.metadata.creator = Some("ooxmlsdk-pdf".to_string());
    options.metadata.producer = Some("ooxmlsdk-pdf configured campaign".to_string());

    let bytes = render(&document, &options).unwrap();
    assert_eq!(&bytes[..16], b"%PDF-1.7\n%\x80\x80\x80\x80\n\n");
    let pdf = String::from_utf8_lossy(&bytes);

    assert!(pdf.contains("/OutputIntents["));
    assert!(pdf.contains("/Type/OutputIntent"));
    assert!(pdf.contains("/S/GTS_PDFA1"));
    assert!(pdf.contains("/DestOutputProfile"));
    assert!(pdf.contains("/OutputConditionIdentifier(sRGB)"));
    assert!(pdf.contains("/N 3"));
    assert!(pdf.contains("/Range[0 1 0 1 0 1]"));
    assert!(pdf.contains("/Filter/FlateDecode"));
    assert!(pdf.contains("<pdfaid:part>3</pdfaid:part>"));
    assert!(pdf.contains("<pdfaid:conformance>A</pdfaid:conformance>"));
    assert!(pdf.contains("pdfaExtension:schemas"));
    assert!(pdf.contains("<xmpMM:History>"));
    assert!(pdf.contains("<stEvt:action>saved</stEvt:action>"));
    assert!(pdf.contains("<stEvt:action>converted</stEvt:action>"));
    assert!(!pdf.contains("<pdfuaid:part>"));
    assert!(pdf.contains("/MarkInfo<</Marked true>>"));
    assert!(pdf.contains("/Tabs/S"));
  }

  #[test]
  fn direct_writer_pdf_a3_associates_each_dated_embedded_file() {
    let document = blank_document(&[(612.0, 792.0)]);
    let creation_date = crate::PdfDateTime {
      year: 2026,
      month: Some(8),
      day: Some(18),
      hour: Some(12),
      minute: Some(0),
      second: Some(0),
      utc_offset_hour: Some(8),
      utc_offset_minute: Some(0),
    };
    let mut options = uncompressed_options();
    options.standards.push(PdfStandard::PdfA3a);
    options.default_document_language = Some("en-US".to_string());
    options.metadata.creation_date = Some(creation_date);
    let mut embedded = attachment(
      "source.txt",
      crate::PdfAttachmentAssociation::Source,
      b"source",
      Some(false),
    );
    embedded.modification_date = Some(creation_date);
    options.attachments.push(embedded);

    let bytes = render(&document, &options).unwrap();
    let pdf = String::from_utf8_lossy(&bytes);
    assert!(pdf.contains("/EmbeddedFiles"));
    assert!(pdf.contains("/AFRelationship/Source"));
    assert!(pdf.contains("/AF["));

    options.attachments[0].modification_date = None;
    assert!(matches!(
      render(&document, &options),
      Err(PdfError::Options(message)) if message.contains("requires a modification date")
    ));
  }

  #[test]
  fn direct_writer_pdf_ua_adds_identity_navigation_and_accessibility_keys() {
    let mut document = blank_document(&[(612.0, 792.0)]);
    document.outline_entries.clear();
    let mut options = uncompressed_options();
    options.standards.push(PdfStandard::PdfUa1);
    options.default_document_language = Some("en-US".to_string());
    options.source_file_name = Some("fallback.docx".to_string());

    let bytes = render(&document, &options).unwrap();
    let pdf = String::from_utf8_lossy(&bytes);
    assert!(pdf.contains("<pdfuaid:part>1</pdfuaid:part>"));
    assert!(pdf.contains("/Lang(en-US)"));
    assert!(pdf.contains("/MarkInfo<</Marked true/Suspects false>>"));
    assert!(pdf.contains("/ViewerPreferences<</DisplayDocTitle true>>"));
    assert!(pdf.contains("/Tabs/S"));
    assert!(pdf.contains("/Outlines"));
    assert!(pdf.contains("/Title(fallback.docx)"));
    assert!(pdf.contains("<dc:title>"));
    assert!(pdf.contains("fallback.docx"));

    let mut embedded = attachment(
      "source.txt",
      crate::PdfAttachmentAssociation::Source,
      b"source",
      Some(false),
    );
    embedded.modification_date = None;
    options.attachments.push(embedded);
    let bytes = render(&document, &options).unwrap();
    let pdf = String::from_utf8_lossy(&bytes);
    assert!(pdf.contains("/EmbeddedFiles"));
    assert!(!pdf.contains("/ModDate"));

    options.default_document_language = None;
    options.ui_language = None;
    assert!(matches!(
      render(&document, &options),
      Err(PdfError::Options(message)) if message.contains("requires a non-empty document or UI language")
    ));
  }

  #[test]
  fn direct_writer_combined_pdf_a3a_pdf_ua_describes_the_pdf_ua_extension_schema() {
    let document = blank_document(&[(612.0, 792.0)]);
    let mut options = uncompressed_options();
    options.standards = vec![PdfStandard::PdfA3a, PdfStandard::PdfUa1];
    options.default_document_language = Some("en-US".to_string());
    options.metadata.title = Some("Combined conformance".to_string());
    options.metadata.creation_date = Some(crate::PdfDateTime {
      year: 2026,
      month: Some(8),
      day: Some(18),
      hour: Some(12),
      minute: Some(0),
      second: Some(0),
      utc_offset_hour: Some(8),
      utc_offset_minute: Some(0),
    });

    let bytes = render(&document, &options).unwrap();
    let pdf = String::from_utf8_lossy(&bytes);
    assert!(pdf.contains("<pdfaid:part>3</pdfaid:part>"));
    assert!(pdf.contains("<pdfuaid:part>1</pdfuaid:part>"));
    assert!(pdf.contains("Part Number of ISO 14289 (PDF/UA)"));
    assert!(pdf.contains("http://www.aiim.org/pdfua/ns/id/"));
    assert!(pdf.contains("<pdfaSchema:prefix>pdfuaid</pdfaSchema:prefix>"));
    assert!(pdf.contains("http://www.aiim.org/pdfua/ns/id/"));
    assert!(pdf.contains("/OutputIntents["));
    assert!(pdf.contains("/Suspects false"));
  }

  #[test]
  fn direct_writer_uses_the_materialized_display_item_as_the_only_page_background_owner() {
    let mut document = blank_document(&[(612.0, 792.0)]);
    let background = color(255, 0, 255, 255);
    document.pages[0].background = Some(common::Fill::Solid(background));
    document.pages[0].setup.background = Some(background);
    document.pages[0]
      .items
      .push(common::DisplayItem::Rect(common::RectItem {
        bounds: common::Rect {
          origin: common::Point {
            x: Pt(0.0),
            y: Pt(0.0),
          },
          size: Size {
            width: Pt(612.0),
            height: Pt(792.0),
          },
        },
        fill: common::Fill::Solid(background),
        stroke: None,
      }));

    let bytes = render(&document, &uncompressed_options()).unwrap();
    let pdf = String::from_utf8_lossy(&bytes);
    assert_eq!(pdf.matches("1 0 1 rg").count(), 1);
    assert_eq!(pdf.matches("0 0 612 792 re").count(), 1);

    document.pages[0].items.clear();
    let bytes = render(&document, &uncompressed_options()).unwrap();
    let pdf = String::from_utf8_lossy(&bytes);
    assert!(!pdf.contains("1 0 1 rg"));
  }

  #[test]
  fn direct_writer_keeps_tiling_patterns_across_raw_and_prepared_paint_paths() {
    let pattern = common::PatternFill::drawingml(
      emfsdk::emfplus::EmfPlusHatchStyle::LightHorizontal,
      color(153, 255, 102, 255),
      color(255, 255, 255, 255),
    );
    let pattern_rect = common::DisplayItem::Rect(common::RectItem {
      bounds: common::Rect {
        origin: common::Point {
          x: Pt(100.0),
          y: Pt(100.0),
        },
        size: Size {
          width: Pt(120.0),
          height: Pt(60.0),
        },
      },
      fill: common::Fill::Pattern(pattern),
      stroke: None,
    });

    let mut raw_document = blank_document(&[(612.0, 792.0)]);
    raw_document.pages[0].items.push(pattern_rect.clone());
    let raw_pdf = render(&raw_document, &uncompressed_options()).unwrap();

    let mut prepared_document = blank_document(&[(612.0, 792.0)]);
    prepared_document.pages[0].items.push(pattern_rect);
    prepared_document.pages[0]
      .items
      .push(common::DisplayItem::LinkArea(common::LinkArea {
        bounds: common::Rect {
          origin: common::Point {
            x: Pt(1.0),
            y: Pt(1.0),
          },
          size: Size {
            width: Pt(1.0),
            height: Pt(1.0),
          },
        },
        target: "https://example.test/".into(),
      }));
    let prepared_pdf = render(&prepared_document, &uncompressed_options()).unwrap();

    for bytes in [&raw_pdf, &prepared_pdf] {
      let source = String::from_utf8_lossy(bytes);
      assert!(source.contains("/PatternType 1"), "{source}");
      assert!(source.contains("/PaintType 1"), "{source}");
      assert!(source.contains("/TilingType 2"), "{source}");
      assert!(source.contains("/BBox[0 0 16 16]"), "{source}");
      assert!(
        source.contains("/Matrix[0.375 0 0 0.375 96 696]"),
        "{source}"
      );
      assert!(source.contains("/Pattern<</TP0"), "{source}");
      assert!(source.contains("/Pattern cs/TP0 scn"), "{source}");
      assert!(source.contains("/XObject<</Im0"), "{source}");
      assert!(!source.contains("/Interpolate true"), "{source}");
    }

    let patterned_stroke = common::Stroke {
      pattern: Some(pattern),
      ..common::Stroke::default()
    };
    assert!(matches!(
      ensure_stroke_supported(&patterned_stroke, StrokeUse::Path),
      Err(PdfError::DirectWriterUnsupported {
        feature: "tiling-pattern strokes"
      })
    ));
  }
}
