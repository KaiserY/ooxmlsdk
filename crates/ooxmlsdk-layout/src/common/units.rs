#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Pt(pub f32);

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Twips(pub i32);

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd)]
pub struct Emu(pub i64);

impl Twips {
  pub fn to_pt(self) -> Pt {
    Pt::from_f64(f64::from(self.0) / ooxmlsdk::units::TWIPS_PER_POINT as f64)
  }
}

impl Emu {
  pub fn to_pt(self) -> Pt {
    Pt::from_f64(ooxmlsdk::units::emu_to_points(self.0))
  }
}

impl Pt {
  /// Narrows a completed high-precision geometry calculation to the layout
  /// and renderer scalar used by Parley and Krilla.
  ///
  /// Valid OOXML coordinates are many orders of magnitude below `f32::MAX`.
  /// Keeping this conversion here makes the Kurbo (`f64`) to display-list
  /// (`f32`) boundary explicit and prevents repeated promotion/narrowing
  /// inside geometry algorithms.
  pub(crate) fn from_f64(value: f64) -> Self {
    debug_assert!(value.is_finite());
    debug_assert!(value >= f64::from(f32::MIN) && value <= f64::from(f32::MAX));
    Self(value as f32)
  }
}

impl From<Pt> for f64 {
  fn from(value: Pt) -> Self {
    Self::from(value.0)
  }
}

pub(crate) fn layout_scalar_from_f64(value: f64) -> f32 {
  Pt::from_f64(value).0
}
