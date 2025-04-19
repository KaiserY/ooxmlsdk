use std::env;

use ooxmlsdk_build::generate_neo;

fn main() {
  let out_dir = env::var("OUT_DIR").unwrap();

  generate_neo("data", &out_dir);
}
