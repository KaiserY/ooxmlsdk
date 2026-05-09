# ooxmlsdk-pdf-test fixtures

This directory contains DOCX fixtures used by `crates/ooxmlsdk-pdf-test` for
LibreOffice-to-`ooxmlsdk-pdf` calibration.

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
