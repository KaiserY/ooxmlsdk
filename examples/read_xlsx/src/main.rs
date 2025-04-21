use ooxmlsdk::parts::spreadsheet_document::SpreadsheetDocument;

fn main() {
  let xlsx = SpreadsheetDocument::new_from_file("examples/read_xlsx/samples/demo.xlsx").unwrap();

  println!("{}", xlsx.workbook_part.root_element);
}
