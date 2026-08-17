use lopdf::{Dictionary, Document, Object};

use crate::error::{PdfError, Result};
use crate::options::{PdfOptions, PdfViewerMagnification, PdfViewerPageMode};

pub(super) fn apply_viewer_preferences(pdf: Vec<u8>, options: &PdfOptions) -> Result<Vec<u8>> {
  let needs_open_action = options.viewer.initial_page != 1
    || !matches!(
      options.viewer.magnification,
      PdfViewerMagnification::Default
    );
  let needs_catalog_page_mode =
    options.viewer.full_screen || !matches!(options.viewer.page_mode, PdfViewerPageMode::Default);
  let display_document_title = options
    .metadata
    .title
    .as_deref()
    .is_some_and(|title| !title.is_empty() && options.viewer.display_document_title);
  let needs_preferences = options.viewer.hide_toolbar
    || options.viewer.hide_menubar
    || options.viewer.hide_window_controls
    || options.viewer.fit_window
    || options.viewer.center_window
    || options.viewer.first_page_left
    || options.viewer.full_screen
    || display_document_title;

  if !needs_open_action && !needs_catalog_page_mode && !needs_preferences {
    return Ok(pdf);
  }

  let mut document =
    Document::load_mem(&pdf).map_err(|error| PdfError::Lopdf(format!("{error}")))?;
  let catalog_id = document
    .trailer
    .get(b"Root")
    .and_then(Object::as_reference)
    .map_err(|error| PdfError::Lopdf(format!("missing PDF catalog: {error}")))?;

  let mut preferences = existing_viewer_preferences(&document, catalog_id)?;
  set_true(&mut preferences, "HideToolbar", options.viewer.hide_toolbar);
  set_true(&mut preferences, "HideMenubar", options.viewer.hide_menubar);
  set_true(
    &mut preferences,
    "HideWindowUI",
    options.viewer.hide_window_controls,
  );
  set_true(&mut preferences, "FitWindow", options.viewer.fit_window);
  set_true(
    &mut preferences,
    "CenterWindow",
    options.viewer.center_window,
  );
  if display_document_title {
    preferences.set("DisplayDocTitle", Object::Boolean(true));
  }
  if options.viewer.first_page_left {
    preferences.set("Direction", Object::Name(b"R2L".to_vec()));
  }
  if options.viewer.full_screen {
    preferences.set(
      "NonFullScreenPageMode",
      Object::Name(page_mode_name(options.viewer.page_mode).to_vec()),
    );
  }

  let open_action = if needs_open_action {
    Some(open_action(&document, options)?)
  } else {
    None
  };
  let page_mode = needs_catalog_page_mode.then(|| {
    if options.viewer.full_screen {
      b"FullScreen".as_slice()
    } else {
      page_mode_name(options.viewer.page_mode)
    }
  });

  let catalog = document
    .get_object_mut(catalog_id)
    .and_then(Object::as_dict_mut)
    .map_err(|error| PdfError::Lopdf(format!("invalid PDF catalog: {error}")))?;
  if !preferences.is_empty() {
    catalog.set("ViewerPreferences", Object::Dictionary(preferences));
  }
  if let Some(page_mode) = page_mode {
    catalog.set("PageMode", Object::Name(page_mode.to_vec()));
  }
  if let Some(open_action) = open_action {
    catalog.set("OpenAction", Object::Array(open_action));
  }

  let mut output = Vec::new();
  document
    .save_to(&mut output)
    .map_err(|error| PdfError::Lopdf(format!("{error}")))?;
  Ok(output)
}

fn existing_viewer_preferences(
  document: &Document,
  catalog_id: lopdf::ObjectId,
) -> Result<Dictionary> {
  let catalog = document
    .get_dictionary(catalog_id)
    .map_err(|error| PdfError::Lopdf(format!("invalid PDF catalog: {error}")))?;
  let Ok(value) = catalog.get(b"ViewerPreferences") else {
    return Ok(Dictionary::new());
  };
  match value {
    Object::Dictionary(dictionary) => Ok(dictionary.clone()),
    Object::Reference(id) => document
      .get_dictionary(*id)
      .cloned()
      .map_err(|error| PdfError::Lopdf(format!("invalid ViewerPreferences: {error}"))),
    _ => Err(PdfError::Lopdf(
      "ViewerPreferences is not a dictionary".to_string(),
    )),
  }
}

fn set_true(dictionary: &mut Dictionary, name: &'static str, value: bool) {
  if value {
    dictionary.set(name, Object::Boolean(true));
  }
}

fn page_mode_name(mode: PdfViewerPageMode) -> &'static [u8] {
  match mode {
    PdfViewerPageMode::Default => b"UseNone",
    PdfViewerPageMode::UseOutlines => b"UseOutlines",
    PdfViewerPageMode::UseThumbs => b"UseThumbs",
  }
}

fn open_action(document: &Document, options: &PdfOptions) -> Result<Vec<Object>> {
  let pages = document.get_pages();
  let page_id = pages
    .get(&options.viewer.initial_page)
    .copied()
    .ok_or_else(|| {
      PdfError::Options(format!(
        "viewer initial page {} exceeds the generated PDF page count {}",
        options.viewer.initial_page,
        pages.len()
      ))
    })?;
  let page = Object::Reference(page_id);
  Ok(match options.viewer.magnification {
    PdfViewerMagnification::Default => vec![
      page,
      Object::Name(b"XYZ".to_vec()),
      Object::Null,
      Object::Null,
      Object::Integer(0),
    ],
    PdfViewerMagnification::FitInWindow => vec![page, Object::Name(b"Fit".to_vec())],
    PdfViewerMagnification::FitWidth => vec![page, Object::Name(b"FitH".to_vec()), Object::Null],
    PdfViewerMagnification::FitVisible => vec![page, Object::Name(b"FitBH".to_vec()), Object::Null],
    PdfViewerMagnification::Zoom(zoom) => vec![
      page,
      Object::Name(b"XYZ".to_vec()),
      Object::Null,
      Object::Null,
      Object::Real(zoom),
    ],
  })
}

#[cfg(test)]
mod tests {
  use krilla::Document as KrillaDocument;
  use krilla::geom::Size;
  use krilla::page::PageSettings;

  use super::*;
  use crate::options::{PdfPageLayout, PdfViewerMagnification, PdfViewerPageMode};

  fn one_page_pdf() -> Vec<u8> {
    let mut document = KrillaDocument::new();
    document
      .start_page_with(PageSettings::new(Size::from_wh(100.0, 100.0).unwrap()))
      .finish();
    document.finish().unwrap()
  }

  #[test]
  fn viewer_options_reach_catalog_and_open_action() {
    let mut options = PdfOptions::default();
    options.metadata.title = Some("Viewer test".to_string());
    options.viewer.page_mode = PdfViewerPageMode::UseOutlines;
    options.viewer.page_layout = PdfPageLayout::ContinuousFacing;
    options.viewer.first_page_left = true;
    options.viewer.hide_toolbar = true;
    options.viewer.magnification = PdfViewerMagnification::FitVisible;

    let output = apply_viewer_preferences(one_page_pdf(), &options).unwrap();
    let parsed = Document::load_mem(&output).unwrap();
    let catalog_id = parsed.trailer.get(b"Root").unwrap().as_reference().unwrap();
    let catalog = parsed.get_dictionary(catalog_id).unwrap();
    assert_eq!(
      catalog.get(b"PageMode").unwrap().as_name().unwrap(),
      b"UseOutlines"
    );
    let preferences = catalog
      .get(b"ViewerPreferences")
      .unwrap()
      .as_dict()
      .unwrap();
    assert!(preferences.get(b"HideToolbar").unwrap().as_bool().unwrap());
    assert!(
      preferences
        .get(b"DisplayDocTitle")
        .unwrap()
        .as_bool()
        .unwrap()
    );
    assert_eq!(
      preferences.get(b"Direction").unwrap().as_name().unwrap(),
      b"R2L"
    );
    let action = catalog.get(b"OpenAction").unwrap().as_array().unwrap();
    assert_eq!(action[1].as_name().unwrap(), b"FitBH");
  }

  #[test]
  fn invalid_initial_page_is_reported_after_page_count_is_known() {
    let mut options = PdfOptions::default();
    options.viewer.initial_page = 2;

    assert!(matches!(
      apply_viewer_preferences(one_page_pdf(), &options),
      Err(PdfError::Options(message)) if message.contains("page count 1")
    ));
  }
}
