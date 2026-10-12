//! Visio sheet lookup and stored-value authoring. Formulas stay text. This
//! crate does not evaluate them or draw pages.

mod cells;
mod connect;
mod drawing;
mod ids;
mod package;
mod sheet;
mod text;

pub use cells::{
  GeometryRow, RowAddress, SectionAddress, StoredGeometryRow, angle, append_geometry_row,
  append_layer, begin_x, begin_y, clipping_path, drawing_scale, end_x, end_y, fill_foregnd,
  fill_foregnd_trans, flip_x, flip_y, geometry_rows, height, img_height, img_offset_x,
  img_offset_y, img_width, layer_member, line_color, line_color_trans, line_weight, loc_pin_x,
  loc_pin_y, page_height, page_scale, page_width, pin_x, pin_y, read_direct_cell,
  read_page_direct_cell, read_page_section_cell, read_section_cell, width, write_direct_cell,
  write_hyperlink, write_page_direct_cell, write_page_section_cell, write_property,
  write_section_cell,
};
pub use connect::{ConnectError, ConnectLink, ResolvedConnect, add_connect, resolve_connects};
pub use drawing::minimal_drawing;
pub use ids::Ids;
pub use package::{
  AuthorRecord, CommentRecord, ImageBytes, add_author, add_comment, add_page, attach_image,
  authors, comment_entries, image_relationship_id, list_authors, list_comments, push_author,
  push_comment,
};
pub use sheet::{
  CellRecord, MasterShapes, StyleSlot, append_child_shape, effective_shape_cell, master_by_id,
  page_by_id, page_sheet_cell, shape_by_id, shape_by_id_mut, shape_cell, styled_shape_cell,
};
pub use text::{TextRun, read_shape_text, write_shape_text};
