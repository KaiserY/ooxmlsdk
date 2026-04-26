#[cfg(feature = "parts")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PartDescriptor {
  pub relationship_type: &'static str,
  pub path_prefix: &'static str,
  pub content_type: &'static str,
  pub target_name: &'static str,
  pub extension: &'static str,
}

#[cfg(feature = "parts")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PartChildDescriptor {
  pub field_name: &'static str,
  pub relationship_type: &'static str,
  pub child_part_type: &'static str,
  pub cardinality: PartChildCardinality,
}

#[cfg(feature = "parts")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PartChildCardinality {
  Optional,
  Required,
  Repeated,
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
pub fn add_part_handle_to_relationship_set<T: SdkPartHandle>(
  relationships: &mut crate::common::RelationshipSet,
  storage: &crate::common::SdkPackageStorage,
  source_part_id: Option<crate::common::PartId>,
  part: &T,
) -> Result<(), crate::common::SdkError> {
  let Some(relationship_id) = part.relationship_id() else {
    return Ok(());
  };
  let Some(stored_part) = storage.part(part.part_id()) else {
    return Ok(());
  };
  if stored_part.is_deleted() {
    return Ok(());
  }
  let relationship =
    storage.internal_part_relationship_info(source_part_id, relationship_id, part.part_id())?;
  relationships.add_relationship_info(relationship)?;
  Ok(())
}

#[cfg(feature = "parts")]
pub fn add_part_ref_to_relationship_set(
  relationships: &mut crate::common::RelationshipSet,
  storage: &crate::common::SdkPackageStorage,
  source_part_id: Option<crate::common::PartId>,
  part: &crate::parts::PartRef,
) -> Result<(), crate::common::SdkError> {
  let Some(relationship_id) = part.relationship_id() else {
    return Ok(());
  };
  let Some(stored_part) = storage.part(part.part_id()) else {
    return Ok(());
  };
  if stored_part.is_deleted() {
    return Ok(());
  }
  let relationship =
    storage.internal_part_relationship_info(source_part_id, relationship_id, part.part_id())?;
  relationships.add_relationship_info(relationship)?;
  Ok(())
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
fn delete_parts_recursively_from_part_id<P, T>(
  package: &mut P,
  source_part_id: crate::common::PartId,
) -> Result<(), crate::common::SdkError>
where
  P: SdkPackage + Sized,
  T: SdkPartHandle + 'static,
{
  let relationship_ids: Vec<_> = package
    .storage()
    .relationships(source_part_id)
    .into_iter()
    .flat_map(|relationships| relationships.iter())
    .filter_map(|relationship| {
      let part_id = relationship.target_part_id()?;
      let part = crate::parts::PartRef::from_part_id(package, part_id)?;
      part
        .downcast::<T>()
        .is_some()
        .then(|| relationship.id().to_string())
    })
    .collect();

  for relationship_id in relationship_ids {
    package
      .storage_mut()
      .delete_child_part(source_part_id, &relationship_id)?;
  }

  let child_part_ids: Vec<_> = package
    .storage()
    .relationships(source_part_id)
    .into_iter()
    .flat_map(crate::common::RelationshipSet::part_relationships)
    .filter_map(crate::common::RelationshipInfo::target_part_id)
    .collect();

  for child_part_id in child_part_ids {
    delete_parts_recursively_from_part_id::<P, T>(package, child_part_id)?;
  }

  Ok(())
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

    if let Some(relationships) = package.storage().relationships(part_id) {
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
impl PartDescriptor {
  pub const fn new(
    relationship_type: &'static str,
    path_prefix: &'static str,
    content_type: &'static str,
    target_name: &'static str,
    extension: &'static str,
  ) -> Self {
    Self {
      relationship_type,
      path_prefix,
      content_type,
      target_name,
      extension,
    }
  }
}

#[cfg(feature = "parts")]
impl PartChildDescriptor {
  pub const fn new(
    field_name: &'static str,
    relationship_type: &'static str,
    child_part_type: &'static str,
    cardinality: PartChildCardinality,
  ) -> Self {
    Self {
      field_name,
      relationship_type,
      child_part_type,
      cardinality,
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
pub trait SdkPackage {
  const CHILD_DESCRIPTORS: &'static [PartChildDescriptor] = &[];

  fn storage(&self) -> &crate::common::SdkPackageStorage;

  fn storage_mut(&mut self) -> &mut crate::common::SdkPackageStorage;

  fn main_part_id(&self) -> Option<crate::common::PartId>;

  #[inline(always)]
  fn child_descriptors() -> &'static [PartChildDescriptor]
  where
    Self: Sized,
  {
    Self::CHILD_DESCRIPTORS
  }

  #[inline]
  fn relationships(&self) -> &crate::common::RelationshipSet {
    self.storage().package_relationships()
  }

  #[inline]
  fn relationships_mut(&mut self) -> &mut crate::common::RelationshipSet {
    self.storage_mut().package_relationships_mut()
  }

  #[inline]
  fn modeled_relationships(
    &self,
  ) -> Result<crate::common::RelationshipSet, crate::common::SdkError> {
    Ok(self.relationships().clone())
  }

  #[inline]
  fn refresh_relationship_model_from_storage(&mut self) {}

  fn collect_modeled_part_relationships(
    &self,
    relationships: &mut std::collections::HashMap<
      crate::common::PartId,
      crate::common::RelationshipSet,
    >,
  ) -> Result<(), crate::common::SdkError> {
    for (index, part) in self.storage().parts().iter().enumerate() {
      if part.is_deleted() {
        continue;
      }
      let part_id = crate::common::PartId::from_index(index);
      if let Some(part_relationships) = self.storage().relationships(part_id) {
        relationships.insert(part_id, part_relationships.clone());
      }
    }
    Ok(())
  }

  #[inline]
  fn add_external_relationship(
    &mut self,
    relationship_id: impl Into<String>,
    relationship_type: impl Into<String>,
    target: impl Into<String>,
  ) -> Result<&crate::common::RelationshipInfo, crate::common::SdkError> {
    let relationship_id = relationship_id.into();
    self.relationships_mut().add_external_relationship(
      relationship_id.clone(),
      relationship_type,
      target,
    )?;
    self.refresh_relationship_model_from_storage();
    Ok(
      self
        .relationships()
        .get(&relationship_id)
        .expect("relationship was just added"),
    )
  }

  #[inline]
  fn add_external_relationship_auto_id(
    &mut self,
    relationship_type: impl Into<String>,
    target: impl Into<String>,
  ) -> Result<&crate::common::RelationshipInfo, crate::common::SdkError> {
    let relationship_id = self.relationships().next_relationship_id();
    self.add_external_relationship(relationship_id, relationship_type, target)
  }

  #[inline]
  fn add_hyperlink_relationship(
    &mut self,
    relationship_id: impl Into<String>,
    target: impl Into<String>,
  ) -> Result<&crate::common::RelationshipInfo, crate::common::SdkError> {
    let relationship_id = relationship_id.into();
    self
      .relationships_mut()
      .add_hyperlink_relationship(relationship_id.clone(), target)?;
    self.refresh_relationship_model_from_storage();
    Ok(
      self
        .relationships()
        .get(&relationship_id)
        .expect("relationship was just added"),
    )
  }

  #[inline]
  fn add_hyperlink_relationship_with_mode(
    &mut self,
    relationship_id: impl Into<String>,
    target: impl Into<String>,
    target_mode: crate::schemas::opc_relationships::TargetMode,
  ) -> Result<&crate::common::RelationshipInfo, crate::common::SdkError> {
    let relationship_id = relationship_id.into();
    self
      .relationships_mut()
      .add_hyperlink_relationship_with_mode(relationship_id.clone(), target, target_mode)?;
    self.refresh_relationship_model_from_storage();
    Ok(
      self
        .relationships()
        .get(&relationship_id)
        .expect("relationship was just added"),
    )
  }

  #[inline]
  fn add_hyperlink_relationship_auto_id(
    &mut self,
    target: impl Into<String>,
    target_mode: crate::schemas::opc_relationships::TargetMode,
  ) -> Result<&crate::common::RelationshipInfo, crate::common::SdkError> {
    let relationship_id = self.relationships().next_relationship_id();
    self.add_hyperlink_relationship_with_mode(relationship_id, target, target_mode)
  }

  #[inline]
  fn remove_relationship(
    &mut self,
    relationship_id: &str,
  ) -> Option<crate::common::RelationshipInfo> {
    let removed = self.relationships_mut().remove(relationship_id);
    if removed.is_some() {
      self.refresh_relationship_model_from_storage();
    }
    removed
  }

  #[inline]
  fn get_reference_relationship(
    &self,
    relationship_id: &str,
  ) -> Option<&crate::common::RelationshipInfo> {
    self
      .relationships()
      .get(relationship_id)
      .filter(|relationship| relationship.is_reference_relationship())
  }

  #[inline]
  fn get_external_relationship(
    &self,
    relationship_id: &str,
  ) -> Option<&crate::common::RelationshipInfo> {
    self
      .relationships()
      .get_external_relationship(relationship_id)
  }

  #[inline]
  fn get_hyperlink_relationship(
    &self,
    relationship_id: &str,
  ) -> Option<&crate::common::RelationshipInfo> {
    self
      .relationships()
      .get_hyperlink_relationship(relationship_id)
  }

  #[inline]
  fn delete_reference_relationship(
    &mut self,
    relationship_id: &str,
  ) -> Result<crate::common::RelationshipInfo, crate::common::SdkError> {
    let relationship = self
      .relationships_mut()
      .remove_reference_relationship(relationship_id)?;
    self.refresh_relationship_model_from_storage();
    Ok(relationship)
  }

  #[inline]
  fn delete_external_relationship(
    &mut self,
    relationship_id: &str,
  ) -> Result<crate::common::RelationshipInfo, crate::common::SdkError> {
    let relationship = self
      .relationships_mut()
      .remove_external_relationship(relationship_id)?;
    self.refresh_relationship_model_from_storage();
    Ok(relationship)
  }

  #[inline]
  fn change_relationship_id(
    &mut self,
    relationship_id: &str,
    new_relationship_id: impl Into<String>,
  ) -> Result<(), crate::common::SdkError> {
    self
      .relationships_mut()
      .change_relationship_id(relationship_id, new_relationship_id)?;
    self.refresh_relationship_model_from_storage();
    Ok(())
  }

  #[inline]
  fn external_relationships(&self) -> impl Iterator<Item = &crate::common::RelationshipInfo> {
    self.relationships().external_relationships()
  }

  #[inline]
  fn hyperlink_relationships(&self) -> impl Iterator<Item = &crate::common::RelationshipInfo> {
    self.relationships().hyperlink_relationships()
  }

  #[inline]
  fn data_part_reference_relationships(
    &self,
  ) -> impl Iterator<Item = &crate::common::RelationshipInfo> {
    self.relationships().data_part_reference_relationships()
  }

  #[inline]
  fn media_data_parts(&self) -> impl Iterator<Item = crate::common::MediaDataPart> + '_ {
    self.storage().media_data_parts().map(|(part_id, part)| {
      crate::common::MediaDataPart::from_part_id(self.storage().id(), part_id, part.path())
    })
  }

  #[inline]
  fn delete_unused_media_data_parts(&mut self) -> usize {
    self.storage_mut().delete_unused_media_data_parts()
  }

  #[inline]
  fn relationships_by_type(
    &self,
    relationship_type: &str,
  ) -> impl Iterator<Item = &crate::common::RelationshipInfo> {
    self.relationships().by_relationship_type(relationship_type)
  }

  #[inline]
  fn parts(&self) -> impl Iterator<Item = crate::parts::IdPartPair<'_>> + '_
  where
    Self: Sized,
  {
    self.relationships().iter().filter_map(|relationship| {
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
    collect_all_parts_from_relationships(self, self.relationships()).into_iter()
  }

  #[inline]
  fn get_part_by_relationship_type(&self, relationship_type: &str) -> Option<crate::parts::PartRef>
  where
    Self: Sized,
  {
    self
      .relationships_by_type(relationship_type)
      .find_map(|relationship| {
        let part_id = relationship.target_part_id()?;
        crate::parts::PartRef::from_part_id(self, part_id)
      })
  }

  #[inline]
  fn is_child_part<T: SdkPartHandle>(&self, part: &T) -> bool {
    let target_part_id = part.part_id();
    self
      .relationships()
      .part_relationships()
      .any(|relationship| relationship.target_part_id() == Some(target_part_id))
  }

  #[inline]
  fn get_part_by_id(&self, relationship_id: &str) -> Option<crate::parts::PartRef>
  where
    Self: Sized,
  {
    let part_id = self
      .relationships()
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
  fn get_parts_of_type<T: SdkPartHandle + 'static>(&self) -> impl Iterator<Item = T> + '_
  where
    Self: Sized,
  {
    self.parts().filter_map(|entry| entry.part.downcast::<T>())
  }

  #[inline]
  fn get_sub_part_of_type<T: SdkPartHandle + 'static>(&self) -> Option<T>
  where
    Self: Sized,
  {
    self.get_parts_of_type::<T>().next()
  }

  #[inline]
  fn get_id_of_part<T: SdkPartHandle>(&self, part: &T) -> Option<&str> {
    let target_part_id = part.part_id();
    self.relationships().iter().find_map(|relationship| {
      (relationship.target_part_id() == Some(target_part_id)).then_some(relationship.id())
    })
  }

  #[inline]
  fn get_id_of_part_required<T: SdkPartHandle>(
    &self,
    part: &T,
  ) -> Result<&str, crate::common::SdkError> {
    self.get_id_of_part(part).ok_or_else(|| {
      crate::common::SdkError::CommonError(format!(
        "part id {:?} is not referenced by this package",
        part.part_id()
      ))
    })
  }

  #[inline]
  fn change_id_of_part<T: SdkPartHandle>(
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
    let deleted = self.storage_mut().delete_package_part(relationship_id)?;
    if deleted {
      self.refresh_relationship_model_from_storage();
    }
    Ok(deleted)
  }

  #[inline]
  fn delete_part<T: SdkPartHandle>(&mut self, part: T) -> Result<bool, crate::common::SdkError> {
    let Some(relationship_id) = self.get_id_of_part(&part).map(str::to_string) else {
      return Ok(false);
    };
    self.delete_part_by_id(&relationship_id)
  }

  #[inline]
  fn delete_parts<T, I>(&mut self, parts: I) -> Result<(), crate::common::SdkError>
  where
    T: SdkPartHandle,
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
  fn delete_parts_of_type<T: SdkPartHandle + 'static>(
    &mut self,
  ) -> Result<(), crate::common::SdkError>
  where
    Self: Sized,
  {
    let parts: Vec<_> = self.get_parts_of_type::<T>().collect();
    self.delete_parts(parts)
  }

  #[inline]
  fn delete_parts_recursively_of_type<T: SdkPartHandle + 'static>(
    &mut self,
  ) -> Result<(), crate::common::SdkError>
  where
    Self: Sized,
  {
    self.delete_parts_of_type::<T>()?;
    let child_part_ids: Vec<_> = self
      .relationships()
      .part_relationships()
      .filter_map(crate::common::RelationshipInfo::target_part_id)
      .collect();
    for child_part_id in child_part_ids {
      delete_parts_recursively_from_part_id::<Self, T>(self, child_part_id)?;
    }
    Ok(())
  }

  #[inline]
  fn add_part<T: SdkPartHandle>(&mut self, part: T) -> Result<T, crate::common::SdkError> {
    if self.get_id_of_part(&part).is_some() {
      return Ok(part);
    }
    let relationship_id = self.relationships().next_relationship_id();
    self.add_part_with_id(part, relationship_id)
  }

  #[inline]
  fn add_part_with_id<T: SdkPartHandle>(
    &mut self,
    part: T,
    relationship_id: impl Into<String>,
  ) -> Result<T, crate::common::SdkError> {
    let relationship_id = relationship_id.into();
    let part_id = part.part_id();
    self
      .storage_mut()
      .add_package_relationship_to_part(relationship_id.clone(), part_id)?;
    self.refresh_relationship_model_from_storage();
    Ok(T::from_relationship_id(relationship_id, part_id))
  }

  #[inline]
  fn add_part_from_package<P, T>(
    &mut self,
    source_package: &P,
    part: &T,
  ) -> Result<T, crate::common::SdkError>
  where
    Self: crate::parts::PartRootCache,
    P: SdkPackage + crate::parts::PartRootCache,
    T: SdkPartHandle,
  {
    let relationship_id = part
      .relationship_id()
      .map(str::to_string)
      .unwrap_or_else(|| self.relationships().next_relationship_id());
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
    Self: crate::parts::PartRootCache,
    P: SdkPackage + crate::parts::PartRootCache,
    T: SdkPartHandle,
  {
    let relationship_id = relationship_id.into();
    if self.storage().id() == source_package.storage().id() {
      return self.add_part_with_id(part.clone(), relationship_id);
    }

    let (imported_part_id, added_count) = self.storage_mut().import_part_tree_from(
      source_package.storage(),
      part.part_id(),
      None,
      relationship_id.clone(),
      |part_id, _| source_package.part_bytes_for_copy(part_id),
    )?;
    for _ in 0..added_count {
      self.push_root_element_slot();
    }
    self.refresh_relationship_model_from_storage();
    Ok(T::from_relationship_id(relationship_id, imported_part_id))
  }

  #[inline]
  fn create_relationship_to_part<T: SdkPartHandle>(
    &mut self,
    part: T,
  ) -> Result<String, crate::common::SdkError> {
    if let Some(relationship_id) = self.get_id_of_part(&part) {
      return Ok(relationship_id.to_string());
    }
    let relationship_id = self.relationships().next_relationship_id();
    self.create_relationship_to_part_with_id(part, relationship_id)
  }

  #[inline]
  fn create_relationship_to_part_with_id<T: SdkPartHandle>(
    &mut self,
    part: T,
    relationship_id: impl Into<String>,
  ) -> Result<String, crate::common::SdkError> {
    let relationship_id = relationship_id.into();
    self
      .storage_mut()
      .add_package_relationship_to_part(relationship_id.clone(), part.part_id())?;
    self.refresh_relationship_model_from_storage();
    Ok(relationship_id)
  }

  #[inline]
  fn create_media_data_part(
    &mut self,
    content_type: impl Into<std::borrow::Cow<'static, str>>,
    extension: impl AsRef<str>,
  ) -> Result<crate::common::MediaDataPart, crate::common::SdkError> {
    let part_id = self
      .storage_mut()
      .create_media_data_part(content_type.into().into_owned(), extension)?;
    let path = self
      .storage()
      .part(part_id)
      .ok_or_else(|| {
        crate::common::SdkError::CommonError(format!(
          "part id {part_id:?} is not present in package storage"
        ))
      })?
      .path()
      .to_string();
    Ok(crate::common::MediaDataPart::from_part_id(
      self.storage().id(),
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
    Self: crate::parts::PartRootCache,
    T: SdkPartHandle,
  {
    self.add_new_part_with_target_mode::<T>(
      relationship_id,
      crate::common::NewPartTargetMode::Indexed,
    )
  }

  #[inline]
  fn add_new_part_auto_id<T>(&mut self) -> Result<T, crate::common::SdkError>
  where
    Self: crate::parts::PartRootCache,
    T: SdkPartHandle,
  {
    let relationship_id = self.relationships().next_relationship_id();
    self.add_new_part::<T>(relationship_id)
  }

  #[inline]
  fn add_new_part_with_content_type<T>(
    &mut self,
    relationship_id: impl Into<String>,
    content_type: impl Into<std::borrow::Cow<'static, str>>,
  ) -> Result<T, crate::common::SdkError>
  where
    Self: crate::parts::PartRootCache,
    T: SdkPartHandle,
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
    Self: crate::parts::PartRootCache,
    T: SdkPartHandle,
  {
    let relationship_id = self.relationships().next_relationship_id();
    self.add_new_part_with_content_type::<T>(relationship_id, content_type)
  }

  #[inline]
  fn add_new_part_with_target_mode<T>(
    &mut self,
    relationship_id: impl Into<String>,
    target_mode: crate::common::NewPartTargetMode,
  ) -> Result<T, crate::common::SdkError>
  where
    Self: crate::parts::PartRootCache,
    T: SdkPartHandle,
  {
    let relationship_id = relationship_id.into();
    let part_id = self.storage_mut().add_package_part(
      relationship_id.clone(),
      crate::common::NewPartDescriptor {
        relationship_type: std::borrow::Cow::Borrowed(T::RELATIONSHIP_TYPE),
        content_type: std::borrow::Cow::Borrowed(T::CONTENT_TYPE),
        path_prefix: T::PATH_PREFIX,
        target_name: T::TARGET_NAME,
        extension: std::borrow::Cow::Borrowed(T::EXTENSION),
      },
      target_mode,
    )?;
    self.push_root_element_slot();
    self.refresh_relationship_model_from_storage();
    Ok(T::from_relationship_id(relationship_id, part_id))
  }

  #[inline]
  fn add_core_file_properties_part(
    &mut self,
  ) -> Result<
    crate::parts::core_file_properties_part::CoreFilePropertiesPart,
    crate::common::SdkError,
  >
  where
    Self: crate::parts::PartRootCache,
  {
    let relationship_id = self.relationships().next_relationship_id();
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
  >
  where
    Self: crate::parts::PartRootCache,
  {
    let relationship_id = self.relationships().next_relationship_id();
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
  >
  where
    Self: crate::parts::PartRootCache,
  {
    let relationship_id = self.relationships().next_relationship_id();
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
  >
  where
    Self: crate::parts::PartRootCache,
  {
    let relationship_id = self.relationships().next_relationship_id();
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
    Self: crate::parts::PartRootCache,
    T: SdkPartHandle,
  {
    let relationship_id = relationship_id.into();
    let part_id = self.storage_mut().add_package_part(
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
    self.push_root_element_slot();
    self.refresh_relationship_model_from_storage();
    Ok(T::from_relationship_id(relationship_id, part_id))
  }

  #[inline]
  fn add_thumbnail_part(
    &mut self,
    content_type: impl Into<std::borrow::Cow<'static, str>>,
  ) -> Result<crate::parts::thumbnail_part::ThumbnailPart, crate::common::SdkError>
  where
    Self: crate::parts::PartRootCache,
  {
    self.add_new_part_with_content_type_auto_id::<crate::parts::thumbnail_part::ThumbnailPart>(
      content_type,
    )
  }

  #[inline]
  fn add_thumbnail_part_with_id(
    &mut self,
    content_type: impl Into<std::borrow::Cow<'static, str>>,
    relationship_id: impl Into<String>,
  ) -> Result<crate::parts::thumbnail_part::ThumbnailPart, crate::common::SdkError>
  where
    Self: crate::parts::PartRootCache,
  {
    self.add_new_part_with_content_type::<crate::parts::thumbnail_part::ThumbnailPart>(
      relationship_id,
      content_type,
    )
  }

  #[inline]
  fn add_thumbnail_part_by_type(
    &mut self,
    part_type: ThumbnailPartType,
  ) -> Result<crate::parts::thumbnail_part::ThumbnailPart, crate::common::SdkError>
  where
    Self: crate::parts::PartRootCache,
  {
    let relationship_id = self.relationships().next_relationship_id();
    self.add_thumbnail_part_by_type_with_id(part_type, relationship_id)
  }

  #[inline]
  fn add_thumbnail_part_by_type_with_id(
    &mut self,
    part_type: ThumbnailPartType,
    relationship_id: impl Into<String>,
  ) -> Result<crate::parts::thumbnail_part::ThumbnailPart, crate::common::SdkError>
  where
    Self: crate::parts::PartRootCache,
  {
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
  ) -> Result<crate::parts::extended_part::ExtendedPart, crate::common::SdkError>
  where
    Self: crate::parts::PartRootCache,
  {
    let relationship_id = self.relationships().next_relationship_id();
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
  ) -> Result<crate::parts::extended_part::ExtendedPart, crate::common::SdkError>
  where
    Self: crate::parts::PartRootCache,
  {
    let relationship_id = relationship_id.into();
    let part_id = self.storage_mut().add_package_part(
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
    self.push_root_element_slot();
    self.refresh_relationship_model_from_storage();
    Ok(crate::parts::extended_part::ExtendedPart::from_relationship_id(relationship_id, part_id))
  }
}

#[cfg(feature = "parts")]
pub trait SdkPartHandle: Clone + Sized + 'static {
  const CHILD_DESCRIPTORS: &'static [PartChildDescriptor] = &[];
  const DESCRIPTOR: PartDescriptor;
  const RELATIONSHIP_TYPE: &'static str = Self::DESCRIPTOR.relationship_type;
  const PATH_PREFIX: &'static str = Self::DESCRIPTOR.path_prefix;
  const CONTENT_TYPE: &'static str = Self::DESCRIPTOR.content_type;
  const TARGET_NAME: &'static str = Self::DESCRIPTOR.target_name;
  const EXTENSION: &'static str = Self::DESCRIPTOR.extension;

  fn from_part_id(part_id: crate::common::PartId) -> Self;

  fn from_relationship_id(
    relationship_id: impl Into<String>,
    part_id: crate::common::PartId,
  ) -> Self;

  fn from_part_id_with_relationships(
    storage: &crate::common::SdkPackageStorage,
    part_id: crate::common::PartId,
  ) -> Self {
    let mut visited = std::collections::HashSet::new();
    Self::from_part_id_with_relationships_visited(storage, part_id, &mut visited)
  }

  fn from_part_id_with_relationships_visited(
    storage: &crate::common::SdkPackageStorage,
    part_id: crate::common::PartId,
    visited: &mut std::collections::HashSet<crate::common::PartId>,
  ) -> Self {
    let _ = storage;
    let _ = visited;
    Self::from_part_id(part_id)
  }

  fn from_relationship_id_with_relationships(
    storage: &crate::common::SdkPackageStorage,
    relationship_id: impl Into<String>,
    part_id: crate::common::PartId,
  ) -> Self {
    let mut visited = std::collections::HashSet::new();
    Self::from_relationship_id_with_relationships_visited(
      storage,
      relationship_id,
      part_id,
      &mut visited,
    )
  }

  fn from_relationship_id_with_relationships_visited(
    storage: &crate::common::SdkPackageStorage,
    relationship_id: impl Into<String>,
    part_id: crate::common::PartId,
    visited: &mut std::collections::HashSet<crate::common::PartId>,
  ) -> Self {
    let mut part = Self::from_part_id_with_relationships_visited(storage, part_id, visited);
    part.set_relationship_id(relationship_id.into());
    part
  }

  fn set_relationship_id(&mut self, relationship_id: String);

  fn part_id(&self) -> crate::common::PartId;

  fn relationship_id(&self) -> Option<&str>;

  #[inline(always)]
  fn child_descriptors() -> &'static [PartChildDescriptor] {
    Self::CHILD_DESCRIPTORS
  }

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
  fn relationships<'a, P: SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> Option<&'a crate::common::RelationshipSet> {
    package.storage().relationships(self.part_id())
  }

  #[inline]
  fn relationships_mut<'a, P: SdkPackage>(
    &self,
    package: &'a mut P,
  ) -> Option<&'a mut crate::common::RelationshipSet> {
    package.storage_mut().relationships_mut(self.part_id())
  }

  #[inline]
  fn modeled_relationships<P: SdkPackage>(
    &self,
    package: &P,
  ) -> Result<crate::common::RelationshipSet, crate::common::SdkError> {
    self.relationships(package).cloned().ok_or_else(|| {
      crate::common::SdkError::CommonError(format!(
        "part id {:?} is not present in package storage",
        self.part_id()
      ))
    })
  }

  fn collect_modeled_part_relationships<P: SdkPackage>(
    &self,
    package: &P,
    relationships: &mut std::collections::HashMap<
      crate::common::PartId,
      crate::common::RelationshipSet,
    >,
  ) -> Result<(), crate::common::SdkError> {
    let Some(part) = package.storage().part(self.part_id()) else {
      return Ok(());
    };
    if part.is_deleted() {
      return Ok(());
    }
    if relationships.contains_key(&self.part_id()) {
      return Ok(());
    }
    relationships.insert(self.part_id(), self.modeled_relationships(package)?);
    Ok(())
  }

  #[inline]
  fn add_external_relationship<'a, P: SdkPackage>(
    &self,
    package: &'a mut P,
    relationship_id: impl Into<String>,
    relationship_type: impl Into<String>,
    target: impl Into<String>,
  ) -> Result<&'a crate::common::RelationshipInfo, crate::common::SdkError> {
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
    package.refresh_relationship_model_from_storage();
    Ok(
      package
        .storage()
        .relationships(self.part_id())
        .and_then(|relationships| relationships.get(&relationship_id))
        .expect("relationship was just added"),
    )
  }

  #[inline]
  fn add_external_relationship_auto_id<'a, P: SdkPackage>(
    &self,
    package: &'a mut P,
    relationship_type: impl Into<String>,
    target: impl Into<String>,
  ) -> Result<&'a crate::common::RelationshipInfo, crate::common::SdkError> {
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
  ) -> Result<&'a crate::common::RelationshipInfo, crate::common::SdkError> {
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
    package.refresh_relationship_model_from_storage();
    Ok(
      package
        .storage()
        .relationships(self.part_id())
        .and_then(|relationships| relationships.get(&relationship_id))
        .expect("relationship was just added"),
    )
  }

  #[inline]
  fn add_hyperlink_relationship_with_mode<'a, P: SdkPackage>(
    &self,
    package: &'a mut P,
    relationship_id: impl Into<String>,
    target: impl Into<String>,
    target_mode: crate::schemas::opc_relationships::TargetMode,
  ) -> Result<&'a crate::common::RelationshipInfo, crate::common::SdkError> {
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
    package.refresh_relationship_model_from_storage();
    Ok(
      package
        .storage()
        .relationships(self.part_id())
        .and_then(|relationships| relationships.get(&relationship_id))
        .expect("relationship was just added"),
    )
  }

  #[inline]
  fn add_hyperlink_relationship_auto_id<'a, P: SdkPackage>(
    &self,
    package: &'a mut P,
    target: impl Into<String>,
    target_mode: crate::schemas::opc_relationships::TargetMode,
  ) -> Result<&'a crate::common::RelationshipInfo, crate::common::SdkError> {
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
    relationship: &crate::common::RelationshipInfo,
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
    let relationship_id = package.storage_mut().add_data_part_reference_relationship(
      self.part_id(),
      relationship.id(),
      relationship.relationship_type(),
      target_part_id,
    )?;
    package.refresh_relationship_model_from_storage();
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
    let relationship_id = package.storage_mut().add_data_part_reference_relationship(
      self.part_id(),
      relationship_id,
      relationship_type,
      target_part_id,
    )?;
    package.refresh_relationship_model_from_storage();
    Ok(relationship_id)
  }

  #[inline]
  fn add_new_part<P, T>(
    &self,
    package: &mut P,
    relationship_id: impl Into<String>,
  ) -> Result<T, crate::common::SdkError>
  where
    P: SdkPackage + crate::parts::PartRootCache,
    T: SdkPartHandle,
  {
    let relationship_id = relationship_id.into();
    let part_id = package.storage_mut().add_child_part(
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
    package.push_root_element_slot();
    package.refresh_relationship_model_from_storage();
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
    P: SdkPackage + crate::parts::PartRootCache,
    T: SdkPartHandle,
  {
    let relationship_id = relationship_id.into();
    let part_id = package.storage_mut().add_child_part(
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
    package.push_root_element_slot();
    package.refresh_relationship_model_from_storage();
    Ok(T::from_relationship_id(relationship_id, part_id))
  }

  #[inline]
  fn add_new_part_auto_id<P, T>(&self, package: &mut P) -> Result<T, crate::common::SdkError>
  where
    P: SdkPackage + crate::parts::PartRootCache,
    T: SdkPartHandle,
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
    P: SdkPackage + crate::parts::PartRootCache,
    T: SdkPartHandle,
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
    P: SdkPackage + crate::parts::PartRootCache,
    T: SdkPartHandle,
  {
    let relationship_id = relationship_id.into();
    let part_id = package.storage_mut().add_child_part(
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
    package.push_root_element_slot();
    package.refresh_relationship_model_from_storage();
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
    P: SdkPackage + crate::parts::PartRootCache,
    T: SdkPartHandle,
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
    P: SdkPackage + crate::parts::PartRootCache,
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
    P: SdkPackage + crate::parts::PartRootCache,
  {
    let relationship_id = relationship_id.into();
    let part_id = package.storage_mut().add_child_part(
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
    package.push_root_element_slot();
    package.refresh_relationship_model_from_storage();
    Ok(crate::parts::extended_part::ExtendedPart::from_relationship_id(relationship_id, part_id))
  }

  #[inline]
  fn add_image_part<P>(
    &self,
    package: &mut P,
    content_type: impl Into<std::borrow::Cow<'static, str>>,
  ) -> Result<crate::parts::image_part::ImagePart, crate::common::SdkError>
  where
    P: SdkPackage + crate::parts::PartRootCache,
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
    P: SdkPackage + crate::parts::PartRootCache,
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
    P: SdkPackage + crate::parts::PartRootCache,
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
    P: SdkPackage + crate::parts::PartRootCache,
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
    P: SdkPackage + crate::parts::PartRootCache,
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
    P: SdkPackage + crate::parts::PartRootCache,
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
    P: SdkPackage + crate::parts::PartRootCache,
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
    P: SdkPackage + crate::parts::PartRootCache,
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
    P: SdkPackage + crate::parts::PartRootCache,
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
    P: SdkPackage + crate::parts::PartRootCache,
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
    P: SdkPackage + crate::parts::PartRootCache,
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
    P: SdkPackage + crate::parts::PartRootCache,
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
    P: SdkPackage + crate::parts::PartRootCache,
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
    P: SdkPackage + crate::parts::PartRootCache,
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
    P: SdkPackage + crate::parts::PartRootCache,
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
    P: SdkPackage + crate::parts::PartRootCache,
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
    P: SdkPackage + crate::parts::PartRootCache,
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
    P: SdkPackage + crate::parts::PartRootCache,
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
    P: SdkPackage + crate::parts::PartRootCache,
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
    P: SdkPackage + crate::parts::PartRootCache,
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
    P: SdkPackage + crate::parts::PartRootCache,
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
    P: SdkPackage + crate::parts::PartRootCache,
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
    P: SdkPackage + crate::parts::PartRootCache,
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
    P: SdkPackage + crate::parts::PartRootCache,
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
    P: SdkPackage + crate::parts::PartRootCache,
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
    P: SdkPackage + crate::parts::PartRootCache,
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
    P: SdkPackage + crate::parts::PartRootCache,
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
    P: SdkPackage + crate::parts::PartRootCache,
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
    P: SdkPackage + crate::parts::PartRootCache,
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
    P: SdkPackage + crate::parts::PartRootCache,
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
    P: SdkPackage + crate::parts::PartRootCache,
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
    P: SdkPackage + crate::parts::PartRootCache,
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
    P: SdkPackage + crate::parts::PartRootCache,
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
    P: SdkPackage + crate::parts::PartRootCache,
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
    P: SdkPackage + crate::parts::PartRootCache,
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
    P: SdkPackage + crate::parts::PartRootCache,
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
    P: SdkPackage + crate::parts::PartRootCache,
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
    P: SdkPackage + crate::parts::PartRootCache,
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
  fn remove_relationship<P: SdkPackage>(
    &self,
    package: &mut P,
    relationship_id: &str,
  ) -> Option<crate::common::RelationshipInfo> {
    let removed = self.relationships_mut(package)?.remove(relationship_id);
    if removed.is_some() {
      package.refresh_relationship_model_from_storage();
    }
    removed
  }

  #[inline]
  fn get_reference_relationship<'a, P: SdkPackage>(
    &'a self,
    package: &'a P,
    relationship_id: &str,
  ) -> Option<&'a crate::common::RelationshipInfo> {
    self
      .relationships(package)?
      .get(relationship_id)
      .filter(|relationship| relationship.is_reference_relationship())
  }

  #[inline]
  fn get_external_relationship<'a, P: SdkPackage>(
    &'a self,
    package: &'a P,
    relationship_id: &str,
  ) -> Option<&'a crate::common::RelationshipInfo> {
    self
      .relationships(package)?
      .get_external_relationship(relationship_id)
  }

  #[inline]
  fn get_hyperlink_relationship<'a, P: SdkPackage>(
    &'a self,
    package: &'a P,
    relationship_id: &str,
  ) -> Option<&'a crate::common::RelationshipInfo> {
    self
      .relationships(package)?
      .get_hyperlink_relationship(relationship_id)
  }

  #[inline]
  fn delete_reference_relationship<P: SdkPackage>(
    &self,
    package: &mut P,
    relationship_id: &str,
  ) -> Result<crate::common::RelationshipInfo, crate::common::SdkError> {
    let relationship = self
      .relationships_mut(package)
      .ok_or_else(|| {
        crate::common::SdkError::CommonError(format!(
          "part id {:?} is not present in package storage",
          self.part_id()
        ))
      })?
      .remove_reference_relationship(relationship_id)?;
    package.refresh_relationship_model_from_storage();
    Ok(relationship)
  }

  #[inline]
  fn delete_external_relationship<P: SdkPackage>(
    &self,
    package: &mut P,
    relationship_id: &str,
  ) -> Result<crate::common::RelationshipInfo, crate::common::SdkError> {
    let relationship = self
      .relationships_mut(package)
      .ok_or_else(|| {
        crate::common::SdkError::CommonError(format!(
          "part id {:?} is not present in package storage",
          self.part_id()
        ))
      })?
      .remove_external_relationship(relationship_id)?;
    package.refresh_relationship_model_from_storage();
    Ok(relationship)
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
    package.refresh_relationship_model_from_storage();
    Ok(())
  }

  #[inline]
  fn external_relationships<'a, P: SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = &'a crate::common::RelationshipInfo> {
    self
      .relationships(package)
      .into_iter()
      .flat_map(crate::common::RelationshipSet::external_relationships)
  }

  #[inline]
  fn hyperlink_relationships<'a, P: SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = &'a crate::common::RelationshipInfo> {
    self
      .relationships(package)
      .into_iter()
      .flat_map(crate::common::RelationshipSet::hyperlink_relationships)
  }

  #[inline]
  fn data_part_reference_relationships<'a, P: SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = &'a crate::common::RelationshipInfo> {
    self
      .relationships(package)
      .into_iter()
      .flat_map(crate::common::RelationshipSet::data_part_reference_relationships)
  }

  #[inline]
  fn relationships_by_type<'a, P: SdkPackage>(
    &'a self,
    package: &'a P,
    relationship_type: &'a str,
  ) -> impl Iterator<Item = &'a crate::common::RelationshipInfo> {
    self
      .relationships(package)
      .into_iter()
      .flat_map(move |relationships| relationships.by_relationship_type(relationship_type))
  }

  #[inline]
  fn stored_part<'a, P: SdkPackage>(
    &self,
    package: &'a P,
  ) -> Option<&'a crate::common::StoredPart> {
    package.storage().part(self.part_id())
  }

  #[inline]
  fn path<'a, P: SdkPackage>(&self, package: &'a P) -> Option<&'a str> {
    self
      .stored_part(package)
      .map(crate::common::StoredPart::path)
  }

  #[inline]
  fn content_type<'a, P: SdkPackage>(&self, package: &'a P) -> Option<&'a str> {
    self
      .stored_part(package)
      .map(crate::common::StoredPart::content_type)
  }

  #[inline]
  fn data_kind<P: SdkPackage>(&self, package: &P) -> Option<crate::common::StoredPartDataKind> {
    self.stored_part(package).map(|part| part.data().kind())
  }

  #[inline]
  fn data<'a, P: SdkPackage>(&self, package: &'a P) -> Option<&'a [u8]> {
    self.stored_part(package).map(|part| part.data().bytes())
  }

  #[inline]
  fn set_data<P: SdkPackage>(
    &self,
    package: &mut P,
    data: impl Into<Vec<u8>>,
  ) -> Result<(), crate::common::SdkError> {
    package.storage_mut().set_part_data(self.part_id(), data)
  }

  #[inline]
  fn feed_data<P: SdkPackage, R: std::io::Read>(
    &self,
    package: &mut P,
    reader: &mut R,
  ) -> Result<(), crate::common::SdkError> {
    package.storage_mut().feed_part_data(self.part_id(), reader)
  }

  #[inline]
  fn target_part_id<P: SdkPackage>(
    &self,
    package: &P,
    relationship_id: &str,
  ) -> Option<crate::common::PartId> {
    package
      .storage()
      .target_part_id(self.part_id(), relationship_id)
  }

  #[inline]
  fn target_part_id_required<P: SdkPackage>(
    &self,
    package: &P,
    relationship_id: &str,
  ) -> Result<crate::common::PartId, crate::common::SdkError> {
    self
      .target_part_id(package, relationship_id)
      .ok_or_else(|| {
        crate::common::SdkError::CommonError(format!(
          "part relationship id {relationship_id} does not target an internal part"
        ))
      })
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
    let Some(relationships) = self.relationships(package) else {
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
        package
          .storage()
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
  fn get_part_by_relationship_type<P: SdkPackage + Sized>(
    &self,
    package: &P,
    relationship_type: &str,
  ) -> Option<crate::parts::PartRef> {
    self
      .relationships(package)?
      .by_relationship_type(relationship_type)
      .find_map(|relationship| {
        let part_id = relationship.target_part_id()?;
        crate::parts::PartRef::from_part_id(package, part_id)
      })
  }

  #[inline]
  fn is_child_part<P: SdkPackage, T: SdkPartHandle>(&self, package: &P, part: &T) -> bool {
    let target_part_id = part.part_id();
    self
      .relationships(package)
      .into_iter()
      .flat_map(crate::common::RelationshipSet::part_relationships)
      .any(|relationship| relationship.target_part_id() == Some(target_part_id))
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
  fn get_parts_of_type<'a, P: SdkPackage + Sized, T: SdkPartHandle + 'static>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = T> + 'a {
    self
      .parts(package)
      .filter_map(|entry| entry.part.downcast::<T>())
  }

  #[inline]
  fn get_sub_part_of_type<'a, P: SdkPackage + Sized, T: SdkPartHandle + 'static>(
    &'a self,
    package: &'a P,
  ) -> Option<T> {
    self.get_parts_of_type::<P, T>(package).next()
  }

  #[inline]
  fn get_id_of_part<'a, P: SdkPackage, T: SdkPartHandle>(
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
  fn get_id_of_part_required<'a, P: SdkPackage, T: SdkPartHandle>(
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
  fn change_id_of_part<P: SdkPackage, T: SdkPartHandle>(
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
    let deleted = package
      .storage_mut()
      .delete_child_part(self.part_id(), relationship_id)?;
    if deleted {
      package.refresh_relationship_model_from_storage();
    }
    Ok(deleted)
  }

  #[inline]
  fn delete_part<P: SdkPackage, T: SdkPartHandle>(
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
    T: SdkPartHandle,
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
  fn delete_parts_of_type<P, T>(&self, package: &mut P) -> Result<(), crate::common::SdkError>
  where
    P: SdkPackage + Sized,
    T: SdkPartHandle + 'static,
  {
    let parts: Vec<_> = self.get_parts_of_type::<P, T>(package).collect();
    self.delete_parts::<P, T, _>(package, parts)
  }

  #[inline]
  fn delete_parts_recursively_of_type<P, T>(
    &self,
    package: &mut P,
  ) -> Result<(), crate::common::SdkError>
  where
    P: SdkPackage + Sized,
    T: SdkPartHandle + 'static,
  {
    self.delete_parts_of_type::<P, T>(package)?;
    let child_part_ids: Vec<_> = self
      .relationships(package)
      .into_iter()
      .flat_map(crate::common::RelationshipSet::part_relationships)
      .filter_map(crate::common::RelationshipInfo::target_part_id)
      .collect();
    for child_part_id in child_part_ids {
      delete_parts_recursively_from_part_id::<P, T>(package, child_part_id)?;
    }
    Ok(())
  }

  #[inline]
  fn add_part<P: SdkPackage, T: SdkPartHandle>(
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
  fn add_part_with_id<P: SdkPackage, T: SdkPartHandle>(
    &self,
    package: &mut P,
    part: T,
    relationship_id: impl Into<String>,
  ) -> Result<T, crate::common::SdkError> {
    let relationship_id = relationship_id.into();
    let part_id = part.part_id();
    package.storage_mut().add_child_relationship_to_part(
      self.part_id(),
      relationship_id.clone(),
      part_id,
    )?;
    package.refresh_relationship_model_from_storage();
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
    P: SdkPackage + crate::parts::PartRootCache,
    S: SdkPackage + crate::parts::PartRootCache,
    T: SdkPartHandle,
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
    P: SdkPackage + crate::parts::PartRootCache,
    S: SdkPackage + crate::parts::PartRootCache,
    T: SdkPartHandle,
  {
    let relationship_id = relationship_id.into();
    if package.storage().id() == source_package.storage().id() {
      return self.add_part_with_id(package, part.clone(), relationship_id);
    }

    let (imported_part_id, added_count) = package.storage_mut().import_part_tree_from(
      source_package.storage(),
      part.part_id(),
      Some(self.part_id()),
      relationship_id.clone(),
      |part_id, _| source_package.part_bytes_for_copy(part_id),
    )?;
    for _ in 0..added_count {
      package.push_root_element_slot();
    }
    package.refresh_relationship_model_from_storage();
    Ok(T::from_relationship_id(relationship_id, imported_part_id))
  }

  #[inline]
  fn create_relationship_to_part<P: SdkPackage, T: SdkPartHandle>(
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
  fn create_relationship_to_part_with_id<P: SdkPackage, T: SdkPartHandle>(
    &self,
    package: &mut P,
    part: T,
    relationship_id: impl Into<String>,
  ) -> Result<String, crate::common::SdkError> {
    let relationship_id = relationship_id.into();
    package.storage_mut().add_child_relationship_to_part(
      self.part_id(),
      relationship_id.clone(),
      part.part_id(),
    )?;
    package.refresh_relationship_model_from_storage();
    Ok(relationship_id)
  }
}

#[cfg(feature = "parts")]
pub trait SdkPart: Sized {
  const DESCRIPTOR: PartDescriptor;
  const RELATIONSHIP_TYPE: &'static str = Self::DESCRIPTOR.relationship_type;
  const PATH_PREFIX: &'static str = Self::DESCRIPTOR.path_prefix;
  const CONTENT_TYPE: &'static str = Self::DESCRIPTOR.content_type;
  const TARGET_NAME: &'static str = Self::DESCRIPTOR.target_name;
  const EXTENSION: &'static str = Self::DESCRIPTOR.extension;

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
