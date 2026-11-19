//! Operations on package relationships whose endpoints are known by value.
use std::borrow::Cow;

use crate::common::{
  NewPartDescriptor, NewPartTargetMode, PartKey, PartSlot, RelationshipSet, SdkError,
  SdkPackageStorage, StoredPart, relationship_type_matches_bytes,
};
use crate::parts::{PartKind, PartRef};
use crate::sdk::{PartConstraint, RelatedPart, SdkPackage, SdkPart};

/// Identifies a model type independently of a particular package or handle.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct PartType(pub(crate) PartKind);

pub(crate) struct Metadata {
  name: &'static str,
  relationship_type: &'static str,
  content_type: &'static str,
  path_prefix: &'static str,
  target_name: &'static str,
  extension: &'static str,
  allows_any_child: bool,
}

impl Metadata {
  pub(super) const fn of<T: SdkPart>(name: &'static str) -> Self {
    Self {
      name,
      relationship_type: T::RELATIONSHIP_TYPE,
      content_type: T::CONTENT_TYPE,
      path_prefix: T::PATH_PREFIX,
      target_name: T::TARGET_NAME,
      extension: T::EXTENSION,
      allows_any_child: T::ALLOWS_ANY_CHILD_PART,
    }
  }
}

impl PartType {
  pub const fn of<T: SdkPart>() -> Self {
    Self(T::KIND)
  }
  pub const fn name(self) -> &'static str {
    self.0.metadata().name
  }
  pub const fn relationship_type(self) -> &'static str {
    self.0.metadata().relationship_type
  }
  /// The declared MIME. The effective MIME of a package main part depends on the document type.
  pub const fn content_type(self) -> Option<&'static str> {
    let value = self.0.metadata().content_type;
    if value.is_empty() { None } else { Some(value) }
  }
  pub const fn path_prefix(self) -> &'static str {
    self.0.metadata().path_prefix
  }
  pub const fn target_name(self) -> &'static str {
    self.0.metadata().target_name
  }
  pub const fn extension(self) -> &'static str {
    self.0.metadata().extension
  }
}

/// The source of a relationship. Relationship IDs are local to this source.
#[derive(Clone, Copy, Debug)]
pub enum RelationshipSource<'a> {
  Package,
  Part(&'a PartRef),
}

impl RelationshipSource<'_> {
  fn resolve<P: SdkPackage>(self, package: &P) -> Result<Source, SdkError> {
    match self {
      Self::Package => Ok(Source::PACKAGE),
      Self::Part(part) => Source::part(package, part.part_key()),
    }
  }

  /// Enumerates resolvable child edges in relationship order. Both filters are optional.
  /// A source that no longer resolves has no children.
  pub fn related_parts<'p, P: SdkPackage>(
    self,
    package: &'p P,
    kind: Option<PartType>,
    relationship_type: Option<&'p str>,
  ) -> impl Iterator<Item = RelatedPart<'p, PartRef>> + use<'p, P> {
    let relationships = self
      .resolve(package)
      .and_then(|source| source.relationships(package))
      .ok();
    relationships
      .into_iter()
      .flat_map(RelationshipSet::part_relationships)
      .filter_map(move |rel| {
        if relationship_type.is_some_and(|uri| {
          !relationship_type_matches_bytes(rel.relationship_type_bytes(), uri.as_bytes())
        }) {
          return None;
        }
        let part = PartRef::from_relationship_storage(P::storage(package), rel)?;
        if kind.is_some_and(|kind| kind != part.part_type()) {
          return None;
        }
        Some(RelatedPart::new(rel.id(), rel.relationship_type(), part))
      })
  }

  /// Returns the first ID pointing to this child.
  pub fn get_id_of_part<'p, P: SdkPackage>(
    self,
    package: &'p P,
    child: &PartRef,
  ) -> Result<&'p str, SdkError> {
    self.resolve(package)?.id_of(package, child.part_key())
  }

  /// Renames the unique relationship to this child and returns the previous ID.
  pub fn change_id_of_part<P: SdkPackage>(
    self,
    package: &mut P,
    child: &PartRef,
    id: &str,
  ) -> Result<String, SdkError> {
    self
      .resolve(package)?
      .change_id_of(package, child.part_key(), id.into())
  }

  /// Renames a specific relationship. XML references can be edited through the part root.
  pub fn change_relationship_id<P: SdkPackage>(
    self,
    package: &mut P,
    old: &str,
    new: &str,
  ) -> Result<(), SdkError> {
    self.resolve(package)?.rename(package, old, new.into())
  }

  /// Links an existing part in this package. With no ID an existing link is reused.
  pub fn create_relationship_to_part<P: SdkPackage>(
    self,
    package: &mut P,
    child: &PartRef,
    id: Option<&str>,
  ) -> Result<String, SdkError> {
    self
      .resolve(package)?
      .link(package, child.part_key(), id.map(str::to_owned))
  }

  /// Copies the reachable source graph and returns a handle belonging to the destination.
  pub fn add_part_from_package<P: SdkPackage, S: SdkPackage>(
    self,
    package: &mut P,
    source_package: &S,
    child: &PartRef,
    id: Option<&str>,
  ) -> Result<PartRef, SdkError> {
    let slot = self.resolve(package)?.import(
      package,
      source_package,
      child.part_key(),
      id.map(str::to_owned),
    )?;
    Ok(PartRef::from_part_slot(package, slot).expect("imported part"))
  }

  /// Previews the next available ID without reserving it.
  pub fn next_relationship_id<P: SdkPackage>(self, package: &P) -> Result<String, SdkError> {
    Ok(
      self
        .resolve(package)?
        .relationships(package)?
        .next_relationship_id(),
    )
  }

  /// Removes a child relationship and parts that become unreachable.
  pub fn delete_part_by_id<P: SdkPackage>(
    self,
    package: &mut P,
    id: &str,
  ) -> Result<bool, SdkError> {
    self.resolve(package)?.delete(package, id)
  }
}

/// Destination and path policy for a new part. An explicit path includes its extension.
#[derive(Clone, Copy, Debug)]
pub enum NewPartTarget<'a> {
  PackageIndexed {
    extension: Option<&'a str>,
  },
  PackageFixed {
    extension: Option<&'a str>,
  },
  Child {
    parent: &'a PartRef,
    extension: Option<&'a str>,
  },
  ChildAt {
    parent: &'a PartRef,
    path: &'a str,
  },
}

impl NewPartTarget<'_> {
  /// Creates a part with the requested type. A variable-content child requires a MIME.
  pub fn add_new_part<P: SdkPackage>(
    self,
    package: &mut P,
    ty: PartType,
    content_type: Option<&str>,
    id: Option<&str>,
  ) -> Result<PartRef, SdkError> {
    let (placement, extension) = match self {
      Self::PackageIndexed { extension } => {
        (Placement::Package(NewPartTargetMode::Indexed), extension)
      }
      Self::PackageFixed { extension } => (Placement::Package(NewPartTargetMode::Fixed), extension),
      Self::Child { parent, extension } => (
        Placement::Child(parent.part_key().resolve(P::storage(package))?, None),
        extension,
      ),
      Self::ChildAt { parent, path } => (
        Placement::Child(parent.part_key().resolve(P::storage(package))?, Some(path)),
        None,
      ),
    };
    let slot = placement.create(package, ty, content_type, extension, id.map(str::to_owned))?;
    Ok(PartRef::from_part_slot(package, slot).expect("created part"))
  }
}

#[derive(Clone, Copy)]
pub(crate) struct Source(Option<PartSlot>);

impl Source {
  pub(crate) const PACKAGE: Self = Self(None);

  pub(crate) fn part<P: SdkPackage>(package: &P, key: PartKey) -> Result<Self, SdkError> {
    Ok(Self(Some(key.resolve(P::storage(package))?)))
  }

  fn relationships<P: SdkPackage>(self, package: &P) -> Result<&RelationshipSet, SdkError> {
    match self.0 {
      None => Ok(P::relationships(package)),
      Some(slot) => P::storage(package)
        .relationships(slot)
        .ok_or(SdkError::StalePart),
    }
  }

  pub(crate) fn id_of<P: SdkPackage>(self, package: &P, target: PartKey) -> Result<&str, SdkError> {
    let slot = target.resolve(P::storage(package))?;
    self
      .relationships(package)?
      .first_id_for_part(slot)
      .ok_or(SdkError::PartNotReferenced)
  }

  pub(crate) fn change_id_of<P: SdkPackage>(
    self,
    package: &mut P,
    target: PartKey,
    new: String,
  ) -> Result<String, SdkError> {
    let slot = target.resolve(P::storage(package))?;
    let old =
      crate::sdk::unique_relationship_id_for_part(self.relationships(package)?, slot)?.to_owned();
    self.rename(package, &old, new)?;
    Ok(old)
  }

  fn rename<P: SdkPackage>(self, package: &mut P, old: &str, new: String) -> Result<(), SdkError> {
    let relationships = match self.0 {
      None => P::relationships_mut(package),
      Some(slot) => P::storage_mut(package)
        .relationships_mut(slot)
        .ok_or(SdkError::StalePart)?,
    };
    relationships.change_relationship_id(old, new)?;
    Ok(())
  }

  pub(crate) fn delete<P: SdkPackage>(self, package: &mut P, id: &str) -> Result<bool, SdkError> {
    let storage = P::storage_mut(package);
    let deleted = match self.0 {
      None => storage.delete_package_part(id)?,
      Some(slot) => storage.delete_child_part(slot, id)?,
    };
    let Some(slots) = deleted else {
      return Ok(false);
    };
    for slot in slots {
      P::take_root_element(package, slot);
    }
    Ok(true)
  }

  fn validate<P: SdkPackage>(
    self,
    package: &P,
    ty: PartType,
    mime: &str,
  ) -> Result<PartConstraint, SdkError> {
    match self.0 {
      None => package.validate_child_part_type(ty, mime),
      Some(slot) => {
        let parent = P::storage(package).part(slot).ok_or(SdkError::StalePart)?;
        crate::sdk::validate_child_part_constraint(
          ty,
          parent.kind().child_constraint(ty.0),
          parent.kind().metadata().allows_any_child,
          parent.relationships(),
          mime,
        )
      }
    }
  }

  fn relationship_type<P: SdkPackage>(
    self,
    package: &P,
    target: &StoredPart,
  ) -> Result<Cow<'static, str>, SdkError> {
    let constraint = self.validate(package, PartType(target.kind()), target.content_type())?;
    Ok(crate::sdk::constrained_relationship_type(
      constraint,
      target.relationship_type(),
    ))
  }

  fn attach(
    self,
    storage: &mut SdkPackageStorage,
    target: PartSlot,
    uri: &str,
    id: String,
  ) -> Result<String, SdkError> {
    match self.0 {
      None => storage.add_package_relationship_to_part(id, uri, target),
      Some(slot) => storage.add_child_relationship_to_part(slot, id, uri, target),
    }
  }

  pub(crate) fn link<P: SdkPackage>(
    self,
    package: &mut P,
    key: PartKey,
    id: Option<String>,
  ) -> Result<String, SdkError> {
    let storage = P::storage(package);
    let target = key.resolve(storage)?;
    let id = match id {
      Some(id) => id,
      None => {
        let relationships = self.relationships(package)?;
        if let Some(id) = relationships.first_id_for_part(target) {
          return Ok(id.to_owned());
        }
        relationships.next_relationship_id()
      }
    };
    let uri = self.relationship_type(package, storage.part(target).ok_or(SdkError::StalePart)?)?;
    self.attach(P::storage_mut(package), target, &uri, id)
  }

  pub(crate) fn import<P: SdkPackage, S: SdkPackage>(
    self,
    package: &mut P,
    source: &S,
    key: PartKey,
    id: Option<String>,
  ) -> Result<PartSlot, SdkError> {
    let storage = S::storage(source);
    let target = key.resolve(storage)?;
    let part = storage.part(target).ok_or(SdkError::StalePart)?;
    let uri = self.relationship_type(package, part)?;
    let id = match id {
      Some(id) => id,
      None => self.relationships(package)?.next_relationship_id(),
    };
    if storage.token() == P::storage(package).token() {
      self.attach(P::storage_mut(package), target, &uri, id)?;
      return Ok(target);
    }
    let (slot, count) = P::storage_mut(package).import_part_tree_from(
      storage,
      target,
      self.0,
      id,
      &uri,
      |slot, _| S::part_bytes_for_copy(source, slot),
    )?;
    for _ in 0..count {
      P::push_root_element_slot(package);
    }
    Ok(slot)
  }
}

pub(crate) enum Placement<'a> {
  Package(NewPartTargetMode),
  Child(PartSlot, Option<&'a str>),
}

impl Placement<'_> {
  pub(crate) fn create<P: SdkPackage>(
    self,
    package: &mut P,
    ty: PartType,
    mime: Option<&str>,
    extension: Option<&str>,
    id: Option<String>,
  ) -> Result<PartSlot, SdkError> {
    let source = match self {
      Self::Package(_) => Source::PACKAGE,
      Self::Child(slot, _) => Source(Some(slot)),
    };
    let mime = mime
      .or_else(|| {
        if source.0.is_none() {
          package.main_part_content_type(ty.0)
        } else {
          None
        }
      })
      .or_else(|| ty.content_type())
      .unwrap_or_default();
    let constraint = source.validate(package, ty, mime)?;
    let id = match id {
      Some(id) => id,
      None => source.relationships(package)?.next_relationship_id(),
    };
    let descriptor = NewPartDescriptor {
      expected_kind: Some(ty.0),
      relationship_type: Cow::Borrowed(constraint.relationship_type),
      content_type: Cow::Borrowed(mime),
      path_prefix: ty.path_prefix(),
      target_name: ty.target_name(),
      extension: Cow::Borrowed(extension.unwrap_or_else(|| ty.extension())),
    };
    let storage = P::storage_mut(package);
    let slot = match self {
      Self::Package(mode) => storage.add_package_part(id, descriptor, mode)?,
      Self::Child(parent, None) => storage.add_child_part(parent, id, descriptor)?,
      Self::Child(parent, Some(path)) => {
        storage.add_child_part_with_path(parent, id, descriptor, path)?
      }
    };
    P::push_root_element_slot(package);
    Ok(slot)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::parts::wordprocessing_document::WordprocessingDocument;
  use crate::sdk::{PresentationDocumentType, SpreadsheetDocumentType, WordprocessingDocumentType};

  fn sample_mime(ty: PartType) -> &'static str {
    match ty.0 {
      PartKind::MainDocumentPart => WordprocessingDocumentType::Document.content_type(),
      PartKind::WorkbookPart => SpreadsheetDocumentType::Workbook.content_type(),
      PartKind::PresentationPart => PresentationDocumentType::Presentation.content_type(),
      _ => ty.content_type().unwrap_or("application/octet-stream"),
    }
  }

  #[test]
  fn every_declared_part_edge_returns_its_catalog_type() {
    for &parent_type in PartType::all() {
      if parent_type.0 == PartKind::ExtendedPart {
        continue;
      }
      let mut doc = WordprocessingDocument::create(WordprocessingDocumentType::Document);
      let extension = doc
        .add_extended_part("urn:test:extension", "text/xml", ".xml")
        .unwrap();
      let extension = PartRef::ExtendedPart(extension);
      let parent = NewPartTarget::Child {
        parent: &extension,
        extension: None,
      }
      .add_new_part(&mut doc, parent_type, Some(sample_mime(parent_type)), None)
      .unwrap_or_else(|e| panic!("cannot create {parent_type:?}: {e}"));
      assert_eq!(parent.part_type(), parent_type);
      let target = NewPartTarget::Child {
        parent: &parent,
        extension: None,
      };
      let source = RelationshipSource::Part(&parent);
      for &child_type in PartType::all() {
        let Some(rule) = parent_type.0.child_constraint(child_type.0) else {
          continue;
        };
        let child = target
          .add_new_part(&mut doc, child_type, Some(sample_mime(child_type)), None)
          .unwrap_or_else(|e| panic!("{parent_type:?} -> {child_type:?}: {e}"));
        assert_eq!(child.part_type(), child_type);
        let edge = source
          .related_parts(&doc, Some(child_type), Some(rule.relationship_type))
          .next()
          .unwrap();
        assert_eq!(edge.part(), &child);
      }
    }
  }
}
