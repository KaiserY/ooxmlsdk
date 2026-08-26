use crate::error::{PdfError, Result};
use crate::options::{PdfOptions, PdfStandard};

#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
enum PdfVersion {
  Pdf14,
  Pdf15,
  Pdf16,
  #[default]
  Pdf17,
  Pdf20,
}

impl PdfVersion {
  const fn label(self) -> &'static str {
    match self {
      Self::Pdf14 => "PDF 1.4",
      Self::Pdf15 => "PDF 1.5",
      Self::Pdf16 => "PDF 1.6",
      Self::Pdf17 => "PDF 1.7",
      Self::Pdf20 => "PDF 2.0",
    }
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct PdfConfiguration {
  version: PdfVersion,
  archival: Option<PdfStandard>,
  accessibility: bool,
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

fn pdf_configuration(options: &PdfOptions) -> Result<PdfConfiguration> {
  let mut explicit_version = None;
  let mut archival = None;
  let mut accessibility = options.general.pdf_ua_compliance;

  for &standard in &options.standards {
    match standard {
      PdfStandard::Pdf14 => set_version(&mut explicit_version, PdfVersion::Pdf14)?,
      PdfStandard::Pdf15 => set_version(&mut explicit_version, PdfVersion::Pdf15)?,
      PdfStandard::Pdf16 => set_version(&mut explicit_version, PdfVersion::Pdf16)?,
      PdfStandard::Pdf17 => set_version(&mut explicit_version, PdfVersion::Pdf17)?,
      PdfStandard::Pdf20 => set_version(&mut explicit_version, PdfVersion::Pdf20)?,
      PdfStandard::PdfUa1 => accessibility = true,
      standard => set_archival(&mut archival, standard)?,
    }
  }

  // The former backend imposed this product policy on top of the standards'
  // nominal version overlap. Keep it stable while the direct conformance
  // implementation is completed.
  if accessibility && matches!(archival, Some(PdfStandard::PdfA1a | PdfStandard::PdfA1b)) {
    return Err(PdfError::Options(
      "PDF/A-1 cannot be combined with PDF/UA in the supported export policy".to_string(),
    ));
  }

  let (mut minimum, mut maximum) = archival
    .map(archival_version_range)
    .unwrap_or((PdfVersion::Pdf14, PdfVersion::Pdf20));
  if accessibility {
    minimum = minimum.max(PdfVersion::Pdf14);
    maximum = maximum.min(PdfVersion::Pdf17);
  }
  let validators = validator_names(archival, accessibility);
  if minimum > maximum {
    return Err(PdfError::Options(format!(
      "the requested validators have no compatible PDF version: {validators}"
    )));
  }

  let version = explicit_version.unwrap_or_else(|| {
    if archival.is_some() || accessibility {
      maximum
    } else {
      PdfVersion::default()
    }
  });
  if version < minimum || version > maximum {
    return Err(PdfError::Options(format!(
      "{} is not compatible with {validators}",
      version.label()
    )));
  }

  Ok(PdfConfiguration {
    version,
    archival,
    accessibility,
  })
}

fn set_version(slot: &mut Option<PdfVersion>, version: PdfVersion) -> Result<()> {
  if let Some(previous) = slot
    && *previous != version
  {
    return Err(PdfError::Options(format!(
      "PDF cannot target both {} and {}",
      previous.label(),
      version.label()
    )));
  }
  *slot = Some(version);
  Ok(())
}

fn set_archival(slot: &mut Option<PdfStandard>, archival: PdfStandard) -> Result<()> {
  debug_assert!(archival.is_archival());
  if let Some(previous) = slot
    && *previous != archival
  {
    return Err(PdfError::Options(format!(
      "PDF cannot conform to both {} and {}",
      standard_label(*previous),
      standard_label(archival)
    )));
  }
  *slot = Some(archival);
  Ok(())
}

fn archival_version_range(standard: PdfStandard) -> (PdfVersion, PdfVersion) {
  match standard {
    PdfStandard::PdfA1a | PdfStandard::PdfA1b => (PdfVersion::Pdf14, PdfVersion::Pdf14),
    PdfStandard::PdfA2a
    | PdfStandard::PdfA2b
    | PdfStandard::PdfA2u
    | PdfStandard::PdfA3a
    | PdfStandard::PdfA3b
    | PdfStandard::PdfA3u => (PdfVersion::Pdf14, PdfVersion::Pdf17),
    PdfStandard::PdfA4 | PdfStandard::PdfA4f | PdfStandard::PdfA4e => {
      (PdfVersion::Pdf20, PdfVersion::Pdf20)
    }
    _ => unreachable!("non-archival standard"),
  }
}

fn validator_names(archival: Option<PdfStandard>, accessibility: bool) -> String {
  let mut names = Vec::with_capacity(2);
  if let Some(archival) = archival {
    names.push(standard_label(archival));
  }
  if accessibility {
    names.push("PDF/UA-1");
  }
  names.join(" + ")
}

fn standard_label(standard: PdfStandard) -> &'static str {
  match standard {
    PdfStandard::PdfA1a => "PDF/A-1a",
    PdfStandard::PdfA1b => "PDF/A-1b",
    PdfStandard::PdfA2a => "PDF/A-2a",
    PdfStandard::PdfA2b => "PDF/A-2b",
    PdfStandard::PdfA2u => "PDF/A-2u",
    PdfStandard::PdfA3a => "PDF/A-3a",
    PdfStandard::PdfA3b => "PDF/A-3b",
    PdfStandard::PdfA3u => "PDF/A-3u",
    PdfStandard::PdfA4 => "PDF/A-4",
    PdfStandard::PdfA4f => "PDF/A-4f",
    PdfStandard::PdfA4e => "PDF/A-4e",
    PdfStandard::PdfUa1 => "PDF/UA-1",
    PdfStandard::Pdf14 => "PDF 1.4",
    PdfStandard::Pdf15 => "PDF 1.5",
    PdfStandard::Pdf16 => "PDF 1.6",
    PdfStandard::Pdf17 => "PDF 1.7",
    PdfStandard::Pdf20 => "PDF 2.0",
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn default_configuration_targets_pdf_17_without_validators() {
    let configuration = pdf_configuration(&PdfOptions::default()).unwrap();
    assert_eq!(configuration.version, PdfVersion::Pdf17);
    assert_eq!(configuration.archival, None);
    assert!(!configuration.accessibility);
  }

  #[test]
  fn pdf_a_1a_uses_its_recommended_pdf_version() {
    let options = PdfOptions {
      standards: vec![PdfStandard::PdfA1a],
      ..PdfOptions::default()
    };
    let configuration = pdf_configuration(&options).unwrap();
    assert_eq!(configuration.version, PdfVersion::Pdf14);
    assert_eq!(configuration.archival, Some(PdfStandard::PdfA1a));
  }

  #[test]
  fn incompatible_explicit_version_and_validator_are_rejected() {
    let options = PdfOptions {
      standards: vec![PdfStandard::Pdf20, PdfStandard::PdfA1b],
      ..PdfOptions::default()
    };
    assert!(matches!(
      pdf_configuration(&options),
      Err(PdfError::Options(message)) if message == "PDF 2.0 is not compatible with PDF/A-1b"
    ));
  }

  #[test]
  fn compatible_pdf_a_plus_pdf_ua_is_preserved() {
    let options = PdfOptions {
      general: crate::options::PdfGeneralOptions {
        pdf_ua_compliance: true,
        ..Default::default()
      },
      standards: vec![PdfStandard::PdfA2a],
      ..PdfOptions::default()
    };
    let configuration = pdf_configuration(&options).unwrap();
    assert_eq!(configuration.version, PdfVersion::Pdf17);
    assert_eq!(configuration.archival, Some(PdfStandard::PdfA2a));
    assert!(configuration.accessibility);
  }

  #[test]
  fn pdf_a_1_plus_pdf_ua_is_rejected_without_a_common_product_policy() {
    let options = PdfOptions {
      general: crate::options::PdfGeneralOptions {
        pdf_ua_compliance: true,
        ..Default::default()
      },
      standards: vec![PdfStandard::PdfA1a],
      ..PdfOptions::default()
    };
    assert!(matches!(
      pdf_configuration(&options),
      Err(PdfError::Options(message)) if message.contains("PDF/A-1 cannot be combined")
    ));
  }

  #[test]
  fn pdf_a_4_and_pdf_ua_have_no_version_overlap() {
    let options = PdfOptions {
      general: crate::options::PdfGeneralOptions {
        pdf_ua_compliance: true,
        ..Default::default()
      },
      standards: vec![PdfStandard::PdfA4],
      ..PdfOptions::default()
    };
    assert!(matches!(
      pdf_configuration(&options),
      Err(PdfError::Options(message)) if message.contains("no compatible PDF version")
    ));
  }
}
