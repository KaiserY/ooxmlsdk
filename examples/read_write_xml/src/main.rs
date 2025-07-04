use ooxmlsdk::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Worksheet;
use std::fs::File;
use std::io::BufReader;

fn main() {
  let value = Worksheet::from_reader(BufReader::new(
    File::open("examples/read_write_xml/samples/sheet1.xml").unwrap(),
  ))
  .unwrap();

  println!("{value}");
}
