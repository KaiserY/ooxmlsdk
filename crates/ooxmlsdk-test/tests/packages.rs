#![cfg(feature = "parts")]

use std::collections::HashSet;
use std::io::{Cursor, Write};

use ooxmlsdk::common::{
  MediaDataPart, PartId, ReferenceRelationshipKind, RelationshipRef, RelationshipTargetKind,
};
use ooxmlsdk::parts::{
  PartRef, custom_xml_part::CustomXmlPart, document_settings_part::DocumentSettingsPart,
  font_table_part::FontTablePart, header_part::HeaderPart, image_part::ImagePart,
  main_document_part::MainDocumentPart, presentation_document::PresentationDocument,
  presentation_part::PresentationPart, ribbon_extensibility_part::RibbonExtensibilityPart,
  slide_part::SlidePart, spreadsheet_document::SpreadsheetDocument,
  style_definitions_part::StyleDefinitionsPart, thumbnail_part::ThumbnailPart,
  wordprocessing_document::WordprocessingDocument,
};
use ooxmlsdk::schemas::opc_relationships::TargetMode;
#[cfg(feature = "microsoft365")]
use ooxmlsdk::schemas::schemas_microsoft_com_office_drawing_2014_chartex::SeriesLayout;
use ooxmlsdk::schemas::schemas_openxmlformats_org_presentationml_2006_main::{
  Presentation as PmlPresentation, Slide,
};
use ooxmlsdk::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::{
  Body, BodyChoice, Document, Header, SdtPropertiesChoice,
};
use ooxmlsdk::sdk::{
  AlternativeFormatImportPartType, CustomPropertyPartType, CustomXmlPartType,
  EmbeddedControlPersistenceBinaryDataPartType, EmbeddedControlPersistencePartType,
  EmbeddedObjectPartType, EmbeddedPackagePartType, FontPartType, MailMergeRecipientDataPartType,
  MediaDataPartType, OpenSettings, SdkPackage, SdkPart, ThumbnailPartType,
};
use ooxmlsdk::sdk::{
  FileFormatVersion, MarkupCompatibilityProcessMode, MarkupCompatibilityProcessSettings,
};
use ooxmlsdk_test::fixtures;

macro_rules! part_ref_variant {
  ($part_ref:expr, $variant:ident) => {
    match $part_ref {
      PartRef::$variant(part) => Some(part),
      _ => None,
    }
  };
}

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

fn media_data_part_by_id<P: SdkPackage>(package: &P, part_id: PartId) -> MediaDataPart {
  package
    .media_data_parts()
    .find(|part| part.part_id() == Some(part_id))
    .unwrap()
}

fn part_relationship_count<P, T>(package: &P, part: &T) -> usize
where
  P: SdkPackage,
  T: SdkPart,
{
  part.parts(package).count()
    + part.external_relationships(package).count()
    + part.hyperlink_relationships(package).count()
    + part.data_part_reference_relationships(package).count()
}

fn package_entry_exists(bytes: Vec<u8>, path: &str) -> bool {
  let mut archive = zip::ZipArchive::new(Cursor::new(bytes)).unwrap();
  archive.by_name(path).is_ok()
}

#[test]
fn wordprocessing_document_loads_minimal_flat_opc_package() {
  // Source: mirrors Open XML SDK FlatOpcExtensions.FromFlatOpcString coverage.
  let flat_opc = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<pkg:package xmlns:pkg="http://schemas.microsoft.com/office/2006/xmlPackage">
  <pkg:part pkg:name="/_rels/.rels" pkg:contentType="application/vnd.openxmlformats-package.relationships+xml">
    <pkg:xmlData>
      <Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
        <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="word/document.xml"/>
      </Relationships>
    </pkg:xmlData>
  </pkg:part>
  <pkg:part pkg:name="/word/document.xml" pkg:contentType="application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml">
    <pkg:xmlData>
      <w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
        <w:body>
          <w:p>
            <w:r>
              <w:t>Hello Flat OPC</w:t>
            </w:r>
          </w:p>
        </w:body>
      </w:document>
    </pkg:xmlData>
  </pkg:part>
</pkg:package>"#;

  let mut package = WordprocessingDocument::from_flat_opc_str(flat_opc).unwrap();
  let main_part = package.main_document_part().unwrap();
  let root = main_part.root_element(&mut package).unwrap();

  assert_eq!(main_document_body_child_count(root), 1);

  let saved = package.to_package_bytes().unwrap();
  let reopened = WordprocessingDocument::new(Cursor::new(saved)).unwrap();
  assert!(reopened.main_document_part().is_ok());
}

#[test]
fn wordprocessing_document_flat_opc_round_trips_xml_and_binary_parts() {
  // Source: aligned with Open XML SDK DocumentTests.FlatOpcTests and SVG/binary Flat OPC coverage.
  let mut package = WordprocessingDocument::new(empty_package()).unwrap();
  let main_part = package.add_main_document_part().unwrap();
  main_part
    .set_root_element(&mut package, empty_body_document())
    .unwrap();
  let image_part = main_part.add_image_part(&mut package, "image/png").unwrap();
  image_part
    .set_data(&mut package, [0_u8, 1, 2, 3, 250, 251, 252])
    .unwrap();
  let image_part_name = format!("/{}", image_part.path(&package).unwrap());

  let flat_opc = package.to_flat_opc_string().unwrap();
  assert!(flat_opc.contains("<pkg:package"));
  assert!(flat_opc.contains("pkg:name=\"/word/document.xml\""));
  assert!(flat_opc.contains(&format!("pkg:name=\"{image_part_name}\"")));
  assert!(flat_opc.contains("<pkg:binaryData>"));

  let reopened = WordprocessingDocument::from_flat_opc_str(&flat_opc).unwrap();
  let reopened_main = reopened.main_document_part().unwrap();
  let reopened_image = reopened_main
    .parts(&reopened)
    .find_map(|part| part_ref_variant!(part.part, ImagePart))
    .unwrap();

  assert_eq!(
    reopened_image.data(&reopened).unwrap(),
    [0_u8, 1, 2, 3, 250, 251, 252]
  );
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
fn wordprocessing_part_unload_root_element_matches_openxml_part_test() {
  // Source: test/DocumentFormat.OpenXml.Tests/ofapiTest/OpenXmlPartTest.cs
  //   UnloadRootElementTest
  let mut package =
    WordprocessingDocument::new_from_file_lazy(doc_sample("Document.docx")).unwrap();
  let main_part = package.main_document_part().unwrap();

  assert!(!main_part.is_root_element_loaded(&package));
  assert!(main_part.unload_root_element(&mut package).is_none());

  let original_child_count = {
    let root = main_part.root_element(&mut package).unwrap();
    main_document_body_child_count(root)
  };
  assert!(original_child_count > 0);
  assert!(main_part.is_root_element_loaded(&package));

  let unloaded_root = main_part.unload_root_element(&mut package).unwrap();
  assert_eq!(unloaded_root.part_type_name(), "MainDocumentPart");
  assert!(!main_part.is_root_element_loaded(&package));

  let reloaded_child_count = {
    let root = main_part.root_element(&mut package).unwrap();
    main_document_body_child_count(root)
  };
  assert_eq!(reloaded_child_count, original_child_count);
  assert!(main_part.is_root_element_loaded(&package));
}

#[test]
fn wordprocessing_mce_packages_open_save_and_reopen_from_autosave_tests() {
  // Source: test/DocumentFormat.OpenXml.Tests/Documents/DocumentTests.Autosave.cs
  //   OpenMcPackage
  for file_name in ["mcdoc.docx", "mcinleaf.docx"] {
    let mut package = WordprocessingDocument::new_from_file_lazy_with_settings(
      doc_sample(file_name),
      OpenSettings::default(),
    )
    .unwrap();
    let main_part = package.main_document_part().unwrap();
    assert!(main_part.root_element(&mut package).is_ok());

    let mut saved = Cursor::new(Vec::new());
    package.save(&mut saved).unwrap();
    saved.set_position(0);

    let mut reopened = WordprocessingDocument::new_lazy(saved).unwrap();
    let reopened_main = reopened.main_document_part().unwrap();
    assert!(reopened_main.root_element(&mut reopened).is_ok());
  }
}

#[test]
fn open_settings_default_markup_compatibility_matches_upstream() {
  let settings = OpenSettings::default();

  assert_eq!(
    settings.markup_compatibility_process_settings,
    MarkupCompatibilityProcessSettings {
      process_mode: MarkupCompatibilityProcessMode::NoProcess,
      target_file_format_version: FileFormatVersion::Office2007,
    }
  );

  let implicit = WordprocessingDocument::new_from_file_lazy(doc_sample("Document.docx")).unwrap();
  assert_eq!(implicit.open_settings(), &settings);

  let explicit =
    WordprocessingDocument::new_from_file_lazy_with_settings(doc_sample("Document.docx"), settings)
      .unwrap();
  assert_eq!(explicit.open_settings(), &settings);

  let spreadsheet =
    SpreadsheetDocument::new_from_file_lazy(doc_sample("basicspreadsheet.xlsx")).unwrap();
  assert_eq!(spreadsheet.open_settings(), &settings);

  let presentation =
    PresentationDocument::new_from_file_lazy(doc_sample("Presentation.pptx")).unwrap();
  assert_eq!(presentation.open_settings(), &settings);
}

#[test]
fn open_settings_no_process_accepts_all_supported_target_versions() {
  // Source: test/DocumentFormat.OpenXml.Tests/OpenXmlDomTest/OpenSettingsTestClass.cs
  //   OpenWithFileFormatVersionsDefaultValue
  // Rust exposes FileFormatVersion as a closed enum, so invalid integer values
  // from the .NET test cannot be represented here.
  for target_file_format_version in [
    FileFormatVersion::Office2007,
    FileFormatVersion::Office2010,
    FileFormatVersion::Office2013,
    FileFormatVersion::Office2016,
    FileFormatVersion::Office2019,
    FileFormatVersion::Office2021,
    FileFormatVersion::Microsoft365,
  ] {
    let settings = OpenSettings {
      markup_compatibility_process_settings: MarkupCompatibilityProcessSettings {
        process_mode: MarkupCompatibilityProcessMode::NoProcess,
        target_file_format_version,
      },
      ..Default::default()
    };
    let package = WordprocessingDocument::new_from_file_lazy_with_settings(
      doc_sample("Document.docx"),
      settings,
    )
    .unwrap();

    assert_eq!(package.open_settings(), &settings);
  }
}

#[test]
fn package_to_owned_package_preserves_open_settings() {
  // Source: test/DocumentFormat.OpenXml.Tests/Documents/FlatOpcAndCloningTests.cs
  //   CanCloneDocxDocument
  // and src/DocumentFormat.OpenXml.Framework/Packaging/CloneableExtensions.cs
  //   clone overloads preserve OpenSettings.
  let settings = OpenSettings {
    markup_compatibility_process_settings: MarkupCompatibilityProcessSettings {
      process_mode: MarkupCompatibilityProcessMode::NoProcess,
      target_file_format_version: FileFormatVersion::Office2013,
    },
    ignore_calculation_chain_part_relationship: true,
  };
  let package =
    WordprocessingDocument::new_from_file_lazy_with_settings(doc_sample("Document.docx"), settings)
      .unwrap();

  let cloned = package.to_owned_package().unwrap();

  assert_eq!(cloned.open_settings(), &settings);
  assert!(cloned.main_document_part().is_ok());
}

#[test]
fn spreadsheet_missing_calc_chain_part_respects_open_settings() {
  // Source: test/DocumentFormat.OpenXml.Packaging.Tests/OpenXmlPackageTests.cs
  //   ThrowWithMissingCalcChainPart
  //   SucceedWithMissingCalcChainPart
  let path = doc_sample("missingcalcchainpart.xlsx");

  let mut default_package = SpreadsheetDocument::new_from_file_lazy(&path).unwrap();
  assert!(default_package.load_all_parts().is_err());

  let settings = OpenSettings {
    ignore_calculation_chain_part_relationship: true,
    ..Default::default()
  };
  let mut ignored_package =
    SpreadsheetDocument::new_from_file_lazy_with_settings(&path, settings).unwrap();
  assert_eq!(ignored_package.open_settings(), &settings);
  ignored_package.load_all_parts().unwrap();
  let workbook_part = ignored_package.workbook_part().unwrap();
  assert!(
    workbook_part
      .calculation_chain_part(&ignored_package)
      .is_none()
  );
}

#[test]
fn wordprocessing_sdt_alias_mutation_is_saved_from_mc_support_test() {
  // Source: test/DocumentFormat.OpenXml.Tests/ofapiTest/MCSupport.cs
  //   ParticalProperty
  let mut package =
    WordprocessingDocument::new_from_file_lazy(doc_sample("simpleSdt.docx")).unwrap();
  let main_part = package.main_document_part().unwrap();
  let root = main_part.root_element_mut(&mut package).unwrap();
  let sdt = root
    .body
    .as_mut()
    .and_then(|body| body.body_choice.first_mut())
    .and_then(|choice| match choice {
      BodyChoice::WSdt(sdt) => Some(sdt.as_mut()),
      _ => None,
    })
    .unwrap();
  let alias = sdt
    .sdt_properties
    .as_mut()
    .and_then(|properties| properties.xml_children.first_mut())
    .and_then(|choice| match choice {
      SdtPropertiesChoice::WAlias(alias) => Some(alias.as_mut()),
      _ => None,
    })
    .unwrap();

  assert_eq!(alias.val.as_str(), "SDT1");
  alias.val = "newsdt".to_string();

  let mut saved = Cursor::new(Vec::new());
  package.save(&mut saved).unwrap();
  saved.set_position(0);

  let mut reopened = WordprocessingDocument::new_lazy(saved).unwrap();
  let reopened_main = reopened.main_document_part().unwrap();
  let reopened_root = reopened_main.root_element(&mut reopened).unwrap();
  let reopened_alias = reopened_root
    .body
    .as_ref()
    .and_then(|body| body.body_choice.first())
    .and_then(|choice| match choice {
      BodyChoice::WSdt(sdt) => Some(sdt.as_ref()),
      _ => None,
    })
    .and_then(|sdt| sdt.sdt_properties.as_ref())
    .and_then(|properties| properties.xml_children.first())
    .and_then(|choice| match choice {
      SdtPropertiesChoice::WAlias(alias) => Some(alias.as_ref()),
      _ => None,
    })
    .unwrap();

  assert_eq!(reopened_alias.val.as_str(), "newsdt");
}

#[test]
fn wordprocessing_font_table_touch_preserves_w14_namespace_from_mc_support_test() {
  // Source: test/DocumentFormat.OpenXml.Tests/ofapiTest/MCSupport.cs
  //   WriteExtraAttr
  let mut package =
    WordprocessingDocument::new_from_file_lazy(doc_sample("HelloO14.docx")).unwrap();
  let main_part = package.main_document_part().unwrap();
  let font_table_part = main_part.font_table_part(&package).unwrap();
  let fonts = font_table_part.root_element(&mut package).unwrap();
  assert_eq!(fonts.mc_ignorable.as_deref(), Some("w14"));

  let mut saved = Cursor::new(Vec::new());
  package.save(&mut saved).unwrap();
  saved.set_position(0);

  let reopened = WordprocessingDocument::new_lazy(saved).unwrap();
  let reopened_main = reopened.main_document_part().unwrap();
  let reopened_font_table_part = reopened_main.font_table_part(&reopened).unwrap();
  let font_table_xml = reopened_font_table_part
    .data_as_str(&reopened)
    .unwrap()
    .unwrap();

  assert!(
    font_table_xml.contains(r#"xmlns:w14="http://schemas.microsoft.com/office/word/2010/wordml""#)
  );
  assert!(font_table_xml.contains(r#"mc:Ignorable="w14""#));
}

#[test]
fn package_relationships_resolve_with_container_local_part_factory() {
  let package = WordprocessingDocument::new_from_file(doc_sample("Of16-01.docx")).unwrap();
  let main_part = package.main_document_part().unwrap();
  let main_part_id = package.get_id_of_part(&main_part).unwrap();

  let resolved = package
    .try_get_part_by_id(main_part_id)
    .and_then(|part| part_ref_variant!(part, MainDocumentPart))
    .unwrap();

  assert_eq!(resolved.part_id(), main_part.part_id());
  assert!(package.parts().count() > 0);
  assert_eq!(
    package
      .get_parts_of_type::<MainDocumentPart>()
      .next()
      .unwrap()
      .part_id(),
    main_part.part_id()
  );
  assert_eq!(package.get_parts_of_type::<MainDocumentPart>().count(), 1);
  assert!(package.try_get_part_by_id("invalidId").is_none());
}

#[test]
fn package_external_relationship_can_be_edited_and_written_back() {
  // Source: adapted from OpenXmlPackage package relationship mutation coverage.
  let mut package = WordprocessingDocument::new_from_file(doc_sample("Of16-01.docx")).unwrap();
  package
    .add_external_relationship(
      "rIdGraphExternal",
      "http://example.com/relationships/custom",
      "https://example.com/graph",
    )
    .unwrap();

  let mut saved = Cursor::new(Vec::new());
  package.save(&mut saved).unwrap();
  saved.set_position(0);

  let reopened = WordprocessingDocument::new(saved).unwrap();
  let relationship = reopened
    .get_reference_relationship("rIdGraphExternal")
    .unwrap();
  assert_eq!(
    relationship.relationship_type(),
    "http://example.com/relationships/custom"
  );
  assert_eq!(relationship.target(), "https://example.com/graph");
  assert_eq!(relationship.target_kind(), RelationshipTargetKind::External);
  assert!(matches!(relationship.target_mode(), TargetMode::External));
  assert!(matches!(
    relationship.reference_kind(),
    Some(ReferenceRelationshipKind::External)
  ));
}

#[test]
fn package_relationship_helpers_match_openxml_container_api_shape() {
  // Source: adapted from OpenXmlPartContainer external relationship and required lookup APIs.
  let mut package = WordprocessingDocument::new_from_file_lazy(doc_sample("Of16-01.docx")).unwrap();
  let main_part = package.main_document_part().unwrap();
  let main_part_id = package
    .get_id_of_part_required(&main_part)
    .unwrap()
    .to_string();
  assert!(package.get_id_of_part(&main_part).is_some());
  assert!(
    package
      .get_parts_of_type::<MainDocumentPart>()
      .next()
      .is_some()
  );
  assert!(matches!(
    package.get_part_by_id_required(&main_part_id).unwrap(),
    PartRef::MainDocumentPart(_)
  ));
  assert!(package.get_part_by_id_required("rIdMissing").is_err());

  let relationship = package
    .add_external_relationship_auto_id(
      "http://example.com/package/external",
      "https://example.com/package-external",
    )
    .unwrap();
  let relationship_id = relationship.id().to_string();
  assert_eq!(
    package
      .get_external_relationship(&relationship_id)
      .unwrap()
      .target(),
    "https://example.com/package-external"
  );
  assert!(
    package
      .get_hyperlink_relationship(&relationship_id)
      .is_none()
  );
  assert_eq!(
    package
      .delete_external_relationship(&relationship_id)
      .unwrap()
      .id(),
    relationship_id
  );
  assert!(
    package
      .get_external_relationship(&relationship_id)
      .is_none()
  );
}

#[test]
fn package_save_writes_package_relationships_from_parts() {
  let package = WordprocessingDocument::new_from_file(doc_sample("Of16-01.docx")).unwrap();
  let main_part_id = package.main_document_part().unwrap().part_id();
  assert!(
    package
      .parts()
      .any(|entry| entry.part.part_id() == main_part_id)
  );

  let mut saved = Cursor::new(Vec::new());
  package.save(&mut saved).unwrap();
  saved.set_position(0);

  let reopened = WordprocessingDocument::new(saved).unwrap();
  assert_eq!(
    reopened.main_document_part().unwrap().part_id(),
    main_part_id
  );
}

#[test]
fn wordprocessing_child_accessors_are_relationship_backed_handles() {
  let package = WordprocessingDocument::new_from_file(doc_sample("Of16-01.docx")).unwrap();
  let main_part = package.main_document_part().unwrap();
  let styles_part = main_part.style_definitions_part(&package).unwrap();

  assert!(main_part.path(&package).is_some());
  assert!(main_part.get_id_of_part(&package, &styles_part).is_some());
  assert_eq!(
    main_part
      .style_definitions_part(&package)
      .unwrap()
      .part_id(),
    styles_part.part_id()
  );
  assert_eq!(
    main_part
      .get_parts_of_type::<_, StyleDefinitionsPart>(&package)
      .next()
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
      .filter(|pair| part_ref_variant!(pair.part.clone(), MainDocumentPart).is_some())
      .count(),
    0
  );
}

#[test]
fn wordprocessing_part_relationship_helpers_classify_children_and_references() {
  // Source: test/DocumentFormat.OpenXml.Tests/ofapiTest/OpenXmlPartTest.cs :: HyperlinkRelationshipTest
  let package = WordprocessingDocument::new_from_file(doc_sample("May_12_04.docx")).unwrap();
  let main_part = package.main_document_part().unwrap();

  assert_eq!(main_part.parts(&package).count(), 13);
  assert_eq!(main_part.external_relationships(&package).count(), 0);
  assert_eq!(main_part.hyperlink_relationships(&package).count(), 71);
  assert_eq!(
    main_part.external_relationships(&package).count()
      + main_part.hyperlink_relationships(&package).count()
      + main_part
        .data_part_reference_relationships(&package)
        .count(),
    71
  );
  let internal_hyperlink = main_part
    .get_reference_relationship(&package, "rId15")
    .unwrap();
  assert_eq!(internal_hyperlink.id(), "rId15");
  assert_eq!(internal_hyperlink.target(), "#_THIS_WEEK_IN");
  assert!(matches!(
    internal_hyperlink.target_mode(),
    TargetMode::Internal
  ));
  assert_eq!(
    internal_hyperlink.reference_kind(),
    Some(ReferenceRelationshipKind::Hyperlink)
  );

  let external_hyperlink = main_part
    .get_reference_relationship(&package, "rId18")
    .unwrap();
  assert_eq!(external_hyperlink.id(), "rId18");
  assert_eq!(external_hyperlink.target(), "http://www.iaswresearch.org/");
  assert!(matches!(
    external_hyperlink.target_mode(),
    TargetMode::External
  ));
  assert!(matches!(
    external_hyperlink.reference_kind(),
    Some(ReferenceRelationshipKind::Hyperlink)
  ));
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
  assert!(!image_part.data(&package).unwrap().is_empty());
  assert_eq!(
    main_part.get_id_of_part(&package, &comments_part),
    Some("rId7")
  );
  assert_eq!(
    main_part
      .get_part_by_id(&package, "rId7")
      .and_then(|part| part_ref_variant!(part, WordprocessingCommentsPart))
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
fn part_root_element_rejects_whitespace_only_xml() {
  // Source: test/DocumentFormat.OpenXml.Tests/ofapiTest/OpenXmlPartTest.cs :: RootElementTest2
  let mut package = WordprocessingDocument::new(empty_package()).unwrap();
  let main_part = package.add_main_document_part().unwrap();

  assert!(main_part.root_element(&mut package).is_err());
  main_part.set_data(&mut package, b" \n".to_vec()).unwrap();
  assert!(main_part.root_element(&mut package).is_err());
}

#[test]
fn part_relationship_ids_can_change_for_child_parts() {
  // Source: test/DocumentFormat.OpenXml.Tests/ofapiTest/OpenXmlPartTest.cs :: ChangePartIdTest
  let mut package =
    WordprocessingDocument::new_from_file_lazy(doc_sample("complex0.docx")).unwrap();
  let main_part = package.main_document_part().unwrap();
  let comments_part = main_part.wordprocessing_comments_part(&package).unwrap();
  let image_part = main_part.image_parts(&package).next().unwrap();
  let comments_original_id = main_part
    .get_id_of_part(&package, &comments_part)
    .unwrap()
    .to_string();
  let image_original_id = main_part
    .get_id_of_part(&package, &image_part)
    .unwrap()
    .to_string();
  let relationship_count = part_relationship_count(&package, &main_part);

  assert_eq!(
    main_part
      .change_id_of_part(&mut package, &comments_part, "rIdSdkComments")
      .unwrap(),
    comments_original_id
  );
  main_part
    .change_relationship_id(&mut package, &image_original_id, &comments_original_id)
    .unwrap();

  let refreshed_main = package.main_document_part().unwrap();
  assert_eq!(
    part_relationship_count(&package, &refreshed_main),
    relationship_count
  );
  assert!(
    refreshed_main
      .get_part_by_id(&package, &image_original_id)
      .is_none()
  );
  assert_eq!(
    refreshed_main.get_id_of_part(&package, &comments_part),
    Some("rIdSdkComments")
  );
  assert_eq!(
    refreshed_main
      .get_part_by_id(&package, &comments_original_id)
      .and_then(|part| part_ref_variant!(part, ImagePart))
      .map(|part| part.part_id()),
    Some(image_part.part_id())
  );

  let mut buffer = Cursor::new(Vec::new());
  package.save(&mut buffer).unwrap();

  let reopened = WordprocessingDocument::new(Cursor::new(buffer.into_inner())).unwrap();
  let reopened_main = reopened.main_document_part().unwrap();
  let reopened_comments = reopened_main
    .wordprocessing_comments_part(&reopened)
    .unwrap();
  assert_eq!(
    reopened_main.get_id_of_part(&reopened, &reopened_comments),
    Some("rIdSdkComments")
  );
  assert!(
    reopened_main
      .get_part_by_id(&reopened, &image_original_id)
      .is_none()
  );
  assert!(
    reopened_main
      .get_part_by_id(&reopened, &comments_original_id)
      .and_then(|part| part_ref_variant!(part, ImagePart))
      .is_some()
  );
}

#[test]
fn delete_invalid_child_part_id_is_safe() {
  // Source: test/DocumentFormat.OpenXml.Tests/ofapiTest/OpenXmlPartTest.cs :: DeleteInvalidPartIdSafely
  let mut package = WordprocessingDocument::new(empty_package()).unwrap();
  let main_part = package.add_main_document_part().unwrap();
  let relationship_count = part_relationship_count(&package, &main_part);

  assert!(
    main_part
      .try_get_part_by_id(&package, "invalidId")
      .is_none()
  );
  assert!(
    !main_part
      .delete_part_by_id(&mut package, "invalidId")
      .unwrap()
  );
  assert_eq!(
    part_relationship_count(&package, &main_part),
    relationship_count
  );
}

#[test]
fn package_get_all_parts_matches_openxml_package_tests() {
  // Source: test/DocumentFormat.OpenXml.Tests/ofapiTest/OpenXmlPackageTest.cs
  //   OpenXmlPackageGetAllPartsTestWord
  //   OpenXmlPackageGetAllPartsTestPowerPoint
  let word = WordprocessingDocument::new_from_file(doc_sample("complex0.docx")).unwrap();
  assert_eq!(word.get_all_parts().count(), 31);

  let presentation =
    PresentationDocument::new_from_file(doc_sample("o09_Performance_typical.pptx")).unwrap();
  let data_parts = presentation.media_data_parts().count();

  assert_eq!(presentation.get_all_parts().count(), 65);
  assert_eq!(data_parts, 1);
}

#[test]
fn part_get_all_parts_and_parent_parts_follow_reachable_relationship_graph() {
  // Source: aligned with OpenXmlPackageExtensions.GetAllParts and OpenXmlPart.GetParentParts.
  let mut package = WordprocessingDocument::new_from_file(doc_sample("Of16-01.docx")).unwrap();
  let main_part = package.main_document_part().unwrap();
  let header_part = main_part
    .add_new_part_auto_id::<_, HeaderPart>(&mut package)
    .unwrap();
  let settings_part = main_part
    .add_new_part_auto_id::<_, DocumentSettingsPart>(&mut package)
    .unwrap();
  let image_part = header_part
    .add_image_part(&mut package, "image/png")
    .unwrap();
  assert!(main_part.get_id_of_part(&package, &header_part).is_some());
  assert!(main_part.get_id_of_part(&package, &settings_part).is_some());
  assert!(header_part.get_id_of_part(&package, &image_part).is_some());
  assert!(main_part.get_id_of_part(&package, &image_part).is_none());
  assert!(
    main_part
      .get_parts_of_type::<_, HeaderPart>(&package)
      .next()
      .is_some()
  );

  let main_descendants: HashSet<_> = main_part
    .get_all_parts(&package)
    .map(|part| part.part_id())
    .collect();
  assert!(main_descendants.contains(&header_part.part_id()));
  assert!(main_descendants.contains(&settings_part.part_id()));
  assert!(main_descendants.contains(&image_part.part_id()));
  assert!(!main_descendants.contains(&main_part.part_id()));

  let image_parents: Vec<_> = image_part
    .get_parent_parts(&package)
    .map(|part| part.part_id())
    .collect();
  assert_eq!(image_parents, vec![header_part.part_id()]);

  let header_parents: Vec<_> = header_part
    .get_parent_parts(&package)
    .map(|part| part.part_id())
    .collect();
  assert_eq!(header_parents, vec![main_part.part_id()]);
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
    slides
      .iter()
      .all(|slide| presentation_part.get_id_of_part(&package, slide).is_some())
  );

  let media_part = package
    .media_data_parts()
    .find(|part| part.path(&package) == Some("ppt/media/media1.wav"))
    .unwrap();
  assert_eq!(media_part.content_type(&package), Some("audio/wav"));

  let mut internal_media_reference_count = 0;
  let mut external_null_audio_reference_count = 0;
  for slide in slides {
    for relationship in slide.data_part_reference_relationships(&package) {
      if relationship.target_part_id() == media_part.part_id() {
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
  let slide1 = slides[0].clone();
  let slide2 = slides[1].clone();

  let slide1_relationships: Vec<_> = slide1.data_part_reference_relationships(&package).collect();
  let slide1_media_relationships: Vec<_> = slide1_relationships
    .iter()
    .copied()
    .filter(|relationship| relationship.target_part_id() == media_data_part.part_id())
    .collect();
  assert_eq!(slide1_media_relationships.len(), 1);
  assert_eq!(
    slide1_media_relationships[0].relationship_type(),
    RelationshipRef::MEDIA_REFERENCE_RELATIONSHIP_TYPE
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
    RelationshipRef::AUDIO_REFERENCE_RELATIONSHIP_TYPE
  );
  assert_eq!(new_reference.target_part_id(), media_data_part.part_id());

  let removed = slide1
    .delete_reference_relationship(&mut package, &old_slide1_reference_id)
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
    .delete_reference_relationship(&mut package, &slide2_reference_id)
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
  let reopened_slide1 = reopened_slides[0].clone();
  let reopened_slide2 = reopened_slides[1].clone();
  let reopened_slide1_relationships: Vec<_> = reopened_slide1
    .data_part_reference_relationships(&reopened)
    .filter(|relationship| relationship.target_part_id() == reopened_media_data_part.part_id())
    .collect();
  assert_eq!(reopened_slide1_relationships.len(), 1);
  assert_eq!(
    reopened_slide1_relationships[0].relationship_type(),
    RelationshipRef::AUDIO_REFERENCE_RELATIONSHIP_TYPE
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
    .delete_reference_relationship(&mut reopened, &reopened_slide1_reference_id)
    .unwrap();
  let reopened_slide2_reference_id = reopened_slide2
    .data_part_reference_relationships(&reopened)
    .find(|relationship| relationship.target_part_id() == reopened_media_data_part.part_id())
    .unwrap()
    .id()
    .to_string();
  reopened_slide2
    .delete_reference_relationship(&mut reopened, &reopened_slide2_reference_id)
    .unwrap();

  let remaining_media_data_parts: Vec<_> = reopened.media_data_parts().collect();
  assert_eq!(remaining_media_data_parts.len(), 1);
  assert!(remaining_media_data_parts[0].is_orphan(&reopened));
  assert_eq!(
    remaining_media_data_parts[0]
      .data_part_reference_relationships(&reopened)
      .count(),
    0
  );
  assert_eq!(reopened.delete_unused_media_data_parts(), 1);
  assert_eq!(reopened.media_data_parts().count(), 0);
  assert_eq!(reopened.delete_unused_media_data_parts(), 0);
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
    RelationshipRef::AUDIO_REFERENCE_RELATIONSHIP_TYPE
  );
  assert_eq!(
    audio_relationship.reference_kind(),
    Some(ReferenceRelationshipKind::Audio)
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
    RelationshipRef::MEDIA_REFERENCE_RELATIONSHIP_TYPE
  );
  assert_eq!(
    media_relationship.reference_kind(),
    Some(ReferenceRelationshipKind::Media)
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
    RelationshipRef::VIDEO_REFERENCE_RELATIONSHIP_TYPE
  );
  assert_eq!(
    video_relationship.reference_kind(),
    Some(ReferenceRelationshipKind::Video)
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
  let reopened_audio_part = media_data_part_by_id(
    &reopened,
    reopened_audio_relationship.target_part_id().unwrap(),
  );
  assert_eq!(
    reopened_audio_part.content_type(&reopened),
    Some("audio/wav")
  );
  assert_eq!(
    reopened_audio_part.data(&reopened),
    Some(b"wav bytes".as_slice())
  );

  let reopened_media_relationship = reopened_relationships
    .iter()
    .find(|relationship| relationship.id() == "rIdSdkMedia")
    .unwrap();
  let reopened_media_part = media_data_part_by_id(
    &reopened,
    reopened_media_relationship.target_part_id().unwrap(),
  );
  assert_eq!(
    reopened_media_part.content_type(&reopened),
    Some("video/avi")
  );
  assert_eq!(
    reopened_media_part.data(&reopened),
    Some(b"avi bytes".as_slice())
  );

  let reopened_video_relationship = reopened_relationships
    .iter()
    .find(|relationship| relationship.id() == "rIdSdkVideo")
    .unwrap();
  assert_eq!(
    reopened_video_relationship.relationship_type(),
    RelationshipRef::VIDEO_REFERENCE_RELATIONSHIP_TYPE
  );
  assert_eq!(
    reopened_video_relationship.target_part_id(),
    reopened_media_relationship.target_part_id()
  );
}

#[test]
fn media_data_part_rejects_foreign_package_relationships() {
  // Source: OpenXmlPartContainer.AddDataPartReferenceRelationship foreign MediaDataPart guard.
  let mut foreign_package = WordprocessingDocument::new(empty_package()).unwrap();
  let foreign_media_data_part = foreign_package
    .create_media_data_part_by_type(MediaDataPartType::Mp3)
    .unwrap();
  foreign_media_data_part
    .set_data(&mut foreign_package, b"foreign mp3 bytes".to_vec())
    .unwrap();

  let mut package = WordprocessingDocument::new(empty_package()).unwrap();
  let main_part = package.add_main_document_part().unwrap();
  main_part
    .set_root_element(&mut package, empty_body_document())
    .unwrap();

  assert!(
    foreign_media_data_part.path(&package).is_none(),
    "foreign media data part should not resolve against another package"
  );
  assert!(
    foreign_media_data_part
      .set_data(&mut package, b"wrong package".to_vec())
      .is_err()
  );
  assert!(
    main_part
      .add_audio_reference_relationship(&mut package, &foreign_media_data_part)
      .is_err()
  );
  assert_eq!(
    main_part
      .data_part_reference_relationships(&package)
      .count(),
    0
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
  let existing_relationship: ooxmlsdk::common::Relationship = main_part
    .get_reference_relationship(&package, "rIdSdkDataRef")
    .unwrap()
    .into();

  let copied_relationship_id = header
    .add_data_part_reference_relationship_from_existing(&mut package, existing_relationship)
    .unwrap();
  assert_eq!(copied_relationship_id, "rIdSdkDataRef");
  let copied_relationship = header
    .get_reference_relationship(&package, "rIdSdkDataRef")
    .unwrap();
  assert_eq!(
    copied_relationship.relationship_type(),
    RelationshipRef::MEDIA_REFERENCE_RELATIONSHIP_TYPE
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
    .find(|part| reopened_main.get_id_of_part(&reopened, part) == Some("rIdSdkHeaderForDataRef"))
    .unwrap();
  let reopened_relationship = reopened_header
    .get_reference_relationship(&reopened, "rIdSdkDataRef")
    .unwrap();
  assert_eq!(
    reopened_relationship.relationship_type(),
    RelationshipRef::MEDIA_REFERENCE_RELATIONSHIP_TYPE
  );
  let reopened_media_part =
    media_data_part_by_id(&reopened, reopened_relationship.target_part_id().unwrap());
  assert_eq!(
    reopened_media_part.content_type(&reopened),
    Some("audio/mp3")
  );
  assert_eq!(
    reopened_media_part.data(&reopened),
    Some(b"mp3 bytes".as_slice())
  );
}

#[test]
fn wordprocessing_hyperlink_relationships_are_preserved_from_openxml_part_test() {
  // Source: test/DocumentFormat.OpenXml.Tests/ofapiTest/OpenXmlPartTest.cs :: HyperlinkRelationshipTest
  let package = WordprocessingDocument::new_from_file(doc_sample("May_12_04.docx")).unwrap();
  let main_part = package.main_document_part().unwrap();

  let hyperlink_relationships: Vec<_> = main_part.hyperlink_relationships(&package).collect();

  assert_eq!(hyperlink_relationships.len(), 71);
  assert!(
    main_part
      .external_relationships(&package)
      .all(|relationship| relationship.relationship_type()
        != ooxmlsdk::common::RelationshipRef::HYPERLINK_RELATIONSHIP_TYPE)
  );
  assert!(
    main_part
      .get_hyperlink_relationship(&package, "rId15")
      .is_some()
  );

  let rid15 = main_part
    .get_hyperlink_relationship(&package, "rId15")
    .unwrap();
  assert_eq!(rid15.target(), "#_THIS_WEEK_IN");
  assert!(matches!(rid15.target_mode(), TargetMode::Internal));
  assert_eq!(rid15.target_kind(), RelationshipTargetKind::Missing);
  assert!(rid15.target_part_id().is_none());

  let rid18 = main_part
    .get_hyperlink_relationship(&package, "rId18")
    .unwrap();
  assert_eq!(rid18.target(), "http://www.iaswresearch.org/");
  assert!(matches!(rid18.target_mode(), TargetMode::External));
  assert_eq!(rid18.target_kind(), RelationshipTargetKind::External);
  assert!(rid18.target_part_id().is_none());
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

  main_part
    .set_root_element(&mut package, empty_body_document())
    .unwrap();

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

  assert!(main_part.root_element(&mut package).is_ok());
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
fn part_hyperlink_relationship_mutation_is_saved() {
  // Source: adapted from OpenXmlPartContainer hyperlink relationship mutation coverage.
  let mut package = WordprocessingDocument::new_from_file_lazy(doc_sample("Of16-01.docx")).unwrap();
  let main_part = package.main_document_part().unwrap();
  let relationship_id = "rIdSdkHyperlink";
  let target = "https://example.com/ooxmlsdk";

  assert!(
    main_part
      .get_hyperlink_relationship(&package, relationship_id)
      .is_none()
  );

  let relationship = main_part
    .add_hyperlink_relationship(&mut package, relationship_id, target)
    .unwrap();
  assert_eq!(relationship.id(), relationship_id);
  assert_eq!(
    relationship.relationship_type(),
    RelationshipRef::HYPERLINK_RELATIONSHIP_TYPE
  );
  assert_eq!(relationship.target(), target);
  assert!(matches!(relationship.target_mode(), TargetMode::External));
  assert_eq!(relationship.target_kind(), RelationshipTargetKind::External);
  assert_eq!(
    relationship.reference_kind(),
    Some(ReferenceRelationshipKind::Hyperlink)
  );

  let mut buffer = Cursor::new(Vec::new());
  package.save(&mut buffer).unwrap();

  let reopened = WordprocessingDocument::new(Cursor::new(buffer.into_inner())).unwrap();
  let reopened_main = reopened.main_document_part().unwrap();
  let reopened_relationship = reopened_main
    .get_hyperlink_relationship(&reopened, relationship_id)
    .unwrap();

  assert_eq!(reopened_relationship.target(), target);
  assert_eq!(
    reopened_relationship.relationship_type(),
    RelationshipRef::HYPERLINK_RELATIONSHIP_TYPE
  );
  assert!(matches!(
    reopened_relationship.target_mode(),
    TargetMode::External
  ));
}

#[test]
fn part_hyperlink_relationship_supports_internal_targets() {
  // Source: adapted from OpenXmlPartContainer.AddHyperlinkRelationship(uri, isExternal).
  let mut package = WordprocessingDocument::new_from_file_lazy(doc_sample("Of16-01.docx")).unwrap();
  let main_part = package.main_document_part().unwrap();
  let relationship_id = "rIdSdkInternalHyperlink";

  let relationship = main_part
    .add_hyperlink_relationship_with_mode(
      &mut package,
      relationship_id,
      "#SdkBookmark",
      TargetMode::Internal,
    )
    .unwrap();
  assert_eq!(relationship.id(), relationship_id);
  assert_eq!(relationship.target(), "#SdkBookmark");
  assert!(matches!(relationship.target_mode(), TargetMode::Internal));
  assert_eq!(
    relationship.reference_kind(),
    Some(ReferenceRelationshipKind::Hyperlink)
  );

  let auto_relationship = main_part
    .add_hyperlink_relationship_auto_id(
      &mut package,
      "https://example.com/auto-hyperlink",
      TargetMode::External,
    )
    .unwrap();
  let auto_relationship_id = auto_relationship.id().to_string();
  assert_eq!(
    main_part
      .get_hyperlink_relationship(&package, &auto_relationship_id)
      .unwrap()
      .target(),
    "https://example.com/auto-hyperlink"
  );

  let mut buffer = Cursor::new(Vec::new());
  package.save(&mut buffer).unwrap();

  let reopened = WordprocessingDocument::new(Cursor::new(buffer.into_inner())).unwrap();
  let reopened_main = reopened.main_document_part().unwrap();
  let reopened_relationship = reopened_main
    .get_hyperlink_relationship(&reopened, relationship_id)
    .unwrap();
  assert_eq!(reopened_relationship.target(), "#SdkBookmark");
  assert!(matches!(
    reopened_relationship.target_mode(),
    TargetMode::Internal
  ));
  assert!(
    reopened_main
      .get_hyperlink_relationship(&reopened, &auto_relationship_id)
      .is_some()
  );
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
      .get_reference_relationship(&package, relationship_id)
      .is_none()
  );
  assert!(
    main_part
      .get_reference_relationship(&package, changed_relationship_id)
      .is_some()
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
    reopened_main
      .get_reference_relationship(&reopened, changed_relationship_id)
      .is_none()
  );
}

#[test]
fn part_external_relationship_helpers_are_kind_specific() {
  // Source: adapted from OpenXmlPartContainer external relationship get/delete coverage.
  let mut package = WordprocessingDocument::new_from_file_lazy(doc_sample("Of16-01.docx")).unwrap();
  let main_part = package.main_document_part().unwrap();
  let relationship = main_part
    .add_external_relationship_auto_id(
      &mut package,
      "http://example.com/relationships/custom",
      "https://example.com/part-external",
    )
    .unwrap();
  let relationship_id = relationship.id().to_string();

  assert_eq!(
    main_part
      .get_external_relationship(&package, &relationship_id)
      .unwrap()
      .target(),
    "https://example.com/part-external"
  );
  assert!(
    main_part
      .get_hyperlink_relationship(&package, &relationship_id)
      .is_none()
  );
  let removed = main_part
    .delete_external_relationship(&mut package, &relationship_id)
    .unwrap();
  assert_eq!(removed.id(), relationship_id);
  assert!(
    main_part
      .delete_external_relationship(&mut package, &relationship_id)
      .is_err()
  );
}

#[test]
fn package_external_relationship_mutation_is_saved() {
  // Source: adapted from OpenXmlPackage relationship mutation coverage.
  let mut package = WordprocessingDocument::new_from_file_lazy(doc_sample("Of16-01.docx")).unwrap();
  let relationship_id = "rIdPackageExternal".to_string();
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
  let relationship = reopened
    .get_reference_relationship(&relationship_id)
    .unwrap();

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
  let relationship_id = "rIdSdkHeader";

  let header_part = main_part
    .add_new_part::<_, HeaderPart>(&mut package, relationship_id)
    .unwrap();
  assert_eq!(
    main_part.get_id_of_part(&package, &header_part),
    Some(relationship_id)
  );
  assert_eq!(
    header_part.content_type(&package),
    Some("application/vnd.openxmlformats-officedocument.wordprocessingml.header+xml")
  );
  assert_eq!(header_part.path(&package), Some("word/header1.xml"));
  header_part
    .set_root_element(&mut package, Header::default())
    .unwrap();

  let mut buffer = Cursor::new(Vec::new());
  package.save(&mut buffer).unwrap();

  let mut reopened = WordprocessingDocument::new(Cursor::new(buffer.into_inner())).unwrap();
  let reopened_main = reopened.main_document_part().unwrap();
  let reopened_header = reopened_main
    .header_parts(&reopened)
    .find(|part| reopened_main.get_id_of_part(&reopened, part) == Some(relationship_id))
    .unwrap();

  assert_eq!(reopened_header.path(&reopened), Some("word/header1.xml"));
  assert!(reopened_header.root_element(&mut reopened).is_ok());
}

#[test]
fn add_new_parts_use_unique_upstream_style_part_names() {
  // Source: test/DocumentFormat.OpenXml.Packaging.Tests/PartUriHelperTests
  let mut package = WordprocessingDocument::new(empty_package()).unwrap();
  let main_part = package.add_main_document_part().unwrap();
  main_part
    .set_root_element(&mut package, empty_body_document())
    .unwrap();

  let header1 = main_part
    .add_new_part_auto_id::<_, HeaderPart>(&mut package)
    .unwrap();
  let header2 = main_part
    .add_new_part_auto_id::<_, HeaderPart>(&mut package)
    .unwrap();
  assert_eq!(header1.path(&package), Some("word/header1.xml"));
  assert_eq!(header2.path(&package), Some("word/header2.xml"));

  let image1 = main_part.add_image_part(&mut package, "image/png").unwrap();
  let image2 = main_part.add_image_part(&mut package, "image/png").unwrap();
  assert_eq!(image1.path(&package), Some("word/media/image1.bin"));
  assert_eq!(image2.path(&package), Some("word/media/image2.bin"));
}

#[test]
fn add_new_part_auto_id_skips_existing_relationship_ids() {
  // Source: upstream AddNewPart<T>() auto relationship-id behavior adapted for Rust handles.
  let mut package = WordprocessingDocument::new_from_file_lazy(doc_sample("Of16-01.docx")).unwrap();
  let main_part = package.main_document_part().unwrap();
  let relationship_count = part_relationship_count(&package, &main_part);

  let header_part = main_part
    .add_new_part_auto_id::<_, HeaderPart>(&mut package)
    .unwrap();
  let relationship_id = main_part
    .get_id_of_part(&package, &header_part)
    .unwrap()
    .to_string();
  assert!(relationship_id.starts_with("rId"));
  assert_eq!(
    part_relationship_count(&package, &main_part),
    relationship_count + 1
  );

  assert!(
    main_part
      .add_new_part::<_, HeaderPart>(&mut package, relationship_id.as_str())
      .is_err()
  );
  assert_eq!(
    part_relationship_count(&package, &main_part),
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
  assert_eq!(package.get_id_of_part(&ribbon_part), Some(relationship_id));
  assert_eq!(ribbon_part.path(&package), Some("customUI/customUI1.xml"));
  assert_eq!(ribbon_part.content_type(&package), Some("application/xml"));

  let mut buffer = Cursor::new(Vec::new());
  package.save(&mut buffer).unwrap();

  let reopened = WordprocessingDocument::new(Cursor::new(buffer.into_inner())).unwrap();
  let reopened_part = reopened
    .get_part_by_id(relationship_id)
    .and_then(|part| part_ref_variant!(part, RibbonExtensibilityPart))
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
    .and_then(|part| part_ref_variant!(part, ImagePart))
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
  let relationship_id = main_part.get_id_of_part(&package, &image_part).unwrap();
  let reopened_image = reopened_main
    .get_part_by_id(&reopened, relationship_id)
    .and_then(|part| part_ref_variant!(part, ImagePart))
    .unwrap();

  assert_eq!(
    reopened_image.data(&reopened),
    Some(&b"replacement image bytes"[..])
  );
}

#[test]
fn part_data_helpers_read_text_and_write_bytes() {
  // Source: upstream GetStream(FileMode.Open) read semantics adapted to Rust helpers.
  let mut package =
    WordprocessingDocument::new_from_file_lazy(doc_sample("Hyperlink.docx")).unwrap();
  let main_part = package.main_document_part().unwrap();
  let xml = "<properties><property name=\"sdk\">text</property></properties>";
  let custom_xml = main_part
    .add_custom_xml_part_by_type(&mut package, CustomXmlPartType::CustomXml)
    .unwrap();
  custom_xml
    .set_data(&mut package, xml.as_bytes().to_vec())
    .unwrap();

  assert_eq!(custom_xml.data_as_str(&package).unwrap(), Some(xml));
  assert_eq!(
    custom_xml.data_to_vec(&package).unwrap(),
    xml.as_bytes().to_vec()
  );

  let mut copied = Vec::new();
  assert!(custom_xml.write_data_to(&package, &mut copied).unwrap());
  assert_eq!(copied, xml.as_bytes());

  let image_part = main_part.add_image_part(&mut package, "image/png").unwrap();
  let image_bytes = b"\x89PNG\r\n\x1a\nsdk-data-helper".to_vec();
  image_part
    .feed_data(&mut package, &mut Cursor::new(image_bytes.clone()))
    .unwrap();
  assert_eq!(image_part.data_to_vec(&package), Some(image_bytes));
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
    main_part.get_id_of_part(&package, &image_part),
    Some(relationship_id)
  );
  assert_eq!(image_part.content_type(&package), Some("image/png"));

  let mut buffer = Cursor::new(Vec::new());
  package.save(&mut buffer).unwrap();

  let reopened = WordprocessingDocument::new(Cursor::new(buffer.into_inner())).unwrap();
  let reopened_main = reopened.main_document_part().unwrap();
  let reopened_image = reopened_main
    .get_part_by_id(&reopened, relationship_id)
    .and_then(|part| part_ref_variant!(part, ImagePart))
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
  let relationship_count = part_relationship_count(&package, &main_part);

  let image_part = main_part
    .add_image_part(&mut package, "image/jpeg")
    .unwrap();
  image_part
    .set_data(&mut package, b"jpeg bytes".to_vec())
    .unwrap();
  let relationship_id = main_part
    .get_id_of_part(&package, &image_part)
    .unwrap()
    .to_string();

  assert!(relationship_id.starts_with("rId"));
  assert_eq!(
    part_relationship_count(&package, &main_part),
    relationship_count + 1
  );
  assert_eq!(image_part.content_type(&package), Some("image/jpeg"));

  let mut buffer = Cursor::new(Vec::new());
  package.save(&mut buffer).unwrap();

  let reopened = WordprocessingDocument::new(Cursor::new(buffer.into_inner())).unwrap();
  let reopened_main = reopened.main_document_part().unwrap();
  let reopened_image = reopened_main
    .get_part_by_id(&reopened, relationship_id.as_str())
    .and_then(|part| part_ref_variant!(part, ImagePart))
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
    main_part.get_id_of_part(&package, &alt_chunk),
    Some(relationship_id)
  );
  assert_eq!(alt_chunk.content_type(&package), Some("text/html"));

  let mut buffer = Cursor::new(Vec::new());
  package.save(&mut buffer).unwrap();

  let reopened = WordprocessingDocument::new(Cursor::new(buffer.into_inner())).unwrap();
  let reopened_main = reopened.main_document_part().unwrap();
  let reopened_chunk = reopened_main
    .get_part_by_id(&reopened, relationship_id)
    .and_then(|part| part_ref_variant!(part, AlternativeFormatImportPart))
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
  let relationship_count = part_relationship_count(&package, &main_part);

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
    .get_id_of_part(&package, &alt_chunk)
    .unwrap()
    .to_string();

  assert!(relationship_id.starts_with("rId"));
  assert_eq!(
    part_relationship_count(&package, &main_part),
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
    .and_then(|part| part_ref_variant!(part, AlternativeFormatImportPart))
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
  let relationship_count = part_relationship_count(&package, &main_part);
  let xml = b"<properties><property name=\"sdk\">custom xml</property></properties>".to_vec();

  let custom_xml = main_part
    .add_custom_xml_part_by_type(&mut package, CustomXmlPartType::CustomXml)
    .unwrap();
  custom_xml
    .feed_data(&mut package, &mut Cursor::new(xml.clone()))
    .unwrap();
  let relationship_id = main_part
    .get_id_of_part(&package, &custom_xml)
    .unwrap()
    .to_string();

  assert!(relationship_id.starts_with("rId"));
  assert_eq!(
    part_relationship_count(&package, &main_part),
    relationship_count + 1
  );
  assert_eq!(custom_xml.content_type(&package), Some("application/xml"));

  let mut buffer = Cursor::new(Vec::new());
  package.save(&mut buffer).unwrap();

  let reopened = WordprocessingDocument::new(Cursor::new(buffer.into_inner())).unwrap();
  let reopened_main = reopened.main_document_part().unwrap();
  let reopened_custom_xml = reopened_main
    .get_part_by_id(&reopened, relationship_id.as_str())
    .and_then(|part| part_ref_variant!(part, CustomXmlPart))
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
    main_part.get_id_of_part(&package, &custom_xml),
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
    .and_then(|part| part_ref_variant!(part, CustomXmlPart))
    .unwrap();

  assert_eq!(
    reopened_custom_xml.content_type(&reopened),
    Some("application/inkml+xml")
  );
  assert_eq!(reopened_custom_xml.data(&reopened), Some(inkml.as_slice()));
}

#[test]
fn generic_add_new_part_with_content_type_and_extension_saves_custom_extension() {
  // Source: upstream AddNewPart<T>(contentType, id) plus PartTypeInfo extension semantics.
  let mut package =
    WordprocessingDocument::new_from_file_lazy(doc_sample("Hyperlink.docx")).unwrap();
  let main_part = package.main_document_part().unwrap();
  let relationship_id = "rIdSdkGenericCustomXml";
  let custom_xml = b"<ink xmlns=\"http://www.w3.org/2003/InkML\"/>".to_vec();

  let part = main_part
    .add_new_part_with_content_type_and_extension::<_, CustomXmlPart>(
      &mut package,
      relationship_id,
      "application/inkml+xml",
      ".inkml",
    )
    .unwrap();
  part.set_data(&mut package, custom_xml.clone()).unwrap();
  let path = part.path(&package).unwrap().to_string();

  assert_eq!(
    main_part.get_id_of_part(&package, &part),
    Some(relationship_id)
  );
  assert_eq!(part.content_type(&package), Some("application/inkml+xml"));
  assert!(path.starts_with("customXml/item"));
  assert!(path.ends_with(".inkml"));

  let mut buffer = Cursor::new(Vec::new());
  package.save(&mut buffer).unwrap();

  let reopened = WordprocessingDocument::new(Cursor::new(buffer.into_inner())).unwrap();
  let reopened_main = reopened.main_document_part().unwrap();
  let reopened_part = reopened_main
    .get_part_by_id(&reopened, relationship_id)
    .and_then(|part| part_ref_variant!(part, CustomXmlPart))
    .unwrap();

  assert_eq!(
    reopened_part.content_type(&reopened),
    Some("application/inkml+xml")
  );
  assert_eq!(reopened_part.data(&reopened), Some(custom_xml.as_slice()));
  assert_eq!(reopened_part.path(&reopened), Some(path.as_str()));
}

#[test]
fn part_extension_selection_matches_openxml_part_tests() {
  // Source: test/DocumentFormat.OpenXml.Framework.Tests/OpenXmlPartTests.cs
  //   ExtensionTest
  //   ExtensionTestUndefinedExtension
  //   ExtensionTestFixedContentType
  let mut package = WordprocessingDocument::new(empty_package()).unwrap();

  let jpg = package
    .add_extended_part_with_id(
      "http://example.com/relationships/jpg",
      "test/mimetype",
      ".jpg",
      "rIdJpg",
    )
    .unwrap();
  assert_eq!(jpg.path(&package), Some("extendedPart1.jpg"));
  assert_eq!(jpg.content_type(&package), Some("test/mimetype"));

  let no_dot = package
    .add_extended_part_with_id(
      "http://example.com/relationships/no-dot",
      "test/mimetype",
      "jpeg",
      "rIdNoDot",
    )
    .unwrap();
  assert_eq!(no_dot.path(&package), Some("extendedPart1.jpeg"));

  let default_extension = package
    .add_extended_part_with_id(
      "http://example.com/relationships/default-extension",
      "test/mimetype",
      "",
      "rIdDefaultExtension",
    )
    .unwrap();
  assert_eq!(default_extension.path(&package), Some("extendedPart1.xml"));

  let fixed_type = package
    .add_thumbnail_part_with_id("image/jpeg", "rIdThumbnail")
    .unwrap();
  assert_eq!(fixed_type.path(&package), Some("docProps/thumbnail1.bin"));
  assert_eq!(fixed_type.content_type(&package), Some("image/jpeg"));
}

#[test]
fn package_add_new_part_with_content_type_and_extension_auto_id_saves_custom_extension() {
  // Source: package-level AddNewPart<T>() creation semantics with Rust extension override.
  let mut package =
    WordprocessingDocument::new_from_file_lazy(doc_sample("Hyperlink.docx")).unwrap();
  let relationship_count = package.parts().count();
  let png = b"package thumbnail png".to_vec();

  let part = package
    .add_new_part_with_content_type_and_extension_auto_id::<ThumbnailPart>("image/png", ".png")
    .unwrap();
  part.set_data(&mut package, png.clone()).unwrap();
  let relationship_id = package.get_id_of_part(&part).unwrap().to_string();
  let path = part.path(&package).unwrap().to_string();

  assert!(relationship_id.starts_with("rId"));
  assert_eq!(package.parts().count(), relationship_count + 1);
  assert_eq!(part.content_type(&package), Some("image/png"));
  assert!(path.starts_with("docProps/thumbnail"));
  assert!(path.ends_with(".png"));

  let mut buffer = Cursor::new(Vec::new());
  package.save(&mut buffer).unwrap();

  let reopened = WordprocessingDocument::new(Cursor::new(buffer.into_inner())).unwrap();
  let reopened_part = reopened
    .get_part_by_id(relationship_id.as_str())
    .and_then(|part| part_ref_variant!(part, ThumbnailPart))
    .unwrap();

  assert_eq!(reopened_part.content_type(&reopened), Some("image/png"));
  assert_eq!(reopened_part.data(&reopened), Some(png.as_slice()));
  assert_eq!(reopened_part.path(&reopened), Some(path.as_str()));
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
    .get_id_of_part(&package, &embedded_object)
    .unwrap()
    .to_string();

  let embedded_package = main_part
    .add_embedded_package_part_by_type(&mut package, EmbeddedPackagePartType::Xlsx)
    .unwrap();
  embedded_package
    .set_data(&mut package, b"xlsx package bytes".to_vec())
    .unwrap();
  let embedded_package_id = main_part
    .get_id_of_part(&package, &embedded_package)
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
    .get_id_of_part(&package, &font_table)
    .unwrap()
    .to_string();
  let font_id = font_table
    .get_id_of_part(&package, &font)
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
    .get_id_of_part(&package, &settings)
    .unwrap()
    .to_string();
  let recipients_id = settings
    .get_id_of_part(&package, &recipients)
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
    embedded_package
      .path(&package)
      .is_some_and(|path| path.ends_with(".xlsx"))
  );
  assert!(
    font
      .path(&package)
      .is_some_and(|path| path.ends_with(".ttf"))
  );

  let mut buffer = Cursor::new(Vec::new());
  package.save(&mut buffer).unwrap();

  let reopened = WordprocessingDocument::new_lazy(Cursor::new(buffer.into_inner())).unwrap();
  let reopened_main = reopened.main_document_part().unwrap();

  let reopened_embedded_object = reopened_main
    .get_part_by_id(&reopened, embedded_object_id.as_str())
    .and_then(|part| part_ref_variant!(part, EmbeddedObjectPart))
    .unwrap();
  let reopened_embedded_package = reopened_main
    .get_part_by_id(&reopened, embedded_package_id.as_str())
    .and_then(|part| part_ref_variant!(part, EmbeddedPackagePart))
    .unwrap();
  let reopened_font_table = reopened_main
    .get_part_by_id(&reopened, font_table_id.as_str())
    .and_then(|part| part_ref_variant!(part, FontTablePart))
    .unwrap();
  let reopened_font = reopened_font_table
    .get_part_by_id(&reopened, font_id.as_str())
    .and_then(|part| part_ref_variant!(part, FontPart))
    .unwrap();
  let reopened_settings = reopened_main
    .get_part_by_id(&reopened, settings_id.as_str())
    .and_then(|part| part_ref_variant!(part, DocumentSettingsPart))
    .unwrap();
  let reopened_recipients = reopened_settings
    .get_part_by_id(&reopened, recipients_id.as_str())
    .and_then(|part| part_ref_variant!(part, MailMergeRecipientDataPart))
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
    .get_id_of_part(&package, &custom_property)
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
    .get_id_of_part(&package, &control)
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
    .get_id_of_part(&package, &direct_binary)
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
    .get_id_of_part(&package, &child_binary)
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
    custom_property
      .path(&package)
      .is_some_and(|path| path.ends_with(".xml"))
  );
  assert!(
    control
      .path(&package)
      .is_some_and(|path| path.ends_with(".xml"))
  );
  assert!(
    direct_binary
      .path(&package)
      .is_some_and(|path| path.ends_with(".bin"))
  );

  let mut buffer = Cursor::new(Vec::new());
  package.save(&mut buffer).unwrap();

  let reopened = SpreadsheetDocument::new_lazy(Cursor::new(buffer.into_inner())).unwrap();
  let reopened_workbook = reopened.workbook_part().unwrap();
  let reopened_worksheet = reopened_workbook.worksheet_parts(&reopened).next().unwrap();
  let reopened_custom_property = reopened_worksheet
    .get_part_by_id(&reopened, custom_property_id.as_str())
    .and_then(|part| part_ref_variant!(part, CustomPropertyPart))
    .unwrap();
  let reopened_control = reopened_worksheet
    .get_part_by_id(&reopened, control_id.as_str())
    .and_then(|part| part_ref_variant!(part, EmbeddedControlPersistencePart))
    .unwrap();
  let reopened_direct_binary = reopened_worksheet
    .get_part_by_id(&reopened, direct_binary_id.as_str())
    .and_then(|part| part_ref_variant!(part, EmbeddedControlPersistenceBinaryDataPart))
    .unwrap();
  let reopened_child_binary = reopened_control
    .get_part_by_id(&reopened, child_binary_id.as_str())
    .and_then(|part| part_ref_variant!(part, EmbeddedControlPersistenceBinaryDataPart))
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
  let relationship_count = package.parts().count();
  let jpeg = b"thumbnail jpeg bytes".to_vec();

  let thumbnail = package
    .add_thumbnail_part_by_type(ThumbnailPartType::Jpeg)
    .unwrap();
  thumbnail
    .feed_data(&mut package, &mut Cursor::new(jpeg.clone()))
    .unwrap();
  let relationship_id = package.get_id_of_part(&thumbnail).unwrap().to_string();
  let thumbnail_path = thumbnail.path(&package).unwrap().to_string();

  assert!(relationship_id.starts_with("rId"));
  assert_eq!(package.parts().count(), relationship_count + 1);
  assert_eq!(thumbnail.content_type(&package), Some("image/jpeg"));
  assert!(thumbnail_path.starts_with("docProps/thumbnail"));
  assert!(thumbnail_path.ends_with(".jpg"));

  let mut buffer = Cursor::new(Vec::new());
  package.save(&mut buffer).unwrap();

  let reopened = WordprocessingDocument::new(Cursor::new(buffer.into_inner())).unwrap();
  let reopened_thumbnail = reopened
    .get_part_by_id(relationship_id.as_str())
    .and_then(|part| part_ref_variant!(part, ThumbnailPart))
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

  assert_eq!(package.get_id_of_part(&thumbnail), Some(relationship_id));
  assert_eq!(thumbnail.content_type(&package), Some("image/jpg"));

  let mut buffer = Cursor::new(Vec::new());
  package.save(&mut buffer).unwrap();

  let reopened = WordprocessingDocument::new(Cursor::new(buffer.into_inner())).unwrap();
  let reopened_thumbnail = reopened
    .get_part_by_id(relationship_id)
    .and_then(|part| part_ref_variant!(part, ThumbnailPart))
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
    package.get_id_of_part(&package_extended),
    Some("rIdSdkPackageExtended")
  );
  assert_eq!(
    main_part.get_id_of_part(&package, &part_extended),
    Some("rIdSdkMainExtended")
  );
  assert_eq!(
    part_extended.get_id_of_part(&package, &nested_extended),
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

  assert!(
    package
      .get_part_by_id("rIdSdkPackageExtended")
      .and_then(|part| part_ref_variant!(part, ExtendedPart))
      .is_some()
  );
  assert!(
    main_part
      .get_part_by_id(&package, "rIdSdkMainExtended")
      .and_then(|part| part_ref_variant!(part, ExtendedPart))
      .is_some()
  );
  assert!(
    part_extended
      .get_part_by_id(&package, "rIdSdkNestedExtended")
      .and_then(|part| part_ref_variant!(part, ExtendedPart))
      .is_some()
  );

  let mut buffer = Cursor::new(Vec::new());
  package.save(&mut buffer).unwrap();

  let reopened = WordprocessingDocument::new_lazy(Cursor::new(buffer.into_inner())).unwrap();
  let reopened_package_extended = reopened
    .get_part_by_id("rIdSdkPackageExtended")
    .and_then(|part| part_ref_variant!(part, ExtendedPart))
    .unwrap();
  let reopened_main = reopened.main_document_part().unwrap();
  let reopened_part_extended = reopened_main
    .get_part_by_id(&reopened, "rIdSdkMainExtended")
    .and_then(|part| part_ref_variant!(part, ExtendedPart))
    .unwrap();
  let reopened_nested_extended = reopened_part_extended
    .get_part_by_id(&reopened, "rIdSdkNestedExtended")
    .and_then(|part| part_ref_variant!(part, ExtendedPart))
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

  assert!(package.delete_part(thumbnail.clone()).unwrap());
  assert!(!package.delete_part_by_id("rIdSdkThumbnail").unwrap());
  assert!(package.get_part_by_id("rIdSdkThumbnail").is_none());
  assert!(thumbnail.path(&package).is_none());

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

  assert!(extended.delete_part(&mut package, nested.clone()).unwrap());
  assert!(!extended.delete_part_by_id(&mut package, "tempId2").unwrap());
  assert!(extended.get_part_by_id(&package, "tempId2").is_none());
  assert!(nested.path(&package).is_none());

  assert!(
    main_part
      .delete_part(&mut package, extended.clone())
      .unwrap()
  );
  assert!(main_part.get_part_by_id(&package, "tempId").is_none());
  assert!(extended.path(&package).is_none());

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
fn delete_parts_removes_selected_direct_children() {
  // Source: aligned with upstream OpenXmlPartContainer.DeleteParts<T>().
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

  let image_parts: Vec<_> = main_part
    .get_parts_of_type::<_, ImagePart>(&package)
    .collect();
  main_part
    .delete_parts::<_, ImagePart, _>(&mut package, image_parts)
    .unwrap();

  assert!(
    main_part
      .get_part_by_id(&package, "rIdDirectImage")
      .is_none()
  );
  assert!(direct_image.path(&package).is_none());
  assert!(
    extended
      .get_part_by_id(&package, "rIdNestedImage")
      .and_then(|part| part_ref_variant!(part, ImagePart))
      .is_some()
  );

  let mut buffer = Cursor::new(Vec::new());
  package.save(&mut buffer).unwrap();
  let bytes = buffer.into_inner();

  assert!(!package_entry_exists(bytes.clone(), &direct_image_path));
  assert!(package_entry_exists(bytes, &nested_image_path));
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
    .create_relationship_to_part_with_id(&mut package, image.clone(), "rIdSharedImage")
    .unwrap();
  assert_eq!(shared_id, "rIdSharedImage");
  assert_eq!(
    extended
      .create_relationship_to_part(&mut package, image.clone())
      .unwrap(),
    "rIdSharedImage"
  );
  assert!(
    extended
      .add_part_with_id(&mut package, image.clone(), "rIdDifferentSharedImage")
      .is_err()
  );
  assert_eq!(
    extended.get_id_of_part(&package, &image),
    Some("rIdSharedImage")
  );

  assert!(main_part.delete_part(&mut package, image.clone()).unwrap());
  assert!(main_part.get_part_by_id(&package, "rIdMainImage").is_none());
  assert_eq!(image.data(&package), Some(&b"shared image"[..]));

  let mut shared_buffer = Cursor::new(Vec::new());
  package.save(&mut shared_buffer).unwrap();
  assert!(package_entry_exists(
    shared_buffer.into_inner(),
    image_path.as_str()
  ));

  assert!(extended.delete_part(&mut package, image.clone()).unwrap());
  assert!(image.path(&package).is_none());

  let mut deleted_buffer = Cursor::new(Vec::new());
  package.save(&mut deleted_buffer).unwrap();
  assert!(!package_entry_exists(
    deleted_buffer.into_inner(),
    image_path.as_str()
  ));
}

#[test]
fn create_relationship_to_part_reuses_existing_parts_from_package() {
  // Source: test/DocumentFormat.OpenXml.Tests/ofapiTest/OpenXmlPackageTest.cs :: CreateRelationshipToPartTest
  let mut package = PresentationDocument::new_from_file_lazy(doc_sample("autosave.pptx")).unwrap();
  let presentation_part = package.presentation_part().unwrap();
  let slides: Vec<_> = presentation_part.slide_parts(&package).collect();
  assert!(slides.len() >= 2);
  let slide1 = slides[0].clone();
  let slide2 = slides[1].clone();
  let slide_layout1 = slide1
    .get_part_by_id(&package, "rId1")
    .and_then(|part| part_ref_variant!(part, SlideLayoutPart))
    .unwrap();
  let slide_layout2 = slide2
    .get_part_by_id(&package, "rId1")
    .and_then(|part| part_ref_variant!(part, SlideLayoutPart))
    .unwrap();

  assert!(
    slide1
      .delete_part(&mut package, slide_layout1.clone())
      .unwrap()
  );
  assert!(slide1.get_part_by_id(&package, "rId1").is_none());
  let slide1_relationship_id = slide1
    .create_relationship_to_part(&mut package, slide_layout2.clone())
    .unwrap();
  assert_eq!(
    slide1
      .get_part_by_id(&package, &slide1_relationship_id)
      .and_then(|part| part_ref_variant!(part, SlideLayoutPart))
      .map(|part| part.part_id()),
    Some(slide_layout2.part_id())
  );

  assert!(slide2.delete_part(&mut package, slide_layout2).unwrap());
  assert!(slide2.get_part_by_id(&package, "rId1").is_none());
  let slide2_relationship_id = slide2
    .create_relationship_to_part_with_id(&mut package, slide_layout1.clone(), "rId1001")
    .unwrap();
  assert_eq!(slide2_relationship_id, "rId1001");
  assert_eq!(
    slide2
      .get_part_by_id(&package, "rId1001")
      .and_then(|part| part_ref_variant!(part, SlideLayoutPart))
      .map(|part| part.part_id()),
    Some(slide_layout1.part_id())
  );

  let mut buffer = Cursor::new(Vec::new());
  package.save(&mut buffer).unwrap();

  let reopened = PresentationDocument::new(Cursor::new(buffer.into_inner())).unwrap();
  let reopened_presentation_part = reopened.presentation_part().unwrap();
  let reopened_slides: Vec<_> = reopened_presentation_part.slide_parts(&reopened).collect();
  assert!(
    reopened_slides[0]
      .get_part_by_id(&reopened, &slide1_relationship_id)
      .and_then(|part| part_ref_variant!(part, SlideLayoutPart))
      .is_some()
  );
  assert!(
    reopened_slides[1]
      .get_part_by_id(&reopened, "rId1001")
      .and_then(|part| part_ref_variant!(part, SlideLayoutPart))
      .is_some()
  );
}

#[test]
fn add_part_from_package_imports_part_tree_relationships_and_data_parts() {
  // Source: adapted from OpenXmlPartContainer.AddPart cross-package copy behavior.
  let mut source = WordprocessingDocument::new_from_file_lazy(doc_sample("Of16-01.docx")).unwrap();
  let source_main = source.main_document_part().unwrap();
  let source_header = source_main
    .add_new_part_auto_id::<_, HeaderPart>(&mut source)
    .unwrap();
  source_header
    .set_root_element(&mut source, Header::default())
    .unwrap();
  let source_image = source_header
    .add_image_part(&mut source, "image/png")
    .unwrap();
  source_image
    .set_data(&mut source, b"imported image".to_vec())
    .unwrap();
  source_header
    .add_external_relationship(
      &mut source,
      "rIdSourceExternal",
      "http://example.com/relationships/custom",
      "https://example.com/source",
    )
    .unwrap();
  source_header
    .add_hyperlink_relationship_with_mode(
      &mut source,
      "rIdSourceHyperlink",
      "#SourceBookmark",
      TargetMode::Internal,
    )
    .unwrap();
  let source_media = source
    .create_media_data_part_by_type(MediaDataPartType::Mp3)
    .unwrap();
  source_media
    .set_data(&mut source, b"imported media".to_vec())
    .unwrap();
  source_header
    .add_media_reference_relationship_with_id(&mut source, &source_media, "rIdSourceMedia")
    .unwrap();

  let mut target = WordprocessingDocument::new_from_file_lazy(doc_sample("Of16-02.docx")).unwrap();
  let target_main = target.main_document_part().unwrap();
  let imported_header = target_main
    .add_part_from_package_with_id(&mut target, &source, &source_header, "rIdImportedHeader")
    .unwrap();

  assert_eq!(
    target_main.get_id_of_part(&target, &imported_header),
    Some("rIdImportedHeader")
  );
  assert!(
    target_main
      .get_id_of_part(&target, &imported_header)
      .is_some()
  );
  assert!(
    imported_header
      .root_element(&mut target)
      .unwrap()
      .xml_children
      .is_empty()
  );

  let imported_image = imported_header
    .get_parts_of_type::<_, ImagePart>(&target)
    .next()
    .unwrap();
  assert_eq!(imported_image.data(&target).unwrap(), b"imported image");

  let external = imported_header
    .get_external_relationship(&target, "rIdSourceExternal")
    .unwrap();
  assert_eq!(external.target(), "https://example.com/source");
  let hyperlink = imported_header
    .get_hyperlink_relationship(&target, "rIdSourceHyperlink")
    .unwrap();
  assert_eq!(hyperlink.target(), "#SourceBookmark");
  assert!(matches!(hyperlink.target_mode(), TargetMode::Internal));

  let data_refs: Vec<_> = imported_header
    .data_part_reference_relationships(&target)
    .collect();
  assert_eq!(data_refs.len(), 1);
  assert_eq!(data_refs[0].id(), "rIdSourceMedia");
  let imported_media_part_id = data_refs[0].target_part_id().unwrap();
  let imported_media_part = media_data_part_by_id(&target, imported_media_part_id);
  assert_eq!(imported_media_part.content_type(&target), Some("audio/mp3"));
  assert_eq!(
    imported_media_part.data(&target),
    Some(b"imported media".as_slice())
  );

  let mut buffer = Cursor::new(Vec::new());
  target.save(&mut buffer).unwrap();
  let mut reopened = WordprocessingDocument::new(Cursor::new(buffer.into_inner())).unwrap();
  let reopened_main = reopened.main_document_part().unwrap();
  let reopened_header = reopened_main
    .get_part_by_id(&reopened, "rIdImportedHeader")
    .and_then(|part| part_ref_variant!(part, HeaderPart))
    .unwrap();
  assert!(reopened_header.root_element(&mut reopened).is_ok());
  assert!(
    reopened_header
      .get_parts_of_type::<_, ImagePart>(&reopened)
      .next()
      .is_some()
  );
  assert_eq!(
    reopened_header
      .data_part_reference_relationships(&reopened)
      .count(),
    1
  );
  assert!(
    reopened_header
      .get_external_relationship(&reopened, "rIdSourceExternal")
      .is_some()
  );
  assert!(
    reopened_header
      .get_hyperlink_relationship(&reopened, "rIdSourceHyperlink")
      .is_some()
  );
}

#[test]
fn add_part_from_package_imports_real_hyperlink_relationships() {
  // Source: test/DocumentFormat.OpenXml.Tests/ofapiTest/OpenXmlPartTest.cs :: HyperlinkRelationshipTest2
  let source = WordprocessingDocument::new_from_file(doc_sample("May_12_04.docx")).unwrap();
  let source_main = source.main_document_part().unwrap();
  let source_part_count = source_main.get_all_parts(&source).count();
  let source_hyperlinks: Vec<_> = source_main
    .hyperlink_relationships(&source)
    .map(|relationship| {
      (
        relationship.id().to_string(),
        relationship.target().to_string(),
        relationship.target_mode(),
      )
    })
    .collect();
  let source_external_count = source_main.external_relationships(&source).count();

  let mut target = WordprocessingDocument::new(empty_package()).unwrap();
  let imported_main = target
    .add_part_from_package_with_id(&source, &source_main, "rIdImportedMain")
    .unwrap();

  assert_eq!(
    imported_main.get_all_parts(&target).count(),
    source_part_count
  );
  let imported_hyperlinks: Vec<_> = imported_main
    .hyperlink_relationships(&target)
    .map(|relationship| {
      (
        relationship.id().to_string(),
        relationship.target().to_string(),
        relationship.target_mode(),
      )
    })
    .collect();
  assert_eq!(imported_hyperlinks, source_hyperlinks);
  assert_eq!(
    imported_main.external_relationships(&target).count(),
    source_external_count
  );

  let mut buffer = Cursor::new(Vec::new());
  target.save(&mut buffer).unwrap();
  let reopened = WordprocessingDocument::new(Cursor::new(buffer.into_inner())).unwrap();
  let reopened_main = reopened.main_document_part().unwrap();
  assert_eq!(
    reopened_main.hyperlink_relationships(&reopened).count(),
    source_hyperlinks.len()
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

  let core_id = package.get_id_of_part(&core).unwrap().to_string();
  let extended_id = package.get_id_of_part(&extended).unwrap().to_string();
  let custom_id = package.get_id_of_part(&custom).unwrap().to_string();
  let signature_origin_id = package
    .get_id_of_part(&signature_origin)
    .unwrap()
    .to_string();
  let mut buffer = Cursor::new(Vec::new());
  package.save(&mut buffer).unwrap();

  let reopened = WordprocessingDocument::new_lazy(Cursor::new(buffer.into_inner())).unwrap();
  let reopened_core = reopened
    .get_part_by_id(core_id.as_str())
    .and_then(|part| part_ref_variant!(part, CoreFilePropertiesPart))
    .unwrap();
  let reopened_extended = reopened
    .get_part_by_id(extended_id.as_str())
    .and_then(|part| part_ref_variant!(part, ExtendedFilePropertiesPart))
    .unwrap();
  let reopened_custom = reopened
    .get_part_by_id(custom_id.as_str())
    .and_then(|part| part_ref_variant!(part, CustomFilePropertiesPart))
    .unwrap();
  let reopened_signature_origin = reopened
    .get_part_by_id(signature_origin_id.as_str())
    .and_then(|part| part_ref_variant!(part, DigitalSignatureOriginPart))
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

  assert!(workbook_part.path(&package).is_some());
  assert!(workbook_part.theme_part(&package).is_some());
  assert!(workbook_part.worksheet_parts(&package).count() >= 1);
}

#[test]
fn spreadsheet_vml_drawing_part_is_raw_package_part_from_low_level_test() {
  // Source: test/DocumentFormat.OpenXml.Tests/OpenXmlDomTest/GenerateList4LowLevelTest.cs
  //   TestRootElementOfVmlDrawingPartIsLoadedAsUnknown
  let package = SpreadsheetDocument::new_from_file(doc_sample("vmldrawingroot.xlsx")).unwrap();
  let workbook_part = package.workbook_part().unwrap();
  let vml_part = workbook_part
    .worksheet_parts(&package)
    .find_map(|worksheet| worksheet.vml_drawing_parts(&package).next())
    .unwrap();

  assert_eq!(
    vml_part.content_type(&package),
    Some("application/vnd.openxmlformats-officedocument.vmlDrawing")
  );
  assert_eq!(vml_part.path(&package), Some("xl/drawings/vmlDrawing1.vml"));
  let data = vml_part.data_as_str(&package).unwrap().unwrap();
  assert!(data.contains("<xml"));
  assert!(data.contains("<v:shapetype"));
  assert!(data.contains("<v:shape"));
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
    .map(|worksheet| workbook_part.get_id_of_part(&package, &worksheet).unwrap())
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

  assert!(presentation_part.path(&package).is_some());
  assert!(presentation_part.slide_parts(&package).count() >= 1);
  assert!(presentation_part.slide_master_parts(&package).count() >= 1);
}

#[cfg(feature = "microsoft365")]
#[test]
fn model3d_reference_relationship_parts_use_powerpoint_content_type() {
  // Source: test/DocumentFormat.OpenXml.Packaging.Tests/OpenXmlPackageTests.cs
  //   TestOpenModel3DWrittenByPowerPoint_DotMime
  //   TestOpenModel3DWrittenByPowerPoint_DashMime
  for file_name in ["3dtestdot.pptx", "3dtestdash.pptx"] {
    let package = PresentationDocument::new_from_file(doc_sample(file_name)).unwrap();
    let presentation_part = package.presentation_part().unwrap();
    let slide_part = presentation_part.slide_parts(&package).next().unwrap();
    let model3d_part = slide_part
      .model3_d_reference_relationship_parts(&package)
      .next()
      .unwrap();

    assert_eq!(
      model3d_part.content_type(&package),
      Some("model/gltf-binary")
    );
  }
}

#[cfg(feature = "microsoft365")]
#[test]
fn wordprocessing_extended_chart_part_root_loads_from_office2016_unknown_element_test() {
  // Source: test/DocumentFormat.OpenXml.Tests/TestOffice2016.cs
  //   OF16_006_AccessChartPart_IntentionalUnknownElement
  let mut package =
    WordprocessingDocument::new_from_file_lazy(doc_sample("Of16-09-UnknownElement.docx")).unwrap();
  let main_part = package.main_document_part().unwrap();
  let chart_part = main_part.extended_chart_parts(&package).next().unwrap();

  assert_eq!(
    chart_part.content_type(&package),
    Some("application/vnd.ms-office.chartex+xml")
  );
  assert_eq!(chart_part.path(&package), Some("word/charts/chartEx1.xml"));
  assert!(chart_part.embedded_package_part(&package).is_some());

  let chart_space = chart_part.root_element(&mut package).unwrap();
  assert!(chart_space.chart_data_intentionally_changed.is_some());
  assert_eq!(
    chart_space
      .chart
      .plot_area
      .plot_area_region
      .cx_series
      .first()
      .map(|series| series.layout_id),
    Some(SeriesLayout::Waterfall)
  );
}

#[test]
fn package_save_roundtrips_unmodified_document() {
  let original = WordprocessingDocument::new_from_file_lazy(doc_sample("Of16-01.docx")).unwrap();
  let mut buffer = Cursor::new(Vec::new());

  original.save(&mut buffer).unwrap();

  let mut roundtripped = WordprocessingDocument::new(Cursor::new(buffer.into_inner())).unwrap();
  let main_part = roundtripped.main_document_part().unwrap();
  let root = main_part.root_element(&mut roundtripped).unwrap();
  assert!(main_document_body_child_count(root) > 0);
}

#[test]
fn package_copy_helpers_include_unsaved_root_changes() {
  // Source: upstream package Clone(stream) behavior adapted to Rust copy helpers.
  let mut package = WordprocessingDocument::new(empty_package()).unwrap();
  let main_part = package.add_main_document_part().unwrap();
  main_part
    .set_root_element(&mut package, empty_body_document())
    .unwrap();
  let header_part = main_part
    .add_new_part::<_, HeaderPart>(&mut package, "rIdCopyHeader")
    .unwrap();
  header_part
    .set_root_element(&mut package, Header::default())
    .unwrap();

  let mut copied_stream = Cursor::new(Vec::new());
  package.copy_to(&mut copied_stream).unwrap();
  let copied_bytes = package.to_package_bytes().unwrap();
  assert!(!copied_bytes.is_empty());
  assert_eq!(copied_stream.into_inner(), copied_bytes);

  let mut owned_copy = package.to_owned_package().unwrap();
  let copied_main = owned_copy.main_document_part().unwrap();
  assert!(
    copied_main
      .root_element(&mut owned_copy)
      .unwrap()
      .body
      .is_some()
  );
  let copied_header = copied_main
    .get_part_by_id(&owned_copy, "rIdCopyHeader")
    .and_then(|part| part_ref_variant!(part, HeaderPart))
    .unwrap();
  assert!(copied_header.root_element(&mut owned_copy).is_ok());

  let mut reopened_from_bytes = WordprocessingDocument::new(Cursor::new(copied_bytes)).unwrap();
  let reopened_main = reopened_from_bytes.main_document_part().unwrap();
  assert!(
    reopened_main
      .root_element(&mut reopened_from_bytes)
      .unwrap()
      .body
      .is_some()
  );
  assert!(
    reopened_main
      .get_part_by_id(&reopened_from_bytes, "rIdCopyHeader")
      .and_then(|part| part_ref_variant!(part, HeaderPart))
      .is_some()
  );
}

#[test]
fn package_copy_retains_part_names_when_adding_more_parts() {
  // Source: test/DocumentFormat.OpenXml.Tests/SaveAndCloneTests.cs :: CloneRetainsPartNames
  let mut presentation = PresentationDocument::new(empty_package()).unwrap();
  let presentation_part = presentation
    .add_new_part::<PresentationPart>("rIdPresentation")
    .unwrap();
  presentation_part
    .set_root_element(&mut presentation, PmlPresentation::default())
    .unwrap();
  let slide1 = presentation_part
    .add_new_part_auto_id::<_, SlidePart>(&mut presentation)
    .unwrap();
  slide1
    .set_root_element(&mut presentation, Slide::default())
    .unwrap();
  assert_eq!(slide1.path(&presentation), Some("ppt/slides/slide1.xml"));

  let mut duplicate = presentation.to_owned_package().unwrap();
  let duplicate_presentation_part = duplicate.presentation_part().unwrap();
  let slide2 = duplicate_presentation_part
    .add_new_part_auto_id::<_, SlidePart>(&mut duplicate)
    .unwrap();
  slide2
    .set_root_element(&mut duplicate, Slide::default())
    .unwrap();
  assert_eq!(slide2.path(&duplicate), Some("ppt/slides/slide2.xml"));

  let slide_paths: Vec<_> = duplicate_presentation_part
    .slide_parts(&duplicate)
    .map(|slide| slide.path(&duplicate).unwrap().to_string())
    .collect();
  assert_eq!(
    slide_paths,
    vec![
      "ppt/slides/slide1.xml".to_string(),
      "ppt/slides/slide2.xml".to_string()
    ]
  );
}
