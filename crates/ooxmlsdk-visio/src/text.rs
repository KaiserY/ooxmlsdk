use ooxmlsdk::schemas::schemas_microsoft_com_office_visio_2012_main::{
  Cp, Fld, Pp, Shape, ShapeChoice, Text, TextChoice, Tp,
};

/// One mark or character run inside `visio:Text`, in document order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TextRun {
  Character(u32),
  Paragraph(u32),
  Tabs(u32),
  Field { index: u32, text: Option<String> },
  Chars(String),
}

pub fn read_shape_text(shape: &Shape) -> Vec<TextRun> {
  let mut runs = Vec::new();
  for choice in &shape.shape_choice {
    let ShapeChoice::Text(text) = choice else {
      continue;
    };
    for mark in &text.text_choice {
      runs.push(match mark {
        TextChoice::Cp(cp) => TextRun::Character(cp.ix),
        TextChoice::Pp(pp) => TextRun::Paragraph(pp.ix),
        TextChoice::Tp(tp) => TextRun::Tabs(tp.ix),
        TextChoice::Fld(field) => TextRun::Field {
          index: field.ix,
          text: field.xml_content.clone(),
        },
        TextChoice::Text(text) => TextRun::Chars(text.clone()),
      });
    }
  }
  runs
}

/// Replace the shape's `Text` child. Other children, such as `PinX`, stay
/// where they are.
pub fn write_shape_text(shape: &mut Shape, runs: &[TextRun]) {
  let first = shape
    .shape_choice
    .iter()
    .position(|choice| matches!(choice, ShapeChoice::Text(_)));
  shape
    .shape_choice
    .retain(|choice| !matches!(choice, ShapeChoice::Text(_)));
  if runs.is_empty() {
    return;
  }
  let text = ShapeChoice::Text(Text {
    text_choice: runs.iter().map(run_to_choice).collect(),
  });
  if let Some(index) = first {
    shape.shape_choice.insert(index, text);
  } else {
    shape.shape_choice.push(text);
  }
}

fn run_to_choice(run: &TextRun) -> TextChoice {
  match run {
    TextRun::Character(index) => TextChoice::Cp(Cp { ix: *index }),
    TextRun::Paragraph(index) => TextChoice::Pp(Pp { ix: *index }),
    TextRun::Tabs(index) => TextChoice::Tp(Tp { ix: *index }),
    TextRun::Field { index, text } => TextChoice::Fld(Fld {
      ix: *index,
      xml_content: text.clone(),
    }),
    TextRun::Chars(text) => TextChoice::Text(text.clone()),
  }
}
