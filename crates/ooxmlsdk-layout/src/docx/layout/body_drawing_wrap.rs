use super::*;

mod column_balance;
pub(super) use column_balance::constrain_flow;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(super) struct Anchor {
  section: usize,
  block: usize,
  inline: usize,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct Reservation {
  anchor: Anchor,
  page: usize,
  minimum_page: Option<usize>,
  active: bool,
  revision: usize,
  restart_block: Option<usize>,
  pending_restart: Option<usize>,
  pub image_x: f32,
  pub image_y: f32,
  pub exclusion: WrapExclusion,
}

#[derive(Clone, Debug, Default)]
pub(super) struct Catalog {
  drawings: Vec<Reservation>,
  retired: Vec<(usize, WrapExclusion)>,
  balances: Vec<column_balance::Plan>,
}

impl Catalog {
  pub(super) fn reservation(
    &self,
    flow: FlowContext,
    inline_index: usize,
    page_index: usize,
  ) -> Option<Reservation> {
    let anchor = Anchor {
      section: flow.section_index,
      block: flow.body_source_block_index?,
      inline: inline_index,
    };
    self
      .drawings
      .iter()
      .find(|drawing| drawing.anchor == anchor && drawing.active && drawing.page == page_index)
      .copied()
  }
}

pub(super) fn square_owns_body_wrap(placement: FloatingImagePlacement, flow: FlowContext) -> bool {
  placement.wrap == ImageWrapMode::Square
    && placement.paint_order.is_vml()
    && matches!(
      placement.vertical_relative_to,
      VerticalImageReference::Page | VerticalImageReference::Margin
    )
    && placement.horizontal_relative_to != HorizontalImageReference::Character
    && flow.text_segmentation == TextSegmentation::Body
    && !flow.inside_paragraph_frame
}

pub(super) fn capture(
  page: &mut Page,
  page_index: usize,
  flow: FlowContext,
  inline_index: usize,
  placement: FloatingImagePlacement,
  bounds: FrameBounds,
) -> Option<Reservation> {
  let vml_square = square_owns_body_wrap(placement, flow);
  if flow.text_segmentation != TextSegmentation::Body
    || flow.inside_paragraph_frame
    || (!vml_square
      && ((placement.behind_text && placement.paint_order.is_vml())
        || placement.wrap != ImageWrapMode::TopBottom
        || (!placement.paint_order.is_vml() && flow.columns.count <= 1)))
    || placement.horizontal_relative_to == HorizontalImageReference::Character
    || placement.vertical_relative_to == VerticalImageReference::Line
    || (placement.vertical_relative_to == VerticalImageReference::Paragraph
      && !(placement.paint_order.is_drawing_ml()
        && placement.vertical_alignment.is_none()
        && placement.vertical_offset_pct.is_none()))
  {
    return None;
  }
  let anchor = Anchor {
    section: flow.section_index,
    block: flow.body_source_block_index?,
    inline: inline_index,
  };
  if let Some(reservation) = page.body_drawing_wraps.as_deref().and_then(|catalog| {
    catalog
      .drawings
      .iter()
      .find(|drawing| drawing.anchor == anchor)
  }) {
    if reservation.active && reservation.page == page_index {
      return Some(*reservation);
    }
    if page_index < reservation.page {
      return None;
    }
  }
  let exclusion = if vml_square {
    WrapExclusion {
      left_pt: bounds.x_pt - placement.margin_left_pt,
      right_pt: bounds.x_pt + bounds.width_pt + placement.margin_right_pt,
      top_pt: bounds.y_pt - placement.margin_top_pt,
      bottom_pt: bounds.y_pt + bounds.height_pt + placement.margin_bottom_pt,
      side: placement.wrap_side,
      blocks_flow: false,
      uses_contour: false,
      contour_band: None,
      owner: WrapExclusionOwner::VmlBodyDrawing,
    }
  } else {
    vertical_wrap_exclusion(
      flow,
      bounds.x_pt - placement.margin_left_pt,
      bounds.y_pt - placement.margin_top_pt,
      bounds.x_pt + bounds.width_pt + placement.margin_right_pt,
      bounds.y_pt + bounds.height_pt + placement.margin_bottom_pt,
    )?
  };
  let reservation = Reservation {
    anchor,
    page: page_index,
    minimum_page: None,
    active: true,
    revision: 0,
    restart_block: None,
    pending_restart: None,
    image_x: bounds.x_pt,
    image_y: bounds.y_pt,
    exclusion,
  };
  let catalog = Arc::make_mut(page.body_drawing_wraps.get_or_insert_with(Default::default));
  if let Some(index) = catalog
    .drawings
    .iter()
    .position(|drawing| drawing.anchor == anchor)
  {
    let previous = catalog.drawings[index];
    if previous.active {
      catalog.retired.push((previous.page, previous.exclusion));
    }
    let reservation = Reservation {
      minimum_page: if previous.active && page_index > previous.page {
        Some(page_index.max(previous.minimum_page.unwrap_or(0)))
      } else {
        previous.minimum_page
      },
      revision: previous.revision + 1,
      // The inline can be reached on the follow before the root sees the
      // paragraph's completed frames. Retain the old band's affected prefix
      // through that capture as well as through a post-format migration.
      pending_restart: previous
        .active
        .then_some(previous.restart_block.unwrap_or(anchor.block)),
      ..reservation
    };
    catalog.drawings[index] = reservation;
    Some(reservation)
  } else {
    catalog.drawings.push(reservation);
    Some(reservation)
  }
}

fn first_intersected_block(drawing: &Reservation, frames: &[LayoutFrame]) -> Option<usize> {
  if !drawing.active {
    return None;
  }
  let overlaps = |x, y, width, height| {
    drawing.exclusion.overlaps_horizontal_span(x, x + width)
      && drawing.exclusion.overlaps_vertical_span(y, y + height)
  };
  frames
    .iter()
    .filter(|frame| {
      frame.page_index == drawing.page
        && frame.kind == FollowFrameKind::Paragraph
        && (frame
          .lines
          .iter()
          .any(|line| overlaps(line.x_pt, line.y_pt, line.width_pt, line.height_pt))
          || frame.fragments.iter().any(|fragment| {
            // Empty marks have physical line boxes but no painted item from
            // which line_boxes_for_items could recover them.
            fragment.kind == FrameFragmentKind::ParagraphLine
              && fragment.bounds.is_some_and(|bounds| {
                if drawing.exclusion.owner == WrapExclusionOwner::VmlBodyDrawing {
                  // The terminal line owns its paragraph's lower spacing.
                  // A later square fly can invalidate that strip even when
                  // none of the earlier glyphs intersects the fly itself.
                  overlaps(
                    bounds.x_pt,
                    fragment.flow_top_pt.unwrap_or(bounds.y_pt),
                    bounds.width_pt,
                    fragment.content_advance_pt.unwrap_or(bounds.height_pt)
                      + fragment.paragraph_spacing_after_pt.max(0.0),
                  )
                } else {
                  fragment.item_start == fragment.item_end
                    && overlaps(bounds.x_pt, bounds.y_pt, bounds.width_pt, bounds.height_pt)
                }
              })
          }))
    })
    .filter_map(|frame| frame.block_index)
    .min()
}

pub(super) fn terminal_line_clearance(
  line: FrameBounds,
  before_pt: f32,
  after_pt: f32,
  exclusions: &[WrapExclusion],
) -> f32 {
  if after_pt <= LAYOUT_EPSILON_PT {
    return line.y_pt;
  }
  let squares = exclusions
    .iter()
    .copied()
    .filter(|exclusion| exclusion.owner == WrapExclusionOwner::VmlBodyDrawing)
    .collect::<Vec<_>>();
  if squares.is_empty() {
    return line.y_pt;
  }
  // Native one/two/three-line controls distinguish the terminal lower space
  // from the first-line upper space: only the intersecting terminal line
  // moves. A single-line paragraph retains its upper space below the fly.
  dodge_text_wrap_exclusions(
    line.y_pt - before_pt,
    line.height_pt + before_pt + after_pt,
    line.x_pt,
    line.x_pt + line.width_pt,
    &squares,
  ) + before_pt
}

pub(super) fn install(page: &mut Page, page_index: usize, section_index: usize) {
  let Some(catalog) = &page.body_drawing_wraps else {
    return;
  };
  page.wrap_exclusions.retain(|exclusion| {
    !catalog
      .retired
      .iter()
      .any(|(old_page, old_exclusion)| *old_page == page_index && old_exclusion == exclusion)
  });
  for drawing in &catalog.drawings {
    if drawing.active && drawing.page == page_index && drawing.anchor.section == section_index {
      extend_wrap_exclusions_unique(&mut page.wrap_exclusions, &[drawing.exclusion]);
    }
  }
}

impl RootFrameLayout<'_> {
  pub(super) fn admit_body_drawing_anchor(
    &mut self,
    flow: &mut FlowContext,
    block_index: usize,
  ) -> Option<LayoutCheckpoint> {
    let minimum_page = self
      .current
      .body_drawing_wraps
      .as_deref()
      .and_then(|catalog| {
        catalog
          .drawings
          .iter()
          .filter(|drawing| {
            drawing.anchor.section == flow.section_index && drawing.anchor.block == block_index
          })
          .filter_map(|drawing| drawing.minimum_page)
          .max()
      });
    let minimum_page = minimum_page?;
    if let Some(restart) = self.balance_before_drawing_follow(block_index, *flow, minimum_page) {
      return Some(restart);
    }
    // A picture displaced by its own wrapping must follow its anchor. Keep
    // that forward decision when withdrawing its old page's exclusion; this
    // is the same monotone placement used by Typst's scoped float migration
    // and Writer's moved-forward anchor registration, not a cached page break.
    while self.pages.len() < minimum_page {
      let transition = self.follow_transition_start(*flow);
      (*flow, self.y) = advance_section_flow(*flow, &mut self.current, &mut self.pages);
      self.record_follow_transition(
        transition,
        *flow,
        FollowFrameKind::Paragraph,
        FollowReason::Overflow,
        Some(block_index),
      );
    }
    None
  }

  pub(super) fn replay_before_body_drawing(
    &mut self,
    block_index: usize,
    checkpoint: LayoutCheckpoint,
    replayed: &mut HashSet<(Anchor, usize)>,
  ) -> Option<LayoutCheckpoint> {
    let catalog = self.current.body_drawing_wraps.as_ref()?;
    if !catalog.drawings.iter().any(|drawing| {
      drawing.anchor.section == checkpoint.section_index && drawing.anchor.block == block_index
    }) {
      return None;
    }
    // LayoutFrame's section fields describe its physical page for final
    // normalization/replay. Several continuous source sections can share that
    // page. Their checkpoints, not the page's style owner, delimit the source
    // frames that this drawing may invalidate.
    let section_frame_start = self
      .checkpoints
      .iter()
      .find(|candidate| candidate.section_index == checkpoint.section_index)
      .map_or(checkpoint.frames_len, |candidate| candidate.frames_len);
    let previous_frames = &self.frames[section_frame_start..checkpoint.frames_len];
    // A floating picture is not its anchor's paragraph line. Empty marks
    // have no painted LineBox; inspect only the physical line fragments
    // created by this block, excluding earlier empty marks at the same item
    // cursor. Native controls give empty and visible anchors the same follow.
    let first_line_page = (checkpoint.page_index..=self.pages.len()).find(|&page_index| {
      let page = self.pages.get(page_index).unwrap_or(&self.current);
      let first_fragment = if page_index == checkpoint.page_index {
        checkpoint.current.frame_fragments_len
      } else {
        0
      };
      page.frame_fragments[first_fragment..]
        .iter()
        .any(|fragment| fragment.kind == FrameFragmentKind::ParagraphLine)
    });
    let has_migration = first_line_page.is_some_and(|first_line_page| {
      catalog.drawings.iter().any(|drawing| {
        drawing.anchor.section == checkpoint.section_index
          && drawing.anchor.block == block_index
          && drawing.active
          && first_line_page > drawing.page
      })
    });
    let catalog = self.current.body_drawing_wraps.as_mut()?;
    if has_migration {
      let first_line_page = first_line_page.expect("migrated anchor line page");
      let catalog = Arc::make_mut(catalog);
      for drawing in &mut catalog.drawings {
        if drawing.anchor.section == checkpoint.section_index
          && drawing.anchor.block == block_index
          && drawing.active
          && first_line_page > drawing.page
        {
          catalog.retired.push((drawing.page, drawing.exclusion));
          drawing.minimum_page = Some(first_line_page);
          drawing.page = first_line_page;
          drawing.active = false;
          drawing.revision += 1;
          drawing.pending_restart = Some(drawing.restart_block.take().unwrap_or(block_index));
        }
      }
    }
    let first_block = catalog
      .drawings
      .iter()
      .filter(|drawing| {
        drawing.anchor.section == checkpoint.section_index
          && drawing.anchor.block == block_index
          && !replayed.contains(&(drawing.anchor, drawing.revision))
      })
      .filter_map(|drawing| {
        first_intersected_block(drawing, previous_frames)
          .into_iter()
          .chain(drawing.pending_restart)
          .min()
      })
      .min()?;
    let (checkpoint_index, restart) =
      self.checkpoints.iter().enumerate().rev().find(|(_, c)| {
        c.section_index == checkpoint.section_index && c.block_index == first_block
      })?;
    let restart = *restart;
    let catalog = Arc::make_mut(catalog);
    for drawing in &mut catalog.drawings {
      if drawing.anchor.section == checkpoint.section_index && drawing.anchor.block == block_index {
        replayed.insert((drawing.anchor, drawing.revision));
        drawing.restart_block = first_intersected_block(drawing, previous_frames);
        drawing.pending_restart = None;
      }
    }
    let catalog = self.current.body_drawing_wraps.as_ref()?.clone();
    // Word's fixed page/margin-positioned VML picture wraps preceding text
    // frames, not just the paragraph containing its run (ECMA-376 Part 4
    // 19.3.2.6). Native two-column controls also keep the picture in its
    // original column when that reformat pushes the anchor text into the
    // following column. Retain both the physical reservation and paint
    // origin, then replay from the first intersected source checkpoint.
    // Ordinary documents take no extra layout pass or font measurement;
    // A placement revision can request this local replay once. Migration
    // retires the old band and replays only its affected prefix, retaining
    // the anchor's new physical-page floor to prevent forward/backward loops.
    self.restore_layout_checkpoint(&restart)?;
    self.checkpoints.truncate(checkpoint_index);
    self.current.body_drawing_wraps = Some(catalog);
    install(&mut self.current, self.pages.len(), restart.section_index);
    Some(restart)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use ooxmlsdk::sdk::SdkType;

  fn paragraph(text: String) -> Block {
    let mut block = crate::docx::simple_text_block(
      text,
      TextStyle {
        font_family: Some("Arial".into()),
        font_size_pt: 11.0,
        ..Default::default()
      },
    );
    let Block::Paragraph(paragraph) = &mut block else {
      unreachable!()
    };
    paragraph.format.line_height_rule = LineHeightRule::Exact;
    paragraph.format.line_height_pt = Some(14.0);
    paragraph.format.widow_control = Some(false);
    paragraph.format.spacing_before_pt = 0.0;
    paragraph.format.spacing_after_pt = 0.0;
    block
  }

  fn document(previous_lines: usize, page_height: f32, wrap: &str) -> DocxDocument {
    let mut images = crate::docx::ImageCatalog::default();
    images.by_relationship_id.insert(
      "rId1".into(),
      crate::docx::package::ImageResource {
        data: vec![1, 2, 3].into(),
        content_type: Some("image/png".into()),
      },
    );
    let shape = ooxmlsdk::schemas::schemas_microsoft_com_vml::Shape::from_bytes(format!(
      r##"<v:shape xmlns:v="urn:schemas-microsoft-com:vml"
        xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
        xmlns:w10="urn:schemas-microsoft-com:office:word" type="#_x0000_t75"
        filled="f" stroked="f"
        style="position:absolute;margin-left:0pt;margin-top:0pt;width:451pt;height:80pt;mso-position-horizontal-relative:text;mso-position-vertical-relative:margin">
        <v:imagedata r:id="rId1"/><w10:wrap type="{wrap}"/>
      </v:shape>"##
    ).as_bytes()).unwrap();
    let image = crate::docx::shape_image_with_style_and_shape_types(
      &shape,
      shape.style.as_deref(),
      &images,
      &[],
    )
    .unwrap();
    let mut anchor = paragraph("ANCHOR".into());
    let Block::Paragraph(p) = &mut anchor else {
      unreachable!()
    };
    p.inlines.insert(0, InlineItem::Image(image));
    DocxDocument {
      page: PageSetup {
        width_pt: 595.3,
        height_pt: page_height,
        margin_left_pt: 72.0,
        margin_right_pt: 72.0,
        margin_top_pt: 72.0,
        margin_bottom_pt: 72.0,
        ..Default::default()
      },
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
      sections: Vec::new(),
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
      blocks: vec![
        paragraph("TITLE".into()),
        paragraph(
          (0..previous_lines)
            .map(|i| format!("PREV{i:02}"))
            .collect::<Vec<_>>()
            .join("\n"),
        ),
        anchor,
      ],
    }
  }

  fn text_y(document: &LayoutDocument, label: &str) -> (usize, f32) {
    document
      .pages
      .iter()
      .enumerate()
      .find_map(|(index, page)| {
        page.items.iter().find_map(|item| {
          if let PageItem::Text(text) = item
            && text.text == label
          {
            Some((index, text.y_pt))
          } else {
            None
          }
        })
      })
      .expect("source label retained")
  }

  fn in_two_columns(mut source: DocxDocument, balance: bool) -> DocxDocument {
    source.page.width_pt = 841.85;
    source.sections.push(crate::docx::ImportedSection {
      break_kind: SectionBreakKind::NextPage,
      discarded_carrier_spacing_after_pt: None,
      section_properties: None,
      page: source.page,
      columns: SectionColumns {
        count: 2,
        gap_pt: 35.4,
        unbalanced: true,
        balance_on_wrap_follow: balance,
        ..Default::default()
      },
      title_page: false,
      header_blocks: Vec::new(),
      footer_blocks: Vec::new(),
      first_header_blocks: Vec::new(),
      first_footer_blocks: Vec::new(),
      even_header_blocks: Vec::new(),
      even_footer_blocks: Vec::new(),
      blocks: std::mem::take(&mut source.blocks),
    });
    source
  }

  fn in_continuous_sections(mut source: DocxDocument) -> DocxDocument {
    let mut blocks = std::mem::take(&mut source.blocks);
    let prefix = crate::docx::ImportedSection {
      break_kind: SectionBreakKind::NextPage,
      discarded_carrier_spacing_after_pt: None,
      section_properties: None,
      page: source.page,
      columns: SectionColumns::default(),
      title_page: false,
      header_blocks: Vec::new(),
      footer_blocks: Vec::new(),
      first_header_blocks: Vec::new(),
      first_footer_blocks: Vec::new(),
      even_header_blocks: Vec::new(),
      even_footer_blocks: Vec::new(),
      blocks: vec![blocks.remove(0)],
    };
    let mut following = prefix.clone();
    following.break_kind = SectionBreakKind::Continuous;
    following.blocks = blocks;
    source.sections = vec![prefix, following];
    source
  }

  #[test]
  fn body_authored_column_break_preserves_partition_and_tallest_column_end() {
    for columns in [2, 3] {
      for left_lines in [2, 8, 30] {
        for right_lines in [1, 6] {
          for before in [0.0, 14.0, 28.0] {
            for unbalanced in [false, true] {
              let mut source = document(0, 841.9, "none");
              source.blocks = vec![paragraph("TOP".into())];
              let mut source = in_continuous_sections(source);
              let mut body = source.sections[1].clone();
              body.columns = SectionColumns {
                count: columns,
                unbalanced,
                ..Default::default()
              };
              let left = paragraph(
                (0..left_lines)
                  .map(|i| format!("L{i:02}"))
                  .collect::<Vec<_>>()
                  .join("\n"),
              );
              let mut right = paragraph(
                (0..right_lines)
                  .map(|i| format!("B{i:02}"))
                  .collect::<Vec<_>>()
                  .join("\n"),
              );
              let Block::Paragraph(p) = &mut right else {
                unreachable!()
              };
              p.format.spacing_before_pt = before;
              p.inlines.insert(0, InlineItem::ColumnBreak);
              body.blocks = vec![left, right];
              let mut after = source.sections[1].clone();
              after.blocks = vec![paragraph("AFTER".into())];
              source.sections = vec![source.sections[0].clone(), body, after];
              let result = layout(&source, &LayoutOptions::default()).unwrap();
              assert_eq!(result.pages.len(), 1);
              let (_, first_y) = text_y(&result, "L00");
              assert!((text_y(&result, "B00").1 - first_y - before).abs() < 0.01);
              let expected_end =
                first_y + (left_lines as f32 * 14.0).max(before + right_lines as f32 * 14.0);
              assert!(
                (text_y(&result, "AFTER").1 - expected_end).abs() < 0.01,
                "cols{columns} left{left_lines} right{right_lines} upper{before} unbalanced{unbalanced}"
              );
            }
          }
        }
      }
    }
  }

  #[test]
  fn body_single_line_balanced_section_keeps_its_actual_terminal_cursor() {
    let mut expected = None;
    for unbalanced in [true, false] {
      let mut source = document(0, 841.9, "none");
      source.blocks = vec![paragraph("TOP".into())];
      let mut source = in_continuous_sections(source);
      let mut body = source.sections[1].clone();
      body.columns = SectionColumns {
        count: 2,
        unbalanced,
        ..Default::default()
      };
      body.blocks = vec![paragraph("SINGLE".into())];
      let mut after = source.sections[1].clone();
      after.blocks = vec![paragraph("AFTER".into())];
      source.sections = vec![source.sections[0].clone(), body, after];
      let result = layout(&source, &LayoutOptions::default()).unwrap();
      let single = text_y(&result, "SINGLE");
      let after = text_y(&result, "AFTER");
      assert_eq!(single.0, 0);
      assert!(
        (after.1 - single.1 - 14.0).abs() < 0.01,
        "unbalanced{unbalanced}: single{single:?}, after{after:?}"
      );
      if let Some(expected) = expected {
        assert_eq!(after, expected);
      } else {
        expected = Some(after);
      }
    }
  }

  #[test]
  fn body_leading_column_break_retains_native_compatibility_spacing_and_end() {
    for compatibility_mode in [12, 14, 15] {
      for before in [0.0, 14.0, 28.0] {
        for tail in [false, true] {
          let mut source = document(0, 841.9, "none");
          source.compatibility_mode = compatibility_mode;
          source.blocks = vec![paragraph("TOP".into())];
          let mut source = in_continuous_sections(source);
          let mut body = source.sections[1].clone();
          body.columns.count = 2;
          let mut second = paragraph("SECOND".into());
          let Block::Paragraph(p) = &mut second else {
            unreachable!()
          };
          p.format.spacing_before_pt = before;
          p.inlines.insert(0, InlineItem::ColumnBreak);
          body.blocks = vec![paragraph("FIRST".into()), second];
          if tail {
            body.blocks.push(paragraph("TAIL".into()));
          }
          let mut after = source.sections[1].clone();
          after.blocks = vec![paragraph("AFTER".into())];
          source.sections = vec![source.sections[0].clone(), body, after];
          let result = layout(&source, &LayoutOptions::default()).unwrap();
          let first = text_y(&result, "FIRST").1;
          assert!((text_y(&result, "SECOND").1 - first - before).abs() < 0.01);
          let master_height: f32 = if compatibility_mode >= 15 { 28.0 } else { 14.0 };
          let follow_height = before + if tail { 28.0 } else { 14.0 };
          let expected_end = first + master_height.max(follow_height);
          assert!(
            (text_y(&result, "AFTER").1 - expected_end).abs() < 0.01,
            "mode{compatibility_mode} upper{before} tail{tail}"
          );
        }
      }
    }
  }

  #[test]
  fn body_earlier_page_column_break_does_not_disable_final_page_balance() {
    let mut ends = Vec::new();
    for unbalanced in [false, true] {
      // Match the native 150-line control: its final-page remainder fits
      // within one unbalanced column. A taller line would legitimately push
      // AFTER to another page once a completed column occupies the body.
      let mut source = document(0, 800.0, "none");
      source.page.width_pt = 600.0;
      source.blocks = vec![paragraph("TOP".into())];
      let mut source = in_continuous_sections(source);
      let mut body = source.sections[1].clone();
      body.columns = SectionColumns {
        count: 3,
        gap_pt: 24.0,
        unbalanced,
        ..Default::default()
      };
      let mut following = paragraph(
        (0..150)
          .map(|i| format!("B{i:03}"))
          .collect::<Vec<_>>()
          .join("\n"),
      );
      let Block::Paragraph(p) = &mut following else {
        unreachable!()
      };
      p.inlines.insert(0, InlineItem::ColumnBreak);
      body.blocks = vec![paragraph("FIRST".into()), following];
      let mut after = source.sections[1].clone();
      after.blocks = vec![paragraph("AFTER".into())];
      source.sections = vec![source.sections[0].clone(), body, after];
      for section in &mut source.sections {
        for block in &mut section.blocks {
          let Block::Paragraph(p) = block else {
            unreachable!()
          };
          p.format.line_height_pt = Some(12.0);
          p.base_style.font_family = Some("Times New Roman".into());
          p.base_style.font_size_pt = 7.0;
          for inline in &mut p.inlines {
            if let InlineItem::Text(run) = inline {
              run.style = p.base_style.clone();
            }
          }
        }
      }
      let result = layout(&source, &LayoutOptions::default()).unwrap();
      assert_eq!(result.pages.len(), 2);
      ends.push(text_y(&result, "AFTER"));
    }
    assert_eq!(ends[0].0, 1);
    assert_eq!(ends[1].0, 1);
    assert!(
      ends[0].1 + 100.0 < ends[1].1,
      "only the page containing the authored break keeps its column partition: {ends:?}"
    );
  }

  #[test]
  fn body_unequal_column_follow_restores_the_destination_frame_width() {
    for widths in [[102.0, 50.3], [50.3, 102.0]] {
      for (left, right) in [(0.0, 0.0), (4.0, 6.0), (-8.0, -6.0)] {
        let mut source = document(0, 800.0, "none");
        source.page.width_pt = 571.2;
        source.page.margin_left_pt = 398.9;
        source.page.margin_right_pt = 15.7;
        source.blocks = vec![paragraph("TOP".into())];
        let mut source = in_continuous_sections(source);
        let mut body = source.sections[1].clone();
        body.columns.count = 2;
        body.columns.explicit_count = 2;
        body.columns.explicit_widths_pt[..2].copy_from_slice(&widths);
        body.columns.explicit_gaps_pt[0] = 4.3;
        let mut switch = paragraph("R".into());
        let Block::Paragraph(p) = &mut switch else {
          unreachable!()
        };
        p.format.indent_left_pt = left;
        p.format.indent_right_pt = right;
        p.inlines.insert(0, InlineItem::ColumnBreak);
        let mut green = paragraph("_xxx".into());
        let Block::Paragraph(p) = &mut green else {
          unreachable!()
        };
        p.format.alignment = ParagraphAlignment::Right;
        body.blocks = vec![paragraph("LEFT".into()), switch, green];
        source.sections[1] = body;
        let result = layout(&source, &LayoutOptions::default()).unwrap();
        assert_eq!(result.pages.len(), 1);
        let text = result.pages[0]
          .items
          .iter()
          .find_map(|item| match item {
            PageItem::Text(text) if text.text == "_xxx" => Some(text),
            _ => None,
          })
          .expect("right-aligned text retained");
        let mut metrics = TextMetrics::new();
        let edge = text.x_pt + metrics.measure_text(&text.text, &text.style);
        assert!(
          (edge - 555.5).abs() < 0.01,
          "widths{widths:?} indents{left}/{right}: right edge{edge}"
        );
      }
    }
  }

  #[test]
  fn body_vml_square_replays_terminal_lower_space_without_moving_earlier_lines() {
    // Native one/two/three-line controls: lower spacing participates only in
    // the terminal line's collision. Fill and negative z-order do not disable
    // square wrap; a usable side lane remains available.
    for lines in [1, 2] {
      for after in [0.0, 26.4] {
        for behind in [false, true] {
          for filled in [false, true] {
            for width in [225.0, 451.3] {
              for continuous in [false, true] {
                let make = |wrapped: bool| {
                  let mut source = document(1, 841.9, "none");
                  let xml = format!(
                    r##"<v:shape xmlns:v="urn:schemas-microsoft-com:vml"
                    xmlns:w10="urn:schemas-microsoft-com:office:word"
                    fillcolor="#ff0000" stroked="f"
                    style="position:absolute;margin-left:72pt;margin-top:120pt;width:{width}pt;height:80pt;z-index:{};mso-position-horizontal-relative:page;mso-position-vertical-relative:page">
                    <w10:wrap type="square"/>
                  </v:shape>"##,
                    if behind { -1 } else { 1 },
                  );
                  let xml = crate::docx::v::Shape::from_bytes(xml.as_bytes()).unwrap();
                  let mut shape =
                    crate::docx::vml_shape_shape(&xml, &crate::docx::ImageCatalog::default(), &[])
                      .expect("rectangular fly");
                  if !filled {
                    shape.fill_color = None;
                  }
                  if !wrapped
                    && let crate::docx::ImagePlacement::Floating(placement) = &mut shape.placement
                  {
                    placement.wrap = ImageWrapMode::Through;
                  }
                  let mut previous = paragraph(if lines == 1 {
                    "LAST".into()
                  } else {
                    "FIRST\nLAST".into()
                  });
                  let Block::Paragraph(p) = &mut previous else {
                    unreachable!()
                  };
                  p.format.spacing_before_pt = 3.0;
                  p.format.spacing_after_pt = after;
                  let mut anchor = paragraph("ANCHOR".into());
                  let Block::Paragraph(p) = &mut anchor else {
                    unreachable!()
                  };
                  p.inlines.insert(0, InlineItem::Shape(shape));
                  source.blocks = vec![paragraph("TITLE".into()), previous, anchor];
                  if continuous {
                    in_continuous_sections(source)
                  } else {
                    source
                  }
                };
                let ordinary = layout(&make(false), &LayoutOptions::default()).unwrap();
                let wrapped = layout(&make(true), &LayoutOptions::default()).unwrap();
                assert_eq!(wrapped.pages.len(), ordinary.pages.len());
                assert_eq!(text_y(&wrapped, "TITLE"), text_y(&ordinary, "TITLE"));
                if continuous {
                  assert_eq!(wrapped.pages[0].section_index, 0);
                  assert!(wrapped.frames.iter().all(|frame| {
                    frame.section_index == wrapped.pages[frame.page_index].section_index
                  }));
                }
                if lines == 2 {
                  assert_eq!(text_y(&wrapped, "FIRST"), text_y(&ordinary, "FIRST"));
                }
                let before = text_y(&ordinary, "LAST");
                let actual = text_y(&wrapped, "LAST");
                if after > 0.0 && width > 400.0 {
                  assert!(
                    actual.1 > before.1 + 80.0,
                    "lines{lines} after{after} behind{behind} filled{filled} width{width}: {before:?} -> {actual:?}"
                  );
                } else {
                  assert_eq!(actual, before);
                }
                if width > 400.0 && (lines == 2 || after > 0.0) {
                  assert!(
                    text_y(&wrapped, "ANCHOR").1 > text_y(&ordinary, "ANCHOR").1 + 50.0,
                    "intersecting anchor: lines{lines} after{after} behind{behind} filled{filled} continuous{continuous}: {:?} -> {:?}",
                    text_y(&ordinary, "ANCHOR"),
                    text_y(&wrapped, "ANCHOR")
                  );
                } else if width > 400.0 {
                  // One preceding line with no lower space leaves this short
                  // anchor line entirely above the fly's top at120pt.
                  assert_eq!(text_y(&wrapped, "ANCHOR"), text_y(&ordinary, "ANCHOR"));
                }
              }
            }
          }
        }
      }
    }
  }

  #[test]
  fn body_vml_square_upper_space_preserves_an_available_side_lane() {
    // Native textbox/rectangle x partial/full x0/12/24pt upper controls.
    // An interior VML square keeps a usable side lane; a full-width fly
    // clears the line and retains that paragraph's upper space below it.
    for before in [0.0, 12.0, 24.0] {
      for width in [225.0, 451.3] {
        let make = |wrapped: bool, upper: f32| {
          let mut source = document(0, 480.0, "none");
          let xml = format!(
            r##"<v:shape xmlns:v="urn:schemas-microsoft-com:vml"
              xmlns:w10="urn:schemas-microsoft-com:office:word"
              fillcolor="#ff0000" stroked="f"
              style="position:absolute;margin-left:0pt;margin-top:0pt;width:{width}pt;height:80pt;mso-position-horizontal:center;mso-position-horizontal-relative:margin;mso-position-vertical:top;mso-position-vertical-relative:margin">
              <w10:wrap type="square" anchorx="margin" anchory="margin"/>
            </v:shape>"##
          );
          let xml = crate::docx::v::Shape::from_bytes(xml.as_bytes()).unwrap();
          let mut shape =
            crate::docx::vml_shape_shape(&xml, &crate::docx::ImageCatalog::default(), &[]).unwrap();
          if !wrapped && let crate::docx::ImagePlacement::Floating(placement) = &mut shape.placement
          {
            placement.wrap = ImageWrapMode::Through;
          }
          let mut anchor = paragraph("ANCHOR".into());
          let Block::Paragraph(p) = &mut anchor else {
            unreachable!()
          };
          p.format.spacing_before_pt = upper;
          p.inlines.insert(0, InlineItem::Shape(shape));
          source.blocks = vec![anchor];
          source
        };
        let baseline = layout(&make(false, 0.0), &LayoutOptions::default()).unwrap();
        let ordinary = layout(&make(false, before), &LayoutOptions::default()).unwrap();
        let wrapped = layout(&make(true, before), &LayoutOptions::default()).unwrap();
        let actual = text_y(&wrapped, "ANCHOR");
        let original = text_y(&ordinary, "ANCHOR");
        assert!((original.1 - text_y(&baseline, "ANCHOR").1 - before).abs() < 0.1);
        if width < 400.0 {
          assert_eq!(actual, original, "partial VML square before{before}");
        } else {
          assert!(
            (actual.1 - original.1 - 80.0).abs() < 0.1,
            "full VML square before{before}: {original:?} -> {actual:?}"
          );
        }
      }
    }
  }

  #[test]
  fn body_expired_square_wrap_does_not_disable_widow_control() {
    let mut ordinary = document(0, 228.0, "none");
    let Block::Paragraph(mut anchor) = ordinary.blocks.pop().unwrap() else {
      unreachable!()
    };
    let InlineItem::Image(mut image) = anchor.inlines.remove(0) else {
      unreachable!()
    };
    image.height_pt = 20.0;
    let crate::docx::ImagePlacement::Floating(placement) = &mut image.placement else {
      unreachable!()
    };
    placement.wrap = ImageWrapMode::Square;
    placement.vertical_relative_to = VerticalImageReference::Page;
    placement.vertical_offset_pt = 0.0;
    let mut body = paragraph("LINE0\nLINE1\nLINE2\nLINE3\nLINE4".into());
    let Block::Paragraph(p) = &mut body else {
      unreachable!()
    };
    p.format.widow_control = Some(true);
    ordinary.blocks = vec![paragraph("PREFIX0\nPREFIX1".into()), body];
    let mut wrapped = ordinary.clone();
    let Block::Paragraph(prefix) = &mut wrapped.blocks[0] else {
      unreachable!()
    };
    prefix.inlines.insert(0, InlineItem::Image(image));
    let ordinary = layout(&ordinary, &LayoutOptions::default()).unwrap();
    let wrapped = layout(&wrapped, &LayoutOptions::default()).unwrap();
    assert_eq!(wrapped.pages.len(), 2);
    let counts = wrapped
      .pages
      .iter()
      .map(|page| {
        page
          .items
          .iter()
          .filter(|item| matches!(item, PageItem::Text(text) if text.text.starts_with("LINE")))
          .count()
      })
      .collect::<Vec<_>>();
    assert_eq!(counts, [3, 2]);
    for line in ["LINE0", "LINE1", "LINE2", "LINE3", "LINE4"] {
      assert_eq!(text_y(&wrapped, line), text_y(&ordinary, line));
    }
  }

  #[test]
  fn body_replay_preserves_continuous_column_frame_and_balancing() {
    for unbalanced in [false, true] {
      for restart_section in [0, 1] {
        let mut source = document(0, 480.0, "none");
        let mut right = paragraph("RIGHT0\nRIGHT1\nRIGHT2".into());
        let Block::Paragraph(p) = &mut right else {
          unreachable!()
        };
        p.inlines.insert(0, InlineItem::ColumnBreak);
        source.blocks = vec![
          paragraph("PREFIX0\nPREFIX1\nPREFIX2\nPREFIX3".into()),
          paragraph("LEFT0\nLEFT1\nLEFT2".into()),
          right,
        ];
        let mut source = in_continuous_sections(source);
        source.sections[1].columns = SectionColumns {
          count: 2,
          unbalanced,
          ..Default::default()
        };
        let mut tail = source.sections[0].clone();
        tail.break_kind = SectionBreakKind::Continuous;
        tail.blocks = vec![paragraph("TAIL".into())];
        source.sections.push(tail);
        let mut next = source.sections[0].clone();
        next.blocks = vec![paragraph("NEXT".into())];
        source.sections.push(next);
        let positions = |root: &RootFrameLayout<'_>| {
          root
            .pages
            .iter()
            .chain(std::iter::once(&root.current))
            .enumerate()
            .flat_map(|(page, p)| {
              p.items.iter().filter_map(move |item| {
                if let PageItem::Text(text) = item {
                  Some((page, text.text.clone(), text.x_pt, text.y_pt))
                } else {
                  None
                }
              })
            })
            .collect::<Vec<_>>()
        };
        let mut root = RootFrameLayout::new(&source, &LayoutOptions::default());
        root.format_body_frames();
        let original = positions(&root);
        let left = original
          .iter()
          .find(|(_, text, _, _)| text == "LEFT0")
          .unwrap();
        let right = original
          .iter()
          .find(|(_, text, _, _)| text == "RIGHT0")
          .unwrap();
        assert_eq!((left.0, left.3), (right.0, right.3));
        let checkpoint_index = root
          .checkpoints
          .iter()
          .position(|checkpoint| {
            checkpoint.section_index == restart_section && checkpoint.block_index == 0
          })
          .unwrap();
        let checkpoint = root.checkpoints[checkpoint_index];
        root.restore_layout_checkpoint(&checkpoint).unwrap();
        root.checkpoints.truncate(checkpoint_index + 1);
        root.format_body_from_checkpoint(&checkpoint, &[]);
        assert_eq!(
          positions(&root),
          original,
          "unbalanced{unbalanced} restart{restart_section}"
        );
      }
    }
  }

  #[test]
  fn body_vml_top_bottom_reformats_preceding_paragraphs() {
    // Native B00/B01 move TITLE and every preceding line by the picture's
    // height; a Through picture at the same position leaves them unchanged.
    let ordinary = layout(&document(3, 841.9, "none"), &LayoutOptions::default()).unwrap();
    let wrapped = layout(
      &document(3, 841.9, "topAndBottom"),
      &LayoutOptions::default(),
    )
    .unwrap();
    assert_eq!(wrapped.pages.len(), 1);
    for label in ["TITLE", "PREV00", "PREV01", "PREV02", "ANCHOR"] {
      let before = text_y(&ordinary, label);
      let after = text_y(&wrapped, label);
      assert_eq!(before.0, after.0);
      assert!(
        (after.1 - before.1 - 80.0).abs() < 0.1,
        "{label}: {before:?} -> {after:?}"
      );
    }
  }

  #[test]
  fn body_vml_top_bottom_reservation_reaches_a_previous_paragraph_follow() {
    // The predecessor starts on the old page, with one line following onto
    // the picture's page. Its checkpoint must carry the reservation across
    // page creation without moving the old-page prefix or duplicating text.
    let ordinary = layout(&document(10, 284.0, "none"), &LayoutOptions::default()).unwrap();
    let wrapped = layout(
      &document(10, 284.0, "topAndBottom"),
      &LayoutOptions::default(),
    )
    .unwrap();
    assert_eq!(wrapped.pages.len(), 2);
    assert_eq!(text_y(&ordinary, "TITLE"), text_y(&wrapped, "TITLE"));
    assert_eq!(text_y(&ordinary, "PREV08"), text_y(&wrapped, "PREV08"));
    assert_eq!(text_y(&wrapped, "PREV09").0, 1);
    assert!((text_y(&wrapped, "PREV09").1 - text_y(&ordinary, "PREV09").1 - 80.0).abs() < 0.1);
    assert_eq!(text_y(&wrapped, "ANCHOR").0, 1);
    assert!(text_y(&wrapped, "ANCHOR").1 > text_y(&wrapped, "PREV09").1);
  }

  #[test]
  fn body_top_bottom_picture_migrates_with_its_displaced_anchor() {
    // Native rectangle/line control: fourteen 14pt predecessor lines fit in
    // two 140pt columns, but the later 80pt picture displaces its anchor to
    // page2. Its former page must release that wrap band without pulling the
    // anchor back. A nonwrapping picture and its anchor both stay on page1.
    fn has_picture(items: &[PageItem]) -> bool {
      items.iter().any(|item| match item {
        PageItem::Image(_) => true,
        PageItem::Group(items)
        | PageItem::InlineObjectGroup(items)
        | PageItem::IndependentTextFrame(items)
        | PageItem::CompositingGroup { items, .. }
        | PageItem::FloatingDrawing { items, .. } => has_picture(items),
        _ => false,
      })
    }
    for drawingml in [false, true] {
      for (wrapped, balance, explicit_break, previous_lines, anchor_height) in [
        (false, true, false, 14, 14.0),
        (true, true, false, 14, 14.0),
        (true, false, false, 14, 14.0),
        (true, true, true, 14, 14.0),
        (true, true, false, 14, 48.0),
        (true, true, false, 17, 48.0),
      ] {
        let mut source = document(
          previous_lines,
          284.0,
          if wrapped { "topAndBottom" } else { "none" },
        );
        source.page.width_pt = 841.85;
        let Block::Paragraph(anchor) = &mut source.blocks[2] else {
          unreachable!()
        };
        anchor.format.page_break_before = explicit_break;
        anchor.format.line_height_pt = Some(anchor_height);
        let InlineItem::Image(image) = &mut anchor.inlines[0] else {
          unreachable!()
        };
        image.width_pt = 174.84;
        if let crate::docx::ImagePlacement::Floating(placement) = &mut image.placement {
          placement.horizontal_relative_to = HorizontalImageReference::Column;
          if drawingml {
            placement.paint_order = FloatingPaintOrder::drawing_ml_relative_height(1);
          }
        }
        let mut buffer = paragraph("END BUFFER".into());
        let Block::Paragraph(p) = &mut buffer else {
          unreachable!()
        };
        p.format.page_break_before = true;
        source.blocks.push(buffer);
        let source = in_two_columns(source, balance);
        let result = layout(&source, &LayoutOptions::default()).unwrap();
        let expected_page = usize::from(wrapped || explicit_break);
        assert_eq!(text_y(&result, "ANCHOR").0, expected_page);
        let picture_pages = result
          .pages
          .iter()
          .enumerate()
          .filter_map(|(index, page)| has_picture(&page.items).then_some(index))
          .collect::<Vec<_>>();
        assert_eq!(picture_pages, [expected_page]);
        for index in 0..previous_lines {
          assert_eq!(text_y(&result, &format!("PREV{index:02}")).0, 0);
        }
        let previous7_x = result.pages[0]
          .items
          .iter()
          .find_map(|item| match item {
            PageItem::Text(text) if text.text == "PREV07" => Some(text.x_pt),
            _ => None,
          })
          .unwrap();
        // Native default wrap follow balances7+7 predecessor lines. Both
        // noColumnBalance and pageBreakBefore retain the original9+5 split.
        assert_eq!(
          previous7_x > 400.0,
          wrapped && balance && !explicit_break && previous_lines == 14
        );
      }
    }
  }

  #[test]
  fn body_paragraph_picture_follows_visible_and_empty_anchor_lines() {
    // Native eight-control matrix:0/4pt intersects the initial14pt anchor
    // line, so picture+mark follow together and the old page balances. At
    //20pt the mark stays before the picture; None keeps the ordinary flow.
    fn pictures(items: &[PageItem], result: &mut Vec<(f32, f32)>) {
      for item in items {
        match item {
          PageItem::Image(image) => result.push((image.x_pt, image.y_pt)),
          PageItem::Group(items)
          | PageItem::InlineObjectGroup(items)
          | PageItem::IndependentTextFrame(items)
          | PageItem::CompositingGroup { items, .. }
          | PageItem::FloatingDrawing { items, .. } => pictures(items, result),
          _ => {}
        }
      }
    }
    for (offset, empty, wrapped, balance) in [
      (0.0, false, true, true),
      (0.0, true, true, true),
      (4.0, false, true, true),
      (4.0, true, true, true),
      (20.0, false, true, true),
      (20.0, true, true, true),
      (4.0, true, false, true),
      (4.0, true, true, false),
    ] {
      let mut source = document(14, 284.0, if wrapped { "topAndBottom" } else { "none" });
      let Block::Paragraph(anchor) = &mut source.blocks[2] else {
        unreachable!()
      };
      if empty {
        anchor
          .inlines
          .retain(|inline| matches!(inline, InlineItem::Image(_)));
      }
      let InlineItem::Image(image) = &mut anchor.inlines[0] else {
        unreachable!()
      };
      image.width_pt = 174.84;
      let crate::docx::ImagePlacement::Floating(placement) = &mut image.placement else {
        unreachable!()
      };
      placement.paint_order = FloatingPaintOrder::drawing_ml_relative_height(1);
      placement.horizontal_relative_to = HorizontalImageReference::Column;
      placement.vertical_relative_to = VerticalImageReference::Paragraph;
      placement.vertical_offset_pt = offset;
      source.blocks.push(paragraph("FOLLOW".into()));
      let mut buffer = paragraph("END BUFFER".into());
      let Block::Paragraph(p) = &mut buffer else {
        unreachable!()
      };
      p.format.page_break_before = true;
      source.blocks.push(buffer);
      let result = layout(&in_two_columns(source, balance), &LayoutOptions::default()).unwrap();
      let migrated = wrapped && offset < 14.0;
      let mut actual = Vec::new();
      for (page, output) in result.pages.iter().enumerate() {
        let mut positions = Vec::new();
        pictures(&output.items, &mut positions);
        actual.extend(positions.into_iter().map(|(x, y)| (page, x, y)));
      }
      assert_eq!(actual.len(), 1, "offset{offset} empty{empty} wrap{wrapped}");
      assert_eq!(actual[0].0, usize::from(migrated));
      assert_eq!(actual[0].1 > 400.0, !migrated);
      assert!((actual[0].2 - (if migrated { 72.0 } else { 142.0 } + offset)).abs() < 0.1);
      let previous7_x = result.pages[0]
        .items
        .iter()
        .find_map(|item| match item {
          PageItem::Text(text) if text.text == "PREV07" => Some(text.x_pt),
          _ => None,
        })
        .unwrap();
      assert_eq!(previous7_x > 400.0, migrated && balance);
      for index in 0..14 {
        assert_eq!(text_y(&result, &format!("PREV{index:02}")).0, 0);
      }
      if !empty {
        assert_eq!(text_y(&result, "ANCHOR").0, usize::from(migrated));
      }
      let follow = text_y(&result, "FOLLOW");
      assert_eq!(follow.0, usize::from(wrapped));
      let expected_advance = if migrated {
        94.0 + offset
      } else if wrapped {
        0.0
      } else {
        84.0
      };
      assert!((follow.1 - text_y(&result, "TITLE").1 - expected_advance).abs() < 0.1);
    }
  }

  #[test]
  fn body_negative_paragraph_picture_reflows_previous_lines_without_drifting() {
    fn picture_y(items: &[PageItem]) -> Option<f32> {
      items.iter().find_map(|item| match item {
        PageItem::Image(image) => Some(image.y_pt),
        PageItem::Group(items)
        | PageItem::InlineObjectGroup(items)
        | PageItem::IndependentTextFrame(items)
        | PageItem::CompositingGroup { items, .. }
        | PageItem::FloatingDrawing { items, .. } => picture_y(items),
        _ => None,
      })
    }
    let make = |offset, wrapped| {
      let mut source = document(2, 792.0, if wrapped { "topAndBottom" } else { "none" });
      let Block::Paragraph(anchor) = &mut source.blocks[2] else {
        unreachable!()
      };
      anchor
        .inlines
        .retain(|inline| matches!(inline, InlineItem::Image(_)));
      let InlineItem::Image(image) = &mut anchor.inlines[0] else {
        unreachable!()
      };
      image.width_pt = 180.0;
      image.height_pt = 60.0;
      let crate::docx::ImagePlacement::Floating(placement) = &mut image.placement else {
        unreachable!()
      };
      placement.paint_order = FloatingPaintOrder::drawing_ml_relative_height(1);
      placement.horizontal_relative_to = HorizontalImageReference::Column;
      placement.vertical_relative_to = VerticalImageReference::Paragraph;
      placement.vertical_offset_pt = offset;
      source.blocks.push(paragraph("FOLLOW".into()));
      in_two_columns(source, true)
    };
    let options = LayoutOptions::default();
    let ordinary = layout(&make(0.0, false), &options).unwrap();
    // Native ten-control matrix: the three previous lines and the empty
    // anchor own separate marks. Negative offsets wrap exactly the earlier
    // intersected lines, while the picture retains its original114+offset.
    for (offset, title, previous0, previous1, follow) in [
      (-40.0, 62.0, 62.0, 62.0, 62.0),
      (-28.0, 0.0, 60.0, 60.0, 60.0),
      (-20.0, 0.0, 68.0, 68.0, 68.0),
      (-1.0, 0.0, 0.0, 73.0, 73.0),
      (0.0, 0.0, 0.0, 0.0, 60.0),
      (1.0, 0.0, 0.0, 0.0, 61.0),
      (20.0, 0.0, 0.0, 0.0, 66.0),
      (40.0, 0.0, 0.0, 0.0, 0.0),
    ] {
      for empty_predecessor in [false, true] {
        let mut source = make(offset, true);
        if empty_predecessor {
          let blocks = &mut source.sections[0].blocks;
          blocks[1] = paragraph("PREV00".into());
          blocks.insert(2, paragraph(String::new()));
        }
        let result = layout(&source, &options).unwrap();
        assert_eq!(result.pages.len(), 1);
        assert!((picture_y(&result.pages[0].items).unwrap() - (114.0 + offset)).abs() < 0.1);
        for (label, expected) in [
          ("TITLE", title),
          ("PREV00", previous0),
          ("PREV01", previous1),
          ("FOLLOW", follow),
        ] {
          if empty_predecessor && label == "PREV01" {
            continue;
          }
          let delta = text_y(&result, label).1 - text_y(&ordinary, label).1;
          assert!(
            (delta - expected).abs() < 0.1,
            "offset{offset} empty{empty_predecessor} {label}: {delta} != {expected}"
          );
        }
      }
    }
  }
}
