use ooxmlsdk::schemas::schemas_microsoft_com_office_visio_2012_main::{
  Cp, Fld, Pp, Shape, ShapeChoice, Text, TextChoice, Tp,
};

/// One mark inside `visio:Text`, in document order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TextRun {
  Character(u32),
  Paragraph(u32),
  Tabs(u32),
  Field { index: u32, text: Option<String> },
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
      });
    }
  }
  runs
}

pub fn write_shape_text(shape: &mut Shape, runs: &[TextRun]) {
  shape
    .shape_choice
    .retain(|choice| !matches!(choice, ShapeChoice::Text(_)));
  if runs.is_empty() {
    return;
  }
  shape.shape_choice.insert(
    0,
    ShapeChoice::Text(Text {
      text_choice: runs.iter().map(run_to_choice).collect(),
    }),
  );
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
  }
}
