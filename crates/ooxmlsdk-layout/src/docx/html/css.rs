//! CSS syntax is parsed by cssparser. The supported selector/property subset
//! is intentionally separate from Word's HTML-to-document layout policy.

use cssparser::{
  AtRuleParser, CowRcStr, DeclarationParser, Delimiter, ParseError, Parser, ParserInput,
  ParserState, QualifiedRuleParser, RuleBodyItemParser, RuleBodyParser, StyleSheetParser, ToCss,
  Token as CssToken, parse_important,
};

use super::*;

#[derive(Default)]
pub(super) struct Stylesheet {
  rules: Vec<Rule>,
}

struct Rule {
  selectors: Vec<Selector>,
  declarations: Vec<Declaration>,
}

#[derive(Clone, Debug)]
pub(super) struct Declaration {
  pub name: String,
  pub value: String,
  important: bool,
}

struct Selector {
  parts: Vec<SelectorPart>,
  specificity: (usize, usize, usize),
}

#[derive(Default)]
struct SelectorPart {
  tag: Option<String>,
  id: Option<String>,
  classes: Vec<String>,
  direct_parent: bool,
}

impl SelectorPart {
  fn matches(&self, context: &ElementContext) -> bool {
    self
      .tag
      .as_ref()
      .is_none_or(|tag| context.tag.eq_ignore_ascii_case(tag))
      && self
        .id
        .as_ref()
        .is_none_or(|id| context.id.as_ref() == Some(id))
      && self
        .classes
        .iter()
        .all(|class| context.classes.contains(class))
  }
}

impl Selector {
  fn parse<'i>(input: &mut Parser<'i, '_>) -> Result<Self, ParseError<'i, ()>> {
    let mut parts = Vec::new();
    let mut specificity = (0, 0, 0);
    let mut part = SelectorPart::default();
    let mut has_component = false;
    let mut separated = false;
    while !input.is_exhausted() {
      let token = input.next_including_whitespace()?.clone();
      if matches!(token, CssToken::WhiteSpace(_)) {
        separated |= has_component;
        continue;
      }
      if token == CssToken::Delim('>') {
        if has_component {
          parts.push(part);
          part = SelectorPart::default();
          has_component = false;
        } else if parts.is_empty() || part.direct_parent {
          return Err(input.new_custom_error(()));
        }
        part.direct_parent = true;
        separated = false;
        continue;
      }
      if separated {
        parts.push(part);
        part = SelectorPart::default();
        has_component = false;
        separated = false;
      }
      match token {
        CssToken::Ident(name) if !has_component => {
          part.tag = Some(name.to_ascii_lowercase());
          specificity.2 += 1;
        }
        CssToken::Delim('*') if !has_component => {}
        CssToken::IDHash(id) if part.id.is_none() => {
          part.id = Some(id.to_string());
          specificity.0 += 1;
        }
        CssToken::Delim('.') => {
          let CssToken::Ident(class) = input.next_including_whitespace()? else {
            return Err(input.new_custom_error(()));
          };
          part.classes.push(class.to_string());
          specificity.1 += 1;
        }
        // Reject an unsupported selector as a whole. For example :hover may
        // never silently become an unconditional rule for the same element.
        _ => return Err(input.new_custom_error(())),
      }
      has_component = true;
    }
    if !has_component {
      return Err(input.new_custom_error(()));
    }
    parts.push(part);
    Ok(Self { parts, specificity })
  }

  fn matches(&self, context: &ElementContext, ancestors: &[ElementContext]) -> bool {
    let last = self.parts.len() - 1;
    if !self.parts[last].matches(context) {
      return false;
    }
    let mut cursor = ancestors.len();
    for index in (0..last).rev() {
      if self.parts[index + 1].direct_parent {
        if cursor == 0 || !self.parts[index].matches(&ancestors[cursor - 1]) {
          return false;
        }
        cursor -= 1;
      } else if let Some(position) = ancestors[..cursor]
        .iter()
        .rposition(|c| self.parts[index].matches(c))
      {
        cursor = position;
      } else {
        return false;
      }
    }
    true
  }
}

impl Stylesheet {
  pub(super) fn from_tokens(tokens: &[Token]) -> Self {
    let mut source = String::new();
    let mut in_style = false;
    for token in tokens {
      match token {
        Token::TagToken(tag) if tag.name.as_ref() == "style" => {
          in_style = tag.kind == TagKind::StartTag;
          source.push('\n');
        }
        Token::CharacterTokens(text) if in_style => source.push_str(text),
        _ => {}
      }
    }
    let mut input = ParserInput::new(&source);
    let mut parser = Parser::new(&mut input);
    Self {
      rules: StyleSheetParser::new(&mut parser, &mut Rules)
        .filter_map(Result::ok)
        .collect(),
    }
  }

  pub(super) fn declarations(
    &self,
    context: &ElementContext,
    ancestors: &[ElementContext],
    inline: Option<&str>,
  ) -> Vec<Declaration> {
    let mut declarations = Vec::new();
    for rule in &self.rules {
      if let Some(specificity) = rule
        .selectors
        .iter()
        .filter(|s| s.matches(context, ancestors))
        .map(|s| s.specificity)
        .max()
      {
        for declaration in &rule.declarations {
          declarations.push((
            (declaration.important, false, specificity),
            declaration.clone(),
          ));
        }
      }
    }
    if let Some(inline) = inline {
      let mut input = ParserInput::new(inline);
      let mut parser = Parser::new(&mut input);
      for declaration in parse_declarations(&mut parser) {
        declarations.push(((declaration.important, true, (0, 0, 0)), declaration));
      }
    }
    // Stable sorting retains source order, including shorthand/longhand and
    // repeated-declaration conflicts at equal specificity.
    declarations.sort_by_key(|(priority, _)| *priority);
    declarations
      .into_iter()
      .map(|(_, declaration)| declaration)
      .collect()
  }
}

struct Rules;
impl<'i> AtRuleParser<'i> for Rules {
  type Prelude = ();
  type AtRule = Rule;
  type Error = ();
}
impl<'i> QualifiedRuleParser<'i> for Rules {
  type Prelude = Vec<Selector>;
  type QualifiedRule = Rule;
  type Error = ();
  fn parse_prelude<'t>(
    &mut self,
    input: &mut Parser<'i, 't>,
  ) -> Result<Self::Prelude, ParseError<'i, ()>> {
    input.parse_comma_separated(Selector::parse)
  }
  fn parse_block<'t>(
    &mut self,
    selectors: Self::Prelude,
    _: &ParserState,
    input: &mut Parser<'i, 't>,
  ) -> Result<Rule, ParseError<'i, ()>> {
    Ok(Rule {
      selectors,
      declarations: parse_declarations(input),
    })
  }
}

struct Declarations;
impl<'i> AtRuleParser<'i> for Declarations {
  type Prelude = ();
  type AtRule = Declaration;
  type Error = ();
}
impl<'i> QualifiedRuleParser<'i> for Declarations {
  type Prelude = ();
  type QualifiedRule = Declaration;
  type Error = ();
}
impl<'i> RuleBodyItemParser<'i, Declaration, ()> for Declarations {
  fn parse_declarations(&self) -> bool {
    true
  }
  fn parse_qualified(&self) -> bool {
    false
  }
}
impl<'i> DeclarationParser<'i> for Declarations {
  type Declaration = Declaration;
  type Error = ();
  fn parse_value<'t>(
    &mut self,
    name: CowRcStr<'i>,
    input: &mut Parser<'i, 't>,
    _: &ParserState,
  ) -> Result<Declaration, ParseError<'i, ()>> {
    let value = input.parse_until_before(Delimiter::Bang, serialize_value)?;
    let important = input.try_parse(parse_important).is_ok();
    input.expect_exhausted()?;
    Ok(Declaration {
      name: name.to_ascii_lowercase(),
      value: value.trim().to_string(),
      important,
    })
  }
}

fn parse_declarations(input: &mut Parser<'_, '_>) -> Vec<Declaration> {
  RuleBodyParser::new(input, &mut Declarations)
    .filter_map(Result::ok)
    .collect()
}

fn serialize_value<'i>(input: &mut Parser<'i, '_>) -> Result<String, ParseError<'i, ()>> {
  let mut result = String::new();
  while let Ok(token) = input.next_including_whitespace_and_comments() {
    let token = token.clone();
    if matches!(token, CssToken::Comment(_)) {
      result.push(' ');
      continue;
    }
    let closing = match token {
      CssToken::Function(_) | CssToken::ParenthesisBlock => Some(')'),
      CssToken::SquareBracketBlock => Some(']'),
      CssToken::CurlyBracketBlock => Some('}'),
      CssToken::BadString(_) | CssToken::BadUrl(_) => return Err(input.new_custom_error(())),
      _ => None,
    };
    result.push_str(&token.to_css_string());
    if let Some(closing) = closing {
      result.push_str(&input.parse_nested_block(serialize_value)?);
      result.push(closing);
    }
  }
  Ok(result)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn css_syntax_keeps_quoted_delimiters_and_recovers_after_invalid_declarations() {
    let mut input = ParserInput::new(
      r#"font-family:"Name;With:Delimiters"; broken; color:rgb(1, 2, 3) ! /* comment */ IMPORTANT; font-size: 12pt /* note */;"#,
    );
    let declarations = parse_declarations(&mut Parser::new(&mut input));
    assert_eq!(declarations.len(), 3);
    assert_eq!(declarations[0].value, "\"Name;With:Delimiters\"");
    assert_eq!(declarations[1].name, "color");
    assert!(declarations[1].important);
    assert_eq!(declarations[2].value, "12pt");
  }
}
