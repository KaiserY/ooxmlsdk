use ooxmlsdk::schemas::schemas_microsoft_com_office_visio_2012_main::{
  Connect, Connects, PageContents, Shape,
};

use crate::sheet::shape_by_id;

/// A `Connect` to add. End coordinates stay on the one-dimensional shape's
/// direct cells. This does not compute glue.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ConnectLink<'a> {
  pub from_sheet: u32,
  pub to_sheet: u32,
  pub from_cell: Option<&'a str>,
  pub from_part: Option<i32>,
  pub to_cell: Option<&'a str>,
  pub to_part: Option<i32>,
}

/// Add a `Connect` between two shape ids on the same page.
pub fn add_connect(page: &mut PageContents, link: ConnectLink<'_>) -> Result<(), ConnectError> {
  let Some(shapes) = &page.shapes else {
    return Err(ConnectError::MissingFrom(link.from_sheet));
  };
  if shape_by_id(shapes, link.from_sheet).is_none() {
    return Err(ConnectError::MissingFrom(link.from_sheet));
  }
  if shape_by_id(shapes, link.to_sheet).is_none() {
    return Err(ConnectError::MissingTo(link.to_sheet));
  }
  let connects = page.connects.get_or_insert_with(Connects::default);
  connects.connect.push(Connect {
    from_sheet: link.from_sheet,
    to_sheet: link.to_sheet,
    from_cell: link.from_cell.map(str::to_string),
    from_part: link.from_part,
    to_cell: link.to_cell.map(str::to_string),
    to_part: link.to_part,
  });
  Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConnectError {
  MissingFrom(u32),
  MissingTo(u32),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ResolvedConnect<'a> {
  pub from: &'a Shape,
  pub to: &'a Shape,
  pub from_cell: Option<&'a str>,
  pub to_cell: Option<&'a str>,
}

/// Resolve each `Connect` to shapes on the same page. A sheet id that is not
/// on the page is an error.
pub fn resolve_connects(page: &PageContents) -> Result<Vec<ResolvedConnect<'_>>, ConnectError> {
  let Some(connects) = &page.connects else {
    return Ok(Vec::new());
  };
  let Some(shapes) = &page.shapes else {
    return match connects.connect.first() {
      Some(connect) => Err(ConnectError::MissingFrom(connect.from_sheet)),
      None => Ok(Vec::new()),
    };
  };
  let mut resolved = Vec::with_capacity(connects.connect.len());
  for connect in &connects.connect {
    let Some(from) = shape_by_id(shapes, connect.from_sheet) else {
      return Err(ConnectError::MissingFrom(connect.from_sheet));
    };
    let Some(to) = shape_by_id(shapes, connect.to_sheet) else {
      return Err(ConnectError::MissingTo(connect.to_sheet));
    };
    resolved.push(ResolvedConnect {
      from,
      to,
      from_cell: connect.from_cell.as_deref(),
      to_cell: connect.to_cell.as_deref(),
    });
  }
  Ok(resolved)
}
