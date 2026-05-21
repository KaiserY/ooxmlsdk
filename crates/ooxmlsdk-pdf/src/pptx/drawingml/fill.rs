#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct FillProperties {
  pub(crate) kind: FillKind,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum FillKind {
  None,
  Solid(String),
  Group,
}
