use super::{LexToken, LexTokenKind, lex_tokens, table::parse_table_reference_selection};

/// Restore formula-bar spellings while preserving authored text and trivia.
pub(crate) fn display_excel_formula_text(source: &str, ui_language: Option<&str>) -> String {
  let tokens = lex_tokens(source).collect::<Vec<_>>();
  let parameter_scopes = parameter_display_scopes(source, &tokens);
  let mut edits = Vec::new();
  for (index, token) in tokens.iter().enumerate() {
    if token.kind != LexTokenKind::Word {
      continue;
    }
    let name = &source[token.start..token.end];
    if let Some(parameter) = parameter_scopes
      .iter()
      .filter(|scope| scope.start <= token.start && token.end <= scope.end)
      .flat_map(|scope| &scope.names)
      .find(|parameter| {
        name.eq_ignore_ascii_case(parameter.name)
          || (parameter.optional
            && name
              .get(..6)
              .is_some_and(|prefix| prefix.eq_ignore_ascii_case("_xlpm."))
            && name[6..].eq_ignore_ascii_case(&parameter.name[6..]))
      })
    {
      if parameter.optional && token.start == parameter.declaration {
        edits.push((token.start, token.start + 6, "["));
        edits.push((token.end, token.end, "]"));
      } else {
        edits.push((token.start, token.start + 6, ""));
      }
    }
    if name
      .get(..7)
      .is_some_and(|prefix| prefix.eq_ignore_ascii_case("_xleta."))
      && crate::function::is_reserved_excel_function_name(&name[7..])
    {
      edits.push((token.start, token.start + 7, ""));
    }
    if let Some(language) = ui_language {
      localize_table_specifiers(name, token.start, language, &mut edits);
    }
    if tokens.get(index + 1).map(|token| token.kind) != Some(LexTokenKind::ParenOpen) {
      continue;
    }
    let upper = name.to_ascii_uppercase();
    if upper == "_XLFN.ANCHORARRAY" || upper == "_XLFN.SINGLE" {
      // Stored functions encode # and @. Drop only the function wrapper;
      // authored grouping remains, and compound operands still need parentheses.
      let Some(close) = single_argument_close(&tokens, index + 1) else {
        continue;
      };
      let (prefix, suffix) = if upper == "_XLFN.SINGLE" {
        ("@", "")
      } else {
        ("", "#")
      };
      if argument_is_atomic(&tokens[index + 2..close]) {
        edits.push((token.start, tokens[index + 1].end, prefix));
        edits.push((tokens[close].start, tokens[close].end, suffix));
      } else {
        edits.push((token.start, token.end, prefix));
        edits.push((tokens[close].end, tokens[close].end, suffix));
      }
    } else if include_str!("excel_future_function_names.txt")
      .split_ascii_whitespace()
      .any(|known| known == upper)
    {
      // The MS-XLSX Functions table lists these storage names independently of which
      // functions this evaluator implements. Unknown future names stay intact.
      let prefix_len = if upper.starts_with("_XLFN._XLWS.") {
        12
      } else {
        6
      };
      edits.push((token.start, token.start + prefix_len, ""));
    }
  }
  edits.sort_by_key(|&(start, end, _)| (start, end));
  let mut result = String::with_capacity(source.len());
  let mut position = 0;
  for (start, end, replacement) in edits {
    result.push_str(&source[position..start]);
    result.push_str(replacement);
    position = end;
  }
  result.push_str(&source[position..]);
  result
}

struct ParameterDisplayScope<'a> {
  start: usize,
  end: usize,
  names: Vec<ParameterDisplayName<'a>>,
}

struct ParameterDisplayName<'a> {
  name: &'a str,
  declaration: usize,
  optional: bool,
}

fn parameter_display_scopes<'a>(
  source: &'a str,
  tokens: &[LexToken],
) -> Vec<ParameterDisplayScope<'a>> {
  let mut scopes = Vec::new();
  for (index, token) in tokens.iter().enumerate() {
    if token.kind != LexTokenKind::Word
      || tokens.get(index + 1).map(|token| token.kind) != Some(LexTokenKind::ParenOpen)
    {
      continue;
    }
    let name = source[token.start..token.end].to_ascii_uppercase();
    let step = match name.as_str() {
      "LET" | "_XLFN.LET" => 2,
      "LAMBDA" | "_XLFN.LAMBDA" => 1,
      _ => continue,
    };
    let Some((args, close)) = call_argument_spans(tokens, index + 1) else {
      continue;
    };
    if args.len() < step + 1 || (step == 2 && args.len().is_multiple_of(2)) {
      continue;
    }
    let mut names = Vec::new();
    for &(start, end) in args[..args.len() - 1].iter().step_by(step) {
      if end != start + 1 || tokens[start].kind != LexTokenKind::Word {
        continue;
      }
      let name = &source[tokens[start].start..tokens[start].end];
      // MS-XLSX parameter-name markers belong to LET/LAMBDA bindings. Do not
      // strip a coincidentally similar workbook name outside that call's scope.
      if name.len() > 6
        && name.get(..6).is_some_and(|prefix| {
          prefix.eq_ignore_ascii_case("_xlpm.")
            || (step == 1 && prefix.eq_ignore_ascii_case("_xlop."))
        })
        && !name.contains(['[', ']', '!', ':'])
      {
        names.push(ParameterDisplayName {
          name,
          declaration: tokens[start].start,
          optional: name[..6].eq_ignore_ascii_case("_xlop."),
        });
      }
    }
    if !names.is_empty() {
      scopes.push(ParameterDisplayScope {
        start: tokens[index + 1].end,
        end: tokens[close].start,
        names,
      });
    }
  }
  scopes
}

fn call_argument_spans(tokens: &[LexToken], open: usize) -> Option<(Vec<(usize, usize)>, usize)> {
  let mut args = Vec::new();
  let mut start = open + 1;
  let mut parentheses = 1;
  let mut arrays = 0;
  for (index, token) in tokens.iter().enumerate().skip(open + 1) {
    match token.kind {
      LexTokenKind::ParenOpen => parentheses += 1,
      LexTokenKind::ParenClose => {
        parentheses -= 1;
        if parentheses == 0 {
          args.push((start, index));
          return Some((args, index));
        }
      }
      LexTokenKind::ArrayOpen => arrays += 1,
      LexTokenKind::ArrayClose => arrays -= 1,
      LexTokenKind::ArgumentSeparator if parentheses == 1 && arrays == 0 => {
        args.push((start, index));
        start = index + 1;
      }
      _ => {}
    }
  }
  None
}

fn argument_is_atomic(tokens: &[LexToken]) -> bool {
  if tokens.len() == 1 {
    return true;
  }
  let open = if tokens
    .first()
    .is_some_and(|token| token.kind == LexTokenKind::Word)
  {
    1
  } else {
    0
  };
  if tokens.get(open).map(|token| token.kind) != Some(LexTokenKind::ParenOpen) {
    return false;
  }
  let mut depth = 0;
  for (index, token) in tokens.iter().enumerate().skip(open) {
    match token.kind {
      LexTokenKind::ParenOpen => depth += 1,
      LexTokenKind::ParenClose => {
        depth -= 1;
        if depth == 0 {
          return index + 1 == tokens.len();
        }
      }
      _ => {}
    }
  }
  false
}

fn localize_table_specifiers(
  token: &str,
  offset: usize,
  language: &str,
  edits: &mut Vec<(usize, usize, &'static str)>,
) {
  // Per-key coverage: the configured de-DE Office tdf162093.xlsx PDF exposes
  // these three resource strings. Other languages/keys retain their spelling.
  if !language
    .split(['-', '_'])
    .next()
    .is_some_and(|language| language.eq_ignore_ascii_case("de"))
    || parse_table_reference_selection(token).is_none()
  {
    return;
  }
  let mut start = None;
  let mut chars = token.char_indices();
  while let Some((index, ch)) = chars.next() {
    match ch {
      '\'' => {
        // A literal # (or bracket) in a column name is apostrophe-escaped.
        let _ = chars.next();
      }
      '[' => start = Some(index + 1),
      ']' => {
        if let Some(start) = start.take() {
          let specifier = &token[start..index];
          let trimmed = specifier.trim();
          let replacement = if trimmed.eq_ignore_ascii_case("#Headers") {
            Some("#Kopfzeilen")
          } else if trimmed.eq_ignore_ascii_case("#Data") {
            Some("#Daten")
          } else if trimmed.eq_ignore_ascii_case("#Totals") {
            Some("#Ergebnisse")
          } else {
            None
          };
          if let Some(replacement) = replacement {
            let start = offset + start + specifier.len() - specifier.trim_start().len();
            edits.push((start, start + trimmed.len(), replacement));
          }
        }
      }
      _ => {}
    }
  }
}

fn single_argument_close(tokens: &[LexToken], open: usize) -> Option<usize> {
  let mut parentheses = 1;
  let mut arrays = 0;
  for (index, token) in tokens.iter().enumerate().skip(open + 1) {
    match token.kind {
      LexTokenKind::ParenOpen => parentheses += 1,
      LexTokenKind::ParenClose => {
        parentheses -= 1;
        if parentheses == 0 {
          return (index > open + 1).then_some(index);
        }
      }
      LexTokenKind::ArrayOpen => arrays += 1,
      LexTokenKind::ArrayClose => arrays -= 1,
      LexTokenKind::ArgumentSeparator if parentheses == 1 && arrays == 0 => return None,
      _ => {}
    }
  }
  None
}
