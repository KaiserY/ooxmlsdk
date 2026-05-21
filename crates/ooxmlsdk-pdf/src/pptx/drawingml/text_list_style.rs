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
      self.default_paragraph_properties = Some(properties.clone());
    }
    for level_style in &source.levels {
      if let Some(existing) = self
        .levels
        .iter_mut()
        .find(|existing| existing.level == level_style.level)
      {
        *existing = level_style.clone();
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
