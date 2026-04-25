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

  #[inline]
  fn relationships(&self) -> &crate::common::RelationshipSet {
    self.storage().package_relationships()
  }

  #[inline]
  fn relationships_mut(&mut self) -> &mut crate::common::RelationshipSet {
    self.storage_mut().package_relationships_mut()
  }

  #[inline]
  fn add_external_relationship(
    &mut self,
    relationship_id: impl Into<String>,
    relationship_type: impl Into<String>,
    target: impl Into<String>,
  ) -> Result<&crate::common::RelationshipInfo, crate::common::SdkError> {
    self
      .relationships_mut()
      .add_external_relationship(relationship_id, relationship_type, target)
  }

  #[inline]
  fn add_hyperlink_relationship(
    &mut self,
    relationship_id: impl Into<String>,
    target: impl Into<String>,
  ) -> Result<&crate::common::RelationshipInfo, crate::common::SdkError> {
    self
      .relationships_mut()
      .add_hyperlink_relationship(relationship_id, target)
  }

  #[inline]
  fn remove_relationship(
    &mut self,
    relationship_id: &str,
  ) -> Option<crate::common::RelationshipInfo> {
    self.relationships_mut().remove(relationship_id)
  }

  #[inline]
  fn change_relationship_id(
    &mut self,
    relationship_id: &str,
    new_relationship_id: impl Into<String>,
  ) -> Result<(), crate::common::SdkError> {
    self
      .relationships_mut()
      .change_relationship_id(relationship_id, new_relationship_id)
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
  fn get_id_of_part<T: SdkPartHandle>(&self, part: T) -> Option<&str> {
    let target_part_id = part.part_id();
    self.relationships().iter().find_map(|relationship| {
      (relationship.target_part_id() == Some(target_part_id)).then_some(relationship.id())
    })
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
  fn add_new_part_with_target_mode<T>(
    &mut self,
    relationship_id: impl Into<String>,
    target_mode: crate::common::NewPartTargetMode,
  ) -> Result<T, crate::common::SdkError>
  where
    Self: crate::parts::PartRootCache,
    T: SdkPartHandle,
  {
    let part_id = self.storage_mut().add_package_part(
      relationship_id,
      crate::common::NewPartDescriptor {
        relationship_type: T::RELATIONSHIP_TYPE,
        content_type: std::borrow::Cow::Borrowed(T::CONTENT_TYPE),
        path_prefix: T::PATH_PREFIX,
        target_name: T::TARGET_NAME,
        extension: T::EXTENSION,
      },
      target_mode,
    )?;
    self.push_root_element_slot();
    Ok(T::from_part_id(part_id))
  }
}

#[cfg(feature = "parts")]
pub trait SdkPartHandle: Copy + Sized + 'static {
  const DESCRIPTOR: PartDescriptor;
  const RELATIONSHIP_TYPE: &'static str = Self::DESCRIPTOR.relationship_type;
  const PATH_PREFIX: &'static str = Self::DESCRIPTOR.path_prefix;
  const CONTENT_TYPE: &'static str = Self::DESCRIPTOR.content_type;
  const TARGET_NAME: &'static str = Self::DESCRIPTOR.target_name;
  const EXTENSION: &'static str = Self::DESCRIPTOR.extension;

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
  fn relationships<P: SdkPackage>(self, package: &P) -> Option<&crate::common::RelationshipSet> {
    package.storage().relationships(self.part_id())
  }

  #[inline]
  fn relationships_mut<P: SdkPackage>(
    self,
    package: &mut P,
  ) -> Option<&mut crate::common::RelationshipSet> {
    package.storage_mut().relationships_mut(self.part_id())
  }

  #[inline]
  fn add_external_relationship<P: SdkPackage>(
    self,
    package: &mut P,
    relationship_id: impl Into<String>,
    relationship_type: impl Into<String>,
    target: impl Into<String>,
  ) -> Result<&crate::common::RelationshipInfo, crate::common::SdkError> {
    self
      .relationships_mut(package)
      .ok_or_else(|| {
        crate::common::SdkError::CommonError(format!(
          "part id {:?} is not present in package storage",
          self.part_id()
        ))
      })?
      .add_external_relationship(relationship_id, relationship_type, target)
  }

  #[inline]
  fn add_hyperlink_relationship<P: SdkPackage>(
    self,
    package: &mut P,
    relationship_id: impl Into<String>,
    target: impl Into<String>,
  ) -> Result<&crate::common::RelationshipInfo, crate::common::SdkError> {
    self
      .relationships_mut(package)
      .ok_or_else(|| {
        crate::common::SdkError::CommonError(format!(
          "part id {:?} is not present in package storage",
          self.part_id()
        ))
      })?
      .add_hyperlink_relationship(relationship_id, target)
  }

  #[inline]
  fn add_new_part<P, T>(
    self,
    package: &mut P,
    relationship_id: impl Into<String>,
  ) -> Result<T, crate::common::SdkError>
  where
    P: SdkPackage + crate::parts::PartRootCache,
    T: SdkPartHandle,
  {
    let part_id = package.storage_mut().add_child_part(
      self.part_id(),
      relationship_id,
      crate::common::NewPartDescriptor {
        relationship_type: T::RELATIONSHIP_TYPE,
        content_type: std::borrow::Cow::Borrowed(T::CONTENT_TYPE),
        path_prefix: T::PATH_PREFIX,
        target_name: T::TARGET_NAME,
        extension: T::EXTENSION,
      },
    )?;
    package.push_root_element_slot();
    Ok(T::from_part_id(part_id))
  }

  #[inline]
  fn add_new_part_with_content_type<P, T>(
    self,
    package: &mut P,
    relationship_id: impl Into<String>,
    content_type: impl Into<std::borrow::Cow<'static, str>>,
  ) -> Result<T, crate::common::SdkError>
  where
    P: SdkPackage + crate::parts::PartRootCache,
    T: SdkPartHandle,
  {
    let part_id = package.storage_mut().add_child_part(
      self.part_id(),
      relationship_id,
      crate::common::NewPartDescriptor {
        relationship_type: T::RELATIONSHIP_TYPE,
        content_type: content_type.into(),
        path_prefix: T::PATH_PREFIX,
        target_name: T::TARGET_NAME,
        extension: T::EXTENSION,
      },
    )?;
    package.push_root_element_slot();
    Ok(T::from_part_id(part_id))
  }

  #[inline]
  fn add_new_part_auto_id<P, T>(self, package: &mut P) -> Result<T, crate::common::SdkError>
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
    self,
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
  fn remove_relationship<P: SdkPackage>(
    self,
    package: &mut P,
    relationship_id: &str,
  ) -> Option<crate::common::RelationshipInfo> {
    self.relationships_mut(package)?.remove(relationship_id)
  }

  #[inline]
  fn change_relationship_id<P: SdkPackage>(
    self,
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
      .change_relationship_id(relationship_id, new_relationship_id)
  }

  #[inline]
  fn external_relationships<P: SdkPackage>(
    self,
    package: &P,
  ) -> impl Iterator<Item = &crate::common::RelationshipInfo> {
    self
      .relationships(package)
      .into_iter()
      .flat_map(crate::common::RelationshipSet::external_relationships)
  }

  #[inline]
  fn hyperlink_relationships<P: SdkPackage>(
    self,
    package: &P,
  ) -> impl Iterator<Item = &crate::common::RelationshipInfo> {
    self
      .relationships(package)
      .into_iter()
      .flat_map(crate::common::RelationshipSet::hyperlink_relationships)
  }

  #[inline]
  fn data_part_reference_relationships<P: SdkPackage>(
    self,
    package: &P,
  ) -> impl Iterator<Item = &crate::common::RelationshipInfo> {
    self
      .relationships(package)
      .into_iter()
      .flat_map(crate::common::RelationshipSet::data_part_reference_relationships)
  }

  #[inline]
  fn relationships_by_type<'a, P: SdkPackage>(
    self,
    package: &'a P,
    relationship_type: &'a str,
  ) -> impl Iterator<Item = &'a crate::common::RelationshipInfo> {
    self
      .relationships(package)
      .into_iter()
      .flat_map(move |relationships| relationships.by_relationship_type(relationship_type))
  }

  #[inline]
  fn stored_part<P: SdkPackage>(self, package: &P) -> Option<&crate::common::StoredPart> {
    package.storage().part(self.part_id())
  }

  #[inline]
  fn path<P: SdkPackage>(self, package: &P) -> Option<&str> {
    self
      .stored_part(package)
      .map(crate::common::StoredPart::path)
  }

  #[inline]
  fn content_type<P: SdkPackage>(self, package: &P) -> Option<&str> {
    self
      .stored_part(package)
      .map(crate::common::StoredPart::content_type)
  }

  #[inline]
  fn data_kind<P: SdkPackage>(self, package: &P) -> Option<crate::common::StoredPartDataKind> {
    self.stored_part(package).map(|part| part.data().kind())
  }

  #[inline]
  fn data<P: SdkPackage>(self, package: &P) -> Option<&[u8]> {
    self.stored_part(package).map(|part| part.data().bytes())
  }

  #[inline]
  fn set_data<P: SdkPackage>(
    self,
    package: &mut P,
    data: impl Into<Vec<u8>>,
  ) -> Result<(), crate::common::SdkError> {
    package.storage_mut().set_part_data(self.part_id(), data)
  }

  #[inline]
  fn feed_data<P: SdkPackage, R: std::io::Read>(
    self,
    package: &mut P,
    reader: &mut R,
  ) -> Result<(), crate::common::SdkError> {
    package.storage_mut().feed_part_data(self.part_id(), reader)
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

  #[inline]
  fn parts<P: SdkPackage + Sized>(
    self,
    package: &P,
  ) -> impl Iterator<Item = crate::parts::IdPartPair<'_>> + '_ {
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
  fn get_part_by_id<P: SdkPackage + Sized>(
    self,
    package: &P,
    relationship_id: &str,
  ) -> Option<crate::parts::PartRef> {
    let part_id = self.target_part_id(package, relationship_id)?;
    crate::parts::PartRef::from_part_id(package, part_id)
  }

  #[inline]
  fn try_get_part_by_id<P: SdkPackage + Sized>(
    self,
    package: &P,
    relationship_id: &str,
  ) -> Option<crate::parts::PartRef> {
    self.get_part_by_id(package, relationship_id)
  }

  #[inline]
  fn get_parts_of_type<P: SdkPackage + Sized, T: SdkPartHandle + 'static>(
    self,
    package: &P,
  ) -> impl Iterator<Item = T> + '_ {
    self
      .parts(package)
      .filter_map(|entry| entry.part.downcast::<T>())
  }

  #[inline]
  fn get_sub_part_of_type<P: SdkPackage + Sized, T: SdkPartHandle + 'static>(
    self,
    package: &P,
  ) -> Option<T> {
    self.get_parts_of_type::<P, T>(package).next()
  }

  #[inline]
  fn get_id_of_part<P: SdkPackage, T: SdkPartHandle>(self, package: &P, part: T) -> Option<&str> {
    let target_part_id = part.part_id();
    self
      .relationships(package)?
      .iter()
      .find_map(|relationship| {
        (relationship.target_part_id() == Some(target_part_id)).then_some(relationship.id())
      })
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
