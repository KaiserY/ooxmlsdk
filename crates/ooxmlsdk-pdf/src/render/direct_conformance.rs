use std::io::Write;

use flate2::Compression;
use flate2::write::ZlibEncoder;
use pdf_writer::types::OutputIntentSubtype;
use pdf_writer::writers::{Catalog, OutputIntent};
use pdf_writer::{Filter, Finish, Name, Pdf, Ref, TextStr};
use xmp_writer::{RenditionClass, ResourceEventAction, XmpWriter};

use super::direct::RefAllocator;
use crate::error::{PdfError, Result};
use crate::options::{PdfOptions, PdfStandard};

const SRGB_V4_PROFILE: &[u8] = include_bytes!("../../assets/icc/sRGB-v4.icc");

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum PdfA3Level {
  A,
  B,
  U,
}

impl PdfA3Level {
  const fn xmp_value(self) -> &'static str {
    match self {
      Self::A => "A",
      Self::B => "B",
      Self::U => "U",
    }
  }
}

/// The conformance contract implemented by the direct object writer.
///
/// Keep this narrower than option validation: emitting a standards identifier
/// without every object-level invariant would create a false conformance claim.
/// PDF/A-1, PDF/A-2, and PDF/A-4 therefore remain typed unsupported profiles
/// until their distinct restrictions have their own checked lowering.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(super) struct DirectConformance {
  pdf_a3: Option<PdfA3Level>,
  pdf_ua1: bool,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(super) struct ConformanceObjects {
  output_intent_id: Option<Ref>,
  rgb_profile_id: Option<Ref>,
}

impl DirectConformance {
  pub(super) fn from_options(options: &PdfOptions) -> Result<Self> {
    let mut pdf_a3 = None;
    let mut pdf_ua1 = options.general.pdf_ua_compliance;
    for &standard in &options.standards {
      match standard {
        PdfStandard::PdfA3a => pdf_a3 = Some(PdfA3Level::A),
        PdfStandard::PdfA3b => pdf_a3 = Some(PdfA3Level::B),
        PdfStandard::PdfA3u => pdf_a3 = Some(PdfA3Level::U),
        PdfStandard::PdfUa1 => pdf_ua1 = true,
        standard if standard.is_archival() => {
          return Err(PdfError::DirectWriterUnsupported {
            feature: "PDF/A profiles outside the implemented PDF/A-3 family",
          });
        }
        _ => {}
      }
    }

    let conformance = Self { pdf_a3, pdf_ua1 };
    conformance.validate(options)?;
    Ok(conformance)
  }

  fn validate(self, options: &PdfOptions) -> Result<()> {
    if self.pdf_a3.is_some() && options.metadata.creation_date.is_none() {
      return Err(PdfError::Options(
        "PDF/A-3 export requires a deterministic metadata creation date".to_string(),
      ));
    }
    if self.pdf_ua1 && options.canonical_document_language().is_none() {
      return Err(PdfError::Options(
        "PDF/UA-1 export requires a non-empty document or UI language".to_string(),
      ));
    }
    if self.pdf_a3.is_some()
      && let Some(attachment) = options
        .attachments
        .iter()
        .find(|attachment| attachment.modification_date.is_none())
    {
      return Err(PdfError::Options(format!(
        "conforming embedded file '{}' requires a modification date",
        attachment.path
      )));
    }
    Ok(())
  }

  pub(super) fn allocate(self, refs: &mut RefAllocator) -> Result<ConformanceObjects> {
    if self.pdf_a3.is_none() {
      return Ok(ConformanceObjects::default());
    }
    Ok(ConformanceObjects {
      output_intent_id: Some(refs.alloc()?),
      rgb_profile_id: Some(refs.alloc()?),
    })
  }

  pub(super) const fn requests_pdf_ua(self) -> bool {
    self.pdf_ua1
  }

  pub(super) const fn requires_structure_tab_order(self) -> bool {
    self.pdf_ua1 || matches!(self.pdf_a3, Some(PdfA3Level::A))
  }

  pub(super) const fn requires_codepoint_mappings(self) -> bool {
    self.pdf_ua1 || matches!(self.pdf_a3, Some(PdfA3Level::A | PdfA3Level::U))
  }

  pub(super) const fn forbids_private_use_mappings(self) -> bool {
    // PDF/A-3a requires portable Unicode semantics for tagged text.  Keep
    // this separate from `requires_codepoint_mappings`: PDF/A-3u and PDF/UA-1
    // require mappings too, but Krilla's profile matrix permits PUA mappings
    // for those profiles.  Known legacy symbol selectors are translated to
    // standard Unicode before this policy is evaluated.
    matches!(self.pdf_a3, Some(PdfA3Level::A))
  }

  pub(super) const fn supports_associated_files(self, version: (u8, u8)) -> bool {
    self.pdf_a3.is_some() || version.0 >= 2
  }

  pub(super) fn write_objects(self, pdf: &mut Pdf, objects: ConformanceObjects) -> Result<()> {
    let (Some(output_intent_id), Some(rgb_profile_id)) =
      (objects.output_intent_id, objects.rgb_profile_id)
    else {
      if objects.output_intent_id.is_some() || objects.rgb_profile_id.is_some() {
        return Err(PdfError::Writer(
          "direct conformance output-intent references are incomplete".to_string(),
        ));
      }
      return Ok(());
    };

    let compressed = deflate(SRGB_V4_PROFILE)?;
    let mut profile = pdf.icc_profile(rgb_profile_id, &compressed);
    profile.filter(Filter::FlateDecode);
    profile.n(3).range([0.0, 1.0, 0.0, 1.0, 0.0, 1.0]);
    profile.finish();

    let mut intent = pdf.indirect(output_intent_id).start::<OutputIntent>();
    intent
      .subtype(OutputIntentSubtype::PDFA)
      .dest_output_profile(rgb_profile_id)
      .output_condition_identifier(TextStr("sRGB"))
      .output_condition(TextStr("sRGB"))
      .registry_name(TextStr("http://www.color.org"))
      .info(TextStr("sRGB v4.2"));
    intent.finish();
    Ok(())
  }

  pub(super) fn write_catalog(self, catalog: &mut Catalog<'_>, objects: ConformanceObjects) {
    if let Some(output_intent_id) = objects.output_intent_id {
      // Output intents are indirect dictionaries. The typed helper constructs
      // inline entries, so write the reference array through the base API.
      catalog
        .insert(Name(b"OutputIntents"))
        .array()
        .item(output_intent_id)
        .finish();
    }
  }

  pub(super) fn write_xmp(
    self,
    xmp: &mut XmpWriter<'_>,
    options: &PdfOptions,
    instance_id: &str,
    creation_date: Option<xmp_writer::DateTime>,
  ) -> Result<()> {
    if self.pdf_a3.is_some() {
      let mut schemas = xmp.extension_schemas();
      schemas
        .xmp_media_management()
        .properties()
        .describe_instance_id();
      schemas.pdf().properties().describe_all();
      if self.pdf_ua1 {
        schemas.pdfua_id().properties().describe_part();
      }
    }

    if let Some(level) = self.pdf_a3 {
      xmp
        .pdfa_part(3)
        .pdfa_conformance(level.xmp_value())
        .rendition_class(RenditionClass::Proof);

      let date = creation_date.ok_or_else(|| {
        PdfError::Options(
          "PDF/A-3 export requires a deterministic metadata creation date".to_string(),
        )
      })?;
      let source_id = format!("{instance_id}_source");
      let mut history = xmp.history();
      {
        let mut saved = history.add_event();
        saved
          .action(ResourceEventAction::Saved)
          .when(date)
          .instance_id(&source_id);
        saved.finish();
      }
      {
        let mut converted = history.add_event();
        converted
          .action(ResourceEventAction::Converted)
          .when(date)
          .instance_id(instance_id);
        if let Some(creator) = options
          .metadata
          .creator
          .as_deref()
          .filter(|creator| !creator.is_empty())
        {
          converted.software_agent(creator);
        }
        converted.finish();
      }
    }
    if self.pdf_ua1 {
      xmp.pdfua_part(1);
    }
    Ok(())
  }
}

fn deflate(data: &[u8]) -> Result<Vec<u8>> {
  let mut encoder = ZlibEncoder::new(Vec::new(), Compression::new(6));
  encoder
    .write_all(data)
    .map_err(|error| PdfError::Writer(format!("failed to compress sRGB ICC profile: {error}")))?;
  encoder
    .finish()
    .map_err(|error| PdfError::Writer(format!("failed to finish sRGB ICC profile: {error}")))
}
