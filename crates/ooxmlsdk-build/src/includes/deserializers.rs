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
pub enum DeError {
  #[error("quick_xml error")]
  QuickXmlError(#[from] quick_xml::Error),
  #[error("quick_xml attr error")]
  AttrError(#[from] AttrError),
  #[error("ParseIntError")]
  ParseIntError(#[from] ParseIntError),
  #[error("ParseFloatError")]
  ParseFloatError(#[from] ParseFloatError),
  #[error("mismatch error (expected {expected:?}, found {found:?})")]
  MismatchError { expected: String, found: String },
  #[error("unknown error")]
  UnknownError,
}

pub trait XmlReader<'de> {
  fn next(&mut self) -> Result<Event<'de>, DeError>;

  fn peek(&mut self) -> Result<&Event<'de>, DeError>;

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
  fn next(&mut self) -> Result<Event<'de>, DeError> {
    if let Some(e) = self.peek.take() {
      return Ok(e);
    }

    self.buf.clear();

    let event = self.reader.read_event_into(&mut self.buf)?;

    Ok(event.into_owned())
  }

  fn peek(&mut self) -> Result<&Event<'de>, DeError> {
    if self.peek.is_none() {
      self.peek = Some(self.next()?);
    }

    match self.peek.as_ref() {
      Some(v) => Ok(v),
      None => Err(DeError::UnknownError),
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
  fn next(&mut self) -> Result<Event<'de>, DeError> {
    if let Some(e) = self.peek.take() {
      return Ok(e);
    }

    let event = self.reader.read_event()?;

    Ok(event.into_owned())
  }

  fn peek(&mut self) -> Result<&Event<'de>, DeError> {
    if self.peek.is_none() {
      self.peek = Some(self.next()?);
    }

    match self.peek.as_ref() {
      Some(v) => Ok(v),
      None => Err(DeError::UnknownError),
    }
  }

  fn decoder(&self) -> Decoder {
    self.reader.decoder()
  }
}
