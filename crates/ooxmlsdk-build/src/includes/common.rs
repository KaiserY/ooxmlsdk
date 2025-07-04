use quick_xml::{
  encoding::EncodingError,
  events::{attributes::AttrError, Event},
  Reader,
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
  #[error("quick_xml encoding error")]
  QuickEncodingError(#[from] EncodingError),
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
}

pub struct IoReader<R: BufRead> {
  reader: Reader<R>,
  buf: Vec<u8>,
}

impl<R: BufRead> IoReader<R> {
  pub fn new(reader: Reader<R>) -> Self {
    Self {
      reader,
      buf: Vec::new(),
    }
  }
}

impl<'de, R: BufRead> XmlReader<'de> for IoReader<R> {
  #[inline]
  fn next(&mut self) -> Result<Event<'de>, SdkError> {
    self.buf.clear();

    Ok(self.reader.read_event_into(&mut self.buf)?.into_owned())
  }
}

pub struct SliceReader<'de> {
  reader: Reader<&'de [u8]>,
}

impl<'de> SliceReader<'de> {
  pub fn new(reader: Reader<&'de [u8]>) -> Self {
    Self { reader }
  }
}

impl<'de> XmlReader<'de> for SliceReader<'de> {
  #[inline]
  fn next(&mut self) -> Result<Event<'de>, SdkError> {
    Ok(self.reader.read_event()?)
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
pub(crate) fn from_reader_inner<R: BufRead>(reader: R) -> Result<IoReader<R>, SdkError> {
  let mut xml_reader = quick_xml::Reader::from_reader(reader);
  xml_reader.config_mut().check_end_names = false;

  Ok(IoReader::new(xml_reader))
}

#[inline]
pub(crate) fn from_str_inner(s: &str) -> Result<SliceReader<'_>, SdkError> {
  let mut xml_reader = quick_xml::Reader::from_str(s);
  xml_reader.config_mut().check_end_names = false;

  Ok(SliceReader::new(xml_reader))
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

macro_rules! expect_event_start {
  ($xml_reader:expr, $xml_event:expr, $tag_prefix:expr, $tag:expr) => {{
    if let Some((e, empty_tag)) = $xml_event {
      (e, empty_tag)
    } else {
      let (e, empty_tag) = loop {
        match $xml_reader.next()? {
          quick_xml::events::Event::Start(b) => break (b, false),
          quick_xml::events::Event::Empty(b) => break (b, true),
          quick_xml::events::Event::Eof => {
            return Err(super::super::common::SdkError::UnknownError)
          }
          _ => continue,
        }
      };

      match e.name().as_ref() {
        $tag_prefix | $tag => (),
        _ => {
          Err(super::super::common::SdkError::MismatchError {
            expected: String::from_utf8_lossy($tag).to_string(),
            found: String::from_utf8_lossy(e.name().as_ref()).to_string(),
          })?;
        }
      }

      (e, empty_tag)
    }
  }};
}

pub(crate) use expect_event_start;
