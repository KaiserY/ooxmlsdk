//! Word's import recovery for custom VML WordArt inside a textbox story.
//!
//! Native controls distinguish this from ordinary preset WordArt, overflow,
//! textbox chaining and AutoSize. The custom object becomes a main-story
//! floating drawing at the page origin, anchored at the story's final mark.
//! The displaced host paragraph mark travels with it, so the following mark
//! owns the combined paragraph. Recover the source story before resolving
//! paragraph styles, rather than moving already formatted glyphs.

use super::{
  ImageCatalog, InlineShapeGeometry, v, vml_fontwork_custom_coordinate_size,
  vml_fontwork_text_path, vml_picture_shape_types, vml_shape_shape, vml_shape_type_for_reference,
  w,
};

pub(super) fn body(source: &w::Body, images: &ImageCatalog) -> Option<w::Body> {
  if !source.body_choice.iter().any(|choice| {
    let w::BodyChoice::Paragraph(paragraph) = choice else {
      return false;
    };
    textbox_paragraphs(paragraph)
      .into_iter()
      .any(|inner| pictures(inner).any(|picture| !custom_shapes(picture, images).is_empty()))
  }) {
    return None;
  }

  let mut recovered = source.clone();
  let mut prefix = Vec::new();
  let mut final_marks = Vec::new();
  for (index, choice) in std::mem::take(&mut recovered.body_choice)
    .into_iter()
    .enumerate()
  {
    let w::BodyChoice::Paragraph(mut paragraph) = choice else {
      debug_assert!(prefix.is_empty());
      recovered.body_choice.push(choice);
      continue;
    };
    let has_section = paragraph
      .paragraph_properties
      .as_deref()
      .is_some_and(|properties| properties.section_properties.is_some());
    let following_paragraph = matches!(
      source.body_choice.get(index + 1),
      Some(w::BodyChoice::Paragraph(_))
    );
    let end_of_body = index + 1 == source.body_choice.len();
    let promoted = if !has_section && (following_paragraph || end_of_body) {
      detach_custom_shapes(&mut paragraph, images)
    } else {
      Vec::new()
    };
    if !prefix.is_empty() {
      prefix.append(&mut paragraph.paragraph_choice);
      paragraph.paragraph_choice = std::mem::take(&mut prefix);
    }
    if promoted.is_empty() {
      recovered
        .body_choice
        .push(w::BodyChoice::Paragraph(paragraph));
      continue;
    }
    let mut mark = paragraph.clone();
    mark.paragraph_choice = promoted.into_iter().map(floating_run).collect();
    if following_paragraph {
      prefix = std::mem::take(&mut paragraph.paragraph_choice);
      final_marks.push(w::BodyChoice::Paragraph(mark));
    } else {
      // A final host already owns the last story mark. Its ordinary content
      // remains there and the promoted drawings share that existing anchor.
      paragraph
        .paragraph_choice
        .append(&mut mark.paragraph_choice);
      recovered
        .body_choice
        .push(w::BodyChoice::Paragraph(paragraph));
    }
  }
  recovered.body_choice.extend(final_marks);
  Some(recovered)
}

fn pictures(paragraph: &w::Paragraph) -> impl Iterator<Item = &w::Picture> {
  paragraph
    .paragraph_choice
    .iter()
    .filter_map(|choice| {
      let w::ParagraphChoice::WRun(run) = choice else {
        return None;
      };
      Some(run.as_ref())
    })
    .flat_map(|run| &run.run_choice)
    .filter_map(|choice| {
      let w::RunChoice::Picture(picture) = choice else {
        return None;
      };
      Some(picture.as_ref())
    })
}

fn textbox_paragraphs(paragraph: &w::Paragraph) -> Vec<&w::Paragraph> {
  let mut paragraphs = Vec::new();
  for picture in pictures(paragraph) {
    for choice in &picture.picture_choice {
      let w::PictureChoice::Shape(shape) = choice else {
        continue;
      };
      for choice in &shape.shape_choice {
        let v::ShapeChoice::TextBox(textbox) = choice else {
          continue;
        };
        let Some(v::TextBoxChoice::TextBoxContent(content)) = &textbox.text_box_choice else {
          continue;
        };
        for choice in &content.text_box_content_choice {
          if let w::TextBoxContentChoice::Paragraph(paragraph) = choice {
            paragraphs.push(paragraph.as_ref());
          }
        }
      }
    }
  }
  paragraphs
}

fn custom_shapes(picture: &w::Picture, images: &ImageCatalog) -> Vec<usize> {
  let definitions = vml_picture_shape_types(picture, images);
  picture
    .picture_choice
    .iter()
    .enumerate()
    .filter_map(|(index, choice)| {
      let w::PictureChoice::Shape(shape) = choice else {
        return None;
      };
      let definition = vml_shape_type_for_reference(shape, &definitions);
      if vml_fontwork_text_path(shape, definition).is_none()
        || vml_fontwork_custom_coordinate_size(shape, definition).is_none()
      {
        return None;
      }
      let imported = vml_shape_shape(shape, images, &definitions)?;
      (imported.text_warp.is_some()
        && matches!(imported.geometry, InlineShapeGeometry::Path { .. }))
      .then_some(index)
    })
    .collect()
}

fn detach_custom_shapes(paragraph: &mut w::Paragraph, images: &ImageCatalog) -> Vec<v::Shape> {
  let mut promoted = Vec::new();
  for choice in &mut paragraph.paragraph_choice {
    let w::ParagraphChoice::WRun(run) = choice else {
      continue;
    };
    for choice in &mut run.run_choice {
      let w::RunChoice::Picture(picture) = choice else {
        continue;
      };
      for choice in &mut picture.picture_choice {
        let w::PictureChoice::Shape(shape) = choice else {
          continue;
        };
        for choice in &mut shape.shape_choice {
          let v::ShapeChoice::TextBox(textbox) = choice else {
            continue;
          };
          let Some(v::TextBoxChoice::TextBoxContent(content)) = &mut textbox.text_box_choice else {
            continue;
          };
          for choice in &mut content.text_box_content_choice {
            let w::TextBoxContentChoice::Paragraph(inner) = choice else {
              continue;
            };
            for choice in &mut inner.paragraph_choice {
              let w::ParagraphChoice::WRun(run) = choice else {
                continue;
              };
              for choice in &mut run.run_choice {
                let w::RunChoice::Picture(picture) = choice else {
                  continue;
                };
                let indices = custom_shapes(picture, images);
                let mut index = 0;
                picture.picture_choice.retain(|choice| {
                  let remove = indices.contains(&index);
                  index += 1;
                  if remove && let w::PictureChoice::Shape(shape) = choice {
                    promoted.push(shape.as_ref().clone());
                  }
                  !remove
                });
              }
            }
          }
        }
      }
    }
  }
  promoted
}

fn floating_run(mut shape: v::Shape) -> w::ParagraphChoice {
  // A detached object's native page-space origin is (0,0). Express it with
  // page-relative coordinates rather than fabricating an anchor-dependent
  // negative paragraph offset. Its authored nominal size/brush/path survive.
  shape.style = Some(format!(
    "{};position:absolute;margin-left:0;margin-top:0;\
     mso-position-horizontal-relative:page;mso-position-vertical-relative:page;z-index:0",
    shape.style.as_deref().unwrap_or_default(),
  ));
  w::ParagraphChoice::WRun(Box::new(w::Run {
    run_choice: vec![w::RunChoice::Picture(Box::new(w::Picture {
      picture_choice: vec![w::PictureChoice::Shape(Box::new(shape))],
      ..Default::default()
    }))],
    ..Default::default()
  }))
}

#[cfg(test)]
mod tests {
  use super::*;
  use ooxmlsdk::sdk::SdkType;

  fn source(coordinates: &str, objects: usize) -> (w::Body, ImageCatalog) {
    let shapes = (0..objects)
      .map(|index| {
        format!(
          r##"<v:shape id="art{index}" type="#_x0000_t136" style="width:115.5pt;height:45pt">
        <v:textpath string="custom" fitpath="t"/></v:shape>"##,
        )
      })
      .collect::<String>();
    let source = w::Body::from_bytes(
      format!(
        r#"<w:body xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"
        xmlns:v="urn:schemas-microsoft-com:vml">
        <w:p><w:pPr><w:pStyle w:val="First"/></w:pPr><w:r><w:t>Before</w:t></w:r>
          <w:r><w:pict><v:shape style="position:absolute;width:250pt;height:75pt">
            <v:textbox><w:txbxContent><w:p><w:r><w:pict>
              <v:shapetype id="_x0000_t136" coordsize="{coordinates}"
                path="m,l21600,m,21600l21600,21600e"/>{shapes}
            </w:pict></w:r></w:p></w:txbxContent></v:textbox>
          </v:shape></w:pict></w:r></w:p>
        <w:p><w:pPr><w:pStyle w:val="Last"/></w:pPr><w:r><w:t>After</w:t></w:r></w:p>
      </w:body>"#,
      )
      .as_bytes(),
    )
    .unwrap();
    let images = ImageCatalog {
      vml_shape_types: super::super::vml_document_shape_types(&source),
      ..Default::default()
    };
    (source, images)
  }

  #[test]
  fn vml_custom_textbox_wordart_recovers_story_and_paragraph_property_owners() {
    for objects in [1, 2] {
      let (source, images) = source("1000,1000", objects);
      let recovered = body(&source, &images).unwrap();
      let paragraphs: Vec<_> = recovered
        .body_choice
        .iter()
        .filter_map(|choice| {
          let w::BodyChoice::Paragraph(paragraph) = choice else {
            return None;
          };
          Some(paragraph.as_ref())
        })
        .collect();
      assert_eq!(paragraphs.len(), 2);
      let text: String = paragraphs[0]
        .paragraph_choice
        .iter()
        .filter_map(|choice| {
          let w::ParagraphChoice::WRun(run) = choice else {
            return None;
          };
          Some(&run.run_choice)
        })
        .flatten()
        .filter_map(|choice| {
          let w::RunChoice::Text(text) = choice else {
            return None;
          };
          text.xml_content.as_deref()
        })
        .collect();
      assert_eq!(text, "BeforeAfter");
      let style = |paragraph: &w::Paragraph| {
        paragraph
          .paragraph_properties
          .as_ref()
          .and_then(|properties| properties.paragraph_style_id.as_ref())
          .map(|style| style.val.as_str().to_owned())
      };
      assert_eq!(style(paragraphs[0]).as_deref(), Some("Last"));
      assert_eq!(style(paragraphs[1]).as_deref(), Some("First"));
      assert_eq!(pictures(paragraphs[1]).count(), objects);
      assert!(
        textbox_paragraphs(paragraphs[0])
          .into_iter()
          .all(|paragraph| {
            pictures(paragraph).all(|picture| custom_shapes(picture, &images).is_empty())
          })
      );
      let w::BodyChoice::Paragraph(original) = &source.body_choice[0] else {
        unreachable!()
      };
      assert!(textbox_paragraphs(original).into_iter().any(|paragraph| {
        pictures(paragraph).any(|picture| custom_shapes(picture, &images).len() == objects)
      }));
    }
  }

  #[test]
  fn vml_recognized_preset_wordart_keeps_its_textbox_story() {
    let (source, images) = source("21600,21600", 1);
    assert!(body(&source, &images).is_none());
  }
}
