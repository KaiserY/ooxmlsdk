type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type Result<T> = std::result::Result<T, BoxError>;

pub mod includes;
pub mod sdk_code;
pub mod sdk_data;
pub mod simple_type;
pub mod utils;

#[cfg(test)]
mod tests {
  use super::*;
  use std::path::PathBuf;

  #[test]
  #[ignore]
  fn test_gen() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = manifest_dir
      .parent()
      .and_then(|path| path.parent())
      .expect("workspace root");
    let sdk_data_dir = workspace_root.join("sdk_data");
    let package_schema_dir = workspace_root.join("schemas/OpenPackagingConventions-XMLSchema");
    let runtime_src_dir = workspace_root.join("crates/ooxmlsdk/src");
    let upstream_data_dir = workspace_root.join("../Open-XML-SDK/data");

    sdk_data::gen_sdk_data(&upstream_data_dir, &sdk_data_dir, &package_schema_dir).unwrap();
    sdk_code::gen_sdk_code(&sdk_data_dir, &runtime_src_dir).unwrap();
  }
}
