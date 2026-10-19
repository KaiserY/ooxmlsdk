//! Physical cell frames and the logical axes of rotated table text.
//!
//! Word's btLr/tbRl table stories rotate about opposite corners. Their line
//! width belongs to the physical height; vAlign places the resulting block
//! across the physical width, including negative slack for overflowing text.

use super::{FrameBounds, TableCellVerticalAlignment};
use crate::docx::CellMargins;

#[derive(Clone, Copy, Debug)]
pub(super) struct VerticalCellFlow {
  pub pivot_x_pt: f32,
  pub pivot_y_pt: f32,
  pub line_width_pt: f32,
  block_extent_pt: f32,
}

#[derive(Clone, Copy)]
pub(super) struct CellBorders {
  pub left_pt: f32,
  pub right_pt: f32,
  pub top_pt: f32,
  pub bottom_pt: f32,
  pub separate: bool,
}

impl VerticalCellFlow {
  pub fn new(
    frame: FrameBounds,
    rotation_deg: f32,
    margins: CellMargins,
    borders: CellBorders,
  ) -> Self {
    let factor = if borders.separate { 1.0 } else { 0.5 };
    let left = borders.left_pt * factor;
    let right = borders.right_pt * factor;
    let top = frame.y_pt + margins.top_pt + borders.top_pt * factor;
    let bottom = frame.y_pt + frame.height_pt - margins.bottom_pt - borders.bottom_pt * factor;
    let start_inset = |padding: f32, border: f32| {
      if borders.separate {
        padding + border
      } else {
        padding.max(border)
      }
    };
    let (pivot_x_pt, pivot_y_pt, block_extent_pt) = if rotation_deg < 0.0 {
      let start = start_inset(margins.left_pt, left);
      (frame.x_pt + start, bottom, frame.width_pt - start - right)
    } else {
      let start = start_inset(margins.right_pt, right);
      (
        frame.x_pt + frame.width_pt - start,
        top,
        frame.width_pt - start - left,
      )
    };
    Self {
      pivot_x_pt,
      pivot_y_pt,
      line_width_pt: (bottom - top).max(0.0),
      block_extent_pt: block_extent_pt.max(0.0),
    }
  }

  pub fn block_offset(self, alignment: TableCellVerticalAlignment, advance_pt: f32) -> f32 {
    let slack = self.block_extent_pt - advance_pt;
    match alignment {
      TableCellVerticalAlignment::Top => 0.0,
      TableCellVerticalAlignment::Center => slack / 2.0,
      TableCellVerticalAlignment::Bottom => slack,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn rotated_cell_uses_opposite_corners_and_preserves_overflow_alignment() {
    let frame = FrameBounds {
      x_pt: 10.0,
      y_pt: 20.0,
      width_pt: 50.0,
      height_pt: 120.0,
    };
    let margins = CellMargins::zero();
    let borders = CellBorders {
      left_pt: 1.0,
      right_pt: 1.0,
      top_pt: 1.0,
      bottom_pt: 1.0,
      separate: false,
    };
    let upward = VerticalCellFlow::new(frame, -90.0, margins, borders);
    let downward = VerticalCellFlow::new(frame, 90.0, margins, borders);
    assert_eq!((upward.pivot_x_pt, upward.pivot_y_pt), (10.5, 139.5));
    assert_eq!((downward.pivot_x_pt, downward.pivot_y_pt), (59.5, 20.5));
    assert_eq!(upward.line_width_pt, 119.0);
    assert_eq!(downward.line_width_pt, 119.0);
    assert_eq!(
      upward.block_offset(TableCellVerticalAlignment::Center, 69.0),
      -10.0
    );
    assert_eq!(
      downward.block_offset(TableCellVerticalAlignment::Bottom, 69.0),
      -20.0
    );
  }
}
