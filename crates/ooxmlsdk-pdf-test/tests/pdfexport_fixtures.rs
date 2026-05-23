use ooxmlsdk_pdf_test::{
  PdfBounds, PdfSummary, assert_pdf_rect_close, pdf_page_count_for_fixture,
  pdf_summary_for_fixture, pdf_summary_for_fixture_with_options, pdfexport_fixture_dir,
};

fn fixture(name: &str) -> std::path::PathBuf {
  pdfexport_fixture_dir().join(name)
}

fn render_summary(name: &str) -> PdfSummary {
  pdf_summary_for_fixture(&fixture(name)).unwrap()
}

fn raw_page(summary: &PdfSummary, page_index: usize) -> &ooxmlsdk_pdf_test::RawPageSummary {
  summary
    .raw_pages
    .iter()
    .find(|page| page.page_index == page_index)
    .unwrap_or_else(|| panic!("missing raw page summary for page {page_index}"))
}

fn page_object_count(summary: &PdfSummary, page_index: usize) -> usize {
  let page = summary
    .page_objects
    .iter()
    .find(|page| page.page_index == page_index)
    .unwrap_or_else(|| panic!("missing page object summary for page {page_index}"));
  page.text_objects
    + page.path_objects
    + page.image_objects
    + page.shading_objects
    + page.form_objects
    + page.unsupported_objects
}

#[test]
// Source: ../core/vcl/qa/cppunit/pdfexport/pdfexport2.cxx:testTdf161346
fn pdfexport_fixture_fdo47811_word2013_has_two_pages() {
  assert_eq!(
    pdf_page_count_for_fixture(&fixture("fdo47811-1_Word2013.docx")).unwrap(),
    2
  );
}

#[test]
// Source: ../core/vcl/qa/cppunit/pdfexport/pdfexport.cxx:testTdf145274
fn pdfexport_fixture_tdf145274_matches_upstream_text_object_expectations() {
  let summary = render_summary("tdf145274.docx");
  assert_eq!(summary.page_count, 1);
  let actual_object_count = page_object_count(&summary, 0);
  assert_eq!(
    actual_object_count,
    6,
    "page_objects={:?}\ntext_objects={:?}\npath_objects={:?}\nimages={:?}",
    summary.page_objects,
    summary
      .text_objects
      .iter()
      .filter(|object| object.page_index == 0)
      .collect::<Vec<_>>(),
    summary
      .paths
      .iter()
      .filter(|object| object.page_index == 0)
      .collect::<Vec<_>>(),
    summary
      .images
      .iter()
      .filter(|object| object.page_index == 0)
      .collect::<Vec<_>>()
  );

  let text_objects = summary
    .text_objects
    .iter()
    .filter(|object| object.page_index == 0)
    .collect::<Vec<_>>();

  for object in text_objects {
    assert_eq!(object.scaled_font_size, "11.00");
    assert_eq!(object.render_mode, "FilledUnstroked");
    assert_eq!(object.fill_color.as_deref(), Some("#800000@ff"));
  }
}

#[test]
// Source: ../core/vcl/qa/cppunit/pdfexport/pdfexport.cxx:testTdf156685
fn pdfexport_fixture_tdf156685_matches_upstream_text_object_expectations() {
  let summary = render_summary("tdf156685.docx");
  assert_eq!(summary.page_count, 1);
  let actual_object_count = page_object_count(&summary, 0);
  assert_eq!(
    actual_object_count,
    9,
    "page_objects={:?}\ntext_objects={:?}\npath_objects={:?}\nimages={:?}",
    summary.page_objects,
    summary
      .text_objects
      .iter()
      .filter(|object| object.page_index == 0)
      .collect::<Vec<_>>(),
    summary
      .paths
      .iter()
      .filter(|object| object.page_index == 0)
      .collect::<Vec<_>>(),
    summary
      .images
      .iter()
      .filter(|object| object.page_index == 0)
      .collect::<Vec<_>>()
  );

  let text_objects = summary
    .text_objects
    .iter()
    .filter(|object| object.page_index == 0)
    .collect::<Vec<_>>();

  for object in text_objects {
    assert_eq!(object.scaled_font_size, "11.00");
    assert_eq!(object.render_mode, "FilledUnstroked");
    assert_eq!(object.fill_color.as_deref(), Some("#000000@ff"));
  }
}

#[test]
// Source: ../core/vcl/qa/cppunit/pdfexport/pdfexport.cxx:testTdf142133
fn pdfexport_fixture_tdf142133_preserves_google_link_annotation() {
  let summary = render_summary("tdf142133.docx");
  assert_eq!(summary.page_count, 1);
  let page = raw_page(&summary, 0);
  assert_eq!(page.annotation_count, 1);
  assert_eq!(page.annotations.len(), 1);
  assert_eq!(page.annotations[0].type_name.as_deref(), Some("Annot"));
  assert_eq!(page.annotations[0].subtype_name.as_deref(), Some("Link"));
  assert_eq!(
    page.annotations[0].action_uri.as_deref(),
    Some("https://google.com/")
  );
}

#[test]
// Source: ../core/vcl/qa/cppunit/pdfexport/pdfexport2.cxx:testTdf152246
fn pdfexport_fixture_content_control_rtl_matches_upstream_widget_rects() {
  let summary = render_summary("content-control-rtl.docx");
  assert_eq!(summary.page_count, 1);
  let page = raw_page(&summary, 0);
  assert_eq!(page.annotation_count, 5);

  let widgets = page
    .annotations
    .iter()
    .filter(|annotation| annotation.type_name.as_deref() == Some("Annot"))
    .filter(|annotation| annotation.subtype_name.as_deref() == Some("Widget"))
    .collect::<Vec<_>>();

  assert_eq!(widgets.len(), 5);

  let expected = [
    PdfBounds {
      left: 55.699,
      bottom: 706.701,
      right: 132.401,
      top: 722.499,
    },
    PdfBounds {
      left: 197.499,
      bottom: 706.701,
      right: 274.201,
      top: 722.499,
    },
    PdfBounds {
      left: 302.349,
      bottom: 679.101,
      right: 379.051,
      top: 694.899,
    },
    PdfBounds {
      left: 479.599,
      bottom: 679.101,
      right: 556.301,
      top: 694.899,
    },
    PdfBounds {
      left: 55.699,
      bottom: 651.501,
      right: 132.401,
      top: 667.299,
    },
  ];

  for (widget, expected_rect) in widgets.iter().zip(expected) {
    let bounds = widget
      .rect
      .as_deref()
      .unwrap_or_else(|| panic!("missing widget bounds for {:?}", widget));
    assert_pdf_rect_close(bounds, expected_rect, 0.001);
  }
}

#[test]
// Source: ../core/vcl/qa/cppunit/pdfexport/pdfexport2.cxx:testTdf129085
fn pdfexport_fixture_tdf129085_preserves_single_jpeg_xobject() {
  let summary = pdf_summary_for_fixture_with_options(
    &fixture("tdf129085.docx"),
    ooxmlsdk_pdf::PdfOptions {
      jpeg_quality: Some(50),
      ..ooxmlsdk_pdf::PdfOptions::default()
    },
  )
  .unwrap();
  assert_eq!(summary.page_count, 1);

  let page = raw_page(&summary, 0);
  assert_eq!(page.xobjects.len(), 1);

  let xobject = &page.xobjects[0];
  assert_eq!(xobject.type_name.as_deref(), Some("XObject"));
  assert_eq!(xobject.subtype_name.as_deref(), Some("Image"));
  assert_eq!(
    xobject.image_format.as_deref(),
    Some("Jpeg"),
    "xobject={xobject:?}"
  );
  assert_eq!(xobject.decoded_width_px, Some(884));
  assert_eq!(xobject.decoded_height_px, Some(925));
  assert_eq!(xobject.bits_per_pixel, Some(24));
}

#[test]
// Source: ../core/svx/qa/unit/svdraw.cxx:testPageViewDrawLayerClip
fn pdfexport_fixture_page_view_draw_layer_clip_matches_page_object_counts() {
  let summary = render_summary("page-view-draw-layer-clip.docx");
  assert_eq!(summary.page_count, 2);
  assert_eq!(page_object_count(&summary, 0), 3);
  assert_eq!(page_object_count(&summary, 1), 2);
}

#[test]
// Source: ../core/sw/qa/core/text/itrform2.cxx:testContentControlHeaderPDFExport
fn pdfexport_fixture_content_control_header_preserves_page_two_text_objects() {
  let summary = render_summary("content-control-header.docx");
  assert!(summary.page_count >= 2);

  let page_two_text_objects = summary
    .text_objects
    .iter()
    .filter(|object| object.page_index == 1)
    .count();
  assert_eq!(page_two_text_objects, 3);
}

#[test]
// Source: ../core/sw/qa/core/text/text.cxx:testDropdownContentControlPDF2
fn pdfexport_fixture_dropdown_content_control_preserves_combo_widget_value() {
  let summary = render_summary("tdf153040.docx");
  assert_eq!(summary.page_count, 1);
  let page = raw_page(&summary, 0);
  assert_eq!(page.annotation_count, 4);

  let first = page.annotations.first().expect("missing first annotation");
  assert_eq!(first.subtype_name.as_deref(), Some("Widget"));
  assert_eq!(first.field_type_name.as_deref(), Some("Ch"));
  assert_eq!(first.field_value.as_deref(), Some("Apfel"));
}

#[test]
// Source: ../core/sw/qa/extras/uiwriter/uiwriter8.cxx:testTdf131728
fn pdfexport_fixture_tdf131728_preserves_bookmark_order() {
  let summary = render_summary("tdf131728.docx");
  assert_eq!(summary.page_count, 1);
  assert_eq!(
    summary.outlines,
    [
      "Article 1. Definitions",
      " Apple",
      " Bread",
      " Cable",
      " Cable",
      "Article 2. Three style separators in one line!",
      " Heading 2",
      " Heading 2 Again",
    ]
  );
}
