//! SIST02 books, chapters, proceedings, reports and periodical articles.
//!
//! ECMA-376 17.16.5.7/8 defines source/field selection. Word's SIST02 templates
//! and native source/locale matrices establish the separate name, punctuation,
//! ordering and rich-text rules below. Unrepresented metadata retains the cache.

use super::*;
use apa::Locale;

#[derive(Clone, Debug, Default)]
pub(super) struct Entry {
  kind: b::DataSourceValues,
  tag: String,
  authors: Authors,
  book_authors: Authors,
  title: String,
  book_title: String,
  year: String,
  month: String,
  day: String,
  city: String,
  publisher: String,
  pages: String,
  conference: String,
  periodical: String,
  locale: Option<Locale>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Authors {
  corporate: Option<String>,
  people: Vec<(String, String, String)>,
}

fn locale_from_lcid(value: &str) -> Option<Locale> {
  match value {
    "1033" | "2057" | "3081" | "4105" | "5129" => Some(Locale::English),
    "2052" | "4100" => Some(Locale::Chinese),
    // Word also resolves culture-name values such as en-US/en-IN. Unknown
    // identifiers have no localized resources; retain their complete cache.
    _ => {
      let locale = crate::localization::canonical_locale(value)?;
      if !locale.id.variants.is_empty() || !locale.extensions.is_empty() {
        return None;
      }
      match locale.id.language.as_str() {
        "en" => Some(Locale::English),
        "zh"
          if locale
            .id
            .region
            .is_some_and(|r| matches!(r.as_str(), "CN" | "SG"))
            || locale.id.script.is_some_and(|s| s.as_str() == "Hans") =>
        {
          Some(Locale::Chinese)
        }
        _ => None,
      }
    }
  }
}

pub(super) fn parse(sources: &b::Sources) -> Option<Vec<Entry>> {
  sources.source.iter().map(Entry::parse).collect()
}

impl Entry {
  fn parse(source: &b::Source) -> Option<Self> {
    let mut entry = Self::default();
    for choice in &source.source_choice {
      match choice {
        b::SourceChoice::SourceType(kind) => entry.kind = *kind,
        b::SourceChoice::Tag(v) => entry.tag = v.to_string(),
        b::SourceChoice::Title(v) => entry.title = v.to_string(),
        b::SourceChoice::BookTitle(v) => entry.book_title = v.to_string(),
        b::SourceChoice::Year(v) => entry.year = v.to_string(),
        b::SourceChoice::Month(v) => entry.month = v.to_string(),
        b::SourceChoice::Day(v) => entry.day = v.to_string(),
        b::SourceChoice::City(v) => entry.city = v.to_string(),
        b::SourceChoice::Publisher(v) => entry.publisher = v.to_string(),
        b::SourceChoice::Pages(v) => entry.pages = v.to_string(),
        b::SourceChoice::ConferenceName(v) => entry.conference = v.to_string(),
        b::SourceChoice::PeriodicalTitle(v) => entry.periodical = v.to_string(),
        b::SourceChoice::GuidString(_) | b::SourceChoice::ReferenceOrder(_) => {}
        b::SourceChoice::LcId(v) if matches!(v.as_str(), "" | "0") => {}
        b::SourceChoice::LcId(v) => entry.locale = Some(locale_from_lcid(v.as_str())?),
        b::SourceChoice::AuthorList(list) => {
          for author in &list.author_list_choice {
            match author {
              b::AuthorListChoice::Author(author) => match author.author_choice.as_ref()? {
                b::AuthorChoice::Corporate(v) => entry.authors.corporate = Some(v.to_string()),
                b::AuthorChoice::NameList(names) => entry.authors = Authors::parse(names)?,
              },
              b::AuthorListChoice::BookAuthor(author) => {
                entry.book_authors = Authors::parse(&author.name_list)?;
              }
              _ => return None,
            }
          }
        }
        _ => return None,
      }
    }
    (matches!(
      entry.kind,
      b::DataSourceValues::Book
        | b::DataSourceValues::BookSection
        | b::DataSourceValues::ConferenceProceedings
        | b::DataSourceValues::Report
        | b::DataSourceValues::ArticleInAPeriodical
    ) && !entry.tag.is_empty())
    .then_some(entry)
  }

  fn sort_key(&self, locale: Locale) -> String {
    let author = self.authors.text(self.locale.unwrap_or(locale), false);
    let parts = if self.kind == b::DataSourceValues::ConferenceProceedings {
      [&self.title, &author]
    } else {
      [&author, &self.title]
    };
    parts
      .into_iter()
      .filter(|s| !s.is_empty())
      .map(String::as_str)
      .collect::<Vec<_>>()
      .join(" ")
      .to_lowercase()
  }

  fn location(&self, locale: Locale) -> String {
    if self.city.is_empty() && self.publisher.is_empty() {
      return String::new();
    }
    let city = if self.city.is_empty() {
      if locale == Locale::Chinese {
        "无出版地"
      } else {
        "s.l."
      }
    } else {
      &self.city
    };
    let publisher = if self.publisher.is_empty() {
      if locale == Locale::Chinese {
        "无名氏"
      } else {
        "s.n."
      }
    } else {
      &self.publisher
    };
    format!("{city}，{publisher}")
  }

  fn pages(&self, locale: Locale) -> String {
    if self.pages.is_empty() {
      return String::new();
    }
    let prefix = if locale == Locale::Chinese {
      "页 "
    } else if self.pages.contains([',', '-', '–', '—']) {
      "pp."
    } else {
      "p."
    };
    format!("{prefix}{}", self.pages)
  }

  fn date(&self, locale: Locale) -> String {
    if self.year.is_empty() {
      return String::new();
    }
    if self.month.is_empty() {
      return if locale == Locale::Chinese {
        format!("{}年", self.year)
      } else {
        self.year.clone()
      };
    }
    [&self.day, &self.month, &self.year]
      .into_iter()
      .filter(|s| !s.is_empty())
      .map(String::as_str)
      .collect::<Vec<_>>()
      .join(" ")
  }

  fn formatted(&self, locale: Locale) -> Text {
    let mut text = Text::default();
    let italic = locale == Locale::English;
    let author = self.authors.text(locale, false);
    let conference = self.kind == b::DataSourceValues::ConferenceProceedings;
    if conference {
      text.sentence(&self.title, italic);
      // Word's HTML import separates adjacent nonempty i/b elements. SIST's
      // own Space template is empty; plain-text title boundaries stay joined.
      if italic && !self.title.is_empty() && !author.is_empty() {
        text.push(" ".into(), false, false);
      }
    }
    text.push(author.clone(), true, false);
    let location = self.location(locale);
    let pages = self.pages(locale);
    match self.kind {
      b::DataSourceValues::Book | b::DataSourceValues::Report => {
        if italic && !author.is_empty() && !self.title.is_empty() {
          text.push(" ".into(), false, false);
        }
        text.sentence(&self.title, italic);
        if self.kind == b::DataSourceValues::Book {
          text.list(&[&location, &self.year, &pages]);
        } else {
          text.list(&[&location, &self.year]);
          text.list(&[&pages]);
        }
      }
      b::DataSourceValues::BookSection => {
        text.sentence(&self.title, false);
        if self.book_authors != self.authors {
          let secondary = self.book_authors.text(locale, true);
          if !secondary.is_empty() {
            let role = if locale == Locale::Chinese {
              "书籍作者"
            } else {
              "book auth."
            };
            text.push(format!("{secondary}{role}．"), false, false);
          }
        }
        text.sentence(&self.book_title, italic);
        text.list(&[&location, &self.year, &pages]);
      }
      b::DataSourceValues::ConferenceProceedings => {
        text.list(&[&location, &self.year]);
        text.sentence(&self.conference, false);
        text.list(&[&pages]);
      }
      b::DataSourceValues::ArticleInAPeriodical => {
        text.sentence(&self.title, false);
        text.sentence(&self.periodical, italic);
        text.list(&[&self.date(locale), &pages]);
      }
      _ => unreachable!("validated source kind"),
    }
    text
  }

  fn citation(&self, locale: Locale) -> String {
    let author =
      if self.kind == b::DataSourceValues::ConferenceProceedings && !self.title.is_empty() {
        self.title.clone()
      } else if let Some(corporate) = &self.authors.corporate {
        corporate.clone()
      } else if let Some((last, first, middle)) = self.authors.people.first() {
        let mut name = if !last.is_empty() {
          last
        } else if !first.is_empty() {
          first
        } else {
          middle
        }
        .clone();
        if self.authors.people.len() > 1 {
          name.push_str(others(locale));
        }
        name
      } else {
        String::new()
      };
    let content = [&author, &self.year]
      .into_iter()
      .filter(|s| !s.is_empty())
      .map(String::as_str)
      .collect::<Vec<_>>()
      .join("，");
    format!(
      "（{}）",
      if content.is_empty() {
        &self.tag
      } else {
        &content
      }
    )
  }
}

fn others(locale: Locale) -> &'static str {
  if locale == Locale::Chinese {
    "以及其他人"
  } else {
    "et al."
  }
}

impl Authors {
  fn parse(names: &b::NameList) -> Option<Self> {
    Some(Self {
      corporate: None,
      people: names
        .person
        .iter()
        .map(|p| {
          Some((
            name(&p.last)?.to_owned(),
            name(&p.first)?.to_owned(),
            name(&p.middle)?.to_owned(),
          ))
        })
        .collect::<Option<_>>()?,
    })
  }

  fn text(&self, locale: Locale, secondary: bool) -> String {
    if let Some(corporate) = &self.corporate {
      return corporate.clone();
    }
    let count = if self.people.len() > 3 {
      1
    } else {
      self.people.len()
    };
    let mut text = self
      .people
      .iter()
      .take(count)
      .map(|(last, first, middle)| {
        if secondary {
          format!("{last}{middle}{first}")
        } else {
          format!("{last}{first}{middle}")
        }
      })
      .collect::<Vec<_>>()
      .join(", ");
    if self.people.len() > 3 {
      text.push_str(others(locale));
    }
    text
  }
}

#[derive(Default)]
struct Text {
  runs: Vec<(String, bool, bool)>,
}
impl Text {
  fn push(&mut self, text: String, bold: bool, italic: bool) {
    if text.is_empty() {
      return;
    }
    if let Some((last, last_bold, last_italic)) = self.runs.last_mut()
      && *last_bold == bold
      && *last_italic == italic
    {
      last.push_str(&text);
    } else {
      self.runs.push((text, bold, italic));
    }
  }
  fn sentence(&mut self, text: &str, italic: bool) {
    if text.is_empty() {
      return;
    }
    let mut text = text.to_owned();
    if !text.ends_with(['.', '!', '?', '．', '。', '！', '？']) {
      text.push('．');
    }
    self.push(text, false, italic);
  }
  fn list(&mut self, items: &[&str]) {
    self.sentence(
      &items
        .iter()
        .copied()
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("，"),
      false,
    );
  }
}

pub(super) fn refresh(
  entries: &[Entry],
  instruction: &str,
  styles: &StylesCatalog,
  context: &TextStyle,
  preceding_locale: LocaleContext,
) -> Option<Result> {
  let tokens = field_instruction_tokens(instruction);
  let citation = tokens.first()?.eq_ignore_ascii_case("CITATION");
  if !citation && !tokens[0].eq_ignore_ascii_case("BIBLIOGRAPHY") {
    return None;
  }
  let mut tag = if citation {
    Some(tokens.get(1)?.as_str())
  } else {
    None
  };
  let mut index = if citation { 2 } else { 1 };
  let mut locale = preceding_locale.0.unwrap_or_else(|| {
    Locale::language(
      styles
        .locales
        .default_document_language()
        .or(context.language.as_deref())
        .unwrap_or("en-US"),
    )
  });
  while index < tokens.len() {
    match tokens[index].to_ascii_lowercase().as_str() {
      "\\l" => locale = locale_from_lcid(tokens.get(index + 1)?),
      "\\m" if !citation => tag = Some(tokens.get(index + 1)?.as_str()),
      _ => return None,
    }
    index += 2;
  }
  let locale = locale?;
  let mut selected = entries
    .iter()
    .filter(|entry| tag.is_none_or(|tag| entry.tag == tag))
    .collect::<Vec<_>>();
  if selected.is_empty() {
    return None;
  }
  if citation {
    let [entry] = selected.as_slice() else {
      return None;
    };
    let mut inlines = Vec::new();
    push_resolved_field_text(&mut inlines, entry.citation(locale), context.clone(), None);
    return Some(Result {
      inlines,
      paragraphs: Vec::new(),
    });
  }
  selected.sort_by_cached_key(|entry| entry.sort_key(locale));
  // Repeated-author/title substitution needs its own observed resource policy.
  if selected.windows(2).any(|p| {
    (p[0].authors == p[1].authors && p[0].authors != Authors::default())
      || (p[0].kind == b::DataSourceValues::ConferenceProceedings
        && p[1].kind == p[0].kind
        && p[0].title == p[1].title)
  }) {
    return None;
  }
  let style_id = styles
    .styles
    .iter()
    .find(|(_, entry)| {
      entry
        .name
        .as_deref()
        .is_some_and(|name| name.eq_ignore_ascii_case("Bibliography"))
    })
    .map(|(id, _)| id.as_str());
  let base =
    styles.run_style_with_base(style_id, TextStyle::default(), RunStyleOverrides::default());
  let format = styles.paragraph_format_with_base(style_id, ParagraphFormat::default());
  let mut result = Result {
    inlines: Vec::new(),
    paragraphs: Vec::new(),
  };
  for (index, entry) in selected.into_iter().enumerate() {
    let locale = entry.locale.unwrap_or(locale);
    for (text, bold, italic) in entry.formatted(locale).runs {
      let mut style = base.clone();
      style.bold |= bold;
      style.italic |= italic;
      if bold {
        style.complex_bold = Some(true);
      }
      if italic {
        style.complex_italic = Some(true);
      }
      if locale == Locale::Chinese {
        style.wordprocessingml_font_hint =
          Some(ooxmlsdk_fonts::WordprocessingFontTypeHint::EastAsia);
      }
      push_resolved_field_text(&mut result.inlines, text, style, None);
    }
    let mut mark = base.clone();
    if index == 0 {
      mark.font_size_pt = 12.0;
      mark.complex_font_size_pt = Some(12.0);
    }
    result
      .paragraphs
      .push(ParagraphFieldEvent::GeneratedParagraphBreak {
        inline_offset: result.inlines.len(),
        format: Box::new(format.clone()),
        base_style: Box::new(mark),
      });
  }
  Some(result)
}

#[cfg(test)]
mod tests {
  use super::*;

  fn entry(kind: b::DataSourceValues) -> Entry {
    Entry {
      kind,
      tag: "Example".into(),
      title: "Title".into(),
      authors: Authors {
        corporate: Some("Institute".into()),
        ..Default::default()
      },
      book_title: "Collected".into(),
      year: "2021".into(),
      month: "October".into(),
      day: "3".into(),
      city: "City".into(),
      publisher: "Press".into(),
      pages: "33".into(),
      conference: "Conference".into(),
      periodical: "Periodical".into(),
      ..Default::default()
    }
  }
  fn plain(text: Text) -> String {
    text.runs.into_iter().map(|r| r.0).collect()
  }

  #[test]
  fn native_sist_source_types_have_distinct_order_and_rich_boundaries() {
    for (kind, expected) in [
      (
        b::DataSourceValues::Book,
        "Institute Title．City，Press，2021，p.33．",
      ),
      (
        b::DataSourceValues::BookSection,
        "InstituteTitle．Collected．City，Press，2021，p.33．",
      ),
      (
        b::DataSourceValues::Report,
        "Institute Title．City，Press，2021．p.33．",
      ),
      (
        b::DataSourceValues::ConferenceProceedings,
        "Title． InstituteCity，Press，2021．Conference．p.33．",
      ),
      (
        b::DataSourceValues::ArticleInAPeriodical,
        "InstituteTitle．Periodical．3 October 2021，p.33．",
      ),
    ] {
      assert_eq!(plain(entry(kind).formatted(Locale::English)), expected);
    }
    let book = entry(b::DataSourceValues::Book);
    assert_eq!(
      book.formatted(Locale::English).runs,
      vec![
        ("Institute".into(), true, false),
        (" ".into(), false, false),
        ("Title．".into(), false, true),
        ("City，Press，2021，p.33．".into(), false, false)
      ]
    );
    assert_eq!(
      plain(book.formatted(Locale::Chinese)),
      "InstituteTitle．City，Press，2021，页 33．"
    );
  }

  #[test]
  fn native_sist_missing_location_date_and_page_ranges() {
    let mut book = entry(b::DataSourceValues::Book);
    book.city.clear();
    book.pages = "21-35".into();
    assert_eq!(
      plain(book.formatted(Locale::English)),
      "Institute Title．s.l.，Press，2021，pp.21-35．"
    );
    book.publisher.clear();
    assert_eq!(
      plain(book.formatted(Locale::English)),
      "Institute Title．2021，pp.21-35．"
    );
    book.kind = b::DataSourceValues::ArticleInAPeriodical;
    book.month.clear();
    assert_eq!(book.date(Locale::English), "2021");
    assert_eq!(book.date(Locale::Chinese), "2021年");
    book.year.clear();
    assert!(book.date(Locale::English).is_empty());
  }

  #[test]
  fn native_sist_full_names_secondary_inversion_and_truncation() {
    let mut names = Authors::default();
    for last in ["Adams", "Baker", "Clark", "Davis"] {
      names
        .people
        .push((last.into(), "Anne".into(), "Beth".into()));
    }
    assert_eq!(names.text(Locale::English, false), "AdamsAnneBethet al.");
    names.people.pop();
    assert_eq!(
      names.text(Locale::English, false),
      "AdamsAnneBeth, BakerAnneBeth, ClarkAnneBeth"
    );
    assert_eq!(
      names.text(Locale::English, true),
      "AdamsBethAnne, BakerBethAnne, ClarkBethAnne"
    );
    let mut chapter = entry(b::DataSourceValues::BookSection);
    chapter.authors = names.clone();
    chapter.book_authors = names;
    assert!(!plain(chapter.formatted(Locale::English)).contains("book auth."));
    chapter.book_authors.people[0].0 = "Other".into();
    assert!(
      plain(chapter.formatted(Locale::English))
        .contains("OtherBethAnne, BakerBethAnne, ClarkBethAnnebook auth.．Collected．")
    );
  }

  #[test]
  fn sist_citation_uses_short_author_and_preserves_unknown_switches() {
    let mut item = entry(b::DataSourceValues::Report);
    assert_eq!(item.citation(Locale::English), "（Institute，2021）");
    item.authors = Authors {
      corporate: None,
      people: vec![
        ("DG".into(), String::new(), String::new()),
        ("D".into(), "A".into(), "G".into()),
      ],
    };
    assert_eq!(item.citation(Locale::English), "（DGet al.，2021）");
    assert!(
      refresh(
        &[item],
        "CITATION Example \\p 4",
        &StylesCatalog::default(),
        &TextStyle::default(),
        LocaleContext::default()
      )
      .is_none()
    );
  }

  #[test]
  fn sist_catalog_is_strict_and_does_not_enable_other_style_profiles() {
    let source = "<b:Source><b:Tag>T</b:Tag><b:SourceType>ConferenceProceedings</b:SourceType><b:ConferenceName>Meeting</b:ConferenceName><b:LCID>en-IN</b:LCID></b:Source>";
    let xml = format!(
      r#"<b:Sources xmlns:b="{WORD_BIBLIOGRAPHY_NAMESPACE}" SelectedStyle="SIST02.XSL">{source}</b:Sources>"#
    );
    let catalog = Catalog::parse(&xml).unwrap();
    assert!(catalog.generates_paragraphs());
    assert_eq!(catalog.sist[0].locale, Some(Locale::English));
    assert_eq!(locale_from_lcid("en-US"), Some(Locale::English));
    for value in ["bogus", "enbogus", "en-US-bogus", "999999", "-1"] {
      assert_eq!(locale_from_lcid(value), None, "{value}");
    }
    assert!(Catalog::parse(&xml.replace("en-IN", "1031")).is_none());
    assert!(Catalog::parse(&xml.replace("SIST02.XSL", "GB.XSL")).is_none());
    assert!(
      Catalog::parse(&xml.replace("</b:Source>", "<b:Unknown>extra</b:Unknown></b:Source>"))
        .is_none()
    );
  }
}
