//! Settled line-owned pictures during a cell's logical paragraph reflow.
use super::*;

#[derive(Clone, Copy)]
struct Picture {
  inline: usize,
  position: (f32, f32),
  exclusion: WrapExclusion,
}

#[derive(Default)]
pub(super) struct Plan {
  pictures: Vec<Picture>,
}

pub(super) fn owns(placement: FloatingImagePlacement, flow: FlowContext) -> bool {
  cell_owned_drawingml_object(placement, flow)
    && flow.layout_cell_bounds.is_some()
    // The table owns pagination of this complete logical cell. Bounded row
    // fragments keep their separate physical-page replay owner.
    && flow.content_bottom >= UNBOUNDED_LAYOUT_EXTENT_PT
    && placement.vertical_relative_to == VerticalImageReference::Line
    && placement.horizontal_relative_to != HorizontalImageReference::Character
    && matches!(placement.wrap, ImageWrapMode::Square | ImageWrapMode::Tight)
}

impl Plan {
  pub(super) fn position(&self, inline: usize) -> Option<(f32, f32)> {
    self
      .pictures
      .binary_search_by_key(&inline, |picture| picture.inline)
      .ok()
      .map(|index| self.pictures[index].position)
  }

  pub(super) fn exclusions(&self) -> impl Iterator<Item = WrapExclusion> + '_ {
    self.pictures.iter().map(|picture| picture.exclusion)
  }

  pub(super) fn capture(
    &mut self,
    inline: usize,
    position: (f32, f32),
    exclusion: WrapExclusion,
  ) -> bool {
    let Err(index) = self
      .pictures
      .binary_search_by_key(&inline, |picture| picture.inline)
    else {
      return false;
    };
    self.pictures.insert(
      index,
      Picture {
        inline,
        position,
        exclusion,
      },
    );
    true
  }
}
