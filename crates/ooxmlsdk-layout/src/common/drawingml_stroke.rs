use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;

use crate::common::{
  Pt, Stroke, StrokeAlignment, StrokeCap, StrokeCompound, StrokeDashPreset, StrokeEnd,
  StrokeEndKind, StrokeEndSize, StrokeJoin,
};

pub(crate) fn apply_outline_style(stroke: &mut Stroke<'_>, outline: &a::Outline) {
  stroke.cap = outline.cap_type.map(line_cap);
  stroke.compound = outline.compound_line_type.map(compound);
  stroke.alignment = outline.alignment.map(alignment);
  match outline.outline_choice2.as_ref() {
    Some(a::OutlineChoice2::PresetDash(dash)) => {
      stroke.preset_dash = dash.val.map(preset_dash);
      stroke.dash = None;
    }
    Some(a::OutlineChoice2::CustomDash(dash)) => {
      stroke.preset_dash = None;
      stroke.dash = Some(
        dash
          .dash_stop
          .iter()
          .flat_map(|stop| {
            [
              Pt(stroke.width.0 * stop.dash_length.as_ratio() as f32),
              Pt(stroke.width.0 * stop.space_length.as_ratio() as f32),
            ]
          })
          .collect(),
      );
    }
    None => {}
  }
  stroke.join = match outline.outline_choice3.as_ref() {
    Some(a::OutlineChoice3::Round) => Some(StrokeJoin::Round),
    Some(a::OutlineChoice3::LineJoinBevel) => Some(StrokeJoin::Bevel),
    Some(a::OutlineChoice3::Miter(miter)) => Some(StrokeJoin::Miter {
      limit: miter.limit.map(|limit| limit.as_ratio() as f32),
    }),
    None => None,
  };
  stroke.head_end = outline.head_end.as_ref().map(|end| StrokeEnd {
    kind: end.r#type.map(line_end).unwrap_or(StrokeEndKind::None),
    width: end.width.map(end_width).unwrap_or(StrokeEndSize::Medium),
    length: end.length.map(end_length).unwrap_or(StrokeEndSize::Medium),
  });
  stroke.tail_end = outline.tail_end.as_ref().map(|end| StrokeEnd {
    kind: end.r#type.map(line_end).unwrap_or(StrokeEndKind::None),
    width: end.width.map(end_width).unwrap_or(StrokeEndSize::Medium),
    length: end.length.map(end_length).unwrap_or(StrokeEndSize::Medium),
  });
}

fn line_cap(value: a::LineCapValues) -> StrokeCap {
  match value {
    a::LineCapValues::Round => StrokeCap::Round,
    a::LineCapValues::Square => StrokeCap::Square,
    a::LineCapValues::Flat => StrokeCap::Flat,
  }
}

fn compound(value: a::CompoundLineValues) -> StrokeCompound {
  match value {
    a::CompoundLineValues::Single => StrokeCompound::Single,
    a::CompoundLineValues::Double => StrokeCompound::Double,
    a::CompoundLineValues::ThickThin => StrokeCompound::ThickThin,
    a::CompoundLineValues::ThinThick => StrokeCompound::ThinThick,
    a::CompoundLineValues::Triple => StrokeCompound::Triple,
  }
}

fn alignment(value: a::PenAlignmentValues) -> StrokeAlignment {
  match value {
    a::PenAlignmentValues::Center => StrokeAlignment::Center,
    a::PenAlignmentValues::Insert => StrokeAlignment::Inside,
  }
}

fn preset_dash(value: a::PresetLineDashValues) -> StrokeDashPreset {
  match value {
    a::PresetLineDashValues::Solid => StrokeDashPreset::Solid,
    a::PresetLineDashValues::Dot => StrokeDashPreset::Dot,
    a::PresetLineDashValues::Dash => StrokeDashPreset::Dash,
    a::PresetLineDashValues::LargeDash => StrokeDashPreset::LargeDash,
    a::PresetLineDashValues::DashDot => StrokeDashPreset::DashDot,
    a::PresetLineDashValues::LargeDashDot => StrokeDashPreset::LargeDashDot,
    a::PresetLineDashValues::LargeDashDotDot => StrokeDashPreset::LargeDashDotDot,
    a::PresetLineDashValues::SystemDash => StrokeDashPreset::SystemDash,
    a::PresetLineDashValues::SystemDot => StrokeDashPreset::SystemDot,
    a::PresetLineDashValues::SystemDashDot => StrokeDashPreset::SystemDashDot,
    a::PresetLineDashValues::SystemDashDotDot => StrokeDashPreset::SystemDashDotDot,
  }
}

fn line_end(value: a::LineEndValues) -> StrokeEndKind {
  match value {
    a::LineEndValues::None => StrokeEndKind::None,
    a::LineEndValues::Triangle => StrokeEndKind::Triangle,
    a::LineEndValues::Stealth => StrokeEndKind::Stealth,
    a::LineEndValues::Diamond => StrokeEndKind::Diamond,
    a::LineEndValues::Oval => StrokeEndKind::Oval,
    a::LineEndValues::Arrow => StrokeEndKind::Arrow,
  }
}

fn end_width(value: a::LineEndWidthValues) -> StrokeEndSize {
  match value {
    a::LineEndWidthValues::Small => StrokeEndSize::Small,
    a::LineEndWidthValues::Medium => StrokeEndSize::Medium,
    a::LineEndWidthValues::Large => StrokeEndSize::Large,
  }
}

fn end_length(value: a::LineEndLengthValues) -> StrokeEndSize {
  match value {
    a::LineEndLengthValues::Small => StrokeEndSize::Small,
    a::LineEndLengthValues::Medium => StrokeEndSize::Medium,
    a::LineEndLengthValues::Large => StrokeEndSize::Large,
  }
}
