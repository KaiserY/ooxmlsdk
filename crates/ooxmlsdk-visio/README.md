# ooxmlsdk-visio

Visio package editing and stored ShapeSheet values for
[`ooxmlsdk`](https://crates.io/crates/ooxmlsdk). The crate shares its typed XML
and package model with the core SDK.

**Status:** experimental workspace crate; publication is deferred. Existing
authoring tests pass, but six real Apache POI VSDX/VSDM fixtures fail during
import because the current font model requires `FaceName/@ID` and `@Name`
instead of the files' `@NameU`. The
[Microsoft element reference](https://learn.microsoft.com/en-us/office/client-developer/visio/facename-element-facenames_type-complextypevisio-xml)
also specifies `NameU`. Resolve the Visio schema compatibility gaps before
publishing this crate.

```rust,no_run
use ooxmlsdk_visio::{Ids, minimal_drawing};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ids = Ids::new();
    let drawing = minimal_drawing(&mut ids)?;
    drawing.save_as_file("drawing.vsdx")?;
    Ok(())
}
```

The API supports page, master, and shape lookup; stored cell values and formula
text; master and style inheritance; geometry rows; text and formatting marks;
images; connections; and comments. The core package type covers drawing,
stencil, and template packages, including their macro-enabled forms. Add
`ooxmlsdk` at the same version when working directly with those package and
schema types.

This is a stored-value editing API. Formula evaluation, page layout, PDF
rendering, and legacy binary `.vsd` parsing are outside its current scope.
Package and XML tests cover the supported operations; the crate does not claim
lossless editing of arbitrary Visio files.

- [API documentation](https://docs.rs/ooxmlsdk-visio)
- [Package model](https://github.com/KaiserY/ooxmlsdk/blob/main/docs/specs/visio_parts.md)
- [ShapeSheet lookup](https://github.com/KaiserY/ooxmlsdk/blob/main/docs/specs/visio_shapesheet.md)
- [Stored-value authoring](https://github.com/KaiserY/ooxmlsdk/blob/main/docs/specs/visio_authoring.md)

Licensed under MIT OR Apache-2.0.
