use super::*;

pub(crate) fn item_pages(pages: Vec<(PageSetup, Vec<PageItem>)>) -> LayoutDocument {
  let mut output_pages = Vec::new();
  for (setup, items) in pages {
    let mut page = empty_page(setup, 0);
    page.items = items;
    output_pages.push(page);
  }

  if output_pages.is_empty() {
    output_pages.push(empty_page(PageSetup::default(), 0));
  }

  LayoutDocument {
    pages: output_pages,
    form_widgets: Vec::new(),
    follows: Vec::new(),
    frames: Vec::new(),
    outline_entries: Vec::new(),
    page_replays: Vec::new(),
    page_replay_applications: Vec::new(),
    backward_moves: Vec::new(),
    layout_reruns: Vec::new(),
    page_invalidations: Vec::new(),
    reflow_executions: Vec::new(),
    reflow_requests: Vec::new(),
    restart_plan: None,
  }
}
