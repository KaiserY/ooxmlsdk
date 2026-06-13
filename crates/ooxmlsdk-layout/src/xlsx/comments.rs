use ooxmlsdk::parts::spreadsheet_document::SpreadsheetDocument;
use ooxmlsdk::parts::worksheet_comments_part::WorksheetCommentsPart;
use ooxmlsdk::parts::worksheet_threaded_comments_part::WorksheetThreadedCommentsPart;
use ooxmlsdk::schemas::schemas_microsoft_com_office_spreadsheetml_2018_threadedcomments as tc;
use ooxmlsdk::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main as x;
use ooxmlsdk::sdk::SdkPart;

use crate::error::Result;

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct CommentsCatalog {
  pub(crate) legacy: Option<LegacyCommentsCatalog>,
  pub(crate) threaded: Vec<ThreadedCommentsCatalog>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct LegacyCommentsCatalog {
  pub(crate) relationship_id: Option<String>,
  pub(crate) authors: Vec<String>,
  pub(crate) comments: Vec<LegacyCommentModel>,
  pub(crate) has_extensions: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct LegacyCommentModel {
  pub(crate) reference: String,
  pub(crate) author_id: u32,
  pub(crate) author: Option<String>,
  pub(crate) guid: Option<String>,
  pub(crate) shape_id: Option<u32>,
  pub(crate) text: String,
  pub(crate) rich_runs: usize,
  pub(crate) phonetic_runs: usize,
  pub(crate) has_comment_properties: bool,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct ThreadedCommentsCatalog {
  pub(crate) relationship_id: Option<String>,
  pub(crate) comments: Vec<ThreadedCommentModel>,
  pub(crate) has_extensions: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ThreadedCommentModel {
  pub(crate) reference: Option<String>,
  pub(crate) id: String,
  pub(crate) parent_id: Option<String>,
  pub(crate) person_id: String,
  pub(crate) date_time: Option<String>,
  pub(crate) done: bool,
  pub(crate) text: Option<String>,
  pub(crate) mentions: usize,
  pub(crate) has_extensions: bool,
}

impl CommentsCatalog {
  pub(crate) fn from_worksheet_part(
    package: &mut SpreadsheetDocument,
    comments_part: Option<WorksheetCommentsPart>,
    threaded_parts: Vec<WorksheetThreadedCommentsPart>,
  ) -> Result<Self> {
    // Source: LibreOffice sc/source/filter/oox/commentsbuffer.cxx and
    // threadedcommentsfragment.cxx. Legacy comments create notes; threaded
    // comments are attached to matching legacy notes by root id/cell later.
    Ok(Self {
      legacy: comments_part
        .as_ref()
        .map(|part| LegacyCommentsCatalog::from_part(package, part))
        .transpose()?,
      threaded: threaded_parts
        .iter()
        .map(|part| ThreadedCommentsCatalog::from_part(package, part))
        .collect::<Result<Vec<_>>>()?,
    })
  }
}

impl LegacyCommentsCatalog {
  fn from_part(package: &mut SpreadsheetDocument, part: &WorksheetCommentsPart) -> Result<Self> {
    let comments = part.root_element(package)?;
    let authors = comments
      .authors
      .author
      .iter()
      .map(|author| author.xml_content.clone().unwrap_or_default())
      .collect::<Vec<_>>();
    Ok(Self {
      relationship_id: part.relationship_id().map(ToString::to_string),
      comments: comments
        .comment_list
        .comment
        .iter()
        .map(|comment| LegacyCommentModel::from_comment(comment, &authors))
        .collect(),
      authors,
      has_extensions: comments.extension_list.is_some(),
    })
  }
}

impl LegacyCommentModel {
  fn from_comment(comment: &x::Comment, authors: &[String]) -> Self {
    Self {
      reference: comment.reference.clone(),
      author_id: comment.author_id,
      author: authors.get(comment.author_id as usize).cloned(),
      guid: comment.guid.clone(),
      shape_id: comment.shape_id,
      text: comment_text(&comment.comment_text),
      rich_runs: comment.comment_text.run.len(),
      phonetic_runs: comment.comment_text.phonetic_run.len(),
      has_comment_properties: comment.comment_properties.is_some(),
    }
  }
}

impl ThreadedCommentsCatalog {
  fn from_part(
    package: &mut SpreadsheetDocument,
    part: &WorksheetThreadedCommentsPart,
  ) -> Result<Self> {
    let comments = part.root_element(package)?;
    Ok(Self {
      relationship_id: part.relationship_id().map(ToString::to_string),
      comments: comments
        .threaded_comment
        .iter()
        .map(ThreadedCommentModel::from_threaded_comment)
        .collect(),
      has_extensions: comments.extension_list.is_some(),
    })
  }
}

impl ThreadedCommentModel {
  fn from_threaded_comment(comment: &tc::ThreadedComment) -> Self {
    Self {
      reference: comment.r#ref.clone(),
      id: comment.id.clone(),
      parent_id: comment.parent_id.clone(),
      person_id: comment.person_id.clone(),
      date_time: comment.d_t.clone(),
      done: comment.done.is_some_and(|value| value.as_bool()),
      text: comment
        .threaded_comment_text
        .as_ref()
        .and_then(|text| text.0.xml_content.clone()),
      mentions: comment
        .threaded_comment_mentions
        .as_ref()
        .map_or(0, |mentions| mentions.mention.len()),
      has_extensions: comment.extension_list.is_some(),
    }
  }
}

fn comment_text(text: &x::CommentText) -> String {
  if let Some(text) = &text.text
    && let Some(content) = &text.xml_content
  {
    return content.clone();
  }

  text
    .run
    .iter()
    .filter_map(|run| run.text.xml_content.as_deref())
    .collect()
}
