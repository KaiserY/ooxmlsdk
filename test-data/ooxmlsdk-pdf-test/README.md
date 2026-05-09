# ooxmlsdk-pdf-test fixtures

This directory contains DOCX fixtures used by `crates/ooxmlsdk-pdf-test` for
upstream-aligned PDF export assertions.

`ooxmlsdk-pdf-test` assumes these DOCX files are valid test inputs. DOCX
package/schema/relationship round-trip behavior is covered by
`crates/ooxmlsdk-test`; this directory is only for `docx -> pdf` behavior.

Fixtures in this directory should come from LibreOffice's PDF export coverage
under `../core/vcl/qa/cppunit/pdfexport/data/`, paired with an existing
assertion in `pdfexport.cxx` or `pdfexport2.cxx`. Avoid adding hand-crafted
fixtures or inferred expectations; copy upstream fixtures and port the upstream
assertion values directly.

Current fixtures:

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
