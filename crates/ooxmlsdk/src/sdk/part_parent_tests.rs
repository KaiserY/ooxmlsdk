//! Differential tests: every `PartParent` operation must have the same result
//! and the same effect on the package as the typed method it mirrors.
//!
//! Each case builds two identical fixtures, runs the typed method on one and
//! the run-time method on the other, and compares the outcome (including the
//! error) and a dump of the whole package.

use std::fmt::Write as _;

use super::{
  NewPartOptions, PartParent, SdkPackage, SdkPart, SdkPartDescriptor, WordprocessingDocumentType,
};
use crate::common::{NewPartTargetMode, PartSlot, SdkError};
use crate::parts::core_file_properties_part::CoreFilePropertiesPart;
use crate::parts::extended_part::ExtendedPart;
use crate::parts::footer_part::FooterPart;
use crate::parts::header_part::HeaderPart;
use crate::parts::image_part::ImagePart;
use crate::parts::main_document_part::MainDocumentPart;
use crate::parts::theme_part::ThemePart;
use crate::parts::wordprocessing_document::WordprocessingDocument;
use crate::parts::worksheet_part::WorksheetPart;
use crate::parts::{PartKind, PartRef};

type Doc = WordprocessingDocument;

const IMAGE_TYPE: &str = "image/png";
const REL_IMAGE: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image";
const STRICT_REL_IMAGE: &str = "http://purl.oclc.org/ooxml/officeDocument/relationships/image";
const REL_THUMBNAIL: &str =
  "http://schemas.openxmlformats.org/package/2006/relationships/metadata/thumbnail";
const STRICT_REL_THUMBNAIL: &str =
  "http://purl.oclc.org/ooxml/officeDocument/relationships/metadata/thumbnail";
const REL_CORE: &str =
  "http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties";
const O12_REL_CORE: &str =
  "http://schemas.microsoft.com/package/2005/06/relationships/metadata/core-properties";
const CORE_TYPE: &str = "application/vnd.openxmlformats-package.core-properties+xml";

// ---------------------------------------------------------------------------
// Fixture and helpers
// ---------------------------------------------------------------------------

/// The state of a handle relative to the package it is used with.
#[derive(Clone, Copy, Debug)]
enum State {
  Valid,
  Stale,
  Foreign,
}

const STATES: [State; 3] = [State::Valid, State::Stale, State::Foreign];

/// The fixture of the differential tests: two Word packages, with part handles
/// in every state.
///
/// `header` and `image` are valid in `doc`. `header` already references
/// `image`; `image_free` is an image that no header references. The `*_stale`
/// handles were deleted from `doc`, and the `*_foreign` handles belong to
/// `other`. `other_*` are valid in `other`, and its `*_stale` handle was
/// deleted from it.
struct Fx {
  doc: Doc,
  main: MainDocumentPart,
  header: HeaderPart,
  header_stale: HeaderPart,
  header_foreign: HeaderPart,
  image: ImagePart,
  image_free: ImagePart,
  image_stale: ImagePart,
  image_foreign: ImagePart,
  extended: ExtendedPart,
  other: Doc,
  other_image: ImagePart,
  other_image_stale: ImagePart,
}

impl Fx {
  fn new() -> Self {
    let mut doc = Doc::create(WordprocessingDocumentType::Document);
    let main = doc.add_main_document_part().expect("main part");
    let header = main
      .add_new_part_auto_id::<_, HeaderPart>(&mut doc)
      .expect("header");
    let header_stale = main
      .add_new_part_auto_id::<_, HeaderPart>(&mut doc)
      .expect("stale header");
    let image = main
      .add_new_part_with_content_type_auto_id::<_, ImagePart>(&mut doc, IMAGE_TYPE)
      .expect("image");
    let image_free = main
      .add_new_part_with_content_type_auto_id::<_, ImagePart>(&mut doc, IMAGE_TYPE)
      .expect("free image");
    let image_stale = main
      .add_new_part_with_content_type_auto_id::<_, ImagePart>(&mut doc, IMAGE_TYPE)
      .expect("stale image");
    let extended = main
      .add_extended_part(
        &mut doc,
        "urn:example:extended",
        "application/example",
        ".bin",
      )
      .expect("extended part");
    header
      .create_relationship_to_part_with_id(&mut doc, image.clone(), "rIdImage")
      .expect("header references the image");
    main
      .delete_part(&mut doc, header_stale.clone())
      .expect("delete the stale header");
    main
      .delete_part(&mut doc, image_stale.clone())
      .expect("delete the stale image");

    let mut other = Doc::create(WordprocessingDocumentType::Document);
    let other_main = other.add_main_document_part().expect("other main part");
    let header_foreign = other_main
      .add_new_part_auto_id::<_, HeaderPart>(&mut other)
      .expect("foreign header");
    let image_foreign = other_main
      .add_new_part_with_content_type_auto_id::<_, ImagePart>(&mut other, IMAGE_TYPE)
      .expect("foreign image");
    header_foreign
      .add_new_part_with_content_type_auto_id::<_, ImagePart>(&mut other, IMAGE_TYPE)
      .expect("image below the foreign header");
    let other_image = other_main
      .add_new_part_with_content_type_auto_id::<_, ImagePart>(&mut other, IMAGE_TYPE)
      .expect("other image");
    let other_image_stale = other_main
      .add_new_part_with_content_type_auto_id::<_, ImagePart>(&mut other, IMAGE_TYPE)
      .expect("other stale image");
    other_main
      .delete_part(&mut other, other_image_stale.clone())
      .expect("delete the other stale image");

    Self {
      doc,
      main,
      header,
      header_stale,
      header_foreign,
      image,
      image_free,
      image_stale,
      image_foreign,
      extended,
      other,
      other_image,
      other_image_stale,
    }
  }

  fn header(&self, state: State) -> HeaderPart {
    match state {
      State::Valid => self.header.clone(),
      State::Stale => self.header_stale.clone(),
      State::Foreign => self.header_foreign.clone(),
    }
  }

  fn image(&self, state: State) -> ImagePart {
    match state {
      State::Valid => self.image.clone(),
      State::Stale => self.image_stale.clone(),
      State::Foreign => self.image_foreign.clone(),
    }
  }

  /// The image to copy from `other` into `doc`, in the given state relative to
  /// `other`.
  fn source_image(&self, state: State) -> ImagePart {
    match state {
      State::Valid => self.other_image.clone(),
      State::Stale => self.other_image_stale.clone(),
      State::Foreign => self.image.clone(),
    }
  }
}

fn part_ref<T: SdkPart>(part: &T) -> PartRef {
  PartRef::from_kind_and_key(T::KIND, part.part_key())
}

fn show<T>(result: Result<T, SdkError>, describe: impl FnOnce(&T) -> String) -> String {
  match result {
    Ok(value) => format!("Ok({})", describe(&value)),
    Err(error) => format!("Err({error:?})"),
  }
}

/// A part handle, as a stable description that does not depend on the package.
fn describe_part<P: SdkPackage>(package: &P, part: &PartRef) -> String {
  let storage = package.storage();
  match part.part_key().resolve(storage) {
    Ok(slot) => {
      let stored = storage.part(slot).expect("resolved part");
      format!(
        "{:?} {} {} {:?}",
        part.kind(),
        stored.path(),
        stored.content_type(),
        stored.relationship_type()
      )
    }
    Err(error) => format!("{:?} {error:?}", part.kind()),
  }
}

fn relationships_dump(
  out: &mut String,
  label: &str,
  relationships: &crate::common::RelationshipSet,
) {
  for relationship in relationships.iter() {
    writeln!(
      out,
      "{label} {} {} {} {:?}",
      relationship.id(),
      relationship.relationship_type(),
      relationship.target(),
      relationship.target_part_slot().map(PartSlot::index),
    )
    .expect("write to a string");
  }
}

/// Every part and relationship of a package, in storage order.
fn dump<P: SdkPackage>(package: &P) -> String {
  let storage = package.storage();
  let mut out = String::new();
  relationships_dump(&mut out, "package", package.relationships());
  for index in 0..storage.parts().len() {
    let slot = PartSlot::from_index(index);
    match storage.part(slot) {
      Some(part) => {
        writeln!(
          out,
          "part {index} {} {} {:?} {:?}",
          part.path(),
          part.content_type(),
          part.relationship_type(),
          part.kind()
        )
        .expect("write to a string");
        if let Some(relationships) = storage.relationships(slot) {
          relationships_dump(&mut out, &format!("part {index}"), relationships);
        }
      }
      None => writeln!(out, "part {index} deleted").expect("write to a string"),
    }
  }
  out
}

/// Runs `typed` on one fixture and `dynamic` on another, and requires equal
/// outcomes and equal packages.
fn assert_same(
  label: &str,
  typed: impl FnOnce(&mut Fx) -> String,
  dynamic: impl FnOnce(&mut Fx) -> String,
) {
  let mut typed_fx = Fx::new();
  let mut dynamic_fx = Fx::new();
  let typed_outcome = typed(&mut typed_fx);
  let dynamic_outcome = dynamic(&mut dynamic_fx);
  assert_eq!(typed_outcome, dynamic_outcome, "{label}: outcome");
  assert_eq!(
    dump(&typed_fx.doc),
    dump(&dynamic_fx.doc),
    "{label}: package"
  );
  assert_eq!(
    dump(&typed_fx.other),
    dump(&dynamic_fx.other),
    "{label}: other package"
  );
}

fn describe_typed<T: SdkPart>(doc: &Doc) -> impl Fn(&T) -> String + '_ {
  move |part| describe_part(doc, &part_ref(part))
}

fn describe_ref(doc: &Doc) -> impl Fn(&PartRef) -> String + '_ {
  move |part| describe_part(doc, part)
}

// ---------------------------------------------------------------------------
// add_new_part
// ---------------------------------------------------------------------------

/// A parent of an added part: the package or one of the parts of the fixture.
#[derive(Clone, Copy, Debug)]
enum Parent {
  Package,
  Main,
  Header,
  Extended,
}

impl Parent {
  fn part_ref(self, fx: &Fx) -> Option<PartRef> {
    match self {
      Self::Package => None,
      Self::Main => Some(part_ref(&fx.main)),
      Self::Header => Some(part_ref(&fx.header)),
      Self::Extended => Some(part_ref(&fx.extended)),
    }
  }
}

/// Options that the typed `add_new_part*` family can express.
#[derive(Clone, Copy, Debug)]
enum Variant {
  Default,
  ContentType(&'static str),
  ContentTypeAndExtension(&'static str, &'static str),
  ContentTypeAndPath(&'static str, &'static str),
  FixedTarget,
}

impl Variant {
  fn options(self) -> NewPartOptions {
    match self {
      Self::Default => NewPartOptions::default(),
      Self::ContentType(content_type) => NewPartOptions {
        content_type: Some(content_type.into()),
        ..NewPartOptions::default()
      },
      Self::ContentTypeAndExtension(content_type, extension) => NewPartOptions {
        content_type: Some(content_type.into()),
        extension: Some(extension.into()),
        ..NewPartOptions::default()
      },
      Self::ContentTypeAndPath(content_type, path) => NewPartOptions {
        content_type: Some(content_type.into()),
        path: Some(path.to_string()),
        ..NewPartOptions::default()
      },
      Self::FixedTarget => NewPartOptions {
        target_mode: NewPartTargetMode::Fixed,
        ..NewPartOptions::default()
      },
    }
  }
}

/// The typed method that `variant` stands for, or `None` if the typed API has
/// no such method for this parent.
fn typed_add_new<T: SdkPart>(fx: &mut Fx, parent: Parent, variant: Variant) -> Option<String> {
  let outcome = match (parent, variant) {
    (Parent::Package, Variant::Default) => {
      let result = SdkPackage::add_new_part_auto_id::<T>(&mut fx.doc);
      show(result, describe_typed::<T>(&fx.doc))
    }
    (Parent::Package, Variant::ContentType(content_type)) => {
      let result =
        SdkPackage::add_new_part_with_content_type_auto_id::<T>(&mut fx.doc, content_type);
      show(result, describe_typed::<T>(&fx.doc))
    }
    (Parent::Package, Variant::ContentTypeAndExtension(content_type, extension)) => {
      let id = fx.doc.relationships().next_relationship_id();
      let result = SdkPackage::add_new_part_with_content_type_and_extension::<T>(
        &mut fx.doc,
        id,
        content_type,
        extension,
        NewPartTargetMode::Indexed,
      );
      show(result, describe_typed::<T>(&fx.doc))
    }
    (Parent::Package, Variant::FixedTarget) => {
      let id = fx.doc.relationships().next_relationship_id();
      let result =
        SdkPackage::add_new_part_with_target_mode::<T>(&mut fx.doc, id, NewPartTargetMode::Fixed);
      show(result, describe_typed::<T>(&fx.doc))
    }
    (Parent::Package, Variant::ContentTypeAndPath(..)) | (_, Variant::FixedTarget) => return None,
    (_, Variant::Default) => {
      let parent = typed_parent(fx, parent);
      let result = parent.add_new_part_auto_id::<Doc, T>(&mut fx.doc);
      show(result, describe_typed::<T>(&fx.doc))
    }
    (_, Variant::ContentType(content_type)) => {
      let parent = typed_parent(fx, parent);
      let result =
        parent.add_new_part_with_content_type_auto_id::<Doc, T>(&mut fx.doc, content_type);
      show(result, describe_typed::<T>(&fx.doc))
    }
    (_, Variant::ContentTypeAndExtension(content_type, extension)) => {
      let parent = typed_parent(fx, parent);
      let result = parent.add_new_part_with_content_type_and_extension_auto_id::<Doc, T>(
        &mut fx.doc,
        content_type,
        extension,
      );
      show(result, describe_typed::<T>(&fx.doc))
    }
    (_, Variant::ContentTypeAndPath(content_type, path)) => {
      let parent = typed_parent(fx, parent);
      let id = parent.next_relationship_id(&fx.doc).expect("next id");
      let result = parent.add_new_part_with_content_type_and_path::<Doc, T>(
        &mut fx.doc,
        id,
        content_type,
        path,
      );
      show(result, describe_typed::<T>(&fx.doc))
    }
  };
  Some(outcome)
}

/// A boxed typed parent, so that the typed side can call trait methods on it.
fn typed_parent(fx: &Fx, parent: Parent) -> TypedParent {
  match parent {
    Parent::Package => unreachable!("the package is not a part"),
    Parent::Main => TypedParent::Main(fx.main.clone()),
    Parent::Header => TypedParent::Header(fx.header.clone()),
    Parent::Extended => TypedParent::Extended(fx.extended.clone()),
  }
}

enum TypedParent {
  Main(MainDocumentPart),
  Header(HeaderPart),
  Extended(ExtendedPart),
}

macro_rules! delegate {
  ($($method:ident<$($generic:ident),*>($($arg:ident: $ty:ty),*) -> $ret:ty;)*) => {
    impl TypedParent {
      $(
        fn $method<$($generic: SdkPackage + 'static,)* T: SdkPart>(
          &self,
          $($arg: $ty),*
        ) -> $ret {
          match self {
            Self::Main(part) => SdkPart::$method::<$($generic,)* T>(part, $($arg),*),
            Self::Header(part) => SdkPart::$method::<$($generic,)* T>(part, $($arg),*),
            Self::Extended(part) => SdkPart::$method::<$($generic,)* T>(part, $($arg),*),
          }
        }
      )*
    }
  };
}

delegate! {
  add_new_part_auto_id<P>(package: &mut P) -> Result<T, SdkError>;
  add_new_part_with_content_type_auto_id<P>(
    package: &mut P,
    content_type: &'static str
  ) -> Result<T, SdkError>;
  add_new_part_with_content_type_and_extension_auto_id<P>(
    package: &mut P,
    content_type: &'static str,
    extension: &'static str
  ) -> Result<T, SdkError>;
}

impl TypedParent {
  fn next_relationship_id<P: SdkPackage>(&self, package: &P) -> Result<String, SdkError> {
    match self {
      Self::Main(part) => SdkPart::next_relationship_id(part, package),
      Self::Header(part) => SdkPart::next_relationship_id(part, package),
      Self::Extended(part) => SdkPart::next_relationship_id(part, package),
    }
  }

  fn add_new_part_with_content_type_and_path<P: SdkPackage, T: SdkPart>(
    &self,
    package: &mut P,
    relationship_id: String,
    content_type: &'static str,
    path: &'static str,
  ) -> Result<T, SdkError> {
    match self {
      Self::Main(part) => SdkPart::add_new_part_with_content_type_and_path::<P, T>(
        part,
        package,
        relationship_id,
        content_type,
        path,
      ),
      Self::Header(part) => SdkPart::add_new_part_with_content_type_and_path::<P, T>(
        part,
        package,
        relationship_id,
        content_type,
        path,
      ),
      Self::Extended(part) => SdkPart::add_new_part_with_content_type_and_path::<P, T>(
        part,
        package,
        relationship_id,
        content_type,
        path,
      ),
    }
  }
}

fn dynamic_add_new<T: SdkPart>(fx: &mut Fx, parent: Parent, variant: Variant) -> String {
  let parent_ref = parent.part_ref(fx);
  let parent = parent_ref
    .as_ref()
    .map_or(PartParent::Package, PartParent::from);
  let id = parent.next_relationship_id(&fx.doc).expect("next id");
  let result = parent.add_new_part(&mut fx.doc, T::KIND, id, variant.options());
  show(result, describe_ref(&fx.doc))
}

fn check_add_new<T: SdkPart>(parent: Parent, variant: Variant) {
  let label = format!("{:?} under {parent:?} with {variant:?}", T::KIND);
  let mut typed_fx = Fx::new();
  let Some(typed_outcome) = typed_add_new::<T>(&mut typed_fx, parent, variant) else {
    // The typed API cannot express this combination; the run-time API rejects
    // it instead (see `new_part_options_reject_unsupported_combinations`).
    return;
  };
  let mut dynamic_fx = Fx::new();
  let dynamic_outcome = dynamic_add_new::<T>(&mut dynamic_fx, parent, variant);
  assert_eq!(typed_outcome, dynamic_outcome, "{label}: outcome");
  assert_eq!(
    dump(&typed_fx.doc),
    dump(&dynamic_fx.doc),
    "{label}: package"
  );
}

const PARENTS: [Parent; 4] = [
  Parent::Package,
  Parent::Main,
  Parent::Header,
  Parent::Extended,
];

#[test]
fn add_new_part_matches_typed_for_representative_pairs() {
  let variants = [
    Variant::Default,
    Variant::ContentType(IMAGE_TYPE),
    Variant::ContentType("text/plain"),
    Variant::ContentTypeAndExtension(IMAGE_TYPE, ".png"),
    Variant::ContentTypeAndPath(IMAGE_TYPE, "/word/media/custom.png"),
    Variant::FixedTarget,
  ];
  for parent in PARENTS {
    for variant in variants {
      // Allowed pairs, disallowed pairs and pairs whose content type may not
      // match, so that errors are compared as well as successes.
      check_add_new::<MainDocumentPart>(parent, variant);
      check_add_new::<CoreFilePropertiesPart>(parent, variant);
      check_add_new::<HeaderPart>(parent, variant);
      check_add_new::<FooterPart>(parent, variant);
      check_add_new::<ImagePart>(parent, variant);
      check_add_new::<ThemePart>(parent, variant);
      check_add_new::<WorksheetPart>(parent, variant);
      check_add_new::<ExtendedPart>(parent, variant);
    }
  }
}

#[test]
fn add_new_part_main_part_content_type_follows_the_package() {
  // A package parent resolves the main part content type from the package,
  // as `add_new_part` does; a template package yields a template main part.
  for document_type in [
    WordprocessingDocumentType::Document,
    WordprocessingDocumentType::Template,
    WordprocessingDocumentType::MacroEnabledDocument,
  ] {
    let mut typed_doc = Doc::create(document_type);
    let typed = typed_doc
      .add_new_part_auto_id::<MainDocumentPart>()
      .expect("typed main part");
    let mut dynamic_doc = Doc::create(document_type);
    let id = dynamic_doc.relationships().next_relationship_id();
    let dynamic = PartParent::Package
      .add_new_part(
        &mut dynamic_doc,
        PartKind::MainDocumentPart,
        id,
        NewPartOptions::default(),
      )
      .expect("dynamic main part");
    assert_eq!(
      describe_part(&typed_doc, &part_ref(&typed)),
      describe_part(&dynamic_doc, &dynamic),
      "{document_type:?}"
    );
    assert_eq!(dump(&typed_doc), dump(&dynamic_doc), "{document_type:?}");
  }
}

#[test]
fn add_new_part_fails_for_extended_part_like_the_typed_api() {
  for parent in PARENTS {
    check_add_new::<ExtendedPart>(parent, Variant::Default);
  }
  let mut fx = Fx::new();
  let id = fx.doc.relationships().next_relationship_id();
  let error = PartParent::Package
    .add_new_part(
      &mut fx.doc,
      PartKind::ExtendedPart,
      id,
      NewPartOptions::default(),
    )
    .expect_err("an extended part has no relationship type");
  let typed_error = Fx::new()
    .doc
    .add_new_part_auto_id::<ExtendedPart>()
    .expect_err("an extended part has no relationship type");
  assert_eq!(format!("{error:?}"), format!("{typed_error:?}"));
}

#[test]
fn new_part_options_reject_unsupported_combinations() {
  let mut fx = Fx::new();
  let main = part_ref(&fx.main);
  let before = dump(&fx.doc);

  let error = PartParent::Package
    .add_new_part(
      &mut fx.doc,
      PartKind::CoreFilePropertiesPart,
      "rIdPath",
      Variant::ContentTypeAndPath(CORE_TYPE, "/docProps/other.xml").options(),
    )
    .expect_err("a package parent takes no explicit path");
  assert!(matches!(error, SdkError::CommonError(_)), "{error:?}");

  let error = PartParent::Part(&main)
    .add_new_part(
      &mut fx.doc,
      PartKind::HeaderPart,
      "rIdFixed",
      Variant::FixedTarget.options(),
    )
    .expect_err("a part parent takes no fixed target mode");
  assert!(matches!(error, SdkError::CommonError(_)), "{error:?}");

  let mut options = Variant::ContentTypeAndPath(IMAGE_TYPE, "/word/media/custom.png").options();
  options.extension = Some(".png".into());
  let error = PartParent::Part(&main)
    .add_new_part(&mut fx.doc, PartKind::ImagePart, "rIdBoth", options)
    .expect_err("a path and an extension cannot be combined");
  assert!(
    matches!(&error, SdkError::CommonError(message) if message.contains("extension")),
    "{error:?}"
  );

  assert_eq!(
    dump(&fx.doc),
    before,
    "a rejected combination changes nothing"
  );
}

#[test]
fn next_relationship_id_matches_the_typed_methods() {
  let fx = Fx::new();
  let package = PartParent::Package
    .next_relationship_id(&fx.doc)
    .expect("package id");
  assert_eq!(package, fx.doc.relationships().next_relationship_id());
  let main = part_ref(&fx.main);
  assert_eq!(
    PartParent::Part(&main)
      .next_relationship_id(&fx.doc)
      .expect("part id"),
    SdkPart::next_relationship_id(&fx.main, &fx.doc).expect("typed part id")
  );
  let stale = part_ref(&fx.header_stale);
  assert_eq!(
    format!(
      "{:?}",
      PartParent::Part(&stale).next_relationship_id(&fx.doc)
    ),
    format!(
      "{:?}",
      SdkPart::next_relationship_id(&fx.header_stale, &fx.doc)
    ),
  );
  let foreign = part_ref(&fx.header_foreign);
  assert_eq!(
    format!(
      "{:?}",
      PartParent::Part(&foreign).next_relationship_id(&fx.doc)
    ),
    format!(
      "{:?}",
      SdkPart::next_relationship_id(&fx.header_foreign, &fx.doc)
    ),
  );
}

// ---------------------------------------------------------------------------
// Relationships between existing parts
// ---------------------------------------------------------------------------

/// Picks the child handle for a state; see `Fx`.
type Pick = fn(&Fx, State) -> ImagePart;

/// The image that `header` already references, or the free one when valid.
fn free_image(fx: &Fx, state: State) -> ImagePart {
  match state {
    State::Valid => fx.image_free.clone(),
    other => fx.image(other),
  }
}

/// Runs `typed` and `dynamic` for every state of a `HeaderPart` parent and of
/// an `ImagePart` child, on fresh fixtures, and requires the same outcome and
/// the same package.
fn for_each_state_pair(
  label: &str,
  child: Pick,
  typed: impl Fn(&mut Fx, HeaderPart, ImagePart) -> String,
  dynamic: impl Fn(&mut Fx, &PartRef, &PartRef) -> String,
) {
  for parent_state in STATES {
    for child_state in STATES {
      assert_same(
        &format!("{label}: header {parent_state:?} / image {child_state:?}"),
        |fx| {
          let (parent, child) = (fx.header(parent_state), child(fx, child_state));
          typed(fx, parent, child)
        },
        |fx| {
          let parent = part_ref(&fx.header(parent_state));
          let child = part_ref(&child(fx, child_state));
          dynamic(fx, &parent, &child)
        },
      );
    }
  }
}

/// Like `for_each_state_pair`, with the package as the parent.
fn for_each_state(
  label: &str,
  child: Pick,
  typed: impl Fn(&mut Fx, ImagePart) -> String,
  dynamic: impl Fn(&mut Fx, &PartRef) -> String,
) {
  for state in STATES {
    assert_same(
      &format!("{label}: package / image {state:?}"),
      |fx| {
        let child = child(fx, state);
        typed(fx, child)
      },
      |fx| {
        let child = part_ref(&child(fx, state));
        dynamic(fx, &child)
      },
    );
  }
}

fn id_string(result: Result<String, SdkError>) -> String {
  show(result, |id| id.clone())
}

fn flag_string(result: Result<bool, SdkError>) -> String {
  show(result, |deleted| deleted.to_string())
}

#[test]
fn id_of_part_matches_typed_in_every_state() {
  for_each_state_pair(
    "id_of_part",
    Fx::image,
    |fx, parent, child| id_string(parent.get_id_of_part(&fx.doc, &child).map(str::to_string)),
    |fx, parent, child| {
      id_string(
        PartParent::Part(parent)
          .id_of_part(&fx.doc, child)
          .map(str::to_string),
      )
    },
  );
  for_each_state(
    "id_of_part",
    Fx::image,
    |fx, child| id_string(fx.doc.get_id_of_part(&child).map(str::to_string)),
    |fx, child| {
      id_string(
        PartParent::Package
          .id_of_part(&fx.doc, child)
          .map(str::to_string),
      )
    },
  );
  // `main` is the one package child that is referenced.
  assert_same(
    "id_of_part: package / main",
    |fx| id_string(fx.doc.get_id_of_part(&fx.main).map(str::to_string)),
    |fx| {
      let main = part_ref(&fx.main);
      id_string(
        PartParent::Package
          .id_of_part(&fx.doc, &main)
          .map(str::to_string),
      )
    },
  );
}

/// `delete_part` is `id_of_part` followed by `delete_part_by_id`, which does
/// not depend on the child type.
#[test]
fn delete_part_is_composed_from_id_of_part_and_delete_part_by_id() {
  for_each_state_pair(
    "delete_part",
    Fx::image,
    |fx, parent, child| flag_string(parent.delete_part(&mut fx.doc, child)),
    |fx, parent, child| {
      let id = PartParent::Part(parent)
        .id_of_part(&fx.doc, child)
        .map(str::to_string);
      // `delete_part_by_id` depends on the parent type only.
      let typed_parent =
        <HeaderPart as crate::private::SdkPartHandle>::from_part_key(parent.part_key());
      flag_string(id.and_then(|id| typed_parent.delete_part_by_id(&mut fx.doc, &id)))
    },
  );
  for_each_state(
    "delete_part",
    Fx::image,
    |fx, child| flag_string(fx.doc.delete_part(child)),
    |fx, child| {
      let id = PartParent::Package
        .id_of_part(&fx.doc, child)
        .map(str::to_string);
      flag_string(id.and_then(|id| fx.doc.delete_part_by_id(&id)))
    },
  );
  assert_same(
    "delete_part: package / main",
    |fx| flag_string(fx.doc.delete_part(fx.main.clone())),
    |fx| {
      let main = part_ref(&fx.main);
      let id = PartParent::Package
        .id_of_part(&fx.doc, &main)
        .map(str::to_string);
      flag_string(id.and_then(|id| fx.doc.delete_part_by_id(&id)))
    },
  );
  // `delete_parts` resolves every id before it deletes anything.
  for (label, with_stale) in [("valid", false), ("with a stale part", true)] {
    assert_same(
      &format!("delete_parts: main / {label}"),
      |fx| {
        let mut images = vec![fx.image_free.clone()];
        if with_stale {
          images.push(fx.image_stale.clone());
        }
        let main = fx.main.clone();
        show(main.delete_parts(&mut fx.doc, images), |()| String::new())
      },
      |fx| {
        let main = part_ref(&fx.main);
        let mut images = vec![part_ref(&fx.image_free)];
        if with_stale {
          images.push(part_ref(&fx.image_stale));
        }
        let parent = PartParent::Part(&main);
        let ids = images
          .iter()
          .map(|image| parent.id_of_part(&fx.doc, image).map(str::to_string))
          .collect::<Result<Vec<_>, _>>();
        let main = fx.main.clone();
        show(
          ids.and_then(|ids| {
            ids
              .iter()
              .try_for_each(|id| main.delete_part_by_id(&mut fx.doc, id).map(|_| ()))
          }),
          |()| String::new(),
        )
      },
    );
  }
}

#[test]
fn change_id_of_part_matches_typed_in_every_state() {
  for_each_state_pair(
    "change_id_of_part",
    Fx::image,
    |fx, parent, child| id_string(parent.change_id_of_part(&mut fx.doc, &child, "rIdRenamed")),
    |fx, parent, child| {
      id_string(PartParent::Part(parent).change_id_of_part(&mut fx.doc, child, "rIdRenamed"))
    },
  );
  for_each_state(
    "change_id_of_part",
    Fx::image,
    |fx, child| id_string(fx.doc.change_id_of_part(&child, "rIdRenamed")),
    |fx, child| id_string(PartParent::Package.change_id_of_part(&mut fx.doc, child, "rIdRenamed")),
  );
  // A new id that is already taken.
  assert_same(
    "change_id_of_part: taken id",
    |fx| {
      let (header, image) = (fx.header.clone(), fx.image.clone());
      id_string(header.change_id_of_part(&mut fx.doc, &image, "rIdImage"))
    },
    |fx| {
      let (header, image) = (part_ref(&fx.header), part_ref(&fx.image));
      id_string(PartParent::Part(&header).change_id_of_part(&mut fx.doc, &image, "rIdImage"))
    },
  );
  assert_same(
    "change_id_of_part: package / main",
    |fx| id_string(fx.doc.change_id_of_part(&fx.main.clone(), "rIdMain")),
    |fx| {
      let main = part_ref(&fx.main);
      id_string(PartParent::Package.change_id_of_part(&mut fx.doc, &main, "rIdMain"))
    },
  );
}

/// A package whose main part references one image through two relationships.
fn package_with_ambiguous_image() -> Vec<u8> {
  use std::io::Write as _;
  let options = zip::write::SimpleFileOptions::default();
  let mut zip = zip::ZipWriter::new(std::io::Cursor::new(Vec::new()));
  let mut file = |name: &str, content: &str| {
    zip.start_file(name, options).expect("start entry");
    zip.write_all(content.as_bytes()).expect("write entry");
  };
  file(
    "[Content_Types].xml",
    r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"><Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/><Default Extension="xml" ContentType="application/xml"/><Default Extension="png" ContentType="image/png"/><Override PartName="/word/document.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"/></Types>"#,
  );
  file(
    "_rels/.rels",
    r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="word/document.xml"/></Relationships>"#,
  );
  file(
    "word/_rels/document.xml.rels",
    r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/image" Target="media/image1.png"/><Relationship Id="rId2" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/image" Target="media/image1.png"/></Relationships>"#,
  );
  file(
    "word/document.xml",
    "<w:document xmlns:w=\"http://schemas.openxmlformats.org/wordprocessingml/2006/main\"/>",
  );
  file("word/media/image1.png", "png");
  zip.finish().expect("finish the archive").into_inner()
}

#[test]
fn change_id_of_part_reports_ambiguous_relationships_like_typed() {
  let bytes = package_with_ambiguous_image();
  let mut typed_doc = Doc::new(std::io::Cursor::new(bytes.clone())).expect("open");
  let mut dynamic_doc = Doc::new(std::io::Cursor::new(bytes)).expect("open");
  let handles = |doc: &Doc| {
    let main = doc.main_document_part().expect("main part");
    let image = main
      .get_parts_of_type::<Doc, ImagePart>(doc)
      .next()
      .expect("image");
    (main, image)
  };

  let (main, image) = handles(&typed_doc);
  let typed = main.change_id_of_part(&mut typed_doc, &image, "rIdNew");
  let (main, image) = handles(&dynamic_doc);
  let (main, image) = (part_ref(&main), part_ref(&image));
  let dynamic = PartParent::Part(&main).change_id_of_part(&mut dynamic_doc, &image, "rIdNew");

  let typed = format!("{typed:?}");
  assert!(typed.contains("AmbiguousPartRelationship"), "{typed}");
  assert_eq!(typed, format!("{dynamic:?}"));
  assert_eq!(dump(&typed_doc), dump(&dynamic_doc));
}

#[test]
fn create_relationship_to_part_matches_typed_in_every_state() {
  // Both the referenced image and a free one, so that creating a new
  // relationship and returning an existing one are both covered.
  for (name, child) in [
    ("referenced", Fx::image as Pick),
    ("free", free_image as Pick),
  ] {
    for_each_state_pair(
      &format!("create_relationship_to_part {name}"),
      child,
      |fx, parent, child| id_string(parent.create_relationship_to_part(&mut fx.doc, child)),
      |fx, parent, child| {
        id_string(PartParent::Part(parent).create_relationship_to_part(&mut fx.doc, child))
      },
    );
    for_each_state_pair(
      &format!("create_relationship_to_part_with_id {name}"),
      child,
      |fx, parent, child| {
        id_string(parent.create_relationship_to_part_with_id(&mut fx.doc, child, "rIdNew"))
      },
      |fx, parent, child| {
        id_string(
          PartParent::Part(parent).create_relationship_to_part_with_id(
            &mut fx.doc,
            child,
            "rIdNew",
          ),
        )
      },
    );
    // `add_part` and `add_part_with_id` return the part instead of the id.
    for_each_state_pair(
      &format!("add_part {name}"),
      child,
      |fx, parent, child| {
        show(parent.add_part(&mut fx.doc, child), |part| {
          describe_part(&fx.doc, &part_ref(part))
        })
      },
      |fx, parent, child| {
        let id = PartParent::Part(parent).create_relationship_to_part(&mut fx.doc, child);
        show(id, |_| describe_part(&fx.doc, child))
      },
    );
    for_each_state_pair(
      &format!("add_part_with_id {name}"),
      child,
      |fx, parent, child| {
        show(
          parent.add_part_with_id(&mut fx.doc, child, "rIdNew"),
          |part| describe_part(&fx.doc, &part_ref(part)),
        )
      },
      |fx, parent, child| {
        let id = PartParent::Part(parent).create_relationship_to_part_with_id(
          &mut fx.doc,
          child,
          "rIdNew",
        );
        show(id, |_| describe_part(&fx.doc, child))
      },
    );
  }
}

#[test]
fn package_relationship_to_part_matches_typed() {
  // The package allows no image; `main` is allowed and already related.
  for (name, child) in [
    ("referenced", Fx::image as Pick),
    ("free", free_image as Pick),
  ] {
    for_each_state(
      &format!("create_relationship_to_part {name}"),
      child,
      |fx, child| id_string(fx.doc.create_relationship_to_part(child)),
      |fx, child| id_string(PartParent::Package.create_relationship_to_part(&mut fx.doc, child)),
    );
    for_each_state(
      &format!("create_relationship_to_part_with_id {name}"),
      child,
      |fx, child| id_string(fx.doc.create_relationship_to_part_with_id(child, "rIdNew")),
      |fx, child| {
        id_string(PartParent::Package.create_relationship_to_part_with_id(
          &mut fx.doc,
          child,
          "rIdNew",
        ))
      },
    );
  }
  assert_same(
    "create_relationship_to_part: package / main again",
    |fx| id_string(fx.doc.create_relationship_to_part(fx.main.clone())),
    |fx| {
      let main = part_ref(&fx.main);
      id_string(PartParent::Package.create_relationship_to_part(&mut fx.doc, &main))
    },
  );
  assert_same(
    "add_part_with_id: package / main with a second id",
    |fx| {
      show(
        fx.doc.add_part_with_id(fx.main.clone(), "rIdSecondMain"),
        |part| describe_part(&fx.doc, &part_ref(part)),
      )
    },
    |fx| {
      let main = part_ref(&fx.main);
      let id = PartParent::Package.create_relationship_to_part_with_id(
        &mut fx.doc,
        &main,
        "rIdSecondMain",
      );
      show(id, |_| describe_part(&fx.doc, &main))
    },
  );
}

#[test]
fn add_part_from_package_matches_typed_in_every_state() {
  for_each_state_pair(
    "add_part_from_package_with_id",
    Fx::source_image,
    |fx, parent, child| {
      let result =
        parent.add_part_from_package_with_id(&mut fx.doc, &fx.other, &child, "rIdCopied");
      show(result, |part| describe_part(&fx.doc, &part_ref(part)))
    },
    |fx, parent, child| {
      let result = PartParent::Part(parent).add_part_from_package_with_id(
        &mut fx.doc,
        &fx.other,
        child,
        "rIdCopied",
      );
      show(result, describe_ref(&fx.doc))
    },
  );
  for_each_state(
    "add_part_from_package_with_id",
    Fx::source_image,
    |fx, child| {
      let result = fx
        .doc
        .add_part_from_package_with_id(&fx.other, &child, "rIdCopied");
      show(result, |part| describe_part(&fx.doc, &part_ref(part)))
    },
    |fx, child| {
      let result = PartParent::Package.add_part_from_package_with_id(
        &mut fx.doc,
        &fx.other,
        child,
        "rIdCopied",
      );
      show(result, describe_ref(&fx.doc))
    },
  );
  // Auto ids are composed with `next_relationship_id`.
  assert_same(
    "add_part_from_package: package / auto id",
    |fx| {
      let child = fx.other_image.clone();
      let result = fx.doc.add_part_from_package(&fx.other, &child);
      show(result, |part| describe_part(&fx.doc, &part_ref(part)))
    },
    |fx| {
      let child = part_ref(&fx.other_image);
      let id = PartParent::Package
        .next_relationship_id(&fx.doc)
        .expect("next id");
      let result =
        PartParent::Package.add_part_from_package_with_id(&mut fx.doc, &fx.other, &child, id);
      show(result, describe_ref(&fx.doc))
    },
  );
  assert_same(
    "add_part_from_package: part / auto id",
    |fx| {
      let (header, child) = (fx.header.clone(), fx.other_image.clone());
      let result = header.add_part_from_package(&mut fx.doc, &fx.other, &child);
      show(result, |part| describe_part(&fx.doc, &part_ref(part)))
    },
    |fx| {
      let (header, child) = (part_ref(&fx.header), part_ref(&fx.other_image));
      let parent = PartParent::Part(&header);
      let id = parent.next_relationship_id(&fx.doc).expect("next id");
      let result = parent.add_part_from_package_with_id(&mut fx.doc, &fx.other, &child, id);
      show(result, describe_ref(&fx.doc))
    },
  );
}

// ---------------------------------------------------------------------------
// related_parts
// ---------------------------------------------------------------------------

type Listing = Vec<String>;

fn listing<'a>(
  doc: &Doc,
  related: impl Iterator<Item = crate::sdk::RelatedPart<'a, PartRef>>,
) -> Listing {
  related
    .map(|related| {
      format!(
        "{} {} {}",
        related.relationship_id(),
        related.relationship_type(),
        describe_part(doc, &related)
      )
    })
    .collect()
}

fn typed_listing<'a, T: SdkPart>(
  doc: &Doc,
  related: impl Iterator<Item = crate::sdk::RelatedPart<'a, T>>,
) -> Listing {
  related
    .map(|related| {
      format!(
        "{} {} {}",
        related.relationship_id(),
        related.relationship_type(),
        describe_part(doc, &part_ref(&*related))
      )
    })
    .collect()
}

/// A document with relationships of several types, aliases included.
fn aliased_fixture() -> Fx {
  let mut fx = Fx::new();
  let main = fx.main.clone();
  for (relationship_type, content_type) in [
    (REL_IMAGE, IMAGE_TYPE),
    (STRICT_REL_IMAGE, IMAGE_TYPE),
    (REL_THUMBNAIL, "image/jpeg"),
    (STRICT_REL_THUMBNAIL, "image/jpeg"),
    ("urn:example:other", "application/example"),
  ] {
    main
      .add_extended_part(&mut fx.doc, relationship_type, content_type, ".bin")
      .expect("extended part");
  }
  for (relationship_type, content_type) in [
    (REL_CORE, CORE_TYPE),
    (O12_REL_CORE, CORE_TYPE),
    (REL_THUMBNAIL, "image/jpeg"),
    (STRICT_REL_THUMBNAIL, "image/jpeg"),
  ] {
    fx.doc
      .add_extended_part(relationship_type, content_type, ".bin")
      .expect("package extended part");
  }
  fx
}

#[test]
fn related_parts_matches_get_parts_of_type() {
  let fx = aliased_fixture();
  let main = part_ref(&fx.main);
  let parent = PartParent::Part(&main);

  macro_rules! same_kind {
    ($($ty:ty),*) => {$(
      assert_eq!(
        typed_listing(&fx.doc, SdkPart::related_parts_of_type::<Doc, $ty>(&fx.main, &fx.doc)),
        listing(&fx.doc, parent.related_parts(&fx.doc, Some(<$ty>::KIND), Some(<$ty>::RELATIONSHIP_TYPE))),
        "{:?} under main",
        <$ty>::KIND
      );
      assert_eq!(
        typed_listing(&fx.doc, SdkPart::child_related_parts_of_type::<Doc, $ty>(&fx.main, &fx.doc)),
        listing(&fx.doc, parent.related_parts(&fx.doc, Some(<$ty>::KIND), Some(<$ty>::RELATIONSHIP_TYPE))),
        "{:?} under main, child_related",
        <$ty>::KIND
      );
      assert_eq!(
        typed_listing(
          &fx.doc,
          fx.main.get_parts_of_type::<Doc, $ty>(&fx.doc).map(|part| {
            crate::sdk::RelatedPart::new("", "", part)
          })
        )
        .iter()
        .map(|line| line.splitn(3, ' ').nth(2).unwrap_or_default().to_string())
        .collect::<Vec<_>>(),
        listing(&fx.doc, parent.related_parts(&fx.doc, Some(<$ty>::KIND), None))
          .iter()
          .map(|line| line.splitn(3, ' ').nth(2).unwrap_or_default().to_string())
          .collect::<Vec<_>>(),
        "{:?} under main, get_parts_of_type",
        <$ty>::KIND
      );
      assert_eq!(
        typed_listing(&fx.doc, SdkPackage::related_parts_of_type::<$ty>(&fx.doc)),
        listing(&fx.doc, PartParent::Package.related_parts(&fx.doc, Some(<$ty>::KIND), None)),
        "{:?} under the package",
        <$ty>::KIND
      );
    )*};
  }
  same_kind!(
    ImagePart,
    MainDocumentPart,
    CoreFilePropertiesPart,
    crate::parts::thumbnail_part::ThumbnailPart,
    ExtendedPart,
    HeaderPart,
    ThemePart
  );
}

#[test]
fn related_parts_filters_by_relationship_type_with_aliases() {
  let fx = aliased_fixture();
  let main = part_ref(&fx.main);
  let parent = PartParent::Part(&main);

  for relationship_type in [
    REL_IMAGE,
    STRICT_REL_IMAGE,
    REL_THUMBNAIL,
    STRICT_REL_THUMBNAIL,
    "urn:example:other",
    "urn:example:absent",
  ] {
    assert_eq!(
      typed_listing(
        &fx.doc,
        fx.main
          .child_related_parts_by_relationship_type::<Doc, ImagePart>(&fx.doc, relationship_type)
      ),
      listing(
        &fx.doc,
        parent.related_parts(&fx.doc, Some(ImagePart::KIND), Some(relationship_type))
      ),
      "image kind, {relationship_type}"
    );
    assert_eq!(
      typed_listing(
        &fx.doc,
        fx.main
          .child_related_parts_by_relationship_type::<Doc, ExtendedPart>(
            &fx.doc,
            relationship_type
          )
      ),
      listing(
        &fx.doc,
        parent.related_parts(&fx.doc, Some(ExtendedPart::KIND), Some(relationship_type))
      ),
      "extended kind, {relationship_type}"
    );
  }
  assert!(
    !listing(
      &fx.doc,
      parent.related_parts(&fx.doc, Some(ImagePart::KIND), Some(STRICT_REL_IMAGE))
    )
    .is_empty(),
    "the strict alias matches the transitional image type"
  );

  // The package: Office 2007 core properties alias and the thumbnail alias.
  for relationship_type in [REL_CORE, O12_REL_CORE, REL_THUMBNAIL, STRICT_REL_THUMBNAIL] {
    assert_eq!(
      typed_listing(
        &fx.doc,
        fx.doc
          .relationships()
          .part_relationships()
          .filter(|relationship| {
            crate::common::relationship_type_matches_bytes(
              relationship.relationship_type_bytes(),
              relationship_type.as_bytes(),
            )
          })
          .filter_map(|relationship| {
            crate::sdk::relationship_target_as_part::<CoreFilePropertiesPart>(
              fx.doc.storage(),
              relationship,
            )
            .map(|part| {
              crate::sdk::RelatedPart::new(
                relationship.id(),
                relationship.relationship_type(),
                part,
              )
            })
          })
      ),
      listing(
        &fx.doc,
        PartParent::Package.related_parts(
          &fx.doc,
          Some(CoreFilePropertiesPart::KIND),
          Some(relationship_type)
        )
      ),
      "package, {relationship_type}"
    );
  }
}

#[test]
fn related_parts_lists_every_child_and_follows_stale_parents() {
  let fx = aliased_fixture();
  let main = part_ref(&fx.main);

  // All children, in relationship order, with their relationship types.
  let all = listing(
    &fx.doc,
    PartParent::Part(&main).related_parts(&fx.doc, None, None),
  );
  let via_parts = fx
    .main
    .parts(&fx.doc)
    .map(|pair| {
      format!(
        "{} {}",
        pair.relationship_id,
        describe_part(&fx.doc, &pair.part)
      )
    })
    .collect::<Vec<_>>();
  assert_eq!(
    all
      .iter()
      .map(|line| {
        let mut fields = line.splitn(3, ' ');
        format!("{} {}", fields.next().unwrap(), fields.nth(1).unwrap())
      })
      .collect::<Vec<_>>(),
    via_parts
  );
  let package = listing(
    &fx.doc,
    PartParent::Package.related_parts(&fx.doc, None, None),
  );
  assert_eq!(
    package.len(),
    fx.doc.parts().count(),
    "every package child is listed"
  );

  // A parent that no longer resolves has no children, as in the typed API.
  for stale_or_foreign in [part_ref(&fx.header_stale), part_ref(&fx.header_foreign)] {
    assert!(
      PartParent::Part(&stale_or_foreign)
        .related_parts(&fx.doc, None, None)
        .next()
        .is_none()
    );
  }
  assert_eq!(
    SdkPart::child_related_parts_of_type::<Doc, ImagePart>(&fx.header_stale, &fx.doc).count(),
    0
  );
}

#[test]
fn related_parts_uses_the_stored_kind_not_the_parents_constraints() {
  // A worksheet-typed child under a main document part is classified as a
  // worksheet part although the parent does not allow that kind.
  let mut fx = Fx::new();
  let main = fx.main.clone();
  main
    .add_extended_part(
      &mut fx.doc,
      WorksheetPart::RELATIONSHIP_TYPE,
      WorksheetPart::CONTENT_TYPE,
      ".xml",
    )
    .expect("worksheet-typed child");
  let parent_ref = part_ref(&main);
  let parent = PartParent::Part(&parent_ref);

  let found = listing(
    &fx.doc,
    parent.related_parts(&fx.doc, Some(PartKind::WorksheetPart), None),
  );
  assert_eq!(found.len(), 1, "{found:?}");
  assert_eq!(
    typed_listing(
      &fx.doc,
      SdkPart::related_parts_of_type::<Doc, WorksheetPart>(&main, &fx.doc)
    )
    .len(),
    1
  );
  assert!(
    parent
      .related_parts(&fx.doc, Some(PartKind::ExtendedPart), None)
      .all(|related| related.kind() == PartKind::ExtendedPart)
  );

  // The handle carries the stored kind, so the constraint check fails exactly
  // as the typed `add_part_with_id` does.
  let worksheet = SdkPart::related_parts_of_type::<Doc, WorksheetPart>(&main, &fx.doc)
    .next()
    .expect("worksheet child")
    .into_part();
  let worksheet_ref = parent
    .related_parts(&fx.doc, Some(PartKind::WorksheetPart), None)
    .next()
    .expect("worksheet child")
    .into_part();
  let typed = main.add_part_with_id(&mut fx.doc, worksheet, "rIdAgain");
  let dynamic = parent.create_relationship_to_part_with_id(&mut fx.doc, &worksheet_ref, "rIdAgain");
  assert_eq!(
    typed.map(|_| ()).map_err(|error| format!("{error:?}")),
    dynamic.map(|_| ()).map_err(|error| format!("{error:?}")),
  );
}

// ---------------------------------------------------------------------------
// Extended parts, deletion
// ---------------------------------------------------------------------------

#[test]
fn extended_parts_as_parents_and_children_match_typed() {
  // An extended part accepts any child; any parent accepts an extended child.
  check_add_new::<HeaderPart>(Parent::Extended, Variant::Default);
  check_add_new::<ImagePart>(Parent::Extended, Variant::ContentType(IMAGE_TYPE));
  check_add_new::<WorksheetPart>(Parent::Extended, Variant::Default);

  // A relationship to an existing extended part keeps its relationship type.
  for parent_state in STATES {
    assert_same(
      &format!("header {parent_state:?} -> extended"),
      |fx| {
        let (parent, child) = (fx.header(parent_state), fx.extended.clone());
        show(
          parent.create_relationship_to_part_with_id(&mut fx.doc, child, "rIdExt"),
          |id| id.clone(),
        )
      },
      |fx| {
        let (parent, child) = (part_ref(&fx.header(parent_state)), part_ref(&fx.extended));
        show(
          PartParent::Part(&parent).create_relationship_to_part_with_id(
            &mut fx.doc,
            &child,
            "rIdExt",
          ),
          |id| id.clone(),
        )
      },
    );
  }
  assert_same(
    "extended -> header",
    |fx| {
      let (parent, child) = (fx.extended.clone(), fx.header.clone());
      show(
        parent.create_relationship_to_part_with_id(&mut fx.doc, child, "rIdHeader"),
        |id| id.clone(),
      )
    },
    |fx| {
      let (parent, child) = (part_ref(&fx.extended), part_ref(&fx.header));
      show(
        PartParent::Part(&parent).create_relationship_to_part_with_id(
          &mut fx.doc,
          &child,
          "rIdHeader",
        ),
        |id| id.clone(),
      )
    },
  );
  assert_same(
    "package -> extended",
    |fx| {
      let child = fx.extended.clone();
      show(
        fx.doc.create_relationship_to_part_with_id(child, "rIdExt"),
        |id| id.clone(),
      )
    },
    |fx| {
      let child = part_ref(&fx.extended);
      show(
        PartParent::Package.create_relationship_to_part_with_id(&mut fx.doc, &child, "rIdExt"),
        |id| id.clone(),
      )
    },
  );
}

// ---------------------------------------------------------------------------
// Root element slots
// ---------------------------------------------------------------------------

/// Every part, deleted ones included, must have a slot in the root element
/// cache; a package that forgot `push_root_element_slot` for a new part would
/// fail to cache its root element.
///
/// This detects a missing slot only. The cache length is not public, so an
/// extra `push_root_element_slot` goes unnoticed.
fn assert_root_slots(label: &str, doc: &Doc) {
  for index in 0..doc.storage().parts().len() {
    let element = crate::parts::PartRootElement::HeaderPart(Box::default());
    assert!(
      doc
        .cache_root_element(PartSlot::from_index(index), element)
        .is_some(),
      "{label}: no root element slot for part {index}"
    );
  }
}

fn check_root_slots_after_add_new<T: SdkPart>(parent: Parent, variant: Variant) {
  let label = format!("{:?} under {parent:?}", T::KIND);
  let mut typed_fx = Fx::new();
  let typed = typed_add_new::<T>(&mut typed_fx, parent, variant).expect("a typed method");
  let mut dynamic_fx = Fx::new();
  let dynamic = dynamic_add_new::<T>(&mut dynamic_fx, parent, variant);
  assert!(typed.starts_with("Ok("), "{label}: {typed}");
  assert_eq!(typed, dynamic, "{label}");
  assert_root_slots(&format!("{label} (typed)"), &typed_fx.doc);
  assert_root_slots(&format!("{label} (dynamic)"), &dynamic_fx.doc);
}

#[test]
fn add_new_part_gives_the_new_part_a_root_element_slot() {
  check_root_slots_after_add_new::<CoreFilePropertiesPart>(Parent::Package, Variant::Default);
  check_root_slots_after_add_new::<HeaderPart>(Parent::Main, Variant::Default);
  check_root_slots_after_add_new::<ImagePart>(Parent::Header, Variant::ContentType(IMAGE_TYPE));
  check_root_slots_after_add_new::<HeaderPart>(Parent::Extended, Variant::Default);
  check_root_slots_after_add_new::<HeaderPart>(
    Parent::Main,
    Variant::ContentTypeAndPath(
      "application/vnd.openxmlformats-officedocument.wordprocessingml.header+xml",
      "/word/custom-header.xml",
    ),
  );
}

#[test]
fn importing_a_part_tree_gives_every_imported_part_a_root_element_slot() {
  // The foreign header brings its image along: two new parts.
  let mut typed_fx = Fx::new();
  let before = typed_fx.doc.storage().parts().len();
  let (main, header) = (typed_fx.main.clone(), typed_fx.header_foreign.clone());
  let typed =
    main.add_part_from_package_with_id(&mut typed_fx.doc, &typed_fx.other, &header, "rIdTree");
  let typed = show(typed, |part| describe_part(&typed_fx.doc, &part_ref(part)));

  let mut dynamic_fx = Fx::new();
  let (main, header) = (
    part_ref(&dynamic_fx.main),
    part_ref(&dynamic_fx.header_foreign),
  );
  let dynamic = PartParent::Part(&main).add_part_from_package_with_id(
    &mut dynamic_fx.doc,
    &dynamic_fx.other,
    &header,
    "rIdTree",
  );
  let dynamic = show(dynamic, describe_ref(&dynamic_fx.doc));

  assert!(typed.starts_with("Ok("), "{typed}");
  assert_eq!(typed, dynamic);
  assert_eq!(dump(&typed_fx.doc), dump(&dynamic_fx.doc));
  assert_eq!(typed_fx.doc.storage().parts().len(), before + 2);
  assert_root_slots("import (typed)", &typed_fx.doc);
  assert_root_slots("import (dynamic)", &dynamic_fx.doc);
}

// ---------------------------------------------------------------------------
// Edge cases
// ---------------------------------------------------------------------------

#[test]
fn a_part_can_reference_itself_like_in_the_typed_api() {
  assert_same(
    "extended part -> itself",
    |fx| {
      let part = fx.extended.clone();
      id_string(part.create_relationship_to_part_with_id(&mut fx.doc, part.clone(), "rIdSelf"))
    },
    |fx| {
      let part = part_ref(&fx.extended);
      id_string(PartParent::Part(&part).create_relationship_to_part_with_id(
        &mut fx.doc,
        &part,
        "rIdSelf",
      ))
    },
  );
  assert_same(
    "extended part -> itself, existing relationship",
    |fx| {
      let part = fx.extended.clone();
      part
        .create_relationship_to_part_with_id(&mut fx.doc, part.clone(), "rIdSelf")
        .expect("first self relationship");
      id_string(part.create_relationship_to_part(&mut fx.doc, part.clone()))
    },
    |fx| {
      let part = part_ref(&fx.extended);
      let parent = PartParent::Part(&part);
      parent
        .create_relationship_to_part_with_id(&mut fx.doc, &part, "rIdSelf")
        .expect("first self relationship");
      id_string(parent.create_relationship_to_part(&mut fx.doc, &part))
    },
  );
}

#[test]
fn an_empty_package_has_no_children_like_in_the_typed_api() {
  let fx = Fx::new();
  let empty = Doc::create(WordprocessingDocumentType::Document);

  assert_eq!(
    SdkPackage::related_parts_of_type::<MainDocumentPart>(&empty).count(),
    0
  );
  assert_eq!(
    PartParent::Package
      .related_parts(&empty, Some(PartKind::MainDocumentPart), None)
      .count(),
    0
  );
  assert_eq!(
    PartParent::Package
      .related_parts(&empty, None, None)
      .count(),
    empty.parts().count()
  );

  // Handles of another package are foreign to the empty one.
  let main = part_ref(&fx.main);
  assert_eq!(
    id_string(empty.get_id_of_part(&fx.main).map(str::to_string)),
    id_string(
      PartParent::Package
        .id_of_part(&empty, &main)
        .map(str::to_string)
    ),
  );
  let header = part_ref(&fx.header);
  assert_eq!(
    id_string(
      fx.header
        .get_id_of_part(&empty, &fx.image)
        .map(str::to_string)
    ),
    id_string(
      PartParent::Part(&header)
        .id_of_part(&empty, &part_ref(&fx.image))
        .map(str::to_string)
    ),
  );
}

#[test]
fn results_do_not_borrow_the_parent_handle() {
  let fx = Fx::new();
  // The parent and child handles are temporaries; only `fx.doc` stays borrowed
  // by the results, as with the typed methods. Compiling is the check.
  let (related, id) = {
    let parent = part_ref(&fx.header);
    let child = part_ref(&fx.image);
    let related = PartParent::Part(&parent).related_parts(&fx.doc, None, None);
    let id = PartParent::Part(&parent).id_of_part(&fx.doc, &child);
    (related, id)
  };
  assert_eq!(related.count(), 1);
  assert_eq!(id.expect("the header references the image"), "rIdImage");
}
