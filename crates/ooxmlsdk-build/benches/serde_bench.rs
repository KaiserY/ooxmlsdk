use criterion::{Criterion, criterion_group, criterion_main};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

const XML: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"><Default Extension="bin" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.printerSettings"/><Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/><Default Extension="xml" ContentType="application/xml"/><Override ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml" PartName="/xl/workbook.xml"/><Override ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml" PartName="/xl/worksheets/sheet1.xml"/><Override ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml" PartName="/xl/worksheets/sheet2.xml"/><Override ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml" PartName="/xl/worksheets/sheet3.xml"/><Override ContentType="application/vnd.openxmlformats-officedocument.theme+xml" PartName="/xl/theme/theme1.xml"/><Override ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.styles+xml" PartName="/xl/styles.xml"/><Override ContentType="application/vnd.openxmlformats-package.core-properties+xml" PartName="/docProps/core.xml"/><Override ContentType="application/vnd.openxmlformats-officedocument.extended-properties+xml" PartName="/docProps/app.xml"/><Override ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.sharedStrings+xml" PartName="/xl/sharedStrings.xml"/><Override ContentType="application/vnd.ms-excel.person+xml" PartName="/xl/persons/person.xml"/></Types>"#;

#[derive(Clone, Deserialize, Serialize)]
pub struct Types {
  #[serde(rename = "@xmlns")]
  pub xmlns: Option<String>,
  #[serde(skip)]
  pub xmlns_map: std::collections::HashMap<String, String>,
  #[serde(rename = "@Ignorable")]
  pub mc_ignorable: Option<String>,
  #[serde(rename = "$value")]
  pub children: Vec<TypesChildChoice>,
}

#[derive(Clone, Deserialize, Serialize)]
pub enum TypesChildChoice {
  Default(std::boxed::Box<Default>),
  Override(std::boxed::Box<Override>),
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Default {
  #[serde(rename = "@Extension")]
  pub extension: String,
  #[serde(rename = "@ContentType")]
  pub content_type: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Override {
  #[serde(rename = "@ContentType")]
  pub content_type: String,
  #[serde(rename = "@PartName")]
  pub part_name: String,
}

fn from_str() {
  let _ = ooxmlsdk_build::includes::packages::opc_content_types::Types::from_str(XML).unwrap();
}

fn serde_from_str() {
  let _: Types = quick_xml::de::from_str(XML).unwrap();
}

fn to_str() {
  let t = ooxmlsdk_build::includes::packages::opc_content_types::Types::from_str(XML).unwrap();

  let _ = t.to_xml();
}

fn bench_from_str(c: &mut Criterion) {
  c.bench_function("from_str", |b| b.iter(from_str));
}

fn bench_serde_from_str(c: &mut Criterion) {
  c.bench_function("serde_from_str", |b| b.iter(serde_from_str));
}

fn bench_to_str(c: &mut Criterion) {
  c.bench_function("to_str", |b| b.iter(to_str));
}

criterion_group!(
  name = benches;
  config = Criterion::default();
  targets = bench_from_str, bench_serde_from_str, bench_to_str
);

criterion_main!(benches);
