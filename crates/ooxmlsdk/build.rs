use std::env;

use ooxmlsdk_build::gen_neo;

fn main() {
  let out_dir = env::var("OUT_DIR").unwrap();

  gen_neo("data", &out_dir);
}
