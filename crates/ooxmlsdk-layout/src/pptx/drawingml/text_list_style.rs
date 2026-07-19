use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;
use ooxmlsdk::schemas::schemas_openxmlformats_org_presentationml_2006_main as p;

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct TextListStyle {
  pub(crate) default_paragraph_properties: Option<Box<a::DefaultParagraphProperties>>,
  pub(crate) levels: Vec<TextListLevelStyle>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct TextListLevelStyle {
  pub(crate) level: u8,
  pub(crate) paragraph_properties: TextListLevelParagraphProperties,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum TextListParagraphStyleRef<'a> {
  Default(&'a a::DefaultParagraphProperties),
  Level(&'a TextListLevelStyle),
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum TextListParagraphStyle {
  Default(Box<a::DefaultParagraphProperties>),
  Level(TextListLevelStyle),
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum TextListLevelParagraphProperties {
  Level1(Box<a::Level1ParagraphProperties>),
  Level2(Box<a::Level2ParagraphProperties>),
  Level3(Box<a::Level3ParagraphProperties>),
  Level4(Box<a::Level4ParagraphProperties>),
  Level5(Box<a::Level5ParagraphProperties>),
  Level6(Box<a::Level6ParagraphProperties>),
  Level7(Box<a::Level7ParagraphProperties>),
  Level8(Box<a::Level8ParagraphProperties>),
  Level9(Box<a::Level9ParagraphProperties>),
}

macro_rules! inherit_missing {
  ($merged:ident, $base:expr, $($field:ident),+ $(,)?) => {
    $(
      if $merged.$field.is_none() {
        $merged.$field = $base.$field.clone();
      }
    )+
  };
}

macro_rules! merge_paragraph_properties {
  ($source:expr, $base:expr, $choice1:ident, $choice2:ident, $choice3:ident,
   $choice4:ident) => {{
    let mut merged = (**$source).clone();
    inherit_missing!(
      merged,
      $base,
      left_margin,
      right_margin,
      level,
      indent,
      alignment,
      default_tab_size,
      right_to_left,
      east_asian_line_break,
      font_alignment,
      latin_line_break,
      height,
      line_spacing,
      space_before,
      space_after,
      $choice1,
      $choice2,
      $choice3,
      $choice4,
      tab_stop_list,
      extension_list,
    );
    merged.default_run_properties = merge_default_run_properties(
      $base.default_run_properties.as_deref(),
      merged.default_run_properties.as_deref(),
    )
    .map(Box::new);
    merged
  }};
}

impl TextListParagraphStyleRef<'_> {
  pub(crate) fn to_owned_style(self) -> TextListParagraphStyle {
    match self {
      TextListParagraphStyleRef::Default(properties) => {
        TextListParagraphStyle::Default(Box::new(properties.clone()))
      }
      TextListParagraphStyleRef::Level(style) => TextListParagraphStyle::Level(style.clone()),
    }
  }
}

impl TextListStyle {
  pub(crate) fn from_dml_list_style(source: &a::ListStyle) -> Self {
    Self::from_parts(TextListStyleParts {
      default_paragraph_properties: source.default_paragraph_properties.clone(),
      level1_paragraph_properties: source.level1_paragraph_properties.clone(),
      level2_paragraph_properties: source.level2_paragraph_properties.clone(),
      level3_paragraph_properties: source.level3_paragraph_properties.clone(),
      level4_paragraph_properties: source.level4_paragraph_properties.clone(),
      level5_paragraph_properties: source.level5_paragraph_properties.clone(),
      level6_paragraph_properties: source.level6_paragraph_properties.clone(),
      level7_paragraph_properties: source.level7_paragraph_properties.clone(),
      level8_paragraph_properties: source.level8_paragraph_properties.clone(),
      level9_paragraph_properties: source.level9_paragraph_properties.clone(),
    })
  }

  pub(crate) fn from_pml_title_style(source: &p::TitleStyle) -> Self {
    Self::from_parts(TextListStyleParts {
      default_paragraph_properties: source.default_paragraph_properties.clone(),
      level1_paragraph_properties: source.level1_paragraph_properties.clone(),
      level2_paragraph_properties: source.level2_paragraph_properties.clone(),
      level3_paragraph_properties: source.level3_paragraph_properties.clone(),
      level4_paragraph_properties: source.level4_paragraph_properties.clone(),
      level5_paragraph_properties: source.level5_paragraph_properties.clone(),
      level6_paragraph_properties: source.level6_paragraph_properties.clone(),
      level7_paragraph_properties: source.level7_paragraph_properties.clone(),
      level8_paragraph_properties: source.level8_paragraph_properties.clone(),
      level9_paragraph_properties: source.level9_paragraph_properties.clone(),
    })
  }

  pub(crate) fn from_pml_body_style(source: &p::BodyStyle) -> Self {
    Self::from_parts(TextListStyleParts {
      default_paragraph_properties: source.default_paragraph_properties.clone(),
      level1_paragraph_properties: source.level1_paragraph_properties.clone(),
      level2_paragraph_properties: source.level2_paragraph_properties.clone(),
      level3_paragraph_properties: source.level3_paragraph_properties.clone(),
      level4_paragraph_properties: source.level4_paragraph_properties.clone(),
      level5_paragraph_properties: source.level5_paragraph_properties.clone(),
      level6_paragraph_properties: source.level6_paragraph_properties.clone(),
      level7_paragraph_properties: source.level7_paragraph_properties.clone(),
      level8_paragraph_properties: source.level8_paragraph_properties.clone(),
      level9_paragraph_properties: source.level9_paragraph_properties.clone(),
    })
  }

  pub(crate) fn from_pml_other_style(source: &p::OtherStyle) -> Self {
    Self::from_parts(TextListStyleParts {
      default_paragraph_properties: source.default_paragraph_properties.clone(),
      level1_paragraph_properties: source.level1_paragraph_properties.clone(),
      level2_paragraph_properties: source.level2_paragraph_properties.clone(),
      level3_paragraph_properties: source.level3_paragraph_properties.clone(),
      level4_paragraph_properties: source.level4_paragraph_properties.clone(),
      level5_paragraph_properties: source.level5_paragraph_properties.clone(),
      level6_paragraph_properties: source.level6_paragraph_properties.clone(),
      level7_paragraph_properties: source.level7_paragraph_properties.clone(),
      level8_paragraph_properties: source.level8_paragraph_properties.clone(),
      level9_paragraph_properties: source.level9_paragraph_properties.clone(),
    })
  }

  pub(crate) fn from_pml_notes_style(source: &p::NotesStyle) -> Self {
    Self::from_parts(TextListStyleParts {
      default_paragraph_properties: source.default_paragraph_properties.clone(),
      level1_paragraph_properties: source.level1_paragraph_properties.clone(),
      level2_paragraph_properties: source.level2_paragraph_properties.clone(),
      level3_paragraph_properties: source.level3_paragraph_properties.clone(),
      level4_paragraph_properties: source.level4_paragraph_properties.clone(),
      level5_paragraph_properties: source.level5_paragraph_properties.clone(),
      level6_paragraph_properties: source.level6_paragraph_properties.clone(),
      level7_paragraph_properties: source.level7_paragraph_properties.clone(),
      level8_paragraph_properties: source.level8_paragraph_properties.clone(),
      level9_paragraph_properties: source.level9_paragraph_properties.clone(),
    })
  }

  pub(crate) fn from_pml_default_text_style(source: &p::DefaultTextStyle) -> Self {
    Self::from_parts(TextListStyleParts {
      default_paragraph_properties: source.default_paragraph_properties.clone(),
      level1_paragraph_properties: source.level1_paragraph_properties.clone(),
      level2_paragraph_properties: source.level2_paragraph_properties.clone(),
      level3_paragraph_properties: source.level3_paragraph_properties.clone(),
      level4_paragraph_properties: source.level4_paragraph_properties.clone(),
      level5_paragraph_properties: source.level5_paragraph_properties.clone(),
      level6_paragraph_properties: source.level6_paragraph_properties.clone(),
      level7_paragraph_properties: source.level7_paragraph_properties.clone(),
      level8_paragraph_properties: source.level8_paragraph_properties.clone(),
      level9_paragraph_properties: source.level9_paragraph_properties.clone(),
    })
  }

  fn from_parts(parts: TextListStyleParts) -> Self {
    let mut style = Self {
      default_paragraph_properties: parts.default_paragraph_properties,
      levels: Vec::new(),
    };
    if let Some(properties) = parts.level1_paragraph_properties {
      style.levels.push(TextListLevelStyle::new(
        1,
        TextListLevelParagraphProperties::Level1(properties),
      ));
    }
    if let Some(properties) = parts.level2_paragraph_properties {
      style.levels.push(TextListLevelStyle::new(
        2,
        TextListLevelParagraphProperties::Level2(properties),
      ));
    }
    if let Some(properties) = parts.level3_paragraph_properties {
      style.levels.push(TextListLevelStyle::new(
        3,
        TextListLevelParagraphProperties::Level3(properties),
      ));
    }
    if let Some(properties) = parts.level4_paragraph_properties {
      style.levels.push(TextListLevelStyle::new(
        4,
        TextListLevelParagraphProperties::Level4(properties),
      ));
    }
    if let Some(properties) = parts.level5_paragraph_properties {
      style.levels.push(TextListLevelStyle::new(
        5,
        TextListLevelParagraphProperties::Level5(properties),
      ));
    }
    if let Some(properties) = parts.level6_paragraph_properties {
      style.levels.push(TextListLevelStyle::new(
        6,
        TextListLevelParagraphProperties::Level6(properties),
      ));
    }
    if let Some(properties) = parts.level7_paragraph_properties {
      style.levels.push(TextListLevelStyle::new(
        7,
        TextListLevelParagraphProperties::Level7(properties),
      ));
    }
    if let Some(properties) = parts.level8_paragraph_properties {
      style.levels.push(TextListLevelStyle::new(
        8,
        TextListLevelParagraphProperties::Level8(properties),
      ));
    }
    if let Some(properties) = parts.level9_paragraph_properties {
      style.levels.push(TextListLevelStyle::new(
        9,
        TextListLevelParagraphProperties::Level9(properties),
      ));
    }
    style
  }

  pub(crate) fn merge_from(&mut self, source: &Self) {
    if let Some(properties) = &source.default_paragraph_properties {
      self.default_paragraph_properties = Some(Box::new(merge_default_paragraph_properties(
        self.default_paragraph_properties.as_deref(),
        properties,
      )));
    }
    for level_style in &source.levels {
      if let Some(existing) = self
        .levels
        .iter_mut()
        .find(|existing| existing.level == level_style.level)
      {
        *existing = level_style.clone_merged_over(existing);
      } else {
        self.levels.push(level_style.clone());
      }
    }
    self.levels.sort_by_key(|style| style.level);
  }

  pub(crate) fn is_empty(&self) -> bool {
    self.default_paragraph_properties.is_none() && self.levels.is_empty()
  }

  pub(crate) fn paragraph_style_for_level(
    &self,
    paragraph_level: Option<u8>,
  ) -> Option<TextListParagraphStyleRef<'_>> {
    if self.levels.is_empty() {
      return self
        .default_paragraph_properties
        .as_deref()
        .map(TextListParagraphStyleRef::Default);
    }

    let requested_level = paragraph_level.unwrap_or(0).saturating_add(1);
    self
      .levels
      .iter()
      .find(|style| style.level == requested_level)
      .or_else(|| self.levels.first())
      .map(TextListParagraphStyleRef::Level)
  }
}

impl TextListLevelStyle {
  fn new(level: u8, paragraph_properties: TextListLevelParagraphProperties) -> Self {
    Self {
      level,
      paragraph_properties,
    }
  }

  fn clone_merged_over(&self, inherited: &Self) -> Self {
    Self {
      level: self.level,
      paragraph_properties: self
        .paragraph_properties
        .clone_merged_over(&inherited.paragraph_properties),
    }
  }
}

impl TextListLevelParagraphProperties {
  fn clone_merged_over(&self, inherited: &Self) -> Self {
    macro_rules! merge_level {
      ($variant:ident, $source:expr, $base:expr, $choice1:ident, $choice2:ident,
       $choice3:ident, $choice4:ident) => {
        Self::$variant(Box::new(merge_paragraph_properties!(
          $source, $base, $choice1, $choice2, $choice3, $choice4
        )))
      };
    }
    match (self, inherited) {
      (Self::Level1(source), Self::Level1(base)) => merge_level!(
        Level1,
        source,
        base,
        level1_paragraph_properties_choice1,
        level1_paragraph_properties_choice2,
        level1_paragraph_properties_choice3,
        level1_paragraph_properties_choice4
      ),
      (Self::Level2(source), Self::Level2(base)) => merge_level!(
        Level2,
        source,
        base,
        level2_paragraph_properties_choice1,
        level2_paragraph_properties_choice2,
        level2_paragraph_properties_choice3,
        level2_paragraph_properties_choice4
      ),
      (Self::Level3(source), Self::Level3(base)) => merge_level!(
        Level3,
        source,
        base,
        level3_paragraph_properties_choice1,
        level3_paragraph_properties_choice2,
        level3_paragraph_properties_choice3,
        level3_paragraph_properties_choice4
      ),
      (Self::Level4(source), Self::Level4(base)) => merge_level!(
        Level4,
        source,
        base,
        level4_paragraph_properties_choice1,
        level4_paragraph_properties_choice2,
        level4_paragraph_properties_choice3,
        level4_paragraph_properties_choice4
      ),
      (Self::Level5(source), Self::Level5(base)) => merge_level!(
        Level5,
        source,
        base,
        level5_paragraph_properties_choice1,
        level5_paragraph_properties_choice2,
        level5_paragraph_properties_choice3,
        level5_paragraph_properties_choice4
      ),
      (Self::Level6(source), Self::Level6(base)) => merge_level!(
        Level6,
        source,
        base,
        level6_paragraph_properties_choice1,
        level6_paragraph_properties_choice2,
        level6_paragraph_properties_choice3,
        level6_paragraph_properties_choice4
      ),
      (Self::Level7(source), Self::Level7(base)) => merge_level!(
        Level7,
        source,
        base,
        level7_paragraph_properties_choice1,
        level7_paragraph_properties_choice2,
        level7_paragraph_properties_choice3,
        level7_paragraph_properties_choice4
      ),
      (Self::Level8(source), Self::Level8(base)) => merge_level!(
        Level8,
        source,
        base,
        level8_paragraph_properties_choice1,
        level8_paragraph_properties_choice2,
        level8_paragraph_properties_choice3,
        level8_paragraph_properties_choice4
      ),
      (Self::Level9(source), Self::Level9(base)) => merge_level!(
        Level9,
        source,
        base,
        level9_paragraph_properties_choice1,
        level9_paragraph_properties_choice2,
        level9_paragraph_properties_choice3,
        level9_paragraph_properties_choice4
      ),
      _ => self.clone(),
    }
  }
}

fn merge_default_paragraph_properties(
  inherited: Option<&a::DefaultParagraphProperties>,
  source: &a::DefaultParagraphProperties,
) -> a::DefaultParagraphProperties {
  let Some(inherited) = inherited else {
    return source.clone();
  };
  let mut merged = source.clone();
  inherit_missing!(
    merged,
    inherited,
    left_margin,
    right_margin,
    level,
    indent,
    alignment,
    default_tab_size,
    right_to_left,
    east_asian_line_break,
    font_alignment,
    latin_line_break,
    height,
    line_spacing,
    space_before,
    space_after,
    default_paragraph_properties_choice1,
    default_paragraph_properties_choice2,
    default_paragraph_properties_choice3,
    default_paragraph_properties_choice4,
    tab_stop_list,
    extension_list,
  );
  merged.default_run_properties = merge_default_run_properties(
    inherited.default_run_properties.as_deref(),
    merged.default_run_properties.as_deref(),
  )
  .map(Box::new);
  merged
}

fn merge_default_run_properties(
  inherited: Option<&a::DefaultRunProperties>,
  source: Option<&a::DefaultRunProperties>,
) -> Option<a::DefaultRunProperties> {
  let Some(inherited) = inherited else {
    return source.cloned();
  };
  let Some(source) = source else {
    return Some(inherited.clone());
  };
  let mut merged = source.clone();
  inherit_missing!(
    merged,
    inherited,
    kumimoji,
    language,
    alternative_language,
    font_size,
    bold,
    italic,
    underline,
    strike,
    kerning,
    capital,
    spacing,
    normalize_height,
    baseline,
    no_proof,
    dirty,
    spelling_error,
    smart_tag_clean,
    smart_tag_id,
    bookmark,
    outline,
    default_run_properties_choice1,
    default_run_properties_choice2,
    highlight,
    default_run_properties_choice3,
    default_run_properties_choice4,
    latin_font,
    east_asian_font,
    complex_script_font,
    symbol_font,
    hyperlink_on_click,
    hyperlink_on_mouse_over,
    right_to_left,
    extension_list,
  );
  Some(merged)
}

struct TextListStyleParts {
  default_paragraph_properties: Option<Box<a::DefaultParagraphProperties>>,
  level1_paragraph_properties: Option<Box<a::Level1ParagraphProperties>>,
  level2_paragraph_properties: Option<Box<a::Level2ParagraphProperties>>,
  level3_paragraph_properties: Option<Box<a::Level3ParagraphProperties>>,
  level4_paragraph_properties: Option<Box<a::Level4ParagraphProperties>>,
  level5_paragraph_properties: Option<Box<a::Level5ParagraphProperties>>,
  level6_paragraph_properties: Option<Box<a::Level6ParagraphProperties>>,
  level7_paragraph_properties: Option<Box<a::Level7ParagraphProperties>>,
  level8_paragraph_properties: Option<Box<a::Level8ParagraphProperties>>,
  level9_paragraph_properties: Option<Box<a::Level9ParagraphProperties>>,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn merge_preserves_unspecified_master_run_properties() {
    let inherited_run = a::DefaultRunProperties {
      font_size: Some(4400),
      latin_font: Some(a::LatinFont {
        typeface: Some("+mj-lt".to_string()),
        ..Default::default()
      }),
      ..Default::default()
    };
    let mut inherited = TextListStyle {
      levels: vec![TextListLevelStyle::new(
        1,
        TextListLevelParagraphProperties::Level1(Box::new(a::Level1ParagraphProperties {
          left_margin: Some(342900),
          default_run_properties: Some(Box::new(inherited_run)),
          ..Default::default()
        })),
      )],
      ..Default::default()
    };
    let source = TextListStyle {
      levels: vec![TextListLevelStyle::new(
        1,
        TextListLevelParagraphProperties::Level1(Box::new(a::Level1ParagraphProperties {
          alignment: Some(a::TextAlignmentTypeValues::Center),
          default_run_properties: Some(Box::new(a::DefaultRunProperties {
            font_size: Some(2400),
            ..Default::default()
          })),
          ..Default::default()
        })),
      )],
      ..Default::default()
    };

    inherited.merge_from(&source);

    let TextListLevelParagraphProperties::Level1(properties) =
      &inherited.levels[0].paragraph_properties
    else {
      panic!("expected level 1 paragraph properties");
    };
    assert_eq!(properties.left_margin, Some(342900));
    assert_eq!(
      properties.alignment,
      Some(a::TextAlignmentTypeValues::Center)
    );
    let run = properties
      .default_run_properties
      .as_deref()
      .expect("merged default run properties");
    assert_eq!(run.font_size, Some(2400));
    assert_eq!(
      run
        .latin_font
        .as_ref()
        .and_then(|font| font.typeface.as_deref()),
      Some("+mj-lt")
    );
  }
}
