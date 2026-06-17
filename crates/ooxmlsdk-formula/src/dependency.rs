use std::borrow::Cow;

use crate::{CellAddress, QualifiedRange, SheetId};

#[derive(Clone, Debug, PartialEq)]
pub enum FormulaDependency<'doc> {
  Cell {
    sheet: SheetId,
    address: CellAddress,
  },
  Range(QualifiedRange<'doc>),
  Name(Cow<'doc, str>),
  External(ExternalReferenceId<'doc>),
  Volatile,
}

impl<'doc> FormulaDependency<'doc> {
  pub(crate) fn into_owned(self) -> FormulaDependency<'static> {
    match self {
      FormulaDependency::Cell { sheet, address } => FormulaDependency::Cell { sheet, address },
      FormulaDependency::Range(value) => FormulaDependency::Range(value.into_owned()),
      FormulaDependency::Name(value) => FormulaDependency::Name(Cow::Owned(value.into_owned())),
      FormulaDependency::External(value) => FormulaDependency::External(value.into_owned()),
      FormulaDependency::Volatile => FormulaDependency::Volatile,
    }
  }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ExternalReferenceId<'doc> {
  pub book: Option<Cow<'doc, str>>,
  pub sheet: Option<Cow<'doc, str>>,
  pub name: Option<Cow<'doc, str>>,
}

impl<'doc> ExternalReferenceId<'doc> {
  pub(crate) fn into_owned(self) -> ExternalReferenceId<'static> {
    ExternalReferenceId {
      book: self.book.map(|value| Cow::Owned(value.into_owned())),
      sheet: self.sheet.map(|value| Cow::Owned(value.into_owned())),
      name: self.name.map(|value| Cow::Owned(value.into_owned())),
    }
  }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct DependencyGraph<'doc> {
  pub nodes: Vec<DependencyNode>,
  pub edges: Vec<DependencyEdge<'doc>>,
  pub defined_name_nodes: Vec<DefinedNameNode<'doc>>,
  pub defined_name_edges: Vec<DefinedNameDependencyEdge<'doc>>,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct DependencyNode {
  pub sheet: SheetId,
  pub cell: CellAddress,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DependencyEdge<'doc> {
  pub from: DependencyNode,
  pub to: FormulaDependency<'doc>,
  pub volatile: bool,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct DefinedNameNode<'doc> {
  pub sheet: Option<SheetId>,
  pub name: Cow<'doc, str>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DefinedNameDependencyEdge<'doc> {
  pub from: DefinedNameNode<'doc>,
  pub to: FormulaDependency<'doc>,
}

#[derive(Debug, Default)]
pub(crate) struct DependencyGraphBuilder<'doc> {
  graph: DependencyGraph<'doc>,
}

impl<'doc> DependencyGraphBuilder<'doc> {
  pub(crate) fn add_formula(
    &mut self,
    sheet: SheetId,
    cell: CellAddress,
    dependencies: impl IntoIterator<Item = FormulaDependency<'doc>>,
    volatile: bool,
  ) {
    let node = DependencyNode { sheet, cell };
    self.graph.nodes.push(node);
    for dependency in dependencies {
      self.graph.edges.push(DependencyEdge {
        from: node,
        to: dependency,
        volatile,
      });
    }
  }

  pub(crate) fn add_defined_name(
    &mut self,
    sheet: Option<SheetId>,
    name: Cow<'doc, str>,
    dependencies: impl IntoIterator<Item = FormulaDependency<'doc>>,
  ) {
    let node = DefinedNameNode { sheet, name };
    self.graph.defined_name_nodes.push(node.clone());
    for dependency in dependencies {
      self
        .graph
        .defined_name_edges
        .push(DefinedNameDependencyEdge {
          from: node.clone(),
          to: dependency,
        });
    }
  }

  pub(crate) fn finish(self) -> DependencyGraph<'doc> {
    self.graph
  }
}

pub(crate) fn dependencies_from_ast<'doc>(
  sheet: SheetId,
  source: &str,
  borrowed_source: Option<&'doc str>,
  ast: Option<&crate::parser::FormulaAst>,
) -> Vec<FormulaDependency<'doc>> {
  let mut dependencies = Vec::new();
  if let Some(ast) = ast {
    collect_dependencies(sheet, source, borrowed_source, ast, &mut dependencies);
  }
  dependencies
}

pub(crate) fn dependency_from_range<'doc>(
  sheet: SheetId,
  range: &QualifiedRange<'doc>,
) -> FormulaDependency<'doc> {
  if range.sheet_name.is_none() && range.range.start == range.range.end {
    FormulaDependency::Cell {
      sheet,
      address: range.range.start,
    }
  } else {
    FormulaDependency::Range(range.clone())
  }
}

pub(crate) fn external_reference_id_from_spans<'doc>(
  source: &str,
  borrowed_source: Option<&'doc str>,
  reference: crate::parser::ExternalReferenceSpans,
) -> ExternalReferenceId<'doc> {
  ExternalReferenceId {
    book: Some(cow_span_text(source, borrowed_source, reference.book)),
    sheet: reference.sheet.map(|sheet| {
      let text = span_text(source, sheet);
      if text.contains("''") {
        Cow::Owned(text.replace("''", "'"))
      } else {
        cow_span_text(source, borrowed_source, sheet)
      }
    }),
    name: reference
      .name
      .map(|name| cow_span_text(source, borrowed_source, name)),
  }
}

pub(crate) fn span_text(source: &str, span: crate::parser::SemanticSpan) -> &str {
  source.get(span.start..span.end).unwrap_or_default()
}

pub(crate) fn cow_span_text<'doc>(
  source: &str,
  borrowed_source: Option<&'doc str>,
  span: crate::parser::SemanticSpan,
) -> Cow<'doc, str> {
  borrowed_source
    .and_then(|source| source.get(span.start..span.end))
    .map(Cow::Borrowed)
    .unwrap_or_else(|| Cow::Owned(span_text(source, span).to_string()))
}

fn collect_dependencies<'doc>(
  sheet: SheetId,
  source: &str,
  borrowed_source: Option<&'doc str>,
  ast: &crate::parser::FormulaAst,
  dependencies: &mut Vec<FormulaDependency<'doc>>,
) {
  match ast {
    crate::parser::FormulaAst::Blank
    | crate::parser::FormulaAst::Text(_)
    | crate::parser::FormulaAst::Number(_)
    | crate::parser::FormulaAst::Error(_) => {}
    crate::parser::FormulaAst::Word { span, kind } => {
      collect_word_dependency(sheet, source, borrowed_source, *span, *kind, dependencies);
    }
    crate::parser::FormulaAst::Unary { expr, .. } => {
      collect_dependencies(sheet, source, borrowed_source, expr, dependencies);
    }
    crate::parser::FormulaAst::Binary { left, right, .. } => {
      collect_dependencies(sheet, source, borrowed_source, left, dependencies);
      collect_dependencies(sheet, source, borrowed_source, right, dependencies);
    }
    crate::parser::FormulaAst::Function { volatile, args, .. } => {
      if *volatile {
        dependencies.push(FormulaDependency::Volatile);
      }
      collect_arg_dependencies(sheet, source, borrowed_source, args, dependencies);
    }
    crate::parser::FormulaAst::LogicalFunction { args, .. } => {
      collect_arg_dependencies(sheet, source, borrowed_source, args, dependencies);
    }
    crate::parser::FormulaAst::Array(rows) => {
      for row in rows {
        collect_arg_dependencies(sheet, source, borrowed_source, row, dependencies);
      }
    }
  }
}

fn collect_arg_dependencies<'doc>(
  sheet: SheetId,
  source: &str,
  borrowed_source: Option<&'doc str>,
  args: &[crate::parser::FormulaAst],
  dependencies: &mut Vec<FormulaDependency<'doc>>,
) {
  for arg in args {
    collect_dependencies(sheet, source, borrowed_source, arg, dependencies);
  }
}

fn collect_word_dependency<'doc>(
  sheet: SheetId,
  source: &str,
  borrowed_source: Option<&'doc str>,
  span: crate::parser::SemanticSpan,
  kind: crate::parser::SemanticWordKind,
  dependencies: &mut Vec<FormulaDependency<'doc>>,
) {
  let word = span_text(source, span);
  match kind {
    crate::parser::SemanticWordKind::Boolean(_) => {}
    crate::parser::SemanticWordKind::ExternalReference(reference) => {
      dependencies.push(FormulaDependency::External(
        external_reference_id_from_spans(
          word,
          borrowed_source.and_then(|source| source.get(span.start..span.end)),
          reference,
        ),
      ));
    }
    crate::parser::SemanticWordKind::ReferenceCandidate => {
      if let Some(range) = crate::parser::parse_formula_range(sheet, word) {
        dependencies.push(dependency_from_range(sheet, &range));
      } else {
        dependencies.push(FormulaDependency::Name(cow_span_text(
          source,
          borrowed_source,
          span,
        )));
      }
    }
    crate::parser::SemanticWordKind::Name => {
      dependencies.push(FormulaDependency::Name(cow_span_text(
        source,
        borrowed_source,
        span,
      )));
    }
  }
}
