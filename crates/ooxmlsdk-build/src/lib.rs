pub mod sdk_code;
pub mod sdk_data;
pub mod simple_type;
pub mod utils;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

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
    let upstream_data_dir = workspace_root.join("data");

    sdk_data::gen_sdk_data(&upstream_data_dir, &sdk_data_dir, &package_schema_dir)
      .expect("failed to generate sdk data");
    sdk_code::gen_sdk_code(&sdk_data_dir, &runtime_src_dir).expect("failed to generate sdk code");
  }

  #[test]
  fn test_gen_sdk_code_to_temp_dir() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = manifest_dir
      .parent()
      .and_then(|path| path.parent())
      .expect("workspace root");
    let sdk_data_dir = workspace_root.join("sdk_data");
    let out_dir = std::env::temp_dir().join(format!("ooxmlsdk-gen-{}", std::process::id()));

    if out_dir.exists() {
      std::fs::remove_dir_all(&out_dir).expect("failed to clear temp generated dir");
    }
    std::fs::create_dir_all(&out_dir).expect("failed to create temp generated dir");

    sdk_code::gen_sdk_code(&sdk_data_dir, &out_dir).expect("failed to generate sdk code");

    assert!(out_dir.join("parts.rs").is_file());
    assert!(out_dir.join("parts/wordprocessing_document.rs").is_file());

    std::fs::remove_dir_all(&out_dir).expect("failed to remove temp generated dir");
  }
}
