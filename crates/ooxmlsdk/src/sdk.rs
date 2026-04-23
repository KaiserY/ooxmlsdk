#[cfg(feature = "parts")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PartDescriptor {
  pub relationship_type: &'static str,
  pub path_prefix: &'static str,
}

#[cfg(feature = "parts")]
impl PartDescriptor {
  pub const fn new(relationship_type: &'static str, path_prefix: &'static str) -> Self {
    Self {
      relationship_type,
      path_prefix,
    }
  }
}

pub trait SdkEnum: Sized {
  fn as_xml_str(&self) -> &'static str;

  fn from_xml_bytes(value: &[u8]) -> Result<Self, crate::common::SdkError>;

  fn to_xml(&self) -> String {
    self.as_xml_str().to_string()
  }
}

pub trait SdkType: Sized {}

pub trait SdkChoice: Sized {
  fn matches_specific_start_qname(name: &[u8]) -> bool;

  #[inline]
  fn matches_start_qname(name: &[u8]) -> bool {
    Self::matches_specific_start_qname(name)
  }

  #[inline]
  fn accepts_any_child() -> bool {
    false
  }

  #[inline]
  fn accepts_text() -> bool {
    false
  }

  #[inline]
  fn from_text_value(_value: &str) -> Option<Self> {
    None
  }
}

#[cfg(feature = "parts")]
pub trait SdkPart: Sized {
  const DESCRIPTOR: PartDescriptor;
  const RELATIONSHIP_TYPE: &'static str = Self::DESCRIPTOR.relationship_type;
  const PATH_PREFIX: &'static str = Self::DESCRIPTOR.path_prefix;

  fn new_from_archive<R: std::io::Read + std::io::Seek>(
    parent_path: &str,
    path: &str,
    r_id: &str,
    part_index: Option<usize>,
    archive: &mut zip::ZipArchive<R>,
    visited: &mut std::collections::HashSet<usize>,
  ) -> Result<Self, crate::common::SdkError>;

  fn save_zip<W: std::io::Write + std::io::Seek>(
    &self,
    parent_path: &str,
    zip: &mut zip::ZipWriter<W>,
    entry_set: &mut std::collections::HashSet<String>,
    visited: &mut std::collections::HashSet<String>,
  ) -> Result<(), crate::common::SdkError>;

  #[inline(always)]
  fn relationship_type() -> &'static str {
    Self::RELATIONSHIP_TYPE
  }

  #[inline(always)]
  fn path_prefix() -> &'static str {
    Self::PATH_PREFIX
  }
}

#[cfg(feature = "parts")]
pub trait SdkDataPartReference: Sized {
  const RELATIONSHIP_TYPE: &'static str;

  fn new_from_archive<R: std::io::Read + std::io::Seek>(
    path: &str,
    r_id: &str,
    part_index: usize,
    archive: &mut zip::ZipArchive<R>,
  ) -> Result<Self, crate::common::SdkError>;

  fn save_zip<W: std::io::Write + std::io::Seek>(
    &self,
    parent_path: &str,
    zip: &mut zip::ZipWriter<W>,
    entry_set: &mut std::collections::HashSet<String>,
  ) -> Result<(), crate::common::SdkError>;
}
