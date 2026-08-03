use ooxmlsdk::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main as x;

#[derive(Clone, Copy, Debug)]
pub(super) struct PresetPivotStyle {
  kind: PresetPivotStyleKind,
}

#[derive(Clone, Copy, Debug)]
enum PresetPivotStyleKind {
  Light1,
  Light2 { accent: u32 },
  LightFramed { accent: Option<u32> },
  Light15,
  LightTinted { accent: u32 },
  Light22,
  LightOutline { accent: u32 },
  MediumAccent { accent: Option<u32> },
  MediumFramed { accent: Option<u32> },
  MediumGrid { accent: Option<u32> },
  MediumBand { accent: Option<u32> },
  Dark1,
  DarkAccent { accent: u32 },
  DarkFramed { accent: Option<u32> },
  DarkGrid { accent: Option<u32> },
  DarkBand { accent: Option<u32> },
}

#[derive(Clone, Copy, Debug)]
pub(super) struct PresetThemeColor {
  pub(super) theme: u32,
  pub(super) tint: f64,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum PresetBorderWeight {
  Thin,
  Medium,
  Double,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct PresetBorderLine {
  pub(super) color: PresetThemeColor,
  pub(super) weight: PresetBorderWeight,
}

#[derive(Clone, Copy, Debug, Default)]
pub(super) enum PresetBorderSide {
  #[default]
  Inherit,
  NoLine,
  Line(PresetBorderLine),
}

#[derive(Clone, Copy, Debug, Default)]
pub(super) struct PresetPivotBorders {
  pub(super) left: PresetBorderSide,
  pub(super) right: PresetBorderSide,
  pub(super) top: PresetBorderSide,
  pub(super) bottom: PresetBorderSide,
  pub(super) horizontal: PresetBorderSide,
  pub(super) vertical: PresetBorderSide,
}

#[derive(Clone, Copy, Debug, Default)]
pub(super) struct PresetPivotDifferential {
  pub(super) font_color: Option<PresetThemeColor>,
  pub(super) bold: bool,
  pub(super) fill: Option<PresetThemeColor>,
  pub(super) borders: PresetPivotBorders,
}

impl PresetPivotStyle {
  pub(super) fn from_name(name: &str) -> Option<Self> {
    let (family, number) = if let Some(number) = name.strip_prefix("PivotStyleLight") {
      ("light", number.parse::<u8>().ok()?)
    } else if let Some(number) = name.strip_prefix("PivotStyleMedium") {
      ("medium", number.parse::<u8>().ok()?)
    } else if let Some(number) = name.strip_prefix("PivotStyleDark") {
      ("dark", number.parse::<u8>().ok()?)
    } else {
      return None;
    };
    let kind = match (family, number) {
      ("light", 1) => PresetPivotStyleKind::Light1,
      ("light", 2..=7) => PresetPivotStyleKind::Light2 {
        accent: u32::from(number + 2),
      },
      ("light", 8) => PresetPivotStyleKind::LightFramed { accent: None },
      ("light", 9..=14) => PresetPivotStyleKind::LightFramed {
        accent: Some(u32::from(number - 5)),
      },
      ("light", 15) => PresetPivotStyleKind::Light15,
      ("light", 16..=21) => PresetPivotStyleKind::LightTinted {
        accent: u32::from(number - 12),
      },
      ("light", 22) => PresetPivotStyleKind::Light22,
      ("light", 23..=28) => PresetPivotStyleKind::LightOutline {
        accent: u32::from(number - 19),
      },
      ("medium", 1) => PresetPivotStyleKind::MediumAccent { accent: None },
      ("medium", 2..=7) => PresetPivotStyleKind::MediumAccent {
        accent: Some(u32::from(number + 2)),
      },
      ("medium", 8) => PresetPivotStyleKind::MediumFramed { accent: None },
      ("medium", 9..=14) => PresetPivotStyleKind::MediumFramed {
        accent: Some(u32::from(number - 5)),
      },
      ("medium", 15) => PresetPivotStyleKind::MediumGrid { accent: None },
      ("medium", 16..=21) => PresetPivotStyleKind::MediumGrid {
        accent: Some(u32::from(number - 12)),
      },
      ("medium", 22) => PresetPivotStyleKind::MediumBand { accent: None },
      ("medium", 23..=28) => PresetPivotStyleKind::MediumBand {
        accent: Some(u32::from(number - 19)),
      },
      ("dark", 1) => PresetPivotStyleKind::Dark1,
      ("dark", 2..=7) => PresetPivotStyleKind::DarkAccent {
        accent: u32::from(number + 2),
      },
      ("dark", 8) => PresetPivotStyleKind::DarkFramed { accent: None },
      ("dark", 9..=14) => PresetPivotStyleKind::DarkFramed {
        accent: Some(u32::from(number - 5)),
      },
      ("dark", 15) => PresetPivotStyleKind::DarkGrid { accent: None },
      ("dark", 16..=21) => PresetPivotStyleKind::DarkGrid {
        accent: Some(u32::from(number - 12)),
      },
      ("dark", 22) => PresetPivotStyleKind::DarkBand { accent: None },
      ("dark", 23..=28) => PresetPivotStyleKind::DarkBand {
        accent: Some(u32::from(number - 19)),
      },
      _ => return None,
    };
    Some(Self { kind })
  }

  pub(super) fn differential(
    self,
    element: x::TableStyleValues,
  ) -> Option<PresetPivotDifferential> {
    match self.kind {
      PresetPivotStyleKind::Light1 => light1_differential(element),
      PresetPivotStyleKind::Light2 { accent } => light2_differential(element, accent),
      PresetPivotStyleKind::LightFramed { accent } => light_framed_differential(element, accent),
      PresetPivotStyleKind::Light15 => light15_differential(element),
      PresetPivotStyleKind::LightTinted { accent } => light_tinted_differential(element, accent),
      PresetPivotStyleKind::Light22 => light22_differential(element),
      PresetPivotStyleKind::LightOutline { accent } => light_outline_differential(element, accent),
      PresetPivotStyleKind::MediumAccent { accent } => medium_accent_differential(element, accent),
      PresetPivotStyleKind::MediumFramed { accent } => medium_framed_differential(element, accent),
      PresetPivotStyleKind::MediumGrid { accent } => medium_grid_differential(element, accent),
      PresetPivotStyleKind::MediumBand { accent } => medium_band_differential(element, accent),
      PresetPivotStyleKind::Dark1 => dark1_differential(element),
      PresetPivotStyleKind::DarkAccent { accent } => dark_accent_differential(element, accent),
      PresetPivotStyleKind::DarkFramed { accent } => dark_framed_differential(element, accent),
      PresetPivotStyleKind::DarkGrid { accent } => dark_grid_differential(element, accent),
      PresetPivotStyleKind::DarkBand { accent } => dark_band_differential(element, accent),
    }
  }
}

impl PresetPivotDifferential {
  const fn new() -> Self {
    Self {
      font_color: None,
      bold: false,
      fill: None,
      borders: PresetPivotBorders::new(),
    }
  }

  const fn font(mut self, theme: u32, tint: f64) -> Self {
    self.font_color = Some(theme_color(theme, tint));
    self
  }

  const fn bold(mut self) -> Self {
    self.bold = true;
    self
  }

  const fn fill(mut self, theme: u32, tint: f64) -> Self {
    self.fill = Some(theme_color(theme, tint));
    self
  }

  const fn left(mut self, theme: u32, tint: f64) -> Self {
    self.borders.left = line(theme, tint, PresetBorderWeight::Thin);
    self
  }

  const fn right(mut self, theme: u32, tint: f64) -> Self {
    self.borders.right = line(theme, tint, PresetBorderWeight::Thin);
    self
  }

  const fn top(mut self, theme: u32, tint: f64) -> Self {
    self.borders.top = line(theme, tint, PresetBorderWeight::Thin);
    self
  }

  const fn bottom(mut self, theme: u32, tint: f64) -> Self {
    self.borders.bottom = line(theme, tint, PresetBorderWeight::Thin);
    self
  }

  const fn top_double(mut self, theme: u32, tint: f64) -> Self {
    self.borders.top = line(theme, tint, PresetBorderWeight::Double);
    self
  }

  const fn left_medium(mut self, theme: u32, tint: f64) -> Self {
    self.borders.left = line(theme, tint, PresetBorderWeight::Medium);
    self
  }

  const fn right_medium(mut self, theme: u32, tint: f64) -> Self {
    self.borders.right = line(theme, tint, PresetBorderWeight::Medium);
    self
  }

  const fn top_medium(mut self, theme: u32, tint: f64) -> Self {
    self.borders.top = line(theme, tint, PresetBorderWeight::Medium);
    self
  }

  const fn bottom_medium(mut self, theme: u32, tint: f64) -> Self {
    self.borders.bottom = line(theme, tint, PresetBorderWeight::Medium);
    self
  }

  const fn horizontal(mut self, theme: u32, tint: f64) -> Self {
    self.borders.horizontal = line(theme, tint, PresetBorderWeight::Thin);
    self
  }

  const fn vertical(mut self, theme: u32, tint: f64) -> Self {
    self.borders.vertical = line(theme, tint, PresetBorderWeight::Thin);
    self
  }

  const fn left_none(mut self) -> Self {
    self.borders.left = PresetBorderSide::NoLine;
    self
  }

  const fn right_none(mut self) -> Self {
    self.borders.right = PresetBorderSide::NoLine;
    self
  }

  const fn vertical_none(mut self) -> Self {
    self.borders.vertical = PresetBorderSide::NoLine;
    self
  }
}

impl PresetPivotBorders {
  const fn new() -> Self {
    Self {
      left: PresetBorderSide::Inherit,
      right: PresetBorderSide::Inherit,
      top: PresetBorderSide::Inherit,
      bottom: PresetBorderSide::Inherit,
      horizontal: PresetBorderSide::Inherit,
      vertical: PresetBorderSide::Inherit,
    }
  }
}

const fn theme_color(theme: u32, tint: f64) -> PresetThemeColor {
  PresetThemeColor { theme, tint }
}

const fn line(theme: u32, tint: f64, weight: PresetBorderWeight) -> PresetBorderSide {
  PresetBorderSide::Line(PresetBorderLine {
    color: theme_color(theme, tint),
    weight,
  })
}

fn light1_differential(element: x::TableStyleValues) -> Option<PresetPivotDifferential> {
  // Apache POI's presetTableStyles.xml is a direct transcription of the
  // built-in Office style. tableStyleElement@dxfId is zero-based, as required
  // by ECMA-376 §18.8.40; resolve each region to that exact DXF rather than
  // shifting the index as XSSFBuiltinTableStyle does for its enum sentinel.
  let dxf = match element {
    x::TableStyleValues::WholeTable => PresetPivotDifferential::new()
      .font(1, 0.0)
      .horizontal(0, -0.149_998_474_074_526_2),
    x::TableStyleValues::HeaderRow => PresetPivotDifferential::new()
      .font(1, 0.0)
      .bold()
      .top(1, 0.499_984_740_745_262)
      .bottom(1, 0.499_984_740_745_262),
    x::TableStyleValues::TotalRow => PresetPivotDifferential::new()
      .font(1, 0.0)
      .bold()
      .fill(0, -0.149_998_474_074_526_2)
      .top(1, 0.499_984_740_745_262)
      .bottom(1, 0.499_984_740_745_262),
    x::TableStyleValues::FirstRowStripe => PresetPivotDifferential::new()
      .fill(0, -0.149_998_474_074_526_2)
      .top(0, -0.349_986_266_670_735_8)
      .bottom(0, -0.349_986_266_670_735_8),
    x::TableStyleValues::FirstColumnStripe => PresetPivotDifferential::new()
      .fill(0, -0.149_998_474_074_526_2)
      .left(0, -0.349_986_266_670_735_8)
      .right(0, -0.349_986_266_670_735_8)
      .top(0, -0.349_986_266_670_735_8)
      .bottom(0, -0.349_986_266_670_735_8)
      .horizontal(0, -0.349_986_266_670_735_8)
      .vertical(0, -0.349_986_266_670_735_8),
    x::TableStyleValues::FirstSubtotalRow | x::TableStyleValues::FirstRowSubheading => {
      PresetPivotDifferential::new().font(1, 0.0).bold()
    }
    x::TableStyleValues::SecondSubtotalRow | x::TableStyleValues::SecondRowSubheading => {
      PresetPivotDifferential::new()
        .font(1, 0.499_984_740_745_262)
        .bold()
    }
    x::TableStyleValues::PageFieldLabels | x::TableStyleValues::PageFieldValues => {
      PresetPivotDifferential::new()
        .top(1, 0.499_984_740_745_262)
        .bottom(1, 0.499_984_740_745_262)
    }
    _ => return None,
  };
  Some(dxf)
}

fn light2_differential(
  element: x::TableStyleValues,
  accent: u32,
) -> Option<PresetPivotDifferential> {
  let dxf = match element {
    x::TableStyleValues::WholeTable => PresetPivotDifferential::new()
      .font(1, 0.0)
      .bold()
      .top(accent, 0.0)
      .bottom(accent, 0.0),
    x::TableStyleValues::HeaderRow => PresetPivotDifferential::new()
      .font(1, 0.0)
      .bold()
      .fill(0, 0.0)
      .top(accent, 0.0)
      .bottom(accent, 0.0),
    x::TableStyleValues::TotalRow => PresetPivotDifferential::new()
      .fill(accent, 0.799_981_688_894_314_4)
      .top(accent, 0.599_993_896_298_104_8)
      .bottom(accent, 0.599_993_896_298_104_8),
    x::TableStyleValues::FirstRowStripe => PresetPivotDifferential::new()
      .fill(accent, 0.799_981_688_894_314_4)
      .left(accent, 0.599_993_896_298_104_8)
      .right(accent, 0.599_993_896_298_104_8)
      .top(accent, 0.599_993_896_298_104_8)
      .bottom(accent, 0.599_993_896_298_104_8)
      .horizontal(accent, 0.599_993_896_298_104_8)
      .vertical(accent, 0.599_993_896_298_104_8),
    x::TableStyleValues::FirstColumnStripe | x::TableStyleValues::SecondSubtotalRow => {
      PresetPivotDifferential::new().font(1, 0.0).bold()
    }
    x::TableStyleValues::FirstSubtotalRow | x::TableStyleValues::FirstRowSubheading => {
      PresetPivotDifferential::new().font(accent, 0.0)
    }
    x::TableStyleValues::SecondRowSubheading | x::TableStyleValues::PageFieldLabels => {
      PresetPivotDifferential::new()
        .top(accent, 0.0)
        .bottom(accent, 0.0)
    }
    x::TableStyleValues::PageFieldValues => PresetPivotDifferential::new()
      .font(1, 0.0)
      .horizontal(accent, 0.799_981_688_894_314_4),
    _ => return None,
  };
  Some(dxf)
}

fn light_framed_differential(
  element: x::TableStyleValues,
  accent: Option<u32>,
) -> Option<PresetPivotDifferential> {
  let (frame_theme, frame_tint, inner_theme, inner_tint, strong_tint, fill_theme, fill_tint) =
    accent.map_or(
      (
        1,
        0.499_984_740_745_262,
        0,
        -0.349_986_266_670_735_8,
        -0.449_995_422_223_578_6,
        0,
        -0.149_998_474_074_526_2,
      ),
      |accent| {
        (
          accent,
          0.0,
          accent,
          0.599_993_896_298_104_8,
          0.0,
          accent,
          0.799_981_688_894_314_4,
        )
      },
    );
  let (secondary_rule_theme, secondary_rule_tint) =
    accent.map_or((inner_theme, inner_tint), |_| (fill_theme, fill_tint));
  let dxf = match element {
    x::TableStyleValues::WholeTable => PresetPivotDifferential::new()
      .font(1, 0.0)
      .bold()
      .left_medium(frame_theme, frame_tint)
      .right_medium(frame_theme, frame_tint)
      .top_medium(frame_theme, frame_tint)
      .bottom_medium(frame_theme, frame_tint)
      .horizontal(0, 0.0),
    x::TableStyleValues::HeaderRow => PresetPivotDifferential::new()
      .font(1, 0.0)
      .bold()
      .left_medium(frame_theme, frame_tint)
      .right_medium(frame_theme, frame_tint)
      .top_medium(frame_theme, frame_tint)
      .bottom_medium(frame_theme, frame_tint),
    x::TableStyleValues::TotalRow => PresetPivotDifferential::new().right(frame_theme, frame_tint),
    x::TableStyleValues::FirstColumn => PresetPivotDifferential::new()
      .left(inner_theme, inner_tint)
      .right(inner_theme, inner_tint)
      .top(inner_theme, inner_tint)
      .bottom(inner_theme, inner_tint),
    x::TableStyleValues::FirstRowStripe => PresetPivotDifferential::new()
      .left(inner_theme, strong_tint)
      .right(inner_theme, strong_tint)
      .top(inner_theme, strong_tint)
      .bottom(inner_theme, strong_tint),
    x::TableStyleValues::FirstColumnStripe => PresetPivotDifferential::new()
      .font(1, 0.0)
      .bold()
      .fill(fill_theme, fill_tint)
      .top(inner_theme, inner_tint)
      .bottom(inner_theme, inner_tint),
    x::TableStyleValues::FirstSubtotalRow | x::TableStyleValues::FirstRowSubheading => {
      PresetPivotDifferential::new().font(1, 0.0).bold()
    }
    x::TableStyleValues::SecondSubtotalRow => {
      PresetPivotDifferential::new().top(secondary_rule_theme, secondary_rule_tint)
    }
    x::TableStyleValues::SecondColumnSubheading if accent.is_some() => {
      PresetPivotDifferential::new().top(fill_theme, fill_tint)
    }
    x::TableStyleValues::SecondColumnSubheading => PresetPivotDifferential::new()
      .font(1, 0.0)
      .bold()
      .fill(fill_theme, fill_tint)
      .bottom(0, 0.0),
    x::TableStyleValues::ThirdColumnSubheading if accent.is_some() => {
      PresetPivotDifferential::new()
        .font(1, 0.0)
        .bold()
        .fill(fill_theme, fill_tint)
        .bottom(0, 0.0)
    }
    x::TableStyleValues::SecondRowSubheading => accent.map_or_else(
      || {
        PresetPivotDifferential::new()
          .font(1, 0.0)
          .horizontal(0, -0.149_998_474_074_526_2)
      },
      |accent| {
        PresetPivotDifferential::new()
          .font(accent, -0.249_977_111_117_893)
          .horizontal(accent, 0.799_981_688_894_314_4)
      },
    ),
    _ => return None,
  };
  Some(dxf)
}

fn light15_differential(element: x::TableStyleValues) -> Option<PresetPivotDifferential> {
  light_tinted_base_differential(element, None)
}

fn light_tinted_differential(
  element: x::TableStyleValues,
  accent: u32,
) -> Option<PresetPivotDifferential> {
  light_tinted_base_differential(element, Some(accent))
}

fn light_tinted_base_differential(
  element: x::TableStyleValues,
  accent: Option<u32>,
) -> Option<PresetPivotDifferential> {
  let (fill_theme, fill_tint, rule_theme, rule_tint) = accent.map_or(
    (0, -0.149_998_474_074_526_2, 0, -0.349_986_266_670_735_8),
    |accent| {
      (
        accent,
        0.799_981_688_894_314_4,
        accent,
        0.399_975_585_192_419_2,
      )
    },
  );
  let subtotal_rule = accent.map_or((1, 0.499_984_740_745_262), |accent| (accent, 0.0));
  // PivotStyleLight15 and Light16..21 share the same eleven-region layout.
  // Apache POI's presetTableStyles.xml stores pageFieldValues at DXF 0 and
  // headerRow at DXF 10; keep that zero-based order intact. In particular,
  // page-field values and first-subtotal columns are fills, not bold text.
  let dxf = match element {
    x::TableStyleValues::HeaderRow => PresetPivotDifferential::new()
      .font(1, 0.0)
      .bold()
      .fill(fill_theme, fill_tint)
      .bottom(rule_theme, rule_tint),
    x::TableStyleValues::TotalRow => PresetPivotDifferential::new()
      .font(1, 0.0)
      .bold()
      .fill(fill_theme, fill_tint)
      .top(rule_theme, rule_tint),
    x::TableStyleValues::FirstRowStripe => {
      PresetPivotDifferential::new().fill(0, -0.149_998_474_074_526_2)
    }
    x::TableStyleValues::FirstColumnStripe => PresetPivotDifferential::new()
      .fill(0, -0.149_998_474_074_526_2)
      .left(0, -0.249_977_111_117_893)
      .right(0, -0.249_977_111_117_893),
    x::TableStyleValues::FirstSubtotalColumn => {
      PresetPivotDifferential::new().fill(0, -0.149_998_474_074_526_2)
    }
    x::TableStyleValues::FirstSubtotalRow => PresetPivotDifferential::new()
      .font(1, 0.0)
      .bold()
      .top(subtotal_rule.0, subtotal_rule.1)
      .bottom(subtotal_rule.0, subtotal_rule.1),
    x::TableStyleValues::SecondSubtotalRow | x::TableStyleValues::SecondRowSubheading => {
      PresetPivotDifferential::new().font(1, 0.0).bold()
    }
    x::TableStyleValues::FirstRowSubheading => PresetPivotDifferential::new()
      .font(1, 0.0)
      .bold()
      .bottom(rule_theme, rule_tint),
    x::TableStyleValues::PageFieldLabels | x::TableStyleValues::PageFieldValues => {
      PresetPivotDifferential::new()
        .fill(fill_theme, fill_tint)
        .bottom(rule_theme, rule_tint)
    }
    _ => return None,
  };
  Some(dxf)
}

fn light22_differential(element: x::TableStyleValues) -> Option<PresetPivotDifferential> {
  // Apache POI's presetTableStyles.xml records the ten Office DXFs in their
  // serialized zero-based order. In particular, wholeTable uses DXF 9 (plain
  // dark text with a frame), while the bold subtotal/header regions use DXFs
  // 0..3 and 6..8. Do not shift these indexes through an enum sentinel.
  let dxf = match element {
    x::TableStyleValues::WholeTable => PresetPivotDifferential::new()
      .font(1, 0.0)
      .left(1, 0.499_984_740_745_262)
      .right(1, 0.499_984_740_745_262)
      .top(1, 0.499_984_740_745_262)
      .bottom(1, 0.499_984_740_745_262)
      .vertical(1, 0.499_984_740_745_262),
    x::TableStyleValues::HeaderRow => PresetPivotDifferential::new()
      .font(1, 0.0)
      .bold()
      .bottom(1, 0.499_984_740_745_262),
    x::TableStyleValues::TotalRow => PresetPivotDifferential::new()
      .font(1, 0.0)
      .bold()
      .top(1, 0.499_984_740_745_262),
    x::TableStyleValues::FirstColumn => PresetPivotDifferential::new().font(1, 0.0).bold(),
    x::TableStyleValues::FirstRowStripe => PresetPivotDifferential::new()
      .fill(0, -0.149_998_474_074_526_2)
      .top(1, 0.499_984_740_745_262)
      .bottom(1, 0.499_984_740_745_262),
    x::TableStyleValues::FirstColumnStripe => {
      PresetPivotDifferential::new().fill(0, -0.149_998_474_074_526_2)
    }
    x::TableStyleValues::FirstSubtotalColumn => PresetPivotDifferential::new()
      .font(1, 0.0)
      .bold()
      .left(0, -0.349_986_266_670_735_8)
      .right(0, -0.349_986_266_670_735_8),
    x::TableStyleValues::FirstSubtotalRow
    | x::TableStyleValues::SecondSubtotalRow
    | x::TableStyleValues::PageFieldLabels => PresetPivotDifferential::new().font(1, 0.0).bold(),
    _ => return None,
  };
  Some(dxf)
}

fn light_outline_differential(
  element: x::TableStyleValues,
  accent: u32,
) -> Option<PresetPivotDifferential> {
  let font = -0.249_977_111_117_893;
  let dxf = match element {
    x::TableStyleValues::WholeTable => PresetPivotDifferential::new()
      .font(accent, font)
      .bottom(accent, 0.0),
    x::TableStyleValues::HeaderRow => PresetPivotDifferential::new()
      .font(accent, font)
      .top(accent, 0.0),
    x::TableStyleValues::TotalRow
    | x::TableStyleValues::FirstSubtotalColumn
    | x::TableStyleValues::FirstSubtotalRow
    | x::TableStyleValues::SecondSubtotalRow
    | x::TableStyleValues::FirstColumnStripe => PresetPivotDifferential::new().font(accent, font),
    x::TableStyleValues::FirstColumn => PresetPivotDifferential::new()
      .fill(accent, 0.799_981_688_894_314_4)
      .top(1, 0.499_984_740_745_262)
      .bottom(1, 0.499_984_740_745_262),
    x::TableStyleValues::FirstRowStripe => {
      PresetPivotDifferential::new().fill(accent, 0.799_981_688_894_314_4)
    }
    x::TableStyleValues::PageFieldLabels => PresetPivotDifferential::new()
      .font(accent, font)
      .left(accent, 0.0)
      .right(accent, 0.0)
      .top(accent, 0.0)
      .bottom(accent, 0.0)
      .vertical(accent, 0.0),
    _ => return None,
  };
  Some(dxf)
}

fn medium_accent_differential(
  element: x::TableStyleValues,
  accent: Option<u32>,
) -> Option<PresetPivotDifferential> {
  let (base_theme, base_tint, strong_fill_theme, strong_fill_tint) = accent.map_or(
    (1, 0.499_984_740_745_262, 0, -0.349_986_266_670_735_8),
    |accent| {
      (
        accent,
        -0.249_977_111_117_893,
        accent,
        0.399_975_585_192_419_2,
      )
    },
  );
  let (soft_theme, soft_tint, soft_rule_theme, soft_rule_tint) = accent.map_or(
    (0, -0.149_998_474_074_526_2, 0, -0.149_998_474_074_526_2),
    |accent| {
      (
        accent,
        0.799_981_688_894_314_4,
        accent,
        0.599_993_896_298_104_8,
      )
    },
  );
  let (subheading_bottom_theme, subheading_bottom_tint) =
    accent.map_or((0, -0.149_998_474_074_526_2), |accent| (accent, 0.0));
  let (subheading_horizontal_theme, subheading_horizontal_tint) = accent
    .map_or((0, -0.149_998_474_074_526_2), |accent| {
      (accent, 0.399_975_585_192_419_2)
    });
  let dxf = match element {
    x::TableStyleValues::WholeTable => PresetPivotDifferential::new()
      .font(0, 0.0)
      .fill(base_theme, base_tint)
      .horizontal(base_theme, base_tint),
    x::TableStyleValues::HeaderRow => PresetPivotDifferential::new()
      .font(1, 0.0)
      .bold()
      .top_double(base_theme, base_tint),
    x::TableStyleValues::TotalRow => PresetPivotDifferential::new()
      .top(base_theme, base_tint)
      .bottom(base_theme, base_tint)
      .horizontal(base_theme, base_tint),
    x::TableStyleValues::FirstRowStripe => PresetPivotDifferential::new()
      .left(base_theme, base_tint)
      .right(base_theme, base_tint),
    x::TableStyleValues::FirstColumnStripe => PresetPivotDifferential::new().font(0, 0.0).bold(),
    x::TableStyleValues::FirstHeaderCell => PresetPivotDifferential::new()
      .font(0, 0.0)
      .bold()
      .fill(strong_fill_theme, strong_fill_tint),
    x::TableStyleValues::SecondSubtotalRow => {
      PresetPivotDifferential::new().bottom(soft_rule_theme, soft_rule_tint)
    }
    x::TableStyleValues::FirstSubtotalRow => PresetPivotDifferential::new()
      .font(1, 0.0)
      .bold()
      .fill(0, -0.149_998_474_074_526_2),
    x::TableStyleValues::FirstColumnSubheading => PresetPivotDifferential::new()
      .font(0, 0.0)
      .fill(strong_fill_theme, strong_fill_tint)
      .bottom(soft_theme, soft_tint)
      .horizontal(subheading_horizontal_theme, subheading_horizontal_tint),
    x::TableStyleValues::FirstRowSubheading => PresetPivotDifferential::new()
      .fill(soft_theme, soft_tint)
      .bottom(subheading_bottom_theme, subheading_bottom_tint),
    x::TableStyleValues::SecondRowSubheading | x::TableStyleValues::PageFieldLabels => {
      PresetPivotDifferential::new()
        .top(soft_theme, soft_tint)
        .bottom(soft_theme, soft_tint)
    }
    x::TableStyleValues::PageFieldValues => PresetPivotDifferential::new()
      .font(1, 0.0)
      .horizontal(soft_theme, soft_tint),
    _ => return None,
  };
  Some(dxf)
}

fn medium_framed_differential(
  element: x::TableStyleValues,
  accent: Option<u32>,
) -> Option<PresetPivotDifferential> {
  let (header_fill_theme, header_fill_tint, frame_theme, frame_tint) = accent.map_or(
    (1, 0.499_984_740_745_262, 1, 0.499_984_740_745_262),
    |accent| (accent, 0.0, accent, -0.249_977_111_117_893),
  );
  let (body_rule_theme, body_rule_tint) = accent.map_or((0, -0.249_977_111_117_893), |accent| {
    (accent, 0.399_975_585_192_419_2)
  });
  let (subtotal_theme, subtotal_tint) = accent.map_or((0, -0.249_977_111_117_893), |accent| {
    (accent, 0.599_993_896_298_104_8)
  });
  let (soft_theme, soft_tint) = accent.map_or((0, -0.149_998_474_074_526_2), |accent| {
    (accent, 0.799_981_688_894_314_4)
  });
  let dxf = match element {
    x::TableStyleValues::WholeTable => PresetPivotDifferential::new()
      .font(0, 0.0)
      .bold()
      .fill(header_fill_theme, header_fill_tint)
      .top_medium(frame_theme, frame_tint),
    x::TableStyleValues::HeaderRow => PresetPivotDifferential::new()
      .font(1, 0.0)
      .bold()
      .top(frame_theme, accent.map_or(0.0, |_| frame_tint))
      .bottom_medium(frame_theme, frame_tint),
    x::TableStyleValues::TotalRow => PresetPivotDifferential::new()
      .top(body_rule_theme, body_rule_tint)
      .bottom(body_rule_theme, body_rule_tint)
      .horizontal(body_rule_theme, body_rule_tint),
    x::TableStyleValues::FirstRowStripe => {
      let dxf = PresetPivotDifferential::new();
      if accent.is_some() {
        dxf
          .left(body_rule_theme, body_rule_tint)
          .right(body_rule_theme, body_rule_tint)
      } else {
        dxf
          .left_medium(0, -0.149_998_474_074_526_2)
          .right_medium(0, -0.149_998_474_074_526_2)
      }
    }
    x::TableStyleValues::FirstColumnStripe => PresetPivotDifferential::new()
      .font(1, 0.0)
      .bold()
      .left_medium(subtotal_theme, subtotal_tint)
      .right_medium(subtotal_theme, subtotal_tint)
      .top_medium(subtotal_theme, subtotal_tint)
      .bottom_medium(subtotal_theme, subtotal_tint),
    x::TableStyleValues::FirstSubtotalColumn => PresetPivotDifferential::new()
      .font(1, 0.0)
      .bold()
      .fill(subtotal_theme, subtotal_tint),
    x::TableStyleValues::FirstSubtotalRow | x::TableStyleValues::FirstRowSubheading => {
      PresetPivotDifferential::new().font(1, 0.0).bold()
    }
    x::TableStyleValues::SecondSubtotalRow => PresetPivotDifferential::new()
      .font(1, 0.0)
      .bold()
      .fill(soft_theme, soft_tint),
    x::TableStyleValues::SecondRowSubheading | x::TableStyleValues::PageFieldLabels => {
      PresetPivotDifferential::new().fill(soft_theme, soft_tint)
    }
    x::TableStyleValues::PageFieldValues => PresetPivotDifferential::new().font(1, 0.0),
    _ => return None,
  };
  Some(dxf)
}

fn medium_grid_differential(
  element: x::TableStyleValues,
  accent: Option<u32>,
) -> Option<PresetPivotDifferential> {
  let (soft_theme, soft_tint, rule_theme, rule_tint, strong_theme, strong_tint) = accent.map_or(
    (
      0,
      -0.149_998_474_074_526_2,
      0,
      -0.249_977_111_117_893,
      0,
      -0.349_986_266_670_735_8,
    ),
    |accent| {
      (
        accent,
        0.799_981_688_894_314_4,
        accent,
        0.599_993_896_298_104_8,
        accent,
        0.399_975_585_192_419_2,
      )
    },
  );
  let (page_rule_theme, page_rule_tint) = accent.map_or((0, -0.149_998_474_074_526_2), |accent| {
    (accent, 0.599_993_896_298_104_8)
  });
  let dxf = match element {
    x::TableStyleValues::WholeTable | x::TableStyleValues::HeaderRow => {
      PresetPivotDifferential::new()
        .font(0, 0.0)
        .fill(1, 0.0)
        .left_none()
        .right_none()
        .vertical_none()
    }
    x::TableStyleValues::TotalRow => PresetPivotDifferential::new()
      .fill(soft_theme, soft_tint)
      .top(rule_theme, rule_tint)
      .bottom(rule_theme, rule_tint),
    x::TableStyleValues::FirstRowStripe => PresetPivotDifferential::new()
      .fill(soft_theme, soft_tint)
      .left(rule_theme, rule_tint)
      .right(rule_theme, rule_tint),
    x::TableStyleValues::FirstColumnStripe => PresetPivotDifferential::new()
      .font(1, 0.0)
      .fill(rule_theme, rule_tint)
      .left(strong_theme, strong_tint)
      .right(strong_theme, strong_tint)
      .top(strong_theme, strong_tint),
    x::TableStyleValues::FirstSubtotalColumn => PresetPivotDifferential::new()
      .font(1, 0.0)
      .bold()
      .bottom(rule_theme, rule_tint),
    x::TableStyleValues::FirstSubtotalRow | x::TableStyleValues::FirstRowSubheading => {
      PresetPivotDifferential::new()
        .font(1, 0.499_984_740_745_262)
        .bold()
    }
    x::TableStyleValues::SecondSubtotalRow | x::TableStyleValues::SecondRowSubheading => {
      PresetPivotDifferential::new().font(1, 0.0).bold()
    }
    x::TableStyleValues::ThirdRowSubheading | x::TableStyleValues::PageFieldLabels => {
      PresetPivotDifferential::new().top(1, 0.0).bottom(1, 0.0)
    }
    x::TableStyleValues::PageFieldValues => {
      let (fill_theme, fill_tint) = accent.map_or((0, -0.049_989_318_521_683_403), |accent| {
        (accent, 0.799_981_688_894_314_4)
      });
      PresetPivotDifferential::new()
        .font(1, 0.0)
        .fill(fill_theme, fill_tint)
        .left(page_rule_theme, page_rule_tint)
        .right(page_rule_theme, page_rule_tint)
        .vertical(page_rule_theme, page_rule_tint)
    }
    _ => return None,
  };
  Some(dxf)
}

fn medium_band_differential(
  element: x::TableStyleValues,
  accent: Option<u32>,
) -> Option<PresetPivotDifferential> {
  let (font_theme, font_tint, fill_theme, fill_tint) = accent.map_or(
    (1, 0.499_984_740_745_262, 0, -0.249_977_111_117_893),
    |accent| {
      (
        accent,
        -0.249_977_111_117_893,
        accent,
        0.599_993_896_298_104_8,
      )
    },
  );
  let dxf = match element {
    x::TableStyleValues::WholeTable => PresetPivotDifferential::new()
      .font(font_theme, font_tint)
      .bold()
      .bottom(0, 0.0),
    x::TableStyleValues::HeaderRow => PresetPivotDifferential::new()
      .font(font_theme, font_tint)
      .bold(),
    x::TableStyleValues::TotalRow => PresetPivotDifferential::new()
      .font(font_theme, font_tint)
      .bold()
      .fill(fill_theme, fill_tint),
    x::TableStyleValues::FirstColumn | x::TableStyleValues::SecondRowStripe => {
      PresetPivotDifferential::new().fill(fill_theme, fill_tint)
    }
    x::TableStyleValues::SecondColumnStripe
    | x::TableStyleValues::FirstSubtotalRow
    | x::TableStyleValues::SecondSubtotalRow => PresetPivotDifferential::new().font(1, 0.0).bold(),
    x::TableStyleValues::FirstRowSubheading => PresetPivotDifferential::new()
      .font(font_theme, font_tint)
      .bold()
      .fill(fill_theme, fill_tint),
    x::TableStyleValues::PageFieldLabels => {
      let (page_fill_theme, page_fill_tint) = accent
        .map_or((0, -0.149_998_474_074_526_2), |accent| {
          (accent, 0.799_981_688_894_314_4)
        });
      PresetPivotDifferential::new()
        .font(font_theme, font_tint)
        .fill(page_fill_theme, page_fill_tint)
        .vertical(0, 0.0)
    }
    _ => return None,
  };
  Some(dxf)
}

fn dark1_differential(element: x::TableStyleValues) -> Option<PresetPivotDifferential> {
  dark_base_differential(element, None)
}

fn dark_accent_differential(
  element: x::TableStyleValues,
  accent: u32,
) -> Option<PresetPivotDifferential> {
  dark_base_differential(element, Some(accent))
}

fn dark_base_differential(
  element: x::TableStyleValues,
  accent: Option<u32>,
) -> Option<PresetPivotDifferential> {
  let (dark_theme, dark_tint, mid_theme, mid_tint, light_theme, light_tint) = accent.map_or(
    (
      1,
      0.499_984_740_745_262,
      0,
      -0.349_986_266_670_735_8,
      0,
      -0.149_998_474_074_526_2,
    ),
    |accent| {
      (
        accent,
        -0.499_984_740_745_262,
        accent,
        0.399_975_585_192_419_2,
        accent,
        0.799_981_688_894_314_4,
      )
    },
  );
  let whole_fill = accent.map_or((1, 0.499_984_740_745_262), |accent| {
    (accent, -0.499_984_740_745_262)
  });
  let whole_bottom = (accent.unwrap_or(0), 0.0);
  let stripe_bottom = accent.map_or((0, -0.349_986_266_670_735_8), |accent| {
    (accent, 0.799_981_688_894_314_4)
  });
  let dxf = match element {
    x::TableStyleValues::WholeTable => PresetPivotDifferential::new()
      .font(0, 0.0)
      .bold()
      .fill(whole_fill.0, whole_fill.1)
      .bottom(whole_bottom.0, whole_bottom.1)
      .horizontal(dark_theme, dark_tint),
    x::TableStyleValues::HeaderRow => PresetPivotDifferential::new()
      .font(0, 0.0)
      .bold()
      .fill(dark_theme, dark_tint),
    x::TableStyleValues::TotalRow => PresetPivotDifferential::new().fill(mid_theme, mid_tint),
    x::TableStyleValues::SecondRowStripe => PresetPivotDifferential::new()
      .left(light_theme, light_tint)
      .right(light_theme, light_tint),
    x::TableStyleValues::SecondColumnStripe => PresetPivotDifferential::new()
      .font(1, 0.0)
      .bold()
      .bottom(stripe_bottom.0, stripe_bottom.1),
    x::TableStyleValues::FirstSubtotalRow => PresetPivotDifferential::new()
      .font(1, 0.0)
      .bold()
      .fill(light_theme, light_tint)
      .top(mid_theme, mid_tint)
      .bottom(mid_theme, mid_tint),
    x::TableStyleValues::FirstRowSubheading => PresetPivotDifferential::new().font(1, 0.0).bold(),
    x::TableStyleValues::SecondRowSubheading => PresetPivotDifferential::new()
      .font(0, 0.0)
      .bold()
      .fill(dark_theme, dark_tint)
      .horizontal(dark_theme, dark_tint),
    x::TableStyleValues::PageFieldLabels => PresetPivotDifferential::new()
      .font(0, 0.0)
      .fill(dark_theme, dark_tint)
      .horizontal(dark_theme, dark_tint),
    x::TableStyleValues::PageFieldValues => {
      let (fill_theme, fill_tint) = accent.map_or((0, -0.249_977_111_117_893), |accent| {
        (accent, 0.599_993_896_298_104_8)
      });
      PresetPivotDifferential::new()
        .font(1, 0.0)
        .fill(fill_theme, fill_tint)
        .horizontal(light_theme, light_tint)
    }
    _ => return None,
  };
  Some(dxf)
}

fn dark_framed_differential(
  element: x::TableStyleValues,
  accent: Option<u32>,
) -> Option<PresetPivotDifferential> {
  let (rule_theme, rule_tint, stripe_theme, stripe_tint, soft_theme, soft_tint) = accent.map_or(
    (
      0,
      -0.249_977_111_117_893,
      0,
      -0.449_995_422_223_578_6,
      0,
      -0.149_998_474_074_526_2,
    ),
    |accent| {
      (
        accent,
        0.599_993_896_298_104_8,
        accent,
        0.399_975_585_192_419_2,
        accent,
        0.799_981_688_894_314_4,
      )
    },
  );
  let (subtotal_theme, subtotal_tint) = accent.map_or((0, -0.249_977_111_117_893), |accent| {
    (accent, 0.799_981_688_894_314_4)
  });
  let dxf = match element {
    x::TableStyleValues::WholeTable | x::TableStyleValues::HeaderRow => {
      PresetPivotDifferential::new()
        .font(0, 0.0)
        .bold()
        .fill(1, 0.249_977_111_117_893)
    }
    x::TableStyleValues::TotalRow => PresetPivotDifferential::new()
      .top(rule_theme, rule_tint)
      .bottom(rule_theme, rule_tint),
    x::TableStyleValues::SecondRowStripe | x::TableStyleValues::FirstColumnStripe => {
      PresetPivotDifferential::new()
        .left(stripe_theme, stripe_tint)
        .right(stripe_theme, stripe_tint)
    }
    x::TableStyleValues::SecondColumnStripe => PresetPivotDifferential::new()
      .font(1, 0.0)
      .bold()
      .fill(rule_theme, rule_tint),
    x::TableStyleValues::FirstSubtotalRow | x::TableStyleValues::SecondColumnSubheading => {
      PresetPivotDifferential::new().top_medium(subtotal_theme, subtotal_tint)
    }
    x::TableStyleValues::ThirdColumnSubheading => PresetPivotDifferential::new()
      .font(1, 0.0)
      .bold()
      .fill(rule_theme, rule_tint)
      .bottom_medium(soft_theme, soft_tint),
    x::TableStyleValues::FirstRowSubheading => PresetPivotDifferential::new().font(1, 0.0).bold(),
    x::TableStyleValues::SecondRowSubheading => PresetPivotDifferential::new()
      .left_medium(1, 0.499_984_740_745_262)
      .top_medium(1, 0.499_984_740_745_262)
      .bottom_medium(1, 0.499_984_740_745_262),
    x::TableStyleValues::PageFieldLabels => PresetPivotDifferential::new()
      .right_medium(1, 0.499_984_740_745_262)
      .top_medium(1, 0.499_984_740_745_262)
      .bottom_medium(1, 0.499_984_740_745_262),
    x::TableStyleValues::PageFieldValues => PresetPivotDifferential::new()
      .font(1, 0.0)
      .fill(soft_theme, soft_tint)
      .left_medium(1, 0.499_984_740_745_262)
      .right_medium(1, 0.499_984_740_745_262)
      .top_medium(1, 0.499_984_740_745_262)
      .bottom_medium(1, 0.499_984_740_745_262),
    _ => return None,
  };
  Some(dxf)
}

fn dark_grid_differential(
  element: x::TableStyleValues,
  accent: Option<u32>,
) -> Option<PresetPivotDifferential> {
  let (rule_theme, rule_tint, fill_theme, fill_tint) = accent.map_or(
    (0, -0.349_986_266_670_735_8, 1, 0.499_984_740_745_262),
    |accent| {
      (
        accent,
        0.399_975_585_192_419_2,
        accent,
        -0.249_977_111_117_893,
      )
    },
  );
  let stripe_tint = accent.map_or(-0.249_977_111_117_893, |_| rule_tint);
  let dxf = match element {
    x::TableStyleValues::WholeTable | x::TableStyleValues::HeaderRow => {
      PresetPivotDifferential::new()
        .font(0, 0.0)
        .bold()
        .fill(1, 0.0)
    }
    x::TableStyleValues::TotalRow => PresetPivotDifferential::new()
      .top(rule_theme, rule_tint)
      .bottom(rule_theme, rule_tint),
    x::TableStyleValues::FirstRowStripe => PresetPivotDifferential::new()
      .left(rule_theme, stripe_tint)
      .right(rule_theme, stripe_tint),
    x::TableStyleValues::FirstColumnStripe => PresetPivotDifferential::new()
      .fill(fill_theme, fill_tint)
      .left(rule_theme, rule_tint)
      .right(rule_theme, rule_tint)
      .top(rule_theme, rule_tint)
      .bottom(rule_theme, rule_tint),
    x::TableStyleValues::FirstSubtotalColumn
    | x::TableStyleValues::FirstRowSubheading
    | x::TableStyleValues::SecondRowSubheading => {
      PresetPivotDifferential::new().font(0, 0.0).bold()
    }
    x::TableStyleValues::FirstSubtotalRow => {
      PresetPivotDifferential::new().bottom(rule_theme, rule_tint)
    }
    x::TableStyleValues::FirstColumnSubheading => {
      let dxf = PresetPivotDifferential::new()
        .font(0, 0.0)
        .bold()
        .fill(fill_theme, fill_tint);
      if accent.is_none() {
        dxf.bottom(4, 0.799_981_688_894_314_4)
      } else {
        dxf
      }
    }
    x::TableStyleValues::PageFieldLabels => accent.map_or_else(
      || {
        PresetPivotDifferential::new()
          .font(0, -0.149_998_474_074_526_2)
          .fill(0, -0.449_995_422_223_578_6)
      },
      |accent| {
        PresetPivotDifferential::new()
          .font(accent, 0.799_981_688_894_314_4)
          .fill(accent, 0.0)
      },
    ),
    _ => return None,
  };
  Some(dxf)
}

fn dark_band_differential(
  element: x::TableStyleValues,
  accent: Option<u32>,
) -> Option<PresetPivotDifferential> {
  let (main_fill_theme, main_fill_tint, stripe_theme, stripe_tint) = accent.map_or(
    (1, 0.499_984_740_745_262, 0, -0.449_995_422_223_578_6),
    |accent| {
      (
        accent,
        -0.249_977_111_117_893,
        accent,
        0.399_975_585_192_419_2,
      )
    },
  );
  let dxf = match element {
    x::TableStyleValues::WholeTable => PresetPivotDifferential::new()
      .fill(main_fill_theme, main_fill_tint)
      .bottom_medium(0, 0.0),
    x::TableStyleValues::HeaderRow => PresetPivotDifferential::new()
      .font(0, 0.0)
      .bold()
      .top_medium(0, 0.0),
    x::TableStyleValues::TotalRow | x::TableStyleValues::SecondRowSubheading => {
      PresetPivotDifferential::new().fill(main_fill_theme, main_fill_tint)
    }
    x::TableStyleValues::FirstColumn | x::TableStyleValues::SecondRowStripe => {
      PresetPivotDifferential::new().fill(stripe_theme, stripe_tint)
    }
    x::TableStyleValues::SecondColumnStripe
    | x::TableStyleValues::SecondSubtotalRow
    | x::TableStyleValues::FirstRowSubheading => PresetPivotDifferential::new().font(0, 0.0).bold(),
    x::TableStyleValues::FirstHeaderCell => accent.map_or_else(
      || {
        PresetPivotDifferential::new()
          .font(0, -0.149_998_474_074_526_2)
          .bold()
      },
      |accent| PresetPivotDifferential::new().font(accent, 0.799_981_688_894_314_4),
    ),
    x::TableStyleValues::FirstSubtotalRow => PresetPivotDifferential::new()
      .font(if accent.is_some() { 0 } else { 1 }, 0.0)
      .bold(),
    x::TableStyleValues::PageFieldLabels => accent.map_or_else(
      || {
        PresetPivotDifferential::new()
          .font(0, -0.149_998_474_074_526_2)
          .fill(0, -0.499_984_740_745_262)
          .vertical(0, 0.0)
      },
      |accent| {
        PresetPivotDifferential::new()
          .font(accent, 0.799_981_688_894_314_4)
          .fill(accent, 0.0)
          .vertical(0, 0.0)
      },
    ),
    _ => return None,
  };
  Some(dxf)
}
