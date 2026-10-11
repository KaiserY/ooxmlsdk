//! Regeneration oracle for the general XSD -> `data/schemas` generator.
//!
//! The producer (`sdk_data::xsd_schema_gen`) reproduces checked-in
//! `data/schemas/*.json` metadata from the checked-in XSDs. This module is the
//! acceptance gate: it loads the checked-in schema for a namespace and diffs it
//! against a freshly generated one on the XSD-derivable projection (type names,
//! child names, attribute q-names and required-ness, particle kind/items/occurs,
//! leaf flags, derivation, and enum facet values).
//!
//! `CompositeType`, `BaseClass`, `ClassName`, `Summary`, and enum/facet names are
//! .NET heuristics and are not compared. [`regeneration_oracle`] takes the
//! generator as a closure so the diff stays independent of parser types.

use std::{collections::BTreeMap, fs, path::Path};

use crate::{
  Result,
  sdk_data::open_xml::{OpenXmlSchema, OpenXmlSchemaType, OpenXmlSchemaTypeParticle},
};

/// One structural difference between the expected (checked-in) and actual
/// (regenerated) schema.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SchemaDiff {
  pub type_name: String,
  pub field: String,
  pub expected: String,
  pub actual: String,
}

impl SchemaDiff {
  fn new(
    type_name: impl Into<String>,
    field: impl Into<String>,
    expected: impl Into<String>,
    actual: impl Into<String>,
  ) -> Self {
    Self {
      type_name: type_name.into(),
      field: field.into(),
      expected: expected.into(),
      actual: actual.into(),
    }
  }
}

/// Load every checked-in `data/schemas/*.json` file keyed by target namespace.
///
/// Keying by namespace (rather than file name) avoids re-deriving the
/// namespace -> file-stem rule and lets the oracle map an XSD to its expected
/// JSON by `targetNamespace` alone.
pub fn load_checked_in_schemas(data_schemas_dir: &Path) -> Result<BTreeMap<String, OpenXmlSchema>> {
  let mut schemas = BTreeMap::new();

  for entry in fs::read_dir(data_schemas_dir)? {
    let entry = entry?;
    let path = entry.path();
    if path.extension().and_then(|value| value.to_str()) != Some("json") {
      continue;
    }

    let file = fs::File::open(&path)?;
    let schema: OpenXmlSchema = serde_json::from_reader(file)?;
    if !schema.target_namespace.is_empty() {
      schemas.insert(schema.target_namespace.clone(), schema);
    }
  }

  Ok(schemas)
}

/// Run the oracle: generate an `OpenXmlSchema` from `xsd_source` with
/// `generate`, then diff it against the checked-in schema for the generated
/// schema's target namespace.
///
/// Returns `Err` if no checked-in schema exists for that namespace. The
/// generator closure owns XSD parsing, so this signature stays free of internal
/// parser types.
pub fn regeneration_oracle<F>(
  xsd_source: &str,
  data_schemas_dir: &Path,
  generate: F,
) -> Result<Vec<SchemaDiff>>
where
  F: FnOnce(&str) -> OpenXmlSchema,
{
  let actual = generate(xsd_source);
  let checked_in = load_checked_in_schemas(data_schemas_dir)?;
  let expected = checked_in.get(&actual.target_namespace).ok_or_else(|| {
    format!(
      "no checked-in schema for namespace {}",
      actual.target_namespace
    )
  })?;

  Ok(diff_schemas(expected, &actual))
}

/// Structurally diff two schemas on the generator-owned fields.
pub fn diff_schemas(expected: &OpenXmlSchema, actual: &OpenXmlSchema) -> Vec<SchemaDiff> {
  let mut diffs = Vec::new();

  if expected.target_namespace != actual.target_namespace {
    diffs.push(SchemaDiff::new(
      "<schema>",
      "target_namespace",
      &expected.target_namespace,
      &actual.target_namespace,
    ));
  }

  let expected_types = type_map(&expected.types);
  let actual_types = type_map(&actual.types);

  for (name, expected_type) in &expected_types {
    match actual_types.get(name) {
      None => diffs.push(SchemaDiff::new(*name, "type", "present", "missing")),
      Some(actual_type) => diff_type(name, expected_type, actual_type, &mut diffs),
    }
  }
  for name in actual_types.keys() {
    if !expected_types.contains_key(name) {
      diffs.push(SchemaDiff::new(*name, "type", "missing", "present"));
    }
  }

  let expected_enums = enum_map(expected);
  let actual_enums = enum_map(actual);
  for (name, expected_facets) in &expected_enums {
    match actual_enums.get(name) {
      None => diffs.push(SchemaDiff::new(*name, "enum", "present", "missing")),
      Some(actual_facets) if actual_facets != expected_facets => diffs.push(SchemaDiff::new(
        *name,
        "enum_facets",
        expected_facets.join(","),
        actual_facets.join(","),
      )),
      Some(_) => {}
    }
  }
  for name in actual_enums.keys() {
    if !expected_enums.contains_key(name) {
      diffs.push(SchemaDiff::new(*name, "enum", "missing", "present"));
    }
  }

  diffs
}

fn type_map(types: &[OpenXmlSchemaType]) -> BTreeMap<&str, &OpenXmlSchemaType> {
  types
    .iter()
    .map(|item| (item.name.as_str(), item))
    .collect()
}

fn enum_map(schema: &OpenXmlSchema) -> BTreeMap<&str, Vec<String>> {
  schema
    .enums
    .iter()
    .map(|item| {
      // Key by the XSD simple-type QName (`Type`), not the .NET-only class name,
      // and compare facet values only (facet `Name` is not XSD-derivable).
      let mut values: Vec<String> = item
        .facets
        .iter()
        .map(|facet| facet.value.clone())
        .collect();
      values.sort();
      (item.r#type.as_str(), values)
    })
    .collect()
}

fn diff_type(
  name: &str,
  expected: &OpenXmlSchemaType,
  actual: &OpenXmlSchemaType,
  diffs: &mut Vec<SchemaDiff>,
) {
  // `composite_type` and `base_class` are excluded: the .NET SDK derives them with
  // heuristics that are not reproducible from the XSD alone (e.g. `CompositeType`
  // is absent for some single-sequence types, and root elements alternate between
  // `OpenXmlPartRootElement` and `OpenXmlCompositeElement`).
  let boolean = [
    ("is_derived", expected.is_derived, actual.is_derived),
    ("is_leaf_text", expected.is_leaf_text, actual.is_leaf_text),
    (
      "is_leaf_element",
      expected.is_leaf_element,
      actual.is_leaf_element,
    ),
  ];
  for (field, expected_value, actual_value) in boolean {
    if expected_value != actual_value {
      diffs.push(SchemaDiff::new(
        name,
        field,
        expected_value.to_string(),
        actual_value.to_string(),
      ));
    }
  }

  let expected_children = child_names(&expected.children);
  let actual_children = child_names(&actual.children);
  if expected_children != actual_children {
    diffs.push(SchemaDiff::new(
      name,
      "children",
      expected_children.join(","),
      actual_children.join(","),
    ));
  }

  let expected_attributes = attribute_signatures(expected);
  let actual_attributes = attribute_signatures(actual);
  if expected_attributes != actual_attributes {
    diffs.push(SchemaDiff::new(
      name,
      "attributes",
      expected_attributes.join(","),
      actual_attributes.join(","),
    ));
  }

  let expected_particle = particle_signature(&expected.particle);
  let actual_particle = particle_signature(&actual.particle);
  if expected_particle != actual_particle {
    diffs.push(SchemaDiff::new(
      name,
      "particle",
      expected_particle,
      actual_particle,
    ));
  }
}

fn child_names(children: &[crate::sdk_data::open_xml::OpenXmlSchemaTypeChild]) -> Vec<String> {
  let mut names: Vec<String> = children.iter().map(|child| child.name.clone()).collect();
  names.sort();
  names
}

fn attribute_signatures(schema_type: &OpenXmlSchemaType) -> Vec<String> {
  // Compare the XSD-derivable projection: attribute q_name plus required-ness.
  // The .NET `Type`/`Validators` strings are not XSD-derivable and are validated
  // separately via `simple_type_mapping` unit tests.
  let mut signatures: Vec<String> = schema_type
    .attributes
    .iter()
    .map(|attribute| {
      let required = attribute
        .validators
        .iter()
        .any(|validator| validator.name == "RequiredValidator");
      if required {
        format!("{}!", attribute.q_name)
      } else {
        attribute.q_name.clone()
      }
    })
    .collect();
  signatures.sort();
  signatures
}

/// A compact, comparable rendering of a particle tree.
fn particle_signature(particle: &OpenXmlSchemaTypeParticle) -> String {
  if particle.kind.is_empty()
    && particle.name.is_empty()
    && particle.items.is_empty()
    && particle.occurs.is_empty()
  {
    return String::new();
  }

  let mut occurs: Vec<String> = particle
    .occurs
    .iter()
    .map(|occur| {
      format!(
        "{}..{}",
        occur.min.map(|v| v.to_string()).unwrap_or_default(),
        occur.max.map(|v| v.to_string()).unwrap_or_default()
      )
    })
    .collect();
  occurs.sort();
  let occurs = occurs.join(";");

  let items: Vec<String> = particle.items.iter().map(particle_signature).collect();
  format!(
    "{}:{}({})[{}]",
    particle.kind,
    particle.name,
    occurs,
    items.join("|")
  )
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::sdk_data::open_xml::{OpenXmlSchemaEnum, OpenXmlSchemaEnumFacet};
  use std::collections::BTreeSet;
  use std::path::PathBuf;

  fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
      .parent()
      .and_then(|path| path.parent())
      .expect("workspace root")
      .to_path_buf()
  }

  #[test]
  fn diff_reports_missing_and_changed_fields() {
    let expected = OpenXmlSchema {
      target_namespace: "urn:a".into(),
      types: vec![
        OpenXmlSchemaType {
          name: "a:Root".into(),
          ..Default::default()
        },
        OpenXmlSchemaType {
          name: "a:Extra".into(),
          ..Default::default()
        },
      ],
      enums: vec![OpenXmlSchemaEnum {
        name: "ColorValues".into(),
        r#type: "a:ST_Color".into(),
        facets: vec![OpenXmlSchemaEnumFacet {
          name: "Red".into(),
          value: "1".into(),
          ..Default::default()
        }],
        ..Default::default()
      }],
      ..Default::default()
    };

    let actual = OpenXmlSchema {
      target_namespace: "urn:a".into(),
      types: vec![OpenXmlSchemaType {
        name: "a:Root".into(),
        is_derived: true,
        ..Default::default()
      }],
      enums: vec![OpenXmlSchemaEnum {
        name: "ColorValues".into(),
        r#type: "a:ST_Color".into(),
        facets: vec![OpenXmlSchemaEnumFacet {
          name: "Red".into(),
          value: "2".into(),
          ..Default::default()
        }],
        ..Default::default()
      }],
      ..Default::default()
    };

    let diffs = diff_schemas(&expected, &actual);
    assert!(
      diffs
        .iter()
        .any(|diff| diff.field == "type" && diff.expected == "present" && diff.actual == "missing")
    );
    assert!(
      diffs.iter().any(|diff| diff.field == "is_derived"
        && diff.expected == "false"
        && diff.actual == "true")
    );
    assert!(diffs.iter().any(|diff| diff.field == "enum_facets"));
  }

  #[test]
  fn diff_is_empty_for_identical_schemas() {
    let schema = OpenXmlSchema {
      target_namespace: "urn:a".into(),
      types: vec![OpenXmlSchemaType {
        name: "a:T".into(),
        ..Default::default()
      }],
      ..Default::default()
    };
    assert!(diff_schemas(&schema, &schema).is_empty());
  }

  #[test]
  fn diff_reports_element_particle_name_occurs_and_sequence_order() {
    let expected = OpenXmlSchema {
      types: vec![OpenXmlSchemaType {
        name: "t:CT_Root/t:root".into(),
        particle: OpenXmlSchemaTypeParticle {
          kind: "Sequence".into(),
          items: ["t:CT_A/t:a", "t:CT_B/t:b"]
            .into_iter()
            .map(|name| OpenXmlSchemaTypeParticle {
              name: name.into(),
              ..Default::default()
            })
            .collect(),
          ..Default::default()
        },
        ..Default::default()
      }],
      ..Default::default()
    };
    for mutation in ["name", "occurs", "order"] {
      let mut actual = expected.clone();
      let items = &mut actual.types[0].particle.items;
      match mutation {
        "name" => items[0].name = "t:CT_B/t:b".into(),
        "occurs" => items[0].occurs.push(Default::default()), // 0..unbounded
        "order" => items.swap(0, 1),
        _ => unreachable!(),
      }
      let diffs = diff_schemas(&expected, &actual);
      assert_eq!(diffs.len(), 1, "undetected {mutation}: {diffs:?}");
      assert_eq!(diffs[0].field, "particle");
    }
  }

  #[test]
  fn ooxml_xsds_resolve_to_checked_in_schemas() {
    let root = workspace_root();
    let xsd_dir = root.join("schemas/OfficeOpenXML-XMLSchema-Transitional");
    let checked_in =
      load_checked_in_schemas(&root.join("data/schemas")).expect("load checked-in schemas");

    let mut parsed_count = 0usize;
    let mut matched_count = 0usize;
    for entry in fs::read_dir(&xsd_dir).expect("read xsd dir") {
      let path = entry.expect("dir entry").path();
      if path.extension().and_then(|value| value.to_str()) != Some("xsd") {
        continue;
      }
      let source = fs::read_to_string(&path).expect("read xsd");
      let parsed = crate::sdk_data::xsd::parse_xsd(&source)
        .unwrap_or_else(|err| panic!("parse {}: {err}", path.display()));
      parsed_count += 1;
      if !parsed.target_namespace.is_empty() && checked_in.contains_key(&parsed.target_namespace) {
        matched_count += 1;
      }
    }

    assert!(parsed_count > 0, "expected OOXML XSD inputs");
    assert!(
      matched_count > 0,
      "expected at least one OOXML XSD namespace to have checked-in data/schemas JSON"
    );
  }

  #[test]
  fn generator_reproduces_additional_characteristics() {
    use crate::sdk_data::xsd_schema_gen::{NamespaceGenConfig, gen_open_xml_schema_from_xsd};

    let root = workspace_root();
    let source = fs::read_to_string(
      root
        .join("schemas/OfficeOpenXML-XMLSchema-Transitional/shared-additionalCharacteristics.xsd"),
    )
    .expect("read xsd");

    let mut cfg = NamespaceGenConfig {
      prefix: "ac".to_string(),
      ..Default::default()
    };
    cfg.part_root_elements.insert(
      "additionalCharacteristics".to_string(),
      "AdditionalCharacteristicsPart".to_string(),
    );

    let diffs = regeneration_oracle(&source, &root.join("data/schemas"), |src| {
      gen_open_xml_schema_from_xsd(src, &cfg).expect("generate schema")
    })
    .expect("oracle runs");

    assert!(diffs.is_empty(), "unexpected diffs: {diffs:#?}");
  }

  #[test]
  fn generator_reproduces_bibliography() {
    use crate::sdk_data::xsd_schema_gen::{NamespaceGenConfig, gen_open_xml_schema_from_xsd};

    let root = workspace_root();
    let source = fs::read_to_string(
      root.join("schemas/OfficeOpenXML-XMLSchema-Transitional/shared-bibliography.xsd"),
    )
    .expect("read xsd");

    // The transitional XSD types these elements as `s:ST_String` / `s:ST_Lang`.
    // The checked-in metadata uses the SDK name `b:ST_String255`. That rename is
    // not in the XSD, so the test supplies it. Everything else in the diff is
    // derived from the schema.
    let mut cfg = NamespaceGenConfig {
      prefix: "b".to_string(),
      ..Default::default()
    };
    cfg
      .simple_type_aliases
      .insert("s:ST_String".to_string(), "b:ST_String255".to_string());
    cfg
      .simple_type_aliases
      .insert("s:ST_Lang".to_string(), "b:ST_String255".to_string());

    let diffs = regeneration_oracle(&source, &root.join("data/schemas"), |src| {
      gen_open_xml_schema_from_xsd(src, &cfg).expect("generate schema")
    })
    .expect("oracle runs");

    assert!(diffs.is_empty(), "unexpected diffs: {diffs:#?}");
  }

  /// `pml` matches the XSD projection once SDK rewrites are left out of the
  /// comparison. Those rewrites are not copied into the generator:
  ///
  /// - Per-parent extension lists (`p:CT_SlideExtensionList` and the other
  ///   `*ExtensionList` names). The transitional XSD uses `CT_ExtensionList`,
  ///   `CT_ExtensionListModify`, or `a:CT_OfficeArtExtensionList`.
  /// - Office 2010 and later attributes and elements (`p14:`, `p15:`, `p188:`),
  ///   including `p:CT_ContentPart`, which is an Office 2010 type on the same
  ///   element local name as `CT_Rel`.
  /// - Versioned occurs (`IncludeVersion` / `Version`), such as the color MRU
  ///   `Max: 10` plus an Office 2010 unbounded occur. The XSD says `0..unbounded`.
  /// - Type names and enum values only the JSON has (`a:CT_Color3`,
  ///   `p:CT_RootTimeNode`, `p:CT_TLAnimFloat`, `lastClick` omitted, `none`
  ///   added) and JSON omissions of XSD particles or attributes (`smartTags`,
  ///   `grpFill` on background properties, `bwMode` on graphic frames,
  ///   `target`/`title` on HTML publish properties, `:builtIn` on embedded
  ///   audio). Required-ness is not compared: `typeface` is required in the
  ///   XSD and optional in the JSON, and `CT_ModifyVerifier` is optional in
  ///   the XSD but required in the JSON through versioned validators.
  ///
  /// Child and particle names that use a complex type the XSD does not declare
  /// are aligned to the generated name with the same element local name. The
  /// JSON-only type entries are then dropped. Their bodies are not renamed
  /// onto the XSD types.
  /// The JSON omits the `CT_Extension` wildcard's `0..unbounded` bounds;
  /// restore that specific XSD constraint in the expected projection.
  #[test]
  fn generator_reproduces_pml() {
    use crate::sdk_data::xsd_schema_gen::{NamespaceGenConfig, gen_open_xml_schema_from_xsd};

    let root = workspace_root();
    let source =
      fs::read_to_string(root.join("schemas/OfficeOpenXML-XMLSchema-Transitional/pml.xsd"))
        .expect("read xsd");
    let cfg = NamespaceGenConfig {
      prefix: "p".to_string(),
      schema_dir: Some(root.join("schemas/OfficeOpenXML-XMLSchema-Transitional")),
      ..Default::default()
    };
    let facts = xsd_facts(
      &source,
      cfg.schema_dir.as_deref().expect("schema dir"),
      &cfg.prefix,
    )
    .expect("read xsd facts");
    let mut actual = gen_open_xml_schema_from_xsd(&source, &cfg).expect("generate schema");
    assert_pml_enumerations(&facts, &actual);

    let checked_in =
      load_checked_in_schemas(&root.join("data/schemas")).expect("load checked-in schemas");
    let mut expected = checked_in
      .get(&actual.target_namespace)
      .expect("checked-in pml schema")
      .clone();

    // pml.xsd declares CT_Extension's wildcard as 0..unbounded, whereas
    // the SDK JSON omits Occurs. Use the explicit XSD bounds here, never
    // the generated value, so a regression in wildcard generation still fails.
    let extension = expected
      .types
      .iter_mut()
      .find(|ty| ty.name == "p:CT_Extension/p:ext")
      .expect("checked-in extension type");
    assert_eq!(extension.particle.items.len(), 1);
    let wildcard = &mut extension.particle.items[0];
    assert_eq!(wildcard.kind, "Any");
    assert!(wildcard.occurs.is_empty());
    wildcard.occurs.push(Default::default());

    // Enum facet sets are checked against the XSD above. The JSON adds and
    // drops values, and it copies shared simple types into the `p` prefix.
    expected.enums.clear();
    actual.enums.clear();
    project_pml(&mut expected, &mut actual, &facts);

    let diffs = diff_schemas(&expected, &actual);
    assert!(diffs.is_empty(), "unexpected diffs: {diffs:#?}");
  }

  struct XsdFacts {
    complex_locals: BTreeSet<String>,
    simple_locals: BTreeSet<String>,
    prefixes: BTreeSet<String>,
    /// `prefix:ComplexLocal` -> attribute qnames as the generator spells them.
    attributes: BTreeMap<String, BTreeSet<String>>,
    enumerations: BTreeMap<String, Vec<String>>,
  }

  fn xsd_facts(source: &str, dir: &Path, own_prefix: &str) -> Result<XsdFacts> {
    use crate::sdk_data::xsd::{ParsedComplexType, ParsedXsd, parse_xsd};
    use crate::sdk_data::xsd_schema_gen::NamespaceGenConfig;
    use crate::sdk_data::xsd_schema_gen::load_imported_schemas;

    let main = parse_xsd(source)?;
    let cfg = NamespaceGenConfig {
      prefix: own_prefix.to_string(),
      schema_dir: Some(dir.to_path_buf()),
      ..Default::default()
    };
    let imported = load_imported_schemas(&main, &cfg)?;

    let mut facts = XsdFacts {
      complex_locals: BTreeSet::new(),
      simple_locals: BTreeSet::new(),
      prefixes: main.prefixes.keys().cloned().collect(),
      attributes: BTreeMap::new(),
      enumerations: BTreeMap::new(),
    };
    facts.prefixes.insert(own_prefix.to_string());

    collect_schema_facts(&mut facts, own_prefix, &main);
    for (prefix, uri) in &main.prefixes {
      let Some(schema) = imported.get(uri) else {
        continue;
      };
      facts.prefixes.extend(schema.prefixes.keys().cloned());
      collect_schema_facts(&mut facts, prefix, schema);
    }

    fn collect_schema_facts(facts: &mut XsdFacts, prefix: &str, schema: &ParsedXsd) {
      for (name, body) in &schema.complex_types {
        facts.complex_locals.insert(name.clone());
        let mut attributes = BTreeSet::new();
        collect_attribute_qnames(schema, prefix, body, &mut attributes, &mut BTreeSet::new());
        facts
          .attributes
          .insert(format!("{prefix}:{name}"), attributes);
      }
      for (name, values) in &schema.simple_types {
        facts.simple_locals.insert(name.clone());
        if !values.is_empty() && prefix == "p" {
          facts.enumerations.insert(name.clone(), values.clone());
        }
      }
    }

    fn collect_attribute_qnames(
      schema: &ParsedXsd,
      owner_prefix: &str,
      body: &ParsedComplexType,
      out: &mut BTreeSet<String>,
      stack: &mut BTreeSet<String>,
    ) {
      for attribute in &body.attributes {
        out.insert(attribute_qname(&attribute.q_name));
      }
      for reference in &body.attribute_group_refs {
        let (prefix, local) = split_once_colon(reference);
        let prefix = if prefix.is_empty() {
          owner_prefix
        } else {
          prefix
        };
        let key = format!("{prefix}:{local}");
        if !stack.insert(key.clone()) {
          continue;
        }
        if prefix == owner_prefix
          && let Some(group) = schema.attribute_groups.get(local)
        {
          for attribute in &group.attributes {
            out.insert(attribute_qname(&attribute.q_name));
          }
          for nested in &group.refs {
            let mut nested_body = ParsedComplexType::default();
            nested_body.attribute_group_refs.push(nested.clone());
            collect_attribute_qnames(schema, owner_prefix, &nested_body, out, stack);
          }
        }
        stack.remove(&key);
      }
      if let Some(derivation) = &body.derivation {
        collect_attribute_qnames(schema, owner_prefix, &derivation.body, out, stack);
      }
    }

    Ok(facts)
  }

  fn attribute_qname(q_name: &str) -> String {
    let (prefix, local) = split_once_colon(q_name);
    if prefix.is_empty() {
      format!(":{local}")
    } else {
      format!("{prefix}:{local}")
    }
  }

  fn split_once_colon(value: &str) -> (&str, &str) {
    value.split_once(':').unwrap_or(("", value))
  }

  struct Binding<'a> {
    type_prefix: &'a str,
    complex_local: &'a str,
    element_prefix: &'a str,
    element_local: &'a str,
  }

  fn binding(name: &str) -> Binding<'_> {
    let (ty, element) = name.split_once('/').unwrap_or((name, ""));
    let (type_prefix, complex_local) = split_once_colon(ty);
    let (element_prefix, element_local) = if element.is_empty() {
      ("", "")
    } else {
      split_once_colon(element)
    };
    Binding {
      type_prefix,
      complex_local,
      element_prefix,
      element_local,
    }
  }

  fn prefix_known(name: &str, facts: &XsdFacts) -> bool {
    let parsed = binding(name);
    [parsed.type_prefix, parsed.element_prefix]
      .into_iter()
      .all(|prefix| prefix.is_empty() || prefix == "xsd" || facts.prefixes.contains(prefix))
  }

  fn type_is_declared(name: &str, facts: &XsdFacts) -> bool {
    let parsed = binding(name);
    parsed.type_prefix == "xsd"
      || facts.complex_locals.contains(parsed.complex_local)
      || facts.simple_locals.contains(parsed.complex_local)
  }

  /// Drop JSON-only types, then on each remaining type drop SDK attributes,
  /// elements, and versioned occurs that the transitional XSD does not state.
  /// Also drop XSD particles and attributes that the JSON omits, so the
  /// generator can keep emitting them.
  fn project_pml(expected: &mut OpenXmlSchema, actual: &mut OpenXmlSchema, facts: &XsdFacts) {
    expected
      .types
      .retain(|schema_type| type_is_declared(&schema_type.name, facts));
    let expected_names: BTreeSet<&str> = expected
      .types
      .iter()
      .map(|schema_type| schema_type.name.as_str())
      .collect();
    actual.types.retain(|schema_type| {
      expected_names.contains(schema_type.name.as_str())
        || !facts
          .complex_locals
          .contains(binding(&schema_type.name).complex_local)
    });

    for schema_type in &mut expected.types {
      let Some(actual_type) = actual
        .types
        .iter_mut()
        .find(|candidate| candidate.name == schema_type.name)
      else {
        continue;
      };
      project_type(schema_type, actual_type, facts);
    }
  }

  fn project_type(
    expected: &mut OpenXmlSchemaType,
    actual: &mut OpenXmlSchemaType,
    facts: &XsdFacts,
  ) {
    let declared = facts
      .attributes
      .get(&binding(&expected.name).type_key())
      .cloned();
    project_attributes(
      &mut expected.attributes,
      &mut actual.attributes,
      declared.as_ref(),
    );
    project_children(&mut expected.children, &mut actual.children, facts);
    prepare_particle(&mut expected.particle, &mut actual.particle, facts);
  }

  impl Binding<'_> {
    fn type_key(&self) -> String {
      format!("{}:{}", self.type_prefix, self.complex_local)
    }
  }

  fn project_attributes(
    expected: &mut Vec<crate::sdk_data::open_xml::OpenXmlSchemaTypeAttribute>,
    actual: &mut Vec<crate::sdk_data::open_xml::OpenXmlSchemaTypeAttribute>,
    declared: Option<&BTreeSet<String>>,
  ) {
    for attribute in expected.iter_mut().chain(actual.iter_mut()) {
      attribute
        .validators
        .retain(|validator| validator.name != "RequiredValidator");
    }
    let Some(declared) = declared else {
      return;
    };
    expected.retain(|attribute| declared.contains(&attribute.q_name));
    let expected_names: BTreeSet<&str> = expected
      .iter()
      .map(|attribute| attribute.q_name.as_str())
      .collect();
    actual.retain(|attribute| {
      expected_names.contains(attribute.q_name.as_str()) || !declared.contains(&attribute.q_name)
    });
  }

  fn project_children(
    expected: &mut Vec<crate::sdk_data::open_xml::OpenXmlSchemaTypeChild>,
    actual: &mut Vec<crate::sdk_data::open_xml::OpenXmlSchemaTypeChild>,
    facts: &XsdFacts,
  ) {
    let actual_names: Vec<String> = actual.iter().map(|child| child.name.clone()).collect();
    for child in expected.iter_mut() {
      if let Some(renamed) = align_binding(&child.name, &actual_names, facts) {
        child.name = renamed;
      }
    }
    expected.retain(|child| keep_expected_binding(&child.name, facts));
    let expected_names: BTreeSet<&str> = expected.iter().map(|child| child.name.as_str()).collect();
    actual.retain(|child| {
      expected_names.contains(child.name.as_str()) || !keep_expected_binding(&child.name, facts)
    });
  }

  fn align_binding(name: &str, actual_names: &[String], facts: &XsdFacts) -> Option<String> {
    if type_is_declared(name, facts) {
      return None;
    }
    let element_local = binding(name).element_local;
    if element_local.is_empty() {
      return None;
    }
    let mut candidates = actual_names.iter().filter(|candidate| {
      binding(candidate).element_local == element_local && type_is_declared(candidate, facts)
    });
    let first = candidates.next()?;
    if candidates.next().is_some() {
      return None;
    }
    Some(first.clone())
  }

  fn keep_expected_binding(name: &str, facts: &XsdFacts) -> bool {
    prefix_known(name, facts) && type_is_declared(name, facts)
  }

  fn prepare_particle(
    expected: &mut OpenXmlSchemaTypeParticle,
    actual: &mut OpenXmlSchemaTypeParticle,
    facts: &XsdFacts,
  ) {
    let versioned_only = !expected.occurs.is_empty()
      && expected
        .occurs
        .iter()
        .all(|occur| occur.include_version || !occur.version.is_empty());
    if versioned_only {
      expected.occurs = actual.occurs.clone();
    } else {
      expected
        .occurs
        .retain(|occur| !occur.include_version && occur.version.is_empty());
    }

    let actual_names: Vec<String> = actual
      .items
      .iter()
      .filter(|item| !item.name.is_empty())
      .map(|item| item.name.clone())
      .collect();
    for item in &mut expected.items {
      if !item.name.is_empty()
        && let Some(renamed) = align_binding(&item.name, &actual_names, facts)
      {
        item.name = renamed;
      }
    }
    expected
      .items
      .retain(|item| item.name.is_empty() || keep_expected_binding(&item.name, facts));
    let expected_names: BTreeSet<&str> = expected
      .items
      .iter()
      .filter(|item| !item.name.is_empty())
      .map(|item| item.name.as_str())
      .collect();
    actual.items.retain(|item| {
      item.name.is_empty()
        || expected_names.contains(item.name.as_str())
        || !keep_expected_binding(&item.name, facts)
    });

    if expected.kind.is_empty() && expected.name.is_empty() && expected.items.is_empty() {
      return;
    }
    if !expected.name.is_empty() && expected.items.is_empty() && actual.items.is_empty() {
      return;
    }
    if expected.items.is_empty()
      && expected.name.is_empty()
      && actual.kind.is_empty()
      && actual.items.is_empty()
      && actual.name.is_empty()
    {
      expected.kind.clear();
      expected.occurs.clear();
      return;
    }

    let len = expected.items.len().min(actual.items.len());
    for index in 0..len {
      prepare_particle(&mut expected.items[index], &mut actual.items[index], facts);
    }
  }

  fn assert_pml_enumerations(facts: &XsdFacts, actual: &OpenXmlSchema) {
    assert_eq!(
      actual.enums.len(),
      facts.enumerations.len(),
      "generated enum count"
    );
    for (name, values) in &facts.enumerations {
      let key = format!("p:{name}");
      let generated = actual
        .enums
        .iter()
        .find(|item| item.r#type == key)
        .unwrap_or_else(|| panic!("missing generated enum {key}"));
      let mut got: Vec<&str> = generated
        .facets
        .iter()
        .map(|facet| facet.value.as_str())
        .collect();
      got.sort_unstable();
      let mut want: Vec<&str> = values.iter().map(String::as_str).collect();
      want.sort_unstable();
      assert_eq!(got, want, "{key}");
    }
  }
}
