#[cfg(feature = "parts")]
pub enum OptionalPartKind {}

#[cfg(feature = "parts")]
pub enum RequiredPartKind {}

#[cfg(feature = "parts")]
pub enum RepeatedPartKind {}

#[cfg(feature = "parts")]
const OLE_COMPOUND_FILE_SIGNATURE: [u8; 8] = [0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1];

#[cfg(feature = "parts")]
const ENCRYPTED_PACKAGE_CONTENT_TYPE: &str =
  "application/vnd.openxmlformats-officedocument.encrypted-package";

#[cfg(feature = "parts")]
pub fn is_encrypted_office_file<R>(reader: &mut R) -> Result<bool, crate::common::SdkError>
where
  R: std::io::Read + std::io::Seek,
{
  let original_position = reader.stream_position()?;
  let result = is_encrypted_office_file_inner(reader);
  reader.seek(std::io::SeekFrom::Start(original_position))?;
  result
}

#[cfg(feature = "parts")]
pub fn is_encrypted_office_file_path(
  path: impl AsRef<std::path::Path>,
) -> Result<bool, crate::common::SdkError> {
  let mut file = std::fs::File::open(path)?;
  is_encrypted_office_file(&mut file)
}

#[cfg(feature = "parts")]
fn is_encrypted_office_file_inner<R>(reader: &mut R) -> Result<bool, crate::common::SdkError>
where
  R: std::io::Read + std::io::Seek,
{
  let mut header = [0; 8];
  reader.seek(std::io::SeekFrom::Start(0))?;
  let read = std::io::Read::read(reader, &mut header)?;
  if read == header.len() && header == OLE_COMPOUND_FILE_SIGNATURE {
    return Ok(true);
  }

  reader.seek(std::io::SeekFrom::Start(0))?;
  let Ok(mut archive) = zip::ZipArchive::new(reader) else {
    return Ok(false);
  };
  let Ok(mut content_types) = archive.by_name("[Content_Types].xml") else {
    return Ok(false);
  };
  let mut content_types_xml = String::new();
  if std::io::Read::read_to_string(&mut content_types, &mut content_types_xml).is_err() {
    return Ok(false);
  }

  Ok(
    content_types_xml
      .to_ascii_lowercase()
      .contains(ENCRYPTED_PACKAGE_CONTENT_TYPE),
  )
}

#[cfg(feature = "parts")]
pub struct PartChild<T, C>(std::marker::PhantomData<(T, C)>);

#[cfg(feature = "parts")]
pub type OptionalPart<T> = PartChild<T, OptionalPartKind>;

#[cfg(feature = "parts")]
pub type RequiredPart<T> = PartChild<T, RequiredPartKind>;

#[cfg(feature = "parts")]
pub type RepeatedPart<T> = PartChild<T, RepeatedPartKind>;

/// Internal storage for a statically typed package-part handle.
///
/// Generated part aliases specialize this type so the common part API is
/// defined and type-checked once instead of being emitted once for every part
/// type.
#[cfg(feature = "parts")]
#[doc(hidden)]
pub struct PartHandle<T> {
  key: crate::common::PartKey,
  marker: std::marker::PhantomData<fn() -> T>,
}

#[cfg(feature = "parts")]
impl<T> PartHandle<T> {
  #[inline]
  pub(crate) const fn new(key: crate::common::PartKey) -> Self {
    Self {
      key,
      marker: std::marker::PhantomData,
    }
  }

  #[inline]
  pub(crate) fn resolve(
    &self,
    storage: &crate::common::SdkPackageStorage,
  ) -> Result<crate::common::PartSlot, crate::common::SdkError> {
    self.key.resolve(storage)
  }

  #[inline]
  pub(crate) fn resolve_optional(
    &self,
    storage: &crate::common::SdkPackageStorage,
  ) -> Option<crate::common::PartSlot> {
    self.key.resolve_optional(storage)
  }
}

#[cfg(feature = "parts")]
impl<T> Clone for PartHandle<T> {
  #[inline]
  fn clone(&self) -> Self {
    Self::new(self.key)
  }
}

#[cfg(feature = "parts")]
impl<T> std::fmt::Debug for PartHandle<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.key.fmt(f)
  }
}

#[cfg(feature = "parts")]
impl<T> Eq for PartHandle<T> {}

#[cfg(feature = "parts")]
impl<T> PartialEq for PartHandle<T> {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    self.key == other.key
  }
}

#[cfg(feature = "parts")]
impl<T> std::hash::Hash for PartHandle<T> {
  #[inline]
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.key.hash(state);
  }
}

#[cfg(feature = "parts")]
#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PartConstraint {
  pub child_kind: crate::parts::PartKind,
  pub relationship_type: &'static str,
  pub content_type: &'static str,
  pub min_occurs_is_non_zero: bool,
  pub max_occurs_great_than_one: bool,
}

#[cfg(feature = "parts")]
impl PartConstraint {
  #[inline]
  pub const fn new(
    child_kind: crate::parts::PartKind,
    relationship_type: &'static str,
    content_type: &'static str,
    min_occurs_is_non_zero: bool,
    max_occurs_great_than_one: bool,
  ) -> Self {
    Self {
      child_kind,
      relationship_type,
      content_type,
      min_occurs_is_non_zero,
      max_occurs_great_than_one,
    }
  }
}

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
impl<T, C> std::hash::Hash for PartChild<T, C> {
  fn hash<H: std::hash::Hasher>(&self, _state: &mut H) {}
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
impl<T> std::hash::Hash for PartRoot<T> {
  fn hash<H: std::hash::Hasher>(&self, _state: &mut H) {}
}

#[cfg(feature = "parts")]
fn collect_all_parts_from_relationships<P: SdkPackage + Sized>(
  package: &P,
  relationships: &crate::common::RelationshipSet,
) -> Vec<crate::parts::PartRef> {
  let mut parts = Vec::new();
  let mut visited = vec![false; crate::sdk::SdkPackage::storage(package).parts().len()];

  for relationship in relationships.part_relationships() {
    if let Some(part_id) = relationship.target_part_slot()
      && let Some(is_visited) = visited.get_mut(part_id.index())
      && !*is_visited
      && let Some(part) = crate::parts::PartRef::from_part_slot(package, part_id)
    {
      *is_visited = true;
      parts.push(part);
    }
  }

  let mut current_index = 0;
  while current_index < parts.len() {
    let Some(part_id) = parts[current_index]
      .part_key()
      .resolve_optional(crate::sdk::SdkPackage::storage(package))
    else {
      current_index += 1;
      continue;
    };
    current_index += 1;

    if let Some(relationships) = crate::sdk::SdkPackage::storage(package).relationships(part_id) {
      for relationship in relationships.part_relationships() {
        if let Some(child_part_id) = relationship.target_part_slot()
          && let Some(is_visited) = visited.get_mut(child_part_id.index())
          && !*is_visited
          && let Some(part) = crate::parts::PartRef::from_part_slot(package, child_part_id)
        {
          *is_visited = true;
          parts.push(part);
        }
      }
    }
  }

  parts
}

#[cfg(feature = "parts")]
pub(crate) fn relationship_target_as_part<T: SdkPart>(
  storage: &crate::common::SdkPackageStorage,
  relationship: &crate::common::RelationshipInfo,
) -> Option<T> {
  if !relationship.is_child_part_relationship() {
    return None;
  }
  let part_id = relationship.target_part_slot()?;
  let part = storage.part(part_id)?;
  (part.kind() == T::KIND)
    .then(|| <T as crate::private::SdkPartHandle>::from_part_key(storage.part_key(part_id)))
}

#[cfg(feature = "parts")]
fn unique_relationship_id_for_part(
  relationships: &crate::common::RelationshipSet,
  target_part_slot: crate::common::PartSlot,
) -> Result<&str, crate::common::SdkError> {
  let mut matching = relationships
    .part_relationships()
    .filter(|relationship| relationship.target_part_slot() == Some(target_part_slot));
  let first = matching
    .next()
    .ok_or(crate::common::SdkError::PartNotReferenced)?;
  let Some(_second) = matching.next() else {
    return Ok(first.id());
  };
  Err(crate::common::SdkError::AmbiguousPartRelationship {
    relationship_count: 2 + matching.count(),
  })
}

#[cfg(feature = "parts")]
#[inline]
pub(crate) fn part_from_slot<T: SdkPart>(
  storage: &crate::common::SdkPackageStorage,
  part_slot: crate::common::PartSlot,
) -> T {
  <T as crate::private::SdkPartHandle>::from_part_key(storage.part_key(part_slot))
}

#[cfg(feature = "parts")]
pub fn default_main_part_content_type<T: SdkPart>() -> Option<&'static str> {
  match (T::RELATIONSHIP_TYPE, T::TARGET_NAME) {
    (
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument",
      "document",
    ) => Some("application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"),
    (
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument",
      "workbook",
    ) => Some("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml"),
    (
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument",
      "presentation",
    ) => Some("application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml"),
    _ => None,
  }
}

#[cfg(feature = "parts")]
pub(crate) fn typed_main_part_content_type<T: SdkPart>(
  storage: &crate::common::SdkPackageStorage,
) -> std::borrow::Cow<'static, str> {
  if T::CONTENT_TYPE.is_empty() {
    storage
      .preferred_main_part_content_type()
      .map(|content_type| std::borrow::Cow::Owned(content_type.to_string()))
      .or_else(|| default_main_part_content_type::<T>().map(std::borrow::Cow::Borrowed))
      .unwrap_or(std::borrow::Cow::Borrowed(T::CONTENT_TYPE))
  } else {
    std::borrow::Cow::Borrowed(T::CONTENT_TYPE)
  }
}

#[cfg(feature = "parts")]
fn extension_for_content_type<T: SdkPart>(content_type: &str) -> std::borrow::Cow<'static, str> {
  crate::common::default_part_extension_for_content_type(content_type)
    .map(std::borrow::Cow::Borrowed)
    .unwrap_or(std::borrow::Cow::Borrowed(T::EXTENSION))
}

#[cfg(feature = "parts")]
#[inline]
pub(crate) fn part_content_type_matches_bytes(
  expected_content_type: &[u8],
  actual_content_type: &[u8],
) -> bool {
  expected_content_type == actual_content_type
    || (expected_content_type == b"model/gltf-binary"
      && actual_content_type == b"model/gltf.binary")
}

#[cfg(feature = "parts")]
#[inline]
pub(crate) fn part_root_content_type_matches_bytes(
  root_content_type: &[u8],
  part_content_type: &[u8],
) -> bool {
  if matches!(root_content_type, b"" | b"application/xml" | b"text/xml") {
    return false;
  }
  part_content_type_matches_bytes(root_content_type, part_content_type)
    || match root_content_type {
      b"application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml" => {
        WordprocessingDocumentType::from_content_type_bytes(part_content_type).is_some()
      }
      b"application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml" => {
        SpreadsheetDocumentType::from_content_type_bytes(part_content_type).is_some()
      }
      b"application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml" => {
        PresentationDocumentType::from_content_type_bytes(part_content_type).is_some()
      }
      _ => false,
    }
}

#[cfg(feature = "parts")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum WordprocessingDocumentType {
  #[default]
  Document,
  Template,
  MacroEnabledDocument,
  MacroEnabledTemplate,
}

#[cfg(feature = "parts")]
impl WordprocessingDocumentType {
  pub const fn content_type(self) -> &'static str {
    match self {
      Self::Document => {
        "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"
      }
      Self::Template => {
        "application/vnd.openxmlformats-officedocument.wordprocessingml.template.main+xml"
      }
      Self::MacroEnabledDocument => "application/vnd.ms-word.document.macroEnabled.main+xml",
      Self::MacroEnabledTemplate => {
        "application/vnd.ms-word.template.macroEnabledTemplate.main+xml"
      }
    }
  }

  pub fn from_content_type(content_type: &str) -> Option<Self> {
    Self::from_content_type_bytes(content_type.as_bytes())
  }

  pub fn from_content_type_bytes(content_type: &[u8]) -> Option<Self> {
    match content_type {
      b"application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml" => {
        Some(Self::Document)
      }
      b"application/vnd.openxmlformats-officedocument.wordprocessingml.template.main+xml" => {
        Some(Self::Template)
      }
      b"application/vnd.ms-word.document.macroEnabled.main+xml" => Some(Self::MacroEnabledDocument),
      b"application/vnd.ms-word.template.macroEnabledTemplate.main+xml" => {
        Some(Self::MacroEnabledTemplate)
      }
      _ => None,
    }
  }
}

#[cfg(feature = "parts")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum SpreadsheetDocumentType {
  #[default]
  Workbook,
  Template,
  MacroEnabledWorkbook,
  MacroEnabledTemplate,
  AddIn,
}

#[cfg(feature = "parts")]
impl SpreadsheetDocumentType {
  pub const fn content_type(self) -> &'static str {
    match self {
      Self::Workbook => {
        "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml"
      }
      Self::Template => {
        "application/vnd.openxmlformats-officedocument.spreadsheetml.template.main+xml"
      }
      Self::MacroEnabledWorkbook => "application/vnd.ms-excel.sheet.macroEnabled.main+xml",
      Self::MacroEnabledTemplate => "application/vnd.ms-excel.template.macroEnabled.main+xml",
      Self::AddIn => "application/vnd.ms-excel.addin.macroEnabled.main+xml",
    }
  }

  pub fn from_content_type(content_type: &str) -> Option<Self> {
    Self::from_content_type_bytes(content_type.as_bytes())
  }

  pub fn from_content_type_bytes(content_type: &[u8]) -> Option<Self> {
    match content_type {
      b"application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml" => {
        Some(Self::Workbook)
      }
      b"application/vnd.openxmlformats-officedocument.spreadsheetml.template.main+xml" => {
        Some(Self::Template)
      }
      b"application/vnd.ms-excel.sheet.macroEnabled.main+xml" => Some(Self::MacroEnabledWorkbook),
      b"application/vnd.ms-excel.template.macroEnabled.main+xml" => {
        Some(Self::MacroEnabledTemplate)
      }
      b"application/vnd.ms-excel.addin.macroEnabled.main+xml" => Some(Self::AddIn),
      _ => None,
    }
  }
}

#[cfg(feature = "parts")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum PresentationDocumentType {
  #[default]
  Presentation,
  Template,
  Slideshow,
  MacroEnabledPresentation,
  MacroEnabledTemplate,
  MacroEnabledSlideshow,
  AddIn,
}

#[cfg(feature = "parts")]
impl PresentationDocumentType {
  pub const fn content_type(self) -> &'static str {
    match self {
      Self::Presentation => {
        "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml"
      }
      Self::Template => {
        "application/vnd.openxmlformats-officedocument.presentationml.template.main+xml"
      }
      Self::Slideshow => {
        "application/vnd.openxmlformats-officedocument.presentationml.slideshow.main+xml"
      }
      Self::MacroEnabledPresentation => {
        "application/vnd.ms-powerpoint.presentation.macroEnabled.main+xml"
      }
      Self::MacroEnabledTemplate => "application/vnd.ms-powerpoint.template.macroEnabled.main+xml",
      Self::MacroEnabledSlideshow => {
        "application/vnd.ms-powerpoint.slideshow.macroEnabled.main+xml"
      }
      Self::AddIn => "application/vnd.ms-powerpoint.addin.macroEnabled.main+xml",
    }
  }

  pub fn from_content_type(content_type: &str) -> Option<Self> {
    Self::from_content_type_bytes(content_type.as_bytes())
  }

  pub fn from_content_type_bytes(content_type: &[u8]) -> Option<Self> {
    match content_type {
      b"application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml" => {
        Some(Self::Presentation)
      }
      b"application/vnd.openxmlformats-officedocument.presentationml.template.main+xml" => {
        Some(Self::Template)
      }
      b"application/vnd.openxmlformats-officedocument.presentationml.slideshow.main+xml" => {
        Some(Self::Slideshow)
      }
      b"application/vnd.ms-powerpoint.presentation.macroEnabled.main+xml" => {
        Some(Self::MacroEnabledPresentation)
      }
      b"application/vnd.ms-powerpoint.template.macroEnabled.main+xml" => {
        Some(Self::MacroEnabledTemplate)
      }
      b"application/vnd.ms-powerpoint.slideshow.macroEnabled.main+xml" => {
        Some(Self::MacroEnabledSlideshow)
      }
      b"application/vnd.ms-powerpoint.addin.macroEnabled.main+xml" => Some(Self::AddIn),
      _ => None,
    }
  }
}

pub trait SdkEnum: Sized {
  #[doc(hidden)]
  const TYPE_NAME: &'static str = "SdkEnum";

  fn as_xml_bytes(&self) -> &[u8];

  fn try_from_xml_bytes(value: &[u8]) -> Option<Self> {
    Self::from_xml_bytes(value).ok()
  }

  fn from_xml_bytes(value: &[u8]) -> Result<Self, crate::common::SdkError> {
    Self::try_from_xml_bytes(value).ok_or_else(|| {
      crate::common::invalid_enum_value(
        Self::TYPE_NAME,
        String::from_utf8_lossy(value).into_owned(),
      )
    })
  }
}

#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct SdkTypeRootInfo {
  owner: &'static str,
  local_name: &'static [u8],
  start_tag_open: &'static [u8],
  end_tag: &'static [u8],
  preparse_namespaces: bool,
  writes_xml_header: bool,
  writes_inner_without_prefix: bool,
}

#[doc(hidden)]
impl SdkTypeRootInfo {
  pub const fn new(
    owner: &'static str,
    local_name: &'static [u8],
    start_tag_open: &'static [u8],
    end_tag: &'static [u8],
    preparse_namespaces: bool,
    writes_xml_header: bool,
    writes_inner_without_prefix: bool,
  ) -> Self {
    Self {
      owner,
      local_name,
      start_tag_open,
      end_tag,
      preparse_namespaces,
      writes_xml_header,
      writes_inner_without_prefix,
    }
  }
}

pub trait SdkType: Sized {
  #[doc(hidden)]
  const ROOT_INFO: Option<SdkTypeRootInfo> = None;

  #[inline(never)]
  fn from_bytes(bytes: &[u8]) -> Result<Self, crate::common::SdkError> {
    let Some(root) = Self::ROOT_INFO else {
      return Err(crate::common::SdkError::CommonError(
        "SdkType does not support borrowed deserialization".to_string(),
      ));
    };

    let mut xml_reader = crate::common::from_bytes_inner(bytes);
    let (start, empty) =
      crate::common::read_root_start_borrowed(&mut xml_reader, root.owner, root.local_name)?;
    let mut read_context = crate::common::ReadContext::default();
    if root.preparse_namespaces {
      read_context.enter_root_scope(empty);
    } else {
      read_context.enter_root(&start, empty, crate::common::XmlRead::decoder(&xml_reader))?;
    }
    Self::read_inner(&mut xml_reader, start, empty, &mut read_context)
  }

  #[inline(never)]
  fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let Some(root) = Self::ROOT_INFO else {
      return Err(crate::common::SdkError::CommonError(
        "SdkType does not support IO deserialization".to_string(),
      ));
    };

    let mut xml_reader = crate::common::from_reader_inner(reader);
    let (start, empty) =
      crate::common::read_root_start_io(&mut xml_reader, root.owner, root.local_name)?;
    let mut read_context = crate::common::ReadContext::default();
    if root.preparse_namespaces {
      read_context.enter_root_scope(empty);
    } else {
      read_context.enter_root(&start, empty, crate::common::XmlRead::decoder(&xml_reader))?;
    }
    Self::read_inner(&mut xml_reader, start, empty, &mut read_context)
  }

  #[inline]
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    let Some(root) = Self::ROOT_INFO else {
      return Err(std::io::Error::new(
        std::io::ErrorKind::Unsupported,
        "SdkType does not support root serialization",
      ));
    };

    if root.writes_xml_header {
      writer.write_all(b"<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n")?;
    }
    writer.write_all(root.start_tag_open)?;
    let is_empty = if root.writes_inner_without_prefix {
      Self::write_inner_no_prefix(self, writer)?
    } else {
      Self::write_inner(self, writer)?
    };
    if !is_empty {
      writer.write_all(root.end_tag)?;
    }
    Ok(())
  }

  fn to_xml(&self) -> Result<String, crate::common::SdkError> {
    let mut writer = Vec::with_capacity(32);
    self.write_to(&mut writer)?;
    sdk_type_xml_string(writer)
  }

  fn read_inner<'xml, R: crate::common::XmlRead<'xml>>(
    _xml_reader: &mut R,
    _start: quick_xml::events::BytesStart<'xml>,
    _empty: bool,
    _read_context: &mut crate::common::ReadContext,
  ) -> Result<Self, crate::common::SdkError> {
    Err(crate::common::SdkError::CommonError(
      "SdkType does not support deserialization".to_string(),
    ))
  }

  #[doc(hidden)]
  fn write_inner<W: std::io::Write>(&self, _writer: &mut W) -> Result<bool, std::io::Error> {
    Err(std::io::Error::new(
      std::io::ErrorKind::Unsupported,
      "SdkType does not support serialization",
    ))
  }

  #[doc(hidden)]
  #[inline]
  fn write_inner_no_prefix<W: std::io::Write>(
    &self,
    writer: &mut W,
  ) -> Result<bool, std::io::Error> {
    self.write_inner(writer)
  }
}

pub(crate) trait SdkTypeDisplayWrite {
  fn write_xml_to_vec(&self, writer: &mut Vec<u8>) -> Result<(), std::io::Error>;
}

impl<T: SdkType> SdkTypeDisplayWrite for T {
  #[inline]
  fn write_xml_to_vec(&self, writer: &mut Vec<u8>) -> Result<(), std::io::Error> {
    self.write_to(writer)
  }
}

#[inline(never)]
pub(crate) fn fmt_sdk_type(
  value: &dyn SdkTypeDisplayWrite,
  f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
  let mut writer = Vec::with_capacity(32);
  value
    .write_xml_to_vec(&mut writer)
    .map_err(|_| std::fmt::Error)?;
  let xml = sdk_type_xml_string(writer).map_err(|_| std::fmt::Error)?;
  f.write_str(&xml)
}

#[inline(never)]
fn sdk_type_xml_string(writer: Vec<u8>) -> Result<String, crate::common::SdkError> {
  match String::from_utf8(writer) {
    Ok(xml) => Ok(xml),
    Err(err) => Err(crate::common::SdkError::CommonError(format!(
      "invalid utf-8 xml: {err}"
    ))),
  }
}

impl<T: SdkType> SdkType for Box<T> {
  #[inline]
  fn from_bytes(bytes: &[u8]) -> Result<Self, crate::common::SdkError> {
    T::from_bytes(bytes).map(Box::new)
  }

  #[inline]
  fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    T::from_reader(reader).map(Box::new)
  }

  #[inline]
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.as_ref().write_to(writer)
  }

  #[inline]
  fn read_inner<'xml, R: crate::common::XmlRead<'xml>>(
    xml_reader: &mut R,
    start: quick_xml::events::BytesStart<'xml>,
    empty: bool,
    read_context: &mut crate::common::ReadContext,
  ) -> Result<Self, crate::common::SdkError> {
    T::read_inner(xml_reader, start, empty, read_context).map(Box::new)
  }

  #[inline]
  fn write_inner<W: std::io::Write>(&self, writer: &mut W) -> Result<bool, std::io::Error> {
    T::write_inner(self.as_ref(), writer)
  }

  #[inline]
  fn write_inner_no_prefix<W: std::io::Write>(
    &self,
    writer: &mut W,
  ) -> Result<bool, std::io::Error> {
    T::write_inner_no_prefix(self.as_ref(), writer)
  }
}

#[cfg(feature = "parts")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AlternativeFormatImportPartType {
  Html,
  Xhtml,
  Xml,
  WordprocessingMl,
}

#[cfg(feature = "parts")]
impl AlternativeFormatImportPartType {
  #[inline]
  pub const fn content_type(self) -> &'static str {
    match self {
      Self::Html => "text/html",
      Self::Xhtml => "application/xhtml+xml",
      Self::Xml => "application/xml",
      Self::WordprocessingMl => {
        "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"
      }
    }
  }
}

#[cfg(feature = "parts")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CustomXmlPartType {
  AdditionalCharacteristicsInfo,
  Bibliography,
  CustomXml,
  InkContent,
}

#[cfg(feature = "parts")]
impl CustomXmlPartType {
  #[inline]
  pub const fn content_type(self) -> &'static str {
    match self {
      Self::AdditionalCharacteristicsInfo | Self::Bibliography | Self::CustomXml => {
        "application/xml"
      }
      Self::InkContent => "application/inkml+xml",
    }
  }
}

#[cfg(feature = "parts")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CustomPropertyPartType {
  Spreadsheet,
  Xml,
}

#[cfg(feature = "parts")]
impl CustomPropertyPartType {
  #[inline]
  pub const fn content_type(self) -> &'static str {
    match self {
      Self::Spreadsheet => {
        "application/vnd.openxmlformats-officedocument.spreadsheetml.customProperty"
      }
      Self::Xml => "application/xml",
    }
  }

  #[inline]
  pub const fn extension(self) -> &'static str {
    ".xml"
  }
}

#[cfg(feature = "parts")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EmbeddedObjectPartType {
  Binary,
}

#[cfg(feature = "parts")]
impl EmbeddedObjectPartType {
  #[inline]
  pub const fn content_type(self) -> &'static str {
    match self {
      Self::Binary => "application/vnd.openxmlformats-officedocument.oleObject",
    }
  }

  #[inline]
  pub const fn extension(self) -> &'static str {
    ".bin"
  }
}

#[cfg(feature = "parts")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EmbeddedPackagePartType {
  Docm,
  Docx,
  Dotm,
  Dotx,
  Potm,
  Potx,
  Ppam,
  Ppsm,
  Ppsx,
  Pptm,
  Pptx,
  Sldm,
  Sldx,
  Thmx,
  Xlam,
  Xlsb,
  Xlsm,
  Xlsx,
  Xltm,
  Xltx,
}

#[cfg(feature = "parts")]
impl EmbeddedPackagePartType {
  #[inline]
  pub const fn content_type(self) -> &'static str {
    match self {
      Self::Docm => "application/vnd.ms-word.document.macroEnabled.12",
      Self::Docx => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
      Self::Dotm => "application/vnd.ms-word.template.macroEnabled.12",
      Self::Dotx => "application/vnd.openxmlformats-officedocument.wordprocessingml.template",
      Self::Potm => "application/vnd.ms-powerpoint.template.macroEnabled.12",
      Self::Potx => "application/vnd.openxmlformats-officedocument.presentationml.template",
      Self::Ppam => "application/vnd.ms-powerpoint.addin.macroEnabled.12",
      Self::Ppsm => "application/vnd.ms-powerpoint.slideshow.macroEnabled.12",
      Self::Ppsx => "application/vnd.openxmlformats-officedocument.presentationml.slideshow",
      Self::Pptm => "application/vnd.ms-powerpoint.presentation.macroEnabled.12",
      Self::Pptx => "application/vnd.openxmlformats-officedocument.presentationml.presentation",
      Self::Sldm => "application/vnd.ms-powerpoint.slide.macroEnabled.12",
      Self::Sldx => "application/vnd.openxmlformats-officedocument.presentationml.slide",
      Self::Thmx => "application/vnd.ms-officetheme",
      Self::Xlam => "application/vnd.ms-excel.addin.macroEnabled.12",
      Self::Xlsb => "application/vnd.ms-excel.sheet.binary.macroEnabled.12",
      Self::Xlsm => "application/vnd.ms-excel.sheet.macroEnabled.12",
      Self::Xlsx => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
      Self::Xltm => "application/vnd.ms-excel.template.macroEnabled.12",
      Self::Xltx => "application/vnd.openxmlformats-officedocument.spreadsheetml.template",
    }
  }

  #[inline]
  pub const fn extension(self) -> &'static str {
    match self {
      Self::Docm => ".docm",
      Self::Docx => ".docx",
      Self::Dotm => ".dotm",
      Self::Dotx => ".dotx",
      Self::Potm => ".potm",
      Self::Potx => ".potx",
      Self::Ppam => ".ppam",
      Self::Ppsm => ".ppsm",
      Self::Ppsx => ".ppsx",
      Self::Pptm => ".pptm",
      Self::Pptx => ".pptx",
      Self::Sldm => ".sldm",
      Self::Sldx => ".sldx",
      Self::Thmx => ".thmx",
      Self::Xlam => ".xlam",
      Self::Xlsb => ".xlsb",
      Self::Xlsm => ".xlsm",
      Self::Xlsx => ".xlsx",
      Self::Xltm => ".xltm",
      Self::Xltx => ".xltx",
    }
  }
}

#[cfg(feature = "parts")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FontPartType {
  FontData,
  FontTtf,
  FontOdttf,
}

#[cfg(feature = "parts")]
impl FontPartType {
  #[inline]
  pub const fn content_type(self) -> &'static str {
    match self {
      Self::FontData => "application/x-fontdata",
      Self::FontTtf => "application/x-font-ttf",
      Self::FontOdttf => "application/vnd.openxmlformats-officedocument.obfuscatedFont",
    }
  }

  #[inline]
  pub const fn extension(self) -> &'static str {
    match self {
      Self::FontData => ".fntdata",
      Self::FontTtf => ".ttf",
      Self::FontOdttf => ".odttf",
    }
  }
}

#[cfg(feature = "parts")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MailMergeRecipientDataPartType {
  OpenXmlMailMergeRecipientData,
  MsWordMailMergeRecipientData,
}

#[cfg(feature = "parts")]
impl MailMergeRecipientDataPartType {
  #[inline]
  pub const fn content_type(self) -> &'static str {
    match self {
      Self::OpenXmlMailMergeRecipientData => {
        "application/vnd.openxmlformats-officedocument.wordprocessingml.mailMergeRecipientData+xml"
      }
      Self::MsWordMailMergeRecipientData => "application/vnd.ms-word.mailMergeRecipientData+xml",
    }
  }

  #[inline]
  pub const fn extension(self) -> &'static str {
    ".xml"
  }
}

#[cfg(feature = "parts")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EmbeddedControlPersistenceBinaryDataPartType {
  ActiveXBin,
}

#[cfg(feature = "parts")]
impl EmbeddedControlPersistenceBinaryDataPartType {
  #[inline]
  pub const fn content_type(self) -> &'static str {
    match self {
      Self::ActiveXBin => "application/vnd.ms-office.activeX",
    }
  }

  #[inline]
  pub const fn extension(self) -> &'static str {
    ".bin"
  }
}

#[cfg(feature = "parts")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EmbeddedControlPersistencePartType {
  ActiveX,
  ActiveXBin,
}

#[cfg(feature = "parts")]
impl EmbeddedControlPersistencePartType {
  #[inline]
  pub const fn content_type(self) -> &'static str {
    match self {
      Self::ActiveX => "application/vnd.ms-office.activeX+xml",
      Self::ActiveXBin => "application/vnd.ms-office.activeX",
    }
  }

  #[inline]
  pub const fn extension(self) -> &'static str {
    match self {
      Self::ActiveX => ".xml",
      Self::ActiveXBin => ".bin",
    }
  }
}

#[cfg(feature = "parts")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ThumbnailPartType {
  Jpeg,
  Emf,
  Wmf,
}

#[cfg(feature = "parts")]
impl ThumbnailPartType {
  #[inline]
  pub const fn content_type(self) -> &'static str {
    match self {
      Self::Jpeg => "image/jpeg",
      Self::Emf => "image/x-emf",
      Self::Wmf => "image/x-wmf",
    }
  }

  #[inline]
  pub const fn extension(self) -> &'static str {
    match self {
      Self::Jpeg => ".jpg",
      Self::Emf => ".emf",
      Self::Wmf => ".wmf",
    }
  }
}

#[cfg(feature = "parts")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MediaDataPartType {
  Aiff,
  Midi,
  Mp3,
  MpegUrl,
  Wav,
  Wma,
  MpegAudio,
  OggAudio,
  Asx,
  Avi,
  Mpg,
  MpegVideo,
  Wmv,
  Wmx,
  Wvx,
  Quicktime,
  OggVideo,
  Vc1,
  Mp4,
}

#[cfg(feature = "parts")]
impl MediaDataPartType {
  #[inline]
  pub const fn content_type(self) -> &'static str {
    match self {
      Self::Aiff => "audio/aiff",
      Self::Midi => "audio/midi",
      Self::Mp3 => "audio/mp3",
      Self::MpegUrl => "audio/mpegurl",
      Self::Wav => "audio/wav",
      Self::Wma => "audio/x-ms-wma",
      Self::MpegAudio => "audio/mpeg",
      Self::OggAudio => "audio/ogg",
      Self::Asx => "video/x-ms-asf-plugin",
      Self::Avi => "video/avi",
      Self::Mpg => "video/mpg",
      Self::MpegVideo => "video/mpeg",
      Self::Wmv => "video/x-ms-wmv",
      Self::Wmx => "video/x-ms-wmx",
      Self::Wvx => "video/x-ms-wvx",
      Self::Quicktime => "video/quicktime",
      Self::OggVideo => "video/ogg",
      Self::Vc1 => "video/vc1",
      Self::Mp4 => "video/mp4",
    }
  }

  #[inline]
  pub const fn extension(self) -> &'static str {
    match self {
      Self::Aiff => ".aiff",
      Self::Midi => ".midi",
      Self::Mp3 => ".mp3",
      Self::MpegUrl => ".m3u",
      Self::Wav => ".wav",
      Self::Wma => ".wma",
      Self::MpegAudio => ".mpeg",
      Self::OggAudio => ".ogg",
      Self::Asx => ".asx",
      Self::Avi => ".avi",
      Self::Mpg => ".mpg",
      Self::MpegVideo => ".mpeg",
      Self::Wmv => ".wmv",
      Self::Wmx => ".wmx",
      Self::Wvx => ".wvx",
      Self::Quicktime => ".mov",
      Self::OggVideo => ".ogg",
      Self::Vc1 => ".wmv",
      Self::Mp4 => ".mp4",
    }
  }
}

#[cfg(feature = "parts")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum PackageOpenMode {
  Eager,
  #[default]
  Lazy,
}

#[cfg(feature = "parts")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum FileFormatVersion {
  Office2007,
  Office2010,
  Office2013,
  Office2016,
  Office2019,
  Office2021,
  #[default]
  Microsoft365,
}

#[cfg(feature = "parts")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum MarkupCompatibilityProcessMode {
  #[default]
  NoProcess,
  #[cfg(feature = "mce")]
  ProcessLoadedPartsOnly,
  #[cfg(feature = "mce")]
  ProcessAllParts,
}

#[cfg(feature = "parts")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MarkupCompatibilityProcessSettings {
  pub process_mode: MarkupCompatibilityProcessMode,
  pub target_file_format_version: FileFormatVersion,
}

#[cfg(feature = "parts")]
impl Default for MarkupCompatibilityProcessSettings {
  #[inline]
  fn default() -> Self {
    Self {
      process_mode: MarkupCompatibilityProcessMode::NoProcess,
      target_file_format_version: FileFormatVersion::Office2007,
    }
  }
}

#[cfg(feature = "parts")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OpenSettings {
  pub open_mode: PackageOpenMode,
  pub markup_compatibility_process_settings: MarkupCompatibilityProcessSettings,
  pub ignore_calculation_chain_part_relationship: bool,
}

#[cfg(feature = "parts")]
impl OpenSettings {
  #[inline]
  pub(crate) fn root_element_open_mode(self) -> PackageOpenMode {
    #[cfg(feature = "mce")]
    if matches!(
      self.markup_compatibility_process_settings.process_mode,
      MarkupCompatibilityProcessMode::ProcessAllParts
    ) {
      return PackageOpenMode::Eager;
    }

    self.open_mode
  }
}

#[cfg(feature = "parts")]
#[derive(Debug)]
pub(crate) struct PartRootCache {
  entries: Vec<std::sync::OnceLock<crate::parts::PartRootElement>>,
}

#[cfg(feature = "parts")]
impl PartRootCache {
  #[inline]
  pub(crate) fn with_len(len: usize) -> Self {
    Self {
      entries: (0..len).map(|_| std::sync::OnceLock::new()).collect(),
    }
  }

  #[inline]
  fn cell(
    &self,
    part_slot: crate::common::PartSlot,
  ) -> Option<&std::sync::OnceLock<crate::parts::PartRootElement>> {
    self.entries.get(part_slot.index())
  }

  #[inline]
  fn cell_mut(
    &mut self,
    part_slot: crate::common::PartSlot,
  ) -> Option<&mut std::sync::OnceLock<crate::parts::PartRootElement>> {
    self.entries.get_mut(part_slot.index())
  }

  #[inline]
  pub(crate) fn get(
    &self,
    part_slot: crate::common::PartSlot,
  ) -> Option<&crate::parts::PartRootElement> {
    self.cell(part_slot)?.get()
  }

  #[inline]
  pub(crate) fn get_mut(
    &mut self,
    part_slot: crate::common::PartSlot,
  ) -> Option<&mut crate::parts::PartRootElement> {
    self.cell_mut(part_slot)?.get_mut()
  }

  #[inline]
  pub(crate) fn cache_loaded(
    &self,
    part_slot: crate::common::PartSlot,
    root_element: crate::parts::PartRootElement,
  ) -> Option<&crate::parts::PartRootElement> {
    let cell = self.cell(part_slot)?;
    let _ = cell.set(root_element);
    cell.get()
  }

  #[inline]
  pub(crate) fn requires_serialization(&self, part_slot: crate::common::PartSlot) -> bool {
    self
      .cell(part_slot)
      .is_some_and(|cell| cell.get().is_some())
  }

  #[inline]
  pub(crate) fn replace(
    &mut self,
    part_slot: crate::common::PartSlot,
    root_element: crate::parts::PartRootElement,
  ) -> bool {
    let Some(cell) = self.cell_mut(part_slot) else {
      return false;
    };
    let _ = cell.take();
    cell
      .set(root_element)
      .expect("empty root cache cell must accept a value");
    true
  }

  #[inline]
  pub(crate) fn take(
    &mut self,
    part_slot: crate::common::PartSlot,
  ) -> Option<crate::parts::PartRootElement> {
    self.cell_mut(part_slot)?.take()
  }

  #[inline]
  pub(crate) fn push_empty(&mut self) {
    self.entries.push(std::sync::OnceLock::new());
  }
}

#[cfg(feature = "parts")]
pub trait SdkPackage: Sized + 'static {
  #[doc(hidden)]
  const CHILD_PART_CONSTRAINTS: &'static [PartConstraint];

  #[doc(hidden)]
  fn child_part_constraint(kind: crate::parts::PartKind) -> Option<PartConstraint>;

  #[doc(hidden)]
  fn storage(&self) -> &crate::common::SdkPackageStorage;

  #[doc(hidden)]
  fn storage_mut(&mut self) -> &mut crate::common::SdkPackageStorage;

  #[inline]
  fn open_settings(&self) -> &OpenSettings {
    static DEFAULT_SETTINGS: OpenSettings = OpenSettings {
      open_mode: PackageOpenMode::Lazy,
      markup_compatibility_process_settings: MarkupCompatibilityProcessSettings {
        process_mode: MarkupCompatibilityProcessMode::NoProcess,
        target_file_format_version: FileFormatVersion::Office2007,
      },
      ignore_calculation_chain_part_relationship: false,
    };
    &DEFAULT_SETTINGS
  }

  #[inline]
  #[doc(hidden)]
  fn relationships(&self) -> &crate::common::RelationshipSet {
    crate::sdk::SdkPackage::storage(self).package_relationships()
  }

  #[inline]
  #[doc(hidden)]
  fn relationships_mut(&mut self) -> &mut crate::common::RelationshipSet {
    crate::sdk::SdkPackage::storage_mut(self).package_relationships_mut()
  }

  #[inline]
  #[doc(hidden)]
  fn validate_child_part<T: SdkPart>(
    &self,
    content_type: &str,
  ) -> Result<PartConstraint, crate::common::SdkError> {
    validate_child_part_constraint::<T>(
      Self::child_part_constraint(T::KIND),
      false,
      Self::relationships(self),
      content_type,
    )
  }

  #[doc(hidden)]
  fn root_element(
    &self,
    part_id: crate::common::PartSlot,
  ) -> Option<&crate::parts::PartRootElement>;

  #[doc(hidden)]
  fn cache_root_element(
    &self,
    part_id: crate::common::PartSlot,
    root_element: crate::parts::PartRootElement,
  ) -> Option<&crate::parts::PartRootElement>;

  #[doc(hidden)]
  fn root_element_requires_serialization(&self, part_id: crate::common::PartSlot) -> bool;

  #[doc(hidden)]
  fn root_element_mut(
    &mut self,
    part_id: crate::common::PartSlot,
  ) -> Option<&mut crate::parts::PartRootElement>;

  #[doc(hidden)]
  fn replace_root_element(
    &mut self,
    part_id: crate::common::PartSlot,
    root_element: crate::parts::PartRootElement,
  ) -> bool;

  #[doc(hidden)]
  fn take_root_element(
    &mut self,
    part_id: crate::common::PartSlot,
  ) -> Option<crate::parts::PartRootElement>;

  #[doc(hidden)]
  fn push_root_element_slot(&mut self);

  #[inline]
  #[doc(hidden)]
  fn is_root_element_loaded(&self, part_id: crate::common::PartSlot) -> bool {
    self.root_element(part_id).is_some()
  }

  #[inline]
  #[doc(hidden)]
  fn unload_root_element(
    &mut self,
    part_id: crate::common::PartSlot,
  ) -> Option<crate::parts::PartRootElement> {
    self.take_root_element(part_id)
  }

  #[inline]
  #[doc(hidden)]
  fn part_bytes_for_copy(
    &self,
    part_id: crate::common::PartSlot,
  ) -> Result<Vec<u8>, crate::common::SdkError> {
    if self.root_element_requires_serialization(part_id) {
      return self
        .root_element(part_id)
        .ok_or(crate::common::SdkError::StalePart)?
        .to_bytes();
    }
    Ok(Self::storage(self).part_bytes(part_id)?.to_vec())
  }

  #[inline]
  fn load_all_parts(&mut self) -> Result<(), crate::common::SdkError>
  where
    Self: Sized,
  {
    crate::parts::load_all_part_roots(self)
  }

  #[cfg(feature = "flat-opc")]
  #[inline]
  fn write_flat_opc_to<W: std::io::Write>(
    &self,
    writer: &mut W,
  ) -> Result<(), crate::common::SdkError> {
    crate::sdk::SdkPackage::storage(self).write_flat_opc(writer, |part_id, _part| {
      if crate::sdk::SdkPackage::root_element_requires_serialization(self, part_id) {
        return crate::sdk::SdkPackage::root_element(self, part_id)
          .ok_or(crate::common::SdkError::StalePart)?
          .to_bytes();
      }
      Ok(
        crate::sdk::SdkPackage::storage(self)
          .part_bytes(part_id)?
          .to_vec(),
      )
    })
  }

  #[inline]
  fn add_external_relationship(
    &mut self,
    relationship_id: impl Into<String>,
    relationship_type: impl Into<String>,
    target: impl Into<String>,
  ) -> Result<crate::common::RelationshipRef<'_>, crate::common::SdkError> {
    let relationship_id = relationship_id.into();
    crate::sdk::SdkPackage::relationships_mut(self).add_external_relationship(
      relationship_id.clone(),
      relationship_type,
      target,
    )?;
    Ok(crate::common::RelationshipRef::new(
      crate::sdk::SdkPackage::storage(self).token(),
      crate::sdk::SdkPackage::relationships(self)
        .get(&relationship_id)
        .expect("relationship was just added"),
    ))
  }

  #[inline]
  fn add_external_relationship_auto_id(
    &mut self,
    relationship_type: impl Into<String>,
    target: impl Into<String>,
  ) -> Result<crate::common::RelationshipRef<'_>, crate::common::SdkError> {
    let relationship_id = crate::sdk::SdkPackage::relationships(self).next_relationship_id();
    self.add_external_relationship(relationship_id, relationship_type, target)
  }

  #[inline]
  fn add_hyperlink_relationship(
    &mut self,
    relationship_id: impl Into<String>,
    target: impl Into<String>,
  ) -> Result<crate::common::RelationshipRef<'_>, crate::common::SdkError> {
    let relationship_id = relationship_id.into();
    crate::sdk::SdkPackage::relationships_mut(self)
      .add_hyperlink_relationship(relationship_id.clone(), target)?;
    Ok(crate::common::RelationshipRef::new(
      crate::sdk::SdkPackage::storage(self).token(),
      crate::sdk::SdkPackage::relationships(self)
        .get(&relationship_id)
        .expect("relationship was just added"),
    ))
  }

  #[inline]
  fn add_hyperlink_relationship_with_mode(
    &mut self,
    relationship_id: impl Into<String>,
    target: impl Into<String>,
    target_mode: crate::schemas::opc_relationships::TargetMode,
  ) -> Result<crate::common::RelationshipRef<'_>, crate::common::SdkError> {
    let relationship_id = relationship_id.into();
    crate::sdk::SdkPackage::relationships_mut(self).add_hyperlink_relationship_with_mode(
      relationship_id.clone(),
      target,
      target_mode,
    )?;
    Ok(crate::common::RelationshipRef::new(
      crate::sdk::SdkPackage::storage(self).token(),
      crate::sdk::SdkPackage::relationships(self)
        .get(&relationship_id)
        .expect("relationship was just added"),
    ))
  }

  #[inline]
  fn add_hyperlink_relationship_auto_id(
    &mut self,
    target: impl Into<String>,
    target_mode: crate::schemas::opc_relationships::TargetMode,
  ) -> Result<crate::common::RelationshipRef<'_>, crate::common::SdkError> {
    let relationship_id = crate::sdk::SdkPackage::relationships(self).next_relationship_id();
    self.add_hyperlink_relationship_with_mode(relationship_id, target, target_mode)
  }

  #[inline]
  fn get_reference_relationship(
    &self,
    relationship_id: &str,
  ) -> Option<crate::common::RelationshipRef<'_>> {
    let package_token = crate::sdk::SdkPackage::storage(self).token();
    crate::sdk::SdkPackage::relationships(self)
      .get(relationship_id)
      .filter(|relationship| relationship.is_reference_relationship())
      .map(|relationship| crate::common::RelationshipRef::new(package_token, relationship))
  }

  #[inline]
  fn get_external_relationship(
    &self,
    relationship_id: &str,
  ) -> Option<crate::common::RelationshipRef<'_>> {
    let package_token = crate::sdk::SdkPackage::storage(self).token();
    crate::sdk::SdkPackage::relationships(self)
      .get_external_relationship(relationship_id)
      .map(|relationship| crate::common::RelationshipRef::new(package_token, relationship))
  }

  #[inline]
  fn get_hyperlink_relationship(
    &self,
    relationship_id: &str,
  ) -> Option<crate::common::RelationshipRef<'_>> {
    let package_token = crate::sdk::SdkPackage::storage(self).token();
    crate::sdk::SdkPackage::relationships(self)
      .get_hyperlink_relationship(relationship_id)
      .map(|relationship| crate::common::RelationshipRef::new(package_token, relationship))
  }

  #[inline]
  fn delete_reference_relationship(
    &mut self,
    relationship_id: &str,
  ) -> Result<crate::common::Relationship, crate::common::SdkError> {
    let package_token = crate::sdk::SdkPackage::storage(self).token();
    let relationship = crate::sdk::SdkPackage::relationships_mut(self)
      .remove_reference_relationship(relationship_id)?;
    Ok(crate::common::Relationship::new(
      package_token,
      relationship,
    ))
  }

  #[inline]
  fn delete_external_relationship(
    &mut self,
    relationship_id: &str,
  ) -> Result<crate::common::Relationship, crate::common::SdkError> {
    let package_token = crate::sdk::SdkPackage::storage(self).token();
    let relationship = crate::sdk::SdkPackage::relationships_mut(self)
      .remove_external_relationship(relationship_id)?;
    Ok(crate::common::Relationship::new(
      package_token,
      relationship,
    ))
  }

  #[inline]
  fn change_relationship_id(
    &mut self,
    relationship_id: &str,
    new_relationship_id: impl Into<String>,
  ) -> Result<(), crate::common::SdkError> {
    crate::sdk::SdkPackage::relationships_mut(self)
      .change_relationship_id(relationship_id, new_relationship_id)?;
    Ok(())
  }

  #[inline]
  fn external_relationships(&self) -> impl Iterator<Item = crate::common::RelationshipRef<'_>> {
    let package_token = crate::sdk::SdkPackage::storage(self).token();
    crate::sdk::SdkPackage::relationships(self)
      .external_relationships()
      .map(move |relationship| crate::common::RelationshipRef::new(package_token, relationship))
  }

  #[inline]
  fn hyperlink_relationships(&self) -> impl Iterator<Item = crate::common::RelationshipRef<'_>> {
    let package_token = crate::sdk::SdkPackage::storage(self).token();
    crate::sdk::SdkPackage::relationships(self)
      .hyperlink_relationships()
      .map(move |relationship| crate::common::RelationshipRef::new(package_token, relationship))
  }

  #[inline]
  fn data_part_reference_relationships(
    &self,
  ) -> impl Iterator<Item = crate::common::RelationshipRef<'_>> {
    let package_token = crate::sdk::SdkPackage::storage(self).token();
    crate::sdk::SdkPackage::relationships(self)
      .data_part_reference_relationships()
      .map(move |relationship| crate::common::RelationshipRef::new(package_token, relationship))
  }

  #[inline]
  fn media_data_parts(&self) -> impl Iterator<Item = crate::common::MediaDataPart> + '_ {
    crate::sdk::SdkPackage::storage(self)
      .media_data_parts()
      .map(|(part_id, _part)| {
        crate::common::MediaDataPart::from_part_slot(
          crate::sdk::SdkPackage::storage(self).token(),
          part_id,
        )
      })
  }

  #[inline]
  fn delete_unused_media_data_parts(&mut self) -> usize {
    crate::sdk::SdkPackage::storage_mut(self).delete_unused_media_data_parts()
  }

  #[inline]
  fn parts(&self) -> impl Iterator<Item = crate::parts::IdPartPair<'_>> + '_
  where
    Self: Sized,
  {
    crate::sdk::SdkPackage::relationships(self)
      .part_relationships()
      .filter_map(|relationship| {
        let part_id = relationship.target_part_slot()?;
        let part = crate::parts::PartRef::from_part_slot(self, part_id)?;
        Some(crate::parts::IdPartPair::new(relationship.id(), part))
      })
  }

  #[inline]
  fn get_all_parts(&self) -> impl Iterator<Item = crate::parts::PartRef> + '_
  where
    Self: Sized,
  {
    collect_all_parts_from_relationships(self, crate::sdk::SdkPackage::relationships(self))
      .into_iter()
  }

  #[inline]
  fn get_part_by_id(&self, relationship_id: &str) -> Option<crate::parts::PartRef>
  where
    Self: Sized,
  {
    let relationship = crate::sdk::SdkPackage::relationships(self).get(relationship_id)?;
    if !relationship.is_child_part_relationship() {
      return None;
    }
    let part_slot = relationship.target_part_slot()?;
    crate::parts::PartRef::from_part_slot(self, part_slot)
  }

  #[inline]
  fn try_get_part_by_id(
    &self,
    relationship_id: &str,
  ) -> Result<crate::parts::PartRef, crate::common::SdkError>
  where
    Self: Sized,
  {
    let part_slot = crate::sdk::SdkPackage::relationships(self)
      .get(relationship_id)
      .filter(|relationship| relationship.is_child_part_relationship())
      .and_then(crate::common::RelationshipInfo::target_part_slot)
      .ok_or_else(|| crate::common::SdkError::PartRelationshipNotFound {
        relationship_id: relationship_id.to_string(),
      })?;
    crate::parts::PartRef::from_part_slot(self, part_slot).ok_or(crate::common::SdkError::StalePart)
  }

  #[inline]
  fn get_parts_of_type<T: SdkPart>(&self) -> impl Iterator<Item = T> + '_
  where
    Self: Sized,
  {
    let storage = crate::sdk::SdkPackage::storage(self);
    crate::sdk::SdkPackage::relationships(self)
      .part_relationships()
      .filter_map(move |relationship| relationship_target_as_part::<T>(storage, relationship))
  }

  #[inline]
  fn related_parts_of_type<T: SdkPart>(&self) -> impl Iterator<Item = RelatedPart<'_, T>> + '_
  where
    Self: Sized,
  {
    let storage = crate::sdk::SdkPackage::storage(self);
    crate::sdk::SdkPackage::relationships(self)
      .part_relationships()
      .filter_map(move |relationship| {
        relationship_target_as_part::<T>(storage, relationship)
          .map(|part| RelatedPart::new(relationship.id(), relationship.relationship_type(), part))
      })
  }

  /// Returns the first matching relationship ID in package relationship order.
  #[inline]
  fn get_id_of_part<T: SdkPart>(&self, part: &T) -> Result<&str, crate::common::SdkError> {
    let target_part_slot = crate::private::SdkPartHandle::part_slot(part, self)?;
    crate::sdk::SdkPackage::relationships(self)
      .part_relationships()
      .find_map(|relationship| {
        (relationship.target_part_slot() == Some(target_part_slot)).then_some(relationship.id())
      })
      .ok_or(crate::common::SdkError::PartNotReferenced)
  }

  #[inline]
  fn change_id_of_part<T: SdkPart>(
    &mut self,
    part: &T,
    new_relationship_id: impl Into<String>,
  ) -> Result<String, crate::common::SdkError> {
    let target_part_slot = crate::private::SdkPartHandle::part_slot(part, self)?;
    let old_relationship_id = unique_relationship_id_for_part(
      crate::sdk::SdkPackage::relationships(self),
      target_part_slot,
    )?
    .to_string();
    self.change_relationship_id(&old_relationship_id, new_relationship_id)?;
    Ok(old_relationship_id)
  }

  #[inline]
  fn delete_part_by_id(&mut self, relationship_id: &str) -> Result<bool, crate::common::SdkError> {
    let Some(deleted_part_slots) =
      crate::sdk::SdkPackage::storage_mut(self).delete_package_part(relationship_id)?
    else {
      return Ok(false);
    };
    for part_slot in deleted_part_slots {
      let _ = crate::sdk::SdkPackage::take_root_element(self, part_slot);
    }
    Ok(true)
  }

  #[inline]
  fn delete_part<T: SdkPart>(&mut self, part: T) -> Result<bool, crate::common::SdkError> {
    let relationship_id = self.get_id_of_part(&part)?.to_string();
    self.delete_part_by_id(&relationship_id)
  }

  #[inline]
  fn delete_parts<T, I>(&mut self, parts: I) -> Result<(), crate::common::SdkError>
  where
    T: SdkPart,
    I: IntoIterator<Item = T>,
  {
    let relationship_ids = parts
      .into_iter()
      .map(|part| self.get_id_of_part(&part).map(str::to_string))
      .collect::<Result<Vec<_>, _>>()?;
    for relationship_id in relationship_ids {
      self.delete_part_by_id(&relationship_id)?;
    }
    Ok(())
  }

  #[inline]
  fn add_part<T: SdkPart>(&mut self, part: T) -> Result<T, crate::common::SdkError> {
    let part_slot = crate::private::SdkPartHandle::part_slot(&part, self)?;
    if crate::sdk::SdkPackage::relationships(self)
      .part_relationships()
      .any(|relationship| relationship.target_part_slot() == Some(part_slot))
    {
      return Ok(part);
    }
    let relationship_id = crate::sdk::SdkPackage::relationships(self).next_relationship_id();
    self.add_part_with_id(part, relationship_id)
  }

  #[inline]
  fn add_part_with_id<T: SdkPart>(
    &mut self,
    part: T,
    relationship_id: impl Into<String>,
  ) -> Result<T, crate::common::SdkError> {
    let relationship_id = relationship_id.into();
    let part_id = crate::private::SdkPartHandle::part_slot(&part, self)?;
    let content_type = crate::sdk::SdkPackage::storage(self)
      .part(part_id)
      .ok_or(crate::common::SdkError::StalePart)?
      .content_type()
      .to_string();
    let constraint = self.validate_child_part::<T>(&content_type)?;
    let relationship_type = constrained_relationship_type(
      constraint,
      crate::sdk::SdkPackage::storage(self)
        .part(part_id)
        .and_then(crate::common::StoredPart::relationship_type),
    );
    crate::sdk::SdkPackage::storage_mut(self).add_package_relationship_to_part(
      relationship_id.clone(),
      relationship_type.as_ref(),
      part_id,
    )?;
    Ok(part_from_slot(
      crate::sdk::SdkPackage::storage(self),
      part_id,
    ))
  }

  #[inline]
  fn add_part_from_package<P, T>(
    &mut self,
    source_package: &P,
    part: &T,
  ) -> Result<T, crate::common::SdkError>
  where
    P: SdkPackage,
    T: SdkPart,
  {
    let relationship_id = crate::sdk::SdkPackage::relationships(self).next_relationship_id();
    self.add_part_from_package_with_id(source_package, part, relationship_id)
  }

  #[inline]
  fn add_part_from_package_with_id<P, T>(
    &mut self,
    source_package: &P,
    part: &T,
    relationship_id: impl Into<String>,
  ) -> Result<T, crate::common::SdkError>
  where
    P: SdkPackage,
    T: SdkPart,
  {
    let relationship_id = relationship_id.into();
    let source_part_slot = crate::private::SdkPartHandle::part_slot(part, source_package)?;
    let source_content_type = crate::sdk::SdkPackage::storage(source_package)
      .part(source_part_slot)
      .ok_or(crate::common::SdkError::StalePart)?
      .content_type()
      .to_string();
    let constraint = self.validate_child_part::<T>(&source_content_type)?;
    let relationship_type = constrained_relationship_type(
      constraint,
      crate::sdk::SdkPackage::storage(source_package)
        .part(source_part_slot)
        .and_then(crate::common::StoredPart::relationship_type),
    );
    if crate::sdk::SdkPackage::storage(self).token()
      == crate::sdk::SdkPackage::storage(source_package).token()
    {
      crate::sdk::SdkPackage::storage_mut(self).add_package_relationship_to_part(
        relationship_id.clone(),
        relationship_type.as_ref(),
        source_part_slot,
      )?;
      return Ok(part_from_slot(
        crate::sdk::SdkPackage::storage(self),
        source_part_slot,
      ));
    }

    let (imported_part_id, added_count) = crate::sdk::SdkPackage::storage_mut(self)
      .import_part_tree_from(
        crate::sdk::SdkPackage::storage(source_package),
        source_part_slot,
        None,
        relationship_id.clone(),
        relationship_type.as_ref(),
        |part_id, _| crate::sdk::SdkPackage::part_bytes_for_copy(source_package, part_id),
      )?;
    for _ in 0..added_count {
      crate::sdk::SdkPackage::push_root_element_slot(self);
    }
    Ok(part_from_slot(
      crate::sdk::SdkPackage::storage(self),
      imported_part_id,
    ))
  }

  #[inline]
  fn create_relationship_to_part<T: SdkPart>(
    &mut self,
    part: T,
  ) -> Result<String, crate::common::SdkError> {
    let part_slot = crate::private::SdkPartHandle::part_slot(&part, self)?;
    if let Some(relationship_id) = crate::sdk::SdkPackage::relationships(self)
      .part_relationships()
      .find_map(|relationship| {
        (relationship.target_part_slot() == Some(part_slot)).then_some(relationship.id())
      })
    {
      return Ok(relationship_id.to_string());
    }
    let relationship_id = crate::sdk::SdkPackage::relationships(self).next_relationship_id();
    self.create_relationship_to_part_with_id(part, relationship_id)
  }

  #[inline]
  fn create_relationship_to_part_with_id<T: SdkPart>(
    &mut self,
    part: T,
    relationship_id: impl Into<String>,
  ) -> Result<String, crate::common::SdkError> {
    let relationship_id = relationship_id.into();
    let part_id = crate::private::SdkPartHandle::part_slot(&part, self)?;
    let content_type = crate::sdk::SdkPackage::storage(self)
      .part(part_id)
      .ok_or(crate::common::SdkError::StalePart)?
      .content_type()
      .to_string();
    let constraint = self.validate_child_part::<T>(&content_type)?;
    let relationship_type = constrained_relationship_type(
      constraint,
      crate::sdk::SdkPackage::storage(self)
        .part(part_id)
        .and_then(crate::common::StoredPart::relationship_type),
    );
    crate::sdk::SdkPackage::storage_mut(self).add_package_relationship_to_part(
      relationship_id.clone(),
      relationship_type.as_ref(),
      part_id,
    )?;
    Ok(relationship_id)
  }

  #[inline]
  fn create_media_data_part(
    &mut self,
    content_type: impl Into<std::borrow::Cow<'static, str>>,
    extension: impl AsRef<str>,
  ) -> Result<crate::common::MediaDataPart, crate::common::SdkError> {
    let part_id = crate::sdk::SdkPackage::storage_mut(self)
      .create_media_data_part(content_type.into().into_owned(), extension)?;
    Ok(crate::common::MediaDataPart::from_part_slot(
      crate::sdk::SdkPackage::storage(self).token(),
      part_id,
    ))
  }

  #[inline]
  fn create_media_data_part_with_content_type(
    &mut self,
    content_type: impl Into<std::borrow::Cow<'static, str>>,
  ) -> Result<crate::common::MediaDataPart, crate::common::SdkError> {
    self.create_media_data_part(content_type, ".bin")
  }

  #[inline]
  fn create_media_data_part_by_type(
    &mut self,
    part_type: MediaDataPartType,
  ) -> Result<crate::common::MediaDataPart, crate::common::SdkError> {
    self.create_media_data_part(part_type.content_type(), part_type.extension())
  }

  #[inline]
  fn add_new_part<T>(
    &mut self,
    relationship_id: impl Into<String>,
  ) -> Result<T, crate::common::SdkError>
  where
    T: SdkPart,
  {
    self.add_new_part_with_target_mode::<T>(
      relationship_id,
      crate::common::NewPartTargetMode::Indexed,
    )
  }

  #[inline]
  fn add_new_part_auto_id<T>(&mut self) -> Result<T, crate::common::SdkError>
  where
    T: SdkPart,
  {
    let relationship_id = crate::sdk::SdkPackage::relationships(self).next_relationship_id();
    self.add_new_part::<T>(relationship_id)
  }

  #[inline]
  fn add_new_part_with_content_type<T>(
    &mut self,
    relationship_id: impl Into<String>,
    content_type: impl Into<std::borrow::Cow<'static, str>>,
  ) -> Result<T, crate::common::SdkError>
  where
    T: SdkPart,
  {
    self.add_new_part_with_content_type_and_extension::<T>(
      relationship_id,
      content_type,
      T::EXTENSION,
      crate::common::NewPartTargetMode::Indexed,
    )
  }

  #[inline]
  fn add_new_part_with_content_type_auto_id<T>(
    &mut self,
    content_type: impl Into<std::borrow::Cow<'static, str>>,
  ) -> Result<T, crate::common::SdkError>
  where
    T: SdkPart,
  {
    let relationship_id = crate::sdk::SdkPackage::relationships(self).next_relationship_id();
    self.add_new_part_with_content_type::<T>(relationship_id, content_type)
  }

  #[inline]
  fn add_new_part_with_target_mode<T>(
    &mut self,
    relationship_id: impl Into<String>,
    target_mode: crate::common::NewPartTargetMode,
  ) -> Result<T, crate::common::SdkError>
  where
    T: SdkPart,
  {
    let relationship_id = relationship_id.into();
    let content_type = typed_main_part_content_type::<T>(crate::sdk::SdkPackage::storage(self));
    let constraint = self.validate_child_part::<T>(content_type.as_ref())?;
    let part_id = crate::sdk::SdkPackage::storage_mut(self).add_package_part(
      relationship_id.clone(),
      crate::common::NewPartDescriptor {
        relationship_type: std::borrow::Cow::Borrowed(constraint.relationship_type),
        content_type,
        path_prefix: T::PATH_PREFIX,
        target_name: T::TARGET_NAME,
        extension: std::borrow::Cow::Borrowed(T::EXTENSION),
      },
      target_mode,
    )?;
    crate::sdk::SdkPackage::push_root_element_slot(self);
    Ok(part_from_slot(
      crate::sdk::SdkPackage::storage(self),
      part_id,
    ))
  }

  #[inline]
  fn add_core_file_properties_part(
    &mut self,
  ) -> Result<
    crate::parts::core_file_properties_part::CoreFilePropertiesPart,
    crate::common::SdkError,
  > {
    let relationship_id = crate::sdk::SdkPackage::relationships(self).next_relationship_id();
    self.add_new_part_with_target_mode::<crate::parts::core_file_properties_part::CoreFilePropertiesPart>(
      relationship_id,
      crate::common::NewPartTargetMode::Fixed,
    )
  }

  #[inline]
  fn add_extended_file_properties_part(
    &mut self,
  ) -> Result<
    crate::parts::extended_file_properties_part::ExtendedFilePropertiesPart,
    crate::common::SdkError,
  > {
    let relationship_id = crate::sdk::SdkPackage::relationships(self).next_relationship_id();
    self.add_new_part_with_target_mode::<crate::parts::extended_file_properties_part::ExtendedFilePropertiesPart>(
      relationship_id,
      crate::common::NewPartTargetMode::Fixed,
    )
  }

  #[inline]
  fn add_custom_file_properties_part(
    &mut self,
  ) -> Result<
    crate::parts::custom_file_properties_part::CustomFilePropertiesPart,
    crate::common::SdkError,
  > {
    let relationship_id = crate::sdk::SdkPackage::relationships(self).next_relationship_id();
    self.add_new_part_with_target_mode::<crate::parts::custom_file_properties_part::CustomFilePropertiesPart>(
      relationship_id,
      crate::common::NewPartTargetMode::Fixed,
    )
  }

  #[inline]
  fn add_digital_signature_origin_part(
    &mut self,
  ) -> Result<
    crate::parts::digital_signature_origin_part::DigitalSignatureOriginPart,
    crate::common::SdkError,
  > {
    let relationship_id = crate::sdk::SdkPackage::relationships(self).next_relationship_id();
    self.add_new_part_with_target_mode::<crate::parts::digital_signature_origin_part::DigitalSignatureOriginPart>(
      relationship_id,
      crate::common::NewPartTargetMode::Fixed,
    )
  }

  #[inline]
  fn add_new_part_with_content_type_and_extension<T>(
    &mut self,
    relationship_id: impl Into<String>,
    content_type: impl Into<std::borrow::Cow<'static, str>>,
    extension: impl Into<std::borrow::Cow<'static, str>>,
    target_mode: crate::common::NewPartTargetMode,
  ) -> Result<T, crate::common::SdkError>
  where
    T: SdkPart,
  {
    let relationship_id = relationship_id.into();
    let content_type = content_type.into();
    let constraint = self.validate_child_part::<T>(content_type.as_ref())?;
    let part_id = crate::sdk::SdkPackage::storage_mut(self).add_package_part(
      relationship_id.clone(),
      crate::common::NewPartDescriptor {
        relationship_type: std::borrow::Cow::Borrowed(constraint.relationship_type),
        content_type,
        path_prefix: T::PATH_PREFIX,
        target_name: T::TARGET_NAME,
        extension: extension.into(),
      },
      target_mode,
    )?;
    crate::sdk::SdkPackage::push_root_element_slot(self);
    Ok(part_from_slot(
      crate::sdk::SdkPackage::storage(self),
      part_id,
    ))
  }

  #[inline]
  fn add_thumbnail_part(
    &mut self,
    content_type: impl Into<std::borrow::Cow<'static, str>>,
  ) -> Result<crate::parts::thumbnail_part::ThumbnailPart, crate::common::SdkError> {
    self.add_new_part_with_content_type_auto_id::<crate::parts::thumbnail_part::ThumbnailPart>(
      content_type,
    )
  }

  #[inline]
  fn add_thumbnail_part_with_id(
    &mut self,
    content_type: impl Into<std::borrow::Cow<'static, str>>,
    relationship_id: impl Into<String>,
  ) -> Result<crate::parts::thumbnail_part::ThumbnailPart, crate::common::SdkError> {
    self.add_new_part_with_content_type::<crate::parts::thumbnail_part::ThumbnailPart>(
      relationship_id,
      content_type,
    )
  }

  #[inline]
  fn add_thumbnail_part_by_type(
    &mut self,
    part_type: ThumbnailPartType,
  ) -> Result<crate::parts::thumbnail_part::ThumbnailPart, crate::common::SdkError> {
    let relationship_id = crate::sdk::SdkPackage::relationships(self).next_relationship_id();
    self.add_thumbnail_part_by_type_with_id(part_type, relationship_id)
  }

  #[inline]
  fn add_thumbnail_part_by_type_with_id(
    &mut self,
    part_type: ThumbnailPartType,
    relationship_id: impl Into<String>,
  ) -> Result<crate::parts::thumbnail_part::ThumbnailPart, crate::common::SdkError> {
    self
      .add_new_part_with_content_type_and_extension::<crate::parts::thumbnail_part::ThumbnailPart>(
        relationship_id,
        part_type.content_type(),
        part_type.extension(),
        crate::common::NewPartTargetMode::Indexed,
      )
  }

  #[inline]
  fn add_extended_part(
    &mut self,
    relationship_type: impl Into<String>,
    content_type: impl Into<std::borrow::Cow<'static, str>>,
    target_extension: impl Into<std::borrow::Cow<'static, str>>,
  ) -> Result<crate::parts::extended_part::ExtendedPart, crate::common::SdkError> {
    let relationship_id = crate::sdk::SdkPackage::relationships(self).next_relationship_id();
    self.add_extended_part_with_id(
      relationship_type,
      content_type,
      target_extension,
      relationship_id,
    )
  }

  #[inline]
  fn add_extended_part_with_id(
    &mut self,
    relationship_type: impl Into<String>,
    content_type: impl Into<std::borrow::Cow<'static, str>>,
    target_extension: impl Into<std::borrow::Cow<'static, str>>,
    relationship_id: impl Into<String>,
  ) -> Result<crate::parts::extended_part::ExtendedPart, crate::common::SdkError> {
    let relationship_id = relationship_id.into();
    let part_id = crate::sdk::SdkPackage::storage_mut(self).add_package_part(
      relationship_id.clone(),
      crate::common::NewPartDescriptor {
        relationship_type: std::borrow::Cow::Owned(relationship_type.into()),
        content_type: content_type.into(),
        path_prefix: "",
        target_name: "extendedPart",
        extension: target_extension.into(),
      },
      crate::common::NewPartTargetMode::Indexed,
    )?;
    crate::sdk::SdkPackage::push_root_element_slot(self);
    Ok(part_from_slot(
      crate::sdk::SdkPackage::storage(self),
      part_id,
    ))
  }
}

#[cfg(feature = "parts")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RelatedPart<'a, T> {
  relationship_id: &'a str,
  relationship_type: &'a str,
  part: T,
}

#[cfg(feature = "parts")]
impl<'a, T> RelatedPart<'a, T> {
  #[inline]
  pub const fn new(relationship_id: &'a str, relationship_type: &'a str, part: T) -> Self {
    Self {
      relationship_id,
      relationship_type,
      part,
    }
  }

  #[inline]
  pub const fn relationship_id(&self) -> &'a str {
    self.relationship_id
  }

  #[inline]
  pub const fn relationship_type(&self) -> &'a str {
    self.relationship_type
  }

  #[inline]
  pub const fn part(&self) -> &T {
    &self.part
  }

  #[inline]
  pub fn into_part(self) -> T {
    self.part
  }
}

#[cfg(feature = "parts")]
impl<T> std::ops::Deref for RelatedPart<'_, T> {
  type Target = T;

  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.part
  }
}

#[cfg(feature = "parts")]
pub trait SdkPartDescriptor {
  #[doc(hidden)]
  const KIND: crate::parts::PartKind;
  const RELATIONSHIP_TYPE: &'static str;
  const PATH_PREFIX: &'static str;
  const CONTENT_TYPE: &'static str;
  const TARGET_NAME: &'static str;
  const EXTENSION: &'static str;
}

#[cfg(feature = "parts")]
fn validate_child_part_constraint<T: SdkPart>(
  constraint: Option<PartConstraint>,
  allows_any_child_part: bool,
  relationships: &crate::common::RelationshipSet,
  content_type: &str,
) -> Result<PartConstraint, crate::common::SdkError> {
  let constraint = match constraint {
    Some(constraint) => constraint,
    None if allows_any_child_part || T::KIND == crate::parts::PartKind::ExtendedPart => {
      PartConstraint::new(T::KIND, T::RELATIONSHIP_TYPE, T::CONTENT_TYPE, false, true)
    }
    None => {
      return Err(crate::common::SdkError::CommonError(format!(
        "part kind {:?} is not allowed as a child",
        T::KIND
      )));
    }
  };
  if !constraint.content_type.is_empty() && constraint.content_type != content_type {
    return Err(crate::common::SdkError::CommonError(format!(
      "part kind {:?} requires content type {}, got {content_type}",
      T::KIND,
      constraint.content_type
    )));
  }
  if !constraint.max_occurs_great_than_one
    && relationships
      .first_target_part_by_relationship_type(constraint.relationship_type)
      .is_some()
  {
    return Err(crate::common::SdkError::CommonError(format!(
      "only one child with relationship type {} is allowed",
      constraint.relationship_type
    )));
  }
  Ok(constraint)
}

#[cfg(feature = "parts")]
#[inline]
fn constrained_relationship_type(
  constraint: PartConstraint,
  stored_relationship_type: Option<&str>,
) -> std::borrow::Cow<'static, str> {
  if constraint.relationship_type.is_empty() {
    std::borrow::Cow::Owned(stored_relationship_type.unwrap_or_default().to_string())
  } else {
    std::borrow::Cow::Borrowed(constraint.relationship_type)
  }
}

#[cfg(feature = "parts")]
pub(crate) trait SdkPartRoot: SdkPart {
  type RootElement;

  fn wrap_root_element(root_element: Self::RootElement) -> crate::parts::PartRootElement;

  fn root_element_ref(root_element: &crate::parts::PartRootElement) -> Option<&Self::RootElement>;

  fn root_element_mut(
    root_element: &mut crate::parts::PartRootElement,
  ) -> Option<&mut Self::RootElement>;
}

#[cfg(feature = "parts")]
pub trait SdkPart:
  crate::private::SdkPartHandle + SdkPartDescriptor + Clone + Sized + 'static
{
  #[doc(hidden)]
  const CHILD_PART_CONSTRAINTS: &'static [PartConstraint];

  #[doc(hidden)]
  const ALLOWS_ANY_CHILD_PART: bool = false;

  #[doc(hidden)]
  fn child_part_constraint(kind: crate::parts::PartKind) -> Option<PartConstraint>;

  #[inline]
  #[doc(hidden)]
  fn validate_child_part<P: SdkPackage, T: SdkPart>(
    &self,
    package: &P,
    content_type: &str,
  ) -> Result<PartConstraint, crate::common::SdkError> {
    let part_slot = crate::private::SdkPartHandle::part_slot(self, package)?;
    validate_child_part_constraint::<T>(
      Self::child_part_constraint(T::KIND),
      Self::ALLOWS_ANY_CHILD_PART,
      crate::sdk::SdkPackage::storage(package)
        .relationships(part_slot)
        .ok_or(crate::common::SdkError::StalePart)?,
      content_type,
    )
  }

  #[inline]
  fn next_relationship_id<P: SdkPackage>(
    &self,
    package: &P,
  ) -> Result<String, crate::common::SdkError> {
    let part_slot = crate::private::SdkPartHandle::part_slot(self, package)?;
    Ok(
      crate::sdk::SdkPackage::storage(package)
        .relationships(part_slot)
        .ok_or(crate::common::SdkError::StalePart)?
        .next_relationship_id(),
    )
  }

  #[inline]
  fn added_relationship_ref<'a, P: SdkPackage>(
    &self,
    package: &'a P,
    relationship_id: &str,
  ) -> Result<crate::common::RelationshipRef<'a>, crate::common::SdkError> {
    let part_slot = crate::private::SdkPartHandle::part_slot(self, package)?;
    let storage = crate::sdk::SdkPackage::storage(package);
    storage
      .relationships(part_slot)
      .and_then(|relationships| relationships.get(relationship_id))
      .map(|relationship| crate::common::RelationshipRef::new(storage.token(), relationship))
      .ok_or_else(|| {
        crate::common::SdkError::CommonError(format!(
          "relationship id {relationship_id} is not present on the part"
        ))
      })
  }

  #[inline]
  fn child_part_by_relationship_type<P, T>(&self, package: &P, relationship_type: &str) -> Option<T>
  where
    P: SdkPackage,
    T: SdkPart,
  {
    self
      .child_related_part_by_relationship_type(package, relationship_type)
      .map(RelatedPart::into_part)
  }

  #[inline]
  fn child_parts_by_relationship_type<'a, P, T>(
    &'a self,
    package: &'a P,
    relationship_type: &'a str,
  ) -> impl Iterator<Item = T> + 'a
  where
    P: SdkPackage,
    T: SdkPart,
  {
    self
      .child_related_parts_by_relationship_type(package, relationship_type)
      .map(RelatedPart::into_part)
  }

  #[inline]
  fn related_part_of_type<'a, P, T>(&'a self, package: &'a P) -> Option<RelatedPart<'a, T>>
  where
    P: SdkPackage,
    T: SdkPart,
  {
    self.child_related_parts_of_type(package).next()
  }

  #[inline]
  fn related_parts_of_type<'a, P, T>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = RelatedPart<'a, T>> + 'a
  where
    P: SdkPackage,
    T: SdkPart,
  {
    self.child_related_parts_of_type(package)
  }

  #[inline]
  fn child_related_parts_of_type<'a, P, T>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = RelatedPart<'a, T>> + 'a
  where
    P: SdkPackage,
    T: SdkPart,
  {
    let storage = crate::sdk::SdkPackage::storage(package);
    let part_slot = crate::private::SdkPartHandle::part_slot_optional(self, package);
    part_slot
      .into_iter()
      .flat_map(move |part_slot| storage.relationships(part_slot))
      .flat_map(|relationships| relationships.part_relationships())
      .filter_map(move |relationship| {
        let matches_type = crate::common::relationship_type_matches_bytes(
          relationship.relationship_type_bytes(),
          T::RELATIONSHIP_TYPE.as_bytes(),
        );
        matches_type
          .then(|| relationship.target_part_slot())
          .flatten()
          .and_then(|part_slot| {
            relationship_target_as_part::<T>(storage, relationship).map(|part| {
              debug_assert_eq!(
                crate::private::SdkPartHandle::part_slot_optional(&part, package),
                Some(part_slot)
              );
              RelatedPart::new(relationship.id(), relationship.relationship_type(), part)
            })
          })
      })
  }

  #[inline]
  fn child_related_part_by_relationship_type<'a, P, T>(
    &'a self,
    package: &'a P,
    relationship_type: &'a str,
  ) -> Option<RelatedPart<'a, T>>
  where
    P: SdkPackage,
    T: SdkPart,
  {
    self
      .child_related_parts_by_relationship_type(package, relationship_type)
      .next()
  }

  #[inline]
  fn child_related_parts_by_relationship_type<'a, P, T>(
    &'a self,
    package: &'a P,
    relationship_type: &'a str,
  ) -> impl Iterator<Item = RelatedPart<'a, T>> + 'a
  where
    P: SdkPackage,
    T: SdkPart,
  {
    let storage = crate::sdk::SdkPackage::storage(package);
    let part_slot = crate::private::SdkPartHandle::part_slot_optional(self, package);
    part_slot
      .into_iter()
      .flat_map(move |part_slot| storage.relationships(part_slot))
      .flat_map(|relationships| relationships.part_relationships())
      .filter_map(move |relationship| {
        crate::common::relationship_type_matches_bytes(
          relationship.relationship_type_bytes(),
          relationship_type.as_bytes(),
        )
        .then(|| relationship.target_part_slot())
        .flatten()
        .and_then(|_| {
          relationship_target_as_part::<T>(storage, relationship)
            .map(|part| RelatedPart::new(relationship.id(), relationship.relationship_type(), part))
        })
      })
  }

  #[inline]
  fn add_external_relationship<'a, P: SdkPackage>(
    &self,
    package: &'a mut P,
    relationship_id: impl Into<String>,
    relationship_type: impl Into<String>,
    target: impl Into<String>,
  ) -> Result<crate::common::RelationshipRef<'a>, crate::common::SdkError> {
    let relationship_id = relationship_id.into();
    let part_slot = crate::private::SdkPartHandle::part_slot(self, package)?;
    crate::sdk::SdkPackage::storage_mut(package)
      .relationships_mut(part_slot)
      .ok_or(crate::common::SdkError::StalePart)?
      .add_external_relationship(relationship_id.clone(), relationship_type, target)?;
    self.added_relationship_ref(package, &relationship_id)
  }

  #[inline]
  fn add_external_relationship_auto_id<'a, P: SdkPackage>(
    &self,
    package: &'a mut P,
    relationship_type: impl Into<String>,
    target: impl Into<String>,
  ) -> Result<crate::common::RelationshipRef<'a>, crate::common::SdkError> {
    let relationship_id = self.next_relationship_id(package)?;
    self.add_external_relationship(package, relationship_id, relationship_type, target)
  }

  #[inline]
  fn add_hyperlink_relationship<'a, P: SdkPackage>(
    &self,
    package: &'a mut P,
    relationship_id: impl Into<String>,
    target: impl Into<String>,
  ) -> Result<crate::common::RelationshipRef<'a>, crate::common::SdkError> {
    let relationship_id = relationship_id.into();
    let part_slot = crate::private::SdkPartHandle::part_slot(self, package)?;
    crate::sdk::SdkPackage::storage_mut(package)
      .relationships_mut(part_slot)
      .ok_or(crate::common::SdkError::StalePart)?
      .add_hyperlink_relationship(relationship_id.clone(), target)?;
    self.added_relationship_ref(package, &relationship_id)
  }

  #[inline]
  fn add_hyperlink_relationship_with_mode<'a, P: SdkPackage>(
    &self,
    package: &'a mut P,
    relationship_id: impl Into<String>,
    target: impl Into<String>,
    target_mode: crate::schemas::opc_relationships::TargetMode,
  ) -> Result<crate::common::RelationshipRef<'a>, crate::common::SdkError> {
    let relationship_id = relationship_id.into();
    let part_slot = crate::private::SdkPartHandle::part_slot(self, package)?;
    crate::sdk::SdkPackage::storage_mut(package)
      .relationships_mut(part_slot)
      .ok_or(crate::common::SdkError::StalePart)?
      .add_hyperlink_relationship_with_mode(relationship_id.clone(), target, target_mode)?;
    self.added_relationship_ref(package, &relationship_id)
  }

  #[inline]
  fn add_hyperlink_relationship_auto_id<'a, P: SdkPackage>(
    &self,
    package: &'a mut P,
    target: impl Into<String>,
    target_mode: crate::schemas::opc_relationships::TargetMode,
  ) -> Result<crate::common::RelationshipRef<'a>, crate::common::SdkError> {
    let relationship_id = self.next_relationship_id(package)?;
    self.add_hyperlink_relationship_with_mode(package, relationship_id, target, target_mode)
  }

  #[inline]
  fn add_audio_reference_relationship<P: SdkPackage>(
    &self,
    package: &mut P,
    media_data_part: &crate::common::MediaDataPart,
  ) -> Result<String, crate::common::SdkError> {
    let relationship_id = self.next_relationship_id(package)?;
    self.add_audio_reference_relationship_with_id(package, media_data_part, relationship_id)
  }

  #[inline]
  fn add_audio_reference_relationship_with_id<P: SdkPackage>(
    &self,
    package: &mut P,
    media_data_part: &crate::common::MediaDataPart,
    relationship_id: impl Into<String>,
  ) -> Result<String, crate::common::SdkError> {
    self.add_data_part_reference_relationship_with_id(
      package,
      media_data_part,
      crate::common::RelationshipSet::AUDIO_REFERENCE_RELATIONSHIP_TYPE,
      relationship_id,
    )
  }

  #[inline]
  fn add_media_reference_relationship<P: SdkPackage>(
    &self,
    package: &mut P,
    media_data_part: &crate::common::MediaDataPart,
  ) -> Result<String, crate::common::SdkError> {
    let relationship_id = self.next_relationship_id(package)?;
    self.add_media_reference_relationship_with_id(package, media_data_part, relationship_id)
  }

  #[inline]
  fn add_media_reference_relationship_with_id<P: SdkPackage>(
    &self,
    package: &mut P,
    media_data_part: &crate::common::MediaDataPart,
    relationship_id: impl Into<String>,
  ) -> Result<String, crate::common::SdkError> {
    self.add_data_part_reference_relationship_with_id(
      package,
      media_data_part,
      crate::common::RelationshipSet::MEDIA_REFERENCE_RELATIONSHIP_TYPE,
      relationship_id,
    )
  }

  #[inline]
  fn add_video_reference_relationship<P: SdkPackage>(
    &self,
    package: &mut P,
    media_data_part: &crate::common::MediaDataPart,
  ) -> Result<String, crate::common::SdkError> {
    let relationship_id = self.next_relationship_id(package)?;
    self.add_video_reference_relationship_with_id(package, media_data_part, relationship_id)
  }

  #[inline]
  fn add_video_reference_relationship_with_id<P: SdkPackage>(
    &self,
    package: &mut P,
    media_data_part: &crate::common::MediaDataPart,
    relationship_id: impl Into<String>,
  ) -> Result<String, crate::common::SdkError> {
    self.add_data_part_reference_relationship_with_id(
      package,
      media_data_part,
      crate::common::RelationshipSet::VIDEO_REFERENCE_RELATIONSHIP_TYPE,
      relationship_id,
    )
  }

  #[inline]
  fn add_data_part_reference_relationship_from_existing<P: SdkPackage>(
    &self,
    package: &mut P,
    relationship: crate::common::Relationship,
  ) -> Result<String, crate::common::SdkError> {
    use crate::common::ReferenceRelationshipKind;

    if !relationship.is_reference_relationship()
      || !matches!(
        relationship.reference_kind(),
        Some(
          ReferenceRelationshipKind::Audio
            | ReferenceRelationshipKind::Media
            | ReferenceRelationshipKind::Video
        )
      )
    {
      return Err(crate::common::SdkError::CommonError(format!(
        "relationship id {} is not a data part reference relationship",
        relationship.id()
      )));
    }
    let target_part_slot = relationship
      .target_part_slot_for_package(package)?
      .ok_or_else(|| {
        crate::common::SdkError::CommonError(format!(
          "data part reference relationship id {} does not target a package part",
          relationship.id()
        ))
      })?;
    let source_part_slot = crate::private::SdkPartHandle::part_slot(self, package)?;
    let relationship_id = crate::sdk::SdkPackage::storage_mut(package)
      .add_data_part_reference_relationship(
        source_part_slot,
        relationship.id(),
        relationship.relationship_type(),
        target_part_slot,
      )?;
    Ok(relationship_id)
  }

  #[inline]
  fn add_data_part_reference_relationship_with_id<P: SdkPackage>(
    &self,
    package: &mut P,
    media_data_part: &crate::common::MediaDataPart,
    relationship_type: &str,
    relationship_id: impl Into<String>,
  ) -> Result<String, crate::common::SdkError> {
    let source_part_slot = crate::private::SdkPartHandle::part_slot(self, package)?;
    let target_part_slot = media_data_part.part_slot_for_package(package)?;
    let relationship_id = crate::sdk::SdkPackage::storage_mut(package)
      .add_data_part_reference_relationship(
        source_part_slot,
        relationship_id,
        relationship_type,
        target_part_slot,
      )?;
    Ok(relationship_id)
  }

  #[inline]
  fn add_new_part<P, T>(
    &self,
    package: &mut P,
    relationship_id: impl Into<String>,
  ) -> Result<T, crate::common::SdkError>
  where
    P: SdkPackage,
    T: SdkPart,
  {
    let relationship_id = relationship_id.into();
    let constraint = self.validate_child_part::<P, T>(package, T::CONTENT_TYPE)?;
    let source_part_slot = crate::private::SdkPartHandle::part_slot(self, package)?;
    let part_id = crate::sdk::SdkPackage::storage_mut(package).add_child_part(
      source_part_slot,
      relationship_id.clone(),
      crate::common::NewPartDescriptor {
        relationship_type: std::borrow::Cow::Borrowed(constraint.relationship_type),
        content_type: std::borrow::Cow::Borrowed(T::CONTENT_TYPE),
        path_prefix: T::PATH_PREFIX,
        target_name: T::TARGET_NAME,
        extension: std::borrow::Cow::Borrowed(T::EXTENSION),
      },
    )?;
    crate::sdk::SdkPackage::push_root_element_slot(package);
    Ok(part_from_slot(
      crate::sdk::SdkPackage::storage(package),
      part_id,
    ))
  }

  #[inline]
  fn add_new_part_with_content_type<P, T>(
    &self,
    package: &mut P,
    relationship_id: impl Into<String>,
    content_type: impl Into<std::borrow::Cow<'static, str>>,
  ) -> Result<T, crate::common::SdkError>
  where
    P: SdkPackage,
    T: SdkPart,
  {
    let relationship_id = relationship_id.into();
    let content_type = content_type.into();
    let constraint = self.validate_child_part::<P, T>(package, content_type.as_ref())?;
    let source_part_slot = crate::private::SdkPartHandle::part_slot(self, package)?;
    let part_id = crate::sdk::SdkPackage::storage_mut(package).add_child_part(
      source_part_slot,
      relationship_id.clone(),
      crate::common::NewPartDescriptor {
        relationship_type: std::borrow::Cow::Borrowed(constraint.relationship_type),
        content_type,
        path_prefix: T::PATH_PREFIX,
        target_name: T::TARGET_NAME,
        extension: std::borrow::Cow::Borrowed(T::EXTENSION),
      },
    )?;
    crate::sdk::SdkPackage::push_root_element_slot(package);
    Ok(part_from_slot(
      crate::sdk::SdkPackage::storage(package),
      part_id,
    ))
  }

  #[inline]
  fn add_new_part_with_content_type_and_path<P, T>(
    &self,
    package: &mut P,
    relationship_id: impl Into<String>,
    content_type: impl Into<std::borrow::Cow<'static, str>>,
    part_path: impl AsRef<str>,
  ) -> Result<T, crate::common::SdkError>
  where
    P: SdkPackage,
    T: SdkPart,
  {
    let relationship_id = relationship_id.into();
    let content_type = content_type.into();
    let constraint = self.validate_child_part::<P, T>(package, content_type.as_ref())?;
    let source_part_slot = crate::private::SdkPartHandle::part_slot(self, package)?;
    let part_id = crate::sdk::SdkPackage::storage_mut(package).add_child_part_with_path(
      source_part_slot,
      relationship_id.clone(),
      crate::common::NewPartDescriptor {
        relationship_type: std::borrow::Cow::Borrowed(constraint.relationship_type),
        content_type,
        path_prefix: T::PATH_PREFIX,
        target_name: T::TARGET_NAME,
        extension: std::borrow::Cow::Borrowed(T::EXTENSION),
      },
      part_path,
    )?;
    crate::sdk::SdkPackage::push_root_element_slot(package);
    Ok(part_from_slot(
      crate::sdk::SdkPackage::storage(package),
      part_id,
    ))
  }

  #[inline]
  fn add_new_part_auto_id<P, T>(&self, package: &mut P) -> Result<T, crate::common::SdkError>
  where
    P: SdkPackage,
    T: SdkPart,
  {
    let relationship_id = self.next_relationship_id(package)?;
    self.add_new_part::<P, T>(package, relationship_id)
  }

  #[inline]
  fn add_new_part_with_content_type_auto_id<P, T>(
    &self,
    package: &mut P,
    content_type: impl Into<std::borrow::Cow<'static, str>>,
  ) -> Result<T, crate::common::SdkError>
  where
    P: SdkPackage,
    T: SdkPart,
  {
    let relationship_id = self.next_relationship_id(package)?;
    self.add_new_part_with_content_type::<P, T>(package, relationship_id, content_type)
  }

  #[inline]
  fn add_new_part_with_content_type_and_extension<P, T>(
    &self,
    package: &mut P,
    relationship_id: impl Into<String>,
    content_type: impl Into<std::borrow::Cow<'static, str>>,
    extension: impl Into<std::borrow::Cow<'static, str>>,
  ) -> Result<T, crate::common::SdkError>
  where
    P: SdkPackage,
    T: SdkPart,
  {
    let relationship_id = relationship_id.into();
    let content_type = content_type.into();
    let constraint = self.validate_child_part::<P, T>(package, content_type.as_ref())?;
    let source_part_slot = crate::private::SdkPartHandle::part_slot(self, package)?;
    let part_id = crate::sdk::SdkPackage::storage_mut(package).add_child_part(
      source_part_slot,
      relationship_id.clone(),
      crate::common::NewPartDescriptor {
        relationship_type: std::borrow::Cow::Borrowed(constraint.relationship_type),
        content_type,
        path_prefix: T::PATH_PREFIX,
        target_name: T::TARGET_NAME,
        extension: extension.into(),
      },
    )?;
    crate::sdk::SdkPackage::push_root_element_slot(package);
    Ok(part_from_slot(
      crate::sdk::SdkPackage::storage(package),
      part_id,
    ))
  }

  #[inline]
  fn add_new_part_with_content_type_and_extension_auto_id<P, T>(
    &self,
    package: &mut P,
    content_type: impl Into<std::borrow::Cow<'static, str>>,
    extension: impl Into<std::borrow::Cow<'static, str>>,
  ) -> Result<T, crate::common::SdkError>
  where
    P: SdkPackage,
    T: SdkPart,
  {
    let relationship_id = self.next_relationship_id(package)?;
    self.add_new_part_with_content_type_and_extension::<P, T>(
      package,
      relationship_id,
      content_type,
      extension,
    )
  }

  #[inline]
  fn add_extended_part<P>(
    &self,
    package: &mut P,
    relationship_type: impl Into<String>,
    content_type: impl Into<std::borrow::Cow<'static, str>>,
    target_extension: impl Into<std::borrow::Cow<'static, str>>,
  ) -> Result<crate::parts::extended_part::ExtendedPart, crate::common::SdkError>
  where
    P: SdkPackage,
  {
    let relationship_id = self.next_relationship_id(package)?;
    self.add_extended_part_with_id(
      package,
      relationship_type,
      content_type,
      target_extension,
      relationship_id,
    )
  }

  #[inline]
  fn add_extended_part_with_id<P>(
    &self,
    package: &mut P,
    relationship_type: impl Into<String>,
    content_type: impl Into<std::borrow::Cow<'static, str>>,
    target_extension: impl Into<std::borrow::Cow<'static, str>>,
    relationship_id: impl Into<String>,
  ) -> Result<crate::parts::extended_part::ExtendedPart, crate::common::SdkError>
  where
    P: SdkPackage,
  {
    let relationship_id = relationship_id.into();
    let source_part_slot = crate::private::SdkPartHandle::part_slot(self, package)?;
    let part_id = crate::sdk::SdkPackage::storage_mut(package).add_child_part(
      source_part_slot,
      relationship_id.clone(),
      crate::common::NewPartDescriptor {
        relationship_type: std::borrow::Cow::Owned(relationship_type.into()),
        content_type: content_type.into(),
        path_prefix: ".",
        target_name: "extendedPart",
        extension: target_extension.into(),
      },
    )?;
    crate::sdk::SdkPackage::push_root_element_slot(package);
    Ok(part_from_slot(
      crate::sdk::SdkPackage::storage(package),
      part_id,
    ))
  }

  #[inline]
  fn add_image_part<P>(
    &self,
    package: &mut P,
    content_type: impl Into<std::borrow::Cow<'static, str>>,
  ) -> Result<crate::parts::image_part::ImagePart, crate::common::SdkError>
  where
    P: SdkPackage,
  {
    let content_type = content_type.into();
    let extension =
      extension_for_content_type::<crate::parts::image_part::ImagePart>(content_type.as_ref());
    self.add_new_part_with_content_type_and_extension_auto_id::<
      P,
      crate::parts::image_part::ImagePart,
    >(
      package,
      content_type,
      extension,
    )
  }

  #[inline]
  fn add_image_part_with_id<P>(
    &self,
    package: &mut P,
    content_type: impl Into<std::borrow::Cow<'static, str>>,
    relationship_id: impl Into<String>,
  ) -> Result<crate::parts::image_part::ImagePart, crate::common::SdkError>
  where
    P: SdkPackage,
  {
    let content_type = content_type.into();
    let extension =
      extension_for_content_type::<crate::parts::image_part::ImagePart>(content_type.as_ref());
    self.add_new_part_with_content_type_and_extension::<P, crate::parts::image_part::ImagePart>(
      package,
      relationship_id,
      content_type,
      extension,
    )
  }

  #[inline]
  fn add_alternative_format_import_part<P>(
    &self,
    package: &mut P,
    content_type: impl Into<std::borrow::Cow<'static, str>>,
  ) -> Result<
    crate::parts::alternative_format_import_part::AlternativeFormatImportPart,
    crate::common::SdkError,
  >
  where
    P: SdkPackage,
  {
    self.add_new_part_with_content_type_auto_id::<
      P,
      crate::parts::alternative_format_import_part::AlternativeFormatImportPart,
    >(package, content_type)
  }

  #[inline]
  fn add_alternative_format_import_part_with_id<P>(
    &self,
    package: &mut P,
    content_type: impl Into<std::borrow::Cow<'static, str>>,
    relationship_id: impl Into<String>,
  ) -> Result<
    crate::parts::alternative_format_import_part::AlternativeFormatImportPart,
    crate::common::SdkError,
  >
  where
    P: SdkPackage,
  {
    self.add_new_part_with_content_type::<P, crate::parts::alternative_format_import_part::AlternativeFormatImportPart>(
      package,
      relationship_id,
      content_type,
    )
  }

  #[inline]
  fn add_alternative_format_import_part_by_type<P>(
    &self,
    package: &mut P,
    part_type: AlternativeFormatImportPartType,
  ) -> Result<
    crate::parts::alternative_format_import_part::AlternativeFormatImportPart,
    crate::common::SdkError,
  >
  where
    P: SdkPackage,
  {
    self.add_alternative_format_import_part(package, part_type.content_type())
  }

  #[inline]
  fn add_alternative_format_import_part_by_type_with_id<P>(
    &self,
    package: &mut P,
    part_type: AlternativeFormatImportPartType,
    relationship_id: impl Into<String>,
  ) -> Result<
    crate::parts::alternative_format_import_part::AlternativeFormatImportPart,
    crate::common::SdkError,
  >
  where
    P: SdkPackage,
  {
    self.add_alternative_format_import_part_with_id(
      package,
      part_type.content_type(),
      relationship_id,
    )
  }

  #[inline]
  fn add_custom_xml_part<P>(
    &self,
    package: &mut P,
    content_type: impl Into<std::borrow::Cow<'static, str>>,
  ) -> Result<crate::parts::custom_xml_part::CustomXmlPart, crate::common::SdkError>
  where
    P: SdkPackage,
  {
    self.add_new_part_with_content_type_auto_id::<P, crate::parts::custom_xml_part::CustomXmlPart>(
      package,
      content_type,
    )
  }

  #[inline]
  fn add_custom_xml_part_with_id<P>(
    &self,
    package: &mut P,
    content_type: impl Into<std::borrow::Cow<'static, str>>,
    relationship_id: impl Into<String>,
  ) -> Result<crate::parts::custom_xml_part::CustomXmlPart, crate::common::SdkError>
  where
    P: SdkPackage,
  {
    self.add_new_part_with_content_type::<P, crate::parts::custom_xml_part::CustomXmlPart>(
      package,
      relationship_id,
      content_type,
    )
  }

  #[inline]
  fn add_custom_xml_part_by_type<P>(
    &self,
    package: &mut P,
    part_type: CustomXmlPartType,
  ) -> Result<crate::parts::custom_xml_part::CustomXmlPart, crate::common::SdkError>
  where
    P: SdkPackage,
  {
    self.add_custom_xml_part(package, part_type.content_type())
  }

  #[inline]
  fn add_custom_xml_part_by_type_with_id<P>(
    &self,
    package: &mut P,
    part_type: CustomXmlPartType,
    relationship_id: impl Into<String>,
  ) -> Result<crate::parts::custom_xml_part::CustomXmlPart, crate::common::SdkError>
  where
    P: SdkPackage,
  {
    self.add_custom_xml_part_with_id(package, part_type.content_type(), relationship_id)
  }

  #[inline]
  fn add_custom_property_part<P>(
    &self,
    package: &mut P,
    content_type: impl Into<std::borrow::Cow<'static, str>>,
  ) -> Result<crate::parts::custom_property_part::CustomPropertyPart, crate::common::SdkError>
  where
    P: SdkPackage,
  {
    self.add_new_part_with_content_type_auto_id::<P, crate::parts::custom_property_part::CustomPropertyPart>(
      package,
      content_type,
    )
  }

  #[inline]
  fn add_custom_property_part_with_id<P>(
    &self,
    package: &mut P,
    content_type: impl Into<std::borrow::Cow<'static, str>>,
    relationship_id: impl Into<String>,
  ) -> Result<crate::parts::custom_property_part::CustomPropertyPart, crate::common::SdkError>
  where
    P: SdkPackage,
  {
    self
      .add_new_part_with_content_type::<P, crate::parts::custom_property_part::CustomPropertyPart>(
        package,
        relationship_id,
        content_type,
      )
  }

  #[inline]
  fn add_custom_property_part_by_type<P>(
    &self,
    package: &mut P,
    part_type: CustomPropertyPartType,
  ) -> Result<crate::parts::custom_property_part::CustomPropertyPart, crate::common::SdkError>
  where
    P: SdkPackage,
  {
    self.add_new_part_with_content_type_and_extension_auto_id::<
      P,
      crate::parts::custom_property_part::CustomPropertyPart,
    >(package, part_type.content_type(), part_type.extension())
  }

  #[inline]
  fn add_custom_property_part_by_type_with_id<P>(
    &self,
    package: &mut P,
    part_type: CustomPropertyPartType,
    relationship_id: impl Into<String>,
  ) -> Result<crate::parts::custom_property_part::CustomPropertyPart, crate::common::SdkError>
  where
    P: SdkPackage,
  {
    self.add_new_part_with_content_type_and_extension::<P, crate::parts::custom_property_part::CustomPropertyPart>(
      package,
      relationship_id,
      part_type.content_type(),
      part_type.extension(),
    )
  }

  #[inline]
  fn add_embedded_object_part<P>(
    &self,
    package: &mut P,
    content_type: impl Into<std::borrow::Cow<'static, str>>,
  ) -> Result<crate::parts::embedded_object_part::EmbeddedObjectPart, crate::common::SdkError>
  where
    P: SdkPackage,
  {
    self.add_new_part_with_content_type_auto_id::<P, crate::parts::embedded_object_part::EmbeddedObjectPart>(
      package,
      content_type,
    )
  }

  #[inline]
  fn add_embedded_object_part_with_id<P>(
    &self,
    package: &mut P,
    content_type: impl Into<std::borrow::Cow<'static, str>>,
    relationship_id: impl Into<String>,
  ) -> Result<crate::parts::embedded_object_part::EmbeddedObjectPart, crate::common::SdkError>
  where
    P: SdkPackage,
  {
    self
      .add_new_part_with_content_type::<P, crate::parts::embedded_object_part::EmbeddedObjectPart>(
        package,
        relationship_id,
        content_type,
      )
  }

  #[inline]
  fn add_embedded_object_part_by_type<P>(
    &self,
    package: &mut P,
    part_type: EmbeddedObjectPartType,
  ) -> Result<crate::parts::embedded_object_part::EmbeddedObjectPart, crate::common::SdkError>
  where
    P: SdkPackage,
  {
    self.add_new_part_with_content_type_and_extension_auto_id::<
      P,
      crate::parts::embedded_object_part::EmbeddedObjectPart,
    >(package, part_type.content_type(), part_type.extension())
  }

  #[inline]
  fn add_embedded_object_part_by_type_with_id<P>(
    &self,
    package: &mut P,
    part_type: EmbeddedObjectPartType,
    relationship_id: impl Into<String>,
  ) -> Result<crate::parts::embedded_object_part::EmbeddedObjectPart, crate::common::SdkError>
  where
    P: SdkPackage,
  {
    self.add_new_part_with_content_type_and_extension::<P, crate::parts::embedded_object_part::EmbeddedObjectPart>(
      package,
      relationship_id,
      part_type.content_type(),
      part_type.extension(),
    )
  }

  #[inline]
  fn add_embedded_package_part<P>(
    &self,
    package: &mut P,
    content_type: impl Into<std::borrow::Cow<'static, str>>,
  ) -> Result<crate::parts::embedded_package_part::EmbeddedPackagePart, crate::common::SdkError>
  where
    P: SdkPackage,
  {
    self.add_new_part_with_content_type_auto_id::<P, crate::parts::embedded_package_part::EmbeddedPackagePart>(
      package,
      content_type,
    )
  }

  #[inline]
  fn add_embedded_package_part_with_id<P>(
    &self,
    package: &mut P,
    content_type: impl Into<std::borrow::Cow<'static, str>>,
    relationship_id: impl Into<String>,
  ) -> Result<crate::parts::embedded_package_part::EmbeddedPackagePart, crate::common::SdkError>
  where
    P: SdkPackage,
  {
    self.add_new_part_with_content_type::<P, crate::parts::embedded_package_part::EmbeddedPackagePart>(
      package,
      relationship_id,
      content_type,
    )
  }

  #[inline]
  fn add_embedded_package_part_by_type<P>(
    &self,
    package: &mut P,
    part_type: EmbeddedPackagePartType,
  ) -> Result<crate::parts::embedded_package_part::EmbeddedPackagePart, crate::common::SdkError>
  where
    P: SdkPackage,
  {
    self.add_new_part_with_content_type_and_extension_auto_id::<
      P,
      crate::parts::embedded_package_part::EmbeddedPackagePart,
    >(package, part_type.content_type(), part_type.extension())
  }

  #[inline]
  fn add_embedded_package_part_by_type_with_id<P>(
    &self,
    package: &mut P,
    part_type: EmbeddedPackagePartType,
    relationship_id: impl Into<String>,
  ) -> Result<crate::parts::embedded_package_part::EmbeddedPackagePart, crate::common::SdkError>
  where
    P: SdkPackage,
  {
    self.add_new_part_with_content_type_and_extension::<P, crate::parts::embedded_package_part::EmbeddedPackagePart>(
      package,
      relationship_id,
      part_type.content_type(),
      part_type.extension(),
    )
  }

  #[inline]
  fn add_font_part<P>(
    &self,
    package: &mut P,
    content_type: impl Into<std::borrow::Cow<'static, str>>,
  ) -> Result<crate::parts::font_part::FontPart, crate::common::SdkError>
  where
    P: SdkPackage,
  {
    self.add_new_part_with_content_type_auto_id::<P, crate::parts::font_part::FontPart>(
      package,
      content_type,
    )
  }

  #[inline]
  fn add_font_part_with_id<P>(
    &self,
    package: &mut P,
    content_type: impl Into<std::borrow::Cow<'static, str>>,
    relationship_id: impl Into<String>,
  ) -> Result<crate::parts::font_part::FontPart, crate::common::SdkError>
  where
    P: SdkPackage,
  {
    self.add_new_part_with_content_type::<P, crate::parts::font_part::FontPart>(
      package,
      relationship_id,
      content_type,
    )
  }

  #[inline]
  fn add_font_part_by_type<P>(
    &self,
    package: &mut P,
    part_type: FontPartType,
  ) -> Result<crate::parts::font_part::FontPart, crate::common::SdkError>
  where
    P: SdkPackage,
  {
    self
      .add_new_part_with_content_type_and_extension_auto_id::<P, crate::parts::font_part::FontPart>(
        package,
        part_type.content_type(),
        part_type.extension(),
      )
  }

  #[inline]
  fn add_font_part_by_type_with_id<P>(
    &self,
    package: &mut P,
    part_type: FontPartType,
    relationship_id: impl Into<String>,
  ) -> Result<crate::parts::font_part::FontPart, crate::common::SdkError>
  where
    P: SdkPackage,
  {
    self.add_new_part_with_content_type_and_extension::<P, crate::parts::font_part::FontPart>(
      package,
      relationship_id,
      part_type.content_type(),
      part_type.extension(),
    )
  }

  #[inline]
  fn add_mail_merge_recipient_data_part<P>(
    &self,
    package: &mut P,
    content_type: impl Into<std::borrow::Cow<'static, str>>,
  ) -> Result<
    crate::parts::mail_merge_recipient_data_part::MailMergeRecipientDataPart,
    crate::common::SdkError,
  >
  where
    P: SdkPackage,
  {
    self.add_new_part_with_content_type_auto_id::<
      P,
      crate::parts::mail_merge_recipient_data_part::MailMergeRecipientDataPart,
    >(package, content_type)
  }

  #[inline]
  fn add_mail_merge_recipient_data_part_with_id<P>(
    &self,
    package: &mut P,
    content_type: impl Into<std::borrow::Cow<'static, str>>,
    relationship_id: impl Into<String>,
  ) -> Result<
    crate::parts::mail_merge_recipient_data_part::MailMergeRecipientDataPart,
    crate::common::SdkError,
  >
  where
    P: SdkPackage,
  {
    self.add_new_part_with_content_type::<P, crate::parts::mail_merge_recipient_data_part::MailMergeRecipientDataPart>(
      package,
      relationship_id,
      content_type,
    )
  }

  #[inline]
  fn add_mail_merge_recipient_data_part_by_type<P>(
    &self,
    package: &mut P,
    part_type: MailMergeRecipientDataPartType,
  ) -> Result<
    crate::parts::mail_merge_recipient_data_part::MailMergeRecipientDataPart,
    crate::common::SdkError,
  >
  where
    P: SdkPackage,
  {
    self.add_new_part_with_content_type_and_extension_auto_id::<
      P,
      crate::parts::mail_merge_recipient_data_part::MailMergeRecipientDataPart,
    >(package, part_type.content_type(), part_type.extension())
  }

  #[inline]
  fn add_mail_merge_recipient_data_part_by_type_with_id<P>(
    &self,
    package: &mut P,
    part_type: MailMergeRecipientDataPartType,
    relationship_id: impl Into<String>,
  ) -> Result<
    crate::parts::mail_merge_recipient_data_part::MailMergeRecipientDataPart,
    crate::common::SdkError,
  >
  where
    P: SdkPackage,
  {
    self.add_new_part_with_content_type_and_extension::<
      P,
      crate::parts::mail_merge_recipient_data_part::MailMergeRecipientDataPart,
    >(
      package,
      relationship_id,
      part_type.content_type(),
      part_type.extension(),
    )
  }

  #[inline]
  fn add_embedded_control_persistence_binary_data_part<P>(
    &self,
    package: &mut P,
    content_type: impl Into<std::borrow::Cow<'static, str>>,
  ) -> Result<
    crate::parts::embedded_control_persistence_binary_data_part::EmbeddedControlPersistenceBinaryDataPart,
    crate::common::SdkError,
  >
  where
    P: SdkPackage,
  {
    self.add_new_part_with_content_type_auto_id::<
      P,
      crate::parts::embedded_control_persistence_binary_data_part::EmbeddedControlPersistenceBinaryDataPart,
    >(package, content_type)
  }

  #[inline]
  fn add_embedded_control_persistence_binary_data_part_with_id<P>(
    &self,
    package: &mut P,
    content_type: impl Into<std::borrow::Cow<'static, str>>,
    relationship_id: impl Into<String>,
  ) -> Result<
    crate::parts::embedded_control_persistence_binary_data_part::EmbeddedControlPersistenceBinaryDataPart,
    crate::common::SdkError,
  >
  where
    P: SdkPackage,
  {
    self.add_new_part_with_content_type::<
      P,
      crate::parts::embedded_control_persistence_binary_data_part::EmbeddedControlPersistenceBinaryDataPart,
    >(package, relationship_id, content_type)
  }

  #[inline]
  fn add_embedded_control_persistence_binary_data_part_by_type<P>(
    &self,
    package: &mut P,
    part_type: EmbeddedControlPersistenceBinaryDataPartType,
  ) -> Result<
    crate::parts::embedded_control_persistence_binary_data_part::EmbeddedControlPersistenceBinaryDataPart,
    crate::common::SdkError,
  >
  where
    P: SdkPackage,
  {
    self.add_new_part_with_content_type_and_extension_auto_id::<
      P,
      crate::parts::embedded_control_persistence_binary_data_part::EmbeddedControlPersistenceBinaryDataPart,
    >(package, part_type.content_type(), part_type.extension())
  }

  #[inline]
  fn add_embedded_control_persistence_binary_data_part_by_type_with_id<P>(
    &self,
    package: &mut P,
    part_type: EmbeddedControlPersistenceBinaryDataPartType,
    relationship_id: impl Into<String>,
  ) -> Result<
    crate::parts::embedded_control_persistence_binary_data_part::EmbeddedControlPersistenceBinaryDataPart,
    crate::common::SdkError,
  >
  where
    P: SdkPackage,
  {
    self.add_new_part_with_content_type_and_extension::<
      P,
      crate::parts::embedded_control_persistence_binary_data_part::EmbeddedControlPersistenceBinaryDataPart,
    >(
      package,
      relationship_id,
      part_type.content_type(),
      part_type.extension(),
    )
  }

  #[inline]
  fn add_embedded_control_persistence_part<P>(
    &self,
    package: &mut P,
    content_type: impl Into<std::borrow::Cow<'static, str>>,
  ) -> Result<
    crate::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart,
    crate::common::SdkError,
  >
  where
    P: SdkPackage,
  {
    self.add_new_part_with_content_type_auto_id::<
      P,
      crate::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart,
    >(package, content_type)
  }

  #[inline]
  fn add_embedded_control_persistence_part_with_id<P>(
    &self,
    package: &mut P,
    content_type: impl Into<std::borrow::Cow<'static, str>>,
    relationship_id: impl Into<String>,
  ) -> Result<
    crate::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart,
    crate::common::SdkError,
  >
  where
    P: SdkPackage,
  {
    self.add_new_part_with_content_type::<
      P,
      crate::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart,
    >(package, relationship_id, content_type)
  }

  #[inline]
  fn add_embedded_control_persistence_part_by_type<P>(
    &self,
    package: &mut P,
    part_type: EmbeddedControlPersistencePartType,
  ) -> Result<
    crate::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart,
    crate::common::SdkError,
  >
  where
    P: SdkPackage,
  {
    self.add_new_part_with_content_type_and_extension_auto_id::<
      P,
      crate::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart,
    >(package, part_type.content_type(), part_type.extension())
  }

  #[inline]
  fn add_embedded_control_persistence_part_by_type_with_id<P>(
    &self,
    package: &mut P,
    part_type: EmbeddedControlPersistencePartType,
    relationship_id: impl Into<String>,
  ) -> Result<
    crate::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart,
    crate::common::SdkError,
  >
  where
    P: SdkPackage,
  {
    self.add_new_part_with_content_type_and_extension::<
      P,
      crate::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart,
    >(
      package,
      relationship_id,
      part_type.content_type(),
      part_type.extension(),
    )
  }

  #[inline]
  fn get_reference_relationship<'a, P: SdkPackage>(
    &'a self,
    package: &'a P,
    relationship_id: &str,
  ) -> Option<crate::common::RelationshipRef<'a>> {
    let part_slot = crate::private::SdkPartHandle::part_slot_optional(self, package)?;
    let storage = crate::sdk::SdkPackage::storage(package);
    storage
      .relationships(part_slot)?
      .get(relationship_id)
      .filter(|relationship| relationship.is_reference_relationship())
      .map(|relationship| crate::common::RelationshipRef::new(storage.token(), relationship))
  }

  #[inline]
  fn get_external_relationship<'a, P: SdkPackage>(
    &'a self,
    package: &'a P,
    relationship_id: &str,
  ) -> Option<crate::common::RelationshipRef<'a>> {
    let part_slot = crate::private::SdkPartHandle::part_slot_optional(self, package)?;
    let storage = crate::sdk::SdkPackage::storage(package);
    storage
      .relationships(part_slot)?
      .get_external_relationship(relationship_id)
      .map(|relationship| crate::common::RelationshipRef::new(storage.token(), relationship))
  }

  #[inline]
  fn get_hyperlink_relationship<'a, P: SdkPackage>(
    &'a self,
    package: &'a P,
    relationship_id: &str,
  ) -> Option<crate::common::RelationshipRef<'a>> {
    let part_slot = crate::private::SdkPartHandle::part_slot_optional(self, package)?;
    let storage = crate::sdk::SdkPackage::storage(package);
    storage
      .relationships(part_slot)?
      .get_hyperlink_relationship(relationship_id)
      .map(|relationship| crate::common::RelationshipRef::new(storage.token(), relationship))
  }

  #[inline]
  fn delete_reference_relationship<P: SdkPackage>(
    &self,
    package: &mut P,
    relationship_id: &str,
  ) -> Result<crate::common::Relationship, crate::common::SdkError> {
    let part_slot = crate::private::SdkPartHandle::part_slot(self, package)?;
    let package_token = crate::sdk::SdkPackage::storage(package).token();
    let relationship = crate::sdk::SdkPackage::storage_mut(package)
      .relationships_mut(part_slot)
      .ok_or(crate::common::SdkError::StalePart)?
      .remove_reference_relationship(relationship_id)?;
    Ok(crate::common::Relationship::new(
      package_token,
      relationship,
    ))
  }

  #[inline]
  fn delete_external_relationship<P: SdkPackage>(
    &self,
    package: &mut P,
    relationship_id: &str,
  ) -> Result<crate::common::Relationship, crate::common::SdkError> {
    let part_slot = crate::private::SdkPartHandle::part_slot(self, package)?;
    let package_token = crate::sdk::SdkPackage::storage(package).token();
    let relationship = crate::sdk::SdkPackage::storage_mut(package)
      .relationships_mut(part_slot)
      .ok_or(crate::common::SdkError::StalePart)?
      .remove_external_relationship(relationship_id)?;
    Ok(crate::common::Relationship::new(
      package_token,
      relationship,
    ))
  }

  #[inline]
  fn change_relationship_id<P: SdkPackage>(
    &self,
    package: &mut P,
    relationship_id: &str,
    new_relationship_id: impl Into<String>,
  ) -> Result<(), crate::common::SdkError> {
    let part_slot = crate::private::SdkPartHandle::part_slot(self, package)?;
    crate::sdk::SdkPackage::storage_mut(package)
      .relationships_mut(part_slot)
      .ok_or(crate::common::SdkError::StalePart)?
      .change_relationship_id(relationship_id, new_relationship_id)?;
    Ok(())
  }

  #[inline]
  fn external_relationships<'a, P: SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::common::RelationshipRef<'a>> {
    let storage = crate::sdk::SdkPackage::storage(package);
    crate::private::SdkPartHandle::part_slot_optional(self, package)
      .into_iter()
      .flat_map(move |part_slot| storage.relationships(part_slot))
      .flat_map(crate::common::RelationshipSet::external_relationships)
      .map(move |relationship| crate::common::RelationshipRef::new(storage.token(), relationship))
  }

  #[inline]
  fn hyperlink_relationships<'a, P: SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::common::RelationshipRef<'a>> {
    let storage = crate::sdk::SdkPackage::storage(package);
    crate::private::SdkPartHandle::part_slot_optional(self, package)
      .into_iter()
      .flat_map(move |part_slot| storage.relationships(part_slot))
      .flat_map(crate::common::RelationshipSet::hyperlink_relationships)
      .map(move |relationship| crate::common::RelationshipRef::new(storage.token(), relationship))
  }

  #[inline]
  fn data_part_reference_relationships<'a, P: SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::common::RelationshipRef<'a>> {
    let storage = crate::sdk::SdkPackage::storage(package);
    crate::private::SdkPartHandle::part_slot_optional(self, package)
      .into_iter()
      .flat_map(move |part_slot| storage.relationships(part_slot))
      .flat_map(crate::common::RelationshipSet::data_part_reference_relationships)
      .map(move |relationship| crate::common::RelationshipRef::new(storage.token(), relationship))
  }

  #[inline]
  fn path<'a, P: SdkPackage>(&self, package: &'a P) -> Option<&'a str> {
    let part_slot = crate::private::SdkPartHandle::part_slot_optional(self, package)?;
    crate::sdk::SdkPackage::storage(package)
      .part(part_slot)
      .map(crate::common::StoredPart::path)
  }

  #[inline]
  fn content_type<'a, P: SdkPackage>(&self, package: &'a P) -> Option<&'a str> {
    let part_slot = crate::private::SdkPartHandle::part_slot_optional(self, package)?;
    crate::sdk::SdkPackage::storage(package)
      .part(part_slot)
      .map(|part| {
        if Self::CONTENT_TYPE == "model/gltf-binary" && part.content_type() == "model/gltf.binary" {
          Self::CONTENT_TYPE
        } else {
          part.content_type()
        }
      })
  }

  #[inline]
  fn data<'a, P: SdkPackage>(&self, package: &'a P) -> Option<&'a [u8]> {
    self.try_data(package).ok().flatten()
  }

  #[inline]
  fn try_data<'a, P: SdkPackage>(
    &self,
    package: &'a P,
  ) -> Result<Option<&'a [u8]>, crate::common::SdkError> {
    let storage = crate::sdk::SdkPackage::storage(package);
    let part_slot = crate::private::SdkPartHandle::part_slot(self, package)?;
    storage.part_bytes(part_slot).map(Some)
  }

  /// Returns an owned, shared view of the part payload.
  ///
  /// Loading an archived part may allocate once; cloning the returned
  /// [`bytes::Bytes`] reuses that cached payload without copying its contents.
  #[inline]
  fn try_data_bytes<P: SdkPackage>(
    &self,
    package: &P,
  ) -> Result<bytes::Bytes, crate::common::SdkError> {
    let storage = crate::sdk::SdkPackage::storage(package);
    let part_slot = crate::private::SdkPartHandle::part_slot(self, package)?;
    storage.part_bytes_owned(part_slot)
  }

  #[inline]
  fn data_to_vec<P: SdkPackage>(&self, package: &P) -> Option<Vec<u8>> {
    self.data(package).map(<[u8]>::to_vec)
  }

  #[inline]
  fn data_as_str<'a, P: SdkPackage>(
    &self,
    package: &'a P,
  ) -> Result<Option<&'a str>, crate::common::SdkError> {
    self
      .try_data(package)?
      .map(std::str::from_utf8)
      .transpose()
      .map_err(|error| crate::common::SdkError::CommonError(error.to_string()))
  }

  #[inline]
  fn write_data_to<P: SdkPackage, W: std::io::Write>(
    &self,
    package: &P,
    writer: &mut W,
  ) -> Result<bool, crate::common::SdkError> {
    let Some(data) = self.try_data(package)? else {
      return Ok(false);
    };
    writer.write_all(data)?;
    Ok(true)
  }

  #[inline]
  fn set_data<P: SdkPackage>(
    &self,
    package: &mut P,
    data: impl Into<Vec<u8>>,
  ) -> Result<(), crate::common::SdkError> {
    let part_slot = crate::private::SdkPartHandle::part_slot(self, package)?;
    let _ = crate::sdk::SdkPackage::unload_root_element(package, part_slot);
    crate::sdk::SdkPackage::storage_mut(package).set_part_data(part_slot, data)
  }

  #[inline]
  fn feed_data<P: SdkPackage, R: std::io::Read>(
    &self,
    package: &mut P,
    reader: &mut R,
  ) -> Result<(), crate::common::SdkError> {
    let part_slot = crate::private::SdkPartHandle::part_slot(self, package)?;
    let _ = crate::sdk::SdkPackage::unload_root_element(package, part_slot);
    crate::sdk::SdkPackage::storage_mut(package).feed_part_data(part_slot, reader)
  }

  #[inline]
  fn parts<'a, P: SdkPackage + Sized>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::parts::IdPartPair<'a>> + 'a {
    let storage = crate::sdk::SdkPackage::storage(package);
    crate::private::SdkPartHandle::part_slot_optional(self, package)
      .into_iter()
      .flat_map(move |part_slot| storage.relationships(part_slot))
      .flat_map(|relationships| relationships.part_relationships())
      .filter_map(move |relationship| {
        let part_id = relationship.target_part_slot()?;
        let part = crate::parts::PartRef::from_part_slot(package, part_id)?;
        Some(crate::parts::IdPartPair::new(relationship.id(), part))
      })
  }

  #[inline]
  fn get_all_parts<'a, P: SdkPackage + Sized>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::parts::PartRef> + 'a {
    let Some(part_slot) = crate::private::SdkPartHandle::part_slot_optional(self, package) else {
      return Vec::new().into_iter();
    };
    let Some(relationships) = crate::sdk::SdkPackage::storage(package).relationships(part_slot)
    else {
      return Vec::new().into_iter();
    };
    collect_all_parts_from_relationships(package, relationships).into_iter()
  }

  #[inline]
  fn get_parent_parts<'a, P: SdkPackage + Sized>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::parts::PartRef> + 'a {
    let target_part_slot = crate::private::SdkPartHandle::part_slot_optional(self, package);
    package.get_all_parts().filter(move |part| {
      let Some(target_part_slot) = target_part_slot else {
        return false;
      };
      let Some(parent_slot) = part
        .part_key()
        .resolve_optional(crate::sdk::SdkPackage::storage(package))
      else {
        return false;
      };
      crate::sdk::SdkPackage::storage(package)
        .relationships(parent_slot)
        .is_some_and(|relationships| {
          relationships
            .part_relationships()
            .any(|relationship| relationship.target_part_slot() == Some(target_part_slot))
        })
    })
  }

  #[inline]
  fn get_part_by_id<P: SdkPackage + Sized>(
    &self,
    package: &P,
    relationship_id: &str,
  ) -> Option<crate::parts::PartRef> {
    let source_part_slot = crate::private::SdkPartHandle::part_slot_optional(self, package)?;
    let target_part_slot = crate::sdk::SdkPackage::storage(package)
      .target_part_slot(source_part_slot, relationship_id)?;
    crate::parts::PartRef::from_part_slot(package, target_part_slot)
  }

  #[inline]
  fn try_get_part_by_id<P: SdkPackage + Sized>(
    &self,
    package: &P,
    relationship_id: &str,
  ) -> Result<crate::parts::PartRef, crate::common::SdkError> {
    let source_part_slot = crate::private::SdkPartHandle::part_slot(self, package)?;
    let target_part_slot = crate::sdk::SdkPackage::storage(package)
      .target_part_slot(source_part_slot, relationship_id)
      .ok_or_else(|| crate::common::SdkError::PartRelationshipNotFound {
        relationship_id: relationship_id.to_string(),
      })?;
    crate::parts::PartRef::from_part_slot(package, target_part_slot)
      .ok_or(crate::common::SdkError::StalePart)
  }

  #[inline]
  fn get_parts_of_type<'a, P: SdkPackage + Sized, T: SdkPart>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = T> + 'a {
    let storage = crate::sdk::SdkPackage::storage(package);
    crate::private::SdkPartHandle::part_slot_optional(self, package)
      .into_iter()
      .flat_map(move |part_slot| storage.relationships(part_slot))
      .flat_map(|relationships| relationships.part_relationships())
      .filter_map(move |relationship| relationship_target_as_part::<T>(storage, relationship))
  }

  /// Returns the first matching relationship ID in source relationship order.
  #[inline]
  fn get_id_of_part<'a, P: SdkPackage, T: SdkPart>(
    &'a self,
    package: &'a P,
    part: &T,
  ) -> Result<&'a str, crate::common::SdkError> {
    let source_part_slot = crate::private::SdkPartHandle::part_slot(self, package)?;
    let target_part_slot = crate::private::SdkPartHandle::part_slot(part, package)?;
    crate::sdk::SdkPackage::storage(package)
      .relationships(source_part_slot)
      .ok_or(crate::common::SdkError::StalePart)?
      .part_relationships()
      .find_map(|relationship| {
        (relationship.target_part_slot() == Some(target_part_slot)).then_some(relationship.id())
      })
      .ok_or(crate::common::SdkError::PartNotReferenced)
  }

  #[inline]
  fn change_id_of_part<P: SdkPackage, T: SdkPart>(
    &self,
    package: &mut P,
    part: &T,
    new_relationship_id: impl Into<String>,
  ) -> Result<String, crate::common::SdkError> {
    let source_part_slot = crate::private::SdkPartHandle::part_slot(self, package)?;
    let target_part_slot = crate::private::SdkPartHandle::part_slot(part, package)?;
    let relationships = crate::sdk::SdkPackage::storage(package)
      .relationships(source_part_slot)
      .ok_or(crate::common::SdkError::StalePart)?;
    let old_relationship_id =
      unique_relationship_id_for_part(relationships, target_part_slot)?.to_string();
    self.change_relationship_id(package, &old_relationship_id, new_relationship_id)?;
    Ok(old_relationship_id)
  }

  #[inline]
  fn delete_part_by_id<P: SdkPackage>(
    &self,
    package: &mut P,
    relationship_id: &str,
  ) -> Result<bool, crate::common::SdkError> {
    let source_part_slot = crate::private::SdkPartHandle::part_slot(self, package)?;
    let Some(deleted_part_slots) = crate::sdk::SdkPackage::storage_mut(package)
      .delete_child_part(source_part_slot, relationship_id)?
    else {
      return Ok(false);
    };
    for part_slot in deleted_part_slots {
      let _ = crate::sdk::SdkPackage::take_root_element(package, part_slot);
    }
    Ok(true)
  }

  #[inline]
  fn delete_part<P: SdkPackage, T: SdkPart>(
    &self,
    package: &mut P,
    part: T,
  ) -> Result<bool, crate::common::SdkError> {
    let relationship_id = self.get_id_of_part(package, &part)?.to_string();
    self.delete_part_by_id(package, &relationship_id)
  }

  #[inline]
  fn delete_parts<P, T, I>(&self, package: &mut P, parts: I) -> Result<(), crate::common::SdkError>
  where
    P: SdkPackage,
    T: SdkPart,
    I: IntoIterator<Item = T>,
  {
    let relationship_ids = parts
      .into_iter()
      .map(|part| self.get_id_of_part(package, &part).map(str::to_string))
      .collect::<Result<Vec<_>, _>>()?;
    for relationship_id in relationship_ids {
      self.delete_part_by_id(package, &relationship_id)?;
    }
    Ok(())
  }

  #[inline]
  fn add_part<P: SdkPackage, T: SdkPart>(
    &self,
    package: &mut P,
    part: T,
  ) -> Result<T, crate::common::SdkError> {
    let source_part_slot = crate::private::SdkPartHandle::part_slot(self, package)?;
    let target_part_slot = crate::private::SdkPartHandle::part_slot(&part, package)?;
    if crate::sdk::SdkPackage::storage(package)
      .relationships(source_part_slot)
      .is_some_and(|relationships| {
        relationships
          .part_relationships()
          .any(|relationship| relationship.target_part_slot() == Some(target_part_slot))
      })
    {
      return Ok(part);
    }
    let relationship_id = self.next_relationship_id(package)?;
    self.add_part_with_id(package, part, relationship_id)
  }

  #[inline]
  fn add_part_with_id<P: SdkPackage, T: SdkPart>(
    &self,
    package: &mut P,
    part: T,
    relationship_id: impl Into<String>,
  ) -> Result<T, crate::common::SdkError> {
    let relationship_id = relationship_id.into();
    let source_part_slot = crate::private::SdkPartHandle::part_slot(self, package)?;
    let part_id = crate::private::SdkPartHandle::part_slot(&part, package)?;
    let content_type = crate::sdk::SdkPackage::storage(package)
      .part(part_id)
      .ok_or(crate::common::SdkError::StalePart)?
      .content_type()
      .to_string();
    let constraint = self.validate_child_part::<P, T>(package, &content_type)?;
    let relationship_type = constrained_relationship_type(
      constraint,
      crate::sdk::SdkPackage::storage(package)
        .part(part_id)
        .and_then(crate::common::StoredPart::relationship_type),
    );
    crate::sdk::SdkPackage::storage_mut(package).add_child_relationship_to_part(
      source_part_slot,
      relationship_id.clone(),
      relationship_type.as_ref(),
      part_id,
    )?;
    Ok(part_from_slot(
      crate::sdk::SdkPackage::storage(package),
      part_id,
    ))
  }

  #[inline]
  fn add_part_from_package<P, S, T>(
    &self,
    package: &mut P,
    source_package: &S,
    part: &T,
  ) -> Result<T, crate::common::SdkError>
  where
    P: SdkPackage,
    S: SdkPackage,
    T: SdkPart,
  {
    let source_part_slot = crate::private::SdkPartHandle::part_slot(self, package)?;
    let relationship_id = crate::sdk::SdkPackage::storage(package)
      .relationships(source_part_slot)
      .map(crate::common::RelationshipSet::next_relationship_id)
      .unwrap_or_else(|| "rId1".to_string());
    self.add_part_from_package_with_id(package, source_package, part, relationship_id)
  }

  #[inline]
  fn add_part_from_package_with_id<P, S, T>(
    &self,
    package: &mut P,
    source_package: &S,
    part: &T,
    relationship_id: impl Into<String>,
  ) -> Result<T, crate::common::SdkError>
  where
    P: SdkPackage,
    S: SdkPackage,
    T: SdkPart,
  {
    let relationship_id = relationship_id.into();
    let destination_source_slot = crate::private::SdkPartHandle::part_slot(self, package)?;
    let source_part_slot = crate::private::SdkPartHandle::part_slot(part, source_package)?;
    let source_content_type = crate::sdk::SdkPackage::storage(source_package)
      .part(source_part_slot)
      .ok_or(crate::common::SdkError::StalePart)?
      .content_type()
      .to_string();
    let constraint = self.validate_child_part::<P, T>(package, &source_content_type)?;
    let relationship_type = constrained_relationship_type(
      constraint,
      crate::sdk::SdkPackage::storage(source_package)
        .part(source_part_slot)
        .and_then(crate::common::StoredPart::relationship_type),
    );
    if crate::sdk::SdkPackage::storage(package).token()
      == crate::sdk::SdkPackage::storage(source_package).token()
    {
      crate::sdk::SdkPackage::storage_mut(package).add_child_relationship_to_part(
        destination_source_slot,
        relationship_id.clone(),
        relationship_type.as_ref(),
        source_part_slot,
      )?;
      return Ok(part_from_slot(
        crate::sdk::SdkPackage::storage(package),
        source_part_slot,
      ));
    }

    let (imported_part_id, added_count) = crate::sdk::SdkPackage::storage_mut(package)
      .import_part_tree_from(
        crate::sdk::SdkPackage::storage(source_package),
        source_part_slot,
        Some(destination_source_slot),
        relationship_id.clone(),
        relationship_type.as_ref(),
        |part_id, _| crate::sdk::SdkPackage::part_bytes_for_copy(source_package, part_id),
      )?;
    for _ in 0..added_count {
      crate::sdk::SdkPackage::push_root_element_slot(package);
    }
    Ok(part_from_slot(
      crate::sdk::SdkPackage::storage(package),
      imported_part_id,
    ))
  }

  #[inline]
  fn create_relationship_to_part<P: SdkPackage, T: SdkPart>(
    &self,
    package: &mut P,
    part: T,
  ) -> Result<String, crate::common::SdkError> {
    let source_part_slot = crate::private::SdkPartHandle::part_slot(self, package)?;
    let target_part_slot = crate::private::SdkPartHandle::part_slot(&part, package)?;
    if let Some(relationship_id) = crate::sdk::SdkPackage::storage(package)
      .relationships(source_part_slot)
      .and_then(|relationships| {
        relationships.part_relationships().find_map(|relationship| {
          (relationship.target_part_slot() == Some(target_part_slot)).then_some(relationship.id())
        })
      })
    {
      return Ok(relationship_id.to_string());
    }
    let relationship_id = self.next_relationship_id(package)?;
    self.create_relationship_to_part_with_id(package, part, relationship_id)
  }

  #[inline]
  fn create_relationship_to_part_with_id<P: SdkPackage, T: SdkPart>(
    &self,
    package: &mut P,
    part: T,
    relationship_id: impl Into<String>,
  ) -> Result<String, crate::common::SdkError> {
    let relationship_id = relationship_id.into();
    let source_part_slot = crate::private::SdkPartHandle::part_slot(self, package)?;
    let part_id = crate::private::SdkPartHandle::part_slot(&part, package)?;
    let content_type = crate::sdk::SdkPackage::storage(package)
      .part(part_id)
      .ok_or(crate::common::SdkError::StalePart)?
      .content_type()
      .to_string();
    let constraint = self.validate_child_part::<P, T>(package, &content_type)?;
    let relationship_type = constrained_relationship_type(
      constraint,
      crate::sdk::SdkPackage::storage(package)
        .part(part_id)
        .and_then(crate::common::StoredPart::relationship_type),
    );
    crate::sdk::SdkPackage::storage_mut(package).add_child_relationship_to_part(
      source_part_slot,
      relationship_id.clone(),
      relationship_type.as_ref(),
      part_id,
    )?;
    Ok(relationship_id)
  }
}

#[cfg(feature = "parts")]
impl<T> crate::private::SdkPartHandle for PartHandle<T> {
  #[inline]
  fn from_part_key(part_key: crate::common::PartKey) -> Self {
    Self::new(part_key)
  }

  #[inline]
  fn part_key(&self) -> crate::common::PartKey {
    self.key
  }
}

#[cfg(feature = "parts")]
impl<T> PartHandle<T>
where
  Self: SdkPart,
{
  #[inline]
  pub fn path<'a, P: SdkPackage>(&self, package: &'a P) -> Option<&'a str> {
    <Self as SdkPart>::path(self, package)
  }

  #[inline]
  pub fn content_type<'a, P: SdkPackage>(&self, package: &'a P) -> Option<&'a str> {
    <Self as SdkPart>::content_type(self, package)
  }

  #[inline]
  pub fn data<'a, P: SdkPackage>(&self, package: &'a P) -> Option<&'a [u8]> {
    <Self as SdkPart>::data(self, package)
  }

  #[inline]
  pub fn try_data<'a, P: SdkPackage>(
    &self,
    package: &'a P,
  ) -> Result<Option<&'a [u8]>, crate::common::SdkError> {
    <Self as SdkPart>::try_data(self, package)
  }

  /// Returns an owned, shared view of the part payload.
  ///
  /// Loading an archived part may allocate once; cloning the returned
  /// [`bytes::Bytes`] reuses that cached payload without copying its contents.
  #[inline]
  pub fn try_data_bytes<P: SdkPackage>(
    &self,
    package: &P,
  ) -> Result<bytes::Bytes, crate::common::SdkError> {
    <Self as SdkPart>::try_data_bytes(self, package)
  }

  #[inline]
  pub fn data_to_vec<P: SdkPackage>(&self, package: &P) -> Option<Vec<u8>> {
    <Self as SdkPart>::data_to_vec(self, package)
  }

  #[inline]
  pub fn data_as_str<'a, P: SdkPackage>(
    &self,
    package: &'a P,
  ) -> Result<Option<&'a str>, crate::common::SdkError> {
    <Self as SdkPart>::data_as_str(self, package)
  }

  #[inline]
  pub fn write_data_to<P: SdkPackage, W: std::io::Write>(
    &self,
    package: &P,
    writer: &mut W,
  ) -> Result<bool, crate::common::SdkError> {
    <Self as SdkPart>::write_data_to(self, package, writer)
  }

  #[inline]
  pub fn set_data<P: SdkPackage>(
    &self,
    package: &mut P,
    data: impl Into<Vec<u8>>,
  ) -> Result<(), crate::common::SdkError> {
    <Self as SdkPart>::set_data(self, package, data)
  }

  #[inline]
  pub fn feed_data<P: SdkPackage, R: std::io::Read>(
    &self,
    package: &mut P,
    reader: &mut R,
  ) -> Result<(), crate::common::SdkError> {
    <Self as SdkPart>::feed_data(self, package, reader)
  }

  #[inline]
  pub fn external_relationships<'a, P: SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::common::RelationshipRef<'a>> {
    <Self as SdkPart>::external_relationships(self, package)
  }

  #[inline]
  pub fn hyperlink_relationships<'a, P: SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::common::RelationshipRef<'a>> {
    <Self as SdkPart>::hyperlink_relationships(self, package)
  }

  #[inline]
  pub fn data_part_reference_relationships<'a, P: SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::common::RelationshipRef<'a>> {
    <Self as SdkPart>::data_part_reference_relationships(self, package)
  }

  #[inline]
  pub fn add_external_relationship<'a, P: SdkPackage>(
    &self,
    package: &'a mut P,
    relationship_id: impl Into<String>,
    relationship_type: impl Into<String>,
    target: impl Into<String>,
  ) -> Result<crate::common::RelationshipRef<'a>, crate::common::SdkError> {
    <Self as SdkPart>::add_external_relationship(
      self,
      package,
      relationship_id,
      relationship_type,
      target,
    )
  }

  #[inline]
  pub fn add_external_relationship_auto_id<'a, P: SdkPackage>(
    &self,
    package: &'a mut P,
    relationship_type: impl Into<String>,
    target: impl Into<String>,
  ) -> Result<crate::common::RelationshipRef<'a>, crate::common::SdkError> {
    <Self as SdkPart>::add_external_relationship_auto_id(self, package, relationship_type, target)
  }

  #[inline]
  pub fn add_hyperlink_relationship<'a, P: SdkPackage>(
    &self,
    package: &'a mut P,
    relationship_id: impl Into<String>,
    target: impl Into<String>,
  ) -> Result<crate::common::RelationshipRef<'a>, crate::common::SdkError> {
    <Self as SdkPart>::add_hyperlink_relationship(self, package, relationship_id, target)
  }

  #[inline]
  pub fn add_hyperlink_relationship_with_mode<'a, P: SdkPackage>(
    &self,
    package: &'a mut P,
    relationship_id: impl Into<String>,
    target: impl Into<String>,
    target_mode: crate::schemas::opc_relationships::TargetMode,
  ) -> Result<crate::common::RelationshipRef<'a>, crate::common::SdkError> {
    <Self as SdkPart>::add_hyperlink_relationship_with_mode(
      self,
      package,
      relationship_id,
      target,
      target_mode,
    )
  }

  #[inline]
  pub fn add_hyperlink_relationship_auto_id<'a, P: SdkPackage>(
    &self,
    package: &'a mut P,
    target: impl Into<String>,
    target_mode: crate::schemas::opc_relationships::TargetMode,
  ) -> Result<crate::common::RelationshipRef<'a>, crate::common::SdkError> {
    <Self as SdkPart>::add_hyperlink_relationship_auto_id(self, package, target, target_mode)
  }

  #[inline]
  pub fn get_reference_relationship<'a, P: SdkPackage>(
    &'a self,
    package: &'a P,
    relationship_id: &str,
  ) -> Option<crate::common::RelationshipRef<'a>> {
    <Self as SdkPart>::get_reference_relationship(self, package, relationship_id)
  }

  #[inline]
  pub fn get_external_relationship<'a, P: SdkPackage>(
    &'a self,
    package: &'a P,
    relationship_id: &str,
  ) -> Option<crate::common::RelationshipRef<'a>> {
    <Self as SdkPart>::get_external_relationship(self, package, relationship_id)
  }

  #[inline]
  pub fn get_hyperlink_relationship<'a, P: SdkPackage>(
    &'a self,
    package: &'a P,
    relationship_id: &str,
  ) -> Option<crate::common::RelationshipRef<'a>> {
    <Self as SdkPart>::get_hyperlink_relationship(self, package, relationship_id)
  }

  #[inline]
  pub fn delete_reference_relationship<P: SdkPackage>(
    &self,
    package: &mut P,
    relationship_id: &str,
  ) -> Result<crate::common::Relationship, crate::common::SdkError> {
    <Self as SdkPart>::delete_reference_relationship(self, package, relationship_id)
  }

  #[inline]
  pub fn delete_external_relationship<P: SdkPackage>(
    &self,
    package: &mut P,
    relationship_id: &str,
  ) -> Result<crate::common::Relationship, crate::common::SdkError> {
    <Self as SdkPart>::delete_external_relationship(self, package, relationship_id)
  }

  #[inline]
  pub fn change_relationship_id<P: SdkPackage>(
    &self,
    package: &mut P,
    relationship_id: &str,
    new_relationship_id: impl Into<String>,
  ) -> Result<(), crate::common::SdkError> {
    <Self as SdkPart>::change_relationship_id(self, package, relationship_id, new_relationship_id)
  }

  #[inline]
  pub fn add_new_part<P, U>(
    &self,
    package: &mut P,
    relationship_id: impl Into<String>,
  ) -> Result<U, crate::common::SdkError>
  where
    P: SdkPackage,
    U: SdkPart,
  {
    <Self as SdkPart>::add_new_part(self, package, relationship_id)
  }

  #[inline]
  pub fn add_new_part_with_content_type<P, U>(
    &self,
    package: &mut P,
    relationship_id: impl Into<String>,
    content_type: impl Into<std::borrow::Cow<'static, str>>,
  ) -> Result<U, crate::common::SdkError>
  where
    P: SdkPackage,
    U: SdkPart,
  {
    <Self as SdkPart>::add_new_part_with_content_type(self, package, relationship_id, content_type)
  }

  #[inline]
  pub fn add_new_part_auto_id<P, U>(&self, package: &mut P) -> Result<U, crate::common::SdkError>
  where
    P: SdkPackage,
    U: SdkPart,
  {
    <Self as SdkPart>::add_new_part_auto_id(self, package)
  }

  #[inline]
  pub fn add_new_part_with_content_type_auto_id<P, U>(
    &self,
    package: &mut P,
    content_type: impl Into<std::borrow::Cow<'static, str>>,
  ) -> Result<U, crate::common::SdkError>
  where
    P: SdkPackage,
    U: SdkPart,
  {
    <Self as SdkPart>::add_new_part_with_content_type_auto_id(self, package, content_type)
  }

  #[inline]
  pub fn add_new_part_with_content_type_and_extension<P, U>(
    &self,
    package: &mut P,
    relationship_id: impl Into<String>,
    content_type: impl Into<std::borrow::Cow<'static, str>>,
    extension: impl Into<std::borrow::Cow<'static, str>>,
  ) -> Result<U, crate::common::SdkError>
  where
    P: SdkPackage,
    U: SdkPart,
  {
    <Self as SdkPart>::add_new_part_with_content_type_and_extension(
      self,
      package,
      relationship_id,
      content_type,
      extension,
    )
  }

  #[inline]
  pub fn add_new_part_with_content_type_and_extension_auto_id<P, U>(
    &self,
    package: &mut P,
    content_type: impl Into<std::borrow::Cow<'static, str>>,
    extension: impl Into<std::borrow::Cow<'static, str>>,
  ) -> Result<U, crate::common::SdkError>
  where
    P: SdkPackage,
    U: SdkPart,
  {
    <Self as SdkPart>::add_new_part_with_content_type_and_extension_auto_id(
      self,
      package,
      content_type,
      extension,
    )
  }

  #[inline]
  pub fn add_extended_part<P>(
    &self,
    package: &mut P,
    relationship_type: impl Into<String>,
    content_type: impl Into<std::borrow::Cow<'static, str>>,
    target_extension: impl Into<std::borrow::Cow<'static, str>>,
  ) -> Result<crate::parts::extended_part::ExtendedPart, crate::common::SdkError>
  where
    P: SdkPackage,
  {
    <Self as SdkPart>::add_extended_part(
      self,
      package,
      relationship_type,
      content_type,
      target_extension,
    )
  }

  #[inline]
  pub fn add_extended_part_with_id<P>(
    &self,
    package: &mut P,
    relationship_type: impl Into<String>,
    content_type: impl Into<std::borrow::Cow<'static, str>>,
    target_extension: impl Into<std::borrow::Cow<'static, str>>,
    relationship_id: impl Into<String>,
  ) -> Result<crate::parts::extended_part::ExtendedPart, crate::common::SdkError>
  where
    P: SdkPackage,
  {
    <Self as SdkPart>::add_extended_part_with_id(
      self,
      package,
      relationship_type,
      content_type,
      target_extension,
      relationship_id,
    )
  }

  #[inline]
  pub fn add_image_part<P>(
    &self,
    package: &mut P,
    content_type: impl Into<std::borrow::Cow<'static, str>>,
  ) -> Result<crate::parts::image_part::ImagePart, crate::common::SdkError>
  where
    P: SdkPackage,
  {
    <Self as SdkPart>::add_image_part(self, package, content_type)
  }

  #[inline]
  pub fn add_image_part_with_id<P>(
    &self,
    package: &mut P,
    content_type: impl Into<std::borrow::Cow<'static, str>>,
    relationship_id: impl Into<String>,
  ) -> Result<crate::parts::image_part::ImagePart, crate::common::SdkError>
  where
    P: SdkPackage,
  {
    <Self as SdkPart>::add_image_part_with_id(self, package, content_type, relationship_id)
  }

  #[inline]
  pub fn add_new_part_with_content_type_and_path<P, U>(
    &self,
    package: &mut P,
    relationship_id: impl Into<String>,
    content_type: impl Into<std::borrow::Cow<'static, str>>,
    part_path: impl AsRef<str>,
  ) -> Result<U, crate::common::SdkError>
  where
    P: SdkPackage,
    U: SdkPart,
  {
    <Self as SdkPart>::add_new_part_with_content_type_and_path::<P, U>(
      self,
      package,
      relationship_id,
      content_type,
      part_path,
    )
  }

  #[inline]
  pub fn add_alternative_format_import_part<P>(
    &self,
    package: &mut P,
    content_type: impl Into<std::borrow::Cow<'static, str>>,
  ) -> Result<
    crate::parts::alternative_format_import_part::AlternativeFormatImportPart,
    crate::common::SdkError,
  >
  where
    P: SdkPackage,
  {
    <Self as SdkPart>::add_alternative_format_import_part(self, package, content_type)
  }

  #[inline]
  pub fn add_alternative_format_import_part_with_id<P>(
    &self,
    package: &mut P,
    content_type: impl Into<std::borrow::Cow<'static, str>>,
    relationship_id: impl Into<String>,
  ) -> Result<
    crate::parts::alternative_format_import_part::AlternativeFormatImportPart,
    crate::common::SdkError,
  >
  where
    P: SdkPackage,
  {
    <Self as SdkPart>::add_alternative_format_import_part_with_id(
      self,
      package,
      content_type,
      relationship_id,
    )
  }

  #[inline]
  pub fn add_alternative_format_import_part_by_type<P>(
    &self,
    package: &mut P,
    part_type: AlternativeFormatImportPartType,
  ) -> Result<
    crate::parts::alternative_format_import_part::AlternativeFormatImportPart,
    crate::common::SdkError,
  >
  where
    P: SdkPackage,
  {
    <Self as SdkPart>::add_alternative_format_import_part_by_type(self, package, part_type)
  }

  #[inline]
  pub fn add_alternative_format_import_part_by_type_with_id<P>(
    &self,
    package: &mut P,
    part_type: AlternativeFormatImportPartType,
    relationship_id: impl Into<String>,
  ) -> Result<
    crate::parts::alternative_format_import_part::AlternativeFormatImportPart,
    crate::common::SdkError,
  >
  where
    P: SdkPackage,
  {
    <Self as SdkPart>::add_alternative_format_import_part_by_type_with_id(
      self,
      package,
      part_type,
      relationship_id,
    )
  }

  #[inline]
  pub fn add_custom_xml_part<P>(
    &self,
    package: &mut P,
    content_type: impl Into<std::borrow::Cow<'static, str>>,
  ) -> Result<crate::parts::custom_xml_part::CustomXmlPart, crate::common::SdkError>
  where
    P: SdkPackage,
  {
    <Self as SdkPart>::add_custom_xml_part(self, package, content_type)
  }

  #[inline]
  pub fn add_custom_xml_part_with_id<P>(
    &self,
    package: &mut P,
    content_type: impl Into<std::borrow::Cow<'static, str>>,
    relationship_id: impl Into<String>,
  ) -> Result<crate::parts::custom_xml_part::CustomXmlPart, crate::common::SdkError>
  where
    P: SdkPackage,
  {
    <Self as SdkPart>::add_custom_xml_part_with_id(self, package, content_type, relationship_id)
  }

  #[inline]
  pub fn add_custom_xml_part_by_type<P>(
    &self,
    package: &mut P,
    part_type: CustomXmlPartType,
  ) -> Result<crate::parts::custom_xml_part::CustomXmlPart, crate::common::SdkError>
  where
    P: SdkPackage,
  {
    <Self as SdkPart>::add_custom_xml_part_by_type(self, package, part_type)
  }

  #[inline]
  pub fn add_custom_xml_part_by_type_with_id<P>(
    &self,
    package: &mut P,
    part_type: CustomXmlPartType,
    relationship_id: impl Into<String>,
  ) -> Result<crate::parts::custom_xml_part::CustomXmlPart, crate::common::SdkError>
  where
    P: SdkPackage,
  {
    <Self as SdkPart>::add_custom_xml_part_by_type_with_id(
      self,
      package,
      part_type,
      relationship_id,
    )
  }

  #[inline]
  pub fn add_custom_property_part_by_type<P>(
    &self,
    package: &mut P,
    part_type: CustomPropertyPartType,
  ) -> Result<crate::parts::custom_property_part::CustomPropertyPart, crate::common::SdkError>
  where
    P: SdkPackage,
  {
    <Self as SdkPart>::add_custom_property_part_by_type(self, package, part_type)
  }

  #[inline]
  pub fn add_custom_property_part_by_type_with_id<P>(
    &self,
    package: &mut P,
    part_type: CustomPropertyPartType,
    relationship_id: impl Into<String>,
  ) -> Result<crate::parts::custom_property_part::CustomPropertyPart, crate::common::SdkError>
  where
    P: SdkPackage,
  {
    <Self as SdkPart>::add_custom_property_part_by_type_with_id(
      self,
      package,
      part_type,
      relationship_id,
    )
  }

  #[inline]
  pub fn add_embedded_object_part_by_type<P>(
    &self,
    package: &mut P,
    part_type: EmbeddedObjectPartType,
  ) -> Result<crate::parts::embedded_object_part::EmbeddedObjectPart, crate::common::SdkError>
  where
    P: SdkPackage,
  {
    <Self as SdkPart>::add_embedded_object_part_by_type(self, package, part_type)
  }

  #[inline]
  pub fn add_embedded_object_part_by_type_with_id<P>(
    &self,
    package: &mut P,
    part_type: EmbeddedObjectPartType,
    relationship_id: impl Into<String>,
  ) -> Result<crate::parts::embedded_object_part::EmbeddedObjectPart, crate::common::SdkError>
  where
    P: SdkPackage,
  {
    <Self as SdkPart>::add_embedded_object_part_by_type_with_id(
      self,
      package,
      part_type,
      relationship_id,
    )
  }

  #[inline]
  pub fn add_embedded_package_part<P>(
    &self,
    package: &mut P,
    content_type: impl Into<std::borrow::Cow<'static, str>>,
  ) -> Result<crate::parts::embedded_package_part::EmbeddedPackagePart, crate::common::SdkError>
  where
    P: SdkPackage,
  {
    <Self as SdkPart>::add_embedded_package_part(self, package, content_type)
  }

  #[inline]
  pub fn add_embedded_package_part_with_id<P>(
    &self,
    package: &mut P,
    content_type: impl Into<std::borrow::Cow<'static, str>>,
    relationship_id: impl Into<String>,
  ) -> Result<crate::parts::embedded_package_part::EmbeddedPackagePart, crate::common::SdkError>
  where
    P: SdkPackage,
  {
    <Self as SdkPart>::add_embedded_package_part_with_id(
      self,
      package,
      content_type,
      relationship_id,
    )
  }

  #[inline]
  pub fn add_embedded_package_part_by_type<P>(
    &self,
    package: &mut P,
    part_type: EmbeddedPackagePartType,
  ) -> Result<crate::parts::embedded_package_part::EmbeddedPackagePart, crate::common::SdkError>
  where
    P: SdkPackage,
  {
    <Self as SdkPart>::add_embedded_package_part_by_type(self, package, part_type)
  }

  #[inline]
  pub fn add_embedded_package_part_by_type_with_id<P>(
    &self,
    package: &mut P,
    part_type: EmbeddedPackagePartType,
    relationship_id: impl Into<String>,
  ) -> Result<crate::parts::embedded_package_part::EmbeddedPackagePart, crate::common::SdkError>
  where
    P: SdkPackage,
  {
    <Self as SdkPart>::add_embedded_package_part_by_type_with_id(
      self,
      package,
      part_type,
      relationship_id,
    )
  }

  #[inline]
  pub fn add_font_part_by_type<P>(
    &self,
    package: &mut P,
    part_type: FontPartType,
  ) -> Result<crate::parts::font_part::FontPart, crate::common::SdkError>
  where
    P: SdkPackage,
  {
    <Self as SdkPart>::add_font_part_by_type(self, package, part_type)
  }

  #[inline]
  pub fn add_font_part_by_type_with_id<P>(
    &self,
    package: &mut P,
    part_type: FontPartType,
    relationship_id: impl Into<String>,
  ) -> Result<crate::parts::font_part::FontPart, crate::common::SdkError>
  where
    P: SdkPackage,
  {
    <Self as SdkPart>::add_font_part_by_type_with_id(self, package, part_type, relationship_id)
  }

  #[inline]
  pub fn add_mail_merge_recipient_data_part_by_type<P>(
    &self,
    package: &mut P,
    part_type: MailMergeRecipientDataPartType,
  ) -> Result<
    crate::parts::mail_merge_recipient_data_part::MailMergeRecipientDataPart,
    crate::common::SdkError,
  >
  where
    P: SdkPackage,
  {
    <Self as SdkPart>::add_mail_merge_recipient_data_part_by_type(self, package, part_type)
  }

  #[inline]
  pub fn add_mail_merge_recipient_data_part_by_type_with_id<P>(
    &self,
    package: &mut P,
    part_type: MailMergeRecipientDataPartType,
    relationship_id: impl Into<String>,
  ) -> Result<
    crate::parts::mail_merge_recipient_data_part::MailMergeRecipientDataPart,
    crate::common::SdkError,
  >
  where
    P: SdkPackage,
  {
    <Self as SdkPart>::add_mail_merge_recipient_data_part_by_type_with_id(
      self,
      package,
      part_type,
      relationship_id,
    )
  }

  #[inline]
  pub fn add_embedded_control_persistence_binary_data_part_by_type<P>(
    &self,
    package: &mut P,
    part_type: EmbeddedControlPersistenceBinaryDataPartType,
  ) -> Result<
    crate::parts::embedded_control_persistence_binary_data_part::EmbeddedControlPersistenceBinaryDataPart,
    crate::common::SdkError,
  >
  where
    P: SdkPackage,
  {
    <Self as SdkPart>::add_embedded_control_persistence_binary_data_part_by_type(
      self, package, part_type,
    )
  }

  #[inline]
  pub fn add_embedded_control_persistence_binary_data_part_by_type_with_id<P>(
    &self,
    package: &mut P,
    part_type: EmbeddedControlPersistenceBinaryDataPartType,
    relationship_id: impl Into<String>,
  ) -> Result<
    crate::parts::embedded_control_persistence_binary_data_part::EmbeddedControlPersistenceBinaryDataPart,
    crate::common::SdkError,
  >
  where
    P: SdkPackage,
  {
    <Self as SdkPart>::add_embedded_control_persistence_binary_data_part_by_type_with_id(
      self,
      package,
      part_type,
      relationship_id,
    )
  }

  #[inline]
  pub fn add_embedded_control_persistence_part_by_type<P>(
    &self,
    package: &mut P,
    part_type: EmbeddedControlPersistencePartType,
  ) -> Result<
    crate::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart,
    crate::common::SdkError,
  >
  where
    P: SdkPackage,
  {
    <Self as SdkPart>::add_embedded_control_persistence_part_by_type(self, package, part_type)
  }

  #[inline]
  pub fn add_embedded_control_persistence_part_by_type_with_id<P>(
    &self,
    package: &mut P,
    part_type: EmbeddedControlPersistencePartType,
    relationship_id: impl Into<String>,
  ) -> Result<
    crate::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart,
    crate::common::SdkError,
  >
  where
    P: SdkPackage,
  {
    <Self as SdkPart>::add_embedded_control_persistence_part_by_type_with_id(
      self,
      package,
      part_type,
      relationship_id,
    )
  }

  pub fn parts<'a, P: SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::parts::IdPartPair<'a>> + 'a {
    <Self as SdkPart>::parts(self, package)
  }

  #[inline]
  pub fn get_all_parts<'a, P: SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::parts::PartRef> + 'a {
    <Self as SdkPart>::get_all_parts(self, package)
  }

  #[inline]
  pub fn get_parent_parts<'a, P: SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::parts::PartRef> + 'a {
    <Self as SdkPart>::get_parent_parts(self, package)
  }

  #[inline]
  pub fn get_part_by_id<P: SdkPackage>(
    &self,
    package: &P,
    relationship_id: &str,
  ) -> Option<crate::parts::PartRef> {
    <Self as SdkPart>::get_part_by_id(self, package, relationship_id)
  }

  #[inline]
  pub fn try_get_part_by_id<P: SdkPackage>(
    &self,
    package: &P,
    relationship_id: &str,
  ) -> Result<crate::parts::PartRef, crate::common::SdkError> {
    <Self as SdkPart>::try_get_part_by_id(self, package, relationship_id)
  }

  pub fn get_parts_of_type<'a, P: SdkPackage, U: SdkPart>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = U> + 'a {
    <Self as SdkPart>::get_parts_of_type::<P, U>(self, package)
  }

  /// Returns the first matching relationship ID in source relationship order.
  pub fn get_id_of_part<'a, P: SdkPackage, U: SdkPart>(
    &'a self,
    package: &'a P,
    part: &U,
  ) -> Result<&'a str, crate::common::SdkError> {
    <Self as SdkPart>::get_id_of_part(self, package, part)
  }

  #[inline]
  pub fn change_id_of_part<P: SdkPackage, U: SdkPart>(
    &self,
    package: &mut P,
    part: &U,
    new_relationship_id: impl Into<String>,
  ) -> Result<String, crate::common::SdkError> {
    <Self as SdkPart>::change_id_of_part(self, package, part, new_relationship_id)
  }

  #[inline]
  pub fn delete_part_by_id<P: SdkPackage>(
    &self,
    package: &mut P,
    relationship_id: &str,
  ) -> Result<bool, crate::common::SdkError> {
    <Self as SdkPart>::delete_part_by_id(self, package, relationship_id)
  }

  #[inline]
  pub fn delete_part<P: SdkPackage, U: SdkPart>(
    &self,
    package: &mut P,
    part: U,
  ) -> Result<bool, crate::common::SdkError> {
    <Self as SdkPart>::delete_part(self, package, part)
  }

  #[inline]
  pub fn delete_parts<P, U, I>(
    &self,
    package: &mut P,
    parts: I,
  ) -> Result<(), crate::common::SdkError>
  where
    P: SdkPackage,
    U: SdkPart,
    I: IntoIterator<Item = U>,
  {
    <Self as SdkPart>::delete_parts(self, package, parts)
  }

  #[inline]
  pub fn add_part<P: SdkPackage, U: SdkPart>(
    &self,
    package: &mut P,
    part: U,
  ) -> Result<U, crate::common::SdkError> {
    <Self as SdkPart>::add_part(self, package, part)
  }

  #[inline]
  pub fn add_part_with_id<P: SdkPackage, U: SdkPart>(
    &self,
    package: &mut P,
    part: U,
    relationship_id: impl Into<String>,
  ) -> Result<U, crate::common::SdkError> {
    <Self as SdkPart>::add_part_with_id(self, package, part, relationship_id)
  }

  #[inline]
  pub fn add_part_from_package<P: SdkPackage, S: SdkPackage, U: SdkPart>(
    &self,
    package: &mut P,
    source_package: &S,
    part: &U,
  ) -> Result<U, crate::common::SdkError> {
    <Self as SdkPart>::add_part_from_package(self, package, source_package, part)
  }

  #[inline]
  pub fn add_part_from_package_with_id<P: SdkPackage, S: SdkPackage, U: SdkPart>(
    &self,
    package: &mut P,
    source_package: &S,
    part: &U,
    relationship_id: impl Into<String>,
  ) -> Result<U, crate::common::SdkError> {
    <Self as SdkPart>::add_part_from_package_with_id(
      self,
      package,
      source_package,
      part,
      relationship_id,
    )
  }

  #[inline]
  pub fn create_relationship_to_part<P: SdkPackage, U: SdkPart>(
    &self,
    package: &mut P,
    part: U,
  ) -> Result<String, crate::common::SdkError> {
    <Self as SdkPart>::create_relationship_to_part(self, package, part)
  }

  #[inline]
  pub fn create_relationship_to_part_with_id<P: SdkPackage, U: SdkPart>(
    &self,
    package: &mut P,
    part: U,
    relationship_id: impl Into<String>,
  ) -> Result<String, crate::common::SdkError> {
    <Self as SdkPart>::create_relationship_to_part_with_id(self, package, part, relationship_id)
  }

  #[inline]
  pub fn add_audio_reference_relationship<P: SdkPackage>(
    &self,
    package: &mut P,
    media_data_part: &crate::common::MediaDataPart,
  ) -> Result<String, crate::common::SdkError> {
    <Self as SdkPart>::add_audio_reference_relationship(self, package, media_data_part)
  }

  #[inline]
  pub fn add_audio_reference_relationship_with_id<P: SdkPackage>(
    &self,
    package: &mut P,
    media_data_part: &crate::common::MediaDataPart,
    relationship_id: impl Into<String>,
  ) -> Result<String, crate::common::SdkError> {
    <Self as SdkPart>::add_audio_reference_relationship_with_id(
      self,
      package,
      media_data_part,
      relationship_id,
    )
  }

  #[inline]
  pub fn add_media_reference_relationship<P: SdkPackage>(
    &self,
    package: &mut P,
    media_data_part: &crate::common::MediaDataPart,
  ) -> Result<String, crate::common::SdkError> {
    <Self as SdkPart>::add_media_reference_relationship(self, package, media_data_part)
  }

  #[inline]
  pub fn add_media_reference_relationship_with_id<P: SdkPackage>(
    &self,
    package: &mut P,
    media_data_part: &crate::common::MediaDataPart,
    relationship_id: impl Into<String>,
  ) -> Result<String, crate::common::SdkError> {
    <Self as SdkPart>::add_media_reference_relationship_with_id(
      self,
      package,
      media_data_part,
      relationship_id,
    )
  }

  #[inline]
  pub fn add_video_reference_relationship<P: SdkPackage>(
    &self,
    package: &mut P,
    media_data_part: &crate::common::MediaDataPart,
  ) -> Result<String, crate::common::SdkError> {
    <Self as SdkPart>::add_video_reference_relationship(self, package, media_data_part)
  }

  #[inline]
  pub fn add_video_reference_relationship_with_id<P: SdkPackage>(
    &self,
    package: &mut P,
    media_data_part: &crate::common::MediaDataPart,
    relationship_id: impl Into<String>,
  ) -> Result<String, crate::common::SdkError> {
    <Self as SdkPart>::add_video_reference_relationship_with_id(
      self,
      package,
      media_data_part,
      relationship_id,
    )
  }

  #[inline]
  pub fn add_data_part_reference_relationship_from_existing<P: SdkPackage>(
    &self,
    package: &mut P,
    relationship: crate::common::Relationship,
  ) -> Result<String, crate::common::SdkError> {
    <Self as SdkPart>::add_data_part_reference_relationship_from_existing(
      self,
      package,
      relationship,
    )
  }
}

#[cfg(all(test, feature = "mce"))]
mod tests {
  fn split_mce_prefix_list(value: &[u8]) -> Vec<Vec<u8>> {
    let mut prefixes = Vec::new();
    crate::mce::for_each_mce_prefix(value, |prefix| {
      prefixes.push(prefix.to_vec());
      Ok(())
    })
    .expect("valid MCE prefix list");
    prefixes
  }

  #[test]
  fn mce_prefix_list_splits_raw_and_entity_whitespace() {
    assert_eq!(
      split_mce_prefix_list(b"w14 wp14"),
      [b"w14".to_vec(), b"wp14".to_vec()]
    );
    assert_eq!(
      split_mce_prefix_list(b"w14\twp14"),
      [b"w14".to_vec(), b"wp14".to_vec()]
    );
    assert_eq!(
      split_mce_prefix_list(b"w14&#x9;wp14"),
      [b"w14".to_vec(), b"wp14".to_vec()]
    );
    assert_eq!(
      split_mce_prefix_list(b"w14&#9;wp14&#10;w15&#13;w16"),
      [
        b"w14".to_vec(),
        b"wp14".to_vec(),
        b"w15".to_vec(),
        b"w16".to_vec()
      ]
    );
    assert!(split_mce_prefix_list(b"  &#x9;&#xA;&#xD; ").is_empty());
    assert_eq!(
      split_mce_prefix_list(b"w14\x0Bwp14"),
      [b"w14\x0Bwp14".to_vec()]
    );
  }
}
