use std::borrow::Cow;

use crate::function::resolve_function_name;
use crate::program::{
  FormulaExprId, FormulaNodeKind, FormulaNodeLabels, FormulaProgram, FormulaReference,
  FormulaSheetReference,
};
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

pub(crate) fn dependencies_from_program<'doc>(
  sheet: SheetId,
  program: Option<&FormulaProgram>,
) -> Vec<FormulaDependency<'doc>> {
  let mut dependencies = Vec::new();
  let Some(program) = program else {
    return dependencies;
  };
  if let Some(root) = program.root {
    dependencies_from_program_node(sheet, program, root, &mut dependencies);
  }
  dependencies
}

fn dependencies_from_program_node<'doc>(
  sheet: SheetId,
  program: &FormulaProgram,
  id: FormulaExprId,
  dependencies: &mut Vec<FormulaDependency<'doc>>,
) -> Option<()> {
  let node = program.node(id)?;
  match &node.kind {
    FormulaNodeKind::Reference(reference) => {
      push_reference_dependency(sheet, program, reference, dependencies)?;
    }
    FormulaNodeKind::Unary { expr, .. } => {
      dependencies_from_program_node(sheet, program, *expr, dependencies)?;
    }
    FormulaNodeKind::Binary { left, right, .. } => {
      dependencies_from_program_node(sheet, program, *left, dependencies)?;
      dependencies_from_program_node(sheet, program, *right, dependencies)?;
    }
    FormulaNodeKind::Function { name, args } => {
      let volatile = node.metadata.labels.contains(FormulaNodeLabels::VOLATILE)
        || crate::code::function_name_cow(program, None, node.span, *name)
          .and_then(|name| resolve_function_name(name.as_ref()))
          .is_some_and(|function| function.is_volatile());
      if volatile {
        dependencies.push(FormulaDependency::Volatile);
      }
      for arg in program.args(*args)? {
        dependencies_from_program_node(sheet, program, *arg, dependencies)?;
      }
    }
    FormulaNodeKind::Call { callee, args } => {
      dependencies_from_program_node(sheet, program, *callee, dependencies)?;
      for arg in program.args(*args)? {
        dependencies_from_program_node(sheet, program, *arg, dependencies)?;
      }
    }
    FormulaNodeKind::Array(span) => {
      for element in program.array_elements(*span)? {
        dependencies_from_program_node(sheet, program, *element, dependencies)?;
      }
    }
    FormulaNodeKind::Blank
    | FormulaNodeKind::Text(_)
    | FormulaNodeKind::Number(_)
    | FormulaNodeKind::Boolean(_)
    | FormulaNodeKind::Error(_)
    | FormulaNodeKind::MissingArgument
    | FormulaNodeKind::Unsupported(_) => {}
  }
  Some(())
}

fn push_reference_dependency<'doc>(
  sheet: SheetId,
  program: &FormulaProgram,
  reference: &FormulaReference,
  dependencies: &mut Vec<FormulaDependency<'doc>>,
) -> Option<()> {
  match reference {
    FormulaReference::Cell(reference) => {
      if let FormulaSheetReference::External { book, sheet } = reference.target.sheet {
        dependencies.push(FormulaDependency::External(
          crate::code::external_reference_from_cell(
            program,
            book,
            sheet,
            reference.target,
            reference.flags,
          )?,
        ));
      } else {
        let range = crate::code::qualified_range_from_points(
          sheet,
          program,
          reference.target,
          reference.target,
          reference.flags,
        )?;
        dependencies.push(dependency_from_range(sheet, &range));
      }
    }
    FormulaReference::Range(reference) => {
      if let FormulaSheetReference::External { book, sheet } = reference.start.sheet {
        dependencies.push(FormulaDependency::External(
          crate::code::external_reference_from_range(
            program,
            book,
            sheet,
            reference.start,
            reference.end,
            reference.flags,
          )?,
        ));
      } else {
        let range = crate::code::qualified_range_from_points(
          sheet,
          program,
          reference.start,
          reference.end,
          reference.flags,
        )?;
        dependencies.push(dependency_from_range(sheet, &range));
      }
    }
    FormulaReference::Named(reference) => {
      dependencies.push(FormulaDependency::Name(crate::code::scoped_name_cow(
        program,
        None,
        None,
        &reference.scope,
        reference.name,
      )?));
    }
    FormulaReference::Structured(reference) => {
      dependencies.push(FormulaDependency::Name(Cow::Owned(
        crate::code::structured_reference_text(program, reference)?,
      )));
    }
    FormulaReference::ExternalName(reference) => {
      dependencies.push(FormulaDependency::External(ExternalReferenceId {
        book: Some(Cow::Owned(program.symbols.get(reference.book)?.to_string())),
        sheet: match reference.sheet {
          Some(sheet) => Some(Cow::Owned(crate::code::sheet_name_text(program, sheet)?)),
          None => None,
        },
        name: Some(Cow::Owned(program.symbols.get(reference.name)?.to_string())),
      }));
    }
    FormulaReference::Deleted(_) => {}
  }
  Some(())
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
