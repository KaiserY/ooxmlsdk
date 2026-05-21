#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct TextBody {
  pub(crate) has_body_properties: bool,
  pub(crate) has_list_style: bool,
  pub(crate) paragraphs: Vec<TextParagraph>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct TextParagraph {
  pub(crate) level: Option<u8>,
  pub(crate) runs: Vec<TextRun>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct TextRun {
  pub(crate) text: String,
  pub(crate) kind: TextRunKind,
  pub(crate) field_type: Option<String>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) enum TextRunKind {
  #[default]
  Run,
  Break,
  Field,
  Math,
}
