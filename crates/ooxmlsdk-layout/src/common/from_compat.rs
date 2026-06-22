use std::borrow::Cow;

use crate::common::*;
use crate::compat as legacy;
use crate::options::LayoutOptions as EngineLayoutOptions;

pub fn layout_document_from_compat(
  engine_kind: LayoutEngineKind,
  document: legacy::LayoutDocument,
  options: &EngineLayoutOptions,
) -> LayoutDocument<'static> {
  let debug_records = if options.diagnostics.collect_debug_records {
    debug_records_from_compat(&document)
  } else {
    Vec::new()
  };
  let legacy::LayoutDocument {
    pages,
    form_widgets,
    follows,
    frames,
    outline_entries,
    anchor_pages,
    page_replays,
    page_replay_applications,
    backward_moves,
    layout_reruns,
    page_invalidations,
    reflow_executions,
    reflow_requests,
    restart_plan,
  } = document;
  LayoutDocument {
    engine_kind,
    options: LayoutOptions {
      collect_debug: options.diagnostics.collect_debug_records,
      approximate_unsupported: false,
      preserve_source_links: options.diagnostics.preserve_source_links,
    },
    pages: pages.into_iter().map(display_page_from_compat).collect(),
    form_widgets: form_widgets
      .into_iter()
      .map(form_widget_from_compat)
      .collect(),
    follows: follows.into_iter().map(frame_follow_from_compat).collect(),
    frames: frames
      .into_iter()
      .enumerate()
      .map(|(index, frame)| frame_record_from_compat(index, frame))
      .collect(),
    outline_entries: outline_entries
      .into_iter()
      .map(outline_entry_from_compat)
      .collect(),
    anchor_pages: anchor_pages
      .into_iter()
      .map(anchor_page_from_compat)
      .collect(),
    reflow: reflow_diagnostics_from_compat(ReflowCompatParts {
      page_replays,
      page_replay_applications,
      backward_moves,
      layout_reruns,
      page_invalidations,
      reflow_executions,
      reflow_requests,
      restart_plan,
    }),
    debug_records,
    ..LayoutDocument::default()
  }
}

fn anchor_page_from_compat(anchor: legacy::AnchorPage) -> AnchorPage<'static> {
  AnchorPage {
    name: Cow::Owned(anchor.name),
    page_index: anchor.page_index,
    section_index: anchor.section_index,
    section_page_index: anchor.section_page_index,
    physical_page_number: anchor.physical_page_number,
    virtual_page_number: anchor.virtual_page_number,
  }
}

fn display_page_from_compat(page: legacy::Page) -> DisplayPage<'static> {
  let setup = page_setup_from_compat(page.setup);
  DisplayPage {
    section_index: page.section_index,
    section_page_index: page.section_page_index,
    bounds: rect(0.0, 0.0, page.setup.width_pt, page.setup.height_pt),
    background: page
      .setup
      .background
      .map(|color| Fill::Solid(rgb(color, 1.0))),
    setup,
    items: page
      .items
      .into_iter()
      .map(display_item_from_compat)
      .collect(),
    ..DisplayPage::default()
  }
}

fn display_item_from_compat(item: legacy::PageItem) -> DisplayItem<'static> {
  match item {
    legacy::PageItem::Text(item) => DisplayItem::Text(text_run_from_compat(item)),
    legacy::PageItem::Image(item) => DisplayItem::Image(image_item_from_compat(item)),
    legacy::PageItem::LinkArea(item) => DisplayItem::LinkArea(LinkArea {
      bounds: rect(item.x_pt, item.y_pt, item.width_pt, item.height_pt),
      target: Cow::Owned(item.hyperlink_url),
    }),
    legacy::PageItem::Rect(item) => DisplayItem::Rect(rect_item_from_compat(item)),
    legacy::PageItem::Fill(item) => DisplayItem::Rect(RectItem {
      bounds: rect(item.x_pt, item.y_pt, item.width_pt, item.height_pt),
      fill: Fill::Solid(rgb(item.color, 1.0)),
      stroke: None,
    }),
    legacy::PageItem::Line(item) => DisplayItem::Line(line_item_from_compat(item)),
    legacy::PageItem::Polyline(item) => DisplayItem::Path(path_item_from_compat(item)),
  }
}

fn text_run_from_compat(item: legacy::TextItem) -> TextRun<'static> {
  let color = rgb(item.style.color, item.style.opacity);
  TextRun {
    text: Cow::Owned(item.text),
    origin: point(item.x_pt, item.y_pt),
    line_height: Pt(item.line_height_pt),
    style: text_style_from_compat(item.style),
    font_id: None,
    color,
    rotation_center: item.rotation_center_pt.map(|(x, y)| point(x, y)),
    hyperlink_url: item.hyperlink_url.map(Cow::Owned),
    dynamic_field: item.dynamic_field.map(dynamic_field_from_compat),
    form_widget_id: item.form_widget_id,
    paragraph_bidi: item.paragraph_bidi,
    preserve_text_portion: item.preserve_text_portion,
    pdf_text_segmentation: match item.pdf_text_segmentation {
      legacy::PdfTextSegmentation::Line => PdfTextSegmentation::Line,
      legacy::PdfTextSegmentation::Portion => PdfTextSegmentation::Portion,
    },
    source: None,
  }
}

fn image_item_from_compat(item: legacy::ImageItem) -> ImageItem<'static> {
  ImageItem {
    bounds: rect(item.x_pt, item.y_pt, item.width_pt, item.height_pt),
    crop: Some(ImageCrop {
      left: item.crop.left,
      top: item.crop.top,
      right: item.crop.right,
      bottom: item.crop.bottom,
    }),
    rotation_degrees: item.rotation_deg,
    flip_horizontal: item.flip_horizontal,
    flip_vertical: item.flip_vertical,
    content_type: item
      .content_type
      .map(Cow::Owned)
      .unwrap_or(Cow::Borrowed("application/octet-stream")),
    bytes: item.data,
    relationship_id: None,
    alt_text: item.alt_text.map(Cow::Owned),
    hyperlink_url: item.hyperlink_url.map(Cow::Owned),
    floating: item.floating,
    behind_text: item.behind_text,
  }
}

fn rect_item_from_compat(item: legacy::RectItem) -> RectItem<'static> {
  RectItem {
    bounds: rect(item.x_pt, item.y_pt, item.width_pt, item.height_pt),
    fill: item
      .fill_color
      .map(|color| Fill::Solid(rgb(color, item.fill_opacity)))
      .unwrap_or(Fill::None),
    stroke: item
      .stroke
      .map(|stroke| stroke_from_border(stroke, item.stroke_opacity)),
  }
}

fn line_item_from_compat(item: legacy::LineItem) -> LineItem<'static> {
  LineItem {
    start: point(item.x1_pt, item.y1_pt),
    end: point(item.x2_pt, item.y2_pt),
    stroke: Stroke {
      width: Pt(item.width_pt),
      color: rgb(item.color, 1.0),
      dash: None,
      source_style_id: None,
    },
    kind: match item.kind {
      legacy::LineItemKind::Stroke => LineKind::Stroke,
      legacy::LineItemKind::FilledRect => LineKind::FilledRect,
    },
  }
}

fn path_item_from_compat(item: legacy::PolylineItem) -> PathItem<'static> {
  PathItem {
    bounds: rect(item.x_pt, item.y_pt, item.width_pt, item.height_pt),
    points: item
      .points
      .into_iter()
      .map(|(x, y)| point(item.x_pt + x, item.y_pt + y))
      .collect(),
    closed: item.closed,
    fill: item
      .fill_color
      .map(|color| Fill::Solid(rgb(color, 1.0)))
      .unwrap_or(Fill::None),
    stroke: item.stroke.map(|stroke| stroke_from_border(stroke, 1.0)),
  }
}

fn page_setup_from_compat(setup: legacy::PageSetup) -> PageSetup {
  PageSetup {
    size: Size {
      width: Pt(setup.width_pt),
      height: Pt(setup.height_pt),
    },
    margins: Insets {
      top: Pt(setup.margin_top_pt),
      right: Pt(setup.margin_right_pt),
      bottom: Pt(setup.margin_bottom_pt),
      left: Pt(setup.margin_left_pt),
    },
    mirror_margins: setup.mirror_margins,
    top_margin_was_negative: setup.top_margin_was_negative,
    bottom_margin_was_negative: setup.bottom_margin_was_negative,
    header_distance: Pt(setup.header_distance_pt),
    footer_distance: Pt(setup.footer_distance_pt),
    background: setup.background.map(|color| rgb(color, 1.0)),
    borders: CellBorders {
      top: setup.borders.top.map(border_style_from_compat),
      right: setup.borders.right.map(border_style_from_compat),
      bottom: setup.borders.bottom.map(border_style_from_compat),
      left: setup.borders.left.map(border_style_from_compat),
    },
    borders_offset_from_text: setup.borders_offset_from_text,
    line_numbering: setup.line_numbering.map(|line| LineNumbering {
      count_by: line.count_by,
      start: line.start,
      distance: Pt(line.distance_pt),
      restart_each_page: line.restart_each_page,
    }),
    doc_grid_line_pitch: setup.doc_grid_line_pitch_pt.map(Pt),
    page_number_start: setup.page_number_start,
  }
}

fn text_style_from_compat(style: legacy::TextStyle) -> TextStyle<'static> {
  TextStyle {
    font_family: style.font_family.map(|value| Cow::Owned(value.to_string())),
    east_asia_font_family: style
      .east_asia_font_family
      .map(|value| Cow::Owned(value.to_string())),
    complex_font_family: style
      .complex_font_family
      .map(|value| Cow::Owned(value.to_string())),
    symbol_font_family: style
      .symbol_font_family
      .map(|value| Cow::Owned(value.to_string())),
    font_size: Pt(style.font_size_pt),
    complex_font_size: style.complex_font_size_pt.map(Pt),
    character_spacing: Pt(style.character_spacing_pt),
    baseline_shift: Pt(style.baseline_shift_pt),
    bold: style.bold,
    italic: style.italic,
    underline: style.underline,
    strikethrough: style.strikethrough,
    uppercase: style.uppercase,
    small_caps: style.small_caps,
    hidden: style.hidden,
    rotation_degrees: style.rotation_deg,
    color: rgb(style.color, style.opacity),
    outline_color: style
      .outline_color
      .map(|color| rgb(color, style.outline_opacity)),
    outline_width: Pt(style.outline_width_pt),
    highlight: style.highlight.map(|color| rgb(color, 1.0)),
    underline_color: style.underline_color.map(|color| rgb(color, 1.0)),
  }
}

fn dynamic_field_from_compat(field: legacy::DynamicFieldKind) -> DynamicField<'static> {
  match field {
    legacy::DynamicFieldKind::Page => DynamicField::Page,
    legacy::DynamicFieldKind::NumPages => DynamicField::NumPages,
    legacy::DynamicFieldKind::PageRef { bookmark_name } => DynamicField::PageRef {
      bookmark_name: Cow::Owned(bookmark_name.to_string()),
    },
    legacy::DynamicFieldKind::StyleRef {
      style_name,
      from_bottom,
    } => DynamicField::StyleRef {
      style_name: Cow::Owned(style_name.to_string()),
      from_bottom,
    },
  }
}

fn form_widget_from_compat(widget: legacy::FormWidget) -> FormWidget<'static> {
  FormWidget {
    id: widget.id,
    kind: match widget.kind {
      legacy::FormWidgetKind::Text => FormWidgetKind::Text,
      legacy::FormWidgetKind::DropDownList => FormWidgetKind::DropDownList,
      legacy::FormWidgetKind::ComboBox => FormWidgetKind::ComboBox,
    },
    entries: widget.entries.into_iter().map(Cow::Owned).collect(),
  }
}

fn outline_entry_from_compat(entry: legacy::OutlineEntry) -> OutlineEntry<'static> {
  OutlineEntry {
    level: entry.level,
    text: Cow::Owned(entry.text),
    page_index: entry.page_index,
    target: point(entry.x_pt, entry.y_pt),
    merged_hidden_separator: entry.merged_hidden_separator,
  }
}

fn frame_record_from_compat(index: usize, frame: legacy::LayoutFrame) -> FrameRecord<'static> {
  FrameRecord {
    id: FrameId(index as u32),
    parent: None,
    kind: Cow::Borrowed(frame_kind_name(frame.kind)),
    block_index: frame.block_index,
    page_index: frame.page_index,
    section_index: frame.section_index,
    section_page_index: frame.section_page_index,
    column_index: frame.column_index,
    item_range: item_range(frame.item_start, frame.item_end),
    split_start: frame_cursor_from_compat(frame.split_start),
    split_end: frame_cursor_from_compat(frame.split_end),
    bounds: frame.bounds.map(rect_from_frame_bounds),
    print_bounds: frame.bounds.map(rect_from_frame_bounds),
    lines: frame.lines.into_iter().map(line_box_from_compat).collect(),
    fragments: frame
      .fragments
      .into_iter()
      .map(frame_fragment_from_compat)
      .collect(),
    influences: frame
      .influences
      .into_iter()
      .map(frame_influence_from_compat)
      .collect(),
    invalidation: frame_invalidation_from_compat(frame.invalidation),
  }
}

fn frame_follow_from_compat(follow: legacy::FrameFollow) -> FrameFollow {
  FrameFollow {
    kind: frame_kind_from_compat(follow.kind),
    reason: match follow.reason {
      legacy::FollowReason::KeepTogether => FollowReason::KeepTogether,
      legacy::FollowReason::Overflow => FollowReason::Overflow,
      legacy::FollowReason::ExplicitBreak => FollowReason::ExplicitBreak,
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

fn frame_cursor_from_compat(cursor: legacy::FrameCursor) -> FrameCursor {
  FrameCursor {
    block_index: cursor.block_index,
    kind: match cursor.kind {
      legacy::FrameCursorKind::BlockStart => FrameCursorKind::BlockStart,
      legacy::FrameCursorKind::Inline => FrameCursorKind::Inline,
      legacy::FrameCursorKind::TableRow => FrameCursorKind::TableRow,
      legacy::FrameCursorKind::TableCell => FrameCursorKind::TableCell,
      legacy::FrameCursorKind::BlockEnd => FrameCursorKind::BlockEnd,
    },
    inline_index: cursor.inline_index,
    text_offset: cursor.text_offset,
    row_index: cursor.row_index,
    cell_index: cursor.cell_index,
  }
}

fn line_box_from_compat(line: legacy::LineBox) -> LineBox {
  LineBox {
    bounds: rect(line.x_pt, line.y_pt, line.width_pt, line.height_pt),
    item_range: item_range(line.item_start, line.item_end),
  }
}

fn frame_fragment_from_compat(fragment: legacy::FrameFragment) -> FrameFragment {
  FrameFragment {
    kind: match fragment.kind {
      legacy::FrameFragmentKind::ParagraphLine => FrameFragmentKind::ParagraphLine,
      legacy::FrameFragmentKind::TableRow => FrameFragmentKind::TableRow,
      legacy::FrameFragmentKind::TableCell => FrameFragmentKind::TableCell,
      legacy::FrameFragmentKind::NoteLine => FrameFragmentKind::NoteLine,
    },
    split: match fragment.split {
      legacy::FragmentSplitKind::Complete => FragmentSplitKind::Complete,
      legacy::FragmentSplitKind::Master => FragmentSplitKind::Master,
      legacy::FragmentSplitKind::Follow => FragmentSplitKind::Follow,
      legacy::FragmentSplitKind::RepeatedHeader => FragmentSplitKind::RepeatedHeader,
    },
    index: fragment.index,
    row_index: fragment.row_index,
    cell_index: fragment.cell_index,
    item_range: item_range(fragment.item_start, fragment.item_end),
    bounds: fragment.bounds.map(rect_from_frame_bounds),
  }
}

fn frame_influence_from_compat(influence: legacy::FrameInfluence) -> FrameInfluence {
  FrameInfluence {
    kind: frame_influence_kind_from_compat(influence.kind),
    count: influence.count,
    block_index: influence.block_index,
    item_range: item_range(influence.item_start, influence.item_end),
    bounds: influence.bounds.map(rect_from_frame_bounds),
  }
}

struct ReflowCompatParts {
  page_replays: Vec<legacy::PageReplay>,
  page_replay_applications: Vec<legacy::PageReplayApplication>,
  backward_moves: Vec<legacy::BackwardMove>,
  layout_reruns: Vec<legacy::LayoutRerun>,
  page_invalidations: Vec<legacy::PageInvalidation>,
  reflow_executions: Vec<legacy::ReflowExecution>,
  reflow_requests: Vec<legacy::ReflowRequest>,
  restart_plan: Option<legacy::RestartPlan>,
}

fn reflow_diagnostics_from_compat(parts: ReflowCompatParts) -> ReflowDiagnostics<'static> {
  ReflowDiagnostics {
    page_replays: parts
      .page_replays
      .into_iter()
      .map(page_replay_from_compat)
      .collect(),
    page_replay_applications: parts
      .page_replay_applications
      .into_iter()
      .map(page_replay_application_from_compat)
      .collect(),
    backward_moves: parts
      .backward_moves
      .into_iter()
      .map(backward_move_from_compat)
      .collect(),
    layout_reruns: parts
      .layout_reruns
      .into_iter()
      .map(layout_rerun_from_compat)
      .collect(),
    page_invalidations: parts
      .page_invalidations
      .into_iter()
      .map(page_invalidation_from_compat)
      .collect(),
    reflow_executions: parts
      .reflow_executions
      .into_iter()
      .map(reflow_execution_from_compat)
      .collect(),
    reflow_requests: parts
      .reflow_requests
      .into_iter()
      .map(reflow_request_from_compat)
      .collect(),
    restart_plan: parts.restart_plan.map(restart_plan_from_compat),
  }
}

fn page_replay_from_compat(replay: legacy::PageReplay) -> PageReplay<'static> {
  PageReplay {
    page_index: replay.page_index,
    section_page_index: replay.section_page_index,
    column_index: replay.column_index,
    scope: reflow_scope_from_compat(replay.scope),
    item_range: item_range(replay.item_start, replay.item_end),
    replacement_items: replay
      .replacement_items
      .into_iter()
      .map(display_item_from_compat)
      .collect(),
  }
}

fn page_replay_application_from_compat(
  application: legacy::PageReplayApplication,
) -> PageReplayApplication {
  PageReplayApplication {
    page_index: application.page_index,
    section_page_index: application.section_page_index,
    column_index: application.column_index,
    scope: reflow_scope_from_compat(application.scope),
    item_range: item_range(application.item_start, application.item_end),
    replacement_count: application.replacement_count,
    applied: application.applied,
  }
}

fn backward_move_from_compat(move_back: legacy::BackwardMove) -> BackwardMove {
  BackwardMove {
    frame_index: move_back.frame_index,
    replay_start_frame_index: move_back.replay_start_frame_index,
    from_page_index: move_back.from_page_index,
    to_page_index: move_back.to_page_index,
    from_section_page_index: move_back.from_section_page_index,
    to_section_page_index: move_back.to_section_page_index,
    scope: reflow_scope_from_compat(move_back.scope),
    reason: reflow_reason_from_compat(move_back.reason),
    suppressed: move_back.suppressed,
    replayed_frames: move_back.replayed_frames,
    replayed_items: move_back.replayed_items,
  }
}

fn layout_rerun_from_compat(rerun: legacy::LayoutRerun) -> LayoutRerun {
  LayoutRerun {
    checkpoint_index: rerun.checkpoint_index,
    section_index: rerun.section_index,
    block_index: rerun.block_index,
    page_index: rerun.page_index,
    frame_index: rerun.frame_index,
    reason: reflow_reason_from_compat(rerun.reason),
    scope: reflow_scope_from_compat(rerun.scope),
    replaced_pages: rerun.replaced_pages,
    produced_pages: rerun.produced_pages,
    produced_frames: rerun.produced_frames,
    constraints: rerun
      .constraints
      .into_iter()
      .map(layout_rerun_constraint_from_compat)
      .collect(),
  }
}

fn layout_rerun_constraint_from_compat(
  constraint: legacy::LayoutRerunConstraint,
) -> LayoutRerunConstraint {
  LayoutRerunConstraint {
    kind: frame_influence_kind_from_compat(constraint.kind),
    scope: reflow_scope_from_compat(constraint.scope),
    bounds: constraint.bounds.map(rect_from_frame_bounds),
    content_left: Pt(constraint.content_left_pt),
    content_width: Pt(constraint.content_width),
    content_bottom: Pt(constraint.content_bottom),
  }
}

fn page_invalidation_from_compat(invalidation: legacy::PageInvalidation) -> PageInvalidation {
  PageInvalidation {
    page_index: invalidation.page_index,
    section_page_index: invalidation.section_page_index,
    first_frame_index: invalidation.first_frame_index,
    reason: reflow_reason_from_compat(invalidation.reason),
    scope: reflow_scope_from_compat(invalidation.scope),
  }
}

fn reflow_execution_from_compat(execution: legacy::ReflowExecution) -> ReflowExecution {
  ReflowExecution {
    first_page_index: execution.first_page_index,
    request_count: execution.request_count,
    action: match execution.action {
      legacy::ReflowAction::StabilizedRetainedDecorationItems => {
        ReflowAction::StabilizedRetainedDecorationItems
      }
      legacy::ReflowAction::StabilizedInsertionInfluences => {
        ReflowAction::StabilizedInsertionInfluences
      }
    },
    scope: reflow_scope_from_compat(execution.scope),
    suppressed_moves: execution.suppressed_moves,
    backward_moves: execution.backward_moves,
    page_replacements: execution.page_replacements,
    replayed_frames: execution.replayed_frames,
    replayed_items: execution.replayed_items,
  }
}

fn reflow_request_from_compat(request: legacy::ReflowRequest) -> ReflowRequest {
  ReflowRequest {
    frame_index: request.frame_index,
    kind: frame_kind_from_compat(request.kind),
    reason: reflow_reason_from_compat(request.reason),
    scope: reflow_scope_from_compat(request.scope),
    restart: frame_cursor_from_compat(request.restart),
    page_index: request.page_index,
    section_page_index: request.section_page_index,
    column_index: request.column_index,
    influence_count: request.influence_count,
  }
}

fn restart_plan_from_compat(plan: legacy::RestartPlan) -> RestartPlan {
  RestartPlan {
    page_index: plan.page_index,
    frame_index: plan.frame_index,
    block_index: plan.block_index,
    cursor: frame_cursor_from_compat(plan.cursor),
    reason: reflow_reason_from_compat(plan.reason),
    scope: reflow_scope_from_compat(plan.scope),
  }
}

fn debug_records_from_compat(document: &legacy::LayoutDocument) -> Vec<DebugRecord<'static>> {
  let page_records = document.pages.iter().enumerate().map(|(index, page)| {
    DebugRecord::Page(DebugPage {
      index,
      name: None,
      bounds: rect(0.0, 0.0, page.setup.width_pt, page.setup.height_pt),
    })
  });
  let frame_records = document.frames.iter().enumerate().map(|(index, frame)| {
    DebugRecord::Frame(DebugFrame {
      id: FrameId(index as u32),
      parent: None,
      kind: Cow::Borrowed(frame_kind_name(frame.kind)),
      bounds: frame.bounds.map(rect_from_frame_bounds).unwrap_or_default(),
      print_bounds: frame.bounds.map(rect_from_frame_bounds).unwrap_or_default(),
    })
  });
  let line_records = document
    .frames
    .iter()
    .enumerate()
    .flat_map(|(frame_index, frame)| {
      frame
        .lines
        .iter()
        .enumerate()
        .map(move |(line_index, line)| {
          DebugRecord::TextLine(DebugTextLine {
            frame: FrameId(frame_index as u32),
            index: line_index,
            text: Cow::Owned(frame_text_for_range(
              &frame.items,
              line.item_start,
              line.item_end,
            )),
            bounds: rect(line.x_pt, line.y_pt, line.width_pt, line.height_pt),
          })
        })
    });
  page_records
    .chain(frame_records)
    .chain(line_records)
    .collect()
}

fn frame_text_for_range(items: &[legacy::PageItem], start: usize, end: usize) -> String {
  items
    .iter()
    .skip(start)
    .take(end.saturating_sub(start))
    .filter_map(|item| match item {
      legacy::PageItem::Text(text) => Some(text.text.as_str()),
      _ => None,
    })
    .collect::<Vec<_>>()
    .join("")
}

fn frame_kind_from_compat(kind: legacy::FollowFrameKind) -> FrameKind {
  match kind {
    legacy::FollowFrameKind::Paragraph => FrameKind::Paragraph,
    legacy::FollowFrameKind::Table => FrameKind::Table,
    legacy::FollowFrameKind::Notes => FrameKind::Notes,
  }
}

fn frame_kind_name(kind: legacy::FollowFrameKind) -> &'static str {
  match kind {
    legacy::FollowFrameKind::Paragraph => "paragraph",
    legacy::FollowFrameKind::Table => "table",
    legacy::FollowFrameKind::Notes => "notes",
  }
}

fn frame_influence_kind_from_compat(kind: legacy::FrameInfluenceKind) -> FrameInfluenceKind {
  match kind {
    legacy::FrameInfluenceKind::FootnoteReservation => FrameInfluenceKind::FootnoteReservation,
    legacy::FrameInfluenceKind::FlyWrap => FrameInfluenceKind::FlyWrap,
    legacy::FrameInfluenceKind::TableSplit => FrameInfluenceKind::TableSplit,
  }
}

fn frame_invalidation_from_compat(invalidation: legacy::FrameInvalidation) -> FrameInvalidation {
  match invalidation {
    legacy::FrameInvalidation::Clean => FrameInvalidation::Clean,
    legacy::FrameInvalidation::PageItemsDecorated => FrameInvalidation::PageItemsDecorated,
    legacy::FrameInvalidation::NeedsReflow => FrameInvalidation::NeedsReflow,
  }
}

fn reflow_scope_from_compat(scope: legacy::ReflowScope) -> ReflowScope {
  match scope {
    legacy::ReflowScope::Frame => ReflowScope::Frame,
    legacy::ReflowScope::Column => ReflowScope::Column,
    legacy::ReflowScope::Page => ReflowScope::Page,
  }
}

fn reflow_reason_from_compat(reason: legacy::ReflowReason) -> ReflowReason {
  match reason {
    legacy::ReflowReason::DecorationChangedItems => ReflowReason::DecorationChangedItems,
    legacy::ReflowReason::InsertionInfluenceChanged => ReflowReason::InsertionInfluenceChanged,
    legacy::ReflowReason::InvalidBounds => ReflowReason::InvalidBounds,
  }
}

fn border_style_from_compat(style: legacy::BorderStyle) -> BorderStyle {
  BorderStyle {
    width: Pt(style.width_pt),
    spacing: Pt(style.spacing_pt),
    color: rgb(style.color, 1.0),
    compound: style.compound,
  }
}

fn stroke_from_border(style: legacy::BorderStyle, opacity: f32) -> Stroke<'static> {
  Stroke {
    width: Pt(style.width_pt),
    color: rgb(style.color, opacity),
    dash: None,
    source_style_id: None,
  }
}

fn rect_from_frame_bounds(bounds: legacy::FrameBounds) -> Rect {
  rect(bounds.x_pt, bounds.y_pt, bounds.width_pt, bounds.height_pt)
}

fn item_range(start: usize, end: usize) -> ItemRange {
  ItemRange { start, end }
}

fn rect(x: f32, y: f32, width: f32, height: f32) -> Rect {
  Rect {
    origin: point(x, y),
    size: Size {
      width: Pt(width),
      height: Pt(height),
    },
  }
}

fn point(x: f32, y: f32) -> Point {
  Point { x: Pt(x), y: Pt(y) }
}

fn rgb(color: legacy::RgbColor, opacity: f32) -> Color {
  Color {
    r: color.r,
    g: color.g,
    b: color.b,
    a: (opacity.clamp(0.0, 1.0) * 255.0).round() as u8,
  }
}
