//! Import the standard SmartArt model when no persisted drawing is available.
//!
//! ECMA-376 DrawingML `relIds` identifies the data, layout, style and color
//! parts independently. The optional MS-ODRAWXML drawing records the last
//! successful layout; it is not a prerequisite for displaying the diagram.
//! Reuse the same model-to-shapes engine as PresentationML and LibreOffice's
//! SmartArtDiagram::createShapeHierarchyFromModel.

use super::*;
use crate::render::diagram as shared;
use ooxmlsdk::simple_type::CoordinateValue;

pub(super) fn layout_shapes(
  ids: &dgm::RelationshipIds,
  data: &dgm::DataModelRoot,
  placement: ImagePlacement,
  transform: DrawingMlGroupTransform,
  context: DrawingShapeImportContext<'_>,
) -> Option<Vec<InlineItem>> {
  let layout = context
    .images
    .diagram_layouts_by_relationship_id
    .get(ids.layout_part.as_str())?;
  let (width, height) = transform.fallback_size?;
  let styles = context
    .images
    .diagram_styles_by_relationship_id
    .get(ids.style_part.as_str())
    .map(|styles| shared::DiagramStyles {
      style_by_label: styles
        .style_label
        .iter()
        .filter_map(|label| Some((label.name.clone(), label.style.clone()?)))
        .collect(),
    });
  let colors = context
    .images
    .diagram_colors_by_relationship_id
    .get(ids.color_part.as_str())
    .map(|colors| style_colors(colors, &context.styles.theme_colors));
  let shapes = shared::layout_shapes(
    data,
    Some(layout),
    styles.as_ref(),
    colors.as_ref(),
    shared::DiagramBounds {
      x: 0.0,
      y: 0.0,
      width,
      height,
    },
    context.styles.theme_colors.accent1.unwrap_or(RgbColor {
      r: 0x4f,
      g: 0x81,
      b: 0xbd,
    }),
  );
  let children: Vec<_> = shapes
    .iter()
    .filter_map(|shape| {
      import_shape(
        shape,
        DrawingShapeImportContext {
          effect_extent: DrawingEffectExtent::default(),
          smartart_text_colors_by_model_id: None,
          ..context
        },
      )
      .map(InlineItem::Shape)
    })
    .collect();
  if children.is_empty() {
    return None;
  }

  Some(wrap_in_host(
    children,
    width,
    height,
    placement,
    context.effect_extent,
  ))
}

pub(super) fn wrap_in_host(
  children: Vec<InlineItem>,
  width: f32,
  height: f32,
  placement: ImagePlacement,
  effect_extent: DrawingEffectExtent,
) -> Vec<InlineItem> {
  // wp:extent belongs to the whole diagram. In particular an inline
  // diagram advances once, and an anchored diagram has one wrap owner.
  // Children use the established canvas coordinate-space formatter.
  let mut host = chart_shape(width, height, 0.0, placement, None);
  apply_drawing_effect_extent_to_shape(&mut host, effect_extent);
  // Like a WordprocessingCanvas, SmartArt has one DrawingML content origin
  // inside the inline effect frame (ECMA-376 Part 1 §20.4.2.6). The children
  // must start after the host's left/top outsets, not at the line-frame edge.
  host.wordprocessing_canvas_host = true;
  host.canvas_children = Some(children);
  vec![InlineItem::Shape(host)]
}

fn style_colors(
  definition: &dgm::ColorsDefinition,
  theme: &ThemeColors,
) -> shared::DiagramStyleColors {
  let mut colors = shared::DiagramStyleColors::default();
  for label in &definition.color_transform_style_label {
    if let Some(list) = &label.fill_color_list {
      let (fills, opacities): (Vec<_>, Vec<_>) = list
        .fill_color_list_choice
        .iter()
        .filter_map(Color::from_diagram_fill_color_choice)
        .filter_map(|color| resolved_docx_drawing_color(color, theme))
        .map(|color| (color.color, color.opacity))
        .unzip();
      if !fills.is_empty() {
        colors.fill_by_label.insert(label.name.clone(), fills);
        colors
          .fill_opacity_by_label
          .insert(label.name.clone(), opacities);
      }
    }
    if let Some(list) = &label.line_color_list {
      let (lines, opacities): (Vec<_>, Vec<_>) = list
        .line_color_list_choice
        .iter()
        .filter_map(Color::from_diagram_line_color_choice)
        .filter_map(|color| resolved_docx_drawing_color(color, theme))
        .map(|color| (color.color, color.opacity))
        .unzip();
      if !lines.is_empty() {
        colors.line_by_label.insert(label.name.clone(), lines);
        colors
          .line_opacity_by_label
          .insert(label.name.clone(), opacities);
      }
    }
    if let Some(list) = &label.text_fill_color_list {
      let fills: Vec<_> = list
        .text_fill_color_list_choice
        .iter()
        .filter_map(Color::from_diagram_text_fill_color_choice)
        .filter_map(|color| resolved_docx_drawing_color(color, theme))
        .map(|color| color.color)
        .collect();
      if !fills.is_empty() {
        colors.text_fill_by_label.insert(label.name.clone(), fills);
      }
    }
  }
  colors
}

fn rgb_color(color: RgbColor, opacity: f32) -> Box<a::RgbColorModelHex> {
  Box::new(a::RgbColorModelHex {
    val: format!("{:02X}{:02X}{:02X}", color.r, color.g, color.b),
    rgb_color_model_hex_choice: (opacity < 1.0)
      .then_some(a::RgbColorModelHexChoice::Alpha(a::Alpha {
        val: sdk_units::DrawingmlPercentageValue::Decimal(
          (opacity.clamp(0.0, 1.0) * 100_000.0).round() as i32,
        ),
      }))
      .into_iter()
      .collect(),
    ..Default::default()
  })
}

fn import_shape(
  source: &shared::DiagramShape,
  context: DrawingShapeImportContext<'_>,
) -> Option<InlineShape> {
  let mut properties = source
    .shape_properties
    .as_deref()
    .cloned()
    .map(shape_properties)
    .unwrap_or_default();
  // Layout atoms compute point coordinates. They replace data-model xfrm
  // bounds; authored geometry, paint and effects retain their typed values.
  let emu = |value: f32| CoordinateValue::Emu((f64::from(value) * 12_700.0).round() as i64);
  properties.transform2_d = Some(Box::new(a::Transform2D {
    offset: Some(a::Offset {
      x: emu(source.x),
      y: emu(source.y),
    }),
    extents: Some(a::Extents {
      cx: emu(source.width),
      cy: emu(source.height),
    }),
    rotation: Some((source.shape_rotation_deg * 60_000.0).round() as i32),
    ..properties
      .transform2_d
      .as_deref()
      .cloned()
      .unwrap_or_default()
  }));
  if properties.shape_properties_choice1.is_none() {
    properties.shape_properties_choice1 = source
      .preset_geometry
      .clone()
      .map(dsp::ShapePropertiesChoice::PresetGeometry);
  }
  let connector = source.is_connector
    && source.connector_dimension == dgm::ConnectorDimensionValues::OneDimension;
  if !source.draw_geometry || connector {
    properties.shape_properties_choice2 =
      Some(dsp::ShapePropertiesChoice2::NoFill(a::NoFill::default()));
  } else if properties.shape_properties_choice2.is_none() {
    properties.shape_properties_choice2 = Some(dsp::ShapePropertiesChoice2::SolidFill(Box::new(
      a::SolidFill {
        solid_fill_choice: Some(a::SolidFillChoice::RgbColorModelHex(rgb_color(
          source.fill,
          source.fill_opacity,
        ))),
        ..Default::default()
      },
    )));
  }
  if !source.draw_geometry {
    properties.outline = Some(Box::new(a::Outline {
      outline_choice1: Some(a::OutlineChoice::NoFill(a::NoFill::default())),
      ..Default::default()
    }));
  }
  let shape_style = source.style.as_deref().map(|style| {
    let mut style = dsp::ShapeStyle {
      line_reference: style.line_reference.clone(),
      fill_reference: style.fill_reference.clone(),
      effect_reference: style.effect_reference.clone(),
      font_reference: style.font_reference.clone(),
    };
    if let Some(color) = source.line_fill {
      style.line_reference.line_reference_choice = Some(a::LineReferenceChoice::RgbColorModelHex(
        rgb_color(color, source.line_opacity),
      ));
    }
    if let Some(color) = source.text_fill {
      style.font_reference.font_reference_choice = Some(a::FontReferenceChoice::RgbColorModelHex(
        rgb_color(color, 1.0),
      ));
    }
    Box::new(style)
  });
  let shape = dsp::Shape {
    model_id: "layout".into(),
    shape_properties: Box::new(properties),
    shape_style,
    text_body: (!source.text_body.is_empty()).then(|| Box::new(text_body(source))),
    ..Default::default()
  };
  let text_colors = source
    .text_fill
    .map(|color| HashMap::from([("layout".into(), color)]));
  let mut imported = drawingml_diagram_shape_shape(
    &shape,
    ImagePlacement::Inline,
    DrawingMlGroupTransform::identity(),
    DrawingShapeImportContext {
      smartart_text_colors_by_model_id: text_colors.as_ref(),
      ..context
    },
  )?;
  if connector {
    let commands = common::drawingml_geometry::transform_commands(
      source.connector_commands(),
      Affine::translate((-f64::from(source.x), -f64::from(source.y))),
    );
    imported.geometry = InlineShapeGeometry::Path {
      paths: vec![common::DrawingPath {
        commands,
        fill_mode: common::DrawingPathFillMode::None,
        ..Default::default()
      }],
      outline: None,
      preset: None,
    };
    if let Some(stroke) = imported.stroke_override.as_deref_mut() {
      source.apply_connector_ends(stroke);
    }
  }
  Some(imported)
}

fn shape_properties(source: dgm::ShapeProperties) -> dsp::ShapeProperties {
  dsp::ShapeProperties {
    black_white_mode: source.black_white_mode,
    transform2_d: source.transform2_d,
    shape_properties_choice1: source.shape_properties_choice1.map(|value| match value {
      dgm::ShapePropertiesChoice::CustomGeometry(value) => {
        dsp::ShapePropertiesChoice::CustomGeometry(value)
      }
      dgm::ShapePropertiesChoice::PresetGeometry(value) => {
        dsp::ShapePropertiesChoice::PresetGeometry(value)
      }
    }),
    shape_properties_choice2: source.shape_properties_choice2.map(|value| match value {
      dgm::ShapePropertiesChoice2::NoFill(value) => dsp::ShapePropertiesChoice2::NoFill(value),
      dgm::ShapePropertiesChoice2::SolidFill(value) => {
        dsp::ShapePropertiesChoice2::SolidFill(value)
      }
      dgm::ShapePropertiesChoice2::GradientFill(value) => {
        dsp::ShapePropertiesChoice2::GradientFill(value)
      }
      dgm::ShapePropertiesChoice2::BlipFill(value) => dsp::ShapePropertiesChoice2::BlipFill(value),
      dgm::ShapePropertiesChoice2::PatternFill(value) => {
        dsp::ShapePropertiesChoice2::PatternFill(value)
      }
      dgm::ShapePropertiesChoice2::GroupFill => dsp::ShapePropertiesChoice2::GroupFill,
    }),
    outline: source.outline,
    shape_properties_choice3: source.shape_properties_choice3.map(|value| match value {
      dgm::ShapePropertiesChoice3::EffectList(value) => {
        dsp::ShapePropertiesChoice3::EffectList(value)
      }
      dgm::ShapePropertiesChoice3::EffectDag(value) => {
        dsp::ShapePropertiesChoice3::EffectDag(value)
      }
    }),
    scene3_d_type: source.scene3_d_type,
    shape3_d_type: source.shape3_d_type,
    shape_properties_extension_list: source.shape_properties_extension_list,
  }
}

fn text_body(shape: &shared::DiagramShape) -> dsp::TextBody {
  dsp::TextBody {
    body_properties: shape.text_body.body_properties.clone().unwrap_or_default(),
    list_style: shape.text_body.list_style.clone(),
    paragraph: shape
      .text_body
      .paragraphs
      .iter()
      .map(|paragraph| {
        let mut properties = paragraph.paragraph_properties.clone();
        if let Some(size) = shape.font_size_pt {
          let properties = properties.get_or_insert_with(Default::default);
          properties
            .default_run_properties
            .get_or_insert_with(Default::default)
            .font_size
            .get_or_insert((size * 100.0).round() as i32);
        }
        a::Paragraph {
          paragraph_properties: properties,
          end_paragraph_run_properties: paragraph.end_paragraph_run_properties.clone(),
          paragraph_choice: paragraph
            .runs
            .iter()
            .map(|run| match run.kind {
              shared::DiagramTextRunKind::Break => a::ParagraphChoice::Break(Box::new(a::Break {
                run_properties: run.run_properties.clone(),
              })),
              shared::DiagramTextRunKind::Field => a::ParagraphChoice::Field(Box::new(a::Field {
                r#type: run.field_type.clone(),
                run_properties: run.run_properties.clone(),
                paragraph_properties: run.field_paragraph_properties.clone(),
                text: Some(run.text.clone()),
                ..Default::default()
              })),
              _ => a::ParagraphChoice::Run(Box::new(a::Run {
                run_properties: run.run_properties.clone(),
                text: run.text.clone(),
              })),
            })
            .collect(),
          ..Default::default()
        }
      })
      .collect(),
  }
}

pub(super) fn apply_paragraph_spacing(
  format: &mut ParagraphFormat,
  properties: &a::ParagraphProperties,
  font_size_pt: f32,
  first: bool,
  last: bool,
) {
  // ECMA-376 §21.1.2.2.5/10/11: percentage line spacing is relative to the
  // natural line; paragraph spacing percentages refer to the text size.
  if let Some(spacing) = properties
    .line_spacing
    .as_ref()
    .and_then(|spacing| spacing.line_spacing_choice.as_ref())
  {
    let (value, rule) = match spacing {
      a::LineSpacingChoice::SpacingPercent(spacing) => {
        (spacing.val.as_ratio() as f32, LineHeightRule::Auto)
      }
      a::LineSpacingChoice::SpacingPoints(spacing) => (
        sdk_units::points100_to_points(spacing.val) as f32,
        LineHeightRule::Exact,
      ),
    };
    format.line_height_pt = Some(value);
    format.line_height_rule = rule;
    format.line_height_set = true;
  }
  // Word SmartArt suppresses the first paragraph's preceding space and the
  // last paragraph's following space, independently of vertical anchoring.
  if !first
    && let Some(spacing) = properties
      .space_before
      .as_ref()
      .and_then(|spacing| spacing.space_before_choice.as_ref())
  {
    format.spacing_before_pt = match spacing {
      a::SpaceBeforeChoice::SpacingPercent(spacing) => font_size_pt * spacing.val.as_ratio() as f32,
      a::SpaceBeforeChoice::SpacingPoints(spacing) => {
        sdk_units::points100_to_points(spacing.val) as f32
      }
    };
    format.spacing_before_set = true;
  }
  if !last
    && let Some(spacing) = properties
      .space_after
      .as_ref()
      .and_then(|spacing| spacing.space_after_choice.as_ref())
  {
    format.spacing_after_pt = match spacing {
      a::SpaceAfterChoice::SpacingPercent(spacing) => font_size_pt * spacing.val.as_ratio() as f32,
      a::SpaceAfterChoice::SpacingPoints(spacing) => {
        sdk_units::points100_to_points(spacing.val) as f32
      }
    };
    format.spacing_after_set = true;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn smartart_line_default_preserves_explicit_and_inherited_widths() {
    let styles = StylesCatalog {
      theme_lines: ThemeLineStyles {
        outlines: vec![a::Outline::from_bytes(br#"<a:ln xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" w="25400"><a:solidFill><a:srgbClr val="000000"/></a:solidFill></a:ln>"#).unwrap()],
      },
      ..Default::default()
    };
    for (line, reference, expected) in [
      (
        "<a:ln><a:solidFill><a:srgbClr val=\"123456\"/></a:solidFill></a:ln>",
        0,
        Some(0.75),
      ),
      (
        "<a:ln w=\"0\"><a:solidFill><a:srgbClr val=\"123456\"/></a:solidFill></a:ln>",
        0,
        Some(0.0),
      ),
      (
        "<a:ln w=\"38100\"><a:solidFill><a:srgbClr val=\"123456\"/></a:solidFill></a:ln>",
        0,
        Some(3.0),
      ),
      (
        "<a:ln><a:solidFill><a:srgbClr val=\"123456\"/></a:solidFill></a:ln>",
        1,
        Some(2.0),
      ),
      ("<a:ln><a:noFill/></a:ln>", 1, None),
      ("", 0, None),
    ] {
      let shape = dsp::Shape::from_bytes(format!(r#"<dsp:sp xmlns:dsp="http://schemas.microsoft.com/office/drawing/2008/diagram" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" modelId="1"><dsp:nvSpPr><dsp:cNvPr id="1" name="outline"/><dsp:cNvSpPr/></dsp:nvSpPr><dsp:spPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="914400" cy="914400"/></a:xfrm><a:solidFill><a:srgbClr val="FFFFFF"/></a:solidFill>{line}</dsp:spPr><dsp:style><a:lnRef idx="{reference}"/><a:fillRef idx="0"/><a:effectRef idx="0"/><a:fontRef idx="minor"/></dsp:style></dsp:sp>"#).as_bytes()).unwrap();
      let imported = drawingml_diagram_shape_shape(
        &shape,
        ImagePlacement::Inline,
        DrawingMlGroupTransform::identity(),
        DrawingShapeImportContext {
          effect_extent: DrawingEffectExtent::default(),
          styles: &styles,
          images: &ImageCatalog::default(),
          hyperlinks: &HyperlinkCatalog::default(),
          smartart_text_colors_by_model_id: None,
          wordprocessing_canvas_has_background_paint: false,
        },
      )
      .unwrap();
      assert_eq!(
        imported
          .stroke_override
          .as_ref()
          .map(|stroke| stroke.width.0),
        expected
      );
      assert_eq!(imported.stroke.map(|stroke| stroke.width_pt), expected);
    }
  }

  #[test]
  fn smartart_paragraph_spacing_keeps_percent_and_point_owners() {
    for (line, expected, rule) in [
      ("<a:spcPct val=\"90000\"/>", 0.9, LineHeightRule::Auto),
      ("<a:spcPts val=\"1800\"/>", 18.0, LineHeightRule::Exact),
    ] {
      let properties = a::ParagraphProperties::from_bytes(format!(r#"<a:pPr xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"><a:lnSpc>{line}</a:lnSpc><a:spcBef><a:spcPct val="25000"/></a:spcBef><a:spcAft><a:spcPts val="300"/></a:spcAft></a:pPr>"#).as_bytes()).unwrap();
      for (first, last) in [(true, true), (false, false)] {
        let mut format = ParagraphFormat::default();
        apply_paragraph_spacing(&mut format, &properties, 9.0, first, last);
        assert_eq!(format.line_height_pt, Some(expected));
        assert_eq!(format.line_height_rule, rule);
        assert_eq!(format.spacing_before_pt, if first { 0.0 } else { 2.25 });
        assert_eq!(format.spacing_after_pt, if last { 0.0 } else { 3.0 });
      }
    }
  }

  #[test]
  fn smartart_theme_effect_reference_yields_to_direct_effect_lists() {
    let effect_styles = a::EffectStyleList::from_bytes(
      br#"<a:effectStyleLst xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"><a:effectStyle><a:effectLst><a:outerShdw dist="12700"><a:schemeClr val="phClr"/></a:outerShdw></a:effectLst></a:effectStyle></a:effectStyleLst>"#,
    )
    .unwrap();
    let styles = StylesCatalog {
      theme_effects: ThemeEffectStyles {
        styles: effect_styles.effect_style,
      },
      ..Default::default()
    };
    for (direct, inherited) in [("", true), ("<a:effectLst/>", false)] {
      let xml = format!(
        r#"<dsp:sp xmlns:dsp="http://schemas.microsoft.com/office/drawing/2008/diagram" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" modelId="1"><dsp:nvSpPr><dsp:cNvPr id="1" name="effect test"/><dsp:cNvSpPr/></dsp:nvSpPr><dsp:spPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="914400" cy="914400"/></a:xfrm><a:solidFill><a:srgbClr val="FFFFFF"/></a:solidFill>{direct}</dsp:spPr><dsp:style><a:lnRef idx="0"/><a:fillRef idx="0"/><a:effectRef idx="1"><a:srgbClr val="123456"/></a:effectRef><a:fontRef idx="minor"/></dsp:style></dsp:sp>"#
      );
      let shape = dsp::Shape::from_bytes(xml.as_bytes()).unwrap();
      let imported = drawingml_diagram_shape_shape(
        &shape,
        ImagePlacement::Inline,
        DrawingMlGroupTransform::identity(),
        DrawingShapeImportContext {
          effect_extent: DrawingEffectExtent::default(),
          styles: &styles,
          images: &ImageCatalog::default(),
          hyperlinks: &HyperlinkCatalog::default(),
          smartart_text_colors_by_model_id: None,
          wordprocessing_canvas_has_background_paint: false,
        },
      )
      .unwrap();
      let Some(common::DrawingEffectSource::List {
        resolved: Some(effects),
        ..
      }) = imported.effects
      else {
        panic!("resolved direct or inherited effect list");
      };
      assert_eq!(!effects.effects.is_empty(), inherited);
      if inherited {
        assert!(effects.effects.iter().any(|effect| matches!(
          effect,
          common::drawingml_image_effects::ImageEffect::OuterShadow { color, .. }
            if color.color == RgbColor { r: 0x12, g: 0x34, b: 0x56 }
        )));
      }
    }
  }

  fn diagram_catalog(count: usize) -> ImageCatalog {
    let mut points = String::from(
      r#"<dgm:pt modelId="0" type="doc"/><dgm:pt modelId="100" type="pres"><dgm:prSet presAssocID="0" presName="diagram"/></dgm:pt>"#,
    );
    let mut connections = String::from(
      r#"<dgm:cxn modelId="0" type="presOf" srcId="0" destId="100" srcOrd="0" destOrd="0"/>"#,
    );
    for i in 1..=count {
      let presentation = 100 + i;
      points.push_str(&format!(
        r#"<dgm:pt modelId="{i}"/><dgm:pt modelId="{presentation}" type="pres"><dgm:prSet presAssocID="{i}" presName="node" presStyleLbl="node0"/></dgm:pt>"#,
      ));
      connections.push_str(&format!(
        r#"<dgm:cxn modelId="{i}" type="parOf" srcId="0" destId="{i}" srcOrd="{i}" destOrd="0"/><dgm:cxn modelId="{presentation}" type="presOf" srcId="{i}" destId="{presentation}" srcOrd="0" destOrd="0"/><dgm:cxn modelId="{}" type="presParOf" srcId="100" destId="{presentation}" srcOrd="{i}" destOrd="0"/>"#,
        200 + i,
      ));
    }
    let data = dgm::DataModelRoot::from_bytes(format!(
      r#"<dgm:dataModel xmlns:dgm="http://schemas.openxmlformats.org/drawingml/2006/diagram"><dgm:ptLst>{points}</dgm:ptLst><dgm:cxnLst>{connections}</dgm:cxnLst></dgm:dataModel>"#,
    ).as_bytes()).expect("diagram data");
    let layout = dgm::LayoutDefinition::from_bytes(
      br#"
      <dgm:layoutDef xmlns:dgm="http://schemas.openxmlformats.org/drawingml/2006/diagram">
        <dgm:layoutNode name="diagram">
          <dgm:alg type="lin"><dgm:param type="linDir" val="fromL"/></dgm:alg>
          <dgm:shape/><dgm:presOf/>
          <dgm:forEach axis="ch" ptType="node"><dgm:layoutNode name="node">
            <dgm:alg type="tx"/><dgm:shape type="rect"/><dgm:presOf axis="self" ptType="node"/>
          </dgm:layoutNode></dgm:forEach>
        </dgm:layoutNode>
      </dgm:layoutDef>"#,
    )
    .expect("diagram layout");
    let colors = dgm::ColorsDefinition::from_bytes(br#"
      <dgm:colorsDef xmlns:dgm="http://schemas.openxmlformats.org/drawingml/2006/diagram" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
        <dgm:styleLbl name="node0"><dgm:fillClrLst><a:srgbClr val="123456"><a:alpha val="50000"/></a:srgbClr></dgm:fillClrLst></dgm:styleLbl>
      </dgm:colorsDef>"#).expect("diagram colors");
    ImageCatalog {
      diagram_data_by_relationship_id: HashMap::from([("data".into(), data)]),
      diagram_layouts_by_relationship_id: HashMap::from([("layout".into(), layout)]),
      diagram_colors_by_relationship_id: HashMap::from([("colors".into(), colors)]),
      ..Default::default()
    }
  }

  fn import(catalog: &ImageCatalog) -> Vec<InlineItem> {
    drawing_diagram_shapes(
      &dgm::RelationshipIds {
        data_part: "data".into(),
        layout_part: "layout".into(),
        color_part: "colors".into(),
        ..Default::default()
      },
      ImagePlacement::Inline,
      DrawingMlGroupTransform::identity().with_fallback_size(Some((120.0, 60.0))),
      DrawingShapeImportContext {
        effect_extent: DrawingEffectExtent {
          left_pt: 3.0,
          ..Default::default()
        },
        styles: &StylesCatalog::default(),
        images: catalog,
        hyperlinks: &HyperlinkCatalog::default(),
        smartart_text_colors_by_model_id: None,
        wordprocessing_canvas_has_background_paint: false,
      },
    )
    .expect("diagram import")
  }

  #[test]
  fn smartart_without_persisted_drawing_imports_model_children_in_one_host() {
    for count in [2, 3] {
      let items = import(&diagram_catalog(count));
      let [InlineItem::Shape(host)] = items.as_slice() else {
        panic!("one host owns wp:extent");
      };
      assert_eq!(
        (host.width_pt, host.height_pt, host.effect_left_pt),
        (120.0, 60.0, 3.0)
      );
      assert!(host.wordprocessing_canvas_host);
      let children = host.canvas_children.as_ref().expect("layout children");
      assert_eq!(children.len(), count);
      let mut prior_x = None;
      for child in children {
        let InlineItem::Shape(child) = child else {
          panic!("rectangle child");
        };
        assert!(child.width_pt > 0.0 && child.height_pt > 0.0);
        assert_eq!(
          child.effect_left_pt, 0.0,
          "wp effects apply only to the host"
        );
        assert_eq!(
          child.fill_color,
          Some(RgbColor {
            r: 0x12,
            g: 0x34,
            b: 0x56
          })
        );
        let Some(common::Fill::Solid(fill)) = child.fill_override.as_deref() else {
          panic!("resolved fill");
        };
        assert_eq!(fill.a, 128);
        if let Some(x) = prior_x {
          assert!(child.offset_x_pt > x);
        }
        prior_x = Some(child.offset_x_pt);
      }
    }
  }

  #[test]
  fn persisted_smartart_drawing_precedes_available_layout_model() {
    let mut catalog = diagram_catalog(3);
    let extension = dgm::DataModelExtensionList::from_bytes(br#"
      <dgm:extLst xmlns:dgm="http://schemas.openxmlformats.org/drawingml/2006/diagram" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:dsp="http://schemas.microsoft.com/office/drawing/2008/diagram">
        <a:ext uri="{C3380CC4-5D6E-409C-BE32-E72D297353CC}"><dsp:dataModelExt relId="drawing"/></a:ext>
      </dgm:extLst>"#).expect("persisted drawing extension");
    catalog
      .diagram_data_by_relationship_id
      .get_mut("data")
      .expect("data")
      .data_model_extension_list = Some(extension);
    let drawing = dsp::Drawing::from_bytes(br#"
      <dsp:drawing xmlns:dsp="http://schemas.microsoft.com/office/drawing/2008/diagram" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
        <dsp:spTree><dsp:nvGrpSpPr><dsp:cNvPr id="0" name="diagram"/><dsp:cNvGrpSpPr/></dsp:nvGrpSpPr><dsp:grpSpPr/>
          <dsp:sp modelId="1"><dsp:nvSpPr><dsp:cNvPr id="1" name="persisted rectangle"/><dsp:cNvSpPr/></dsp:nvSpPr><dsp:spPr>
            <a:xfrm><a:off x="12700" y="25400"/><a:ext cx="152400" cy="101600"/></a:xfrm>
            <a:prstGeom prst="rect"><a:avLst/></a:prstGeom><a:solidFill><a:srgbClr val="FF0000"/></a:solidFill>
          </dsp:spPr></dsp:sp>
        </dsp:spTree>
      </dsp:drawing>"#).expect("persisted drawing");
    catalog.diagram_drawings_by_relationship_id.insert(
      "drawing".into(),
      crate::docx::package::DiagramDrawingResource {
        drawing,
        images: ImageCatalog::default(),
      },
    );
    let items = import(&catalog);
    let [InlineItem::Shape(host)] = items.as_slice() else {
      panic!("one persisted diagram host");
    };
    assert_eq!(
      (host.width_pt, host.height_pt, host.effect_left_pt),
      (120.0, 60.0, 3.0)
    );
    let [InlineItem::Shape(shape)] = host.canvas_children.as_deref().unwrap() else {
      panic!("persisted shape, not the three regenerated model nodes");
    };
    assert_eq!(
      shape.effect_left_pt, 0.0,
      "effect extent belongs to the host"
    );
    assert_eq!((shape.width_pt, shape.height_pt), (12.0, 8.0));
    assert_eq!((shape.offset_x_pt, shape.offset_y_pt), (1.0, 2.0));
    assert_eq!(shape.fill_color, Some(RgbColor { r: 255, g: 0, b: 0 }));
  }

  #[test]
  fn persisted_smartart_images_resolve_in_the_drawing_part_only() {
    let mut catalog = diagram_catalog(3);
    let image = |bytes: &'static [u8]| crate::docx::package::ImageResource {
      data: Bytes::from_static(bytes),
      content_type: Some("image/png".into()),
    };
    catalog
      .by_relationship_id
      .insert("rId1".into(), image(b"main story image"));
    let drawing = dsp::Drawing::from_bytes(
      br#"<dsp:drawing xmlns:dsp="http://schemas.microsoft.com/office/drawing/2008/diagram"
        xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
        xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
        <dsp:spTree><dsp:nvGrpSpPr><dsp:cNvPr id="0" name="diagram"/>
          <dsp:cNvGrpSpPr/></dsp:nvGrpSpPr><dsp:grpSpPr/>
          <dsp:sp modelId="1"><dsp:nvSpPr><dsp:cNvPr id="1" name="picture"/>
            <dsp:cNvSpPr/></dsp:nvSpPr><dsp:spPr>
            <a:xfrm><a:off x="12700" y="25400"/><a:ext cx="152400" cy="101600"/></a:xfrm>
            <a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
            <a:blipFill><a:blip r:embed="rId1"/><a:stretch><a:fillRect/></a:stretch></a:blipFill>
          </dsp:spPr></dsp:sp>
        </dsp:spTree>
      </dsp:drawing>"#,
    )
    .unwrap();
    for (id, bytes) in [
      ("first", Some(b"first drawing image".as_slice())),
      ("second", Some(b"second drawing image".as_slice())),
      ("missing", None),
    ] {
      let images = ImageCatalog {
        by_relationship_id: bytes
          .map(|bytes| ("rId1".into(), image(bytes)))
          .into_iter()
          .collect(),
        ..Default::default()
      };
      catalog.diagram_drawings_by_relationship_id.insert(
        id.into(),
        crate::docx::package::DiagramDrawingResource {
          drawing: drawing.clone(),
          images,
        },
      );
      catalog
        .diagram_data_by_relationship_id
        .get_mut("data")
        .unwrap()
        .data_model_extension_list = Some(
        dgm::DataModelExtensionList::from_bytes(format!(
          r#"<dgm:extLst xmlns:dgm="http://schemas.openxmlformats.org/drawingml/2006/diagram"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
            xmlns:dsp="http://schemas.microsoft.com/office/drawing/2008/diagram">
            <a:ext uri="{{C3380CC4-5D6E-409C-BE32-E72D297353CC}}"><dsp:dataModelExt relId="{id}"/></a:ext>
          </dgm:extLst>"#,
        ).as_bytes()).unwrap(),
      );
      let items = import(&catalog);
      let [InlineItem::Shape(host)] = items.as_slice() else {
        panic!("one inline drawing owner");
      };
      let children = host.canvas_children.as_deref().unwrap();
      if bytes.is_none() {
        assert!(
          children.is_empty(),
          "an unresolved, otherwise unpainted child must not borrow the main story image"
        );
        continue;
      }
      let [InlineItem::Shape(child)] = children else {
        panic!("persisted picture child: {id}");
      };
      assert_eq!(
        child.fill_image.as_ref().map(|fill| fill.data.as_ref()),
        bytes,
        "part-scoped relationship {id} must not alias another drawing or the main story"
      );
    }
  }
}
