use quick_xml::{
  Reader,
  events::{Event, attributes::AttrError},
  name::QName,
};
use std::{
  io::BufRead,
  num::{ParseFloatError, ParseIntError},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SdkError {
  #[error("quick_xml error")]
  QuickXmlError(#[from] quick_xml::Error),
  #[error("quick_xml attr error")]
  AttrError(#[from] AttrError),
  #[error("ParseIntError")]
  ParseIntError(#[from] ParseIntError),
  #[error("ParseFloatError")]
  ParseFloatError(#[from] ParseFloatError),
  #[error("StdFmtError")]
  StdFmtError(#[from] std::fmt::Error),
  #[error("StdIoError")]
  StdIoError(#[from] std::io::Error),
  #[cfg(feature = "parts")]
  #[error("ZipError")]
  ZipError(#[from] zip::result::ZipError),
  #[error("mismatch error (expected {expected:?}, found {found:?})")]
  MismatchError { expected: String, found: String },
  #[error("`{0}` common error")]
  CommonError(String),
  #[error("unknown error")]
  UnknownError,
}

pub trait XmlReader<'de> {
  fn next(&mut self) -> Result<Event<'de>, SdkError>;

  fn peek(&mut self) -> Result<&Event<'de>, SdkError>;

  fn read_to_end(&mut self, name: QName) -> Result<(), SdkError>;
}

pub struct IoReader<'de, R: BufRead> {
  reader: Reader<R>,
  peek: Option<Event<'de>>,
  buf: Vec<u8>,
}

impl<R: BufRead> IoReader<'_, R> {
  pub fn new(reader: Reader<R>) -> Self {
    Self {
      reader,
      peek: None,
      buf: Vec::new(),
    }
  }
}

impl<'de, R: BufRead> XmlReader<'de> for IoReader<'de, R> {
  fn next(&mut self) -> Result<Event<'de>, SdkError> {
    if let Some(e) = self.peek.take() {
      return Ok(e);
    }

    self.buf.clear();

    let event = self.reader.read_event_into(&mut self.buf)?;

    Ok(event.into_owned())
  }

  fn peek(&mut self) -> Result<&Event<'de>, SdkError> {
    if self.peek.is_none() {
      self.peek = Some(self.next()?);
    }

    match self.peek.as_ref() {
      Some(v) => Ok(v),
      None => Err(SdkError::UnknownError),
    }
  }

  fn read_to_end(&mut self, name: QName) -> Result<(), SdkError> {
    match self.reader.read_to_end_into(name, &mut self.buf) {
      Err(e) => Err(e.into()),
      Ok(_) => Ok(()),
    }
  }
}

pub struct SliceReader<'de> {
  reader: Reader<&'de [u8]>,
  peek: Option<Event<'de>>,
}

impl<'de> SliceReader<'de> {
  pub fn new(reader: Reader<&'de [u8]>) -> Self {
    Self { reader, peek: None }
  }
}

impl<'de> XmlReader<'de> for SliceReader<'de> {
  fn next(&mut self) -> Result<Event<'de>, SdkError> {
    if let Some(e) = self.peek.take() {
      return Ok(e);
    }

    let event = self.reader.read_event()?;

    Ok(event.into_owned())
  }

  fn peek(&mut self) -> Result<&Event<'de>, SdkError> {
    if self.peek.is_none() {
      self.peek = Some(self.next()?);
    }

    match self.peek.as_ref() {
      Some(v) => Ok(v),
      None => Err(SdkError::UnknownError),
    }
  }

  fn read_to_end(&mut self, name: QName) -> Result<(), SdkError> {
    match self.reader.read_to_end(name) {
      Err(e) => Err(e.into()),
      Ok(_) => Ok(()),
    }
  }
}

pub fn resolve_zip_file_path(path: &str) -> String {
  let mut stack = Vec::new();

  for component in path.split('/') {
    match component {
      "" | "." => {
        // Ignore empty components and current directory symbol
      }
      ".." => {
        // Go up one directory if possible
        stack.pop();
      }
      _ => {
        // Add the component to the path
        stack.push(component);
      }
    }
  }
  // Join the components back into a path
  stack.join("/")
}

#[inline]
pub(crate) fn from_reader_inner<'de, R: BufRead>(reader: R) -> Result<IoReader<'de, R>, SdkError> {
  let mut xml_reader = quick_xml::Reader::from_reader(reader);
  xml_reader.config_mut().check_end_names = false;

  let mut reader = IoReader::new(xml_reader);

  loop {
    let peek_event = reader.peek()?;

    match peek_event {
      quick_xml::events::Event::Decl(_)
      | quick_xml::events::Event::Text(_)
      | quick_xml::events::Event::CData(_) => {
        reader.next()?;
      }
      _ => {
        break;
      }
    }
  }

  Ok(reader)
}

#[inline]
pub(crate) fn from_str_inner(s: &str) -> Result<SliceReader<'_>, SdkError> {
  let mut xml_reader = quick_xml::Reader::from_str(s);
  xml_reader.config_mut().check_end_names = false;

  let mut reader = SliceReader::new(xml_reader);

  loop {
    let peek_event = reader.peek()?;

    match peek_event {
      quick_xml::events::Event::Decl(_)
      | quick_xml::events::Event::Text(_)
      | quick_xml::events::Event::CData(_) => {
        reader.next()?;
      }
      _ => {
        break;
      }
    }
  }

  Ok(reader)
}

#[inline]
pub fn parse_bool_bytes(b: &[u8]) -> Result<bool, SdkError> {
  match b {
    b"true" | b"1" | b"True" | b"TRUE" | b"t" | b"Yes" | b"YES" | b"yes" | b"y" => Ok(true),
    b"false" | b"0" | b"False" | b"FALSE" | b"f" | b"No" | b"NO" | b"no" | b"n" | b"" => Ok(false),
    other => Err(SdkError::CommonError(
      String::from_utf8_lossy(other).into_owned(),
    )),
  }
}
