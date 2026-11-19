use super::{field_instruction_tokens, toc};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Kind {
  Explicit,
  Implicit,
}

/// ECMA-376 §17.16.5.51 permits an omitted REF keyword. MS-OI29500
/// §2.1.495g also reserves Word's private and legacy field instructions.
/// Recognizing an instruction does not imply that its formatter is supported:
/// an unimplemented field must keep its own cached-result path.
pub(super) fn kind(name: &str) -> Option<Kind> {
  let name = name.to_ascii_uppercase();
  if name == "REF" {
    return Some(Kind::Explicit);
  }
  // Word recognizes an omitted REF only for a bookmark identifier. Custom
  // instructions such as names containing a colon are unrecognized fields,
  // not references to missing bookmarks. Explicit REF retains its own argument
  // grammar. Native controls also admit non-ASCII letters and hidden names.
  let mut characters = name.chars();
  let identifier = characters
    .next()
    .is_some_and(|first| first == '_' || first.is_alphabetic())
    && characters.all(|character| character == '_' || character.is_alphanumeric());
  if !identifier
    || name.starts_with('=')
    || matches!(
      name.as_str(),
      "ADDIN"
        | "ADDRESSBLOCK"
        | "ADVANCE"
        | "ASK"
        | "AUTHOR"
        | "AUTONUM"
        | "AUTONUMLGL"
        | "AUTONUMOUT"
        | "AUTOTEXT"
        | "AUTOTEXTLIST"
        | "BARCODE"
        | "BIBLIOGRAPHY"
        | "BIDIOUTLINE"
        | "CITATION"
        | "COMMENTS"
        | "COMPARE"
        | "CONTROL"
        | "CREATEDATE"
        | "DATA"
        | "DATABASE"
        | "DATE"
        | "DDE"
        | "DDEAUTO"
        | "DISPLAYBARCODE"
        | "DISPLAYNFC"
        | "DOCPROPERTY"
        | "DOCVARIABLE"
        | "EDITTIME"
        | "EMBED"
        | "EQ"
        | "FILENAME"
        | "FILESIZE"
        | "FILLIN"
        | "FORMCHECKBOX"
        | "FORMDROPDOWN"
        | "FORMTEXT"
        | "FTNREF"
        | "GLOSSARY"
        | "GOTOBUTTON"
        | "GREETINGLINE"
        | "HTMLCONTROL"
        | "HYPERLINK"
        | "IF"
        | "IMPORT"
        | "INCLUDE"
        | "INCLUDEPICTURE"
        | "INCLUDETEXT"
        | "INDEX"
        | "INFO"
        | "KEYWORDS"
        | "LASTSAVEDBY"
        | "LINK"
        | "LISTNUM"
        | "MACROBUTTON"
        | "MERGEBARCODE"
        | "MERGEFIELD"
        | "MERGEREC"
        | "MERGESEQ"
        | "NEXT"
        | "NEXTIF"
        | "NOTEREF"
        | "NUMCHARS"
        | "NUMPAGES"
        | "NUMWORDS"
        | "PAGE"
        | "PAGEREF"
        | "PRINT"
        | "PRINTDATE"
        | "PRIVATE"
        | "QUOTE"
        | "RD"
        | "REVNUM"
        | "SAVEDATE"
        | "SECTION"
        | "SECTIONPAGES"
        | "SEQ"
        | "SET"
        | "SHAPE"
        | "SKIPIF"
        | "STYLEREF"
        | "SUBJECT"
        | "SUBSCRIBER"
        | "SYMBOL"
        | "TA"
        | "TC"
        | "TEMPLATE"
        | "TIME"
        | "TITLE"
        | "TOA"
        | "TOC"
        | "USERADDRESS"
        | "USERINITIALS"
        | "USERNAME"
        | "XE"
    )
  {
    None
  } else {
    Some(Kind::Implicit)
  }
}

pub(super) fn parse(instruction: &str) -> Option<(Kind, String)> {
  let tokens = field_instruction_tokens(instruction);
  let kind = kind(tokens.first()?)?;
  let name = match kind {
    Kind::Explicit => toc::reference_bookmark_argument(&tokens)?,
    Kind::Implicit => &tokens[0],
  };
  (!name.is_empty()).then(|| (kind, name.to_string()))
}

pub(super) fn requests_paragraph_number(instruction: &str) -> bool {
  let tokens = field_instruction_tokens(instruction);
  let Some(kind) = tokens.first().and_then(|name| kind(name)) else {
    return false;
  };
  let mut have_name = kind == Kind::Implicit;
  let mut numbering = false;
  let mut tokens = tokens.iter().skip(1);
  while let Some(token) = tokens.next() {
    let Some(switch) = token.strip_prefix('\\') else {
      if have_name {
        return false;
      }
      have_name = true;
      continue;
    };
    if ["n", "r", "w"]
      .iter()
      .any(|flag| switch.eq_ignore_ascii_case(flag))
    {
      numbering = true;
    } else if switch.eq_ignore_ascii_case("h")
      || switch.eq_ignore_ascii_case("t")
      || (switch == "*"
        && tokens
          .next()
          .is_some_and(|format| format.eq_ignore_ascii_case("MERGEFORMAT")))
    {
      // Hyperlinks and the numeric projection do not change an unnumbered
      // target's zero. Formatting is retained by the result-span owner.
    } else {
      // Position, numeric pictures and other field transforms have separate
      // result contracts; leave their existing cached-result route intact.
      return false;
    }
  }
  have_name && numbering
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn numbered_reference_requests_keep_text_position_and_picture_owners_separate() {
    for instruction in [
      r"REF Target \r \h",
      r"REF Target \n",
      r"REF Target \W \t",
      r"REF Target \r \* MERGEFORMAT",
      r"Target \n",
    ] {
      assert!(requests_paragraph_number(instruction), "{instruction}");
    }
    for instruction in [
      "REF Target",
      r"REF Target \h",
      r"REF Target \p",
      r#"REF Target \n \# "00""#,
      r"REF Target \n \* CHARFORMAT",
      r"PAGEREF Target \n",
      r"REF \r",
    ] {
      assert!(!requests_paragraph_number(instruction), "{instruction}");
    }
  }

  #[test]
  fn implicit_reference_syntax_keeps_bookmark_case_and_reserved_field_owners() {
    for (instruction, expected) in [
      ("Target", Some((Kind::Implicit, "Target"))),
      ("A_B", Some((Kind::Implicit, "A_B"))),
      ("_Missing", Some((Kind::Implicit, "_Missing"))),
      ("éName", Some((Kind::Implicit, "éName"))),
      ("名称", Some((Kind::Implicit, "名称"))),
      (r#"t "ignored""#, Some((Kind::Implicit, "t"))),
      (r"Target \h", Some((Kind::Implicit, "Target"))),
      (
        r"REF \* MERGEFORMAT Target",
        Some((Kind::Explicit, "Target")),
      ),
      ("", None),
      ("REF", None),
      ("m:usercontent zone1", None),
      ("m:endusercontent", None),
      ("A:B", None),
      ("A.B", None),
      ("A-B", None),
      ("A/B", None),
      ("A@B", None),
      ("123", None),
      ("1abc", None),
      ("+Name", None),
      ("REF m:usercontent", Some((Kind::Explicit, "m:usercontent"))),
      ("=12+34", None),
      ("addin private data", None),
      ("CONTROL Forms.CheckBox.1", None),
      ("EMBED Excel.Sheet.8", None),
      ("HTMLCONTROL", None),
      ("SUBSCRIBER", None),
      ("DISPLAYNFC", None),
      ("FTNREF", None),
      ("AUTOTEXT Missing", None),
      ("TC entry", None),
      ("DDEAUTO", None),
      ("MERGEBARCODE", None),
      ("SHAPE", None),
      ("INCLUDE", None),
      ("PRIVATE", None),
    ] {
      assert_eq!(
        parse(instruction),
        expected.map(|(kind, name)| (kind, name.to_string())),
        "{instruction}"
      );
    }
  }
}
