use pdf_writer::{Date, Finish, Name, Pdf, Ref, Str, TextStr};
use uuid::Uuid;
use xmp_writer::{LangId, Timezone, XmpWriter};

use crate::error::{PdfError, Result};
use crate::options::{PdfDateTime, PdfOptions};
use crate::render::direct_conformance::DirectConformance;

const PDF_ID_NAMESPACE_NAME: &[u8] = b"https://github.com/KaiserY/ooxmlsdk#pdf";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct MetadataObjects {
  info_id: Option<Ref>,
  xmp_id: Ref,
}

impl MetadataObjects {
  pub(super) fn allocate(
    options: &PdfOptions,
    version: (u8, u8),
    mut allocate: impl FnMut() -> Result<Ref>,
  ) -> Result<Self> {
    let info_id = has_document_info(options, version)
      .then(&mut allocate)
      .transpose()?;
    Ok(Self {
      info_id,
      xmp_id: allocate()?,
    })
  }

  pub(super) fn xmp_id(self) -> Ref {
    self.xmp_id
  }
}

pub(super) fn write(
  pdf: &mut Pdf,
  objects: MetadataObjects,
  options: &PdfOptions,
  version: (u8, u8),
  page_count: usize,
  conformance: DirectConformance,
) -> Result<()> {
  let title = non_empty(options.metadata.title.as_deref());
  let author = non_empty(options.metadata.author.as_deref());
  let subject = non_empty(options.metadata.subject.as_deref());
  let keywords = joined_keywords(options);
  let creator = non_empty(options.metadata.creator.as_deref());
  let producer = non_empty(options.metadata.producer.as_deref());

  if let Some(info_id) = objects.info_id {
    let mut info = pdf.document_info(info_id);
    if version < (2, 0) {
      if let Some(title) = title {
        info.title(TextStr(title));
      }
      if let Some(author) = author {
        info.author(TextStr(author));
      }
      if let Some(subject) = subject {
        info.subject(TextStr(subject));
      }
      if let Some(keywords) = keywords.as_deref() {
        info.keywords(TextStr(keywords));
      }
      if let Some(creator) = creator {
        info.creator(TextStr(creator));
      }
      if let Some(producer) = producer {
        info.producer(TextStr(producer));
      }
    }
    if let Some(date) = options.metadata.creation_date {
      let date = pdf_date(date);
      info
        .pair(Name(b"CreationDate"), Str(date.as_bytes()))
        .pair(Name(b"ModDate"), Str(date.as_bytes()));
    }
    info.finish();
  }

  // The namespace separates content-derived PDF identities from other UUIDv5
  // users. Hash only the already serialized, deterministic document objects:
  // XMP, catalog, xref, and trailer must not make the identifier recursive.
  let namespace = Uuid::new_v5(&Uuid::NAMESPACE_URL, PDF_ID_NAMESPACE_NAME);
  let identifier = Uuid::new_v5(&namespace, pdf.as_bytes());
  let identifier_text = format!("uuid:{identifier}");
  pdf.set_file_id((
    identifier.as_bytes().to_vec(),
    identifier.as_bytes().to_vec(),
  ));

  let mut xmp = XmpWriter::new();
  if let Some(title) = title {
    xmp.title([(None, title)]);
  }
  if let Some(subject) = subject {
    xmp.description([(None, subject)]);
  }
  if let Some(keywords) = keywords.as_deref() {
    xmp.pdf_keywords(keywords);
  }
  if let Some(author) = author {
    xmp.creator([author]);
  }
  if let Some(creator) = creator {
    xmp.creator_tool(creator);
  }
  if let Some(producer) = producer {
    xmp.producer(producer);
  }
  if let Some(language) = options.canonical_document_language() {
    xmp.language([LangId(&language)]);
  }
  let creation_date = options.metadata.creation_date.map(xmp_date);
  if let Some(date) = creation_date {
    xmp.create_date(date).modify_date(date);
  }

  let page_count = u32::try_from(page_count)
    .map_err(|_| PdfError::Writer("XMP page count exceeds the u32 range".to_string()))?;
  let version = format!("{}.{}", version.0, version.1);
  xmp
    .num_pages(page_count)
    .format("application/pdf")
    .document_id(&identifier_text)
    .instance_id(&identifier_text)
    .pdf_version(&version);

  conformance.write_xmp(&mut xmp, options, &identifier_text, creation_date)?;

  let xmp = xmp.finish(None);
  pdf.metadata(objects.xmp_id, xmp.as_bytes()).finish();
  Ok(())
}

fn has_document_info(options: &PdfOptions, version: (u8, u8)) -> bool {
  options.metadata.creation_date.is_some()
    || (version < (2, 0)
      && (non_empty(options.metadata.title.as_deref()).is_some()
        || non_empty(options.metadata.author.as_deref()).is_some()
        || non_empty(options.metadata.subject.as_deref()).is_some()
        || joined_keywords(options).is_some()
        || non_empty(options.metadata.creator.as_deref()).is_some()
        || non_empty(options.metadata.producer.as_deref()).is_some()))
}

fn non_empty(value: Option<&str>) -> Option<&str> {
  value.filter(|value| !value.is_empty())
}

fn joined_keywords(options: &PdfOptions) -> Option<String> {
  let keywords = options
    .metadata
    .keywords
    .as_deref()?
    .split([',', ';'])
    .map(str::trim)
    .filter(|keyword| !keyword.is_empty())
    .collect::<Vec<_>>();
  (!keywords.is_empty()).then(|| keywords.join(", "))
}

fn pdf_date(value: PdfDateTime) -> String {
  let mut date = format!(
    "D:{:04}{:02}{:02}{:02}{:02}{:02}",
    value.year,
    value.month.unwrap_or(1),
    value.day.unwrap_or(1),
    value.hour.unwrap_or(0),
    value.minute.unwrap_or(0),
    value.second.unwrap_or(0),
  );
  match value.utc_offset_hour {
    Some(hour) => {
      let sign = if hour < 0 { '-' } else { '+' };
      date.push_str(&format!(
        "{sign}{:02}'{:02}'",
        hour.unsigned_abs(),
        value.utc_offset_minute.unwrap_or(0)
      ));
    }
    None => date.push('Z'),
  }
  date
}

pub(super) fn pdf_writer_date(value: PdfDateTime) -> Date {
  Date::new(value.year)
    .month(value.month.unwrap_or(1))
    .day(value.day.unwrap_or(1))
    .hour(value.hour.unwrap_or(0))
    .minute(value.minute.unwrap_or(0))
    .second(value.second.unwrap_or(0))
    .utc_offset_hour(value.utc_offset_hour.unwrap_or(0))
    .utc_offset_minute(value.utc_offset_minute.unwrap_or(0))
}

fn xmp_date(value: PdfDateTime) -> xmp_writer::DateTime {
  let timezone = match value.utc_offset_hour {
    Some(hour) => Timezone::Local {
      hour,
      minute: value.utc_offset_minute.unwrap_or(0) as i8,
    },
    None => Timezone::Utc,
  };
  xmp_writer::DateTime::new(
    value.year,
    value.month.unwrap_or(1),
    value.day.unwrap_or(1),
    value.hour.unwrap_or(0),
    value.minute.unwrap_or(0),
    value.second.unwrap_or(0),
    timezone,
  )
}
