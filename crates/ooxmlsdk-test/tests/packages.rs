#![cfg(feature = "parts")]

use std::io::{Cursor, Write};

use ooxmlsdk::common::{RelationshipSet, RelationshipTargetKind, StoredPartDataKind};
use ooxmlsdk::parts::{
  PartRef, PartRootCache, header_part::HeaderPart, image_part::ImagePart,
  main_document_part::MainDocumentPart, presentation_document::PresentationDocument,
  ribbon_extensibility_part::RibbonExtensibilityPart, spreadsheet_document::SpreadsheetDocument,
  style_definitions_part::StyleDefinitionsPart,
  wordprocessing_comments_part::WordprocessingCommentsPart,
  wordprocessing_document::WordprocessingDocument,
};
use ooxmlsdk::schemas::opc_relationships::TargetMode;
use ooxmlsdk::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::{
  Body, Document, Header,
};
use ooxmlsdk::sdk::SdkPackage;
use ooxmlsdk_test::fixtures;

fn doc_sample(file_name: &str) -> std::path::PathBuf {
  let path = fixtures::doc_sample_path(file_name);
  assert!(path.is_file(), "missing doc sample: {}", path.display());
  path
}

fn main_document_body_child_count(
  document: &ooxmlsdk::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Document,
) -> usize {
  document
    .body
    .as_ref()
    .map(|body| body.body_choice.len() + usize::from(body.w_sect_pr.is_some()))
    .unwrap_or_default()
}

fn empty_body_document() -> Document {
  Document {
    body: Some(Box::new(Body::default())),
    ..Default::default()
  }
}

fn empty_package() -> Cursor<Vec<u8>> {
  let mut buffer = Cursor::new(Vec::new());
  {
    let mut zip = zip::ZipWriter::new(&mut buffer);
    let options = zip::write::SimpleFileOptions::default();
    zip.start_file("[Content_Types].xml", options).unwrap();
    zip
      .write_all(
        br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"/>"#,
      )
      .unwrap();
    zip.finish().unwrap();
  }
  buffer.set_position(0);
  buffer
}

#[test]
fn wordprocessing_document_supports_eager_and_lazy_root_loading() {
  let path = doc_sample("Of16-01.docx");

  let mut eager = WordprocessingDocument::new_from_file(&path).unwrap();
  let eager_main = eager.main_document_part().unwrap();
  let eager_root = eager_main.root_element(&mut eager).unwrap();
  assert!(main_document_body_child_count(eager_root) > 0);

  let mut lazy = WordprocessingDocument::new_from_file_lazy(&path).unwrap();
  let lazy_main = lazy.main_document_part().unwrap();
  let lazy_root = lazy_main.root_element(&mut lazy).unwrap();
  assert!(main_document_body_child_count(lazy_root) > 0);
}

#[test]
fn package_relationships_resolve_with_container_local_part_factory() {
  let package = WordprocessingDocument::new_from_file(doc_sample("Of16-01.docx")).unwrap();
  let main_part = package.main_document_part().unwrap();
  let main_part_id = package.get_id_of_part(main_part).unwrap();

  let resolved = package
    .try_get_part_by_id(main_part_id)
    .and_then(PartRef::downcast::<MainDocumentPart>)
    .unwrap();

  assert_eq!(resolved.part_id(), main_part.part_id());
  assert_eq!(package.parts().count(), package.relationships().len());
  assert_eq!(
    package
      .get_sub_part_of_type::<MainDocumentPart>()
      .unwrap()
      .part_id(),
    main_part.part_id()
  );
  assert_eq!(package.get_parts_of_type::<MainDocumentPart>().count(), 1);
  assert!(package.try_get_part_by_id("invalidId").is_none());
}

#[test]
fn wordprocessing_child_accessors_are_relationship_backed_handles() {
  let package = WordprocessingDocument::new_from_file(doc_sample("Of16-01.docx")).unwrap();
  let main_part = package.main_document_part().unwrap();
  let styles_part = main_part.style_definitions_part(&package).unwrap();

  assert!(main_part.relationships(&package).is_some());
  assert_eq!(
    main_part.get_id_of_part(&package, styles_part),
    main_part
      .style_definitions_part_relationships(&package)
      .next()
      .map(|relationship| relationship.id())
  );
  assert_eq!(
    main_part
      .style_definitions_part_relationships(&package)
      .count(),
    1
  );
  assert_eq!(
    main_part
      .get_sub_part_of_type::<_, StyleDefinitionsPart>(&package)
      .unwrap()
      .part_id(),
    styles_part.part_id()
  );
  assert!(styles_part.path(&package).unwrap().ends_with("styles.xml"));
  assert!(
    styles_part
      .content_type(&package)
      .unwrap()
      .ends_with("+xml")
  );
  assert!(!styles_part.data(&package).unwrap().is_empty());
  assert!(main_part.document_settings_part(&package).is_some());
  assert_eq!(
    main_part
      .parts(&package)
      .filter(|pair| pair.part.downcast::<MainDocumentPart>().is_some())
      .count(),
    0
  );
}

#[test]
fn wordprocessing_root_element_access_matches_openxml_part_root_element_test() {
  // Source: test/DocumentFormat.OpenXml.Tests/ofapiTest/OpenXmlPartTest.cs :: RootElementTest
  let mut package = WordprocessingDocument::new_from_file(doc_sample("complex0.docx")).unwrap();
  let main_part = package.main_document_part().unwrap();
  let comments_part = main_part.wordprocessing_comments_part(&package).unwrap();
  let image_part = main_part.image_parts(&package).next().unwrap();

  assert!(main_part.root_element(&mut package).is_ok());
  assert!(comments_part.root_element(&mut package).is_ok());
  assert!(package.root_element(image_part.part_id()).is_none());
  assert_eq!(
    image_part.data_kind(&package),
    Some(StoredPartDataKind::Binary)
  );
  assert_eq!(
    main_part.get_id_of_part(&package, comments_part),
    Some("rId7")
  );
  assert_eq!(
    main_part
      .get_part_by_id(&package, "rId7")
      .and_then(PartRef::downcast::<WordprocessingCommentsPart>)
      .map(|part| part.part_id()),
    Some(comments_part.part_id())
  );
  assert_eq!(
    main_part
      .get_parts_of_type::<_, ImagePart>(&package)
      .count(),
    13
  );
}

#[test]
fn package_storage_parts_match_openxml_package_get_all_parts_tests() {
  // Source: test/DocumentFormat.OpenXml.Tests/ofapiTest/OpenXmlPackageTest.cs
  //   OpenXmlPackageGetAllPartsTestWord
  //   OpenXmlPackageGetAllPartsTestPowerPoint
  let word = WordprocessingDocument::new_from_file(doc_sample("complex0.docx")).unwrap();
  assert_eq!(word.storage().parts().len(), 31);
  assert_eq!(word.storage().parts().len(), 31);

  let presentation =
    PresentationDocument::new_from_file(doc_sample("o09_Performance_typical.pptx")).unwrap();
  let data_parts = presentation
    .storage()
    .parts()
    .iter()
    .filter(|part| {
      matches!(
        part.relationship_type(),
        Some(
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/audio"
            | "http://schemas.microsoft.com/office/2007/relationships/media"
            | "http://schemas.openxmlformats.org/officeDocument/2006/relationships/video"
        )
      )
    })
    .count();

  assert_eq!(presentation.storage().parts().len() - data_parts, 65);
  assert_eq!(data_parts, 1);
}

#[test]
fn media_reference_relationships_resolve_shared_data_part_from_openxml_package_test() {
  // Source: test/DocumentFormat.OpenXml.Tests/ofapiTest/OpenXmlPackageTest.cs
  //   LoadPackageWithMediaReferenceTest
  let package = PresentationDocument::new_from_file(doc_sample("mediareference.pptx")).unwrap();
  let presentation_part = package.presentation_part().unwrap();
  let slides: Vec<_> = presentation_part.slide_parts(&package).collect();
  assert_eq!(slides.len(), 2);
  assert!(
    presentation_part
      .slide_parts_relationships(&package)
      .count()
      >= 2
  );

  let media_part = package
    .storage()
    .parts()
    .iter()
    .enumerate()
    .find(|(_, part)| part.path() == "ppt/media/media1.wav")
    .map(|(index, part)| (ooxmlsdk::common::PartId::from_index(index), part))
    .unwrap();
  assert_eq!(media_part.1.content_type(), "audio/wav");
  assert_eq!(media_part.1.data().kind(), StoredPartDataKind::Binary);

  let mut internal_media_reference_count = 0;
  let mut external_null_audio_reference_count = 0;
  for slide in slides {
    for relationship in slide.data_part_reference_relationships(&package) {
      if relationship.target_part_id() == Some(media_part.0) {
        internal_media_reference_count += 1;
      } else {
        assert_eq!(relationship.target(), "NULL");
        assert!(matches!(relationship.target_mode(), TargetMode::External));
        assert_eq!(relationship.target_kind(), RelationshipTargetKind::External);
        external_null_audio_reference_count += 1;
      }
      match relationship.relationship_type() {
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/audio"
        | "http://schemas.microsoft.com/office/2007/relationships/media" => {}
        other => {
          panic!("unexpected data part reference relationship type: {other}");
        }
      }
    }
  }

  assert_eq!(internal_media_reference_count, 3);
  assert_eq!(external_null_audio_reference_count, 1);
}

#[test]
fn wordprocessing_hyperlink_relationships_are_preserved_from_openxml_part_test() {
  // Source: test/DocumentFormat.OpenXml.Tests/ofapiTest/OpenXmlPartTest.cs :: HyperlinkRelationshipTest
  let package = WordprocessingDocument::new_from_file(doc_sample("May_12_04.docx")).unwrap();
  let main_part = package.main_document_part().unwrap();
  let relationships = main_part.relationships(&package).unwrap();

  let hyperlink_relationships: Vec<_> = main_part.hyperlink_relationships(&package).collect();

  assert_eq!(hyperlink_relationships.len(), 71);
  assert_eq!(relationships.hyperlink_relationships().count(), 71);
  assert!(
    main_part
      .external_relationships(&package)
      .all(|relationship| relationship.relationship_type()
        != ooxmlsdk::common::RelationshipSet::HYPERLINK_RELATIONSHIP_TYPE)
  );
  assert!(relationships.contains_id("rId15"));

  let rid15 = relationships.get("rId15").unwrap();
  assert_eq!(rid15.target(), "#_THIS_WEEK_IN");
  assert!(matches!(rid15.target_mode(), TargetMode::Internal));
  assert_eq!(rid15.target_kind(), RelationshipTargetKind::Missing);
  assert!(rid15.target_part_id().is_none());

  let rid18 = relationships.get("rId18").unwrap();
  assert_eq!(rid18.target(), "http://www.iaswresearch.org/");
  assert!(matches!(rid18.target_mode(), TargetMode::External));
  assert_eq!(rid18.target_kind(), RelationshipTargetKind::External);
  assert!(rid18.target_part_id().is_none());
}

#[test]
fn root_cache_reports_and_unloads_lazy_roots() {
  // Source: test/DocumentFormat.OpenXml.Tests/ofapiTest/OpenXmlPartTest.cs :: UnloadRootElementTest
  let mut package = WordprocessingDocument::new_from_file_lazy(doc_sample("Of16-01.docx")).unwrap();
  let main_part = package.main_document_part().unwrap();
  let part_id = main_part.part_id();

  assert!(!package.is_root_element_loaded(part_id));
  assert!(main_part.root_element(&mut package).is_ok());
  assert!(package.is_root_element_loaded(part_id));
  assert!(package.unload_root_element(part_id).is_some());
  assert!(!package.is_root_element_loaded(part_id));
}

#[test]
fn part_root_element_mutation_is_saved() {
  // Source: adapted from OpenXmlPart root element save/mutation coverage.
  let mut package = WordprocessingDocument::new_from_file_lazy(doc_sample("Of16-01.docx")).unwrap();
  let main_part = package.main_document_part().unwrap();

  let root = main_part.root_element_mut(&mut package).unwrap();
  assert!(root.body.is_some());
  root.body = None;

  let mut buffer = Cursor::new(Vec::new());
  package.save(&mut buffer).unwrap();

  let mut reopened = WordprocessingDocument::new(Cursor::new(buffer.into_inner())).unwrap();
  let reopened_main = reopened.main_document_part().unwrap();
  let reopened_root = reopened_main.root_element(&mut reopened).unwrap();

  assert!(reopened_root.body.is_none());
}

#[test]
fn set_root_element_replaces_lazy_root_and_is_saved() {
  // Source: adapted from OpenXmlPart root element save/mutation coverage.
  let mut package = WordprocessingDocument::new_from_file_lazy(doc_sample("Of16-01.docx")).unwrap();
  let main_part = package.main_document_part().unwrap();
  let part_id = main_part.part_id();

  assert!(!package.is_root_element_loaded(part_id));

  main_part
    .set_root_element(&mut package, empty_body_document())
    .unwrap();

  assert!(package.is_root_element_loaded(part_id));
  assert_eq!(
    main_document_body_child_count(main_part.root_element(&mut package).unwrap()),
    0
  );

  let mut buffer = Cursor::new(Vec::new());
  package.save(&mut buffer).unwrap();

  let mut reopened = WordprocessingDocument::new(Cursor::new(buffer.into_inner())).unwrap();
  let reopened_main = reopened.main_document_part().unwrap();
  let reopened_root = reopened_main.root_element(&mut reopened).unwrap();

  assert_eq!(main_document_body_child_count(reopened_root), 0);
  assert!(reopened_root.body.is_some());
}

#[test]
fn root_element_mut_updates_eager_root_and_is_saved() {
  // Source: adapted from OpenXmlPart root element save/mutation coverage.
  let mut package = WordprocessingDocument::new_from_file(doc_sample("Of16-01.docx")).unwrap();
  let main_part = package.main_document_part().unwrap();
  let part_id = main_part.part_id();

  assert!(main_part.root_element(&mut package).is_ok());
  assert!(package.is_root_element_loaded(part_id));
  main_part.root_element_mut(&mut package).unwrap().body = Some(Box::new(Body::default()));

  let mut buffer = Cursor::new(Vec::new());
  package.save(&mut buffer).unwrap();

  let mut reopened = WordprocessingDocument::new(Cursor::new(buffer.into_inner())).unwrap();
  let reopened_main = reopened.main_document_part().unwrap();
  let reopened_root = reopened_main.root_element(&mut reopened).unwrap();

  assert_eq!(main_document_body_child_count(reopened_root), 0);
  assert!(reopened_root.body.is_some());
}

#[test]
fn set_root_element_replaces_unloaded_root_cache() {
  // Source: adapted from OpenXmlPart root element load/unload/save coverage.
  let mut package = WordprocessingDocument::new_from_file_lazy(doc_sample("Of16-01.docx")).unwrap();
  let main_part = package.main_document_part().unwrap();
  let part_id = main_part.part_id();

  assert!(main_part.root_element(&mut package).is_ok());
  assert!(package.unload_root_element(part_id).is_some());
  assert!(!package.is_root_element_loaded(part_id));

  main_part
    .set_root_element(&mut package, empty_body_document())
    .unwrap();

  assert!(package.is_root_element_loaded(part_id));

  let mut buffer = Cursor::new(Vec::new());
  package.save(&mut buffer).unwrap();

  let mut reopened = WordprocessingDocument::new(Cursor::new(buffer.into_inner())).unwrap();
  let reopened_main = reopened.main_document_part().unwrap();
  let reopened_root = reopened_main.root_element(&mut reopened).unwrap();

  assert_eq!(main_document_body_child_count(reopened_root), 0);
  assert!(reopened_root.body.is_some());
}

#[test]
fn part_hyperlink_relationship_mutation_is_saved() {
  // Source: adapted from OpenXmlPartContainer hyperlink relationship mutation coverage.
  let mut package = WordprocessingDocument::new_from_file_lazy(doc_sample("Of16-01.docx")).unwrap();
  let main_part = package.main_document_part().unwrap();
  let relationship_id = "rIdSdkHyperlink";
  let target = "https://example.com/ooxmlsdk";

  assert!(
    !main_part
      .relationships(&package)
      .unwrap()
      .contains_id(relationship_id)
  );

  let relationship = main_part
    .add_hyperlink_relationship(&mut package, relationship_id, target)
    .unwrap();
  assert_eq!(relationship.id(), relationship_id);
  assert_eq!(
    relationship.relationship_type(),
    RelationshipSet::HYPERLINK_RELATIONSHIP_TYPE
  );
  assert_eq!(relationship.target(), target);
  assert!(matches!(relationship.target_mode(), TargetMode::External));
  assert_eq!(relationship.target_kind(), RelationshipTargetKind::External);

  let mut buffer = Cursor::new(Vec::new());
  package.save(&mut buffer).unwrap();

  let reopened = WordprocessingDocument::new(Cursor::new(buffer.into_inner())).unwrap();
  let reopened_main = reopened.main_document_part().unwrap();
  let reopened_relationship = reopened_main
    .relationships(&reopened)
    .unwrap()
    .get(relationship_id)
    .unwrap();

  assert_eq!(reopened_relationship.target(), target);
  assert_eq!(
    reopened_relationship.relationship_type(),
    RelationshipSet::HYPERLINK_RELATIONSHIP_TYPE
  );
  assert!(matches!(
    reopened_relationship.target_mode(),
    TargetMode::External
  ));
}

#[test]
fn part_external_relationship_ids_can_change_and_remove() {
  // Source: adapted from OpenXmlPartContainer external relationship mutation coverage.
  let mut package = WordprocessingDocument::new_from_file_lazy(doc_sample("Of16-01.docx")).unwrap();
  let main_part = package.main_document_part().unwrap();
  let relationship_id = "rIdSdkExternal";
  let changed_relationship_id = "rIdSdkExternalChanged";

  main_part
    .add_external_relationship(
      &mut package,
      relationship_id,
      "http://example.com/relationships/custom",
      "https://example.com/custom",
    )
    .unwrap();
  assert!(
    main_part
      .add_external_relationship(
        &mut package,
        relationship_id,
        "http://example.com/relationships/custom",
        "https://example.com/duplicate",
      )
      .is_err()
  );

  main_part
    .change_relationship_id(&mut package, relationship_id, changed_relationship_id)
    .unwrap();
  assert!(
    main_part
      .relationships(&package)
      .unwrap()
      .get(relationship_id)
      .is_none()
  );
  assert!(
    main_part
      .relationships(&package)
      .unwrap()
      .contains_id(changed_relationship_id)
  );

  let removed = main_part
    .remove_relationship(&mut package, changed_relationship_id)
    .unwrap();
  assert_eq!(removed.id(), changed_relationship_id);

  let mut buffer = Cursor::new(Vec::new());
  package.save(&mut buffer).unwrap();

  let reopened = WordprocessingDocument::new(Cursor::new(buffer.into_inner())).unwrap();
  let reopened_main = reopened.main_document_part().unwrap();
  assert!(
    !reopened_main
      .relationships(&reopened)
      .unwrap()
      .contains_id(changed_relationship_id)
  );
}

#[test]
fn package_external_relationship_mutation_is_saved() {
  // Source: adapted from OpenXmlPackage relationship mutation coverage.
  let mut package = WordprocessingDocument::new_from_file_lazy(doc_sample("Of16-01.docx")).unwrap();
  let relationship_id = package.relationships().next_relationship_id();
  let relationship_type = "http://example.com/package/relationship";
  let target = "https://example.com/package";

  package
    .add_external_relationship(relationship_id.as_str(), relationship_type, target)
    .unwrap();

  let mut buffer = Cursor::new(Vec::new());
  package.save(&mut buffer).unwrap();

  let reopened = WordprocessingDocument::new(Cursor::new(buffer.into_inner())).unwrap();
  let relationship = reopened.relationships().get(&relationship_id).unwrap();

  assert_eq!(relationship.relationship_type(), relationship_type);
  assert_eq!(relationship.target(), target);
  assert_eq!(relationship.target_kind(), RelationshipTargetKind::External);
  assert!(matches!(relationship.target_mode(), TargetMode::External));
}

#[test]
fn add_new_header_part_creates_relationship_content_type_and_root_slot() {
  // Source: upstream AddNewPart<HeaderPart> coverage adapted to package-level save/reopen.
  let mut package = WordprocessingDocument::new_from_file_lazy(doc_sample("Of16-01.docx")).unwrap();
  let main_part = package.main_document_part().unwrap();
  let relationship_id = main_part
    .relationships(&package)
    .unwrap()
    .next_relationship_id();

  let header_part = main_part
    .add_new_part::<_, HeaderPart>(&mut package, relationship_id.as_str())
    .unwrap();
  assert_eq!(
    main_part.get_id_of_part(&package, header_part),
    Some(relationship_id.as_str())
  );
  assert_eq!(
    header_part.content_type(&package),
    Some("application/vnd.openxmlformats-officedocument.wordprocessingml.header+xml")
  );
  assert_eq!(header_part.path(&package), Some("word/header1.xml"));
  assert!(
    !package.is_root_element_loaded(header_part.part_id()),
    "new part should start with an empty lazy root slot"
  );

  header_part
    .set_root_element(&mut package, Header::default())
    .unwrap();
  assert!(package.is_root_element_loaded(header_part.part_id()));

  let mut buffer = Cursor::new(Vec::new());
  package.save(&mut buffer).unwrap();

  let mut reopened = WordprocessingDocument::new(Cursor::new(buffer.into_inner())).unwrap();
  let reopened_main = reopened.main_document_part().unwrap();
  let reopened_header = reopened_main
    .header_parts(&reopened)
    .find(|part| reopened_main.get_id_of_part(&reopened, *part) == Some(relationship_id.as_str()))
    .unwrap();

  assert_eq!(reopened_header.path(&reopened), Some("word/header1.xml"));
  assert!(reopened_header.root_element(&mut reopened).is_ok());
}

#[test]
fn add_new_part_auto_id_skips_existing_relationship_ids() {
  // Source: upstream AddNewPart<T>() auto relationship-id behavior adapted for Rust handles.
  let mut package = WordprocessingDocument::new_from_file_lazy(doc_sample("Of16-01.docx")).unwrap();
  let main_part = package.main_document_part().unwrap();
  let relationship_count = main_part.relationships(&package).unwrap().len();

  let header_part = main_part
    .add_new_part_auto_id::<_, HeaderPart>(&mut package)
    .unwrap();
  let relationship_id = main_part
    .get_id_of_part(&package, header_part)
    .unwrap()
    .to_string();
  assert!(relationship_id.starts_with("rId"));
  assert_eq!(
    main_part.relationships(&package).unwrap().len(),
    relationship_count + 1
  );

  assert!(
    main_part
      .add_new_part::<_, HeaderPart>(&mut package, relationship_id.as_str())
      .is_err()
  );
  assert_eq!(
    main_part.relationships(&package).unwrap().len(),
    relationship_count + 1
  );
}

#[test]
fn package_add_new_part_creates_package_relationship() {
  // Source: upstream WordprocessingDocument.AddNewPart<T>() package-level coverage.
  let mut package = WordprocessingDocument::new_from_file_lazy(doc_sample("Of16-01.docx")).unwrap();
  let relationship_id = "rIdSdkRibbon";

  let ribbon_part = package
    .add_new_part::<RibbonExtensibilityPart>(relationship_id)
    .unwrap();
  assert_eq!(package.get_id_of_part(ribbon_part), Some(relationship_id));
  assert_eq!(ribbon_part.path(&package), Some("customUI/customUI1.xml"));
  assert_eq!(ribbon_part.content_type(&package), Some("application/xml"));

  let mut buffer = Cursor::new(Vec::new());
  package.save(&mut buffer).unwrap();

  let reopened = WordprocessingDocument::new(Cursor::new(buffer.into_inner())).unwrap();
  let reopened_part = reopened
    .get_part_by_id(relationship_id)
    .and_then(PartRef::downcast::<RibbonExtensibilityPart>)
    .unwrap();
  assert_eq!(
    reopened_part.path(&reopened),
    Some("customUI/customUI1.xml")
  );
}

#[test]
fn image_part_feed_data_is_saved() {
  // Source: upstream AddNewPart<ImagePart>(mime).FeedData(...) coverage.
  let mut package =
    WordprocessingDocument::new_from_file_lazy(doc_sample("Hyperlink.docx")).unwrap();
  let main_part = package.main_document_part().unwrap();
  let relationship_id = "rIdSdkImage";
  let image_bytes = b"\x89PNG\r\n\x1a\nsdk-image-bytes".to_vec();

  let image_part = main_part
    .add_new_part_with_content_type::<_, ImagePart>(&mut package, relationship_id, "image/png")
    .unwrap();
  image_part
    .feed_data(&mut package, &mut Cursor::new(image_bytes.clone()))
    .unwrap();

  assert_eq!(image_part.content_type(&package), Some("image/png"));
  assert_eq!(image_part.data(&package), Some(image_bytes.as_slice()));

  let mut buffer = Cursor::new(Vec::new());
  package.save(&mut buffer).unwrap();

  let reopened = WordprocessingDocument::new(Cursor::new(buffer.into_inner())).unwrap();
  let reopened_main = reopened.main_document_part().unwrap();
  let reopened_image = reopened_main
    .get_part_by_id(&reopened, relationship_id)
    .and_then(PartRef::downcast::<ImagePart>)
    .unwrap();

  assert_eq!(reopened_image.content_type(&reopened), Some("image/png"));
  assert_eq!(reopened_image.data(&reopened), Some(image_bytes.as_slice()));
}

#[test]
fn set_data_replaces_existing_part_bytes() {
  // Source: upstream GetStream(FileMode.Create) replacement semantics adapted to raw bytes.
  let mut package =
    WordprocessingDocument::new_from_file_lazy(doc_sample("Hyperlink.docx")).unwrap();
  let main_part = package.main_document_part().unwrap();
  let image_part = main_part
    .add_new_part_with_content_type_auto_id::<_, ImagePart>(&mut package, "image/png")
    .unwrap();

  image_part
    .set_data(&mut package, b"first image bytes".to_vec())
    .unwrap();
  image_part
    .set_data(&mut package, b"replacement image bytes".to_vec())
    .unwrap();

  let mut buffer = Cursor::new(Vec::new());
  package.save(&mut buffer).unwrap();

  let reopened = WordprocessingDocument::new(Cursor::new(buffer.into_inner())).unwrap();
  let reopened_main = reopened.main_document_part().unwrap();
  let relationship_id = main_part.get_id_of_part(&package, image_part).unwrap();
  let reopened_image = reopened_main
    .get_part_by_id(&reopened, relationship_id)
    .and_then(PartRef::downcast::<ImagePart>)
    .unwrap();

  assert_eq!(
    reopened_image.data(&reopened),
    Some(&b"replacement image bytes"[..])
  );
}

#[test]
fn add_main_document_part_creates_fixed_main_part_path() {
  // Source: upstream WordprocessingDocument.Create(...).AddMainDocumentPart() coverage.
  let mut package = WordprocessingDocument::new(empty_package()).unwrap();
  assert!(package.main_document_part().is_err());

  let main_part = package.add_main_document_part().unwrap();
  assert_eq!(main_part.path(&package), Some("word/document.xml"));
  assert_eq!(
    main_part.content_type(&package),
    Some("application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml")
  );
  main_part
    .set_root_element(&mut package, empty_body_document())
    .unwrap();

  let mut buffer = Cursor::new(Vec::new());
  package.save(&mut buffer).unwrap();

  let mut reopened = WordprocessingDocument::new(Cursor::new(buffer.into_inner())).unwrap();
  let reopened_main = reopened.main_document_part().unwrap();
  assert_eq!(reopened_main.path(&reopened), Some("word/document.xml"));
  assert_eq!(
    main_document_body_child_count(reopened_main.root_element(&mut reopened).unwrap()),
    0
  );
}

#[test]
fn add_main_document_part_errors_when_main_part_exists() {
  // Source: upstream AddMainDocumentPart duplicate-main-part exception coverage.
  let mut package = WordprocessingDocument::new_from_file_lazy(doc_sample("Of16-01.docx")).unwrap();

  assert!(package.add_main_document_part().is_err());
}

#[test]
fn spreadsheet_child_accessors_resolve_repeated_parts() {
  let package = SpreadsheetDocument::new_from_file(doc_sample("basicspreadsheet.xlsx")).unwrap();
  let workbook_part = package.workbook_part().unwrap();

  assert!(workbook_part.relationships(&package).is_some());
  assert!(workbook_part.theme_part(&package).is_some());
  assert!(workbook_part.worksheet_parts(&package).count() >= 1);
}

#[test]
fn spreadsheet_sheet_relationship_ids_match_workbook_part_relationships() {
  // Source: test/DocumentFormat.OpenXml.Tests/XlsxTests01.cs :: X002_XlsxCreation / X003_XlsxCreation_Stream
  let mut package =
    SpreadsheetDocument::new_from_file(doc_sample("basicspreadsheet.xlsx")).unwrap();
  let workbook_part = package.workbook_part().unwrap();
  let sheet_relationship_ids: Vec<String> = workbook_part
    .root_element(&mut package)
    .unwrap()
    .sheets
    .x_sheet
    .iter()
    .map(|sheet| sheet.id.as_str().to_string())
    .collect();
  let worksheet_relationship_ids: Vec<&str> = workbook_part
    .worksheet_parts(&package)
    .map(|worksheet| workbook_part.get_id_of_part(&package, worksheet).unwrap())
    .collect();

  assert!(!sheet_relationship_ids.is_empty());
  assert_eq!(
    sheet_relationship_ids.len(),
    worksheet_relationship_ids.len()
  );
  for relationship_id in sheet_relationship_ids {
    assert!(worksheet_relationship_ids.contains(&relationship_id.as_str()));
  }
}

#[test]
fn presentation_child_accessors_resolve_repeated_parts() {
  let package = PresentationDocument::new_from_file(doc_sample("mcppt.pptx")).unwrap();
  let presentation_part = package.presentation_part().unwrap();

  assert!(presentation_part.relationships(&package).is_some());
  assert!(presentation_part.slide_parts(&package).count() >= 1);
  assert!(presentation_part.slide_master_parts(&package).count() >= 1);
}

#[test]
fn package_save_roundtrips_unmodified_storage() {
  let original = WordprocessingDocument::new_from_file_lazy(doc_sample("Of16-01.docx")).unwrap();
  let mut buffer = Cursor::new(Vec::new());

  original.save(&mut buffer).unwrap();

  let mut roundtripped = WordprocessingDocument::new(Cursor::new(buffer.into_inner())).unwrap();
  let main_part = roundtripped.main_document_part().unwrap();
  let root = main_part.root_element(&mut roundtripped).unwrap();
  assert!(main_document_body_child_count(root) > 0);
}
