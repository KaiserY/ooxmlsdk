use super::*;

/// Retain field syntax independently of the visible cached result.  A
/// WordprocessingML complex field is a document-story construct: its begin,
/// instruction, separator, and end characters do not have to live in one
/// paragraph.  In particular, Word writes an outer TOC field whose result is
/// a sequence of entry paragraphs containing nested PAGEREF fields.
pub(super) fn paragraph_field_events(paragraph: &w::Paragraph) -> Vec<ParagraphFieldEvent> {
  let mut events = Vec::new();
  for choice in &paragraph.paragraph_choice {
    collect_paragraph_choice(choice, &mut events);
  }
  events
}

pub(super) fn body_level_bookmark_names(body: &w::Body) -> HashSet<String> {
  body
    .body_choice
    .iter()
    .filter_map(|choice| match choice {
      w::BodyChoice::BookmarkStart(bookmark) if !bookmark.name.is_empty() => {
        Some(bookmark.name.to_string())
      }
      _ => None,
    })
    .collect()
}

fn collect_paragraph_choice(choice: &w::ParagraphChoice, events: &mut Vec<ParagraphFieldEvent>) {
  match choice {
    w::ParagraphChoice::WRun(run) => collect_run(run, events),
    w::ParagraphChoice::SimpleField(field) => collect_simple_field(field, events),
    w::ParagraphChoice::Hyperlink(hyperlink) => collect_hyperlink(hyperlink, events),
    w::ParagraphChoice::CustomXmlRun(custom_xml) | w::ParagraphChoice::SmartTagRun(custom_xml) => {
      collect_custom_xml_run(custom_xml, events)
    }
    w::ParagraphChoice::SdtRun(sdt) => collect_sdt_run(sdt, events),
    w::ParagraphChoice::InsertedRun(inserted) => collect_inserted_run(inserted, events),
    // Deleted and move-from content is not part of the current document
    // story.  This mirrors the visible-run importer.
    w::ParagraphChoice::DeletedRun(_) | w::ParagraphChoice::MoveFromRun(_) => {}
    w::ParagraphChoice::MoveToRun(moved) => collect_move_to_run(moved, events),
    w::ParagraphChoice::BookmarkStart(bookmark) => collect_bookmark_start(bookmark, events),
    w::ParagraphChoice::BookmarkEnd(bookmark) => collect_bookmark_end(bookmark, events),
    _ => {}
  }
}

fn collect_run(run: &w::Run, events: &mut Vec<ParagraphFieldEvent>) {
  for choice in &run.run_choice {
    match choice {
      w::RunChoice::FieldChar(field) => match field.field_char_type {
        w::FieldCharValues::Begin => events.push(ParagraphFieldEvent::Begin {
          locked: on_off(field.field_lock),
          dirty: on_off(field.dirty),
        }),
        w::FieldCharValues::Separate => events.push(ParagraphFieldEvent::Separate),
        w::FieldCharValues::End => events.push(ParagraphFieldEvent::End),
      },
      w::RunChoice::FieldCode(code) => {
        if let Some(text) = code.0.xml_content.as_deref() {
          events.push(ParagraphFieldEvent::Instruction(text.to_string()));
        }
      }
      w::RunChoice::Run(nested) => collect_run(nested, events),
      // Everything else in w:r is visible story content or a visible layout
      // control (text, tab, break, drawing, note reference, and so on).
      _ => events.push(ParagraphFieldEvent::Content),
    }
  }
}

fn collect_simple_field(field: &w::SimpleField, events: &mut Vec<ParagraphFieldEvent>) {
  events.push(ParagraphFieldEvent::Simple {
    instruction: field.instruction.to_string(),
    locked: on_off(field.field_lock),
    dirty: on_off(field.dirty),
  });
  for choice in &field.simple_field_choice {
    match choice {
      w::SimpleFieldChoice::WRun(run) => collect_run(run, events),
      w::SimpleFieldChoice::SimpleField(nested) => collect_simple_field(nested, events),
      w::SimpleFieldChoice::Hyperlink(hyperlink) => collect_hyperlink(hyperlink, events),
      w::SimpleFieldChoice::CustomXmlRun(custom_xml) => collect_custom_xml_run(custom_xml, events),
      w::SimpleFieldChoice::SdtRun(sdt) => collect_sdt_run(sdt, events),
      w::SimpleFieldChoice::InsertedRun(inserted) => collect_inserted_run(inserted, events),
      w::SimpleFieldChoice::DeletedRun(_) | w::SimpleFieldChoice::MoveFromRun(_) => {}
      w::SimpleFieldChoice::MoveToRun(moved) => collect_move_to_run(moved, events),
      w::SimpleFieldChoice::BookmarkStart(bookmark) => collect_bookmark_start(bookmark, events),
      w::SimpleFieldChoice::BookmarkEnd(bookmark) => collect_bookmark_end(bookmark, events),
      _ => {}
    }
  }
}

fn collect_hyperlink(hyperlink: &w::Hyperlink, events: &mut Vec<ParagraphFieldEvent>) {
  for choice in &hyperlink.hyperlink_choice {
    match choice {
      w::HyperlinkChoice::WRun(run) => collect_run(run, events),
      w::HyperlinkChoice::SimpleField(field) => collect_simple_field(field, events),
      w::HyperlinkChoice::Hyperlink(nested) => collect_hyperlink(nested, events),
      w::HyperlinkChoice::CustomXmlRun(custom_xml) => collect_custom_xml_run(custom_xml, events),
      w::HyperlinkChoice::SdtRun(sdt) => collect_sdt_run(sdt, events),
      w::HyperlinkChoice::InsertedRun(inserted) => collect_inserted_run(inserted, events),
      w::HyperlinkChoice::DeletedRun(_) | w::HyperlinkChoice::MoveFromRun(_) => {}
      w::HyperlinkChoice::MoveToRun(moved) => collect_move_to_run(moved, events),
      w::HyperlinkChoice::BookmarkStart(bookmark) => collect_bookmark_start(bookmark, events),
      w::HyperlinkChoice::BookmarkEnd(bookmark) => collect_bookmark_end(bookmark, events),
      _ => {}
    }
  }
}

fn collect_custom_xml_run(custom_xml: &w::CustomXmlRun, events: &mut Vec<ParagraphFieldEvent>) {
  for choice in &custom_xml.custom_xml_run_choice {
    match choice {
      w::CustomXmlRunChoice::WRun(run) => collect_run(run, events),
      w::CustomXmlRunChoice::SimpleField(field) => collect_simple_field(field, events),
      w::CustomXmlRunChoice::Hyperlink(hyperlink) => collect_hyperlink(hyperlink, events),
      w::CustomXmlRunChoice::CustomXmlRun(nested) | w::CustomXmlRunChoice::SmartTagRun(nested) => {
        collect_custom_xml_run(nested, events)
      }
      w::CustomXmlRunChoice::SdtRun(sdt) => collect_sdt_run(sdt, events),
      w::CustomXmlRunChoice::InsertedRun(inserted) => collect_inserted_run(inserted, events),
      w::CustomXmlRunChoice::DeletedRun(_) | w::CustomXmlRunChoice::MoveFromRun(_) => {}
      w::CustomXmlRunChoice::MoveToRun(moved) => collect_move_to_run(moved, events),
      w::CustomXmlRunChoice::BookmarkStart(bookmark) => collect_bookmark_start(bookmark, events),
      w::CustomXmlRunChoice::BookmarkEnd(bookmark) => collect_bookmark_end(bookmark, events),
      _ => {}
    }
  }
}

fn collect_sdt_run(sdt: &w::SdtRun, events: &mut Vec<ParagraphFieldEvent>) {
  let Some(content) = sdt.sdt_content_run.as_ref() else {
    return;
  };
  for choice in &content.sdt_content_run_choice {
    match choice {
      w::SdtContentRunChoice::WRun(run) => collect_run(run, events),
      w::SdtContentRunChoice::SimpleField(field) => collect_simple_field(field, events),
      w::SdtContentRunChoice::Hyperlink(hyperlink) => collect_hyperlink(hyperlink, events),
      w::SdtContentRunChoice::CustomXmlRun(custom_xml) => {
        collect_custom_xml_run(custom_xml, events)
      }
      w::SdtContentRunChoice::SdtRun(nested) => collect_sdt_run(nested, events),
      w::SdtContentRunChoice::InsertedRun(inserted) => collect_inserted_run(inserted, events),
      w::SdtContentRunChoice::DeletedRun(_) | w::SdtContentRunChoice::MoveFromRun(_) => {}
      w::SdtContentRunChoice::MoveToRun(moved) => collect_move_to_run(moved, events),
      w::SdtContentRunChoice::BookmarkStart(bookmark) => collect_bookmark_start(bookmark, events),
      w::SdtContentRunChoice::BookmarkEnd(bookmark) => collect_bookmark_end(bookmark, events),
      _ => {}
    }
  }
}

fn collect_inserted_run(inserted: &w::InsertedRun, events: &mut Vec<ParagraphFieldEvent>) {
  for choice in &inserted.inserted_run_choice {
    match choice {
      w::InsertedRunChoice::WRun(run) => collect_run(run, events),
      w::InsertedRunChoice::SdtRun(sdt) => collect_sdt_run(sdt, events),
      w::InsertedRunChoice::InsertedRun(nested) => collect_inserted_run(nested, events),
      w::InsertedRunChoice::DeletedRun(_) | w::InsertedRunChoice::MoveFromRun(_) => {}
      w::InsertedRunChoice::MoveToRun(moved) => collect_move_to_run(moved, events),
      w::InsertedRunChoice::BookmarkStart(bookmark) => collect_bookmark_start(bookmark, events),
      w::InsertedRunChoice::BookmarkEnd(bookmark) => collect_bookmark_end(bookmark, events),
      _ => {}
    }
  }
}

fn collect_move_to_run(moved: &w::MoveToRun, events: &mut Vec<ParagraphFieldEvent>) {
  for choice in &moved.move_to_run_choice {
    match choice {
      w::MoveToRunChoice::WRun(run) => collect_run(run, events),
      w::MoveToRunChoice::SdtRun(sdt) => collect_sdt_run(sdt, events),
      w::MoveToRunChoice::InsertedRun(inserted) => collect_inserted_run(inserted, events),
      w::MoveToRunChoice::DeletedRun(_) | w::MoveToRunChoice::MoveFromRun(_) => {}
      w::MoveToRunChoice::MoveToRun(nested) => collect_move_to_run(nested, events),
      w::MoveToRunChoice::BookmarkStart(bookmark) => collect_bookmark_start(bookmark, events),
      w::MoveToRunChoice::BookmarkEnd(bookmark) => collect_bookmark_end(bookmark, events),
      _ => {}
    }
  }
}

fn collect_bookmark_start(bookmark: &w::BookmarkStart, events: &mut Vec<ParagraphFieldEvent>) {
  if !bookmark.name.is_empty() {
    events.push(ParagraphFieldEvent::BookmarkStart {
      id: bookmark.id.to_string(),
      name: bookmark.name.to_string(),
    });
  }
}

fn collect_bookmark_end(bookmark: &w::BookmarkEnd, events: &mut Vec<ParagraphFieldEvent>) {
  events.push(ParagraphFieldEvent::BookmarkEnd {
    id: bookmark.id.to_string(),
  });
}

fn on_off(value: Option<ooxmlsdk::simple_type::OnOffValue>) -> bool {
  value.is_some_and(ooxmlsdk::simple_type::OnOffValue::as_bool)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct TocLevelRange {
  start: u8,
  end: u8,
}

impl TocLevelRange {
  const ALL: Self = Self { start: 1, end: 9 };

  fn contains(self, level: u8) -> bool {
    (self.start..=self.end).contains(&level)
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct TocSpec {
  outline_levels: Option<TocLevelRange>,
  use_applied_outline_level: bool,
  tc_entries: bool,
  tc_identifier: Option<String>,
  tc_levels: TocLevelRange,
  caption_identifier: Option<String>,
  omit_caption_label_and_number: bool,
  bookmark_name: Option<String>,
  custom_style_levels: Vec<(String, u8)>,
  hyperlinks: bool,
  no_page_number_levels: Option<TocLevelRange>,
  page_separator: Option<char>,
  chapter_sequence: Option<String>,
  chapter_separator: String,
  preserve_tabs: bool,
  preserve_newlines: bool,
  hide_page_numbers_in_web_layout: bool,
}

impl Default for TocSpec {
  fn default() -> Self {
    Self {
      outline_levels: None,
      use_applied_outline_level: false,
      tc_entries: false,
      tc_identifier: None,
      tc_levels: TocLevelRange::ALL,
      caption_identifier: None,
      omit_caption_label_and_number: false,
      bookmark_name: None,
      custom_style_levels: Vec::new(),
      hyperlinks: false,
      no_page_number_levels: None,
      page_separator: None,
      chapter_sequence: None,
      chapter_separator: "-".to_string(),
      preserve_tabs: false,
      preserve_newlines: false,
      hide_page_numbers_in_web_layout: false,
    }
  }
}

impl TocSpec {
  fn parse(instruction: &str) -> Option<Self> {
    let tokens = field_instruction_tokens(instruction);
    if !tokens
      .first()
      .is_some_and(|name| name.eq_ignore_ascii_case("TOC"))
    {
      return None;
    }

    let mut spec = Self::default();
    let mut explicit_source = false;
    let mut tc_level_seen_before_f = false;
    let mut index = 1;
    while index < tokens.len() {
      let Some(switch) = field_switch(&tokens[index]) else {
        index += 1;
        continue;
      };
      let switch = switch.to_ascii_lowercase();
      match switch.as_str() {
        "h" => spec.hyperlinks = true,
        "u" => {
          explicit_source = true;
          spec.use_applied_outline_level = true;
        }
        "w" => spec.preserve_tabs = true,
        "x" => spec.preserve_newlines = true,
        "z" => spec.hide_page_numbers_in_web_layout = true,
        "n" => {
          spec.no_page_number_levels = Some(
            next_switch_argument(&tokens, index)
              .and_then(parse_level_range)
              .unwrap_or(TocLevelRange::ALL),
          );
          index += usize::from(next_switch_argument(&tokens, index).is_some());
        }
        "o" => {
          explicit_source = true;
          spec.outline_levels = Some(
            next_switch_argument(&tokens, index)
              .and_then(parse_level_range)
              .unwrap_or(TocLevelRange::ALL),
          );
          index += usize::from(next_switch_argument(&tokens, index).is_some());
        }
        "l" => {
          if let Some(argument) = next_switch_argument(&tokens, index) {
            if let Some(range) = parse_level_range(argument) {
              spec.tc_levels = range;
            }
            index += 1;
          }
          tc_level_seen_before_f = true;
        }
        "f" => {
          explicit_source = true;
          spec.tc_entries = true;
          if tc_level_seen_before_f {
            // [MS-OI29500] §2.1.509(a): a later \f supersedes an earlier
            // \l and restores inclusion of every TC level.
            spec.tc_levels = TocLevelRange::ALL;
          }
          if let Some(argument) = next_switch_argument(&tokens, index) {
            spec.tc_identifier = Some(argument.to_string());
            index += 1;
          }
        }
        "a" | "c" => {
          explicit_source = true;
          if let Some(argument) = next_switch_argument(&tokens, index) {
            spec.caption_identifier = Some(argument.to_string());
            index += 1;
          }
          spec.omit_caption_label_and_number = switch == "a";
        }
        "b" => {
          if let Some(argument) = next_switch_argument(&tokens, index) {
            spec.bookmark_name = Some(argument.to_string());
            index += 1;
          }
        }
        "p" => {
          if let Some(argument) = next_switch_argument(&tokens, index) {
            // [MS-OI29500] §2.1.509(b): Word consumes only the first
            // character, even though ECMA calls this a sequence.
            spec.page_separator = argument.chars().next();
            index += 1;
          }
        }
        "s" => {
          if let Some(argument) = next_switch_argument(&tokens, index) {
            spec.chapter_sequence = Some(argument.to_string());
            index += 1;
          }
        }
        "d" => {
          if let Some(argument) = next_switch_argument(&tokens, index) {
            spec.chapter_separator = argument.chars().take(15).collect();
            index += 1;
          }
        }
        "t" => {
          explicit_source = true;
          if let Some(argument) = next_switch_argument(&tokens, index) {
            spec.custom_style_levels = parse_style_level_pairs(argument);
            index += 1;
          }
        }
        _ => {}
      }
      index += 1;
    }

    // Word and Writer both create a switchless TOC from outline levels.
    if !explicit_source {
      spec.outline_levels = Some(TocLevelRange::ALL);
    }
    Some(spec)
  }

  fn suppress_page_number(&self, level: u8) -> bool {
    self
      .no_page_number_levels
      .is_some_and(|range| range.contains(level))
  }
}

fn field_switch(token: &str) -> Option<&str> {
  token
    .strip_prefix('\\')
    // Old Word and several upstream fixtures serialize slash switches.
    .or_else(|| token.strip_prefix('/'))
    .filter(|switch| !switch.is_empty())
}

fn next_switch_argument(tokens: &[String], index: usize) -> Option<&str> {
  tokens
    .get(index + 1)
    .filter(|token| field_switch(token).is_none())
    .map(String::as_str)
}

fn parse_level_range(value: &str) -> Option<TocLevelRange> {
  let value = value.trim().trim_matches('"');
  let (start, end) = value
    .split_once('-')
    .map_or((value, value), |(start, end)| (start, end));
  let start = start.trim().parse::<u8>().ok()?;
  let end = end.trim().parse::<u8>().ok()?;
  (start >= 1 && start <= end && end <= 9).then_some(TocLevelRange { start, end })
}

fn parse_style_level_pairs(value: &str) -> Vec<(String, u8)> {
  let separator = if value.contains(',') {
    ','
  } else if value.contains(';') {
    ';'
  } else {
    return Vec::new();
  };
  let components = value.split(separator).map(str::trim).collect::<Vec<_>>();
  components
    .chunks_exact(2)
    .filter_map(|pair| {
      let style = pair[0].trim_matches('"');
      let level = pair[1].parse::<u8>().ok()?;
      (!style.is_empty() && (1..=9).contains(&level)).then(|| (style.to_string(), level))
    })
    .collect()
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ParagraphAddress {
  section_index: usize,
  path: Vec<StoryPathStep>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum StoryPathStep {
  Block(usize),
  TableCell { row_index: usize, cell_index: usize },
}

impl ParagraphAddress {
  fn block_index(&self) -> Option<usize> {
    match self.path.last()? {
      StoryPathStep::Block(index) => Some(*index),
      StoryPathStep::TableCell { .. } => None,
    }
  }

  fn container_path(&self) -> &[StoryPathStep] {
    &self.path[..self.path.len().saturating_sub(1)]
  }

  fn is_top_level(&self) -> bool {
    matches!(self.path.as_slice(), [StoryPathStep::Block(_)])
  }
}

#[derive(Clone, Debug)]
struct StoryParagraph {
  address: ParagraphAddress,
  bookmark_scopes: HashSet<String>,
  content_bookmark_scopes: HashSet<String>,
  source_bookmarks: Vec<String>,
}

#[derive(Clone, Debug)]
struct StoryField {
  start_ordinal: usize,
  end_ordinal: usize,
  instruction: String,
  locked: bool,
  dirty: bool,
  starts_at_paragraph_start: bool,
  ends_at_paragraph_end: bool,
}

#[derive(Clone, Debug)]
struct TocSpan {
  start_ordinal: usize,
  end_ordinal: usize,
  locked: bool,
  dirty: bool,
  spec: TocSpec,
}

#[derive(Clone, Debug, Default)]
struct StoryScan {
  paragraphs: Vec<StoryParagraph>,
  fields: Vec<StoryField>,
  toc_spans: Vec<TocSpan>,
  bookmark_names: HashSet<String>,
}

#[derive(Clone, Debug)]
struct OpenStoryField {
  start_ordinal: usize,
  instruction: String,
  locked: bool,
  dirty: bool,
  separated: bool,
  starts_at_paragraph_start: bool,
}

fn scan_main_story(sections: &[ImportedSection]) -> StoryScan {
  let mut scan = StoryScan::default();
  let mut fields = Vec::<OpenStoryField>::new();
  let mut active_bookmarks = HashMap::<String, String>::new();

  for (section_index, section) in sections.iter().enumerate() {
    scan_story_blocks(
      section_index,
      &section.blocks,
      &[],
      &mut scan,
      &mut fields,
      &mut active_bookmarks,
    );
  }

  scan.toc_spans = scan
    .fields
    .iter()
    .filter_map(|field| {
      TocSpec::parse(&field.instruction).map(|spec| TocSpan {
        start_ordinal: field.start_ordinal,
        end_ordinal: field.end_ordinal,
        locked: field.locked,
        dirty: field.dirty,
        spec,
      })
    })
    .collect();
  scan
}

fn scan_story_blocks(
  section_index: usize,
  blocks: &[Block],
  parent_path: &[StoryPathStep],
  scan: &mut StoryScan,
  fields: &mut Vec<OpenStoryField>,
  active_bookmarks: &mut HashMap<String, String>,
) {
  for (block_index, block) in blocks.iter().enumerate() {
    let mut path = parent_path.to_vec();
    path.push(StoryPathStep::Block(block_index));
    match block {
      Block::Paragraph(paragraph) => scan_story_paragraph(
        ParagraphAddress {
          section_index,
          path,
        },
        paragraph,
        scan,
        fields,
        active_bookmarks,
      ),
      Block::Table(table) => {
        for (row_index, row) in table.rows.iter().enumerate() {
          for (cell_index, cell) in row.cells.iter().enumerate() {
            let mut cell_path = path.clone();
            cell_path.push(StoryPathStep::TableCell {
              row_index,
              cell_index,
            });
            scan_story_blocks(
              section_index,
              &cell.blocks,
              &cell_path,
              scan,
              fields,
              active_bookmarks,
            );
          }
        }
      }
      // Floating frames and their text boxes are separate Word stories. They
      // neither supply main-story TOC entries nor extend main-story fields.
      Block::Frame(_) => {}
    }
  }
}

fn scan_story_paragraph(
  address: ParagraphAddress,
  paragraph: &Paragraph,
  scan: &mut StoryScan,
  fields: &mut Vec<OpenStoryField>,
  active_bookmarks: &mut HashMap<String, String>,
) {
  let ordinal = scan.paragraphs.len();
  let mut bookmark_scopes = active_bookmarks.values().cloned().collect::<HashSet<_>>();
  let mut content_bookmark_scopes = HashSet::new();
  let mut source_bookmarks = Vec::new();

  for (event_index, event) in paragraph.field_events.iter().enumerate() {
    match event {
      ParagraphFieldEvent::Content => {
        content_bookmark_scopes.extend(active_bookmarks.values().cloned());
      }
      ParagraphFieldEvent::Begin { locked, dirty } => fields.push(OpenStoryField {
        start_ordinal: ordinal,
        instruction: String::new(),
        locked: *locked,
        dirty: *dirty,
        separated: false,
        starts_at_paragraph_start: !paragraph.field_events[..event_index]
          .iter()
          .any(|event| matches!(event, ParagraphFieldEvent::Content)),
      }),
      ParagraphFieldEvent::Instruction(text) => {
        if let Some(field) = fields.last_mut()
          && !field.separated
        {
          field.instruction.push_str(text);
        }
      }
      ParagraphFieldEvent::Separate => {
        if let Some(field) = fields.last_mut() {
          field.separated = true;
        }
      }
      ParagraphFieldEvent::End => {
        if let Some(field) = fields.pop() {
          scan.fields.push(StoryField {
            start_ordinal: field.start_ordinal,
            end_ordinal: ordinal,
            instruction: field.instruction,
            locked: field.locked,
            dirty: field.dirty,
            starts_at_paragraph_start: field.starts_at_paragraph_start,
            ends_at_paragraph_end: !paragraph.field_events[event_index + 1..]
              .iter()
              .any(|event| matches!(event, ParagraphFieldEvent::Content)),
          });
        }
      }
      ParagraphFieldEvent::Simple {
        instruction,
        locked,
        dirty,
      } => scan.fields.push(StoryField {
        start_ordinal: ordinal,
        end_ordinal: ordinal,
        instruction: instruction.clone(),
        locked: *locked,
        dirty: *dirty,
        starts_at_paragraph_start: !paragraph.field_events[..event_index]
          .iter()
          .any(|event| matches!(event, ParagraphFieldEvent::Content)),
        ends_at_paragraph_end: !paragraph.field_events[event_index + 1..]
          .iter()
          .any(|event| matches!(event, ParagraphFieldEvent::Content)),
      }),
      ParagraphFieldEvent::BookmarkStart { id, name } => {
        scan.bookmark_names.insert(name.clone());
        bookmark_scopes.insert(name.clone());
        source_bookmarks.push(name.clone());
        active_bookmarks.insert(id.clone(), name.clone());
      }
      ParagraphFieldEvent::BookmarkEnd { id } => {
        if let Some(name) = active_bookmarks.remove(id) {
          // A bookmark that closes within this paragraph still contains part
          // of that paragraph and remains a valid \b source range for it.
          bookmark_scopes.insert(name);
        }
      }
      ParagraphFieldEvent::SuppressParagraphBreak { .. }
      | ParagraphFieldEvent::SuppressReferenceParagraphBreak { .. }
      | ParagraphFieldEvent::DeferredParagraphBreak { .. }
      | ParagraphFieldEvent::DeferredReferenceParagraphBreak { .. }
      | ParagraphFieldEvent::ReferenceResultSpan { .. } => {}
    }
  }

  scan.paragraphs.push(StoryParagraph {
    address,
    bookmark_scopes,
    content_bookmark_scopes,
    source_bookmarks,
  });
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct TcSpec {
  text: String,
  identifier: Option<String>,
  level: u8,
  omit_page_number: bool,
}

impl TcSpec {
  fn parse(instruction: &str) -> Option<Self> {
    let tokens = field_instruction_tokens(instruction);
    if !tokens
      .first()
      .is_some_and(|name| name.eq_ignore_ascii_case("TC"))
    {
      return None;
    }
    let mut text = None;
    let mut identifier = None;
    let mut level = 1;
    let mut omit_page_number = false;
    let mut index = 1;
    while index < tokens.len() {
      if let Some(switch) = field_switch(&tokens[index]) {
        match switch.to_ascii_lowercase().as_str() {
          "f" => {
            if let Some(argument) = next_switch_argument(&tokens, index) {
              identifier = Some(argument.to_string());
              index += 1;
            }
          }
          "l" => {
            if let Some(argument) = next_switch_argument(&tokens, index) {
              if let Ok(value) = argument.parse::<u8>()
                && (1..=9).contains(&value)
              {
                level = value;
              }
              index += 1;
            }
          }
          "n" => omit_page_number = true,
          // General formatting switches have one argument which is not the
          // TC entry text.
          "*" | "#" | "@" => {
            index += usize::from(next_switch_argument(&tokens, index).is_some());
          }
          _ => {}
        }
      } else if text.is_none() {
        text = Some(tokens[index].clone());
      }
      index += 1;
    }
    let text = text?.trim().to_string();
    (!text.is_empty()).then_some(Self {
      text,
      identifier,
      level,
      omit_page_number,
    })
  }
}

fn seq_identifier(instruction: &str) -> Option<String> {
  let tokens = field_instruction_tokens(instruction);
  if !tokens
    .first()
    .is_some_and(|name| name.eq_ignore_ascii_case("SEQ"))
  {
    return None;
  }
  tokens
    .iter()
    .skip(1)
    .find(|token| field_switch(token).is_none())
    .cloned()
}

#[derive(Clone, Debug)]
struct TocEntrySource {
  source_ordinal: usize,
  level: u8,
  text: String,
  omit_page_number: bool,
  chapter_prefix: Option<String>,
}

#[derive(Clone, Debug)]
struct TocReplacement {
  span: TocSpan,
  blocks: Vec<Block>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum MissingReferenceError {
  ReferenceSourceNotFound,
  BookmarkNameNotSpecified,
}

#[derive(Clone, Debug)]
struct StoryReplacement {
  start_ordinal: usize,
  end_ordinal: usize,
  blocks: Vec<Block>,
}

#[derive(Clone, Debug)]
struct ReferenceResultRefresh {
  paragraph_ordinal: usize,
  field_id: u64,
  inline_start: usize,
  inline_end: usize,
  replacement: Vec<InlineItem>,
  paragraph_break_offsets: Vec<usize>,
}

pub(super) fn refresh_tables_of_contents(
  sections: &mut [ImportedSection],
  styles: &StylesCatalog,
  update_fields_on_open: bool,
  ui_language: Option<&str>,
  body_level_bookmarks: &HashSet<String>,
) {
  let mut scan = scan_main_story(sections);
  scan
    .bookmark_names
    .extend(body_level_bookmarks.iter().cloned());
  let resolved_page_references =
    resolve_layout_stable_page_reference_fields(sections, &scan, ui_language);
  let refreshed_body_references =
    refresh_body_level_reference_fields(sections, &scan, body_level_bookmarks);
  let refreshed_reference_spans = refresh_missing_reference_fields(sections, &scan, ui_language);
  let refreshed_references =
    resolved_page_references || refreshed_body_references || refreshed_reference_spans;
  let scan = if refreshed_references {
    let mut refreshed_scan = scan_main_story(sections);
    refreshed_scan
      .bookmark_names
      .extend(body_level_bookmarks.iter().cloned());
    refreshed_scan
  } else {
    scan
  };
  if scan.toc_spans.is_empty() {
    return;
  }

  let mut bookmark_names = scan.bookmark_names.clone();
  let mut bookmarks_by_source = HashMap::<usize, String>::new();
  let mut generated_bookmarks = HashMap::<usize, String>::new();
  let mut next_bookmark_id = 1_u32;
  let mut replacements = Vec::new();

  for span in &scan.toc_spans {
    normalize_cached_toc_hyperlink_style(sections, &scan, span, styles);
    // ECMA-376 §17.16.18 defines everything after the optional separator as
    // the current field result. That result may legitimately be empty. Its
    // absence is not an implicit update request: a TOC is recalculated only
    // when its begin character is dirty or settings request field updates.
    if span.locked || (!span.dirty && !update_fields_on_open) {
      continue;
    }

    let mut entries = collect_toc_entry_sources(sections, &scan, span);
    let templates = cached_toc_templates(sections, &scan, span, styles);
    let page = scan
      .paragraphs
      .get(span.start_ordinal)
      .and_then(|paragraph| sections.get(paragraph.address.section_index))
      .map(|section| section.page)
      .unwrap_or_default();

    let mut blocks = Vec::new();
    for entry in &mut entries {
      let bookmark_name = bookmarks_by_source
        .entry(entry.source_ordinal)
        .or_insert_with(|| {
          if let Some(existing) = preferred_source_bookmark(&scan, entry.source_ordinal) {
            return existing.to_string();
          }
          let name = loop {
            let candidate = format!("_Toc{next_bookmark_id:08X}");
            next_bookmark_id = next_bookmark_id.saturating_add(1);
            if bookmark_names.insert(candidate.clone()) {
              break candidate;
            }
          };
          generated_bookmarks.insert(entry.source_ordinal, name.clone());
          name
        })
        .clone();
      blocks.push(Block::paragraph(build_toc_entry_paragraph(
        entry,
        &bookmark_name,
        &span.spec,
        templates.get(&entry.level),
        styles,
        page,
      )));
    }

    if blocks.is_empty() {
      blocks.push(Block::paragraph(build_empty_toc_result(
        &span.spec,
        templates.get(&1),
        styles,
        page,
        ui_language,
      )));
    }
    replacements.push(TocReplacement {
      span: span.clone(),
      blocks,
    });
  }

  for (ordinal, bookmark_name) in generated_bookmarks {
    let Some(paragraph) = paragraph_mut(sections, &scan, ordinal) else {
      continue;
    };
    if !paragraph
      .inlines
      .iter()
      .any(|inline| matches!(inline, InlineItem::BookmarkStart(name) if name == &bookmark_name))
    {
      paragraph
        .inlines
        .insert(0, InlineItem::BookmarkStart(bookmark_name));
    }
  }

  replacements.sort_by_key(|replacement| std::cmp::Reverse(replacement.span.start_ordinal));
  for replacement in replacements {
    replace_toc_span(sections, &scan, replacement);
  }
}

fn refresh_body_level_reference_fields(
  sections: &mut [ImportedSection],
  scan: &StoryScan,
  body_level_bookmarks: &HashSet<String>,
) -> bool {
  let mut refreshes = Vec::new();
  for paragraph_ordinal in 0..scan.paragraphs.len() {
    let Some(cached_paragraph) = paragraph(sections, scan, paragraph_ordinal) else {
      continue;
    };
    for event in &cached_paragraph.field_events {
      let ParagraphFieldEvent::ReferenceResultSpan {
        field_id,
        bookmark_name,
        inline_start,
        inline_end,
        merge_format,
      } = event
      else {
        continue;
      };
      let Some(canonical_name) = body_level_bookmarks
        .iter()
        .find(|candidate| candidate.eq_ignore_ascii_case(bookmark_name))
      else {
        continue;
      };
      let Some(source_paragraphs) = body_level_reference_source(sections, scan, canonical_name)
      else {
        continue;
      };
      let inline_start = (*inline_start).min(cached_paragraph.inlines.len());
      let inline_end = (*inline_end)
        .max(inline_start)
        .min(cached_paragraph.inlines.len());
      let cached_result = &cached_paragraph.inlines[inline_start..inline_end];
      if !cached_result
        .iter()
        .all(|inline| matches!(inline, InlineItem::Text(_)))
      {
        // A text-only body-level REF is a closed, evidence-backed unit. Keep
        // cached drawings, notes, controls, and mixed structural results until
        // their bookmark-copy semantics are modeled without loss.
        continue;
      }

      let cached_template = cached_result.iter().find_map(|inline| match inline {
        InlineItem::Text(run) => Some(run.clone()),
        _ => None,
      });
      let mut replacement = Vec::with_capacity(source_paragraphs.len());
      let mut paragraph_break_offsets =
        Vec::with_capacity(source_paragraphs.len().saturating_sub(1));
      for (source_index, (text, source_style)) in source_paragraphs.iter().enumerate() {
        let mut run = cached_template.clone().unwrap_or_else(|| TextRun {
          text: String::new(),
          style: cached_paragraph.base_style.clone(),
          hyperlink_url: None,
          dynamic_field: None,
          style_ref_keys: Vec::new(),
          style_ref_text: None,
          style_ref_numbering_text: None,
          preserve_text_portion: false,
        });
        run.text.clone_from(text);
        if !merge_format {
          run.style.clone_from(source_style);
        }
        run.dynamic_field = None;
        run.style_ref_keys.clear();
        run.style_ref_text = None;
        run.style_ref_numbering_text = None;
        run.preserve_text_portion = false;
        replacement.push(InlineItem::Text(run));
        if source_index + 1 < source_paragraphs.len() {
          paragraph_break_offsets.push(replacement.len());
        }
      }
      mark_wordprocessing_field_result(&mut replacement);
      refreshes.push(ReferenceResultRefresh {
        paragraph_ordinal,
        field_id: *field_id,
        inline_start,
        inline_end,
        replacement,
        paragraph_break_offsets,
      });
    }
  }

  if refreshes.is_empty() {
    return false;
  }

  // Later spans first keep every earlier inline offset valid. Top-level REF
  // fields cannot overlap; sorting also handles multiple sequential fields in
  // one paragraph without field-specific offset heuristics.
  refreshes.sort_by_key(|refresh| {
    (
      std::cmp::Reverse(refresh.paragraph_ordinal),
      std::cmp::Reverse(refresh.inline_start),
    )
  });
  let resolved_field_ids = refreshes
    .iter()
    .map(|refresh| refresh.field_id)
    .collect::<HashSet<_>>();

  for refresh in refreshes {
    let Some(paragraph) = paragraph_mut(sections, scan, refresh.paragraph_ordinal) else {
      continue;
    };
    let inserted_len = refresh.replacement.len();
    adjust_reference_event_offsets_for_splice(
      &mut paragraph.field_events,
      refresh.inline_start,
      refresh.inline_end,
      inserted_len,
    );
    paragraph.inlines.splice(
      refresh.inline_start..refresh.inline_end,
      refresh.replacement,
    );
    paragraph
      .field_events
      .extend(
        refresh
          .paragraph_break_offsets
          .into_iter()
          .map(
            |relative_offset| ParagraphFieldEvent::DeferredParagraphBreak {
              inline_offset: refresh.inline_start + relative_offset,
            },
          ),
      );
    refresh_paragraph_story_derivatives(paragraph);
  }

  // Switch only the resolved REF's authored boundaries from cached-result
  // reconstruction to fixed-output replacement. Unresolved or unsupported
  // REF fields keep the old reversible events and therefore their cache.
  for paragraph_ordinal in 0..scan.paragraphs.len() {
    let Some(paragraph) = paragraph_mut(sections, scan, paragraph_ordinal) else {
      continue;
    };
    let mut events = Vec::with_capacity(paragraph.field_events.len());
    for event in std::mem::take(&mut paragraph.field_events) {
      match event {
        ParagraphFieldEvent::SuppressReferenceParagraphBreak { field_id }
          if resolved_field_ids.contains(&field_id) =>
        {
          events.push(ParagraphFieldEvent::SuppressParagraphBreak { deferred: false });
        }
        ParagraphFieldEvent::DeferredReferenceParagraphBreak { field_id, .. }
        | ParagraphFieldEvent::ReferenceResultSpan { field_id, .. }
          if resolved_field_ids.contains(&field_id) => {}
        event => events.push(event),
      }
    }
    paragraph.field_events = events;
  }
  true
}

fn body_level_reference_source(
  sections: &[ImportedSection],
  scan: &StoryScan,
  bookmark_name: &str,
) -> Option<Vec<(String, TextStyle)>> {
  let mut source = Vec::new();
  for (ordinal, scanned_paragraph) in scan.paragraphs.iter().enumerate() {
    if !scanned_paragraph
      .content_bookmark_scopes
      .iter()
      .any(|candidate| candidate.eq_ignore_ascii_case(bookmark_name))
    {
      continue;
    }
    let paragraph = paragraph(sections, scan, ordinal)?;
    if !paragraph
      .inlines
      .iter()
      .all(|inline| matches!(inline, InlineItem::Text(_) | InlineItem::BookmarkStart(_)))
    {
      return None;
    }
    let text = paragraph_source_text(paragraph)?;
    let style = paragraph
      .inlines
      .iter()
      .find_map(|inline| match inline {
        InlineItem::Text(run) => Some(run.style.clone()),
        _ => None,
      })
      .unwrap_or_else(|| paragraph.base_style.clone());
    source.push((text, style));
  }
  (!source.is_empty()).then_some(source)
}

fn adjust_reference_event_offsets_for_splice(
  events: &mut [ParagraphFieldEvent],
  inline_start: usize,
  inline_end: usize,
  inserted_len: usize,
) {
  let shift = |offset: &mut usize| {
    if *offset >= inline_end {
      *offset = if inserted_len >= inline_end.saturating_sub(inline_start) {
        offset.saturating_add(inserted_len - inline_end.saturating_sub(inline_start))
      } else {
        offset.saturating_sub(inline_end.saturating_sub(inline_start) - inserted_len)
      };
    } else if *offset > inline_start {
      *offset = inline_start + inserted_len;
    }
  };

  for event in events {
    match event {
      ParagraphFieldEvent::DeferredParagraphBreak { inline_offset }
      | ParagraphFieldEvent::DeferredReferenceParagraphBreak { inline_offset, .. } => {
        shift(inline_offset);
      }
      ParagraphFieldEvent::ReferenceResultSpan {
        inline_start: start,
        inline_end: end,
        ..
      } => {
        shift(start);
        shift(end);
      }
      _ => {}
    }
  }
}

fn resolve_layout_stable_page_reference_fields(
  sections: &mut [ImportedSection],
  scan: &StoryScan,
  ui_language: Option<&str>,
) -> bool {
  let mut resolved = false;
  for ordinal in 0..scan.paragraphs.len() {
    let Some(paragraph) = paragraph_mut(sections, scan, ordinal) else {
      continue;
    };
    for inline in &mut paragraph.inlines {
      let InlineItem::Text(run) = inline else {
        continue;
      };
      resolved |= resolve_layout_stable_page_reference_run(run, &scan.bookmark_names, ui_language);
    }
    #[cfg(test)]
    for run in &mut paragraph.runs {
      resolve_layout_stable_page_reference_run(run, &scan.bookmark_names, ui_language);
    }
  }
  resolved
}

fn resolve_layout_stable_page_reference_run(
  run: &mut TextRun,
  bookmark_names: &HashSet<String>,
  ui_language: Option<&str>,
) -> bool {
  let Some(DynamicFieldKind::PageRef { bookmark_name, .. }) = run.dynamic_field.as_ref() else {
    return false;
  };
  if bookmark_names
    .iter()
    .any(|candidate| candidate.eq_ignore_ascii_case(bookmark_name))
  {
    return false;
  }

  // Bookmark existence is document structure, not pagination state. Resolve
  // this replacement before line formatting so a long localized diagnostic
  // can wrap and participate in center/right tab formatting. The dynamic
  // field importer stores the complete cached result in this run, including
  // any field-owned tab, so replacing the run also removes that stale tab.
  let message = FieldMessage::UndefinedBookmark;
  run.text = localized_field_message(message, ui_language);
  apply_generated_field_message_style(&mut run.style, message, ui_language);
  run.dynamic_field = None;
  true
}

fn refresh_missing_reference_fields(
  sections: &mut [ImportedSection],
  scan: &StoryScan,
  ui_language: Option<&str>,
) -> bool {
  let mut replacements = scan
    .fields
    .iter()
    .filter_map(|field| {
      if field.locked || !field.starts_at_paragraph_start || !field.ends_at_paragraph_end {
        return None;
      }
      let error = missing_reference_error(field, &scan.bookmark_names)?;
      let paragraph = build_missing_reference_result(
        paragraph(sections, scan, field.start_ordinal)?,
        error,
        ui_language,
      );
      Some(StoryReplacement {
        start_ordinal: field.start_ordinal,
        end_ordinal: field.end_ordinal,
        blocks: vec![Block::paragraph(paragraph)],
      })
    })
    .collect::<Vec<_>>();

  // Replacing later story ranges first keeps every earlier ParagraphAddress
  // valid even when a multi-block cached result collapses to one paragraph.
  replacements.sort_by_key(|replacement| std::cmp::Reverse(replacement.start_ordinal));
  let refreshed = !replacements.is_empty();
  for replacement in replacements {
    replace_story_span(sections, scan, replacement);
  }
  refreshed
}

fn missing_reference_error(
  field: &StoryField,
  bookmark_names: &HashSet<String>,
) -> Option<MissingReferenceError> {
  let tokens = field_instruction_tokens(&field.instruction);
  let instruction_name = tokens.first()?;
  let is_ref = instruction_name.eq_ignore_ascii_case("REF");
  let is_noteref = instruction_name.eq_ignore_ascii_case("NOTEREF");
  if !is_ref && !is_noteref {
    return None;
  }

  let bookmark_name = reference_bookmark_argument(&tokens);
  if is_noteref && bookmark_name.is_none() {
    return Some(MissingReferenceError::BookmarkNameNotSpecified);
  }
  let bookmark_name = bookmark_name?;
  (!bookmark_names
    .iter()
    .any(|candidate| candidate.eq_ignore_ascii_case(bookmark_name)))
  .then_some(MissingReferenceError::ReferenceSourceNotFound)
}

fn reference_bookmark_argument(tokens: &[String]) -> Option<&str> {
  let mut index = 1;
  while index < tokens.len() {
    if let Some(switch) = field_switch(&tokens[index]) {
      // General formatting switches and REF's delimiter switch consume their
      // following token. The other REF/NOTEREF switches are flags.
      index += if matches!(switch.to_ascii_lowercase().as_str(), "*" | "#" | "@" | "d")
        && next_switch_argument(tokens, index).is_some()
      {
        2
      } else {
        1
      };
      continue;
    }
    return Some(tokens[index].as_str());
  }
  None
}

fn build_missing_reference_result(
  template: &Paragraph,
  error: MissingReferenceError,
  ui_language: Option<&str>,
) -> Paragraph {
  let mut paragraph = template.clone();
  let mut style = paragraph
    .inlines
    .iter()
    .find_map(|inline| match inline {
      InlineItem::Text(run) => Some(run.style.clone()),
      InlineItem::Ruby(ruby) => ruby.base.first().map(|run| run.style.clone()),
      _ => None,
    })
    .unwrap_or_else(|| paragraph.base_style.clone());
  let message = match error {
    MissingReferenceError::ReferenceSourceNotFound => FieldMessage::ReferenceSourceNotFound,
    MissingReferenceError::BookmarkNameNotSpecified => FieldMessage::BookmarkNameNotSpecified,
  };
  apply_generated_field_message_style(&mut style, message, ui_language);
  let text = localized_field_message(message, ui_language);
  let run = TextRun {
    text: text.clone(),
    style,
    hyperlink_url: None,
    dynamic_field: None,
    style_ref_keys: Vec::new(),
    style_ref_text: None,
    style_ref_numbering_text: None,
    preserve_text_portion: false,
  };

  paragraph.inlines.clear();
  paragraph.inlines.push(InlineItem::Text(run.clone()));
  paragraph.field_events.clear();
  paragraph.footnote_reference_ids.clear();
  paragraph.endnote_reference_ids.clear();
  paragraph.starts_after_last_rendered_page_break = false;
  paragraph.style_ref_text = Some(Arc::<str>::from(text));
  paragraph.style_ref_numbering_text = None;
  #[cfg(test)]
  {
    paragraph.runs.clear();
    paragraph.runs.push(run);
  }
  paragraph
}

fn normalize_cached_toc_hyperlink_style(
  sections: &mut [ImportedSection],
  scan: &StoryScan,
  span: &TocSpan,
  styles: &StylesCatalog,
) {
  for ordinal in span.start_ordinal..=span.end_ordinal {
    let Some(paragraph) = paragraph_mut(sections, scan, ordinal) else {
      continue;
    };
    let base_style = paragraph.base_style.clone();
    let hyperlink_style = styles.synthesized_hyperlink_run_style(paragraph.base_style.clone());
    for inline in &mut paragraph.inlines {
      let InlineItem::Text(run) = inline else {
        continue;
      };
      if !run
        .hyperlink_url
        .as_deref()
        .is_some_and(|url| url.starts_with("ooxmlsdk-pdf:bookmark:"))
        || !run
          .style_ref_keys
          .iter()
          .any(|key| key.eq_ignore_ascii_case("Hyperlink"))
      {
        continue;
      }
      // Word's TOC \h result remains an active internal hyperlink, but the
      // generated entry is painted with its TOC paragraph style rather than
      // the ordinary blue/underlined Hyperlink character style. Only remove
      // properties that still equal that character style so direct run
      // formatting remains authoritative.
      if run.style.color == hyperlink_style.color
        && (run.style.opacity - hyperlink_style.opacity).abs() <= f32::EPSILON
      {
        run.style.color = base_style.color;
        run.style.opacity = base_style.opacity;
        run.style.color_is_automatic = base_style.color_is_automatic;
      }
      if run.style.underline == hyperlink_style.underline
        && run.style.underline_color == hyperlink_style.underline_color
      {
        run.style.underline = base_style.underline;
        run.style.underline_color = base_style.underline_color;
      }
    }
  }
}

fn collect_toc_entry_sources(
  sections: &[ImportedSection],
  scan: &StoryScan,
  span: &TocSpan,
) -> Vec<TocEntrySource> {
  let mut entries = Vec::new();

  for ordinal in 0..scan.paragraphs.len() {
    if ordinal_inside_any_toc(scan, ordinal)
      || !paragraph_in_bookmark_scope(scan, ordinal, span.spec.bookmark_name.as_deref())
    {
      continue;
    }
    let Some(paragraph) = paragraph(sections, scan, ordinal) else {
      continue;
    };
    let Some(level) = paragraph_toc_level(paragraph, &span.spec) else {
      continue;
    };
    let Some(text) = paragraph_source_text(paragraph)
      .map(|text| normalize_toc_entry_text(text, &span.spec))
      .filter(|text| !text.is_empty())
    else {
      continue;
    };
    entries.push(TocEntrySource {
      source_ordinal: ordinal,
      level,
      text,
      omit_page_number: span.spec.suppress_page_number(level),
      chapter_prefix: chapter_prefix_for_source(sections, scan, ordinal, &span.spec),
    });
  }

  if span.spec.tc_entries {
    for field in &scan.fields {
      if ordinal_inside_any_toc(scan, field.start_ordinal)
        || !paragraph_in_bookmark_scope(
          scan,
          field.start_ordinal,
          span.spec.bookmark_name.as_deref(),
        )
      {
        continue;
      }
      let Some(tc) = TcSpec::parse(&field.instruction) else {
        continue;
      };
      if span
        .spec
        .tc_identifier
        .as_deref()
        .is_some_and(|identifier| tc.identifier.as_deref() != Some(identifier))
        || !span.spec.tc_levels.contains(tc.level)
      {
        continue;
      }
      let text = normalize_toc_entry_text(tc.text, &span.spec);
      if text.is_empty() {
        continue;
      }
      entries.push(TocEntrySource {
        source_ordinal: field.start_ordinal,
        level: tc.level,
        text,
        omit_page_number: tc.omit_page_number || span.spec.suppress_page_number(tc.level),
        chapter_prefix: chapter_prefix_for_source(sections, scan, field.start_ordinal, &span.spec),
      });
    }
  }

  if let Some(caption_identifier) = span.spec.caption_identifier.as_deref() {
    for field in &scan.fields {
      if ordinal_inside_any_toc(scan, field.start_ordinal)
        || !paragraph_in_bookmark_scope(
          scan,
          field.start_ordinal,
          span.spec.bookmark_name.as_deref(),
        )
        || seq_identifier(&field.instruction).as_deref() != Some(caption_identifier)
      {
        continue;
      }
      let Some(paragraph) = paragraph(sections, scan, field.start_ordinal) else {
        continue;
      };
      let Some(mut text) = paragraph_source_text(paragraph) else {
        continue;
      };
      if span.spec.omit_caption_label_and_number {
        text = strip_caption_label_and_number(text, caption_identifier);
      }
      let text = normalize_toc_entry_text(text, &span.spec);
      if text.is_empty() {
        continue;
      }
      entries.push(TocEntrySource {
        source_ordinal: field.start_ordinal,
        level: 1,
        text,
        omit_page_number: span.spec.suppress_page_number(1),
        chapter_prefix: chapter_prefix_for_source(sections, scan, field.start_ordinal, &span.spec),
      });
    }
  }

  entries.sort_by(|left, right| {
    left
      .source_ordinal
      .cmp(&right.source_ordinal)
      .then(left.level.cmp(&right.level))
  });
  entries.dedup_by(|left, right| {
    left.source_ordinal == right.source_ordinal
      && left.level == right.level
      && left.text == right.text
  });
  entries
}

fn paragraph_toc_level(paragraph: &Paragraph, spec: &TocSpec) -> Option<u8> {
  for (style_name, level) in &spec.custom_style_levels {
    if paragraph
      .style_ref_keys
      .iter()
      .any(|key| key.eq_ignore_ascii_case(style_name))
    {
      return Some(*level);
    }
  }

  let style_outline_level = paragraph
    .format
    .style_outline_level
    .and_then(|level| level.checked_add(1))
    .filter(|level| (1..=9).contains(level));
  if let Some(level) = style_outline_level
    && spec
      .outline_levels
      .is_some_and(|range| range.contains(level))
  {
    return Some(level);
  }

  let applied_outline_level = paragraph
    .format
    .outline_level
    .and_then(|level| level.checked_add(1))
    .filter(|level| (1..=9).contains(level));
  if let Some(level) = applied_outline_level
    && spec.use_applied_outline_level
  {
    return Some(level);
  }
  None
}

fn paragraph_source_text(paragraph: &Paragraph) -> Option<String> {
  if let Some(text) = paragraph.style_ref_text.as_deref()
    && !text.trim().is_empty()
  {
    return Some(text.to_string());
  }
  let mut text = String::new();
  if let Some(label) = paragraph.list_label.as_deref() {
    text.push_str(label);
  }
  for inline in &paragraph.inlines {
    match inline {
      InlineItem::Text(run) if !run.style.hidden => text.push_str(&run.text),
      InlineItem::Ruby(ruby) => {
        for run in &ruby.base {
          if !run.style.hidden {
            text.push_str(&run.text);
          }
        }
      }
      _ => {}
    }
  }
  (!text.trim().is_empty()).then_some(text)
}

fn normalize_toc_entry_text(text: String, spec: &TocSpec) -> String {
  let text = text
    .chars()
    .map(|character| match character {
      '\t' if !spec.preserve_tabs => ' ',
      '\r' | '\n' if !spec.preserve_newlines => ' ',
      character => character,
    })
    .collect::<String>();
  if spec.preserve_tabs || spec.preserve_newlines {
    text.trim().to_string()
  } else {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
  }
}

fn strip_caption_label_and_number(mut text: String, identifier: &str) -> String {
  let trimmed = text.trim_start();
  if trimmed
    .get(..identifier.len())
    .is_some_and(|prefix| prefix.eq_ignore_ascii_case(identifier))
  {
    text = trimmed[identifier.len()..].trim_start().to_string();
  }
  let first_end = text.find(char::is_whitespace).unwrap_or(text.len());
  let first = &text[..first_end];
  if first.chars().any(|character| character.is_ascii_digit()) {
    text = text[first_end..].trim_start().to_string();
    text = text
      .trim_start_matches([':', '.', '-', '–', '—'])
      .trim_start()
      .to_string();
  }
  text
}

fn chapter_prefix_for_source(
  sections: &[ImportedSection],
  scan: &StoryScan,
  source_ordinal: usize,
  spec: &TocSpec,
) -> Option<String> {
  let identifier = spec.chapter_sequence.as_deref()?;
  let has_sequence = scan.fields.iter().any(|field| {
    field.start_ordinal == source_ordinal
      && seq_identifier(&field.instruction).as_deref() == Some(identifier)
  });
  if !has_sequence {
    return None;
  }
  paragraph(sections, scan, source_ordinal)
    .and_then(paragraph_source_text)
    .and_then(|text| {
      text
        .split_whitespace()
        .find(|token| token.chars().any(|character| character.is_ascii_digit()))
        .map(|token| {
          token
            .trim_matches(|character: char| !character.is_alphanumeric())
            .to_string()
        })
    })
    .filter(|value| !value.is_empty())
}

fn ordinal_inside_any_toc(scan: &StoryScan, ordinal: usize) -> bool {
  scan
    .toc_spans
    .iter()
    .any(|span| (span.start_ordinal..=span.end_ordinal).contains(&ordinal))
}

fn paragraph_in_bookmark_scope(
  scan: &StoryScan,
  ordinal: usize,
  bookmark_name: Option<&str>,
) -> bool {
  bookmark_name.is_none_or(|bookmark_name| {
    scan
      .paragraphs
      .get(ordinal)
      .is_some_and(|paragraph| paragraph.bookmark_scopes.contains(bookmark_name))
  })
}

fn preferred_source_bookmark(scan: &StoryScan, ordinal: usize) -> Option<&str> {
  let bookmarks = &scan.paragraphs.get(ordinal)?.source_bookmarks;
  bookmarks
    .iter()
    .find(|name| name.starts_with("_Toc"))
    .or_else(|| bookmarks.iter().find(|name| name.as_str() != "_GoBack"))
    .map(String::as_str)
}

fn paragraph<'a>(
  sections: &'a [ImportedSection],
  scan: &StoryScan,
  ordinal: usize,
) -> Option<&'a Paragraph> {
  let address = &scan.paragraphs.get(ordinal)?.address;
  let Block::Paragraph(paragraph) =
    story_block(&sections.get(address.section_index)?.blocks, &address.path)?
  else {
    return None;
  };
  Some(paragraph)
}

fn paragraph_mut<'a>(
  sections: &'a mut [ImportedSection],
  scan: &StoryScan,
  ordinal: usize,
) -> Option<&'a mut Paragraph> {
  let address = &scan.paragraphs.get(ordinal)?.address;
  let Block::Paragraph(paragraph) = story_block_mut(
    &mut sections.get_mut(address.section_index)?.blocks,
    &address.path,
  )?
  else {
    return None;
  };
  Some(paragraph)
}

fn story_block<'a>(blocks: &'a [Block], path: &[StoryPathStep]) -> Option<&'a Block> {
  let (StoryPathStep::Block(block_index), remaining) = path.split_first()? else {
    return None;
  };
  let block = blocks.get(*block_index)?;
  if remaining.is_empty() {
    return Some(block);
  }
  let (
    StoryPathStep::TableCell {
      row_index,
      cell_index,
    },
    remaining,
  ) = remaining.split_first()?
  else {
    return None;
  };
  let Block::Table(table) = block else {
    return None;
  };
  let cell = table.rows.get(*row_index)?.cells.get(*cell_index)?;
  story_block(&cell.blocks, remaining)
}

fn story_block_mut<'a>(blocks: &'a mut [Block], path: &[StoryPathStep]) -> Option<&'a mut Block> {
  let (StoryPathStep::Block(block_index), remaining) = path.split_first()? else {
    return None;
  };
  let block = blocks.get_mut(*block_index)?;
  if remaining.is_empty() {
    return Some(block);
  }
  let (
    StoryPathStep::TableCell {
      row_index,
      cell_index,
    },
    remaining,
  ) = remaining.split_first()?
  else {
    return None;
  };
  let Block::Table(table) = block else {
    return None;
  };
  let cell = table.rows.get_mut(*row_index)?.cells.get_mut(*cell_index)?;
  story_block_mut(&mut cell.blocks, remaining)
}

fn story_block_container_mut<'a>(
  blocks: &'a mut Vec<Block>,
  path: &[StoryPathStep],
) -> Option<&'a mut Vec<Block>> {
  if path.is_empty() {
    return Some(blocks);
  }
  let [
    StoryPathStep::Block(block_index),
    StoryPathStep::TableCell {
      row_index,
      cell_index,
    },
    remaining @ ..,
  ] = path
  else {
    return None;
  };
  let Block::Table(table) = blocks.get_mut(*block_index)? else {
    return None;
  };
  let cell = table.rows.get_mut(*row_index)?.cells.get_mut(*cell_index)?;
  story_block_container_mut(&mut cell.blocks, remaining)
}

fn cached_toc_templates(
  sections: &[ImportedSection],
  scan: &StoryScan,
  span: &TocSpan,
  styles: &StylesCatalog,
) -> HashMap<u8, Paragraph> {
  let mut templates = HashMap::new();
  for ordinal in span.start_ordinal..=span.end_ordinal {
    let Some(paragraph) = paragraph(sections, scan, ordinal) else {
      continue;
    };
    let Some(level) = paragraph
      .format
      .style_id
      .as_deref()
      .and_then(|style_id| toc_level_for_style(styles, style_id))
    else {
      continue;
    };
    templates.entry(level).or_insert_with(|| paragraph.clone());
  }
  templates
}

fn toc_level_for_style(styles: &StylesCatalog, style_id: &str) -> Option<u8> {
  toc_level_from_name(style_id).or_else(|| {
    styles
      .styles
      .get(style_id)
      .and_then(|entry| entry.name.as_deref())
      .and_then(toc_level_from_name)
  })
}

fn toc_level_from_name(name: &str) -> Option<u8> {
  let normalized = name
    .chars()
    .filter(|character| !character.is_whitespace() && *character != '-' && *character != '_')
    .flat_map(char::to_lowercase)
    .collect::<String>();
  let suffix = normalized
    .strip_prefix("toc")
    .or_else(|| normalized.strip_prefix("contents"))?;
  let level = suffix.parse::<u8>().ok()?;
  (1..=9).contains(&level).then_some(level)
}

fn toc_style_id(styles: &StylesCatalog, level: u8) -> String {
  for candidate in [format!("TOC{level}"), format!("Contents{level}")] {
    if styles.styles.contains_key(&candidate) {
      return candidate;
    }
  }
  let mut candidates = styles
    .styles
    .iter()
    .filter_map(|(style_id, entry)| {
      (toc_level_from_name(style_id) == Some(level)
        || entry.name.as_deref().and_then(toc_level_from_name) == Some(level))
      .then_some(style_id)
    })
    .cloned()
    .collect::<Vec<_>>();
  candidates.sort();
  candidates
    .into_iter()
    .next()
    .unwrap_or_else(|| format!("TOC{level}"))
}

fn empty_toc_paragraph(styles: &StylesCatalog, level: u8) -> Paragraph {
  let style_id = toc_style_id(styles, level);
  let style_exists = styles.styles.contains_key(&style_id);
  let mut format = styles.paragraph_format_with_base(Some(&style_id), ParagraphFormat::default());
  format.style_id = Some(Arc::<str>::from(style_id.as_str()));
  if !style_exists {
    // Word's latent TOC styles indent successive levels by 180 twips.
    format.indent_left_pt = f32::from(level.saturating_sub(1)) * 9.0;
    format.indent_left_set = true;
    format.spacing_after_pt = 0.0;
    format.spacing_after_set = true;
  }
  let mut base_style = styles.run_style_with_base(
    Some(&style_id),
    TextStyle::default(),
    RunStyleOverrides::default(),
  );
  base_style.line_vertical_alignment = format.line_vertical_alignment.unwrap_or_default();
  Paragraph {
    inlines: Vec::new(),
    field_events: Vec::new(),
    footnote_reference_ids: Vec::new(),
    endnote_reference_ids: Vec::new(),
    starts_after_last_rendered_page_break: false,
    base_style,
    #[cfg(test)]
    runs: Vec::new(),
    format: Box::new(format),
    style_ref_keys: styles.style_ref_keys(&style_id),
    style_ref_text: None,
    style_ref_numbering_text: None,
    list_label: None,
    list_label_image: None,
    list_label_style: TextStyle::default(),
    list_label_hyperlink_url: None,
    list_label_tab_stop_pt: None,
  }
}

fn build_toc_entry_paragraph(
  entry: &TocEntrySource,
  bookmark_name: &str,
  spec: &TocSpec,
  template: Option<&Paragraph>,
  styles: &StylesCatalog,
  page: PageSetup,
) -> Paragraph {
  let mut paragraph = template
    .cloned()
    .unwrap_or_else(|| empty_toc_paragraph(styles, entry.level));
  paragraph.inlines.clear();
  paragraph.field_events.clear();
  paragraph.footnote_reference_ids.clear();
  paragraph.endnote_reference_ids.clear();
  paragraph.starts_after_last_rendered_page_break = false;
  paragraph.list_label = None;
  paragraph.list_label_hyperlink_url = None;
  paragraph.list_label_tab_stop_pt = None;
  paragraph.style_ref_text = None;
  paragraph.style_ref_numbering_text = None;
  #[cfg(test)]
  paragraph.runs.clear();

  let hyperlink_url = spec
    .hyperlinks
    .then(|| format!("ooxmlsdk-pdf:bookmark:{bookmark_name}"));
  // TOC \h creates a link target without applying the ordinary blue
  // Hyperlink character appearance in Word's fixed output.
  let run_style = paragraph.base_style.clone();
  paragraph.inlines.push(InlineItem::Text(TextRun {
    text: entry.text.clone(),
    style: run_style.clone(),
    hyperlink_url: hyperlink_url.clone(),
    dynamic_field: None,
    style_ref_keys: Vec::new(),
    style_ref_text: None,
    style_ref_numbering_text: None,
    preserve_text_portion: false,
  }));

  if !entry.omit_page_number {
    if let Some(separator) = spec.page_separator {
      paragraph.inlines.push(InlineItem::Text(TextRun {
        text: separator.to_string(),
        style: run_style.clone(),
        hyperlink_url: hyperlink_url.clone(),
        dynamic_field: None,
        style_ref_keys: Vec::new(),
        style_ref_text: None,
        style_ref_numbering_text: None,
        preserve_text_portion: false,
      }));
    } else {
      ensure_toc_page_tab_stop(&mut paragraph, page);
      paragraph.inlines.push(InlineItem::Text(TextRun {
        text: "\t".to_string(),
        style: run_style.clone(),
        hyperlink_url: hyperlink_url.clone(),
        dynamic_field: None,
        style_ref_keys: Vec::new(),
        style_ref_text: None,
        style_ref_numbering_text: None,
        preserve_text_portion: false,
      }));
    }
    if let Some(prefix) = entry.chapter_prefix.as_deref() {
      paragraph.inlines.push(InlineItem::Text(TextRun {
        text: format!("{prefix}{}", spec.chapter_separator),
        style: run_style.clone(),
        hyperlink_url: hyperlink_url.clone(),
        dynamic_field: None,
        style_ref_keys: Vec::new(),
        style_ref_text: None,
        style_ref_numbering_text: None,
        preserve_text_portion: false,
      }));
    }
    paragraph.inlines.push(InlineItem::Text(TextRun {
      text: "1".to_string(),
      style: run_style,
      hyperlink_url,
      dynamic_field: Some(DynamicFieldKind::PageRef {
        bookmark_name: Arc::<str>::from(bookmark_name),
        number_format: FieldNumberFormat::PageStyle,
        relative_position: false,
      }),
      style_ref_keys: Vec::new(),
      style_ref_text: None,
      style_ref_numbering_text: None,
      preserve_text_portion: false,
    }));
  }
  paragraph
}

fn ensure_toc_page_tab_stop(paragraph: &mut Paragraph, page: PageSetup) {
  let content_width = (page.width_pt - page.margin_left_pt - page.margin_right_pt).max(1.0);
  let position_pt =
    (content_width - paragraph.format.indent_left_pt - paragraph.format.indent_right_pt).max(1.0);
  if let Some(stop) = paragraph
    .format
    .tab_stops
    .iter_mut()
    .find(|stop| matches!(stop.alignment, TabStopAlignment::Right))
  {
    stop.position_pt = position_pt;
    stop.leader = TabLeader::Dot;
  } else {
    paragraph.format.tab_stops.push(TabStop {
      position_pt,
      alignment: TabStopAlignment::Right,
      leader: TabLeader::Dot,
    });
    paragraph
      .format
      .tab_stops
      .sort_by(|left, right| left.position_pt.total_cmp(&right.position_pt));
  }
  paragraph.format.tab_stops_set = true;
}

fn build_empty_toc_result(
  _spec: &TocSpec,
  template: Option<&Paragraph>,
  styles: &StylesCatalog,
  _page: PageSetup,
  ui_language: Option<&str>,
) -> Paragraph {
  let mut paragraph = template
    .cloned()
    .unwrap_or_else(|| empty_toc_paragraph(styles, 1));
  paragraph.inlines.clear();
  paragraph.field_events.clear();
  paragraph.footnote_reference_ids.clear();
  paragraph.endnote_reference_ids.clear();
  paragraph.list_label = None;
  paragraph.list_label_hyperlink_url = None;
  let text = localized_field_message(FieldMessage::EmptyTableOfContents, ui_language);
  paragraph.inlines.push(InlineItem::Text(TextRun {
    text,
    style: paragraph.base_style.clone(),
    hyperlink_url: None,
    dynamic_field: None,
    style_ref_keys: Vec::new(),
    style_ref_text: None,
    style_ref_numbering_text: None,
    preserve_text_portion: false,
  }));
  paragraph
}

fn replace_toc_span(
  sections: &mut [ImportedSection],
  scan: &StoryScan,
  replacement: TocReplacement,
) {
  replace_story_span(
    sections,
    scan,
    StoryReplacement {
      start_ordinal: replacement.span.start_ordinal,
      end_ordinal: replacement.span.end_ordinal,
      blocks: replacement.blocks,
    },
  );
}

fn replace_story_span(
  sections: &mut [ImportedSection],
  scan: &StoryScan,
  replacement: StoryReplacement,
) {
  let Some(start) = scan.paragraphs.get(replacement.start_ordinal) else {
    return;
  };
  let Some(end) = scan.paragraphs.get(replacement.end_ordinal) else {
    return;
  };
  if start.address.section_index == end.address.section_index
    && start.address.container_path() == end.address.container_path()
  {
    let (Some(start_block_index), Some(end_block_index)) =
      (start.address.block_index(), end.address.block_index())
    else {
      return;
    };
    let Some(section) = sections.get_mut(start.address.section_index) else {
      return;
    };
    let Some(blocks) =
      story_block_container_mut(&mut section.blocks, start.address.container_path())
    else {
      return;
    };
    if start_block_index <= end_block_index && end_block_index < blocks.len() {
      blocks.splice(start_block_index..=end_block_index, replacement.blocks);
    }
    return;
  }

  // A complex field may legally cross a section boundary. Preserve all
  // non-field content on either side, put the rebuilt result in the starting
  // section, and remove only the intervening field-result blocks.
  if !start.address.is_top_level() || !end.address.is_top_level() {
    // A field crossing table-cell or floating-story boundaries is malformed.
    // Keep its cached result rather than deleting unrelated table structure.
    return;
  }
  let (Some(start_block_index), Some(end_block_index)) =
    (start.address.block_index(), end.address.block_index())
  else {
    return;
  };
  if let Some(end_section) = sections.get_mut(end.address.section_index)
    && end_block_index < end_section.blocks.len()
  {
    end_section.blocks.drain(..=end_block_index);
  }
  for section_index in start.address.section_index + 1..end.address.section_index {
    if let Some(section) = sections.get_mut(section_index) {
      section.blocks.clear();
    }
  }
  if let Some(start_section) = sections.get_mut(start.address.section_index)
    && start_block_index < start_section.blocks.len()
  {
    start_section.blocks.truncate(start_block_index);
    start_section.blocks.extend(replacement.blocks);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn refresh_tables_of_contents(
    sections: &mut [ImportedSection],
    styles: &StylesCatalog,
    update_fields_on_open: bool,
    ui_language: Option<&str>,
  ) {
    super::refresh_tables_of_contents(
      sections,
      styles,
      update_fields_on_open,
      ui_language,
      &HashSet::new(),
    );
  }

  fn test_paragraph(text: &str) -> Paragraph {
    let styles = StylesCatalog::default();
    let mut paragraph = empty_toc_paragraph(&styles, 1);
    paragraph.inlines.push(InlineItem::Text(TextRun {
      text: text.to_string(),
      style: paragraph.base_style.clone(),
      hyperlink_url: None,
      dynamic_field: None,
      style_ref_keys: Vec::new(),
      style_ref_text: None,
      style_ref_numbering_text: None,
      preserve_text_portion: false,
    }));
    paragraph.style_ref_text = (!text.is_empty()).then(|| Arc::<str>::from(text));
    paragraph
  }

  fn test_table(blocks: Vec<Block>) -> Table {
    Table {
      column_widths_pt: vec![300.0],
      preferred_width_pt: None,
      preferred_width_pct: None,
      layout: TableLayoutMode::AutoFit,
      indent_left_pt: 0.0,
      alignment: TableAlignment::Left,
      right_to_left: false,
      align_leading_cell_content: false,
      in_header_footer: false,
      placement: None,
      allow_overlap: true,
      split_allowed: true,
      following_text_flow: false,
      explicit_no_repeat_header: false,
      page_break_before: false,
      starts_after_last_rendered_page_break: false,
      borders: None,
      cell_spacing_pt: 0.0,
      rows: vec![TableRow {
        cells: vec![TableCell {
          blocks,
          shading: None,
          borders: CellBordersModel::default(),
          border_suppressions: CellBorderSuppressions::default(),
          margins: CellMargins::default(),
          preferred_width_pt: None,
          preferred_width_pct: None,
          grid_span: 1,
          vertical_merge_continue: false,
          no_wrap: false,
          fit_text: false,
          hide_end_mark: false,
          vertical_alignment: TableCellVerticalAlignment::Top,
          text_rotation_deg: None,
        }],
        height_pt: None,
        exact_height: false,
        repeat_header: false,
        keep_with_next: false,
        cant_split: false,
        cell_spacing_pt: None,
        grid_before: 0,
        grid_after: 0,
        width_before_pt: None,
        width_after_pt: None,
        layout: None,
        borders: None,
        spacing_shading: None,
        redline_color: None,
      }],
    }
  }

  #[test]
  fn toc_parser_covers_all_ecma_switch_families_and_word_deviations() {
    let spec = TocSpec::parse(
      r#" TOC \o "2-4" \u \f A \l "2-3" \h \n "3-4" \p ":-ignored" \s chapter \d "::" \b scope \t "Appendix,1,Side,3" \w \x \z "#,
    )
    .unwrap();
    assert_eq!(
      spec.outline_levels,
      Some(TocLevelRange { start: 2, end: 4 })
    );
    assert!(spec.use_applied_outline_level);
    assert!(spec.tc_entries);
    assert_eq!(spec.tc_identifier.as_deref(), Some("A"));
    assert_eq!(spec.tc_levels, TocLevelRange { start: 2, end: 3 });
    assert!(spec.hyperlinks);
    assert_eq!(
      spec.no_page_number_levels,
      Some(TocLevelRange { start: 3, end: 4 })
    );
    assert_eq!(spec.page_separator, Some(':'));
    assert_eq!(spec.chapter_sequence.as_deref(), Some("chapter"));
    assert_eq!(spec.chapter_separator, "::");
    assert_eq!(spec.bookmark_name.as_deref(), Some("scope"));
    assert_eq!(
      spec.custom_style_levels,
      vec![("Appendix".to_string(), 1), ("Side".to_string(), 3)]
    );
    assert!(spec.preserve_tabs);
    assert!(spec.preserve_newlines);
    assert!(spec.hide_page_numbers_in_web_layout);
  }

  #[test]
  fn toc_parser_defaults_to_outline_and_later_f_supersedes_l() {
    let default = TocSpec::parse("TOC").unwrap();
    assert_eq!(default.outline_levels, Some(TocLevelRange::ALL));

    let spec = TocSpec::parse(r#"TOC \l "3-4" \f C"#).unwrap();
    assert_eq!(spec.tc_levels, TocLevelRange::ALL);
    assert_eq!(spec.tc_identifier.as_deref(), Some("C"));
    assert_eq!(spec.outline_levels, None);
  }

  #[test]
  fn toc_parser_accepts_legacy_slash_switches() {
    let spec = TocSpec::parse("TOC /l 1-3 /f").unwrap();
    assert!(spec.tc_entries);
    assert_eq!(spec.tc_levels, TocLevelRange::ALL);
  }

  #[test]
  fn clean_empty_toc_result_is_preserved_until_an_update_is_requested() {
    let mut first_cached = test_paragraph("");
    first_cached.field_events = vec![
      ParagraphFieldEvent::Begin {
        locked: false,
        dirty: false,
      },
      ParagraphFieldEvent::Instruction(r#" TOC \o "1-3" \h "#.to_string()),
      ParagraphFieldEvent::Separate,
    ];
    let mut second_cached = test_paragraph("");
    second_cached.field_events = vec![ParagraphFieldEvent::End];
    let mut heading = test_paragraph("Current heading");
    heading.format.style_outline_level = Some(0);
    heading.format.outline_level = Some(0);

    let sections = vec![default_section(vec![
      Block::paragraph(first_cached),
      Block::paragraph(second_cached),
      Block::paragraph(heading),
    ])];
    let mut cached = sections.clone();
    refresh_tables_of_contents(&mut cached, &StylesCatalog::default(), false, Some("en-US"));

    assert_eq!(cached[0].blocks.len(), 3);
    assert!(matches!(
      &cached[0].blocks[2],
      Block::Paragraph(source)
        if !matches!(source.inlines.first(), Some(InlineItem::BookmarkStart(_)))
    ));

    let mut requested = sections;
    refresh_tables_of_contents(
      &mut requested,
      &StylesCatalog::default(),
      true,
      Some("en-US"),
    );

    assert_eq!(requested[0].blocks.len(), 2);
    assert!(matches!(
      &requested[0].blocks[0],
      Block::Paragraph(entry)
        if paragraph_source_text(entry)
          .is_some_and(|text| text.starts_with("Current heading"))
    ));
    assert!(matches!(
      &requested[0].blocks[1],
      Block::Paragraph(source)
        if matches!(source.inlines.first(), Some(InlineItem::BookmarkStart(_)))
    ));
  }

  #[test]
  fn missing_ref_replaces_a_cross_table_cached_result_without_dropping_neighbors() {
    let before = test_paragraph("before");
    let mut field_start = test_paragraph("");
    field_start.field_events = vec![
      ParagraphFieldEvent::Begin {
        locked: false,
        dirty: false,
      },
      ParagraphFieldEvent::Instruction(r#" REF MissingBookmark \h "#.to_string()),
      ParagraphFieldEvent::Separate,
    ];
    let cached_table = test_table(vec![Block::paragraph(test_paragraph("stale table"))]);
    let mut field_end = test_paragraph("");
    field_end.field_events = vec![ParagraphFieldEvent::End];
    let after = test_paragraph("after");
    let mut sections = vec![default_section(vec![
      Block::paragraph(before),
      Block::paragraph(field_start),
      Block::Table(cached_table),
      Block::paragraph(field_end),
      Block::paragraph(after),
    ])];

    refresh_tables_of_contents(
      &mut sections,
      &StylesCatalog::default(),
      false,
      Some("zh-CN"),
    );

    assert_eq!(sections[0].blocks.len(), 3);
    assert!(matches!(
      &sections[0].blocks[0],
      Block::Paragraph(paragraph)
        if paragraph_source_text(paragraph).as_deref() == Some("before")
    ));
    assert!(matches!(
      &sections[0].blocks[1],
      Block::Paragraph(paragraph)
        if paragraph_source_text(paragraph).as_deref() == Some("错误!未找到引用源。")
    ));
    assert!(matches!(
      &sections[0].blocks[2],
      Block::Paragraph(paragraph)
        if paragraph_source_text(paragraph).as_deref() == Some("after")
    ));
  }

  #[test]
  fn missing_ref_replaces_all_cached_paragraphs_inside_one_table_cell() {
    let mut field_start = test_paragraph("stale first");
    field_start.field_events = vec![
      ParagraphFieldEvent::Begin {
        locked: false,
        dirty: false,
      },
      ParagraphFieldEvent::Instruction(" REF MissingBookmark ".to_string()),
      ParagraphFieldEvent::Separate,
      ParagraphFieldEvent::Content,
    ];
    let mut field_end = test_paragraph("");
    field_end.field_events = vec![ParagraphFieldEvent::End];
    let table = test_table(vec![
      Block::paragraph(field_start),
      Block::paragraph(test_paragraph("stale second")),
      Block::paragraph(field_end),
    ]);
    let mut sections = vec![default_section(vec![Block::Table(table)])];

    refresh_tables_of_contents(
      &mut sections,
      &StylesCatalog::default(),
      false,
      Some("zh-CN"),
    );

    let Block::Table(table) = &sections[0].blocks[0] else {
      panic!("expected table");
    };
    assert_eq!(table.rows[0].cells[0].blocks.len(), 1);
    assert!(matches!(
      &table.rows[0].cells[0].blocks[0],
      Block::Paragraph(paragraph)
        if paragraph_source_text(paragraph).as_deref() == Some("错误!未找到引用源。")
    ));
  }

  #[test]
  fn missing_reference_refresh_preserves_locked_and_embedded_fields() {
    let mut locked = test_paragraph("locked cache");
    locked.field_events = vec![
      ParagraphFieldEvent::Begin {
        locked: true,
        dirty: true,
      },
      ParagraphFieldEvent::Instruction("REF MissingBookmark".to_string()),
      ParagraphFieldEvent::Separate,
      ParagraphFieldEvent::Content,
      ParagraphFieldEvent::End,
    ];
    let mut embedded = test_paragraph("prefix stale suffix");
    embedded.field_events = vec![
      ParagraphFieldEvent::Content,
      ParagraphFieldEvent::Begin {
        locked: false,
        dirty: true,
      },
      ParagraphFieldEvent::Instruction("REF MissingBookmark".to_string()),
      ParagraphFieldEvent::Separate,
      ParagraphFieldEvent::Content,
      ParagraphFieldEvent::End,
      ParagraphFieldEvent::Content,
    ];
    let mut sections = vec![default_section(vec![
      Block::paragraph(locked),
      Block::paragraph(embedded),
    ])];

    refresh_tables_of_contents(
      &mut sections,
      &StylesCatalog::default(),
      true,
      Some("zh-CN"),
    );

    assert_eq!(sections[0].blocks.len(), 2);
    assert!(matches!(
      &sections[0].blocks[0],
      Block::Paragraph(paragraph)
        if paragraph_source_text(paragraph).as_deref() == Some("locked cache")
    ));
    assert!(matches!(
      &sections[0].blocks[1],
      Block::Paragraph(paragraph)
        if paragraph_source_text(paragraph).as_deref() == Some("prefix stale suffix")
    ));
  }

  #[test]
  fn body_level_bookmark_keeps_a_valid_ref_cached_result() {
    let mut cached = test_paragraph("Text1");
    cached.field_events = vec![
      ParagraphFieldEvent::Begin {
        locked: false,
        dirty: false,
      },
      ParagraphFieldEvent::Instruction(r#" REF BM1 \* MERGEFORMAT "#.to_string()),
      ParagraphFieldEvent::Separate,
      ParagraphFieldEvent::Content,
      ParagraphFieldEvent::End,
    ];
    let mut sections = vec![default_section(vec![Block::paragraph(cached)])];

    super::refresh_tables_of_contents(
      &mut sections,
      &StylesCatalog::default(),
      false,
      Some("zh-CN"),
      &HashSet::from(["BM1".to_string()]),
    );

    assert!(matches!(
      &sections[0].blocks[0],
      Block::Paragraph(paragraph)
        if paragraph_source_text(paragraph).as_deref() == Some("Text1")
    ));
  }

  #[test]
  fn missing_pageref_is_resolved_before_layout_without_consuming_an_outer_tab() {
    let page_ref_run = |text: &str, bookmark_name: &str| {
      InlineItem::Text(TextRun {
        text: text.to_string(),
        style: TextStyle::default(),
        hyperlink_url: None,
        dynamic_field: Some(DynamicFieldKind::PageRef {
          bookmark_name: Arc::<str>::from(bookmark_name),
          number_format: FieldNumberFormat::Decimal,
          relative_position: false,
        }),
        style_ref_keys: Vec::new(),
        style_ref_text: None,
        style_ref_numbering_text: None,
        preserve_text_portion: false,
      })
    };

    // tdf64531 stores its tab inside the missing PAGEREF result. Word replaces
    // that complete result, so the generated diagnostic must not retain it.
    let mut field_owned_tab = test_paragraph("");
    field_owned_tab.inlines.clear();
    field_owned_tab
      .inlines
      .push(page_ref_run("\t2", "MissingFieldOwnedTab"));

    // fdo78654 has the TOC tab before PAGEREF. That tab is paragraph content
    // and must survive while only the cached field result is replaced.
    let mut outer_tab = test_paragraph("Heading\t");
    outer_tab
      .inlines
      .push(page_ref_run("48", "MissingAfterOuterTab"));

    let mut valid = test_paragraph("");
    valid.inlines.clear();
    valid.inlines.push(page_ref_run("7", "present"));
    let mut target = test_paragraph("Target");
    target
      .inlines
      .insert(0, InlineItem::BookmarkStart("Present".to_string()));
    target.field_events = vec![
      ParagraphFieldEvent::BookmarkStart {
        id: "1".to_string(),
        name: "Present".to_string(),
      },
      ParagraphFieldEvent::Content,
      ParagraphFieldEvent::BookmarkEnd {
        id: "1".to_string(),
      },
    ];

    let mut sections = vec![default_section(vec![
      Block::paragraph(field_owned_tab),
      Block::paragraph(outer_tab),
      Block::paragraph(valid),
      Block::paragraph(target),
    ])];

    refresh_tables_of_contents(
      &mut sections,
      &StylesCatalog::default(),
      false,
      Some("zh-CN"),
    );

    let Block::Paragraph(field_owned_tab) = &sections[0].blocks[0] else {
      panic!("expected field-owned-tab paragraph");
    };
    let InlineItem::Text(result) = &field_owned_tab.inlines[0] else {
      panic!("expected resolved PAGEREF result");
    };
    assert_eq!(result.text, "错误!未定义书签。");
    assert_eq!(
      result.style.east_asia_font_family.as_deref(),
      Some("SimSun")
    );
    assert!(result.dynamic_field.is_none());

    let Block::Paragraph(outer_tab) = &sections[0].blocks[1] else {
      panic!("expected outer-tab paragraph");
    };
    assert!(matches!(
      &outer_tab.inlines[0],
      InlineItem::Text(run) if run.text == "Heading\t"
    ));
    assert!(matches!(
      &outer_tab.inlines[1],
      InlineItem::Text(run)
        if run.text == "错误!未定义书签。" && run.dynamic_field.is_none()
    ));

    let Block::Paragraph(valid) = &sections[0].blocks[2] else {
      panic!("expected valid PAGEREF paragraph");
    };
    assert!(matches!(
      &valid.inlines[0],
      InlineItem::Text(run)
        if run.text == "7"
          && matches!(
            run.dynamic_field.as_ref(),
            Some(DynamicFieldKind::PageRef { .. })
          )
    ));
  }

  #[test]
  fn dirty_cross_paragraph_toc_is_rebuilt_with_bookmark_and_pageref() {
    let mut first_cached = test_paragraph("stale first");
    first_cached.field_events = vec![
      ParagraphFieldEvent::Begin {
        locked: false,
        dirty: true,
      },
      ParagraphFieldEvent::Instruction(r#" TOC \o "1-2" \h "#.to_string()),
      ParagraphFieldEvent::Separate,
    ];
    let mut second_cached = test_paragraph("stale second");
    second_cached.field_events = vec![ParagraphFieldEvent::End];
    let mut heading = test_paragraph("Current heading");
    heading.format.style_outline_level = Some(0);
    heading.format.outline_level = Some(0);

    let mut sections = vec![default_section(vec![
      Block::paragraph(first_cached),
      Block::paragraph(second_cached),
      Block::paragraph(heading),
    ])];
    refresh_tables_of_contents(
      &mut sections,
      &StylesCatalog::default(),
      false,
      Some("en-US"),
    );

    assert_eq!(sections[0].blocks.len(), 2);
    let Block::Paragraph(entry) = &sections[0].blocks[0] else {
      panic!("expected rebuilt TOC entry");
    };
    assert!(matches!(
      entry.inlines.first(),
      Some(InlineItem::Text(run)) if run.text == "Current heading"
        && run.hyperlink_url.as_deref().is_some_and(|url| url.starts_with("ooxmlsdk-pdf:bookmark:"))
    ));
    let target = entry
      .inlines
      .iter()
      .find_map(|inline| match inline {
        InlineItem::Text(TextRun {
          dynamic_field: Some(DynamicFieldKind::PageRef { bookmark_name, .. }),
          ..
        }) => Some(bookmark_name.clone()),
        _ => None,
      })
      .expect("generated PAGEREF");
    assert!(entry.format.tab_stops.iter().any(|stop| {
      matches!(stop.alignment, TabStopAlignment::Right) && matches!(stop.leader, TabLeader::Dot)
    }));

    let Block::Paragraph(source) = &sections[0].blocks[1] else {
      panic!("expected source heading");
    };
    assert!(matches!(
      source.inlines.first(),
      Some(InlineItem::BookmarkStart(name)) if name == target.as_ref()
    ));
  }

  #[test]
  fn main_story_toc_includes_heading_inside_nested_table_cell() {
    let mut cached = test_paragraph("stale");
    cached.field_events = vec![
      ParagraphFieldEvent::Begin {
        locked: false,
        dirty: true,
      },
      ParagraphFieldEvent::Instruction(r#"TOC \o "1-3" \h"#.to_string()),
      ParagraphFieldEvent::Separate,
      ParagraphFieldEvent::End,
    ];
    let mut heading = test_paragraph("Table heading");
    heading.format.style_outline_level = Some(0);
    heading.format.outline_level = Some(0);
    let nested = test_table(vec![Block::paragraph(heading)]);
    let table = test_table(vec![Block::Table(nested)]);
    let mut sections = vec![default_section(vec![
      Block::paragraph(cached),
      Block::Table(table),
    ])];

    refresh_tables_of_contents(
      &mut sections,
      &StylesCatalog::default(),
      false,
      Some("en-US"),
    );

    let Block::Paragraph(entry) = &sections[0].blocks[0] else {
      panic!("expected rebuilt TOC entry");
    };
    assert!(matches!(
      entry.inlines.first(),
      Some(InlineItem::Text(run)) if run.text == "Table heading"
    ));
    let Block::Table(table) = &sections[0].blocks[1] else {
      panic!("expected outer table");
    };
    let Block::Table(nested) = &table.rows[0].cells[0].blocks[0] else {
      panic!("expected nested table");
    };
    let Block::Paragraph(source) = &nested.rows[0].cells[0].blocks[0] else {
      panic!("expected nested source paragraph");
    };
    assert!(matches!(
      source.inlines.first(),
      Some(InlineItem::BookmarkStart(name)) if name.starts_with("_Toc")
    ));
  }

  #[test]
  fn dirty_toc_inside_table_cell_rebuilds_only_its_cell_blocks() {
    let mut heading = test_paragraph("Outer heading");
    heading.format.style_outline_level = Some(0);
    heading.format.outline_level = Some(0);
    let mut cached = test_paragraph("stale");
    cached.field_events = vec![
      ParagraphFieldEvent::Begin {
        locked: false,
        dirty: true,
      },
      ParagraphFieldEvent::Instruction(r#"TOC \o "1-3""#.to_string()),
      ParagraphFieldEvent::Separate,
      ParagraphFieldEvent::End,
    ];
    let mut sections = vec![default_section(vec![
      Block::paragraph(heading),
      Block::Table(test_table(vec![Block::paragraph(cached)])),
    ])];

    refresh_tables_of_contents(
      &mut sections,
      &StylesCatalog::default(),
      false,
      Some("en-US"),
    );

    let Block::Table(table) = &sections[0].blocks[1] else {
      panic!("expected table");
    };
    assert_eq!(table.rows[0].cells[0].blocks.len(), 1);
    let Block::Paragraph(entry) = &table.rows[0].cells[0].blocks[0] else {
      panic!("expected rebuilt cell TOC");
    };
    assert!(matches!(
      entry.inlines.first(),
      Some(InlineItem::Text(run)) if run.text == "Outer heading"
    ));
  }

  #[test]
  fn locked_toc_keeps_its_persisted_result_even_when_dirty() {
    let mut cached = test_paragraph("locked cached result");
    cached.field_events = vec![
      ParagraphFieldEvent::Begin {
        locked: true,
        dirty: true,
      },
      ParagraphFieldEvent::Instruction("TOC".to_string()),
      ParagraphFieldEvent::Separate,
      ParagraphFieldEvent::End,
    ];
    let mut heading = test_paragraph("Heading");
    heading.format.style_outline_level = Some(0);
    heading.format.outline_level = Some(0);
    let mut sections = vec![default_section(vec![
      Block::paragraph(cached),
      Block::paragraph(heading),
    ])];

    refresh_tables_of_contents(
      &mut sections,
      &StylesCatalog::default(),
      true,
      Some("en-US"),
    );

    let Block::Paragraph(cached) = &sections[0].blocks[0] else {
      panic!("expected cached paragraph");
    };
    assert_eq!(
      paragraph_source_text(cached).as_deref(),
      Some("locked cached result")
    );
    assert_eq!(sections[0].blocks.len(), 2);
  }

  #[test]
  fn cached_toc_link_keeps_annotation_but_uses_toc_paragraph_paint() {
    let styles = StylesCatalog::default();
    let mut cached = test_paragraph("Linked heading");
    cached.field_events = vec![
      ParagraphFieldEvent::Begin {
        locked: true,
        dirty: false,
      },
      ParagraphFieldEvent::Instruction(r#"TOC \h"#.to_string()),
      ParagraphFieldEvent::Separate,
      ParagraphFieldEvent::End,
    ];
    let InlineItem::Text(run) = &mut cached.inlines[0] else {
      panic!("expected cached text");
    };
    run.style = styles.synthesized_hyperlink_run_style(cached.base_style.clone());
    run.hyperlink_url = Some("ooxmlsdk-pdf:bookmark:_Toc1".to_string());
    run.style_ref_keys = vec![Arc::<str>::from("Hyperlink")];
    assert!(run.style.underline);
    assert_ne!(run.style.color, cached.base_style.color);

    let mut sections = vec![default_section(vec![Block::paragraph(cached)])];
    refresh_tables_of_contents(&mut sections, &styles, false, Some("en-US"));

    let Block::Paragraph(cached) = &sections[0].blocks[0] else {
      panic!("expected cached TOC");
    };
    let InlineItem::Text(run) = &cached.inlines[0] else {
      panic!("expected cached link");
    };
    assert_eq!(
      run.hyperlink_url.as_deref(),
      Some("ooxmlsdk-pdf:bookmark:_Toc1")
    );
    assert_eq!(run.style.color, cached.base_style.color);
    assert_eq!(run.style.underline, cached.base_style.underline);
  }

  #[test]
  fn toc_o_uses_style_outline_while_u_accepts_direct_outline() {
    let mut direct_outline = test_paragraph("Direct outline");
    direct_outline.format.style_outline_level = None;
    direct_outline.format.outline_level = Some(1);
    let built_in = TocSpec::parse(r#"TOC \o "1-3""#).unwrap();
    let applied = TocSpec::parse(r#"TOC \u"#).unwrap();

    assert_eq!(paragraph_toc_level(&direct_outline, &built_in), None);
    assert_eq!(paragraph_toc_level(&direct_outline, &applied), Some(2));

    direct_outline.format.style_outline_level = Some(0);
    assert_eq!(paragraph_toc_level(&direct_outline, &built_in), Some(1));
  }

  #[test]
  fn tc_parser_retains_identifier_level_and_page_suppression() {
    assert_eq!(
      TcSpec::parse(r#"TC "Illustration 1" \f i \l 4 \n"#),
      Some(TcSpec {
        text: "Illustration 1".to_string(),
        identifier: Some("i".to_string()),
        level: 4,
        omit_page_number: true,
      })
    );
  }
}
