use std::sync::Arc;

use ooxmlsdk::schemas::{m, schemas_openxmlformats_org_wordprocessingml_2006_main as w};

use super::{
  ImageCrop, ImagePlacement, InlineImage, RgbColor, StylesCatalog, TextStyle, properties,
};
use crate::render::math as shared_math;
use crate::text_metrics::{MathFontMetrics, TextMetrics};

const OFFICE_MATH_SVG_CONTENT_TYPE: &str = "application/vnd.ooxmlsdk.office-math+xml";
const MIN_MATH_SIZE_PT: f32 = 1.0;
const MIN_RULE_WIDTH_PT: f32 = 0.2;

macro_rules! parse_math_choice {
  ($parser:expr, $choice_type:ident, $choice:expr) => {
    match $choice {
      m::$choice_type::Accent(value) => $parser.accent(value),
      m::$choice_type::Bar(value) => $parser.bar(value),
      m::$choice_type::Box(value) => $parser.base(&value.base),
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

pub(super) fn wordprocessing_math_image(
  choice: &w::ParagraphChoice,
  base_style: &TextStyle,
  styles: &StylesCatalog,
) -> Option<InlineImage> {
  let parser = MathParser {
    base_style,
    styles,
    math_font_family: styles
      .math_font_family
      .clone()
      .unwrap_or_else(|| Arc::from("Cambria Math")),
  };
  let node = parser.wordprocessing_choice(choice)?;
  if !node.needs_two_dimensional_layout() {
    return None;
  }

  let mut text_metrics = TextMetrics::new();
  let mut math_box = layout_node(&node, 0, &mut text_metrics);
  if math_box.width_pt <= f32::EPSILON || math_box.ascent_pt + math_box.descent_pt <= f32::EPSILON {
    return None;
  }
  // Keep adjacent m:oMath zones visually separate while making the boundary
  // part of their measured advance for centering and wrapping.
  math_box.pad_horizontally(base_style.font_size_pt * 0.08);
  let semantic_text = node.semantic_text();
  let svg = math_box.to_svg();
  let padding = MIN_RULE_WIDTH_PT;
  Some(InlineImage {
    data: Arc::<[u8]>::from(svg.into_bytes()),
    content_type: Some(OFFICE_MATH_SVG_CONTENT_TYPE.to_string()),
    picture_frame: None,
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
    crop: ImageCrop::default(),
    rotation_deg: 0.0,
    flip_horizontal: false,
    flip_vertical: false,
    metafile_background_color: None,
    alt_text: (!semantic_text.is_empty()).then_some(semantic_text),
    hyperlink_url: None,
    semantic_metafile_text: false,
    metafile_native_size: false,
    picture_content_control: false,
    placement: ImagePlacement::Inline,
  })
}

struct MathParser<'a> {
  base_style: &'a TextStyle,
  styles: &'a StylesCatalog,
  math_font_family: Arc<str>,
}

impl MathParser<'_> {
  fn default_math_style(&self) -> TextStyle {
    let mut style = self.base_style.clone();
    style.font_family = Some(self.math_font_family.clone());
    style.fallback_font_family = self.base_style.fallback_font_family.clone();
    style
  }

  fn wordprocessing_choice(&self, choice: &w::ParagraphChoice) -> Option<MathNode> {
    Some(match choice {
      w::ParagraphChoice::Paragraph(value) => self.paragraph(value),
      w::ParagraphChoice::OfficeMath(value) => self.office_math(value),
      w::ParagraphChoice::Accent(value) => self.accent(value),
      w::ParagraphChoice::Bar(value) => self.bar(value),
      w::ParagraphChoice::Box(value) => self.base(&value.base),
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
    MathNode::row(
      value
        .base_choice
        .iter()
        .map(|choice| parse_math_choice!(self, BaseChoice, choice)),
    )
  }

  fn numerator(&self, value: &m::Numerator) -> MathNode {
    MathNode::row(
      value
        .numerator_choice
        .iter()
        .map(|choice| parse_math_choice!(self, NumeratorChoice, choice)),
    )
  }

  fn denominator(&self, value: &m::Denominator) -> MathNode {
    MathNode::row(
      value
        .denominator_choice
        .iter()
        .map(|choice| parse_math_choice!(self, DenominatorChoice, choice)),
    )
  }

  fn function_name(&self, value: &m::FunctionName) -> MathNode {
    MathNode::row(
      value
        .function_name_choice
        .iter()
        .map(|choice| parse_math_choice!(self, FunctionNameChoice, choice)),
    )
  }

  fn limit(&self, value: &m::Limit) -> MathNode {
    MathNode::row(
      value
        .limit_choice
        .iter()
        .map(|choice| parse_math_choice!(self, LimitChoice, choice)),
    )
  }

  fn sub_argument(&self, value: &m::SubArgument) -> MathNode {
    MathNode::row(
      value
        .sub_argument_choice
        .iter()
        .map(|choice| parse_math_choice!(self, SubArgumentChoice, choice)),
    )
  }

  fn super_argument(&self, value: &m::SuperArgument) -> MathNode {
    MathNode::row(
      value
        .super_argument_choice
        .iter()
        .map(|choice| parse_math_choice!(self, SuperArgumentChoice, choice)),
    )
  }

  fn degree(&self, value: &m::Degree) -> MathNode {
    MathNode::row(
      value
        .degree_choice
        .iter()
        .map(|choice| parse_math_choice!(self, DegreeChoice, choice)),
    )
  }

  fn run(&self, run: &m::Run) -> MathNode {
    let text = shared_math::math_run_text(run);
    if text.is_empty() {
      return MathNode::Empty;
    }
    let mut style = properties::run_style(
      run.run_properties.as_deref(),
      self.base_style.clone(),
      self.styles,
    );
    // ECMA-376 Part 1 §22.1.2.61: ordinary w:rFonts on a math run does not
    // replace the document-wide math font, but all other run properties still
    // participate in the character cascade.
    style.font_family = Some(self.math_font_family.clone());
    style.fallback_font_family = self.base_style.fallback_font_family.clone();
    split_math_text(text, style)
  }

  fn accent(&self, value: &m::Accent) -> MathNode {
    MathNode::Accent {
      base: Box::new(self.base(&value.base)),
      character: value
        .accent_properties
        .as_deref()
        .and_then(|properties| properties.accent_char.as_ref())
        .map_or("\u{0302}", |character| character.val.as_str())
        .to_string(),
    }
  }

  fn bar(&self, value: &m::Bar) -> MathNode {
    let bottom = value
      .bar_properties
      .as_deref()
      .and_then(|properties| properties.position.as_ref())
      .is_some_and(|position| position.val == m::VerticalJustificationValues::Bottom);
    MathNode::Bar {
      base: Box::new(self.base(&value.base)),
      bottom,
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
      arguments: value.base.iter().map(|base| self.base(base)).collect(),
    }
  }

  fn equation_array(&self, value: &m::EquationArray) -> MathNode {
    MathNode::EquationArray(value.base.iter().map(|base| self.base(base)).collect())
  }

  fn fraction(&self, value: &m::Fraction) -> MathNode {
    MathNode::Fraction {
      numerator: Box::new(self.numerator(&value.numerator)),
      denominator: Box::new(self.denominator(&value.denominator)),
      kind: value
        .fraction_properties
        .as_deref()
        .and_then(|properties| properties.fraction_type.as_ref())
        .map(|value| value.val)
        .unwrap_or_default(),
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
      .is_some_and(|position| position.val == m::VerticalJustificationValues::Bottom);
    MathNode::GroupChar {
      base: Box::new(self.base(&value.base)),
      character: properties
        .and_then(|value| value.accent_char.as_ref())
        .map_or(if bottom { "\u{23df}" } else { "\u{23de}" }, |value| {
          value.val.as_str()
        })
        .to_string(),
      bottom,
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
    MathNode::Nary {
      operator: properties
        .and_then(|value| value.accent_char.as_ref())
        .map_or("\u{222b}", |value| value.val.as_str())
        .to_string(),
      lower: (!properties
        .and_then(|value| value.hide_sub_argument.as_ref())
        .is_some_and(|value| math_on(value.val)))
      .then(|| Box::new(self.sub_argument(&value.sub_argument))),
      upper: (!properties
        .and_then(|value| value.hide_super_argument.as_ref())
        .is_some_and(|value| math_on(value.val)))
      .then(|| Box::new(self.super_argument(&value.super_argument))),
      base: Box::new(self.base(&value.base)),
      side_limits: properties
        .and_then(|value| value.limit_location.as_ref())
        .is_some_and(|value| value.val == m::LimitLocationValues::SubscriptSuperscript),
      grow: properties
        .and_then(|value| value.grow_operators.as_ref())
        .is_none_or(|value| math_on(value.val)),
      style: self.default_math_style(),
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
    let degree_hidden = value
      .radical_properties
      .as_deref()
      .and_then(|properties| properties.hide_degree.as_ref())
      .is_some_and(|value| math_on(value.val));
    MathNode::Radical {
      degree: (!degree_hidden).then(|| Box::new(self.degree(&value.degree))),
      base: Box::new(self.base(&value.base)),
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

fn split_math_text(text: String, style: TextStyle) -> MathNode {
  let mut nodes = Vec::new();
  let mut plain = String::new();
  for character in text.chars() {
    if math_operator_spacing_em(character).is_some() {
      if !plain.is_empty() {
        nodes.push(MathNode::Text {
          text: std::mem::take(&mut plain),
          style: style.clone(),
        });
      }
      nodes.push(MathNode::Text {
        text: character.to_string(),
        style: style.clone(),
      });
    } else {
      plain.push(character);
    }
  }
  if !plain.is_empty() {
    nodes.push(MathNode::Text { text: plain, style });
  }
  MathNode::row(nodes)
}

fn math_operator_spacing_em(character: char) -> Option<f32> {
  match character {
    '=' | '<' | '>' | '\u{2260}' | '\u{2264}' | '\u{2265}' | '\u{2248}' | '\u{223c}'
    | '\u{2208}' | '\u{2209}' | '\u{2282}' | '\u{2283}' | '\u{2286}' | '\u{2287}' => Some(0.28),
    '+' | '-' | '\u{2212}' | '\u{00b1}' | '\u{2213}' | '\u{00d7}' | '\u{00f7}' | '\u{22c5}'
    | '\u{2217}' | '\u{2227}' | '\u{2228}' | '\u{2229}' | '\u{222a}' => Some(0.20),
    ',' | ';' | ':' => Some(0.10),
    _ => None,
  }
}

#[derive(Clone, Debug)]
enum MathNode {
  Empty,
  Row(Vec<Self>),
  Text {
    text: String,
    style: TextStyle,
  },
  Fraction {
    numerator: Box<Self>,
    denominator: Box<Self>,
    kind: m::FractionTypeValues,
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
    arguments: Vec<Self>,
  },
  EquationArray(Vec<Self>),
  Matrix(Vec<Vec<Self>>),
  Nary {
    operator: String,
    lower: Option<Box<Self>>,
    upper: Option<Box<Self>>,
    base: Box<Self>,
    side_limits: bool,
    grow: bool,
    style: TextStyle,
  },
  Radical {
    degree: Option<Box<Self>>,
    base: Box<Self>,
  },
  Accent {
    base: Box<Self>,
    character: String,
  },
  Bar {
    base: Box<Self>,
    bottom: bool,
  },
  GroupChar {
    base: Box<Self>,
    character: String,
    bottom: bool,
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
    match output.len() {
      0 => Self::Empty,
      1 => output.pop().unwrap_or(Self::Empty),
      _ => Self::Row(output),
    }
  }

  fn needs_two_dimensional_layout(&self) -> bool {
    match self {
      Self::Empty | Self::Text { .. } => false,
      Self::Row(nodes) => nodes.iter().any(Self::needs_two_dimensional_layout),
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
      Self::Empty => {}
      Self::Text { text, .. } => output.push_str(text),
      Self::Row(nodes) => nodes
        .iter()
        .for_each(|node| node.append_semantic_text(output)),
      Self::Fraction {
        numerator,
        denominator,
        kind,
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
      Self::Radical { degree, base } => {
        output.push('\u{221a}');
        base.append_semantic_text(output);
        degree
          .as_deref()
          .into_iter()
          .for_each(|node| node.append_semantic_text(output));
      }
      Self::Accent { base, character }
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

#[derive(Clone, Debug)]
struct MathBox {
  width_pt: f32,
  ascent_pt: f32,
  descent_pt: f32,
  items: Vec<MathPaintItem>,
}

impl MathBox {
  fn empty() -> Self {
    Self {
      width_pt: 0.0,
      ascent_pt: 0.0,
      descent_pt: 0.0,
      items: Vec::new(),
    }
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

  fn pad_horizontally(&mut self, padding_pt: f32) {
    let padding_pt = padding_pt.max(0.0);
    if padding_pt <= f32::EPSILON {
      return;
    }
    self.items = std::mem::take(&mut self.items)
      .into_iter()
      .map(|item| item.translated(padding_pt, 0.0))
      .collect();
    self.width_pt += padding_pt * 2.0;
  }

  fn to_svg(&self) -> String {
    let padding = MIN_RULE_WIDTH_PT;
    let width = (self.width_pt + padding * 2.0).max(1.0);
    let height = (self.ascent_pt + self.descent_pt + padding * 2.0).max(1.0);
    let baseline = self.ascent_pt + padding;
    let mut svg = format!(
      "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{width:.4}\" height=\"{height:.4}\" viewBox=\"0 0 {width:.4} {height:.4}\">"
    );
    for item in &self.items {
      match item {
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
            "<text x=\"0\" y=\"0\" transform=\"translate({:.4} {:.4}) scale({:.6} 1)\" font-family=\"{}\" font-size=\"{:.4}\" font-weight=\"{}\" font-style=\"{}\" fill=\"#{:02x}{:02x}{:02x}\" fill-opacity=\"{:.4}\" xml:space=\"preserve\">{}</text>",
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

#[derive(Clone, Debug)]
enum MathPaintItem {
  Text {
    text: String,
    style: TextStyle,
    x_pt: f32,
    baseline_y_pt: f32,
    horizontal_scale: f32,
    opacity: f32,
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
      Self::Text {
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

fn layout_node(node: &MathNode, script_level: u8, metrics: &mut TextMetrics) -> MathBox {
  match node {
    MathNode::Empty => MathBox::empty(),
    MathNode::Text { text, style } => layout_text(text, style, script_level, metrics),
    MathNode::Row(nodes) => layout_row(nodes, script_level, metrics),
    MathNode::Fraction {
      numerator,
      denominator,
      kind,
    } => layout_fraction(numerator, denominator, *kind, script_level, metrics),
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
      script_level,
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
      script_level,
      metrics,
    ),
    MathNode::Delimiter {
      begin,
      separator,
      end,
      grow,
      arguments,
    } => layout_delimiter(
      begin,
      separator,
      end,
      *grow,
      arguments,
      script_level,
      metrics,
    ),
    MathNode::EquationArray(rows) => layout_stack(rows, script_level, metrics, 0.25),
    MathNode::Matrix(rows) => layout_matrix(rows, script_level, metrics),
    MathNode::Nary {
      operator,
      lower,
      upper,
      base,
      side_limits,
      grow,
      style,
    } => layout_nary(
      operator,
      lower.as_deref(),
      upper.as_deref(),
      base,
      *side_limits,
      *grow,
      style,
      script_level,
      metrics,
    ),
    MathNode::Radical { degree, base } => {
      layout_radical(degree.as_deref(), base, script_level, metrics)
    }
    MathNode::Accent { base, character } => {
      layout_accent(base, character, false, script_level, metrics)
    }
    MathNode::Bar { base, bottom } => layout_bar(base, *bottom, script_level, metrics),
    MathNode::GroupChar {
      base,
      character,
      bottom,
    } => layout_accent(base, character, *bottom, script_level, metrics),
    MathNode::Function { name, argument } => layout_function(name, argument, script_level, metrics),
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
      script_level,
      metrics,
    ),
    MathNode::Phantom {
      base,
      transparent,
      zero_width,
      zero_ascent,
      zero_descent,
    } => {
      let mut result = layout_node(base, script_level, metrics);
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

fn layout_text(
  text: &str,
  source_style: &TextStyle,
  script_level: u8,
  metrics: &mut TextMetrics,
) -> MathBox {
  if text.is_empty() {
    return MathBox::empty();
  }
  let style = math_script_style(source_style, script_level, metrics);
  let shaped = metrics.shape_text(text, &style);
  let width_pt = shaped.as_ref().map_or_else(
    || metrics.measure_text(text, &style),
    |shaped| shaped.width_pt,
  );
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
    if ink_ascent > 0.0 {
      ascent_pt = ink_ascent;
    }
    if ink_descent > 0.0 {
      descent_pt = ink_descent;
    }
  }
  MathBox {
    width_pt,
    ascent_pt,
    descent_pt,
    items: vec![MathPaintItem::Text {
      text: text.to_string(),
      style,
      x_pt: 0.0,
      baseline_y_pt: 0.0,
      horizontal_scale: 1.0,
      opacity: 1.0,
    }],
  }
}

fn math_script_style(source: &TextStyle, script_level: u8, metrics: &mut TextMetrics) -> TextStyle {
  let mut style = source.clone();
  if script_level > 0 {
    let math = metrics.math_font_metrics(source);
    let scale = if script_level == 1 {
      math.script_scale
    } else {
      math.script_script_scale
    };
    style.font_size_pt = (style.font_size_pt * scale).max(MIN_MATH_SIZE_PT);
    if let Some(size) = &mut style.complex_font_size_pt {
      *size = (*size * scale).max(MIN_MATH_SIZE_PT);
    }
  }
  style.baseline_shift_pt = 0.0;
  style
}

fn layout_row(nodes: &[MathNode], script_level: u8, metrics: &mut TextMetrics) -> MathBox {
  let mut result = MathBox::empty();
  let mut x = 0.0;
  for (index, node) in nodes.iter().enumerate() {
    if index > 0 {
      x += math_node_spacing(&nodes[index - 1], node, script_level, metrics);
    }
    let child = layout_node(node, script_level, metrics);
    result.append(child, x, 0.0);
    x = result.width_pt;
  }
  result.width_pt = x;
  result
}

fn math_node_spacing(
  left: &MathNode,
  right: &MathNode,
  script_level: u8,
  metrics: &mut TextMetrics,
) -> f32 {
  let coefficient = trailing_math_character(left)
    .and_then(math_operator_spacing_em)
    .into_iter()
    .chain(leading_math_character(right).and_then(math_operator_spacing_em))
    .fold(0.0_f32, f32::max);
  if coefficient <= f32::EPSILON {
    return 0.0;
  }
  representative_style(left)
    .or_else(|| representative_style(right))
    .map(|style| math_script_style(style, script_level, metrics).font_size_pt * coefficient)
    .unwrap_or(0.0)
}

fn leading_math_character(node: &MathNode) -> Option<char> {
  match node {
    MathNode::Text { text, .. } => text.chars().next(),
    MathNode::Row(nodes) => nodes.iter().find_map(leading_math_character),
    _ => None,
  }
}

fn trailing_math_character(node: &MathNode) -> Option<char> {
  match node {
    MathNode::Text { text, .. } => text.chars().next_back(),
    MathNode::Row(nodes) => nodes.iter().rev().find_map(trailing_math_character),
    _ => None,
  }
}

fn representative_style(node: &MathNode) -> Option<&TextStyle> {
  match node {
    MathNode::Text { style, .. } => Some(style),
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
    MathNode::Radical { degree, base } => {
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
  script_level: u8,
  metrics: &mut TextMetrics,
) -> MathBox {
  if matches!(
    kind,
    m::FractionTypeValues::Linear | m::FractionTypeValues::Skewed
  ) {
    let Some(style) = representative_style(numerator).or_else(|| representative_style(denominator))
    else {
      return MathBox::empty();
    };
    let slash = MathNode::Text {
      text: "\u{2044}".to_string(),
      style: style.clone(),
    };
    let nodes = [numerator.clone(), slash, denominator.clone()];
    let mut result = layout_row(&nodes, script_level + 1, metrics);
    if kind == m::FractionTypeValues::Skewed {
      let shift = style.font_size_pt * 0.12;
      if let Some(first) = result.items.first_mut() {
        *first = first.clone().translated(0.0, -shift);
      }
      result.ascent_pt += shift;
    }
    return result;
  }

  let Some(style) = representative_style(numerator)
    .or_else(|| representative_style(denominator))
    .cloned()
  else {
    // Empty binomials still occupy no visual area.
    return MathBox::empty();
  };
  let numerator = layout_node(numerator, script_level + 1, metrics);
  let denominator = layout_node(denominator, script_level + 1, metrics);
  let math = metrics.math_font_metrics(&style);
  let rule = math.fraction_rule_thickness_pt.max(MIN_RULE_WIDTH_PT);
  let axis_y = -math.axis_height_pt;
  let padding = style.font_size_pt * 0.08;
  let width = numerator.width_pt.max(denominator.width_pt) + padding * 2.0;
  let numerator_y =
    (axis_y - rule / 2.0 - math.fraction_numerator_gap_min_pt - numerator.descent_pt)
      .min(-math.fraction_numerator_shift_up_pt);
  let denominator_y =
    (axis_y + rule / 2.0 + math.fraction_denominator_gap_min_pt + denominator.ascent_pt)
      .max(math.fraction_denominator_shift_down_pt);
  let mut result = MathBox::empty();
  let numerator_width = numerator.width_pt;
  let denominator_width = denominator.width_pt;
  result.append(numerator, (width - numerator_width) / 2.0, numerator_y);
  result.append(
    denominator,
    (width - denominator_width) / 2.0,
    denominator_y,
  );
  if kind != m::FractionTypeValues::NoBar {
    result.items.push(MathPaintItem::Line {
      x1_pt: 0.0,
      y1_pt: axis_y,
      x2_pt: width,
      y2_pt: axis_y,
      width_pt: rule,
      color: style.color,
      opacity: style.opacity,
    });
  }
  result.width_pt = width;
  result.ascent_pt = result.ascent_pt.max(-axis_y + rule / 2.0);
  result.descent_pt = result.descent_pt.max(axis_y + rule / 2.0);
  result
}

fn layout_scripts(
  base: &MathNode,
  lower: Option<&MathNode>,
  upper: Option<&MathNode>,
  pre: bool,
  script_level: u8,
  metrics: &mut TextMetrics,
) -> MathBox {
  let base_box = layout_node(base, script_level, metrics);
  let lower_box = lower.map(|node| layout_node(node, script_level + 1, metrics));
  let upper_box = upper.map(|node| layout_node(node, script_level + 1, metrics));
  let Some(style) = representative_style(base)
    .or_else(|| lower.and_then(representative_style))
    .or_else(|| upper.and_then(representative_style))
  else {
    return base_box;
  };
  let math = metrics.math_font_metrics(style);
  layout_scripts_around_box(base_box, lower_box, upper_box, pre, style, math)
}

fn layout_scripts_around_box(
  base_box: MathBox,
  lower_box: Option<MathBox>,
  upper_box: Option<MathBox>,
  pre: bool,
  style: &TextStyle,
  math: MathFontMetrics,
) -> MathBox {
  let mut upper_y = upper_box.as_ref().map(|upper| {
    (-math.superscript_shift_up_pt).min(-base_box.ascent_pt * 0.55 - upper.descent_pt)
  });
  let mut lower_y = lower_box.as_ref().map(|lower| {
    math
      .subscript_shift_down_pt
      .max(base_box.descent_pt * 0.55 + lower.ascent_pt * 0.35)
  });
  if let (Some(upper), Some(lower), Some(upper_y_value), Some(lower_y_value)) =
    (upper_box.as_ref(), lower_box.as_ref(), upper_y, lower_y)
  {
    let gap = lower_y_value - lower.ascent_pt - (upper_y_value + upper.descent_pt);
    if gap < math.sub_superscript_gap_min_pt {
      let adjustment = (math.sub_superscript_gap_min_pt - gap) / 2.0;
      upper_y = Some(upper_y_value - adjustment);
      lower_y = Some(lower_y_value + adjustment);
    }
  }
  let script_width = lower_box
    .as_ref()
    .map_or(0.0_f32, |value| value.width_pt)
    .max(upper_box.as_ref().map_or(0.0, |value| value.width_pt));
  let script_gap = math.space_after_script_pt.max(style.font_size_pt * 0.02);
  let mut result = MathBox::empty();
  let base_x = if pre { script_width + script_gap } else { 0.0 };
  let script_x = if pre {
    0.0
  } else {
    base_box.width_pt + script_gap
  };
  let base_width = base_box.width_pt;
  result.append(base_box, base_x, 0.0);
  if let (Some(lower), Some(y)) = (lower_box, lower_y) {
    let x = if pre {
      (script_width - lower.width_pt).max(0.0)
    } else {
      script_x
    };
    result.append(lower, x, y);
  }
  if let (Some(upper), Some(y)) = (upper_box, upper_y) {
    let x = if pre {
      (script_width - upper.width_pt).max(0.0)
    } else {
      script_x
    };
    result.append(upper, x, y);
  }
  result.width_pt = base_width + script_width + script_gap;
  result
}

fn layout_function(
  name: &MathNode,
  argument: &MathNode,
  script_level: u8,
  metrics: &mut TextMetrics,
) -> MathBox {
  let name_box = layout_node(name, script_level, metrics);
  let argument_box = layout_node(argument, script_level, metrics);
  let gap = representative_style(name)
    .or_else(|| representative_style(argument))
    .map_or(0.0, |style| style.font_size_pt * 0.12);
  let name_width = name_box.width_pt;
  let mut result = MathBox::empty();
  result.append(name_box, 0.0, 0.0);
  result.append(argument_box, name_width + gap, 0.0);
  result
}

fn layout_limits(
  base: &MathNode,
  lower: Option<&MathNode>,
  upper: Option<&MathNode>,
  side: bool,
  script_level: u8,
  metrics: &mut TextMetrics,
) -> MathBox {
  if side {
    return layout_scripts(base, lower, upper, false, script_level, metrics);
  }
  let base_box = layout_node(base, script_level, metrics);
  let lower_box = lower.map(|node| layout_node(node, script_level + 1, metrics));
  let upper_box = upper.map(|node| layout_node(node, script_level + 1, metrics));
  let Some(style) = representative_style(base)
    .or_else(|| lower.and_then(representative_style))
    .or_else(|| upper.and_then(representative_style))
  else {
    return base_box;
  };
  let math = metrics.math_font_metrics(style);
  let width = base_box
    .width_pt
    .max(lower_box.as_ref().map_or(0.0, |value| value.width_pt))
    .max(upper_box.as_ref().map_or(0.0, |value| value.width_pt));
  let base_width = base_box.width_pt;
  let base_ascent = base_box.ascent_pt;
  let base_descent = base_box.descent_pt;
  let mut result = MathBox::empty();
  result.append(base_box, (width - base_width) / 2.0, 0.0);
  if let Some(lower) = lower_box {
    let y = base_descent + math.lower_limit_gap_min_pt + lower.ascent_pt;
    let child_width = lower.width_pt;
    result.append(lower, (width - child_width) / 2.0, y);
  }
  if let Some(upper) = upper_box {
    let y = -(base_ascent + math.upper_limit_gap_min_pt + upper.descent_pt);
    let child_width = upper.width_pt;
    result.append(upper, (width - child_width) / 2.0, y);
  }
  result.width_pt = width;
  result
}

fn layout_delimiter(
  begin: &str,
  separator: &str,
  end: &str,
  grow: bool,
  arguments: &[MathNode],
  script_level: u8,
  metrics: &mut TextMetrics,
) -> MathBox {
  let Some(style) = arguments.iter().find_map(representative_style) else {
    return MathBox::empty();
  };
  let mut contents = MathBox::empty();
  let mut x = 0.0;
  for (index, argument) in arguments.iter().enumerate() {
    if index > 0 && !separator.is_empty() {
      let separator_box = layout_stretched_symbol(
        separator,
        style,
        script_level,
        contents.ascent_pt + contents.descent_pt,
        false,
        metrics,
      );
      contents.append(separator_box, x, 0.0);
      x = contents.width_pt;
    }
    let argument = layout_node(argument, script_level, metrics);
    contents.append(argument, x, 0.0);
    x = contents.width_pt;
  }
  contents.width_pt = x;
  let target_height = if grow {
    (contents.ascent_pt + contents.descent_pt).max(style.font_size_pt)
  } else {
    style.font_size_pt
  };
  let begin_box = if begin.is_empty() {
    MathBox::empty()
  } else {
    layout_stretched_symbol(begin, style, script_level, target_height, true, metrics)
  };
  let end_box = if end.is_empty() {
    MathBox::empty()
  } else {
    layout_stretched_symbol(end, style, script_level, target_height, true, metrics)
  };
  let gap = style.font_size_pt * 0.04;
  let mut result = MathBox::empty();
  let begin_width = begin_box.width_pt;
  result.append(begin_box, 0.0, 0.0);
  result.append(contents, begin_width + gap, 0.0);
  let content_end = result.width_pt;
  result.append(end_box, content_end + gap, 0.0);
  result
}

fn layout_stretched_symbol(
  symbol: &str,
  source_style: &TextStyle,
  script_level: u8,
  target_height_pt: f32,
  narrow: bool,
  metrics: &mut TextMetrics,
) -> MathBox {
  let normal = layout_text(symbol, source_style, script_level, metrics);
  let normal_height = (normal.ascent_pt + normal.descent_pt).max(1.0);
  if target_height_pt <= normal_height * 1.05 {
    return normal;
  }
  let mut style = math_script_style(source_style, script_level, metrics);
  let scale = (target_height_pt / normal_height).clamp(1.0, 4.0);
  style.font_size_pt *= scale;
  let mut result = layout_text(symbol, &style, 0, metrics);
  if narrow && scale > 1.2 {
    // OpenType MATH vertical variants grow far less horizontally than scaling
    // an ordinary text glyph. Approximate that variant advance while applying
    // the same horizontal compression to the painted glyph, so siblings never
    // overlap the operator's visual bounds.
    let target_width = result
      .width_pt
      .min(normal.width_pt * (1.0 + (scale - 1.0) * 0.25));
    let horizontal_scale = target_width / result.width_pt.max(f32::EPSILON);
    scale_box_horizontally(&mut result, horizontal_scale);
  }
  result
}

fn scale_box_horizontally(math_box: &mut MathBox, scale: f32) {
  for item in &mut math_box.items {
    match item {
      MathPaintItem::Text {
        x_pt,
        horizontal_scale,
        ..
      } => {
        *x_pt *= scale;
        *horizontal_scale *= scale;
      }
      MathPaintItem::Line { x1_pt, x2_pt, .. } => {
        *x1_pt *= scale;
        *x2_pt *= scale;
      }
    }
  }
  math_box.width_pt *= scale;
}

fn layout_stack(
  rows: &[MathNode],
  script_level: u8,
  metrics: &mut TextMetrics,
  gap_em: f32,
) -> MathBox {
  let boxes = rows
    .iter()
    .map(|row| layout_node(row, script_level, metrics))
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

fn layout_matrix(rows: &[Vec<MathNode>], script_level: u8, metrics: &mut TextMetrics) -> MathBox {
  let column_count = rows.iter().map(Vec::len).max().unwrap_or(0);
  if column_count == 0 {
    return MathBox::empty();
  }
  let mut boxes = Vec::with_capacity(rows.len());
  let mut column_widths = vec![0.0_f32; column_count];
  for row in rows {
    let row_boxes = row
      .iter()
      .map(|cell| layout_node(cell, script_level, metrics))
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

#[allow(clippy::too_many_arguments)]
fn layout_nary(
  operator: &str,
  lower: Option<&MathNode>,
  upper: Option<&MathNode>,
  base: &MathNode,
  side_limits: bool,
  grow: bool,
  fallback_style: &TextStyle,
  script_level: u8,
  metrics: &mut TextMetrics,
) -> MathBox {
  let Some(style) = representative_style(base)
    .or_else(|| lower.and_then(representative_style))
    .or_else(|| upper.and_then(representative_style))
    .or(Some(fallback_style))
  else {
    return MathBox::empty();
  };
  let math = metrics.math_font_metrics(style);
  let target_height = if grow {
    math
      .display_operator_min_height_pt
      .max(style.font_size_pt * 1.2)
  } else {
    style.font_size_pt
  };
  let operator_box =
    layout_stretched_symbol(operator, style, script_level, target_height, false, metrics);
  let operator_with_limits = if side_limits {
    let lower_box = lower.map(|node| layout_node(node, script_level + 1, metrics));
    let upper_box = upper.map(|node| layout_node(node, script_level + 1, metrics));
    layout_scripts_around_box(operator_box, lower_box, upper_box, false, style, math)
  } else {
    let lower_box = lower.map(|node| layout_node(node, script_level + 1, metrics));
    let upper_box = upper.map(|node| layout_node(node, script_level + 1, metrics));
    let width = operator_box
      .width_pt
      .max(lower_box.as_ref().map_or(0.0, |value| value.width_pt))
      .max(upper_box.as_ref().map_or(0.0, |value| value.width_pt));
    let operator_width = operator_box.width_pt;
    let operator_ascent = operator_box.ascent_pt;
    let operator_descent = operator_box.descent_pt;
    let mut result = MathBox::empty();
    result.append(operator_box, (width - operator_width) / 2.0, 0.0);
    if let Some(lower) = lower_box {
      let child_width = lower.width_pt;
      let y = operator_descent + math.lower_limit_gap_min_pt + lower.ascent_pt;
      result.append(lower, (width - child_width) / 2.0, y);
    }
    if let Some(upper) = upper_box {
      let child_width = upper.width_pt;
      let y = -(operator_ascent + math.upper_limit_gap_min_pt + upper.descent_pt);
      result.append(upper, (width - child_width) / 2.0, y);
    }
    result.width_pt = width;
    result
  };
  let base_box = layout_node(base, script_level, metrics);
  // LibreOffice's default DIS_OPERATORSPACE is 20% of the formula font.
  let gap = style.font_size_pt * 0.20;
  let mut result = MathBox::empty();
  let operator_width = operator_with_limits.width_pt;
  result.append(operator_with_limits, 0.0, 0.0);
  result.append(base_box, operator_width + gap, 0.0);
  result
}

fn layout_radical(
  degree: Option<&MathNode>,
  base: &MathNode,
  script_level: u8,
  metrics: &mut TextMetrics,
) -> MathBox {
  let base_box = layout_node(base, script_level, metrics);
  let Some(style) = representative_style(base).or_else(|| degree.and_then(representative_style))
  else {
    return MathBox::empty();
  };
  let math = metrics.math_font_metrics(style);
  let target_height = base_box.ascent_pt
    + base_box.descent_pt
    + math.radical_vertical_gap_pt
    + math.radical_rule_thickness_pt
    + math.radical_extra_ascender_pt;
  let radical = layout_stretched_symbol(
    "\u{221a}",
    style,
    script_level,
    target_height,
    true,
    metrics,
  );
  let degree_box = degree.map(|node| layout_node(node, script_level + 2, metrics));
  let degree_width = degree_box.as_ref().map_or(0.0, |value| value.width_pt);
  let degree_prefix = if degree_box.is_some() {
    (degree_width + math.radical_kern_before_degree_pt + math.radical_kern_after_degree_pt).max(0.0)
  } else {
    0.0
  };
  let radical_base_gap = style.font_size_pt * 0.08;
  let base_x = degree_prefix + radical.width_pt + radical_base_gap;
  let bar_y = -(base_box.ascent_pt + math.radical_vertical_gap_pt);
  let base_descent = base_box.descent_pt;
  let mut result = MathBox::empty();
  let radical_width = radical.width_pt;
  let radical_descent = radical.descent_pt;
  result.append(radical, degree_prefix, base_descent - radical_descent);
  let base_width = base_box.width_pt;
  result.append(base_box, base_x, 0.0);
  result.items.push(MathPaintItem::Line {
    x1_pt: degree_prefix + radical_width * 0.72,
    y1_pt: bar_y,
    x2_pt: base_x + base_width,
    y2_pt: bar_y,
    width_pt: math.radical_rule_thickness_pt,
    color: style.color,
    opacity: style.opacity,
  });
  if let Some(degree) = degree_box {
    let raise = target_height * math.radical_degree_bottom_raise_percent / 100.0;
    let y = base_descent - raise - degree.descent_pt;
    result.append(degree, 0.0, y);
  }
  result.width_pt = base_x + base_width;
  result.ascent_pt = result
    .ascent_pt
    .max(-bar_y + math.radical_rule_thickness_pt + math.radical_extra_ascender_pt);
  result
}

fn layout_accent(
  base: &MathNode,
  character: &str,
  bottom: bool,
  script_level: u8,
  metrics: &mut TextMetrics,
) -> MathBox {
  let base_box = layout_node(base, script_level, metrics);
  let Some(style) = representative_style(base) else {
    return base_box;
  };
  let math = metrics.math_font_metrics(style);
  let accent = layout_stretched_symbol(
    character,
    style,
    script_level,
    style.font_size_pt,
    false,
    metrics,
  );
  let width = base_box.width_pt.max(accent.width_pt);
  let base_width = base_box.width_pt;
  let base_ascent = base_box.ascent_pt;
  let base_descent = base_box.descent_pt;
  let accent_width = accent.width_pt;
  let accent_y = if bottom {
    base_descent + math.underbar_vertical_gap_pt + accent.ascent_pt
  } else {
    -(base_ascent + math.overbar_vertical_gap_pt + accent.descent_pt)
  };
  let mut result = MathBox::empty();
  result.append(base_box, (width - base_width) / 2.0, 0.0);
  result.append(accent, (width - accent_width) / 2.0, accent_y);
  result.width_pt = width;
  result
}

fn layout_bar(
  base: &MathNode,
  bottom: bool,
  script_level: u8,
  metrics: &mut TextMetrics,
) -> MathBox {
  let mut result = layout_node(base, script_level, metrics);
  let Some(style) = representative_style(base) else {
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
  result.items.push(MathPaintItem::Line {
    x1_pt: 0.0,
    y1_pt: y,
    x2_pt: result.width_pt,
    y2_pt: y,
    width_pt: width,
    color: style.color,
    opacity: style.opacity,
  });
  if bottom {
    result.descent_pt = result.descent_pt.max(y + width / 2.0);
  } else {
    result.ascent_pt = result.ascent_pt.max(-y + width / 2.0);
  }
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
  script_level: u8,
  metrics: &mut TextMetrics,
) -> MathBox {
  let child = layout_node(base, script_level, metrics);
  let Some(style) = representative_style(base) else {
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
  let mut line = |x1: f32, y1: f32, x2: f32, y2: f32| {
    result.items.push(MathPaintItem::Line {
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
  result.width_pt = width;
  result.ascent_pt = -top + rule / 2.0;
  result.descent_pt = bottom + rule / 2.0;
  result
}

fn set_box_opacity(math_box: &mut MathBox, opacity: f32) {
  for item in &mut math_box.items {
    match item {
      MathPaintItem::Text {
        opacity: item_opacity,
        ..
      }
      | MathPaintItem::Line {
        opacity: item_opacity,
        ..
      } => *item_opacity *= opacity,
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
