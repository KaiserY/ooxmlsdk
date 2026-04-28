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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RelationshipModelEntry {
  Child { field_index: u16, item_index: usize },
  Fallback(usize),
  Relationship(usize),
}

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
fn collect_all_parts_from_relationships<P: SdkPackage + Sized>(
  package: &P,
  relationships: &crate::common::RelationshipSet,
) -> Vec<crate::parts::PartRef> {
  let mut parts = Vec::new();
  let mut visited = std::collections::HashSet::new();
  let mut queue = std::collections::VecDeque::new();

  for relationship in relationships.part_relationships() {
    if is_data_part_reference_relationship(relationship) {
      continue;
    }
    if let Some(part_id) = relationship.target_part_id()
      && visited.insert(part_id)
    {
      queue.push_back(part_id);
    }
  }

  while let Some(part_id) = queue.pop_front() {
    if let Some(part) = crate::parts::PartRef::from_part_id(package, part_id) {
      parts.push(part);
    }

    if let Some(relationships) =
      crate::sdk::SdkPackageInternal::storage(package).relationships(part_id)
    {
      for relationship in relationships.part_relationships() {
        if is_data_part_reference_relationship(relationship) {
          continue;
        }
        if let Some(child_part_id) = relationship.target_part_id()
          && visited.insert(child_part_id)
        {
          queue.push_back(child_part_id);
        }
      }
    }
  }

  parts
}

#[cfg(feature = "parts")]
fn relationship_target_as_part<T: SdkPart>(
  storage: &crate::common::SdkPackageStorage,
  relationship: &crate::common::RelationshipInfo,
) -> Option<T> {
  let part_id = relationship.target_part_id()?;
  let part = storage.part(part_id)?;

  if std::any::TypeId::of::<T>()
    == std::any::TypeId::of::<crate::parts::extended_part::ExtendedPart>()
  {
    if let Some(crate::parts::PartRef::ExtendedPart(part)) =
      crate::parts::PartRef::from_relationship_storage(storage, relationship)
    {
      let part: Box<dyn std::any::Any> = Box::new(part);
      return part.downcast::<T>().ok().map(|part| *part);
    }
    return None;
  }

  let relationship_type = part.relationship_type()?;
  if crate::common::part_descriptor_matches(
    relationship_type,
    part.content_type(),
    part.path(),
    T::RELATIONSHIP_TYPE,
    T::CONTENT_TYPE,
    T::PATH_PREFIX,
    T::TARGET_NAME,
  ) {
    Some(T::from_relationship_id_with_relationships(
      storage,
      relationship.id(),
      part_id,
    ))
  } else {
    None
  }
}

#[cfg(feature = "parts")]
#[inline]
fn is_data_part_reference_relationship(relationship: &crate::common::RelationshipInfo) -> bool {
  matches!(
    relationship.relationship_type(),
    crate::common::RelationshipSet::AUDIO_REFERENCE_RELATIONSHIP_TYPE
      | crate::common::RelationshipSet::MEDIA_REFERENCE_RELATIONSHIP_TYPE
      | crate::common::RelationshipSet::VIDEO_REFERENCE_RELATIONSHIP_TYPE
  )
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
#[inline]
fn default_new_package_part_content_type<T: SdkPart>() -> std::borrow::Cow<'static, str> {
  if T::CONTENT_TYPE.is_empty() {
    default_main_part_content_type::<T>()
      .map(std::borrow::Cow::Borrowed)
      .unwrap_or_else(|| std::borrow::Cow::Borrowed(T::CONTENT_TYPE))
  } else {
    std::borrow::Cow::Borrowed(T::CONTENT_TYPE)
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
  ProcessLoadedPartsOnly,
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
  pub markup_compatibility_process_settings: MarkupCompatibilityProcessSettings,
  pub ignore_calculation_chain_part_relationship: bool,
}

#[cfg(feature = "parts")]
pub(crate) trait SdkPackageInternal {
  fn storage(&self) -> &crate::common::SdkPackageStorage;

  fn storage_mut(&mut self) -> &mut crate::common::SdkPackageStorage;

  #[inline]
  fn open_settings(&self) -> &OpenSettings {
    static DEFAULT_SETTINGS: OpenSettings = OpenSettings {
      markup_compatibility_process_settings: MarkupCompatibilityProcessSettings {
        process_mode: MarkupCompatibilityProcessMode::NoProcess,
        target_file_format_version: FileFormatVersion::Office2007,
      },
      ignore_calculation_chain_part_relationship: false,
    };
    &DEFAULT_SETTINGS
  }

  #[inline]
  fn relationships(&self) -> &crate::common::RelationshipSet {
    crate::sdk::SdkPackageInternal::storage(self).package_relationships()
  }

  #[inline]
  fn relationships_mut(&mut self) -> &mut crate::common::RelationshipSet {
    crate::sdk::SdkPackageInternal::storage_mut(self).package_relationships_mut()
  }

  #[inline]
  fn refresh_relationship_model_from_storage(&mut self) {}

  #[inline]
  fn root_element(&self, part_id: crate::common::PartId) -> Option<&crate::parts::PartRootElement> {
    let _ = part_id;
    None
  }

  #[inline]
  fn root_element_slot_mut(
    &mut self,
    part_id: crate::common::PartId,
  ) -> Option<&mut Option<crate::parts::PartRootElement>> {
    let _ = part_id;
    None
  }

  #[inline]
  fn push_root_element_slot(&mut self) {}

  #[inline]
  fn is_root_element_loaded(&self, part_id: crate::common::PartId) -> bool {
    self.root_element(part_id).is_some()
  }

  #[inline]
  fn unload_root_element(
    &mut self,
    part_id: crate::common::PartId,
  ) -> Option<crate::parts::PartRootElement> {
    self.root_element_slot_mut(part_id)?.take()
  }

  #[inline]
  fn part_bytes_for_copy(
    &self,
    part_id: crate::common::PartId,
  ) -> Result<Vec<u8>, crate::common::SdkError> {
    if let Some(root_element) = self.root_element(part_id) {
      root_element.to_xml_bytes()
    } else {
      let part = Self::storage(self).part(part_id).ok_or_else(|| {
        crate::common::SdkError::CommonError(format!(
          "part id {part_id:?} is not present in package storage"
        ))
      })?;
      Ok(part.data().bytes().to_vec())
    }
  }
}

#[cfg(feature = "parts")]
#[allow(private_bounds)]
pub trait SdkPackage: SdkPackageInternal {
  #[inline]
  fn open_settings(&self) -> &OpenSettings {
    crate::sdk::SdkPackageInternal::open_settings(self)
  }

  #[inline]
  fn load_all_parts(&mut self) -> Result<(), crate::common::SdkError>
  where
    Self: Sized,
  {
    crate::parts::load_all_part_roots(self)
  }

  #[inline]
  fn add_external_relationship(
    &mut self,
    relationship_id: impl Into<String>,
    relationship_type: impl Into<String>,
    target: impl Into<String>,
  ) -> Result<crate::common::RelationshipRef<'_>, crate::common::SdkError> {
    let relationship_id = relationship_id.into();
    crate::sdk::SdkPackageInternal::relationships_mut(self).add_external_relationship(
      relationship_id.clone(),
      relationship_type,
      target,
    )?;
    crate::sdk::SdkPackageInternal::refresh_relationship_model_from_storage(self);
    Ok(
      crate::sdk::SdkPackageInternal::relationships(self)
        .get(&relationship_id)
        .expect("relationship was just added")
        .into(),
    )
  }

  #[inline]
  fn add_external_relationship_auto_id(
    &mut self,
    relationship_type: impl Into<String>,
    target: impl Into<String>,
  ) -> Result<crate::common::RelationshipRef<'_>, crate::common::SdkError> {
    let relationship_id =
      crate::sdk::SdkPackageInternal::relationships(self).next_relationship_id();
    self.add_external_relationship(relationship_id, relationship_type, target)
  }

  #[inline]
  fn add_hyperlink_relationship(
    &mut self,
    relationship_id: impl Into<String>,
    target: impl Into<String>,
  ) -> Result<crate::common::RelationshipRef<'_>, crate::common::SdkError> {
    let relationship_id = relationship_id.into();
    crate::sdk::SdkPackageInternal::relationships_mut(self)
      .add_hyperlink_relationship(relationship_id.clone(), target)?;
    crate::sdk::SdkPackageInternal::refresh_relationship_model_from_storage(self);
    Ok(
      crate::sdk::SdkPackageInternal::relationships(self)
        .get(&relationship_id)
        .expect("relationship was just added")
        .into(),
    )
  }

  #[inline]
  fn add_hyperlink_relationship_with_mode(
    &mut self,
    relationship_id: impl Into<String>,
    target: impl Into<String>,
    target_mode: crate::schemas::opc_relationships::TargetMode,
  ) -> Result<crate::common::RelationshipRef<'_>, crate::common::SdkError> {
    let relationship_id = relationship_id.into();
    crate::sdk::SdkPackageInternal::relationships_mut(self).add_hyperlink_relationship_with_mode(
      relationship_id.clone(),
      target,
      target_mode,
    )?;
    crate::sdk::SdkPackageInternal::refresh_relationship_model_from_storage(self);
    Ok(
      crate::sdk::SdkPackageInternal::relationships(self)
        .get(&relationship_id)
        .expect("relationship was just added")
        .into(),
    )
  }

  #[inline]
  fn add_hyperlink_relationship_auto_id(
    &mut self,
    target: impl Into<String>,
    target_mode: crate::schemas::opc_relationships::TargetMode,
  ) -> Result<crate::common::RelationshipRef<'_>, crate::common::SdkError> {
    let relationship_id =
      crate::sdk::SdkPackageInternal::relationships(self).next_relationship_id();
    self.add_hyperlink_relationship_with_mode(relationship_id, target, target_mode)
  }

  #[inline]
  fn get_reference_relationship(
    &self,
    relationship_id: &str,
  ) -> Option<crate::common::RelationshipRef<'_>> {
    crate::sdk::SdkPackageInternal::relationships(self)
      .get(relationship_id)
      .filter(|relationship| relationship.is_reference_relationship())
      .map(Into::into)
  }

  #[inline]
  fn get_external_relationship(
    &self,
    relationship_id: &str,
  ) -> Option<crate::common::RelationshipRef<'_>> {
    crate::sdk::SdkPackageInternal::relationships(self)
      .get_external_relationship(relationship_id)
      .map(Into::into)
  }

  #[inline]
  fn get_hyperlink_relationship(
    &self,
    relationship_id: &str,
  ) -> Option<crate::common::RelationshipRef<'_>> {
    crate::sdk::SdkPackageInternal::relationships(self)
      .get_hyperlink_relationship(relationship_id)
      .map(Into::into)
  }

  #[inline]
  fn delete_reference_relationship(
    &mut self,
    relationship_id: &str,
  ) -> Result<crate::common::Relationship, crate::common::SdkError> {
    let relationship = crate::sdk::SdkPackageInternal::relationships_mut(self)
      .remove_reference_relationship(relationship_id)?;
    crate::sdk::SdkPackageInternal::refresh_relationship_model_from_storage(self);
    Ok(relationship.into())
  }

  #[inline]
  fn delete_external_relationship(
    &mut self,
    relationship_id: &str,
  ) -> Result<crate::common::Relationship, crate::common::SdkError> {
    let relationship = crate::sdk::SdkPackageInternal::relationships_mut(self)
      .remove_external_relationship(relationship_id)?;
    crate::sdk::SdkPackageInternal::refresh_relationship_model_from_storage(self);
    Ok(relationship.into())
  }

  #[inline]
  fn change_relationship_id(
    &mut self,
    relationship_id: &str,
    new_relationship_id: impl Into<String>,
  ) -> Result<(), crate::common::SdkError> {
    crate::sdk::SdkPackageInternal::relationships_mut(self)
      .change_relationship_id(relationship_id, new_relationship_id)?;
    crate::sdk::SdkPackageInternal::refresh_relationship_model_from_storage(self);
    Ok(())
  }

  #[inline]
  fn external_relationships(&self) -> impl Iterator<Item = crate::common::RelationshipRef<'_>> {
    crate::sdk::SdkPackageInternal::relationships(self)
      .external_relationships()
      .map(Into::into)
  }

  #[inline]
  fn hyperlink_relationships(&self) -> impl Iterator<Item = crate::common::RelationshipRef<'_>> {
    crate::sdk::SdkPackageInternal::relationships(self)
      .hyperlink_relationships()
      .map(Into::into)
  }

  #[inline]
  fn data_part_reference_relationships(
    &self,
  ) -> impl Iterator<Item = crate::common::RelationshipRef<'_>> {
    crate::sdk::SdkPackageInternal::relationships(self)
      .data_part_reference_relationships()
      .map(Into::into)
  }

  #[inline]
  fn media_data_parts(&self) -> impl Iterator<Item = crate::common::MediaDataPart> + '_ {
    crate::sdk::SdkPackageInternal::storage(self)
      .media_data_parts()
      .map(|(part_id, part)| {
        crate::common::MediaDataPart::from_part_id(
          crate::sdk::SdkPackageInternal::storage(self).id(),
          part_id,
          part.path(),
        )
      })
  }

  #[inline]
  fn delete_unused_media_data_parts(&mut self) -> usize {
    crate::sdk::SdkPackageInternal::storage_mut(self).delete_unused_media_data_parts()
  }

  #[inline]
  fn parts(&self) -> impl Iterator<Item = crate::parts::IdPartPair<'_>> + '_
  where
    Self: Sized,
  {
    crate::sdk::SdkPackageInternal::relationships(self)
      .iter()
      .filter_map(|relationship| {
        let part_id = relationship.target_part_id()?;
        let part = crate::parts::PartRef::from_part_id(self, part_id)?;
        Some(crate::parts::IdPartPair::new(relationship.id(), part))
      })
  }

  #[inline]
  fn get_all_parts(&self) -> impl Iterator<Item = crate::parts::PartRef> + '_
  where
    Self: Sized,
  {
    collect_all_parts_from_relationships(self, crate::sdk::SdkPackageInternal::relationships(self))
      .into_iter()
  }

  #[inline]
  fn get_part_by_id(&self, relationship_id: &str) -> Option<crate::parts::PartRef>
  where
    Self: Sized,
  {
    let part_id = crate::sdk::SdkPackageInternal::relationships(self)
      .get(relationship_id)?
      .target_part_id()?;
    crate::parts::PartRef::from_part_id(self, part_id)
  }

  #[inline]
  fn get_part_by_id_required(
    &self,
    relationship_id: &str,
  ) -> Result<crate::parts::PartRef, crate::common::SdkError>
  where
    Self: Sized,
  {
    self.get_part_by_id(relationship_id).ok_or_else(|| {
      crate::common::SdkError::CommonError(format!(
        "part relationship id {relationship_id} does not exist"
      ))
    })
  }

  #[inline]
  fn try_get_part_by_id(&self, relationship_id: &str) -> Option<crate::parts::PartRef>
  where
    Self: Sized,
  {
    self.get_part_by_id(relationship_id)
  }

  #[inline]
  fn get_parts_of_type<T: SdkPart>(&self) -> impl Iterator<Item = T> + '_
  where
    Self: Sized,
  {
    let storage = crate::sdk::SdkPackageInternal::storage(self);
    crate::sdk::SdkPackageInternal::relationships(self)
      .part_relationships()
      .filter_map(move |relationship| relationship_target_as_part::<T>(storage, relationship))
  }

  #[inline]
  fn get_id_of_part<T: SdkPart>(&self, part: &T) -> Option<&str> {
    let target_part_id = part.part_id();
    crate::sdk::SdkPackageInternal::relationships(self)
      .iter()
      .find_map(|relationship| {
        (relationship.target_part_id() == Some(target_part_id)).then_some(relationship.id())
      })
  }

  #[inline]
  fn get_id_of_part_required<T: SdkPart>(&self, part: &T) -> Result<&str, crate::common::SdkError> {
    self.get_id_of_part(part).ok_or_else(|| {
      crate::common::SdkError::CommonError(format!(
        "part id {:?} is not referenced by this package",
        part.part_id()
      ))
    })
  }

  #[inline]
  fn change_id_of_part<T: SdkPart>(
    &mut self,
    part: &T,
    new_relationship_id: impl Into<String>,
  ) -> Result<String, crate::common::SdkError> {
    let old_relationship_id = self.get_id_of_part_required(part)?.to_string();
    self.change_relationship_id(&old_relationship_id, new_relationship_id)?;
    Ok(old_relationship_id)
  }

  #[inline]
  fn delete_part_by_id(&mut self, relationship_id: &str) -> Result<bool, crate::common::SdkError> {
    let deleted =
      crate::sdk::SdkPackageInternal::storage_mut(self).delete_package_part(relationship_id)?;
    if deleted {
      crate::sdk::SdkPackageInternal::refresh_relationship_model_from_storage(self);
    }
    Ok(deleted)
  }

  #[inline]
  fn delete_part<T: SdkPart>(&mut self, part: T) -> Result<bool, crate::common::SdkError> {
    let Some(relationship_id) = self.get_id_of_part(&part).map(str::to_string) else {
      return Ok(false);
    };
    self.delete_part_by_id(&relationship_id)
  }

  #[inline]
  fn delete_parts<T, I>(&mut self, parts: I) -> Result<(), crate::common::SdkError>
  where
    T: SdkPart,
    I: IntoIterator<Item = T>,
  {
    let relationship_ids: Vec<_> = parts
      .into_iter()
      .filter_map(|part| self.get_id_of_part(&part).map(str::to_string))
      .collect();
    for relationship_id in relationship_ids {
      self.delete_part_by_id(&relationship_id)?;
    }
    Ok(())
  }

  #[inline]
  fn add_part<T: SdkPart>(&mut self, part: T) -> Result<T, crate::common::SdkError> {
    if self.get_id_of_part(&part).is_some() {
      return Ok(part);
    }
    let relationship_id =
      crate::sdk::SdkPackageInternal::relationships(self).next_relationship_id();
    self.add_part_with_id(part, relationship_id)
  }

  #[inline]
  fn add_part_with_id<T: SdkPart>(
    &mut self,
    part: T,
    relationship_id: impl Into<String>,
  ) -> Result<T, crate::common::SdkError> {
    let relationship_id = relationship_id.into();
    let part_id = part.part_id();
    crate::sdk::SdkPackageInternal::storage_mut(self)
      .add_package_relationship_to_part(relationship_id.clone(), part_id)?;
    crate::sdk::SdkPackageInternal::refresh_relationship_model_from_storage(self);
    Ok(T::from_relationship_id(relationship_id, part_id))
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
    let relationship_id = part
      .relationship_id()
      .map(str::to_string)
      .unwrap_or_else(|| {
        crate::sdk::SdkPackageInternal::relationships(self).next_relationship_id()
      });
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
    if crate::sdk::SdkPackageInternal::storage(self).id()
      == crate::sdk::SdkPackageInternal::storage(source_package).id()
    {
      return self.add_part_with_id(part.clone(), relationship_id);
    }

    let (imported_part_id, added_count) = crate::sdk::SdkPackageInternal::storage_mut(self)
      .import_part_tree_from(
        crate::sdk::SdkPackageInternal::storage(source_package),
        part.part_id(),
        None,
        relationship_id.clone(),
        |part_id, _| crate::sdk::SdkPackageInternal::part_bytes_for_copy(source_package, part_id),
      )?;
    for _ in 0..added_count {
      crate::sdk::SdkPackageInternal::push_root_element_slot(self);
    }
    crate::sdk::SdkPackageInternal::refresh_relationship_model_from_storage(self);
    Ok(T::from_relationship_id(relationship_id, imported_part_id))
  }

  #[inline]
  fn create_relationship_to_part<T: SdkPart>(
    &mut self,
    part: T,
  ) -> Result<String, crate::common::SdkError> {
    if let Some(relationship_id) = self.get_id_of_part(&part) {
      return Ok(relationship_id.to_string());
    }
    let relationship_id =
      crate::sdk::SdkPackageInternal::relationships(self).next_relationship_id();
    self.create_relationship_to_part_with_id(part, relationship_id)
  }

  #[inline]
  fn create_relationship_to_part_with_id<T: SdkPart>(
    &mut self,
    part: T,
    relationship_id: impl Into<String>,
  ) -> Result<String, crate::common::SdkError> {
    let relationship_id = relationship_id.into();
    crate::sdk::SdkPackageInternal::storage_mut(self)
      .add_package_relationship_to_part(relationship_id.clone(), part.part_id())?;
    crate::sdk::SdkPackageInternal::refresh_relationship_model_from_storage(self);
    Ok(relationship_id)
  }

  #[inline]
  fn create_media_data_part(
    &mut self,
    content_type: impl Into<std::borrow::Cow<'static, str>>,
    extension: impl AsRef<str>,
  ) -> Result<crate::common::MediaDataPart, crate::common::SdkError> {
    let part_id = crate::sdk::SdkPackageInternal::storage_mut(self)
      .create_media_data_part(content_type.into().into_owned(), extension)?;
    let path = crate::sdk::SdkPackageInternal::storage(self)
      .part(part_id)
      .ok_or_else(|| {
        crate::common::SdkError::CommonError(format!(
          "part id {part_id:?} is not present in package storage"
        ))
      })?
      .path()
      .to_string();
    Ok(crate::common::MediaDataPart::from_part_id(
      crate::sdk::SdkPackageInternal::storage(self).id(),
      part_id,
      path,
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
    let relationship_id =
      crate::sdk::SdkPackageInternal::relationships(self).next_relationship_id();
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
    let relationship_id =
      crate::sdk::SdkPackageInternal::relationships(self).next_relationship_id();
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
    let part_id = crate::sdk::SdkPackageInternal::storage_mut(self).add_package_part(
      relationship_id.clone(),
      crate::common::NewPartDescriptor {
        relationship_type: std::borrow::Cow::Borrowed(T::RELATIONSHIP_TYPE),
        content_type: default_new_package_part_content_type::<T>(),
        path_prefix: T::PATH_PREFIX,
        target_name: T::TARGET_NAME,
        extension: std::borrow::Cow::Borrowed(T::EXTENSION),
      },
      target_mode,
    )?;
    crate::sdk::SdkPackageInternal::push_root_element_slot(self);
    crate::sdk::SdkPackageInternal::refresh_relationship_model_from_storage(self);
    Ok(T::from_relationship_id(relationship_id, part_id))
  }

  #[inline]
  fn add_core_file_properties_part(
    &mut self,
  ) -> Result<
    crate::parts::core_file_properties_part::CoreFilePropertiesPart,
    crate::common::SdkError,
  > {
    let relationship_id =
      crate::sdk::SdkPackageInternal::relationships(self).next_relationship_id();
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
    let relationship_id =
      crate::sdk::SdkPackageInternal::relationships(self).next_relationship_id();
    self.add_new_part_with_target_mode::<
      crate::parts::extended_file_properties_part::ExtendedFilePropertiesPart,
    >(relationship_id, crate::common::NewPartTargetMode::Fixed)
  }

  #[inline]
  fn add_custom_file_properties_part(
    &mut self,
  ) -> Result<
    crate::parts::custom_file_properties_part::CustomFilePropertiesPart,
    crate::common::SdkError,
  > {
    let relationship_id =
      crate::sdk::SdkPackageInternal::relationships(self).next_relationship_id();
    self.add_new_part_with_target_mode::<
      crate::parts::custom_file_properties_part::CustomFilePropertiesPart,
    >(relationship_id, crate::common::NewPartTargetMode::Fixed)
  }

  #[inline]
  fn add_digital_signature_origin_part(
    &mut self,
  ) -> Result<
    crate::parts::digital_signature_origin_part::DigitalSignatureOriginPart,
    crate::common::SdkError,
  > {
    let relationship_id =
      crate::sdk::SdkPackageInternal::relationships(self).next_relationship_id();
    self.add_new_part_with_target_mode::<
      crate::parts::digital_signature_origin_part::DigitalSignatureOriginPart,
    >(relationship_id, crate::common::NewPartTargetMode::Fixed)
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
    let part_id = crate::sdk::SdkPackageInternal::storage_mut(self).add_package_part(
      relationship_id.clone(),
      crate::common::NewPartDescriptor {
        relationship_type: std::borrow::Cow::Borrowed(T::RELATIONSHIP_TYPE),
        content_type: content_type.into(),
        path_prefix: T::PATH_PREFIX,
        target_name: T::TARGET_NAME,
        extension: extension.into(),
      },
      target_mode,
    )?;
    crate::sdk::SdkPackageInternal::push_root_element_slot(self);
    crate::sdk::SdkPackageInternal::refresh_relationship_model_from_storage(self);
    Ok(T::from_relationship_id(relationship_id, part_id))
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
    let relationship_id =
      crate::sdk::SdkPackageInternal::relationships(self).next_relationship_id();
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
    let relationship_id =
      crate::sdk::SdkPackageInternal::relationships(self).next_relationship_id();
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
    let part_id = crate::sdk::SdkPackageInternal::storage_mut(self).add_package_part(
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
    crate::sdk::SdkPackageInternal::push_root_element_slot(self);
    crate::sdk::SdkPackageInternal::refresh_relationship_model_from_storage(self);
    Ok(crate::parts::extended_part::ExtendedPart::from_relationship_id(relationship_id, part_id))
  }
}

#[cfg(feature = "parts")]
#[allow(private_bounds)]
pub trait SdkPart: SdkPartInternal + Clone + Sized + 'static {
  const RELATIONSHIP_TYPE: &'static str;
  const PATH_PREFIX: &'static str;
  const CONTENT_TYPE: &'static str;
  const TARGET_NAME: &'static str;
  const EXTENSION: &'static str;

  fn from_part_id(part_id: crate::common::PartId) -> Self;

  fn from_relationship_id(
    relationship_id: impl Into<String>,
    part_id: crate::common::PartId,
  ) -> Self;

  fn set_relationship_id(&mut self, relationship_id: String);

  fn part_id(&self) -> crate::common::PartId;

  fn relationship_id(&self) -> Option<&str>;

  #[inline(always)]
  fn relationship_type() -> &'static str {
    Self::RELATIONSHIP_TYPE
  }

  #[inline(always)]
  fn path_prefix() -> &'static str {
    Self::PATH_PREFIX
  }

  #[inline(always)]
  fn static_content_type() -> &'static str {
    Self::CONTENT_TYPE
  }

  #[inline(always)]
  fn target_name() -> &'static str {
    Self::TARGET_NAME
  }

  #[inline(always)]
  fn extension() -> &'static str {
    Self::EXTENSION
  }

  #[inline]
  fn child_part_by_relationship_type<P, T>(&self, package: &P, relationship_type: &str) -> Option<T>
  where
    P: SdkPackage,
    T: SdkPart,
  {
    <Self as crate::sdk::SdkPartInternal>::relationships(self, package)
      .into_iter()
      .flat_map(|relationships| relationships.iter())
      .find_map(|relationship| {
        crate::common::relationship_type_matches(
          relationship.relationship_type(),
          relationship_type,
        )
        .then(|| relationship.target_part_id())
        .flatten()
        .map(|part_id| T::from_relationship_id(relationship.id(), part_id))
      })
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
    <Self as crate::sdk::SdkPartInternal>::relationships(self, package)
      .into_iter()
      .flat_map(|relationships| relationships.iter())
      .filter_map(move |relationship| {
        crate::common::relationship_type_matches(
          relationship.relationship_type(),
          relationship_type,
        )
        .then(|| relationship.target_part_id())
        .flatten()
        .map(|part_id| T::from_relationship_id(relationship.id(), part_id))
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
    self
      .relationships_mut(package)
      .ok_or_else(|| {
        crate::common::SdkError::CommonError(format!(
          "part id {:?} is not present in package storage",
          self.part_id()
        ))
      })?
      .add_external_relationship(relationship_id.clone(), relationship_type, target)?;
    crate::sdk::SdkPackageInternal::refresh_relationship_model_from_storage(package);
    Ok(
      crate::sdk::SdkPackageInternal::storage(package)
        .relationships(self.part_id())
        .and_then(|relationships| relationships.get(&relationship_id))
        .expect("relationship was just added")
        .into(),
    )
  }

  #[inline]
  fn add_external_relationship_auto_id<'a, P: SdkPackage>(
    &self,
    package: &'a mut P,
    relationship_type: impl Into<String>,
    target: impl Into<String>,
  ) -> Result<crate::common::RelationshipRef<'a>, crate::common::SdkError> {
    let relationship_id = self
      .relationships(package)
      .ok_or_else(|| {
        crate::common::SdkError::CommonError(format!(
          "part id {:?} is not present in package storage",
          self.part_id()
        ))
      })?
      .next_relationship_id();
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
    self
      .relationships_mut(package)
      .ok_or_else(|| {
        crate::common::SdkError::CommonError(format!(
          "part id {:?} is not present in package storage",
          self.part_id()
        ))
      })?
      .add_hyperlink_relationship(relationship_id.clone(), target)?;
    crate::sdk::SdkPackageInternal::refresh_relationship_model_from_storage(package);
    Ok(
      crate::sdk::SdkPackageInternal::storage(package)
        .relationships(self.part_id())
        .and_then(|relationships| relationships.get(&relationship_id))
        .expect("relationship was just added")
        .into(),
    )
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
    self
      .relationships_mut(package)
      .ok_or_else(|| {
        crate::common::SdkError::CommonError(format!(
          "part id {:?} is not present in package storage",
          self.part_id()
        ))
      })?
      .add_hyperlink_relationship_with_mode(relationship_id.clone(), target, target_mode)?;
    crate::sdk::SdkPackageInternal::refresh_relationship_model_from_storage(package);
    Ok(
      crate::sdk::SdkPackageInternal::storage(package)
        .relationships(self.part_id())
        .and_then(|relationships| relationships.get(&relationship_id))
        .expect("relationship was just added")
        .into(),
    )
  }

  #[inline]
  fn add_hyperlink_relationship_auto_id<'a, P: SdkPackage>(
    &self,
    package: &'a mut P,
    target: impl Into<String>,
    target_mode: crate::schemas::opc_relationships::TargetMode,
  ) -> Result<crate::common::RelationshipRef<'a>, crate::common::SdkError> {
    let relationship_id = self
      .relationships(package)
      .ok_or_else(|| {
        crate::common::SdkError::CommonError(format!(
          "part id {:?} is not present in package storage",
          self.part_id()
        ))
      })?
      .next_relationship_id();
    self.add_hyperlink_relationship_with_mode(package, relationship_id, target, target_mode)
  }

  #[inline]
  fn add_audio_reference_relationship<P: SdkPackage>(
    &self,
    package: &mut P,
    media_data_part: &crate::common::MediaDataPart,
  ) -> Result<String, crate::common::SdkError> {
    let relationship_id = self
      .relationships(package)
      .ok_or_else(|| {
        crate::common::SdkError::CommonError(format!(
          "part id {:?} is not present in package storage",
          self.part_id()
        ))
      })?
      .next_relationship_id();
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
    let relationship_id = self
      .relationships(package)
      .ok_or_else(|| {
        crate::common::SdkError::CommonError(format!(
          "part id {:?} is not present in package storage",
          self.part_id()
        ))
      })?
      .next_relationship_id();
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
    let relationship_id = self
      .relationships(package)
      .ok_or_else(|| {
        crate::common::SdkError::CommonError(format!(
          "part id {:?} is not present in package storage",
          self.part_id()
        ))
      })?
      .next_relationship_id();
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
    if !relationship.is_reference_relationship()
      || !matches!(
        relationship.relationship_type(),
        crate::common::RelationshipSet::AUDIO_REFERENCE_RELATIONSHIP_TYPE
          | crate::common::RelationshipSet::MEDIA_REFERENCE_RELATIONSHIP_TYPE
          | crate::common::RelationshipSet::VIDEO_REFERENCE_RELATIONSHIP_TYPE
      )
    {
      return Err(crate::common::SdkError::CommonError(format!(
        "relationship id {} is not a data part reference relationship",
        relationship.id()
      )));
    }
    let target_part_id = relationship.target_part_id().ok_or_else(|| {
      crate::common::SdkError::CommonError(format!(
        "data part reference relationship id {} does not target a package part",
        relationship.id()
      ))
    })?;
    let relationship_id = crate::sdk::SdkPackageInternal::storage_mut(package)
      .add_data_part_reference_relationship(
        self.part_id(),
        relationship.id(),
        relationship.relationship_type(),
        target_part_id,
      )?;
    crate::sdk::SdkPackageInternal::refresh_relationship_model_from_storage(package);
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
    let target_part_id = media_data_part.part_id_for_package(package)?;
    let relationship_id = crate::sdk::SdkPackageInternal::storage_mut(package)
      .add_data_part_reference_relationship(
        self.part_id(),
        relationship_id,
        relationship_type,
        target_part_id,
      )?;
    crate::sdk::SdkPackageInternal::refresh_relationship_model_from_storage(package);
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
    let part_id = crate::sdk::SdkPackageInternal::storage_mut(package).add_child_part(
      self.part_id(),
      relationship_id.clone(),
      crate::common::NewPartDescriptor {
        relationship_type: std::borrow::Cow::Borrowed(T::RELATIONSHIP_TYPE),
        content_type: std::borrow::Cow::Borrowed(T::CONTENT_TYPE),
        path_prefix: T::PATH_PREFIX,
        target_name: T::TARGET_NAME,
        extension: std::borrow::Cow::Borrowed(T::EXTENSION),
      },
    )?;
    crate::sdk::SdkPackageInternal::push_root_element_slot(package);
    crate::sdk::SdkPackageInternal::refresh_relationship_model_from_storage(package);
    Ok(T::from_relationship_id(relationship_id, part_id))
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
    let part_id = crate::sdk::SdkPackageInternal::storage_mut(package).add_child_part(
      self.part_id(),
      relationship_id.clone(),
      crate::common::NewPartDescriptor {
        relationship_type: std::borrow::Cow::Borrowed(T::RELATIONSHIP_TYPE),
        content_type: content_type.into(),
        path_prefix: T::PATH_PREFIX,
        target_name: T::TARGET_NAME,
        extension: std::borrow::Cow::Borrowed(T::EXTENSION),
      },
    )?;
    crate::sdk::SdkPackageInternal::push_root_element_slot(package);
    crate::sdk::SdkPackageInternal::refresh_relationship_model_from_storage(package);
    Ok(T::from_relationship_id(relationship_id, part_id))
  }

  #[inline]
  fn add_new_part_auto_id<P, T>(&self, package: &mut P) -> Result<T, crate::common::SdkError>
  where
    P: SdkPackage,
    T: SdkPart,
  {
    let relationship_id = self
      .relationships(package)
      .ok_or_else(|| {
        crate::common::SdkError::CommonError(format!(
          "part id {:?} is not present in package storage",
          self.part_id()
        ))
      })?
      .next_relationship_id();
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
    let relationship_id = self
      .relationships(package)
      .ok_or_else(|| {
        crate::common::SdkError::CommonError(format!(
          "part id {:?} is not present in package storage",
          self.part_id()
        ))
      })?
      .next_relationship_id();
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
    let part_id = crate::sdk::SdkPackageInternal::storage_mut(package).add_child_part(
      self.part_id(),
      relationship_id.clone(),
      crate::common::NewPartDescriptor {
        relationship_type: std::borrow::Cow::Borrowed(T::RELATIONSHIP_TYPE),
        content_type: content_type.into(),
        path_prefix: T::PATH_PREFIX,
        target_name: T::TARGET_NAME,
        extension: extension.into(),
      },
    )?;
    crate::sdk::SdkPackageInternal::push_root_element_slot(package);
    crate::sdk::SdkPackageInternal::refresh_relationship_model_from_storage(package);
    Ok(T::from_relationship_id(relationship_id, part_id))
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
    let relationship_id = self
      .relationships(package)
      .ok_or_else(|| {
        crate::common::SdkError::CommonError(format!(
          "part id {:?} is not present in package storage",
          self.part_id()
        ))
      })?
      .next_relationship_id();
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
    let relationship_id = self
      .relationships(package)
      .ok_or_else(|| {
        crate::common::SdkError::CommonError(format!(
          "part id {:?} is not present in package storage",
          self.part_id()
        ))
      })?
      .next_relationship_id();
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
    let part_id = crate::sdk::SdkPackageInternal::storage_mut(package).add_child_part(
      self.part_id(),
      relationship_id.clone(),
      crate::common::NewPartDescriptor {
        relationship_type: std::borrow::Cow::Owned(relationship_type.into()),
        content_type: content_type.into(),
        path_prefix: ".",
        target_name: "extendedPart",
        extension: target_extension.into(),
      },
    )?;
    crate::sdk::SdkPackageInternal::push_root_element_slot(package);
    crate::sdk::SdkPackageInternal::refresh_relationship_model_from_storage(package);
    Ok(crate::parts::extended_part::ExtendedPart::from_relationship_id(relationship_id, part_id))
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
    self.add_new_part_with_content_type_auto_id::<P, crate::parts::image_part::ImagePart>(
      package,
      content_type,
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
    self.add_new_part_with_content_type::<P, crate::parts::image_part::ImagePart>(
      package,
      relationship_id,
      content_type,
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
    self.add_new_part_with_content_type::<
      P,
      crate::parts::alternative_format_import_part::AlternativeFormatImportPart,
    >(package, relationship_id, content_type)
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
    self.add_new_part_with_content_type_auto_id::<
      P,
      crate::parts::custom_property_part::CustomPropertyPart,
    >(package, content_type)
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
    self.add_new_part_with_content_type_and_extension::<
      P,
      crate::parts::custom_property_part::CustomPropertyPart,
    >(
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
    self.add_new_part_with_content_type_auto_id::<
      P,
      crate::parts::embedded_object_part::EmbeddedObjectPart,
    >(package, content_type)
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
    self.add_new_part_with_content_type_and_extension::<
      P,
      crate::parts::embedded_object_part::EmbeddedObjectPart,
    >(
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
    self.add_new_part_with_content_type_auto_id::<
      P,
      crate::parts::embedded_package_part::EmbeddedPackagePart,
    >(package, content_type)
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
    self.add_new_part_with_content_type::<
      P,
      crate::parts::embedded_package_part::EmbeddedPackagePart,
    >(package, relationship_id, content_type)
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
    self.add_new_part_with_content_type_and_extension::<
      P,
      crate::parts::embedded_package_part::EmbeddedPackagePart,
    >(
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
    self.add_new_part_with_content_type::<
      P,
      crate::parts::mail_merge_recipient_data_part::MailMergeRecipientDataPart,
    >(package, relationship_id, content_type)
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
    self
      .relationships(package)?
      .get(relationship_id)
      .filter(|relationship| relationship.is_reference_relationship())
      .map(Into::into)
  }

  #[inline]
  fn get_external_relationship<'a, P: SdkPackage>(
    &'a self,
    package: &'a P,
    relationship_id: &str,
  ) -> Option<crate::common::RelationshipRef<'a>> {
    self
      .relationships(package)?
      .get_external_relationship(relationship_id)
      .map(Into::into)
  }

  #[inline]
  fn get_hyperlink_relationship<'a, P: SdkPackage>(
    &'a self,
    package: &'a P,
    relationship_id: &str,
  ) -> Option<crate::common::RelationshipRef<'a>> {
    self
      .relationships(package)?
      .get_hyperlink_relationship(relationship_id)
      .map(Into::into)
  }

  #[inline]
  fn delete_reference_relationship<P: SdkPackage>(
    &self,
    package: &mut P,
    relationship_id: &str,
  ) -> Result<crate::common::Relationship, crate::common::SdkError> {
    let relationship = self
      .relationships_mut(package)
      .ok_or_else(|| {
        crate::common::SdkError::CommonError(format!(
          "part id {:?} is not present in package storage",
          self.part_id()
        ))
      })?
      .remove_reference_relationship(relationship_id)?;
    crate::sdk::SdkPackageInternal::refresh_relationship_model_from_storage(package);
    Ok(relationship.into())
  }

  #[inline]
  fn delete_external_relationship<P: SdkPackage>(
    &self,
    package: &mut P,
    relationship_id: &str,
  ) -> Result<crate::common::Relationship, crate::common::SdkError> {
    let relationship = self
      .relationships_mut(package)
      .ok_or_else(|| {
        crate::common::SdkError::CommonError(format!(
          "part id {:?} is not present in package storage",
          self.part_id()
        ))
      })?
      .remove_external_relationship(relationship_id)?;
    crate::sdk::SdkPackageInternal::refresh_relationship_model_from_storage(package);
    Ok(relationship.into())
  }

  #[inline]
  fn change_relationship_id<P: SdkPackage>(
    &self,
    package: &mut P,
    relationship_id: &str,
    new_relationship_id: impl Into<String>,
  ) -> Result<(), crate::common::SdkError> {
    self
      .relationships_mut(package)
      .ok_or_else(|| {
        crate::common::SdkError::CommonError(format!(
          "part id {:?} is not present in package storage",
          self.part_id()
        ))
      })?
      .change_relationship_id(relationship_id, new_relationship_id)?;
    crate::sdk::SdkPackageInternal::refresh_relationship_model_from_storage(package);
    Ok(())
  }

  #[inline]
  fn external_relationships<'a, P: SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::common::RelationshipRef<'a>> {
    self
      .relationships(package)
      .into_iter()
      .flat_map(crate::common::RelationshipSet::external_relationships)
      .map(Into::into)
  }

  #[inline]
  fn hyperlink_relationships<'a, P: SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::common::RelationshipRef<'a>> {
    self
      .relationships(package)
      .into_iter()
      .flat_map(crate::common::RelationshipSet::hyperlink_relationships)
      .map(Into::into)
  }

  #[inline]
  fn data_part_reference_relationships<'a, P: SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::common::RelationshipRef<'a>> {
    self
      .relationships(package)
      .into_iter()
      .flat_map(crate::common::RelationshipSet::data_part_reference_relationships)
      .map(Into::into)
  }

  #[inline]
  fn path<'a, P: SdkPackage>(&self, package: &'a P) -> Option<&'a str> {
    crate::sdk::SdkPackageInternal::storage(package)
      .part(self.part_id())
      .map(crate::common::StoredPart::path)
  }

  #[inline]
  fn content_type<'a, P: SdkPackage>(&self, package: &'a P) -> Option<&'a str> {
    crate::sdk::SdkPackageInternal::storage(package)
      .part(self.part_id())
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
    crate::sdk::SdkPackageInternal::storage(package)
      .part(self.part_id())
      .map(|part| part.data().bytes())
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
      .data(package)
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
    let Some(data) = self.data(package) else {
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
    crate::sdk::SdkPackageInternal::storage_mut(package).set_part_data(self.part_id(), data)
  }

  #[inline]
  fn feed_data<P: SdkPackage, R: std::io::Read>(
    &self,
    package: &mut P,
    reader: &mut R,
  ) -> Result<(), crate::common::SdkError> {
    crate::sdk::SdkPackageInternal::storage_mut(package).feed_part_data(self.part_id(), reader)
  }

  #[inline]
  fn target_part_id<P: SdkPackage>(
    &self,
    package: &P,
    relationship_id: &str,
  ) -> Option<crate::common::PartId> {
    crate::sdk::SdkPackageInternal::storage(package).target_part_id(self.part_id(), relationship_id)
  }

  #[inline]
  fn parts<'a, P: SdkPackage + Sized>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::parts::IdPartPair<'a>> + 'a {
    self
      .relationships(package)
      .into_iter()
      .flat_map(|relationships| relationships.iter())
      .filter_map(move |relationship| {
        let part_id = relationship.target_part_id()?;
        let part = crate::parts::PartRef::from_part_id(package, part_id)?;
        Some(crate::parts::IdPartPair::new(relationship.id(), part))
      })
  }

  #[inline]
  fn get_all_parts<'a, P: SdkPackage + Sized>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::parts::PartRef> + 'a {
    let Some(relationships) = <Self as crate::sdk::SdkPartInternal>::relationships(self, package)
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
    let target_part_id = self.part_id();
    package
      .get_all_parts()
      .filter(move |part| {
        crate::sdk::SdkPackageInternal::storage(package)
          .relationships(part.part_id())
          .is_some_and(|relationships| {
            relationships
              .part_relationships()
              .any(|relationship| relationship.target_part_id() == Some(target_part_id))
          })
      })
      .collect::<Vec<_>>()
      .into_iter()
  }

  #[inline]
  fn get_part_by_id<P: SdkPackage + Sized>(
    &self,
    package: &P,
    relationship_id: &str,
  ) -> Option<crate::parts::PartRef> {
    let part_id = self.target_part_id(package, relationship_id)?;
    crate::parts::PartRef::from_part_id(package, part_id)
  }

  #[inline]
  fn get_part_by_id_required<P: SdkPackage + Sized>(
    &self,
    package: &P,
    relationship_id: &str,
  ) -> Result<crate::parts::PartRef, crate::common::SdkError> {
    self
      .get_part_by_id(package, relationship_id)
      .ok_or_else(|| {
        crate::common::SdkError::CommonError(format!(
          "part relationship id {relationship_id} does not exist"
        ))
      })
  }

  #[inline]
  fn try_get_part_by_id<P: SdkPackage + Sized>(
    &self,
    package: &P,
    relationship_id: &str,
  ) -> Option<crate::parts::PartRef> {
    self.get_part_by_id(package, relationship_id)
  }

  #[inline]
  fn get_parts_of_type<'a, P: SdkPackage + Sized, T: SdkPart>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = T> + 'a {
    let storage = crate::sdk::SdkPackageInternal::storage(package);
    self
      .relationships(package)
      .into_iter()
      .flat_map(|relationships| relationships.part_relationships())
      .filter_map(move |relationship| relationship_target_as_part::<T>(storage, relationship))
  }

  #[inline]
  fn get_id_of_part<'a, P: SdkPackage, T: SdkPart>(
    &'a self,
    package: &'a P,
    part: &T,
  ) -> Option<&'a str> {
    let target_part_id = part.part_id();
    self
      .relationships(package)?
      .iter()
      .find_map(|relationship| {
        (relationship.target_part_id() == Some(target_part_id)).then_some(relationship.id())
      })
  }

  #[inline]
  fn get_id_of_part_required<'a, P: SdkPackage, T: SdkPart>(
    &'a self,
    package: &'a P,
    part: &T,
  ) -> Result<&'a str, crate::common::SdkError> {
    self.get_id_of_part(package, part).ok_or_else(|| {
      crate::common::SdkError::CommonError(format!(
        "part id {:?} is not referenced by part id {:?}",
        part.part_id(),
        self.part_id()
      ))
    })
  }

  #[inline]
  fn change_id_of_part<P: SdkPackage, T: SdkPart>(
    &self,
    package: &mut P,
    part: &T,
    new_relationship_id: impl Into<String>,
  ) -> Result<String, crate::common::SdkError> {
    let old_relationship_id = self.get_id_of_part_required(package, part)?.to_string();
    self.change_relationship_id(package, &old_relationship_id, new_relationship_id)?;
    Ok(old_relationship_id)
  }

  #[inline]
  fn delete_part_by_id<P: SdkPackage>(
    &self,
    package: &mut P,
    relationship_id: &str,
  ) -> Result<bool, crate::common::SdkError> {
    let deleted = crate::sdk::SdkPackageInternal::storage_mut(package)
      .delete_child_part(self.part_id(), relationship_id)?;
    if deleted {
      crate::sdk::SdkPackageInternal::refresh_relationship_model_from_storage(package);
    }
    Ok(deleted)
  }

  #[inline]
  fn delete_part<P: SdkPackage, T: SdkPart>(
    &self,
    package: &mut P,
    part: T,
  ) -> Result<bool, crate::common::SdkError> {
    let Some(relationship_id) = self.get_id_of_part(package, &part).map(str::to_string) else {
      return Ok(false);
    };
    self.delete_part_by_id(package, &relationship_id)
  }

  #[inline]
  fn delete_parts<P, T, I>(&self, package: &mut P, parts: I) -> Result<(), crate::common::SdkError>
  where
    P: SdkPackage,
    T: SdkPart,
    I: IntoIterator<Item = T>,
  {
    let relationship_ids: Vec<_> = parts
      .into_iter()
      .filter_map(|part| self.get_id_of_part(package, &part).map(str::to_string))
      .collect();
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
    if self.get_id_of_part(package, &part).is_some() {
      return Ok(part);
    }
    let relationship_id = self
      .relationships(package)
      .ok_or_else(|| {
        crate::common::SdkError::CommonError(format!(
          "part id {:?} is not present in package storage",
          self.part_id()
        ))
      })?
      .next_relationship_id();
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
    let part_id = part.part_id();
    crate::sdk::SdkPackageInternal::storage_mut(package).add_child_relationship_to_part(
      self.part_id(),
      relationship_id.clone(),
      part_id,
    )?;
    crate::sdk::SdkPackageInternal::refresh_relationship_model_from_storage(package);
    Ok(T::from_relationship_id(relationship_id, part_id))
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
    let relationship_id = part
      .relationship_id()
      .map(str::to_string)
      .unwrap_or_else(|| {
        self
          .relationships(package)
          .map(crate::common::RelationshipSet::next_relationship_id)
          .unwrap_or_else(|| "rId1".to_string())
      });
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
    if crate::sdk::SdkPackageInternal::storage(package).id()
      == crate::sdk::SdkPackageInternal::storage(source_package).id()
    {
      return self.add_part_with_id(package, part.clone(), relationship_id);
    }

    let (imported_part_id, added_count) = crate::sdk::SdkPackageInternal::storage_mut(package)
      .import_part_tree_from(
        crate::sdk::SdkPackageInternal::storage(source_package),
        part.part_id(),
        Some(self.part_id()),
        relationship_id.clone(),
        |part_id, _| crate::sdk::SdkPackageInternal::part_bytes_for_copy(source_package, part_id),
      )?;
    for _ in 0..added_count {
      crate::sdk::SdkPackageInternal::push_root_element_slot(package);
    }
    crate::sdk::SdkPackageInternal::refresh_relationship_model_from_storage(package);
    Ok(T::from_relationship_id(relationship_id, imported_part_id))
  }

  #[inline]
  fn create_relationship_to_part<P: SdkPackage, T: SdkPart>(
    &self,
    package: &mut P,
    part: T,
  ) -> Result<String, crate::common::SdkError> {
    if let Some(relationship_id) = self.get_id_of_part(package, &part) {
      return Ok(relationship_id.to_string());
    }
    let relationship_id = self
      .relationships(package)
      .ok_or_else(|| {
        crate::common::SdkError::CommonError(format!(
          "part id {:?} is not present in package storage",
          self.part_id()
        ))
      })?
      .next_relationship_id();
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
    crate::sdk::SdkPackageInternal::storage_mut(package).add_child_relationship_to_part(
      self.part_id(),
      relationship_id.clone(),
      part.part_id(),
    )?;
    crate::sdk::SdkPackageInternal::refresh_relationship_model_from_storage(package);
    Ok(relationship_id)
  }
}

#[cfg(feature = "parts")]
pub(crate) trait SdkPartInternal: Clone + Sized + 'static {
  fn from_part_id_with_relationships(
    storage: &crate::common::SdkPackageStorage,
    part_id: crate::common::PartId,
  ) -> Self
  where
    Self: SdkPart,
  {
    let _ = storage;
    Self::from_part_id(part_id)
  }

  fn from_relationship_id_with_relationships(
    storage: &crate::common::SdkPackageStorage,
    relationship_id: impl Into<String>,
    part_id: crate::common::PartId,
  ) -> Self
  where
    Self: SdkPart,
  {
    let mut part = Self::from_part_id_with_relationships(storage, part_id);
    part.set_relationship_id(relationship_id.into());
    part
  }

  #[inline]
  fn relationships<'a, P: SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> Option<&'a crate::common::RelationshipSet>
  where
    Self: SdkPart,
  {
    crate::sdk::SdkPackageInternal::storage(package).relationships(self.part_id())
  }

  #[inline]
  fn relationships_mut<'a, P: SdkPackage>(
    &self,
    package: &'a mut P,
  ) -> Option<&'a mut crate::common::RelationshipSet>
  where
    Self: SdkPart,
  {
    crate::sdk::SdkPackageInternal::storage_mut(package).relationships_mut(self.part_id())
  }
}

#[cfg(feature = "parts")]
#[macro_export]
macro_rules! sdk_part_root_methods {
  ($root_ty:ty, $part_variant:ident, $root_accessor:ident, $root_accessor_mut:ident) => {
    #[inline]
    pub fn is_root_element_loaded<P: $crate::sdk::SdkPackage>(&self, package: &P) -> bool {
      $crate::sdk::SdkPackageInternal::is_root_element_loaded(package, self.id)
    }

    #[inline]
    pub fn unload_root_element<P: $crate::sdk::SdkPackage>(
      &self,
      package: &mut P,
    ) -> Option<$crate::parts::PartRootElement> {
      $crate::sdk::SdkPackageInternal::unload_root_element(package, self.id)
    }

    pub fn root_element<'a, P: $crate::sdk::SdkPackage>(
      &self,
      package: &'a mut P,
    ) -> Result<&'a $root_ty, $crate::common::SdkError> {
      if $crate::sdk::SdkPackageInternal::root_element(package, self.id)
        .and_then($crate::parts::PartRootElement::$root_accessor)
        .is_none()
      {
        let root_element = {
          let part = $crate::sdk::SdkPackageInternal::storage(package)
            .part(self.id)
            .ok_or_else(|| {
              $crate::common::SdkError::CommonError(format!(
                "part id {:?} is not present in package storage",
                self.id,
              ))
            })?;
          <$root_ty>::from_bytes(part.data().bytes())?
        };

        *$crate::sdk::SdkPackageInternal::root_element_slot_mut(package, self.id).ok_or_else(
          || {
            $crate::common::SdkError::CommonError(format!(
              "part id {:?} is not present in package root cache",
              self.id,
            ))
          },
        )? = Some($crate::parts::PartRootElement::$part_variant(Box::new(
          root_element,
        )));
      }

      $crate::sdk::SdkPackageInternal::root_element(package, self.id)
        .and_then($crate::parts::PartRootElement::$root_accessor)
        .ok_or_else(|| {
          $crate::common::SdkError::CommonError(
            concat!(
              "cached root element has unexpected type for ",
              stringify!($part_variant)
            )
            .to_string(),
          )
        })
    }

    pub fn root_element_mut<'a, P: $crate::sdk::SdkPackage>(
      &self,
      package: &'a mut P,
    ) -> Result<&'a mut $root_ty, $crate::common::SdkError> {
      if $crate::sdk::SdkPackageInternal::root_element(package, self.id)
        .and_then($crate::parts::PartRootElement::$root_accessor)
        .is_none()
      {
        let root_element = {
          let part = $crate::sdk::SdkPackageInternal::storage(package)
            .part(self.id)
            .ok_or_else(|| {
              $crate::common::SdkError::CommonError(format!(
                "part id {:?} is not present in package storage",
                self.id,
              ))
            })?;
          <$root_ty>::from_bytes(part.data().bytes())?
        };

        *$crate::sdk::SdkPackageInternal::root_element_slot_mut(package, self.id).ok_or_else(
          || {
            $crate::common::SdkError::CommonError(format!(
              "part id {:?} is not present in package root cache",
              self.id,
            ))
          },
        )? = Some($crate::parts::PartRootElement::$part_variant(Box::new(
          root_element,
        )));
      }

      $crate::sdk::SdkPackageInternal::root_element_slot_mut(package, self.id)
        .and_then(Option::as_mut)
        .and_then($crate::parts::PartRootElement::$root_accessor_mut)
        .ok_or_else(|| {
          $crate::common::SdkError::CommonError(
            concat!(
              "cached root element has unexpected type for ",
              stringify!($part_variant)
            )
            .to_string(),
          )
        })
    }

    pub fn set_root_element<P: $crate::sdk::SdkPackage>(
      &self,
      package: &mut P,
      root_element: $root_ty,
    ) -> Result<(), $crate::common::SdkError> {
      *$crate::sdk::SdkPackageInternal::root_element_slot_mut(package, self.id).ok_or_else(
        || {
          $crate::common::SdkError::CommonError(format!(
            "part id {:?} is not present in package root cache",
            self.id,
          ))
        },
      )? = Some($crate::parts::PartRootElement::$part_variant(Box::new(
        root_element,
      )));

      Ok(())
    }
  };
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
