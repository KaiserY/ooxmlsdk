use std::env;
use std::fs::File;
use std::io::Write;

use ooxmlsdk_pdf::{PdfOptions, convert_docx, convert_pptx, convert_xlsx};

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let mut args = env::args().skip(1);
  let input = args
    .next()
    .ok_or("usage: docx_to_pdf <input.docx|input.pptx|input.xlsx> <output.pdf>")?;
  let output = args
    .next()
    .ok_or("usage: docx_to_pdf <input.docx|input.pptx|input.xlsx> <output.pdf>")?;
  if args.next().is_some() {
    return Err("usage: docx_to_pdf <input.docx|input.pptx|input.xlsx> <output.pdf>".into());
  }

  let pdf = match std::path::Path::new(&input)
    .extension()
    .and_then(|extension| extension.to_str())
  {
    Some("pptx" | "pptm" | "ppsx" | "ppsm") => {
      convert_pptx(File::open(input)?, PdfOptions::default())?
    }
    Some("xlsx" | "xlsm") => convert_xlsx(File::open(input)?, PdfOptions::default())?,
    _ => convert_docx(File::open(input)?, PdfOptions::default())?,
  };
  File::create(output)?.write_all(&pdf)?;
  Ok(())
}
