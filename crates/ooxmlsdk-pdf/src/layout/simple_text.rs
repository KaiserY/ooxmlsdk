use super::*;

pub(crate) fn text_pages(pages: Vec<(PageSetup, Vec<String>)>) -> LayoutDocument {
  let mut output_pages = Vec::new();
  for (setup, lines) in pages {
    let mut page = empty_page(setup, 0);
    let mut y = setup.margin_top_pt;
    let content_bottom = setup.height_pt - setup.margin_bottom_pt;
    for line in lines {
      if y + DEFAULT_LINE_HEIGHT_PT > content_bottom {
        output_pages.push(page);
        page = empty_page(setup, 0);
        y = setup.margin_top_pt;
      }
      if !line.is_empty() {
        page.items.push(PageItem::Text(TextItem {
          x_pt: setup.margin_left_pt,
          y_pt: y,
          line_height_pt: DEFAULT_LINE_HEIGHT_PT,
          text: line,
          style: TextStyle::default(),
          hyperlink_url: None,
          dynamic_field: None,
          style_ref_keys: Vec::new(),
          style_ref_text: None,
          preserve_text_portion: false,
          form_widget_id: None,
          paragraph_bidi: false,
          decoration_span_start_x_pt: None,
          pdf_text_segmentation: PdfTextSegmentation::Line,
        }));
      }
      y += DEFAULT_LINE_HEIGHT_PT;
    }
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
