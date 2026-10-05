//! Word separated-cell coordinates and their independent rounding owners.
use super::*;

pub(super) fn border_width(border: Option<BorderStyle>) -> f32 {
  border.map_or(0.0, |border| (border.width_pt * 20.0).floor() / 20.0)
}

fn device_coordinate(pt: f32) -> i64 {
  // The layout reference point is integral before MulDiv to the printer.
  // In particular, 56.7pt lies just below a half-dot after this conversion.
  wordprocessing_table_device::source_coordinate(pt)
}

pub(super) struct RowVertical {
  content_top: i64,
  content_bottom: i64,
}

impl RowVertical {
  pub(super) fn new(bounds: (f32, f32), spacing: (f32, f32), insets: (f32, f32)) -> Self {
    // Word's row painter receives positioned endpoints and independent full
    // top/bottom insets. Its cell painter then removes each margin and border
    // separately. Native independent spacing/margin/edge controls distinguish
    // this ownership from rounding the final floating-point cell rectangle.
    Self {
      content_top: device_coordinate(bounds.0 - spacing.0)
        + device_coordinate(spacing.0 + insets.0),
      content_bottom: device_coordinate(bounds.1 + spacing.1)
        - device_coordinate(spacing.1 + insets.1),
    }
  }

  pub(super) fn cell_edges(
    &self,
    margins: (f32, f32),
    borders: crate::model::CellBordersModel,
  ) -> (f32, f32) {
    (
      (self.content_top
        - device_coordinate(margins.0)
        - device_coordinate(border_width(borders.top))) as f32
        * 0.12,
      (self.content_bottom
        + device_coordinate(margins.1)
        + device_coordinate(border_width(borders.bottom))) as f32
        * 0.12,
    )
  }
}

pub(super) struct Frame {
  pub grid_left: f32,
  pub grid_width: f32,
  pub available_width: f32,
}

pub(super) fn text_left(
  table: &Table,
  row: &TableRow,
  cell: &TableCell,
  frame: Frame,
  cell_left: f32,
) -> Option<f32> {
  if !row_has_separate_borders(table, row) {
    return None;
  }
  let first_row = table.rows.first()?;
  let first = first_row.cells.first()?;
  let last = first_row.cells.last()?;
  let border_twips = |border: Option<BorderStyle>| {
    border.map_or(0, |border| {
      (f64::from(border.width_pt) * 20.0).floor() as i64
    })
  };
  // Word's separated-table frame includes the leading/trailing border
  // halves. Its twip grid gives the odd twip to the leading half. bidiVisual
  // uses the logical first/last start borders; our model is already mirrored.
  // Native independent-edge controls distinguish this from simply adding a
  // half-width paint offset to a grid-only centered table.
  let (leading, trailing) = if table.right_to_left {
    (
      border_twips(last.borders.right),
      border_twips(first.borders.right),
    )
  } else {
    (
      border_twips(first.borders.left),
      border_twips(last.borders.right),
    )
  };
  let leading_half = (leading + 1) / 2;
  let trailing_half = trailing / 2;
  let alignment: f64 = match table.alignment {
    TableAlignment::Left => 0.0,
    TableAlignment::Center => 0.5,
    TableAlignment::Right => 1.0,
  };
  let ideal = |twips: i64| (twips as f64 * 4096.0 / 20.0).round() as i64;
  let grid_width = (f64::from(frame.grid_width) * 20.0).round() as i64;
  let available = (f64::from(frame.available_width) * 4096.0).round() as i64;
  let remaining = available - ideal(grid_width + leading_half + trailing_half);
  let unaligned = f64::from(frame.grid_left)
    - alignment * (f64::from(frame.available_width) - f64::from(frame.grid_width));
  // Convert the owner and the complete frame width independently, then align
  // in integer layout units. Rounding their difference instead loses a unit
  // at right alignment. Native over-wide controls also require truncating
  // a negative odd centered remainder toward zero.
  let parent = (unaligned * 4096.0).round() as i64
    + match table.alignment {
      TableAlignment::Left => 0,
      TableAlignment::Center => remaining / 2,
      TableAlignment::Right => remaining,
    };
  let local_grid = ((f64::from(cell_left) - f64::from(frame.grid_left)) * 20.0).round() as i64;
  let margin = (f64::from(cell.margins.left_pt) * 20.0).round() as i64;
  let local_text = if table.right_to_left { 0 } else { leading_half }
    + local_grid
    + border_twips(cell.borders.left)
    + margin;
  // The cell coordinate helper converts its complete text edge before the
  // frame subtracts the margin. Zero spacing then charges the logical start
  // half-border to that margin, clamped at zero. Keep these conversions
  // separate: an eight-twip margin distinguishes their rounding by 1/4096pt.
  let text_margin = if row_cell_spacing_pt(table, row) == 0.0 {
    let border = if table.right_to_left {
      cell.borders.right
    } else {
      cell.borders.left
    };
    (margin - border_twips(border) / 2).max(0)
  } else {
    margin
  };
  Some((parent + ideal(local_text) - ideal(margin) + ideal(text_margin)) as f32 / 4096.0)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn separated_row_vertical_owners_match_native_word_coordinates() {
    // Native row rectangles from 20 independent Office controls / 60 rows.
    // Inputs are logical point bounds after the authored borders are lowered
    // to twips. Cases independently vary zero/fractional/large spacing, both
    // margins, both cell edges and both outside frame edges.
    for (bounds, spacing, insets, expected) in [
      (
        (57.45, 73.877_73),
        (0.750000, 0.000000),
        (1.5, 1.5),
        (491, 603),
      ),
      (
        (73.877_73, 90.305466),
        (0.000000, 0.000000),
        (1.5, 1.5),
        (629, 740),
      ),
      (
        (90.305466, 106.733_2),
        (0.000000, 0.750000),
        (1.5, 1.5),
        (766, 877),
      ),
      (
        (57.55, 73.977_74),
        (0.850000, 0.050000),
        (1.5, 1.5),
        (492, 604),
      ),
      (
        (74.077736, 90.505_47),
        (0.050000, 0.050000),
        (1.5, 1.5),
        (630, 742),
      ),
      (
        (90.605_47, 107.033_2),
        (0.050000, 0.850000),
        (1.5, 1.5),
        (768, 879),
      ),
      ((58.95, 75.377_73), (2.25, 0.750000), (1.5, 1.5), (503, 615)),
      (
        (76.877_73, 93.305466),
        (0.750000, 0.750000),
        (1.5, 1.5),
        (653, 765),
      ),
      (
        (94.805466, 111.233_2),
        (0.750000, 2.25),
        (1.5, 1.5),
        (803, 915),
      ),
      ((63.45, 79.877_73), (6.75, 3.000000), (1.5, 1.5), (541, 653)),
      (
        (85.877_73, 102.305466),
        (3.000000, 3.000000),
        (1.5, 1.5),
        (729, 840),
      ),
      (
        (108.305466, 124.733_2),
        (3.000000, 6.75),
        (1.5, 1.5),
        (916, 1027),
      ),
      (
        (58.95, 73.877_73),
        (2.25, 0.750000),
        (0.750000, 0.750000),
        (497, 609),
      ),
      (
        (75.377_73, 90.305466),
        (0.750000, 0.750000),
        (0.750000, 0.750000),
        (635, 746),
      ),
      (
        (91.805466, 106.733_2),
        (0.750000, 2.25),
        (0.750000, 0.750000),
        (772, 883),
      ),
      (
        (58.95, 74.327736),
        (2.25, 0.750000),
        (0.800000, 1.15),
        (497, 610),
      ),
      (
        (75.827736, 91.205_47),
        (0.750000, 0.750000),
        (0.800000, 1.15),
        (639, 750),
      ),
      (
        (92.705_47, 108.083_2),
        (0.750000, 2.25),
        (0.800000, 1.15),
        (779, 891),
      ),
      (
        (58.95, 74.327736),
        (2.25, 0.750000),
        (1.15, 0.800000),
        (500, 613),
      ),
      (
        (75.827736, 91.205_47),
        (0.750000, 0.750000),
        (1.15, 0.800000),
        (642, 753),
      ),
      (
        (92.705_47, 108.083_2),
        (0.750000, 2.25),
        (1.15, 0.800000),
        (782, 894),
      ),
      (
        (58.95, 82.127_73),
        (2.25, 0.750000),
        (8.25, 1.5),
        (560, 672),
      ),
      (
        (83.627_73, 106.805466),
        (0.750000, 0.750000),
        (8.25, 1.5),
        (766, 877),
      ),
      (
        (108.305466, 131.483_2),
        (0.750000, 2.25),
        (8.25, 1.5),
        (971, 1083),
      ),
      (
        (58.95, 82.127_73),
        (2.25, 0.750000),
        (1.5, 8.25),
        (503, 616),
      ),
      (
        (83.627_73, 106.805466),
        (0.750000, 0.750000),
        (1.5, 8.25),
        (710, 821),
      ),
      (
        (108.305466, 131.483_2),
        (0.750000, 2.25),
        (1.5, 8.25),
        (915, 1026),
      ),
      (
        (58.95, 88.877_73),
        (2.25, 0.750000),
        (8.25, 8.25),
        (560, 672),
      ),
      (
        (90.377_73, 120.305466),
        (0.750000, 0.750000),
        (8.25, 8.25),
        (822, 934),
      ),
      (
        (121.805466, 151.733_2),
        (0.750000, 2.25),
        (8.25, 8.25),
        (1084, 1195),
      ),
      (
        (58.95, 74.077_73),
        (2.25, 0.750000),
        (0.850000, 0.850000),
        (498, 611),
      ),
      (
        (75.577_73, 90.705_47),
        (0.750000, 0.750000),
        (0.850000, 0.850000),
        (637, 749),
      ),
      (
        (92.205_47, 107.333_2),
        (0.750000, 2.25),
        (0.850000, 0.850000),
        (775, 887),
      ),
      (
        (58.95, 76.977_73),
        (2.25, 0.750000),
        (0.850000, 3.75),
        (498, 610),
      ),
      (
        (78.477_73, 96.505_46),
        (0.750000, 0.750000),
        (0.850000, 3.75),
        (661, 772),
      ),
      (
        (98.005_46, 116.033_2),
        (0.750000, 2.25),
        (0.850000, 3.75),
        (823, 936),
      ),
      (
        (58.95, 76.977_73),
        (2.25, 0.750000),
        (3.75, 0.850000),
        (522, 635),
      ),
      (
        (78.477_73, 96.505_46),
        (0.750000, 0.750000),
        (3.75, 0.850000),
        (686, 797),
      ),
      (
        (98.005_46, 116.033_2),
        (0.750000, 2.25),
        (3.75, 0.850000),
        (848, 960),
      ),
      (
        (58.95, 79.877_73),
        (2.25, 0.750000),
        (3.75, 3.75),
        (522, 634),
      ),
      (
        (81.377_73, 102.305466),
        (0.750000, 0.750000),
        (3.75, 3.75),
        (710, 821),
      ),
      (
        (103.805466, 124.733_2),
        (0.750000, 2.25),
        (3.75, 3.75),
        (897, 1008),
      ),
      ((58.3, 74.727_73), (1.6, 0.750000), (1.5, 1.5), (498, 610)),
      (
        (76.227_73, 92.655466),
        (0.750000, 0.750000),
        (1.5, 1.5),
        (648, 759),
      ),
      (
        (94.155466, 110.583_2),
        (0.750000, 4.5),
        (1.5, 1.5),
        (797, 909),
      ),
      ((61.2, 77.627_73), (4.5, 0.750000), (1.5, 1.5), (522, 634)),
      (
        (79.127_73, 95.555466),
        (0.750000, 0.750000),
        (1.5, 1.5),
        (672, 784),
      ),
      (
        (97.055466, 113.483_2),
        (0.750000, 1.6),
        (1.5, 1.5),
        (822, 933),
      ),
      (
        (59.7, 76.677736),
        (3.000000, 0.000000),
        (0.150000, 3.4),
        (498, 611),
      ),
      (
        (76.677736, 93.655_47),
        (0.000000, 0.000000),
        (0.150000, 3.4),
        (640, 752),
      ),
      (
        (93.655_47, 110.633_21),
        (0.000000, 0.100000),
        (0.150000, 3.4),
        (781, 894),
      ),
      (
        (62.8, 79.777_73),
        (6.1, 3.000000),
        (3.4, 0.150000),
        (551, 664),
      ),
      (
        (85.777_73, 102.755_48),
        (3.000000, 3.000000),
        (3.4, 0.150000),
        (743, 855),
      ),
      (
        (108.755_48, 125.733_21),
        (3.000000, 9.000000),
        (3.4, 0.150000),
        (934, 1047),
      ),
      (
        (57.55, 82.227_74),
        (0.850000, 0.050000),
        (8.25, 3.000000),
        (548, 661),
      ),
      (
        (82.327736, 107.005_47),
        (0.050000, 0.050000),
        (8.25, 3.000000),
        (755, 867),
      ),
      (
        (107.105_47, 131.783_2),
        (0.050000, 0.200000),
        (8.25, 3.000000),
        (961, 1073),
      ),
      (
        (61.2, 85.877_73),
        (4.5, 0.750000),
        (3.000000, 8.25),
        (535, 647),
      ),
      (
        (87.377_73, 112.055466),
        (0.750000, 0.750000),
        (3.000000, 8.25),
        (753, 865),
      ),
      (
        (113.555466, 138.233_2),
        (0.750000, 2.25),
        (3.000000, 8.25),
        (971, 1083),
      ),
    ] {
      let actual = RowVertical::new(bounds, spacing, insets);
      assert_eq!(
        (actual.content_top, actual.content_bottom),
        expected,
        "bounds={bounds:?}, spacing={spacing:?}, insets={insets:?}"
      );
    }
  }

  #[test]
  fn separated_cell_text_origins_match_native_word_coordinates() {
    let blocks = crate::docx::html::import_blocks(
      "<table><tr><td>Left</td><td>Right</td></tr></table>",
      false,
    );
    let Block::Table(template) = &blocks[0] else {
      panic!("table")
    };
    // Integer 1/4096pt points observed before Word's 600-DPI conversion.
    // Authored 400pt grid, 200pt columns; edge widths are in physical order.
    // Expectations include independent first/last edges, RTL, fractional-twip
    // border widths and a margin straddling the zero-spacing border charge.
    for (rtl, alignment, edges, gap, margin, grid_left, expected) in [
      (
        false,
        TableAlignment::Right,
        [0.75, 0.75, 0.75, 3.0],
        1.5,
        7.5,
        152.8,
        [659661, 1475789],
      ),
      (
        false,
        TableAlignment::Left,
        [0.750, 0.750, 0.750, 0.750],
        1.50,
        7.50,
        85.050,
        [389939, 1206067],
      ),
      (
        false,
        TableAlignment::Left,
        [0.750, 0.750, 0.750, 0.750],
        1.50,
        7.50,
        85.150,
        [390348, 1206476],
      ),
      (
        false,
        TableAlignment::Left,
        [0.750, 0.750, 0.750, 0.750],
        1.50,
        7.50,
        85.500,
        [391782, 1207910],
      ),
      (
        false,
        TableAlignment::Left,
        [3.000, 0.750, 3.000, 0.750],
        0.00,
        7.50,
        85.050,
        [391373, 1210573],
      ),
      (
        false,
        TableAlignment::Left,
        [0.750, 0.750, 0.750, 0.750],
        0.00,
        7.50,
        85.050,
        [382361, 1201561],
      ),
      (
        false,
        TableAlignment::Left,
        [3.000, 0.750, 3.000, 0.750],
        1.50,
        7.50,
        85.050,
        [403661, 1219789],
      ),
      (
        false,
        TableAlignment::Left,
        [0.750, 3.000, 0.750, 3.000],
        1.50,
        7.50,
        85.050,
        [389939, 1206067],
      ),
      (
        true,
        TableAlignment::Center,
        [3.000, 3.000, 3.000, 3.000],
        0.00,
        7.50,
        118.925,
        [517837, 1337037],
      ),
      (
        true,
        TableAlignment::Center,
        [0.750, 0.750, 0.750, 0.750],
        0.00,
        7.50,
        118.925,
        [517939, 1337139],
      ),
      (
        true,
        TableAlignment::Right,
        [0.750, 3.000, 0.750, 3.000],
        0.00,
        7.50,
        152.800,
        [641229, 1460429],
      ),
      (
        false,
        TableAlignment::Center,
        [3.000, 0.750, 3.000, 0.750],
        0.00,
        7.50,
        118.925,
        [526336, 1345536],
      ),
      (
        false,
        TableAlignment::Right,
        [3.000, 0.750, 3.000, 0.750],
        0.00,
        7.50,
        152.800,
        [661299, 1480499],
      ),
      (
        true,
        TableAlignment::Right,
        [0.750, 3.000, 0.750, 3.000],
        1.50,
        7.50,
        152.800,
        [653517, 1469645],
      ),
      (
        false,
        TableAlignment::Center,
        [3.000, 0.750, 3.000, 0.750],
        1.50,
        7.50,
        118.925,
        [538624, 1354752],
      ),
      (
        false,
        TableAlignment::Right,
        [3.000, 0.750, 3.000, 0.750],
        1.50,
        7.50,
        152.800,
        [673587, 1489715],
      ),
      (
        false,
        TableAlignment::Center,
        [3.000, 0.750, 0.750, 0.750],
        1.50,
        7.50,
        118.925,
        [538624, 1345536],
      ),
      (
        false,
        TableAlignment::Center,
        [0.750, 3.000, 0.750, 0.750],
        1.50,
        7.50,
        118.925,
        [527155, 1343283],
      ),
      (
        false,
        TableAlignment::Center,
        [0.750, 0.750, 3.000, 0.750],
        1.50,
        7.50,
        118.925,
        [527155, 1352499],
      ),
      (
        false,
        TableAlignment::Center,
        [0.750, 0.750, 0.750, 3.000],
        1.50,
        7.50,
        118.925,
        [524800, 1340928],
      ),
      (
        true,
        TableAlignment::Center,
        [0.750, 0.750, 0.750, 3.000],
        1.50,
        7.50,
        118.925,
        [523264, 1339392],
      ),
      (
        true,
        TableAlignment::Center,
        [0.750, 0.750, 3.000, 0.750],
        1.50,
        7.50,
        118.925,
        [525517, 1350861],
      ),
      (
        true,
        TableAlignment::Center,
        [0.750, 3.000, 0.750, 0.750],
        1.50,
        7.50,
        118.925,
        [523162, 1339290],
      ),
      (
        true,
        TableAlignment::Center,
        [3.000, 0.750, 0.750, 0.750],
        1.50,
        7.50,
        118.925,
        [534733, 1341645],
      ),
      (
        false,
        TableAlignment::Center,
        [0.125, 0.125, 0.125, 0.125],
        0.00,
        7.50,
        118.925,
        [518041, 1337241],
      ),
      (
        false,
        TableAlignment::Center,
        [0.125, 0.125, 0.125, 0.125],
        1.50,
        7.50,
        118.925,
        [524390, 1340518],
      ),
      (
        false,
        TableAlignment::Center,
        [0.250, 0.250, 0.250, 0.250],
        0.00,
        7.50,
        118.925,
        [518553, 1337753],
      ),
      (
        false,
        TableAlignment::Center,
        [0.250, 0.250, 0.250, 0.250],
        1.50,
        7.50,
        118.925,
        [525107, 1341235],
      ),
      (
        false,
        TableAlignment::Center,
        [0.375, 0.375, 0.375, 0.375],
        0.00,
        7.50,
        118.925,
        [518759, 1337959],
      ),
      (
        false,
        TableAlignment::Center,
        [0.375, 0.375, 0.375, 0.375],
        1.50,
        7.50,
        118.925,
        [525517, 1341645],
      ),
      (
        false,
        TableAlignment::Center,
        [0.625, 0.625, 0.625, 0.625],
        0.00,
        7.50,
        118.925,
        [519065, 1338265],
      ),
      (
        false,
        TableAlignment::Center,
        [0.625, 0.625, 0.625, 0.625],
        1.50,
        7.50,
        118.925,
        [526438, 1342566],
      ),
      (
        false,
        TableAlignment::Center,
        [0.875, 0.875, 0.875, 0.875],
        0.00,
        7.50,
        118.925,
        [519783, 1338983],
      ),
      (
        false,
        TableAlignment::Center,
        [0.875, 0.875, 0.875, 0.875],
        1.50,
        7.50,
        118.925,
        [527565, 1343693],
      ),
      (
        false,
        TableAlignment::Center,
        [1.125, 1.125, 1.125, 1.125],
        0.00,
        7.50,
        118.925,
        [520089, 1339289],
      ),
      (
        false,
        TableAlignment::Center,
        [1.125, 1.125, 1.125, 1.125],
        1.50,
        7.50,
        118.925,
        [528486, 1344614],
      ),
      (
        false,
        TableAlignment::Center,
        [1.500, 1.500, 1.500, 1.500],
        0.00,
        7.50,
        118.925,
        [520909, 1340109],
      ),
      (
        false,
        TableAlignment::Center,
        [1.500, 1.500, 1.500, 1.500],
        1.50,
        7.50,
        118.925,
        [530125, 1346253],
      ),
      (
        false,
        TableAlignment::Center,
        [2.250, 2.250, 2.250, 2.250],
        0.00,
        7.50,
        118.925,
        [522649, 1341849],
      ),
      (
        false,
        TableAlignment::Center,
        [2.250, 2.250, 2.250, 2.250],
        1.50,
        7.50,
        118.925,
        [533299, 1349427],
      ),
      (
        false,
        TableAlignment::Center,
        [0.750, 0.750, 0.750, 0.750],
        0.00,
        0.00,
        118.925,
        [490291, 1309491],
      ),
      (
        false,
        TableAlignment::Center,
        [0.750, 0.750, 0.750, 0.750],
        0.00,
        0.05,
        118.925,
        [490291, 1309491],
      ),
      (
        false,
        TableAlignment::Center,
        [0.750, 0.750, 0.750, 0.750],
        0.00,
        0.30,
        118.925,
        [490291, 1309491],
      ),
      (
        false,
        TableAlignment::Center,
        [0.750, 0.750, 0.750, 0.750],
        0.00,
        0.40,
        118.925,
        [490497, 1309697],
      ),
      (
        false,
        TableAlignment::Center,
        [3.000, 3.000, 3.000, 3.000],
        0.00,
        0.00,
        118.925,
        [499405, 1318605],
      ),
      (
        false,
        TableAlignment::Center,
        [3.000, 3.000, 3.000, 3.000],
        0.00,
        0.40,
        118.925,
        [499405, 1318605],
      ),
      (
        true,
        TableAlignment::Center,
        [0.750, 0.750, 0.750, 0.750],
        0.00,
        0.00,
        118.925,
        [488653, 1307853],
      ),
      (
        true,
        TableAlignment::Center,
        [0.750, 0.750, 0.750, 0.750],
        1.50,
        0.00,
        118.925,
        [494797, 1310925],
      ),
      (
        true,
        TableAlignment::Center,
        [3.000, 3.000, 3.000, 3.000],
        0.00,
        0.00,
        118.925,
        [493261, 1312461],
      ),
      (
        true,
        TableAlignment::Center,
        [3.000, 3.000, 3.000, 3.000],
        1.50,
        0.00,
        118.925,
        [499405, 1315533],
      ),
    ] {
      let mut table = template.clone();
      table.right_to_left = rtl;
      table.alignment = alignment;
      table.cell_spacing = TableCellSpacing::Separated(gap / 2.0);
      for (index, cell) in table.rows[0].cells.iter_mut().enumerate() {
        cell.borders.left = Some(BorderStyle {
          width_pt: edges[index * 2],
          ..BorderStyle::default()
        });
        cell.borders.right = Some(BorderStyle {
          width_pt: edges[index * 2 + 1],
          ..BorderStyle::default()
        });
        cell.margins.left_pt = margin;
      }
      let row = &table.rows[0];
      for (index, cell) in row.cells.iter().enumerate() {
        let cell_left = grid_left + index as f32 * 200.0 + if index == 0 { gap } else { gap / 2.0 };
        let actual = text_left(
          &table,
          row,
          cell,
          Frame {
            grid_left,
            grid_width: 400.0,
            available_width: 467.75,
          },
          cell_left,
        )
        .unwrap();
        assert_eq!(
          (actual * 4096.0) as i32,
          expected[index],
          "rtl={rtl}, align={alignment:?}, edges={edges:?}, gap={gap}, margin={margin}, cell={index}"
        );
      }
    }
  }
}
