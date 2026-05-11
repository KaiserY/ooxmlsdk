# misc

Use this bucket only for fixtures that are:

- not copied from `../Open-XML-SDK`, and
- not part of the project-owned spec fixture sets under `specs/`.

This directory is the escape hatch for intentionally out-of-band coverage.
Keep the reason explicit in the commit diff and add a short provenance note when
the first fixture lands here.

Current fixtures moved out of `Open-XML-SDK/` after checking against
`../Open-XML-SDK/test/DocumentFormat.OpenXml.Tests.Assets/assets/TestFiles`:

- `Dickinson_Sample_Slides.pptx`
- `Products.xlsx`
- `demo.docx`
- `layout_altcontent.pptx`
- `master_altcontent.pptx`
- `notes_altcontent.pptx`
- `slide_altcontent.pptx`
