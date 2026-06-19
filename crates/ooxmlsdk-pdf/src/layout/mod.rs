use ooxmlsdk_layout::common;

pub(crate) use ooxmlsdk_layout::compat::DynamicFieldKind;
pub(crate) use ooxmlsdk_layout::compat::{
  FillItem, FollowFrameKind, FormWidget, FormWidgetKind, ImageCrop, ImageItem, LayoutDocument,
  LineBox, LineItem, LineItemKind, LinkAreaItem, OutlineEntry, Page, PageItem, PageSetup,
  PdfTextSegmentation, PolylineItem, RectItem, RgbColor, TextItem, TextStyle,
};

pub(crate) fn from_common_document(document: common::LayoutDocument<'static>) -> LayoutDocument {
  let pages: Vec<Page> = document.pages.into_iter().map(page_from_common).collect();
  let frames = document
    .frames
    .into_iter()
    .map(|frame| frame_from_common(frame, &pages))
    .collect();
  LayoutDocument {
    pages,
    form_widgets: document
      .form_widgets
      .into_iter()
      .map(form_widget_from_common)
      .collect(),
    follows: document
      .follows
      .into_iter()
      .map(follow_from_common)
      .collect(),
    frames,
    outline_entries: document
      .outline_entries
      .into_iter()
      .map(outline_from_common)
      .collect(),
    page_replays: document
      .reflow
      .page_replays
      .into_iter()
      .map(page_replay_from_common)
      .collect(),
    page_replay_applications: document
      .reflow
      .page_replay_applications
      .into_iter()
      .map(page_replay_application_from_common)
      .collect(),
    backward_moves: document
      .reflow
      .backward_moves
      .into_iter()
      .map(backward_move_from_common)
      .collect(),
    layout_reruns: document
      .reflow
      .layout_reruns
      .into_iter()
      .map(layout_rerun_from_common)
      .collect(),
    page_invalidations: document
      .reflow
      .page_invalidations
      .into_iter()
      .map(page_invalidation_from_common)
      .collect(),
    reflow_executions: document
      .reflow
      .reflow_executions
      .into_iter()
      .map(reflow_execution_from_common)
      .collect(),
    reflow_requests: document
      .reflow
      .reflow_requests
      .into_iter()
      .map(reflow_request_from_common)
      .collect(),
    restart_plan: document.reflow.restart_plan.map(restart_plan_from_common),
  }
}

fn page_from_common(page: common::DisplayPage<'static>) -> Page {
  Page {
    setup: page_setup_from_common(page.setup),
    section_index: page.section_index,
    section_page_index: page.section_page_index,
    items: page
      .items
      .into_iter()
      .filter_map(page_item_from_common)
      .collect(),
  }
}

fn page_item_from_common(item: common::DisplayItem<'static>) -> Option<PageItem> {
  match item {
    common::DisplayItem::Text(text) => Some(PageItem::Text(text_item_from_common(text))),
    common::DisplayItem::Image(image) => Some(PageItem::Image(image_item_from_common(image))),
    common::DisplayItem::Path(path) => Some(PageItem::Polyline(polyline_from_common(path))),
    common::DisplayItem::Rect(rect) => Some(PageItem::Rect(rect_item_from_common(rect))),
    common::DisplayItem::Line(line) => Some(PageItem::Line(line_item_from_common(line))),
    common::DisplayItem::LinkArea(link) => Some(PageItem::LinkArea(link_area_from_common(link))),
    common::DisplayItem::Glyphs(_)
    | common::DisplayItem::AnnotationHint(_)
    | common::DisplayItem::Clip(_)
    | common::DisplayItem::Transform(_) => None,
  }
}

fn text_item_from_common(text: common::TextRun<'static>) -> TextItem {
  TextItem {
    x_pt: text.origin.x.0,
    y_pt: text.origin.y.0,
    line_height_pt: text.line_height.0,
    text: text.text.into_owned(),
    style: text_style_from_common(text.style),
    rotation_center_pt: text.rotation_center.map(|point| (point.x.0, point.y.0)),
    hyperlink_url: text.hyperlink_url.map(|url| url.into_owned()),
    dynamic_field: text.dynamic_field.map(dynamic_field_from_common),
    style_ref_keys: Vec::new(),
    style_ref_text: None,
    form_widget_id: text.form_widget_id,
    paragraph_bidi: text.paragraph_bidi,
    preserve_text_portion: text.preserve_text_portion,
    decoration_span_start_x_pt: None,
    pdf_text_segmentation: match text.pdf_text_segmentation {
      common::PdfTextSegmentation::Line => PdfTextSegmentation::Line,
      common::PdfTextSegmentation::Portion => PdfTextSegmentation::Portion,
    },
  }
}

fn image_item_from_common(image: common::ImageItem<'static>) -> ImageItem {
  ImageItem {
    x_pt: image.bounds.origin.x.0,
    y_pt: image.bounds.origin.y.0,
    width_pt: image.bounds.size.width.0,
    height_pt: image.bounds.size.height.0,
    crop: image
      .crop
      .map(|crop| ImageCrop {
        left: crop.left,
        top: crop.top,
        right: crop.right,
        bottom: crop.bottom,
      })
      .unwrap_or_default(),
    rotation_deg: image.rotation_degrees,
    flip_horizontal: image.flip_horizontal,
    flip_vertical: image.flip_vertical,
    data: image.bytes.into_owned(),
    content_type: Some(image.content_type.into_owned()),
    alt_text: image.alt_text.map(|text| text.into_owned()),
    hyperlink_url: image.hyperlink_url.map(|url| url.into_owned()),
    floating: image.floating,
    behind_text: image.behind_text,
  }
}

fn polyline_from_common(path: common::PathItem<'static>) -> PolylineItem {
  let x_pt = path.bounds.origin.x.0;
  let y_pt = path.bounds.origin.y.0;
  PolylineItem {
    x_pt,
    y_pt,
    width_pt: path.bounds.size.width.0,
    height_pt: path.bounds.size.height.0,
    points: path
      .points
      .into_iter()
      .map(|point| (point.x.0 - x_pt, point.y.0 - y_pt))
      .collect(),
    closed: path.closed,
    fill_color: solid_rgb(path.fill),
    stroke: path.stroke.map(stroke_from_common),
  }
}

fn rect_item_from_common(rect: common::RectItem<'static>) -> RectItem {
  let (fill_color, fill_opacity) = solid_color(rect.fill)
    .map(|color| (Some(rgb(color)), opacity(color)))
    .unwrap_or((None, 1.0));
  RectItem {
    x_pt: rect.bounds.origin.x.0,
    y_pt: rect.bounds.origin.y.0,
    width_pt: rect.bounds.size.width.0,
    height_pt: rect.bounds.size.height.0,
    fill_color,
    fill_opacity,
    stroke: rect.stroke.clone().map(stroke_from_common),
    stroke_opacity: rect.stroke.map_or(1.0, |stroke| opacity(stroke.color)),
  }
}

fn line_item_from_common(line: common::LineItem<'static>) -> LineItem {
  LineItem {
    x1_pt: line.start.x.0,
    y1_pt: line.start.y.0,
    x2_pt: line.end.x.0,
    y2_pt: line.end.y.0,
    width_pt: line.stroke.width.0,
    color: rgb(line.stroke.color),
    kind: match line.kind {
      common::LineKind::Stroke => LineItemKind::Stroke,
      common::LineKind::FilledRect => LineItemKind::FilledRect,
    },
  }
}

fn link_area_from_common(link: common::LinkArea<'static>) -> LinkAreaItem {
  LinkAreaItem {
    x_pt: link.bounds.origin.x.0,
    y_pt: link.bounds.origin.y.0,
    width_pt: link.bounds.size.width.0,
    height_pt: link.bounds.size.height.0,
    hyperlink_url: link.target.into_owned(),
  }
}

fn page_setup_from_common(setup: common::PageSetup) -> PageSetup {
  PageSetup {
    width_pt: setup.size.width.0,
    height_pt: setup.size.height.0,
    margin_top_pt: setup.margins.top.0,
    margin_right_pt: setup.margins.right.0,
    margin_bottom_pt: setup.margins.bottom.0,
    margin_left_pt: setup.margins.left.0,
    mirror_margins: setup.mirror_margins,
    top_margin_was_negative: setup.top_margin_was_negative,
    bottom_margin_was_negative: setup.bottom_margin_was_negative,
    header_distance_pt: setup.header_distance.0,
    footer_distance_pt: setup.footer_distance.0,
    background: setup.background.map(rgb),
    borders: ooxmlsdk_layout::compat::CellBordersModel {
      top: setup.borders.top.map(border_style_from_common),
      right: setup.borders.right.map(border_style_from_common),
      bottom: setup.borders.bottom.map(border_style_from_common),
      left: setup.borders.left.map(border_style_from_common),
    },
    borders_offset_from_text: setup.borders_offset_from_text,
    line_numbering: setup
      .line_numbering
      .map(|line| ooxmlsdk_layout::compat::LineNumbering {
        count_by: line.count_by,
        start: line.start,
        distance_pt: line.distance.0,
        restart_each_page: line.restart_each_page,
      }),
    doc_grid_line_pitch_pt: setup.doc_grid_line_pitch.map(|pitch| pitch.0),
    page_number_start: setup.page_number_start,
  }
}

fn text_style_from_common(style: common::TextStyle<'static>) -> TextStyle {
  TextStyle {
    font_family: style.font_family.map(|value| value.into_owned().into()),
    symbol_font_family: style
      .symbol_font_family
      .map(|value| value.into_owned().into()),
    font_size_pt: style.font_size.0,
    complex_font_size_pt: style.complex_font_size.map(|size| size.0),
    character_spacing_pt: style.character_spacing.0,
    baseline_shift_pt: style.baseline_shift.0,
    bold: style.bold,
    italic: style.italic,
    underline: style.underline,
    strikethrough: style.strikethrough,
    uppercase: style.uppercase,
    small_caps: style.small_caps,
    hidden: style.hidden,
    rotation_deg: style.rotation_degrees,
    color: rgb(style.color),
    opacity: opacity(style.color),
    outline_color: style.outline_color.map(rgb),
    outline_opacity: style.outline_color.map_or(1.0, opacity),
    outline_width_pt: style.outline_width.0,
    highlight: style.highlight.map(rgb),
    underline_color: style.underline_color.map(rgb),
  }
}

fn dynamic_field_from_common(
  field: common::DynamicField<'static>,
) -> ooxmlsdk_layout::compat::DynamicFieldKind {
  match field {
    common::DynamicField::Page => ooxmlsdk_layout::compat::DynamicFieldKind::Page,
    common::DynamicField::NumPages => ooxmlsdk_layout::compat::DynamicFieldKind::NumPages,
    common::DynamicField::StyleRef {
      style_name,
      from_bottom,
    } => ooxmlsdk_layout::compat::DynamicFieldKind::StyleRef {
      style_name: style_name.into_owned().into(),
      from_bottom,
    },
  }
}

fn form_widget_from_common(widget: common::FormWidget<'static>) -> FormWidget {
  FormWidget {
    id: widget.id,
    kind: match widget.kind {
      common::FormWidgetKind::Text => FormWidgetKind::Text,
      common::FormWidgetKind::DropDownList => FormWidgetKind::DropDownList,
      common::FormWidgetKind::ComboBox => FormWidgetKind::ComboBox,
    },
    entries: widget
      .entries
      .into_iter()
      .map(|entry| entry.into_owned())
      .collect(),
  }
}

fn outline_from_common(entry: common::OutlineEntry<'static>) -> OutlineEntry {
  OutlineEntry {
    level: entry.level,
    text: entry.text.into_owned(),
    page_index: entry.page_index,
    x_pt: entry.target.x.0,
    y_pt: entry.target.y.0,
    merged_hidden_separator: entry.merged_hidden_separator,
  }
}

fn follow_from_common(follow: common::FrameFollow) -> ooxmlsdk_layout::compat::FrameFollow {
  ooxmlsdk_layout::compat::FrameFollow {
    kind: frame_kind_from_common(follow.kind),
    reason: match follow.reason {
      common::FollowReason::KeepTogether => ooxmlsdk_layout::compat::FollowReason::KeepTogether,
      common::FollowReason::Overflow => ooxmlsdk_layout::compat::FollowReason::Overflow,
      common::FollowReason::ExplicitBreak => ooxmlsdk_layout::compat::FollowReason::ExplicitBreak,
    },
    block_index: follow.block_index,
    from_page_index: follow.from_page_index,
    to_page_index: follow.to_page_index,
    from_section_page_index: follow.from_section_page_index,
    to_section_page_index: follow.to_section_page_index,
    from_column_index: follow.from_column_index,
    to_column_index: follow.to_column_index,
  }
}

fn frame_from_common(
  frame: common::FrameRecord<'static>,
  pages: &[Page],
) -> ooxmlsdk_layout::compat::LayoutFrame {
  let items = pages
    .get(frame.page_index)
    .map(|page| {
      let start = frame.item_range.start.min(page.items.len());
      let end = frame.item_range.end.min(page.items.len());
      page.items[start..end].to_vec()
    })
    .unwrap_or_default();
  ooxmlsdk_layout::compat::LayoutFrame {
    kind: frame_kind_name_from_common(&frame.kind),
    block_index: frame.block_index,
    split_start: frame_cursor_from_common(frame.split_start),
    split_end: frame_cursor_from_common(frame.split_end),
    page_index: frame.page_index,
    section_index: frame.section_index,
    section_page_index: frame.section_page_index,
    column_index: frame.column_index,
    items,
    item_start: frame.item_range.start,
    item_end: frame.item_range.end,
    bounds: frame.bounds.map(frame_bounds_from_common),
    lines: frame.lines.into_iter().map(line_box_from_common).collect(),
    fragments: frame
      .fragments
      .into_iter()
      .map(frame_fragment_from_common)
      .collect(),
    influences: frame
      .influences
      .into_iter()
      .map(frame_influence_from_common)
      .collect(),
    invalidation: match frame.invalidation {
      common::FrameInvalidation::Clean => ooxmlsdk_layout::compat::FrameInvalidation::Clean,
      common::FrameInvalidation::PageItemsDecorated => {
        ooxmlsdk_layout::compat::FrameInvalidation::PageItemsDecorated
      }
      common::FrameInvalidation::NeedsReflow => {
        ooxmlsdk_layout::compat::FrameInvalidation::NeedsReflow
      }
    },
  }
}

fn frame_kind_from_common(kind: common::FrameKind) -> FollowFrameKind {
  match kind {
    common::FrameKind::Paragraph => FollowFrameKind::Paragraph,
    common::FrameKind::Table => FollowFrameKind::Table,
    common::FrameKind::Notes => FollowFrameKind::Notes,
  }
}

fn frame_kind_name_from_common(kind: &str) -> FollowFrameKind {
  match kind {
    "table" => FollowFrameKind::Table,
    "notes" => FollowFrameKind::Notes,
    _ => FollowFrameKind::Paragraph,
  }
}

fn frame_cursor_from_common(cursor: common::FrameCursor) -> ooxmlsdk_layout::compat::FrameCursor {
  ooxmlsdk_layout::compat::FrameCursor {
    block_index: cursor.block_index,
    kind: match cursor.kind {
      common::FrameCursorKind::BlockStart => ooxmlsdk_layout::compat::FrameCursorKind::BlockStart,
      common::FrameCursorKind::Inline => ooxmlsdk_layout::compat::FrameCursorKind::Inline,
      common::FrameCursorKind::TableRow => ooxmlsdk_layout::compat::FrameCursorKind::TableRow,
      common::FrameCursorKind::TableCell => ooxmlsdk_layout::compat::FrameCursorKind::TableCell,
      common::FrameCursorKind::BlockEnd => ooxmlsdk_layout::compat::FrameCursorKind::BlockEnd,
    },
    inline_index: cursor.inline_index,
    text_offset: cursor.text_offset,
    row_index: cursor.row_index,
    cell_index: cursor.cell_index,
  }
}

fn line_box_from_common(line: common::LineBox) -> LineBox {
  LineBox {
    x_pt: line.bounds.origin.x.0,
    y_pt: line.bounds.origin.y.0,
    width_pt: line.bounds.size.width.0,
    height_pt: line.bounds.size.height.0,
    item_start: line.item_range.start,
    item_end: line.item_range.end,
  }
}

fn frame_fragment_from_common(
  fragment: common::FrameFragment,
) -> ooxmlsdk_layout::compat::FrameFragment {
  ooxmlsdk_layout::compat::FrameFragment {
    kind: match fragment.kind {
      common::FrameFragmentKind::ParagraphLine => {
        ooxmlsdk_layout::compat::FrameFragmentKind::ParagraphLine
      }
      common::FrameFragmentKind::TableRow => ooxmlsdk_layout::compat::FrameFragmentKind::TableRow,
      common::FrameFragmentKind::TableCell => ooxmlsdk_layout::compat::FrameFragmentKind::TableCell,
      common::FrameFragmentKind::NoteLine => ooxmlsdk_layout::compat::FrameFragmentKind::NoteLine,
    },
    split: match fragment.split {
      common::FragmentSplitKind::Complete => ooxmlsdk_layout::compat::FragmentSplitKind::Complete,
      common::FragmentSplitKind::Master => ooxmlsdk_layout::compat::FragmentSplitKind::Master,
      common::FragmentSplitKind::Follow => ooxmlsdk_layout::compat::FragmentSplitKind::Follow,
      common::FragmentSplitKind::RepeatedHeader => {
        ooxmlsdk_layout::compat::FragmentSplitKind::RepeatedHeader
      }
    },
    index: fragment.index,
    row_index: fragment.row_index,
    cell_index: fragment.cell_index,
    item_start: fragment.item_range.start,
    item_end: fragment.item_range.end,
    bounds: fragment.bounds.map(frame_bounds_from_common),
  }
}

fn frame_influence_from_common(
  influence: common::FrameInfluence,
) -> ooxmlsdk_layout::compat::FrameInfluence {
  ooxmlsdk_layout::compat::FrameInfluence {
    kind: influence_kind_from_common(influence.kind),
    count: influence.count,
    block_index: influence.block_index,
    item_start: influence.item_range.start,
    item_end: influence.item_range.end,
    bounds: influence.bounds.map(frame_bounds_from_common),
  }
}

fn influence_kind_from_common(
  kind: common::FrameInfluenceKind,
) -> ooxmlsdk_layout::compat::FrameInfluenceKind {
  match kind {
    common::FrameInfluenceKind::FootnoteReservation => {
      ooxmlsdk_layout::compat::FrameInfluenceKind::FootnoteReservation
    }
    common::FrameInfluenceKind::FlyWrap => ooxmlsdk_layout::compat::FrameInfluenceKind::FlyWrap,
    common::FrameInfluenceKind::TableSplit => {
      ooxmlsdk_layout::compat::FrameInfluenceKind::TableSplit
    }
  }
}

fn frame_bounds_from_common(bounds: common::Rect) -> ooxmlsdk_layout::compat::FrameBounds {
  ooxmlsdk_layout::compat::FrameBounds {
    x_pt: bounds.origin.x.0,
    y_pt: bounds.origin.y.0,
    width_pt: bounds.size.width.0,
    height_pt: bounds.size.height.0,
  }
}

fn page_replay_from_common(
  replay: common::PageReplay<'static>,
) -> ooxmlsdk_layout::compat::PageReplay {
  ooxmlsdk_layout::compat::PageReplay {
    page_index: replay.page_index,
    section_page_index: replay.section_page_index,
    column_index: replay.column_index,
    scope: reflow_scope_from_common(replay.scope),
    item_start: replay.item_range.start,
    item_end: replay.item_range.end,
    replacement_items: replay
      .replacement_items
      .into_iter()
      .filter_map(page_item_from_common)
      .collect(),
  }
}

fn page_replay_application_from_common(
  application: common::PageReplayApplication,
) -> ooxmlsdk_layout::compat::PageReplayApplication {
  ooxmlsdk_layout::compat::PageReplayApplication {
    page_index: application.page_index,
    section_page_index: application.section_page_index,
    column_index: application.column_index,
    scope: reflow_scope_from_common(application.scope),
    item_start: application.item_range.start,
    item_end: application.item_range.end,
    replacement_count: application.replacement_count,
    applied: application.applied,
  }
}

fn backward_move_from_common(
  move_back: common::BackwardMove,
) -> ooxmlsdk_layout::compat::BackwardMove {
  ooxmlsdk_layout::compat::BackwardMove {
    frame_index: move_back.frame_index,
    replay_start_frame_index: move_back.replay_start_frame_index,
    from_page_index: move_back.from_page_index,
    to_page_index: move_back.to_page_index,
    from_section_page_index: move_back.from_section_page_index,
    to_section_page_index: move_back.to_section_page_index,
    scope: reflow_scope_from_common(move_back.scope),
    reason: reflow_reason_from_common(move_back.reason),
    suppressed: move_back.suppressed,
    replayed_frames: move_back.replayed_frames,
    replayed_items: move_back.replayed_items,
  }
}

fn layout_rerun_from_common(rerun: common::LayoutRerun) -> ooxmlsdk_layout::compat::LayoutRerun {
  ooxmlsdk_layout::compat::LayoutRerun {
    checkpoint_index: rerun.checkpoint_index,
    section_index: rerun.section_index,
    block_index: rerun.block_index,
    page_index: rerun.page_index,
    frame_index: rerun.frame_index,
    reason: reflow_reason_from_common(rerun.reason),
    scope: reflow_scope_from_common(rerun.scope),
    replaced_pages: rerun.replaced_pages,
    produced_pages: rerun.produced_pages,
    produced_frames: rerun.produced_frames,
    constraints: rerun
      .constraints
      .into_iter()
      .map(layout_rerun_constraint_from_common)
      .collect(),
  }
}

fn layout_rerun_constraint_from_common(
  constraint: common::LayoutRerunConstraint,
) -> ooxmlsdk_layout::compat::LayoutRerunConstraint {
  ooxmlsdk_layout::compat::LayoutRerunConstraint {
    kind: influence_kind_from_common(constraint.kind),
    scope: reflow_scope_from_common(constraint.scope),
    bounds: constraint.bounds.map(frame_bounds_from_common),
    content_left_pt: constraint.content_left.0,
    content_width: constraint.content_width.0,
    content_bottom: constraint.content_bottom.0,
  }
}

fn page_invalidation_from_common(
  invalidation: common::PageInvalidation,
) -> ooxmlsdk_layout::compat::PageInvalidation {
  ooxmlsdk_layout::compat::PageInvalidation {
    page_index: invalidation.page_index,
    section_page_index: invalidation.section_page_index,
    scope: reflow_scope_from_common(invalidation.scope),
    reason: reflow_reason_from_common(invalidation.reason),
    first_frame_index: invalidation.first_frame_index,
  }
}

fn reflow_execution_from_common(
  execution: common::ReflowExecution,
) -> ooxmlsdk_layout::compat::ReflowExecution {
  ooxmlsdk_layout::compat::ReflowExecution {
    action: match execution.action {
      common::ReflowAction::StabilizedRetainedDecorationItems => {
        ooxmlsdk_layout::compat::ReflowAction::StabilizedRetainedDecorationItems
      }
      common::ReflowAction::StabilizedInsertionInfluences => {
        ooxmlsdk_layout::compat::ReflowAction::StabilizedInsertionInfluences
      }
    },
    scope: reflow_scope_from_common(execution.scope),
    first_page_index: execution.first_page_index,
    request_count: execution.request_count,
    suppressed_moves: execution.suppressed_moves,
    backward_moves: execution.backward_moves,
    page_replacements: execution.page_replacements,
    replayed_frames: execution.replayed_frames,
    replayed_items: execution.replayed_items,
  }
}

fn reflow_request_from_common(
  request: common::ReflowRequest,
) -> ooxmlsdk_layout::compat::ReflowRequest {
  ooxmlsdk_layout::compat::ReflowRequest {
    kind: frame_kind_from_common(request.kind),
    reason: reflow_reason_from_common(request.reason),
    scope: reflow_scope_from_common(request.scope),
    frame_index: request.frame_index,
    page_index: request.page_index,
    section_page_index: request.section_page_index,
    column_index: request.column_index,
    restart: frame_cursor_from_common(request.restart),
    influence_count: request.influence_count,
  }
}

fn restart_plan_from_common(plan: common::RestartPlan) -> ooxmlsdk_layout::compat::RestartPlan {
  ooxmlsdk_layout::compat::RestartPlan {
    reason: reflow_reason_from_common(plan.reason),
    scope: reflow_scope_from_common(plan.scope),
    frame_index: plan.frame_index,
    page_index: plan.page_index,
    block_index: plan.block_index,
    cursor: frame_cursor_from_common(plan.cursor),
  }
}

fn reflow_scope_from_common(scope: common::ReflowScope) -> ooxmlsdk_layout::compat::ReflowScope {
  match scope {
    common::ReflowScope::Frame => ooxmlsdk_layout::compat::ReflowScope::Frame,
    common::ReflowScope::Column => ooxmlsdk_layout::compat::ReflowScope::Column,
    common::ReflowScope::Page => ooxmlsdk_layout::compat::ReflowScope::Page,
  }
}

fn reflow_reason_from_common(
  reason: common::ReflowReason,
) -> ooxmlsdk_layout::compat::ReflowReason {
  match reason {
    common::ReflowReason::DecorationChangedItems => {
      ooxmlsdk_layout::compat::ReflowReason::DecorationChangedItems
    }
    common::ReflowReason::InsertionInfluenceChanged => {
      ooxmlsdk_layout::compat::ReflowReason::InsertionInfluenceChanged
    }
    common::ReflowReason::InvalidBounds => ooxmlsdk_layout::compat::ReflowReason::InvalidBounds,
  }
}

fn stroke_from_common(stroke: common::Stroke<'static>) -> ooxmlsdk_layout::compat::BorderStyle {
  ooxmlsdk_layout::compat::BorderStyle {
    width_pt: stroke.width.0,
    spacing_pt: 0.0,
    color: rgb(stroke.color),
    compound: false,
  }
}

fn border_style_from_common(border: common::BorderStyle) -> ooxmlsdk_layout::compat::BorderStyle {
  ooxmlsdk_layout::compat::BorderStyle {
    width_pt: border.width.0,
    spacing_pt: border.spacing.0,
    color: rgb(border.color),
    compound: border.compound,
  }
}

fn solid_rgb(fill: common::Fill<'static>) -> Option<RgbColor> {
  solid_color(fill).map(rgb)
}

fn solid_color(fill: common::Fill<'static>) -> Option<common::Color> {
  match fill {
    common::Fill::Solid(color) => Some(color),
    common::Fill::None
    | common::Fill::Theme(_)
    | common::Fill::Gradient(_)
    | common::Fill::Image { .. }
    | common::Fill::Pattern { .. } => None,
  }
}

fn rgb(color: common::Color) -> RgbColor {
  RgbColor {
    r: color.r,
    g: color.g,
    b: color.b,
  }
}

fn opacity(color: common::Color) -> f32 {
  f32::from(color.a) / f32::from(u8::MAX)
}
