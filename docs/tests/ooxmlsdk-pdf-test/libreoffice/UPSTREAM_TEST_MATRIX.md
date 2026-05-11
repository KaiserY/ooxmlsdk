# LibreOffice PDF Fixture Matrix

This matrix tracks the `test-data/ooxmlsdk-pdf-test/libreoffice/` fixture set
against the local LibreOffice checkout at `../core`.

## Scope Rules

- Keep `libreoffice/` limited to fixtures copied from `../core`.
- Prefer `../core/vcl/qa/cppunit/pdfexport/data/` when a fixture is covered by
  an explicit upstream PDF export assertion.
- Allow a small number of DOCX fixtures from other LibreOffice suites only when
  the local Rust assertion checks a final PDF-visible property and does not
  depend on LibreOffice internals.
- Put any non-`../core` PDF fixture in a sibling `misc/` bucket instead of
  weakening this boundary.

## Status Legend

- `covered`: fixture is present locally and exercised by a Rust PDF test.
- `planned`: fixture provenance is known, but no Rust assertion exists yet.

## Summary

- `covered`: 13
- `planned`: 0

## Fixtures

| Fixture | Upstream source | Local Rust test | Status | Notes |
|---|---|---|---|---|
| `content-control-rtl.docx` | `../core/vcl/qa/cppunit/pdfexport/data/content-control-rtl.docx` | `crates/ooxmlsdk-pdf-test/tests/pdfexport_fixtures.rs::pdfexport_fixture_content_control_rtl_matches_upstream_widget_rects` | `covered` | Source assertion: `pdfexport2.cxx:testTdf152246` |
| `fdo47811-1_Word2013.docx` | `../core/vcl/qa/cppunit/pdfexport/data/fdo47811-1_Word2013.docx` | `crates/ooxmlsdk-pdf-test/tests/pdfexport_fixtures.rs::pdfexport_fixture_fdo47811_word2013_has_two_pages` | `covered` | Source assertion: `pdfexport2.cxx:testTdf161346` |
| `tdf129085.docx` | `../core/vcl/qa/cppunit/pdfexport/data/tdf129085.docx` | `crates/ooxmlsdk-pdf-test/tests/pdfexport_fixtures.rs::pdfexport_fixture_tdf129085_preserves_single_jpeg_xobject` | `covered` | Source assertion: `pdfexport2.cxx:testTdf129085` |
| `tdf142133.docx` | `../core/vcl/qa/cppunit/pdfexport/data/tdf142133.docx` | `crates/ooxmlsdk-pdf-test/tests/pdfexport_fixtures.rs::pdfexport_fixture_tdf142133_preserves_google_link_annotation` | `covered` | Source assertion: `pdfexport.cxx:testTdf142133` |
| `tdf145274.docx` | `../core/vcl/qa/cppunit/pdfexport/data/tdf145274.docx` | `crates/ooxmlsdk-pdf-test/tests/pdfexport_fixtures.rs::pdfexport_fixture_tdf145274_matches_upstream_text_object_expectations` | `covered` | Source assertion: `pdfexport.cxx:testTdf145274` |
| `tdf156685.docx` | `../core/vcl/qa/cppunit/pdfexport/data/tdf156685.docx` | `crates/ooxmlsdk-pdf-test/tests/pdfexport_fixtures.rs::pdfexport_fixture_tdf156685_matches_upstream_text_object_expectations` | `covered` | Source assertion: `pdfexport.cxx:testTdf156685` |
| `chart-data-label-char-color.docx` | `../core/oox/qa/unit/data/chart-data-label-char-color.docx` | `crates/ooxmlsdk-pdf-test/tests/core_docx_pdf_fixtures.rs::core_docx_pdf_fixture_chart_data_label_char_color_preserves_white_label` | `covered` | Source assertion: `drawingml.cxx:testChartDataLabelCharColor` |
| `semi-transparent-text.docx` | `../core/sw/qa/writerfilter/dmapper/data/semi-transparent-text.docx` | `crates/ooxmlsdk-pdf-test/tests/core_docx_pdf_fixtures.rs::core_docx_pdf_fixture_semi_transparent_text_preserves_alpha` | `covered` | Source assertion: `TextEffectsHandler.cxx:testSemiTransparentText` |
| `tdf125885_WordArt2.docx` | `../core/oox/qa/unit/data/tdf125885_WordArt2.docx` | `crates/ooxmlsdk-pdf-test/tests/core_docx_pdf_fixtures.rs::core_docx_pdf_fixture_wordart2_preserves_fill_and_stroke_colors` | `covered` | Source assertion: `shape.cxx:testWriterFontwork2` |
| `tdf152840_WordArt_non_accent_color.docx` | `../core/oox/qa/unit/data/tdf152840_WordArt_non_accent_color.docx` | `crates/ooxmlsdk-pdf-test/tests/core_docx_pdf_fixtures.rs::core_docx_pdf_fixture_wordart_non_accent_colors_preserve_all_expected_fills` | `covered` | Source assertion: `shape.cxx:testWriterFontworkNonAccentColor` |
| `tdf152884_Char_Transparency.docx` | `../core/sw/qa/writerfilter/dmapper/data/tdf152884_Char_Transparency.docx` | `crates/ooxmlsdk-pdf-test/tests/core_docx_pdf_fixtures.rs::core_docx_pdf_fixture_theme_color_transparency_preserves_alpha` | `covered` | Source assertion: `TextEffectsHandler.cxx:testThemeColorTransparency` |
| `tdf152896_WordArt_color_darken.docx` | `../core/oox/qa/unit/data/tdf152896_WordArt_color_darken.docx` | `crates/ooxmlsdk-pdf-test/tests/core_docx_pdf_fixtures.rs::core_docx_pdf_fixture_wordart_darken_color_preserves_result_fill` | `covered` | Source assertion: `shape.cxx:testWriterFontworkDarkenTransparency` |
| `tdf54095_SmartArtThemeTextColor.docx` | `../core/oox/qa/unit/data/tdf54095_SmartArtThemeTextColor.docx` | `crates/ooxmlsdk-pdf-test/tests/core_docx_pdf_fixtures.rs::core_docx_pdf_fixture_smartart_theme_text_color_preserves_dark2_text` | `covered` | Source assertion: `shape.cxx:testTdf54095_SmartArtThemeTextColor` |
