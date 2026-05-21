#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct LineProperties {
  pub(crate) color: Option<String>,
  pub(crate) width_emu: Option<i64>,
}
