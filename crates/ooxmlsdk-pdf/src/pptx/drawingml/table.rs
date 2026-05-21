use quick_xml::XmlVersion;
use quick_xml::events::{BytesStart, Event};
use quick_xml::{Decoder, Reader};

use super::text_body::{TextBody, TextParagraph, TextRun, TextRunKind};

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct TableProperties {
  pub(crate) style_id: Option<String>,
  pub(crate) first_row: bool,
  pub(crate) first_column: bool,
  pub(crate) last_row: bool,
  pub(crate) last_column: bool,
  pub(crate) band_row: bool,
  pub(crate) band_column: bool,
  pub(crate) grid: Vec<i64>,
  pub(crate) rows: Vec<TableRow>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct TableRow {
  pub(crate) height: i64,
  pub(crate) cells: Vec<TableCell>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct TableCell {
  pub(crate) row_span: Option<i32>,
  pub(crate) grid_span: Option<i32>,
  pub(crate) horizontal_merge: bool,
  pub(crate) vertical_merge: bool,
  pub(crate) margins: TableCellMargins,
  pub(crate) text_body: Option<TextBody>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct TableCellMargins {
  pub(crate) left: i32,
  pub(crate) right: i32,
  pub(crate) top: i32,
  pub(crate) bottom: i32,
}

impl Default for TableProperties {
  fn default() -> Self {
    Self {
      style_id: None,
      first_row: false,
      first_column: false,
      last_row: false,
      last_column: false,
      band_row: false,
      band_column: false,
      grid: Vec::new(),
      rows: Vec::new(),
    }
  }
}

impl Default for TableCellMargins {
  fn default() -> Self {
    Self {
      left: 91_440,
      right: 91_440,
      top: 45_720,
      bottom: 45_720,
    }
  }
}

impl TableProperties {
  pub(crate) fn from_graphic_data_xml_children(children: &[Box<str>]) -> Option<Self> {
    // Source: LibreOffice oox/source/drawingml/table/tablecontext.cxx
    // TableContext owns tblPr, tblGrid/gridCol, and tr/tc import after
    // GraphicalObjectFrameContext dispatches a table graphicData URI.
    children.iter().find_map(|child| parse_table(child))
  }
}

fn parse_table(xml: &str) -> Option<TableProperties> {
  let mut reader = Reader::from_str(xml);
  reader.config_mut().trim_text(true);
  let mut table = TableProperties::default();
  let mut rows = Vec::new();
  let mut current_row: Option<TableRow> = None;
  let mut current_cell: Option<TableCell> = None;
  let mut text_body: Option<TextBody> = None;
  let mut current_paragraph: Option<TextParagraph> = None;
  let mut in_table = false;
  let mut in_text = false;
  let mut in_table_style_id = false;

  loop {
    match reader.read_event() {
      Ok(Event::Start(event)) => {
        let name = local_name(event.name().as_ref()).to_vec();
        open_element(
          &name,
          &event,
          reader.decoder(),
          &mut table,
          &mut current_row,
          &mut current_cell,
          &mut text_body,
          &mut current_paragraph,
          &mut in_table,
          &mut in_text,
          &mut in_table_style_id,
        );
      }
      Ok(Event::Empty(event)) => {
        let name = local_name(event.name().as_ref()).to_vec();
        open_element(
          &name,
          &event,
          reader.decoder(),
          &mut table,
          &mut current_row,
          &mut current_cell,
          &mut text_body,
          &mut current_paragraph,
          &mut in_table,
          &mut in_text,
          &mut in_table_style_id,
        );
        close_element(
          &name,
          &mut rows,
          &mut current_row,
          &mut current_cell,
          &mut text_body,
          &mut current_paragraph,
          &mut in_text,
          &mut in_table_style_id,
        );
      }
      Ok(Event::Text(event)) if in_table_style_id => {
        if let Ok(text) = event.xml10_content() {
          table.style_id = Some(text.into_owned());
        }
      }
      Ok(Event::Text(event)) if in_text => {
        if let Some(paragraph) = &mut current_paragraph
          && let Ok(text) = event.xml10_content()
        {
          paragraph.runs.push(TextRun {
            text: text.into_owned(),
            kind: TextRunKind::Run,
            field_type: None,
            run_properties: None,
            field_paragraph_properties: None,
          });
        }
      }
      Ok(Event::End(event)) => {
        let name = local_name(event.name().as_ref()).to_vec();
        close_element(
          &name,
          &mut rows,
          &mut current_row,
          &mut current_cell,
          &mut text_body,
          &mut current_paragraph,
          &mut in_text,
          &mut in_table_style_id,
        );
      }
      Ok(Event::Eof) => break,
      Err(_) => return None,
      _ => {}
    }
  }

  if !in_table {
    return None;
  }
  table.rows = rows;
  Some(table)
}

fn open_element(
  name: &[u8],
  event: &BytesStart<'_>,
  decoder: Decoder,
  table: &mut TableProperties,
  current_row: &mut Option<TableRow>,
  current_cell: &mut Option<TableCell>,
  text_body: &mut Option<TextBody>,
  current_paragraph: &mut Option<TextParagraph>,
  in_table: &mut bool,
  in_text: &mut bool,
  in_table_style_id: &mut bool,
) {
  match name {
    b"tbl" => *in_table = true,
    b"tblPr" if *in_table => apply_table_properties(table, event, decoder),
    b"tableStyleId" if *in_table => *in_table_style_id = true,
    b"gridCol" if *in_table => {
      table.grid.push(attr_i64(event, decoder, b"w").unwrap_or(0));
    }
    b"tr" if *in_table => {
      *current_row = Some(TableRow {
        height: attr_i64(event, decoder, b"h").unwrap_or(0),
        cells: Vec::new(),
      });
    }
    b"tc" if current_row.is_some() => {
      *current_cell = Some(TableCell {
        row_span: attr_i32(event, decoder, b"rowSpan"),
        grid_span: attr_i32(event, decoder, b"gridSpan"),
        horizontal_merge: attr_bool(event, decoder, b"hMerge"),
        vertical_merge: attr_bool(event, decoder, b"vMerge"),
        margins: TableCellMargins::default(),
        text_body: None,
      });
    }
    b"tcPr" if current_cell.is_some() => {
      if let Some(cell) = current_cell {
        apply_cell_properties(cell, event, decoder);
      }
    }
    b"txBody" if current_cell.is_some() => {
      *text_body = Some(TextBody {
        has_body_properties: false,
        has_noninherited_body_properties: false,
        body_properties: None,
        has_list_style: false,
        list_style: None,
        paragraphs: Vec::new(),
      });
    }
    b"bodyPr" if text_body.is_some() => {
      if let Some(body) = text_body {
        body.has_body_properties = true;
      }
    }
    b"lstStyle" if text_body.is_some() => {
      if let Some(body) = text_body {
        body.has_list_style = true;
      }
    }
    b"p" if text_body.is_some() => {
      *current_paragraph = Some(TextParagraph {
        level: None,
        paragraph_properties: None,
        end_paragraph_run_properties: None,
        master_paragraph_style: None,
        text_paragraph_style: None,
        runs: Vec::new(),
      });
    }
    b"pPr" if current_paragraph.is_some() => {
      if let Some(paragraph) = current_paragraph {
        paragraph.level = attr_i32(event, decoder, b"lvl").map(|level| level as u8);
      }
    }
    b"t" if current_paragraph.is_some() => *in_text = true,
    _ => {}
  }
}

fn close_element(
  name: &[u8],
  rows: &mut Vec<TableRow>,
  current_row: &mut Option<TableRow>,
  current_cell: &mut Option<TableCell>,
  text_body: &mut Option<TextBody>,
  current_paragraph: &mut Option<TextParagraph>,
  in_text: &mut bool,
  in_table_style_id: &mut bool,
) {
  match name {
    b"t" => *in_text = false,
    b"tableStyleId" => *in_table_style_id = false,
    b"p" => {
      if let Some(paragraph) = current_paragraph.take()
        && let Some(body) = text_body.as_mut()
      {
        body.paragraphs.push(paragraph);
      }
    }
    b"txBody" => {
      if let Some(body) = text_body.take()
        && let Some(cell) = current_cell.as_mut()
      {
        cell.text_body = Some(body);
      }
    }
    b"tc" => {
      if let Some(cell) = current_cell.take()
        && let Some(row) = current_row.as_mut()
      {
        row.cells.push(cell);
      }
    }
    b"tr" => {
      if let Some(row) = current_row.take() {
        rows.push(row);
      }
    }
    _ => {}
  }
}

fn apply_table_properties(table: &mut TableProperties, event: &BytesStart<'_>, decoder: Decoder) {
  table.first_row = attr_bool(event, decoder, b"firstRow");
  table.first_column = attr_bool(event, decoder, b"firstCol");
  table.last_row = attr_bool(event, decoder, b"lastRow");
  table.last_column = attr_bool(event, decoder, b"lastCol");
  table.band_row = attr_bool(event, decoder, b"bandRow");
  table.band_column = attr_bool(event, decoder, b"bandCol");
}

fn apply_cell_properties(cell: &mut TableCell, event: &BytesStart<'_>, decoder: Decoder) {
  cell.margins.left = attr_i32(event, decoder, b"marL").unwrap_or(cell.margins.left);
  cell.margins.right = attr_i32(event, decoder, b"marR").unwrap_or(cell.margins.right);
  cell.margins.top = attr_i32(event, decoder, b"marT").unwrap_or(cell.margins.top);
  cell.margins.bottom = attr_i32(event, decoder, b"marB").unwrap_or(cell.margins.bottom);
}

fn attr_bool(event: &BytesStart<'_>, decoder: Decoder, local: &[u8]) -> bool {
  attr_string(event, decoder, local)
    .as_deref()
    .is_some_and(|value| value == "1" || value == "true")
}

fn attr_i32(event: &BytesStart<'_>, decoder: Decoder, local: &[u8]) -> Option<i32> {
  attr_string(event, decoder, local)?.parse().ok()
}

fn attr_i64(event: &BytesStart<'_>, decoder: Decoder, local: &[u8]) -> Option<i64> {
  attr_string(event, decoder, local)?.parse().ok()
}

fn attr_string(event: &BytesStart<'_>, decoder: Decoder, local: &[u8]) -> Option<String> {
  event
    .attributes()
    .flatten()
    .find(|attr| local_name(attr.key.as_ref()) == local)
    .and_then(|attr| {
      attr
        .decoded_and_normalized_value(XmlVersion::Implicit1_0, decoder)
        .ok()
        .map(|value| value.into_owned())
    })
}

fn local_name(name: &[u8]) -> &[u8] {
  name.rsplit(|byte| *byte == b':').next().unwrap_or(name)
}
