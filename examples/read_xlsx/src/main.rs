use ooxmlsdk::parts::spreadsheet_document::SpreadsheetDocument;

fn main() {
  let xlsx = SpreadsheetDocument::new("examples/read_xlsx/samples/demo.xlsx").unwrap();

  println!("{:?}", xlsx.workbook_part.root_element);
}
