# ooxmlsdk-pdf-test Fixture Layout

`test-data/ooxmlsdk-pdf-test/` is reserved for `crates/ooxmlsdk-pdf-test`.
These fixtures are not package round-trip fixtures; they are `docx -> pdf`
parity inputs for PDFium-observable assertions.

Buckets:

- `libreoffice/`: fixtures copied from the local LibreOffice checkout under
  `../core`, primarily `vcl/qa/cppunit/pdfexport/data/` plus a small number of
  visible-output DOCX fixtures from other LibreOffice test suites.
- `misc/`: reserved for future non-LibreOffice PDF fixtures. Do not mix those
  into `libreoffice/`.

Boundary:

- If a DOCX is traceable to `../core`, it belongs under `libreoffice/`.
- If it is not traceable to `../core`, it must not be stored under
  `libreoffice/`.
