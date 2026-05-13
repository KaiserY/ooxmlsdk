# ooxmlsdk-pdf-test fixtures

This directory contains DOCX fixtures used by `crates/ooxmlsdk-pdf-test` for
upstream-aligned PDF export assertions.

`ooxmlsdk-pdf-test` assumes these DOCX files are valid test inputs. DOCX
package/schema/relationship round-trip behavior is covered by
`crates/ooxmlsdk-test`; this directory is only for `docx -> pdf` behavior.

Fixtures in this directory should come from LibreOffice DOCX -> visible PDF
coverage, paired with an existing upstream assertion. Prefer strict
`../core/vcl/qa/cppunit/pdfexport/data/` fixtures when available, then scattered
Writer/SVX/OoXML tests that export or assert visible page output. Avoid adding
hand-crafted fixtures or inferred expectations; copy upstream fixtures and port
the upstream assertion values directly.

Fixtures that are not traceable to `../core` should not live here. Put them in
a sibling `misc/` bucket once that category exists so the LibreOffice boundary
stays explicit.

Strict upstream `pdfexport` / `pdfexport2` DOCX fixtures:

- `content-control-rtl.docx`: copied from
  `../core/vcl/qa/cppunit/pdfexport/data/content-control-rtl.docx`.
- `fdo47811-1_Word2013.docx`: copied from
  `../core/vcl/qa/cppunit/pdfexport/data/fdo47811-1_Word2013.docx`.
- `tdf142133.docx`: copied from
  `../core/vcl/qa/cppunit/pdfexport/data/tdf142133.docx`.
- `tdf145274.docx`: copied from
  `../core/vcl/qa/cppunit/pdfexport/data/tdf145274.docx`.
- `tdf156685.docx`: copied from
  `../core/vcl/qa/cppunit/pdfexport/data/tdf156685.docx`.
- `tdf129085.docx`: copied from
  `../core/vcl/qa/cppunit/pdfexport/data/tdf129085.docx`.

Additional direct PDF/object fixtures:

- `page-view-draw-layer-clip.docx`: copied from
  `../core/svx/qa/unit/data/page-view-draw-layer-clip.docx`.
- `content-control-header.docx`: copied from
  `../core/sw/qa/core/text/data/content-control-header.docx`.
- `tdf153040.docx`: copied from
  `../core/sw/qa/core/text/data/tdf153040.docx`.
- `tdf131728.docx`: copied from
  `../core/sw/qa/extras/uiwriter/data/tdf131728.docx`.

Core-derived visible-output DOCX fixtures:

- `semi-transparent-text.docx`: copied from
  `../core/sw/qa/writerfilter/dmapper/data/semi-transparent-text.docx`.
- `tdf152884_Char_Transparency.docx`: copied from
  `../core/sw/qa/writerfilter/dmapper/data/tdf152884_Char_Transparency.docx`.
- `chart-data-label-char-color.docx`: copied from
  `../core/oox/qa/unit/data/chart-data-label-char-color.docx`.
- `tdf54095_SmartArtThemeTextColor.docx`: copied from
  `../core/oox/qa/unit/data/tdf54095_SmartArtThemeTextColor.docx`.
- `tdf125885_WordArt2.docx`: copied from
  `../core/oox/qa/unit/data/tdf125885_WordArt2.docx`.
- `tdf152840_WordArt_non_accent_color.docx`: copied from
  `../core/oox/qa/unit/data/tdf152840_WordArt_non_accent_color.docx`.
- `tdf152896_WordArt_color_darken.docx`: copied from
  `../core/oox/qa/unit/data/tdf152896_WordArt_color_darken.docx`.

These supplemental DOCX fixtures are not native `pdfexport` cases. They come
from other LibreOffice subsystems, but each one asserts a final visible-output
property that can be checked directly on the generated PDF without depending on
Writer's internal document model.

Assertion policy:

- Render the fixture only through `ooxmlsdk-pdf`; do not shell out to
  `soffice` or compare against a dynamically generated LibreOffice PDF.
- Prefer PDFium-observable assertions that mirror upstream `pdfexport` tests:
  page counts, annotation counts and bounds, text object font/color details,
  and link targets.
- When upstream checks PDF object dictionaries directly and PDFium cannot expose
  the same signal, add a narrow PDF-structure assertion instead of inventing a
  proxy metric.
- Tests are allowed to fail while the renderer is being brought up to parity;
  the goal is to expose gaps against upstream expectations, not to normalize
  them away.
