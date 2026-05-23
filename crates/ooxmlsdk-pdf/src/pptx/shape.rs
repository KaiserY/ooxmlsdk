use ooxmlsdk::schemas::schemas_openxmlformats_org_presentationml_2006_main as p;

use super::drawingml::shape::{Shape, ShapeService};
use super::drawingml::text_body::{TextBody, TextParagraph, TextRun, TextRunKind};
use super::drawingml::text_list_style::TextListStyle;
use super::slide::{ShapeLocation, SlidePersist};

const SKIP_PLACEHOLDER_INDEX: u32 = u32::MAX;

#[derive(Clone, Debug)]
pub(crate) struct PptShape {
  pub(crate) shape: Shape,
  pub(crate) model_id: Option<String>,
  pub(crate) shape_location: ShapeLocation,
  pub(crate) referenced: bool,
  pub(crate) placeholder: Option<Box<Shape>>,
  pub(crate) placeholder_reference_path: Option<Vec<usize>>,
  pub(crate) has_noninherited_shape_properties: bool,
}

impl PptShape {
  pub(crate) fn new(service_name: ShapeService, shape_location: ShapeLocation) -> Self {
    let mut shape = Shape::new(service_name);
    shape.shape_location = Some(shape_location);
    Self {
      shape,
      model_id: None,
      shape_location,
      referenced: false,
      placeholder: None,
      placeholder_reference_path: None,
      has_noninherited_shape_properties: false,
    }
  }

  pub(crate) fn add_shape(self, slide_persist: &mut SlidePersist) {
    // Source: LibreOffice oox/source/ppt/pptshape.cxx
    // PPTShape::addShape selects service names and applies placeholder text
    // styles before generic DrawingML createAndInsert.
    let shape = self.into_shape(slide_persist);
    slide_persist.shapes.push(shape);
  }

  pub(crate) fn into_shape(mut self, slide_persist: &SlidePersist) -> Shape {
    self.set_text_master_styles(slide_persist);
    self.apply_empty_placeholder_text(slide_persist);
    self.shape
  }

  pub(crate) fn apply_placeholder_reference(&mut self, slide_persist: &mut SlidePersist) {
    let Some(sub_type) = self.shape.sub_type else {
      return;
    };
    if !matches!(
      self.shape_location,
      ShapeLocation::Slide | ShapeLocation::Layout
    ) {
      return;
    }
    if self.shape.sub_type_index == Some(SKIP_PLACEHOLDER_INDEX) {
      return;
    }
    let (first, second) = placeholder_candidates(sub_type);
    let Some(first) = first else {
      return;
    };
    let master_only = self.shape_location == ShapeLocation::Layout;
    let Some((path, reference)) = find_placeholder_path(
      &slide_persist.shapes,
      first,
      second,
      self.shape.sub_type_index,
      master_only,
    ) else {
      return;
    };
    self.shape.apply_shape_reference(&reference);
    self.shape.set_placeholder(reference);
    mark_referenced(&mut slide_persist.shapes, &path);
    self.placeholder_reference_path = Some(path);
  }

  pub(crate) fn apply_graphic_placeholder_reference(&mut self, slide_persist: &mut SlidePersist) {
    let Some(sub_type) = self.shape.sub_type else {
      return;
    };
    if !matches!(
      self.shape_location,
      ShapeLocation::Slide | ShapeLocation::Layout
    ) {
      return;
    }
    if self.shape.sub_type_index == Some(SKIP_PLACEHOLDER_INDEX) {
      return;
    }
    let (first, second) = placeholder_candidates(sub_type);
    let Some(first) = first else {
      return;
    };
    let master_only = self.shape_location == ShapeLocation::Layout;
    let Some((path, reference)) = find_placeholder_path(
      &slide_persist.shapes,
      first,
      second,
      self.shape.sub_type_index,
      master_only,
    ) else {
      return;
    };
    self
      .shape
      .apply_shape_reference_with_text(&reference, false);
    self.shape.set_placeholder(reference);
    mark_referenced(&mut slide_persist.shapes, &path);
  }

  pub(crate) fn inherit_placeholder_type_by_index(&mut self, slide_persist: &SlidePersist) {
    if self.shape.sub_type.is_some() || self.shape.sub_type_index == Some(SKIP_PLACEHOLDER_INDEX) {
      return;
    }
    let Some(index) = self.shape.sub_type_index else {
      return;
    };
    if let Some(reference) = find_placeholder_by_index(&slide_persist.shapes, index, false) {
      self.shape.sub_type = reference.sub_type;
    }
  }

  pub(crate) fn placeholder_type_by_index(
    slide_persist: &SlidePersist,
    index: u32,
  ) -> Option<p::PlaceholderValues> {
    find_placeholder_by_index(&slide_persist.shapes, index, false).and_then(|shape| shape.sub_type)
  }

  pub(crate) fn get_sub_type_text_list_style<'a>(
    &self,
    slide_persist: &'a SlidePersist,
  ) -> Option<&'a TextListStyle> {
    slide_persist.get_sub_type_text_list_style(self.shape.sub_type)
  }

  pub(crate) fn is_placeholder_candidate(&self) -> bool {
    self.shape.sub_type.is_some() || self.shape.sub_type_index.is_some()
  }

  pub(crate) fn set_text_master_styles(&mut self, slide_persist: &SlidePersist) {
    // Source: LibreOffice oox/source/ppt/pptshape.cxx
    // PPTShape::addShape merges master text list style, placeholder text body
    // style, and the current shape text body style before shape creation.
    let mut style = TextListStyle::default();
    if let Some(master_style) = self.get_sub_type_text_list_style(slide_persist) {
      style.merge_from(master_style);
    }
    if let Some(placeholder) = &self.shape.placeholder {
      if let Some(placeholder_master_style) = &placeholder.master_text_list_style {
        style.merge_from(placeholder_master_style);
      }
      if let Some(placeholder_text_style) = placeholder
        .text_body
        .as_ref()
        .and_then(|text_body| text_body.list_style.as_ref())
      {
        style.merge_from(placeholder_text_style);
      }
    }
    if let Some(text_style) = self
      .shape
      .text_body
      .as_ref()
      .and_then(|text_body| text_body.list_style.as_ref())
    {
      style.merge_from(text_style);
    }
    if !style.is_empty() {
      self.shape.set_master_text_list_style(style);
    }
  }

  pub(crate) fn set_has_noninherited_shape_properties(&mut self) {
    self.has_noninherited_shape_properties = true;
  }

  fn apply_empty_placeholder_text(&mut self, slide_persist: &SlidePersist) {
    if self.shape_location != ShapeLocation::Slide || slide_persist.is_master {
      return;
    }
    let Some(prompt) = placeholder_prompt_text(self.shape.sub_type, slide_persist.is_notes) else {
      return;
    };
    let Some(text_body) = &mut self.shape.text_body else {
      return;
    };
    if !text_body_is_empty(text_body) {
      return;
    }
    // Source: LibreOffice sd/source/core/sdpage.cxx::GetPresObjText
    // creates localized prompt text for empty presentation objects.
    if text_body.paragraphs.is_empty() {
      text_body.paragraphs.push(TextParagraph::default());
    }
    let paragraph = text_body
      .paragraphs
      .first_mut()
      .expect("placeholder text body has a paragraph");
    paragraph.runs.push(TextRun {
      text: prompt.to_string(),
      kind: TextRunKind::Run,
      hyperlink_url: None,
      field_type: None,
      run_properties: None,
      field_paragraph_properties: None,
    });
  }
}

fn text_body_is_empty(text_body: &TextBody) -> bool {
  text_body
    .paragraphs
    .iter()
    .flat_map(|paragraph| paragraph.runs.iter())
    .all(|run| run.text.is_empty())
}

fn placeholder_prompt_text(
  sub_type: Option<p::PlaceholderValues>,
  is_notes: bool,
) -> Option<&'static str> {
  match (sub_type?, is_notes) {
    (p::PlaceholderValues::Title | p::PlaceholderValues::CenteredTitle, false) => {
      Some("Click to add Title")
    }
    (p::PlaceholderValues::Body | p::PlaceholderValues::Object, false) => Some("Click to add Text"),
    (p::PlaceholderValues::Body, true) => Some("Click to add Notes"),
    _ => None,
  }
}

fn placeholder_candidates(
  sub_type: p::PlaceholderValues,
) -> (Option<p::PlaceholderValues>, Option<p::PlaceholderValues>) {
  match sub_type {
    p::PlaceholderValues::CenteredTitle => (
      Some(p::PlaceholderValues::CenteredTitle),
      Some(p::PlaceholderValues::Title),
    ),
    p::PlaceholderValues::SubTitle => (
      Some(p::PlaceholderValues::SubTitle),
      Some(p::PlaceholderValues::Body),
    ),
    p::PlaceholderValues::Object => (
      Some(p::PlaceholderValues::Object),
      Some(p::PlaceholderValues::Body),
    ),
    p::PlaceholderValues::DateAndTime
    | p::PlaceholderValues::SlideNumber
    | p::PlaceholderValues::Footer
    | p::PlaceholderValues::Header
    | p::PlaceholderValues::Body
    | p::PlaceholderValues::Title
    | p::PlaceholderValues::Chart
    | p::PlaceholderValues::Table
    | p::PlaceholderValues::ClipArt
    | p::PlaceholderValues::Diagram
    | p::PlaceholderValues::Media
    | p::PlaceholderValues::SlideImage
    | p::PlaceholderValues::Picture => (Some(sub_type), None),
  }
}

fn find_placeholder_path(
  shapes: &[Shape],
  first: p::PlaceholderValues,
  second: Option<p::PlaceholderValues>,
  index: Option<u32>,
  master_only: bool,
) -> Option<(Vec<usize>, Shape)> {
  let mut choices: [Option<(Vec<usize>, Shape)>; 5] = Default::default();
  find_placeholder_candidates(
    shapes,
    first,
    second,
    index,
    master_only,
    &mut Vec::new(),
    &mut choices,
  );
  choices.into_iter().flatten().next()
}

fn find_placeholder_candidates(
  shapes: &[Shape],
  first: p::PlaceholderValues,
  second: Option<p::PlaceholderValues>,
  index: Option<u32>,
  master_only: bool,
  path: &mut Vec<usize>,
  choices: &mut [Option<(Vec<usize>, Shape)>; 5],
) {
  for (shape_index, shape) in shapes.iter().enumerate().rev() {
    path.push(shape_index);
    if !master_only || shape.shape_location == Some(ShapeLocation::Master) {
      add_placeholder_choice(shape, first, second, index, path, choices);
    }
    if !shape.children.is_empty() {
      find_placeholder_candidates(
        &shape.children,
        first,
        second,
        index,
        master_only,
        path,
        choices,
      );
      if choices[0].is_some() {
        path.pop();
        return;
      }
    }
    path.pop();
  }
}

fn add_placeholder_choice(
  shape: &Shape,
  first: p::PlaceholderValues,
  second: Option<p::PlaceholderValues>,
  index: Option<u32>,
  path: &[usize],
  choices: &mut [Option<(Vec<usize>, Shape)>; 5],
) {
  let same_first = shape.sub_type == Some(first);
  let same_second = second.is_some_and(|candidate| shape.sub_type == Some(candidate));
  let same_index = shape.sub_type_index == index;
  let priority = if same_index && same_first {
    Some(0)
  } else if !same_index && same_first {
    Some(1)
  } else if same_index && same_second {
    Some(2)
  } else if !same_index && same_second {
    Some(3)
  } else if same_index {
    Some(4)
  } else {
    None
  };
  if let Some(priority) = priority
    && choices[priority].is_none()
  {
    choices[priority] = Some((path.to_vec(), shape.clone()));
  }
}

fn find_placeholder_by_index(shapes: &[Shape], index: u32, master_only: bool) -> Option<&Shape> {
  for shape in shapes.iter().rev() {
    if shape.sub_type_index == Some(index)
      && (!master_only || shape.shape_location == Some(ShapeLocation::Master))
    {
      return Some(shape);
    }
    if let Some(reference) = find_placeholder_by_index(&shape.children, index, master_only) {
      return Some(reference);
    }
  }
  None
}

fn mark_referenced(shapes: &mut [Shape], path: &[usize]) {
  let Some((&index, rest)) = path.split_first() else {
    return;
  };
  let Some(shape) = shapes.get_mut(index) else {
    return;
  };
  if rest.is_empty() {
    shape.set_referenced(true);
  } else {
    mark_referenced(&mut shape.children, rest);
  }
}
