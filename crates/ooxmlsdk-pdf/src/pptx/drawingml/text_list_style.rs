#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct TextListStyle {
  pub(crate) levels: Vec<TextListLevelStyle>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct TextListLevelStyle {
  pub(crate) level: u8,
}
