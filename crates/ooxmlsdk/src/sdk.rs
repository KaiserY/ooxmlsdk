#[cfg(feature = "parts")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PartDescriptor {
  pub relationship_type: &'static str,
  pub path_prefix: &'static str,
}

#[cfg(feature = "parts")]
pub enum OptionalPartKind {}

#[cfg(feature = "parts")]
pub enum RequiredPartKind {}

#[cfg(feature = "parts")]
pub enum RepeatedPartKind {}

#[cfg(feature = "parts")]
pub struct PartChild<T, C>(std::marker::PhantomData<(T, C)>);

#[cfg(feature = "parts")]
pub type OptionalPart<T> = PartChild<T, OptionalPartKind>;

#[cfg(feature = "parts")]
pub type RequiredPart<T> = PartChild<T, RequiredPartKind>;

#[cfg(feature = "parts")]
pub type RepeatedPart<T> = PartChild<T, RepeatedPartKind>;

#[cfg(feature = "parts")]
impl<T, C> PartChild<T, C> {
  #[inline]
  pub const fn new() -> Self {
    Self(std::marker::PhantomData)
  }
}

#[cfg(feature = "parts")]
impl<T, C> Clone for PartChild<T, C> {
  fn clone(&self) -> Self {
    *self
  }
}

#[cfg(feature = "parts")]
impl<T, C> Copy for PartChild<T, C> {}

#[cfg(feature = "parts")]
impl<T, C> std::fmt::Debug for PartChild<T, C> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_tuple("PartChild").finish()
  }
}

#[cfg(feature = "parts")]
impl<T, C> Default for PartChild<T, C> {
  fn default() -> Self {
    Self::new()
  }
}

#[cfg(feature = "parts")]
impl<T, C> Eq for PartChild<T, C> {}

#[cfg(feature = "parts")]
impl<T, C> PartialEq for PartChild<T, C> {
  fn eq(&self, _other: &Self) -> bool {
    true
  }
}

#[cfg(feature = "parts")]
pub struct PartRoot<T>(std::marker::PhantomData<T>);

#[cfg(feature = "parts")]
impl<T> PartRoot<T> {
  #[inline]
  pub const fn new() -> Self {
    Self(std::marker::PhantomData)
  }
}

#[cfg(feature = "parts")]
impl<T> Clone for PartRoot<T> {
  fn clone(&self) -> Self {
    *self
  }
}

#[cfg(feature = "parts")]
impl<T> Copy for PartRoot<T> {}

#[cfg(feature = "parts")]
impl<T> std::fmt::Debug for PartRoot<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_tuple("PartRoot").finish()
  }
}

#[cfg(feature = "parts")]
impl<T> Default for PartRoot<T> {
  fn default() -> Self {
    Self::new()
  }
}

#[cfg(feature = "parts")]
impl<T> Eq for PartRoot<T> {}

#[cfg(feature = "parts")]
impl<T> PartialEq for PartRoot<T> {
  fn eq(&self, _other: &Self) -> bool {
    true
  }
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
pub trait SdkPackage {
  fn storage(&self) -> &crate::common::SdkPackageStorage;

  fn storage_mut(&mut self) -> &mut crate::common::SdkPackageStorage;

  fn main_part_id(&self) -> Option<crate::common::PartId>;
}

#[cfg(feature = "parts")]
pub trait SdkPartHandle: Copy + Sized + 'static {
  const DESCRIPTOR: PartDescriptor;
  const RELATIONSHIP_TYPE: &'static str = Self::DESCRIPTOR.relationship_type;
  const PATH_PREFIX: &'static str = Self::DESCRIPTOR.path_prefix;

  fn from_part_id(part_id: crate::common::PartId) -> Self;

  fn part_id(self) -> crate::common::PartId;

  #[inline(always)]
  fn relationship_type() -> &'static str {
    Self::RELATIONSHIP_TYPE
  }

  #[inline(always)]
  fn path_prefix() -> &'static str {
    Self::PATH_PREFIX
  }

  #[inline]
  fn relationships<P: SdkPackage>(self, package: &P) -> Option<&crate::common::RelationshipSet> {
    package.storage().relationships(self.part_id())
  }

  #[inline]
  fn target_part_id<P: SdkPackage>(
    self,
    package: &P,
    relationship_id: &str,
  ) -> Option<crate::common::PartId> {
    package
      .storage()
      .target_part_id(self.part_id(), relationship_id)
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
