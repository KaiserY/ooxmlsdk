use std::io::Write;

use flate2::Compression;
use flate2::write::ZlibEncoder;
use pdf_writer::{Filter, Finish, Pdf, Rect, Ref};

use super::direct::{PageResources, RefAllocator};
use crate::error::{PdfError, Result};

#[derive(Debug)]
struct DirectForm {
  id: Ref,
  width_pt: f32,
  height_pt: f32,
  content: Vec<u8>,
  resources: PageResources,
  compress: bool,
}

/// Owns the form XObjects used as explicit transparency-group boundaries.
///
/// Names are allocated independently from object references so nested groups
/// can be completed in child-first order without making output unstable.
#[derive(Debug, Default)]
pub(super) struct DirectFormSet {
  next_name: usize,
  forms: Vec<DirectForm>,
}

impl DirectFormSet {
  pub(super) fn allocate(&mut self, refs: &mut RefAllocator) -> Result<(Vec<u8>, Ref)> {
    let index = self.next_name;
    self.next_name = self
      .next_name
      .checked_add(1)
      .ok_or_else(|| PdfError::Writer("form XObject name space exhausted".to_string()))?;
    Ok((format!("Fm{index}").into_bytes(), refs.alloc()?))
  }

  pub(super) fn register(
    &mut self,
    id: Ref,
    width_pt: f32,
    height_pt: f32,
    content: Vec<u8>,
    resources: PageResources,
    compress: bool,
  ) {
    self.forms.push(DirectForm {
      id,
      width_pt,
      height_pt,
      content,
      resources,
      compress,
    });
  }

  pub(super) fn write_objects(&self, pdf: &mut Pdf) -> Result<()> {
    for form in &self.forms {
      form.resources.write_objects(pdf);

      let compressed;
      let content = if form.compress {
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder
          .write_all(&form.content)
          .map_err(|error| PdfError::Writer(format!("form compression failed: {error}")))?;
        compressed = encoder
          .finish()
          .map_err(|error| PdfError::Writer(format!("form compression failed: {error}")))?;
        compressed.as_slice()
      } else {
        form.content.as_slice()
      };

      let mut object = pdf.form_xobject(form.id, content);
      if form.compress {
        object.filter(Filter::FlateDecode);
      }
      object.bbox(Rect::new(0.0, 0.0, form.width_pt, form.height_pt));
      {
        let mut resources = object.resources();
        form.resources.write_dictionary(&mut resources);
        resources.finish();
      }
      {
        let mut group = object.group();
        group.transparency().isolated(true);
        group.color_space().device_rgb();
        group.finish();
      }
      object.finish();
    }
    Ok(())
  }
}
