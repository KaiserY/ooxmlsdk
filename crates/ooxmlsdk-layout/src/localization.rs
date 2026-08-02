use std::sync::Arc;

use icu_locale::fallback::LocaleFallbacker;
use icu_locale::{DataLocale, Locale, LocaleCanonicalizer, LocaleExpander};

/// The locale dimensions needed while reproducing application output.
///
/// Office UI resources, value formatting, and document authoring defaults
/// are separate inputs. A missing format or authoring locale deliberately
/// follows the UI locale only as a backwards-compatible fallback.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct OfficeLocaleContext {
  ui_language: Option<Arc<str>>,
  format_locale: Option<Arc<str>>,
  default_document_language: Option<Arc<str>>,
  resource_locale: OfficeResourceLocale,
}

impl OfficeLocaleContext {
  pub(crate) fn new(
    ui_language: Option<&str>,
    format_locale: Option<&str>,
    default_document_language: Option<&str>,
  ) -> Self {
    let ui_language = canonical_locale_tag(ui_language);
    let format_locale = canonical_locale_tag(format_locale).or_else(|| ui_language.clone());
    let default_document_language =
      canonical_locale_tag(default_document_language).or_else(|| ui_language.clone());
    let resource_locale = resolve_resource_locale(ui_language.as_deref());
    Self {
      ui_language,
      format_locale,
      default_document_language,
      resource_locale,
    }
  }

  pub(crate) fn ui_language(&self) -> Option<&str> {
    self.ui_language.as_deref()
  }

  pub(crate) fn ui_language_or_default(&self) -> &str {
    self.ui_language().unwrap_or("en-US")
  }

  pub(crate) fn format_locale(&self) -> Option<&str> {
    self.format_locale.as_deref()
  }

  pub(crate) fn default_document_language(&self) -> Option<&str> {
    self.default_document_language.as_deref()
  }

  pub(crate) fn default_document_resource_locale(&self) -> OfficeResourceLocale {
    resolve_resource_locale(self.default_document_language())
  }

  pub(crate) fn format_locale_is_simplified_chinese(&self) -> bool {
    let Some(locale) = self.format_locale().and_then(maximized_locale) else {
      return false;
    };
    locale.id.language.as_str() == "zh"
      && locale
        .id
        .script
        .is_some_and(|script| script.as_str() == "Hans")
  }

  pub(crate) fn resource_locale(&self) -> OfficeResourceLocale {
    self.resource_locale
  }

  pub(crate) fn strings(&self) -> &'static OfficeStringCatalog {
    OfficeStringCatalog::for_resource_locale(self.resource_locale)
  }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) enum OfficeResourceLocale {
  #[default]
  English,
  SimplifiedChinese,
  TraditionalChinese,
}

impl OfficeResourceLocale {
  pub(crate) fn is_simplified_chinese(self) -> bool {
    self == Self::SimplifiedChinese
  }

  pub(crate) fn is_chinese(self) -> bool {
    matches!(self, Self::SimplifiedChinese | Self::TraditionalChinese)
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ChartDisplayUnit {
  Hundreds,
  Thousands,
  TenThousands,
  HundredThousands,
  Millions,
  TenMillions,
  HundredMillions,
  Billions,
  Trillions,
}

/// Cross-crate resource needed by the PDF lowering layer.
///
/// This is public only because `ooxmlsdk-pdf` is a separate crate; callers
/// should select application locale through `LayoutOptions` or `PdfOptions`.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OfficeMissingLinkedImageResource {
  pub text: &'static str,
  pub font_family: &'static str,
}

#[doc(hidden)]
pub fn office_missing_linked_image_resource(
  ui_language: Option<&str>,
) -> OfficeMissingLinkedImageResource {
  let strings = OfficeStringCatalog::for_ui_language(ui_language);
  OfficeMissingLinkedImageResource {
    text: strings.missing_linked_image_text,
    font_family: strings.missing_linked_image_font_family,
  }
}

pub(crate) fn drawingml_theme_script(value: &str) -> Option<Arc<str>> {
  let locale = maximized_locale(value)?;
  // Office's theme vocabulary is mostly ISO 15924, but its documented LCID
  // mapping has three language-specific keys that differ from CLDR's likely
  // script: Korean `Hang` instead of aggregate `Kore`, Vietnamese `Viet`
  // instead of `Latn`, and Uyghur `Uigh` instead of `Arab`.
  match locale.id.language.as_str() {
    "ko" => Some(Arc::from("Hang")),
    "vi" => Some(Arc::from("Viet")),
    "ug" => Some(Arc::from("Uigh")),
    _ => locale.id.script.map(|script| Arc::from(script.as_str())),
  }
}

fn maximized_locale(value: &str) -> Option<Locale> {
  let mut locale = canonical_locale(value)?;
  LocaleExpander::new_extended().maximize(&mut locale.id);
  Some(locale)
}

/// Compile-time-complete Office application string pack.
///
/// These are application-generated resources, never replacements for
/// explicit OOXML text or captions. Adding a locale requires initializing
/// every field, which keeps missing keys visible during review.
#[derive(Clone, Copy, Debug)]
pub(crate) struct OfficeStringCatalog {
  resource_locale: OfficeResourceLocale,
  chart_title: &'static str,
  chart_axis_title: &'static str,
  chart_series: &'static str,
  chart_row: &'static str,
  chart_column: &'static str,
  waterfall_increase: &'static str,
  waterfall_decrease: &'static str,
  waterfall_total: &'static str,
  chart_display_units: [&'static str; 9],
  field_undefined_bookmark: &'static str,
  field_reference_source_not_found: &'static str,
  field_bookmark_name_not_specified: &'static str,
  field_empty_table_of_contents: &'static str,
  field_above: &'static str,
  field_below: &'static str,
  missing_linked_image_text: &'static str,
  missing_linked_image_font_family: &'static str,
}

impl OfficeStringCatalog {
  pub(crate) fn for_ui_language(ui_language: Option<&str>) -> &'static Self {
    Self::for_resource_locale(resolve_resource_locale(ui_language))
  }

  pub(crate) fn for_resource_locale(locale: OfficeResourceLocale) -> &'static Self {
    match locale {
      OfficeResourceLocale::English => &OFFICE_STRINGS_EN,
      OfficeResourceLocale::SimplifiedChinese => &OFFICE_STRINGS_ZH_HANS,
      OfficeResourceLocale::TraditionalChinese => &OFFICE_STRINGS_ZH_HANT,
    }
  }

  pub(crate) fn resource_locale(self) -> OfficeResourceLocale {
    self.resource_locale
  }

  pub(crate) fn chart_title(self) -> &'static str {
    self.chart_title
  }

  pub(crate) fn chart_axis_title(self) -> &'static str {
    self.chart_axis_title
  }

  pub(crate) fn chart_series_title(self, index: usize) -> String {
    format!("{} {index}", self.chart_series)
  }

  pub(crate) fn chart_row_title(self, index: usize) -> String {
    format!("{} {index}", self.chart_row)
  }

  pub(crate) fn chart_column_title(self, index: usize) -> String {
    format!("{} {index}", self.chart_column)
  }

  pub(crate) fn waterfall_legend(self) -> [&'static str; 3] {
    [
      self.waterfall_increase,
      self.waterfall_decrease,
      self.waterfall_total,
    ]
  }

  pub(crate) fn chart_display_unit(self, unit: ChartDisplayUnit) -> &'static str {
    self.chart_display_units[unit as usize]
  }

  pub(crate) fn field_undefined_bookmark(self) -> &'static str {
    self.field_undefined_bookmark
  }

  pub(crate) fn field_reference_source_not_found(self) -> &'static str {
    self.field_reference_source_not_found
  }

  pub(crate) fn field_bookmark_name_not_specified(self) -> &'static str {
    self.field_bookmark_name_not_specified
  }

  pub(crate) fn field_empty_table_of_contents(self) -> &'static str {
    self.field_empty_table_of_contents
  }

  pub(crate) fn field_above(self) -> &'static str {
    self.field_above
  }

  pub(crate) fn field_below(self) -> &'static str {
    self.field_below
  }

  pub(crate) fn field_missing_style(self, style_name: &str) -> String {
    match self.resource_locale {
      OfficeResourceLocale::English => format!(
        "Error! Use the Home tab to apply {style_name} to the text that you want to appear here."
      ),
      OfficeResourceLocale::SimplifiedChinese => {
        format!("错误!使用“开始”选项卡将 {style_name} 应用于要在此处显示的文字。")
      }
      OfficeResourceLocale::TraditionalChinese => {
        format!("錯誤!請使用 [常用] 索引標籤將 {style_name} 套用到您想要在此顯示的文字。")
      }
    }
  }

  pub(crate) fn field_on_page(self, page_number: &str) -> String {
    match self.resource_locale {
      OfficeResourceLocale::English => format!("on page {page_number}"),
      OfficeResourceLocale::SimplifiedChinese => format!("第 {page_number} 页"),
      OfficeResourceLocale::TraditionalChinese => format!("第 {page_number} 頁"),
    }
  }
}

const OFFICE_STRINGS_EN: OfficeStringCatalog = OfficeStringCatalog {
  resource_locale: OfficeResourceLocale::English,
  chart_title: "Chart Title",
  chart_axis_title: "Axis Title",
  chart_series: "Series",
  chart_row: "Row",
  chart_column: "Column",
  waterfall_increase: "Increase",
  waterfall_decrease: "Decrease",
  waterfall_total: "Total",
  chart_display_units: [
    "Hundreds",
    "Thousands",
    "Ten Thousands",
    "Hundred Thousands",
    "Millions",
    "Ten Millions",
    "Hundred Millions",
    "Billions",
    "Trillions",
  ],
  field_undefined_bookmark: "Error! Bookmark not defined.",
  field_reference_source_not_found: "Error! Reference source not found.",
  field_bookmark_name_not_specified: "Error! No bookmark name given.",
  field_empty_table_of_contents: "No table of contents entries found.",
  field_above: "above",
  field_below: "below",
  missing_linked_image_text: "The linked image cannot be displayed. The file may have been moved, renamed, or deleted. Verify that the link points to the correct file and location.",
  missing_linked_image_font_family: "Arial",
};

const OFFICE_STRINGS_ZH_HANS: OfficeStringCatalog = OfficeStringCatalog {
  resource_locale: OfficeResourceLocale::SimplifiedChinese,
  chart_title: "图表标题",
  chart_axis_title: "坐标轴标题",
  chart_series: "系列",
  chart_row: "行",
  chart_column: "列",
  waterfall_increase: "增加",
  waterfall_decrease: "减少",
  waterfall_total: "汇总",
  chart_display_units: [
    "百", "千", "万", "十万", "百万", "千万", "亿", "十亿", "万亿",
  ],
  field_undefined_bookmark: "错误!未定义书签。",
  field_reference_source_not_found: "错误!未找到引用源。",
  field_bookmark_name_not_specified: "错误!未指定书签。",
  field_empty_table_of_contents: "错误!未找到目录项。",
  field_above: "上方",
  field_below: "下方",
  missing_linked_image_text: "无法显示链接的图像。该文件可能已被移动、重命名或删除。请验证该链接是否指向正确的文件和位置。",
  missing_linked_image_font_family: "SimSun",
};

const OFFICE_STRINGS_ZH_HANT: OfficeStringCatalog = OfficeStringCatalog {
  resource_locale: OfficeResourceLocale::TraditionalChinese,
  chart_title: "圖表標題",
  chart_axis_title: "座標軸標題",
  chart_series: "數列",
  chart_row: "列",
  chart_column: "欄",
  waterfall_increase: "增加",
  waterfall_decrease: "減少",
  waterfall_total: "總計",
  // The retained fixed-output evidence establishes only the existing Chinese
  // display-unit spellings. Keep those exact resources until a Traditional
  // Chinese Office sample establishes a distinct pack.
  chart_display_units: OFFICE_STRINGS_ZH_HANS.chart_display_units,
  field_undefined_bookmark: "錯誤！ 書籤未定義。",
  field_reference_source_not_found: "錯誤! 找不到參照來源。",
  field_bookmark_name_not_specified: "錯誤! 未提供書籤名稱。",
  field_empty_table_of_contents: "錯誤! 找不到目錄項目。",
  field_above: "上面",
  field_below: "下面",
  // No Traditional Chinese Office fixed-output sample establishes this
  // resource yet. Fall back to the complete English resource instead of
  // inventing a translation.
  missing_linked_image_text: OFFICE_STRINGS_EN.missing_linked_image_text,
  missing_linked_image_font_family: OFFICE_STRINGS_EN.missing_linked_image_font_family,
};

fn canonical_locale_tag(value: Option<&str>) -> Option<Arc<str>> {
  canonical_locale(value?).map(|locale| Arc::from(locale.to_string()))
}

pub(crate) fn canonical_locale(value: &str) -> Option<Locale> {
  let value = value.trim();
  if value.is_empty() {
    return None;
  }
  let normalized = value.replace('_', "-");
  let mut locale = normalized.parse::<Locale>().ok()?;
  LocaleCanonicalizer::new_extended().canonicalize(&mut locale);
  Some(locale)
}

/// Canonical BCP 47 document language shared with the PDF lowering crate.
///
/// This is public only because `ooxmlsdk-pdf` is a separate crate. Office
/// resource selection remains internal to the layout layer.
#[doc(hidden)]
pub fn canonical_office_locale_tag(value: Option<&str>) -> Option<String> {
  canonical_locale_tag(value).map(|locale| locale.to_string())
}

fn resolve_resource_locale(ui_language: Option<&str>) -> OfficeResourceLocale {
  let Some(locale) =
    canonical_locale_tag(ui_language).and_then(|language| language.parse::<Locale>().ok())
  else {
    return OfficeResourceLocale::English;
  };
  let fallbacker = LocaleFallbacker::new();
  let mut fallback = fallbacker
    .for_config(Default::default())
    .fallback_for(DataLocale::from(locale));
  loop {
    let locale = fallback.get();
    match locale.language.as_str() {
      "en" => return OfficeResourceLocale::English,
      "zh" => {
        if let Some(script) = locale.script {
          match script.as_str() {
            "Hant" => return OfficeResourceLocale::TraditionalChinese,
            "Hans" => return OfficeResourceLocale::SimplifiedChinese,
            _ => {}
          }
        } else {
          if locale
            .region
            .is_some_and(|region| matches!(region.as_str(), "TW" | "HK" | "MO"))
          {
            return OfficeResourceLocale::TraditionalChinese;
          }
          if locale
            .region
            .is_some_and(|region| matches!(region.as_str(), "CN" | "SG"))
            || locale.region.is_none()
          {
            return OfficeResourceLocale::SimplifiedChinese;
          }
        }
      }
      "und" => return OfficeResourceLocale::English,
      _ => {}
    }
    fallback.step();
  }
}
