use ooxmlsdk::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main as x;

#[derive(Clone, Copy, Debug)]
pub(super) struct PresetPivotStyle {
  kind: PresetPivotStyleKind,
}

#[derive(Clone, Copy, Debug)]
enum PresetPivotStyleKind {
  Light1,
  Light2 { accent: u32 },
  Light15,
  LightTinted { accent: u32 },
  Light22,
  LightOutline { accent: u32 },
  MediumAccent { accent: u32 },
  Dark1,
  DarkAccent { accent: u32 },
}

#[derive(Clone, Copy, Debug)]
pub(super) struct PresetThemeColor {
  pub(super) theme: u32,
  pub(super) tint: f64,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum PresetBorderWeight {
  Thin,
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
      ("light", 2) => PresetPivotStyleKind::Light2 { accent: 4 },
      ("light", 15) => PresetPivotStyleKind::Light15,
      ("light", 16..=21) => PresetPivotStyleKind::LightTinted {
        accent: u32::from(number - 12),
      },
      ("light", 22) => PresetPivotStyleKind::Light22,
      ("light", 23..=28) => PresetPivotStyleKind::LightOutline {
        accent: u32::from(number - 19),
      },
      ("medium", 2..=7) => PresetPivotStyleKind::MediumAccent {
        accent: u32::from(number + 2),
      },
      ("dark", 1) => PresetPivotStyleKind::Dark1,
      ("dark", 2..=7) => PresetPivotStyleKind::DarkAccent {
        accent: u32::from(number + 2),
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
      PresetPivotStyleKind::Light15 => light15_differential(element),
      PresetPivotStyleKind::LightTinted { accent } => light_tinted_differential(element, accent),
      PresetPivotStyleKind::Light22 => light22_differential(element),
      PresetPivotStyleKind::LightOutline { accent } => light_outline_differential(element, accent),
      PresetPivotStyleKind::MediumAccent { accent } => medium_accent_differential(element, accent),
      PresetPivotStyleKind::Dark1 => dark1_differential(element),
      PresetPivotStyleKind::DarkAccent { accent } => dark_accent_differential(element, accent),
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

  const fn horizontal(mut self, theme: u32, tint: f64) -> Self {
    self.borders.horizontal = line(theme, tint, PresetBorderWeight::Thin);
    self
  }

  const fn vertical(mut self, theme: u32, tint: f64) -> Self {
    self.borders.vertical = line(theme, tint, PresetBorderWeight::Thin);
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
  // Apache POI's presetTableStyles.xml stores one-based dxfId values. Its
  // XSSFBuiltinTableStyle parser inserts an empty DXF at index zero before
  // resolving them. The semantic descriptors below include that shift.
  let dxf = match element {
    x::TableStyleValues::WholeTable => PresetPivotDifferential::new()
      .font(1, 0.0)
      .bold()
      .top(1, 0.499_984_740_745_262)
      .bottom(1, 0.499_984_740_745_262),
    x::TableStyleValues::HeaderRow => PresetPivotDifferential::new()
      .font(1, 0.0)
      .bold()
      .fill(0, 0.0)
      .top(1, 0.499_984_740_745_262)
      .bottom(1, 0.499_984_740_745_262),
    x::TableStyleValues::TotalRow => PresetPivotDifferential::new()
      .fill(0, -0.149_998_474_074_526_2)
      .top(0, -0.349_986_266_670_735_8)
      .bottom(0, -0.349_986_266_670_735_8),
    x::TableStyleValues::FirstRowStripe => PresetPivotDifferential::new()
      .fill(0, -0.149_998_474_074_526_2)
      .left(0, -0.349_986_266_670_735_8)
      .right(0, -0.349_986_266_670_735_8)
      .top(0, -0.349_986_266_670_735_8)
      .bottom(0, -0.349_986_266_670_735_8)
      .horizontal(0, -0.349_986_266_670_735_8)
      .vertical(0, -0.349_986_266_670_735_8),
    x::TableStyleValues::FirstColumnStripe | x::TableStyleValues::SecondSubtotalRow => {
      PresetPivotDifferential::new().font(1, 0.0).bold()
    }
    x::TableStyleValues::FirstSubtotalRow => PresetPivotDifferential::new().font(1, 0.0).bold(),
    x::TableStyleValues::FirstRowSubheading => PresetPivotDifferential::new()
      .font(1, 0.499_984_740_745_262)
      .bold(),
    x::TableStyleValues::SecondRowSubheading | x::TableStyleValues::PageFieldLabels => {
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
  let dxf = match element {
    x::TableStyleValues::HeaderRow => PresetPivotDifferential::new()
      .font(1, 0.0)
      .bold()
      .fill(fill_theme, fill_tint)
      .top(rule_theme, rule_tint),
    x::TableStyleValues::TotalRow | x::TableStyleValues::FirstColumnStripe => {
      PresetPivotDifferential::new().fill(0, -0.149_998_474_074_526_2)
    }
    x::TableStyleValues::FirstRowStripe => PresetPivotDifferential::new()
      .fill(0, -0.149_998_474_074_526_2)
      .left(0, -0.249_977_111_117_893)
      .right(0, -0.249_977_111_117_893),
    x::TableStyleValues::FirstSubtotalColumn => PresetPivotDifferential::new()
      .font(1, 0.0)
      .bold()
      .top(subtotal_rule.0, subtotal_rule.1)
      .bottom(subtotal_rule.0, subtotal_rule.1),
    x::TableStyleValues::FirstSubtotalRow | x::TableStyleValues::FirstRowSubheading => {
      PresetPivotDifferential::new().font(1, 0.0).bold()
    }
    x::TableStyleValues::SecondSubtotalRow => PresetPivotDifferential::new()
      .font(1, 0.0)
      .bold()
      .bottom(rule_theme, rule_tint),
    x::TableStyleValues::SecondRowSubheading | x::TableStyleValues::PageFieldLabels => {
      PresetPivotDifferential::new()
        .fill(fill_theme, fill_tint)
        .bottom(rule_theme, rule_tint)
    }
    _ => return None,
  };
  Some(dxf)
}

fn light22_differential(element: x::TableStyleValues) -> Option<PresetPivotDifferential> {
  let dxf = match element {
    x::TableStyleValues::WholeTable => PresetPivotDifferential::new()
      .font(1, 0.0)
      .bottom(1, 0.499_984_740_745_262)
      .bold(),
    x::TableStyleValues::HeaderRow => PresetPivotDifferential::new()
      .font(1, 0.0)
      .bold()
      .top(1, 0.499_984_740_745_262),
    x::TableStyleValues::TotalRow
    | x::TableStyleValues::FirstSubtotalColumn
    | x::TableStyleValues::FirstSubtotalRow
    | x::TableStyleValues::SecondSubtotalRow => PresetPivotDifferential::new().font(1, 0.0).bold(),
    x::TableStyleValues::FirstColumn => PresetPivotDifferential::new()
      .fill(0, -0.149_998_474_074_526_2)
      .top(1, 0.499_984_740_745_262)
      .bottom(1, 0.499_984_740_745_262),
    x::TableStyleValues::FirstRowStripe => {
      PresetPivotDifferential::new().fill(0, -0.149_998_474_074_526_2)
    }
    x::TableStyleValues::FirstColumnStripe => PresetPivotDifferential::new()
      .font(1, 0.0)
      .bold()
      .left(0, -0.349_986_266_670_735_8)
      .right(0, -0.349_986_266_670_735_8),
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
    _ => return None,
  };
  Some(dxf)
}

fn medium_accent_differential(
  element: x::TableStyleValues,
  accent: u32,
) -> Option<PresetPivotDifferential> {
  let dxf = match element {
    x::TableStyleValues::WholeTable => PresetPivotDifferential::new()
      .font(0, 0.0)
      .fill(accent, -0.249_977_111_117_893)
      .horizontal(accent, -0.249_977_111_117_893),
    x::TableStyleValues::HeaderRow => PresetPivotDifferential::new()
      .font(1, 0.0)
      .bold()
      .top_double(accent, -0.249_977_111_117_893),
    x::TableStyleValues::TotalRow => PresetPivotDifferential::new()
      .top(accent, -0.249_977_111_117_893)
      .bottom(accent, -0.249_977_111_117_893)
      .horizontal(accent, -0.249_977_111_117_893),
    x::TableStyleValues::FirstRowStripe => PresetPivotDifferential::new()
      .left(accent, -0.249_977_111_117_893)
      .right(accent, -0.249_977_111_117_893),
    x::TableStyleValues::FirstColumnStripe => PresetPivotDifferential::new().font(0, 0.0).bold(),
    x::TableStyleValues::FirstHeaderCell => PresetPivotDifferential::new()
      .font(0, 0.0)
      .bold()
      .fill(accent, 0.399_975_585_192_419_2),
    x::TableStyleValues::SecondSubtotalRow => {
      PresetPivotDifferential::new().bottom(accent, 0.599_993_896_298_104_8)
    }
    x::TableStyleValues::FirstSubtotalRow => PresetPivotDifferential::new()
      .font(1, 0.0)
      .bold()
      .fill(0, -0.149_998_474_074_526_2),
    x::TableStyleValues::FirstColumnSubheading => PresetPivotDifferential::new()
      .font(0, 0.0)
      .fill(accent, 0.399_975_585_192_419_2)
      .bottom(accent, 0.799_981_688_894_314_4)
      .horizontal(accent, 0.399_975_585_192_419_2),
    x::TableStyleValues::FirstRowSubheading => PresetPivotDifferential::new()
      .fill(accent, 0.799_981_688_894_314_4)
      .bottom(accent, 0.0),
    x::TableStyleValues::SecondRowSubheading | x::TableStyleValues::PageFieldLabels => {
      PresetPivotDifferential::new()
        .top(accent, 0.799_981_688_894_314_4)
        .bottom(accent, 0.799_981_688_894_314_4)
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
    _ => return None,
  };
  Some(dxf)
}
