#![cfg(feature = "parts")]

use std::io::{Cursor, Write};

use ooxmlsdk::common::{RelationshipSet, RelationshipTargetKind, StoredPartDataKind};
use ooxmlsdk::parts::{
  PartRef, PartRootCache, alternative_format_import_part::AlternativeFormatImportPart,
  core_file_properties_part::CoreFilePropertiesPart,
  custom_file_properties_part::CustomFilePropertiesPart, custom_property_part::CustomPropertyPart,
  custom_xml_part::CustomXmlPart, digital_signature_origin_part::DigitalSignatureOriginPart,
  document_settings_part::DocumentSettingsPart,
  embedded_control_persistence_binary_data_part::EmbeddedControlPersistenceBinaryDataPart,
  embedded_control_persistence_part::EmbeddedControlPersistencePart,
  embedded_object_part::EmbeddedObjectPart, embedded_package_part::EmbeddedPackagePart,
  extended_file_properties_part::ExtendedFilePropertiesPart, extended_part::ExtendedPart,
  font_part::FontPart, font_table_part::FontTablePart, header_part::HeaderPart,
  image_part::ImagePart, mail_merge_recipient_data_part::MailMergeRecipientDataPart,
  main_document_part::MainDocumentPart, presentation_document::PresentationDocument,
  ribbon_extensibility_part::RibbonExtensibilityPart, spreadsheet_document::SpreadsheetDocument,
  style_definitions_part::StyleDefinitionsPart, thumbnail_part::ThumbnailPart,
  wordprocessing_comments_part::WordprocessingCommentsPart,
  wordprocessing_document::WordprocessingDocument,
};
use ooxmlsdk::schemas::opc_relationships::TargetMode;
use ooxmlsdk::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::{
  Body, Document, Header,
};
use ooxmlsdk::sdk::{
  AlternativeFormatImportPartType, CustomPropertyPartType, CustomXmlPartType,
  EmbeddedControlPersistenceBinaryDataPartType, EmbeddedControlPersistencePartType,
  EmbeddedObjectPartType, EmbeddedPackagePartType, FontPartType, MailMergeRecipientDataPartType,
  MediaDataPartType, SdkPackage, SdkPartHandle, ThumbnailPartType,
};
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

fn package_entry_exists(bytes: Vec<u8>, path: &str) -> bool {
  let mut archive = zip::ZipArchive::new(Cursor::new(bytes)).unwrap();
  archive.by_name(path).is_ok()
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
fn media_data_part_reference_relationships_can_be_added_removed_and_reopened() {
  // Source: test/DocumentFormat.OpenXml.Tests/ofapiTest/OpenXmlPackageTest.cs
  //   LoadPackageWithMediaReferenceTest
  let mut package =
    PresentationDocument::new_from_file_lazy(doc_sample("mediareference.pptx")).unwrap();
  let media_data_parts: Vec<_> = package.media_data_parts().collect();
  assert_eq!(media_data_parts.len(), 1);
  let media_data_part = media_data_parts[0].clone();
  assert_eq!(media_data_part.content_type(&package), Some("audio/wav"));
  assert_eq!(
    media_data_part
      .data_part_reference_relationships(&package)
      .count(),
    3
  );

  let presentation_part = package.presentation_part().unwrap();
  let slides: Vec<_> = presentation_part.slide_parts(&package).collect();
  let slide1 = slides[0];
  let slide2 = slides[1];

  let slide1_relationships: Vec<_> = slide1.data_part_reference_relationships(&package).collect();
  let slide1_media_relationships: Vec<_> = slide1_relationships
    .iter()
    .copied()
    .filter(|relationship| relationship.target_part_id() == media_data_part.part_id())
    .collect();
  assert_eq!(slide1_media_relationships.len(), 1);
  assert_eq!(
    slide1_media_relationships[0].relationship_type(),
    RelationshipSet::MEDIA_REFERENCE_RELATIONSHIP_TYPE
  );
  let old_slide1_reference_id = slide1_media_relationships[0].id().to_string();

  assert_eq!(
    slide2.data_part_reference_relationships(&package).count(),
    2
  );

  let new_reference_id = slide1
    .add_audio_reference_relationship(&mut package, &media_data_part)
    .unwrap();
  let slide1_relationships: Vec<_> = slide1.data_part_reference_relationships(&package).collect();
  let slide1_media_relationships: Vec<_> = slide1_relationships
    .iter()
    .copied()
    .filter(|relationship| relationship.target_part_id() == media_data_part.part_id())
    .collect();
  assert_eq!(slide1_media_relationships.len(), 2);
  let new_reference = slide1_relationships
    .iter()
    .find(|relationship| relationship.id() == new_reference_id)
    .unwrap();
  assert_eq!(
    new_reference.relationship_type(),
    RelationshipSet::AUDIO_REFERENCE_RELATIONSHIP_TYPE
  );
  assert_eq!(new_reference.target_part_id(), media_data_part.part_id());

  let removed = slide1
    .remove_relationship(&mut package, &old_slide1_reference_id)
    .unwrap();
  assert_eq!(removed.id(), old_slide1_reference_id);
  assert_eq!(
    slide1
      .data_part_reference_relationships(&package)
      .filter(|relationship| relationship.target_part_id() == media_data_part.part_id())
      .count(),
    1
  );

  let slide2_reference_id = slide2
    .data_part_reference_relationships(&package)
    .find(|relationship| relationship.target_part_id() == media_data_part.part_id())
    .unwrap()
    .id()
    .to_string();
  let removed = slide2
    .remove_relationship(&mut package, &slide2_reference_id)
    .unwrap();
  assert_eq!(removed.id(), slide2_reference_id);
  assert_eq!(
    slide2.data_part_reference_relationships(&package).count(),
    1
  );
  assert_eq!(
    media_data_part
      .data_part_reference_relationships(&package)
      .count(),
    2
  );

  let mut buffer = Cursor::new(Vec::new());
  package.save(&mut buffer).unwrap();

  let mut reopened = PresentationDocument::new(Cursor::new(buffer.into_inner())).unwrap();
  let reopened_media_data_parts: Vec<_> = reopened.media_data_parts().collect();
  assert_eq!(reopened_media_data_parts.len(), 1);
  let reopened_media_data_part = reopened_media_data_parts[0].clone();
  assert_eq!(
    reopened_media_data_part.content_type(&reopened),
    Some("audio/wav")
  );
  assert_eq!(
    reopened_media_data_part
      .data_part_reference_relationships(&reopened)
      .count(),
    2
  );

  let reopened_presentation_part = reopened.presentation_part().unwrap();
  let reopened_slides: Vec<_> = reopened_presentation_part.slide_parts(&reopened).collect();
  let reopened_slide1 = reopened_slides[0];
  let reopened_slide2 = reopened_slides[1];
  let reopened_slide1_relationships: Vec<_> = reopened_slide1
    .data_part_reference_relationships(&reopened)
    .filter(|relationship| relationship.target_part_id() == reopened_media_data_part.part_id())
    .collect();
  assert_eq!(reopened_slide1_relationships.len(), 1);
  assert_eq!(
    reopened_slide1_relationships[0].relationship_type(),
    RelationshipSet::AUDIO_REFERENCE_RELATIONSHIP_TYPE
  );
  assert_eq!(
    reopened_slide1_relationships[0].target_part_id(),
    reopened_media_data_part.part_id()
  );
  assert_eq!(
    reopened_slide2
      .data_part_reference_relationships(&reopened)
      .filter(|relationship| relationship.target_part_id() == reopened_media_data_part.part_id())
      .count(),
    1
  );

  let reopened_slide1_reference_id = reopened_slide1_relationships[0].id().to_string();
  reopened_slide1
    .remove_relationship(&mut reopened, &reopened_slide1_reference_id)
    .unwrap();
  let reopened_slide2_reference_id = reopened_slide2
    .data_part_reference_relationships(&reopened)
    .find(|relationship| relationship.target_part_id() == reopened_media_data_part.part_id())
    .unwrap()
    .id()
    .to_string();
  reopened_slide2
    .remove_relationship(&mut reopened, &reopened_slide2_reference_id)
    .unwrap();

  let remaining_media_data_parts: Vec<_> = reopened.media_data_parts().collect();
  assert_eq!(remaining_media_data_parts.len(), 1);
  assert_eq!(
    remaining_media_data_parts[0]
      .data_part_reference_relationships(&reopened)
      .count(),
    0
  );
}

#[test]
fn media_data_part_type_maps_upstream_content_types_and_extensions() {
  // Source: src/DocumentFormat.OpenXml.Framework/Packaging/MediaDataPartTypeInfo.cs
  let cases = [
    (MediaDataPartType::Aiff, "audio/aiff", ".aiff"),
    (MediaDataPartType::Midi, "audio/midi", ".midi"),
    (MediaDataPartType::Mp3, "audio/mp3", ".mp3"),
    (MediaDataPartType::MpegUrl, "audio/mpegurl", ".m3u"),
    (MediaDataPartType::Wav, "audio/wav", ".wav"),
    (MediaDataPartType::Wma, "audio/x-ms-wma", ".wma"),
    (MediaDataPartType::MpegAudio, "audio/mpeg", ".mpeg"),
    (MediaDataPartType::OggAudio, "audio/ogg", ".ogg"),
    (MediaDataPartType::Asx, "video/x-ms-asf-plugin", ".asx"),
    (MediaDataPartType::Avi, "video/avi", ".avi"),
    (MediaDataPartType::Mpg, "video/mpg", ".mpg"),
    (MediaDataPartType::MpegVideo, "video/mpeg", ".mpeg"),
    (MediaDataPartType::Wmv, "video/x-ms-wmv", ".wmv"),
    (MediaDataPartType::Wmx, "video/x-ms-wmx", ".wmx"),
    (MediaDataPartType::Wvx, "video/x-ms-wvx", ".wvx"),
    (MediaDataPartType::Quicktime, "video/quicktime", ".mov"),
    (MediaDataPartType::OggVideo, "video/ogg", ".ogg"),
    (MediaDataPartType::Vc1, "video/vc1", ".wmv"),
    (MediaDataPartType::Mp4, "video/mp4", ".mp4"),
  ];

  for (part_type, content_type, extension) in cases {
    assert_eq!(part_type.content_type(), content_type);
    assert_eq!(part_type.extension(), extension);
  }
}

#[test]
fn create_media_data_parts_adds_data_part_reference_relationships_and_saves() {
  // Source: test/DocumentFormat.OpenXml.Tests/ofapiTest/OpenXmlPackageTest.cs
  //   MediaDataPartReferenceTest
  let mut package =
    WordprocessingDocument::new_from_file_lazy(doc_sample("Hyperlink.docx")).unwrap();
  let main_part = package.main_document_part().unwrap();

  let default_ext = package
    .create_media_data_part_with_content_type("application/octet-stream")
    .unwrap();
  default_ext
    .set_data(&mut package, b"default media bytes".to_vec())
    .unwrap();
  assert_eq!(
    default_ext.content_type(&package),
    Some("application/octet-stream")
  );
  assert!(default_ext.path(&package).unwrap().ends_with(".bin"));

  let wav = package.create_media_data_part("audio/wav", ".wav").unwrap();
  wav.set_data(&mut package, b"wav bytes".to_vec()).unwrap();
  assert_eq!(wav.content_type(&package), Some("audio/wav"));
  assert!(wav.path(&package).unwrap().ends_with(".wav"));

  let avi = package
    .create_media_data_part_by_type(MediaDataPartType::Avi)
    .unwrap();
  avi.set_data(&mut package, b"avi bytes".to_vec()).unwrap();
  assert_eq!(avi.content_type(&package), Some("video/avi"));
  assert!(avi.path(&package).unwrap().ends_with(".avi"));

  let audio_relationship_id = main_part
    .add_audio_reference_relationship_with_id(&mut package, &wav, "rIdSdkAudio")
    .unwrap();
  let media_relationship_id = main_part
    .add_media_reference_relationship_with_id(&mut package, &avi, "rIdSdkMedia")
    .unwrap();
  let video_relationship_id = main_part
    .add_video_reference_relationship_with_id(&mut package, &avi, "rIdSdkVideo")
    .unwrap();
  assert_eq!(audio_relationship_id, "rIdSdkAudio");
  assert_eq!(media_relationship_id, "rIdSdkMedia");
  assert_eq!(video_relationship_id, "rIdSdkVideo");

  let relationships: Vec<_> = main_part
    .data_part_reference_relationships(&package)
    .collect();
  let audio_relationship = relationships
    .iter()
    .find(|relationship| relationship.id() == "rIdSdkAudio")
    .unwrap();
  assert_eq!(
    audio_relationship.relationship_type(),
    RelationshipSet::AUDIO_REFERENCE_RELATIONSHIP_TYPE
  );
  assert_eq!(audio_relationship.target_part_id(), wav.part_id());
  assert_eq!(
    audio_relationship.target_kind(),
    RelationshipTargetKind::InternalPart
  );

  let media_relationship = relationships
    .iter()
    .find(|relationship| relationship.id() == "rIdSdkMedia")
    .unwrap();
  assert_eq!(
    media_relationship.relationship_type(),
    RelationshipSet::MEDIA_REFERENCE_RELATIONSHIP_TYPE
  );
  assert_eq!(media_relationship.target_part_id(), avi.part_id());
  assert_eq!(
    media_relationship.target_kind(),
    RelationshipTargetKind::InternalPart
  );

  let video_relationship = relationships
    .iter()
    .find(|relationship| relationship.id() == "rIdSdkVideo")
    .unwrap();
  assert_eq!(
    video_relationship.relationship_type(),
    RelationshipSet::VIDEO_REFERENCE_RELATIONSHIP_TYPE
  );
  assert_eq!(video_relationship.target_part_id(), avi.part_id());
  assert_eq!(
    video_relationship.target_kind(),
    RelationshipTargetKind::InternalPart
  );

  let default_ext_path = default_ext.path(&package).unwrap().to_string();
  let wav_path = wav.path(&package).unwrap().to_string();
  let avi_path = avi.path(&package).unwrap().to_string();
  let mut buffer = Cursor::new(Vec::new());
  package.save(&mut buffer).unwrap();
  let bytes = buffer.into_inner();

  assert!(package_entry_exists(bytes.clone(), &default_ext_path));
  assert!(package_entry_exists(bytes.clone(), &wav_path));
  assert!(package_entry_exists(bytes.clone(), &avi_path));

  let reopened = WordprocessingDocument::new(Cursor::new(bytes)).unwrap();
  let reopened_main_part = reopened.main_document_part().unwrap();
  let reopened_relationships: Vec<_> = reopened_main_part
    .data_part_reference_relationships(&reopened)
    .collect();
  let reopened_audio_relationship = reopened_relationships
    .iter()
    .find(|relationship| relationship.id() == "rIdSdkAudio")
    .unwrap();
  let reopened_audio_part = reopened
    .storage()
    .part(reopened_audio_relationship.target_part_id().unwrap())
    .unwrap();
  assert_eq!(reopened_audio_part.content_type(), "audio/wav");
  assert_eq!(reopened_audio_part.data().bytes(), b"wav bytes");

  let reopened_media_relationship = reopened_relationships
    .iter()
    .find(|relationship| relationship.id() == "rIdSdkMedia")
    .unwrap();
  let reopened_media_part = reopened
    .storage()
    .part(reopened_media_relationship.target_part_id().unwrap())
    .unwrap();
  assert_eq!(reopened_media_part.content_type(), "video/avi");
  assert_eq!(reopened_media_part.data().bytes(), b"avi bytes");

  let reopened_video_relationship = reopened_relationships
    .iter()
    .find(|relationship| relationship.id() == "rIdSdkVideo")
    .unwrap();
  assert_eq!(
    reopened_video_relationship.relationship_type(),
    RelationshipSet::VIDEO_REFERENCE_RELATIONSHIP_TYPE
  );
  assert_eq!(
    reopened_video_relationship.target_part_id(),
    reopened_media_relationship.target_part_id()
  );
}

#[test]
fn add_data_part_reference_relationship_from_existing_reuses_id_type_and_target() {
  // Source: src/DocumentFormat.OpenXml.Framework/Packaging/OpenXmlPartContainer.cs
  //   AddDataPartReferenceRelationship(DataPartReferenceRelationship)
  let mut package =
    WordprocessingDocument::new_from_file_lazy(doc_sample("Hyperlink.docx")).unwrap();
  let main_part = package.main_document_part().unwrap();
  let header = main_part
    .add_new_part::<_, HeaderPart>(&mut package, "rIdSdkHeaderForDataRef")
    .unwrap();
  header
    .set_root_element(&mut package, Header::default())
    .unwrap();

  let media_data_part = package
    .create_media_data_part_by_type(MediaDataPartType::Mp3)
    .unwrap();
  media_data_part
    .set_data(&mut package, b"mp3 bytes".to_vec())
    .unwrap();
  main_part
    .add_media_reference_relationship_with_id(&mut package, &media_data_part, "rIdSdkDataRef")
    .unwrap();
  let existing_relationship = main_part
    .get_reference_relationship(&package, "rIdSdkDataRef")
    .unwrap()
    .clone();

  let copied_relationship_id = header
    .add_data_part_reference_relationship_from_existing(&mut package, &existing_relationship)
    .unwrap();
  assert_eq!(copied_relationship_id, "rIdSdkDataRef");
  let copied_relationship = header
    .get_reference_relationship(&package, "rIdSdkDataRef")
    .unwrap();
  assert_eq!(
    copied_relationship.relationship_type(),
    RelationshipSet::MEDIA_REFERENCE_RELATIONSHIP_TYPE
  );
  assert_eq!(
    copied_relationship.target_part_id(),
    media_data_part.part_id()
  );

  let mut buffer = Cursor::new(Vec::new());
  package.save(&mut buffer).unwrap();

  let reopened = WordprocessingDocument::new(Cursor::new(buffer.into_inner())).unwrap();
  let reopened_main = reopened.main_document_part().unwrap();
  let reopened_header = reopened_main
    .header_parts(&reopened)
    .find(|part| reopened_main.get_id_of_part(&reopened, *part) == Some("rIdSdkHeaderForDataRef"))
    .unwrap();
  let reopened_relationship = reopened_header
    .get_reference_relationship(&reopened, "rIdSdkDataRef")
    .unwrap();
  assert_eq!(
    reopened_relationship.relationship_type(),
    RelationshipSet::MEDIA_REFERENCE_RELATIONSHIP_TYPE
  );
  let reopened_media_part = reopened
    .storage()
    .part(reopened_relationship.target_part_id().unwrap())
    .unwrap();
  assert_eq!(reopened_media_part.content_type(), "audio/mp3");
  assert_eq!(reopened_media_part.data().bytes(), b"mp3 bytes");
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
  assert!(
    main_part
      .get_reference_relationship(&package, relationship_id)
      .is_some()
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
    .delete_reference_relationship(&mut package, changed_relationship_id)
    .unwrap();
  assert_eq!(removed.id(), changed_relationship_id);
  assert!(
    main_part
      .delete_reference_relationship(&mut package, changed_relationship_id)
      .is_err()
  );

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
  assert!(
    package
      .get_reference_relationship(&relationship_id)
      .is_some()
  );

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
fn add_image_part_with_id_feeds_data_and_saves() {
  // Source: upstream MainDocumentPart.AddImagePart(mime, id).FeedData(...) coverage.
  let mut package =
    WordprocessingDocument::new_from_file_lazy(doc_sample("Hyperlink.docx")).unwrap();
  let main_part = package.main_document_part().unwrap();
  let relationship_id = "rIdSdkImageWithId";
  let image_bytes = b"\x89PNG\r\n\x1a\nsdk-image-with-id".to_vec();

  let image_part = main_part
    .add_image_part_with_id(&mut package, "image/png", relationship_id)
    .unwrap();
  image_part
    .feed_data(&mut package, &mut Cursor::new(image_bytes.clone()))
    .unwrap();

  assert_eq!(
    main_part.get_id_of_part(&package, image_part),
    Some(relationship_id)
  );
  assert_eq!(image_part.content_type(&package), Some("image/png"));

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
fn add_image_part_auto_id_uses_next_relationship_id() {
  // Source: upstream MainDocumentPart.AddImagePart(mime).FeedData(...) coverage.
  let mut package =
    WordprocessingDocument::new_from_file_lazy(doc_sample("Hyperlink.docx")).unwrap();
  let main_part = package.main_document_part().unwrap();
  let relationship_count = main_part.relationships(&package).unwrap().len();

  let image_part = main_part
    .add_image_part(&mut package, "image/jpeg")
    .unwrap();
  image_part
    .set_data(&mut package, b"jpeg bytes".to_vec())
    .unwrap();
  let relationship_id = main_part
    .get_id_of_part(&package, image_part)
    .unwrap()
    .to_string();

  assert!(relationship_id.starts_with("rId"));
  assert_eq!(
    main_part.relationships(&package).unwrap().len(),
    relationship_count + 1
  );
  assert_eq!(image_part.content_type(&package), Some("image/jpeg"));

  let mut buffer = Cursor::new(Vec::new());
  package.save(&mut buffer).unwrap();

  let reopened = WordprocessingDocument::new(Cursor::new(buffer.into_inner())).unwrap();
  let reopened_main = reopened.main_document_part().unwrap();
  let reopened_image = reopened_main
    .get_part_by_id(&reopened, relationship_id.as_str())
    .and_then(PartRef::downcast::<ImagePart>)
    .unwrap();

  assert_eq!(reopened_image.content_type(&reopened), Some("image/jpeg"));
  assert_eq!(reopened_image.data(&reopened), Some(&b"jpeg bytes"[..]));
}

#[test]
fn add_alternative_format_import_part_with_id_feeds_data_and_saves() {
  // Source: upstream AddAlternativeFormatImportPart(type, id).GetStream(Create) coverage.
  let mut package =
    WordprocessingDocument::new_from_file_lazy(doc_sample("Hyperlink.docx")).unwrap();
  let main_part = package.main_document_part().unwrap();
  let relationship_id = "rIdSdkAltChunk";
  let html = b"<!doctype html><html><body>alt chunk</body></html>".to_vec();

  let alt_chunk = main_part
    .add_alternative_format_import_part_by_type_with_id(
      &mut package,
      AlternativeFormatImportPartType::Html,
      relationship_id,
    )
    .unwrap();
  alt_chunk
    .feed_data(&mut package, &mut Cursor::new(html.clone()))
    .unwrap();

  assert_eq!(
    main_part.get_id_of_part(&package, alt_chunk),
    Some(relationship_id)
  );
  assert_eq!(alt_chunk.content_type(&package), Some("text/html"));

  let mut buffer = Cursor::new(Vec::new());
  package.save(&mut buffer).unwrap();

  let reopened = WordprocessingDocument::new(Cursor::new(buffer.into_inner())).unwrap();
  let reopened_main = reopened.main_document_part().unwrap();
  let reopened_chunk = reopened_main
    .get_part_by_id(&reopened, relationship_id)
    .and_then(PartRef::downcast::<AlternativeFormatImportPart>)
    .unwrap();

  assert_eq!(reopened_chunk.content_type(&reopened), Some("text/html"));
  assert_eq!(reopened_chunk.data(&reopened), Some(html.as_slice()));
}

#[test]
fn add_alternative_format_import_part_auto_id_uses_part_type_content_type() {
  // Source: upstream AddAlternativeFormatImportPart(type) coverage.
  let mut package =
    WordprocessingDocument::new_from_file_lazy(doc_sample("Hyperlink.docx")).unwrap();
  let main_part = package.main_document_part().unwrap();
  let relationship_count = main_part.relationships(&package).unwrap().len();

  let alt_chunk = main_part
    .add_alternative_format_import_part_by_type(
      &mut package,
      AlternativeFormatImportPartType::Xhtml,
    )
    .unwrap();
  alt_chunk
    .set_data(
      &mut package,
      b"<html xmlns=\"http://www.w3.org/1999/xhtml\"/>".to_vec(),
    )
    .unwrap();
  let relationship_id = main_part
    .get_id_of_part(&package, alt_chunk)
    .unwrap()
    .to_string();

  assert!(relationship_id.starts_with("rId"));
  assert_eq!(
    main_part.relationships(&package).unwrap().len(),
    relationship_count + 1
  );
  assert_eq!(
    alt_chunk.content_type(&package),
    Some("application/xhtml+xml")
  );

  let mut buffer = Cursor::new(Vec::new());
  package.save(&mut buffer).unwrap();

  let reopened = WordprocessingDocument::new(Cursor::new(buffer.into_inner())).unwrap();
  let reopened_main = reopened.main_document_part().unwrap();
  let reopened_chunk = reopened_main
    .get_part_by_id(&reopened, relationship_id.as_str())
    .and_then(PartRef::downcast::<AlternativeFormatImportPart>)
    .unwrap();

  assert_eq!(
    reopened_chunk.content_type(&reopened),
    Some("application/xhtml+xml")
  );
  assert_eq!(
    reopened_chunk.data(&reopened),
    Some(&b"<html xmlns=\"http://www.w3.org/1999/xhtml\"/>"[..])
  );
}

#[test]
fn add_custom_xml_part_by_type_feeds_data_and_saves() {
  // Source: upstream AddCustomXmlPart(CustomXmlPartType.CustomXml) coverage.
  let mut package =
    WordprocessingDocument::new_from_file_lazy(doc_sample("Hyperlink.docx")).unwrap();
  let main_part = package.main_document_part().unwrap();
  let relationship_count = main_part.relationships(&package).unwrap().len();
  let xml = b"<properties><property name=\"sdk\">custom xml</property></properties>".to_vec();

  let custom_xml = main_part
    .add_custom_xml_part_by_type(&mut package, CustomXmlPartType::CustomXml)
    .unwrap();
  custom_xml
    .feed_data(&mut package, &mut Cursor::new(xml.clone()))
    .unwrap();
  let relationship_id = main_part
    .get_id_of_part(&package, custom_xml)
    .unwrap()
    .to_string();

  assert!(relationship_id.starts_with("rId"));
  assert_eq!(
    main_part.relationships(&package).unwrap().len(),
    relationship_count + 1
  );
  assert_eq!(custom_xml.content_type(&package), Some("application/xml"));

  let mut buffer = Cursor::new(Vec::new());
  package.save(&mut buffer).unwrap();

  let reopened = WordprocessingDocument::new(Cursor::new(buffer.into_inner())).unwrap();
  let reopened_main = reopened.main_document_part().unwrap();
  let reopened_custom_xml = reopened_main
    .get_part_by_id(&reopened, relationship_id.as_str())
    .and_then(PartRef::downcast::<CustomXmlPart>)
    .unwrap();

  assert_eq!(
    reopened_custom_xml.content_type(&reopened),
    Some("application/xml")
  );
  assert_eq!(reopened_custom_xml.data(&reopened), Some(xml.as_slice()));
}

#[test]
fn add_custom_xml_part_with_id_uses_content_type_and_relationship_id() {
  // Source: upstream AddCustomXmlPart(contentType, id) supported relationship overload.
  let mut package =
    WordprocessingDocument::new_from_file_lazy(doc_sample("Hyperlink.docx")).unwrap();
  let main_part = package.main_document_part().unwrap();
  let relationship_id = "rIdSdkCustomXml";
  let inkml = b"<ink xmlns=\"http://www.w3.org/2003/InkML\"/>".to_vec();

  let custom_xml = main_part
    .add_custom_xml_part_with_id(&mut package, "application/inkml+xml", relationship_id)
    .unwrap();
  custom_xml.set_data(&mut package, inkml.clone()).unwrap();

  assert_eq!(
    main_part.get_id_of_part(&package, custom_xml),
    Some(relationship_id)
  );
  assert_eq!(
    custom_xml.content_type(&package),
    Some("application/inkml+xml")
  );

  let mut buffer = Cursor::new(Vec::new());
  package.save(&mut buffer).unwrap();

  let reopened = WordprocessingDocument::new(Cursor::new(buffer.into_inner())).unwrap();
  let reopened_main = reopened.main_document_part().unwrap();
  let reopened_custom_xml = reopened_main
    .get_part_by_id(&reopened, relationship_id)
    .and_then(PartRef::downcast::<CustomXmlPart>)
    .unwrap();

  assert_eq!(
    reopened_custom_xml.content_type(&reopened),
    Some("application/inkml+xml")
  );
  assert_eq!(reopened_custom_xml.data(&reopened), Some(inkml.as_slice()));
}

#[test]
fn add_extensible_supported_relationship_parts_by_type_save_and_reopen() {
  // Source: upstream OpenXmlSupportedRelationshipExtensions typed PartTypeInfo overloads.
  let mut package =
    WordprocessingDocument::new_from_file_lazy(doc_sample("Hyperlink.docx")).unwrap();
  let main_part = package.main_document_part().unwrap();

  let embedded_object = main_part
    .add_embedded_object_part_by_type(&mut package, EmbeddedObjectPartType::Binary)
    .unwrap();
  embedded_object
    .set_data(&mut package, b"ole object bytes".to_vec())
    .unwrap();
  let embedded_object_id = main_part
    .get_id_of_part(&package, embedded_object)
    .unwrap()
    .to_string();

  let embedded_package = main_part
    .add_embedded_package_part_by_type(&mut package, EmbeddedPackagePartType::Xlsx)
    .unwrap();
  embedded_package
    .set_data(&mut package, b"xlsx package bytes".to_vec())
    .unwrap();
  let embedded_package_id = main_part
    .get_id_of_part(&package, embedded_package)
    .unwrap()
    .to_string();

  let font_table = main_part
    .add_new_part_auto_id::<_, FontTablePart>(&mut package)
    .unwrap();
  let font = font_table
    .add_font_part_by_type(&mut package, FontPartType::FontTtf)
    .unwrap();
  font.set_data(&mut package, b"ttf bytes".to_vec()).unwrap();
  let font_table_id = main_part
    .get_id_of_part(&package, font_table)
    .unwrap()
    .to_string();
  let font_id = font_table
    .get_id_of_part(&package, font)
    .unwrap()
    .to_string();

  let settings = main_part
    .add_new_part_auto_id::<_, DocumentSettingsPart>(&mut package)
    .unwrap();
  let recipients = settings
    .add_mail_merge_recipient_data_part_by_type(
      &mut package,
      MailMergeRecipientDataPartType::OpenXmlMailMergeRecipientData,
    )
    .unwrap();
  recipients
    .set_data(&mut package, b"<recipients/>".to_vec())
    .unwrap();
  let settings_id = main_part
    .get_id_of_part(&package, settings)
    .unwrap()
    .to_string();
  let recipients_id = settings
    .get_id_of_part(&package, recipients)
    .unwrap()
    .to_string();

  assert_eq!(
    embedded_object.content_type(&package),
    Some("application/vnd.openxmlformats-officedocument.oleObject")
  );
  assert_eq!(
    embedded_package.content_type(&package),
    Some("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet")
  );
  assert_eq!(font.content_type(&package), Some("application/x-font-ttf"));
  assert_eq!(
    recipients.content_type(&package),
    Some(
      "application/vnd.openxmlformats-officedocument.wordprocessingml.mailMergeRecipientData+xml"
    )
  );
  assert!(
    package
      .storage()
      .part(embedded_package.part_id())
      .unwrap()
      .path()
      .ends_with(".xlsx")
  );
  assert!(
    package
      .storage()
      .part(font.part_id())
      .unwrap()
      .path()
      .ends_with(".ttf")
  );

  let mut buffer = Cursor::new(Vec::new());
  package.save(&mut buffer).unwrap();

  let reopened = WordprocessingDocument::new_lazy(Cursor::new(buffer.into_inner())).unwrap();
  let reopened_main = reopened.main_document_part().unwrap();

  let reopened_embedded_object = reopened_main
    .get_part_by_id(&reopened, embedded_object_id.as_str())
    .and_then(PartRef::downcast::<EmbeddedObjectPart>)
    .unwrap();
  let reopened_embedded_package = reopened_main
    .get_part_by_id(&reopened, embedded_package_id.as_str())
    .and_then(PartRef::downcast::<EmbeddedPackagePart>)
    .unwrap();
  let reopened_font_table = reopened_main
    .get_part_by_id(&reopened, font_table_id.as_str())
    .and_then(PartRef::downcast::<FontTablePart>)
    .unwrap();
  let reopened_font = reopened_font_table
    .get_part_by_id(&reopened, font_id.as_str())
    .and_then(PartRef::downcast::<FontPart>)
    .unwrap();
  let reopened_settings = reopened_main
    .get_part_by_id(&reopened, settings_id.as_str())
    .and_then(PartRef::downcast::<DocumentSettingsPart>)
    .unwrap();
  let reopened_recipients = reopened_settings
    .get_part_by_id(&reopened, recipients_id.as_str())
    .and_then(PartRef::downcast::<MailMergeRecipientDataPart>)
    .unwrap();

  assert_eq!(
    reopened_embedded_object.data(&reopened),
    Some(&b"ole object bytes"[..])
  );
  assert_eq!(
    reopened_embedded_package.data(&reopened),
    Some(&b"xlsx package bytes"[..])
  );
  assert_eq!(reopened_font.data(&reopened), Some(&b"ttf bytes"[..]));
  assert_eq!(
    reopened_recipients.data(&reopened),
    Some(&b"<recipients/>"[..])
  );
}

#[test]
fn add_spreadsheet_supported_relationship_parts_by_type_save_and_reopen() {
  // Source: upstream WorksheetPart supported relationship constraints for custom properties and controls.
  let mut package =
    SpreadsheetDocument::new_from_file_lazy(doc_sample("basicspreadsheet.xlsx")).unwrap();
  let workbook_part = package.workbook_part().unwrap();
  let worksheet = workbook_part.worksheet_parts(&package).next().unwrap();

  let custom_property = worksheet
    .add_custom_property_part_by_type(&mut package, CustomPropertyPartType::Spreadsheet)
    .unwrap();
  custom_property
    .set_data(&mut package, b"<customProperty/>".to_vec())
    .unwrap();
  let custom_property_id = worksheet
    .get_id_of_part(&package, custom_property)
    .unwrap()
    .to_string();

  let control = worksheet
    .add_embedded_control_persistence_part_by_type(
      &mut package,
      EmbeddedControlPersistencePartType::ActiveX,
    )
    .unwrap();
  control
    .set_data(&mut package, b"<control/>".to_vec())
    .unwrap();
  let control_id = worksheet
    .get_id_of_part(&package, control)
    .unwrap()
    .to_string();

  let direct_binary = worksheet
    .add_embedded_control_persistence_binary_data_part_by_type(
      &mut package,
      EmbeddedControlPersistenceBinaryDataPartType::ActiveXBin,
    )
    .unwrap();
  direct_binary
    .set_data(&mut package, b"worksheet activeX bin".to_vec())
    .unwrap();
  let direct_binary_id = worksheet
    .get_id_of_part(&package, direct_binary)
    .unwrap()
    .to_string();

  let child_binary = control
    .add_embedded_control_persistence_binary_data_part_by_type(
      &mut package,
      EmbeddedControlPersistenceBinaryDataPartType::ActiveXBin,
    )
    .unwrap();
  child_binary
    .set_data(&mut package, b"control activeX bin".to_vec())
    .unwrap();
  let child_binary_id = control
    .get_id_of_part(&package, child_binary)
    .unwrap()
    .to_string();

  assert_eq!(
    custom_property.content_type(&package),
    Some("application/vnd.openxmlformats-officedocument.spreadsheetml.customProperty")
  );
  assert_eq!(
    control.content_type(&package),
    Some("application/vnd.ms-office.activeX+xml")
  );
  assert_eq!(
    direct_binary.content_type(&package),
    Some("application/vnd.ms-office.activeX")
  );
  assert!(
    package
      .storage()
      .part(custom_property.part_id())
      .unwrap()
      .path()
      .ends_with(".xml")
  );
  assert!(
    package
      .storage()
      .part(control.part_id())
      .unwrap()
      .path()
      .ends_with(".xml")
  );
  assert!(
    package
      .storage()
      .part(direct_binary.part_id())
      .unwrap()
      .path()
      .ends_with(".bin")
  );

  let mut buffer = Cursor::new(Vec::new());
  package.save(&mut buffer).unwrap();

  let reopened = SpreadsheetDocument::new_lazy(Cursor::new(buffer.into_inner())).unwrap();
  let reopened_workbook = reopened.workbook_part().unwrap();
  let reopened_worksheet = reopened_workbook.worksheet_parts(&reopened).next().unwrap();
  let reopened_custom_property = reopened_worksheet
    .get_part_by_id(&reopened, custom_property_id.as_str())
    .and_then(PartRef::downcast::<CustomPropertyPart>)
    .unwrap();
  let reopened_control = reopened_worksheet
    .get_part_by_id(&reopened, control_id.as_str())
    .and_then(PartRef::downcast::<EmbeddedControlPersistencePart>)
    .unwrap();
  let reopened_direct_binary = reopened_worksheet
    .get_part_by_id(&reopened, direct_binary_id.as_str())
    .and_then(PartRef::downcast::<EmbeddedControlPersistenceBinaryDataPart>)
    .unwrap();
  let reopened_child_binary = reopened_control
    .get_part_by_id(&reopened, child_binary_id.as_str())
    .and_then(PartRef::downcast::<EmbeddedControlPersistenceBinaryDataPart>)
    .unwrap();

  assert_eq!(
    reopened_custom_property.data(&reopened),
    Some(&b"<customProperty/>"[..])
  );
  assert_eq!(reopened_control.data(&reopened), Some(&b"<control/>"[..]));
  assert_eq!(
    reopened_direct_binary.data(&reopened),
    Some(&b"worksheet activeX bin"[..])
  );
  assert_eq!(
    reopened_child_binary.data(&reopened),
    Some(&b"control activeX bin"[..])
  );
}

#[test]
fn add_thumbnail_part_by_type_uses_jpeg_content_type_and_extension() {
  // Source: upstream AddThumbnailPart(ThumbnailPartType.Jpeg) package coverage.
  let mut package =
    WordprocessingDocument::new_from_file_lazy(doc_sample("Hyperlink.docx")).unwrap();
  let relationship_count = package.relationships().len();
  let jpeg = b"thumbnail jpeg bytes".to_vec();

  let thumbnail = package
    .add_thumbnail_part_by_type(ThumbnailPartType::Jpeg)
    .unwrap();
  thumbnail
    .feed_data(&mut package, &mut Cursor::new(jpeg.clone()))
    .unwrap();
  let relationship_id = package.get_id_of_part(thumbnail).unwrap().to_string();
  let thumbnail_path = package
    .storage()
    .part(thumbnail.part_id())
    .unwrap()
    .path()
    .to_string();

  assert!(relationship_id.starts_with("rId"));
  assert_eq!(package.relationships().len(), relationship_count + 1);
  assert_eq!(thumbnail.content_type(&package), Some("image/jpeg"));
  assert!(thumbnail_path.starts_with("docProps/thumbnail"));
  assert!(thumbnail_path.ends_with(".jpg"));

  let mut buffer = Cursor::new(Vec::new());
  package.save(&mut buffer).unwrap();

  let reopened = WordprocessingDocument::new(Cursor::new(buffer.into_inner())).unwrap();
  let reopened_thumbnail = reopened
    .get_part_by_id(relationship_id.as_str())
    .and_then(PartRef::downcast::<ThumbnailPart>)
    .unwrap();

  assert_eq!(
    reopened_thumbnail.content_type(&reopened),
    Some("image/jpeg")
  );
  assert_eq!(reopened_thumbnail.data(&reopened), Some(jpeg.as_slice()));
}

#[test]
fn add_thumbnail_part_with_id_uses_content_type_and_relationship_id() {
  // Source: upstream AddThumbnailPart(contentType) package coverage.
  let mut package =
    WordprocessingDocument::new_from_file_lazy(doc_sample("Hyperlink.docx")).unwrap();
  let relationship_id = "rIdSdkThumbnail";
  let jpeg = b"thumbnail jpg bytes".to_vec();

  let thumbnail = package
    .add_thumbnail_part_with_id("image/jpg", relationship_id)
    .unwrap();
  thumbnail.set_data(&mut package, jpeg.clone()).unwrap();

  assert_eq!(package.get_id_of_part(thumbnail), Some(relationship_id));
  assert_eq!(thumbnail.content_type(&package), Some("image/jpg"));

  let mut buffer = Cursor::new(Vec::new());
  package.save(&mut buffer).unwrap();

  let reopened = WordprocessingDocument::new(Cursor::new(buffer.into_inner())).unwrap();
  let reopened_thumbnail = reopened
    .get_part_by_id(relationship_id)
    .and_then(PartRef::downcast::<ThumbnailPart>)
    .unwrap();

  assert_eq!(
    reopened_thumbnail.content_type(&reopened),
    Some("image/jpg")
  );
  assert_eq!(reopened_thumbnail.data(&reopened), Some(jpeg.as_slice()));
}

#[test]
fn add_extended_part_with_id_supports_package_part_and_nested_extended_parts() {
  // Source: upstream AddExtendedPart(..., rId) coverage on packages, parts, and ExtendedPart.
  let mut package = WordprocessingDocument::new(empty_package()).unwrap();
  let main_part = package.add_main_document_part().unwrap();
  main_part
    .set_root_element(&mut package, empty_body_document())
    .unwrap();

  let package_extended = package
    .add_extended_part_with_id(
      "http://temp/package",
      "text/xml",
      ".xml",
      "rIdSdkPackageExtended",
    )
    .unwrap();
  package_extended
    .set_data(&mut package, b"<packageExtended/>".to_vec())
    .unwrap();

  let part_extended = main_part
    .add_extended_part_with_id(
      &mut package,
      "http://temp/main",
      "application/custom+xml",
      "xml",
      "rIdSdkMainExtended",
    )
    .unwrap();
  part_extended
    .set_data(&mut package, b"<mainExtended/>".to_vec())
    .unwrap();

  let nested_extended = part_extended
    .add_extended_part_with_id(
      &mut package,
      "http://temp/nested",
      "text/plain",
      "txt",
      "rIdSdkNestedExtended",
    )
    .unwrap();
  nested_extended
    .set_data(&mut package, b"nested extended".to_vec())
    .unwrap();

  assert_eq!(
    package.get_id_of_part(package_extended),
    Some("rIdSdkPackageExtended")
  );
  assert_eq!(
    main_part.get_id_of_part(&package, part_extended),
    Some("rIdSdkMainExtended")
  );
  assert_eq!(
    part_extended.get_id_of_part(&package, nested_extended),
    Some("rIdSdkNestedExtended")
  );
  assert_eq!(package_extended.content_type(&package), Some("text/xml"));
  assert_eq!(
    part_extended.content_type(&package),
    Some("application/custom+xml")
  );
  assert_eq!(nested_extended.content_type(&package), Some("text/plain"));
  assert!(
    package_extended
      .path(&package)
      .is_some_and(|path| path.starts_with("extendedPart") && path.ends_with(".xml"))
  );
  assert!(
    part_extended
      .path(&package)
      .is_some_and(|path| path.starts_with("word/extendedPart") && path.ends_with(".xml"))
  );
  assert!(
    nested_extended
      .path(&package)
      .is_some_and(|path| path.starts_with("word/extendedPart") && path.ends_with(".txt"))
  );

  let package_relationship = package
    .relationships()
    .get("rIdSdkPackageExtended")
    .unwrap();
  assert_eq!(
    package_relationship.relationship_type(),
    "http://temp/package"
  );
  let part_relationship = main_part
    .relationships(&package)
    .unwrap()
    .get("rIdSdkMainExtended")
    .unwrap();
  assert_eq!(part_relationship.relationship_type(), "http://temp/main");
  let nested_relationship = part_extended
    .relationships(&package)
    .unwrap()
    .get("rIdSdkNestedExtended")
    .unwrap();
  assert_eq!(
    nested_relationship.relationship_type(),
    "http://temp/nested"
  );

  let mut buffer = Cursor::new(Vec::new());
  package.save(&mut buffer).unwrap();

  let reopened = WordprocessingDocument::new_lazy(Cursor::new(buffer.into_inner())).unwrap();
  let reopened_package_extended = reopened
    .get_part_by_id("rIdSdkPackageExtended")
    .and_then(PartRef::downcast::<ExtendedPart>)
    .unwrap();
  let reopened_main = reopened.main_document_part().unwrap();
  let reopened_part_extended = reopened_main
    .get_part_by_id(&reopened, "rIdSdkMainExtended")
    .and_then(PartRef::downcast::<ExtendedPart>)
    .unwrap();
  let reopened_nested_extended = reopened_part_extended
    .get_part_by_id(&reopened, "rIdSdkNestedExtended")
    .and_then(PartRef::downcast::<ExtendedPart>)
    .unwrap();

  assert_eq!(
    reopened_package_extended.data(&reopened),
    Some(&b"<packageExtended/>"[..])
  );
  assert_eq!(
    reopened_part_extended.data(&reopened),
    Some(&b"<mainExtended/>"[..])
  );
  assert_eq!(
    reopened_nested_extended.data(&reopened),
    Some(&b"nested extended"[..])
  );
}

#[test]
fn delete_package_part_removes_relationship_part_and_content_type() {
  // Source: upstream Docx/Xlsx/Pptx DeletePart(package part) coverage.
  let mut package = WordprocessingDocument::new(empty_package()).unwrap();
  let thumbnail = package
    .add_thumbnail_part_with_id("image/jpeg", "rIdSdkThumbnail")
    .unwrap();
  thumbnail
    .set_data(&mut package, b"thumbnail".to_vec())
    .unwrap();
  let thumbnail_path = thumbnail.path(&package).unwrap().to_string();

  assert!(package.delete_part(thumbnail).unwrap());
  assert!(!package.delete_part_by_id("rIdSdkThumbnail").unwrap());
  assert!(package.get_part_by_id("rIdSdkThumbnail").is_none());
  assert!(package.storage().part(thumbnail.part_id()).is_none());
  assert!(
    !package
      .storage()
      .content_types()
      .xml_children
      .iter()
      .any(|child| matches!(
        child,
        ooxmlsdk::schemas::opc_content_types::TypesChoice::Override(override_type)
          if override_type.part_name == format!("/{thumbnail_path}")
      ))
  );

  let mut buffer = Cursor::new(Vec::new());
  package.save(&mut buffer).unwrap();
  let bytes = buffer.into_inner();

  assert!(!package_entry_exists(bytes.clone(), &thumbnail_path));
  assert!(!package_entry_exists(bytes, "_rels/.rels"));
}

#[test]
fn delete_child_parts_removes_unreachable_descendants_and_supports_batches() {
  // Source: upstream OpenXmlPartContainer.DeletePart/DeleteParts and ExtendedPart child coverage.
  let mut package = WordprocessingDocument::new(empty_package()).unwrap();
  let main_part = package.add_main_document_part().unwrap();
  main_part
    .set_root_element(&mut package, empty_body_document())
    .unwrap();

  let extended = main_part
    .add_extended_part_with_id(&mut package, "http://temp", "text/xml", ".xml", "tempId")
    .unwrap();
  extended
    .set_data(&mut package, b"<extended/>".to_vec())
    .unwrap();
  let nested = extended
    .add_extended_part_with_id(
      &mut package,
      "http://temp/nested",
      "text/xml",
      ".xml",
      "tempId2",
    )
    .unwrap();
  nested
    .set_data(&mut package, b"<nested/>".to_vec())
    .unwrap();
  let extended_path = extended.path(&package).unwrap().to_string();
  let nested_path = nested.path(&package).unwrap().to_string();

  assert!(extended.delete_part(&mut package, nested).unwrap());
  assert!(!extended.delete_part_by_id(&mut package, "tempId2").unwrap());
  assert!(extended.get_part_by_id(&package, "tempId2").is_none());
  assert!(package.storage().part(nested.part_id()).is_none());

  assert!(main_part.delete_part(&mut package, extended).unwrap());
  assert!(main_part.get_part_by_id(&package, "tempId").is_none());
  assert!(package.storage().part(extended.part_id()).is_none());

  let image1 = main_part
    .add_image_part_with_id(&mut package, "image/png", "rIdDeleteImage1")
    .unwrap();
  let image2 = main_part
    .add_image_part_with_id(&mut package, "image/png", "rIdDeleteImage2")
    .unwrap();
  let image1_path = image1.path(&package).unwrap().to_string();
  let image2_path = image2.path(&package).unwrap().to_string();
  main_part
    .delete_parts::<_, ImagePart, _>(&mut package, [image1, image2])
    .unwrap();
  assert!(
    main_part
      .get_part_by_id(&package, "rIdDeleteImage1")
      .is_none()
  );
  assert!(
    main_part
      .get_part_by_id(&package, "rIdDeleteImage2")
      .is_none()
  );

  let mut buffer = Cursor::new(Vec::new());
  package.save(&mut buffer).unwrap();
  let bytes = buffer.into_inner();

  assert!(!package_entry_exists(bytes.clone(), &nested_path));
  assert!(!package_entry_exists(bytes.clone(), &extended_path));
  assert!(!package_entry_exists(bytes.clone(), &image1_path));
  assert!(!package_entry_exists(bytes, &image2_path));
}

#[test]
fn delete_parts_of_type_only_removes_direct_children() {
  // Source: upstream OpenXmlPartContainer.DeletePartsOfType<T>() direct-child semantics.
  let mut package = WordprocessingDocument::new(empty_package()).unwrap();
  let main_part = package.add_main_document_part().unwrap();
  main_part
    .set_root_element(&mut package, empty_body_document())
    .unwrap();

  let direct_image = main_part
    .add_image_part_with_id(&mut package, "image/png", "rIdDirectImage")
    .unwrap();
  direct_image
    .set_data(&mut package, b"direct".to_vec())
    .unwrap();
  let extended = main_part
    .add_extended_part_with_id(
      &mut package,
      "http://temp",
      "text/xml",
      ".xml",
      "rIdExtended",
    )
    .unwrap();
  let nested_image = extended
    .add_image_part_with_id(&mut package, "image/png", "rIdNestedImage")
    .unwrap();
  nested_image
    .set_data(&mut package, b"nested".to_vec())
    .unwrap();
  let direct_image_path = direct_image.path(&package).unwrap().to_string();
  let nested_image_path = nested_image.path(&package).unwrap().to_string();

  main_part
    .delete_parts_of_type::<_, ImagePart>(&mut package)
    .unwrap();

  assert!(
    main_part
      .get_part_by_id(&package, "rIdDirectImage")
      .is_none()
  );
  assert!(package.storage().part(direct_image.part_id()).is_none());
  assert!(
    extended
      .get_part_by_id(&package, "rIdNestedImage")
      .and_then(PartRef::downcast::<ImagePart>)
      .is_some()
  );

  let mut buffer = Cursor::new(Vec::new());
  package.save(&mut buffer).unwrap();
  let bytes = buffer.into_inner();

  assert!(!package_entry_exists(bytes.clone(), &direct_image_path));
  assert!(package_entry_exists(bytes, &nested_image_path));
}

#[test]
fn delete_parts_recursively_of_type_removes_descendant_matches() {
  // Source: upstream OpenXmlPackage.DeletePartsRecursivelyOfType<T>() traversal semantics.
  let mut package = WordprocessingDocument::new(empty_package()).unwrap();
  let main_part = package.add_main_document_part().unwrap();
  main_part
    .set_root_element(&mut package, empty_body_document())
    .unwrap();

  let direct_image = main_part
    .add_image_part_with_id(&mut package, "image/png", "rIdDirectImage")
    .unwrap();
  let extended = main_part
    .add_extended_part_with_id(
      &mut package,
      "http://temp",
      "text/xml",
      ".xml",
      "rIdExtended",
    )
    .unwrap();
  let nested_image = extended
    .add_image_part_with_id(&mut package, "image/png", "rIdNestedImage")
    .unwrap();
  let direct_image_path = direct_image.path(&package).unwrap().to_string();
  let nested_image_path = nested_image.path(&package).unwrap().to_string();

  package
    .delete_parts_recursively_of_type::<ImagePart>()
    .unwrap();

  assert!(
    main_part
      .get_part_by_id(&package, "rIdDirectImage")
      .is_none()
  );
  assert!(
    extended
      .get_part_by_id(&package, "rIdNestedImage")
      .is_none()
  );
  assert!(package.storage().part(direct_image.part_id()).is_none());
  assert!(package.storage().part(nested_image.part_id()).is_none());
  assert!(main_part.get_part_by_id(&package, "rIdExtended").is_some());

  let mut buffer = Cursor::new(Vec::new());
  package.save(&mut buffer).unwrap();
  let bytes = buffer.into_inner();

  assert!(!package_entry_exists(bytes.clone(), &direct_image_path));
  assert!(!package_entry_exists(bytes, &nested_image_path));
}

#[test]
fn add_part_and_create_relationship_to_part_share_existing_parts() {
  // Source: upstream AddPart(existing part) / CreateRelationshipToPart same-package semantics.
  let mut package = WordprocessingDocument::new(empty_package()).unwrap();
  let main_part = package.add_main_document_part().unwrap();
  main_part
    .set_root_element(&mut package, empty_body_document())
    .unwrap();

  let image = main_part
    .add_image_part_with_id(&mut package, "image/png", "rIdMainImage")
    .unwrap();
  image
    .set_data(&mut package, b"shared image".to_vec())
    .unwrap();
  let image_path = image.path(&package).unwrap().to_string();
  let extended = main_part
    .add_extended_part_with_id(
      &mut package,
      "http://temp",
      "text/xml",
      ".xml",
      "rIdExtended",
    )
    .unwrap();

  let shared_id = extended
    .create_relationship_to_part_with_id(&mut package, image, "rIdSharedImage")
    .unwrap();
  assert_eq!(shared_id, "rIdSharedImage");
  assert_eq!(
    extended
      .create_relationship_to_part(&mut package, image)
      .unwrap(),
    "rIdSharedImage"
  );
  assert!(
    extended
      .add_part_with_id(&mut package, image, "rIdDifferentSharedImage")
      .is_err()
  );
  assert_eq!(
    extended.get_id_of_part(&package, image),
    Some("rIdSharedImage")
  );

  assert!(main_part.delete_part(&mut package, image).unwrap());
  assert!(main_part.get_part_by_id(&package, "rIdMainImage").is_none());
  assert_eq!(image.data(&package), Some(&b"shared image"[..]));

  let mut shared_buffer = Cursor::new(Vec::new());
  package.save(&mut shared_buffer).unwrap();
  assert!(package_entry_exists(
    shared_buffer.into_inner(),
    image_path.as_str()
  ));

  assert!(extended.delete_part(&mut package, image).unwrap());
  assert!(package.storage().part(image.part_id()).is_none());

  let mut deleted_buffer = Cursor::new(Vec::new());
  package.save(&mut deleted_buffer).unwrap();
  assert!(!package_entry_exists(
    deleted_buffer.into_inner(),
    image_path.as_str()
  ));
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
fn add_file_properties_and_signature_origin_parts_create_fixed_package_relationships() {
  // Source: upstream W050/X006 AddCoreFilePropertiesPart/AddExtendedFilePropertiesPart/AddCustomFilePropertiesPart/AddDigitalSignatureOriginPart coverage.
  let mut package = WordprocessingDocument::new(empty_package()).unwrap();
  let main_part = package.add_main_document_part().unwrap();
  main_part
    .set_root_element(&mut package, empty_body_document())
    .unwrap();

  let core = package.add_core_file_properties_part().unwrap();
  core
    .set_data(
      &mut package,
      br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<cp:coreProperties xmlns:cp="http://schemas.openxmlformats.org/package/2006/metadata/core-properties" xmlns:dc="http://purl.org/dc/elements/1.1/"><dc:title>hello</dc:title></cp:coreProperties>"#
        .to_vec(),
    )
    .unwrap();

  let extended = package.add_extended_file_properties_part().unwrap();
  extended
    .set_data(
      &mut package,
      br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Properties xmlns="http://schemas.openxmlformats.org/officeDocument/2006/extended-properties"><Application>ooxmlsdk</Application></Properties>"#
        .to_vec(),
    )
    .unwrap();

  let custom = package.add_custom_file_properties_part().unwrap();
  custom
    .set_data(
      &mut package,
      br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Properties xmlns="http://schemas.openxmlformats.org/officeDocument/2006/custom-properties" xmlns:vt="http://schemas.openxmlformats.org/officeDocument/2006/docPropsVTypes"><property fmtid="{D5CDD505-2E9C-101B-9397-08002B2CF9AE}" pid="2" name="Sdk"><vt:lpwstr>yes</vt:lpwstr></property></Properties>"#
        .to_vec(),
    )
    .unwrap();
  let signature_origin = package.add_digital_signature_origin_part().unwrap();

  assert_eq!(core.path(&package), Some("docProps/core.xml"));
  assert_eq!(extended.path(&package), Some("docProps/app.xml"));
  assert_eq!(custom.path(&package), Some("docProps/custom.xml"));
  assert_eq!(
    signature_origin.path(&package),
    Some("_xmlsignatures/origin.sigs")
  );
  assert_eq!(
    core.content_type(&package),
    Some("application/vnd.openxmlformats-package.core-properties+xml")
  );
  assert_eq!(
    extended.content_type(&package),
    Some("application/vnd.openxmlformats-officedocument.extended-properties+xml")
  );
  assert_eq!(
    custom.content_type(&package),
    Some("application/vnd.openxmlformats-officedocument.custom-properties+xml")
  );
  assert_eq!(
    signature_origin.content_type(&package),
    Some("application/vnd.openxmlformats-package.digital-signature-origin")
  );
  assert!(package.add_core_file_properties_part().is_err());
  assert!(package.add_extended_file_properties_part().is_err());
  assert!(package.add_custom_file_properties_part().is_err());
  assert!(package.add_digital_signature_origin_part().is_err());

  let core_id = package.get_id_of_part(core).unwrap().to_string();
  let extended_id = package.get_id_of_part(extended).unwrap().to_string();
  let custom_id = package.get_id_of_part(custom).unwrap().to_string();
  let signature_origin_id = package
    .get_id_of_part(signature_origin)
    .unwrap()
    .to_string();
  let mut buffer = Cursor::new(Vec::new());
  package.save(&mut buffer).unwrap();

  let reopened = WordprocessingDocument::new_lazy(Cursor::new(buffer.into_inner())).unwrap();
  let reopened_core = reopened
    .get_part_by_id(core_id.as_str())
    .and_then(PartRef::downcast::<CoreFilePropertiesPart>)
    .unwrap();
  let reopened_extended = reopened
    .get_part_by_id(extended_id.as_str())
    .and_then(PartRef::downcast::<ExtendedFilePropertiesPart>)
    .unwrap();
  let reopened_custom = reopened
    .get_part_by_id(custom_id.as_str())
    .and_then(PartRef::downcast::<CustomFilePropertiesPart>)
    .unwrap();
  let reopened_signature_origin = reopened
    .get_part_by_id(signature_origin_id.as_str())
    .and_then(PartRef::downcast::<DigitalSignatureOriginPart>)
    .unwrap();

  assert_eq!(reopened_core.path(&reopened), Some("docProps/core.xml"));
  assert_eq!(reopened_extended.path(&reopened), Some("docProps/app.xml"));
  assert_eq!(reopened_custom.path(&reopened), Some("docProps/custom.xml"));
  assert_eq!(
    reopened_signature_origin.path(&reopened),
    Some("_xmlsignatures/origin.sigs")
  );
  assert!(reopened_core.data(&reopened).is_some_and(|data| {
    data
      .windows(b"hello".len())
      .any(|window| window == b"hello")
  }));
  assert!(reopened_extended.data(&reopened).is_some_and(|data| {
    data
      .windows(b"ooxmlsdk".len())
      .any(|window| window == b"ooxmlsdk")
  }));
  assert!(
    reopened_custom
      .data(&reopened)
      .is_some_and(|data| data.windows(b"Sdk".len()).any(|window| window == b"Sdk"))
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
