//! Formatting of paragraphs and table rows associated with webSettings divisions.
use std::collections::HashMap;

use super::{BodySectionChoice, on_off_only_value, signed_twips_measure_to_points, w};

#[derive(Clone, Debug, Default)]
pub(super) struct Divisions {
  ids: HashMap<i64, usize>,
  entries: Vec<Division>,
}

#[derive(Clone, Debug)]
struct Division {
  parent: Option<usize>,
  left_pt: f32,
  right_pt: f32,
  cell_vertical: Option<(f32, f32)>,
  paragraph_table_depth: usize,
}

impl Divisions {
  pub(super) fn from_settings(settings: &w::WebSettings) -> Self {
    let mut divisions = Self::default();
    if let Some(root) = &settings.divs {
      divisions.collect(&root.div, None);
    }
    divisions
  }

  fn collect(&mut self, divs: &[w::Div], parent: Option<usize>) {
    for div in divs {
      let mut enclosing = parent;
      if let Ok(id) = div.id.parse::<i64>()
        && id != 0
      {
        let index = self.entries.len();
        self.ids.insert(id, index);
        let body = div
          .body_div
          .as_ref()
          .is_some_and(|flag| on_off_only_value(flag.val));
        self.entries.push(Division {
          parent,
          left_pt: signed_twips_measure_to_points(&div.left_margin_div.val).unwrap_or_default(),
          right_pt: signed_twips_measure_to_points(&div.right_margin_div.val).unwrap_or_default(),
          cell_vertical: (!body).then(|| {
            (
              signed_twips_measure_to_points(&div.top_margin_div.val).unwrap_or_default(),
              signed_twips_measure_to_points(&div.bottom_margin_div.val).unwrap_or_default(),
            )
          }),
          paragraph_table_depth: 0,
        });
        enclosing = Some(index);
      }
      for child in &div.divs_child {
        self.collect(&child.div, enclosing);
      }
    }
  }

  pub(super) fn bind_body(&mut self, body: &w::Body) {
    if self.entries.is_empty() {
      return;
    }
    // A div used inside a cell supplies paragraph margins there. Word then
    // omits that div from row positioning even in an earlier, separate table.
    for choice in &body.body_choice {
      BodySectionChoice::from(choice).visit_paragraphs(0, &mut |p, depth| {
        if depth > 0
          && let Some(index) = self.index(
            p.paragraph_properties
              .as_ref()
              .and_then(|pr| pr.div_id.as_ref()),
          )
        {
          let div = &mut self.entries[index];
          div.paragraph_table_depth = div.paragraph_table_depth.max(depth);
        }
      });
    }
  }

  fn index(&self, id: Option<&w::DivId>) -> Option<usize> {
    self.ids.get(&id?.val.parse::<i64>().ok()?).copied()
  }

  pub(super) fn cell_vertical(&self, id: i64) -> Option<(f32, f32)> {
    self.entries[*self.ids.get(&id)?].cell_vertical
  }

  pub(super) fn paragraph_margins(&self, id: Option<&w::DivId>, depth: usize) -> (f32, f32) {
    self.margins(id, |div, direct| {
      direct || div.paragraph_table_depth >= depth
    })
  }

  pub(super) fn row_margins(&self, row: &w::TableRow, depth: usize) -> (f32, f32) {
    if self.entries.is_empty() {
      return (0.0, 0.0);
    }
    let id = row.table_row_properties.as_ref().and_then(|pr| {
      pr.table_row_properties_choice1
        .iter()
        .find_map(|choice| match choice {
          w::TableRowPropertiesChoice::DivId(id) => Some(id),
          _ => None,
        })
    });
    self.margins(id, |div, direct| {
      div.paragraph_table_depth < depth
        && (direct || div.paragraph_table_depth == depth.saturating_sub(1))
    })
  }

  fn margins(
    &self,
    id: Option<&w::DivId>,
    include: impl Fn(&Division, bool) -> bool,
  ) -> (f32, f32) {
    let first = self.index(id);
    let mut current = first;
    let mut margins = (0.0, 0.0);
    while let Some(index) = current {
      let div = &self.entries[index];
      if include(div, current == first) {
        margins.0 += div.left_pt;
        margins.1 += div.right_pt;
      }
      current = div.parent;
    }
    margins
  }
}
