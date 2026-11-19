use super::*;

#[derive(Clone, Copy, Debug)]
struct MeasuredLine {
  height_pt: f64,
  before_pt: f64,
  after_pt: f64,
  break_after: bool,
  source_bottom_pt: f32,
}

#[derive(Clone, Copy, Debug)]
struct MeasuredHeights {
  first_height_pt: f32,
  first_line_count: usize,
  frame_pt: f32,
}

struct SourceColumnChain {
  column: usize,
  end_column: usize,
  top_pt: f64,
  bottom_pt: f64,
  precision_pt: f64,
}

impl SourceColumnChain {
  fn before_line(&mut self, top: f64, within_paragraph: bool) -> Option<f64> {
    let followed = top < self.bottom_pt - self.precision_pt;
    if followed {
      if self.column != 0 || self.end_column != 1 {
        return None;
      }
      // LayoutFrame retains the paragraph's incoming column even when its
      // whole text follows. Physical line origins identify the actual chain;
      // unused space at the old column bottom is not a text advance.
      self.column = 1;
      self.bottom_pt = self.top_pt;
    }
    let gap = top - self.bottom_pt;
    if gap < -self.precision_pt || (within_paragraph && !followed && gap > self.precision_pt) {
      return None;
    }
    Some(if gap.abs() <= self.precision_pt {
      0.0
    } else {
      gap
    })
  }
}

// Legacy Word starts at the physical mean, then relaxes by the mean advance
// of lines actually admitted in the preceding trial. Word 1.1a's published
// layout reference describes this policy; independent current Office controls
// confirm it for ordinary text, mixed Exact heights, and leading obstacles.
// Word 2013 has a separate policy. Reuse committed lines without shaping or
// adding formatting passes to the existing section checkpoint replay.
fn measured_heights(
  lines: &[MeasuredLine],
  available_pt: f32,
  following_top_pt: f64,
) -> Option<MeasuredHeights> {
  if lines.len() < 2 {
    return None;
  }
  let mut ends = Vec::with_capacity(lines.len() + 1);
  let mut fits = Vec::with_capacity(lines.len() + 1);
  ends.push(0.0);
  fits.push(0.0);
  for line in lines {
    if !line.height_pt.is_finite()
      || line.height_pt <= 0.0
      || !line.before_pt.is_finite()
      || line.before_pt < 0.0
      || !line.after_pt.is_finite()
      || line.after_pt < 0.0
    {
      return None;
    }
    let fit = ends.last()? + line.before_pt + line.height_pt;
    fits.push(fit);
    ends.push(fit + line.after_pt);
  }
  let total = *ends.last()?;
  // Bound only arithmetic accumulation/division error at a source endpoint.
  // This is many orders below a physical line or a device/layout unit.
  let precision =
    f64::EPSILON * (total.abs() + following_top_pt.abs()) * (lines.len() + 1) as f64 * 4.0;
  let mut trial = total / 2.0;
  // Both admitted prefixes advance monotonically. Skip retries that cannot
  // change either prefix; allow one floating-point retry at each source edge.
  for _ in 0..lines.len().saturating_mul(4) {
    if trial < f64::from(DEFAULT_LINE_HEIGHT_PT) || trial >= f64::from(available_pt) {
      return None;
    }
    let mut cut = fits
      .partition_point(|end| *end <= trial + precision)
      .saturating_sub(1);
    while cut > 0 && !lines[cut - 1].break_after {
      cut -= 1;
    }
    let first = ends[cut].min(trial);
    if cut == lines.len() {
      // A blocked following column can leave all content in the first one.
      // Its drawing remains independently positioned and does not create
      // text-flow height in an otherwise unused column (Native Word controls).
      return (following_top_pt > 0.0).then_some(MeasuredHeights {
        first_height_pt: first as f32,
        first_line_count: cut,
        frame_pt: first as f32,
      });
    }
    // A terminal paragraph's text can fit even when its lower spacing cannot.
    // Word clips that lower to the column edge; its source value still owns
    // the initial total. Keep admission separate from the full flow advance.
    let last = following_top_pt + total - ends[cut];
    let following_available = (trial - following_top_pt).max(0.0);
    let following_limit = ends[cut] + following_available;
    let mut placed = fits
      .partition_point(|end| *end <= following_limit + precision)
      .saturating_sub(1)
      .max(cut);
    while placed > cut && !lines[placed - 1].break_after {
      placed -= 1;
    }
    if placed == lines.len() {
      return Some(MeasuredHeights {
        first_height_pt: first as f32,
        first_line_count: cut,
        frame_pt: first.max(last.min(trial)) as f32,
      });
    }
    // Include actual paragraph upper spacing in the admitted line advances;
    // the independent leading wrap gap is not a text line or line advance.
    let step = if placed == 0 {
      f64::from(DEFAULT_LINE_HEIGHT_PT)
    } else {
      (first + (ends[placed] - ends[cut]).min(following_available)) / placed as f64
    };
    let next_break =
      |after: usize| (after + 1..=lines.len()).find(|&index| lines[index - 1].break_after);
    let first_event = fits[next_break(cut)?];
    let following_event = following_top_pt + fits[next_break(placed)?] - ends[cut];
    let event = first_event.min(following_event);
    let clipped_lower = ends[cut] > trial || ends[placed] - ends[cut] > following_available;
    let steps = if clipped_lower {
      // Clipping changes the mean as the trial grows, even if no line moves.
      1.0
    } else {
      ((event - trial - precision) / step).ceil().max(1.0)
    };
    let next = trial + steps * step;
    if !next.is_finite() || next <= trial {
      return None;
    }
    trial = next;
  }
  None
}

fn following_column_top(
  exclusions: &[WrapExclusion],
  geometry: &ColumnGeometry,
  start_y: f32,
) -> Option<f64> {
  let mut following_top = 0.0_f64;
  for exclusion in exclusions {
    if exclusion.bottom_pt <= start_y {
      continue;
    }
    let overlaps = |column: usize| {
      exclusion.left_pt < geometry.lefts[column] + geometry.widths[column]
        && exclusion.right_pt > geometry.lefts[column]
    };
    if overlaps(0) {
      return None;
    }
    if !overlaps(1) {
      continue;
    }
    // The measured source lines remain reusable only when the first column
    // is free and the following column has a wholly blocked leading region.
    // Partial lanes, contours, and interior obstacles need their own line fit.
    let precision = f32::EPSILON
      * geometry.lefts[1]
        .abs()
        .max(exclusion.right_pt.abs())
        .max(start_y.abs())
      * 4.0;
    if !matches!(exclusion.owner, WrapExclusionOwner::VmlBodyDrawing)
      || exclusion.blocks_flow
      || exclusion.uses_contour
      || exclusion.side != ImageWrapSide::BothSides
      || exclusion.top_pt > start_y + precision
      || exclusion.left_pt > geometry.lefts[1] + precision
      || exclusion.right_pt + precision < geometry.lefts[1] + geometry.widths[1]
    {
      return None;
    }
    following_top = following_top.max(f64::from(exclusion.bottom_pt) - f64::from(start_y));
  }
  Some(following_top)
}

fn uniform_line_metrics(
  fragment: &FrameFragment,
  paragraph: &crate::docx::Paragraph,
) -> Option<(f32, f32)> {
  let top = fragment.flow_top_pt?;
  let height = if let Some(height) = fragment.content_advance_pt {
    height
  } else if paragraph_is_effectively_empty(paragraph) {
    // A real empty body mark owns physical bounds but no painted advance.
    fragment.bounds?.height_pt
  } else {
    return None;
  };
  (top.is_finite() && height.is_finite() && height > 0.0).then_some((top, height))
}

fn format_uniform_paragraphs(
  blocks: &[Block],
  mut flow: FlowContext,
  target: ParagraphLayoutTarget<'_>,
  mut y: f32,
  line_height: f32,
  precision: f32,
) -> Option<(FlowContext, f32, usize)> {
  let mut count = 0;
  for block in blocks {
    let Block::Paragraph(paragraph) = block else {
      return None;
    };
    let (next_flow, next_y, lines) = format_paragraph_with_left_indent_compat(
      paragraph,
      flow,
      ParagraphLayoutTarget {
        current: &mut *target.current,
        pages: &mut *target.pages,
        anchor_pages: None,
        text_metrics: &mut *target.text_metrics,
      },
      ParagraphLayoutOptions {
        y,
        paragraph_anchor_top: y,
        spacing_after_pt: 0.0,
        ignore_left_indent: false,
        border_context: ParagraphBorderContext::default(),
        shading_context: ParagraphShadingContext::default(),
        spacing_before_pt: 0.0,
        suppressed_spacing_before_pt: 0.0,
        decoration_outer_bottom_pt: None,
      },
    );
    if lines
      .iter()
      .any(|line| (line.height_pt - line_height).abs() > precision)
    {
      return None;
    }
    flow = restore_block_flow_width(flow, next_flow);
    y = next_y;
    count += lines.len();
  }
  Some((flow, y, count))
}

pub(super) fn unequal_paragraph_section_heights(
  root: &mut RootFrameLayout<'_>,
  section_index: usize,
  start_y: f32,
  flow: FlowContext,
  available_pt: f32,
) -> Option<BalancedSectionHeights> {
  let section = root.document.sections.get(section_index)?;
  let multiple = section.blocks.len() > 1;
  if section.blocks.is_empty()
    || section.columns.count < 2
    || !root.current.wrap_exclusions.is_empty()
    || root.current.footnote_reserved_height_pt != 0.0
    || !root
      .document
      .sections
      .get(section_index + 1)
      .is_some_and(|next| next.break_kind == SectionBreakKind::Continuous)
    || section
      .section_properties
      .as_ref()
      .and_then(super::super::section_vertical_text_flow)
      .is_some()
    || section.blocks.iter().any(|block| {
      let Block::Paragraph(paragraph) = block else {
        return true;
      };
      !(if multiple {
        cell_text_measure::supports_master_paragraph(paragraph)
      } else {
        cell_text_measure::supports_paragraph(paragraph)
      }) || !paragraph.footnote_reference_ids.is_empty()
        || !paragraph.endnote_reference_ids.is_empty()
        || paragraph.format.keep_lines
        || paragraph.format.keep_with_next
        || !paragraph.format.borders.is_empty()
        || paragraph.format.shading.is_some()
        || (multiple
          && (paragraph.format.page_break_before
            || paragraph.format.spacing_before_pt != 0.0
            || paragraph.format.spacing_after_pt != 0.0
            || paragraph.format.spacing_before_lines.is_some()
            || paragraph.format.spacing_after_lines.is_some()
            || paragraph.format.spacing_before_auto == Some(true)
            || paragraph.format.spacing_after_auto == Some(true)))
    })
  {
    return None;
  }
  let geometry = column_geometry(flow.setup, section.columns, flow.column_page_index);
  if geometry.widths.windows(2).all(|pair| pair[0] == pair[1]) {
    return None;
  }
  let checkpoint = root.checkpoints.iter().rev().find(|checkpoint| {
    checkpoint.section_index == section_index
      && checkpoint.block_index == 0
      && checkpoint.flow.column_index == 0
  })?;
  let continued = checkpoint.page_index < root.pages.len();
  let (blocks, frames, initial_flow, baseline) = if continued {
    // Native 12/14 follow controls retain the earlier page's source prefix.
    // The diagnostic FrameCursor counts lines, so it cannot identify this
    // tail. Carry the real formatter's InlineCursor instead, and measure
    // only the final page's unconsumed source.
    if multiple || section.columns.count != 2 || !matches!(flow.compatibility_mode, 11 | 12 | 14) {
      return None;
    }
    let follow = flow.column_paragraph_follow?;
    if follow.page_index != root.pages.len()
      || follow.start == InlineCursor::default()
      || (follow.y_pt - start_y).abs() > LAYOUT_EPSILON_PT
    {
      return None;
    }
    let Block::Paragraph(paragraph) = &section.blocks[0] else {
      return None;
    };
    let suffix = plain_paragraph_measurement_suffix(paragraph, follow.start)?;
    let first = root
      .frames
      .iter()
      .position(|frame| frame.page_index == root.pages.len())?;
    let frames = &root.frames[first..];
    // This single continued paragraph is the first body owner on the page.
    // Preserve the page/header metadata while removing its original trial
    // paint and fragments. Obstructions and notes were rejected above.
    if frames.first()?.item_start != 0 {
      return None;
    }
    let mut baseline = page_checkpoint(&root.current);
    baseline.items_len = 0;
    baseline.frame_fragments_len = 0;
    baseline.frame_influences_len = 0;
    baseline.body_content_frames = 0;
    let initial_flow = FlowContext {
      columns: section.columns,
      column_index: 0,
      content_top_pt: start_y,
      content_left_pt: geometry.lefts[0],
      content_width: geometry.widths[0],
      content_bottom: flow.body_content_bottom_pt,
      ..flow
    };
    (
      std::borrow::Cow::Owned(vec![Block::paragraph(suffix)]),
      frames,
      initial_flow,
      baseline,
    )
  } else {
    if checkpoint.page_index != root.pages.len() {
      return None;
    }
    (
      std::borrow::Cow::Borrowed(section.blocks.as_slice()),
      root.frames.get(checkpoint.frames_len..)?,
      checkpoint.flow,
      checkpoint.current,
    )
  };
  let first_frame = frames.first()?;
  let Block::Paragraph(first_paragraph) = section.blocks.get(first_frame.block_index?)? else {
    return None;
  };
  let (first_top, line_height) =
    uniform_line_metrics(first_frame.fragments.first()?, first_paragraph)?;
  if first_top != start_y {
    return None;
  }
  let precision = f32::EPSILON * (start_y.abs() + available_pt) * 4.0;
  for frame in frames {
    let Block::Paragraph(paragraph) = section.blocks.get(frame.block_index?)? else {
      return None;
    };
    if frame.page_index != root.pages.len()
      || frame.kind != FollowFrameKind::Paragraph
      || !frame.influences.is_empty()
      || frame.fragments.is_empty()
      || frame.fragments.iter().any(|line| {
        line.kind != FrameFragmentKind::ParagraphLine
          || line.paragraph_spacing_after_pt != 0.0
          || uniform_line_metrics(line, paragraph)
            .is_none_or(|(_, height)| (height - line_height).abs() > precision)
      })
    {
      return None;
    }
  }
  // Different destination widths change wrapping. Enumerate actual uniform
  // line endpoints and accept the first complete real-formatter fit; a
  // balanced mean does not prove the last column fits. Native short blank/
  // heading sequences and final-page tails pin both owners independently.
  let mut current = root.current.clone();
  restore_page_checkpoint(&mut current, baseline);
  let mut pages = Vec::new();
  let widest = geometry.widths.iter().copied().reduce(f32::max)?;
  let single_flow = FlowContext {
    columns: SectionColumns::default(),
    column_index: 0,
    content_width: widest,
    content_bottom: UNBOUNDED_LAYOUT_EXTENT_PT,
    body_content_bottom_pt: UNBOUNDED_LAYOUT_EXTENT_PT,
    ..initial_flow
  };
  let (_, _, whole_lines) = format_uniform_paragraphs(
    &blocks,
    single_flow,
    ParagraphLayoutTarget {
      current: &mut current,
      pages: &mut pages,
      anchor_pages: None,
      text_metrics: &mut root.text_metrics,
    },
    start_y,
    line_height,
    precision,
  )?;
  let narrowing_follow = continued && geometry.widths[0] > geometry.widths[1];
  let source_widows = narrowing_follow
    && matches!(&section.blocks[0], Block::Paragraph(paragraph)
      if paragraph_widow_control_applies(paragraph, initial_flow));
  let first_rows = whole_lines.div_ceil(section.columns.count).max(1);
  let last_rows = ((available_pt / line_height).floor() as usize).min(whole_lines);
  let mut previous_completed_follow_rows = None;
  for rows in first_rows..=last_rows {
    if !pages.is_empty() {
      current = pages.drain(..).next()?;
    }
    restore_page_checkpoint(&mut current, baseline);
    let frame_height = rows as f32 * line_height;
    let bottom = start_y + frame_height;
    let layout_height = bottom.next_up() - start_y;
    let trial_flow = FlowContext {
      columns: SectionColumns {
        balanced_height_pt: Some(layout_height),
        ..initial_flow.columns
      },
      content_bottom: bottom,
      ..initial_flow
    };
    let (end_flow, end_y, _) = format_uniform_paragraphs(
      &blocks,
      trial_flow,
      ParagraphLayoutTarget {
        current: &mut current,
        pages: &mut pages,
        anchor_pages: None,
        text_metrics: &mut root.text_metrics,
      },
      start_y,
      line_height,
      precision,
    )?;
    let followed = end_flow.column_index + 1 == section.columns.count;
    let previous_rows = previous_completed_follow_rows;
    previous_completed_follow_rows = (pages.is_empty() && followed).then_some(rows);
    if pages.is_empty() && end_y <= bottom + precision {
      let selected_rows = if source_widows && followed {
        // Widow control belongs to the incoming source width. One remaining
        // wide line can wrap into two narrow lines without satisfying that
        // minimum. Native on/off controls distinguish3/4 from4/2. Reuse the
        // preceding complete physical-page trial when a source line must
        // transfer; its final column is allowed to exceed the planned height.
        if rows < DEFAULT_ORPHAN_LINES {
          continue;
        }
        let source_tail = whole_lines.saturating_sub(rows);
        if source_tail < DEFAULT_WIDOW_LINES {
          let Some(previous) = previous_rows.filter(|&previous| {
            previous >= DEFAULT_ORPHAN_LINES && rows - previous == DEFAULT_WIDOW_LINES - source_tail
          }) else {
            continue;
          };
          previous
        } else {
          rows
        }
      } else {
        rows
      };
      let selected_height = selected_rows as f32 * line_height;
      // advance_section_flow releases the final column's provisional bound;
      // occupied_paragraph_bottom owns its actual physical frame closure.
      return Some(BalancedSectionHeights {
        page_index: root.pages.len(),
        section_page_index: flow.section_page_index,
        frame_top_pt: start_y,
        layout_height_pt: (start_y + selected_height).next_up() - start_y,
        frame_height_pt: selected_height,
        retained_floating_table: None,
      });
    }
  }
  None
}

pub(super) fn measured_section_heights(
  root: &RootFrameLayout<'_>,
  section_index: usize,
  start_y: f32,
  end_y: f32,
  flow: FlowContext,
  available_pt: f32,
) -> Option<BalancedSectionHeights> {
  let document = root.document;
  let section = document.sections.get(section_index)?;
  if document.compatibility_mode >= 15 {
    return uniform_paragraph_section_heights(
      root,
      section_index,
      start_y,
      end_y,
      flow,
      available_pt,
    );
  }
  if matches!(document.compatibility_mode, 11 | 12 | 14)
    && section.columns.count > 2
    && root.current.footnote_reserved_height_pt == 0.0
    && document
      .sections
      .get(section_index + 1)
      .is_some_and(|next| next.break_kind == SectionBreakKind::Continuous)
    && let Some(heights) =
      uniform_paragraph_section_heights(root, section_index, start_y, end_y, flow, available_pt)
  {
    // Legacy 4-column controls also admit complete measured lines. A slot
    // rounded to the default font can be just shorter than two real lines,
    // leaving one in each earlier column and overflowing the final column.
    // Reuse the uniform source endpoints; mixed/kept/obstructed text keeps
    // its existing balance owner, including legacy two-column relaxation.
    return Some(heights);
  }
  if !matches!(document.compatibility_mode, 11 | 12 | 14)
    || section.columns.count != 2
    || flow.column_index >= 2
    || root.current.footnote_reserved_height_pt > 0.0
    || !document
      .sections
      .get(section_index + 1)
      .is_some_and(|next| next.break_kind == SectionBreakKind::Continuous)
    || section
      .section_properties
      .as_ref()
      .and_then(super::super::section_vertical_text_flow)
      .is_some()
  {
    return None;
  }
  let geometry = column_geometry(flow.setup, section.columns, flow.column_page_index);
  if geometry.widths[0] != geometry.widths[1] {
    // A different destination width invalidates the measured line partition.
    return None;
  }
  if let Some(heights) = fixed_floating_table_section_heights(
    root,
    section_index,
    start_y,
    flow,
    available_pt,
    &geometry,
  ) {
    return Some(heights);
  }
  let following_top = following_column_top(&root.current.wrap_exclusions, &geometry, start_y)?;
  if flow.column_index != 0 && following_top > 0.0 {
    return None;
  }
  let page_index = root.pages.len();
  // Continuous sections can share an earlier section's physical page owner.
  // The source checkpoint, rather than LayoutFrame.section_index, owns the
  // new section's frame range. Retain the final-page lookup when its source
  // checkpoint belongs to an earlier physical page.
  let first = root
    .checkpoints
    .iter()
    .rev()
    .find(|checkpoint| {
      checkpoint.section_index == section_index
        && checkpoint.block_index == 0
        && checkpoint.page_index == page_index
    })
    .map(|checkpoint| checkpoint.frames_len)
    .unwrap_or_else(|| {
      root
        .frames
        .iter()
        .rposition(|frame| frame.section_index != section_index || frame.page_index != page_index)
        .map_or(0, |index| index + 1)
    });
  let frames = &root.frames[first..];
  let line_count = frames
    .iter()
    .map(|frame| frame.fragments.len())
    .sum::<usize>();
  if line_count < 2 {
    return None;
  }
  // Only cover floating-point accumulation error in the recorded flow
  // coordinates. This is not an admission or golden-comparison tolerance.
  let precision =
    f64::from(f32::EPSILON) * f64::from(start_y.abs() + end_y.abs()) * (line_count + 1) as f64;
  let mut lines = Vec::with_capacity(line_count);
  let mut chain = SourceColumnChain {
    column: 0,
    end_column: flow.column_index,
    top_pt: f64::from(start_y),
    bottom_pt: f64::from(start_y),
    precision_pt: precision,
  };
  let mut first_column_line_count = 0;
  let reusable_float = |placement, legacy_absolute_vml| {
    matches!(
      placement,
      crate::docx::ImagePlacement::Floating(floating)
        if following_top > 0.0
          || floating.wrap == ImageWrapMode::None
          // The body formatter resolves legacy absolute VML's Through to
          // omitted w10:wrap/type=none (MS-OI29500 19.3.2.6). Its independent
          // paint does not turn these measured text lines into wrapped lines.
          || (legacy_absolute_vml && floating.wrap == ImageWrapMode::Through)
    )
  };
  for frame in frames {
    if frame.kind != FollowFrameKind::Paragraph
      || frame.column_index >= 2
      || frame
        .influences
        .iter()
        .any(|influence| following_top == 0.0 || influence.kind != FrameInfluenceKind::FlyWrap)
      || frame.fragments.is_empty()
    {
      return None;
    }
    let Block::Paragraph(paragraph) = section.blocks.get(frame.block_index?)? else {
      return None;
    };
    if paragraph.format.line_height_rule != LineHeightRule::Exact
      || paragraph.format.keep_lines
      || paragraph.format.keep_with_next
      || paragraph.format.spacing_before_auto == Some(true)
      || paragraph.format.spacing_after_auto == Some(true)
      || paragraph_frame(paragraph).is_some()
      || !paragraph.footnote_reference_ids.is_empty()
      || !paragraph.endnote_reference_ids.is_empty()
      || paragraph.inlines.iter().any(|inline| !match inline {
        InlineItem::Text(_)
        | InlineItem::BookmarkStart(_)
        | InlineItem::FormWidgetStart(_)
        | InlineItem::FormWidgetEnd(_)
        | InlineItem::LastRenderedPageBreak
        | InlineItem::ColumnBreak => true,
        InlineItem::Image(image) => reusable_float(image.placement, false),
        InlineItem::Shape(shape) => reusable_float(shape.placement, shape.allow_outside_page),
        _ => false,
      })
    {
      return None;
    }
    let widow = paragraph_widow_control_applies(paragraph, flow);
    for (index, fragment) in frame.fragments.iter().enumerate() {
      if fragment.kind != FrameFragmentKind::ParagraphLine {
        return None;
      }
      let top = f64::from(fragment.flow_top_pt?);
      let height = f64::from(fragment.content_advance_pt?);
      let after = f64::from(fragment.paragraph_spacing_after_pt);
      let before = chain.before_line(top, index > 0)?;
      first_column_line_count += usize::from(chain.column == 0);
      let remaining = frame.fragments.len() - index - 1;
      lines.push(MeasuredLine {
        height_pt: height,
        before_pt: before,
        after_pt: after,
        break_after: remaining == 0 || !widow || (index + 1 >= 2 && remaining >= 2),
        source_bottom_pt: fragment.flow_top_pt? + fragment.content_advance_pt?,
      });
      chain.bottom_pt = top + height + after;
    }
    if (f64::from(frame.flow_bottom_pt?) - chain.bottom_pt).abs() > precision {
      return None;
    }
  }
  if chain.column != flow.column_index || (chain.bottom_pt - f64::from(end_y)).abs() > precision {
    return None;
  }
  let heights = measured_heights(&lines, available_pt, following_top)?;
  if heights.first_line_count > first_column_line_count {
    // Only an endpoint already measured in this physical first column can
    // serve as the exact admission deadline without another formatting pass.
    return None;
  }
  let source_bottom = lines
    .get(heights.first_line_count.checked_sub(1)?)?
    .source_bottom_pt;
  // The trial height locates a legal partition; it is not its admission
  // deadline. A larger trial can admit an extra line when an active fly makes
  // the paragraph formatter conservatively skip widow rebalancing. Enforce
  // the selected prefix at its already-recorded source endpoint instead.
  let first_height = heights.first_height_pt.max(source_bottom - start_y);
  let mut layout_height = first_height.max(DEFAULT_LINE_HEIGHT_PT);
  if start_y + layout_height < source_bottom {
    // Only repair subtraction/addition round-trip loss at this source edge.
    layout_height = layout_height.next_up();
  }
  if heights.first_line_count < first_column_line_count
    && lines
      .get(heights.first_line_count)
      .is_some_and(|next| start_y + layout_height >= next.source_bottom_pt)
  {
    // The existing minimum body height cannot enforce this smaller prefix.
    return None;
  }
  Some(BalancedSectionHeights {
    page_index,
    section_page_index: flow.section_page_index,
    frame_top_pt: start_y,
    layout_height_pt: layout_height,
    frame_height_pt: heights.frame_pt.max(first_height),
    retained_floating_table: None,
  })
}

fn fixed_floating_table_section_heights(
  root: &RootFrameLayout<'_>,
  section_index: usize,
  start_y: f32,
  flow: FlowContext,
  available_pt: f32,
  geometry: &ColumnGeometry,
) -> Option<BalancedSectionHeights> {
  if flow.column_index != 0 {
    return None;
  }
  let section = root.document.sections.get(section_index)?;
  let exclusion = *root.current.wrap_exclusions.first()?;
  if root
    .current
    .wrap_exclusions
    .iter()
    .any(|other| *other != exclusion)
    || exclusion.owner != WrapExclusionOwner::FloatingTable
    || exclusion.blocks_flow
    || exclusion.uses_contour
    || exclusion.side != ImageWrapSide::BothSides
    || ![exclusion.top_pt, exclusion.bottom_pt]
      .iter()
      .all(|v| v.is_finite())
    || geometry
      .lefts
      .iter()
      .zip(&geometry.widths)
      .any(|(&left, &width)| exclusion.left_pt > left || exclusion.right_pt < left + width)
  {
    // Side lanes, different obstacles and fragmented source columns need
    // their own line fitting. Duplicate preseed/materialized bounds are OK.
    return None;
  }
  let checkpoint = root.checkpoints.iter().rev().find(|checkpoint| {
    checkpoint.section_index == section_index
      && checkpoint.block_index == 0
      && checkpoint.page_index == root.pages.len()
  })?;
  let frames = root.frames.get(checkpoint.frames_len..)?;
  if frames.len() != section.blocks.len() {
    return None;
  }
  let mut line_height = None;
  let mut line_count = 0usize;
  let mut table_count = 0usize;
  for (index, (frame, block)) in frames.iter().zip(&section.blocks).enumerate() {
    if frame.block_index != Some(index)
      || frame.page_index != root.pages.len()
      || frame.column_index != 0
      || frame.influences.iter().any(|influence| {
        influence.kind != FrameInfluenceKind::FlyWrap
            // Grid spans mark a table as split-sensitive even when its
            // entire master is present. Verify actual row fragments below.
            && !(frame.kind == FollowFrameKind::Table
              && influence.kind == FrameInfluenceKind::TableSplit)
      })
    {
      return None;
    }
    match block {
      Block::Table(table) => {
        let placement = table.placement?;
        if frame.kind != FollowFrameKind::Table
          || !table.split_allowed
          || table.following_text_flow
          || placement.vertical_anchor != FrameVerticalAnchor::Text
          || placement.vertical_offset_pt >= -LAYOUT_EPSILON_PT
          || placement.horizontal_anchor != FrameHorizontalAnchor::Margin
          || placement.horizontal_alignment.is_some()
          || placement.vertical_alignment.is_some()
          || frame.fragments.iter().any(|fragment| {
            fragment.kind == FrameFragmentKind::TableRow
              && fragment.split != FragmentSplitKind::Complete
          })
          || frame
            .fragments
            .iter()
            .filter(|f| f.kind == FrameFragmentKind::TableRow)
            .count()
            != table.rows.len()
        {
          return None;
        }
        table_count += 1;
      }
      Block::Paragraph(paragraph) => {
        let format = &paragraph.format;
        if frame.kind != FollowFrameKind::Paragraph
          || frame.fragments.len() != 1
          || format.keep_with_next
          || format.page_break_before
          || format.spacing_before_pt != 0.0
          || format.spacing_before_lines.is_some_and(|v| v != 0.0)
          || format.spacing_before_auto == Some(true)
          || paragraph_frame(paragraph).is_some()
          || !paragraph.footnote_reference_ids.is_empty()
          || !paragraph.endnote_reference_ids.is_empty()
          || paragraph.inlines.iter().any(|inline| {
            !matches!(
              inline,
              InlineItem::Text(_)
                | InlineItem::BookmarkStart(_)
                | InlineItem::LastRenderedPageBreak
            )
          })
        {
          return None;
        }
        let line = &frame.fragments[0];
        let height = line.content_advance_pt?;
        if line.kind != FrameFragmentKind::ParagraphLine
          || !height.is_finite()
          || height <= 0.0
          || line.paragraph_spacing_after_pt != 0.0
          || frame.flow_bottom_pt? != line.flow_top_pt? + height
          || line_height.is_some_and(|prior| prior != height)
        {
          return None;
        }
        line_height = Some(height);
        line_count += 1;
      }
      Block::Frame(_) => return None,
    }
  }
  if table_count != 1 || line_count < 2 {
    return None;
  }
  // Native legacy controls keep the negative fly at its source position,
  // including when its anchor changes columns or the fly lies below both
  // balanced columns. Uniform single-line paragraphs divide ceil(N/2)+floor(N/2).
  // Each column must include its own avoidance gap; averaging the initial
  // single-column occupied height counts that gap only once. Reuse measured
  // advances and the ordinary wrap query, without shaping or trial layouts.
  let endpoint = uniform_wrapped_column_bottom(
    start_y,
    line_height?,
    line_count.div_ceil(2),
    geometry,
    exclusion,
  );
  let frame_height = endpoint - start_y;
  if frame_height >= available_pt || frame_height < DEFAULT_LINE_HEIGHT_PT {
    return None;
  }
  // A multi-row master below the new deadline could split on replay. That
  // independent page-capacity policy needs a separate reservation plan.
  if exclusion.bottom_pt > endpoint
    && section
      .blocks
      .iter()
      .any(|block| matches!(block, Block::Table(table) if table.rows.len() > 1))
  {
    return None;
  }
  let mut layout_height = frame_height;
  if start_y + layout_height < endpoint {
    layout_height = layout_height.next_up();
  }
  Some(BalancedSectionHeights {
    page_index: root.pages.len(),
    section_page_index: flow.section_page_index,
    frame_top_pt: start_y,
    layout_height_pt: layout_height,
    frame_height_pt: frame_height,
    retained_floating_table: Some(exclusion),
  })
}

fn uniform_wrapped_column_bottom(
  mut y: f32,
  line_height: f32,
  rows: usize,
  geometry: &ColumnGeometry,
  exclusion: WrapExclusion,
) -> f32 {
  for _ in 0..rows {
    y = dodge_text_wrap_exclusions(
      y,
      line_height,
      geometry.lefts[0],
      geometry.lefts[0] + geometry.widths[0],
      &[exclusion],
    ) + line_height;
  }
  y
}

pub(super) fn occupied_paragraph_bottom(
  root: &RootFrameLayout<'_>,
  section: &crate::docx::ImportedSection,
  first_frame: usize,
  page_index: usize,
) -> Option<f32> {
  if !root.current.wrap_exclusions.is_empty()
    || root.current.footnote_reserved_height_pt > 0.0
    || section
      .section_properties
      .as_ref()
      .and_then(super::super::section_vertical_text_flow)
      .is_some()
  {
    return None;
  }
  let mut bottom: Option<f32> = None;
  for frame in root.frames.get(first_frame..)? {
    if frame.page_index != page_index {
      continue;
    }
    let Block::Paragraph(paragraph) = section.blocks.get(frame.block_index?)? else {
      return None;
    };
    if frame.kind != FollowFrameKind::Paragraph
      || !frame.influences.is_empty()
      || frame.fragments.is_empty()
      || paragraph_frame(paragraph).is_some()
      || !paragraph.footnote_reference_ids.is_empty()
      || !paragraph.endnote_reference_ids.is_empty()
      || paragraph.inlines.iter().any(|inline| {
        !matches!(
          inline,
          InlineItem::Text(_)
            | InlineItem::BookmarkStart(_)
            | InlineItem::FormWidgetStart(_)
            | InlineItem::FormWidgetEnd(_)
            | InlineItem::LastRenderedPageBreak
        )
      })
    {
      return None;
    }
    for fragment in &frame.fragments {
      let top = fragment.flow_top_pt?;
      let advance = fragment.content_advance_pt?;
      if fragment.kind != FrameFragmentKind::ParagraphLine
        || !top.is_finite()
        || !advance.is_finite()
        || advance <= 0.0
      {
        return None;
      }
      // Paragraph lower spacing participates in the balance measurement but
      // does not establish an occupied line end. Native ordinary-text controls
      // close below every real line, even when the last column is shorter.
      let end = top + advance;
      if !end.is_finite() {
        return None;
      }
      bottom = Some(bottom.map_or(end, |previous| previous.max(end)));
    }
  }
  bottom
}

fn uniform_paragraph_section_heights(
  root: &RootFrameLayout<'_>,
  section_index: usize,
  start_y: f32,
  end_y: f32,
  flow: FlowContext,
  available_pt: f32,
) -> Option<BalancedSectionHeights> {
  let section = root.document.sections.get(section_index)?;
  if section.columns.count < 2
    || flow.column_index != 0
    || !root.current.wrap_exclusions.is_empty()
    || section
      .section_properties
      .as_ref()
      .and_then(super::super::section_vertical_text_flow)
      .is_some()
  {
    return None;
  }
  let geometry = column_geometry(flow.setup, section.columns, flow.column_page_index);
  if geometry.widths.windows(2).any(|pair| pair[0] != pair[1]) {
    return None;
  }
  // Checkpoints retain the source section's frame range. A continuous section
  // shares its physical page owner with earlier sections, so the page owner's
  // section index is not sufficient to identify these newly formatted lines.
  let checkpoint = root.checkpoints.iter().rev().find(|checkpoint| {
    checkpoint.section_index == section_index
      && checkpoint.block_index == 0
      && checkpoint.page_index == root.pages.len()
  })?;
  let frames = root.frames.get(checkpoint.frames_len..)?;
  let line_count = frames
    .iter()
    .map(|frame| frame.fragments.len())
    .sum::<usize>();
  if line_count < 2 {
    return None;
  }
  let wrapped_paragraph = section.columns.count > 2 && section.blocks.len() == 1;
  let precision =
    f64::from(f32::EPSILON) * f64::from(start_y.abs() + end_y.abs()) * (line_count + 1) as f64;
  let mut bottom = f64::from(start_y);
  let mut height = None;
  let mut endpoints = Vec::with_capacity(line_count);
  for frame in frames {
    let Block::Paragraph(paragraph) = section.blocks.get(frame.block_index?)? else {
      return None;
    };
    if frame.kind != FollowFrameKind::Paragraph
      || frame.column_index != 0
      || frame.fragments.is_empty()
      || (frame.fragments.len() != 1
        && (!wrapped_paragraph
          || root.current.footnote_reserved_height_pt != 0.0
          || !cell_text_measure::supports_paragraph(paragraph)
          || paragraph.format.keep_lines
          || !paragraph.format.borders.is_empty()
          || paragraph.format.shading.is_some()
          || !paragraph.footnote_reference_ids.is_empty()
          || !paragraph.endnote_reference_ids.is_empty()))
      || frame
        .influences
        .iter()
        .any(|influence| influence.kind != FrameInfluenceKind::FootnoteReservation)
      || paragraph.format.keep_with_next
      || paragraph_frame(paragraph).is_some()
      || paragraph.inlines.iter().any(|inline| {
        !matches!(
          inline,
          InlineItem::Text(_) | InlineItem::BookmarkStart(_) | InlineItem::LastRenderedPageBreak
        )
      })
    {
      return None;
    }
    for line in &frame.fragments {
      let top = f64::from(line.flow_top_pt?);
      let advance = f64::from(line.content_advance_pt?);
      if line.kind != FrameFragmentKind::ParagraphLine
        || !advance.is_finite()
        || advance <= 0.0
        || line.paragraph_spacing_after_pt != 0.0
        || (top - bottom).abs() > precision
        || height.is_some_and(|height: f64| (height - advance).abs() > precision)
      {
        return None;
      }
      height = Some(advance);
      bottom = top + advance;
      endpoints.push(bottom as f32);
    }
    if (f64::from(frame.flow_bottom_pt?) - bottom).abs() > precision {
      return None;
    }
  }
  if (bottom - f64::from(end_y)).abs() > precision {
    return None;
  }
  // Native modern 3-column controls keep 8/7/10 uniform one-line paragraphs
  // as 3+3+2 / 3+3+1 / 4+4+2. Both admission and the following continuous
  // section use that complete measured row height. The legacy physical mean
  // and the unrelated default 14.4pt line cannot supply either boundary.
  // A plain wrapped paragraph has the same measured rows at equal widths:
  // native Forte/Arial 3-column controls retain 28lines as10/10/8 and32as
  // 11/11/10 in both legacy and modern modes. Reuse those already formatted
  // line ends rather than rounding their mean to an unrelated font's height.
  let rows = endpoints.len().div_ceil(section.columns.count);
  let endpoint = endpoints[rows - 1];
  let frame_height = endpoint - start_y;
  if frame_height >= available_pt || frame_height < DEFAULT_LINE_HEIGHT_PT {
    return None;
  }
  let mut layout_height = frame_height;
  if start_y + layout_height < endpoint {
    layout_height = layout_height.next_up();
  }
  Some(BalancedSectionHeights {
    page_index: root.pages.len(),
    section_page_index: flow.section_page_index,
    frame_top_pt: start_y,
    layout_height_pt: layout_height,
    frame_height_pt: frame_height,
    retained_floating_table: None,
  })
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::docx::{Paragraph, ParagraphFormat};

  fn column_test_section(
    page: PageSetup,
    widths: &[f32],
    break_kind: SectionBreakKind,
    blocks: Vec<Block>,
  ) -> crate::docx::ImportedSection {
    let mut columns = SectionColumns {
      count: widths.len(),
      explicit_count: widths.len(),
      ..Default::default()
    };
    columns.explicit_widths_pt[..widths.len()].copy_from_slice(widths);
    columns.explicit_gaps_pt[..widths.len() - 1].fill(36.0);
    crate::docx::ImportedSection {
      note_reference_order: Default::default(),
      page,
      columns,
      break_kind,
      discarded_carrier: None,
      section_properties: None,
      title_page: false,
      header_blocks: Vec::new(),
      footer_blocks: Vec::new(),
      first_header_blocks: Vec::new(),
      first_footer_blocks: Vec::new(),
      even_header_blocks: Vec::new(),
      even_footer_blocks: Vec::new(),
      blocks,
    }
  }

  fn column_test_text(text: &str, height: f32) -> Block {
    let mut block = super::super::footnote_follow::tests::paragraph(text, false, false);
    let Block::Paragraph(paragraph) = &mut block else {
      unreachable!();
    };
    paragraph.format.line_height_pt = Some(height);
    let style = TextStyle {
      font_family: Some("Courier New".into()),
      font_size_pt: 10.0,
      ..Default::default()
    };
    paragraph.base_style = style.clone();
    for inline in &mut paragraph.inlines {
      if let InlineItem::Text(run) = inline {
        run.style = style.clone();
        run.hyperlink_url = None;
      }
    }
    block
  }

  #[test]
  fn an_empty_terminal_mark_keeps_the_taller_columns_occupied_end() {
    // Native empty/visible-mark opposites retain the same physical column
    // closure. A mark without painted items still supplies its full advance;
    // otherwise it invalidates all measured occupied lines and exposes the
    // shorter geometric mean to the following continuous section.
    for mode in [12, 15] {
      for visible in [false, true] {
        let mut document = super::super::footnote_follow::tests::document();
        document.compatibility_mode = mode;
        let mut blocks = vec![
          column_test_text("A", 18.0),
          column_test_text("B", 18.0),
          column_test_text("C", 18.0),
          column_test_text(if visible { "EMPTY" } else { "" }, 10.0),
        ];
        for block in &mut blocks {
          let Block::Paragraph(paragraph) = block else {
            unreachable!();
          };
          paragraph.format.line_height_rule = LineHeightRule::Exact;
          paragraph.format.widow_control = Some(false);
        }
        let page = document.page;
        document.sections = vec![
          column_test_section(
            page,
            &[288.0],
            SectionBreakKind::NextPage,
            vec![column_test_text("BEFORE", 18.0)],
          ),
          column_test_section(page, &[144.0, 144.0], SectionBreakKind::Continuous, blocks),
          column_test_section(
            page,
            &[288.0],
            SectionBreakKind::Continuous,
            vec![column_test_text("AFTER", 18.0)],
          ),
        ];
        let mut root = RootFrameLayout::new(&document, &LayoutOptions::default());
        root.format_body_frames();
        let source_frames = root
          .frames
          .iter()
          .filter(|frame| frame.body_section_index == Some(1))
          .collect::<Vec<_>>();
        assert_eq!(source_frames.len(), 4);
        assert_eq!(
          source_frames
            .iter()
            .map(|frame| frame.column_index)
            .collect::<Vec<_>>(),
          vec![0, 0, 1, 1],
          "mode={mode}, visible={visible}"
        );
        let mut occupied_bottom = f32::NEG_INFINITY;
        for frame in &source_frames {
          for line in &frame.fragments {
            let advance = line
              .content_advance_pt
              .expect("the mark owns a measured flow line");
            occupied_bottom = occupied_bottom.max(line.flow_top_pt.unwrap() + advance);
          }
        }
        let after = root
          .frames
          .iter()
          .find(|frame| frame.body_section_index == Some(2))
          .unwrap();
        assert_eq!(after.fragments[0].flow_top_pt, Some(occupied_bottom));
      }
    }
  }

  #[test]
  fn unequal_uniform_paragraphs_reflow_before_accepting_a_balanced_height() {
    // Native short-section opposites vary the blank's visibility and both
    // width directions. A caption which fits the wide column must not be
    // accepted as two overflowing lines in the narrow following column.
    for widths in [[60.0, 120.0], [120.0, 60.0]] {
      for mode in [12, 15] {
        for visible in [false, true] {
          let mut document = super::super::footnote_follow::tests::document();
          document.compatibility_mode = mode;
          let page = document.page;
          document.sections = vec![
            column_test_section(
              page,
              &[288.0],
              SectionBreakKind::NextPage,
              vec![column_test_text("BEFORE", 18.0)],
            ),
            column_test_section(
              page,
              &widths,
              SectionBreakKind::Continuous,
              vec![
                column_test_text(if visible { "BLANK" } else { "" }, 18.0),
                column_test_text("hello world caption", 18.0),
              ],
            ),
            column_test_section(
              page,
              &[288.0],
              SectionBreakKind::Continuous,
              vec![column_test_text("AFTER", 18.0)],
            ),
          ];
          let mut root = RootFrameLayout::new(&document, &LayoutOptions::default());
          root.format_body_frames();
          let caption_frames = root
            .frames
            .iter()
            .filter(|frame| frame.body_section_index == Some(1) && frame.block_index == Some(1))
            .collect::<Vec<_>>();
          assert!(!caption_frames.is_empty());
          assert!(
            caption_frames.iter().all(|frame| {
              frame
                .fragments
                .iter()
                .all(|line| line.content_advance_pt.is_some())
            }),
            "the caption must not inherit the preceding unpainted paragraph mark"
          );
          root.finish_current_page();
          assert_eq!(root.pages.len(), 1);
          let items = &root.pages[0].items;
          let marker = |label: &str| {
            items
              .iter()
              .find_map(|item| match item {
                PageItem::Text(text) if text.text == label => Some(text.y_pt),
                _ => None,
              })
              .expect("source marker")
          };
          let wide_first = widths[0] > widths[1];
          let rows = if wide_first { 2.0 } else { 1.0 };
          assert!(
            (marker("AFTER") - marker("BEFORE") - (rows + 1.0) * 18.0).abs() < 0.001,
            "widths={widths:?},mode={mode},visible={visible}: before={},after={}",
            marker("BEFORE"),
            marker("AFTER")
          );
          let caption = items
            .iter()
            .find_map(|item| match item {
              PageItem::Text(text) if text.text.contains("hello") => Some(text),
              _ => None,
            })
            .expect("caption retained");
          let expected_x = page.margin_left_pt + if wide_first { 0.0 } else { widths[0] + 36.0 };
          assert!(
            (caption.x_pt - expected_x).abs() < 0.001,
            "widths={widths:?},mode={mode},visible={visible}"
          );
        }
      }
    }
  }

  #[test]
  fn unequal_final_page_balance_uses_source_suffix_without_moving_the_prefix() {
    for (widths, words, widow, final_rows, left_words) in [
      ([60.0, 120.0], 60, false, 2.0, 10),
      ([120.0, 60.0], 60, false, 2.0, 20),
      ([120.0, 60.0], 80, true, 4.0, 30),
      ([120.0, 60.0], 50, true, 2.0, 20),
      ([120.0, 60.0], 80, false, 4.0, 40),
      ([120.0, 60.0], 50, false, 2.0, 20),
    ] {
      let mut document = super::super::footnote_follow::tests::document();
      document.compatibility_mode = 12;
      document.page.height_pt = 792.0;
      let page = document.page;
      let prelude = page.height_pt - page.margin_top_pt - page.margin_bottom_pt - 36.0;
      document.sections = vec![
        column_test_section(
          page,
          &[288.0],
          SectionBreakKind::NextPage,
          vec![column_test_text("BEFORE", prelude)],
        ),
        column_test_section(
          page,
          &widths,
          SectionBreakKind::Continuous,
          vec![{
            let mut block = column_test_text("a ".repeat(words).trim_end(), 18.0);
            let Block::Paragraph(paragraph) = &mut block else {
              unreachable!()
            };
            paragraph.format.widow_control = Some(widow);
            block
          }],
        ),
        column_test_section(
          page,
          &[288.0],
          SectionBreakKind::Continuous,
          vec![column_test_text("AFTER", 18.0)],
        ),
      ];
      let mut root = RootFrameLayout::new(&document, &LayoutOptions::default());
      root.format_body_frames();
      root.finish_current_page();
      assert_eq!(root.pages.len(), 2);
      let letters = |items: &[PageItem]| {
        items
          .iter()
          .filter_map(|item| match item {
            PageItem::Text(text) => Some(text.text.matches('a').count()),
            _ => None,
          })
          .sum::<usize>()
      };
      assert_eq!(
        letters(&root.pages[0].items),
        30,
        "original prefix at {widths:?}"
      );
      assert_eq!(
        letters(&root.pages[1].items),
        words - 30,
        "source suffix at {widths:?}"
      );
      let last = &root.pages[1].items;
      let split_x = page.margin_left_pt + widths[0] + 36.0;
      let left = last
        .iter()
        .filter_map(|item| match item {
          PageItem::Text(text) if text.x_pt < split_x => Some(text.text.matches('a').count()),
          _ => None,
        })
        .sum::<usize>();
      assert_eq!(
        left, left_words,
        "source cut at {widths:?},words={words},widow={widow}"
      );
      let top = last
        .iter()
        .filter_map(|item| match item {
          PageItem::Text(text) if text.text.contains('a') => Some(text.y_pt),
          _ => None,
        })
        .reduce(f32::min)
        .expect("continued body");
      let after = last
        .iter()
        .find_map(|item| match item {
          PageItem::Text(text) if text.text == "AFTER" => Some(text.y_pt),
          _ => None,
        })
        .expect("following single-column marker");
      assert!(
        (after - top - final_rows * 18.0).abs() < 0.001,
        "{widths:?}: top={top},after={after}"
      );
    }
  }

  #[test]
  fn plain_columns_use_real_line_ends_at_equal_and_unequal_widths() {
    for widths in [
      &[60.0, 120.0][..],
      &[120.0, 60.0],
      &[60.0, 120.0, 36.0],
      &[72.0, 72.0, 72.0],
      &[45.0, 45.0, 45.0, 45.0],
    ] {
      for mode in [12, 15] {
        for unbalanced in [false, true] {
          let mut document = super::super::footnote_follow::tests::document();
          document.page.height_pt = 792.0;
          document.compatibility_mode = mode;
          let page = document.page;
          let mut columns = column_test_section(
            page,
            widths,
            SectionBreakKind::Continuous,
            vec![column_test_text("a ".repeat(60).trim_end(), 18.0)],
          );
          columns.columns.unbalanced = unbalanced;
          document.sections = vec![
            column_test_section(
              page,
              &[288.0],
              SectionBreakKind::NextPage,
              vec![column_test_text("BEFORE", 18.0)],
            ),
            columns,
            column_test_section(
              page,
              &[288.0],
              SectionBreakKind::Continuous,
              vec![column_test_text("AFTER", 18.0)],
            ),
          ];
          let mut root = RootFrameLayout::new(&document, &LayoutOptions::default());
          root.format_body_frames();
          root.finish_current_page();
          assert_eq!(root.pages.len(), 1);
          let y = |label: &str| {
            root.pages[0]
              .items
              .iter()
              .find_map(|item| match item {
                PageItem::Text(text) if text.text == label => Some(text.y_pt),
                _ => None,
              })
              .expect("source marker is present")
          };
          // Courier's 6pt advances admit 5/10/3/6/4 words in these widths.
          // widths. Four complete rows suffice in either column direction;
          // an authored no-balance keeps the paragraph in its first column.
          let rows = if unbalanced {
            if widths[0] == 60.0 {
              12
            } else if widths[0] == 120.0 {
              6
            } else if widths[0] == 72.0 {
              10
            } else {
              15
            }
          } else {
            4
          };
          assert!(
            (y("AFTER") - y("BEFORE") - (rows + 1) as f32 * 18.0).abs() < 0.001,
            "widths={widths:?}, mode={mode}, unbalanced={unbalanced}",
          );
          let letters = root.pages[0]
            .items
            .iter()
            .filter_map(|item| match item {
              PageItem::Text(text) => Some(text.text.matches('a').count()),
              _ => None,
            })
            .sum::<usize>();
          assert_eq!(letters, 60, "all source words survive column reflow");
        }
      }
    }
  }

  #[test]
  fn balanced_follow_uses_its_physical_page_and_current_section_counter() {
    for prelude_lines in [13_usize, 26] {
      for admitted_blank in [false, true] {
        let mut document = super::super::footnote_follow::tests::document();
        document.compatibility_mode = 12;
        document.page = PageSetup {
          width_pt: 612.0,
          height_pt: 792.0,
          margin_top_pt: 72.0,
          margin_bottom_pt: 72.0,
          margin_left_pt: 90.0,
          margin_right_pt: 90.0,
          ..Default::default()
        };
        let page = document.page;
        let mut prelude = (0..prelude_lines)
          .map(|index| column_test_text(&format!("PRELUDE{index:02}"), 48.0))
          .collect::<Vec<_>>();
        // Leave 8pt on the old physical page. A 6pt blank is a real source
        // admission; without it the first content follows with counter0.
        prelude.push(column_test_text("", 16.0));
        let mut body = Vec::new();
        if admitted_blank {
          body.push(column_test_text("", 6.0));
        }
        body.push(column_test_text("a ".repeat(30).trim_end(), 18.0));
        document.sections = vec![
          column_test_section(page, &[432.0], SectionBreakKind::NextPage, prelude),
          column_test_section(page, &[264.0, 132.0], SectionBreakKind::Continuous, body),
          column_test_section(
            page,
            &[432.0],
            SectionBreakKind::Continuous,
            vec![column_test_text("END", 18.0)],
          ),
        ];
        let mut root = RootFrameLayout::new(&document, &LayoutOptions::default());
        root.format_body_frames();
        root.finish_current_page();
        assert_eq!(root.pages.len(), prelude_lines / 13 + 1);
        let last = root.pages.last().expect("physical follow");
        let mut left = None;
        let mut right = None;
        let mut letters = 0;
        for item in &last.items {
          if let PageItem::Text(text) = item
            && text.text.contains('a')
          {
            letters += text.text.matches('a').count();
            if text.x_pt < 390.0 {
              left.get_or_insert(text.y_pt);
            } else {
              right.get_or_insert(text.y_pt);
            }
          }
        }
        assert_eq!(
          letters, 30,
          "all text follows to its selected physical page"
        );
        let left = left.expect("first column retains a source line");
        let right = right.expect("second column receives the balanced suffix");
        assert!(
          (left - right).abs() < 0.001,
          "prelude_lines={prelude_lines}, admitted_blank={admitted_blank}",
        );
      }
    }
  }

  #[test]
  fn balanced_uniform_lines_reserve_each_columns_fixed_fly_gap() {
    let geometry = ColumnGeometry {
      lefts: vec![36.0, 324.0],
      widths: vec![252.0, 252.0],
      gaps: vec![36.0],
    };
    let fly = WrapExclusion {
      left_pt: 26.0,
      right_pt: 595.0,
      top_pt: 66.0,
      bottom_pt: 84.0,
      side: ImageWrapSide::BothSides,
      blocks_flow: false,
      uses_contour: false,
      contour_band: None,
      owner: WrapExclusionOwner::FloatingTable,
    };
    // Native odd-tail controls keep 3/5/7/9 body paragraphs as 2+1,
    // 3+2, 4+3 and 5+4, with the same fixed fly avoidance in both columns.
    for (count, bottom) in [
      (3usize, 101.0),
      (4, 101.0),
      (5, 118.0),
      (7, 135.0),
      (9, 152.0),
    ] {
      assert_eq!(
        uniform_wrapped_column_bottom(36.0, 17.0, count.div_ceil(2), &geometry, fly),
        bottom
      );
    }
    // A larger negative offset reaches the first line; a later source
    // anchor can put the table entirely below the balanced text frame.
    assert_eq!(
      uniform_wrapped_column_bottom(
        36.0,
        17.0,
        2,
        &geometry,
        WrapExclusion {
          top_pt: 50.0,
          bottom_pt: 68.0,
          ..fly
        }
      ),
      102.0
    );
    assert_eq!(
      uniform_wrapped_column_bottom(
        36.0,
        17.0,
        2,
        &geometry,
        WrapExclusion {
          top_pt: 100.0,
          bottom_pt: 118.0,
          ..fly
        }
      ),
      70.0
    );
    // Exact line heights use their real advances, not the default-font slot.
    assert_eq!(
      uniform_wrapped_column_bottom(36.0, 12.0, 3, &geometry, fly),
      96.0
    );
    assert_eq!(
      uniform_wrapped_column_bottom(36.0, 24.0, 3, &geometry, fly),
      132.0
    );
  }

  #[test]
  fn continuous_columns_preserve_tallest_text_and_authored_lower_spacing() {
    let page = PageSetup {
      width_pt: 612.0,
      height_pt: 792.0,
      margin_top_pt: 72.0,
      margin_bottom_pt: 72.0,
      margin_left_pt: 72.0,
      margin_right_pt: 72.0,
      ..Default::default()
    };
    let paragraph = |text: String, after: f32| {
      let style = TextStyle {
        font_family: Some("Calibri".into()),
        font_size_pt: 11.0,
        ..Default::default()
      };
      let run = TextRun {
        text,
        style: style.clone(),
        hyperlink_url: None,
        dynamic_field: None,
        style_ref_keys: Vec::new(),
        style_ref_text: None,
        style_ref_numbering_text: None,
        preserve_text_portion: false,
      };
      Block::paragraph(Paragraph {
        inlines: vec![InlineItem::Text(run.clone())],
        field_events: Vec::new(),
        footnote_reference_ids: Vec::new(),
        endnote_reference_ids: Vec::new(),
        starts_after_last_rendered_page_break: false,
        base_style: style,
        runs: vec![run],
        format: Box::new(ParagraphFormat {
          line_height_rule: LineHeightRule::Exact,
          line_height_pt: Some(14.0),
          spacing_after_pt: after,
          widow_control: Some(false),
          ..Default::default()
        }),
        style_ref_keys: Vec::new(),
        style_ref_text: None,
        style_ref_numbering_text: None,
        list_label: None,
        list_label_image: None,
        list_label_style: TextStyle::default(),
        list_label_hyperlink_url: None,
        list_label_tab_stop_pt: None,
      })
    };
    let section = |count, break_kind, blocks| crate::docx::ImportedSection {
      note_reference_order: Default::default(),
      page,
      columns: SectionColumns {
        count,
        ..Default::default()
      },
      break_kind,
      discarded_carrier: None,
      section_properties: None,
      title_page: false,
      header_blocks: Vec::new(),
      footer_blocks: Vec::new(),
      first_header_blocks: Vec::new(),
      first_footer_blocks: Vec::new(),
      even_header_blocks: Vec::new(),
      even_footer_blocks: Vec::new(),
      blocks,
    };
    // Independent Word controls cover odd tails, an unused last column and
    // lower spacing that keeps the physical mean above occupied text. Every
    // section shares page owner0: source section1 must use its checkpoint.
    for (columns, count, after, expected_height) in [
      (1, 3, 0.0, 42.0),
      (2, 3, 0.0, 28.0),
      (2, 4, 0.0, 28.0),
      (2, 5, 0.0, 42.0),
      (4, 5, 0.0, 28.0),
      (4, 6, 0.0, 28.0),
      (2, 2, 14.0, 28.0),
      (2, 3, 14.0, 42.0),
    ] {
      let document = DocxDocument {
        page,
        page_background_pattern: None,
        page_background_texture: None,
        page_background_gradient: None,
        line_number_style: TextStyle::default(),
        note_separator_style: TextStyle::default(),
        footnote_separator_stories: Default::default(),
        endnote_separator_stories: Default::default(),
        default_tab_stop_pt: DEFAULT_TAB_STOP_PT,
        hyphenation: Default::default(),
        compatibility_mode: 14,
        justify_lines_with_shrinking: false,
        do_not_expand_shift_return: false,
        suppress_top_spacing: false,
        even_and_odd_headers: false,
        split_page_break_and_paragraph_mark: false,
        form_widgets: Vec::new(),
        sections: vec![
          section(
            1,
            SectionBreakKind::NextPage,
            vec![paragraph("BEFORE".into(), 0.0)],
          ),
          section(
            columns,
            SectionBreakKind::Continuous,
            (0..count)
              .map(|index| paragraph(format!("LINE{index:02}"), after))
              .collect(),
          ),
          section(
            1,
            SectionBreakKind::Continuous,
            vec![paragraph("AFTER".into(), 0.0)],
          ),
        ],
        toc_print_cache: None,
        header_blocks: Vec::new(),
        footer_blocks: Vec::new(),
        first_header_blocks: Vec::new(),
        first_footer_blocks: Vec::new(),
        footnote_blocks: Vec::new(),
        footnotes: Default::default(),
        footnote_numbering: Vec::new(),
        footnote_positions: Vec::new(),
        endnotes: Default::default(),
        endnote_numbering: Vec::new(),
        endnote_position: w::EndnotePositionValues::DocumentEnd,
        title_page: false,
        num_pages_excludes_cover: false,
        blocks: Vec::new(),
      };
      let mut root = RootFrameLayout::new(&document, &LayoutOptions::default());
      root.format_body_frames();
      root.finish_current_page();
      assert_eq!(root.pages.len(), 1);
      assert!(root.frames.iter().all(|frame| frame.section_index == 0));
      let text_y = |label: &str| {
        root.pages[0]
          .items
          .iter()
          .find_map(|item| match item {
            PageItem::Text(text) if text.text == label => Some(text.y_pt),
            _ => None,
          })
          .expect("all source text is present")
      };
      let actual = text_y("AFTER") - text_y("LINE00");
      assert!(
        (actual - expected_height).abs() < 0.001,
        "columns={columns},count={count},after={after}: {actual} != {expected_height}",
      );
      if columns == 4 && after == 0.0 {
        for line_height in [13.427_734, 14.0, 14.038_086] {
          let mut measured = document.clone();
          for section in &mut measured.sections {
            for block in &mut section.blocks {
              if let Block::Paragraph(paragraph) = block {
                paragraph.format.line_height_pt = Some(line_height);
              }
            }
          }
          let mut measured_root = RootFrameLayout::new(&measured, &LayoutOptions::default());
          measured_root.format_body_frames();
          measured_root.finish_current_page();
          assert_eq!(measured_root.pages.len(), 1);
          let position = |label: &str| {
            measured_root.pages[0]
              .items
              .iter()
              .find_map(|item| match item {
                PageItem::Text(text) if text.text == label => Some((text.x_pt, text.y_pt)),
                _ => None,
              })
              .expect("all measured source text is present")
          };
          // Native five/six-line four-column controls occupy three columns
          // in pairs. A default-font slot can preserve the final marker yet
          // still split five lines as1/1/1/2, so check the actual partition.
          assert_eq!(position("LINE00").0, position("LINE01").0);
          assert!(position("LINE02").0 > position("LINE01").0);
          assert_eq!(position("LINE02").0, position("LINE03").0);
          assert!(position("LINE04").0 > position("LINE03").0);
          if count == 6 {
            assert_eq!(position("LINE04").0, position("LINE05").0);
          }
          assert!((position("LINE01").1 - position("LINE00").1 - line_height).abs() < 0.001);
          assert!((position("AFTER").1 - position("LINE00").1 - 2.0 * line_height).abs() < 0.001);
        }
      }
    }
  }

  fn control(title: f64, body: f64, before: f64, count: usize, widow: bool) -> Vec<MeasuredLine> {
    paragraph_lines(&[(1, title, 0.0), (count, body, before)], widow)
  }

  fn paragraph_lines(paragraphs: &[(usize, f64, f64)], widow: bool) -> Vec<MeasuredLine> {
    let mut lines = Vec::new();
    let mut bottom = 0.0;
    for &(count, height, before) in paragraphs {
      for index in 0..count {
        let upper = if index == 0 { before } else { 0.0 };
        bottom += upper + height;
        lines.push(MeasuredLine {
          height_pt: height,
          before_pt: upper,
          after_pt: 0.0,
          break_after: index + 1 == count || !widow || (index + 1 >= 2 && count - index > 2),
          source_bottom_pt: bottom as f32,
        });
      }
    }
    lines
  }

  #[test]
  fn legacy_fixed_line_columns_retain_native_nonoptimal_partitions() {
    // Independent Word controls: the 0/1.7pt upper-spacing change reverses
    // the five-line partition. A minimax partition is incorrect in the first.
    for widow in [false, true] {
      for (title, body, before, count, left, frame) in [
        (8.95, 9.4, 0.0, 5, 3, 37.15),
        (8.95, 9.4, 1.7, 5, 2, 29.45),
        (8.95, 9.4, 1.7, 4, 2, 29.45),
        (11.2, 9.4, 1.7, 6, 3, 41.1),
        (8.95, 9.4, 0.0, 9, 5, 55.95),
      ] {
        let lines = control(title, body, before, count, widow);
        let heights = measured_heights(&lines, 700.0, 0.0).expect("native control fits");
        assert!((f64::from(heights.frame_pt) - frame).abs() < 0.000_01);
        assert_eq!(heights.first_line_count, left + 1);
        let occupied_left = title + before + left as f64 * body;
        assert!((occupied_left - f64::from(heights.first_height_pt)).abs() < 0.000_01);
      }
    }
  }

  #[test]
  fn measured_column_plan_rejects_unusable_or_unbounded_line_metrics() {
    assert!(measured_heights(&[], 700.0, 0.0).is_none());
    let mut lines = control(8.95, 9.4, 1.7, 5, true);
    assert!(measured_heights(&lines, 28.0, 0.0).is_none());
    lines[1].height_pt = f64::NAN;
    assert!(measured_heights(&lines, 700.0, 0.0).is_none());
  }

  #[test]
  fn blocked_following_column_retains_native_text_capacity_and_empty_column_policy() {
    for widow in [false, true] {
      for (count, blocked, left, frame) in [
        (16, 80.0, 12, 123.45),
        (32, 160.0, 24, 236.25),
        (16, 240.0, 16, 161.05),
      ] {
        let lines = control(8.95, 9.4, 1.7, count, widow);
        let heights =
          measured_heights(&lines, 700.0, blocked).expect("native obstacle control fits");
        assert!((f64::from(heights.frame_pt) - frame).abs() < 0.000_02);
        assert_eq!(heights.first_line_count, left + 1);
      }
    }
  }

  #[test]
  fn leading_obstacle_preserves_the_native_widow_prefix_deadline() {
    // Native95case21: a189.0625pt trial admits15visible body lines if its
    // slack becomes the actual first-column deadline. Word retains14/2.
    let lines = control(8.95, 11.2, 1.7, 16, true);
    let heights = measured_heights(&lines, 700.0, 160.0).expect("native widow control fits");
    assert_eq!(heights.first_line_count, 15); // title plus14bodylines
    assert!((heights.first_height_pt - 167.45).abs() < 0.000_02);
    assert!((heights.frame_pt - 182.4).abs() < 0.000_02);
    assert_eq!(
      lines
        .iter()
        .take_while(|line| line.source_bottom_pt <= heights.first_height_pt)
        .count(),
      15,
    );
  }

  #[test]
  fn measured_relaxation_matches_independent_native_mixed_height_and_widow_controls() {
    // Native103's independent two-paragraph matrix. These include every
    // counterexample to the former half-overhang/next-line growth rule.
    for (high_count, low_count, low_height, before, obstacle, left) in [
      (1, 4, 11.05, 19.05, 0.0, 3),
      (1, 4, 11.2, 19.05, 0.0, 3),
      (1, 18, 11.2, 19.05, 207.1, 17),
      (6, 18, 11.05, 0.0, 207.1, 22),
      (12, 12, 11.05, 19.05, 207.1, 21),
      (12, 12, 11.2, 19.05, 207.1, 21),
      (12, 18, 11.05, 19.05, 207.1, 23),
      (12, 18, 11.05, 0.0, 207.1, 25),
      (12, 18, 11.2, 19.05, 207.1, 23),
      (20, 12, 11.05, 19.05, 207.1, 24),
      (20, 12, 11.2, 19.05, 207.1, 24),
      (20, 18, 11.2, 0.0, 207.1, 29),
      (29, 18, 11.05, 19.05, 207.1, 33),
      (29, 18, 11.2, 19.05, 207.1, 32),
    ] {
      let lines = paragraph_lines(
        &[(high_count, 11.2, before), (low_count, low_height, 0.0)],
        true,
      );
      let heights = measured_heights(&lines, 700.0, obstacle).expect("native profile fits");
      assert_eq!(
        heights.first_line_count, left,
        "high={high_count},low={low_count},height={low_height},before={before},obstacle={obstacle}",
      );
    }
  }

  #[test]
  fn measured_relaxation_skips_retries_with_unchanged_admitted_prefixes() {
    let lines = paragraph_lines(&[(1, 1.0, 0.0), (1, 1_000_000.0, 0.0)], false);
    // A one-point step would otherwise need half a million identical trials.
    let heights = measured_heights(&lines, 2_000_000.0, 0.0).expect("finite profile fits");
    assert_eq!(heights.first_line_count, 1);
    assert_eq!(heights.first_height_pt, 1.0);
    assert_eq!(heights.frame_pt, 1_000_000.0);
  }

  #[test]
  fn terminal_lower_space_participates_in_balance_but_clips_after_admitted_text() {
    // Independent Native106 before/after controls, including lower-space
    // clipping in either column and a subsequent mean-height relaxation.
    for (high, low, height, before, first_after, last_after, left, frame) in [
      (12, 12, 11.05, 0.0, 0.0, 5.95, 12, 136.475),
      (12, 12, 11.05, 0.0, 11.5, 0.0, 12, 139.25),
      (12, 12, 11.05, 19.05, 11.5, 5.95, 12, 164.270_454_545_454_6),
      (12, 18, 11.05, 0.0, 11.5, 5.95, 15, 179.05),
      (12, 18, 11.05, 19.05, 11.5, 5.95, 14, 187.05),
      (29, 12, 11.05, 19.05, 11.5, 5.95, 20, 246.95),
      (29, 18, 11.2, 19.05, 11.5, 5.95, 23, 281.45),
    ] {
      let mut lines = paragraph_lines(&[(high, 11.2, before), (low, height, 0.0)], true);
      lines[high - 1].after_pt = first_after;
      lines.last_mut().unwrap().after_pt = last_after;
      let measured = measured_heights(&lines, 700.0, 0.0).expect("native lower-space control fits");
      assert_eq!(measured.first_line_count, left);
      assert!((f64::from(measured.frame_pt) - frame).abs() < 0.000_02);
    }
  }

  #[test]
  fn physical_column_follow_omits_unused_capacity_and_incoming_frame_column() {
    // Actual GDB source: a paragraph follows whole from the first column's
    // 744.40033pt lower to48.5pt, while LayoutFrame still records incoming0.
    let mut chain = SourceColumnChain {
      column: 0,
      end_column: 1,
      top_pt: 48.5,
      bottom_pt: 744.400_33,
      precision_pt: 0.001,
    };
    assert_eq!(chain.before_line(48.5, false), Some(0.0));
    assert_eq!(chain.column, 1);
    chain.bottom_pt = 74.6;
    assert!((chain.before_line(75.2, false).unwrap() - 0.6).abs() < 0.000_01);
    assert!(chain.before_line(48.0, false).is_none());
  }

  #[test]
  fn following_column_plan_requires_a_complete_leading_vml_obstacle() {
    let geometry = ColumnGeometry {
      lefts: vec![50.0, 305.0],
      widths: vec![245.0, 245.0],
      gaps: vec![10.0],
    };
    let full = WrapExclusion {
      left_pt: 305.0,
      right_pt: 600.0,
      top_pt: 0.0,
      bottom_pt: 130.0,
      side: ImageWrapSide::BothSides,
      blocks_flow: false,
      uses_contour: false,
      contour_band: None,
      owner: WrapExclusionOwner::VmlBodyDrawing,
    };
    assert_eq!(following_column_top(&[full], &geometry, 50.0), Some(80.0));
    for unsupported in [
      WrapExclusion {
        left_pt: 350.0,
        ..full
      },
      WrapExclusion {
        left_pt: 50.0,
        ..full
      },
      WrapExclusion {
        top_pt: 75.0,
        ..full
      },
      WrapExclusion {
        uses_contour: true,
        ..full
      },
    ] {
      assert!(following_column_top(&[unsupported], &geometry, 50.0).is_none());
    }
    let outside = WrapExclusion {
      left_pt: 551.0,
      ..full
    };
    assert_eq!(following_column_top(&[outside], &geometry, 50.0), Some(0.0));
  }
}
