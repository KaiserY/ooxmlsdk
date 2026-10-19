use super::*;

#[derive(Clone, Copy, Debug)]
pub(super) struct Plan {
  section: usize,
  page: usize,
  end_block: usize,
  first_block: usize,
  top_pt: f32,
  height_pt: f32,
  available_height_pt: f32,
  settled: bool,
}

pub(in super::super) fn constrain_flow(page: &Page, page_index: usize, flow: &mut FlowContext) {
  if flow.text_segmentation != TextSegmentation::Body || flow.inside_paragraph_frame {
    return;
  }
  if let Some(plan) = page.body_drawing_wraps.as_deref().and_then(|catalog| {
    catalog
      .balances
      .iter()
      .find(|plan| plan.section == flow.section_index && plan.page == page_index)
  }) {
    flow.content_bottom = flow.content_bottom.min(plan.top_pt + plan.height_pt);
  }
}

fn minimum_trial_growth(frames: &[LayoutFrame], plan: Plan, flow: FlowContext) -> f32 {
  let geometry = column_geometry(flow.setup, flow.columns, plan.page);
  let mut first_lines = vec![None; geometry.widths.len()];
  for frame in frames.iter().filter(|frame| {
    frame.page_index == plan.page
      && frame.section_index == plan.section
      && frame.kind == FollowFrameKind::Paragraph
      && frame
        .block_index
        .is_some_and(|index| index >= plan.first_block && index < plan.end_block)
  }) {
    for fragment in &frame.fragments {
      if fragment.kind != FrameFragmentKind::ParagraphLine {
        continue;
      }
      let Some(bounds) = fragment.bounds else {
        continue;
      };
      let column = geometry
        .lefts
        .partition_point(|left| *left <= bounds.x_pt + LAYOUT_EPSILON_PT)
        .saturating_sub(1);
      if column == 0 || first_lines[column].is_some() {
        continue;
      }
      let height = fragment.content_advance_pt.unwrap_or(bounds.height_pt);
      if height.is_finite() && height > LAYOUT_EPSILON_PT {
        first_lines[column] = Some(height);
      }
    }
  }
  first_lines
    .into_iter()
    .flatten()
    .reduce(f32::min)
    .unwrap_or(0.0)
}

impl RootFrameLayout<'_> {
  pub(super) fn balance_before_drawing_follow(
    &mut self,
    block_index: usize,
    flow: FlowContext,
    minimum_page: usize,
  ) -> Option<LayoutCheckpoint> {
    if flow.columns.count <= 1
      || !flow.columns.balance_on_wrap_follow
      || self.current.footnote_reserved_height_pt > LAYOUT_EPSILON_PT
    {
      return None;
    }
    let catalog = self.current.body_drawing_wraps.as_ref()?;
    let existing = catalog
      .balances
      .iter()
      .position(|plan| plan.section == flow.section_index && plan.end_block == block_index);
    let plan = if let Some(index) = existing {
      let mut plan = catalog.balances[index];
      if plan.settled {
        return None;
      }
      // The previous block can advance the cursor for this anchor after its
      // own final line was already committed. Judge the trial by that source
      // prefix's actual frames, not by the next paragraph's admission page.
      let last_content_page = self
        .frames
        .iter()
        .rev()
        .find(|frame| {
          frame.section_index == plan.section
            && frame.kind != FollowFrameKind::Notes
            && frame
              .block_index
              .is_some_and(|index| index >= plan.first_block && index < block_index)
        })
        .map_or(self.pages.len(), |frame| frame.page_index);
      if last_content_page <= plan.page
        || plan.height_pt >= plan.available_height_pt - LAYOUT_EPSILON_PT
      {
        Arc::make_mut(self.current.body_drawing_wraps.as_mut()?).balances[index].settled = true;
        return None;
      }
      // A fixed object or an indivisible prefix can defeat the average.
      // Keep the known full-height upper bound, but grow by at least the
      // first line of the following columns (Writer lcl_CalcMinColDiff).
      // Native mixed Exact-line controls distinguish this measured minimum
      // from an arbitrary14pt grid and from an unrestricted half interval.
      let remaining_height = plan.available_height_pt - plan.height_pt;
      let growth = (remaining_height / 2.0)
        .max(minimum_trial_growth(&self.frames, plan, flow))
        .min(remaining_height);
      let next_height = plan.height_pt + growth;
      plan.height_pt = if next_height > plan.height_pt {
        next_height
      } else {
        plan.available_height_pt
      };
      plan
    } else {
      if self.pages.len() >= minimum_page || flow.column_index == 0 {
        return None;
      }
      let first_block = self
        .frames
        .iter()
        .filter(|frame| {
          frame.page_index == self.pages.len() && frame.section_index == flow.section_index
        })
        .filter_map(|frame| frame.block_index)
        .min()?;
      let available_height_pt = flow.content_bottom - flow.content_top_pt;
      let content_height = balanced_column_content_height(
        flow.content_top_pt,
        self.y,
        flow.columns.completed_content_height_pt,
      );
      let height_pt = (content_height / flow.columns.count as f32).min(available_height_pt);
      if height_pt >= available_height_pt - LAYOUT_EPSILON_PT {
        return None;
      }
      Plan {
        section: flow.section_index,
        page: self.pages.len(),
        end_block: block_index,
        first_block,
        top_pt: flow.content_top_pt,
        height_pt,
        available_height_pt,
        settled: false,
      }
    };
    let (checkpoint_index, restart) = self
      .checkpoints
      .iter()
      .enumerate()
      .rev()
      .find(|(_, c)| c.section_index == plan.section && c.block_index == plan.first_block)?;
    let restart = *restart;
    let catalog = Arc::make_mut(self.current.body_drawing_wraps.as_mut()?);
    if let Some(index) = existing {
      catalog.balances[index] = plan;
    } else {
      catalog.balances.push(plan);
    }
    let catalog = self.current.body_drawing_wraps.as_ref()?.clone();
    // Native noColumnBalance controls distinguish this natural wrap follow
    // from an authored pageBreakBefore: only the former balances the partial
    // previous page. Preserve the migrated anchor's floor and all fixed
    // drawing origins while replaying this page's source prefix.
    self.restore_layout_checkpoint(&restart)?;
    self.checkpoints.truncate(checkpoint_index);
    self.current.body_drawing_wraps = Some(catalog);
    install(&mut self.current, self.pages.len(), restart.section_index);
    Some(restart)
  }
}
