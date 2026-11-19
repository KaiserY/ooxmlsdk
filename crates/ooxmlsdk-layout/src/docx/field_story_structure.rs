//! Field instructions hide structural marks without deleting source sections.
use super::{ImportedSection, section_fields};

pub(super) fn coalesce(sections: &mut Vec<ImportedSection>, instruction_boundaries: &[usize]) {
  if instruction_boundaries.is_empty() {
    return;
  }
  // Native Word keeps SECTION's source ordinal while SECTIONPAGES counts the
  // complete visible frame. Freeze the former before moving source blocks.
  section_fields::bind(sections);
  let mut visible: Vec<ImportedSection> = Vec::with_capacity(sections.len());
  for (index, mut section) in std::mem::take(sections).into_iter().enumerate() {
    if index > 0 && instruction_boundaries.binary_search(&(index - 1)).is_ok() {
      let previous = visible.last_mut().expect("preceding source section");
      previous.blocks.append(&mut section.blocks);
      previous
        .note_reference_order
        .append(&mut section.note_reference_order);
      // The first source section owns the page/columns/repeating stories even
      // after a real page break. Only the final visible carrier owns its end.
      // Deleting sectPr or borrowing the last source page setup changes the
      // native margin and baseline (90/709.54 becomes72/719.64 in controls).
      previous.discarded_carrier = section.discarded_carrier;
    } else {
      visible.push(section);
    }
  }
  *sections = visible;
  section_fields::bind(sections);
}
