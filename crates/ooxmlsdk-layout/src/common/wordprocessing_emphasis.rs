//! Word emphasis metadata after the run-property cascade.
//!
//! ECMA-376 §17.3.2.12 owns presence/inheritance; MS-DOC sprmCKcd owns
//! the East Asian language selection. Native57 controls retain modern
//! Word's comma behavior where it differs from the old published table.

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum EmphasisMark {
  Dot,
  Comma,
  Circle,
  UnderDot,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum EmphasisLanguage {
  Japanese,
  Korean,
  SimplifiedChinese,
  TraditionalChinese,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct WordprocessingEmphasis {
  pub mark: EmphasisMark,
  pub language: EmphasisLanguage,
}

impl WordprocessingEmphasis {
  pub fn new(mark: EmphasisMark, east_asian_language: Option<&str>) -> Self {
    let language = match east_asian_language {
      Some(value) if value.eq_ignore_ascii_case("ko-KR") => EmphasisLanguage::Korean,
      Some(value) if value.eq_ignore_ascii_case("zh-CN") => EmphasisLanguage::SimplifiedChinese,
      Some(value) if value.eq_ignore_ascii_case("zh-TW") => EmphasisLanguage::TraditionalChinese,
      // The source East Asian language is independent of UI/document locale.
      // The published default is Japanese for unsupported source languages.
      _ => EmphasisLanguage::Japanese,
    };
    Self { mark, language }
  }

  /// Line reserve ownership differs from painted position in zh-CN:
  /// Native comma/circle remain painted above while their extra line extent
  /// is below, established by the five paired zh-CN controls.
  pub fn reserves_above(self) -> bool {
    self.language != EmphasisLanguage::SimplifiedChinese && self.mark != EmphasisMark::UnderDot
  }

  /// The painted glyph can use a different language strategy from the run's
  /// line-reserve language. Modern Word always treats a comma as Japanese;
  /// underDot uses Simplified Chinese, and the ordinary ring uses Korean.
  pub fn paint_language(self) -> EmphasisLanguage {
    use EmphasisLanguage::*;
    use EmphasisMark::*;
    match (self.mark, self.language) {
      (Comma, _) => Japanese,
      (UnderDot, _) => SimplifiedChinese,
      (Circle, TraditionalChinese) => TraditionalChinese,
      (Circle, _) => Korean,
      (Dot, language) => language,
    }
  }

  /// Native17 font/style/script controls distinguish this annotation font
  /// selection from ordinary missing-glyph fallback of the body face.
  /// Keep the mark at the source full font size; it is not CSS half-size text.
  pub fn glyph(self) -> (&'static str, &'static str) {
    use EmphasisLanguage::*;
    use EmphasisMark::*;
    match (self.mark, self.language) {
      (Comma, _) => ("、", "MS Mincho"),
      (UnderDot, _) | (Dot, SimplifiedChinese) => ("．", "SimSun"),
      (Circle, TraditionalChinese) => ("。", "MingLiU"),
      (Circle, _) => ("˚", "Batang"),
      (Dot, Korean) => ("˙", "Batang"),
      (Dot, TraditionalChinese) => ("‧", "MingLiU"),
      (Dot, Japanese) => ("．", "MS Mincho"),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn word_emphasis_native_languages_keep_glyph_and_line_side_independent() {
    for (language, dot, circle, above) in [
      ("ja-JP", ("．", "MS Mincho"), ("˚", "Batang"), true),
      ("ko-KR", ("˙", "Batang"), ("˚", "Batang"), true),
      ("zh-CN", ("．", "SimSun"), ("˚", "Batang"), false),
      ("zh-TW", ("‧", "MingLiU"), ("。", "MingLiU"), true),
    ] {
      for (mark, glyph, upper) in [
        (EmphasisMark::Dot, dot, above),
        (EmphasisMark::Comma, ("、", "MS Mincho"), above),
        (EmphasisMark::Circle, circle, above),
        (EmphasisMark::UnderDot, ("．", "SimSun"), false),
      ] {
        let value = WordprocessingEmphasis::new(mark, Some(language));
        assert_eq!(value.glyph(), glyph);
        assert_eq!(value.reserves_above(), upper);
      }
    }
    assert_eq!(
      WordprocessingEmphasis::new(EmphasisMark::Dot, Some("en-US")),
      WordprocessingEmphasis::new(EmphasisMark::Dot, None)
    );
  }
}
