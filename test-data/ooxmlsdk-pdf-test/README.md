# ooxmlsdk-pdf-test fixtures

This directory contains DOCX fixtures used by `crates/ooxmlsdk-pdf-test` for
LibreOffice-to-`ooxmlsdk-pdf` calibration.

`ooxmlsdk-pdf-test` assumes these DOCX files are valid test inputs. DOCX
package/schema/relationship round-trip behavior is covered by
`crates/ooxmlsdk-test`; this directory is only for PDF export rendering parity.

Fixtures in this directory should come from real upstream compatibility suites,
starting with LibreOffice Writer QA data under `../core/sw/qa/`. Avoid adding
hand-crafted DOCX files for renderer-specific behavior; real-world and upstream
regression files give better parity signal.

Current fixtures:

- `libreoffice-ooxmlexport-1_page.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/1_page.docx`.
- `libreoffice-ooxmlexport-footnote.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/footnote.docx`.
- `libreoffice-ooxmlexport-multi-column-separator-with-line.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/multi-column-separator-with-line.docx`.
- `libreoffice-ooxmlexport-table-auto-nested.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/table-auto-nested.docx`.
- `libreoffice-ooxmlexport-tdf78657-picture-hyperlink.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf78657_picture_hyperlink.docx`.

Strict comparison policy:

- Allowed PDF container differences: object numbers, xref layout, stream
  compression, producer metadata, and harmless numeric serialization precision.
- Required content/layout parity: PDFium-observed page geometry, text
  segments/character boxes, text object font-size/color/bounds details, image
  object geometry, path object segment-coordinate/fill/stroke/bounds details,
  page object summaries, link target kinds/targets/rectangles, and raster render
  checksums must match LibreOffice.
- Poppler `pdftotext` and decoded content-stream operation summaries are
  diagnostics, not replacements for PDFium object/raster comparison.
- The strict test intentionally reports every fixture mismatch in one run. Do
  not add known-issue whitelists; fix the renderer or make an explicitly
  documented PDF-compatibility normalization in `ooxmlsdk-pdf-test`.
