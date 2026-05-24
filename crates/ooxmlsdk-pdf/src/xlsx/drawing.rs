use ooxmlsdk::parts::chart_part::ChartPart;
use ooxmlsdk::parts::drawings_part::DrawingsPart;
use ooxmlsdk::parts::extended_chart_part::ExtendedChartPart;
use ooxmlsdk::parts::spreadsheet_document::SpreadsheetDocument;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct DrawingResourceCatalog {
  pub(crate) charts: Vec<ChartResourceCatalog>,
  pub(crate) extended_charts: Vec<ChartResourceCatalog>,
  pub(crate) diagram_data: usize,
  pub(crate) diagram_layouts: usize,
  pub(crate) diagram_styles: usize,
  pub(crate) diagram_colors: usize,
  pub(crate) diagram_drawings: usize,
  pub(crate) images: usize,
  pub(crate) custom_xml_parts: usize,
  pub(crate) web_extensions: usize,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct ChartResourceCatalog {
  pub(crate) has_chart_drawing: bool,
  pub(crate) has_embedded_package: bool,
  pub(crate) images: usize,
  pub(crate) has_theme_override: bool,
  pub(crate) styles: usize,
  pub(crate) color_styles: usize,
}

impl DrawingResourceCatalog {
  pub(crate) fn from_part(package: &mut SpreadsheetDocument, part: &DrawingsPart) -> Self {
    let chart_parts = part.chart_parts(package).collect::<Vec<_>>();
    let extended_chart_parts = part.extended_chart_parts(package).collect::<Vec<_>>();
    Self {
      charts: chart_parts
        .iter()
        .map(|chart| ChartResourceCatalog::from_chart_part(package, &chart))
        .collect(),
      extended_charts: extended_chart_parts
        .iter()
        .map(|chart| ChartResourceCatalog::from_extended_chart_part(package, &chart))
        .collect(),
      diagram_data: part.diagram_data_parts(package).count(),
      diagram_layouts: part.diagram_layout_definition_parts(package).count(),
      diagram_styles: part.diagram_style_parts(package).count(),
      diagram_colors: part.diagram_colors_parts(package).count(),
      diagram_drawings: part.diagram_persist_layout_parts(package).count(),
      images: part.image_parts(package).count(),
      custom_xml_parts: part.custom_xml_parts(package).count(),
      web_extensions: part.web_extension_parts(package).count(),
    }
  }

  pub(crate) fn chart_count(&self) -> usize {
    self.charts.len() + self.extended_charts.len()
  }

  pub(crate) fn diagram_resource_count(&self) -> usize {
    self.diagram_data
      + self.diagram_layouts
      + self.diagram_styles
      + self.diagram_colors
      + self.diagram_drawings
  }

  pub(crate) fn chart_child_resource_count(&self) -> usize {
    self
      .charts
      .iter()
      .chain(self.extended_charts.iter())
      .map(ChartResourceCatalog::child_resource_count)
      .sum()
  }
}

impl ChartResourceCatalog {
  pub(crate) fn from_chart_part(package: &mut SpreadsheetDocument, part: &ChartPart) -> Self {
    Self {
      has_chart_drawing: part.chart_drawing_part(package).is_some(),
      has_embedded_package: part.embedded_package_part(package).is_some(),
      images: part.image_parts(package).count(),
      has_theme_override: part.theme_override_part(package).is_some(),
      styles: part.chart_style_parts(package).count(),
      color_styles: part.chart_color_style_parts(package).count(),
    }
  }

  pub(crate) fn from_extended_chart_part(
    package: &mut SpreadsheetDocument,
    part: &ExtendedChartPart,
  ) -> Self {
    Self {
      has_chart_drawing: part.chart_drawing_part(package).is_some(),
      has_embedded_package: part.embedded_package_part(package).is_some(),
      images: part.image_parts(package).count(),
      has_theme_override: part.theme_override_part(package).is_some(),
      styles: part.chart_style_parts(package).count(),
      color_styles: part.chart_color_style_parts(package).count(),
    }
  }

  pub(crate) fn child_resource_count(&self) -> usize {
    usize::from(self.has_chart_drawing)
      + usize::from(self.has_embedded_package)
      + self.images
      + usize::from(self.has_theme_override)
      + self.styles
      + self.color_styles
  }
}
