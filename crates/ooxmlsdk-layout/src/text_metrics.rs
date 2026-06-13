use crate::compat::TextStyle;

pub(crate) fn measure_text(text: &str, style: &TextStyle) -> f32 {
  if text.is_empty() {
    return 0.0;
  }

  crate::fonts::measure_text_width(text, style).unwrap_or(0.0)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn shaped_measurement_handles_ligatures_and_cjk() {
    let style = TextStyle::default();

    assert!(measure_text("office", &style) > 0.0);
    assert!(measure_text("商务文档", &style) > measure_text("abc", &style));
  }
}
