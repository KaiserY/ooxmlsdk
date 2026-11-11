//! Pins the behavior of the typed helpers that the run-time child-part API
//! shares with the typed API, so that factoring them out cannot change it.

use super::{
  SdkPackage, SdkPart, SdkPartDescriptor, default_main_part_content_type,
  typed_main_part_content_type, validate_child_part_constraint,
};
use crate::common::SdkError;
use crate::parts::PartRef;
use crate::parts::core_file_properties_part::CoreFilePropertiesPart;
use crate::parts::extended_part::ExtendedPart;
use crate::parts::footer_part::FooterPart;
use crate::parts::header_part::HeaderPart;
use crate::parts::main_document_part::MainDocumentPart;
use crate::parts::presentation_part::PresentationPart;
use crate::parts::theme_part::ThemePart;
use crate::parts::visio_document_part::VisioDocumentPart;
use crate::parts::wordprocessing_document::WordprocessingDocument;
use crate::parts::workbook_part::WorkbookPart;
use crate::parts::worksheet_part::WorksheetPart;
use crate::sdk::{
  PartConstraint, PresentationDocumentType, SpreadsheetDocumentType, WordprocessingDocumentType,
};

const WORD_MAIN: &str =
  "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml";
const WORD_TEMPLATE_MAIN: &str =
  "application/vnd.openxmlformats-officedocument.wordprocessingml.template.main+xml";
const SHEET_MAIN: &str =
  "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml";
const PRESENTATION_MAIN: &str =
  "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml";
const CORE_PROPERTIES_TYPE: &str = "application/vnd.openxmlformats-package.core-properties+xml";
const CORE_PROPERTIES_RELATIONSHIP: &str =
  "http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties";
const THEME_TYPE: &str = "application/vnd.openxmlformats-officedocument.theme+xml";
const THEME_RELATIONSHIP: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme";

fn message(error: SdkError) -> String {
  match error {
    SdkError::CommonError(message) => message,
    other => panic!("expected CommonError, got {other:?}"),
  }
}

fn word_document() -> (WordprocessingDocument, MainDocumentPart) {
  let mut document = WordprocessingDocument::create(WordprocessingDocumentType::Document);
  let main = document.add_main_document_part().expect("main part");
  (document, main)
}

#[test]
fn validate_child_part_constraint_messages() {
  let (mut document, main) = word_document();

  // Not allowed: the parent has no constraint for the kind.
  assert_eq!(
    message(
      document
        .validate_child_part::<WorksheetPart>("any")
        .unwrap_err()
    ),
    "part kind WorksheetPart is not allowed as a child"
  );
  assert_eq!(
    message(
      main
        .validate_child_part::<_, WorksheetPart>(&document, "any")
        .unwrap_err()
    ),
    "part kind WorksheetPart is not allowed as a child"
  );

  // Content type mismatch.
  assert_eq!(
    message(
      document
        .validate_child_part::<CoreFilePropertiesPart>("text/plain")
        .unwrap_err()
    ),
    format!(
      "part kind CoreFilePropertiesPart requires content type {CORE_PROPERTIES_TYPE}, \
       got text/plain"
    )
  );
  assert_eq!(
    message(
      main
        .validate_child_part::<_, ThemePart>(&document, "text/plain")
        .unwrap_err()
    ),
    format!("part kind ThemePart requires content type {THEME_TYPE}, got text/plain")
  );

  // A matching content type is accepted and returns the parent's constraint.
  let constraint = document
    .validate_child_part::<CoreFilePropertiesPart>(CORE_PROPERTIES_TYPE)
    .expect("core properties are allowed");
  assert_eq!(constraint.relationship_type, CORE_PROPERTIES_RELATIONSHIP);
  assert!(!constraint.max_occurs_great_than_one);

  // At most one: the second child is rejected once the first exists.
  document
    .add_new_part_auto_id::<CoreFilePropertiesPart>()
    .expect("first core properties");
  assert_eq!(
    message(
      document
        .validate_child_part::<CoreFilePropertiesPart>(CORE_PROPERTIES_TYPE)
        .unwrap_err()
    ),
    format!("only one child with relationship type {CORE_PROPERTIES_RELATIONSHIP} is allowed")
  );
  main
    .add_new_part_auto_id::<_, ThemePart>(&mut document)
    .expect("first theme");
  assert_eq!(
    message(
      main
        .validate_child_part::<_, ThemePart>(&document, THEME_TYPE)
        .unwrap_err()
    ),
    format!("only one child with relationship type {THEME_RELATIONSHIP} is allowed")
  );
}

#[test]
fn validate_child_part_constraint_direct_calls() {
  let (document, _main) = word_document();
  let relationships = SdkPackage::relationships(&document);

  // Without an explicit constraint, `ExtendedPart` is always accepted.
  let constraint =
    validate_child_part_constraint::<ExtendedPart>(None, false, relationships, "any/type")
      .expect("extended part is accepted");
  assert_eq!(
    constraint,
    PartConstraint::new(
      <ExtendedPart as SdkPartDescriptor>::KIND,
      "",
      "",
      false,
      true
    )
  );

  // A parent that allows any child gets the child's descriptor values.
  let constraint = validate_child_part_constraint::<HeaderPart>(
    None,
    true,
    relationships,
    <HeaderPart as SdkPartDescriptor>::CONTENT_TYPE,
  )
  .expect("any child is allowed");
  assert_eq!(
    constraint,
    PartConstraint::new(
      <HeaderPart as SdkPartDescriptor>::KIND,
      <HeaderPart as SdkPartDescriptor>::RELATIONSHIP_TYPE,
      <HeaderPart as SdkPartDescriptor>::CONTENT_TYPE,
      false,
      true
    )
  );
  // The descriptor's content type is enforced for such a parent too.
  assert_eq!(
    message(
      validate_child_part_constraint::<HeaderPart>(None, true, relationships, "x/y").unwrap_err()
    ),
    format!(
      "part kind HeaderPart requires content type {}, got x/y",
      <HeaderPart as SdkPartDescriptor>::CONTENT_TYPE
    )
  );
  // ... and so is an explicit constraint's content type.
  let constrained = PartConstraint::new(
    <HeaderPart as SdkPartDescriptor>::KIND,
    <HeaderPart as SdkPartDescriptor>::RELATIONSHIP_TYPE,
    <HeaderPart as SdkPartDescriptor>::CONTENT_TYPE,
    false,
    true,
  );
  assert_eq!(
    message(
      validate_child_part_constraint::<HeaderPart>(Some(constrained), false, relationships, "x/y")
        .unwrap_err()
    ),
    format!(
      "part kind HeaderPart requires content type {}, got x/y",
      <HeaderPart as SdkPartDescriptor>::CONTENT_TYPE
    )
  );

  // No constraint and no permission to accept anything.
  assert_eq!(
    message(
      validate_child_part_constraint::<FooterPart>(None, false, relationships, "x/y").unwrap_err()
    ),
    "part kind FooterPart is not allowed as a child"
  );
}

#[test]
fn typed_main_part_content_type_follows_the_package_type() {
  let storage_of =
    |document_type: WordprocessingDocumentType| WordprocessingDocument::create(document_type);

  let document = storage_of(WordprocessingDocumentType::Document);
  let content_type = typed_main_part_content_type::<MainDocumentPart>(document.storage());
  assert_eq!(content_type, WORD_MAIN);
  assert!(matches!(content_type, std::borrow::Cow::Owned(_)));

  let template = storage_of(WordprocessingDocumentType::Template);
  assert_eq!(
    typed_main_part_content_type::<MainDocumentPart>(template.storage()),
    WORD_TEMPLATE_MAIN
  );

  // A part with a fixed content type ignores the preferred main part type.
  let content_type = typed_main_part_content_type::<ThemePart>(template.storage());
  assert_eq!(content_type, THEME_TYPE);
  assert!(matches!(content_type, std::borrow::Cow::Borrowed(_)));

  // A part without a content type falls back to the empty descriptor value
  // only when the package has no preferred main part type.
  let content_type =
    typed_main_part_content_type::<crate::parts::image_part::ImagePart>(template.storage());
  assert_eq!(content_type, WORD_TEMPLATE_MAIN);

  let spreadsheet = crate::parts::spreadsheet_document::SpreadsheetDocument::create(
    SpreadsheetDocumentType::Workbook,
  );
  assert_eq!(
    typed_main_part_content_type::<WorkbookPart>(spreadsheet.storage()),
    SHEET_MAIN
  );
  let presentation = crate::parts::presentation_document::PresentationDocument::create(
    PresentationDocumentType::Presentation,
  );
  assert_eq!(
    typed_main_part_content_type::<PresentationPart>(presentation.storage()),
    PRESENTATION_MAIN
  );
}

#[test]
fn default_main_part_content_type_covers_the_three_main_parts() {
  assert_eq!(
    default_main_part_content_type::<MainDocumentPart>(),
    Some(WORD_MAIN)
  );
  assert_eq!(
    default_main_part_content_type::<WorkbookPart>(),
    Some(SHEET_MAIN)
  );
  assert_eq!(
    default_main_part_content_type::<PresentationPart>(),
    Some(PRESENTATION_MAIN)
  );
  assert_eq!(default_main_part_content_type::<VisioDocumentPart>(), None);
  assert_eq!(default_main_part_content_type::<HeaderPart>(), None);
  assert_eq!(default_main_part_content_type::<ExtendedPart>(), None);
}

#[test]
fn part_refs_are_built_with_the_stored_kind() {
  let (mut document, main) = word_document();
  let header = main
    .add_new_part_auto_id::<_, HeaderPart>(&mut document)
    .expect("header");
  let theme = main
    .add_new_part_auto_id::<_, ThemePart>(&mut document)
    .expect("theme");
  // A worksheet-typed child classified by its relationship type, although the
  // parent does not allow the kind through `add_part`.
  let extended = main
    .add_extended_part(
      &mut document,
      WorksheetPart::RELATIONSHIP_TYPE,
      WorksheetPart::CONTENT_TYPE,
      ".xml",
    )
    .expect("extended part");
  let unknown = main
    .add_extended_part(
      &mut document,
      "urn:example:unknown",
      "application/example",
      ".bin",
    )
    .expect("unknown extended part");

  let refs = document.get_all_parts().collect::<Vec<_>>();
  assert!(
    refs
      .iter()
      .any(|part| matches!(part, PartRef::MainDocumentPart(_)))
  );
  let slot_ref = |part_slot| PartRef::from_part_slot(&document, part_slot).expect("part ref");
  let header_slot = crate::private::SdkPartHandle::part_slot(&header, &document).unwrap();
  let theme_slot = crate::private::SdkPartHandle::part_slot(&theme, &document).unwrap();
  let extended_slot = crate::private::SdkPartHandle::part_slot(&extended, &document).unwrap();
  let unknown_slot = crate::private::SdkPartHandle::part_slot(&unknown, &document).unwrap();
  assert!(matches!(slot_ref(header_slot), PartRef::HeaderPart(_)));
  assert!(matches!(slot_ref(theme_slot), PartRef::ThemePart(_)));
  assert!(matches!(slot_ref(extended_slot), PartRef::WorksheetPart(_)));
  assert!(matches!(slot_ref(unknown_slot), PartRef::ExtendedPart(_)));

  // Every route that builds a handle agrees on the variant.
  let by_id = document
    .parts()
    .map(|pair| (pair.relationship_id.to_string(), pair.part))
    .collect::<Vec<_>>();
  assert!(
    by_id
      .iter()
      .any(|(_, part)| matches!(part, PartRef::MainDocumentPart(_)))
  );
  let main_children = main.parts(&document).collect::<Vec<_>>();
  assert_eq!(main_children.len(), 4);
  assert!(
    main_children
      .iter()
      .any(|pair| matches!(pair.part, PartRef::WorksheetPart(_)))
  );
  assert!(
    main_children
      .iter()
      .any(|pair| matches!(pair.part, PartRef::ExtendedPart(_)))
  );
}
