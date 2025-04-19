use std::env;

use ooxmlsdk_build::generate;

fn main() {
  let out_dir = env::var("OUT_DIR").unwrap();

  generate("data", &out_dir);
}
