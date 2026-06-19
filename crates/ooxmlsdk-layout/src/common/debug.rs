use std::borrow::Cow;

use crate::common::{FrameId, Rect};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct DebugDump<'doc> {
  pub engine: Cow<'doc, str>,
  pub records: Vec<DebugRecord<'doc>>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum DebugRecord<'doc> {
  Page(DebugPage<'doc>),
  Frame(DebugFrame<'doc>),
  TextLine(DebugTextLine<'doc>),
  Cell(DebugCell<'doc>),
  Shape(DebugShape<'doc>),
  Unsupported(DebugUnsupported<'doc>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct DebugPage<'doc> {
  pub index: usize,
  pub name: Option<Cow<'doc, str>>,
  pub bounds: Rect,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DebugFrame<'doc> {
  pub id: FrameId,
  pub parent: Option<FrameId>,
  pub kind: Cow<'doc, str>,
  pub bounds: Rect,
  pub print_bounds: Rect,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DebugTextLine<'doc> {
  pub frame: FrameId,
  pub index: usize,
  pub text: Cow<'doc, str>,
  pub bounds: Rect,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DebugCell<'doc> {
  pub sheet: Cow<'doc, str>,
  pub reference: Cow<'doc, str>,
  pub bounds: Rect,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DebugShape<'doc> {
  pub page_index: usize,
  pub path: Vec<usize>,
  pub kind: Cow<'doc, str>,
  pub bounds: Rect,
  pub metadata: Vec<DebugProperty<'doc>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DebugUnsupported<'doc> {
  pub owner: Cow<'doc, str>,
  pub feature: Cow<'doc, str>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DebugProperty<'doc> {
  pub name: Cow<'doc, str>,
  pub value: DebugValue<'doc>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum DebugValue<'doc> {
  Bool(bool),
  Integer(i64),
  Float(f32),
  Text(Cow<'doc, str>),
}
