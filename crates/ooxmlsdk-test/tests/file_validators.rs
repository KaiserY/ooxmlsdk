#![cfg(all(feature = "parts", feature = "validators"))]

use ooxmlsdk::parts::{
  presentation_document::PresentationDocument, spreadsheet_document::SpreadsheetDocument,
  wordprocessing_document::WordprocessingDocument,
};
use ooxmlsdk_test::fixtures;

fn test_file_path(file_name: &str) -> std::path::PathBuf {
  let path = fixtures::doc_sample_path(file_name);
  assert!(path.is_file(), "missing doc sample: {}", path.display());
  path
}

fn assert_presentation_document_validates(file_name: &str) {
  let package = PresentationDocument::new_from_file(test_file_path(file_name)).unwrap();
  package.presentation_part.validate().unwrap();
}

fn assert_spreadsheet_document_validates(file_name: &str) {
  let package = SpreadsheetDocument::new_from_file(test_file_path(file_name)).unwrap();
  package.workbook_part.validate().unwrap();
}

fn assert_wordprocessing_document_validates(file_name: &str) {
  let package = WordprocessingDocument::new_from_file(test_file_path(file_name)).unwrap();
  package.main_document_part.validate().unwrap();
}

#[cfg(feature = "microsoft365")]
#[test]
fn youtube_xlsx_validation_from_web_extension_test() {
  // Source: test/DocumentFormat.OpenXml.Tests/ConformanceTest/WebExtension/WebExtensionTest.cs :: WebExtensionAcceptance
  assert_spreadsheet_document_validates("Youtube.xlsx");
}

#[test]
fn basicspreadsheet_xlsx_validation_from_openxml_validator_test() {
  // Source: test/DocumentFormat.OpenXml.Tests/ofapiTest/OpenXmlValidatorTest.cs :: SpreadsheetDocumentValidatingTest
  assert_spreadsheet_document_validates("basicspreadsheet.xlsx");
}

#[test]
fn additional_spreadsheet_doc_samples_validate() {
  // Source: upstream Open XML SDK test assets under test/DocumentFormat.OpenXml.Tests.Assets/TestFiles
  for file_name in ["Comments.xlsx", "Products.xlsx"] {
    assert_spreadsheet_document_validates(file_name);
  }
}

#[cfg(feature = "microsoft365")]
#[test]
fn microsoft365_spreadsheet_doc_samples_validate() {
  // Source: upstream Open XML SDK test assets under test/DocumentFormat.OpenXml.Tests.Assets/TestFiles
  assert_spreadsheet_document_validates("Spreadsheet.xlsx");
}

#[cfg(feature = "microsoft365")]
#[test]
fn office2016_pptx_validation_from_test_office2016() {
  // Source: test/DocumentFormat.OpenXml.Tests/TestOffice2016.cs :: OF16_002_ValidatePptx_2016
  for file_name in ["Of16-01.pptx", "Of16-02.pptx", "Of16-03.pptx"] {
    assert_presentation_document_validates(file_name);
  }
}

#[cfg(feature = "microsoft365")]
#[test]
fn office2013_pptx_validation_from_test_office2016() {
  // Source: test/DocumentFormat.OpenXml.Tests/TestOffice2016.cs :: OF16_004_ValidatePptx_2013
  for file_name in ["Of16-01.pptx", "Of16-02.pptx", "Of16-03.pptx"] {
    assert_presentation_document_validates(file_name);
  }
}

#[test]
fn plain_docx_validation_from_openxml_validator_test() {
  // Source: test/DocumentFormat.OpenXml.Tests/ofapiTest/OpenXmlValidatorTest.cs
  assert_wordprocessing_document_validates("Plain.docx");
}

#[test]
fn may_12_04_docx_validation_from_openxml_validator_test() {
  // Source: test/DocumentFormat.OpenXml.Tests/ofapiTest/OpenXmlValidatorTest.cs
  assert_wordprocessing_document_validates("May_12_04.docx");
}

#[test]
fn additional_wordprocessing_doc_samples_validate() {
  // Source: upstream Open XML SDK test assets under test/DocumentFormat.OpenXml.Tests.Assets/TestFiles
  for file_name in ["Document.docx", "mailmerge.docx", "simpleSdt.docx"] {
    assert_wordprocessing_document_validates(file_name);
  }
}

#[cfg(feature = "microsoft365")]
#[test]
fn microsoft365_wordprocessing_doc_samples_validate() {
  // Source: upstream Open XML SDK test assets under test/DocumentFormat.OpenXml.Tests.Assets/TestFiles
  for file_name in ["HelloWorld.docx", "Hyperlink.docx", "Notes.docx"] {
    assert_wordprocessing_document_validates(file_name);
  }
}

#[test]
fn presentation_pptx_validation_from_openxml_validator_test() {
  // Source: test/DocumentFormat.OpenXml.Tests/ofapiTest/OpenXmlValidatorTest.cs
  assert_presentation_document_validates("Presentation.pptx");
}

#[test]
fn o09_performance_typical_pptx_validation_from_openxml_validator_test() {
  // Source: test/DocumentFormat.OpenXml.Tests/ofapiTest/OpenXmlValidatorTest.cs :: PresentationDocumentValidatingTest
  assert_presentation_document_validates("o09_Performance_typical.pptx");
}

#[test]
fn additional_presentation_doc_samples_validate() {
  // Source: upstream Open XML SDK test assets under test/DocumentFormat.OpenXml.Tests.Assets/TestFiles
  for file_name in ["animation.pptx", "autosave.pptx", "mediareference.pptx"] {
    assert_presentation_document_validates(file_name);
  }
}

#[cfg(feature = "microsoft365")]
#[test]
fn microsoft365_presentation_doc_samples_validate() {
  // Source: upstream Open XML SDK test assets under test/DocumentFormat.OpenXml.Tests.Assets/TestFiles
  for file_name in ["Algn_tab_TabAlignment.pptx", "Presentation.potx"] {
    assert_presentation_document_validates(file_name);
  }
}
