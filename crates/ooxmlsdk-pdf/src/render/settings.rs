use krilla::SerializeSettings;
use krilla::configure::{Configuration, PdfVersion, Validator};

use crate::error::{PdfError, Result};
use crate::options::{PdfOptions, PdfStandard};

pub(crate) fn serialize_settings(options: &PdfOptions) -> Result<SerializeSettings> {
  Ok(SerializeSettings {
    compress_content_streams: options.compress_content_streams,
    no_device_cs: true,
    ascii_compatible: false,
    xmp_metadata: true,
    cmyk_profile: None,
    configuration: pdf_configuration(options)?,
    enable_tagging: options.general.tagged_pdf || options.general.pdf_ua_compliance,
    render_svg_glyph_fn: krilla_svg::render_svg_glyph,
  })
}

fn pdf_configuration(options: &PdfOptions) -> Result<Configuration> {
  let mut version = None::<PdfVersion>;
  let mut validator = None::<Validator>;

  if options.general.pdf_ua_compliance {
    set_validator(&mut validator, Validator::UA1)?;
  }

  for standard in &options.standards {
    match standard {
      PdfStandard::Pdf14 => set_version(&mut version, PdfVersion::Pdf14)?,
      PdfStandard::Pdf15 => set_version(&mut version, PdfVersion::Pdf15)?,
      PdfStandard::Pdf16 => set_version(&mut version, PdfVersion::Pdf16)?,
      PdfStandard::Pdf17 => set_version(&mut version, PdfVersion::Pdf17)?,
      PdfStandard::Pdf20 => set_version(&mut version, PdfVersion::Pdf20)?,
      PdfStandard::PdfA1a => set_validator(&mut validator, Validator::A1_A)?,
      PdfStandard::PdfA1b => set_validator(&mut validator, Validator::A1_B)?,
      PdfStandard::PdfA2a => set_validator(&mut validator, Validator::A2_A)?,
      PdfStandard::PdfA2b => set_validator(&mut validator, Validator::A2_B)?,
      PdfStandard::PdfA2u => set_validator(&mut validator, Validator::A2_U)?,
      PdfStandard::PdfA3a => set_validator(&mut validator, Validator::A3_A)?,
      PdfStandard::PdfA3b => set_validator(&mut validator, Validator::A3_B)?,
      PdfStandard::PdfA3u => set_validator(&mut validator, Validator::A3_U)?,
      PdfStandard::PdfA4 => set_validator(&mut validator, Validator::A4)?,
      PdfStandard::PdfA4f => set_validator(&mut validator, Validator::A4F)?,
      PdfStandard::PdfA4e => set_validator(&mut validator, Validator::A4E)?,
      PdfStandard::PdfUa1 => set_validator(&mut validator, Validator::UA1)?,
    }
  }

  let validator = validator.unwrap_or(Validator::None);
  let version = version.unwrap_or_else(|| validator.recommended_version());
  Configuration::new_with(validator, version).ok_or_else(|| {
    PdfError::Options(format!(
      "{} is not compatible with {}",
      version.as_str(),
      validator.as_str()
    ))
  })
}

fn set_version(slot: &mut Option<PdfVersion>, version: PdfVersion) -> Result<()> {
  if let Some(previous) = slot
    && *previous != version
  {
    return Err(PdfError::Options(format!(
      "PDF cannot target both {} and {}",
      previous.as_str(),
      version.as_str()
    )));
  }
  *slot = Some(version);
  Ok(())
}

fn set_validator(slot: &mut Option<Validator>, validator: Validator) -> Result<()> {
  if let Some(previous) = slot
    && *previous != validator
  {
    return Err(PdfError::Options(format!(
      "PDF cannot conform to both {} and {}",
      previous.as_str(),
      validator.as_str()
    )));
  }
  *slot = Some(validator);
  Ok(())
}
