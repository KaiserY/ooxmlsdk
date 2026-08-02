use ooxmlsdk::schemas::schemas_microsoft_com_office_drawing_2012_chart_style as cs;
use ooxmlsdk::schemas::schemas_microsoft_com_office_drawing_2014_chartex as cx;

use crate::model::{PageItem, TextStyle};
use crate::pptx::chart::ChartFrame;
use crate::render::chartex::{
  self as shared_chartex, ChartExHost, ChartExRenderOptions, ChartExStyleResources, ChartExTheme,
  FormulaMatrix,
};

use super::import::ExcelImport;
use super::worksheet::{CellAddress, CellRange, CellRect};

pub(crate) fn lower_extended_chart(
  import: &ExcelImport,
  chart_space: &cx::ChartSpace,
  frame: CellRect,
  chart_styles: &[cs::ChartStyle],
  color_styles: &[cs::ColorStyle],
) -> Vec<PageItem> {
  let mut title_style = import.styles.default_chart_text_style();
  title_style.font_size_pt = 14.0;
  title_style.bold = true;
  let mut label_style = import.styles.default_chart_text_style();
  label_style.font_size_pt = 9.0;
  let ui_language = import.styles.output_ui_language();
  let mut resolver = |formula: &str| defined_matrix(import, formula);
  shared_chartex::lower_extended_chart(
    chart_space,
    ChartExRenderOptions {
      host: ChartExHost::Excel,
      frame: ChartFrame {
        x_pt: frame.x_pt,
        y_pt: frame.y_pt,
        width_pt: frame.width_pt,
        height_pt: frame.height_pt,
      },
      title_style,
      label_style,
      ui_language: Some(ui_language),
      theme: xlsx_theme(import),
      resources: ChartExStyleResources {
        chart_styles,
        color_styles,
      },
    },
    Some(&mut resolver),
  )
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn lower_extended_chart_cached_with_resources(
  chart_space: &cx::ChartSpace,
  frame: ChartFrame,
  title_style: TextStyle,
  label_style: TextStyle,
  ui_language: Option<&str>,
  host: ChartExHost,
  theme: ChartExTheme,
  chart_styles: &[cs::ChartStyle],
  color_styles: &[cs::ColorStyle],
) -> Vec<PageItem> {
  shared_chartex::lower_extended_chart(
    chart_space,
    ChartExRenderOptions {
      host,
      frame,
      title_style,
      label_style,
      ui_language,
      theme,
      resources: ChartExStyleResources {
        chart_styles,
        color_styles,
      },
    },
    None,
  )
}

fn xlsx_theme(import: &ExcelImport) -> ChartExTheme {
  let defaults = ChartExTheme::default();
  ChartExTheme {
    light1: import.styles.theme_color(0, 0.0).unwrap_or(defaults.light1),
    dark1: import.styles.theme_color(1, 0.0).unwrap_or(defaults.dark1),
    light2: import.styles.theme_color(2, 0.0).unwrap_or(defaults.light2),
    dark2: import.styles.theme_color(3, 0.0).unwrap_or(defaults.dark2),
    accents: std::array::from_fn(|index| {
      import
        .styles
        .theme_color(4 + index as u32, 0.0)
        .unwrap_or(defaults.accents[index])
    }),
    hyperlink: import
      .styles
      .theme_color(10, 0.0)
      .unwrap_or(defaults.hyperlink),
    followed_hyperlink: import
      .styles
      .theme_color(11, 0.0)
      .unwrap_or(defaults.followed_hyperlink),
  }
}

fn defined_matrix(import: &ExcelImport, name_or_formula: &str) -> Option<FormulaMatrix> {
  let formula = import
    .defined_names
    .records
    .iter()
    .find(|record| record.name == name_or_formula)
    .map_or(name_or_formula, |record| record.formula.as_str());
  let sheet_name = formula
    .rsplit_once('!')
    .map(|(sheet, _)| sheet.trim_matches('\''));
  let sheet = sheet_name
    .and_then(|name| import.sheets.iter().find(|sheet| sheet.name == name))
    .or_else(|| import.sheets.first())?;
  let range = CellRange::parse_a1_range(formula)?;
  let rows = (range.start.row..=range.end.row)
    .map(|row| {
      (range.start.col..=range.end.col)
        .map(|col| {
          sheet
            .cell_at(CellAddress { col, row })
            .map(|cell| {
              cell
                .cached_value
                .clone()
                .unwrap_or_else(|| cell.display_text.clone())
            })
            .unwrap_or_default()
        })
        .collect()
    })
    .collect();
  Some(FormulaMatrix { rows })
}
