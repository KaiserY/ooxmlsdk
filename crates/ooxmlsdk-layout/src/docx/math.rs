use std::fmt::Write as _;
use std::sync::Arc;

use bytes::Bytes;
use ooxmlsdk::schemas::{m, schemas_openxmlformats_org_wordprocessingml_2006_main as w};
use ooxmlsdk_fonts::FeatureValue;
use skrifa::{
  FontRef, GlyphId, MetadataProvider,
  instance::{LocationRef, Size},
  outline::{DrawSettings, OutlinePen},
  raw::{FontData, FontRead, TableProvider, tables::layout::CoverageTable, types::Tag},
  string::StringId,
};
use unicode_bidi::{BidiDataSource, HardcodedBidiData};
use unicode_math_class::{MathClass, class as unicode_math_class};

use super::math_type::MathTypeEquation;
use super::{
  ImageCrop, ImagePlacement, InlineImage, InlineImageLineBox, MathWrapContinuation,
  OfficeMathBreakKind, OfficeMathDisplayLayout, OfficeMathLineFragment, OfficeMathLineLayout,
  RgbColor, StylesCatalog, TextStyle, office_math_display_alignment, properties,
};
use crate::fonts::{FontFaceData, materialize_wordprocessingml_source_font_slot};
use crate::render::math as shared_math;
use crate::text_metrics::{MathFontMetrics, ShapedText, TextMetrics};

const OFFICE_MATH_SVG_CONTENT_TYPE: &str = "application/vnd.ooxmlsdk.office-math+xml";
const MIN_MATH_SIZE_PT: f32 = 1.0;
const MIN_RULE_WIDTH_PT: f32 = 0.2;
const MAX_MATH_ASSEMBLY_EXTENDER_REPEATS: usize = 1024;
const MATH_THIN_SPACE_EM: f32 = 1.0 / 6.0;
const MATH_MEDIUM_SPACE_EM: f32 = 2.0 / 9.0;
const MATH_THICK_SPACE_EM: f32 = 5.0 / 18.0;
const MATH_VERY_THICK_SPACE_EM: f32 = 1.0 / 3.0;
const MATH_CLAUSE_SEPARATOR_SPACE_EM: f32 = 2.0;

pub(super) fn is_office_math_content_type(content_type: Option<&str>) -> bool {
  content_type
    .is_some_and(|content_type| content_type.eq_ignore_ascii_case(OFFICE_MATH_SVG_CONTENT_TYPE))
}

#[derive(Clone, Debug)]
pub(super) struct MathTypeSemanticRun {
  pub(super) text: String,
  pub(super) x: f32,
  pub(super) baseline_y: f32,
  pub(super) font_size: Option<f32>,
  pub(super) font_family: Option<String>,
  pub(super) bold: bool,
  pub(super) italic: bool,
  pub(super) width: Option<f32>,
  pub(super) advances: Option<Vec<f32>>,
}

/// Reconstructs MathType's searchable replacement layer from the editable
/// MTEF tree and the associated WMF text records.
///
/// MTEF validates that this is an editable MathType equation and supplies its
/// logical content. The static WMF owns PDF-facing font encodings, record
/// order, `MoveTo`/`TA_UPDATECP` positions, and `ExtTextOut` `Dx` advances.
/// Those authored WMF values must remain authoritative for PDF geometry.
pub(super) fn math_type_semantic_runs(
  equation: &MathTypeEquation,
  preview_data: &[u8],
  preview_content_type: Option<&str>,
) -> Vec<MathTypeSemanticRun> {
  let Some(_document) = equation.mtef5_document() else {
    let text = equation.semantic_text();
    return (!text.is_empty())
      .then_some(MathTypeSemanticRun {
        text,
        x: 0.0,
        baseline_y: 0.5,
        font_size: None,
        font_family: Some("Times New Roman".to_string()),
        bold: false,
        italic: false,
        width: None,
        advances: None,
      })
      .into_iter()
      .collect();
  };
  let preview_runs =
    crate::render::emf_wmf::extract_metafile_text_runs(preview_data, preview_content_type, false);
  if preview_runs.is_empty() {
    return Vec::new();
  }

  let mut output = Vec::new();
  for preview in preview_runs {
    let text = if preview
      .font_family
      .as_deref()
      .is_some_and(|family| family.eq_ignore_ascii_case("Symbol"))
      && !preview.italic
    {
      preview
        .text
        .chars()
        .map(|character| {
          // Keep Symbol's transport code for minus so shaping stays in the
          // selected Symbol face. The PDF renderer supplies U+2212 through
          // ToUnicode while retaining the authored F02D glyph. Other ASCII
          // operators can use their semantic scalar because Symbol transport
          // shaping maps them back to F0XX before glyph selection.
          if character == '\u{f02d}' {
            character
          } else {
            crate::render::symbol::font_symbol_code(Some("Symbol"), character as u32)
              .unwrap_or(character)
          }
        })
        .collect()
    } else {
      preview.text
    };
    output.push(MathTypeSemanticRun {
      text,
      x: preview.x,
      baseline_y: preview.y,
      font_size: preview.font_size,
      font_family: preview.font_family,
      bold: preview.bold,
      italic: preview.italic,
      width: preview.width,
      advances: preview.advances,
    });
  }
  output
}

macro_rules! parse_math_choice {
  ($parser:expr, $choice_type:ident, $choice:expr) => {
    match $choice {
      m::$choice_type::Accent(value) => $parser.accent(value),
      m::$choice_type::Bar(value) => $parser.bar(value),
      m::$choice_type::Box(value) => $parser.box_object(value),
      m::$choice_type::BorderBox(value) => $parser.border_box(value),
      m::$choice_type::Delimiter(value) => $parser.delimiter(value),
      m::$choice_type::EquationArray(value) => $parser.equation_array(value),
      m::$choice_type::Fraction(value) => $parser.fraction(value),
      m::$choice_type::MathFunction(value) => $parser.math_function(value),
      m::$choice_type::GroupChar(value) => $parser.group_char(value),
      m::$choice_type::LimitLower(value) => $parser.limit_lower(value),
      m::$choice_type::LimitUpper(value) => $parser.limit_upper(value),
      m::$choice_type::Matrix(value) => $parser.matrix(value),
      m::$choice_type::Nary(value) => $parser.nary(value),
      m::$choice_type::Phantom(value) => $parser.phantom(value),
      m::$choice_type::Radical(value) => $parser.radical(value),
      m::$choice_type::PreSubSuper(value) => $parser.pre_sub_super(value),
      m::$choice_type::Subscript(value) => $parser.subscript(value),
      m::$choice_type::SubSuperscript(value) => $parser.sub_superscript(value),
      m::$choice_type::Superscript(value) => $parser.superscript(value),
      m::$choice_type::Run(value) => $parser.run(value),
      m::$choice_type::Paragraph(value) => $parser.paragraph(value),
      m::$choice_type::OfficeMath(value) => $parser.office_math(value),
      _ => MathNode::Empty,
    }
  };
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ManualMathBreak {
  align_at: Option<u8>,
}

#[derive(Clone, Debug)]
struct RootMathPart {
  node: MathNode,
  manual_break: Option<ManualMathBreak>,
  /// The first atom of every `m:oMath` after the first one in an explicit
  /// `m:oMathPara`. ECMA-376 Part 1 §22.1.2.78 makes those separate display
  /// equations, not adjacent atoms on the same physical line.
  equation_start: bool,
}

fn manual_math_break(value: Option<&m::Break>) -> Option<ManualMathBreak> {
  let value = value?;
  Some(ManualMathBreak {
    // Transitional producers have used m:val for the same operator index.
    // Preserve both schema attributes and give the normative m:alnAt form
    // precedence when both are present.
    align_at: value
      .align_at
      .or(value.val)
      .and_then(|value| u8::try_from(value).ok()),
  })
}

fn math_run_manual_break(run: &m::Run) -> Option<ManualMathBreak> {
  manual_math_break(
    run
      .math_run_properties
      .as_deref()
      .and_then(|properties| properties.r#break.as_ref()),
  )
}

fn math_box_manual_break(value: &m::Box) -> Option<ManualMathBreak> {
  manual_math_break(
    value
      .box_properties
      .as_deref()
      .and_then(|properties| properties.r#break.as_ref()),
  )
}

fn office_math_root_parts(
  parser: &MathParser<'_>,
  value: &m::OfficeMath,
  equation_start: bool,
) -> Vec<RootMathPart> {
  let mut parts = value
    .office_math_choice
    .iter()
    .map(|choice| RootMathPart {
      manual_break: match choice {
        m::OfficeMathChoice::Run(run) => math_run_manual_break(run),
        m::OfficeMathChoice::Box(value) => math_box_manual_break(value),
        _ => None,
      },
      node: parse_math_choice!(parser, OfficeMathChoice, choice),
      equation_start: false,
    })
    .collect::<Vec<_>>();
  if equation_start
    && let Some(first_visible) = parts
      .iter_mut()
      .find(|part| !matches!(part.node, MathNode::Empty))
  {
    first_visible.equation_start = true;
  }
  parts
}

fn wordprocessing_math_root_parts(
  parser: &MathParser<'_>,
  choice: &w::ParagraphChoice,
) -> Vec<RootMathPart> {
  match choice {
    w::ParagraphChoice::Paragraph(paragraph) => {
      let mut parts = Vec::new();
      let mut has_equation = false;
      for choice in &paragraph.paragraph_choice {
        match choice {
          m::ParagraphChoice::OfficeMath(value) => {
            let start = parts.len();
            parts.extend(office_math_root_parts(parser, value, has_equation));
            has_equation |= parts[start..]
              .iter()
              .any(|part| !matches!(part.node, MathNode::Empty));
          }
          m::ParagraphChoice::MRun(run) => {
            parts.push(RootMathPart {
              node: parser.run(run),
              manual_break: math_run_manual_break(run),
              equation_start: false,
            });
          }
          _ => {}
        }
      }
      parts
    }
    w::ParagraphChoice::OfficeMath(value) => office_math_root_parts(parser, value, false),
    choice => parser
      .wordprocessing_choice(choice)
      .map(|node| RootMathPart {
        manual_break: match choice {
          w::ParagraphChoice::MRun(run) => math_run_manual_break(run),
          w::ParagraphChoice::Box(value) => math_box_manual_break(value),
          _ => None,
        },
        node,
        equation_start: false,
      })
      .into_iter()
      .collect(),
  }
}

fn wordprocessing_math_display_layout(
  choices: &[&w::ParagraphChoice],
  styles: &StylesCatalog,
  display_math: bool,
) -> Option<OfficeMathDisplayLayout> {
  if !display_math {
    return None;
  }
  // [MS-OI29500] §22.1.2.51(a) overrides the ECMA omission rule for Word:
  // an omitted m:jc inherits m:defJc. When m:dispDef is off, the document
  // defaults (including margins) are ignored and the owning w:p performs the
  // final horizontal adjustment instead.
  let explicit_alignment = choices.iter().find_map(|choice| {
    let w::ParagraphChoice::Paragraph(paragraph) = choice else {
      return None;
    };
    paragraph
      .paragraph_properties
      .as_deref()
      .and_then(|properties| properties.justification.as_ref())
      .map(|justification| office_math_display_alignment(justification.val))
  });
  Some(OfficeMathDisplayLayout {
    alignment: explicit_alignment.or(styles.display_math_alignment),
    left_margin_pt: styles.display_math_left_margin_pt.unwrap_or(0.0),
    right_margin_pt: styles.display_math_right_margin_pt.unwrap_or(0.0),
  })
}

pub(super) fn wordprocessing_math_zone_image<'a>(
  choices: impl IntoIterator<Item = &'a w::ParagraphChoice>,
  base_style: &TextStyle,
  styles: &StylesCatalog,
  display_math: bool,
) -> Option<InlineImage> {
  let parser = MathParser {
    base_style,
    styles,
    math_font_family: styles
      .math_font_family
      .clone()
      .unwrap_or_else(|| Arc::from("Cambria Math")),
  };
  let choices = choices.into_iter().collect::<Vec<_>>();
  let explicit_math_paragraph = choices
    .iter()
    .any(|choice| matches!(choice, w::ParagraphChoice::Paragraph(_)));
  let display_layout = wordprocessing_math_display_layout(&choices, styles, display_math);
  let root_parts = choices
    .into_iter()
    .flat_map(|choice| wordprocessing_math_root_parts(&parser, choice))
    .filter(|part| !matches!(part.node, MathNode::Empty))
    .collect::<Vec<_>>();
  let node = MathNode::row(root_parts.iter().map(|part| part.node.clone()));
  if display_layout.is_none()
    && !node.needs_two_dimensional_layout()
    && !root_parts.iter().any(|part| part.manual_break.is_some())
  {
    return None;
  }

  let line_style = representative_style(&node)
    .cloned()
    .unwrap_or_else(|| parser.default_math_style());
  let mut text_metrics = TextMetrics::new();
  let context = MathLayoutContext::root(
    display_math,
    styles.small_math_fractions,
    explicit_math_paragraph,
  )
  .with_compatibility_mode(styles.import_settings.compatibility_mode);
  let mut math_box = layout_node(&node, context, &mut text_metrics);
  finish_math_box(
    &mut math_box,
    &node,
    &line_style,
    base_style.font_size_pt * 0.08,
    base_style.font_size_pt * 0.08,
    &mut text_metrics,
  );
  if math_box.width_pt <= f32::EPSILON || math_box.ascent_pt + math_box.descent_pt <= f32::EPSILON {
    return None;
  }
  let line_layout =
    office_math_line_layout(&root_parts, context, base_style, styles, &mut text_metrics)
      .map(Arc::new);
  let mut image = inline_image_from_math_box(math_box, node.semantic_text());
  image.office_math_line_layout = line_layout;
  image.office_math_display_layout = display_layout;
  Some(image)
}

/// Materialize the active OfficeMath face as one coherent font-binding state.
///
/// OfficeMath uses a Unicode OpenType math font whenever that face covers the
/// requested character.  The enclosing WordprocessingML run can still carry
/// four independent `w:rFonts` slots, so replacing only the ASCII family lets
/// High ANSI operators, East Asian text, or complex-script text escape back to
/// the paragraph face.  Keep Word's slot classifier and other character
/// properties, but make every slot start from the selected math face and
/// rebuild any document-authored font-table substitution for that face.
pub(super) fn apply_office_math_font_family(
  style: &mut TextStyle,
  styles: &StylesCatalog,
  math_font_family: &Arc<str>,
) {
  style.font_family = Some(math_font_family.clone());
  style.high_ansi_font_family = Some(math_font_family.clone());
  style.east_asia_font_family = Some(math_font_family.clone());
  style.complex_font_family = Some(math_font_family.clone());

  // These fields describe the previously selected slot faces.  Retaining
  // them would make a missing glyph or unavailable face fall back through a
  // different paragraph-font chain instead of the selected math-font chain.
  style.fallback_font_family = None;
  style.high_ansi_fallback_font_family = None;
  style.east_asia_fallback_font_family = None;
  style.complex_fallback_font_family = None;
  style.font_family_class = None;
  style.high_ansi_font_family_class = None;
  style.east_asia_font_family_class = None;
  style.complex_font_family_class = None;
  style.font_charset = None;
  style.high_ansi_font_charset = None;
  style.east_asia_font_charset = None;
  style.complex_font_charset = None;
  style.font_pitch = None;
  style.high_ansi_font_pitch = None;
  style.east_asia_font_pitch = None;
  style.complex_font_pitch = None;
  styles.apply_font_substitution(style);
}

fn finish_math_box(
  math_box: &mut MathBox,
  node: &MathNode,
  line_style: &TextStyle,
  left_surround_pt: f32,
  right_surround_pt: f32,
  text_metrics: &mut TextMetrics,
) {
  expand_to_open_type_math_line_bounds(math_box, line_style, text_metrics);
  if let MathBackgroundCoverage::Uniform(color) = math_background_coverage(node) {
    // Every selectable run and every non-selectable ctrlPr in this realized
    // math zone owns the same background. Word paints that complete content
    // cell as one rectangle (equation.docx rows 7 and 8), just as adjacent
    // ordinary text portions with a common line box leave no seams. Mixed or
    // partially unpainted zones deliberately retain their individual boxes.
    math_box.replace_backgrounds_with_union(color);
  }
  // Run highlighting is paint decoration, so it must not feed transparent
  // font line metrics back into fractions, radicals, delimiters, or scripts
  // while they are being measured. Once the complete formula's OpenType
  // external line box is known, include authored background paint in the
  // final SVG canvas so large or otherwise ink-empty highlighted runs remain
  // visible without changing any internal math geometry.
  math_box.expand_to_background_bounds();
  // OfficeMath's zone-surround spacing belongs only at the realized zone's
  // two outer edges. Break fragments therefore receive one-sided padding.
  math_box.pad_left(left_surround_pt);
  math_box.pad_right(right_surround_pt);
}

fn inline_image_from_math_box(math_box: MathBox, semantic_text: String) -> InlineImage {
  let svg = math_box.to_svg();
  let padding = MIN_RULE_WIDTH_PT;
  InlineImage {
    data: Bytes::from(svg.into_bytes()),
    content_type: Some(OFFICE_MATH_SVG_CONTENT_TYPE.to_string()),
    picture_frame: None,
    picture_frame_clips_image: false,
    effects: None,
    static3d: None,
    width_pt: math_box.width_pt + padding * 2.0,
    height_pt: math_box.ascent_pt + math_box.descent_pt + padding * 2.0,
    inline_offset_x_pt: 0.0,
    inline_offset_y_pt: 0.0,
    effect_left_pt: 0.0,
    effect_top_pt: 0.0,
    effect_right_pt: 0.0,
    effect_bottom_pt: 0.0,
    inline_baseline_gap_pt: Some(-(math_box.descent_pt + padding)),
    line_box: InlineImageLineBox::OfficeMathExternal,
    office_math_line_layout: None,
    office_math_display_layout: None,
    crop: ImageCrop::default(),
    rotation_deg: 0.0,
    flip_horizontal: false,
    flip_vertical: false,
    metafile_background_color: None,
    alt_text: (!semantic_text.is_empty()).then_some(semantic_text),
    hyperlink_url: None,
    semantic_metafile_text: false,
    metafile_semantic_text_includes_raster_backdrop: false,
    signature_line: None,
    semantic_metafile_font_family: None,
    native_ole_equation: None,
    metafile_native_size: false,
    placement: ImagePlacement::Inline,
  }
}

#[derive(Clone, Copy, Debug)]
struct MathLineBoundary {
  kind: OfficeMathBreakKind,
  repeat_operator_index: Option<usize>,
}

fn flatten_root_math_atoms(node: MathNode, output: &mut Vec<MathNode>) {
  match node {
    MathNode::Empty => {}
    MathNode::Row(nodes) => nodes
      .into_iter()
      .for_each(|node| flatten_root_math_atoms(node, output)),
    MathNode::RunBackground { base, style } => {
      let mut children = Vec::new();
      flatten_root_math_atoms(*base, &mut children);
      output.extend(children.into_iter().map(|base| MathNode::RunBackground {
        base: Box::new(base),
        style: style.clone(),
      }));
    }
    node => output.push(node),
  }
}

fn office_math_line_layout(
  root_parts: &[RootMathPart],
  context: MathLayoutContext,
  base_style: &TextStyle,
  styles: &StylesCatalog,
  text_metrics: &mut TextMetrics,
) -> Option<OfficeMathLineLayout> {
  let mut nodes = Vec::new();
  let mut manual_breaks = Vec::<(usize, ManualMathBreak)>::new();
  let mut equation_starts = Vec::<usize>::new();
  for part in root_parts {
    let start = nodes.len();
    flatten_root_math_atoms(part.node.clone(), &mut nodes);
    if nodes.len() > start && part.equation_start {
      equation_starts.push(start);
    }
    if nodes.len() > start
      && let Some(manual_break) = part.manual_break
    {
      manual_breaks.push((start, manual_break));
    }
  }
  if nodes.is_empty() {
    return None;
  }

  // Normalize the complete root sequence before binary-break fragmentation.
  // `m:brkBin="before"` may put U+2026 in a fragment whose local first atom
  // has no preceding operator, even though the authored root sequence has a
  // preceding `+`. The row-level normalizer must therefore see the complete
  // root context once; each fragment can then retain the resolved scalar when
  // it is rebuilt for PDF output. Explicit m:lit/m:nor classes remain
  // protected by the same override check as the normal display path.
  normalize_row_automatic_math_ellipsis(&mut nodes);

  let classes = resolved_row_math_classes(&nodes);
  let mut boundaries = vec![None; nodes.len() + 1];
  for (index, manual_break) in manual_breaks {
    // [MS-OI29500] §22.1.2.15(b): Word ignores m:brk on a run which
    // does not begin with an operator. Binary and relation atoms are the
    // OfficeMath line-breaking operators identified by DopMth.
    if math_classes_are_line_break_operator(classes[index]) {
      boundaries[index] = Some(MathLineBoundary {
        kind: OfficeMathBreakKind::Manual {
          align_at: manual_break.align_at,
        },
        repeat_operator_index: None,
      });
    }
  }
  for index in equation_starts {
    boundaries[index] = Some(MathLineBoundary {
      kind: OfficeMathBreakKind::Equation,
      repeat_operator_index: None,
    });
  }

  let surround_pt = base_style.font_size_pt * 0.08;
  let operator_offsets_pt =
    math_operator_offsets(&nodes, &classes, context, surround_pt, text_metrics);
  let break_binary = styles.math_break_binary.unwrap_or_default();
  for (index, classes) in classes.iter().copied().enumerate() {
    if !math_classes_are_line_break_operator(classes) {
      continue;
    }
    // [MS-DOC] DopMth names placement relative to the physical break:
    // Before leaves the operator before the break, After starts the wrapped
    // line with it, and Repeat realizes it on both sides.
    let (position, repeat_operator_index) = match break_binary {
      m::BreakBinaryOperatorValues::Before => (index + 1, None),
      m::BreakBinaryOperatorValues::After => (index, None),
      m::BreakBinaryOperatorValues::Repeat => (index + 1, Some(index)),
    };
    if position == 0 || position >= nodes.len() {
      continue;
    }
    if boundaries[position].is_some() {
      continue;
    }
    boundaries[position] = Some(MathLineBoundary {
      kind: OfficeMathBreakKind::Automatic,
      repeat_operator_index,
    });
  }

  let mut starts = vec![0];
  starts.extend(
    boundaries
      .iter()
      .enumerate()
      .skip(1)
      .take(nodes.len().saturating_sub(1))
      .filter_map(|(position, boundary)| boundary.map(|_| position)),
  );
  starts.push(nodes.len());
  starts.sort_unstable();
  starts.dedup();
  if starts.len() <= 2 && boundaries[0].is_none() && !context.display_math {
    return None;
  }

  let mut fragments = Vec::<OfficeMathLineFragment>::new();
  for segment_index in 0..starts.len() - 1 {
    let start = starts[segment_index];
    let end = starts[segment_index + 1];
    if start >= end {
      continue;
    }
    let same_line_gap_pt = if start == 0 {
      0.0
    } else {
      math_spacing_between_root_nodes(&nodes, &classes, start - 1, start, context, text_metrics)
    };
    let image = render_math_line_fragment(
      &nodes[start..end],
      &classes[start..end],
      context,
      if start == 0 { surround_pt } else { 0.0 },
      if end == nodes.len() { surround_pt } else { 0.0 },
      text_metrics,
    )?;
    let boundary = boundaries[start];
    let mut wrapped_prefix = None;
    if let Some(operator_index) = boundary.and_then(|boundary| boundary.repeat_operator_index) {
      let (line_end_character, wrapped_character) = repeated_subtraction_characters(
        &nodes[operator_index],
        styles.math_break_binary_subtraction.unwrap_or_default(),
      );
      let wrapped_node = math_atom_with_replacement(
        &nodes[operator_index],
        '\u{2212}',
        wrapped_character.unwrap_or('\u{2212}'),
      );
      wrapped_prefix = render_math_line_fragment(
        std::slice::from_ref(&wrapped_node),
        std::slice::from_ref(&classes[operator_index]),
        context,
        0.0,
        0.0,
        text_metrics,
      );

      if let Some(line_end_character) = line_end_character
        && line_end_character != '\u{2212}'
        && let Some(previous_start) = starts.get(segment_index.wrapping_sub(1)).copied()
        && let Some(previous) = fragments.last_mut()
      {
        let mut variant_nodes = nodes[previous_start..start].to_vec();
        if let Some(last) = variant_nodes.last_mut() {
          *last = math_atom_with_replacement(last, '\u{2212}', line_end_character);
        }
        previous.line_end_variant = render_math_line_fragment(
          &variant_nodes,
          &classes[previous_start..start],
          context,
          if previous_start == 0 {
            surround_pt
          } else {
            0.0
          },
          0.0,
          text_metrics,
        );
      }
    }
    fragments.push(OfficeMathLineFragment {
      image,
      same_line_gap_pt,
      break_before: boundary.map(|boundary| boundary.kind),
      wrapped_prefix,
      line_end_variant: None,
      first_operator_offset_pt: math_operator_offsets(
        &nodes[start..end],
        &classes[start..end],
        context,
        if start == 0 { surround_pt } else { 0.0 },
        text_metrics,
      )
      .into_iter()
      .next(),
    });
  }
  if fragments.is_empty() {
    return None;
  }

  let (display_wrap_indent_pt, display_wrap_right) = if context.display_math {
    match styles.display_math_wrap {
      Some(MathWrapContinuation::Indent(indent)) => (Some(indent.max(0.0)), false),
      Some(MathWrapContinuation::Right) => (None, true),
      None => (None, false),
    }
  } else {
    (None, false)
  };
  Some(OfficeMathLineLayout {
    has_manual_break: fragments.iter().any(|fragment| {
      matches!(
        fragment.break_before,
        Some(OfficeMathBreakKind::Manual { .. })
      )
    }),
    fragments,
    display_wrap_indent_pt,
    display_wrap_right,
    operator_offsets_pt,
  })
}

fn math_classes_are_line_break_operator(classes: Option<MathAtomClasses>) -> bool {
  classes.is_some_and(|classes| matches!(classes.class, MathClass::Binary | MathClass::Relation))
}

fn math_spacing_between_root_nodes(
  nodes: &[MathNode],
  classes: &[Option<MathAtomClasses>],
  left: usize,
  right: usize,
  context: MathLayoutContext,
  metrics: &mut TextMetrics,
) -> f32 {
  match (classes[left], classes[right]) {
    (Some(left_classes), Some(right_classes)) => math_node_spacing(
      &nodes[left],
      &nodes[right],
      left_classes,
      right_classes,
      context,
      metrics,
    ),
    _ => 0.0,
  }
}

fn math_operator_offsets(
  nodes: &[MathNode],
  classes: &[Option<MathAtomClasses>],
  context: MathLayoutContext,
  left_surround_pt: f32,
  metrics: &mut TextMetrics,
) -> Vec<f32> {
  let mut offsets = Vec::new();
  let mut x = MIN_RULE_WIDTH_PT + left_surround_pt.max(0.0);
  let mut previous_spacing_node: Option<usize> = None;
  for (index, node) in nodes.iter().enumerate() {
    if let Some(right_classes) = classes[index] {
      if let Some(previous_index) = previous_spacing_node {
        x += math_node_spacing(
          &nodes[previous_index],
          node,
          classes[previous_index].expect("spacing node has math classes"),
          right_classes,
          context,
          metrics,
        );
      }
      previous_spacing_node = Some(index);
    }
    if math_classes_are_line_break_operator(classes[index]) {
      offsets.push(x);
    }
    x += layout_node(node, context, metrics).width_pt;
  }
  offsets
}

fn render_math_line_fragment(
  nodes: &[MathNode],
  classes: &[Option<MathAtomClasses>],
  context: MathLayoutContext,
  left_surround_pt: f32,
  right_surround_pt: f32,
  metrics: &mut TextMetrics,
) -> Option<InlineImage> {
  let node = MathNode::row(nodes.iter().cloned());
  let line_style = representative_style(&node)?.clone();
  let mut math_box = layout_row_with_classes(nodes, classes, context, metrics);
  finish_math_box(
    &mut math_box,
    &node,
    &line_style,
    left_surround_pt,
    right_surround_pt,
    metrics,
  );
  (math_box.width_pt > f32::EPSILON && math_box.ascent_pt + math_box.descent_pt > f32::EPSILON)
    .then(|| inline_image_from_math_box(math_box, node.semantic_text()))
}

fn repeated_subtraction_characters(
  node: &MathNode,
  policy: m::BreakBinarySubtractionValues,
) -> (Option<char>, Option<char>) {
  if !math_atom_contains_character(node, '\u{2212}') {
    return (None, None);
  }
  match policy {
    m::BreakBinarySubtractionValues::MinusMinus => (Some('\u{2212}'), Some('\u{2212}')),
    m::BreakBinarySubtractionValues::MinusPlus => (Some('\u{2212}'), Some('+')),
    m::BreakBinarySubtractionValues::PlusMinus => (Some('+'), Some('\u{2212}')),
  }
}

fn math_atom_contains_character(node: &MathNode, character: char) -> bool {
  match node {
    MathNode::Text { text, .. } => text.chars().any(|value| value == character),
    MathNode::Row(nodes) => nodes
      .iter()
      .any(|node| math_atom_contains_character(node, character)),
    MathNode::Argument { base, .. }
    | MathNode::RunBackground { base, .. }
    | MathNode::OperatorEmulator { base } => math_atom_contains_character(base, character),
    _ => false,
  }
}

fn math_atom_with_replacement(node: &MathNode, source: char, replacement: char) -> MathNode {
  let mut node = node.clone();
  replace_first_math_atom_character(&mut node, source, replacement);
  node
}

fn replace_first_math_atom_character(node: &mut MathNode, source: char, replacement: char) -> bool {
  match node {
    MathNode::Text { text, .. } => {
      let Some((offset, _)) = text.char_indices().find(|(_, value)| *value == source) else {
        return false;
      };
      text.replace_range(offset..offset + source.len_utf8(), &replacement.to_string());
      true
    }
    MathNode::Row(nodes) => nodes
      .iter_mut()
      .any(|node| replace_first_math_atom_character(node, source, replacement)),
    MathNode::Argument { base, .. }
    | MathNode::RunBackground { base, .. }
    | MathNode::OperatorEmulator { base } => {
      replace_first_math_atom_character(base, source, replacement)
    }
    _ => false,
  }
}

struct MathParser<'a> {
  base_style: &'a TextStyle,
  styles: &'a StylesCatalog,
  math_font_family: Arc<str>,
}

impl MathParser<'_> {
  fn default_math_style(&self) -> TextStyle {
    let mut style = self.base_style.clone();
    apply_office_math_font_family(&mut style, self.styles, &self.math_font_family);
    style
  }

  fn wordprocessing_choice(&self, choice: &w::ParagraphChoice) -> Option<MathNode> {
    Some(match choice {
      w::ParagraphChoice::Paragraph(value) => self.paragraph(value),
      w::ParagraphChoice::OfficeMath(value) => self.office_math(value),
      w::ParagraphChoice::Accent(value) => self.accent(value),
      w::ParagraphChoice::Bar(value) => self.bar(value),
      w::ParagraphChoice::Box(value) => self.box_object(value),
      w::ParagraphChoice::BorderBox(value) => self.border_box(value),
      w::ParagraphChoice::Delimiter(value) => self.delimiter(value),
      w::ParagraphChoice::EquationArray(value) => self.equation_array(value),
      w::ParagraphChoice::Fraction(value) => self.fraction(value),
      w::ParagraphChoice::MathFunction(value) => self.math_function(value),
      w::ParagraphChoice::GroupChar(value) => self.group_char(value),
      w::ParagraphChoice::LimitLower(value) => self.limit_lower(value),
      w::ParagraphChoice::LimitUpper(value) => self.limit_upper(value),
      w::ParagraphChoice::Matrix(value) => self.matrix(value),
      w::ParagraphChoice::Nary(value) => self.nary(value),
      w::ParagraphChoice::Phantom(value) => self.phantom(value),
      w::ParagraphChoice::Radical(value) => self.radical(value),
      w::ParagraphChoice::PreSubSuper(value) => self.pre_sub_super(value),
      w::ParagraphChoice::Subscript(value) => self.subscript(value),
      w::ParagraphChoice::SubSuperscript(value) => self.sub_superscript(value),
      w::ParagraphChoice::Superscript(value) => self.superscript(value),
      w::ParagraphChoice::MRun(value) => self.run(value),
      _ => return None,
    })
  }

  fn paragraph(&self, paragraph: &m::Paragraph) -> MathNode {
    MathNode::row(
      paragraph
        .paragraph_choice
        .iter()
        .filter_map(|choice| match choice {
          m::ParagraphChoice::OfficeMath(value) => Some(self.office_math(value)),
          m::ParagraphChoice::MRun(value) => Some(self.run(value)),
          _ => None,
        }),
    )
  }

  fn office_math(&self, math: &m::OfficeMath) -> MathNode {
    MathNode::row(
      math
        .office_math_choice
        .iter()
        .map(|choice| parse_math_choice!(self, OfficeMathChoice, choice)),
    )
  }

  fn base(&self, value: &m::Base) -> MathNode {
    self.argument(
      MathNode::row(
        value
          .base_choice
          .iter()
          .map(|choice| parse_math_choice!(self, BaseChoice, choice)),
      ),
      value.argument_properties.as_deref(),
      value.control_properties.as_deref(),
    )
  }

  fn numerator(&self, value: &m::Numerator) -> MathNode {
    self.argument(
      MathNode::row(
        value
          .numerator_choice
          .iter()
          .map(|choice| parse_math_choice!(self, NumeratorChoice, choice)),
      ),
      value.argument_properties.as_deref(),
      value.control_properties.as_deref(),
    )
  }

  fn denominator(&self, value: &m::Denominator) -> MathNode {
    self.argument(
      MathNode::row(
        value
          .denominator_choice
          .iter()
          .map(|choice| parse_math_choice!(self, DenominatorChoice, choice)),
      ),
      value.argument_properties.as_deref(),
      value.control_properties.as_deref(),
    )
  }

  fn function_name(&self, value: &m::FunctionName) -> MathNode {
    self.argument(
      MathNode::row(
        value
          .function_name_choice
          .iter()
          .map(|choice| parse_math_choice!(self, FunctionNameChoice, choice)),
      ),
      value.argument_properties.as_deref(),
      value.control_properties.as_deref(),
    )
  }

  fn limit(&self, value: &m::Limit) -> MathNode {
    self.argument(
      MathNode::row(
        value
          .limit_choice
          .iter()
          .map(|choice| parse_math_choice!(self, LimitChoice, choice)),
      ),
      value.argument_properties.as_deref(),
      value.control_properties.as_deref(),
    )
  }

  fn sub_argument(&self, value: &m::SubArgument) -> MathNode {
    self.argument(
      MathNode::row(
        value
          .sub_argument_choice
          .iter()
          .map(|choice| parse_math_choice!(self, SubArgumentChoice, choice)),
      ),
      value.argument_properties.as_deref(),
      value.control_properties.as_deref(),
    )
  }

  fn super_argument(&self, value: &m::SuperArgument) -> MathNode {
    self.argument(
      MathNode::row(
        value
          .super_argument_choice
          .iter()
          .map(|choice| parse_math_choice!(self, SuperArgumentChoice, choice)),
      ),
      value.argument_properties.as_deref(),
      value.control_properties.as_deref(),
    )
  }

  fn degree(&self, value: &m::Degree) -> MathNode {
    self.argument(
      MathNode::row(
        value
          .degree_choice
          .iter()
          .map(|choice| parse_math_choice!(self, DegreeChoice, choice)),
      ),
      value.argument_properties.as_deref(),
      value.control_properties.as_deref(),
    )
  }

  fn argument(
    &self,
    node: MathNode,
    properties: Option<&m::ArgumentProperties>,
    control: Option<&m::ControlProperties>,
  ) -> MathNode {
    let node = if matches!(node, MathNode::Empty) {
      MathNode::Placeholder {
        style: self.control_style(control),
      }
    } else {
      node
    };
    let size_delta = math_argument_size_delta(properties);
    if size_delta == 0 {
      node
    } else {
      MathNode::Argument {
        base: Box::new(node),
        size_delta,
      }
    }
  }

  fn control_style(&self, control: Option<&m::ControlProperties>) -> TextStyle {
    let mut style = match control.and_then(|control| control.control_properties_choice.as_ref()) {
      Some(m::ControlPropertiesChoice::RunProperties(properties)) => {
        properties::run_style(Some(properties), self.base_style.clone(), self.styles)
      }
      Some(m::ControlPropertiesChoice::DrawingRunProperties(properties)) => {
        let mut style = self.base_style.clone();
        super::apply_chart_run_properties(&mut style, properties, self.styles);
        style
      }
      _ => self.base_style.clone(),
    };
    // The active OfficeMath face owns the control character across every
    // Word font slot; ctrlPr still supplies its size, color, weight, and other
    // character properties.
    apply_office_math_font_family(&mut style, self.styles, &self.math_font_family);
    style
  }

  fn explicit_control_style(&self, control: Option<&m::ControlProperties>) -> Option<TextStyle> {
    // ECMA-376 Part 1 §22.1.2.23 makes ctrlPr a difference from paragraph
    // formatting for the non-selectable control character. Its absence has
    // distinct semantics (inherit the math object's first character), so do
    // not eagerly replace None with the paragraph style here.
    control.map(|control| self.control_style(Some(control)))
  }

  fn run(&self, run: &m::Run) -> MathNode {
    let characters = shared_math::math_run_characters(run);
    if characters.is_empty() {
      return MathNode::Empty;
    }
    let text = characters
      .iter()
      .map(|character| character.rendered)
      .collect::<String>();
    let mut style = properties::run_style(
      run.run_properties.as_deref(),
      self.base_style.clone(),
      self.styles,
    );
    // Materialize the active OfficeMath face after the ordinary run cascade.
    // Other run properties still participate, while none of the four inherited
    // Word font slots can redirect a covered math character to a text face.
    apply_office_math_font_family(&mut style, self.styles, &self.math_font_family);
    let properties = run.math_run_properties.as_deref();
    let literal = properties
      .and_then(|properties| properties.literal.as_ref())
      .is_some_and(|literal| math_on(literal.val));
    let normal = properties
      .and_then(|properties| properties.run_properties_choice.as_ref())
      .is_some_and(|choice| match choice {
        m::RunPropertiesChoice::NormalText(normal) => math_on(normal.val),
        m::RunPropertiesChoice::Sequence(_) => false,
      });
    let plain_math_style = properties
      .and_then(|properties| properties.run_properties_choice.as_ref())
      .and_then(|choice| match choice {
        m::RunPropertiesChoice::Sequence(sequence) => sequence.style.as_ref(),
        m::RunPropertiesChoice::NormalText(_) => None,
      })
      .is_some_and(|style| style.val == m::StyleValues::Plain);
    if !(literal || normal) && plain_math_style && text.chars().all(|character| character == ' ') {
      // Word persists user-entered OfficeMath spacing as a plain-style,
      // space-only m:r. Keep that run boundary: UnicodeMath gives two ASCII
      // spaces after a comma the distinct clause-separator meaning, while
      // spaces embedded in an unbuilt linear run remain ordinary glyph
      // advances. The row normalizer resolves the preceding punctuation.
      return MathNode::UserSpace {
        text,
        style,
        clause_separator: false,
      };
    }
    let background_style = style.clone();
    let node = if literal || normal {
      split_math_text(text, style, false)
    } else {
      split_math_run_characters(characters, style)
    };
    if background_style.highlight.is_some() {
      // Automatic atom splitting must not discard the authored m:r paint
      // boundary: ECMA-376 §§17.3.2.15 and 17.3.2.32 apply the background to
      // the complete run contents, including spacing realized between its
      // math atoms.
      MathNode::RunBackground {
        base: Box::new(node),
        style: background_style,
      }
    } else {
      node
    }
  }

  fn box_object(&self, value: &m::Box) -> MathNode {
    let base = self.base(&value.base);
    let operator_emulator = value
      .box_properties
      .as_deref()
      .and_then(|properties| properties.operator_emulator.as_ref())
      .is_some_and(|operator_emulator| math_on(operator_emulator.val));
    if operator_emulator {
      MathNode::OperatorEmulator {
        base: Box::new(base),
      }
    } else {
      base
    }
  }

  fn accent(&self, value: &m::Accent) -> MathNode {
    let properties = value.accent_properties.as_deref();
    MathNode::Accent {
      base: Box::new(self.base(&value.base)),
      character: properties
        .and_then(|properties| properties.accent_char.as_ref())
        .map_or("\u{0302}", |character| character.val.as_str())
        .to_string(),
      control_style: self.explicit_control_style(
        properties.and_then(|properties| properties.control_properties.as_deref()),
      ),
    }
  }

  fn bar(&self, value: &m::Bar) -> MathNode {
    let properties = value.bar_properties.as_deref();
    let bottom = properties
      .and_then(|properties| properties.position.as_ref())
      .is_none_or(|position| position.val == m::VerticalJustificationValues::Bottom);
    MathNode::Bar {
      base: Box::new(self.base(&value.base)),
      bottom,
      control_style: self.explicit_control_style(
        properties.and_then(|properties| properties.control_properties.as_deref()),
      ),
    }
  }

  fn border_box(&self, value: &m::BorderBox) -> MathNode {
    let properties = value.border_box_properties.as_deref();
    MathNode::BorderBox {
      base: Box::new(self.base(&value.base)),
      hide_top: properties
        .and_then(|value| value.hide_top.as_ref())
        .is_some_and(|value| math_on(value.val)),
      hide_bottom: properties
        .and_then(|value| value.hide_bottom.as_ref())
        .is_some_and(|value| math_on(value.val)),
      hide_left: properties
        .and_then(|value| value.hide_left.as_ref())
        .is_some_and(|value| math_on(value.val)),
      hide_right: properties
        .and_then(|value| value.hide_right.as_ref())
        .is_some_and(|value| math_on(value.val)),
      strike_horizontal: properties
        .and_then(|value| value.strike_horizontal.as_ref())
        .is_some_and(|value| math_on(value.val)),
      strike_vertical: properties
        .and_then(|value| value.strike_vertical.as_ref())
        .is_some_and(|value| math_on(value.val)),
      strike_bottom_left_to_top_right: properties
        .and_then(|value| value.strike_bottom_left_to_top_right.as_ref())
        .is_some_and(|value| math_on(value.val)),
      strike_top_left_to_bottom_right: properties
        .and_then(|value| value.strike_top_left_to_bottom_right.as_ref())
        .is_some_and(|value| math_on(value.val)),
      control_style: self.explicit_control_style(
        properties.and_then(|properties| properties.control_properties.as_deref()),
      ),
    }
  }

  fn delimiter(&self, value: &m::Delimiter) -> MathNode {
    let properties = value.delimiter_properties.as_deref();
    MathNode::Delimiter {
      begin: properties
        .and_then(|value| value.begin_char.as_ref())
        .map_or("(", |value| value.val.as_str())
        .to_string(),
      separator: properties
        .and_then(|value| value.separator_char.as_ref())
        .map_or("|", |value| value.val.as_str())
        .to_string(),
      end: properties
        .and_then(|value| value.end_char.as_ref())
        .map_or(")", |value| value.val.as_str())
        .to_string(),
      grow: properties
        .and_then(|value| value.grow_operators.as_ref())
        .is_none_or(|value| math_on(value.val)),
      shape: properties
        .and_then(|value| value.shape.as_ref())
        .map_or(m::ShapeDelimiterValues::Centered, |value| value.val),
      arguments: value.base.iter().map(|base| self.base(base)).collect(),
      control_style: self.explicit_control_style(
        properties.and_then(|properties| properties.control_properties.as_deref()),
      ),
    }
  }

  fn equation_array(&self, value: &m::EquationArray) -> MathNode {
    MathNode::EquationArray(value.base.iter().map(|base| self.base(base)).collect())
  }

  fn fraction(&self, value: &m::Fraction) -> MathNode {
    let properties = value.fraction_properties.as_deref();
    MathNode::Fraction {
      numerator: Box::new(self.numerator(&value.numerator)),
      denominator: Box::new(self.denominator(&value.denominator)),
      kind: properties
        .and_then(|properties| properties.fraction_type.as_ref())
        .map(|value| value.val)
        .unwrap_or_default(),
      control_style: self.explicit_control_style(
        properties.and_then(|properties| properties.control_properties.as_deref()),
      ),
    }
  }

  fn math_function(&self, value: &m::MathFunction) -> MathNode {
    MathNode::Function {
      name: Box::new(self.function_name(&value.function_name)),
      argument: Box::new(self.base(&value.base)),
    }
  }

  fn group_char(&self, value: &m::GroupChar) -> MathNode {
    let properties = value.group_char_properties.as_deref();
    let bottom = properties
      .and_then(|value| value.position.as_ref())
      .is_none_or(|position| position.val == m::VerticalJustificationValues::Bottom);
    MathNode::GroupChar {
      base: Box::new(self.base(&value.base)),
      character: properties
        .and_then(|value| value.accent_char.as_ref())
        .map_or(if bottom { "\u{23df}" } else { "\u{23de}" }, |value| {
          value.val.as_str()
        })
        .to_string(),
      bottom,
      vertical_justification: properties
        .and_then(|value| value.vertical_justification.as_ref())
        .map_or(m::VerticalJustificationValues::Top, |value| value.val),
      control_style: self.explicit_control_style(
        properties.and_then(|properties| properties.control_properties.as_deref()),
      ),
    }
  }

  fn limit_lower(&self, value: &m::LimitLower) -> MathNode {
    MathNode::Limits {
      base: Box::new(self.base(&value.base)),
      lower: Some(Box::new(self.limit(&value.limit))),
      upper: None,
      side: false,
    }
  }

  fn limit_upper(&self, value: &m::LimitUpper) -> MathNode {
    MathNode::Limits {
      base: Box::new(self.base(&value.base)),
      lower: None,
      upper: Some(Box::new(self.limit(&value.limit))),
      side: false,
    }
  }

  fn matrix(&self, value: &m::Matrix) -> MathNode {
    MathNode::Matrix(
      value
        .matrix_row
        .iter()
        .map(|row| row.base.iter().map(|base| self.base(base)).collect())
        .collect(),
    )
  }

  fn nary(&self, value: &m::Nary) -> MathNode {
    let properties = value.nary_properties.as_deref();
    let operator = properties
      .and_then(|value| value.accent_char.as_ref())
      .map_or("\u{222b}", |value| value.val.as_str())
      .to_string();
    let document_limit_location = if is_integral_nary_operator(&operator) {
      self.styles.integral_limit_location
    } else {
      self.styles.nary_limit_location
    };
    MathNode::Nary {
      operator,
      lower: (!properties
        .and_then(|value| value.hide_sub_argument.as_ref())
        .is_some_and(|value| math_on(value.val)))
      .then(|| Box::new(self.sub_argument(&value.sub_argument))),
      upper: (!properties
        .and_then(|value| value.hide_super_argument.as_ref())
        .is_some_and(|value| math_on(value.val)))
      .then(|| Box::new(self.super_argument(&value.super_argument))),
      base: Box::new(self.base(&value.base)),
      limit_location: properties
        .and_then(|value| value.limit_location.as_ref())
        .map(|value| value.val),
      document_limit_location,
      grow: properties
        .and_then(|value| value.grow_operators.as_ref())
        .is_some_and(|value| math_on(value.val)),
      style: Box::new(self.default_math_style()),
      control_style: self.explicit_control_style(
        properties.and_then(|properties| properties.control_properties.as_deref()),
      ),
    }
  }

  fn phantom(&self, value: &m::Phantom) -> MathNode {
    let properties = value.phantom_properties.as_deref();
    MathNode::Phantom {
      base: Box::new(self.base(&value.base)),
      transparent: properties
        .and_then(|value| value.transparent.as_ref())
        .is_some_and(|value| math_on(value.val)),
      zero_width: properties
        .and_then(|value| value.zero_width.as_ref())
        .is_some_and(|value| math_on(value.val)),
      zero_ascent: properties
        .and_then(|value| value.zero_ascent.as_ref())
        .is_some_and(|value| math_on(value.val)),
      zero_descent: properties
        .and_then(|value| value.zero_descent.as_ref())
        .is_some_and(|value| math_on(value.val)),
    }
  }

  fn radical(&self, value: &m::Radical) -> MathNode {
    let properties = value.radical_properties.as_deref();
    let degree_hidden = properties
      .and_then(|properties| properties.hide_degree.as_ref())
      .is_some_and(|value| math_on(value.val));
    MathNode::Radical {
      degree: (!degree_hidden).then(|| Box::new(self.degree(&value.degree))),
      base: Box::new(self.base(&value.base)),
      control_style: self.explicit_control_style(
        properties.and_then(|properties| properties.control_properties.as_deref()),
      ),
    }
  }

  fn pre_sub_super(&self, value: &m::PreSubSuper) -> MathNode {
    MathNode::Scripts {
      base: Box::new(self.base(&value.base)),
      lower: Some(Box::new(self.sub_argument(&value.sub_argument))),
      upper: Some(Box::new(self.super_argument(&value.super_argument))),
      pre: true,
    }
  }

  fn subscript(&self, value: &m::Subscript) -> MathNode {
    MathNode::Scripts {
      base: Box::new(self.base(&value.base)),
      lower: Some(Box::new(self.sub_argument(&value.sub_argument))),
      upper: None,
      pre: false,
    }
  }

  fn sub_superscript(&self, value: &m::SubSuperscript) -> MathNode {
    MathNode::Scripts {
      base: Box::new(self.base(&value.base)),
      lower: Some(Box::new(self.sub_argument(&value.sub_argument))),
      upper: Some(Box::new(self.super_argument(&value.super_argument))),
      pre: false,
    }
  }

  fn superscript(&self, value: &m::Superscript) -> MathNode {
    MathNode::Scripts {
      base: Box::new(self.base(&value.base)),
      lower: None,
      upper: Some(Box::new(self.super_argument(&value.super_argument))),
      pre: false,
    }
  }
}

fn math_on(value: Option<m::BooleanValues>) -> bool {
  value.is_none_or(|value| {
    matches!(
      value,
      m::BooleanValues::True | m::BooleanValues::On | m::BooleanValues::One
    )
  })
}

fn is_integral_nary_operator(operator: &str) -> bool {
  let mut characters = operator.chars();
  matches!(characters.next(), Some('\u{222b}'..='\u{2233}')) && characters.next().is_none()
}

fn math_argument_size_delta(properties: Option<&m::ArgumentProperties>) -> i8 {
  properties
    .and_then(|properties| properties.argument_size.as_ref())
    .map_or(0, |size| match size.val {
      -2..=2 => size.val as i8,
      _ => 0,
    })
}

fn split_math_text(text: String, style: TextStyle, automatic_spacing: bool) -> MathNode {
  if !automatic_spacing {
    // ECMA-376 Part 1 §§22.1.2.58 and 22.1.2.74 explicitly suppress
    // operator interpretation/math spacing for literal and normal-text runs.
    return MathNode::Text {
      text,
      style,
      math_class_override: Some(MathClass::Normal),
    };
  }

  let source = text.chars().collect::<Vec<_>>();
  let rendered = normalize_automatic_math_text(text);
  split_math_run_characters(
    source
      .into_iter()
      .zip(rendered.chars())
      .map(|(source, rendered)| shared_math::MathRunCharacter { source, rendered })
      .collect(),
    style,
  )
}

fn split_math_run_characters(
  characters: Vec<shared_math::MathRunCharacter>,
  style: TextStyle,
) -> MathNode {
  let mut nodes = Vec::new();
  let mut current = String::new();
  let mut current_style = None::<TextStyle>;
  let mut current_is_spacing_atom = false;
  for character in characters {
    let rendered = normalize_automatic_math_character(character.rendered);
    let class = math_character_class(rendered);
    let attaches_to_previous = matches!(class, MathClass::Diacritic | MathClass::GlyphPart);
    let is_spacing_atom = math_class_controls_spacing(class) || rendered == '\u{2061}';
    let character_style = if attaches_to_previous && !current.is_empty() {
      current_style.clone().unwrap_or_else(|| style.clone())
    } else {
      materialize_wordprocessingml_source_font_slot(&style, character.source)
    };
    let style_changed = current_style
      .as_ref()
      .is_some_and(|current| current != &character_style);
    if !current.is_empty()
      && (style_changed || (!attaches_to_previous && (current_is_spacing_atom || is_spacing_atom)))
    {
      nodes.push(MathNode::Text {
        text: std::mem::take(&mut current),
        style: current_style.take().unwrap_or_else(|| style.clone()),
        math_class_override: None,
      });
    }
    current_style.get_or_insert(character_style);
    current.push(rendered);
    if !attaches_to_previous {
      current_is_spacing_atom = is_spacing_atom;
    }
  }
  if !current.is_empty() {
    nodes.push(MathNode::Text {
      text: current,
      style: current_style.unwrap_or(style),
      math_class_override: None,
    });
  }
  MathNode::row(nodes)
}

fn normalize_automatic_math_character(character: char) -> char {
  match character {
    '-' => '\u{2212}',
    '\u{2329}' => '\u{27e8}',
    '\u{232a}' => '\u{27e9}',
    _ => character,
  }
}

fn normalize_automatic_math_text(text: String) -> String {
  if !text
    .chars()
    .any(|character| matches!(character, '-' | '\u{2329}' | '\u{232a}'))
  {
    return text;
  }
  // OfficeMath's automatic math input replaces the non-mathematical source
  // characters U+002D, U+2329, and U+232A with their mathematical Unicode
  // counterparts U+2212, U+27E8, and U+27E9. ECMA-376's m:lit and m:nor
  // properties take the early return above and therefore preserve the source
  // scalar and suppress this operator interpretation.
  text
    .chars()
    .map(normalize_automatic_math_character)
    .collect()
}

fn math_character_class(character: char) -> MathClass {
  match character {
    // UnicodeMath §2.1 assigns U+002F SOLIDUS and U+2044 FRACTION
    // SLASH to fraction construction, with U+FF0F FULLWIDTH SOLIDUS as
    // an East Asian input alias. If one of those input characters remains
    // as text in persisted OMML instead of becoming an m:f object, it is an
    // ordinary atom. U+2215 DIVISION SLASH is the distinct binary operator.
    // Unicode MathClass revision 15 classifies all three non-fullwidth
    // slashes as Binary, so OfficeMath needs this format-specific layer.
    '/' | '\u{2044}' | '\u{ff0f}' => MathClass::Normal,
    // ECMA-376 §22.1.2.39 defines U+2061 as the linear form of m:func.
    // It is an invisible structural marker, not a binary line-breaking
    // operator. Its single argument-side gap is resolved by
    // math_node_spacing through the same function_application_spacing path
    // used for the built-up object.
    '\u{2061}' => MathClass::Special,
    _ => unicode_math_class(character)
      .or_else(|| {
        // MathClass revision 15 omits a number of Unicode paired brackets,
        // including the East Asian U+3016/U+3017 pair used by Word's linear
        // OfficeMath. UAX #9 BidiBrackets carries the complete open/close
        // pairing needed for the TeX spacing table without replacing the
        // more specific math classes above.
        HardcodedBidiData
          .bidi_matched_opening_bracket(character)
          .map(|bracket| {
            if bracket.is_open {
              MathClass::Opening
            } else {
              MathClass::Closing
            }
          })
      })
      .unwrap_or(MathClass::Normal),
  }
}

fn math_class_controls_spacing(class: MathClass) -> bool {
  matches!(
    class,
    MathClass::Binary
      | MathClass::Closing
      | MathClass::Fence
      | MathClass::Large
      | MathClass::Opening
      | MathClass::Punctuation
      | MathClass::Relation
      | MathClass::Space
      | MathClass::Vary
  )
}

#[derive(Clone, Debug)]
enum MathNode {
  Empty,
  Placeholder {
    style: TextStyle,
  },
  Argument {
    base: Box<Self>,
    size_delta: i8,
  },
  RunBackground {
    base: Box<Self>,
    style: TextStyle,
  },
  Row(Vec<Self>),
  Text {
    text: String,
    style: TextStyle,
    math_class_override: Option<MathClass>,
  },
  UserSpace {
    text: String,
    style: TextStyle,
    clause_separator: bool,
  },
  OperatorEmulator {
    base: Box<Self>,
  },
  Fraction {
    numerator: Box<Self>,
    denominator: Box<Self>,
    kind: m::FractionTypeValues,
    control_style: Option<TextStyle>,
  },
  Scripts {
    base: Box<Self>,
    lower: Option<Box<Self>>,
    upper: Option<Box<Self>>,
    pre: bool,
  },
  Limits {
    base: Box<Self>,
    lower: Option<Box<Self>>,
    upper: Option<Box<Self>>,
    side: bool,
  },
  Delimiter {
    begin: String,
    separator: String,
    end: String,
    grow: bool,
    shape: m::ShapeDelimiterValues,
    arguments: Vec<Self>,
    control_style: Option<TextStyle>,
  },
  EquationArray(Vec<Self>),
  Matrix(Vec<Vec<Self>>),
  Nary {
    operator: String,
    lower: Option<Box<Self>>,
    upper: Option<Box<Self>>,
    base: Box<Self>,
    limit_location: Option<m::LimitLocationValues>,
    document_limit_location: Option<m::LimitLocationValues>,
    grow: bool,
    style: Box<TextStyle>,
    control_style: Option<TextStyle>,
  },
  Radical {
    degree: Option<Box<Self>>,
    base: Box<Self>,
    control_style: Option<TextStyle>,
  },
  Accent {
    base: Box<Self>,
    character: String,
    control_style: Option<TextStyle>,
  },
  Bar {
    base: Box<Self>,
    bottom: bool,
    control_style: Option<TextStyle>,
  },
  GroupChar {
    base: Box<Self>,
    character: String,
    bottom: bool,
    vertical_justification: m::VerticalJustificationValues,
    control_style: Option<TextStyle>,
  },
  Function {
    name: Box<Self>,
    argument: Box<Self>,
  },
  BorderBox {
    base: Box<Self>,
    hide_top: bool,
    hide_bottom: bool,
    hide_left: bool,
    hide_right: bool,
    strike_horizontal: bool,
    strike_vertical: bool,
    strike_bottom_left_to_top_right: bool,
    strike_top_left_to_bottom_right: bool,
    control_style: Option<TextStyle>,
  },
  Phantom {
    base: Box<Self>,
    transparent: bool,
    zero_width: bool,
    zero_ascent: bool,
    zero_descent: bool,
  },
}

impl MathNode {
  fn row(nodes: impl IntoIterator<Item = Self>) -> Self {
    let mut output = Vec::new();
    for node in nodes {
      match node {
        Self::Empty => {}
        Self::Row(children) => output.extend(children),
        node => output.push(node),
      }
    }
    normalize_row_automatic_math_clause_separators(&mut output);
    normalize_row_automatic_math_ellipsis(&mut output);
    match output.len() {
      0 => Self::Empty,
      1 => output.pop().unwrap_or(Self::Empty),
      _ => Self::Row(output),
    }
  }

  fn needs_two_dimensional_layout(&self) -> bool {
    match self {
      Self::Empty | Self::Text { .. } => false,
      Self::UserSpace {
        clause_separator, ..
      } => *clause_separator,
      Self::Placeholder { .. } => true,
      // m:argSz changes the realized size even when the argument is otherwise
      // a one-dimensional run, so it must stay on the structural math path.
      Self::Argument { .. } => true,
      Self::RunBackground { base, .. } => base.needs_two_dimensional_layout(),
      Self::Row(nodes) => {
        nodes.iter().any(Self::needs_two_dimensional_layout)
          || row_has_automatic_math_spacing(nodes, MathLayoutContext::root(false, false, false))
      }
      Self::OperatorEmulator { base } => base.needs_two_dimensional_layout(),
      // Function spacing and phantom visibility/zero-size properties are
      // structural even when their arguments contain only text.
      Self::Function { .. } | Self::Phantom { .. } => true,
      Self::Fraction { .. }
      | Self::Scripts { .. }
      | Self::Limits { .. }
      | Self::Delimiter { .. }
      | Self::EquationArray(_)
      | Self::Matrix(_)
      | Self::Nary { .. }
      | Self::Radical { .. }
      | Self::Accent { .. }
      | Self::Bar { .. }
      | Self::GroupChar { .. }
      | Self::BorderBox { .. } => true,
    }
  }

  fn semantic_text(&self) -> String {
    let mut output = String::new();
    self.append_semantic_text(&mut output);
    output
  }

  fn append_semantic_text(&self, output: &mut String) {
    match self {
      Self::Empty | Self::Placeholder { .. } => {}
      Self::Text { text, .. } | Self::UserSpace { text, .. } => output.push_str(text),
      Self::Argument { base, .. }
      | Self::RunBackground { base, .. }
      | Self::OperatorEmulator { base } => base.append_semantic_text(output),
      Self::Row(nodes) => nodes
        .iter()
        .for_each(|node| node.append_semantic_text(output)),
      Self::Fraction {
        numerator,
        denominator,
        kind,
        ..
      } => {
        numerator.append_semantic_text(output);
        if matches!(
          kind,
          m::FractionTypeValues::Linear | m::FractionTypeValues::Skewed
        ) {
          output.push('\u{2044}');
        }
        denominator.append_semantic_text(output);
      }
      Self::Scripts {
        base,
        lower,
        upper,
        pre,
        ..
      } => {
        if *pre {
          lower
            .as_deref()
            .into_iter()
            .for_each(|node| node.append_semantic_text(output));
          upper
            .as_deref()
            .into_iter()
            .for_each(|node| node.append_semantic_text(output));
        }
        base.append_semantic_text(output);
        if !*pre {
          lower
            .as_deref()
            .into_iter()
            .for_each(|node| node.append_semantic_text(output));
          upper
            .as_deref()
            .into_iter()
            .for_each(|node| node.append_semantic_text(output));
        }
      }
      Self::Limits {
        base, lower, upper, ..
      } => {
        base.append_semantic_text(output);
        lower
          .as_deref()
          .into_iter()
          .for_each(|node| node.append_semantic_text(output));
        upper
          .as_deref()
          .into_iter()
          .for_each(|node| node.append_semantic_text(output));
      }
      Self::Delimiter {
        begin,
        separator,
        end,
        arguments,
        ..
      } => {
        output.push_str(begin);
        for (index, argument) in arguments.iter().enumerate() {
          if index > 0 {
            output.push_str(separator);
          }
          argument.append_semantic_text(output);
        }
        output.push_str(end);
      }
      Self::EquationArray(rows) => {
        for (index, row) in rows.iter().enumerate() {
          if index > 0 {
            output.push('\n');
          }
          row.append_semantic_text(output);
        }
      }
      Self::Matrix(rows) => {
        for (row_index, row) in rows.iter().enumerate() {
          if row_index > 0 {
            output.push('\n');
          }
          row
            .iter()
            .for_each(|cell| cell.append_semantic_text(output));
        }
      }
      Self::Nary {
        operator,
        lower,
        upper,
        base,
        ..
      } => {
        output.push_str(operator);
        lower
          .as_deref()
          .into_iter()
          .for_each(|node| node.append_semantic_text(output));
        upper
          .as_deref()
          .into_iter()
          .for_each(|node| node.append_semantic_text(output));
        base.append_semantic_text(output);
      }
      Self::Radical { degree, base, .. } => {
        output.push('\u{221a}');
        base.append_semantic_text(output);
        degree
          .as_deref()
          .into_iter()
          .for_each(|node| node.append_semantic_text(output));
      }
      Self::Accent {
        base, character, ..
      }
      | Self::GroupChar {
        base, character, ..
      } => {
        base.append_semantic_text(output);
        output.push_str(character);
      }
      Self::Bar { base, .. } | Self::BorderBox { base, .. } | Self::Phantom { base, .. } => {
        base.append_semantic_text(output)
      }
      Self::Function { name, argument } => {
        name.append_semantic_text(output);
        argument.append_semantic_text(output);
      }
    }
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum MathBackgroundCoverage {
  Empty,
  Unpainted,
  Uniform(RgbColor),
  Mixed,
}

fn combine_background_coverage(
  left: MathBackgroundCoverage,
  right: MathBackgroundCoverage,
) -> MathBackgroundCoverage {
  use MathBackgroundCoverage::{Empty, Mixed, Uniform, Unpainted};
  match (left, right) {
    (Mixed, _) | (_, Mixed) => Mixed,
    (Empty, right) => right,
    (left, Empty) => left,
    (Unpainted, Unpainted) => Unpainted,
    (Uniform(left), Uniform(right)) if left == right => Uniform(left),
    (Unpainted | Uniform(_), Unpainted | Uniform(_)) => Mixed,
  }
}

fn combine_node_backgrounds<'a>(
  nodes: impl IntoIterator<Item = &'a MathNode>,
) -> MathBackgroundCoverage {
  nodes
    .into_iter()
    .fold(MathBackgroundCoverage::Empty, |coverage, node| {
      combine_background_coverage(coverage, math_background_coverage(node))
    })
}

fn style_background_coverage(style: Option<&TextStyle>) -> MathBackgroundCoverage {
  style.and_then(|style| style.highlight).map_or(
    MathBackgroundCoverage::Unpainted,
    MathBackgroundCoverage::Uniform,
  )
}

fn math_background_coverage(node: &MathNode) -> MathBackgroundCoverage {
  use MathBackgroundCoverage::{Empty, Uniform};
  match node {
    MathNode::Empty => Empty,
    MathNode::Placeholder { style }
    | MathNode::Text { style, .. }
    | MathNode::UserSpace { style, .. } => style_background_coverage(Some(style)),
    MathNode::RunBackground { base, style } => style
      .highlight
      .map_or_else(|| math_background_coverage(base), Uniform),
    MathNode::Argument { base, .. } | MathNode::OperatorEmulator { base } => {
      math_background_coverage(base)
    }
    MathNode::Row(nodes) | MathNode::EquationArray(nodes) => combine_node_backgrounds(nodes),
    MathNode::Matrix(rows) => combine_node_backgrounds(rows.iter().flatten()),
    MathNode::Fraction {
      numerator,
      denominator,
      kind,
      control_style,
    } => {
      let arguments = combine_node_backgrounds([numerator.as_ref(), denominator.as_ref()]);
      if *kind == m::FractionTypeValues::NoBar {
        arguments
      } else {
        let control = style_background_coverage(control_style.as_ref().or_else(|| {
          representative_style(numerator).or_else(|| representative_style(denominator))
        }));
        combine_background_coverage(control, arguments)
      }
    }
    MathNode::Scripts {
      base, lower, upper, ..
    }
    | MathNode::Limits {
      base, lower, upper, ..
    } => {
      let mut coverage = math_background_coverage(base);
      if let Some(lower) = lower {
        coverage = combine_background_coverage(coverage, math_background_coverage(lower));
      }
      if let Some(upper) = upper {
        coverage = combine_background_coverage(coverage, math_background_coverage(upper));
      }
      coverage
    }
    MathNode::Delimiter {
      begin,
      separator,
      end,
      arguments,
      control_style,
      ..
    } => {
      let arguments_coverage = combine_node_backgrounds(arguments);
      let has_control =
        !begin.is_empty() || !end.is_empty() || (arguments.len() > 1 && !separator.is_empty());
      if !has_control {
        return arguments_coverage;
      }
      let control = style_background_coverage(
        control_style
          .as_ref()
          .or_else(|| arguments.iter().find_map(representative_style)),
      );
      combine_background_coverage(control, arguments_coverage)
    }
    MathNode::Nary {
      operator,
      lower,
      upper,
      base,
      style,
      control_style,
      ..
    } => {
      let mut coverage = math_background_coverage(base);
      if let Some(lower) = lower {
        coverage = combine_background_coverage(coverage, math_background_coverage(lower));
      }
      if let Some(upper) = upper {
        coverage = combine_background_coverage(coverage, math_background_coverage(upper));
      }
      if operator.is_empty() {
        return coverage;
      }
      let control = style_background_coverage(control_style.as_ref().or_else(|| {
        representative_style(base)
          .or_else(|| lower.as_deref().and_then(representative_style))
          .or_else(|| upper.as_deref().and_then(representative_style))
          .or(Some(style))
      }));
      combine_background_coverage(control, coverage)
    }
    MathNode::Radical {
      degree,
      base,
      control_style,
    } => {
      let mut coverage = math_background_coverage(base);
      if let Some(degree) = degree {
        coverage = combine_background_coverage(coverage, math_background_coverage(degree));
      }
      let control = style_background_coverage(
        control_style
          .as_ref()
          .or_else(|| representative_style(base))
          .or_else(|| degree.as_deref().and_then(representative_style)),
      );
      combine_background_coverage(control, coverage)
    }
    MathNode::Accent {
      base,
      character,
      control_style,
    }
    | MathNode::GroupChar {
      base,
      character,
      control_style,
      ..
    } => {
      let base_coverage = math_background_coverage(base);
      if character.is_empty() {
        return base_coverage;
      }
      let control = style_background_coverage(
        control_style
          .as_ref()
          .or_else(|| representative_style(base)),
      );
      combine_background_coverage(control, base_coverage)
    }
    MathNode::Bar {
      base,
      control_style,
      ..
    } => combine_background_coverage(
      style_background_coverage(
        control_style
          .as_ref()
          .or_else(|| representative_style(base)),
      ),
      math_background_coverage(base),
    ),
    MathNode::BorderBox {
      base,
      hide_top,
      hide_bottom,
      hide_left,
      hide_right,
      strike_horizontal,
      strike_vertical,
      strike_bottom_left_to_top_right,
      strike_top_left_to_bottom_right,
      control_style,
    } => {
      let base_coverage = math_background_coverage(base);
      let has_control = !(*hide_top && *hide_bottom && *hide_left && *hide_right)
        || *strike_horizontal
        || *strike_vertical
        || *strike_bottom_left_to_top_right
        || *strike_top_left_to_bottom_right;
      if !has_control {
        return base_coverage;
      }
      combine_background_coverage(
        style_background_coverage(
          control_style
            .as_ref()
            .or_else(|| representative_style(base)),
        ),
        base_coverage,
      )
    }
    MathNode::Function { name, argument } => {
      combine_node_backgrounds([name.as_ref(), argument.as_ref()])
    }
    MathNode::Phantom {
      base, transparent, ..
    } => {
      if *transparent {
        Empty
      } else {
        math_background_coverage(base)
      }
    }
  }
}

fn run_background_base(node: &MathNode) -> &MathNode {
  match node {
    MathNode::RunBackground { base, .. } => run_background_base(base),
    node => node,
  }
}

fn run_background_base_mut(node: &mut MathNode) -> &mut MathNode {
  match node {
    MathNode::RunBackground { base, .. } => run_background_base_mut(base),
    node => node,
  }
}

fn normalize_row_automatic_math_clause_separators(nodes: &mut [MathNode]) {
  // UnicodeMath §3.14 assigns a separate clause-separator meaning to an
  // ASCII comma followed by two ASCII spaces. In built-up OMML, Word keeps
  // those authored spaces in a dedicated plain-style m:r; retaining that
  // provenance avoids reinterpreting the same bytes inside a linear run.
  for index in 1..nodes.len() {
    let comma = matches!(
      run_background_base(&nodes[index - 1]),
      MathNode::Text {
        text,
        math_class_override: None,
        ..
      } if text == ","
    );
    if !comma {
      continue;
    }
    if let MathNode::UserSpace {
      text,
      clause_separator,
      ..
    } = run_background_base_mut(&mut nodes[index])
      && text.chars().take(2).eq([' ', ' '])
    {
      *clause_separator = true;
    }
  }
}

fn normalize_row_automatic_math_ellipsis(nodes: &mut [MathNode]) {
  // UTR #25 recommends U+22EF MIDLINE HORIZONTAL ELLIPSIS instead of
  // U+2026 HORIZONTAL ELLIPSIS after a binary operator. Do this at the row
  // layer so the operator and ellipsis may be in separate m:r elements, and
  // so m:lit/m:nor text (identified by an explicit class override) remains
  // byte-for-byte authored. Explicit spaces do not end the syntactic operator
  // context, matching the row class resolver.
  let mut previous_class = None;
  for node in nodes {
    normalize_automatic_math_ellipsis_node(node, &mut previous_class);
  }
}

fn normalize_automatic_math_ellipsis_node(
  node: &mut MathNode,
  previous_class: &mut Option<MathClass>,
) {
  match node {
    // ECMA-376 §§17.3.2.15 and 17.3.2.32 make highlighting/shading a paint
    // property, while §22.1.2.107 makes m:argSz a size property. Neither
    // creates an OfficeMath syntax boundary. Continue through those wrappers
    // and through their atom-split row so a preceding structural object can
    // still determine whether a following UTR #25 Vary operator is binary.
    MathNode::RunBackground { base, .. } | MathNode::Argument { base, .. } => {
      normalize_automatic_math_ellipsis_node(base, previous_class);
    }
    MathNode::Row(nodes) => {
      for node in nodes {
        normalize_automatic_math_ellipsis_node(node, previous_class);
      }
    }
    MathNode::Text {
      text,
      math_class_override: None,
      ..
    } if text.contains('\u{2026}') => {
      let mut normalized = String::with_capacity(text.len());
      for source_character in text.chars() {
        let character =
          if source_character == '\u{2026}' && *previous_class == Some(MathClass::Binary) {
            '\u{22ef}'
          } else {
            source_character
          };
        normalized.push(character);

        let class = math_character_class(character);
        if !matches!(
          class,
          MathClass::Diacritic | MathClass::GlyphPart | MathClass::Space
        ) {
          *previous_class = Some(
            resolve_vary_math_classes(MathAtomClasses::single(class), *previous_class)
              .right
              .syntax_class(),
          );
        }
      }
      *text = normalized;
    }
    node => {
      let Some(classes) = math_node_classes(node) else {
        return;
      };
      let classes = resolve_vary_math_classes(classes, *previous_class);
      if classes.class != MathClass::Space {
        *previous_class = Some(classes.right.syntax_class());
      }
    }
  }
}

#[derive(Clone, Debug)]
struct MathBox {
  width_pt: f32,
  ascent_pt: f32,
  descent_pt: f32,
  script_base_ascent_pt: Option<f32>,
  script_base_descent_pt: Option<f32>,
  top_accent_attachment_pt: Option<f32>,
  italics_correction_pt: f32,
  italics_correction_in_advance: bool,
  text_like: bool,
  math_kerns: MathGlyphKerns,
  items: Vec<MathPaintItem>,
}

#[derive(Clone, Debug, Default, PartialEq)]
struct MathKernTable {
  correction_heights_pt: Vec<f32>,
  kern_values_pt: Vec<f32>,
}

impl MathKernTable {
  fn at_height(&self, height_pt: f32) -> f32 {
    let index = self
      .correction_heights_pt
      .iter()
      .position(|correction_height| height_pt < *correction_height)
      .unwrap_or(self.correction_heights_pt.len());
    self.kern_values_pt.get(index).copied().unwrap_or(0.0)
  }
}

#[derive(Clone, Debug, Default, PartialEq)]
struct MathGlyphKerns {
  top_right: Option<MathKernTable>,
  top_left: Option<MathKernTable>,
  bottom_right: Option<MathKernTable>,
  bottom_left: Option<MathKernTable>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum MathKernCorner {
  TopRight,
  TopLeft,
  BottomRight,
  BottomLeft,
}

impl MathKernCorner {
  const fn opposite(self) -> Self {
    match self {
      Self::TopRight => Self::BottomLeft,
      Self::TopLeft => Self::BottomRight,
      Self::BottomRight => Self::TopLeft,
      Self::BottomLeft => Self::TopRight,
    }
  }
}

impl MathGlyphKerns {
  fn at_height(&self, corner: MathKernCorner, height_pt: f32) -> f32 {
    let table = match corner {
      MathKernCorner::TopRight => self.top_right.as_ref(),
      MathKernCorner::TopLeft => self.top_left.as_ref(),
      MathKernCorner::BottomRight => self.bottom_right.as_ref(),
      MathKernCorner::BottomLeft => self.bottom_left.as_ref(),
    };
    table.map_or(0.0, |table| table.at_height(height_pt))
  }
}

struct AccentLayout {
  math_box: MathBox,
  accent_baseline_y_pt: f32,
}

#[derive(Clone, Copy)]
struct AccentLayoutOptions {
  bottom: bool,
  exact_frame_width: bool,
  replace_variant_semantics: bool,
}

#[derive(Clone, Debug)]
struct MathSemanticTextPlacement {
  style: TextStyle,
  x_pt: f32,
  baseline_y_pt: f32,
  horizontal_scale: f32,
}

impl MathBox {
  fn empty() -> Self {
    Self {
      width_pt: 0.0,
      ascent_pt: 0.0,
      descent_pt: 0.0,
      script_base_ascent_pt: None,
      script_base_descent_pt: None,
      top_accent_attachment_pt: None,
      italics_correction_pt: 0.0,
      italics_correction_in_advance: false,
      text_like: false,
      math_kerns: MathGlyphKerns::default(),
      items: Vec::new(),
    }
  }

  fn top_accent_attachment_pt(&self) -> f32 {
    self.top_accent_attachment_pt.unwrap_or_else(|| {
      if self.italics_correction_in_advance {
        self.width_pt / 2.0
      } else {
        (self.width_pt + self.italics_correction_pt) / 2.0
      }
    })
  }

  fn semantic_text_placement(&self) -> Option<MathSemanticTextPlacement> {
    self.items.iter().find_map(|item| match item {
      MathPaintItem::Text {
        style,
        x_pt,
        baseline_y_pt,
        horizontal_scale,
        ..
      }
      | MathPaintItem::SemanticText {
        style,
        x_pt,
        baseline_y_pt,
        horizontal_scale,
        ..
      } => Some(MathSemanticTextPlacement {
        style: style.clone(),
        x_pt: *x_pt,
        baseline_y_pt: *baseline_y_pt,
        horizontal_scale: *horizontal_scale,
      }),
      MathPaintItem::Background { .. }
      | MathPaintItem::GlyphPath { .. }
      | MathPaintItem::Line { .. } => None,
    })
  }

  fn replace_variant_semantics_with_combining_accent(
    &mut self,
    text: &str,
    placement: MathSemanticTextPlacement,
    x_pt: f32,
  ) {
    if !self
      .items
      .iter()
      .any(|item| matches!(item, MathPaintItem::SemanticText { .. }))
    {
      return;
    }

    // ECMA-376 Part 1 §22.1.2.1 defines m:acc as the base followed by one
    // combining diacritical mark. A prepared MATH variant or assembly owns
    // the visible outline geometry, but it must not replace that source
    // scalar with its wider part origins in the searchable PDF layer. Keep a
    // single clipped Unicode object at the base cmap glyph's attachment
    // position and retain the normal glyph's text matrix. OpenType MATH then
    // independently aligns the visible variant by TopAccentAttachment.
    self
      .items
      .retain(|item| !matches!(item, MathPaintItem::SemanticText { .. }));
    self.items.push(MathPaintItem::SemanticText {
      text: text.to_string(),
      style: placement.style,
      x_pt,
      baseline_y_pt: placement.baseline_y_pt,
      horizontal_scale: placement.horizontal_scale,
      glyph_id: None,
    });
  }

  fn script_base_ascent_pt(&self) -> f32 {
    self.script_base_ascent_pt.unwrap_or(self.ascent_pt)
  }

  fn script_base_descent_pt(&self) -> f32 {
    self.script_base_descent_pt.unwrap_or(self.descent_pt)
  }

  fn bottom_accent_attachment_pt(&self) -> f32 {
    // OpenType supplies only a top-accent attachment. Its per-glyph italics
    // correction still describes the protrusion to the right; mirroring that
    // correction around the advance center gives the bottom attachment used
    // by complete MATH layout implementations.
    if self.italics_correction_in_advance {
      self.width_pt / 2.0 - self.italics_correction_pt
    } else {
      (self.width_pt - self.italics_correction_pt) / 2.0
    }
  }

  fn rebase_to_baseline(mut self, baseline_y_pt: f32) -> Self {
    self.items = std::mem::take(&mut self.items)
      .into_iter()
      .map(|item| item.translated(0.0, -baseline_y_pt))
      .collect();
    self.ascent_pt = (self.ascent_pt + baseline_y_pt).max(0.0);
    self.descent_pt = (self.descent_pt - baseline_y_pt).max(0.0);
    self.script_base_ascent_pt = self
      .script_base_ascent_pt
      .map(|ascent| (ascent + baseline_y_pt).max(0.0));
    self.script_base_descent_pt = self
      .script_base_descent_pt
      .map(|descent| (descent - baseline_y_pt).max(0.0));
    self
  }

  fn append(&mut self, child: Self, x_pt: f32, baseline_y_pt: f32) {
    let child_width = child.width_pt;
    let child_ascent = child.ascent_pt;
    let child_descent = child.descent_pt;
    self.items.extend(
      child
        .items
        .into_iter()
        .map(|item| item.translated(x_pt, baseline_y_pt)),
    );
    self.width_pt = self.width_pt.max(x_pt + child_width);
    self.ascent_pt = self.ascent_pt.max(child_ascent - baseline_y_pt);
    self.descent_pt = self.descent_pt.max(child_descent + baseline_y_pt);
  }

  fn push_background(
    &mut self,
    x_pt: f32,
    y_pt: f32,
    width_pt: f32,
    height_pt: f32,
    color: RgbColor,
  ) {
    if width_pt <= f32::EPSILON || height_pt <= f32::EPSILON {
      return;
    }
    self.items.push(MathPaintItem::Background {
      x_pt,
      y_pt,
      width_pt,
      height_pt,
      color,
      opacity: 1.0,
    });
  }

  fn expand_to_background_bounds(&mut self) {
    let mut ascent_pt = self.ascent_pt;
    let mut descent_pt = self.descent_pt;
    for item in &self.items {
      if let MathPaintItem::Background {
        y_pt, height_pt, ..
      } = item
      {
        ascent_pt = ascent_pt.max(-y_pt);
        descent_pt = descent_pt.max(y_pt + height_pt);
      }
    }
    self.ascent_pt = ascent_pt;
    self.descent_pt = descent_pt;
  }

  fn replace_backgrounds_with_union(&mut self, color: RgbColor) {
    let mut left = f32::INFINITY;
    let mut top = f32::INFINITY;
    let mut right = f32::NEG_INFINITY;
    let mut bottom = f32::NEG_INFINITY;
    for item in &self.items {
      if let MathPaintItem::Background {
        x_pt,
        y_pt,
        width_pt,
        height_pt,
        ..
      } = item
      {
        left = left.min(*x_pt);
        top = top.min(*y_pt);
        right = right.max(*x_pt + *width_pt);
        bottom = bottom.max(*y_pt + *height_pt);
      }
    }
    if !left.is_finite() || !top.is_finite() || !right.is_finite() || !bottom.is_finite() {
      return;
    }
    self
      .items
      .retain(|item| !matches!(item, MathPaintItem::Background { .. }));
    self.push_background(left, top, right - left, bottom - top, color);
  }

  fn pad_left(&mut self, padding_pt: f32) {
    let padding_pt = padding_pt.max(0.0);
    if padding_pt <= f32::EPSILON {
      return;
    }
    self.items = std::mem::take(&mut self.items)
      .into_iter()
      .map(|item| item.translated(padding_pt, 0.0))
      .collect();
    self.top_accent_attachment_pt = self
      .top_accent_attachment_pt
      .map(|attachment| attachment + padding_pt);
    self.width_pt += padding_pt;
  }

  fn pad_right(&mut self, padding_pt: f32) {
    let padding_pt = padding_pt.max(0.0);
    if padding_pt > f32::EPSILON {
      self.width_pt += padding_pt;
    }
  }

  fn to_svg(&self) -> String {
    let padding = MIN_RULE_WIDTH_PT;
    let width = (self.width_pt + padding * 2.0).max(1.0);
    let height = (self.ascent_pt + self.descent_pt + padding * 2.0).max(1.0);
    let baseline = self.ascent_pt + padding;
    let mut svg = format!(
      "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{width:.4}\" height=\"{height:.4}\" viewBox=\"0 0 {width:.4} {height:.4}\">"
    );
    if self
      .items
      .iter()
      .any(|item| matches!(item, MathPaintItem::SemanticText { .. }))
    {
      // PDF lowering replays these child text objects beneath the authored
      // empty SVG clip. Clipping suppresses their ink while retaining the
      // exact MATH-table GID, source Unicode, font, and paint; generic SVG
      // consumers continue to use the visible outline path.
      svg.push_str(
        "<defs><clipPath id=\"math-semantic-clip\" clipPathUnits=\"userSpaceOnUse\"><path d=\"M0 0L0 0\"/></clipPath></defs>",
      );
    }
    // Run highlighting and shading are behind all of the formula's visible
    // ink, including overlapping scripts and MATH assemblies. Paint every
    // authored background in a first pass so a later child background cannot
    // cover an earlier control glyph merely because their boxes overlap.
    for item in &self.items {
      if let MathPaintItem::Background {
        x_pt,
        y_pt,
        width_pt,
        height_pt,
        color,
        opacity,
      } = item
      {
        svg.push_str(&format!(
          "<rect x=\"{:.4}\" y=\"{:.4}\" width=\"{:.4}\" height=\"{:.4}\" fill=\"#{:02x}{:02x}{:02x}\" fill-opacity=\"{:.4}\"/>",
          x_pt + padding,
          baseline + y_pt,
          width_pt,
          height_pt,
          color.r,
          color.g,
          color.b,
          opacity.clamp(0.0, 1.0),
        ));
      }
    }
    for (item_index, item) in self.items.iter().enumerate() {
      match item {
        MathPaintItem::Background { .. } => {}
        MathPaintItem::Text {
          text,
          style,
          x_pt,
          baseline_y_pt,
          horizontal_scale,
          opacity,
        } => {
          let family = xml_escape_attribute(
            style.font_family.as_deref().unwrap_or("Cambria Math"),
          );
          let text = xml_escape_text(text);
          let color = style.color;
          let weight = if style.bold { "bold" } else { "normal" };
          let font_style = if style.italic { "italic" } else { "normal" };
          let opacity = opacity * style.opacity;
          svg.push_str(&format!(
            "<text id=\"ooxmlsdk-math-visible-{item_index}\" visibility=\"hidden\" x=\"0\" y=\"0\" transform=\"translate({:.4} {:.4}) scale({:.6} 1)\" font-family=\"{}\" font-size=\"{:.4}\" font-weight=\"{}\" font-style=\"{}\" fill=\"#{:02x}{:02x}{:02x}\" fill-opacity=\"{:.4}\" xml:space=\"preserve\">{}</text>",
            x_pt + padding,
            baseline + baseline_y_pt,
            horizontal_scale,
            family,
            style.font_size_pt.max(MIN_MATH_SIZE_PT),
            weight,
            font_style,
            color.r,
            color.g,
            color.b,
            opacity.clamp(0.0, 1.0),
            text,
          ));
        }
        MathPaintItem::GlyphPath {
          path_data,
          style,
          x_pt,
          baseline_y_pt,
          scale_pt_per_unit,
          horizontal_scale,
          synthetic_bold,
          synthetic_italic,
          opacity,
        } => {
          let color = style.color;
          let opacity = opacity * style.opacity;
          let horizontal_scale_pt_per_unit = scale_pt_per_unit * horizontal_scale;
          let synthetic_italic_shear = if *synthetic_italic {
            horizontal_scale_pt_per_unit / 3.0
          } else {
            0.0
          };
          let (stroke, stroke_width_units) = if *synthetic_bold {
            (
              format!("#{:02x}{:02x}{:02x}", color.r, color.g, color.b),
              style.font_size_pt.max(MIN_MATH_SIZE_PT)
                / 30.0
                / scale_pt_per_unit.max(f32::EPSILON),
            )
          } else {
            ("none".to_string(), 0.0)
          };
          svg.push_str(&format!(
            "<path d=\"{}\" transform=\"matrix({:.8} 0 {:.8} -{:.8} {:.4} {:.4})\" fill=\"#{:02x}{:02x}{:02x}\" fill-opacity=\"{:.4}\" stroke=\"{}\" stroke-width=\"{:.4}\" stroke-opacity=\"{:.4}\" stroke-linejoin=\"round\" paint-order=\"fill stroke\"/>",
            path_data,
            horizontal_scale_pt_per_unit,
            synthetic_italic_shear,
            scale_pt_per_unit,
            x_pt + padding,
            baseline + baseline_y_pt,
            color.r,
            color.g,
            color.b,
            opacity.clamp(0.0, 1.0),
            stroke,
            stroke_width_units,
            opacity.clamp(0.0, 1.0),
          ));
        }
        MathPaintItem::SemanticText {
          text,
          style,
          x_pt,
          baseline_y_pt,
          horizontal_scale,
          glyph_id,
        } => {
          let family =
            xml_escape_attribute(style.font_family.as_deref().unwrap_or("Cambria Math"));
          let text = xml_escape_text(text);
          let weight = if style.bold { "bold" } else { "normal" };
          let font_style = if style.italic { "italic" } else { "normal" };
          let color = style.color;
          let marker = glyph_id.map_or_else(
            || format!("ooxmlsdk-math-semantic-{item_index}"),
            |glyph_id| format!("ooxmlsdk-math-semantic-{item_index}-gid-{glyph_id}"),
          );
          svg.push_str(&format!(
            "<g clip-path=\"url(#math-semantic-clip)\"><text id=\"{}\" visibility=\"hidden\" x=\"0\" y=\"0\" transform=\"translate({:.4} {:.4}) scale({:.6} 1)\" font-family=\"{}\" font-size=\"{:.4}\" font-weight=\"{}\" font-style=\"{}\" fill=\"#{:02x}{:02x}{:02x}\" fill-opacity=\"{:.4}\" stroke=\"none\" xml:space=\"preserve\">{}</text></g>",
            marker,
            x_pt + padding,
            baseline + baseline_y_pt,
            horizontal_scale,
            family,
            style.font_size_pt.max(MIN_MATH_SIZE_PT),
            weight,
            font_style,
            color.r,
            color.g,
            color.b,
            style.opacity.clamp(0.0, 1.0),
            text,
          ));
        }
        MathPaintItem::Line {
          x1_pt,
          y1_pt,
          x2_pt,
          y2_pt,
          width_pt,
          color,
          opacity,
        } => svg.push_str(&format!(
          "<line x1=\"{:.4}\" y1=\"{:.4}\" x2=\"{:.4}\" y2=\"{:.4}\" stroke=\"#{:02x}{:02x}{:02x}\" stroke-width=\"{:.4}\" stroke-opacity=\"{:.4}\" stroke-linecap=\"butt\"/>",
          x1_pt + padding,
          baseline + y1_pt,
          x2_pt + padding,
          baseline + y2_pt,
          color.r,
          color.g,
          color.b,
          width_pt.max(MIN_RULE_WIDTH_PT),
          opacity.clamp(0.0, 1.0),
        )),
      }
    }
    svg.push_str("</svg>");
    svg
  }
}

/// Resolve a complete formula's external line box without changing the
/// internal placement of fractions, radicals, delimiters, or their glyphs.
/// OpenType MATH defines MathLeading against the formula ink as a whole. For
/// applications such as Word that put the OS/2 line gap above the baseline,
/// ink crossing `sTypoAscender + sTypoLineGap - MathLeading` expands the
/// ascent; ink crossing sTypoDescender expands the descent.
fn expand_to_open_type_math_line_bounds(
  math_box: &mut MathBox,
  style: &TextStyle,
  metrics: &mut TextMetrics,
) {
  let vertical = metrics.vertical_metrics(style);
  let math_leading_pt = metrics.math_font_metrics(style).math_leading_pt;
  (math_box.ascent_pt, math_box.descent_pt) = open_type_math_line_extents(
    math_box.ascent_pt,
    math_box.descent_pt,
    vertical,
    math_leading_pt,
  );
}

fn open_type_math_line_extents(
  ink_ascent_pt: f32,
  ink_descent_pt: f32,
  vertical: crate::text_metrics::TextVerticalMetrics,
  math_leading_pt: f32,
) -> (f32, f32) {
  (
    (vertical.ascent_pt + vertical.line_gap_pt).max(ink_ascent_pt + math_leading_pt),
    vertical.descent_pt.max(ink_descent_pt),
  )
}

#[derive(Clone, Debug)]
enum MathPaintItem {
  Background {
    x_pt: f32,
    y_pt: f32,
    width_pt: f32,
    height_pt: f32,
    color: RgbColor,
    opacity: f32,
  },
  Text {
    text: String,
    style: TextStyle,
    x_pt: f32,
    baseline_y_pt: f32,
    horizontal_scale: f32,
    opacity: f32,
  },
  GlyphPath {
    path_data: Arc<str>,
    style: TextStyle,
    x_pt: f32,
    baseline_y_pt: f32,
    scale_pt_per_unit: f32,
    horizontal_scale: f32,
    synthetic_bold: bool,
    synthetic_italic: bool,
    opacity: f32,
  },
  SemanticText {
    text: String,
    style: TextStyle,
    x_pt: f32,
    baseline_y_pt: f32,
    horizontal_scale: f32,
    /// Exact OpenType glyph selected by a MATH variant or assembly. The SVG
    /// carrier retains the source Unicode, while PDF lowering uses this GID
    /// so character geometry comes from the same font record as the visible
    /// outline. `None` deliberately keeps the source cmap glyph, as required
    /// by the specialized OMML accent semantic contract.
    glyph_id: Option<u32>,
  },
  Line {
    x1_pt: f32,
    y1_pt: f32,
    x2_pt: f32,
    y2_pt: f32,
    width_pt: f32,
    color: RgbColor,
    opacity: f32,
  },
}

impl MathPaintItem {
  fn translated(mut self, dx_pt: f32, dy_pt: f32) -> Self {
    match &mut self {
      Self::Background { x_pt, y_pt, .. } => {
        *x_pt += dx_pt;
        *y_pt += dy_pt;
      }
      Self::Text {
        x_pt,
        baseline_y_pt,
        ..
      } => {
        *x_pt += dx_pt;
        *baseline_y_pt += dy_pt;
      }
      Self::GlyphPath {
        x_pt,
        baseline_y_pt,
        ..
      }
      | Self::SemanticText {
        x_pt,
        baseline_y_pt,
        ..
      } => {
        *x_pt += dx_pt;
        *baseline_y_pt += dy_pt;
      }
      Self::Line {
        x1_pt,
        y1_pt,
        x2_pt,
        y2_pt,
        ..
      } => {
        *x1_pt += dx_pt;
        *x2_pt += dx_pt;
        *y1_pt += dy_pt;
        *y2_pt += dy_pt;
      }
    }
    self
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum MathLayoutStyle {
  Display,
  Text,
  Script,
  ScriptScript,
}

impl MathLayoutStyle {
  const fn script(self) -> Self {
    match self {
      Self::Display | Self::Text => Self::Script,
      Self::Script | Self::ScriptScript => Self::ScriptScript,
    }
  }

  const fn fraction_argument(self) -> Self {
    match self {
      Self::Display => Self::Text,
      Self::Text => Self::Script,
      Self::Script | Self::ScriptScript => Self::ScriptScript,
    }
  }

  const fn script_level(self) -> u8 {
    match self {
      Self::Display | Self::Text => 0,
      Self::Script => 1,
      Self::ScriptScript => 2,
    }
  }

  const fn smaller_argument(self) -> Self {
    match self {
      Self::Display | Self::Text => Self::Script,
      Self::Script | Self::ScriptScript => Self::ScriptScript,
    }
  }

  const fn larger_argument(self) -> Self {
    match self {
      Self::Display => Self::Display,
      Self::Text | Self::Script => Self::Text,
      Self::ScriptScript => Self::Script,
    }
  }

  const fn argument_size(self, delta: i8) -> Self {
    let mut style = self;
    let mut remaining = delta;
    while remaining < 0 {
      style = style.smaller_argument();
      remaining += 1;
    }
    while remaining > 0 {
      style = style.larger_argument();
      remaining -= 1;
    }
    style
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ParentFractionKind {
  None,
  Bar,
  NoBar,
  Skewed,
  Other,
}

impl ParentFractionKind {
  const fn from_fraction(kind: m::FractionTypeValues) -> Self {
    match kind {
      m::FractionTypeValues::Bar => Self::Bar,
      m::FractionTypeValues::NoBar => Self::NoBar,
      m::FractionTypeValues::Skewed => Self::Skewed,
      _ => Self::Other,
    }
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct MathLayoutContext {
  style: MathLayoutStyle,
  display_math: bool,
  small_fraction: bool,
  explicit_math_paragraph: bool,
  compatibility_mode: u16,
  parent_fraction: ParentFractionKind,
}

impl MathLayoutContext {
  const fn root(display_math: bool, small_fraction: bool, explicit_math_paragraph: bool) -> Self {
    Self {
      style: if display_math {
        MathLayoutStyle::Display
      } else {
        MathLayoutStyle::Text
      },
      display_math,
      small_fraction,
      explicit_math_paragraph,
      // ECMA-376 does not serialize a mode on the math object itself. A DOCX
      // without w:compatSetting uses Word 2007 / mode 12, matching the
      // document-level resolver in docx::settings.
      compatibility_mode: 12,
      parent_fraction: ParentFractionKind::None,
    }
  }

  const fn with_compatibility_mode(self, compatibility_mode: u16) -> Self {
    Self {
      compatibility_mode,
      ..self
    }
  }

  const fn script(self) -> Self {
    Self {
      style: self.style.script(),
      ..self
    }
  }

  const fn script_script(self) -> Self {
    self.script().script()
  }

  const fn argument_size(self, delta: i8) -> Self {
    Self {
      style: self.style.argument_size(delta),
      ..self
    }
  }

  const fn fraction_argument(self, kind: m::FractionTypeValues) -> Self {
    let nested_word_fraction = self.explicit_math_paragraph
      && self.display_math
      && matches!(
        kind,
        m::FractionTypeValues::Bar | m::FractionTypeValues::NoBar
      )
      && matches!(
        self.parent_fraction,
        ParentFractionKind::Bar | ParentFractionKind::NoBar | ParentFractionKind::Skewed
      );
    // [MS-OI29500] §22.1.2.98 limits m:smallFrac to bar/noBar
    // fractions nested in bar/noBar/skw fractions in display math. With the
    // option off, Word keeps that nested fraction's arguments at regular-text
    // size in an explicit display-math paragraph. A lone bare m:oMath promoted
    // to display by paragraph finalization is the structural counterexample:
    // it follows the standard Display -> Text -> Script -> ScriptScript chain.
    let style = if nested_word_fraction && !self.small_fraction {
      self.style
    } else {
      self.style.fraction_argument()
    };
    Self {
      style,
      parent_fraction: ParentFractionKind::from_fraction(kind),
      ..self
    }
  }

  const fn same_style_in_fraction(self, kind: m::FractionTypeValues) -> Self {
    Self {
      parent_fraction: ParentFractionKind::from_fraction(kind),
      ..self
    }
  }
}

fn layout_node(node: &MathNode, context: MathLayoutContext, metrics: &mut TextMetrics) -> MathBox {
  match node {
    MathNode::Empty => MathBox::empty(),
    MathNode::Placeholder { style } => layout_placeholder(style, context, metrics),
    MathNode::Argument { base, size_delta } => {
      layout_node(base, context.argument_size(*size_delta), metrics)
    }
    MathNode::RunBackground { base, style } => layout_run_background(base, style, context, metrics),
    MathNode::Text { text, style, .. } => layout_text(text, style, context, metrics),
    MathNode::UserSpace {
      text,
      style,
      clause_separator,
    } => layout_user_space(text, style, *clause_separator, context, metrics),
    MathNode::OperatorEmulator { base } => layout_node(base, context, metrics),
    MathNode::Row(nodes) => layout_row(nodes, context, metrics),
    MathNode::Fraction {
      numerator,
      denominator,
      kind,
      control_style,
    } => layout_fraction(
      numerator,
      denominator,
      *kind,
      control_style.as_ref(),
      context,
      metrics,
    ),
    MathNode::Scripts {
      base,
      lower,
      upper,
      pre,
    } => layout_scripts(
      base,
      lower.as_deref(),
      upper.as_deref(),
      *pre,
      context,
      metrics,
    ),
    MathNode::Limits {
      base,
      lower,
      upper,
      side,
    } => layout_limits(
      base,
      lower.as_deref(),
      upper.as_deref(),
      *side,
      context,
      metrics,
    ),
    MathNode::Delimiter {
      begin,
      separator,
      end,
      grow,
      shape,
      arguments,
      control_style,
    } => layout_delimiter(
      DelimiterLayoutSpec {
        begin,
        separator,
        end,
        grow: *grow,
        shape: *shape,
        control_style: control_style.as_ref(),
      },
      arguments,
      context,
      metrics,
    ),
    MathNode::EquationArray(rows) => layout_stack(rows, context, metrics, 0.25),
    MathNode::Matrix(rows) => layout_matrix(rows, context, metrics),
    MathNode::Nary {
      operator,
      lower,
      upper,
      base,
      limit_location,
      document_limit_location,
      grow,
      style,
      control_style,
    } => layout_nary(
      operator,
      lower.as_deref(),
      upper.as_deref(),
      base,
      *limit_location,
      *document_limit_location,
      *grow,
      style,
      control_style.as_ref(),
      context,
      metrics,
    ),
    MathNode::Radical {
      degree,
      base,
      control_style,
    } => layout_radical(
      degree.as_deref(),
      base,
      control_style.as_ref(),
      context,
      metrics,
    ),
    MathNode::Accent {
      base,
      character,
      control_style,
    } => {
      layout_accent(
        base,
        character,
        AccentLayoutOptions {
          bottom: false,
          exact_frame_width: false,
          replace_variant_semantics: true,
        },
        control_style.as_ref(),
        context,
        metrics,
      )
      .math_box
    }
    MathNode::Bar {
      base,
      bottom,
      control_style,
    } => layout_bar(base, *bottom, control_style.as_ref(), context, metrics),
    MathNode::GroupChar {
      base,
      character,
      bottom,
      vertical_justification,
      control_style,
    } => {
      let group = layout_accent(
        base,
        character,
        AccentLayoutOptions {
          bottom: *bottom,
          exact_frame_width: true,
          replace_variant_semantics: false,
        },
        control_style.as_ref(),
        context,
        metrics,
      );
      if group_character_aligns_character(*bottom, *vertical_justification) {
        group
          .math_box
          .rebase_to_baseline(group.accent_baseline_y_pt)
      } else {
        group.math_box
      }
    }
    MathNode::Function { name, argument } => layout_function(name, argument, context, metrics),
    MathNode::BorderBox {
      base,
      hide_top,
      hide_bottom,
      hide_left,
      hide_right,
      strike_horizontal,
      strike_vertical,
      strike_bottom_left_to_top_right,
      strike_top_left_to_bottom_right,
      control_style,
    } => layout_border_box(
      base,
      *hide_top,
      *hide_bottom,
      *hide_left,
      *hide_right,
      *strike_horizontal,
      *strike_vertical,
      *strike_bottom_left_to_top_right,
      *strike_top_left_to_bottom_right,
      control_style.as_ref(),
      context,
      metrics,
    ),
    MathNode::Phantom {
      base,
      transparent,
      zero_width,
      zero_ascent,
      zero_descent,
    } => {
      let mut result = layout_node(base, context, metrics);
      if *transparent {
        set_box_opacity(&mut result, 0.0);
      }
      if *zero_width {
        result.width_pt = 0.0;
      }
      if *zero_ascent {
        result.ascent_pt = 0.0;
      }
      if *zero_descent {
        result.descent_pt = 0.0;
      }
      result
    }
  }
}

fn layout_run_background(
  base: &MathNode,
  source_style: &TextStyle,
  context: MathLayoutContext,
  metrics: &mut TextMetrics,
) -> MathBox {
  let mut result = layout_node(base, context, metrics);
  let Some(color) = source_style.highlight else {
    return result;
  };
  let style = math_script_style(source_style, context, metrics);
  let vertical = metrics.vertical_metrics(&style);
  let y_pt = -(vertical.ascent_pt + vertical.leading_above_pt());
  let width_pt = result.width_pt;
  result.push_background(0.0, y_pt, width_pt, vertical.line_height_pt(), color);
  result
}

fn apply_control_background(math_box: &mut MathBox, style: &TextStyle) {
  let Some(color) = style.highlight else {
    return;
  };
  // A built-up control character may be a prepared MATH variant or an
  // authored assembly. Its realized character cell is therefore the full
  // control box, not the original cmap glyph's line-height cell.
  let y_pt = -math_box.ascent_pt;
  let width_pt = math_box.width_pt;
  let height_pt = math_box.ascent_pt + math_box.descent_pt;
  math_box.push_background(0.0, y_pt, width_pt, height_pt, color);
}

fn layout_user_space(
  text: &str,
  source_style: &TextStyle,
  clause_separator: bool,
  context: MathLayoutContext,
  metrics: &mut TextMetrics,
) -> MathBox {
  if !clause_separator {
    return layout_text(text, source_style, context, metrics);
  }

  // Word realizes the two-space UnicodeMath clause separator as two nominal
  // ems without emitting visible space glyphs. Any additional authored ASCII
  // spaces retain their font advances after the clause separator.
  let style = math_script_style(source_style, context, metrics);
  let extra_spaces = text.chars().count().saturating_sub(2);
  let extra_width_pt = if extra_spaces == 0 {
    0.0
  } else {
    metrics.measure_text(&" ".repeat(extra_spaces), &style)
  };
  let mut space = MathBox::empty();
  space.width_pt = style.font_size_pt * MATH_CLAUSE_SEPARATOR_SPACE_EM + extra_width_pt;
  space
}

fn layout_text(
  text: &str,
  source_style: &TextStyle,
  context: MathLayoutContext,
  metrics: &mut TextMetrics,
) -> MathBox {
  if text.is_empty() {
    return MathBox::empty();
  }
  let style = math_script_style(source_style, context, metrics);
  let shaped = metrics.shape_text(text, &style);
  let script_level = context.style.script_level().min(2);
  if script_level > 0 {
    // OpenType's registered `ssty` feature is selected by the math layout
    // client with value 1 for scripts and 2 for scripts-on-scripts. The
    // feature substitutes only covered glyphs; HarfBuzz therefore preserves
    // the specified original-glyph fallback for the rest of the run.
    let script_style_feature = [FeatureValue {
      tag: "ssty".into(),
      value: u32::from(script_level),
    }];
    if let Some(script_shaped) =
      metrics.shape_text_with_features(text, &style, &script_style_feature)
      && shaped
        .as_ref()
        .is_some_and(|normal| math_script_glyph_selection_changed(normal, &script_shaped))
      && let Some(script_box) = layout_shaped_math_text(text, &style, script_shaped)
    {
      return script_box;
    }
  }
  if shaped
    .as_ref()
    .is_some_and(shaped_math_text_requires_exact_transport)
    && let Some(exact_box) = shaped
      .clone()
      .and_then(|shaped| layout_shaped_math_text(text, &style, shaped))
  {
    return exact_box;
  }
  // The WordprocessingML slot and fallback resolver has already selected the
  // actual face used for measurement. Preserve that selection across the SVG
  // carrier rather than asking CSS font matching to repeat a request which no
  // longer contains the ASCII/high-ANSI/East-Asia/complex-script slot. A
  // mixed-face run takes the exact outline/semantic path above.
  let paint_style = shaped
    .as_ref()
    .and_then(|shaped| shaped_math_text_single_face_style(&style, shaped))
    .unwrap_or_else(|| style.clone());
  let raw_width_pt = shaped.as_ref().map_or_else(
    || metrics.measure_text(text, &style),
    |shaped| shaped.width_pt,
  );
  let top_accent_attachment_pt = shaped
    .as_ref()
    .and_then(|shaped| shaped_top_accent_attachment_pt(shaped, &style));
  let positioning = shaped
    .as_ref()
    .map_or_else(MathGlyphPositioning::default, |shaped| {
      shaped_math_positioning(shaped, &style)
    });
  let width_pt = raw_width_pt
    + if positioning.add_italics_to_advance {
      positioning.italics_correction_pt
    } else {
      0.0
    };
  let vertical = metrics.vertical_metrics_for_text(text, &style);
  let (mut ascent_pt, mut descent_pt) = (vertical.ascent_pt, vertical.descent_pt);
  if let Some(shaped) = shaped {
    let mut ink_ascent = 0.0_f32;
    let mut ink_descent = 0.0_f32;
    for glyph in shaped.glyphs {
      if let Some(bounds) = glyph.bounds_em {
        let size = glyph.font_size_pt;
        ink_ascent = ink_ascent.max((bounds.y_max_em + glyph.y_offset_em) * size);
        ink_descent = ink_descent.max(-(bounds.y_min_em + glyph.y_offset_em) * size);
      }
    }
    // MathBox owns the formula's internal ink frame. Once shaping supplies
    // glyph bounds, use yMax/-yMin on both sides of the baseline exactly: a
    // digit with no ink below the baseline has zero descent rather than the
    // font's transparent typographic descent. OpenType MathLeading and the
    // OS/2 metrics expand the complete formula's external line box later in
    // expand_to_open_type_math_line_bounds. Keep logical vertical metrics
    // only as the fallback when no shaped run is available.
    ascent_pt = ink_ascent;
    descent_pt = ink_descent;
  }
  MathBox {
    width_pt,
    ascent_pt,
    descent_pt,
    script_base_ascent_pt: None,
    script_base_descent_pt: None,
    top_accent_attachment_pt,
    italics_correction_pt: positioning.italics_correction_pt,
    italics_correction_in_advance: positioning.add_italics_to_advance,
    text_like: positioning.text_like,
    math_kerns: positioning.math_kerns,
    items: vec![MathPaintItem::Text {
      text: text.to_string(),
      style: paint_style,
      x_pt: 0.0,
      baseline_y_pt: 0.0,
      horizontal_scale: 1.0,
      opacity: 1.0,
    }],
  }
}

fn math_script_glyph_selection_changed(normal: &ShapedText, script: &ShapedText) -> bool {
  normal.glyphs.len() != script.glyphs.len()
    || normal
      .glyphs
      .iter()
      .zip(&script.glyphs)
      .any(|(normal_glyph, script_glyph)| {
        normal_glyph.glyph_id != script_glyph.glyph_id
          || normal_glyph.text_range != script_glyph.text_range
          || normal.font_faces.get(normal_glyph.font_index)
            != script.font_faces.get(script_glyph.font_index)
      })
}

fn shaped_math_text_requires_exact_transport(shaped: &ShapedText) -> bool {
  let Some(first) = shaped.glyphs.first().map(|glyph| glyph.font_index) else {
    return false;
  };
  shaped.glyphs.iter().any(|glyph| {
    glyph.font_index != first
      || shaped
        .font_faces
        .get(glyph.font_index)
        .is_some_and(|face| face.synthetic_bold || face.synthetic_italic)
  })
}

fn resolved_math_face_style(source: &TextStyle, face_data: &FontFaceData) -> Option<TextStyle> {
  let face = FontRef::from_index(face_data.data.as_ref(), face_data.index).ok()?;
  let family = [StringId::FAMILY_NAME, StringId::TYPOGRAPHIC_FAMILY_NAME]
    .into_iter()
    .find_map(|name_id| {
      face
        .localized_strings(name_id)
        .english_or_first()
        .map(|name| Arc::<str>::from(name.to_string()))
    })?;
  let mut style = source.clone();
  style.font_family = Some(family);
  Some(style)
}

fn shaped_math_text_single_face_style(
  source: &TextStyle,
  shaped: &ShapedText,
) -> Option<TextStyle> {
  let font_index = shaped.glyphs.first()?.font_index;
  if shaped
    .glyphs
    .iter()
    .any(|glyph| glyph.font_index != font_index)
  {
    return None;
  }
  resolved_math_face_style(source, shaped.font_faces.get(font_index)?)
}

fn layout_shaped_math_text(text: &str, style: &TextStyle, shaped: ShapedText) -> Option<MathBox> {
  if shaped.glyphs.is_empty() {
    return None;
  }

  let top_accent_attachment_pt = shaped_top_accent_attachment_pt(&shaped, style);
  let positioning = shaped_math_positioning(&shaped, style);
  let mut ink_ascent_pt = 0.0_f32;
  let mut ink_descent_pt = 0.0_f32;
  let mut cursor_x_pt = 0.0_f32;
  let mut items = Vec::with_capacity(shaped.glyphs.len() * 2);
  let mut semantic_ranges = Vec::new();
  let mut index = 0;

  while index < shaped.glyphs.len() {
    let first = &shaped.glyphs[index];
    let font_index = first.font_index;
    let font_size_pt = first.font_size_pt;
    let text_range = first.text_range.clone();
    let face_data = shaped.font_faces.get(font_index)?;
    let face = FontRef::from_index(face_data.data.as_ref(), face_data.index).ok()?;
    let face_style = resolved_math_face_style(style, face_data)?;
    let units_per_em = f32::from(face.head().ok()?.units_per_em()).max(1.0);
    let scale_pt_per_unit = font_size_pt / units_per_em;
    let horizontal_scale = style.horizontal_scale.unwrap_or(1.0).max(f32::EPSILON);
    let cluster_origin_x_pt = cursor_x_pt;

    while let Some(glyph) = shaped.glyphs.get(index)
      && glyph.font_index == font_index
      && glyph.font_size_pt.to_bits() == font_size_pt.to_bits()
      && glyph.text_range == text_range
    {
      if let Some(geometry) = math_glyph_geometry(&face, GlyphId::new(glyph.glyph_id)) {
        items.push(MathPaintItem::GlyphPath {
          path_data: geometry.path_data,
          style: face_style.clone(),
          x_pt: cursor_x_pt + glyph.x_offset_em * glyph.font_size_pt,
          baseline_y_pt: -glyph.y_offset_em * glyph.font_size_pt,
          scale_pt_per_unit,
          horizontal_scale,
          synthetic_bold: face_data.synthetic_bold,
          synthetic_italic: face_data.synthetic_italic,
          opacity: 1.0,
        });
      } else if glyph.bounds_em.is_some() {
        // A selected outline glyph must remain paintable. If the selected
        // face exposes only a non-outline representation, retain the normal
        // SVG text path rather than silently dropping visible ink.
        return None;
      }

      if let Some(bounds) = glyph.bounds_em {
        ink_ascent_pt =
          ink_ascent_pt.max((bounds.y_max_em + glyph.y_offset_em) * glyph.font_size_pt);
        ink_descent_pt =
          ink_descent_pt.max(-(bounds.y_min_em + glyph.y_offset_em) * glyph.font_size_pt);
      }
      cursor_x_pt += glyph.x_advance_em * glyph.font_size_pt;
      index += 1;
    }

    if !text_range.is_empty() {
      // HarfBuzz may return multiple glyphs for one source cluster. Preserve
      // that cluster once in the searchable PDF layer and match its complete
      // selected-glyph advance, including authored scaling and spacing.
      if semantic_ranges.contains(&text_range) {
        return None;
      }
      let cluster_text = text.get(text_range.clone())?;
      if !cluster_text.is_empty() {
        let cluster_advance_pt = cursor_x_pt - cluster_origin_x_pt;
        items.push(math_variant_semantic_text(
          &face,
          None,
          cluster_text,
          &face_style,
          MathSemanticPlacement {
            origin_x_units: cluster_origin_x_pt / scale_pt_per_unit,
            advance_units: cluster_advance_pt / scale_pt_per_unit,
            baseline_y_pt: 0.0,
            scale_pt_per_unit,
          },
        ));
        semantic_ranges.push(text_range);
      }
    }
  }

  // Exact-outline transport has the same internal-ink contract as the SVG
  // text path above. In particular, an empty side of every glyph bounding
  // box remains empty instead of inheriting transparent font line metrics.
  Some(MathBox {
    width_pt: shaped.width_pt
      + if positioning.add_italics_to_advance {
        positioning.italics_correction_pt
      } else {
        0.0
      },
    ascent_pt: ink_ascent_pt,
    descent_pt: ink_descent_pt,
    script_base_ascent_pt: None,
    script_base_descent_pt: None,
    top_accent_attachment_pt,
    italics_correction_pt: positioning.italics_correction_pt,
    italics_correction_in_advance: positioning.add_italics_to_advance,
    text_like: positioning.text_like,
    math_kerns: positioning.math_kerns,
    items,
  })
}

fn layout_placeholder(
  source_style: &TextStyle,
  context: MathLayoutContext,
  metrics: &mut TextMetrics,
) -> MathBox {
  // ECMA-376 Part 1 §22.1.2.32 gives an empty CT_OMathArg a real argument
  // slot, and §22.1.2.83 illustrates that slot with U+2B1A DOTTED SQUARE.
  // Microsoft fixed output suppresses the editing mark itself, but retains
  // its advance and vertical metrics in surrounding math layout.
  let mut placeholder = layout_text("\u{2b1a}", source_style, context, metrics);
  placeholder.items.clear();
  placeholder
}

fn math_script_style(
  source: &TextStyle,
  context: MathLayoutContext,
  metrics: &mut TextMetrics,
) -> TextStyle {
  let mut style = source.clone();
  // ECMA-376 Part 1 §17.15.1.18 defines characterSpacingControl for
  // full-width characters in WordprocessingML. OMML math text has its own
  // typography: Microsoft's OfficeMath model applies the math-zone spacing
  // rules summarized by UnicodeMath/TeX. Do not leak the enclosing document's
  // Word text-justification trim into m:t glyph advances.
  style.cjk_punctuation_compression_ratio = 0.0;
  let script_level = context.style.script_level();
  if script_level > 0 {
    let math = metrics.math_font_metrics(source);
    let scale = if script_level == 1 {
      math.script_scale
    } else {
      math.script_script_scale
    };
    style.font_size_pt = wordprocessing_math_script_size(style.font_size_pt, scale);
    if let Some(size) = &mut style.complex_font_size_pt {
      *size = wordprocessing_math_script_size(*size, scale);
    }
  }
  style.baseline_shift_pt = 0.0;
  style
}

fn wordprocessing_math_script_size(font_size_pt: f32, scale: f32) -> f32 {
  // OpenType MATH supplies the level-1 and level-2 percentages. Word then
  // stores the realized script face on the same half-point grid used by
  // ECMA-376 Part 1 §17.3.2.39/§17.18.42, truncating to the next lower
  // representable size. This is observable for both 11 pt and 12 pt Office
  // Math: 73% realizes as 8 pt and 8.5 pt respectively.
  let scaled = font_size_pt * scale;
  let rounding_guard = f32::EPSILON * scaled.abs().max(1.0) * 4.0;
  (((scaled * 2.0) + rounding_guard).floor() / 2.0).max(MIN_MATH_SIZE_PT)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum MathSpacingClass {
  Unicode(MathClass),
  Ellipsis,
}

impl MathSpacingClass {
  const fn syntax_class(self) -> MathClass {
    match self {
      Self::Unicode(class) => class,
      // OfficeMath treats horizontal ellipsis as an ordinary atom for
      // operator-context resolution. In particular, a following Vary-class
      // operator remains binary instead of becoming unary as it would after
      // Unicode's Relation classification.
      Self::Ellipsis => MathClass::Normal,
    }
  }

  const fn resolve_vary_as_binary(&mut self) {
    if matches!(self, Self::Unicode(MathClass::Vary)) {
      *self = Self::Unicode(MathClass::Binary);
    }
  }
}

impl From<MathClass> for MathSpacingClass {
  fn from(class: MathClass) -> Self {
    Self::Unicode(class)
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct MathAtomClasses {
  class: MathClass,
  left: MathSpacingClass,
  right: MathSpacingClass,
}

impl MathAtomClasses {
  const fn single(class: MathClass) -> Self {
    Self {
      class,
      left: MathSpacingClass::Unicode(class),
      right: MathSpacingClass::Unicode(class),
    }
  }

  fn resolve_vary_as_binary(&mut self) {
    if self.class == MathClass::Vary {
      self.class = MathClass::Binary;
    }
    self.left.resolve_vary_as_binary();
    self.right.resolve_vary_as_binary();
  }
}

fn text_math_classes(text: &str, override_class: Option<MathClass>) -> MathAtomClasses {
  if let Some(class) = override_class {
    return MathAtomClasses::single(class);
  }

  // UTR #25 assigns a class to the base scalar of a grapheme. Diacritics and
  // glyph-part characters stay attached to that base and do not replace its
  // spacing class at either edge.
  let base_classes = text.chars().map(|character| {
    if character == '\u{22ef}' {
      MathSpacingClass::Ellipsis
    } else {
      MathSpacingClass::Unicode(math_character_class(character))
    }
  });
  let mut base_classes = base_classes.filter(|class| {
    !matches!(
      class.syntax_class(),
      MathClass::Diacritic | MathClass::GlyphPart
    )
  });
  let first = base_classes
    .next()
    .unwrap_or(MathSpacingClass::Unicode(MathClass::Normal));
  let last = base_classes.next_back().unwrap_or(first);
  MathAtomClasses {
    class: last.syntax_class(),
    left: first,
    right: last,
  }
}

fn math_node_classes(node: &MathNode) -> Option<MathAtomClasses> {
  match node {
    MathNode::Empty => None,
    MathNode::Placeholder { .. } => Some(MathAtomClasses::single(MathClass::Normal)),
    MathNode::Text {
      text,
      math_class_override,
      ..
    } => Some(text_math_classes(text, *math_class_override)),
    MathNode::UserSpace { .. } => Some(MathAtomClasses::single(MathClass::Space)),
    MathNode::Argument { base, .. } | MathNode::RunBackground { base, .. } => {
      math_node_classes(base)
    }
    MathNode::OperatorEmulator { base } => {
      Some(MathAtomClasses::single(operator_emulator_class(base)))
    }
    MathNode::Row(nodes) => {
      let first = nodes.iter().find_map(math_node_classes)?;
      let last = nodes
        .iter()
        .rev()
        .find_map(math_node_classes)
        .unwrap_or(first);
      Some(MathAtomClasses {
        class: MathClass::Normal,
        left: first.left,
        right: last.right,
      })
    }
    MathNode::Fraction { .. }
    | MathNode::EquationArray(_)
    | MathNode::Matrix(_)
    | MathNode::Radical { .. } => Some(MathAtomClasses::single(MathClass::Normal)),
    MathNode::Scripts { base, .. } | MathNode::Limits { base, .. } => math_node_classes(base),
    MathNode::Delimiter {
      begin,
      end,
      arguments,
      ..
    } => {
      let first_argument = arguments.iter().find_map(math_node_classes);
      let last_argument = arguments.iter().rev().find_map(math_node_classes);
      let left = if begin.is_empty() {
        first_argument.map_or(MathSpacingClass::Unicode(MathClass::Normal), |classes| {
          classes.left
        })
      } else {
        MathSpacingClass::Unicode(MathClass::Opening)
      };
      let right = if end.is_empty() {
        last_argument.map_or(left, |classes| classes.right)
      } else {
        MathSpacingClass::Unicode(MathClass::Closing)
      };
      Some(MathAtomClasses {
        class: MathClass::Normal,
        left,
        right,
      })
    }
    MathNode::Nary { base, .. } => Some(MathAtomClasses {
      class: MathClass::Normal,
      left: MathSpacingClass::Unicode(MathClass::Large),
      right: math_node_classes(base)
        .map_or(MathSpacingClass::Unicode(MathClass::Normal), |classes| {
          classes.right
        }),
    }),
    MathNode::Accent { base, .. }
    | MathNode::Bar { base, .. }
    | MathNode::GroupChar { base, .. }
    | MathNode::BorderBox { base, .. }
    | MathNode::Phantom { base, .. } => math_node_classes(base),
    MathNode::Function { name, argument } => {
      let name = math_node_classes(name);
      let argument = math_node_classes(argument);
      Some(MathAtomClasses {
        class: MathClass::Normal,
        // A recognized function is a TeX operator to the surrounding row,
        // even though its authored fName text is upright. This preserves the
        // thin implicit-multiplication space in forms such as 2 sin(x) and
        // (...) cos(x); opening delimiters and stronger binary/relation glue
        // are still resolved by the row class table.
        left: MathSpacingClass::Unicode(MathClass::Large),
        right: argument
          .or(name)
          .map_or(MathSpacingClass::Unicode(MathClass::Normal), |classes| {
            classes.right
          }),
      })
    }
  }
}

fn operator_emulator_class(base: &MathNode) -> MathClass {
  let classes = math_node_classes(base);
  let edge_class = classes.map(|classes| classes.left.syntax_class());
  if let Some(classes) = classes
    && classes.left == classes.right
    && edge_class.is_some_and(math_class_is_operator)
  {
    return edge_class.unwrap_or(MathClass::Normal);
  }
  base
    .semantic_text()
    .chars()
    .map(math_character_class)
    .find(|class| math_class_is_operator(*class))
    .or_else(|| classes.map(|classes| classes.class))
    .unwrap_or(MathClass::Normal)
}

fn math_class_is_operator(class: MathClass) -> bool {
  matches!(
    class,
    MathClass::Binary
      | MathClass::Closing
      | MathClass::Fence
      | MathClass::Large
      | MathClass::Opening
      | MathClass::Relation
      | MathClass::Unary
      | MathClass::Vary
  )
}

fn resolved_row_math_classes(nodes: &[MathNode]) -> Vec<Option<MathAtomClasses>> {
  let mut previous_class = None;
  nodes
    .iter()
    .map(|node| {
      let classes = math_node_classes(node)?;
      // UTR #25's Vary class is unary at the start of a formula or after an
      // operator/comparator, and binary only after an ordinary/closing atom.
      // Explicit spaces do not change that syntactic predecessor.
      let classes = resolve_vary_math_classes(classes, previous_class);
      if classes.class != MathClass::Space {
        previous_class = Some(classes.class);
      }
      Some(classes)
    })
    .collect()
}

fn resolve_vary_math_classes(
  mut classes: MathAtomClasses,
  previous_class: Option<MathClass>,
) -> MathAtomClasses {
  if classes.class == MathClass::Vary
    && previous_class.is_some_and(|class| {
      matches!(
        class,
        MathClass::Normal | MathClass::Alphabetic | MathClass::Closing | MathClass::Fence
      )
    })
  {
    classes.resolve_vary_as_binary();
  }
  classes
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum MathSpacingOwner {
  Left,
  Right,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct MathSpacing {
  em: f32,
  owner: MathSpacingOwner,
}

// Preserve the class-pair frame in dev/test binaries so GDB can inspect the
// complete OfficeMath spacing decision. Release builds remain inlineable.
#[cfg_attr(debug_assertions, inline(never))]
fn automatic_math_spacing(
  left: impl Into<MathSpacingClass>,
  right: impl Into<MathSpacingClass>,
  context: MathLayoutContext,
) -> Option<MathSpacing> {
  use MathClass::{Binary, Closing, Fence, Large, Opening, Punctuation, Relation, Space};

  let left = left.into();
  let right = right.into();
  let left_class = left.syntax_class();
  let right_class = right.syntax_class();
  if left_class == Space || right_class == Space {
    return None;
  }
  let in_script = context.style.script_level() > 0;
  if !in_script && left == MathSpacingClass::Ellipsis && right_class == Punctuation {
    // OfficeMath extends the TeX/UTR pair table with an Ellipsis atom. Its
    // syntax behavior is ordinary, but Office fixed output retains a six-mu
    // trailing gap before punctuation. Keeping this as a pair rule preserves
    // the generic Relation -> Punctuation zero-glue counterexample.
    return Some(MathSpacing {
      em: MATH_VERY_THICK_SPACE_EM,
      owner: MathSpacingOwner::Left,
    });
  }
  match (left_class, right_class) {
    // TeX/UTR-derived spacing order used by Typst: punctuation owns a thin
    // trailing space, opening/closing delimiters suppress adjacent glue,
    // relations own thick glue, binary operators medium glue, and large
    // operators thin glue. Relation-to-relation has no internal gap.
    (_, Punctuation) => None,
    (Punctuation, _) if !in_script => Some(MathSpacing {
      em: MATH_THIN_SPACE_EM,
      owner: MathSpacingOwner::Left,
    }),
    (Opening, _) | (_, Closing) => None,
    (Relation, Relation) => None,
    (Relation, _) if !in_script => Some(MathSpacing {
      em: MATH_THICK_SPACE_EM,
      owner: MathSpacingOwner::Left,
    }),
    (_, Relation) if !in_script => Some(MathSpacing {
      em: MATH_THICK_SPACE_EM,
      owner: MathSpacingOwner::Right,
    }),
    (Binary, _) if !in_script => Some(MathSpacing {
      em: MATH_MEDIUM_SPACE_EM,
      owner: MathSpacingOwner::Left,
    }),
    (_, Binary) if !in_script => Some(MathSpacing {
      em: MATH_MEDIUM_SPACE_EM,
      owner: MathSpacingOwner::Right,
    }),
    (Large, Opening | Fence) => None,
    (Large, _) => Some(MathSpacing {
      em: MATH_THIN_SPACE_EM,
      owner: MathSpacingOwner::Left,
    }),
    (_, Large) => Some(MathSpacing {
      em: MATH_THIN_SPACE_EM,
      owner: MathSpacingOwner::Right,
    }),
    _ => None,
  }
}

fn row_has_automatic_math_spacing(nodes: &[MathNode], context: MathLayoutContext) -> bool {
  let classes = resolved_row_math_classes(nodes);
  let mut previous: Option<MathAtomClasses> = None;
  for classes in classes.into_iter().flatten() {
    if let Some(left) = previous
      && automatic_math_spacing(left.right, classes.left, context).is_some()
    {
      return true;
    }
    previous = Some(classes);
  }
  false
}

fn layout_row(
  nodes: &[MathNode],
  context: MathLayoutContext,
  metrics: &mut TextMetrics,
) -> MathBox {
  let classes = resolved_row_math_classes(nodes);
  layout_row_with_classes(nodes, &classes, context, metrics)
}

fn layout_row_with_classes(
  nodes: &[MathNode],
  classes: &[Option<MathAtomClasses>],
  context: MathLayoutContext,
  metrics: &mut TextMetrics,
) -> MathBox {
  debug_assert_eq!(nodes.len(), classes.len());
  let mut result = MathBox::empty();
  let mut x = 0.0;
  let mut previous_spacing_node: Option<usize> = None;
  for (index, node) in nodes.iter().enumerate() {
    if let Some(right_classes) = classes[index] {
      if let Some(previous_index) = previous_spacing_node {
        x += math_node_spacing(
          &nodes[previous_index],
          node,
          classes[previous_index].expect("spacing node has math classes"),
          right_classes,
          context,
          metrics,
        );
      }
      previous_spacing_node = Some(index);
    }
    let child = layout_node(node, context, metrics);
    result.append(child, x, 0.0);
    x = result.width_pt;
  }
  result.width_pt = x;
  result
}

// Keep the realized glue and its owning styles visible beside the class-pair
// frame during source-level golden diagnostics without affecting release code.
#[cfg_attr(debug_assertions, inline(never))]
fn math_node_spacing(
  left: &MathNode,
  right: &MathNode,
  left_classes: MathAtomClasses,
  right_classes: MathAtomClasses,
  context: MathLayoutContext,
  metrics: &mut TextMetrics,
) -> f32 {
  if math_node_is_function_application_marker(right) {
    return 0.0;
  }
  let spacing = if math_node_is_function_application_marker(left) {
    function_application_spacing(right, context)
  } else {
    automatic_math_spacing(left_classes.right, right_classes.left, context)
      .or_else(|| office_math_object_surround_spacing(left, right, context))
  };
  let Some(spacing) = spacing else {
    return 0.0;
  };
  let owner = match spacing.owner {
    MathSpacingOwner::Left => representative_style(left).or_else(|| representative_style(right)),
    MathSpacingOwner::Right => representative_style(right).or_else(|| representative_style(left)),
  };
  owner
    .map(|style| math_script_style(style, context, metrics).font_size_pt * spacing.em)
    .unwrap_or(0.0)
}

fn math_node_is_function_application_marker(node: &MathNode) -> bool {
  match node {
    MathNode::Text { text, .. } => text == "\u{2061}",
    MathNode::Argument { base, .. } | MathNode::RunBackground { base, .. } => {
      math_node_is_function_application_marker(base)
    }
    MathNode::Row(nodes) if nodes.len() == 1 => math_node_is_function_application_marker(&nodes[0]),
    _ => false,
  }
}

fn office_math_object_surround_spacing(
  left: &MathNode,
  right: &MathNode,
  context: MathLayoutContext,
) -> Option<MathSpacing> {
  // Microsoft's *Mathematical Typesetting* describes an extended spacing
  // layer between built-up math-handler objects, above the TeX operator-class
  // table. A vertical fraction is one such object. Its weak thin glue stays
  // outside an enclosing delimiter and is naturally absent at a math-zone
  // edge; a stronger binary/relation spacing rule has already won above.
  if context.style.script_level() > 0 {
    return None;
  }
  if math_node_is_vertical_fraction_object(right) {
    Some(MathSpacing {
      em: MATH_THIN_SPACE_EM,
      owner: MathSpacingOwner::Right,
    })
  } else if math_node_is_vertical_fraction_object(left) {
    Some(MathSpacing {
      em: MATH_THIN_SPACE_EM,
      owner: MathSpacingOwner::Left,
    })
  } else {
    None
  }
}

fn math_node_is_vertical_fraction_object(node: &MathNode) -> bool {
  match node {
    MathNode::Fraction { kind, .. } => matches!(
      kind,
      m::FractionTypeValues::Bar | m::FractionTypeValues::NoBar
    ),
    MathNode::Argument { base, .. }
    | MathNode::RunBackground { base, .. }
    | MathNode::OperatorEmulator { base }
    | MathNode::Scripts { base, .. }
    | MathNode::Limits { base, .. }
    | MathNode::Accent { base, .. }
    | MathNode::Bar { base, .. }
    | MathNode::GroupChar { base, .. }
    | MathNode::BorderBox { base, .. }
    | MathNode::Phantom { base, .. } => math_node_is_vertical_fraction_object(base),
    MathNode::Delimiter { arguments, .. } => {
      arguments.len() == 1 && math_node_is_vertical_fraction_object(&arguments[0])
    }
    _ => false,
  }
}

fn representative_style(node: &MathNode) -> Option<&TextStyle> {
  match node {
    MathNode::Text { style, .. }
    | MathNode::UserSpace { style, .. }
    | MathNode::Placeholder { style } => Some(style),
    MathNode::Argument { base, .. } | MathNode::OperatorEmulator { base } => {
      representative_style(base)
    }
    MathNode::RunBackground { base, style } => representative_style(base).or(Some(style)),
    MathNode::Row(nodes) | MathNode::EquationArray(nodes) => {
      nodes.iter().find_map(representative_style)
    }
    MathNode::Matrix(rows) => rows.iter().flatten().find_map(representative_style),
    MathNode::Fraction {
      numerator,
      denominator,
      ..
    } => representative_style(numerator).or_else(|| representative_style(denominator)),
    MathNode::Scripts {
      base, lower, upper, ..
    }
    | MathNode::Limits {
      base, lower, upper, ..
    } => representative_style(base)
      .or_else(|| lower.as_deref().and_then(representative_style))
      .or_else(|| upper.as_deref().and_then(representative_style)),
    MathNode::Delimiter { arguments, .. } => arguments.iter().find_map(representative_style),
    MathNode::Nary {
      base,
      lower,
      upper,
      style,
      ..
    } => representative_style(base)
      .or_else(|| lower.as_deref().and_then(representative_style))
      .or_else(|| upper.as_deref().and_then(representative_style))
      .or(Some(style)),
    MathNode::Radical { degree, base, .. } => {
      representative_style(base).or_else(|| degree.as_deref().and_then(representative_style))
    }
    MathNode::Accent { base, .. }
    | MathNode::Bar { base, .. }
    | MathNode::GroupChar { base, .. }
    | MathNode::BorderBox { base, .. }
    | MathNode::Phantom { base, .. } => representative_style(base),
    MathNode::Function { name, argument } => {
      representative_style(name).or_else(|| representative_style(argument))
    }
    MathNode::Empty => None,
  }
}

fn layout_fraction(
  numerator: &MathNode,
  denominator: &MathNode,
  kind: m::FractionTypeValues,
  control_style: Option<&TextStyle>,
  context: MathLayoutContext,
  metrics: &mut TextMetrics,
) -> MathBox {
  if kind == m::FractionTypeValues::Linear {
    let Some(style) = control_style
      .or_else(|| representative_style(numerator))
      .or_else(|| representative_style(denominator))
    else {
      return MathBox::empty();
    };
    let slash_text = MathNode::Text {
      text: "\u{2044}".to_string(),
      style: style.clone(),
      math_class_override: None,
    };
    let slash = if style.highlight.is_some() {
      MathNode::RunBackground {
        base: Box::new(slash_text),
        style: style.clone(),
      }
    } else {
      slash_text
    };
    let nodes = [numerator.clone(), slash, denominator.clone()];
    return layout_row(&nodes, context.same_style_in_fraction(kind), metrics);
  }

  let Some(style) = control_style
    .or_else(|| representative_style(numerator))
    .or_else(|| representative_style(denominator))
    .cloned()
  else {
    // Empty binomials still occupy no visual area.
    return MathBox::empty();
  };
  let argument_context = context.fraction_argument(kind);
  let numerator = layout_node(numerator, argument_context, metrics);
  let denominator = layout_node(denominator, argument_context, metrics);
  let effective_style = math_script_style(&style, context, metrics);
  let math = metrics.math_font_metrics(&effective_style);

  if kind == m::FractionTypeValues::Skewed {
    let vertical_gap = math.skewed_fraction_vertical_gap_pt;
    let horizontal_gap = math.skewed_fraction_horizontal_gap_pt;
    let numerator_height = numerator.ascent_pt + numerator.descent_pt;
    let denominator_height = denominator.ascent_pt + denominator.descent_pt;
    let content_height = numerator_height + denominator_height + vertical_gap;
    let mut slash = layout_stretched_symbol(
      "\u{2044}",
      &style,
      context,
      content_height,
      content_height,
      MathVariantSizePolicy::AtLeast,
      metrics,
    );
    apply_control_background(&mut slash, &style);
    let slash_height = slash.ascent_pt + slash.descent_pt;
    let height = content_height.max(slash_height);
    let vertical_offset = ((slash_height - content_height) / 2.0).max(0.0);
    let baseline_from_top = height / 2.0 + math.axis_height_pt;

    let mut numerator_x = 0.0;
    let mut slash_x = numerator.width_pt + horizontal_gap / 2.0 - slash.width_pt / 2.0;
    let mut denominator_x = numerator.width_pt + horizontal_gap;
    let horizontal_offset = (-slash_x).max(0.0);
    numerator_x += horizontal_offset;
    slash_x += horizontal_offset;
    denominator_x += horizontal_offset;
    let width = (denominator_x + denominator.width_pt).max(slash_x + slash.width_pt);

    let numerator_y = vertical_offset + numerator.ascent_pt - baseline_from_top;
    let denominator_y =
      vertical_offset + numerator_height + vertical_gap + denominator.ascent_pt - baseline_from_top;
    let slash_y = (height - slash_height) / 2.0 + slash.ascent_pt - baseline_from_top;
    let mut result = MathBox::empty();
    result.append(numerator, numerator_x, numerator_y);
    result.append(denominator, denominator_x, denominator_y);
    result.append(slash, slash_x, slash_y);
    result.width_pt = width;
    return result;
  }

  // TeX Appendix G's `make_fraction` first reboxes the narrower argument to
  // the wider argument, then gives the fraction vbox exactly that width. Its
  // rule spans the same box. OfficeMath follows those algorithms and applies
  // its additional spacing between math-handler objects, so do not turn
  // Typst's configurable fraction padding or LibreOffice's fraction-bar
  // excess into transparent space inside every OMML fraction.
  let content_width = numerator.width_pt.max(denominator.width_pt);
  let width = content_width;

  if kind == m::FractionTypeValues::NoBar {
    let (shift_up, shift_down, gap_min) = if context.style == MathLayoutStyle::Display {
      (
        math.stack_top_display_style_shift_up_pt,
        math.stack_bottom_display_style_shift_down_pt,
        math.stack_display_style_gap_min_pt,
      )
    } else {
      (
        math.stack_top_shift_up_pt,
        math.stack_bottom_shift_down_pt,
        math.stack_gap_min_pt,
      )
    };
    let natural_gap = (shift_up - numerator.descent_pt) + (shift_down - denominator.ascent_pt);
    let extra = ((gap_min - natural_gap) / 2.0).max(0.0);
    let numerator_y = -shift_up - extra;
    let denominator_y = shift_down + extra;
    let numerator_width = numerator.width_pt;
    let denominator_width = denominator.width_pt;
    let mut result = MathBox::empty();
    result.append(
      numerator,
      (content_width - numerator_width) / 2.0,
      numerator_y,
    );
    result.append(
      denominator,
      (content_width - denominator_width) / 2.0,
      denominator_y,
    );
    result.width_pt = width;
    return result;
  }

  let rule = math.fraction_rule_thickness_pt.max(MIN_RULE_WIDTH_PT);
  let axis_y = -math.axis_height_pt;
  let (numerator_shift, denominator_shift, numerator_gap, denominator_gap) =
    if context.style == MathLayoutStyle::Display {
      (
        math.fraction_numerator_display_style_shift_up_pt,
        math.fraction_denominator_display_style_shift_down_pt,
        math.fraction_num_display_style_gap_min_pt,
        math.fraction_denom_display_style_gap_min_pt,
      )
    } else {
      (
        math.fraction_numerator_shift_up_pt,
        math.fraction_denominator_shift_down_pt,
        math.fraction_numerator_gap_min_pt,
        math.fraction_denominator_gap_min_pt,
      )
    };
  let numerator_y =
    (axis_y - rule / 2.0 - numerator_gap - numerator.descent_pt).min(-numerator_shift);
  let denominator_y =
    (axis_y + rule / 2.0 + denominator_gap + denominator.ascent_pt).max(denominator_shift);
  let mut result = MathBox::empty();
  let numerator_width = numerator.width_pt;
  let denominator_width = denominator.width_pt;
  result.append(
    numerator,
    (content_width - numerator_width) / 2.0,
    numerator_y,
  );
  result.append(
    denominator,
    (content_width - denominator_width) / 2.0,
    denominator_y,
  );
  let mut bar = MathBox::empty();
  bar.width_pt = content_width;
  bar.ascent_pt = (-axis_y + rule / 2.0).max(0.0);
  bar.descent_pt = (axis_y + rule / 2.0).max(0.0);
  bar.items.push(MathPaintItem::Line {
    x1_pt: 0.0,
    y1_pt: axis_y,
    x2_pt: content_width,
    y2_pt: axis_y,
    width_pt: rule,
    color: effective_style.color,
    opacity: effective_style.opacity,
  });
  apply_control_background(&mut bar, &effective_style);
  result.append(bar, 0.0, 0.0);
  result.width_pt = width;
  result
}

fn layout_scripts(
  base: &MathNode,
  lower: Option<&MathNode>,
  upper: Option<&MathNode>,
  pre: bool,
  context: MathLayoutContext,
  metrics: &mut TextMetrics,
) -> MathBox {
  let base_box = layout_node(base, context, metrics);
  let lower_box = lower.map(|node| layout_node(node, context.script(), metrics));
  let upper_box = upper.map(|node| layout_node(node, context.script(), metrics));
  let Some(style) = representative_style(base)
    .or_else(|| lower.and_then(representative_style))
    .or_else(|| upper.and_then(representative_style))
  else {
    return base_box;
  };
  let math = metrics.math_font_metrics(style);
  layout_scripts_around_box(base_box, lower_box, upper_box, pre, false, math)
}

fn side_script_math_kern(
  base: &MathBox,
  script: &MathBox,
  baseline_shift_pt: f32,
  corner: MathKernCorner,
) -> f32 {
  let (first_height_pt, second_height_pt) = match corner {
    MathKernCorner::TopRight | MathKernCorner::TopLeft => (
      base.ascent_pt - baseline_shift_pt,
      baseline_shift_pt - script.descent_pt,
    ),
    MathKernCorner::BottomRight | MathKernCorner::BottomLeft => (
      script.ascent_pt - baseline_shift_pt,
      baseline_shift_pt - base.descent_pt,
    ),
  };
  let summed_kern = |height_pt| {
    base.math_kerns.at_height(corner, height_pt)
      + script.math_kerns.at_height(corner.opposite(), height_pt)
  };
  // OpenType 1.9.1 specifies the minimum of the sums evaluated at the two
  // correction heights. Missing corner tables contribute the specified zero.
  summed_kern(first_height_pt).min(summed_kern(second_height_pt))
}

fn layout_scripts_around_box(
  base_box: MathBox,
  lower_box: Option<MathBox>,
  upper_box: Option<MathBox>,
  pre: bool,
  cramped: bool,
  math: MathFontMetrics,
) -> MathBox {
  let mut upper_shift = upper_box.as_ref().map(|upper| {
    let standard_shift = if cramped {
      math.superscript_shift_up_cramped_pt
    } else {
      math.superscript_shift_up_pt
    };
    standard_shift
      .max(if base_box.text_like {
        0.0
      } else {
        base_box.script_base_ascent_pt() - math.superscript_baseline_drop_max_pt
      })
      .max(math.superscript_bottom_min_pt + upper.descent_pt)
  });
  let mut lower_shift = lower_box.as_ref().map(|lower| {
    math
      .subscript_shift_down_pt
      .max(if base_box.text_like {
        0.0
      } else {
        base_box.script_base_descent_pt() + math.subscript_baseline_drop_min_pt
      })
      .max(lower.ascent_pt - math.subscript_top_max_pt)
  });
  if let (Some(upper), Some(lower), Some(upper_shift_value), Some(lower_shift_value)) = (
    upper_box.as_ref(),
    lower_box.as_ref(),
    upper_shift,
    lower_shift,
  ) {
    let upper_bottom = upper_shift_value - upper.descent_pt;
    let lower_top = lower.ascent_pt - lower_shift_value;
    let gap = upper_bottom - lower_top;
    if gap < math.sub_superscript_gap_min_pt {
      let increase = math.sub_superscript_gap_min_pt - gap;
      let upper_only =
        (math.superscript_bottom_max_with_subscript_pt - upper_bottom).clamp(0.0, increase);
      let shared = (increase - upper_only) / 2.0;
      upper_shift = Some(upper_shift_value + upper_only + shared);
      lower_shift = Some(lower_shift_value + shared);
    }
  }

  let upper_kern = upper_box.as_ref().zip(upper_shift).map(|(upper, shift)| {
    side_script_math_kern(
      &base_box,
      upper,
      shift,
      if pre {
        MathKernCorner::TopLeft
      } else {
        MathKernCorner::TopRight
      },
    )
  });
  let lower_kern = lower_box.as_ref().zip(lower_shift).map(|(lower, shift)| {
    side_script_math_kern(
      &base_box,
      lower,
      shift,
      if pre {
        MathKernCorner::BottomLeft
      } else {
        MathKernCorner::BottomRight
      },
    )
  });
  let space_after_script = math.space_after_script_pt.max(0.0);
  let upper_extent = upper_box
    .as_ref()
    .zip(upper_kern)
    .map_or(0.0, |(upper, kern)| {
      (upper.width_pt + kern + space_after_script).max(0.0)
    });
  let lower_extent = lower_box
    .as_ref()
    .zip(lower_kern)
    .map_or(0.0, |(lower, kern)| {
      let italics = if pre {
        0.0
      } else {
        base_box.italics_correction_pt
      };
      (lower.width_pt + kern - italics + space_after_script).max(0.0)
    });
  let script_extent = upper_extent.max(lower_extent);
  let mut result = MathBox::empty();
  let base_x = if pre { script_extent } else { 0.0 };
  let base_width = base_box.width_pt;
  let base_italics = base_box.italics_correction_pt;
  let script_base_ascent = base_box.script_base_ascent_pt();
  let script_base_descent = base_box.script_base_descent_pt();
  result.append(base_box, base_x, 0.0);
  if let (Some(lower), Some(shift), Some(kern)) = (lower_box, lower_shift, lower_kern) {
    let x = if pre {
      base_x - lower.width_pt - kern
    } else {
      base_width + kern - base_italics
    };
    result.append(lower, x, shift);
  }
  if let (Some(upper), Some(shift), Some(kern)) = (upper_box, upper_shift, upper_kern) {
    let x = if pre {
      base_x - upper.width_pt - kern
    } else {
      base_width + kern
    };
    result.append(upper, x, -shift);
  }
  result.width_pt = script_extent + base_width;
  result.script_base_ascent_pt = Some(script_base_ascent);
  result.script_base_descent_pt = Some(script_base_descent);
  result
}

fn layout_function(
  name: &MathNode,
  argument: &MathNode,
  context: MathLayoutContext,
  metrics: &mut TextMetrics,
) -> MathBox {
  let name_box = layout_node(name, context, metrics);
  let argument_box = layout_node(argument, context, metrics);
  let gap_em = function_application_spacing(argument, context).map_or(0.0, |spacing| spacing.em);
  let gap = representative_style(name)
    .or_else(|| representative_style(argument))
    .map_or(0.0, |style| style.font_size_pt * gap_em);
  let name_width = name_box.width_pt;
  let mut result = MathBox::empty();
  result.append(name_box, 0.0, 0.0);
  result.append(argument_box, name_width + gap, 0.0);
  result
}

fn function_application_spacing(
  argument: &MathNode,
  context: MathLayoutContext,
) -> Option<MathSpacing> {
  // ECMA-376 §22.1.2.39 defines m:func through U+2061 FUNCTION
  // APPLICATION. OfficeMath applies the TeX math-class table to that
  // invisible operator: an ordinary argument gets thin operator glue, an
  // opening/fence is contiguous, and the em scales with the current math
  // style. MathML permits renderers to suppress most operator spacing in
  // scripts but does not require that choice; OfficeMath's fraction-argument
  // output and Typst's TeX-derived Large-class table both retain this gap.
  let argument_left = math_node_classes(argument)
    .map_or(MathSpacingClass::Unicode(MathClass::Normal), |classes| {
      classes.left
    });
  automatic_math_spacing(MathClass::Large, argument_left, context)
}

fn nary_argument_spacing(base: &MathNode, context: MathLayoutContext) -> Option<MathSpacing> {
  // ECMA-376 Part 1 §22.1.2.70 makes the operand an argument inside the
  // n-ary object rather than a following row atom. Microsoft's OfficeMath
  // description says that this explicit n-aryand boundary is used to compute
  // the correct math spacing, and that true inline zones use reduced spacing.
  // Current full-fidelity Word applies the ordinary math-class boundary in a
  // true inline zone: Large + Opening/Fence is contiguous, while an ordinary
  // operand retains thin glue. Word 2007 compatibility and display math keep
  // the original n-aryand gap independently of an opening delimiter; Writer
  // models that path in SmOperNode::Arrange with DIS_OPERATORSPACE. The paired
  // Office fixed-output counterexamples are tdf133030.docx (mode 15 inline)
  // and equation.docx (omitted mode => mode 12 inline). An explicit leading
  // space owns either boundary and suppresses inferred glue.
  let base_left = math_node_classes(base)
    .map_or(MathSpacingClass::Unicode(MathClass::Normal), |classes| {
      classes.left
    });
  if context.compatibility_mode >= 15 && !context.display_math {
    return automatic_math_spacing(MathClass::Large, base_left, context);
  }
  (base_left.syntax_class() != MathClass::Space).then_some(MathSpacing {
    em: MATH_THIN_SPACE_EM,
    owner: MathSpacingOwner::Left,
  })
}

fn layout_limits(
  base: &MathNode,
  lower: Option<&MathNode>,
  upper: Option<&MathNode>,
  side: bool,
  context: MathLayoutContext,
  metrics: &mut TextMetrics,
) -> MathBox {
  if side {
    return layout_scripts(base, lower, upper, false, context, metrics);
  }
  let base_box = layout_node(base, context, metrics);
  let lower_box = lower.map(|node| layout_node(node, context.script(), metrics));
  let upper_box = upper.map(|node| layout_node(node, context.script(), metrics));
  let Some(style) = representative_style(base)
    .or_else(|| lower.and_then(representative_style))
    .or_else(|| upper.and_then(representative_style))
  else {
    return base_box;
  };
  let math = metrics.math_font_metrics(style);
  layout_limits_around_box(base_box, lower_box, upper_box, math)
}

fn layout_limits_around_box(
  base_box: MathBox,
  lower_box: Option<MathBox>,
  upper_box: Option<MathBox>,
  math: MathFontMetrics,
) -> MathBox {
  if lower_box.is_none() && upper_box.is_none() {
    return base_box;
  }
  let base_width = base_box.width_pt;
  let base_ascent = base_box.ascent_pt;
  let base_descent = base_box.descent_pt;
  let script_base_ascent = base_box.script_base_ascent_pt();
  let script_base_descent = base_box.script_base_descent_pt();
  let correction_delta = base_box.italics_correction_pt / 2.0;
  let upper_x = upper_box
    .as_ref()
    .map(|upper| (base_width - upper.width_pt) / 2.0 + correction_delta);
  let lower_x = lower_box
    .as_ref()
    .map(|lower| (base_width - lower.width_pt) / 2.0 - correction_delta);
  let left = upper_x.into_iter().chain(lower_x).fold(0.0_f32, f32::min);
  let right = upper_box
    .as_ref()
    .zip(upper_x)
    .map(|(upper, x)| x + upper.width_pt)
    .into_iter()
    .chain(
      lower_box
        .as_ref()
        .zip(lower_x)
        .map(|(lower, x)| x + lower.width_pt),
    )
    .fold(base_width, f32::max);
  let width = right - left;
  let mut result = MathBox::empty();
  result.append(base_box, -left, 0.0);
  if let Some(lower) = lower_box {
    let y = limit_baseline_distance_pt(
      base_descent,
      lower.ascent_pt,
      math.lower_limit_gap_min_pt,
      math.lower_limit_baseline_drop_min_pt,
    );
    result.append(lower, lower_x.unwrap_or(-left) - left, y);
  }
  if let Some(upper) = upper_box {
    let y = -limit_baseline_distance_pt(
      base_ascent,
      upper.descent_pt,
      math.upper_limit_gap_min_pt,
      math.upper_limit_baseline_rise_min_pt,
    );
    result.append(upper, upper_x.unwrap_or(-left) - left, y);
  }
  result.width_pt = width;
  result.script_base_ascent_pt = Some(script_base_ascent);
  result.script_base_descent_pt = Some(script_base_descent);
  result
}

fn limit_baseline_distance_pt(
  base_ink_extent_pt: f32,
  limit_ink_extent_pt: f32,
  ink_gap_min_pt: f32,
  baseline_distance_min_pt: f32,
) -> f32 {
  base_ink_extent_pt + baseline_distance_min_pt.max(ink_gap_min_pt + limit_ink_extent_pt)
}

struct DelimiterLayoutSpec<'a> {
  begin: &'a str,
  separator: &'a str,
  end: &'a str,
  grow: bool,
  shape: m::ShapeDelimiterValues,
  control_style: Option<&'a TextStyle>,
}

fn layout_delimiter(
  spec: DelimiterLayoutSpec<'_>,
  arguments: &[MathNode],
  context: MathLayoutContext,
  metrics: &mut TextMetrics,
) -> MathBox {
  let DelimiterLayoutSpec {
    begin,
    separator,
    end,
    grow,
    shape,
    control_style,
  } = spec;
  let Some(style) = control_style.or_else(|| arguments.iter().find_map(representative_style))
  else {
    return MathBox::empty();
  };
  let argument_boxes = arguments
    .iter()
    .map(|argument| layout_node(argument, context, metrics))
    .collect::<Vec<_>>();
  let content_ascent_pt = argument_boxes
    .iter()
    .map(|argument| argument.ascent_pt)
    .fold(0.0_f32, f32::max);
  let content_descent_pt = argument_boxes
    .iter()
    .map(|argument| argument.descent_pt)
    .fold(0.0_f32, f32::max);
  let delimiter_axis_height_pt = if grow {
    let effective_style = math_script_style(style, context, metrics);
    metrics.math_font_metrics(&effective_style).axis_height_pt
  } else {
    0.0
  };
  let delimiter = |symbol: &str, metrics: &mut TextMetrics| {
    if symbol.is_empty() {
      MathBox::empty()
    } else if !grow {
      // ECMA-376 Part 1 §22.1.2.43 defines delimiter growth as matching the
      // operand height. With growth disabled, retain the ordinary glyph;
      // asking the MATH table for an em-high variant can enlarge even a
      // one-line delimiter whose base glyph already fits its operand.
      layout_text(symbol, style, context, metrics)
    } else {
      let delimiter = layout_stretched_symbol(
        symbol,
        style,
        context,
        content_ascent_pt + content_descent_pt,
        0.0,
        MathVariantSizePolicy::Closest,
        metrics,
      );
      // ECMA-376 Part 1 §22.1.2.97 centers a growing delimiter on the
      // mathematical axis by default. This applies even when the base cmap
      // glyph already satisfies the operand target and no prepared MATH
      // variant is selected. Prepared variants and assemblies arrive
      // axis-centered already, so the same complete step is a no-op for them.
      let mut delimiter = center_math_box_on_axis(delimiter, delimiter_axis_height_pt);
      apply_control_background(&mut delimiter, style);
      delimiter
    }
  };
  let begin_box = delimiter(begin, metrics);
  let end_box = delimiter(end, metrics);
  let delimiter_baseline_y_pt = if grow && shape == m::ShapeDelimiterValues::Match {
    // ECMA-376 Part 1 §22.1.2.97 distinguishes delimiters centered on the
    // math axis from delimiters matched to the argument. [MS-OI29500]
    // §22.1.2.97 further specifies that Office ignores shape when delimiter
    // growth is off. A growing prepared MATH variant is intrinsically
    // centered on AxisHeight, so translate its center to the argument's
    // center while retaining the font-authored outline and advance.
    (content_descent_pt - content_ascent_pt) / 2.0 + delimiter_axis_height_pt
  } else {
    0.0
  };

  // All separators belong to the same delimiter object. Measure every
  // argument first so a tall later argument grows earlier separators to the
  // same target instead of leaving them at a prefix-local height.
  let mut contents = MathBox::empty();
  let mut x = 0.0;
  for (index, argument) in argument_boxes.into_iter().enumerate() {
    if index > 0 && !separator.is_empty() {
      let separator_box = delimiter(separator, metrics);
      contents.append(separator_box, x, delimiter_baseline_y_pt);
      x = contents.width_pt;
    }
    contents.append(argument, x, 0.0);
    x = contents.width_pt;
  }
  contents.width_pt = x;
  let mut result = MathBox::empty();
  let begin_width = begin_box.width_pt;
  result.append(begin_box, 0.0, delimiter_baseline_y_pt);
  // TeX Appendix G assigns no automatic space from an opening atom to its
  // argument or from the argument to a closing atom. OfficeMath uses those
  // algorithms, and ECMA-376 models these characters as the delimiter object
  // itself rather than as independently spaced operators.
  result.append(contents, begin_width, 0.0);
  let content_end = result.width_pt;
  result.append(end_box, content_end, delimiter_baseline_y_pt);
  result
}

fn center_math_box_on_axis(math_box: MathBox, axis_height_pt: f32) -> MathBox {
  let glyph_center_above_baseline_pt = (math_box.ascent_pt - math_box.descent_pt) / 2.0;
  math_box.rebase_to_baseline(axis_height_pt - glyph_center_above_baseline_pt)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum MathVariantSizePolicy {
  /// The requested size is a minimum, as for a non-growing display operator.
  AtLeast,
  /// Office size-matching objects use the prepared variant closest to the
  /// measured expression. The caller may retain a separate OpenType minimum;
  /// an assembly is used only when every prepared variant is too small. See
  /// Microsoft's *Mathematical Typesetting* (2007), pp. 31–32.
  Closest,
}

fn layout_stretched_symbol(
  symbol: &str,
  source_style: &TextStyle,
  context: MathLayoutContext,
  target_height_pt: f32,
  minimum_height_pt: f32,
  size_policy: MathVariantSizePolicy,
  metrics: &mut TextMetrics,
) -> MathBox {
  let normal = layout_text(symbol, source_style, context, metrics);
  let normal_height = (normal.ascent_pt + normal.descent_pt).max(1.0);
  if target_height_pt <= normal_height * 1.05 {
    return normal;
  }
  let style = math_script_style(source_style, context, metrics);
  layout_vertical_math_variant(
    symbol,
    &style,
    target_height_pt,
    minimum_height_pt,
    size_policy,
    metrics,
  )
  .unwrap_or(normal)
}

#[derive(Clone, Debug)]
struct MathGlyphGeometry {
  path_data: Arc<str>,
  x_min_units: f32,
  y_min_units: f32,
  x_max_units: f32,
  y_max_units: f32,
  horizontal_advance_units: f32,
}

#[derive(Clone, Debug)]
struct MathGlyphPositioning {
  italics_correction_pt: f32,
  add_italics_to_advance: bool,
  text_like: bool,
  math_kerns: MathGlyphKerns,
}

impl Default for MathGlyphPositioning {
  fn default() -> Self {
    Self {
      italics_correction_pt: 0.0,
      add_italics_to_advance: true,
      // A glyph not listed as an OpenType extended shape follows the ordinary
      // text-like side-script constraints.
      text_like: true,
      math_kerns: MathGlyphKerns::default(),
    }
  }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct MathAssemblyPart {
  glyph_id: u16,
  start_connector_units: f32,
  end_connector_units: f32,
  full_advance_units: f32,
  extender: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct MathAssemblyPlacement {
  part: MathAssemblyPart,
  offset_units: f32,
}

#[derive(Clone, Debug, PartialEq)]
struct MathAssemblyPlan {
  placements: Vec<MathAssemblyPlacement>,
  extent_units: f32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct MathGlyphVariant {
  glyph_id: u16,
  advance_units: u16,
}

fn prepared_math_variant_for_target(
  variants: &[MathGlyphVariant],
  target_units: f32,
  minimum_units: f32,
  size_policy: MathVariantSizePolicy,
) -> Option<MathGlyphVariant> {
  // When every prepared form is too small, OpenType MATH directs the client
  // to the authored assembly. Do not let the closest-size policy suppress
  // that transition by returning the largest undersized glyph.
  if !variants
    .iter()
    .any(|variant| f32::from(variant.advance_units) >= target_units)
  {
    return None;
  }

  match size_policy {
    MathVariantSizePolicy::AtLeast => variants
      .iter()
      .filter(|variant| f32::from(variant.advance_units) >= target_units)
      .min_by_key(|variant| variant.advance_units)
      .copied(),
    MathVariantSizePolicy::Closest => variants
      .iter()
      .filter(|variant| f32::from(variant.advance_units) >= minimum_units)
      .min_by(|left, right| {
        (f32::from(left.advance_units) - target_units)
          .abs()
          .total_cmp(&(f32::from(right.advance_units) - target_units).abs())
      })
      .copied(),
  }
}

#[derive(Clone, Debug, PartialEq)]
struct MathGlyphAssembly {
  italics_correction_units: f32,
  parts: Vec<MathAssemblyPart>,
}

#[derive(Clone, Debug, PartialEq)]
struct MathGlyphConstruction {
  min_connector_overlap_units: u16,
  variants: Vec<MathGlyphVariant>,
  assembly: Option<MathGlyphAssembly>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum MathStretchAxis {
  Horizontal,
  Vertical,
}

/// Reads the complete horizontal or vertical construction for one glyph from
/// an OpenType MATH table. The MATH structures are not typed by the
/// `read-fonts` version re-exported by our locked `skrifa`; shared Coverage
/// formats 1 and 2 are, so those stay delegated to fontations while the
/// MATH-relative offsets are resolved here according to the OpenType contract.
fn math_glyph_construction(
  table: FontData<'_>,
  glyph_id: u16,
  axis: MathStretchAxis,
) -> Option<MathGlyphConstruction> {
  let major_version = table.read_at::<u16>(0).ok()?;
  let _minor_version = table.read_at::<u16>(2).ok()?;
  if major_version != 1 {
    return None;
  }

  let variants_offset = usize::from(table.read_at::<u16>(8).ok()?);
  if variants_offset == 0 {
    return None;
  }
  let variants_table = table.split_off(variants_offset)?;
  let min_connector_overlap_units = variants_table.read_at::<u16>(0).ok()?;
  let vertical_coverage_offset = usize::from(variants_table.read_at::<u16>(2).ok()?);
  let horizontal_coverage_offset = usize::from(variants_table.read_at::<u16>(4).ok()?);
  let vertical_glyph_count = usize::from(variants_table.read_at::<u16>(6).ok()?);
  let horizontal_glyph_count = usize::from(variants_table.read_at::<u16>(8).ok()?);
  let vertical_offsets_end = 10usize.checked_add(vertical_glyph_count.checked_mul(2)?)?;
  let all_offsets_end = vertical_offsets_end.checked_add(horizontal_glyph_count.checked_mul(2)?)?;
  variants_table.slice(..all_offsets_end)?;
  let (coverage_offset, glyph_count, construction_offsets_start) = match axis {
    MathStretchAxis::Vertical => (vertical_coverage_offset, vertical_glyph_count, 10),
    MathStretchAxis::Horizontal => (
      horizontal_coverage_offset,
      horizontal_glyph_count,
      vertical_offsets_end,
    ),
  };
  if coverage_offset == 0 {
    return None;
  }

  let coverage = CoverageTable::read(variants_table.split_off(coverage_offset)?).ok()?;
  let construction_index = usize::from(coverage.get(GlyphId::new(u32::from(glyph_id)))?);
  if construction_index >= glyph_count {
    return None;
  }
  let construction_offset_position =
    construction_offsets_start.checked_add(construction_index.checked_mul(2)?)?;
  let construction_offset = usize::from(
    variants_table
      .read_at::<u16>(construction_offset_position)
      .ok()?,
  );
  if construction_offset == 0 {
    return None;
  }

  let construction_table = variants_table.split_off(construction_offset)?;
  let assembly_offset = usize::from(construction_table.read_at::<u16>(0).ok()?);
  let variant_count = usize::from(construction_table.read_at::<u16>(2).ok()?);
  let variant_records_end = 4usize.checked_add(variant_count.checked_mul(4)?)?;
  construction_table.slice(..variant_records_end)?;
  let mut variants = Vec::with_capacity(variant_count);
  for index in 0..variant_count {
    let offset = 4usize.checked_add(index.checked_mul(4)?)?;
    variants.push(MathGlyphVariant {
      glyph_id: construction_table.read_at::<u16>(offset).ok()?,
      advance_units: construction_table
        .read_at::<u16>(offset.checked_add(2)?)
        .ok()?,
    });
  }

  let assembly = if assembly_offset == 0 {
    None
  } else {
    let assembly_table = construction_table.split_off(assembly_offset)?;
    // MathValueRecord is a signed design-unit value followed by an optional
    // device/variation offset. Retain the authored correction for callers
    // that attach scripts to the realized construction, while validating the
    // complete record keeps partCount aligned with the specified layout.
    let italics_correction_units = f32::from(assembly_table.read_at::<i16>(0).ok()?);
    let _italics_device_or_variation_offset = assembly_table.read_at::<u16>(2).ok()?;
    let part_count = usize::from(assembly_table.read_at::<u16>(4).ok()?);
    let part_records_end = 6usize.checked_add(part_count.checked_mul(10)?)?;
    assembly_table.slice(..part_records_end)?;
    let mut parts = Vec::with_capacity(part_count);
    for index in 0..part_count {
      let offset = 6usize.checked_add(index.checked_mul(10)?)?;
      parts.push(MathAssemblyPart {
        glyph_id: assembly_table.read_at::<u16>(offset).ok()?,
        start_connector_units: f32::from(
          assembly_table.read_at::<u16>(offset.checked_add(2)?).ok()?,
        ),
        end_connector_units: f32::from(assembly_table.read_at::<u16>(offset.checked_add(4)?).ok()?),
        full_advance_units: f32::from(assembly_table.read_at::<u16>(offset.checked_add(6)?).ok()?),
        extender: assembly_table.read_at::<u16>(offset.checked_add(8)?).ok()? & 0x0001 != 0,
      });
    }
    Some(MathGlyphAssembly {
      italics_correction_units,
      parts,
    })
  };

  Some(MathGlyphConstruction {
    min_connector_overlap_units,
    variants,
    assembly,
  })
}

fn math_glyph_info_table(table: FontData<'_>) -> Option<FontData<'_>> {
  if table.read_at::<u16>(0).ok()? != 1 {
    return None;
  }
  let glyph_info_offset = usize::from(table.read_at::<u16>(6).ok()?);
  if glyph_info_offset == 0 {
    return None;
  }
  table.split_off(glyph_info_offset)
}

/// Reads one coverage-indexed MathValueRecord array under MathGlyphInfo.
/// MATH uses the shared OpenType Coverage formats, so coverage decoding stays
/// delegated to fontations while all MATH-relative offsets and complete
/// records are validated here.
fn math_glyph_info_value(
  table: FontData<'_>,
  glyph_id: u16,
  glyph_info_field_offset: usize,
) -> Option<f32> {
  let glyph_info = math_glyph_info_table(table)?;
  let values_offset = usize::from(glyph_info.read_at::<u16>(glyph_info_field_offset).ok()?);
  if values_offset == 0 {
    return None;
  }
  let values = glyph_info.split_off(values_offset)?;
  let coverage_offset = usize::from(values.read_at::<u16>(0).ok()?);
  let value_count = usize::from(values.read_at::<u16>(2).ok()?);
  if coverage_offset == 0 {
    return None;
  }
  let records_end = 4usize.checked_add(value_count.checked_mul(4)?)?;
  values.slice(..records_end)?;
  let coverage = CoverageTable::read(values.split_off(coverage_offset)?).ok()?;
  let value_index = usize::from(coverage.get(GlyphId::new(u32::from(glyph_id)))?);
  if value_index >= value_count {
    return None;
  }
  let record_offset = 4usize.checked_add(value_index.checked_mul(4)?)?;
  let value = values.read_at::<i16>(record_offset).ok()?;
  // Validate the complete MathValueRecord even though fixed-output point
  // layout has no device-pixel delta to apply at this stage.
  let _device_or_variation_offset = values.read_at::<u16>(record_offset.checked_add(2)?).ok()?;
  Some(f32::from(value))
}

/// Returns the authored horizontal attachment for a top accent in font
/// design units.
fn math_top_accent_attachment(table: FontData<'_>, glyph_id: u16) -> Option<f32> {
  math_glyph_info_value(table, glyph_id, 2)
}

/// Returns the authored protrusion correction for a slanted glyph in font
/// design units. Glyphs outside the coverage have an implicit zero value.
fn math_italics_correction(table: FontData<'_>, glyph_id: u16) -> Option<f32> {
  math_glyph_info_value(table, glyph_id, 0)
}

fn math_is_extended_shape(table: FontData<'_>, glyph_id: u16) -> bool {
  let Some(glyph_info) = math_glyph_info_table(table) else {
    return false;
  };
  let Ok(extended_shape_offset) = glyph_info.read_at::<u16>(4) else {
    return false;
  };
  if extended_shape_offset == 0 {
    return false;
  }
  glyph_info
    .split_off(usize::from(extended_shape_offset))
    .and_then(|coverage| CoverageTable::read(coverage).ok())
    .and_then(|coverage| coverage.get(GlyphId::new(u32::from(glyph_id))))
    .is_some()
}

fn read_math_kern_table(
  kern_info: FontData<'_>,
  offset: usize,
  scale_pt_per_unit: f32,
  horizontal_scale: f32,
) -> Option<Option<MathKernTable>> {
  if offset == 0 {
    return Some(None);
  }
  let table = kern_info.split_off(offset)?;
  let height_count = usize::from(table.read_at::<u16>(0).ok()?);
  let heights_start = 2usize;
  let kern_values_start = heights_start.checked_add(height_count.checked_mul(4)?)?;
  let records_end = kern_values_start.checked_add(height_count.checked_add(1)?.checked_mul(4)?)?;
  table.slice(..records_end)?;

  let mut correction_heights_pt = Vec::with_capacity(height_count);
  for index in 0..height_count {
    let record_offset = heights_start.checked_add(index.checked_mul(4)?)?;
    correction_heights_pt
      .push(f32::from(table.read_at::<i16>(record_offset).ok()?) * scale_pt_per_unit);
    let _device_or_variation_offset = table.read_at::<u16>(record_offset.checked_add(2)?).ok()?;
  }

  let mut kern_values_pt = Vec::with_capacity(height_count + 1);
  for index in 0..=height_count {
    let record_offset = kern_values_start.checked_add(index.checked_mul(4)?)?;
    kern_values_pt.push(
      f32::from(table.read_at::<i16>(record_offset).ok()?) * scale_pt_per_unit * horizontal_scale,
    );
    let _device_or_variation_offset = table.read_at::<u16>(record_offset.checked_add(2)?).ok()?;
  }

  Some(Some(MathKernTable {
    correction_heights_pt,
    kern_values_pt,
  }))
}

/// Reads all four height-dependent mathematical kerning tables for a glyph.
/// A missing corner has the specified zero-kern behavior; a malformed covered
/// record rejects the whole lookup so partially shifted scripts are avoided.
fn math_glyph_kerns(
  table: FontData<'_>,
  glyph_id: u16,
  scale_pt_per_unit: f32,
  horizontal_scale: f32,
) -> Option<MathGlyphKerns> {
  let glyph_info = math_glyph_info_table(table)?;
  let kern_info_offset = usize::from(glyph_info.read_at::<u16>(6).ok()?);
  if kern_info_offset == 0 {
    return None;
  }
  let kern_info = glyph_info.split_off(kern_info_offset)?;
  let coverage_offset = usize::from(kern_info.read_at::<u16>(0).ok()?);
  let kern_count = usize::from(kern_info.read_at::<u16>(2).ok()?);
  if coverage_offset == 0 {
    return None;
  }
  let records_end = 4usize.checked_add(kern_count.checked_mul(8)?)?;
  kern_info.slice(..records_end)?;
  let coverage = CoverageTable::read(kern_info.split_off(coverage_offset)?).ok()?;
  let record_index = usize::from(coverage.get(GlyphId::new(u32::from(glyph_id)))?);
  if record_index >= kern_count {
    return None;
  }
  let record_offset = 4usize.checked_add(record_index.checked_mul(8)?)?;
  let corner = |field_offset: usize| -> Option<Option<MathKernTable>> {
    let offset = usize::from(
      kern_info
        .read_at::<u16>(record_offset.checked_add(field_offset)?)
        .ok()?,
    );
    read_math_kern_table(kern_info, offset, scale_pt_per_unit, horizontal_scale)
  };
  Some(MathGlyphKerns {
    top_right: corner(0)?,
    top_left: corner(2)?,
    bottom_right: corner(4)?,
    bottom_left: corner(6)?,
  })
}

fn math_glyph_positioning(
  face: &FontRef<'_>,
  glyph_id: GlyphId,
  scale_pt_per_unit: f32,
  horizontal_scale: f32,
) -> MathGlyphPositioning {
  let Ok(glyph_id) = u16::try_from(glyph_id.to_u32()) else {
    return MathGlyphPositioning::default();
  };
  let Some(table) = face.table_data(Tag::new(b"MATH")) else {
    return MathGlyphPositioning::default();
  };
  let extended_shape = math_is_extended_shape(table, glyph_id);
  MathGlyphPositioning {
    italics_correction_pt: math_italics_correction(table, glyph_id).unwrap_or(0.0)
      * scale_pt_per_unit
      * horizontal_scale,
    add_italics_to_advance: !extended_shape,
    text_like: !extended_shape,
    math_kerns: math_glyph_kerns(table, glyph_id, scale_pt_per_unit, horizontal_scale)
      .unwrap_or_default(),
  }
}

fn shaped_math_positioning(shaped: &ShapedText, style: &TextStyle) -> MathGlyphPositioning {
  let horizontal_scale = style.horizontal_scale.unwrap_or(1.0).max(f32::EPSILON);
  let positions = shaped
    .glyphs
    .iter()
    .map(|glyph| {
      let font_face = shaped.font_faces.get(glyph.font_index)?;
      let face = FontRef::from_index(font_face.data.as_ref(), font_face.index).ok()?;
      let units_per_em = f32::from(face.head().ok()?.units_per_em()).max(1.0);
      Some(math_glyph_positioning(
        &face,
        GlyphId::new(glyph.glyph_id),
        glyph.font_size_pt / units_per_em,
        horizontal_scale,
      ))
    })
    .collect::<Option<Vec<_>>>();
  let Some(positions) = positions else {
    return MathGlyphPositioning::default();
  };
  let (Some(first), Some(last)) = (positions.first(), positions.last()) else {
    return MathGlyphPositioning::default();
  };
  MathGlyphPositioning {
    italics_correction_pt: last.italics_correction_pt,
    add_italics_to_advance: last.add_italics_to_advance,
    text_like: positions.iter().all(|positioning| positioning.text_like),
    math_kerns: MathGlyphKerns {
      top_right: last.math_kerns.top_right.clone(),
      top_left: first.math_kerns.top_left.clone(),
      bottom_right: last.math_kerns.bottom_right.clone(),
      bottom_left: first.math_kerns.bottom_left.clone(),
    },
  }
}

fn shaped_top_accent_attachment_pt(shaped: &ShapedText, style: &TextStyle) -> Option<f32> {
  let [glyph] = shaped.glyphs.as_slice() else {
    return None;
  };
  let font_face = shaped.font_faces.get(glyph.font_index)?;
  let face = FontRef::from_index(font_face.data.as_ref(), font_face.index).ok()?;
  let units_per_em = f32::from(face.head().ok()?.units_per_em()).max(1.0);
  let glyph_id = u16::try_from(glyph.glyph_id).ok()?;
  let table = face.table_data(Tag::new(b"MATH"))?;
  let attachment_units = math_top_accent_attachment(table, glyph_id)?;
  let horizontal_scale = style.horizontal_scale.unwrap_or(1.0).max(f32::EPSILON);
  Some(
    glyph.x_offset_em * glyph.font_size_pt
      + attachment_units * glyph.font_size_pt / units_per_em * horizontal_scale,
  )
}

fn prepared_top_accent_attachment_pt(
  face: &FontRef<'_>,
  glyph_id: GlyphId,
  horizontal_shift_units: f32,
  scale_pt_per_unit: f32,
) -> Option<f32> {
  let glyph_id = u16::try_from(glyph_id.to_u32()).ok()?;
  let table = face.table_data(Tag::new(b"MATH"))?;
  math_top_accent_attachment(table, glyph_id)
    .map(|attachment| (horizontal_shift_units + attachment) * scale_pt_per_unit)
}

fn layout_vertical_math_variant(
  symbol: &str,
  style: &TextStyle,
  target_height_pt: f32,
  minimum_height_pt: f32,
  size_policy: MathVariantSizePolicy,
  metrics: &mut TextMetrics,
) -> Option<MathBox> {
  let shaped = metrics.shape_text(symbol, style)?;
  let [glyph] = shaped.glyphs.as_slice() else {
    return None;
  };
  let font_face = shaped.font_faces.get(glyph.font_index)?;
  let face = FontRef::from_index(font_face.data.as_ref(), font_face.index).ok()?;
  let units_per_em = f32::from(face.head().ok()?.units_per_em()).max(1.0);
  let scale_pt_per_unit = glyph.font_size_pt.max(MIN_MATH_SIZE_PT) / units_per_em;
  let target_units = target_height_pt.max(0.0) / scale_pt_per_unit;
  let minimum_units = minimum_height_pt.max(0.0) / scale_pt_per_unit;
  let glyph_id = u16::try_from(glyph.glyph_id).ok()?;
  let math_table = face.table_data(Tag::new(b"MATH"))?;
  let construction = math_glyph_construction(math_table, glyph_id, MathStretchAxis::Vertical)?;
  let axis_height_pt = metrics.math_font_metrics(style).axis_height_pt;

  let largest_variant = construction
    .variants
    .iter()
    .max_by_key(|variant| variant.advance_units)
    .map(|variant| GlyphId::new(u32::from(variant.glyph_id)));
  if let Some(variant) = prepared_math_variant_for_target(
    &construction.variants,
    target_units,
    minimum_units,
    size_policy,
  ) {
    return layout_prepared_math_variant(
      &face,
      GlyphId::new(u32::from(variant.glyph_id)),
      symbol,
      style,
      scale_pt_per_unit,
      axis_height_pt,
    );
  }

  if let Some(assembly) = construction.assembly
    && let Some(plan) = plan_math_glyph_assembly(
      &assembly.parts,
      f32::from(construction.min_connector_overlap_units),
      target_units,
    )
    && let Some(result) = layout_vertical_math_glyph_assembly(
      &face,
      &plan,
      assembly.italics_correction_units,
      symbol,
      style,
      scale_pt_per_unit,
      axis_height_pt,
    )
  {
    return Some(result);
  }

  // The OpenType MATH fallback after exhausting prepared variants is the
  // largest supplied glyph. Do not synthesize a geometrically unrelated
  // anisotropic scale when the font provides no assembly.
  layout_prepared_math_variant(
    &face,
    largest_variant?,
    symbol,
    style,
    scale_pt_per_unit,
    axis_height_pt,
  )
}

fn layout_horizontal_math_variant(
  symbol: &str,
  style: &TextStyle,
  target_width_pt: f32,
  features: &[FeatureValue],
  metrics: &mut TextMetrics,
) -> Option<MathBox> {
  let shaped = if features.is_empty() {
    metrics.shape_text(symbol, style)?
  } else {
    metrics.shape_text_with_features(symbol, style, features)?
  };
  let [glyph] = shaped.glyphs.as_slice() else {
    return None;
  };
  let font_face = shaped.font_faces.get(glyph.font_index)?;
  let face = FontRef::from_index(font_face.data.as_ref(), font_face.index).ok()?;
  let units_per_em = f32::from(face.head().ok()?.units_per_em()).max(1.0);
  let scale_pt_per_unit = glyph.font_size_pt.max(MIN_MATH_SIZE_PT) / units_per_em;
  let target_units = target_width_pt.max(0.0) / scale_pt_per_unit;
  let glyph_id = u16::try_from(glyph.glyph_id).ok()?;
  let math_table = face.table_data(Tag::new(b"MATH"))?;
  let construction = math_glyph_construction(math_table, glyph_id, MathStretchAxis::Horizontal)?;

  let mut largest_variant = None;
  for variant in construction.variants {
    let variant_glyph = GlyphId::new(u32::from(variant.glyph_id));
    largest_variant = Some(variant_glyph);
    if f32::from(variant.advance_units) >= target_units {
      return layout_prepared_horizontal_math_variant(
        &face,
        variant_glyph,
        symbol,
        style,
        scale_pt_per_unit,
      );
    }
  }

  if let Some(assembly) = construction.assembly
    && let Some(plan) = plan_math_glyph_assembly(
      &assembly.parts,
      f32::from(construction.min_connector_overlap_units),
      target_units,
    )
    && let Some(result) = layout_horizontal_math_glyph_assembly(
      &face,
      &plan,
      assembly.italics_correction_units,
      symbol,
      style,
      scale_pt_per_unit,
    )
  {
    return Some(result);
  }

  // As for vertical growth, the specified fallback is the largest prepared
  // variant. A horizontal assembly may not be replaced by anisotropic scaling
  // of an unrelated cmap outline.
  layout_prepared_horizontal_math_variant(&face, largest_variant?, symbol, style, scale_pt_per_unit)
}

fn layout_prepared_horizontal_math_variant(
  face: &FontRef<'_>,
  glyph_id: GlyphId,
  semantic_text: &str,
  style: &TextStyle,
  scale_pt_per_unit: f32,
) -> Option<MathBox> {
  let geometry = math_glyph_geometry(face, glyph_id)?;
  let horizontal_shift_units = (-geometry.x_min_units).max(0.0);
  let ascent_pt = (geometry.y_max_units * scale_pt_per_unit).max(0.0);
  let descent_pt = (-geometry.y_min_units * scale_pt_per_unit).max(0.0);
  let positioning = math_glyph_positioning(face, glyph_id, scale_pt_per_unit, 1.0);
  let width_pt = ((geometry.horizontal_advance_units + horizontal_shift_units) * scale_pt_per_unit
    + if positioning.add_italics_to_advance {
      positioning.italics_correction_pt
    } else {
      0.0
    })
  .max((geometry.x_max_units + horizontal_shift_units) * scale_pt_per_unit)
  .max(0.0);
  let semantic = math_variant_semantic_text(
    face,
    Some(glyph_id),
    semantic_text,
    style,
    MathSemanticPlacement {
      origin_x_units: horizontal_shift_units,
      advance_units: geometry.horizontal_advance_units,
      baseline_y_pt: 0.0,
      scale_pt_per_unit,
    },
  );
  Some(MathBox {
    width_pt,
    ascent_pt,
    descent_pt,
    script_base_ascent_pt: None,
    script_base_descent_pt: None,
    top_accent_attachment_pt: prepared_top_accent_attachment_pt(
      face,
      glyph_id,
      horizontal_shift_units,
      scale_pt_per_unit,
    ),
    italics_correction_pt: positioning.italics_correction_pt,
    italics_correction_in_advance: positioning.add_italics_to_advance,
    text_like: positioning.text_like,
    math_kerns: positioning.math_kerns,
    items: vec![
      MathPaintItem::GlyphPath {
        path_data: geometry.path_data,
        style: style.clone(),
        x_pt: horizontal_shift_units * scale_pt_per_unit,
        baseline_y_pt: 0.0,
        scale_pt_per_unit,
        horizontal_scale: 1.0,
        synthetic_bold: false,
        synthetic_italic: false,
        opacity: 1.0,
      },
      semantic,
    ],
  })
}

fn layout_prepared_math_variant(
  face: &FontRef<'_>,
  glyph_id: GlyphId,
  semantic_text: &str,
  style: &TextStyle,
  scale_pt_per_unit: f32,
  axis_height_pt: f32,
) -> Option<MathBox> {
  let geometry = math_glyph_geometry(face, glyph_id)?;
  let axis_units = axis_height_pt / scale_pt_per_unit;
  let vertical_shift_units = axis_units - (geometry.y_min_units + geometry.y_max_units) / 2.0;
  let horizontal_shift_units = (-geometry.x_min_units).max(0.0);
  let ascent_pt = ((geometry.y_max_units + vertical_shift_units) * scale_pt_per_unit).max(0.0);
  let descent_pt = (-(geometry.y_min_units + vertical_shift_units) * scale_pt_per_unit).max(0.0);
  let positioning = math_glyph_positioning(face, glyph_id, scale_pt_per_unit, 1.0);
  let width_pt = ((geometry.horizontal_advance_units + horizontal_shift_units) * scale_pt_per_unit
    + if positioning.add_italics_to_advance {
      positioning.italics_correction_pt
    } else {
      0.0
    })
  .max((geometry.x_max_units + horizontal_shift_units) * scale_pt_per_unit)
  .max(0.0);
  let semantic = math_variant_semantic_text(
    face,
    Some(glyph_id),
    semantic_text,
    style,
    MathSemanticPlacement {
      origin_x_units: horizontal_shift_units,
      advance_units: geometry.horizontal_advance_units,
      baseline_y_pt: -vertical_shift_units * scale_pt_per_unit,
      scale_pt_per_unit,
    },
  );
  Some(MathBox {
    width_pt,
    ascent_pt,
    descent_pt,
    script_base_ascent_pt: None,
    script_base_descent_pt: None,
    top_accent_attachment_pt: prepared_top_accent_attachment_pt(
      face,
      glyph_id,
      horizontal_shift_units,
      scale_pt_per_unit,
    ),
    italics_correction_pt: positioning.italics_correction_pt,
    italics_correction_in_advance: positioning.add_italics_to_advance,
    text_like: positioning.text_like,
    math_kerns: positioning.math_kerns,
    items: vec![
      MathPaintItem::GlyphPath {
        path_data: geometry.path_data,
        style: style.clone(),
        x_pt: horizontal_shift_units * scale_pt_per_unit,
        baseline_y_pt: -vertical_shift_units * scale_pt_per_unit,
        scale_pt_per_unit,
        horizontal_scale: 1.0,
        synthetic_bold: false,
        synthetic_italic: false,
        opacity: 1.0,
      },
      semantic,
    ],
  })
}

fn math_assembly_kerns(
  face: &FontRef<'_>,
  plan: &MathAssemblyPlan,
  axis: MathStretchAxis,
  scale_pt_per_unit: f32,
) -> MathGlyphKerns {
  let (Some(first), Some(last)) = (plan.placements.first(), plan.placements.last()) else {
    return MathGlyphKerns::default();
  };
  let first = math_glyph_positioning(
    face,
    GlyphId::new(u32::from(first.part.glyph_id)),
    scale_pt_per_unit,
    1.0,
  );
  let last = math_glyph_positioning(
    face,
    GlyphId::new(u32::from(last.part.glyph_id)),
    scale_pt_per_unit,
    1.0,
  );
  match axis {
    // OpenType stores vertical assembly parts bottom-to-top.
    MathStretchAxis::Vertical => MathGlyphKerns {
      top_right: last.math_kerns.top_right,
      top_left: last.math_kerns.top_left,
      bottom_right: first.math_kerns.bottom_right,
      bottom_left: first.math_kerns.bottom_left,
    },
    MathStretchAxis::Horizontal => MathGlyphKerns {
      top_right: last.math_kerns.top_right,
      top_left: first.math_kerns.top_left,
      bottom_right: last.math_kerns.bottom_right,
      bottom_left: first.math_kerns.bottom_left,
    },
  }
}

fn layout_vertical_math_glyph_assembly(
  face: &FontRef<'_>,
  plan: &MathAssemblyPlan,
  italics_correction_units: f32,
  semantic_text: &str,
  style: &TextStyle,
  scale_pt_per_unit: f32,
  axis_height_pt: f32,
) -> Option<MathBox> {
  if plan.placements.is_empty() || plan.extent_units <= f32::EPSILON {
    return None;
  }
  let geometries = plan
    .placements
    .iter()
    .map(|placement| {
      math_glyph_geometry(face, GlyphId::new(u32::from(placement.part.glyph_id)))
        .map(|geometry| (placement, geometry))
    })
    .collect::<Option<Vec<_>>>()?;
  let x_min_units = geometries
    .iter()
    .map(|(_, geometry)| geometry.x_min_units)
    .fold(0.0_f32, f32::min);
  let horizontal_shift_units = -x_min_units;
  let width_units = geometries
    .iter()
    .map(|(_, geometry)| geometry.horizontal_advance_units.max(geometry.x_max_units))
    .fold(0.0_f32, f32::max)
    + horizontal_shift_units;
  let logical_ascent_pt = plan.extent_units * scale_pt_per_unit / 2.0 + axis_height_pt;
  let logical_descent_pt = plan.extent_units * scale_pt_per_unit / 2.0 - axis_height_pt;
  let logical_descent_units = logical_descent_pt / scale_pt_per_unit;
  let mut ascent_pt = logical_ascent_pt.max(0.0);
  let mut descent_pt = logical_descent_pt.max(0.0);
  let mut items = Vec::with_capacity(geometries.len() * 2);
  for (part_index, (placement, geometry)) in geometries.into_iter().enumerate() {
    // OpenType stores vertical parts bottom-to-top. Math assembly glyphs are
    // normally aligned at their vertical origin; matching Typst's complete
    // implementation, compensate a non-zero outline descent before applying
    // the bottom-to-top advance.
    let glyph_descent_units = -geometry.y_min_units;
    let baseline_up_units = placement.offset_units + glyph_descent_units;
    let baseline_y_pt = (logical_descent_units - baseline_up_units) * scale_pt_per_unit;
    let ink_top_pt = baseline_y_pt - geometry.y_max_units * scale_pt_per_unit;
    let ink_bottom_pt = baseline_y_pt - geometry.y_min_units * scale_pt_per_unit;
    ascent_pt = ascent_pt.max(-ink_top_pt);
    descent_pt = descent_pt.max(ink_bottom_pt);
    items.push(MathPaintItem::GlyphPath {
      path_data: geometry.path_data,
      style: style.clone(),
      x_pt: horizontal_shift_units * scale_pt_per_unit,
      baseline_y_pt,
      scale_pt_per_unit,
      horizontal_scale: 1.0,
      synthetic_bold: false,
      synthetic_italic: false,
      opacity: 1.0,
    });
    items.push(math_variant_semantic_text(
      face,
      Some(GlyphId::new(u32::from(placement.part.glyph_id))),
      if part_index == 0 { semantic_text } else { " " },
      style,
      MathSemanticPlacement {
        origin_x_units: horizontal_shift_units,
        advance_units: geometry.horizontal_advance_units,
        baseline_y_pt,
        scale_pt_per_unit,
      },
    ));
  }
  Some(MathBox {
    width_pt: width_units.max(0.0) * scale_pt_per_unit,
    ascent_pt,
    descent_pt,
    script_base_ascent_pt: None,
    script_base_descent_pt: None,
    top_accent_attachment_pt: None,
    italics_correction_pt: italics_correction_units * scale_pt_per_unit,
    italics_correction_in_advance: false,
    text_like: false,
    math_kerns: math_assembly_kerns(face, plan, MathStretchAxis::Vertical, scale_pt_per_unit),
    items,
  })
}

fn layout_horizontal_math_glyph_assembly(
  face: &FontRef<'_>,
  plan: &MathAssemblyPlan,
  italics_correction_units: f32,
  semantic_text: &str,
  style: &TextStyle,
  scale_pt_per_unit: f32,
) -> Option<MathBox> {
  if plan.placements.is_empty() || plan.extent_units <= f32::EPSILON {
    return None;
  }
  let geometries = plan
    .placements
    .iter()
    .map(|placement| {
      math_glyph_geometry(face, GlyphId::new(u32::from(placement.part.glyph_id)))
        .map(|geometry| (placement, geometry))
    })
    .collect::<Option<Vec<_>>>()?;
  let ink_left_units = geometries
    .iter()
    .map(|(placement, geometry)| placement.offset_units + geometry.x_min_units)
    .fold(0.0_f32, f32::min);
  let horizontal_shift_units = -ink_left_units;
  let ink_right_units = geometries
    .iter()
    .map(|(placement, geometry)| placement.offset_units + geometry.x_max_units)
    .fold(0.0_f32, f32::max);
  let width_units = plan.extent_units.max(ink_right_units) + horizontal_shift_units;
  let mut ascent_pt = 0.0_f32;
  let mut descent_pt = 0.0_f32;
  let mut items = Vec::with_capacity(geometries.len() * 2);
  for (part_index, (placement, geometry)) in geometries.into_iter().enumerate() {
    ascent_pt = ascent_pt.max(geometry.y_max_units * scale_pt_per_unit);
    descent_pt = descent_pt.max(-geometry.y_min_units * scale_pt_per_unit);
    let part_origin_units = horizontal_shift_units + placement.offset_units;
    items.push(MathPaintItem::GlyphPath {
      path_data: geometry.path_data,
      style: style.clone(),
      x_pt: part_origin_units * scale_pt_per_unit,
      baseline_y_pt: 0.0,
      scale_pt_per_unit,
      horizontal_scale: 1.0,
      synthetic_bold: false,
      synthetic_italic: false,
      opacity: 1.0,
    });
    // A horizontal MATH assembly is a sequence of authored font glyphs, not
    // one stretched glyph. Word preserves that sequence in the PDF text
    // layer: the first part maps back to the source scalar and continuation
    // parts map to spaces, each retaining its own glyph advance and origin.
    // Keep the exact outlines visible while reproducing that per-part text
    // geometry beneath the semantic clip.
    items.push(math_variant_semantic_text(
      face,
      Some(GlyphId::new(u32::from(placement.part.glyph_id))),
      if part_index == 0 { semantic_text } else { " " },
      style,
      MathSemanticPlacement {
        origin_x_units: part_origin_units,
        advance_units: geometry.horizontal_advance_units,
        baseline_y_pt: 0.0,
        scale_pt_per_unit,
      },
    ));
  }
  Some(MathBox {
    width_pt: width_units.max(0.0) * scale_pt_per_unit,
    ascent_pt,
    descent_pt,
    script_base_ascent_pt: None,
    script_base_descent_pt: None,
    top_accent_attachment_pt: Some(width_units.max(0.0) * scale_pt_per_unit / 2.0),
    italics_correction_pt: italics_correction_units * scale_pt_per_unit,
    italics_correction_in_advance: false,
    text_like: false,
    math_kerns: math_assembly_kerns(face, plan, MathStretchAxis::Horizontal, scale_pt_per_unit),
    items,
  })
}

#[derive(Clone, Copy, Debug)]
struct MathSemanticPlacement {
  origin_x_units: f32,
  advance_units: f32,
  baseline_y_pt: f32,
  scale_pt_per_unit: f32,
}

fn math_variant_semantic_text(
  face: &FontRef<'_>,
  glyph_id: Option<GlyphId>,
  text: &str,
  style: &TextStyle,
  placement: MathSemanticPlacement,
) -> MathPaintItem {
  let MathSemanticPlacement {
    origin_x_units,
    advance_units,
    baseline_y_pt,
    scale_pt_per_unit,
  } = placement;
  // The PDF text object must retain the selected MATH glyph's horizontal
  // contract even though its searchable Unicode maps to the base cmap glyph.
  // Office embeds that variant advance in the CID font /W array. Reproduce
  // the same text matrix while the empty clip keeps the base outline hidden.
  let horizontal_scale = if glyph_id.is_some() {
    1.0
  } else {
    let charmap = face.charmap();
    let glyph_metrics = face.glyph_metrics(Size::unscaled(), LocationRef::default());
    let base_advance_units = text
      .chars()
      .filter_map(|character| charmap.map(character))
      .filter_map(|glyph_id| glyph_metrics.advance_width(glyph_id))
      .sum::<f32>();
    if base_advance_units > f32::EPSILON {
      advance_units.max(0.0) / base_advance_units
    } else {
      1.0
    }
  };
  MathPaintItem::SemanticText {
    text: text.to_string(),
    style: style.clone(),
    x_pt: origin_x_units * scale_pt_per_unit,
    baseline_y_pt,
    horizontal_scale,
    glyph_id: glyph_id.map(GlyphId::to_u32),
  }
}

fn plan_math_glyph_assembly(
  authored_parts: &[MathAssemblyPart],
  min_connector_overlap_units: f32,
  target_units: f32,
) -> Option<MathAssemblyPlan> {
  let min_connector_overlap_units = min_connector_overlap_units.max(0.0);
  let target_units = target_units.max(0.0);
  for extender_repeats in 0..=MAX_MATH_ASSEMBLY_EXTENDER_REPEATS {
    let parts = authored_parts
      .iter()
      .flat_map(|part| {
        let count = if part.extender { extender_repeats } else { 1 };
        std::iter::repeat_n(*part, count)
      })
      .collect::<Vec<_>>();
    if parts.is_empty() {
      continue;
    }
    let (minimum_extent, growable_extent) =
      math_assembly_extents(&parts, min_connector_overlap_units);
    let spread_ratio = if minimum_extent < target_units && growable_extent > f32::EPSILON {
      ((target_units - minimum_extent) / growable_extent).clamp(0.0, 1.0)
    } else {
      0.0
    };
    let extent_units = minimum_extent + spread_ratio * growable_extent;
    if extent_units + f32::EPSILON < target_units
      && extender_repeats < MAX_MATH_ASSEMBLY_EXTENDER_REPEATS
    {
      continue;
    }

    let mut offset_units = 0.0;
    let mut placements = Vec::with_capacity(parts.len());
    for (index, part) in parts.iter().copied().enumerate() {
      placements.push(MathAssemblyPlacement { part, offset_units });
      if let Some(next) = parts.get(index + 1) {
        let maximum_overlap = part
          .end_connector_units
          .min(next.start_connector_units)
          .max(0.0);
        let distributable_overlap = (maximum_overlap - min_connector_overlap_units).max(0.0);
        offset_units +=
          part.full_advance_units - maximum_overlap + spread_ratio * distributable_overlap;
      }
    }
    let measured_extent = placements
      .last()
      .map(|placement| placement.offset_units + placement.part.full_advance_units)?;
    return Some(MathAssemblyPlan {
      placements,
      extent_units: measured_extent.max(extent_units),
    });
  }
  None
}

fn math_assembly_extents(
  parts: &[MathAssemblyPart],
  min_connector_overlap_units: f32,
) -> (f32, f32) {
  let mut minimum_extent = 0.0;
  let mut growable_extent = 0.0;
  for (index, part) in parts.iter().enumerate() {
    minimum_extent += part.full_advance_units;
    if let Some(next) = parts.get(index + 1) {
      let maximum_overlap = part
        .end_connector_units
        .min(next.start_connector_units)
        .max(0.0);
      minimum_extent -= maximum_overlap;
      growable_extent += (maximum_overlap - min_connector_overlap_units).max(0.0);
    }
  }
  (minimum_extent.max(0.0), growable_extent)
}

fn math_glyph_geometry(face: &FontRef<'_>, glyph_id: GlyphId) -> Option<MathGlyphGeometry> {
  let glyph_metrics = face.glyph_metrics(Size::unscaled(), LocationRef::default());
  let bounds = glyph_metrics.bounds(glyph_id)?;
  let mut outline = SvgGlyphOutline::default();
  face
    .outline_glyphs()
    .get(glyph_id)?
    .draw(
      DrawSettings::unhinted(Size::unscaled(), LocationRef::default()),
      &mut outline,
    )
    .ok()?;
  let horizontal_advance_units = glyph_metrics
    .advance_width(glyph_id)
    .unwrap_or_else(|| (bounds.x_max - bounds.x_min).max(0.0));
  Some(MathGlyphGeometry {
    path_data: Arc::from(outline.path),
    x_min_units: bounds.x_min,
    y_min_units: bounds.y_min,
    x_max_units: bounds.x_max,
    y_max_units: bounds.y_max,
    horizontal_advance_units,
  })
}

#[derive(Default)]
struct SvgGlyphOutline {
  path: String,
}

impl OutlinePen for SvgGlyphOutline {
  fn move_to(&mut self, x: f32, y: f32) {
    let _ = write!(self.path, "M{x:.3} {y:.3}");
  }

  fn line_to(&mut self, x: f32, y: f32) {
    let _ = write!(self.path, "L{x:.3} {y:.3}");
  }

  fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
    let _ = write!(self.path, "Q{x1:.3} {y1:.3} {x:.3} {y:.3}");
  }

  fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
    let _ = write!(self.path, "C{x1:.3} {y1:.3} {x2:.3} {y2:.3} {x:.3} {y:.3}");
  }

  fn close(&mut self) {
    self.path.push('Z');
  }
}

fn layout_stack(
  rows: &[MathNode],
  context: MathLayoutContext,
  metrics: &mut TextMetrics,
  gap_em: f32,
) -> MathBox {
  let boxes = rows
    .iter()
    .map(|row| layout_node(row, context, metrics))
    .collect::<Vec<_>>();
  let Some(style) = rows.iter().find_map(representative_style) else {
    return MathBox::empty();
  };
  let width = boxes
    .iter()
    .map(|value| value.width_pt)
    .fold(0.0_f32, f32::max);
  let gap = style.font_size_pt * gap_em;
  let total_height = boxes
    .iter()
    .map(|value| value.ascent_pt + value.descent_pt)
    .sum::<f32>()
    + gap * boxes.len().saturating_sub(1) as f32;
  let axis = metrics.math_font_metrics(style).axis_height_pt;
  let mut top = -axis - total_height / 2.0;
  let mut result = MathBox::empty();
  for row in boxes {
    let baseline = top + row.ascent_pt;
    let row_width = row.width_pt;
    let row_height = row.ascent_pt + row.descent_pt;
    result.append(row, (width - row_width) / 2.0, baseline);
    top += row_height + gap;
  }
  result.width_pt = width;
  result
}

fn layout_matrix(
  rows: &[Vec<MathNode>],
  context: MathLayoutContext,
  metrics: &mut TextMetrics,
) -> MathBox {
  let column_count = rows.iter().map(Vec::len).max().unwrap_or(0);
  if column_count == 0 {
    return MathBox::empty();
  }
  let mut boxes = Vec::with_capacity(rows.len());
  let mut column_widths = vec![0.0_f32; column_count];
  for row in rows {
    let row_boxes = row
      .iter()
      .map(|cell| layout_node(cell, context, metrics))
      .collect::<Vec<_>>();
    for (index, cell) in row_boxes.iter().enumerate() {
      column_widths[index] = column_widths[index].max(cell.width_pt);
    }
    boxes.push(row_boxes);
  }
  let Some(style) = rows.iter().flatten().find_map(representative_style) else {
    return MathBox::empty();
  };
  // SmMatrixNode derives the default column distance from three font heights
  // times LibreOffice's DIS_MATRIXCOL=30%, i.e. 0.9 em.
  let column_gap = style.font_size_pt * 0.9;
  let row_gap = style.font_size_pt * 0.2;
  let row_metrics = boxes
    .iter()
    .map(|row| {
      (
        row
          .iter()
          .map(|cell| cell.ascent_pt)
          .fold(0.0_f32, f32::max),
        row
          .iter()
          .map(|cell| cell.descent_pt)
          .fold(0.0_f32, f32::max),
      )
    })
    .collect::<Vec<_>>();
  let total_width =
    column_widths.iter().sum::<f32>() + column_gap * column_count.saturating_sub(1) as f32;
  let total_height = row_metrics
    .iter()
    .map(|(ascent, descent)| ascent + descent)
    .sum::<f32>()
    + row_gap * rows.len().saturating_sub(1) as f32;
  let axis = metrics.math_font_metrics(style).axis_height_pt;
  let mut top = -axis - total_height / 2.0;
  let mut result = MathBox::empty();
  for (row_index, row) in boxes.into_iter().enumerate() {
    let (row_ascent, row_descent) = row_metrics[row_index];
    let baseline = top + row_ascent;
    let mut x = 0.0;
    for (column_index, cell) in row.into_iter().enumerate() {
      let cell_width = cell.width_pt;
      result.append(
        cell,
        x + (column_widths[column_index] - cell_width) / 2.0,
        baseline,
      );
      x += column_widths[column_index] + column_gap;
    }
    top += row_ascent + row_descent + row_gap;
  }
  result.width_pt = total_width;
  result
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum NaryLimitLayout {
  Side,
  UnderOver,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct NaryLayoutPolicy {
  limits: NaryLimitLayout,
  use_display_operator_minimum: bool,
  size_policy: MathVariantSizePolicy,
}

fn nary_layout_policy(
  operator: &str,
  limit_location: Option<m::LimitLocationValues>,
  document_limit_location: Option<m::LimitLocationValues>,
  grow: bool,
  context: MathLayoutContext,
) -> NaryLayoutPolicy {
  // ECMA-376 Part 1 §22.1.2.53 gives an authored m:limLoc ownership of the
  // object. When it is omitted, intLim/naryLim is a displayed-math document
  // default; the ultimate non-display default is subSup. [MS-OE376]
  // §7.1.2.77(c) supplies the structural boundary and requires inline math to
  // keep its height small. The two fixed-output states in tdf133030.docx and
  // equation.docx confirm that an omitted limLoc stays at the side in a true
  // inline zone even when the document naryLim is undOvr.
  let default_display_location = || {
    document_limit_location.unwrap_or_else(|| {
      if is_integral_nary_operator(operator) {
        m::LimitLocationValues::SubscriptSuperscript
      } else {
        m::LimitLocationValues::UnderOver
      }
    })
  };
  let resolved_location = limit_location.unwrap_or_else(|| {
    if context.display_math {
      default_display_location()
    } else {
      m::LimitLocationValues::SubscriptSuperscript
    }
  });
  let limits = if resolved_location == m::LimitLocationValues::SubscriptSuperscript {
    NaryLimitLayout::Side
  } else {
    NaryLimitLayout::UnderOver
  };
  NaryLayoutPolicy {
    limits,
    // OpenType MATH limits DisplayOperatorMinHeight to display style; limit
    // placement is an independent OMML property. A growing text-style
    // operator must reach its measured operand rather than selecting the
    // closer undersized prepared form. Display style already owns a large
    // minimum and Word fixed output uses its closest prepared grown form.
    use_display_operator_minimum: context.style == MathLayoutStyle::Display,
    size_policy: if grow && context.style == MathLayoutStyle::Display {
      MathVariantSizePolicy::Closest
    } else {
      MathVariantSizePolicy::AtLeast
    },
  }
}

#[allow(clippy::too_many_arguments)]
fn layout_nary(
  operator: &str,
  lower: Option<&MathNode>,
  upper: Option<&MathNode>,
  base: &MathNode,
  limit_location: Option<m::LimitLocationValues>,
  document_limit_location: Option<m::LimitLocationValues>,
  grow: bool,
  fallback_style: &TextStyle,
  control_style: Option<&TextStyle>,
  context: MathLayoutContext,
  metrics: &mut TextMetrics,
) -> MathBox {
  let Some(style) = control_style
    .or_else(|| representative_style(base))
    .or_else(|| lower.and_then(representative_style))
    .or_else(|| upper.and_then(representative_style))
    .or(Some(fallback_style))
  else {
    return MathBox::empty();
  };
  let effective_style = math_script_style(style, context, metrics);
  let math = metrics.math_font_metrics(&effective_style);
  let policy = nary_layout_policy(
    operator,
    limit_location,
    document_limit_location,
    grow,
    context,
  );
  let base_box = layout_node(base, context, metrics);
  let mut minimum_height = effective_style.font_size_pt;
  if policy.use_display_operator_minimum {
    // Word's OfficeMath implementation does not use the OpenType
    // displayOperatorMinHeight value when selecting a display n-ary variant;
    // the Microsoft typography issue for that parameter records that Word
    // uses delimitedSubFormulaMinHeight instead.  This matters for Cambria
    // Math, whose authored values are 3000 and 2500 design units: the former
    // selects Word's 4406-unit ∭/∯ variants while the latter selects the
    // visibly smaller 2769-unit forms.  AxisHeight independently controls
    // centering and is not part of this lower bound.  Use the realized
    // script-sized MATH metrics so a nested n-ary remains in its authored
    // style rather than inheriting the root zone's display minimum.
    minimum_height = minimum_height.max(math.office_display_operator_min_height_pt);
  }
  let target_height = if grow {
    minimum_height.max(base_box.ascent_pt + base_box.descent_pt)
  } else {
    minimum_height
  };
  let mut operator_box = layout_stretched_symbol(
    operator,
    style,
    context,
    target_height,
    minimum_height,
    policy.size_policy,
    metrics,
  );
  apply_control_background(&mut operator_box, &effective_style);
  let lower_box = lower.map(|node| layout_node(node, context.script(), metrics));
  let upper_box = upper.map(|node| layout_node(node, context.script(), metrics));
  let operator_with_limits = match policy.limits {
    NaryLimitLayout::Side => {
      layout_scripts_around_box(operator_box, lower_box, upper_box, false, false, math)
    }
    NaryLimitLayout::UnderOver => {
      layout_limits_around_box(operator_box, lower_box, upper_box, math)
    }
  };
  let gap =
    nary_argument_spacing(base, context).map_or(0.0, |spacing| spacing.em * style.font_size_pt);
  let mut result = MathBox::empty();
  let operator_width = operator_with_limits.width_pt;
  result.append(operator_with_limits, 0.0, 0.0);
  result.append(base_box, operator_width + gap, 0.0);
  result
}

fn layout_radical(
  degree: Option<&MathNode>,
  base: &MathNode,
  control_style: Option<&TextStyle>,
  context: MathLayoutContext,
  metrics: &mut TextMetrics,
) -> MathBox {
  let base_box = layout_node(base, context, metrics);
  let Some(style) = control_style
    .or_else(|| representative_style(base))
    .or_else(|| degree.and_then(representative_style))
  else {
    return MathBox::empty();
  };
  let math = metrics.math_font_metrics(style);
  let radical_gap = if context.style == MathLayoutStyle::Display {
    math.radical_display_style_vertical_gap_pt
  } else {
    math.radical_vertical_gap_pt
  };
  let radicand_height = base_box.ascent_pt + base_box.descent_pt;
  // Microsoft's *Mathematical Typesetting* describes Office's radical rule
  // separately from MathML Core: the math handler measures the expression
  // contained by a radical and selects the MATH variant whose height most
  // closely matches that expression. RadicalVerticalGap and
  // RadicalRuleThickness place the overbar after that glyph choice; adding
  // them to the requested variant height selects an oversized Cambria Math
  // surd. The Word fixed output for math-mso2k7.docx confirms the boundary:
  // its 2147-unit radicand keeps the 1972-unit base radical (GID 958), whereas
  // a 2624-unit request would choose the 2544-unit prepared form (GID 3493).
  let mut radical = layout_stretched_symbol(
    "\u{221a}",
    style,
    context,
    radicand_height,
    0.0,
    MathVariantSizePolicy::Closest,
    metrics,
  );
  apply_control_background(&mut radical, style);
  let degree_box = degree.map(|node| layout_node(node, context.script_script(), metrics));
  let degree_width = degree_box.as_ref().map_or(0.0, |value| value.width_pt);
  let degree_prefix = if degree_box.is_some() {
    (degree_width + math.radical_kern_before_degree_pt + math.radical_kern_after_degree_pt).max(0.0)
  } else {
    0.0
  };
  // The radical glyph advance owns the complete horizontal offset. OpenType
  // exposes horizontal kerns only for an optional degree; MathML Core and
  // Typst's full placement algorithm both start the radicand and its overbar
  // exactly at the surd advance. Word's PDF uses the same coordinate for the
  // radicand and overbar and contains no additional radical/base gap.
  let base_x = degree_prefix + radical.width_pt;
  let bar_y = -(base_box.ascent_pt + radical_gap);
  let base_descent = base_box.descent_pt;
  let mut result = MathBox::empty();
  let radical_descent = radical.descent_pt;
  result.append(radical, degree_prefix, base_descent - radical_descent);
  let base_width = base_box.width_pt;
  result.append(base_box, base_x, 0.0);
  let rule = math.radical_rule_thickness_pt;
  let mut overbar = MathBox::empty();
  overbar.width_pt = base_width;
  overbar.ascent_pt = rule / 2.0;
  overbar.descent_pt = rule / 2.0;
  overbar.items.push(MathPaintItem::Line {
    x1_pt: 0.0,
    y1_pt: 0.0,
    x2_pt: base_width,
    y2_pt: 0.0,
    width_pt: rule,
    color: style.color,
    opacity: style.opacity,
  });
  apply_control_background(&mut overbar, style);
  result.append(overbar, base_x, bar_y);
  if let Some(degree) = degree_box {
    let completed_radical_height = radicand_height + radical_gap + rule;
    let raise = completed_radical_height * math.radical_degree_bottom_raise_percent / 100.0;
    let y = base_descent - raise - degree.descent_pt;
    result.append(degree, 0.0, y);
  }
  result.width_pt = base_x + base_width;
  result.ascent_pt = result
    .ascent_pt
    .max(-bar_y + rule + math.radical_extra_ascender_pt);
  result
}

fn layout_accent(
  base: &MathNode,
  character: &str,
  options: AccentLayoutOptions,
  control_style: Option<&TextStyle>,
  context: MathLayoutContext,
  metrics: &mut TextMetrics,
) -> AccentLayout {
  let base_box = layout_node(base, context, metrics);
  let Some(style) = control_style.or_else(|| representative_style(base)) else {
    return AccentLayout {
      math_box: base_box,
      accent_baseline_y_pt: 0.0,
    };
  };
  let math = metrics.math_font_metrics(style);
  let base_width = base_box.width_pt;
  let base_ascent = base_box.ascent_pt;
  let base_descent = base_box.descent_pt;
  let script_base_ascent = base_box.script_base_ascent_pt();
  let script_base_descent = base_box.script_base_descent_pt();
  let base_italics_correction = base_box.italics_correction_pt;
  let base_italics_correction_in_advance = base_box.italics_correction_in_advance;
  let base_text_like = !options.exact_frame_width && base_box.text_like;
  let base_attachment = if options.bottom {
    base_box.bottom_accent_attachment_pt()
  } else {
    base_box.top_accent_attachment_pt()
  };
  let accent_style = math_script_style(style, context, metrics);
  let mut fallback_features = Vec::with_capacity(1);
  let script_level = context.style.script_level().min(2);
  if script_level > 0 {
    fallback_features.push(FeatureValue {
      tag: "ssty".into(),
      value: u32::from(script_level),
    });
  }
  let flatten = !options.bottom && base_ascent > math.flattened_accent_base_height_pt;
  let mut accent_features = fallback_features.clone();
  if flatten {
    accent_features.push(FeatureValue {
      tag: "flac".into(),
      value: 1,
    });
  }

  let mut normal_accent = layout_text(character, style, context, metrics);
  if !accent_features.is_empty()
    && let Some(feature_shaped) =
      metrics.shape_text_with_features(character, &accent_style, &accent_features)
    && metrics
      .shape_text(character, &accent_style)
      .as_ref()
      .is_none_or(|normal| math_script_glyph_selection_changed(normal, &feature_shaped))
    && let Some(feature_box) = layout_shaped_math_text(character, &accent_style, feature_shaped)
  {
    normal_accent = feature_box;
  }
  let normal_accent_attachment = normal_accent.top_accent_attachment_pt();
  let normal_semantic_placement = normal_accent.semantic_text_placement();

  let mut accent = layout_horizontal_math_variant(
    character,
    &accent_style,
    base_width,
    &accent_features,
    metrics,
  )
  .or_else(|| {
    flatten.then(|| {
      layout_horizontal_math_variant(
        character,
        &accent_style,
        base_width,
        &fallback_features,
        metrics,
      )
    })?
  })
  .unwrap_or(normal_accent);
  apply_control_background(&mut accent, &accent_style);
  let accent_width = accent.width_pt;
  let accent_attachment = accent.top_accent_attachment_pt();
  let (width, base_x, accent_x) = if options.exact_frame_width {
    let pre_width = accent_attachment - base_attachment;
    let post_width = (accent_width - accent_attachment) - (base_width - base_attachment);
    let width = pre_width.max(0.0) + base_width + post_width.max(0.0);
    if pre_width < 0.0 {
      (width, 0.0, -pre_width)
    } else {
      (width, pre_width, 0.0)
    }
  } else {
    (base_width, 0.0, base_attachment - accent_attachment)
  };
  if options.replace_variant_semantics
    && let Some(placement) = normal_semantic_placement
  {
    let logical_semantic_x = base_x + base_attachment - normal_accent_attachment + placement.x_pt;
    accent.replace_variant_semantics_with_combining_accent(
      character,
      placement,
      logical_semantic_x - accent_x,
    );
  }
  // OpenType authors the accent's vertical position in its glyph outline.
  // AccentBaseHeight raises a top accent only by the excess height of a tall
  // base; bottom accents start on the base's descent. Overbar/underbar gaps
  // belong to rule objects and must not be reused here.
  let accent_y = if options.bottom {
    base_descent
  } else {
    base_ascent.min(math.accent_base_height_pt) - base_ascent
  };
  let mut result = MathBox::empty();
  result.append(base_box, base_x, 0.0);
  result.append(accent, accent_x, accent_y);
  result.width_pt = width;
  result.script_base_ascent_pt = Some(script_base_ascent);
  result.script_base_descent_pt = Some(script_base_descent);
  result.top_accent_attachment_pt = Some(base_x + base_attachment);
  result.italics_correction_pt = base_italics_correction;
  result.italics_correction_in_advance = base_italics_correction_in_advance;
  result.text_like = base_text_like;
  AccentLayout {
    math_box: result,
    accent_baseline_y_pt: accent_y,
  }
}

fn group_character_aligns_character(
  bottom: bool,
  vertical_justification: m::VerticalJustificationValues,
) -> bool {
  // ECMA-376 §22.1.2.119 aligns the top or bottom component of the two-row
  // object with the surrounding baseline. Word exposes the same distinction
  // as OMathGroupChar.AlignTop: the grouping-character row is aligned when it
  // occupies the selected edge; otherwise the base-text row is aligned.
  matches!(
    (bottom, vertical_justification),
    (false, m::VerticalJustificationValues::Top) | (true, m::VerticalJustificationValues::Bottom)
  )
}

fn layout_bar(
  base: &MathNode,
  bottom: bool,
  control_style: Option<&TextStyle>,
  context: MathLayoutContext,
  metrics: &mut TextMetrics,
) -> MathBox {
  let mut result = layout_node(base, context, metrics);
  let Some(style) = control_style.or_else(|| representative_style(base)) else {
    return result;
  };
  let math = metrics.math_font_metrics(style);
  let (y, width) = if bottom {
    (
      result.descent_pt + math.underbar_vertical_gap_pt,
      math.underbar_rule_thickness_pt,
    )
  } else {
    (
      -(result.ascent_pt + math.overbar_vertical_gap_pt),
      math.overbar_rule_thickness_pt,
    )
  };
  let mut bar = MathBox::empty();
  bar.width_pt = result.width_pt;
  bar.ascent_pt = width / 2.0;
  bar.descent_pt = width / 2.0;
  bar.items.push(MathPaintItem::Line {
    x1_pt: 0.0,
    y1_pt: 0.0,
    x2_pt: result.width_pt,
    y2_pt: 0.0,
    width_pt: width,
    color: style.color,
    opacity: style.opacity,
  });
  apply_control_background(&mut bar, style);
  result.append(bar, 0.0, y);
  result
}

#[allow(clippy::too_many_arguments)]
fn layout_border_box(
  base: &MathNode,
  hide_top: bool,
  hide_bottom: bool,
  hide_left: bool,
  hide_right: bool,
  strike_horizontal: bool,
  strike_vertical: bool,
  strike_bottom_left_to_top_right: bool,
  strike_top_left_to_bottom_right: bool,
  control_style: Option<&TextStyle>,
  context: MathLayoutContext,
  metrics: &mut TextMetrics,
) -> MathBox {
  let child = layout_node(base, context, metrics);
  let Some(style) = control_style.or_else(|| representative_style(base)) else {
    return child;
  };
  let padding = style.font_size_pt * 0.1;
  let rule = metrics
    .math_font_metrics(style)
    .fraction_rule_thickness_pt
    .max(MIN_RULE_WIDTH_PT);
  let width = child.width_pt + padding * 2.0;
  let top = -(child.ascent_pt + padding);
  let bottom = child.descent_pt + padding;
  let mut result = MathBox::empty();
  result.append(child, padding, 0.0);
  let mut control = MathBox::empty();
  control.width_pt = width;
  control.ascent_pt = -top + rule / 2.0;
  control.descent_pt = bottom + rule / 2.0;
  let mut line = |x1: f32, y1: f32, x2: f32, y2: f32| {
    control.items.push(MathPaintItem::Line {
      x1_pt: x1,
      y1_pt: y1,
      x2_pt: x2,
      y2_pt: y2,
      width_pt: rule,
      color: style.color,
      opacity: style.opacity,
    });
  };
  if !hide_top {
    line(0.0, top, width, top);
  }
  if !hide_bottom {
    line(0.0, bottom, width, bottom);
  }
  if !hide_left {
    line(0.0, top, 0.0, bottom);
  }
  if !hide_right {
    line(width, top, width, bottom);
  }
  if strike_horizontal {
    line(0.0, (top + bottom) / 2.0, width, (top + bottom) / 2.0);
  }
  if strike_vertical {
    line(width / 2.0, top, width / 2.0, bottom);
  }
  if strike_bottom_left_to_top_right {
    line(0.0, bottom, width, top);
  }
  if strike_top_left_to_bottom_right {
    line(0.0, top, width, bottom);
  }
  apply_control_background(&mut control, style);
  result.append(control, 0.0, 0.0);
  result.width_pt = width;
  result
}

fn set_box_opacity(math_box: &mut MathBox, opacity: f32) {
  for item in &mut math_box.items {
    match item {
      MathPaintItem::Background {
        opacity: item_opacity,
        ..
      }
      | MathPaintItem::Text {
        opacity: item_opacity,
        ..
      }
      | MathPaintItem::GlyphPath {
        opacity: item_opacity,
        ..
      }
      | MathPaintItem::Line {
        opacity: item_opacity,
        ..
      } => *item_opacity *= opacity,
      MathPaintItem::SemanticText { .. } => {}
    }
  }
}

fn xml_escape_text(value: &str) -> String {
  value
    .replace('&', "&amp;")
    .replace('<', "&lt;")
    .replace('>', "&gt;")
}

fn xml_escape_attribute(value: &str) -> String {
  xml_escape_text(value)
    .replace('"', "&quot;")
    .replace('\'', "&apos;")
}

#[cfg(test)]
mod tests {
  use std::sync::Arc;

  use ooxmlsdk::schemas::{m, schemas_openxmlformats_org_wordprocessingml_2006_main as w};
  use ooxmlsdk::sdk::SdkType;
  use skrifa::raw::FontData;
  use unicode_math_class::MathClass;

  use crate::docx::{RgbColor, StylesCatalog, TextStyle};
  use crate::text_metrics::{TextMetrics, TextVerticalMetrics};

  use super::{
    MATH_MEDIUM_SPACE_EM, MATH_THICK_SPACE_EM, MATH_THIN_SPACE_EM, MATH_VERY_THICK_SPACE_EM,
    MathAssemblyPart, MathAtomClasses, MathBackgroundCoverage, MathBox, MathGlyphAssembly,
    MathGlyphConstruction, MathGlyphKerns, MathGlyphVariant, MathLayoutContext, MathLayoutStyle,
    MathNode, MathPaintItem, MathParser, MathSemanticTextPlacement, MathSpacing, MathSpacingClass,
    MathSpacingOwner, MathStretchAxis, MathVariantSizePolicy, NaryLayoutPolicy, NaryLimitLayout,
    apply_office_math_font_family, automatic_math_spacing, function_application_spacing,
    group_character_aligns_character, layout_node, layout_user_space, limit_baseline_distance_pt,
    math_argument_size_delta, math_background_coverage, math_character_class,
    math_glyph_construction, math_glyph_kerns, math_is_extended_shape, math_italics_correction,
    math_node_classes, math_node_is_vertical_fraction_object, math_script_style,
    math_top_accent_attachment, nary_argument_spacing, nary_layout_policy,
    normalize_automatic_math_text, office_math_object_surround_spacing,
    open_type_math_line_extents, plan_math_glyph_assembly, prepared_math_variant_for_target,
    representative_style, resolve_vary_math_classes, split_math_text, text_math_classes,
    wordprocessing_math_script_size, wordprocessing_math_zone_image,
  };

  #[test]
  fn office_math_font_binding_replaces_every_word_slot_as_one_state() {
    let original = TextStyle {
      font_family: Some(Arc::from("ASCII text")),
      high_ansi_font_family: Some(Arc::from("High ANSI text")),
      east_asia_font_family: Some(Arc::from("East Asian text")),
      complex_font_family: Some(Arc::from("Complex text")),
      fallback_font_family: Some(Arc::from("ASCII fallback")),
      high_ansi_fallback_font_family: Some(Arc::from("High ANSI fallback")),
      east_asia_fallback_font_family: Some(Arc::from("East Asian fallback")),
      complex_fallback_font_family: Some(Arc::from("Complex fallback")),
      font_family_class: Some(ooxmlsdk_fonts::FontFamilyClass::Serif),
      high_ansi_font_family_class: Some(ooxmlsdk_fonts::FontFamilyClass::SansSerif),
      east_asia_font_family_class: Some(ooxmlsdk_fonts::FontFamilyClass::Schoolbook),
      complex_font_family_class: Some(ooxmlsdk_fonts::FontFamilyClass::OldStyle),
      font_charset: Some(ooxmlsdk_fonts::FontCharset::Ansi),
      high_ansi_font_charset: Some(ooxmlsdk_fonts::FontCharset::Russian),
      east_asia_font_charset: Some(ooxmlsdk_fonts::FontCharset::Gb2312),
      complex_font_charset: Some(ooxmlsdk_fonts::FontCharset::Arabic),
      font_pitch: Some(ooxmlsdk_fonts::FontPitch::Variable),
      high_ansi_font_pitch: Some(ooxmlsdk_fonts::FontPitch::Fixed),
      east_asia_font_pitch: Some(ooxmlsdk_fonts::FontPitch::Variable),
      complex_font_pitch: Some(ooxmlsdk_fonts::FontPitch::Fixed),
      wordprocessingml_font_slots: true,
      ..TextStyle::default()
    };
    let mut math = original.clone();
    apply_office_math_font_family(
      &mut math,
      &StylesCatalog::default(),
      &Arc::from("Cambria Math"),
    );

    for family in [
      &math.font_family,
      &math.high_ansi_font_family,
      &math.east_asia_font_family,
      &math.complex_font_family,
    ] {
      assert_eq!(family.as_deref(), Some("Cambria Math"));
    }
    assert!(math.fallback_font_family.is_none());
    assert!(math.high_ansi_fallback_font_family.is_none());
    assert!(math.east_asia_fallback_font_family.is_none());
    assert!(math.complex_fallback_font_family.is_none());
    assert!(math.font_family_class.is_none());
    assert!(math.high_ansi_font_family_class.is_none());
    assert!(math.east_asia_font_family_class.is_none());
    assert!(math.complex_font_family_class.is_none());
    assert!(math.font_charset.is_none());
    assert!(math.high_ansi_font_charset.is_none());
    assert!(math.east_asia_font_charset.is_none());
    assert!(math.complex_font_charset.is_none());
    assert!(math.font_pitch.is_none());
    assert!(math.high_ansi_font_pitch.is_none());
    assert!(math.east_asia_font_pitch.is_none());
    assert!(math.complex_font_pitch.is_none());
    assert!(math.wordprocessingml_font_slots);

    // Applying the math-zone state to a clone must not mutate the enclosing
    // paragraph style used by adjacent ordinary WordprocessingML text.
    assert_eq!(original.font_family.as_deref(), Some("ASCII text"));
    assert_eq!(
      original.high_ansi_font_family.as_deref(),
      Some("High ANSI text")
    );
    assert_eq!(
      original.east_asia_font_family.as_deref(),
      Some("East Asian text")
    );
    assert_eq!(
      original.complex_font_family.as_deref(),
      Some("Complex text")
    );
  }

  fn write_u16(bytes: &mut [u8], offset: usize, value: u16) {
    bytes[offset..offset + 2].copy_from_slice(&value.to_be_bytes());
  }

  fn push_u16(bytes: &mut Vec<u8>, value: u16) {
    bytes.extend_from_slice(&value.to_be_bytes());
  }

  fn push_i16(bytes: &mut Vec<u8>, value: i16) {
    bytes.extend_from_slice(&value.to_be_bytes());
  }

  fn push_math_construction(
    table: &mut Vec<u8>,
    variant_glyph: u16,
    assembly_glyph: u16,
    italics_correction: i16,
  ) {
    push_u16(table, 12); // assembly follows both variant records
    push_u16(table, 2);
    push_u16(table, variant_glyph);
    push_u16(table, 900);
    push_u16(table, variant_glyph + 1);
    push_u16(table, 1200);

    push_i16(table, italics_correction);
    push_u16(table, 0); // no device/variation correction
    push_u16(table, 3);
    for (glyph_id, start, end, advance, flags) in [
      (assembly_glyph, 0, 80, 400, 0),
      (assembly_glyph + 1, 80, 80, 300, 1),
      (assembly_glyph + 2, 80, 0, 400, 0),
    ] {
      push_u16(table, glyph_id);
      push_u16(table, start);
      push_u16(table, end);
      push_u16(table, advance);
      push_u16(table, flags);
    }
  }

  fn push_coverage(table: &mut Vec<u8>, coverage_format: u16, glyph_id: u16) {
    match coverage_format {
      1 => {
        push_u16(table, 1);
        push_u16(table, 1);
        push_u16(table, glyph_id);
      }
      2 => {
        push_u16(table, 2);
        push_u16(table, 1);
        push_u16(table, glyph_id);
        push_u16(table, glyph_id);
        push_u16(table, 0);
      }
      _ => unreachable!(),
    }
  }

  fn complete_math_variants_table(coverage_format: u16) -> Vec<u8> {
    // MATH v1 header. MathConstants and MathGlyphInfo are not needed for
    // construction lookup; MathVariants immediately follows it. Keep both
    // construction arrays populated so horizontal offsets are proven to start
    // after the complete vertical-offset array.
    let mut table = vec![0; 10];
    write_u16(&mut table, 0, 1);
    write_u16(&mut table, 8, 10);

    let variants_start = table.len();
    push_u16(&mut table, 40); // minConnectorOverlap
    push_u16(&mut table, 0); // vertical coverage; patched after construction
    push_u16(&mut table, 0); // horizontal coverage; patched after construction
    push_u16(&mut table, 1); // one vertical construction
    push_u16(&mut table, 1); // one horizontal construction
    push_u16(&mut table, 14); // vertical construction offset
    push_u16(&mut table, 0); // horizontal construction offset; patched below

    let vertical_construction_start = table.len();
    push_math_construction(&mut table, 100, 200, -20);
    let horizontal_construction_offset = u16::try_from(table.len() - variants_start).unwrap();
    write_u16(
      &mut table,
      variants_start + 12,
      horizontal_construction_offset,
    );
    push_math_construction(&mut table, 110, 210, 30);

    let vertical_coverage_offset = u16::try_from(table.len() - variants_start).unwrap();
    write_u16(&mut table, variants_start + 2, vertical_coverage_offset);
    push_coverage(&mut table, coverage_format, 42);
    let horizontal_coverage_offset = u16::try_from(table.len() - variants_start).unwrap();
    write_u16(&mut table, variants_start + 4, horizontal_coverage_offset);
    push_coverage(&mut table, coverage_format, 43);
    assert_eq!(vertical_construction_start - variants_start, 14);
    table
  }

  fn complete_math_top_accent_table(coverage_format: u16) -> Vec<u8> {
    let mut table = vec![0; 10];
    write_u16(&mut table, 0, 1);
    write_u16(&mut table, 6, 10); // MathGlyphInfo follows the header.

    let glyph_info_start = table.len();
    push_u16(&mut table, 0); // MathItalicsCorrectionInfo
    push_u16(&mut table, 8); // MathTopAccentAttachment
    push_u16(&mut table, 0); // ExtendedShapeCoverage
    push_u16(&mut table, 0); // MathKernInfo
    let top_accent_start = table.len();
    push_u16(&mut table, 8); // coverage follows the one-record array
    push_u16(&mut table, 1);
    push_i16(&mut table, 321);
    push_u16(&mut table, 0); // no device/variation correction
    push_coverage(&mut table, coverage_format, 52);
    assert_eq!(glyph_info_start, 10);
    assert_eq!(top_accent_start - glyph_info_start, 8);
    table
  }

  fn push_math_kern_table(table: &mut Vec<u8>, heights: [i16; 2], values: [i16; 3]) {
    push_u16(table, 2);
    for height in heights {
      push_i16(table, height);
      push_u16(table, 0);
    }
    for value in values {
      push_i16(table, value);
      push_u16(table, 0);
    }
  }

  fn complete_math_glyph_positioning_table(coverage_format: u16) -> Vec<u8> {
    let mut table = vec![0; 10];
    write_u16(&mut table, 0, 1);
    write_u16(&mut table, 6, 10);

    let glyph_info_start = table.len();
    table.resize(table.len() + 8, 0);

    let italics_offset = u16::try_from(table.len() - glyph_info_start).unwrap();
    write_u16(&mut table, glyph_info_start, italics_offset);
    push_u16(&mut table, 8);
    push_u16(&mut table, 1);
    push_i16(&mut table, 321);
    push_u16(&mut table, 0);
    push_coverage(&mut table, coverage_format, 62);

    let extended_shape_offset = u16::try_from(table.len() - glyph_info_start).unwrap();
    write_u16(&mut table, glyph_info_start + 4, extended_shape_offset);
    push_coverage(&mut table, coverage_format, 62);

    let kern_info_offset = u16::try_from(table.len() - glyph_info_start).unwrap();
    write_u16(&mut table, glyph_info_start + 6, kern_info_offset);
    let kern_info_start = table.len();
    table.resize(table.len() + 12, 0);
    write_u16(&mut table, kern_info_start + 2, 1);

    for (field_offset, values) in [
      (4, [10, 20, 30]),
      (6, [40, 50, 60]),
      (8, [-10, -20, -30]),
      (10, [-40, -50, -60]),
    ] {
      let offset = u16::try_from(table.len() - kern_info_start).unwrap();
      write_u16(&mut table, kern_info_start + field_offset, offset);
      push_math_kern_table(&mut table, [-100, 200], values);
    }
    let coverage_offset = u16::try_from(table.len() - kern_info_start).unwrap();
    write_u16(&mut table, kern_info_start, coverage_offset);
    push_coverage(&mut table, coverage_format, 62);
    table
  }

  fn part(
    glyph_id: u16,
    start_connector_units: f32,
    end_connector_units: f32,
    full_advance_units: f32,
    extender: bool,
  ) -> MathAssemblyPart {
    MathAssemblyPart {
      glyph_id,
      start_connector_units,
      end_connector_units,
      full_advance_units,
      extender,
    }
  }

  #[test]
  fn math_assembly_spreads_connectors_before_repeating_extenders() {
    let authored = [
      part(1, 0.0, 200.0, 500.0, false),
      part(2, 200.0, 200.0, 400.0, true),
      part(3, 200.0, 0.0, 500.0, false),
    ];

    let without_extender = plan_math_glyph_assembly(&authored, 100.0, 850.0)
      .expect("two fixed parts can reach the target by reducing connector overlap");
    assert_eq!(without_extender.placements.len(), 2);
    assert_eq!(without_extender.placements[0].part.glyph_id, 1);
    assert_eq!(without_extender.placements[1].part.glyph_id, 3);
    assert!((without_extender.extent_units - 850.0).abs() < f32::EPSILON);

    let with_two_extenders = plan_math_glyph_assembly(&authored, 100.0, 1300.0)
      .expect("the target beyond one extender's range repeats every authored extender");
    assert_eq!(
      with_two_extenders
        .placements
        .iter()
        .map(|placement| placement.part.glyph_id)
        .collect::<Vec<_>>(),
      [1, 2, 2, 3]
    );
    assert!((with_two_extenders.extent_units - 1300.0).abs() < f32::EPSILON);
  }

  #[test]
  fn math_construction_parses_both_axes_coverage_formats_and_full_assembly() {
    let expected_vertical = MathGlyphConstruction {
      min_connector_overlap_units: 40,
      variants: vec![
        MathGlyphVariant {
          glyph_id: 100,
          advance_units: 900,
        },
        MathGlyphVariant {
          glyph_id: 101,
          advance_units: 1200,
        },
      ],
      assembly: Some(MathGlyphAssembly {
        italics_correction_units: -20.0,
        parts: vec![
          part(200, 0.0, 80.0, 400.0, false),
          part(201, 80.0, 80.0, 300.0, true),
          part(202, 80.0, 0.0, 400.0, false),
        ],
      }),
    };
    let expected_horizontal = MathGlyphConstruction {
      min_connector_overlap_units: 40,
      variants: vec![
        MathGlyphVariant {
          glyph_id: 110,
          advance_units: 900,
        },
        MathGlyphVariant {
          glyph_id: 111,
          advance_units: 1200,
        },
      ],
      assembly: Some(MathGlyphAssembly {
        italics_correction_units: 30.0,
        parts: vec![
          part(210, 0.0, 80.0, 400.0, false),
          part(211, 80.0, 80.0, 300.0, true),
          part(212, 80.0, 0.0, 400.0, false),
        ],
      }),
    };

    for coverage_format in [1, 2] {
      let table = complete_math_variants_table(coverage_format);
      assert_eq!(
        math_glyph_construction(FontData::new(&table), 42, MathStretchAxis::Vertical),
        Some(expected_vertical.clone())
      );
      assert_eq!(
        math_glyph_construction(FontData::new(&table), 43, MathStretchAxis::Horizontal),
        Some(expected_horizontal.clone())
      );
      assert_eq!(
        math_glyph_construction(FontData::new(&table), 41, MathStretchAxis::Vertical),
        None
      );
      assert_eq!(
        math_glyph_construction(FontData::new(&table), 42, MathStretchAxis::Horizontal),
        None
      );

      let truncated = &table[..14];
      assert_eq!(
        math_glyph_construction(FontData::new(truncated), 42, MathStretchAxis::Vertical),
        None
      );
    }
  }

  #[test]
  fn math_top_accent_attachment_uses_coverage_order_and_complete_value_records() {
    for coverage_format in [1, 2] {
      let table = complete_math_top_accent_table(coverage_format);
      assert_eq!(
        math_top_accent_attachment(FontData::new(&table), 52),
        Some(321.0)
      );
      assert_eq!(math_top_accent_attachment(FontData::new(&table), 51), None);
      assert_eq!(
        math_top_accent_attachment(FontData::new(&table[..20]), 52),
        None
      );
    }
  }

  #[test]
  fn math_glyph_positioning_parses_italics_extended_shapes_and_all_kern_corners() {
    for coverage_format in [1, 2] {
      let table = complete_math_glyph_positioning_table(coverage_format);
      let data = FontData::new(&table);
      assert_eq!(math_italics_correction(data, 62), Some(321.0));
      assert_eq!(math_italics_correction(data, 61), None);
      assert!(math_is_extended_shape(data, 62));
      assert!(!math_is_extended_shape(data, 61));

      let kerns = math_glyph_kerns(data, 62, 0.25, 2.0)
        .expect("the covered glyph has four complete MathKern tables");
      let expected = MathGlyphKerns {
        top_right: Some(super::MathKernTable {
          correction_heights_pt: vec![-25.0, 50.0],
          kern_values_pt: vec![5.0, 10.0, 15.0],
        }),
        top_left: Some(super::MathKernTable {
          correction_heights_pt: vec![-25.0, 50.0],
          kern_values_pt: vec![20.0, 25.0, 30.0],
        }),
        bottom_right: Some(super::MathKernTable {
          correction_heights_pt: vec![-25.0, 50.0],
          kern_values_pt: vec![-5.0, -10.0, -15.0],
        }),
        bottom_left: Some(super::MathKernTable {
          correction_heights_pt: vec![-25.0, 50.0],
          kern_values_pt: vec![-20.0, -25.0, -30.0],
        }),
      };
      assert_eq!(kerns, expected);
      assert_eq!(kerns.top_right.as_ref().unwrap().at_height(-30.0), 5.0);
      assert_eq!(kerns.top_right.as_ref().unwrap().at_height(-25.0), 10.0);
      assert_eq!(kerns.top_right.as_ref().unwrap().at_height(50.0), 15.0);
      assert_eq!(math_glyph_kerns(data, 61, 0.25, 2.0), None);
      assert_eq!(
        math_glyph_kerns(FontData::new(&table[..20]), 62, 0.25, 2.0),
        None
      );
    }
  }

  #[test]
  fn math_italics_advance_preserves_top_and_bottom_attachment_contracts() {
    let mut extended_shape = MathBox::empty();
    extended_shape.width_pt = 10.0;
    extended_shape.italics_correction_pt = 2.0;

    let mut ordinary_glyph = MathBox::empty();
    ordinary_glyph.width_pt = 12.0;
    ordinary_glyph.italics_correction_pt = 2.0;
    ordinary_glyph.italics_correction_in_advance = true;

    assert_eq!(extended_shape.top_accent_attachment_pt(), 6.0);
    assert_eq!(extended_shape.bottom_accent_attachment_pt(), 4.0);
    assert_eq!(ordinary_glyph.top_accent_attachment_pt(), 6.0);
    assert_eq!(ordinary_glyph.bottom_accent_attachment_pt(), 4.0);
  }

  #[test]
  fn accent_variant_semantics_keep_one_source_mark_at_normal_cmap_attachment() {
    let normal_style = TextStyle {
      font_family: Some(Arc::from("Cambria Math")),
      font_size_pt: 12.0,
      ..TextStyle::default()
    };
    let variant_style = TextStyle {
      font_size_pt: 18.0,
      ..normal_style.clone()
    };
    let mut accent = MathBox::empty();
    accent.items = vec![
      MathPaintItem::GlyphPath {
        path_data: Arc::from("M0 0L1 1"),
        style: variant_style.clone(),
        x_pt: 2.0,
        baseline_y_pt: -1.0,
        scale_pt_per_unit: 0.01,
        horizontal_scale: 1.0,
        synthetic_bold: false,
        synthetic_italic: false,
        opacity: 1.0,
      },
      MathPaintItem::SemanticText {
        text: "\u{030c}".into(),
        style: variant_style.clone(),
        x_pt: 2.0,
        baseline_y_pt: -1.0,
        horizontal_scale: 1.8,
        glyph_id: Some(3542),
      },
      MathPaintItem::GlyphPath {
        path_data: Arc::from("M1 1L2 0"),
        style: variant_style.clone(),
        x_pt: 7.0,
        baseline_y_pt: -1.0,
        scale_pt_per_unit: 0.01,
        horizontal_scale: 1.0,
        synthetic_bold: false,
        synthetic_italic: false,
        opacity: 1.0,
      },
      MathPaintItem::SemanticText {
        text: " ".into(),
        style: variant_style,
        x_pt: 7.0,
        baseline_y_pt: -1.0,
        horizontal_scale: 2.0,
        glyph_id: Some(3543),
      },
    ];

    accent.replace_variant_semantics_with_combining_accent(
      "\u{030c}",
      MathSemanticTextPlacement {
        style: normal_style.clone(),
        x_pt: 0.25,
        baseline_y_pt: 0.5,
        horizontal_scale: 0.9,
      },
      5.25,
    );

    assert_eq!(
      accent
        .items
        .iter()
        .filter(|item| matches!(item, MathPaintItem::GlyphPath { .. }))
        .count(),
      2
    );
    let semantics = accent
      .items
      .iter()
      .filter_map(|item| match item {
        MathPaintItem::SemanticText {
          text,
          style,
          x_pt,
          baseline_y_pt,
          horizontal_scale,
          glyph_id,
        } => Some((text, style, x_pt, baseline_y_pt, horizontal_scale, glyph_id)),
        _ => None,
      })
      .collect::<Vec<_>>();
    assert_eq!(semantics.len(), 1);
    assert_eq!(semantics[0].0, "\u{030c}");
    assert_eq!(semantics[0].1, &normal_style);
    assert_eq!(*semantics[0].2, 5.25);
    assert_eq!(*semantics[0].3, 0.5);
    assert_eq!(*semantics[0].4, 0.9);
    assert_eq!(*semantics[0].5, None);
  }

  #[test]
  fn exact_math_glyph_semantics_survive_svg_transport_with_source_unicode() {
    let mut math_box = MathBox::empty();
    math_box.width_pt = 8.0;
    math_box.ascent_pt = 6.0;
    math_box.descent_pt = 2.0;
    math_box.items.push(MathPaintItem::Text {
      text: "x".into(),
      style: TextStyle {
        font_family: Some(Arc::from("Cambria Math")),
        font_size_pt: 12.0,
        ..TextStyle::default()
      },
      x_pt: 0.0,
      baseline_y_pt: 0.0,
      horizontal_scale: 1.0,
      opacity: 1.0,
    });
    math_box.items.push(MathPaintItem::SemanticText {
      text: ")".into(),
      style: TextStyle {
        font_family: Some(Arc::from("Cambria Math")),
        font_size_pt: 12.0,
        ..TextStyle::default()
      },
      x_pt: 1.5,
      baseline_y_pt: 0.25,
      horizontal_scale: 1.0,
      glyph_id: Some(3542),
    });

    let svg = math_box.to_svg();
    assert!(svg.contains("id=\"ooxmlsdk-math-visible-0\" visibility=\"hidden\""));
    assert!(svg.contains("id=\"ooxmlsdk-math-semantic-1-gid-3542\" visibility=\"hidden\""));
    assert!(svg.contains(">)</text>"));
    assert_eq!(svg.matches("ooxmlsdk-math-visible-").count(), 1);
    assert_eq!(svg.matches("ooxmlsdk-math-semantic-").count(), 1);
  }

  #[test]
  fn group_character_carries_ecma_position_and_baseline_justification() {
    let base_style = TextStyle::default();
    let styles = StylesCatalog::default();
    let parser = MathParser {
      base_style: &base_style,
      styles: &styles,
      math_font_family: Arc::from("Cambria Math"),
    };
    let explicit = m::GroupChar {
      group_char_properties: Some(Box::new(m::GroupCharProperties {
        position: Some(m::Position {
          val: m::VerticalJustificationValues::Top,
        }),
        vertical_justification: Some(m::VerticalJustification {
          val: m::VerticalJustificationValues::Bottom,
        }),
        ..m::GroupCharProperties::default()
      })),
      ..m::GroupChar::default()
    };
    assert!(matches!(
      parser.group_char(&explicit),
      MathNode::GroupChar {
        bottom: false,
        vertical_justification: m::VerticalJustificationValues::Bottom,
        ..
      }
    ));
    assert!(matches!(
      parser.group_char(&m::GroupChar::default()),
      MathNode::GroupChar {
        bottom: true,
        vertical_justification: m::VerticalJustificationValues::Top,
        ..
      }
    ));

    for (bottom, justification, aligns_character) in [
      (false, m::VerticalJustificationValues::Top, true),
      (false, m::VerticalJustificationValues::Bottom, false),
      (true, m::VerticalJustificationValues::Top, false),
      (true, m::VerticalJustificationValues::Bottom, true),
    ] {
      assert_eq!(
        group_character_aligns_character(bottom, justification),
        aligns_character
      );
    }

    let natural = MathBox {
      width_pt: 7.0,
      ascent_pt: 2.0,
      descent_pt: 3.0,
      script_base_ascent_pt: None,
      script_base_descent_pt: None,
      top_accent_attachment_pt: Some(4.0),
      italics_correction_pt: 0.0,
      italics_correction_in_advance: false,
      text_like: false,
      math_kerns: MathGlyphKerns::default(),
      items: Vec::new(),
    };
    let top_character = natural.clone().rebase_to_baseline(-1.0);
    assert_eq!(
      (top_character.ascent_pt, top_character.descent_pt),
      (1.0, 4.0)
    );
    let bottom_character = natural.rebase_to_baseline(2.0);
    assert_eq!(
      (bottom_character.ascent_pt, bottom_character.descent_pt),
      (4.0, 1.0)
    );
  }

  #[test]
  fn limit_baselines_satisfy_both_open_type_math_constraints() {
    assert_eq!(limit_baseline_distance_pt(6.0, 1.0, 2.0, 4.0), 10.0);
    assert_eq!(limit_baseline_distance_pt(6.0, 3.0, 2.0, 4.0), 11.0);
  }

  #[test]
  fn office_delimiters_choose_the_closest_prepared_math_variant() {
    let variants = [
      MathGlyphVariant {
        glyph_id: 4666,
        advance_units: 1898,
      },
      MathGlyphVariant {
        glyph_id: 3435,
        advance_units: 2475,
      },
      MathGlyphVariant {
        glyph_id: 4672,
        advance_units: 3379,
      },
      MathGlyphVariant {
        glyph_id: 3436,
        advance_units: 4047,
      },
      MathGlyphVariant {
        glyph_id: 4678,
        advance_units: 5223,
      },
    ];

    assert_eq!(
      prepared_math_variant_for_target(&variants, 2048.0, 0.0, MathVariantSizePolicy::Closest),
      Some(variants[0])
    );
    assert_eq!(
      prepared_math_variant_for_target(&variants, 2048.0, 2048.0, MathVariantSizePolicy::AtLeast,),
      Some(variants[1])
    );
    assert_eq!(
      prepared_math_variant_for_target(&variants, 4329.0, 0.0, MathVariantSizePolicy::Closest),
      Some(variants[3])
    );
    assert_eq!(
      prepared_math_variant_for_target(&variants, 4329.0, 4329.0, MathVariantSizePolicy::AtLeast,),
      Some(variants[4])
    );
    assert_eq!(
      prepared_math_variant_for_target(&variants, 5300.0, 0.0, MathVariantSizePolicy::Closest),
      None
    );
  }

  #[test]
  fn office_radicals_match_the_radicand_before_placement_constants() {
    // Cambria Math's authored radical construction starts with the 1972-unit
    // base glyph and a 2544-unit second form. Microsoft documents that Office
    // matches a radical to the height of the expression it contains. The
    // realized radicand in math-mso2k7.docx is about 2147 units, so Word keeps
    // GID 958. Adding RadicalDisplayStyleVerticalGap and
    // RadicalRuleThickness would inflate the request to about 2624 units and
    // incorrectly select GID 3493; retain that opposite-state counterexample.
    let variants = [
      MathGlyphVariant {
        glyph_id: 958,
        advance_units: 1972,
      },
      MathGlyphVariant {
        glyph_id: 3493,
        advance_units: 2544,
      },
      MathGlyphVariant {
        glyph_id: 3495,
        advance_units: 4569,
      },
      MathGlyphVariant {
        glyph_id: 3496,
        advance_units: 6829,
      },
      MathGlyphVariant {
        glyph_id: 3497,
        advance_units: 9129,
      },
      MathGlyphVariant {
        glyph_id: 3498,
        advance_units: 11429,
      },
    ];

    assert_eq!(
      prepared_math_variant_for_target(&variants, 2147.0, 0.0, MathVariantSizePolicy::Closest),
      Some(variants[0])
    );
    assert_eq!(
      prepared_math_variant_for_target(&variants, 2624.0, 0.0, MathVariantSizePolicy::Closest),
      Some(variants[1])
    );
    assert_eq!(
      prepared_math_variant_for_target(&variants, 2624.0, 0.0, MathVariantSizePolicy::AtLeast),
      Some(variants[2])
    );
    assert_eq!(
      prepared_math_variant_for_target(&variants, 12000.0, 0.0, MathVariantSizePolicy::Closest),
      None
    );
  }

  #[test]
  fn delimiter_parser_retains_the_standard_growth_and_shape_properties() {
    let base_style = TextStyle::default();
    let styles = StylesCatalog::default();
    let parser = MathParser {
      base_style: &base_style,
      styles: &styles,
      math_font_family: Arc::from("Cambria Math"),
    };
    assert!(matches!(
      parser.delimiter(&m::Delimiter::default()),
      MathNode::Delimiter {
        grow: true,
        shape: m::ShapeDelimiterValues::Centered,
        ..
      }
    ));

    let matched = m::Delimiter {
      delimiter_properties: Some(Box::new(m::DelimiterProperties {
        shape: Some(m::Shape {
          val: m::ShapeDelimiterValues::Match,
        }),
        ..m::DelimiterProperties::default()
      })),
      ..m::Delimiter::default()
    };
    assert!(matches!(
      parser.delimiter(&matched),
      MathNode::Delimiter {
        grow: true,
        shape: m::ShapeDelimiterValues::Match,
        ..
      }
    ));

    let fixed = m::Delimiter {
      delimiter_properties: Some(Box::new(m::DelimiterProperties {
        grow_operators: Some(m::GrowOperators {
          val: Some(m::BooleanValues::False),
        }),
        shape: Some(m::Shape {
          val: m::ShapeDelimiterValues::Match,
        }),
        ..m::DelimiterProperties::default()
      })),
      ..m::Delimiter::default()
    };
    assert!(matches!(
      parser.delimiter(&fixed),
      MathNode::Delimiter {
        grow: false,
        shape: m::ShapeDelimiterValues::Match,
        ..
      }
    ));
  }

  #[test]
  fn office_math_run_and_control_backgrounds_preserve_ownership_and_precedence() {
    let base_style = TextStyle::default();
    let styles = StylesCatalog::default();
    let parser = MathParser {
      base_style: &base_style,
      styles: &styles,
      math_font_family: Arc::from("Cambria Math"),
    };
    let math_run = |text: &str, properties: w::RunProperties| m::Run {
      run_properties: Some(Box::new(properties)),
      run_choice: vec![m::RunChoice::MText(m::Text {
        xml_content: Some(text.to_string()),
        ..m::Text::default()
      })],
      ..m::Run::default()
    };
    let percentage_shading = || {
      w::RunPropertiesChoice::Shading(Box::new(w::Shading {
        val: Some(w::ShadingPatternValues::Percent15),
        color: Some("auto".to_string()),
        fill: Some("FFFFFF".to_string()),
        ..w::Shading::default()
      }))
    };
    let yellow_highlight = || {
      w::RunPropertiesChoice::Highlight(w::Highlight {
        val: w::HighlightColorValues::Yellow,
      })
    };
    let run_properties = |choices| w::RunProperties {
      run_properties_choice: choices,
      ..w::RunProperties::default()
    };
    let gray = RgbColor {
      r: 0xd9,
      g: 0xd9,
      b: 0xd9,
    };
    let yellow = RgbColor {
      r: 0xff,
      g: 0xff,
      b: 0x00,
    };

    let shaded = parser.run(&math_run("x+y", run_properties(vec![percentage_shading()])));
    let MathNode::RunBackground {
      style: shaded_style,
      ..
    } = &shaded
    else {
      panic!("percentage run shading must retain the complete authored run boundary");
    };
    assert_eq!(shaded_style.highlight, Some(gray));
    assert_eq!(
      math_background_coverage(&shaded),
      MathBackgroundCoverage::Uniform(gray)
    );
    let mut metrics = TextMetrics::new();
    let shaded_box = layout_node(
      &shaded,
      MathLayoutContext::root(false, false, false),
      &mut metrics,
    );
    assert_eq!(
      shaded_box
        .items
        .iter()
        .filter(|item| matches!(item, MathPaintItem::Background { color, .. } if *color == gray))
        .count(),
      1,
      "atom splitting must not fragment one m:r background"
    );
    let shaded_svg = shaded_box.to_svg();
    let background_index = shaded_svg.find("<rect").expect("shading rectangle");
    let foreground_index = shaded_svg
      .find("<text")
      .or_else(|| shaded_svg.find("<path"))
      .expect("math foreground");
    assert!(background_index < foreground_index);

    let highlighted = parser.run(&math_run(
      "x",
      run_properties(vec![percentage_shading(), yellow_highlight()]),
    ));
    let MathNode::RunBackground {
      style: highlighted_style,
      ..
    } = highlighted
    else {
      panic!("highlighted run must retain its paint boundary");
    };
    assert_eq!(
      highlighted_style.highlight,
      Some(yellow),
      "ECMA-376 §17.3.2.15 makes highlight supersede run shading"
    );

    let control = m::ControlProperties {
      control_properties_choice: Some(m::ControlPropertiesChoice::RunProperties(Box::new(
        run_properties(vec![yellow_highlight()]),
      ))),
    };
    let delimiter = parser.delimiter(&m::Delimiter {
      delimiter_properties: Some(Box::new(m::DelimiterProperties {
        control_properties: Some(Box::new(control)),
        ..m::DelimiterProperties::default()
      })),
      base: vec![m::Base {
        base_choice: vec![m::BaseChoice::Run(Box::new(math_run(
          "x",
          w::RunProperties::default(),
        )))],
        ..m::Base::default()
      }],
    });
    let MathNode::Delimiter {
      arguments,
      control_style: Some(control_style),
      ..
    } = &delimiter
    else {
      panic!("explicit delimiter ctrlPr must survive parsing");
    };
    assert_eq!(control_style.highlight, Some(yellow));
    assert_eq!(
      math_background_coverage(&delimiter),
      MathBackgroundCoverage::Mixed,
      "a highlighted ctrlPr and an unpainted argument must not fill the complete object"
    );
    assert_eq!(
      arguments
        .first()
        .and_then(representative_style)
        .and_then(|style| style.highlight),
      None,
      "ctrlPr must not leak into the delimiter argument"
    );
    let mut metrics = TextMetrics::new();
    let delimiter_box = layout_node(
      &delimiter,
      MathLayoutContext::root(false, false, false),
      &mut metrics,
    );
    assert_eq!(
      delimiter_box
        .items
        .iter()
        .filter(|item| matches!(item, MathPaintItem::Background { color, .. } if *color == yellow))
        .count(),
      2,
      "the opening and closing controls own their backgrounds independently of the base"
    );

    let fully_painted = MathNode::Delimiter {
      begin: "(".to_string(),
      separator: "|".to_string(),
      end: ")".to_string(),
      grow: true,
      shape: m::ShapeDelimiterValues::Centered,
      arguments: vec![MathNode::RunBackground {
        base: Box::new(MathNode::Text {
          text: "x".to_string(),
          style: control_style.clone(),
          math_class_override: None,
        }),
        style: control_style.clone(),
      }],
      control_style: Some(control_style.clone()),
    };
    assert_eq!(
      math_background_coverage(&fully_painted),
      MathBackgroundCoverage::Uniform(yellow)
    );
    let mut metrics = TextMetrics::new();
    let mut fully_painted_box = layout_node(
      &fully_painted,
      MathLayoutContext::root(false, false, false),
      &mut metrics,
    );
    fully_painted_box.replace_backgrounds_with_union(yellow);
    fully_painted_box.expand_to_background_bounds();
    let backgrounds = fully_painted_box
      .items
      .iter()
      .filter(|item| matches!(item, MathPaintItem::Background { .. }))
      .collect::<Vec<_>>();
    assert_eq!(backgrounds.len(), 1);
    assert!(matches!(
      backgrounds[0],
      MathPaintItem::Background {
        x_pt,
        y_pt,
        width_pt,
        height_pt,
        color,
        ..
      } if x_pt.abs() <= f32::EPSILON
        && (*width_pt - fully_painted_box.width_pt).abs() <= f32::EPSILON
        && *y_pt >= -fully_painted_box.ascent_pt - f32::EPSILON
        && *y_pt + *height_pt <= fully_painted_box.descent_pt + f32::EPSILON
        && *height_pt > 0.0
        && *color == yellow
    ));

    let plain = || MathNode::Text {
      text: "x".to_string(),
      style: base_style.clone(),
      math_class_override: None,
    };
    let fraction = MathNode::Fraction {
      numerator: Box::new(plain()),
      denominator: Box::new(plain()),
      kind: m::FractionTypeValues::Bar,
      control_style: Some(control_style.clone()),
    };
    let nary = MathNode::Nary {
      operator: "\u{2211}".to_string(),
      lower: None,
      upper: None,
      base: Box::new(plain()),
      limit_location: None,
      document_limit_location: None,
      grow: false,
      style: Box::new(base_style.clone()),
      control_style: Some(control_style.clone()),
    };
    for control_node in [&fraction, &nary] {
      let mut metrics = TextMetrics::new();
      let control_box = layout_node(
        control_node,
        MathLayoutContext::root(false, false, false),
        &mut metrics,
      );
      assert_eq!(
        control_box
          .items
          .iter()
          .filter(
            |item| matches!(item, MathPaintItem::Background { color, .. } if *color == yellow)
          )
          .count(),
        1,
        "a fraction bar or n-ary operator owns one control background without painting its arguments"
      );
    }
  }

  #[test]
  fn nary_limit_cascade_drives_placement_and_operator_size_together() {
    let base_style = TextStyle::default();
    let styles = StylesCatalog::default();
    let parser = MathParser {
      base_style: &base_style,
      styles: &styles,
      math_font_family: Arc::from("Cambria Math"),
    };
    let nary = |operator: &str| m::Nary {
      nary_properties: Some(Box::new(m::NaryProperties {
        accent_char: Some(m::AccentChar {
          val: operator.to_string(),
        }),
        ..m::NaryProperties::default()
      })),
      ..m::Nary::default()
    };
    let inline = MathLayoutContext::root(false, false, false);
    let display = MathLayoutContext::root(true, false, true);
    let parsed_policy = |parser: &MathParser<'_>, value: &m::Nary, context| {
      let MathNode::Nary {
        operator,
        limit_location,
        document_limit_location,
        grow,
        ..
      } = parser.nary(value)
      else {
        panic!("n-ary parser must retain its layout properties");
      };
      (
        nary_layout_policy(
          &operator,
          limit_location,
          document_limit_location,
          grow,
          context,
        ),
        grow,
      )
    };

    // An omitted limLoc stays on the side in true inline math, independently
    // of the operator family and document display defaults.
    assert_eq!(
      parsed_policy(&parser, &nary("∰"), inline),
      (
        NaryLayoutPolicy {
          limits: NaryLimitLayout::Side,
          use_display_operator_minimum: false,
          size_policy: MathVariantSizePolicy::AtLeast,
        },
        false,
      )
    );
    assert_eq!(
      parsed_policy(&parser, &nary("∑"), inline),
      (
        NaryLayoutPolicy {
          limits: NaryLimitLayout::Side,
          use_display_operator_minimum: false,
          size_policy: MathVariantSizePolicy::AtLeast,
        },
        false,
      )
    );

    // In display math, the integral and other-n-ary ultimate defaults are
    // opposite, but both operator families receive the display minimum.
    assert_eq!(
      parsed_policy(&parser, &nary("∰"), display).0,
      NaryLayoutPolicy {
        limits: NaryLimitLayout::Side,
        use_display_operator_minimum: true,
        size_policy: MathVariantSizePolicy::AtLeast,
      }
    );
    assert_eq!(
      parsed_policy(&parser, &nary("∑"), display).0,
      NaryLayoutPolicy {
        limits: NaryLimitLayout::UnderOver,
        use_display_operator_minimum: true,
        size_policy: MathVariantSizePolicy::AtLeast,
      }
    );

    let explicit = m::Nary {
      nary_properties: Some(Box::new(m::NaryProperties {
        accent_char: Some(m::AccentChar {
          val: "∰".to_string(),
        }),
        limit_location: Some(m::LimitLocation {
          val: m::LimitLocationValues::UnderOver,
        }),
        grow_operators: Some(m::GrowOperators {
          val: Some(m::BooleanValues::One),
        }),
        ..m::NaryProperties::default()
      })),
      ..m::Nary::default()
    };
    assert_eq!(
      parsed_policy(&parser, &explicit, inline),
      (
        NaryLayoutPolicy {
          limits: NaryLimitLayout::UnderOver,
          use_display_operator_minimum: false,
          size_policy: MathVariantSizePolicy::AtLeast,
        },
        true,
      )
    );

    let growing_sum = m::Nary {
      nary_properties: Some(Box::new(m::NaryProperties {
        accent_char: Some(m::AccentChar {
          val: "∑".to_string(),
        }),
        grow_operators: Some(m::GrowOperators {
          val: Some(m::BooleanValues::One),
        }),
        ..m::NaryProperties::default()
      })),
      ..m::Nary::default()
    };
    assert_eq!(
      parsed_policy(&parser, &growing_sum, inline).0,
      NaryLayoutPolicy {
        limits: NaryLimitLayout::Side,
        use_display_operator_minimum: false,
        size_policy: MathVariantSizePolicy::AtLeast,
      }
    );
    assert_eq!(
      parsed_policy(&parser, &growing_sum, display).0,
      NaryLayoutPolicy {
        limits: NaryLimitLayout::UnderOver,
        use_display_operator_minimum: true,
        size_policy: MathVariantSizePolicy::Closest,
      }
    );

    // Object limLoc owns the final choice. Paired document defaults feed only
    // display zones and cannot turn an omitted inline limLoc into under/over.
    let document_styles = StylesCatalog {
      integral_limit_location: Some(m::LimitLocationValues::UnderOver),
      nary_limit_location: Some(m::LimitLocationValues::SubscriptSuperscript),
      ..StylesCatalog::default()
    };
    let document_parser = MathParser {
      base_style: &base_style,
      styles: &document_styles,
      math_font_family: Arc::from("Cambria Math"),
    };
    assert_eq!(
      parsed_policy(&document_parser, &nary("∰"), inline).0,
      NaryLayoutPolicy {
        limits: NaryLimitLayout::Side,
        use_display_operator_minimum: false,
        size_policy: MathVariantSizePolicy::AtLeast,
      }
    );
    assert_eq!(
      parsed_policy(&document_parser, &nary("∑"), display).0,
      NaryLayoutPolicy {
        limits: NaryLimitLayout::Side,
        use_display_operator_minimum: true,
        size_policy: MathVariantSizePolicy::AtLeast,
      }
    );
    assert_eq!(
      parsed_policy(&document_parser, &nary("∰"), display).0,
      NaryLayoutPolicy {
        limits: NaryLimitLayout::UnderOver,
        use_display_operator_minimum: true,
        size_policy: MathVariantSizePolicy::AtLeast,
      }
    );
  }

  #[test]
  fn grown_nary_matches_operand_without_crossing_display_minimum() {
    // Cambria Math's summation construction supplies these authored vertical
    // advances. At 11 pt, the 4046-unit operand target in math-mso2k7.docx is
    // just above the 3911-unit form. Word uses that closer form (GID 3533),
    // while GID 3534 at 5911 units is unnecessarily oversized. The 2500-unit
    // DisplayOperatorMinHeight still excludes the smaller 2312-unit form.
    let variants = [
      MathGlyphVariant {
        glyph_id: 3532,
        advance_units: 2312,
      },
      MathGlyphVariant {
        glyph_id: 3533,
        advance_units: 3911,
      },
      MathGlyphVariant {
        glyph_id: 3534,
        advance_units: 5911,
      },
    ];

    assert_eq!(
      prepared_math_variant_for_target(&variants, 4046.0, 2500.0, MathVariantSizePolicy::Closest,),
      Some(variants[1])
    );
    assert_eq!(
      prepared_math_variant_for_target(&variants, 2500.0, 2500.0, MathVariantSizePolicy::AtLeast,),
      Some(variants[1])
    );
    assert_eq!(
      prepared_math_variant_for_target(&variants, 7000.0, 2500.0, MathVariantSizePolicy::Closest,),
      None
    );
  }

  #[test]
  fn open_type_math_line_extents_keep_font_line_box_until_formula_ink_crosses_it() {
    let vertical = TextVerticalMetrics {
      ascent_pt: 9.0,
      descent_pt: 3.0,
      line_gap_pt: 2.0,
      baseline_offset_pt: 9.0,
      directwrite_baseline_offset_pt: 11.0,
      wordprocessingml_cjk_line_metrics: false,
    };

    assert_eq!(
      open_type_math_line_extents(9.5, 2.0, vertical, 1.0),
      (11.0, 3.0)
    );
    assert_eq!(
      open_type_math_line_extents(12.0, 5.0, vertical, 1.0),
      (13.0, 5.0)
    );
  }

  #[test]
  fn office_math_script_sizes_follow_math_percentages_on_the_half_point_grid() {
    assert_eq!(wordprocessing_math_script_size(12.0, 0.73), 8.5);
    assert_eq!(wordprocessing_math_script_size(11.0, 0.73), 8.0);
    assert_eq!(wordprocessing_math_script_size(12.0, 0.60), 7.0);
    assert_eq!(wordprocessing_math_script_size(11.0, 0.60), 6.5);

    let source = TextStyle {
      cjk_punctuation_compression_ratio: 1.0,
      ..TextStyle::default()
    };
    let mut metrics = TextMetrics::new();
    let math = math_script_style(
      &source,
      MathLayoutContext::root(false, false, false),
      &mut metrics,
    );
    assert_eq!(source.cjk_punctuation_compression_ratio, 1.0);
    assert_eq!(math.cjk_punctuation_compression_ratio, 0.0);
  }

  #[test]
  fn office_math_automatic_text_uses_unicode_while_literal_text_preserves_source_scalars() {
    assert_eq!(math_character_class('/'), MathClass::Normal);
    assert_eq!(math_character_class('\u{2044}'), MathClass::Normal);
    assert_eq!(math_character_class('\u{ff0f}'), MathClass::Normal);
    assert_eq!(math_character_class('\u{2215}'), MathClass::Binary);

    assert_eq!(
      normalize_automatic_math_text("a-b\u{2329}c\u{232a}".to_string()),
      "a\u{2212}b\u{27e8}c\u{27e9}"
    );
    assert_eq!(normalize_automatic_math_text("a+b".to_string()), "a+b");

    let literal = split_math_text(
      "a-b\u{2329}c\u{232a}".to_string(),
      TextStyle::default(),
      false,
    );
    assert_eq!(literal.semantic_text(), "a-b\u{2329}c\u{232a}");

    assert_eq!(
      split_math_text("+\u{2026}".to_string(), TextStyle::default(), true).semantic_text(),
      "+\u{2026}"
    );
    assert_eq!(
      split_math_text("x+\u{2026}".to_string(), TextStyle::default(), true).semantic_text(),
      "x+\u{22ef}"
    );
    assert_eq!(
      split_math_text("x\u{2026}".to_string(), TextStyle::default(), true).semantic_text(),
      "x\u{2026}"
    );
    assert_eq!(
      split_math_text("+\u{2026}".to_string(), TextStyle::default(), false).semantic_text(),
      "+\u{2026}"
    );

    let automatic = |text: &str| MathNode::Text {
      text: text.to_string(),
      style: TextStyle::default(),
      math_class_override: None,
    };
    assert_eq!(
      MathNode::row([automatic("+"), automatic("\u{2026}")]).semantic_text(),
      "+\u{2026}"
    );
    assert_eq!(
      MathNode::row([automatic("x"), automatic("+"), automatic("\u{2026}")]).semantic_text(),
      "x+\u{22ef}"
    );
    let fraction = || MathNode::Fraction {
      numerator: Box::new(automatic("x")),
      denominator: Box::new(automatic("1")),
      kind: m::FractionTypeValues::Bar,
      control_style: None,
    };
    assert_eq!(
      MathNode::row([fraction(), automatic("+"), automatic("\u{2026}")]).semantic_text(),
      "x1+\u{22ef}"
    );
    assert_eq!(
      text_math_classes("\u{22ef}", None),
      MathAtomClasses {
        class: MathClass::Normal,
        left: MathSpacingClass::Ellipsis,
        right: MathSpacingClass::Ellipsis,
      }
    );
    assert_eq!(
      text_math_classes("\u{22ef}", Some(MathClass::Normal)),
      MathAtomClasses::single(MathClass::Normal)
    );
    assert_eq!(
      MathNode::row([
        automatic("x"),
        automatic("+"),
        automatic(" "),
        automatic("\u{2026}")
      ])
      .semantic_text(),
      "x+ \u{22ef}"
    );

    let highlighted = |base| MathNode::RunBackground {
      base: Box::new(base),
      style: TextStyle::default(),
    };
    assert_eq!(
      MathNode::row([highlighted(split_math_text(
        "+\u{2026}".to_string(),
        TextStyle::default(),
        true,
      ))])
      .semantic_text(),
      "+\u{2026}"
    );
    assert_eq!(
      MathNode::row([
        automatic("x"),
        highlighted(split_math_text(
          "+\u{2026}".to_string(),
          TextStyle::default(),
          true,
        )),
      ])
      .semantic_text(),
      "x+\u{22ef}"
    );
  }

  #[test]
  fn office_math_zone_normalizes_ellipsis_after_a_built_up_object() {
    let xml = r#"<w:p xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:m="http://schemas.openxmlformats.org/officeDocument/2006/math"><m:oMathPara><m:oMath><m:f><m:num><m:r><m:t>x</m:t></m:r></m:num><m:den><m:r><m:t>1</m:t></m:r></m:den></m:f><m:r><m:t>+…</m:t></m:r></m:oMath></m:oMathPara></w:p>"#;
    let paragraph = w::Paragraph::from_bytes(xml.as_bytes()).unwrap();
    let choice = paragraph.paragraph_choice.first().expect("math paragraph");
    let image = wordprocessing_math_zone_image(
      std::iter::once(choice),
      &TextStyle::default(),
      &StylesCatalog::default(),
      true,
    )
    .expect("built-up math must produce an image");
    let alt_text = image.alt_text.as_deref().expect("math semantic text");
    assert!(alt_text.contains("+\u{22ef}"), "alt text: {alt_text:?}");
    let svg = std::str::from_utf8(&image.data).expect("math SVG");
    assert!(svg.contains("\u{22ef}"), "SVG semantic text: {svg}");
    assert!(
      !svg.contains("\u{2026}"),
      "SVG retained baseline ellipsis: {svg}"
    );
  }

  #[test]
  fn office_math_clause_separator_preserves_built_up_user_space_provenance() {
    let base_style = TextStyle::default();
    let styles = StylesCatalog::default();
    let parser = MathParser {
      base_style: &base_style,
      styles: &styles,
      math_font_family: Arc::from("Cambria Math"),
    };
    let plain_space_run = m::Run {
      math_run_properties: Some(Box::new(m::RunProperties {
        run_properties_choice: Some(m::RunPropertiesChoice::Sequence(Box::new(
          m::RunPropertiesChoiceSequence {
            style: Some(m::Style {
              val: m::StyleValues::Plain,
            }),
            ..m::RunPropertiesChoiceSequence::default()
          },
        ))),
        ..m::RunProperties::default()
      })),
      run_choice: vec![m::RunChoice::MText(m::Text {
        xml_content: Some("  ".to_string()),
        ..m::Text::default()
      })],
      ..m::Run::default()
    };
    let authored_spaces = parser.run(&plain_space_run);
    assert!(matches!(
      &authored_spaces,
      MathNode::UserSpace {
        clause_separator: false,
        ..
      }
    ));

    let automatic = |text: &str| MathNode::Text {
      text: text.to_string(),
      style: TextStyle::default(),
      math_class_override: None,
    };
    let clause = MathNode::row([automatic(","), authored_spaces.clone(), automatic("x")]);
    assert_eq!(clause.semantic_text(), ",  x");
    let MathNode::Row(clause_nodes) = clause else {
      panic!("clause separator must remain a row");
    };
    assert!(matches!(
      clause_nodes.get(1),
      Some(MathNode::UserSpace {
        clause_separator: true,
        ..
      })
    ));

    let non_clause = MathNode::row([automatic(")"), authored_spaces, automatic("x")]);
    let MathNode::Row(non_clause_nodes) = non_clause else {
      panic!("ordinary user spacing must remain a row");
    };
    assert!(matches!(
      non_clause_nodes.get(1),
      Some(MathNode::UserSpace {
        clause_separator: false,
        ..
      })
    ));

    // The equation.docx linear counterexample stores comma and both spaces
    // in the same unbuilt m:t. Those spaces remain ordinary automatic text
    // atoms instead of acquiring the built-up user-space marker.
    let linear = split_math_text(",  x".to_string(), TextStyle::default(), true);
    let MathNode::Row(linear_nodes) = linear else {
      panic!("linear comma expression must remain a row");
    };
    assert!(
      !linear_nodes
        .iter()
        .any(|node| matches!(node, MathNode::UserSpace { .. }))
    );

    let clause_style = TextStyle {
      font_size_pt: 11.0,
      ..TextStyle::default()
    };
    let mut metrics = TextMetrics::new();
    let clause_box = layout_user_space(
      "  ",
      &clause_style,
      true,
      MathLayoutContext::root(false, false, false),
      &mut metrics,
    );
    assert_eq!(clause_box.width_pt, 22.0);
    assert!(clause_box.items.is_empty());
  }

  #[test]
  fn argument_size_applies_the_complete_ecma_script_level_range() {
    use MathLayoutStyle::{Display, Script, ScriptScript, Text};

    let expected = [
      (Display, [ScriptScript, Script, Display, Display, Display]),
      (Text, [ScriptScript, Script, Text, Text, Text]),
      (Script, [ScriptScript, ScriptScript, Script, Text, Text]),
      (
        ScriptScript,
        [ScriptScript, ScriptScript, ScriptScript, Script, Text],
      ),
    ];
    for (style, results) in expected {
      for (delta, expected) in (-2_i8..=2).zip(results) {
        assert_eq!(style.argument_size(delta), expected);
      }
    }

    assert_eq!(math_argument_size_delta(None), 0);
    for value in -2_i64..=2 {
      let properties = m::ArgumentProperties {
        argument_size: Some(m::ArgumentSize { val: value }),
      };
      assert_eq!(math_argument_size_delta(Some(&properties)), value as i8);
    }
    for value in [-3, 3] {
      let properties = m::ArgumentProperties {
        argument_size: Some(m::ArgumentSize { val: value }),
      };
      assert_eq!(math_argument_size_delta(Some(&properties)), 0);
    }
  }

  #[test]
  fn fraction_style_chain_distinguishes_explicit_and_promoted_display_math() {
    let promoted_display = MathLayoutContext::root(true, false, false);
    let promoted_outer = promoted_display.fraction_argument(m::FractionTypeValues::NoBar);
    assert_eq!(promoted_outer.style, MathLayoutStyle::Text);
    assert_eq!(
      promoted_outer
        .fraction_argument(m::FractionTypeValues::NoBar)
        .style,
      MathLayoutStyle::Script
    );

    for parent in [
      m::FractionTypeValues::Bar,
      m::FractionTypeValues::NoBar,
      m::FractionTypeValues::Skewed,
    ] {
      for nested in [m::FractionTypeValues::Bar, m::FractionTypeValues::NoBar] {
        let explicit_regular = MathLayoutContext::root(true, false, true)
          .fraction_argument(parent)
          .fraction_argument(nested);
        let explicit_small = MathLayoutContext::root(true, true, true)
          .fraction_argument(parent)
          .fraction_argument(nested);
        assert_eq!(explicit_regular.style, MathLayoutStyle::Text);
        assert_eq!(explicit_small.style, MathLayoutStyle::Script);
      }
    }

    let explicit_linear_parent = MathLayoutContext::root(true, false, true)
      .fraction_argument(m::FractionTypeValues::Linear)
      .fraction_argument(m::FractionTypeValues::Bar);
    let inline_fraction =
      MathLayoutContext::root(false, true, false).fraction_argument(m::FractionTypeValues::Bar);
    assert_eq!(explicit_linear_parent.style, MathLayoutStyle::Script);
    assert_eq!(inline_fraction.style, MathLayoutStyle::Script);
  }

  #[test]
  fn office_vertical_fraction_spacing_stays_at_the_built_up_object_boundary() {
    let fraction = |kind| MathNode::Fraction {
      numerator: Box::new(MathNode::Empty),
      denominator: Box::new(MathNode::Empty),
      kind,
      control_style: None,
    };
    let delimiter = |argument| MathNode::Delimiter {
      begin: "(".to_string(),
      separator: "|".to_string(),
      end: ")".to_string(),
      grow: true,
      shape: m::ShapeDelimiterValues::Centered,
      arguments: vec![argument],
      control_style: None,
    };

    let bar = fraction(m::FractionTypeValues::Bar);
    let no_bar = fraction(m::FractionTypeValues::NoBar);
    let linear = fraction(m::FractionTypeValues::Linear);
    let skewed = fraction(m::FractionTypeValues::Skewed);
    let ordinary_delimiter = delimiter(MathNode::Empty);
    let delimited_fraction = delimiter(bar.clone());

    assert!(math_node_is_vertical_fraction_object(&bar));
    assert!(math_node_is_vertical_fraction_object(&no_bar));
    assert!(math_node_is_vertical_fraction_object(&delimited_fraction));
    assert!(!math_node_is_vertical_fraction_object(&linear));
    assert!(!math_node_is_vertical_fraction_object(&skewed));
    assert!(!math_node_is_vertical_fraction_object(&ordinary_delimiter));

    let text_context = MathLayoutContext::root(false, false, false);
    assert_eq!(
      office_math_object_surround_spacing(&ordinary_delimiter, &delimited_fraction, text_context),
      Some(MathSpacing {
        em: MATH_THIN_SPACE_EM,
        owner: MathSpacingOwner::Right,
      })
    );
    assert_eq!(
      office_math_object_surround_spacing(&delimited_fraction, &ordinary_delimiter, text_context),
      Some(MathSpacing {
        em: MATH_THIN_SPACE_EM,
        owner: MathSpacingOwner::Left,
      })
    );
    assert_eq!(
      office_math_object_surround_spacing(
        &ordinary_delimiter,
        &delimited_fraction,
        text_context.script()
      ),
      None
    );
  }

  #[test]
  fn office_math_function_and_nary_arguments_keep_their_distinct_spacing_boundaries() {
    let text = |value: &str| MathNode::Text {
      text: value.to_string(),
      style: TextStyle::default(),
      math_class_override: None,
    };
    let argument = text("x");
    let parenthesized = MathNode::Delimiter {
      begin: "(".to_string(),
      separator: "|".to_string(),
      end: ")".to_string(),
      grow: true,
      shape: m::ShapeDelimiterValues::Centered,
      arguments: vec![argument.clone()],
      control_style: None,
    };
    let text_context = MathLayoutContext::root(false, false, false);
    let modern_inline = text_context.with_compatibility_mode(15);
    let modern_display = MathLayoutContext::root(true, false, true).with_compatibility_mode(15);

    assert_eq!(
      function_application_spacing(&argument, text_context),
      Some(MathSpacing {
        em: MATH_THIN_SPACE_EM,
        owner: MathSpacingOwner::Left,
      })
    );
    assert_eq!(
      function_application_spacing(&parenthesized, text_context),
      None
    );
    assert_eq!(
      function_application_spacing(&argument, text_context.script()),
      Some(MathSpacing {
        em: MATH_THIN_SPACE_EM,
        owner: MathSpacingOwner::Left,
      })
    );
    assert_eq!(function_application_spacing(&text(" "), text_context), None);

    let thin = Some(MathSpacing {
      em: MATH_THIN_SPACE_EM,
      owner: MathSpacingOwner::Left,
    });
    // An m:nary operand is an object-internal n-aryand. Word 2007
    // compatibility keeps its dedicated gap even before a delimiter. Modern
    // display math keeps that established n-aryand boundary, while modern
    // true-inline math resolves the TeX class pair and suppresses Large +
    // Opening. Ordinary operands retain thin glue in every branch, and an
    // authored leading space owns the boundary.
    assert_eq!(nary_argument_spacing(&argument, text_context), thin);
    assert_eq!(nary_argument_spacing(&parenthesized, text_context), thin);
    assert_eq!(nary_argument_spacing(&argument, modern_inline), thin);
    assert_eq!(nary_argument_spacing(&parenthesized, modern_inline), None);
    assert_eq!(nary_argument_spacing(&argument, modern_display), thin);
    assert_eq!(nary_argument_spacing(&parenthesized, modern_display), thin);
    assert_eq!(nary_argument_spacing(&text(" "), text_context), None);
    assert_eq!(nary_argument_spacing(&text(" "), modern_inline), None);

    let function = MathNode::Function {
      name: Box::new(text("sin")),
      argument: Box::new(argument),
    };
    let function_classes = math_node_classes(&function).expect("function has row classes");
    assert_eq!(
      function_classes.left,
      MathSpacingClass::Unicode(MathClass::Large)
    );
    assert_eq!(
      function_classes.right,
      MathSpacingClass::Unicode(MathClass::Alphabetic)
    );
    assert_eq!(
      automatic_math_spacing(MathClass::Normal, function_classes.left, text_context),
      Some(MathSpacing {
        em: MATH_THIN_SPACE_EM,
        owner: MathSpacingOwner::Right,
      })
    );
    assert_eq!(
      automatic_math_spacing(MathClass::Opening, function_classes.left, text_context),
      None
    );
    assert_eq!(
      automatic_math_spacing(MathClass::Binary, function_classes.left, text_context),
      Some(MathSpacing {
        em: MATH_MEDIUM_SPACE_EM,
        owner: MathSpacingOwner::Left,
      })
    );
  }

  #[test]
  fn unicode_math_spacing_resolves_operator_context_and_complete_class_table() {
    use MathClass::{
      Alphabetic, Binary, Closing, Fence, Large, Normal, Opening, Punctuation, Relation, Space,
      Vary,
    };

    assert_eq!(math_character_class('\u{2212}'), Vary);
    assert_eq!(math_character_class('\u{221e}'), Normal);
    assert_eq!(math_character_class('<'), Relation);
    assert_eq!(math_character_class('x'), Alphabetic);

    let vary = MathAtomClasses::single(Vary);
    for previous in [Normal, Alphabetic, Closing, Fence] {
      assert_eq!(
        resolve_vary_math_classes(vary, Some(previous)),
        MathAtomClasses::single(Binary)
      );
    }
    for previous in [None, Some(Binary), Some(Relation), Some(Opening)] {
      assert_eq!(resolve_vary_math_classes(vary, previous), vary);
    }

    let thin_left = Some(MathSpacing {
      em: MATH_THIN_SPACE_EM,
      owner: MathSpacingOwner::Left,
    });
    let thin_right = Some(MathSpacing {
      em: MATH_THIN_SPACE_EM,
      owner: MathSpacingOwner::Right,
    });
    let medium_left = Some(MathSpacing {
      em: MATH_MEDIUM_SPACE_EM,
      owner: MathSpacingOwner::Left,
    });
    let medium_right = Some(MathSpacing {
      em: MATH_MEDIUM_SPACE_EM,
      owner: MathSpacingOwner::Right,
    });
    let thick_left = Some(MathSpacing {
      em: MATH_THICK_SPACE_EM,
      owner: MathSpacingOwner::Left,
    });
    let thick_right = Some(MathSpacing {
      em: MATH_THICK_SPACE_EM,
      owner: MathSpacingOwner::Right,
    });
    let very_thick_left = Some(MathSpacing {
      em: MATH_VERY_THICK_SPACE_EM,
      owner: MathSpacingOwner::Left,
    });

    let text_context = MathLayoutContext::root(false, false, false);
    let script_context = text_context.script();

    assert_eq!(
      automatic_math_spacing(Punctuation, Normal, text_context),
      thin_left
    );
    assert_eq!(
      automatic_math_spacing(Normal, Punctuation, text_context),
      None
    );
    assert_eq!(automatic_math_spacing(Opening, Binary, text_context), None);
    assert_eq!(automatic_math_spacing(Binary, Closing, text_context), None);
    assert_eq!(
      automatic_math_spacing(Relation, Relation, text_context),
      None
    );
    assert_eq!(
      automatic_math_spacing(Relation, Normal, text_context),
      thick_left
    );
    assert_eq!(
      automatic_math_spacing(Normal, Relation, text_context),
      thick_right
    );
    assert_eq!(
      automatic_math_spacing(Binary, Normal, text_context),
      medium_left
    );
    assert_eq!(
      automatic_math_spacing(Normal, Binary, text_context),
      medium_right
    );
    assert_eq!(automatic_math_spacing(Large, Opening, text_context), None);
    assert_eq!(automatic_math_spacing(Large, Fence, text_context), None);
    assert_eq!(
      automatic_math_spacing(Large, Normal, text_context),
      thin_left
    );
    assert_eq!(
      automatic_math_spacing(Normal, Large, text_context),
      thin_right
    );
    assert_eq!(automatic_math_spacing(Space, Relation, text_context), None);
    assert_eq!(automatic_math_spacing(Relation, Space, text_context), None);

    let ellipsis = MathSpacingClass::Ellipsis;
    assert_eq!(
      automatic_math_spacing(Binary, ellipsis, text_context),
      medium_left
    );
    assert_eq!(
      automatic_math_spacing(ellipsis, Binary, text_context),
      medium_right
    );
    assert_eq!(
      automatic_math_spacing(Relation, ellipsis, text_context),
      thick_left
    );
    assert_eq!(
      automatic_math_spacing(ellipsis, Relation, text_context),
      thick_right
    );
    assert_eq!(
      automatic_math_spacing(Punctuation, ellipsis, text_context),
      thin_left
    );
    assert_eq!(
      automatic_math_spacing(ellipsis, Punctuation, text_context),
      very_thick_left
    );
    assert_eq!(
      automatic_math_spacing(Relation, Punctuation, text_context),
      None
    );
    assert_eq!(
      automatic_math_spacing(Opening, ellipsis, text_context),
      None
    );
    assert_eq!(
      automatic_math_spacing(ellipsis, Closing, text_context),
      None
    );

    assert_eq!(
      automatic_math_spacing(Punctuation, Normal, script_context),
      None
    );
    assert_eq!(
      automatic_math_spacing(Relation, Normal, script_context),
      None
    );
    assert_eq!(automatic_math_spacing(Binary, Normal, script_context), None);
    assert_eq!(
      automatic_math_spacing(Large, Normal, script_context),
      thin_left
    );
    assert_eq!(
      automatic_math_spacing(ellipsis, Punctuation, script_context),
      None
    );
  }

  fn office_math_break_layout(
    body: &str,
    break_binary: m::BreakBinaryOperatorValues,
    break_subtraction: m::BreakBinarySubtractionValues,
  ) -> Arc<crate::docx::OfficeMathLineLayout> {
    let xml = format!(
      r#"<w:p xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:m="http://schemas.openxmlformats.org/officeDocument/2006/math"><m:oMath>{body}</m:oMath></w:p>"#
    );
    let paragraph = w::Paragraph::from_bytes(xml.as_bytes()).unwrap();
    let styles = StylesCatalog {
      math_break_binary: Some(break_binary),
      math_break_binary_subtraction: Some(break_subtraction),
      ..StylesCatalog::default()
    };
    wordprocessing_math_zone_image(
      std::iter::once(&paragraph.paragraph_choice[0]),
      &TextStyle::default(),
      &styles,
      false,
    )
    .and_then(|image| image.office_math_line_layout)
    .expect("binary and relation operators provide OfficeMath break fragments")
  }

  #[test]
  fn office_math_break_binary_placement_covers_before_after_and_repeat() {
    let run = r#"<m:r><m:rPr><m:sty m:val="p"/></m:rPr><m:t>a+b=c</m:t></m:r>"#;
    let before = office_math_break_layout(
      run,
      m::BreakBinaryOperatorValues::Before,
      m::BreakBinarySubtractionValues::MinusMinus,
    );
    assert_eq!(
      before
        .fragments
        .iter()
        .map(|fragment| fragment.image.alt_text.as_deref().unwrap_or_default())
        .collect::<Vec<_>>(),
      ["a+", "b=", "c"]
    );

    let after = office_math_break_layout(
      run,
      m::BreakBinaryOperatorValues::After,
      m::BreakBinarySubtractionValues::MinusMinus,
    );
    assert_eq!(
      after
        .fragments
        .iter()
        .map(|fragment| fragment.image.alt_text.as_deref().unwrap_or_default())
        .collect::<Vec<_>>(),
      ["a", "+b", "=c"]
    );

    let repeat = office_math_break_layout(
      run,
      m::BreakBinaryOperatorValues::Repeat,
      m::BreakBinarySubtractionValues::MinusMinus,
    );
    assert_eq!(
      repeat
        .fragments
        .iter()
        .map(|fragment| fragment.image.alt_text.as_deref().unwrap_or_default())
        .collect::<Vec<_>>(),
      ["a+", "b=", "c"]
    );
    assert_eq!(
      repeat
        .fragments
        .iter()
        .map(|fragment| {
          fragment
            .wrapped_prefix
            .as_ref()
            .and_then(|image| image.alt_text.as_deref())
        })
        .collect::<Vec<_>>(),
      [None, Some("+"), Some("=")]
    );
  }

  #[test]
  fn office_math_break_before_keeps_ellipsis_context_across_fragments() {
    let run = r#"<m:f><m:num><m:r><m:t>x</m:t></m:r></m:num><m:den><m:r><m:t>1</m:t></m:r></m:den></m:f><m:r><m:t>+…</m:t></m:r>"#;
    let layout = office_math_break_layout(
      run,
      m::BreakBinaryOperatorValues::Before,
      m::BreakBinarySubtractionValues::MinusMinus,
    );
    assert_eq!(
      layout
        .fragments
        .iter()
        .map(|fragment| fragment.image.alt_text.as_deref().unwrap_or_default())
        .collect::<Vec<_>>(),
      ["𝑥1+", "\u{22ef}"]
    );
  }

  #[test]
  fn office_math_break_repeat_subtraction_preserves_both_serialized_signs() {
    let run = r#"<m:r><m:rPr><m:sty m:val="p"/></m:rPr><m:t>a-b</m:t></m:r>"#;
    let minus_plus = office_math_break_layout(
      run,
      m::BreakBinaryOperatorValues::Repeat,
      m::BreakBinarySubtractionValues::MinusPlus,
    );
    assert_eq!(
      minus_plus.fragments[0].image.alt_text.as_deref(),
      Some("a−")
    );
    assert_eq!(
      minus_plus.fragments[1]
        .wrapped_prefix
        .as_ref()
        .and_then(|image| image.alt_text.as_deref()),
      Some("+")
    );
    assert!(minus_plus.fragments[0].line_end_variant.is_none());

    let plus_minus = office_math_break_layout(
      run,
      m::BreakBinaryOperatorValues::Repeat,
      m::BreakBinarySubtractionValues::PlusMinus,
    );
    assert_eq!(
      plus_minus.fragments[0]
        .line_end_variant
        .as_ref()
        .and_then(|image| image.alt_text.as_deref()),
      Some("a+")
    );
    assert_eq!(
      plus_minus.fragments[1]
        .wrapped_prefix
        .as_ref()
        .and_then(|image| image.alt_text.as_deref()),
      Some("−")
    );
  }

  #[test]
  fn office_math_break_manual_requires_an_operator_and_keeps_align_at() {
    let valid = office_math_break_layout(
      r#"<m:r><m:rPr><m:sty m:val="p"/></m:rPr><m:t>a+</m:t></m:r><m:r><m:rPr><m:sty m:val="p"/><m:brk m:alnAt="1"/></m:rPr><m:t>=b</m:t></m:r>"#,
      m::BreakBinaryOperatorValues::Before,
      m::BreakBinarySubtractionValues::MinusMinus,
    );
    assert!(valid.has_manual_break);
    assert!(matches!(
      valid.fragments[1].break_before,
      Some(crate::docx::OfficeMathBreakKind::Manual { align_at: Some(1) })
    ));

    let ignored = office_math_break_layout(
      r#"<m:r><m:rPr><m:sty m:val="p"/></m:rPr><m:t>a+</m:t></m:r><m:r><m:rPr><m:sty m:val="p"/><m:brk m:alnAt="1"/></m:rPr><m:t>b</m:t></m:r>"#,
      m::BreakBinaryOperatorValues::Before,
      m::BreakBinarySubtractionValues::MinusMinus,
    );
    assert!(!ignored.has_manual_break);
  }
}
