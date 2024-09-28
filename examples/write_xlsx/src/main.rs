use ooxmlsdk::parts::spreadsheet_document::SpreadsheetDocument;

fn main() {
  let xlsx = SpreadsheetDocument::new("examples/read_xlsx/samples/demo.xlsx").unwrap();

  xlsx.save("/tmp/demo.xlsx").unwrap();
}
