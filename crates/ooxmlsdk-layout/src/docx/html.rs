use std::cell::RefCell;
use std::sync::Arc;

use html5ever::interface::Attribute;
use html5ever::tendril::StrTendril;
use html5ever::tokenizer::{
  BufferQueue, Tag, TagKind, Token, TokenSink, TokenSinkResult, Tokenizer, TokenizerOpts,
};

use super::model::{
  Block, InlineItem, Paragraph, ParagraphAdjust, ParagraphAlignment, ParagraphFormat, TextRun,
  TextStyle,
};
use super::{parse_vml_color, vml_measure_to_points};

const HTML_DEFAULT_FONT_FAMILY: &str = "Times New Roman";
const HTML_DEFAULT_FONT_SIZE_PT: f32 = 12.0;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum WhiteSpaceMode {
  #[default]
  Collapse,
  Preserve,
  PreserveWrap,
}

#[derive(Clone, Debug)]
struct ElementContext {
  tag: String,
  style: TextStyle,
  paragraph_format: ParagraphFormat,
  hyperlink_url: Option<String>,
  white_space: WhiteSpaceMode,
  hidden: bool,
  block: bool,
  explicit_paragraph: bool,
  saw_substantial_child: bool,
}

impl ElementContext {
  fn root() -> Self {
    Self {
      tag: String::new(),
      style: html_default_text_style(),
      paragraph_format: ParagraphFormat::default(),
      hyperlink_url: None,
      white_space: WhiteSpaceMode::Collapse,
      hidden: false,
      block: false,
      explicit_paragraph: false,
      saw_substantial_child: false,
    }
  }
}

#[derive(Default)]
struct TokenCollector {
  tokens: RefCell<Vec<Token>>,
}

impl TokenSink for TokenCollector {
  type Handle = ();

  fn process_token(&self, token: Token, _line_number: u64) -> TokenSinkResult<Self::Handle> {
    self.tokens.borrow_mut().push(token);
    TokenSinkResult::Continue
  }
}

struct ParagraphBuilder {
  inlines: Vec<InlineItem>,
  base_style: TextStyle,
  format: ParagraphFormat,
  explicit: bool,
  pending_space: Option<PendingSpace>,
}

struct PendingSpace {
  style: TextStyle,
  hyperlink_url: Option<String>,
}

impl ParagraphBuilder {
  fn new(context: &ElementContext) -> Self {
    Self {
      inlines: Vec::new(),
      base_style: context.style.clone(),
      format: context.paragraph_format.clone(),
      explicit: context.explicit_paragraph,
      pending_space: None,
    }
  }

  fn has_text(&self) -> bool {
    self
      .inlines
      .iter()
      .any(|inline| matches!(inline, InlineItem::Text(run) if !run.text.is_empty()))
  }

  fn ends_with_line_break(&self) -> bool {
    self.inlines.iter().rev().find_map(|inline| match inline {
      InlineItem::Text(run) => run.text.chars().next_back(),
      _ => None,
    }) == Some('\n')
  }

  fn append_text(&mut self, text: &str, context: &ElementContext) {
    self.append_styled_text(text, &context.style, context.hyperlink_url.as_ref());
  }

  fn append_styled_text(&mut self, text: &str, style: &TextStyle, hyperlink_url: Option<&String>) {
    if text.is_empty() {
      return;
    }
    if let Some(InlineItem::Text(previous)) = self.inlines.last_mut()
      && previous.style == *style
      && previous.hyperlink_url.as_ref() == hyperlink_url
    {
      previous.text.push_str(text);
      return;
    }
    self.inlines.push(InlineItem::Text(TextRun {
      text: text.to_string(),
      style: style.clone(),
      hyperlink_url: hyperlink_url.cloned(),
      dynamic_field: None,
      style_ref_keys: Vec::new(),
      style_ref_text: None,
      style_ref_numbering_text: None,
      preserve_text_portion: false,
    }));
  }

  fn append_collapsed(&mut self, text: &str, context: &ElementContext) {
    let mut word = String::new();
    for character in text.chars() {
      if character.is_whitespace() {
        if !word.is_empty() {
          self.append_text(&word, context);
          word.clear();
        }
        self.pending_space.get_or_insert_with(|| PendingSpace {
          style: context.style.clone(),
          hyperlink_url: context.hyperlink_url.clone(),
        });
        continue;
      }
      if let Some(pending) = self.pending_space.take()
        && self.has_text()
        && !self.ends_with_line_break()
      {
        self.append_styled_text(" ", &pending.style, pending.hyperlink_url.as_ref());
      }
      word.push(character);
    }
    if !word.is_empty() {
      self.append_text(&word, context);
    }
  }

  fn append_characters(&mut self, text: &str, context: &ElementContext) {
    match context.white_space {
      WhiteSpaceMode::Collapse => self.append_collapsed(text, context),
      WhiteSpaceMode::Preserve | WhiteSpaceMode::PreserveWrap => {
        self.pending_space = None;
        self.append_text(text, context);
      }
    }
  }

  fn append_line_break(&mut self, context: &ElementContext) {
    self.pending_space = None;
    self.append_text("\n", context);
  }

  fn into_block(self) -> Option<Block> {
    if !self.explicit && !self.has_text() {
      return None;
    }
    Some(Block::paragraph(Paragraph {
      inlines: self.inlines,
      field_events: Vec::new(),
      footnote_reference_ids: Vec::new(),
      endnote_reference_ids: Vec::new(),
      starts_after_last_rendered_page_break: false,
      base_style: self.base_style,
      #[cfg(test)]
      runs: Vec::new(),
      format: Box::new(self.format),
      style_ref_keys: Vec::new(),
      style_ref_text: None,
      style_ref_numbering_text: None,
      list_label: None,
      list_label_image: None,
      list_label_style: TextStyle::default(),
      list_label_hyperlink_url: None,
      list_label_tab_stop_pt: None,
    }))
  }
}

struct HtmlImporter {
  contexts: Vec<ElementContext>,
  current: Option<ParagraphBuilder>,
  blocks: Vec<Block>,
  fixed_paragraph_auto_spacing: bool,
}

impl HtmlImporter {
  fn new(fixed_paragraph_auto_spacing: bool) -> Self {
    Self {
      contexts: vec![ElementContext::root()],
      current: None,
      blocks: Vec::new(),
      fixed_paragraph_auto_spacing,
    }
  }

  fn finish_paragraph(&mut self) {
    if let Some(block) = self.current.take().and_then(ParagraphBuilder::into_block) {
      self.blocks.push(block);
    }
  }

  fn start_tag(&mut self, tag: &Tag) {
    let name = tag.name.as_ref();
    if name.eq_ignore_ascii_case("br") {
      if !self.contexts.last().is_some_and(|context| context.hidden) {
        let context = self
          .contexts
          .last()
          .cloned()
          .unwrap_or_else(ElementContext::root);
        let paragraph = self
          .current
          .get_or_insert_with(|| ParagraphBuilder::new(&context));
        paragraph.append_line_break(&context);
      }
      return;
    }

    let parent = self
      .contexts
      .last()
      .cloned()
      .unwrap_or_else(ElementContext::root);
    let context = element_context(&parent, tag, self.fixed_paragraph_auto_spacing);
    if context.block {
      self.finish_paragraph();
    }
    if let Some(parent) = self.contexts.last_mut() {
      parent.saw_substantial_child = true;
    }
    if context.explicit_paragraph {
      self.current = Some(ParagraphBuilder::new(&context));
    }
    let void = html_void_tag(name) || tag.self_closing;
    if !void {
      self.contexts.push(context);
    } else if context.block {
      self.finish_paragraph();
    }
  }

  fn end_tag(&mut self, tag: &Tag) {
    let name = tag.name.as_ref();
    let Some(position) = self
      .contexts
      .iter()
      .rposition(|context| context.tag.eq_ignore_ascii_case(name))
    else {
      return;
    };
    if self.contexts[position..]
      .iter()
      .any(|context| context.block)
    {
      self.finish_paragraph();
    }
    self.contexts.truncate(position);
    if self.contexts.is_empty() {
      self.contexts.push(ElementContext::root());
    }
  }

  fn characters(&mut self, text: &str) {
    let context = self
      .contexts
      .last()
      .cloned()
      .unwrap_or_else(ElementContext::root);
    if context.hidden {
      return;
    }
    if text.chars().any(|character| !character.is_whitespace())
      && let Some(parent) = self.contexts.last_mut()
    {
      parent.saw_substantial_child = true;
    }
    let paragraph = self
      .current
      .get_or_insert_with(|| ParagraphBuilder::new(&context));
    paragraph.append_characters(text, &context);
  }

  fn finish(mut self) -> Vec<Block> {
    self.finish_paragraph();
    self.blocks
  }
}

pub(super) fn import_blocks(source: &str, fixed_paragraph_auto_spacing: bool) -> Vec<Block> {
  let input = BufferQueue::default();
  input.push_back(StrTendril::from_slice(source));
  let tokenizer = Tokenizer::new(TokenCollector::default(), TokenizerOpts::default());
  let _ = tokenizer.feed(&input);
  tokenizer.end();
  let tokens = tokenizer.sink.tokens.into_inner();
  let mut importer = HtmlImporter::new(fixed_paragraph_auto_spacing);
  for token in tokens {
    match token {
      Token::TagToken(tag) if tag.kind == TagKind::StartTag => importer.start_tag(&tag),
      Token::TagToken(tag) => importer.end_tag(&tag),
      Token::CharacterTokens(text) => importer.characters(&text),
      _ => {}
    }
  }
  importer.finish()
}

fn html_default_text_style() -> TextStyle {
  let family: Arc<str> = Arc::from(HTML_DEFAULT_FONT_FAMILY);
  TextStyle {
    font_family: Some(family.clone()),
    east_asia_font_family: Some(family.clone()),
    complex_font_family: Some(family),
    font_size_pt: HTML_DEFAULT_FONT_SIZE_PT,
    complex_font_size_pt: Some(HTML_DEFAULT_FONT_SIZE_PT),
    wordprocessingml_font_slots: true,
    color_is_automatic: false,
    ..TextStyle::default()
  }
}

fn element_context(
  parent: &ElementContext,
  tag: &Tag,
  fixed_paragraph_auto_spacing: bool,
) -> ElementContext {
  let name = tag.name.as_ref();
  let mut context = ElementContext {
    tag: name.to_string(),
    style: parent.style.clone(),
    paragraph_format: ParagraphFormat::default(),
    hyperlink_url: parent.hyperlink_url.clone(),
    white_space: parent.white_space,
    hidden: parent.hidden || html_hidden_tag(name) || attribute_present(&tag.attrs, "hidden"),
    block: html_block_tag(name),
    explicit_paragraph: html_paragraph_tag(name),
    saw_substantial_child: false,
  };
  apply_html_element_defaults(&mut context, name, fixed_paragraph_auto_spacing);
  apply_html_presentational_attributes(&mut context, tag);
  if let Some(style) = attribute_value(&tag.attrs, "style") {
    apply_css_declarations(&mut context, style);
  }
  context
}

fn apply_html_element_defaults(
  context: &mut ElementContext,
  name: &str,
  fixed_paragraph_auto_spacing: bool,
) {
  if matches!(
    name.to_ascii_lowercase().as_str(),
    "code" | "kbd" | "samp" | "tt" | "pre" | "listing" | "plaintext" | "xmp"
  ) {
    set_html_font_family(context, "Courier New");
  }
  match name.to_ascii_lowercase().as_str() {
    "b" | "strong" => context.style.bold = true,
    "i" | "em" | "cite" | "var" | "address" => context.style.italic = true,
    "u" | "ins" => context.style.underline = true,
    "s" | "strike" | "del" => context.style.strikethrough = true,
    "small" => set_html_font_size(context, context.style.font_size_pt * 0.8),
    "big" => set_html_font_size(context, context.style.font_size_pt * 1.2),
    "a" => {
      context.style.underline = true;
      context.style.color = super::RgbColor { r: 0, g: 0, b: 238 };
      context.style.color_is_automatic = false;
    }
    "p" => {
      // ECMA-376 Part 1 §17.3.1.33 represents an HTML paragraph's
      // application-determined default margins with beforeAutospacing and
      // afterAutospacing. Word's SpaceBeforeAuto/SpaceAfterAuto API likewise
      // sets both properties when it imports HTML without an explicit CSS
      // margin. Part 4 §14.8.3.15 changes the resolved values from 14/14pt
      // to the compatibility-fixed 5/10pt pair; keep the automatic state so
      // explicit CSS can override either side independently below.
      context.paragraph_format.spacing_before_auto = Some(true);
      context.paragraph_format.spacing_after_auto = Some(true);
      if fixed_paragraph_auto_spacing {
        context.paragraph_format.spacing_before_auto_pt =
          Some(super::OFFICE_FIXED_AUTOMATIC_PARAGRAPH_BEFORE_PT);
        context.paragraph_format.spacing_after_auto_pt =
          Some(super::OFFICE_FIXED_AUTOMATIC_PARAGRAPH_AFTER_PT);
      } else {
        context.paragraph_format.spacing_before_auto_pt =
          Some(super::OFFICE_AUTOMATIC_PARAGRAPH_SPACING_PT);
        context.paragraph_format.spacing_after_auto_pt =
          Some(super::OFFICE_AUTOMATIC_PARAGRAPH_SPACING_PT);
      }
    }
    "blockquote" | "figure" | "listing" | "plaintext" | "pre" | "xmp" => {
      let margin = context.style.font_size_pt;
      context.paragraph_format.spacing_before_pt = margin;
      context.paragraph_format.spacing_after_pt = margin;
      context.paragraph_format.spacing_before_set = true;
      context.paragraph_format.spacing_after_set = true;
    }
    "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => {
      let (font_scale, margin_scale) = match name.as_bytes()[1] {
        b'1' => (2.0, 0.67),
        b'2' => (1.5, 0.83),
        b'3' => (1.17, 1.0),
        b'4' => (1.0, 1.33),
        b'5' => (0.83, 1.67),
        _ => (0.67, 2.33),
      };
      set_html_font_size(context, context.style.font_size_pt * font_scale);
      context.style.bold = true;
      let margin = context.style.font_size_pt * margin_scale;
      context.paragraph_format.spacing_before_pt = margin;
      context.paragraph_format.spacing_after_pt = margin;
      context.paragraph_format.spacing_before_set = true;
      context.paragraph_format.spacing_after_set = true;
    }
    _ => {}
  }
  if matches!(name.to_ascii_lowercase().as_str(), "blockquote" | "figure") {
    context.paragraph_format.indent_left_pt += 30.0;
    context.paragraph_format.indent_right_pt += 30.0;
    context.paragraph_format.indent_left_set = true;
    context.paragraph_format.indent_right_set = true;
  }
  if matches!(
    name.to_ascii_lowercase().as_str(),
    "pre" | "listing" | "plaintext" | "xmp"
  ) {
    context.white_space = WhiteSpaceMode::Preserve;
  }
}

fn apply_html_presentational_attributes(context: &mut ElementContext, tag: &Tag) {
  let name = tag.name.as_ref();
  if let Some(direction) = attribute_value(&tag.attrs, "dir") {
    apply_direction(context, direction);
  }
  if let Some(alignment) = attribute_value(&tag.attrs, "align") {
    apply_text_alignment(context, alignment);
  }
  if name.eq_ignore_ascii_case("a") {
    context.hyperlink_url = attribute_value(&tag.attrs, "href").map(str::to_string);
  }
  if name.eq_ignore_ascii_case("font") {
    if let Some(family) = attribute_value(&tag.attrs, "face") {
      apply_font_family(context, family);
    }
    if let Some(color) = attribute_value(&tag.attrs, "color").and_then(parse_vml_color) {
      context.style.color = color;
      context.style.color_is_automatic = false;
    }
    if let Some(size) = attribute_value(&tag.attrs, "size").and_then(html_legacy_font_size_pt) {
      set_html_font_size(context, size);
    }
  }
  if name.eq_ignore_ascii_case("body")
    && let Some(color) = attribute_value(&tag.attrs, "text").and_then(parse_vml_color)
  {
    context.style.color = color;
    context.style.color_is_automatic = false;
  }
}

fn apply_css_declarations(context: &mut ElementContext, declarations: &str) {
  for declaration in declarations.split(';') {
    let Some((name, value)) = declaration.split_once(':') else {
      continue;
    };
    let name = name.trim().to_ascii_lowercase();
    let value = value.trim().trim_end_matches("!important").trim();
    match name.as_str() {
      "font-family" => apply_font_family(context, value),
      "font-size" => {
        if let Some(size) = css_font_size_pt(value, context.style.font_size_pt) {
          set_html_font_size(context, size);
        }
      }
      "font-weight" => {
        context.style.bold = matches!(
          value.to_ascii_lowercase().as_str(),
          "bold" | "bolder" | "600" | "700" | "800" | "900"
        )
      }
      "font-style" => {
        context.style.italic = matches!(value.to_ascii_lowercase().as_str(), "italic" | "oblique")
      }
      "color" => {
        if let Some(color) = parse_vml_color(value) {
          context.style.color = color;
          context.style.color_is_automatic = false;
        }
      }
      "text-decoration" | "text-decoration-line" => apply_text_decoration(context, value),
      "text-align" => apply_text_alignment(context, value),
      "direction" => apply_direction(context, value),
      "white-space" => {
        context.white_space = match value.to_ascii_lowercase().as_str() {
          "pre" | "nowrap" => WhiteSpaceMode::Preserve,
          "pre-wrap" | "break-spaces" => WhiteSpaceMode::PreserveWrap,
          _ => WhiteSpaceMode::Collapse,
        }
      }
      "display" if value.eq_ignore_ascii_case("none") => context.hidden = true,
      "display" if value.eq_ignore_ascii_case("block") => context.block = true,
      "margin" => apply_css_margin_shorthand(context, value),
      "margin-block" => apply_css_margin_block(context, value),
      "margin-top" | "margin-block-start" => {
        if let Some(points) = css_length_pt(value, context.style.font_size_pt) {
          set_explicit_spacing_before(context, points);
        }
      }
      "margin-bottom" | "margin-block-end" => {
        if let Some(points) = css_length_pt(value, context.style.font_size_pt) {
          set_explicit_spacing_after(context, points);
        }
      }
      "margin-left" | "margin-inline-start" => {
        if let Some(points) = css_length_pt(value, context.style.font_size_pt) {
          context.paragraph_format.indent_left_pt = points;
          context.paragraph_format.indent_left_set = true;
        }
      }
      "margin-right" | "margin-inline-end" => {
        if let Some(points) = css_length_pt(value, context.style.font_size_pt) {
          context.paragraph_format.indent_right_pt = points;
          context.paragraph_format.indent_right_set = true;
        }
      }
      _ => {}
    }
  }
}

fn apply_css_margin_shorthand(context: &mut ElementContext, value: &str) {
  let values = value
    .split_whitespace()
    .filter_map(|value| css_length_pt(value, context.style.font_size_pt))
    .collect::<Vec<_>>();
  let (top, right, bottom, left) = match values.as_slice() {
    [all] => (*all, *all, *all, *all),
    [vertical, horizontal] => (*vertical, *horizontal, *vertical, *horizontal),
    [top, horizontal, bottom] => (*top, *horizontal, *bottom, *horizontal),
    [top, right, bottom, left] => (*top, *right, *bottom, *left),
    _ => return,
  };
  set_explicit_spacing_before(context, top);
  set_explicit_spacing_after(context, bottom);
  context.paragraph_format.indent_left_pt = left;
  context.paragraph_format.indent_right_pt = right;
  context.paragraph_format.indent_left_set = true;
  context.paragraph_format.indent_right_set = true;
}

fn apply_css_margin_block(context: &mut ElementContext, value: &str) {
  let values = value
    .split_whitespace()
    .filter_map(|value| css_length_pt(value, context.style.font_size_pt))
    .collect::<Vec<_>>();
  let (before, after) = match values.as_slice() {
    [both] => (*both, *both),
    [before, after] => (*before, *after),
    _ => return,
  };
  set_explicit_spacing_before(context, before);
  set_explicit_spacing_after(context, after);
}

fn set_explicit_spacing_before(context: &mut ElementContext, points: f32) {
  context.paragraph_format.spacing_before_pt = points;
  context.paragraph_format.spacing_before_set = true;
  context.paragraph_format.spacing_before_auto = Some(false);
  context.paragraph_format.spacing_before_auto_pt = None;
}

fn set_explicit_spacing_after(context: &mut ElementContext, points: f32) {
  context.paragraph_format.spacing_after_pt = points;
  context.paragraph_format.spacing_after_set = true;
  context.paragraph_format.spacing_after_auto = Some(false);
  context.paragraph_format.spacing_after_auto_pt = None;
}

fn apply_text_decoration(context: &mut ElementContext, value: &str) {
  let value = value.to_ascii_lowercase();
  if value.split_whitespace().any(|part| part == "none") {
    context.style.underline = false;
    context.style.strikethrough = false;
    return;
  }
  context.style.underline |= value.split_whitespace().any(|part| part == "underline");
  context.style.strikethrough |= value.split_whitespace().any(|part| part == "line-through");
}

fn apply_text_alignment(context: &mut ElementContext, value: &str) {
  let alignment = match value.trim().to_ascii_lowercase().as_str() {
    "center" => ParagraphAlignment::Center,
    "right" | "end" => ParagraphAlignment::Right,
    "justify" => ParagraphAlignment::Justify,
    "left" | "start" => ParagraphAlignment::Left,
    _ => return,
  };
  context.paragraph_format.alignment = alignment;
  context.paragraph_format.justification.adjust = match alignment {
    ParagraphAlignment::Center => ParagraphAdjust::Center,
    ParagraphAlignment::Right => ParagraphAdjust::Right,
    ParagraphAlignment::Justify => ParagraphAdjust::Block,
    ParagraphAlignment::Left => ParagraphAdjust::Left,
  };
  context.paragraph_format.justification_set = true;
}

fn apply_direction(context: &mut ElementContext, value: &str) {
  match value.trim().to_ascii_lowercase().as_str() {
    "rtl" => {
      context.paragraph_format.bidi = true;
      context.paragraph_format.bidi_set = true;
      context.style.right_to_left = Some(true);
    }
    "ltr" => {
      context.paragraph_format.bidi = false;
      context.paragraph_format.bidi_set = true;
      context.style.right_to_left = Some(false);
    }
    _ => {}
  }
}

fn apply_font_family(context: &mut ElementContext, value: &str) {
  let Some(family) = value
    .split(',')
    .map(|family| family.trim().trim_matches(['\'', '"']))
    .find(|family| !family.is_empty())
  else {
    return;
  };
  let family = match family.to_ascii_lowercase().as_str() {
    "serif" => "Times New Roman",
    "sans-serif" => "Arial",
    "monospace" => "Courier New",
    _ => family,
  };
  set_html_font_family(context, family);
}

fn set_html_font_family(context: &mut ElementContext, family: &str) {
  let family: Arc<str> = Arc::from(family);
  context.style.font_family = Some(family.clone());
  context.style.east_asia_font_family = Some(family.clone());
  context.style.complex_font_family = Some(family);
}

fn set_html_font_size(context: &mut ElementContext, size: f32) {
  let size = size.max(1.0);
  context.style.font_size_pt = size;
  context.style.complex_font_size_pt = Some(size);
}

fn css_font_size_pt(value: &str, inherited: f32) -> Option<f32> {
  match value.trim().to_ascii_lowercase().as_str() {
    "xx-small" => Some(7.0),
    "x-small" => Some(9.0),
    "small" => Some(10.0),
    "medium" => Some(12.0),
    "large" => Some(14.0),
    "x-large" => Some(18.0),
    "xx-large" => Some(24.0),
    "smaller" => Some(inherited * 0.8),
    "larger" => Some(inherited * 1.2),
    _ => css_length_pt(value, inherited),
  }
}

fn css_length_pt(value: &str, em_size: f32) -> Option<f32> {
  let value = value.trim().to_ascii_lowercase();
  if let Some(number) = value.strip_suffix("rem") {
    return number
      .trim()
      .parse::<f32>()
      .ok()
      .map(|number| number * HTML_DEFAULT_FONT_SIZE_PT);
  }
  if let Some(number) = value.strip_suffix("em") {
    return number
      .trim()
      .parse::<f32>()
      .ok()
      .map(|number| number * em_size);
  }
  if let Some(number) = value.strip_suffix('%') {
    return number
      .trim()
      .parse::<f32>()
      .ok()
      .map(|number| number * em_size / 100.0);
  }
  vml_measure_to_points(&value)
}

fn html_legacy_font_size_pt(value: &str) -> Option<f32> {
  const SIZES: [f32; 7] = [7.0, 10.0, 12.0, 14.0, 18.0, 24.0, 36.0];
  let value = value.trim();
  let index = value
    .parse::<usize>()
    .ok()
    .map(|value| value.clamp(1, 7) - 1)?;
  Some(SIZES[index])
}

fn attribute_value<'a>(attributes: &'a [Attribute], name: &str) -> Option<&'a str> {
  attributes
    .iter()
    .find(|attribute| attribute.name.local.as_ref().eq_ignore_ascii_case(name))
    .map(|attribute| attribute.value.as_ref())
}

fn attribute_present(attributes: &[Attribute], name: &str) -> bool {
  attribute_value(attributes, name).is_some()
}

fn html_block_tag(name: &str) -> bool {
  matches!(
    name.to_ascii_lowercase().as_str(),
    "address"
      | "article"
      | "aside"
      | "blockquote"
      | "body"
      | "dd"
      | "div"
      | "dl"
      | "dt"
      | "figcaption"
      | "figure"
      | "footer"
      | "form"
      | "h1"
      | "h2"
      | "h3"
      | "h4"
      | "h5"
      | "h6"
      | "header"
      | "html"
      | "li"
      | "listing"
      | "main"
      | "nav"
      | "ol"
      | "p"
      | "plaintext"
      | "pre"
      | "section"
      | "table"
      | "tbody"
      | "td"
      | "tfoot"
      | "th"
      | "thead"
      | "tr"
      | "ul"
      | "xmp"
  )
}

fn html_paragraph_tag(name: &str) -> bool {
  matches!(
    name.to_ascii_lowercase().as_str(),
    "address"
      | "blockquote"
      | "dd"
      | "dt"
      | "figcaption"
      | "h1"
      | "h2"
      | "h3"
      | "h4"
      | "h5"
      | "h6"
      | "li"
      | "listing"
      | "p"
      | "plaintext"
      | "pre"
      | "td"
      | "th"
      | "xmp"
  )
}

fn html_hidden_tag(name: &str) -> bool {
  matches!(
    name.to_ascii_lowercase().as_str(),
    "head" | "noscript" | "script" | "style" | "template" | "title"
  )
}

fn html_void_tag(name: &str) -> bool {
  matches!(
    name.to_ascii_lowercase().as_str(),
    "area"
      | "base"
      | "br"
      | "col"
      | "embed"
      | "hr"
      | "img"
      | "input"
      | "link"
      | "meta"
      | "param"
      | "source"
      | "track"
      | "wbr"
  )
}

#[cfg(test)]
pub(super) fn visible_paragraph_texts(blocks: &[Block]) -> Vec<String> {
  blocks
    .iter()
    .filter_map(|block| {
      let Block::Paragraph(paragraph) = block else {
        return None;
      };
      Some(
        paragraph
          .inlines
          .iter()
          .filter_map(|inline| match inline {
            InlineItem::Text(run) => Some(run.text.as_str()),
            _ => None,
          })
          .collect(),
      )
    })
    .collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  fn paragraph(block: &Block) -> &Paragraph {
    let Block::Paragraph(paragraph) = block else {
      panic!("HTML block is not a paragraph")
    };
    paragraph
  }

  #[test]
  fn html_source_defaults_do_not_inherit_word_doc_defaults() {
    let blocks = import_blocks("<html><body><p>HTML AltChunk</p></body></html>", false);
    let paragraph = paragraph(&blocks[0]);
    let InlineItem::Text(run) = &paragraph.inlines[0] else {
      panic!("HTML text run")
    };

    assert_eq!(run.style.font_family.as_deref(), Some("Times New Roman"));
    assert_eq!(run.style.font_size_pt, 12.0);
    assert_eq!(paragraph.format.spacing_before_auto, Some(true));
    assert_eq!(
      paragraph.format.spacing_before_auto_pt,
      Some(super::super::OFFICE_AUTOMATIC_PARAGRAPH_SPACING_PT)
    );
    assert_eq!(paragraph.format.spacing_after_auto, Some(true));
    assert_eq!(
      paragraph.format.spacing_after_auto_pt,
      Some(super::super::OFFICE_AUTOMATIC_PARAGRAPH_SPACING_PT)
    );
  }

  #[test]
  fn compatibility_setting_resolves_html_auto_spacing_to_fixed_pair() {
    let blocks = import_blocks("<!doctype html><html><body><p>text</p></body></html>", true);
    let paragraph = paragraph(&blocks[0]);

    assert_eq!(paragraph.format.spacing_before_auto, Some(true));
    assert_eq!(
      paragraph.format.spacing_before_auto_pt,
      Some(super::super::OFFICE_FIXED_AUTOMATIC_PARAGRAPH_BEFORE_PT)
    );
    assert_eq!(paragraph.format.spacing_after_auto, Some(true));
    assert_eq!(
      paragraph.format.spacing_after_auto_pt,
      Some(super::super::OFFICE_FIXED_AUTOMATIC_PARAGRAPH_AFTER_PT)
    );
  }

  #[test]
  fn explicit_css_margins_override_html_auto_spacing_per_side() {
    let blocks = import_blocks(
      "<html><body><p style='margin-top: 3pt; margin-bottom: 7pt'>text</p></body></html>",
      false,
    );
    let paragraph = paragraph(&blocks[0]);

    assert_eq!(paragraph.format.spacing_before_auto, Some(false));
    assert_eq!(paragraph.format.spacing_before_auto_pt, None);
    assert_eq!(paragraph.format.spacing_before_pt, 3.0);
    assert_eq!(paragraph.format.spacing_after_auto, Some(false));
    assert_eq!(paragraph.format.spacing_after_auto_pt, None);
    assert_eq!(paragraph.format.spacing_after_pt, 7.0);
  }

  #[test]
  fn semantic_and_inline_css_styles_survive_html_import() {
    let blocks = import_blocks(
      "<!doctype html><html><body><p><strong>bold</strong> <span style='font-family: Arial; font-size: 10pt; color: #804000'>styled</span><br>line</p></body></html>",
      false,
    );
    let paragraph = paragraph(&blocks[0]);
    let runs = paragraph
      .inlines
      .iter()
      .filter_map(|inline| match inline {
        InlineItem::Text(run) => Some(run),
        _ => None,
      })
      .collect::<Vec<_>>();

    assert!(runs[0].style.bold);
    assert_eq!(
      runs[1].style.font_family.as_deref(),
      Some("Times New Roman")
    );
    assert_eq!(runs[2].style.font_family.as_deref(), Some("Arial"));
    assert_eq!(runs[2].style.font_size_pt, 10.0);
    assert_eq!(
      runs[2].style.color,
      super::super::RgbColor {
        r: 128,
        g: 64,
        b: 0
      }
    );
    assert!(visible_paragraph_texts(&blocks)[0].contains("\nline"));
  }
}
