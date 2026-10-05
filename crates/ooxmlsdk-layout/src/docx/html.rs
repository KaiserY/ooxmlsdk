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

mod css;
mod table;

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
  id: Option<String>,
  classes: Vec<String>,
  style: TextStyle,
  paragraph_format: ParagraphFormat,
  hyperlink_url: Option<String>,
  white_space: WhiteSpaceMode,
  hidden: bool,
  block: bool,
  explicit_paragraph: bool,
  saw_substantial_child: bool,
  box_style: table::BoxStyle,
  block_start: usize,
}

impl ElementContext {
  fn root() -> Self {
    Self {
      tag: String::new(),
      id: None,
      classes: Vec::new(),
      style: html_default_text_style(),
      paragraph_format: ParagraphFormat::default(),
      hyperlink_url: None,
      white_space: WhiteSpaceMode::Collapse,
      hidden: false,
      block: false,
      explicit_paragraph: false,
      saw_substantial_child: false,
      box_style: table::BoxStyle::default(),
      block_start: 0,
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
      if html_space(character) {
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
  stylesheet: css::Stylesheet,
  tables: Vec<table::TableBuilder>,
  table_default_style: TextStyle,
}

impl HtmlImporter {
  fn new(fixed_paragraph_auto_spacing: bool, stylesheet: css::Stylesheet) -> Self {
    let mut root = ElementContext::root();
    root.paragraph_format.additive_paragraph_spacing = fixed_paragraph_auto_spacing;
    Self {
      contexts: vec![root],
      current: None,
      blocks: Vec::new(),
      fixed_paragraph_auto_spacing,
      stylesheet,
      tables: Vec::new(),
      table_default_style: html_default_text_style(),
    }
  }

  fn finish_paragraph(&mut self) {
    if let Some(block) = self.current.take().and_then(ParagraphBuilder::into_block) {
      self.blocks.push(block);
    }
  }

  fn start_tag(&mut self, tag: &Tag) {
    let name = tag.name.as_ref();
    // HTML closes a paragraph at the next block start. In particular, editor
    // output containing <p><p> or <p><table> must not add an empty Word line.
    if html_block_tag(name)
      && let Some(position) = self.contexts.iter().rposition(|c| c.tag == "p")
    {
      if self.current.as_ref().is_some_and(|p| !p.has_text()) {
        self.current = None;
      }
      self.close_contexts(position);
    }
    // Optional table end tags are implied by the next cell/row/group, bounded
    // by the innermost table so nested tables cannot close their owner's cell.
    if matches!(name, "td" | "th" | "tr" | "thead" | "tbody" | "tfoot") {
      let table_start = self
        .contexts
        .iter()
        .rposition(|c| c.tag == "table")
        .unwrap_or(0);
      let row_start = matches!(name, "tr" | "thead" | "tbody" | "tfoot");
      let position = self
        .contexts
        .iter()
        .enumerate()
        .skip(table_start + 1)
        .find(|(_, c)| {
          if row_start {
            c.tag == "tr"
          } else {
            matches!(c.tag.as_str(), "td" | "th")
          }
        })
        .map(|(i, _)| i);
      if let Some(position) = position {
        self.close_contexts(position);
      }
    }
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

    let mut parent = self
      .contexts
      .last()
      .cloned()
      .unwrap_or_else(ElementContext::root);
    if name == "table" {
      // Word's HTML reader starts a table's character defaults from the body
      // tag rule, not the enclosing block/class/inline character formatting.
      // Explicit table/row/cell rules are then applied normally.
      parent.style = self.table_default_style.clone();
    }
    let mut context = element_context(&parent, tag, self.fixed_paragraph_auto_spacing);
    let declarations = self.stylesheet.declarations(
      &context,
      &self.contexts,
      attribute_value(&tag.attrs, "style"),
    );
    apply_css_declarations(&mut context, &declarations);
    context.box_style = table::BoxStyle::from_tag(tag, &declarations, context.style.font_size_pt);
    if name == "body" {
      let mut body_default = element_context(
        &ElementContext::root(),
        tag,
        self.fixed_paragraph_auto_spacing,
      );
      body_default.id = None;
      body_default.classes.clear();
      let declarations = self
        .stylesheet
        .declarations(&body_default, &self.contexts, None);
      apply_css_declarations(&mut body_default, &declarations);
      self.table_default_style = body_default.style;
    }
    if context.block {
      self.finish_paragraph();
    }
    if let Some(parent) = self.contexts.last_mut() {
      parent.saw_substantial_child = true;
    }
    if context.explicit_paragraph {
      self.current = Some(ParagraphBuilder::new(&context));
    }
    context.block_start = self.blocks.len();
    if !context.hidden {
      match name {
        "table" => self.tables.push(table::TableBuilder::new(tag, &context)),
        "tr" => {
          let header = self
            .contexts
            .iter()
            .rev()
            .take_while(|c| c.tag != "table")
            .any(|c| c.tag == "thead");
          if let Some(table) = self.tables.last_mut() {
            table.start_row(&context, header);
          }
        }
        "td" | "th" => {
          if let Some(table) = self.tables.last_mut() {
            table.start_cell(tag, &context, self.blocks.len());
          }
        }
        _ => {}
      }
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
    self.close_contexts(position);
  }

  fn close_contexts(&mut self, position: usize) {
    while self.contexts.len() > position {
      let context = self.contexts.pop().unwrap();
      if context.block {
        self.finish_paragraph();
      }
      if !context.hidden {
        if matches!(context.tag.as_str(), "body" | "div" | "section" | "article") {
          apply_container_margins(
            &mut self.blocks[context.block_start..],
            &context.paragraph_format,
          );
        }
        match context.tag.as_str() {
          "td" | "th" => {
            if let Some(table) = self.tables.last_mut() {
              table.finish_cell(&mut self.blocks);
            }
          }
          "tr" => {
            if let Some(table) = self.tables.last_mut() {
              table.finish_cell(&mut self.blocks);
              table.finish_row();
            }
          }
          "table" => {
            if let Some(mut table) = self.tables.pop() {
              table.finish_cell(&mut self.blocks);
              self.blocks.push(table.finish());
            }
          }
          _ => {}
        }
      }
    }
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
    if text.chars().any(|character| !html_space(character))
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
    self.close_contexts(1);
    self.finish_paragraph();
    self.blocks
  }
}

pub(super) fn import_blocks(source: &str, fixed_paragraph_auto_spacing: bool) -> Vec<Block> {
  let input = BufferQueue::default();
  input.push_back(StrTendril::from_slice(
    source.trim_start_matches('\u{feff}'),
  ));
  let tokenizer = Tokenizer::new(TokenCollector::default(), TokenizerOpts::default());
  let _ = tokenizer.feed(&input);
  tokenizer.end();
  let tokens = tokenizer.sink.tokens.into_inner();
  let stylesheet = css::Stylesheet::from_tokens(&tokens);
  let mut importer = HtmlImporter::new(fixed_paragraph_auto_spacing, stylesheet);
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
    // Word's Normal (Web) style explicitly disables pair kerning.
    kerning_minimum_size_pt: Some(f32::INFINITY),
    color_is_automatic: false,
    ..TextStyle::default()
  }
}

fn html_space(character: char) -> bool {
  matches!(character, ' ' | '\t' | '\r' | '\n' | '\u{c}')
}

fn apply_container_margins(blocks: &mut [Block], format: &ParagraphFormat) {
  // Word retains body/div margins as w:webSettings/w:divs. They surround the
  // division's children; they do not enter table cells. The lower margin also
  // closes a paragraph segment before a table. Native 0/20/40px body-bottom
  // controls move the table by max(body-bottom, paragraph-after), while the
  // preceding text and body-top controls keep that boundary unchanged.
  for block in blocks.iter_mut() {
    match block {
      Block::Paragraph(paragraph) => {
        paragraph.format.indent_left_pt += format.indent_left_pt;
        paragraph.format.indent_right_pt += format.indent_right_pt;
      }
      Block::Table(table) => table.indent_left_pt += format.indent_left_pt,
      Block::Frame(_) => {}
    }
  }
  if format.spacing_before_set
    && let Some(Block::Paragraph(paragraph)) = blocks.first_mut()
  {
    let before = paragraph
      .format
      .spacing_before_auto_pt
      .unwrap_or(paragraph.format.spacing_before_pt);
    set_explicit_spacing_before_format(&mut paragraph.format, before.max(format.spacing_before_pt));
  }
  if format.spacing_after_set {
    for index in 0..blocks.len() {
      if (index + 1 == blocks.len() || matches!(blocks.get(index + 1), Some(Block::Table(_))))
        && let Block::Paragraph(paragraph) = &mut blocks[index]
      {
        let after = paragraph
          .format
          .spacing_after_auto_pt
          .unwrap_or(paragraph.format.spacing_after_pt);
        paragraph.format.spacing_after_pt = after.max(format.spacing_after_pt);
        paragraph.format.spacing_after_set = true;
        paragraph.format.spacing_after_auto = Some(false);
        paragraph.format.spacing_after_auto_pt = None;
      }
    }
  }
}

fn css_color(value: &str) -> Option<super::model::RgbColor> {
  let value = value.trim();
  if let Some(rgb) = value.strip_prefix("rgb(").and_then(|v| v.strip_suffix(')')) {
    let components = rgb
      .split(',')
      .map(|part| {
        let part = part.trim();
        let number = if let Some(percent) = part.strip_suffix('%') {
          percent.parse::<f32>().ok()? * 255.0 / 100.0
        } else {
          part.parse::<f32>().ok()?
        };
        number
          .is_finite()
          .then_some(number.round().clamp(0.0, 255.0) as u8)
      })
      .collect::<Option<Vec<_>>>()?;
    if let [r, g, b] = components.as_slice() {
      return Some(super::model::RgbColor {
        r: *r,
        g: *g,
        b: *b,
      });
    }
    return None;
  }
  parse_vml_color(value)
}

fn element_context(
  parent: &ElementContext,
  tag: &Tag,
  fixed_paragraph_auto_spacing: bool,
) -> ElementContext {
  let name = tag.name.as_ref();
  let mut context = ElementContext {
    tag: name.to_string(),
    id: attribute_value(&tag.attrs, "id").map(str::to_string),
    classes: attribute_value(&tag.attrs, "class")
      .unwrap_or_default()
      .split_ascii_whitespace()
      .map(str::to_string)
      .collect(),
    style: parent.style.clone(),
    paragraph_format: ParagraphFormat::default(),
    hyperlink_url: parent.hyperlink_url.clone(),
    white_space: parent.white_space,
    hidden: parent.hidden || html_hidden_tag(name) || attribute_present(&tag.attrs, "hidden"),
    block: html_block_tag(name),
    explicit_paragraph: html_paragraph_tag(name),
    saw_substantial_child: false,
    box_style: table::BoxStyle::default(),
    block_start: 0,
  };
  // CSS text alignment and line height inherit; margins do not.
  context.paragraph_format.alignment = parent.paragraph_format.alignment;
  context.paragraph_format.justification = parent.paragraph_format.justification;
  context.paragraph_format.justification_set = parent.paragraph_format.justification_set;
  context.paragraph_format.bidi = parent.paragraph_format.bidi;
  context.paragraph_format.bidi_set = parent.paragraph_format.bidi_set;
  context.paragraph_format.line_height_pt = parent.paragraph_format.line_height_pt;
  context.paragraph_format.line_height_rule = parent.paragraph_format.line_height_rule;
  context.paragraph_format.line_height_set = parent.paragraph_format.line_height_set;
  context.paragraph_format.additive_paragraph_spacing = fixed_paragraph_auto_spacing;
  apply_html_element_defaults(&mut context, name, fixed_paragraph_auto_spacing);
  apply_html_presentational_attributes(&mut context, tag);
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
    "th" => {
      context.style.bold = true;
      apply_text_alignment(context, "center");
    }
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
      // HTML headings map to Word's heading styles: Heading 1 enables
      // kerning from 18pt, while Heading 2 through Heading 6 disable it.
      context.style.kerning_minimum_size_pt = Some(if name == "h1" { 18.0 } else { f32::INFINITY });
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

fn apply_css_declarations(context: &mut ElementContext, declarations: &[css::Declaration]) {
  // Resolve the winning font size against the inherited size, before em-based
  // margins/line heights. Repeated declarations are alternatives, not scales
  // applied successively to the previous losing declaration.
  let inherited_size = context.style.font_size_pt;
  for declaration in declarations {
    if declaration.name == "font-size"
      && let Some(size) = css_font_size_pt(&declaration.value, inherited_size)
    {
      set_html_font_size(context, size);
    }
  }
  for declaration in declarations {
    let name = &declaration.name;
    let value = declaration.value.as_str();
    match name.as_str() {
      "font-family" => apply_font_family(context, value),
      "font-size" => {}
      "line-height" => {
        use super::model::LineHeightRule;
        let (height, rule) = if value == "normal" {
          (Some(1.0), LineHeightRule::Auto)
        } else if let Ok(multiple) = value.parse::<f32>() {
          (Some(multiple), LineHeightRule::Auto)
        } else {
          (
            css_length_pt(value, context.style.font_size_pt),
            LineHeightRule::AtLeast,
          )
        };
        if let Some(height) = height.filter(|v| v.is_finite() && *v >= 0.0) {
          context.paragraph_format.line_height_pt = Some(height);
          context.paragraph_format.line_height_rule = rule;
          context.paragraph_format.line_height_set = true;
        }
      }
      "background-color" | "background" => {
        if let Some(color) = css_color(value) {
          if context.block {
            context.paragraph_format.shading = Some(super::model::ShadingPaint::Solid(color));
          } else {
            context.style.highlight = Some(color);
          }
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
        if let Some(color) = css_color(value) {
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
  set_explicit_spacing_before_format(&mut context.paragraph_format, points);
}

fn set_explicit_spacing_before_format(format: &mut ParagraphFormat, points: f32) {
  format.spacing_before_pt = points;
  format.spacing_before_set = true;
  format.spacing_before_auto = Some(false);
  format.spacing_before_auto_pt = None;
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
  vml_measure_to_points(&value).filter(|v| v.is_finite())
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

  fn table(block: &Block) -> &super::super::model::Table {
    let Block::Table(table) = block else {
      panic!("HTML table expected")
    };
    table
  }

  #[test]
  fn stylesheet_cascade_preserves_specificity_inheritance_and_important() {
    let blocks = import_blocks(
      r#"<head><style>
      /* p { font-size: 90pt } */
      body { font-family: Arial; font-size: 9pt; color: #333 }
      .editor { font-family: Georgia; font-size: 11pt }
      p, h1 { margin: 8px 0; line-height: 16pt }
      .editor > p.note { color: #008000 !important }
      #chosen { color: #0000ff }
      p:hover { font-size: 90pt }
      @media screen { p { font-size: 80pt } }
    </style></head><body class='editor'><p id='chosen' class='note' style='color:red'>A<span>B</span></p>
    <div><p style='font-size:200%'>C</p></div></body>"#,
      false,
    );
    let p = paragraph(&blocks[0]);
    assert_eq!(p.base_style.font_family.as_deref(), Some("Georgia"));
    assert_eq!(p.base_style.font_size_pt, 11.0);
    assert_eq!(
      p.base_style.color,
      super::super::model::RgbColor { r: 0, g: 128, b: 0 }
    );
    assert_eq!(p.format.spacing_before_pt, 6.0);
    assert_eq!(p.format.spacing_after_pt, 6.0);
    assert_eq!(p.format.line_height_pt, Some(16.0));
    assert_eq!(
      p.format.line_height_rule,
      super::super::model::LineHeightRule::AtLeast
    );
    assert_eq!(paragraph(&blocks[1]).base_style.font_size_pt, 22.0);
    assert_eq!(
      paragraph(&blocks[1]).base_style.color,
      super::super::model::RgbColor {
        r: 51,
        g: 51,
        b: 51
      }
    );
  }

  #[test]
  fn html_whitespace_preserves_nonbreaking_runs_and_css_backgrounds() {
    let blocks = import_blocks(
      "\u{feff}<p> A  B <span style='background-color:rgb(211, 211, 211)'>&nbsp;&nbsp;X</span> </p>",
      false,
    );
    assert_eq!(visible_paragraph_texts(&blocks), ["A B \u{a0}\u{a0}X"]);
    let p = paragraph(&blocks[0]);
    let Some(InlineItem::Text(run)) = p.inlines.last() else {
      panic!("colored run")
    };
    assert_eq!(
      run.style.highlight,
      Some(super::super::model::RgbColor {
        r: 211,
        g: 211,
        b: 211
      })
    );
  }

  #[test]
  fn html_table_retains_cells_widths_and_word_body_tag_defaults() {
    let blocks = import_blocks(
      r#"<head><style>
      body { font-family:Arial; font-size:9pt } .editor { font-family:Georgia; font-size:16pt }
      p { margin:8px 0 } td.override { font-size:14pt }
    </style></head><body class='editor'><p><p>body</p>
    <table border='1' cellpadding='0' cellspacing='0' width='200'>
      <tr><td style='width:80px;height:60px'><p>left</p></td>
          <td class='override' style='width:120px'><p>right</p><p>second</p></td></tr>
      <tr><td colspan='2'><p>merged</p></td></tr>
    </table><p>after</p></body>"#,
      false,
    );
    assert_eq!(blocks.len(), 3);
    assert_eq!(paragraph(&blocks[0]).base_style.font_size_pt, 16.0);
    let t = table(&blocks[1]);
    assert_eq!(t.rows.len(), 2);
    assert_eq!(t.column_widths_pt, [60.0, 90.0]);
    assert_eq!(t.preferred_width_pt, Some(150.0));
    assert_eq!(t.rows[0].height_pt, Some(45.0));
    assert_eq!(t.rows[1].cells[0].grid_span, 2);
    assert_eq!(t.rows[0].cells[0].blocks.len(), 1);
    assert_eq!(t.rows[0].cells[1].blocks.len(), 2);
    assert_eq!(
      paragraph(&t.rows[0].cells[0].blocks[0])
        .base_style
        .font_size_pt,
      9.0
    );
    assert_eq!(
      paragraph(&t.rows[0].cells[1].blocks[0])
        .base_style
        .font_size_pt,
      14.0
    );
    assert_eq!(
      t.rows[0].cells[0].margins,
      super::super::model::CellMargins::zero()
    );
    assert_eq!(t.rows[0].cells[0].borders.top.unwrap().width_pt, 0.75);
    assert_eq!(paragraph(&blocks[2]).base_style.font_size_pt, 16.0);
  }

  #[test]
  fn html_nested_tables_optional_end_tags_and_vertical_spans_keep_ownership() {
    let blocks = import_blocks(
      "<table><tr><td rowspan='2'>a<td><table><tr><td>nested</table><tr><td>b</table><p>after",
      false,
    );
    assert_eq!(blocks.len(), 2);
    let outer = table(&blocks[0]);
    assert_eq!(outer.rows.len(), 2);
    assert_eq!(outer.rows[0].cells.len(), 2);
    let nested = table(&outer.rows[0].cells[1].blocks[0]);
    assert_eq!(
      visible_paragraph_texts(&nested.rows[0].cells[0].blocks),
      ["nested"]
    );
    assert_eq!(outer.rows[1].cells.len(), 2);
    assert!(outer.rows[1].cells[0].vertical_merge_continue);
    assert!(outer.rows[1].cells[0].blocks.is_empty());
    assert_eq!(
      visible_paragraph_texts(&outer.rows[1].cells[1].blocks),
      ["b"]
    );
  }

  #[test]
  fn body_division_margins_surround_children_without_entering_table_cells() {
    let blocks = import_blocks(
      "<style>body{margin:20px}p{margin:10px 0}</style><body><p>first</p><table><tr><td><p>cell</p></table><div style='margin-left:2pt'><p>last</p></div></body>",
      false,
    );
    assert_eq!(paragraph(&blocks[0]).format.spacing_after_pt, 15.0);
    assert_eq!(paragraph(&blocks[0]).format.indent_left_pt, 15.0);
    assert_eq!(paragraph(&blocks[0]).format.indent_right_pt, 15.0);
    assert_eq!(paragraph(&blocks[0]).format.spacing_before_pt, 15.0);
    let t = table(&blocks[1]);
    assert_eq!(t.indent_left_pt, 15.0);
    assert_eq!(
      paragraph(&t.rows[0].cells[0].blocks[0])
        .format
        .indent_left_pt,
      0.0
    );
    assert_eq!(paragraph(&blocks[2]).format.indent_left_pt, 17.0);
    assert_eq!(paragraph(&blocks[2]).format.spacing_after_pt, 15.0);
  }

  #[test]
  fn html_heading_kerning_defaults_follow_word_styles() {
    let blocks = import_blocks(
      "<p>AV</p><h1 style='font-size:13pt'>AV</h1><h2 style='font-size:20pt'>AV</h2><h6><span>AV</span></h6>",
      false,
    );
    for (block, threshold) in blocks
      .iter()
      .zip([f32::INFINITY, 18.0, f32::INFINITY, f32::INFINITY])
    {
      let paragraph = paragraph(block);
      assert_eq!(
        paragraph.base_style.kerning_minimum_size_pt,
        Some(threshold)
      );
      for inline in &paragraph.inlines {
        if let InlineItem::Text(run) = inline {
          assert_eq!(run.style.kerning_minimum_size_pt, Some(threshold));
        }
      }
    }
    assert_eq!(blocks.len(), 4);
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
