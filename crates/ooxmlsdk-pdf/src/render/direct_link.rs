use pdf_writer::types::{ActionType, AnnotationFlags, AnnotationType};
use pdf_writer::writers::Destination;
use pdf_writer::{Finish, Name, Pdf, Rect, Ref, Str, TextStr};

use super::direct::RefAllocator;
use super::link::{InternalLinkTargets, LinkRect, ResolvedLink, ResolvedLinkTarget, resolve_link};
use crate::error::{PdfError, Result};
use crate::options::PdfOptions;

#[derive(Debug)]
struct DirectLinkAnnotation {
  id: Ref,
  link: ResolvedLink,
  struct_parent: Option<i32>,
}

#[derive(Debug, Default)]
pub(super) struct DirectPageLinks {
  annotations: Vec<DirectLinkAnnotation>,
}

impl DirectPageLinks {
  pub(super) fn resolve_and_push(
    &mut self,
    rect: LinkRect,
    alt_text: Option<&str>,
    url: &str,
    internal_links: &InternalLinkTargets,
    options: &PdfOptions,
    refs: &mut RefAllocator,
  ) -> Result<()> {
    let Some(link) = resolve_link(rect, alt_text, url, internal_links, options)? else {
      return Ok(());
    };
    self.annotations.push(DirectLinkAnnotation {
      id: refs.alloc()?,
      link,
      struct_parent: None,
    });
    Ok(())
  }

  pub(super) fn len(&self) -> usize {
    self.annotations.len()
  }

  pub(super) fn assign_struct_parent(&mut self, index: usize, key: i32) -> Result<Ref> {
    let annotation_count = self.annotations.len();
    let annotation = self.annotations.get_mut(index).ok_or_else(|| {
      PdfError::Writer(format!(
        "tagged link annotation {index} exceeds the page annotation count {annotation_count}"
      ))
    })?;
    if annotation.struct_parent.replace(key).is_some() {
      return Err(PdfError::Writer(format!(
        "tagged link annotation {index} has more than one structure parent"
      )));
    }
    Ok(annotation.id)
  }

  pub(super) fn ids(&self) -> impl Iterator<Item = Ref> + '_ {
    self.annotations.iter().map(|annotation| annotation.id)
  }

  pub(super) fn is_empty(&self) -> bool {
    self.annotations.is_empty()
  }

  pub(super) fn write_objects(
    &self,
    pdf: &mut Pdf,
    page_height_pt: f32,
    page_objects: &[(Ref, Ref)],
    page_heights_pt: &[f32],
  ) -> Result<()> {
    if page_objects.len() != page_heights_pt.len() {
      return Err(PdfError::Writer(
        "direct link page-object and page-height maps are inconsistent".to_string(),
      ));
    }
    for entry in &self.annotations {
      let link = &entry.link;
      let left = link.rect.x_pt;
      let bottom = page_height_pt - link.rect.y_pt - link.rect.height_pt;
      let right = link.rect.x_pt + link.rect.width_pt;
      let top = page_height_pt - link.rect.y_pt;
      if ![left, bottom, right, top].into_iter().all(f32::is_finite) {
        return Err(PdfError::Writer(
          "direct link annotation has non-finite PDF coordinates".to_string(),
        ));
      }

      let mut annotation = pdf.annotation(entry.id);
      annotation
        .subtype(AnnotationType::Link)
        .rect(Rect::new(left, bottom, right, top))
        .border(0.0, 0.0, 0.0, None)
        .flags(AnnotationFlags::PRINT)
        .contents(TextStr(&link.alt_text));
      if let Some(struct_parent) = entry.struct_parent {
        annotation.struct_parent(struct_parent);
      }
      match &link.target {
        ResolvedLinkTarget::Internal(position) => {
          let Some(&(target_page_id, _)) = page_objects.get(position.output_page_index) else {
            return Err(PdfError::Writer(format!(
              "internal link target page {} is outside the selected page map",
              position.output_page_index
            )));
          };
          let target_page_height = page_heights_pt[position.output_page_index];
          let target_top = target_page_height - position.y_pt;
          if ![position.x_pt, target_top].into_iter().all(f32::is_finite) {
            return Err(PdfError::Writer(
              "internal link destination has non-finite PDF coordinates".to_string(),
            ));
          }
          let destination = annotation.insert(Name(b"Dest")).start::<Destination<'_>>();
          destination
            .page(target_page_id)
            .xyz(position.x_pt, target_top, None);
        }
        ResolvedLinkTarget::Uri(uri) => {
          let mut action = annotation.action();
          action.action_type(ActionType::Uri).uri(Str(uri.as_bytes()));
          action.finish();
        }
      }
      annotation.finish();
    }
    Ok(())
  }
}
