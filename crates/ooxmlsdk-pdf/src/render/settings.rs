use krilla::SerializeSettings;
use krilla::configure::{
  Accessibility, Archival, Configuration, ConfigurationBuilder, PdfVersion, Validator,
};

use crate::error::{PdfError, Result};
use crate::options::{PdfOptions, PdfStandard};

pub(crate) fn serialize_settings(options: &PdfOptions) -> Result<SerializeSettings> {
  Ok(SerializeSettings {
    pretty: false,
    compress_content_streams: options.compress_content_streams,
    // Word's ordinary fixed-format PDF output uses DeviceRGB for unprofiled
    // RGB images. Keep device-independent color for explicitly requested
    // archival profiles, where the conformance contract is stronger than
    // Office's default PDF behavior.
    no_device_cs: requests_archival_standard(options),
    ascii_compatible: false,
    xmp_metadata: true,
    cmyk_profile: None,
    configuration: pdf_configuration(options)?,
    enable_tagging: options.general.tagged_pdf || options.general.pdf_ua_compliance,
    render_svg_glyph_fn: krilla_svg::render_svg_glyph,
  })
}

fn requests_archival_standard(options: &PdfOptions) -> bool {
  options.standards.iter().any(|standard| {
    matches!(
      standard,
      PdfStandard::PdfA1a
        | PdfStandard::PdfA1b
        | PdfStandard::PdfA2a
        | PdfStandard::PdfA2b
        | PdfStandard::PdfA2u
        | PdfStandard::PdfA3a
        | PdfStandard::PdfA3b
        | PdfStandard::PdfA3u
        | PdfStandard::PdfA4
        | PdfStandard::PdfA4f
        | PdfStandard::PdfA4e
    )
  })
}

fn pdf_configuration(options: &PdfOptions) -> Result<Configuration> {
  let mut version = None::<PdfVersion>;
  let mut validator = None::<Validator>;

  if options.general.pdf_ua_compliance {
    set_validator(&mut validator, Accessibility::UA1.into())?;
  }

  for standard in &options.standards {
    match standard {
      PdfStandard::Pdf14 => set_version(&mut version, PdfVersion::Pdf14)?,
      PdfStandard::Pdf15 => set_version(&mut version, PdfVersion::Pdf15)?,
      PdfStandard::Pdf16 => set_version(&mut version, PdfVersion::Pdf16)?,
      PdfStandard::Pdf17 => set_version(&mut version, PdfVersion::Pdf17)?,
      PdfStandard::Pdf20 => set_version(&mut version, PdfVersion::Pdf20)?,
      PdfStandard::PdfA1a => set_validator(&mut validator, Archival::A1_A.into())?,
      PdfStandard::PdfA1b => set_validator(&mut validator, Archival::A1_B.into())?,
      PdfStandard::PdfA2a => set_validator(&mut validator, Archival::A2_A.into())?,
      PdfStandard::PdfA2b => set_validator(&mut validator, Archival::A2_B.into())?,
      PdfStandard::PdfA2u => set_validator(&mut validator, Archival::A2_U.into())?,
      PdfStandard::PdfA3a => set_validator(&mut validator, Archival::A3_A.into())?,
      PdfStandard::PdfA3b => set_validator(&mut validator, Archival::A3_B.into())?,
      PdfStandard::PdfA3u => set_validator(&mut validator, Archival::A3_U.into())?,
      PdfStandard::PdfA4 => set_validator(&mut validator, Archival::A4.into())?,
      PdfStandard::PdfA4f => set_validator(&mut validator, Archival::A4F.into())?,
      PdfStandard::PdfA4e => set_validator(&mut validator, Archival::A4E.into())?,
      PdfStandard::PdfUa1 => set_validator(&mut validator, Accessibility::UA1.into())?,
    }
  }

  let version = version.unwrap_or_else(|| validator.map_or(PdfVersion::Pdf17, Validator::max));
  let mut builder = ConfigurationBuilder::new().with_version(version);
  if let Some(validator) = validator {
    builder = builder.set_validator(validator);
  }
  builder.finish().map_err(|_| {
    PdfError::Options(format!(
      "{} is not compatible with {}",
      version.as_str(),
      validator.map_or("the requested PDF configuration", Validator::as_str)
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn default_configuration_targets_pdf_17_without_validators() {
    let configuration = pdf_configuration(&PdfOptions::default()).unwrap();

    assert_eq!(configuration.version(), PdfVersion::Pdf17);
    assert!(configuration.validators().is_empty());
  }

  #[test]
  fn ordinary_pdf_uses_device_color_but_pdf_a_does_not() {
    let ordinary = serialize_settings(&PdfOptions::default()).unwrap();
    let archival_options = PdfOptions {
      standards: vec![PdfStandard::PdfA2b],
      ..PdfOptions::default()
    };
    let archival = serialize_settings(&archival_options).unwrap();

    assert!(!ordinary.no_device_cs);
    assert!(archival.no_device_cs);
  }

  #[test]
  fn pdf_a_1a_uses_its_recommended_pdf_version() {
    let mut options = PdfOptions::default();
    options.standards.push(PdfStandard::PdfA1a);

    let configuration = pdf_configuration(&options).unwrap();

    assert_eq!(configuration.version(), PdfVersion::Pdf14);
    assert_eq!(configuration.validators().archival(), Some(Archival::A1_A));
  }

  #[test]
  fn incompatible_explicit_version_and_validator_are_rejected() {
    let options = PdfOptions {
      standards: vec![PdfStandard::Pdf20, PdfStandard::PdfA1b],
      ..PdfOptions::default()
    };

    assert!(matches!(
      pdf_configuration(&options),
      Err(PdfError::Options(message))
        if message == "PDF 2.0 is not compatible with PDF/A-1b"
    ));
  }

  #[test]
  fn existing_single_validator_policy_rejects_pdf_a_plus_pdf_ua() {
    let mut options = PdfOptions::default();
    options.general.pdf_ua_compliance = true;
    options.standards.push(PdfStandard::PdfA2a);

    assert!(matches!(
      pdf_configuration(&options),
      Err(PdfError::Options(_))
    ));
  }
}
