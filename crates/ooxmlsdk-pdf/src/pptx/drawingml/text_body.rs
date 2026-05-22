use ooxmlsdk::schemas::{
  schemas_openxmlformats_org_drawingml_2006_main as a,
  schemas_openxmlformats_org_presentationml_2006_main as p,
};

use super::text_list_style::{TextListParagraphStyle, TextListParagraphStyleRef, TextListStyle};

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct TextBody {
  pub(crate) has_body_properties: bool,
  pub(crate) has_noninherited_body_properties: bool,
  pub(crate) body_properties: Option<Box<a::BodyProperties>>,
  pub(crate) has_list_style: bool,
  pub(crate) list_style: Option<TextListStyle>,
  pub(crate) paragraphs: Vec<TextParagraph>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct TextParagraph {
  pub(crate) level: Option<u8>,
  pub(crate) paragraph_properties: Option<Box<a::ParagraphProperties>>,
  pub(crate) end_paragraph_run_properties: Option<Box<a::EndParagraphRunProperties>>,
  pub(crate) master_paragraph_style: Option<TextListParagraphStyle>,
  pub(crate) text_paragraph_style: Option<TextListParagraphStyle>,
  pub(crate) runs: Vec<TextRun>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct TextRun {
  pub(crate) text: String,
  pub(crate) kind: TextRunKind,
  pub(crate) field_type: Option<String>,
  pub(crate) run_properties: Option<Box<a::RunProperties>>,
  pub(crate) field_paragraph_properties: Option<Box<a::ParagraphProperties>>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) enum TextRunKind {
  #[default]
  Run,
  Break,
  Field,
  Math,
}

impl TextBody {
  pub(crate) fn from_dml(source: &a::TextBody) -> Self {
    // Source: LibreOffice oox/source/drawingml/textbodycontext.cxx
    // TextBodyContext owns bodyPr, lstStyle, and paragraph import for both
    // shape text and DrawingML table cell text.
    Self::from_parts(
      &source.body_properties,
      source.list_style.as_deref(),
      &source.paragraph,
    )
  }

  pub(crate) fn from_pml(source: &p::TextBody) -> Self {
    // Source: LibreOffice oox/source/drawingml/textbodycontext.cxx
    // PresentationML p:txBody carries DrawingML bodyPr/lstStyle/a:p
    // children; import it through the same typed DrawingML paragraph path.
    Self::from_parts(
      &source.body_properties,
      source.list_style.as_deref(),
      &source.paragraph,
    )
  }

  fn from_parts(
    body_properties: &a::BodyProperties,
    list_style: Option<&a::ListStyle>,
    paragraphs: &[a::Paragraph],
  ) -> Self {
    Self {
      has_body_properties: true,
      has_noninherited_body_properties: has_noninherited_body_properties(body_properties),
      body_properties: Some(Box::new(body_properties.clone())),
      has_list_style: list_style.is_some(),
      list_style: list_style.map(TextListStyle::from_dml_list_style),
      paragraphs: paragraphs.iter().map(TextParagraph::from_dml).collect(),
    }
  }

  pub(crate) fn apply_text_styles(&mut self, master_text_list_style: Option<&TextListStyle>) {
    for paragraph in &mut self.paragraphs {
      paragraph.apply_text_styles(master_text_list_style, self.list_style.as_ref());
    }
  }
}

impl TextParagraph {
  pub(crate) fn from_dml(source: &a::Paragraph) -> Self {
    let level = source
      .paragraph_properties
      .as_ref()
      .and_then(|properties| properties.level)
      .map(|level| level as u8);
    let runs = source
      .paragraph_choice
      .iter()
      .filter_map(TextRun::from_dml)
      .collect();
    Self {
      level,
      paragraph_properties: source.paragraph_properties.clone(),
      end_paragraph_run_properties: source.end_paragraph_run_properties.clone(),
      master_paragraph_style: None,
      text_paragraph_style: None,
      runs,
    }
  }

  pub(crate) fn apply_text_styles(
    &mut self,
    master_text_list_style: Option<&TextListStyle>,
    text_list_style: Option<&TextListStyle>,
  ) {
    self.master_paragraph_style = master_text_list_style
      .and_then(|style| self.get_paragraph_style(style))
      .map(TextListParagraphStyleRef::to_owned_style);
    self.text_paragraph_style = text_list_style
      .and_then(|style| self.get_paragraph_style(style))
      .map(TextListParagraphStyleRef::to_owned_style);
  }

  pub(crate) fn get_paragraph_style<'a>(
    &self,
    text_list_style: &'a TextListStyle,
  ) -> Option<TextListParagraphStyleRef<'a>> {
    text_list_style.paragraph_style_for_level(self.level)
  }
}

impl TextRun {
  fn from_dml(choice: &a::ParagraphChoice) -> Option<Self> {
    match choice {
      a::ParagraphChoice::Run(run) => Some(Self {
        text: run.text.clone(),
        kind: TextRunKind::Run,
        field_type: None,
        run_properties: run.run_properties.clone(),
        field_paragraph_properties: None,
      }),
      a::ParagraphChoice::Break(line_break) => Some(Self {
        text: "\n".to_string(),
        kind: TextRunKind::Break,
        field_type: None,
        run_properties: line_break.run_properties.clone(),
        field_paragraph_properties: None,
      }),
      a::ParagraphChoice::Field(field) => field.text.as_ref().map(|text| Self {
        text: text.clone(),
        kind: TextRunKind::Field,
        field_type: field.r#type.clone(),
        run_properties: field.run_properties.clone(),
        field_paragraph_properties: field.paragraph_properties.clone(),
      }),
      a::ParagraphChoice::TextMath(_) => Some(Self {
        text: String::new(),
        kind: TextRunKind::Math,
        field_type: None,
        run_properties: None,
        field_paragraph_properties: None,
      }),
    }
  }
}

pub(crate) fn has_noninherited_body_properties(properties: &a::BodyProperties) -> bool {
  properties.rotation.is_some()
    || properties.use_paragraph_spacing.is_some()
    || properties.vertical_overflow.is_some()
    || properties.horizontal_overflow.is_some()
    || properties.vertical.is_some()
    || properties.wrap.is_some()
    || properties.left_inset.is_some()
    || properties.top_inset.is_some()
    || properties.right_inset.is_some()
    || properties.bottom_inset.is_some()
    || properties.column_count.is_some()
    || properties.column_spacing.is_some()
    || properties.right_to_left_columns.is_some()
    || properties.from_word_art.is_some()
    || properties.anchor.is_some()
    || properties.anchor_center.is_some()
    || properties.force_anti_alias.is_some()
    || properties.up_right.is_some()
    || properties.compatible_line_spacing.is_some()
    || properties.preset_text_warp.is_some()
    || properties.body_properties_choice1.is_some()
    || properties.scene3_d_type.is_some()
    || properties.body_properties_choice2.is_some()
    || properties.extension_list.is_some()
}
