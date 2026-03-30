use std::path::Path;

type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type Result<T> = std::result::Result<T, BoxError>;

pub mod includes;
pub mod sdk_code;
pub mod sdk_data;
pub mod simple_type;
pub mod utils;

pub fn generate<P: AsRef<Path>, Q: AsRef<Path>, R: AsRef<Path>, S: AsRef<Path>>(
  data_dir: P,
  sdk_data_dir: Q,
  package_schemas_dir: R,
  out_dir: S,
) -> Result<()> {
  sdk_data::gen_sdk_data(
    data_dir.as_ref(),
    sdk_data_dir.as_ref(),
    package_schemas_dir.as_ref(),
  )?;
  sdk_code::gen_sdk_code(sdk_data_dir.as_ref(), out_dir.as_ref())?;
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  #[ignore]
  fn test_gen() {
    sdk_code::gen_sdk_code("../../sdk_data", "../ooxmlsdk/src").unwrap();
  }

  #[test]
  #[ignore]
  fn test_sync_sdk_data() {
    generate(
      "../../../Open-XML-SDK/data",
      "../../sdk_data",
      "../../schemas/OpenPackagingConventions-XMLSchema",
      "../ooxmlsdk/src",
    )
    .unwrap();
  }
}
