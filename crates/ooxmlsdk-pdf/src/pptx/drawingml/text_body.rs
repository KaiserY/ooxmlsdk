#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct TextBody {
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
}
