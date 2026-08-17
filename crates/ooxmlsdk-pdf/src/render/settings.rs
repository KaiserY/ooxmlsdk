use krilla::SerializeSettings;
use krilla::configure::{
  Accessibility, Archival, Configuration, ConfigurationBuilder, ConfigurationError, PdfVersion,
  Validators,
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
    enable_tagging: requests_tagging(options),
    render_svg_glyph_fn: krilla_svg::render_svg_glyph,
  })
}

pub(crate) fn validate_options(options: &PdfOptions) -> Result<()> {
  pdf_configuration(options).map(|_| ())
}

pub(crate) fn requests_tagging(options: &PdfOptions) -> bool {
  options.general.tagged_pdf
    || options.general.pdf_ua_compliance
    || options.standards.iter().any(|standard| {
      matches!(
        standard,
        PdfStandard::PdfA1a | PdfStandard::PdfA2a | PdfStandard::PdfA3a | PdfStandard::PdfUa1
      )
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
  let mut archival = None::<Archival>;
  let mut accessibility = options
    .general
    .pdf_ua_compliance
    .then_some(Accessibility::UA1);

  for standard in &options.standards {
    match standard {
      PdfStandard::Pdf14 => set_version(&mut version, PdfVersion::Pdf14)?,
      PdfStandard::Pdf15 => set_version(&mut version, PdfVersion::Pdf15)?,
      PdfStandard::Pdf16 => set_version(&mut version, PdfVersion::Pdf16)?,
      PdfStandard::Pdf17 => set_version(&mut version, PdfVersion::Pdf17)?,
      PdfStandard::Pdf20 => set_version(&mut version, PdfVersion::Pdf20)?,
      PdfStandard::PdfA1a => set_archival(&mut archival, Archival::A1_A)?,
      PdfStandard::PdfA1b => set_archival(&mut archival, Archival::A1_B)?,
      PdfStandard::PdfA2a => set_archival(&mut archival, Archival::A2_A)?,
      PdfStandard::PdfA2b => set_archival(&mut archival, Archival::A2_B)?,
      PdfStandard::PdfA2u => set_archival(&mut archival, Archival::A2_U)?,
      PdfStandard::PdfA3a => set_archival(&mut archival, Archival::A3_A)?,
      PdfStandard::PdfA3b => set_archival(&mut archival, Archival::A3_B)?,
      PdfStandard::PdfA3u => set_archival(&mut archival, Archival::A3_U)?,
      PdfStandard::PdfA4 => set_archival(&mut archival, Archival::A4)?,
      PdfStandard::PdfA4f => set_archival(&mut archival, Archival::A4F)?,
      PdfStandard::PdfA4e => set_archival(&mut archival, Archival::A4E)?,
      PdfStandard::PdfUa1 => set_accessibility(&mut accessibility, Accessibility::UA1)?,
    }
  }

  if matches!(archival, Some(Archival::A1_A | Archival::A1_B)) && accessibility.is_some() {
    return Err(PdfError::Options(
      "PDF/A-1 cannot be combined with PDF/UA in the supported export policy".to_string(),
    ));
  }

  let mut builder = ConfigurationBuilder::new();
  if let Some(version) = version {
    builder = builder.with_version(version);
  }
  if let Some(archival) = archival {
    builder = builder.with_archival_validator(archival);
  }
  if let Some(accessibility) = accessibility {
    builder = builder.with_accessibility_validator(accessibility);
  }
  builder.finish().map_err(configuration_error)
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

fn set_archival(slot: &mut Option<Archival>, archival: Archival) -> Result<()> {
  if let Some(previous) = slot
    && *previous != archival
  {
    return Err(PdfError::Options(format!(
      "PDF cannot conform to both {} and {}",
      previous.as_str(),
      archival.as_str()
    )));
  }
  *slot = Some(archival);
  Ok(())
}

fn set_accessibility(slot: &mut Option<Accessibility>, accessibility: Accessibility) -> Result<()> {
  if let Some(previous) = slot
    && *previous != accessibility
  {
    return Err(PdfError::Options(format!(
      "PDF cannot conform to both {} and {}",
      previous.as_str(),
      accessibility.as_str()
    )));
  }
  *slot = Some(accessibility);
  Ok(())
}

fn configuration_error(error: ConfigurationError) -> PdfError {
  match error {
    ConfigurationError::VersionDoesNotMatchValidatorsRange(version, validators) => {
      PdfError::Options(format!(
        "{} is not compatible with {}",
        version.as_str(),
        validator_names(validators)
      ))
    }
    ConfigurationError::NoOverlappingValidatorsRange(validators) => PdfError::Options(format!(
      "the requested validators have no compatible PDF version: {}",
      validator_names(validators)
    )),
  }
}

fn validator_names(validators: Validators) -> String {
  validators
    .into_iter()
    .map(|validator| validator.as_str())
    .collect::<Vec<_>>()
    .join(" + ")
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
  fn compatible_pdf_a_plus_pdf_ua_is_preserved() {
    let mut options = PdfOptions::default();
    options.general.pdf_ua_compliance = true;
    options.standards.push(PdfStandard::PdfA2a);

    let configuration = pdf_configuration(&options).unwrap();
    assert_eq!(configuration.version(), PdfVersion::Pdf17);
    assert_eq!(configuration.validators().archival(), Some(Archival::A2_A));
    assert_eq!(
      configuration.validators().accessibility(),
      Some(Accessibility::UA1)
    );
  }

  #[test]
  fn pdf_a_1_plus_pdf_ua_is_rejected_without_a_common_version() {
    let mut options = PdfOptions::default();
    options.general.pdf_ua_compliance = true;
    options.standards.push(PdfStandard::PdfA1a);

    assert!(matches!(
      pdf_configuration(&options),
      Err(PdfError::Options(message))
        if message.contains("PDF/A-1 cannot be combined")
    ));
  }
}
