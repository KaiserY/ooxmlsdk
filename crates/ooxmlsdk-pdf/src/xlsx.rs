use ooxmlsdk::parts::spreadsheet_document::SpreadsheetDocument;
use ooxmlsdk::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main as x;

use crate::docx::PageSetup;
use crate::error::Result;
use crate::layout::{self, LayoutDocument};
use crate::options::PdfOptions;

pub(crate) fn layout(
  package: &mut SpreadsheetDocument,
  _options: &PdfOptions,
) -> Result<LayoutDocument> {
  let workbook_part = package.workbook_part()?;
  let shared_strings = shared_strings(package).unwrap_or_default();
  let mut pages = Vec::new();
  let worksheet_parts = workbook_part.worksheet_parts(package).collect::<Vec<_>>();

  for (index, worksheet_part) in worksheet_parts.iter().enumerate() {
    let worksheet = worksheet_part.root_element(package)?;
    let mut lines = vec![format!("Sheet {}", index + 1)];
    for row in &worksheet.sheet_data.row {
      let values = row
        .cell
        .iter()
        .map(|cell| cell_text(cell, &shared_strings))
        .collect::<Vec<_>>();
      if values.iter().any(|value| !value.is_empty()) {
        lines.push(values.join("    "));
      }
    }
    pages.push((PageSetup::default(), lines));
  }

  Ok(layout::text_pages(pages))
}

fn shared_strings(package: &mut SpreadsheetDocument) -> Result<Vec<String>> {
  let workbook_part = package.workbook_part()?;
  let Some(shared_string_part) = workbook_part.shared_string_table_part(package) else {
    return Ok(Vec::new());
  };
  let table = shared_string_part.root_element(package)?;
  Ok(
    table
      .shared_string_item
      .iter()
      .map(shared_string_item_text)
      .collect(),
  )
}

fn shared_string_item_text(item: &x::SharedStringItem) -> String {
  if let Some(text) = &item.text
    && let Some(content) = &text.xml_content
  {
    return content.to_string();
  }

  item
    .run
    .iter()
    .filter_map(|run| run.text.xml_content.as_deref())
    .collect()
}

fn inline_string_text(value: &x::InlineString) -> String {
  if let Some(text) = &value.text
    && let Some(content) = &text.xml_content
  {
    return content.to_string();
  }

  value
    .run
    .iter()
    .filter_map(|run| run.text.xml_content.as_deref())
    .collect()
}

fn cell_text(cell: &x::Cell, shared_strings: &[String]) -> String {
  match cell.data_type {
    Some(x::CellValues::SharedString) => cell
      .cell_value
      .as_ref()
      .and_then(|value| value.xml_content.as_deref())
      .and_then(|index| index.parse::<usize>().ok())
      .and_then(|index| shared_strings.get(index))
      .cloned()
      .unwrap_or_default(),
    Some(x::CellValues::InlineString) => cell
      .inline_string
      .as_deref()
      .map(inline_string_text)
      .unwrap_or_default(),
    Some(x::CellValues::Boolean) => match cell
      .cell_value
      .as_ref()
      .and_then(|value| value.xml_content.as_deref())
    {
      Some("1") => "TRUE".to_string(),
      Some("0") => "FALSE".to_string(),
      Some(value) => value.to_string(),
      None => String::new(),
    },
    _ => cell
      .cell_value
      .as_ref()
      .and_then(|value| value.xml_content.as_deref())
      .map(ToString::to_string)
      .unwrap_or_default(),
  }
}
