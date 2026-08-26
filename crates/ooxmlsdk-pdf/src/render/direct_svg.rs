use std::collections::BTreeMap;

use pdf_writer::types::{LineCapStyle, LineJoinStyle, TextRenderingMode};
use pdf_writer::{Content, Finish, Name, Str, TextStr};
use quick_xml::events::Event;
use quick_xml::{Reader, XmlVersion};
use usvg::tiny_skia_path::{Path as TinyPath, PathSegment, Transform};

use super::direct::{PageResources, RefAllocator};
use super::direct_font::DirectFontSet;
use crate::error::{PdfError, Result};

const OFFICE_MATH_VISIBLE_GLYPH_PREFIX: &str = "ooxmlsdk-math-visible-";
const OFFICE_MATH_SEMANTIC_GLYPH_PREFIX: &str = "ooxmlsdk-math-semantic-";
const OFFICE_MATH_SEMANTIC_CLIP_ID: &str = "math-semantic-clip";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum OfficeMathTextMarker {
  Visible,
  Semantic { exact_glyph_id: Option<u32> },
}

#[derive(Clone, Debug)]
struct OfficeMathSemanticSource {
  id: String,
  marker: OfficeMathTextMarker,
  text: String,
  transform: Transform,
  font_family: String,
  font_size: f32,
  font_weight: usvg::fontdb::Weight,
  font_style: usvg::fontdb::Style,
  color: [u8; 3],
  opacity: f32,
}

pub(super) struct PreparedOfficeMathSvg {
  tree: usvg::Tree,
  semantic_sources: Vec<OfficeMathSemanticSource>,
}

/// Parsed package SVG shared by vector lowering and the complete raster
/// fallback. Keeping one normalized usvg tree avoids reparsing repeated image
/// relationships and guarantees that both paths resolve fonts and resources
/// identically.
pub(super) struct PreparedSvg {
  tree: usvg::Tree,
}

impl PreparedSvg {
  pub(super) fn tree(&self) -> &usvg::Tree {
    &self.tree
  }
}

impl PreparedOfficeMathSvg {
  pub(super) fn tree(&self) -> &usvg::Tree {
    &self.tree
  }
}

pub(super) fn prepare_svg(data: &[u8], options: &usvg::Options<'_>) -> Result<PreparedSvg> {
  let tree = usvg::Tree::from_data(data, options)
    .map_err(|error| PdfError::Image(format!("failed to decode SVG image: {error}")))?;
  validate_tree_size(&tree, "SVG image")?;
  Ok(PreparedSvg { tree })
}

pub(super) fn prepare_office_math_svg(
  data: &[u8],
  options: &usvg::Options<'_>,
) -> Result<PreparedOfficeMathSvg> {
  let semantic_sources = parse_office_math_semantic_sources(data)?;
  let tree = usvg::Tree::from_data(data, options)
    .map_err(|error| PdfError::Image(format!("failed to decode OfficeMath SVG: {error}")))?;
  Ok(PreparedOfficeMathSvg {
    tree,
    semantic_sources,
  })
}

#[derive(Debug)]
struct OfficeMathSourceFrame {
  name: Vec<u8>,
  semantic_group: bool,
  semantic_text: bool,
  semantic_children: usize,
}

fn parse_office_math_semantic_sources(data: &[u8]) -> Result<Vec<OfficeMathSemanticSource>> {
  let mut reader = Reader::from_reader(data);
  reader.config_mut().trim_text(false);
  let mut frames = Vec::<OfficeMathSourceFrame>::new();
  let mut pending = None::<OfficeMathSemanticSource>;
  let mut sources = Vec::new();

  loop {
    match reader.read_event() {
      Ok(Event::Start(event)) => {
        if pending.is_some() {
          return source_unsupported("nested markup in semantic text");
        }
        let name = event.name().as_ref().to_vec();
        let mut attributes = decode_source_attributes(&event, &reader)?;
        let semantic_group = if name.as_slice() == b"g"
          && attributes.get("clip-path").map(String::as_str) == Some("url(#math-semantic-clip)")
        {
          if attributes.len() != 1 {
            return source_unsupported("semantic clip group attributes");
          }
          if frames.iter().any(|frame| frame.semantic_group) {
            return source_unsupported("nested semantic clip groups");
          }
          true
        } else {
          false
        };

        let marker = attributes
          .get("id")
          .and_then(|id| office_math_text_marker(id));
        let semantic_text = matches!(marker, Some(OfficeMathTextMarker::Semantic { .. }));
        if let Some(parent) = frames.last_mut()
          && parent.semantic_group
        {
          if name.as_slice() != b"text" || !semantic_text {
            return source_unsupported("non-semantic child in semantic clip group");
          }
          parent.semantic_children += 1;
          if parent.semantic_children != 1 {
            return source_unsupported("multiple semantic text children in one clip group");
          }
        } else if semantic_text {
          return source_unsupported("unclipped semantic text source");
        }

        if semantic_text {
          pending = Some(parse_semantic_source_attributes(&mut attributes)?);
        }
        frames.push(OfficeMathSourceFrame {
          name,
          semantic_group,
          semantic_text,
          semantic_children: 0,
        });
      }
      Ok(Event::Empty(event)) => {
        if pending.is_some() || frames.last().is_some_and(|frame| frame.semantic_group) {
          return source_unsupported("empty or nested semantic text source");
        }
        let attributes = decode_source_attributes(&event, &reader)?;
        if attributes
          .get("id")
          .and_then(|id| office_math_text_marker(id))
          .is_some_and(|marker| matches!(marker, OfficeMathTextMarker::Semantic { .. }))
        {
          return source_unsupported("empty semantic text source");
        }
      }
      Ok(Event::Text(text)) => {
        let decoded = text
          .xml10_content()
          .map_err(|error| source_error(format!("text decoding failed: {error}")))?;
        let decoded = quick_xml::escape::unescape(&decoded)
          .map_err(|error| source_error(format!("text entity decoding failed: {error}")))?;
        if let Some(source) = pending.as_mut() {
          source.text.push_str(&decoded);
        } else if frames.last().is_some_and(|frame| frame.semantic_group)
          && !decoded.chars().all(char::is_whitespace)
        {
          return source_unsupported("character data beside semantic text");
        }
      }
      Ok(Event::GeneralRef(reference)) => {
        let decoded = reference
          .decode()
          .map_err(|error| source_error(format!("entity name decoding failed: {error}")))?;
        let entity = format!("&{decoded};");
        let decoded = quick_xml::escape::unescape(&entity)
          .map_err(|error| source_error(format!("entity decoding failed: {error}")))?;
        if let Some(source) = pending.as_mut() {
          source.text.push_str(&decoded);
        } else if frames.last().is_some_and(|frame| frame.semantic_group) {
          return source_unsupported("entity beside semantic text");
        }
      }
      Ok(Event::CData(_)) if pending.is_some() => {
        return source_unsupported("CDATA in semantic text");
      }
      Ok(Event::End(event)) => {
        let frame = frames
          .pop()
          .ok_or_else(|| source_error("unexpected closing element"))?;
        if frame.name.as_slice() != event.name().as_ref() {
          return source_unsupported("mismatched closing element");
        }
        if frame.semantic_text {
          let source = pending
            .take()
            .ok_or_else(|| source_error("missing semantic text source state"))?;
          if source.text.is_empty() {
            return source_unsupported("empty semantic text source");
          }
          sources.push(source);
        }
        if frame.semantic_group && frame.semantic_children != 1 {
          return source_unsupported("semantic clip group child count");
        }
      }
      Ok(Event::Eof) => break,
      Err(error) => return Err(source_error(format!("XML parsing failed: {error}"))),
      _ => {}
    }
  }

  if !frames.is_empty() || pending.is_some() {
    return source_unsupported("unterminated semantic source markup");
  }
  Ok(sources)
}

fn decode_source_attributes(
  event: &quick_xml::events::BytesStart<'_>,
  reader: &Reader<&[u8]>,
) -> Result<BTreeMap<String, String>> {
  let mut decoded = BTreeMap::new();
  for attribute in event.attributes().with_checks(true) {
    let attribute =
      attribute.map_err(|error| source_error(format!("attribute parsing failed: {error}")))?;
    let key = std::str::from_utf8(attribute.key.as_ref())
      .map_err(|error| source_error(format!("attribute name is not UTF-8: {error}")))?
      .to_string();
    let value = attribute
      .decoded_and_normalized_value(XmlVersion::Implicit1_0, reader.decoder())
      .map_err(|error| source_error(format!("attribute decoding failed: {error}")))?
      .into_owned();
    if decoded.insert(key, value).is_some() {
      return source_unsupported("duplicate source attribute");
    }
  }
  Ok(decoded)
}

fn parse_semantic_source_attributes(
  attributes: &mut BTreeMap<String, String>,
) -> Result<OfficeMathSemanticSource> {
  let id = take_source_attribute(attributes, "id")?;
  let marker = office_math_text_marker(&id)
    .filter(|marker| matches!(marker, OfficeMathTextMarker::Semantic { .. }))
    .ok_or_else(|| source_error("invalid semantic text marker"))?;
  if take_source_attribute(attributes, "visibility")? != "hidden"
    || take_source_attribute(attributes, "x")? != "0"
    || take_source_attribute(attributes, "y")? != "0"
    || take_source_attribute(attributes, "stroke")? != "none"
    || take_source_attribute(attributes, "xml:space")? != "preserve"
  {
    return source_unsupported("semantic text fixed attributes");
  }
  let transform = parse_source_text_transform(&take_source_attribute(attributes, "transform")?)?;
  let font_family = take_source_attribute(attributes, "font-family")?;
  if font_family.is_empty() {
    return source_unsupported("empty semantic font family");
  }
  let font_size = parse_source_f32(
    "font-size",
    &take_source_attribute(attributes, "font-size")?,
  )?;
  if font_size <= 0.0 {
    return source_unsupported("non-positive semantic font size");
  }
  let font_weight = match take_source_attribute(attributes, "font-weight")?.as_str() {
    "normal" => usvg::fontdb::Weight::NORMAL,
    "bold" => usvg::fontdb::Weight::BOLD,
    _ => return source_unsupported("semantic font weight"),
  };
  let font_style = match take_source_attribute(attributes, "font-style")?.as_str() {
    "normal" => usvg::fontdb::Style::Normal,
    "italic" => usvg::fontdb::Style::Italic,
    _ => return source_unsupported("semantic font style"),
  };
  let color = parse_source_color(&take_source_attribute(attributes, "fill")?)?;
  let opacity = parse_source_f32(
    "fill-opacity",
    &take_source_attribute(attributes, "fill-opacity")?,
  )?;
  if !(0.0..=1.0).contains(&opacity) {
    return source_unsupported("semantic fill opacity range");
  }
  if !attributes.is_empty() {
    return source_unsupported("unknown semantic text attributes");
  }
  Ok(OfficeMathSemanticSource {
    id,
    marker,
    text: String::new(),
    transform,
    font_family,
    font_size,
    font_weight,
    font_style,
    color,
    opacity,
  })
}

fn take_source_attribute(
  attributes: &mut BTreeMap<String, String>,
  name: &'static str,
) -> Result<String> {
  attributes
    .remove(name)
    .ok_or_else(|| source_error(format!("missing {name} attribute")))
}

fn parse_source_text_transform(value: &str) -> Result<Transform> {
  let value = value
    .strip_prefix("translate(")
    .ok_or_else(|| source_error("semantic text translate transform"))?;
  let (translation, scale) = value
    .split_once(") scale(")
    .ok_or_else(|| source_error("semantic text transform sequence"))?;
  let scale = scale
    .strip_suffix(')')
    .ok_or_else(|| source_error("semantic text scale transform"))?;
  let [x, y] = parse_source_pair("translation", translation)?;
  let [horizontal_scale, vertical_scale] = parse_source_pair("scale", scale)?;
  if horizontal_scale <= 0.0 || vertical_scale != 1.0 {
    return source_unsupported("semantic text scale values");
  }
  Ok(Transform::from_row(horizontal_scale, 0.0, 0.0, 1.0, x, y))
}

fn parse_source_pair(kind: &'static str, value: &str) -> Result<[f32; 2]> {
  let mut values = value.split_ascii_whitespace();
  let first = values
    .next()
    .ok_or_else(|| source_error(format!("missing semantic {kind} value")))?;
  let second = values
    .next()
    .ok_or_else(|| source_error(format!("missing semantic {kind} value")))?;
  if values.next().is_some() {
    return source_unsupported("extra semantic transform values");
  }
  Ok([
    parse_source_f32(kind, first)?,
    parse_source_f32(kind, second)?,
  ])
}

fn parse_source_f32(kind: &'static str, value: &str) -> Result<f32> {
  let value = value
    .parse::<f32>()
    .map_err(|error| source_error(format!("invalid semantic {kind}: {error}")))?;
  if value.is_finite() {
    Ok(value)
  } else {
    source_unsupported("non-finite semantic numeric value")
  }
}

fn parse_source_color(value: &str) -> Result<[u8; 3]> {
  let value = value
    .strip_prefix('#')
    .filter(|value| value.len() == 6)
    .ok_or_else(|| source_error("semantic fill color"))?;
  let component = |range| {
    u8::from_str_radix(&value[range], 16)
      .map_err(|error| source_error(format!("invalid semantic fill color: {error}")))
  };
  Ok([component(0..2)?, component(2..4)?, component(4..6)?])
}

fn source_error(message: impl Into<String>) -> PdfError {
  PdfError::Image(format!("invalid OfficeMath SVG source: {}", message.into()))
}

fn source_unsupported<T>(feature: &'static str) -> Result<T> {
  Err(source_error(feature))
}

pub(super) struct OfficeMathWriteContext<'a> {
  pub(super) resources: &'a mut PageResources,
  pub(super) fonts: &'a mut DirectFontSet,
  pub(super) refs: &'a mut RefAllocator,
  pub(super) requires_codepoint_mappings: bool,
  pub(super) forbids_private_use_mappings: bool,
}

pub(super) struct SvgWriteContext<'a> {
  pub(super) resources: &'a mut PageResources,
  pub(super) refs: &'a mut RefAllocator,
}

/// Returns whether the normalized SVG can be represented directly with PDF
/// path and graphics-state operators without changing SVG compositing.
///
/// Unsupported paint servers, clips, masks, filters, isolated/transparent
/// groups, and embedded images are intentionally routed to the complete resvg
/// fallback before any page content or resource is mutated.
pub(super) fn svg_scene_is_directly_writable(scene: &PreparedSvg) -> bool {
  svg_group_is_directly_writable(scene.tree().root())
}

fn svg_group_is_directly_writable(group: &usvg::Group) -> bool {
  if group.opacity().get() != 1.0
    || group.blend_mode() != usvg::BlendMode::Normal
    || group.isolate()
    || group.clip_path().is_some()
    || group.mask().is_some()
    || !group.filters().is_empty()
  {
    return false;
  }
  group.children().iter().all(|child| match child {
    usvg::Node::Path(path) => svg_path_is_directly_writable(path),
    usvg::Node::Text(text) => svg_group_is_directly_writable(text.flattened()),
    usvg::Node::Group(group) => svg_group_is_directly_writable(group),
    usvg::Node::Image(_) => false,
  })
}

fn svg_path_is_directly_writable(path: &usvg::Path) -> bool {
  let solid = |paint: &usvg::Paint| matches!(paint, usvg::Paint::Color(_));
  path.fill().is_none_or(|fill| solid(fill.paint()))
    && path
      .stroke()
      .is_none_or(|stroke| solid(stroke.paint()) && stroke.linejoin() != usvg::LineJoin::MiterClip)
    && !(path.fill().is_some()
      && path.stroke().is_some()
      && path.paint_order() != usvg::PaintOrder::FillAndStroke)
}

pub(super) fn write_svg_scene(
  content: &mut Content,
  scene: &PreparedSvg,
  context: &mut SvgWriteContext<'_>,
) -> Result<()> {
  validate_tree_size(scene.tree(), "SVG image")?;
  if !svg_scene_is_directly_writable(scene) {
    return unsupported("SVG scene requiring raster compositing");
  }
  write_svg_group(content, scene.tree().root(), context)
}

fn write_svg_group(
  content: &mut Content,
  group: &usvg::Group,
  context: &mut SvgWriteContext<'_>,
) -> Result<()> {
  for child in group.children() {
    match child {
      usvg::Node::Path(path) => {
        write_path(content, path, context.resources, context.refs)?;
      }
      usvg::Node::Text(text) => {
        write_svg_group(content, text.flattened(), context)?;
      }
      usvg::Node::Group(group) => write_svg_group(content, group, context)?,
      usvg::Node::Image(_) => return unsupported("SVG embedded image vector lowering"),
    }
  }
  Ok(())
}

fn validate_tree_size(tree: &usvg::Tree, label: &str) -> Result<()> {
  let size = tree.size();
  if size.width().is_finite()
    && size.height().is_finite()
    && size.width() > 0.0
    && size.height() > 0.0
  {
    Ok(())
  } else {
    Err(PdfError::Image(format!(
      "{label} has an invalid intrinsic size"
    )))
  }
}

/// Writes the closed SVG scene produced by `docx::math::MathBox::to_svg`.
///
/// This is intentionally not a generic SVG renderer. OfficeMath owns this
/// transport and emits only solid path/line/rectangle paint plus marked hidden
/// text. Keeping that contract explicit prevents an arbitrary package SVG from
/// silently receiving incomplete filter, mask, image, or paint-server behavior.
pub(super) fn write_office_math_scene(
  content: &mut Content,
  scene: &PreparedOfficeMathSvg,
  context: &mut OfficeMathWriteContext<'_>,
) -> Result<()> {
  let tree = scene.tree();
  let size = tree.size();
  if !size.width().is_finite()
    || !size.height().is_finite()
    || size.width() <= 0.0
    || size.height() <= 0.0
  {
    return Err(PdfError::Image(
      "OfficeMath SVG has an invalid intrinsic size".to_string(),
    ));
  }
  if !tree.linear_gradients().is_empty()
    || !tree.radial_gradients().is_empty()
    || !tree.patterns().is_empty()
  {
    return unsupported("OfficeMath SVG paint servers");
  }
  if !tree.masks().is_empty() || !tree.filters().is_empty() {
    return unsupported("OfficeMath SVG masks or filters");
  }
  if tree
    .clip_paths()
    .iter()
    .any(|clip| clip.id() != OFFICE_MATH_SEMANTIC_CLIP_ID)
  {
    return unsupported("OfficeMath SVG non-semantic clip paths");
  }

  let mut semantic_index = 0;
  write_group(
    content,
    tree.root(),
    scene,
    context,
    &mut semantic_index,
    true,
  )?;
  if semantic_index != scene.semantic_sources.len() {
    return unsupported("OfficeMath SVG source/tree semantic count mismatch");
  }
  Ok(())
}

fn write_group(
  content: &mut Content,
  group: &usvg::Group,
  scene: &PreparedOfficeMathSvg,
  context: &mut OfficeMathWriteContext<'_>,
  semantic_index: &mut usize,
  root: bool,
) -> Result<()> {
  validate_plain_group(group, true)?;

  if let Some(clip) = group.clip_path() {
    if root {
      return unsupported("OfficeMath SVG nested or root clipping");
    }
    let carrier = validate_semantic_clip(group, clip)?;
    let source =
      scene
        .semantic_sources
        .get(*semantic_index)
        .ok_or(PdfError::DirectWriterUnsupported {
          feature: "OfficeMath SVG tree/source semantic count mismatch",
        })?;
    *semantic_index += 1;
    content.save_state();
    append_tiny_path(content, semantic_clip_path(clip)?)?;
    content.clip_nonzero().end_path();
    match carrier {
      OfficeMathSemanticTreeCarrier::Text(text) => {
        if text.id() != source.id {
          content.restore_state();
          return unsupported("OfficeMath SVG source/tree semantic marker mismatch");
        }
        write_text(content, text, scene.tree(), context, Some(source))?;
      }
      OfficeMathSemanticTreeCarrier::DroppedWhitespace(transform) => {
        write_dropped_semantic_space(content, transform, source, scene.tree(), context)?;
      }
    }
    content.restore_state();
    return Ok(());
  }

  for child in group.children() {
    match child {
      usvg::Node::Path(path) => {
        write_path(content, path, context.resources, context.refs)?;
      }
      usvg::Node::Text(text) => write_text(content, text, scene.tree(), context, None)?,
      usvg::Node::Group(child) => {
        write_group(content, child, scene, context, semantic_index, false)?;
      }
      usvg::Node::Image(_) => return unsupported("OfficeMath SVG image nodes"),
    }
  }
  Ok(())
}

enum OfficeMathSemanticTreeCarrier<'a> {
  Text(&'a usvg::Text),
  DroppedWhitespace(Transform),
}

fn validate_semantic_clip<'a>(
  group: &'a usvg::Group,
  clip: &usvg::ClipPath,
) -> Result<OfficeMathSemanticTreeCarrier<'a>> {
  if clip.id() != OFFICE_MATH_SEMANTIC_CLIP_ID
    || clip.clip_path().is_some()
    || !is_identity(clip.transform())
    || !is_identity(group.abs_transform())
  {
    return unsupported("OfficeMath SVG semantic clip contract");
  }
  let mut texts = Vec::new();
  let mut empty_transforms = Vec::new();
  collect_semantic_tree_carriers(group, &mut texts, &mut empty_transforms)?;
  match (texts.as_slice(), empty_transforms.as_slice()) {
    ([text], []) => Ok(OfficeMathSemanticTreeCarrier::Text(text)),
    ([], [transform]) => Ok(OfficeMathSemanticTreeCarrier::DroppedWhitespace(*transform)),
    _ => unsupported("OfficeMath SVG semantic carrier cardinality"),
  }
}

fn collect_semantic_tree_carriers<'a>(
  group: &'a usvg::Group,
  texts: &mut Vec<&'a usvg::Text>,
  empty_transforms: &mut Vec<Transform>,
) -> Result<()> {
  for child in group.children() {
    match child {
      usvg::Node::Text(text)
        if matches!(
          office_math_text_marker(text.id()),
          Some(OfficeMathTextMarker::Semantic { .. })
        ) =>
      {
        texts.push(text);
      }
      usvg::Node::Group(group) => {
        validate_plain_group(group, false)?;
        if group.children().is_empty() {
          empty_transforms.push(group.abs_transform());
        } else {
          collect_semantic_tree_carriers(group, texts, empty_transforms)?;
        }
      }
      usvg::Node::Text(_) | usvg::Node::Path(_) | usvg::Node::Image(_) => {
        return unsupported("OfficeMath SVG non-semantic clipped carrier");
      }
    }
  }
  Ok(())
}

fn validate_plain_group(group: &usvg::Group, allow_clip: bool) -> Result<()> {
  if group.opacity().get() != 1.0
    || group.blend_mode() != usvg::BlendMode::Normal
    || group.isolate()
    || group.mask().is_some()
    || !group.filters().is_empty()
    || (!allow_clip && group.clip_path().is_some())
  {
    return unsupported("OfficeMath SVG group compositing");
  }
  Ok(())
}

fn semantic_clip_path(clip: &usvg::ClipPath) -> Result<&TinyPath> {
  let root = clip.root();
  if !is_identity(root.transform()) || root.children().len() != 1 {
    return unsupported("OfficeMath SVG semantic clip geometry");
  }
  let usvg::Node::Path(path) = &root.children()[0] else {
    return unsupported("OfficeMath SVG semantic clip geometry");
  };
  if !path.is_visible()
    || !is_identity(path.abs_transform())
    || path
      .fill()
      .is_some_and(|fill| fill.rule() != usvg::FillRule::NonZero)
    || path.stroke().is_some()
  {
    return unsupported("OfficeMath SVG semantic clip geometry");
  }
  let mut has_move = false;
  let mut has_segment = false;
  for segment in path.data().segments() {
    match segment {
      PathSegment::MoveTo(point) if point.x == 0.0 && point.y == 0.0 => has_move = true,
      PathSegment::LineTo(point) if point.x == 0.0 && point.y == 0.0 => has_segment = true,
      PathSegment::Close => {}
      _ => return unsupported("OfficeMath SVG semantic clip geometry"),
    }
  }
  if !has_move || !has_segment {
    return unsupported("OfficeMath SVG semantic clip geometry");
  }
  Ok(path.data())
}

fn write_path(
  content: &mut Content,
  path: &usvg::Path,
  resources: &mut PageResources,
  refs: &mut RefAllocator,
) -> Result<()> {
  if !path.is_visible() {
    return Ok(());
  }
  let fill = path.fill();
  let stroke = path.stroke();
  if fill.is_none() && stroke.is_none() {
    return Ok(());
  }
  if fill.is_some() && stroke.is_some() && path.paint_order() != usvg::PaintOrder::FillAndStroke {
    return unsupported("OfficeMath SVG stroke-before-fill paths");
  }
  let transform = pdf_transform(path.abs_transform())?;

  content.save_state().transform(transform);
  if let Some(fill) = fill {
    let color = solid_color(fill.paint())?;
    content.set_fill_rgb(
      channel(color.red),
      channel(color.green),
      channel(color.blue),
    );
  }
  if let Some(stroke) = stroke {
    let color = solid_color(stroke.paint())?;
    let line_cap = match stroke.linecap() {
      usvg::LineCap::Butt => LineCapStyle::ButtCap,
      usvg::LineCap::Round => LineCapStyle::RoundCap,
      usvg::LineCap::Square => LineCapStyle::ProjectingSquareCap,
    };
    let line_join = match stroke.linejoin() {
      usvg::LineJoin::Miter => LineJoinStyle::MiterJoin,
      usvg::LineJoin::Round => LineJoinStyle::RoundJoin,
      usvg::LineJoin::Bevel => LineJoinStyle::BevelJoin,
      usvg::LineJoin::MiterClip => {
        content.restore_state();
        return unsupported("OfficeMath SVG miter-clip strokes");
      }
    };
    content
      .set_stroke_rgb(
        channel(color.red),
        channel(color.green),
        channel(color.blue),
      )
      .set_line_width(stroke.width().get())
      .set_line_cap(line_cap)
      .set_line_join(line_join)
      .set_miter_limit(stroke.miterlimit().get());
    if let Some(dash) = stroke.dasharray() {
      content.set_dash_pattern(dash.iter().copied(), stroke.dashoffset());
    } else {
      content.set_dash_pattern(std::iter::empty(), 0.0);
    }
  }
  super::direct::set_alpha(
    content,
    resources,
    refs,
    stroke.map(|stroke| opacity_byte(stroke.opacity().get())),
    fill.map(|fill| opacity_byte(fill.opacity().get())),
  )?;
  append_tiny_path(content, path.data())?;
  match (fill.map(usvg::Fill::rule), stroke) {
    (Some(usvg::FillRule::EvenOdd), Some(_)) => {
      content.fill_even_odd_and_stroke();
    }
    (Some(usvg::FillRule::NonZero), Some(_)) => {
      content.fill_nonzero_and_stroke();
    }
    (Some(usvg::FillRule::EvenOdd), None) => {
      content.fill_even_odd();
    }
    (Some(usvg::FillRule::NonZero), None) => {
      content.fill_nonzero();
    }
    (None, Some(_)) => {
      content.stroke();
    }
    (None, None) => unreachable!("an unpainted OfficeMath path returned above"),
  }
  content.restore_state();
  Ok(())
}

fn write_text(
  content: &mut Content,
  text: &usvg::Text,
  tree: &usvg::Tree,
  context: &mut OfficeMathWriteContext<'_>,
  semantic_source: Option<&OfficeMathSemanticSource>,
) -> Result<()> {
  let marker = office_math_text_marker(text.id()).ok_or(PdfError::DirectWriterUnsupported {
    feature: "unmarked OfficeMath SVG text",
  })?;
  if matches!(marker, OfficeMathTextMarker::Semantic { .. }) != semantic_source.is_some() {
    return unsupported("OfficeMath SVG text marker clipping");
  }
  if semantic_source.is_some_and(|source| source.id != text.id() || source.marker != marker) {
    return unsupported("OfficeMath SVG source/tree semantic marker mismatch");
  }

  let mut wrote_exact_glyph = false;
  let mut wrote_any_glyph = false;
  for span in text.layouted() {
    if span.stroke.is_some()
      || span.underline.is_some()
      || span.overline.is_some()
      || span.line_through.is_some()
    {
      return unsupported("OfficeMath SVG decorated or stroked text");
    }
    let fill = span
      .fill
      .as_ref()
      .ok_or(PdfError::DirectWriterUnsupported {
        feature: "unpainted OfficeMath SVG text",
      })?;
    let color = solid_color(fill.paint())?;
    for positioned in &span.positioned_glyphs {
      let glyph_id = match marker {
        OfficeMathTextMarker::Visible
        | OfficeMathTextMarker::Semantic {
          exact_glyph_id: None,
        } => u32::from(positioned.id.0),
        OfficeMathTextMarker::Semantic {
          exact_glyph_id: Some(glyph_id),
        } => glyph_id,
      };
      let (handle, units_per_em, num_glyphs) =
        context
          .fonts
          .register_usvg_span_face(tree.fontdb(), span, positioned.font, || {
            context.refs.alloc()
          })?;
      if glyph_id >= num_glyphs {
        return Err(PdfError::Writer(format!(
          "OfficeMath SVG glyph ID {glyph_id} exceeds the selected font's {num_glyphs} glyphs"
        )));
      }
      let font_size = span.font_size.get();
      let glyph_transform = positioned.transform().pre_concat(Transform::from_scale(
        units_per_em / font_size,
        units_per_em / font_size,
      ));
      let text_transform = text.abs_transform().pre_concat(glyph_transform);
      let matrix = multiply_transform(
        pdf_transform(text_transform)?,
        [1.0, 0.0, 0.0, -1.0, 0.0, 0.0],
      );
      if !matrix.into_iter().all(f32::is_finite) {
        return Err(PdfError::Writer(
          "OfficeMath SVG glyph transform contains a non-finite value".to_string(),
        ));
      }
      let semantic_text = match marker {
        OfficeMathTextMarker::Semantic {
          exact_glyph_id: Some(_),
        } => semantic_source.map(|source| source.text.as_str()).ok_or(
          PdfError::DirectWriterUnsupported {
            feature: "OfficeMath SVG exact semantic glyph source",
          },
        )?,
        OfficeMathTextMarker::Visible
        | OfficeMathTextMarker::Semantic {
          exact_glyph_id: None,
        } => positioned.text.as_str(),
      };
      write_registered_glyph(
        content,
        context,
        OfficeMathGlyphWrite {
          handle,
          glyph_id,
          semantic_text,
          font_size,
          matrix,
          color: [color.red, color.green, color.blue],
          opacity: fill.opacity().get(),
        },
      )?;
      wrote_any_glyph = true;

      if matches!(
        marker,
        OfficeMathTextMarker::Semantic {
          exact_glyph_id: Some(_)
        }
      ) {
        wrote_exact_glyph = true;
        break;
      }
    }
    if wrote_exact_glyph {
      break;
    }
  }
  if wrote_any_glyph {
    Ok(())
  } else {
    unsupported("OfficeMath SVG text without positioned glyphs")
  }
}

fn write_dropped_semantic_space(
  content: &mut Content,
  tree_transform: Transform,
  source: &OfficeMathSemanticSource,
  tree: &usvg::Tree,
  context: &mut OfficeMathWriteContext<'_>,
) -> Result<()> {
  let OfficeMathTextMarker::Semantic {
    exact_glyph_id: Some(glyph_id),
  } = source.marker
  else {
    return unsupported("OfficeMath SVG dropped semantic carrier without exact GID");
  };
  if source.text != " " {
    return unsupported("OfficeMath SVG dropped semantic carrier other than one U+0020");
  }
  if tree_transform != source.transform {
    return unsupported("OfficeMath SVG dropped semantic transform mismatch");
  }
  let families = [usvg::fontdb::Family::Name(&source.font_family)];
  let font_id = tree
    .fontdb()
    .query(&usvg::fontdb::Query {
      families: &families,
      weight: source.font_weight,
      stretch: usvg::fontdb::Stretch::Normal,
      style: source.font_style,
    })
    .ok_or_else(|| {
      PdfError::Writer(format!(
        "OfficeMath SVG semantic font {:?} could not be resolved",
        source.font_family
      ))
    })?;
  let (handle, _, num_glyphs) =
    context
      .fonts
      .register_usvg_source_face(tree.fontdb(), font_id, source.font_size, || {
        context.refs.alloc()
      })?;
  if glyph_id >= num_glyphs {
    return Err(PdfError::Writer(format!(
      "OfficeMath SVG glyph ID {glyph_id} exceeds the selected font's {num_glyphs} glyphs"
    )));
  }
  let matrix = multiply_transform(
    pdf_transform(source.transform)?,
    [1.0, 0.0, 0.0, -1.0, 0.0, 0.0],
  );
  write_registered_glyph(
    content,
    context,
    OfficeMathGlyphWrite {
      handle,
      glyph_id,
      semantic_text: &source.text,
      font_size: source.font_size,
      matrix,
      color: source.color,
      opacity: source.opacity,
    },
  )
}

struct OfficeMathGlyphWrite<'a> {
  handle: super::direct_font::FontHandle,
  glyph_id: u32,
  semantic_text: &'a str,
  font_size: f32,
  matrix: [f32; 6],
  color: [u8; 3],
  opacity: f32,
}

fn write_registered_glyph(
  content: &mut Content,
  context: &mut OfficeMathWriteContext<'_>,
  glyph: OfficeMathGlyphWrite<'_>,
) -> Result<()> {
  super::direct::ensure_glyph_semantic_mapping_supported(
    glyph.semantic_text,
    context.requires_codepoint_mappings,
    context.forbids_private_use_mappings,
  )?;
  let registered =
    context
      .fonts
      .register_glyph(glyph.handle, glyph.glyph_id, Some(glyph.semantic_text))?;
  let (resource_name, font_ref) = context.fonts.resource(glyph.handle)?;
  let resource_name = resource_name.to_vec();
  context.resources.register_font(&resource_name, font_ref);

  content.save_state();
  content.set_fill_rgb(
    channel(glyph.color[0]),
    channel(glyph.color[1]),
    channel(glyph.color[2]),
  );
  super::direct::set_alpha(
    content,
    context.resources,
    context.refs,
    None,
    Some(opacity_byte(glyph.opacity)),
  )?;
  if registered.semantic_conflict {
    begin_actual_text(content, glyph.semantic_text);
  }
  let encoded = [(registered.cid >> 8) as u8, registered.cid as u8];
  content
    .begin_text()
    .set_text_rendering_mode(TextRenderingMode::Fill)
    .set_font(Name(&resource_name), glyph.font_size)
    .set_text_matrix(glyph.matrix)
    .show(Str(&encoded))
    .end_text();
  if registered.semantic_conflict {
    content.end_marked_content();
  }
  content.restore_state();
  Ok(())
}

fn office_math_text_marker(id: &str) -> Option<OfficeMathTextMarker> {
  if let Some(item_index) = id.strip_prefix(OFFICE_MATH_VISIBLE_GLYPH_PREFIX) {
    item_index.parse::<usize>().ok()?;
    return Some(OfficeMathTextMarker::Visible);
  }
  let marker = id.strip_prefix(OFFICE_MATH_SEMANTIC_GLYPH_PREFIX)?;
  if marker.parse::<usize>().is_ok() {
    return Some(OfficeMathTextMarker::Semantic {
      exact_glyph_id: None,
    });
  }
  let (item_index, glyph_id) = marker.rsplit_once("-gid-")?;
  item_index.parse::<usize>().ok()?;
  Some(OfficeMathTextMarker::Semantic {
    exact_glyph_id: Some(glyph_id.parse::<u32>().ok()?),
  })
}

fn append_tiny_path(content: &mut Content, path: &TinyPath) -> Result<()> {
  let mut current = None::<(f32, f32)>;
  let mut subpath_start = None::<(f32, f32)>;
  for segment in path.segments() {
    match segment {
      PathSegment::MoveTo(point) => {
        validate_point(point.x, point.y)?;
        content.move_to(point.x, point.y);
        current = Some((point.x, point.y));
        subpath_start = current;
      }
      PathSegment::LineTo(point) => {
        validate_point(point.x, point.y)?;
        content.line_to(point.x, point.y);
        current = Some((point.x, point.y));
      }
      PathSegment::QuadTo(control, end) => {
        validate_point(control.x, control.y)?;
        validate_point(end.x, end.y)?;
        let (x0, y0) = current.ok_or_else(|| {
          PdfError::Image("OfficeMath SVG quadratic path has no current point".to_string())
        })?;
        content.cubic_to(
          x0 + (control.x - x0) * (2.0 / 3.0),
          y0 + (control.y - y0) * (2.0 / 3.0),
          end.x + (control.x - end.x) * (2.0 / 3.0),
          end.y + (control.y - end.y) * (2.0 / 3.0),
          end.x,
          end.y,
        );
        current = Some((end.x, end.y));
      }
      PathSegment::CubicTo(control1, control2, end) => {
        validate_point(control1.x, control1.y)?;
        validate_point(control2.x, control2.y)?;
        validate_point(end.x, end.y)?;
        content.cubic_to(control1.x, control1.y, control2.x, control2.y, end.x, end.y);
        current = Some((end.x, end.y));
      }
      PathSegment::Close => {
        content.close_path();
        current = subpath_start;
      }
    }
  }
  Ok(())
}

fn validate_point(x: f32, y: f32) -> Result<()> {
  if x.is_finite() && y.is_finite() {
    Ok(())
  } else {
    Err(PdfError::Image(
      "OfficeMath SVG path contains a non-finite point".to_string(),
    ))
  }
}

fn solid_color(paint: &usvg::Paint) -> Result<usvg::Color> {
  match paint {
    usvg::Paint::Color(color) => Ok(*color),
    usvg::Paint::LinearGradient(_) | usvg::Paint::RadialGradient(_) | usvg::Paint::Pattern(_) => {
      Err(PdfError::DirectWriterUnsupported {
        feature: "OfficeMath SVG non-solid paint",
      })
    }
  }
}

fn pdf_transform(transform: Transform) -> Result<[f32; 6]> {
  let matrix = [
    transform.sx,
    transform.ky,
    transform.kx,
    transform.sy,
    transform.tx,
    transform.ty,
  ];
  if matrix.into_iter().all(f32::is_finite) {
    Ok(matrix)
  } else {
    Err(PdfError::Image(
      "OfficeMath SVG transform contains a non-finite value".to_string(),
    ))
  }
}

fn multiply_transform(left: [f32; 6], right: [f32; 6]) -> [f32; 6] {
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

fn begin_actual_text(content: &mut Content, text: &str) {
  let mut marked = content.begin_marked_content_with_properties(Name(b"Span"));
  let mut properties = marked.properties();
  properties.actual_text(TextStr(text));
  properties.finish();
  marked.finish();
}

fn is_identity(transform: Transform) -> bool {
  transform == Transform::identity()
}

fn channel(value: u8) -> f32 {
  f32::from(value) / f32::from(u8::MAX)
}

fn opacity_byte(value: f32) -> u8 {
  (value.clamp(0.0, 1.0) * f32::from(u8::MAX)).round() as u8
}

fn unsupported<T>(feature: &'static str) -> Result<T> {
  Err(PdfError::DirectWriterUnsupported { feature })
}

#[cfg(test)]
mod tests {
  use super::{OfficeMathTextMarker, office_math_text_marker};

  #[test]
  fn office_math_text_markers_are_closed_and_exact() {
    assert_eq!(
      office_math_text_marker("ooxmlsdk-math-visible-16"),
      Some(OfficeMathTextMarker::Visible)
    );
    assert_eq!(
      office_math_text_marker("ooxmlsdk-math-semantic-17"),
      Some(OfficeMathTextMarker::Semantic {
        exact_glyph_id: None
      })
    );
    assert_eq!(
      office_math_text_marker("ooxmlsdk-math-semantic-17-gid-3542"),
      Some(OfficeMathTextMarker::Semantic {
        exact_glyph_id: Some(3542)
      })
    );
    assert_eq!(office_math_text_marker("ooxmlsdk-math-visible-x"), None);
    assert_eq!(
      office_math_text_marker("ooxmlsdk-math-semantic-17-gid-x"),
      None
    );
    assert_eq!(office_math_text_marker("ordinary-svg-text"), None);
  }
}
