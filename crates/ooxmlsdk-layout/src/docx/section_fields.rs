//! SECTION belongs to the source section, including on a shared physical page.
//! Headers remain unbound so their repeated page supplies the section context.
use super::{Block, DynamicFieldKind, ImportedSection, InlineItem, TextRun};

pub(super) fn bind(sections: &mut [ImportedSection]) {
  for (index, section) in sections.iter_mut().enumerate() {
    bind_blocks(&mut section.blocks, index);
  }
}

fn bind_blocks(blocks: &mut [Block], index: usize) {
  for block in blocks {
    match block {
      Block::Paragraph(paragraph) => {
        for inline in &mut paragraph.inlines {
          match inline {
            InlineItem::Text(run) => bind_run(run, index),
            InlineItem::Ruby(ruby) => {
              for run in ruby.base.iter_mut().chain(&mut ruby.guide) {
                bind_run(run, index);
              }
            }
            InlineItem::Overstrike(overstrike) => {
              for run in overstrike.operands.iter_mut().flatten() {
                bind_run(run, index);
              }
            }
            InlineItem::EqArray(array) => {
              for run in array.cells.iter_mut().flatten() {
                bind_run(run, index);
              }
            }
            InlineItem::LegacyEquation(equation) => {
              for run in &mut equation.runs {
                bind_run(run, index);
              }
            }
            InlineItem::TwoLines(combined) => {
              for run in &mut combined.runs {
                bind_run(run, index);
              }
            }
            _ => {}
          }
        }
      }
      Block::Table(table) => {
        for row in &mut table.rows {
          for cell in &mut row.cells {
            bind_blocks(&mut cell.blocks, index);
          }
        }
      }
      Block::Frame(frame) => bind_blocks(&mut frame.blocks, index),
    }
  }
}

fn bind_run(run: &mut TextRun, index: usize) {
  match &mut run.dynamic_field {
    Some(DynamicFieldKind::Section { section_index, .. }) => {
      // Hidden field-code delimiters still count as source sections. A
      // prebound ordinal must survive coalescing their visible layout frames.
      section_index.get_or_insert(index);
    }
    Some(DynamicFieldKind::SectionPages { section_index, .. }) => {
      // Page counts belong to the resulting visible section frame, including
      // real page breaks after an unrendered field-code boundary.
      *section_index = Some(index);
    }
    _ => {}
  }
}
