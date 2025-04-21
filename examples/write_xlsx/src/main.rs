use ooxmlsdk::parts::spreadsheet_document::SpreadsheetDocument;
use std::error::Error;
use std::fs::OpenOptions;

fn main() -> Result<(), Box<dyn Error>> {
  let xlsx = SpreadsheetDocument::new_from_file("examples/read_xlsx/samples/demo.xlsx")?;

  let file = OpenOptions::new()
    .create(true)
    .write(true)
    .truncate(true)
    .open("/tmp/demo.xlsx")?;

  xlsx.save(file)?;

  Ok(())
}
