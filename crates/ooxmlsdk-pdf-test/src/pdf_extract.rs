#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PdfSummary {
  pub page_count: usize,
  pub image_count: usize,
  pub link_annotation_count: usize,
  pub outline_marker_count: usize,
  pub media_box_count: usize,
  pub contains_eof: bool,
}

impl PdfSummary {
  pub fn from_bytes(pdf: &[u8]) -> Self {
    let text = String::from_utf8_lossy(pdf);
    Self {
      page_count: count_pdf_pages(&text),
      image_count: count_pdf_name_value(&text, "/Subtype", "Image"),
      link_annotation_count: count_pdf_name_value(&text, "/Subtype", "Link"),
      outline_marker_count: text.matches("/Outlines").count() + text.matches("/Title").count(),
      media_box_count: text.matches("/MediaBox").count(),
      contains_eof: pdf.strip_suffix_ascii_whitespace().ends_with(b"%%EOF"),
    }
  }
}

fn count_pdf_pages(text: &str) -> usize {
  count_pdf_name_value(text, "/Type", "Page")
}

fn count_pdf_name_value(text: &str, key: &str, value: &str) -> usize {
  text
    .match_indices(key)
    .filter(|(index, _)| has_pdf_name_value(text.as_bytes(), *index + key.len(), value))
    .count()
}

fn has_pdf_name_value(bytes: &[u8], mut index: usize, value: &str) -> bool {
  while matches!(bytes.get(index), Some(b' ' | b'\t' | b'\r' | b'\n')) {
    index += 1;
  }
  if bytes.get(index) != Some(&b'/') {
    return false;
  }
  index += 1;

  let value_bytes = value.as_bytes();
  let Some(end) = index.checked_add(value_bytes.len()) else {
    return false;
  };
  if bytes.get(index..end) != Some(value_bytes) {
    return false;
  }

  !matches!(
    bytes.get(end),
    Some(b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'_' | b'-')
  )
}

trait PdfBytesExt {
  fn strip_suffix_ascii_whitespace(&self) -> &[u8];
}

impl PdfBytesExt for [u8] {
  fn strip_suffix_ascii_whitespace(&self) -> &[u8] {
    let mut end = self.len();
    while end > 0 && self[end - 1].is_ascii_whitespace() {
      end -= 1;
    }
    &self[..end]
  }
}
