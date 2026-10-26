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
    section_page_index: root.current.section_page_index,
    frame_top_pt: start_y,
    layout_height_pt: layout_height,
    frame_height_pt: heights.frame_pt.max(first_height),
  })
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
  if frames.len() < 2 {
    return None;
  }
  let precision =
    f64::from(f32::EPSILON) * f64::from(start_y.abs() + end_y.abs()) * (frames.len() + 1) as f64;
  let mut bottom = f64::from(start_y);
  let mut height = None;
  let mut endpoints = Vec::with_capacity(frames.len());
  for frame in frames {
    let Block::Paragraph(paragraph) = section.blocks.get(frame.block_index?)? else {
      return None;
    };
    if frame.kind != FollowFrameKind::Paragraph
      || frame.column_index != 0
      || frame.fragments.len() != 1
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
    let line = &frame.fragments[0];
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
    if (f64::from(frame.flow_bottom_pt?) - bottom).abs() > precision {
      return None;
    }
    endpoints.push(bottom as f32);
  }
  if (bottom - f64::from(end_y)).abs() > precision {
    return None;
  }
  // Native modern 3-column controls keep 8/7/10 uniform one-line paragraphs
  // as 3+3+2 / 3+3+1 / 4+4+2. Both admission and the following continuous
  // section use that complete measured row height. The legacy physical mean
  // and the unrelated default 14.4pt line cannot supply either boundary.
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
    section_page_index: root.current.section_page_index,
    frame_top_pt: start_y,
    layout_height_pt: layout_height,
    frame_height_pt: frame_height,
  })
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::docx::{Paragraph, ParagraphFormat};

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
      page,
      columns: SectionColumns {
        count,
        ..Default::default()
      },
      break_kind,
      discarded_carrier_spacing_after_pt: None,
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
