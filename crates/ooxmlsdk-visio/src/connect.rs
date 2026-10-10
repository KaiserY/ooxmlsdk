use ooxmlsdk::schemas::schemas_microsoft_com_office_visio_2012_main::{PageContents, Shape};

use crate::sheet::shape_by_id;

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
    return connects.connect.iter().try_fold(Vec::new(), |_, connect| {
      Err(ConnectError::MissingFrom(connect.from_sheet))
    });
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
