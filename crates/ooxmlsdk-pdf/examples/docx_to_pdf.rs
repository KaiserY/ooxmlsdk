use std::env;
use std::fs::File;
use std::io::Write;

use ooxmlsdk_pdf::{PdfOptions, convert_docx};

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let mut args = env::args().skip(1);
  let input = args
    .next()
    .ok_or("usage: docx_to_pdf <input.docx> <output.pdf>")?;
  let output = args
    .next()
    .ok_or("usage: docx_to_pdf <input.docx> <output.pdf>")?;
  if args.next().is_some() {
    return Err("usage: docx_to_pdf <input.docx> <output.pdf>".into());
  }

  let pdf = convert_docx(File::open(input)?, PdfOptions::default())?;
  File::create(output)?.write_all(&pdf)?;
  Ok(())
}
