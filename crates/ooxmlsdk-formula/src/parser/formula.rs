use super::semantic::{
  ExternalReferenceSpans, SemanticSpan, SemanticTokenKind, SemanticWordKind, semantic_tokens,
  semantic_word_kind,
};
use super::syntax::{SyntaxIssue, SyntaxNode, SyntaxSpan, parse_syntax_ast};
use super::{LexErrorValue, LexLogicalFunction, LexOperator};

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct FormulaBodyParse {
  pub tokens: Vec<FormulaBodyToken>,
  pub ast: Option<FormulaNode>,
  pub issues: Vec<FormulaParseIssue>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct FormulaSyntaxParse {
  pub ast: Option<FormulaNode>,
  pub issues: Vec<FormulaParseIssue>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct FormulaBodyToken {
  pub kind: FormulaBodyTokenKind,
  pub span: SemanticSpan,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum FormulaBodyTokenKind {
  Text,
  Number(f64),
  Error(LexErrorValue),
  Operator(LexOperator),
  ArrayOpen,
  ArrayClose,
  ArgumentSeparator,
  RowSeparator,
  Function { volatile: bool },
  Boolean(bool),
  ExternalReference(ExternalReferenceSpans),
  ReferenceCandidate,
  Name,
  Unsupported,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum FormulaParseIssue {
  UnrecognizedCharacter(SemanticSpan),
  MissingClosingParenthesis,
  IncompleteExpression,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum FormulaNode {
  Blank,
  Text(SemanticSpan),
  Number(f64),
  Error(LexErrorValue),
  Word {
    span: SemanticSpan,
    kind: SemanticWordKind,
  },
  Unary {
    op: LexOperator,
    expr: Box<FormulaNode>,
  },
  Binary {
    op: LexOperator,
    left: Box<FormulaNode>,
    right: Box<FormulaNode>,
  },
  Function {
    name: SemanticSpan,
    args: Vec<FormulaNode>,
  },
  LogicalFunction {
    function: LexLogicalFunction,
    args: Vec<FormulaNode>,
  },
  Array(Vec<Vec<FormulaNode>>),
}

pub(crate) fn parse_formula_body(source: &str) -> FormulaBodyParse {
  let mut issues = Vec::new();
  let tokens = semantic_tokens(source)
    .filter_map(|token| {
      let span = SemanticSpan {
        start: token.start,
        end: token.end,
      };
      let kind = match token.kind {
        SemanticTokenKind::Text => FormulaBodyTokenKind::Text,
        SemanticTokenKind::Number(value) => FormulaBodyTokenKind::Number(value),
        SemanticTokenKind::Error(value) => FormulaBodyTokenKind::Error(value),
        SemanticTokenKind::Operator(value) => FormulaBodyTokenKind::Operator(value),
        SemanticTokenKind::ArrayOpen => FormulaBodyTokenKind::ArrayOpen,
        SemanticTokenKind::ArrayClose => FormulaBodyTokenKind::ArrayClose,
        SemanticTokenKind::ArgumentSeparator => FormulaBodyTokenKind::ArgumentSeparator,
        SemanticTokenKind::RowSeparator => FormulaBodyTokenKind::RowSeparator,
        SemanticTokenKind::ParenOpen | SemanticTokenKind::ParenClose => return None,
        SemanticTokenKind::Function { volatile } => FormulaBodyTokenKind::Function { volatile },
        SemanticTokenKind::Boolean(value) => FormulaBodyTokenKind::Boolean(value),
        SemanticTokenKind::ExternalReference(value) => {
          FormulaBodyTokenKind::ExternalReference(value)
        }
        SemanticTokenKind::ReferenceCandidate => FormulaBodyTokenKind::ReferenceCandidate,
        SemanticTokenKind::Name => FormulaBodyTokenKind::Name,
        SemanticTokenKind::Unsupported => {
          issues.push(FormulaParseIssue::UnrecognizedCharacter(span));
          FormulaBodyTokenKind::Unsupported
        }
      };
      Some(FormulaBodyToken { kind, span })
    })
    .collect();

  let syntax = parse_formula_syntax(source);
  issues.extend(syntax.issues);

  FormulaBodyParse {
    tokens,
    ast: syntax.ast,
    issues,
  }
}

pub(crate) fn parse_formula_syntax(source: &str) -> FormulaSyntaxParse {
  let mut issues = Vec::new();
  let syntax = parse_syntax_ast(source);
  for issue in syntax.issues {
    match issue {
      SyntaxIssue::MissingClosingParenthesis => {
        issues.push(FormulaParseIssue::MissingClosingParenthesis);
      }
    }
  }
  if syntax.ast.is_none() || !syntax.complete {
    issues.push(FormulaParseIssue::IncompleteExpression);
  }
  let ast = syntax
    .ast
    .as_ref()
    .and_then(|node| formula_node_from_syntax(source, node));

  FormulaSyntaxParse { ast, issues }
}

fn formula_node_from_syntax(source: &str, node: &SyntaxNode) -> Option<FormulaNode> {
  match node {
    SyntaxNode::Blank => Some(FormulaNode::Blank),
    SyntaxNode::Text(span) => Some(FormulaNode::Text((*span).into())),
    SyntaxNode::Number(value) => Some(FormulaNode::Number(*value)),
    SyntaxNode::Error(value) => Some(FormulaNode::Error(*value)),
    SyntaxNode::Word(span) => {
      let span = SemanticSpan::from(*span);
      let word = source.get(span.start..span.end)?;
      Some(FormulaNode::Word {
        span,
        kind: semantic_word_kind(word),
      })
    }
    SyntaxNode::Unary { op, expr } => Some(FormulaNode::Unary {
      op: *op,
      expr: Box::new(formula_node_from_syntax(source, expr)?),
    }),
    SyntaxNode::Binary { op, left, right } => Some(FormulaNode::Binary {
      op: *op,
      left: Box::new(formula_node_from_syntax(source, left)?),
      right: Box::new(formula_node_from_syntax(source, right)?),
    }),
    SyntaxNode::Function { name, args } => Some(FormulaNode::Function {
      name: (*name).into(),
      args: formula_nodes_from_syntax(source, args)?,
    }),
    SyntaxNode::LogicalFunction { function, args } => Some(FormulaNode::LogicalFunction {
      function: *function,
      args: formula_nodes_from_syntax(source, args)?,
    }),
    SyntaxNode::Array(rows) => Some(FormulaNode::Array(
      rows
        .iter()
        .map(|row| formula_nodes_from_syntax(source, row))
        .collect::<Option<Vec<_>>>()?,
    )),
  }
}

fn formula_nodes_from_syntax(source: &str, nodes: &[SyntaxNode]) -> Option<Vec<FormulaNode>> {
  nodes
    .iter()
    .map(|node| formula_node_from_syntax(source, node))
    .collect()
}

impl From<SyntaxSpan> for SemanticSpan {
  fn from(span: SyntaxSpan) -> Self {
    Self {
      start: span.start,
      end: span.end,
    }
  }
}
