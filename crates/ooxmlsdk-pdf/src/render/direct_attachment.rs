use std::borrow::Cow;
use std::io::Write;

use flate2::Compression;
use flate2::write::ZlibEncoder;
use pdf_writer::types::AssociationKind;
use pdf_writer::writers::{Catalog, Names};
use pdf_writer::{Filter, Finish, Name, Pdf, Ref, Str, TextStr};

use crate::error::{PdfError, Result};
use crate::options::{PdfAttachment, PdfAttachmentAssociation, PdfOptions, validate_attachments};

#[derive(Clone, Copy, Debug)]
struct AttachmentObject<'a> {
  attachment: &'a PdfAttachment,
  file_spec_id: Ref,
  stream_id: Ref,
}

#[derive(Debug, Default)]
pub(super) struct AttachmentObjects<'a> {
  entries: Vec<AttachmentObject<'a>>,
}

impl<'a> AttachmentObjects<'a> {
  pub(super) fn allocate(
    options: &'a PdfOptions,
    mut allocate: impl FnMut() -> Result<Ref>,
  ) -> Result<Self> {
    validate_attachments(&options.attachments)?;

    let mut attachments = options.attachments.iter().collect::<Vec<_>>();
    attachments.sort_by(|left, right| left.path.as_bytes().cmp(right.path.as_bytes()));
    let entries = attachments
      .into_iter()
      .map(|attachment| {
        Ok(AttachmentObject {
          attachment,
          file_spec_id: allocate()?,
          stream_id: allocate()?,
        })
      })
      .collect::<Result<Vec<_>>>()?;
    Ok(Self { entries })
  }

  pub(super) fn is_empty(&self) -> bool {
    self.entries.is_empty()
  }

  pub(super) fn write_objects(
    &self,
    pdf: &mut Pdf,
    version: (u8, u8),
    supports_associated_files: bool,
  ) -> Result<()> {
    for entry in &self.entries {
      let attachment = entry.attachment;
      let size = i32::try_from(attachment.data.len()).map_err(|_| {
        PdfError::Writer(format!(
          "attachment '{}' exceeds the PDF integer size range",
          attachment.path
        ))
      })?;
      let encoded =
        encode_attachment(attachment.data.as_ref(), attachment.compress).map_err(|error| {
          PdfError::Writer(format!(
            "attachment '{}' compression failed: {error}",
            attachment.path
          ))
        })?;

      let mut stream = pdf.embedded_file(entry.stream_id, encoded.bytes.as_ref());
      // `Name` escapes the MIME slash as `#2F`; pass the original media type
      // bytes so the escape marker itself is not escaped a second time.
      stream.subtype(Name(attachment.mime_type.as_bytes()));
      if encoded.compressed {
        stream.filter(Filter::FlateDecode);
      }
      {
        let mut params = stream.params();
        params.size(size);
        if let Some(date) = attachment.modification_date {
          // The Preview-specific trailing-apostrophe fact is pinned only for
          // Info dates. Embedded-file parameters retain the standard form used
          // by the legacy backend until independent evidence says otherwise.
          params.modification_date(super::direct_metadata::pdf_writer_date(date));
        }
        params.finish();
      }
      stream.finish();

      let mut file_spec = pdf.file_spec(entry.file_spec_id);
      file_spec.path(Str(attachment.path.as_bytes()));
      if version >= (1, 7) {
        file_spec
          .unic_file(TextStr(&attachment.path))
          .embedded_file_with_unicode(entry.stream_id);
      } else {
        file_spec.embedded_file(entry.stream_id);
      }
      // The public option and both pre-migration writers require a human-readable
      // description. Keep emitting it for PDF 1.4/1.5 as the established reader
      // compatibility extension even though it became a core key in PDF 1.6.
      file_spec.description(TextStr(&attachment.description));
      if supports_associated_files {
        file_spec.association_kind(association_kind(attachment.association));
      }
      file_spec.finish();
    }
    Ok(())
  }

  pub(super) fn write_name_tree(&self, names: &mut Names<'_>) {
    let mut tree = names.embedded_files();
    let mut entries = tree.names();
    for entry in &self.entries {
      entries.insert(Str(entry.attachment.path.as_bytes()), entry.file_spec_id);
    }
    entries.finish();
    tree.finish();
  }

  pub(super) fn write_catalog_associations(
    &self,
    catalog: &mut Catalog<'_>,
    supports_associated_files: bool,
  ) {
    if !supports_associated_files || self.entries.is_empty() {
      return;
    }
    // Associated-file arrays contain indirect file-specification references.
    // The high-level pdf-writer helper is typed for inline FileSpec writers,
    // so use the underlying array API to preserve the required object graph.
    let mut associated = catalog.insert(Name(b"AF")).array();
    associated.items(self.entries.iter().map(|entry| entry.file_spec_id));
    associated.finish();
  }
}

fn association_kind(association: PdfAttachmentAssociation) -> AssociationKind {
  match association {
    PdfAttachmentAssociation::Source => AssociationKind::Source,
    PdfAttachmentAssociation::Data => AssociationKind::Data,
    PdfAttachmentAssociation::Alternative => AssociationKind::Alternative,
    PdfAttachmentAssociation::Supplement => AssociationKind::Supplement,
    PdfAttachmentAssociation::Unspecified => AssociationKind::Unspecified,
  }
}

struct EncodedAttachment<'a> {
  bytes: Cow<'a, [u8]>,
  compressed: bool,
}

fn encode_attachment(
  data: &[u8],
  requested: Option<bool>,
) -> std::io::Result<EncodedAttachment<'_>> {
  if requested == Some(false) || requested.is_none() && data.is_empty() {
    return Ok(EncodedAttachment {
      bytes: Cow::Borrowed(data),
      compressed: false,
    });
  }

  let compressed = deflate(data)?;
  if requested.is_none() && (compressed.len() as u128) * 100 > (data.len() as u128) * 75 {
    return Ok(EncodedAttachment {
      bytes: Cow::Borrowed(data),
      compressed: false,
    });
  }
  Ok(EncodedAttachment {
    bytes: Cow::Owned(compressed),
    compressed: true,
  })
}

fn deflate(data: &[u8]) -> std::io::Result<Vec<u8>> {
  let mut encoder = ZlibEncoder::new(Vec::new(), Compression::new(6));
  encoder.write_all(data)?;
  encoder.finish()
}

#[cfg(test)]
mod tests {
  use std::io::Read;

  use flate2::read::ZlibDecoder;

  use super::*;

  fn decoded(encoded: &EncodedAttachment<'_>) -> Vec<u8> {
    if !encoded.compressed {
      return encoded.bytes.to_vec();
    }
    let mut output = Vec::new();
    ZlibDecoder::new(encoded.bytes.as_ref())
      .read_to_end(&mut output)
      .unwrap();
    output
  }

  #[test]
  fn attachment_compression_preserves_explicit_and_auto_boundaries() {
    let compressible = vec![b'a'; 1024];
    let auto_compressed = encode_attachment(&compressible, None).unwrap();
    assert!(auto_compressed.compressed);
    assert_eq!(decoded(&auto_compressed), compressible);

    let short_incompressible = b"0123456789abcdef";
    let auto_raw = encode_attachment(short_incompressible, None).unwrap();
    assert!(!auto_raw.compressed);
    assert_eq!(decoded(&auto_raw), short_incompressible);

    let forced_raw = encode_attachment(&compressible, Some(false)).unwrap();
    assert!(!forced_raw.compressed);
    assert_eq!(decoded(&forced_raw), compressible);

    let forced_empty = encode_attachment(&[], Some(true)).unwrap();
    assert!(forced_empty.compressed);
    assert!(decoded(&forced_empty).is_empty());
    assert!(!encode_attachment(&[], None).unwrap().compressed);
  }
}
