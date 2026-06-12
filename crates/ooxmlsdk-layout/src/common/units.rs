#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Pt(pub f32);

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Twips(pub i32);

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd)]
pub struct Emu(pub i64);

impl Twips {
  pub fn to_pt(self) -> Pt {
    Pt(self.0 as f32 / 20.0)
  }
}

impl Emu {
  pub fn to_pt(self) -> Pt {
    Pt(self.0 as f32 / 12_700.0)
  }
}
