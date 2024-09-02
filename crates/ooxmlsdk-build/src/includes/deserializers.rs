use quick_xml::{events::Event, Decoder, Reader};
use std::io::BufRead;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DeError {
  #[error("quick_xml error")]
  QuickXmlError(#[from] quick_xml::Error),

  #[error("mismatch error (expected {expected:?}, found {found:?})")]
  MismatchError { expected: String, found: String },

  #[error("unknown error")]
  UnknownError,
}

pub trait XmlReader<'de> {
  fn next(&mut self) -> Result<Event<'de>, DeError>;

  fn decoder(&self) -> Decoder;
}

pub struct IoReader<R: BufRead> {
  reader: Reader<R>,

  buf: Vec<u8>,
}

impl<'de, R: BufRead> XmlReader<'de> for IoReader<R> {
  fn next(&mut self) -> Result<Event<'de>, DeError> {
    self.buf.clear();

    let event = self.reader.read_event_into(&mut self.buf)?;

    Ok(event.into_owned())
  }

  fn decoder(&self) -> Decoder {
    self.reader.decoder()
  }
}

pub struct SliceReader<'de> {
  reader: Reader<&'de [u8]>,
}

impl<'de> XmlReader<'de> for SliceReader<'de> {
  fn next(&mut self) -> Result<Event<'de>, DeError> {
    let event = self.reader.read_event()?;

    Ok(event.into_owned())
  }

  fn decoder(&self) -> Decoder {
    self.reader.decoder()
  }
}
