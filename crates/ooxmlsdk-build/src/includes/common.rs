use quick_xml::{
  events::{attributes::AttrError, Event},
  Decoder, Reader,
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

  fn decoder(&self) -> Decoder;
}

pub struct IoReader<'de, R: BufRead> {
  reader: Reader<R>,
  peek: Option<Event<'de>>,
  buf: Vec<u8>,
}

impl<'de, R: BufRead> IoReader<'de, R> {
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

  fn decoder(&self) -> Decoder {
    self.reader.decoder()
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

  fn decoder(&self) -> Decoder {
    self.reader.decoder()
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
pub fn from_reader_inner<'de, R: BufRead>(reader: R) -> Result<IoReader<'de, R>, SdkError> {
  let mut xml_reader = IoReader::new(quick_xml::Reader::from_reader(reader));

  loop {
    let peek_event = xml_reader.peek()?;

    match peek_event {
      quick_xml::events::Event::Decl(_) | quick_xml::events::Event::Text(_) => {
        xml_reader.next()?;
      }
      _ => {
        break;
      }
    }
  }

  Ok(xml_reader)
}

#[inline]
pub fn from_str_inner(s: &str) -> Result<SliceReader<'_>, SdkError> {
  let mut xml_reader = SliceReader::new(quick_xml::Reader::from_str(s));

  loop {
    let peek_event = xml_reader.peek()?;

    match peek_event {
      quick_xml::events::Event::Decl(_) | quick_xml::events::Event::Text(_) => {
        xml_reader.next()?;
      }
      _ => {
        break;
      }
    }
  }

  Ok(xml_reader)
}
