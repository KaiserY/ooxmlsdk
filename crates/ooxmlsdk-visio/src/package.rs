use ooxmlsdk::common::{SdkError, XmlNamespace};
use ooxmlsdk::namespaces::XmlKnownNamespace;
use ooxmlsdk::parts::visio_comments_part::VisioCommentsPart;
use ooxmlsdk::parts::visio_document::VisioDocument as VisioPackage;
use ooxmlsdk::parts::visio_master_part::VisioMasterPart;
use ooxmlsdk::parts::visio_masters_part::VisioMastersPart;
use ooxmlsdk::parts::visio_page_part::VisioPagePart;
use ooxmlsdk::parts::visio_pages_part::VisioPagesPart;
use ooxmlsdk::schemas::schemas_microsoft_com_office_visio_2012_main::{
  AuthorEntry, AuthorList, ColorEntry, Colors, CommentEntry, CommentList, Comments, FaceName,
  FaceNames, ForeignData, Master, MasterChoice, MasterContents, Masters, Page, PageChoice,
  PageContents, PageSheet, Pages, Rel, Shape, ShapeChoice, StyleSheet, StyleSheets,
  VisioDocument as Drawing,
};
use ooxmlsdk::sdk::{SdkPart, SdkType};
use ooxmlsdk::simple_type::BooleanValue;

use crate::cells::{page_height, page_width};
use crate::ids::Ids;

const PAGES: &str = "http://schemas.microsoft.com/visio/2010/relationships/pages";
const MASTERS: &str = "http://schemas.microsoft.com/visio/2010/relationships/masters";
const COMMENTS: &str = "http://schemas.microsoft.com/visio/2010/relationships/comments";

/// Append a page, its page part, and the page sheet's `PageWidth` and
/// `PageHeight`. The page id comes from `ids`.
pub fn add_page(
  package: &mut VisioPackage,
  ids: &mut Ids,
  name: Option<&str>,
  width: &str,
  height: &str,
) -> Result<u32, SdkError> {
  let document = package.visio_document_part()?;
  let pages_part =
    match document.child_part_by_relationship_type::<_, VisioPagesPart>(package, PAGES) {
      Some(part) => part,
      None => document.add_new_part_auto_id::<_, VisioPagesPart>(package)?,
    };
  let mut pages = read_xml::<Pages>(package, &pages_part)?;
  ensure_namespaces(
    &mut pages.xmlns,
    &[XmlKnownNamespace::Visio, XmlKnownNamespace::R],
  )?;
  let page_part = pages_part.add_new_part_auto_id::<_, VisioPagePart>(package)?;
  let rel_id = pages_part.get_id_of_part(package, &page_part)?.to_string();
  let page_id = ids.page();
  let mut sheet = PageSheet::default();
  page_width(&mut sheet, width);
  page_height(&mut sheet, height);
  pages.page.push(Page {
    id: page_id,
    name: name.map(str::to_string),
    page_choice: vec![
      PageChoice::PageSheet(Box::new(sheet)),
      PageChoice::Rel(Rel { id: Some(rel_id) }),
    ],
    ..Default::default()
  });
  pages_part.set_data(package, pages.to_xml()?.into_bytes())?;
  let mut contents = PageContents::default();
  ensure_namespaces(
    &mut contents.xmlns,
    &[XmlKnownNamespace::Visio, XmlKnownNamespace::R],
  )?;
  page_part.set_data(package, contents.to_xml()?.into_bytes())?;
  Ok(page_id)
}

/// Append a master and its master part. The master id comes from `ids`.
pub fn add_master(
  package: &mut VisioPackage,
  ids: &mut Ids,
  name: Option<&str>,
) -> Result<u32, SdkError> {
  let document = package.visio_document_part()?;
  let masters_part =
    match document.child_part_by_relationship_type::<_, VisioMastersPart>(package, MASTERS) {
      Some(part) => part,
      None => document.add_new_part_auto_id::<_, VisioMastersPart>(package)?,
    };
  let mut masters = read_xml::<Masters>(package, &masters_part)?;
  ensure_namespaces(
    &mut masters.xmlns,
    &[XmlKnownNamespace::Visio, XmlKnownNamespace::R],
  )?;
  let master_part = masters_part.add_new_part_auto_id::<_, VisioMasterPart>(package)?;
  let rel_id = masters_part
    .get_id_of_part(package, &master_part)?
    .to_string();
  let master_id = ids.master();
  masters.master.push(Master {
    id: master_id,
    name: name.map(str::to_string),
    master_choice: vec![MasterChoice::Rel(Rel { id: Some(rel_id) })],
    ..Default::default()
  });
  masters_part.set_data(package, masters.to_xml()?.into_bytes())?;
  let mut contents = MasterContents::default();
  ensure_namespaces(
    &mut contents.xmlns,
    &[XmlKnownNamespace::Visio, XmlKnownNamespace::R],
  )?;
  master_part.set_data(package, contents.to_xml()?.into_bytes())?;
  Ok(master_id)
}

/// Append a style sheet. The caller supplies the style id.
pub fn add_style(package: &mut VisioPackage, id: u32, name: Option<&str>) -> Result<(), SdkError> {
  let mut drawing = read_drawing(package)?;
  let sheets = drawing
    .style_sheets
    .get_or_insert_with(StyleSheets::default);
  sheets.style_sheet.push(StyleSheet {
    id,
    name: name.map(str::to_string),
    ..Default::default()
  });
  write_drawing(package, &drawing)
}

/// Append a font face the `Character` `Font` cell can name by id.
pub fn add_face_name(package: &mut VisioPackage, id: u32, name: &str) -> Result<(), SdkError> {
  let mut drawing = read_drawing(package)?;
  let faces = drawing.face_names.get_or_insert_with(FaceNames::default);
  faces.face_name.push(FaceName {
    id,
    name: name.to_string(),
    ..Default::default()
  });
  write_drawing(package, &drawing)
}

/// Append a color table entry.
pub fn add_color(package: &mut VisioPackage, index: u32, rgb: &str) -> Result<(), SdkError> {
  let mut drawing = read_drawing(package)?;
  let colors = drawing.colors.get_or_insert_with(Colors::default);
  colors.color_entry.push(ColorEntry {
    ix: index,
    rgb: rgb.to_string(),
  });
  write_drawing(package, &drawing)
}

pub(crate) fn read_drawing(package: &VisioPackage) -> Result<Drawing, SdkError> {
  let part = package.visio_document_part()?;
  let mut drawing = read_xml::<Drawing>(package, &part)?;
  ensure_namespaces(&mut drawing.xmlns, &[XmlKnownNamespace::Visio])?;
  Ok(drawing)
}

pub(crate) fn write_drawing(package: &mut VisioPackage, drawing: &Drawing) -> Result<(), SdkError> {
  let part = package.visio_document_part()?;
  part.set_data(package, drawing.to_xml()?.into_bytes())
}

/// Bytes for one image on a page, plus the `ForeignData` attributes from the
/// schema. The relationship id is written on `Rel`.
pub struct ImageBytes<'a> {
  pub content_type: &'a str,
  pub bytes: &'a [u8],
  pub foreign_type: &'a str,
  pub object_type: Option<u32>,
  pub show_as_icon: Option<bool>,
  pub object_width: Option<f64>,
  pub object_height: Option<f64>,
  pub mapping_mode: Option<u16>,
  pub extent_x: Option<f64>,
  pub extent_y: Option<f64>,
  pub compression_type: Option<&'a str>,
  pub compression_level: Option<f64>,
}

/// Store image bytes on the page and point the shape's `ForeignData` at that
/// part. Other shape children stay in place.
pub fn attach_image(
  shape: &mut Shape,
  page_part: &VisioPagePart,
  package: &mut VisioPackage,
  image: ImageBytes<'_>,
) -> Result<String, SdkError> {
  let image_part = page_part.add_image_part(package, image.content_type.to_string())?;
  image_part.set_data(package, image.bytes.to_vec())?;
  let rel_id = page_part.get_id_of_part(package, &image_part)?.to_string();
  let foreign = ShapeChoice::ForeignData(Box::new(ForeignData {
    foreign_type: image.foreign_type.to_string(),
    object_type: image.object_type,
    show_as_icon: image.show_as_icon.map(BooleanValue::from),
    object_width: image.object_width,
    object_height: image.object_height,
    mapping_mode: image.mapping_mode,
    extent_x: image.extent_x,
    extent_y: image.extent_y,
    compression_type: image.compression_type.map(str::to_string),
    compression_level: image.compression_level,
    rel: Rel {
      id: Some(rel_id.clone()),
    },
  }));
  let first = shape
    .shape_choice
    .iter()
    .position(|choice| matches!(choice, ShapeChoice::ForeignData(_)));
  shape
    .shape_choice
    .retain(|choice| !matches!(choice, ShapeChoice::ForeignData(_)));
  if let Some(index) = first {
    shape.shape_choice.insert(index, foreign);
  } else {
    shape.shape_choice.push(foreign);
  }
  Ok(rel_id)
}

pub fn image_relationship_id(shape: &Shape) -> Option<&str> {
  shape.shape_choice.iter().find_map(|choice| {
    let ShapeChoice::ForeignData(data) = choice else {
      return None;
    };
    data.rel.id.as_deref()
  })
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AuthorRecord {
  pub id: u32,
  pub name: Option<String>,
  pub initials: Option<String>,
  pub sip: Option<String>,
  pub smtp: Option<String>,
  pub resolution_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommentRecord {
  pub author_id: u32,
  pub page_id: u32,
  pub comment_id: u32,
  pub date: String,
  pub text: Option<String>,
  pub shape_id: Option<u32>,
  pub edit_date: Option<String>,
  pub done: Option<BooleanValue>,
  pub auto_comment_type: Option<u32>,
}

pub fn authors(comments: &Comments) -> Vec<AuthorRecord> {
  comments
    .author_list
    .iter()
    .flat_map(|list| &list.author_entry)
    .map(|author| AuthorRecord {
      id: author.id,
      name: author.name.clone(),
      initials: author.initials.clone(),
      sip: author.sip.clone(),
      smtp: author.smtp.clone(),
      resolution_id: author.resolution_id.clone(),
    })
    .collect()
}

pub fn push_author(comments: &mut Comments, author: AuthorRecord) {
  let list = comments.author_list.get_or_insert_with(AuthorList::default);
  list.author_entry.push(AuthorEntry {
    id: author.id,
    name: author.name,
    initials: author.initials,
    sip: author.sip,
    smtp: author.smtp,
    resolution_id: author.resolution_id,
  });
}

pub fn comment_entries(comments: &Comments) -> Vec<CommentRecord> {
  comments
    .comment_list
    .iter()
    .flat_map(|list| &list.comment_entry)
    .map(|comment| CommentRecord {
      author_id: comment.author_id,
      page_id: comment.page_id,
      comment_id: comment.comment_id,
      date: comment.date.clone(),
      text: comment.xml_content.clone(),
      shape_id: comment.shape_id,
      edit_date: comment.edit_date.clone(),
      done: comment.done,
      auto_comment_type: comment.auto_comment_type,
    })
    .collect()
}

pub fn push_comment(comments: &mut Comments, comment: CommentRecord) {
  let list = comments
    .comment_list
    .get_or_insert_with(CommentList::default);
  list.comment_entry.push(CommentEntry {
    author_id: comment.author_id,
    page_id: comment.page_id,
    comment_id: comment.comment_id,
    date: comment.date,
    xml_content: comment.text,
    shape_id: comment.shape_id,
    edit_date: comment.edit_date,
    done: comment.done,
    auto_comment_type: comment.auto_comment_type,
  });
}

pub fn list_authors(package: &VisioPackage) -> Result<Vec<AuthorRecord>, SdkError> {
  Ok(authors(&read_comments(package)?))
}

pub fn add_author(package: &mut VisioPackage, author: AuthorRecord) -> Result<(), SdkError> {
  let mut comments = read_comments(package)?;
  push_author(&mut comments, author);
  write_comments(package, &comments)
}

pub fn list_comments(package: &VisioPackage) -> Result<Vec<CommentRecord>, SdkError> {
  Ok(comment_entries(&read_comments(package)?))
}

pub fn add_comment(package: &mut VisioPackage, comment: CommentRecord) -> Result<(), SdkError> {
  let mut comments = read_comments(package)?;
  push_comment(&mut comments, comment);
  write_comments(package, &comments)
}

fn read_comments(package: &VisioPackage) -> Result<Comments, SdkError> {
  let document = package.visio_document_part()?;
  let mut comments =
    match document.child_part_by_relationship_type::<_, VisioCommentsPart>(package, COMMENTS) {
      Some(part) => read_xml::<Comments>(package, &part)?,
      None => Comments::default(),
    };
  ensure_namespaces(&mut comments.xmlns, &[XmlKnownNamespace::Visio])?;
  Ok(comments)
}

pub(crate) fn ensure_namespaces(
  namespaces: &mut Vec<XmlNamespace>,
  required: &[XmlKnownNamespace],
) -> Result<(), SdkError> {
  for &namespace in required {
    let mut present = false;
    for declaration in namespaces.iter() {
      let (prefix, uri) = match declaration {
        XmlNamespace::Known(existing) => (existing.prefix_bytes(), existing.uri_bytes()),
        XmlNamespace::Raw(raw) => {
          let mut parts = raw.splitn(2, |&byte| byte == 0);
          (
            parts.next().unwrap_or_default(),
            parts.next().unwrap_or_default(),
          )
        }
      };
      if prefix == namespace.prefix_bytes() {
        if uri != namespace.uri_bytes() {
          return Err(SdkError::CommonError(format!(
            "namespace prefix '{}' is already bound to a different URI",
            String::from_utf8_lossy(prefix)
          )));
        }
        present = true;
      }
    }
    if !present {
      namespaces.push(XmlNamespace::known(namespace));
    }
  }
  Ok(())
}

fn write_comments(package: &mut VisioPackage, comments: &Comments) -> Result<(), SdkError> {
  let document = package.visio_document_part()?;
  let part =
    match document.child_part_by_relationship_type::<_, VisioCommentsPart>(package, COMMENTS) {
      Some(part) => part,
      None => document.add_new_part_auto_id::<_, VisioCommentsPart>(package)?,
    };
  part.set_data(package, comments.to_xml()?.into_bytes())
}

fn read_xml<T>(package: &VisioPackage, part: &impl SdkPart) -> Result<T, SdkError>
where
  T: SdkType + Default,
{
  let mut bytes = Vec::new();
  if part.write_data_to(package, &mut bytes)? && !bytes.is_empty() {
    T::from_bytes(&bytes)
  } else {
    Ok(T::default())
  }
}
