use quick_xml::Reader;
use quick_xml::events::{BytesStart, Event};
use quick_xml::name::QName;
use std::collections::BTreeMap;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

use crate::Result;
use crate::sdk_data::sdk_data_model::{
  Schema, SchemaEnum, SchemaEnumFacet, SchemaType, SchemaTypeApiKind, SchemaTypeAttribute,
  SchemaTypeAttributeValidator, SchemaTypeChild, SchemaTypeCompositeKind, SchemaTypeKind,
  SchemaTypeParticle, SchemaTypeParticleOccur, SchemaTypeXmlHeader,
};
use crate::simple_type::simple_type_mapping;

pub fn read_opc_schemas(schemas_dir: &Path) -> Result<Vec<Schema>> {
  let mut schemas = vec![];

  if !schemas_dir.exists() {
    return Ok(schemas);
  }

  for entry in fs::read_dir(schemas_dir)? {
    let entry = entry?;
    let path = entry.path();

    if !path.is_file() || path.extension() != Some(OsStr::new("xsd")) {
      continue;
    }

    let Some(file_name) = path.file_name().and_then(|name| name.to_str()) else {
      continue;
    };

    let source = fs::read_to_string(&path)?;

    let schema = match file_name {
      "opc-relationships.xsd" => parse_opc_relationships_xsd(&source)?,
      "opc-contentTypes.xsd" => parse_opc_content_types_xsd(&source)?,
      "opc-coreProperties.xsd" => parse_opc_core_properties_xsd(&source)?,
      _ => continue,
    };

    schemas.push(schema);
  }

  schemas.sort_by(|left, right| left.module_name.cmp(&right.module_name));
  Ok(schemas)
}

fn parse_opc_relationships_xsd(source: &str) -> Result<Schema> {
  let xsd = parse_xsd(source)?;
  xsd
    .complex_types
    .get("CT_Relationships")
    .ok_or("missing CT_Relationships")?;
  let relationship = xsd
    .complex_types
    .get("CT_Relationship")
    .ok_or("missing CT_Relationship")?;
  let target_mode = xsd
    .simple_types
    .get("ST_TargetMode")
    .ok_or("missing ST_TargetMode")?;

  Ok(Schema {
    target_namespace: xsd.target_namespace,
    prefix: String::new(),
    typed_namespace: "opc_relationships".to_string(),
    module_name: "opc_relationships".to_string(),
    types: vec![
      SchemaType {
        name: "CT_Relationships/Relationships".to_string(),
        class_name: "Relationships".to_string(),
        summary: "Relationships.".to_string(),
        version: String::new(),
        part: String::new(),
        base_class: String::new(),
        kind: SchemaTypeKind::Composite,
        composite_kind: SchemaTypeCompositeKind::SdkSequence,
        xml_header: SchemaTypeXmlHeader::Plain,
        is_abstract: false,
        has_xmlns_fields: true,
        has_mc_ignorable_field: false,
        text_value_type: String::new(),
        api_kind: SchemaTypeApiKind::Struct,
        attributes: Vec::new(),
        children: vec![SchemaTypeChild {
          name: "CT_Relationship/Relationship".to_string(),
          property_name: "relationship".to_string(),
          property_comments: "Relationship".to_string(),
        }],
        particle: SchemaTypeParticle {
          kind: "Sequence".to_string(),
          name: String::new(),
          occurs: Vec::new(),
          items: vec![SchemaTypeParticle {
            kind: String::new(),
            name: "CT_Relationship/Relationship".to_string(),
            occurs: vec![SchemaTypeParticleOccur {
              min: 0,
              max: u64::MAX,
              include_version: false,
              version: String::new(),
            }],
            items: Vec::new(),
            initial_version: String::new(),
            require_filter: false,
            namespace: String::new(),
          }],
          initial_version: String::new(),
          require_filter: false,
          namespace: String::new(),
        },
        collection_sequence_root: false,
      },
      SchemaType {
        name: "CT_Relationship/Relationship".to_string(),
        class_name: "Relationship".to_string(),
        summary: "Relationship.".to_string(),
        version: String::new(),
        part: String::new(),
        base_class: String::new(),
        kind: SchemaTypeKind::Leaf,
        composite_kind: SchemaTypeCompositeKind::None,
        xml_header: SchemaTypeXmlHeader::None,
        is_abstract: false,
        has_xmlns_fields: false,
        has_mc_ignorable_field: false,
        text_value_type: String::new(),
        api_kind: SchemaTypeApiKind::Struct,
        attributes: relationship
          .attributes
          .iter()
          .map(|attribute| SchemaTypeAttribute {
            q_name: attribute.q_name.clone(),
            property_name: attribute.field.clone(),
            r#type: if attribute.r#type == "ST_TargetMode" {
              "EnumValue<opc_relationships.TargetMode>".to_string()
            } else {
              attribute.r#type.clone()
            },
            property_comments: attribute.field.replace('_', " ").to_uppercase(),
            version: String::new(),
            validators: if attribute.required {
              vec![SchemaTypeAttributeValidator {
                name: "RequiredValidator".to_string(),
                is_list: false,
                r#type: String::new(),
                union_id: 0,
                is_initial_version: false,
                arguments: Vec::new(),
              }]
            } else {
              Vec::new()
            },
          })
          .collect(),
        children: Vec::new(),
        particle: SchemaTypeParticle::default(),
        collection_sequence_root: false,
      },
    ],
    enums: vec![SchemaEnum {
      name: "TargetMode".to_string(),
      r#type: "ST_TargetMode".to_string(),
      facets: target_mode
        .iter()
        .map(|value| SchemaEnumFacet {
          name: value.clone(),
          value: value.clone(),
          version: String::new(),
        })
        .collect(),
      version: String::new(),
    }],
  })
}

fn parse_opc_content_types_xsd(source: &str) -> Result<Schema> {
  let xsd = parse_xsd(source)?;
  let types = xsd
    .complex_types
    .get("CT_Types")
    .ok_or("missing CT_Types")?;
  let default_type = xsd
    .complex_types
    .get("CT_Default")
    .ok_or("missing CT_Default")?;
  let override_type = xsd
    .complex_types
    .get("CT_Override")
    .ok_or("missing CT_Override")?;

  Ok(Schema {
    target_namespace: xsd.target_namespace,
    prefix: String::new(),
    typed_namespace: String::new(),
    module_name: "opc_content_types".to_string(),
    types: vec![
      SchemaType {
        name: "CT_Types/Types".to_string(),
        class_name: "Types".to_string(),
        summary: "Content Types.".to_string(),
        version: String::new(),
        part: String::new(),
        base_class: String::new(),
        kind: SchemaTypeKind::Composite,
        composite_kind: SchemaTypeCompositeKind::OneChoice,
        xml_header: SchemaTypeXmlHeader::Plain,
        is_abstract: false,
        has_xmlns_fields: true,
        has_mc_ignorable_field: false,
        text_value_type: String::new(),
        api_kind: SchemaTypeApiKind::Struct,
        attributes: Vec::new(),
        children: types
          .children
          .iter()
          .map(|child| SchemaTypeChild {
            name: content_types_child_type_name(child.q_name.as_str()),
            property_name: String::new(),
            property_comments: String::new(),
          })
          .collect(),
        particle: SchemaTypeParticle {
          kind: "Choice".to_string(),
          name: String::new(),
          occurs: vec![SchemaTypeParticleOccur {
            min: 0,
            max: u64::MAX,
            include_version: false,
            version: String::new(),
          }],
          items: types
            .children
            .iter()
            .map(|child| SchemaTypeParticle {
              kind: String::new(),
              name: content_types_child_type_name(child.q_name.as_str()),
              occurs: vec![SchemaTypeParticleOccur {
                min: 1,
                max: 1,
                include_version: false,
                version: String::new(),
              }],
              items: Vec::new(),
              initial_version: String::new(),
              require_filter: false,
              namespace: String::new(),
            })
            .collect(),
          initial_version: String::new(),
          require_filter: false,
          namespace: String::new(),
        },
        collection_sequence_root: false,
      },
      simple_leaf_type(
        "CT_Default/Default",
        "Default",
        "Default content type.",
        &default_type.attributes,
      ),
      simple_leaf_type(
        "CT_Override/Override",
        "Override",
        "Override content type.",
        &override_type.attributes,
      ),
    ],
    enums: Vec::new(),
  })
}

fn content_types_child_type_name(q_name: &str) -> String {
  match q_name {
    "Default" => "CT_Default/Default".to_string(),
    "Override" => "CT_Override/Override".to_string(),
    _ => q_name.to_string(),
  }
}

fn parse_opc_core_properties_xsd(source: &str) -> Result<Schema> {
  let xsd = parse_xsd(source)?;
  let core_properties = xsd
    .complex_types
    .get("CT_CoreProperties")
    .ok_or("missing CT_CoreProperties")?;
  let keywords = xsd
    .complex_types
    .get("CT_Keywords")
    .ok_or("missing CT_Keywords")?;
  let keyword = xsd
    .complex_types
    .get("CT_Keyword")
    .ok_or("missing CT_Keyword")?;

  let mut types = vec![SchemaType {
    name: "cp:CT_CoreProperties/cp:coreProperties".to_string(),
    class_name: "CoreProperties".to_string(),
    summary: "Core File Properties.".to_string(),
    version: String::new(),
    part: String::new(),
    base_class: String::new(),
    kind: SchemaTypeKind::Composite,
    composite_kind: SchemaTypeCompositeKind::OneAll,
    xml_header: SchemaTypeXmlHeader::Standalone,
    is_abstract: false,
    has_xmlns_fields: true,
    has_mc_ignorable_field: false,
    text_value_type: String::new(),
    api_kind: SchemaTypeApiKind::Struct,
    attributes: Vec::new(),
    children: core_properties
      .children
      .iter()
      .map(|child| SchemaTypeChild {
        name: core_property_child_type_name(child),
        property_name: core_property_field_name(child.q_name.as_str()).to_string(),
        property_comments: child.q_name.clone(),
      })
      .collect(),
    particle: SchemaTypeParticle {
      kind: "All".to_string(),
      name: String::new(),
      occurs: Vec::new(),
      items: core_properties
        .children
        .iter()
        .map(|child| SchemaTypeParticle {
          kind: String::new(),
          name: core_property_child_type_name(child),
          occurs: vec![SchemaTypeParticleOccur {
            min: child.min_occurs,
            max: child.max_occurs,
            include_version: false,
            version: String::new(),
          }],
          items: Vec::new(),
          initial_version: String::new(),
          require_filter: false,
          namespace: String::new(),
        })
        .collect(),
      initial_version: String::new(),
      require_filter: false,
      namespace: String::new(),
    },
    collection_sequence_root: false,
  }];

  for child in &core_properties.children {
    if child.q_name == "cp:keywords" {
      continue;
    }
    types.push(core_property_text_type(child));
  }

  types.push(SchemaType {
    name: "cp:CT_Keywords/cp:keywords".to_string(),
    class_name: "Keywords".to_string(),
    summary: "Keywords.".to_string(),
    version: String::new(),
    part: String::new(),
    base_class: String::new(),
    kind: SchemaTypeKind::Composite,
    composite_kind: SchemaTypeCompositeKind::None,
    xml_header: SchemaTypeXmlHeader::None,
    is_abstract: false,
    has_xmlns_fields: false,
    has_mc_ignorable_field: false,
    text_value_type: "StringValue".to_string(),
    api_kind: SchemaTypeApiKind::Struct,
    attributes: keyword_attributes(&keywords.attributes),
    children: keywords
      .children
      .iter()
      .map(|child| SchemaTypeChild {
        name: "cp:CT_Keyword/cp:value".to_string(),
        property_name: "value".to_string(),
        property_comments: child.q_name.clone(),
      })
      .collect(),
    particle: SchemaTypeParticle {
      kind: "Sequence".to_string(),
      name: String::new(),
      occurs: Vec::new(),
      items: keywords
        .children
        .iter()
        .map(|child| SchemaTypeParticle {
          kind: String::new(),
          name: "cp:CT_Keyword/cp:value".to_string(),
          occurs: vec![SchemaTypeParticleOccur {
            min: child.min_occurs,
            max: child.max_occurs,
            include_version: false,
            version: String::new(),
          }],
          items: Vec::new(),
          initial_version: String::new(),
          require_filter: false,
          namespace: String::new(),
        })
        .collect(),
      initial_version: String::new(),
      require_filter: false,
      namespace: String::new(),
    },
    collection_sequence_root: false,
  });

  types.push(SchemaType {
    name: "cp:CT_Keyword/cp:value".to_string(),
    class_name: "Keyword".to_string(),
    summary: "Keyword.".to_string(),
    version: String::new(),
    part: String::new(),
    base_class: String::new(),
    kind: SchemaTypeKind::LeafText,
    composite_kind: SchemaTypeCompositeKind::None,
    xml_header: SchemaTypeXmlHeader::None,
    is_abstract: false,
    has_xmlns_fields: false,
    has_mc_ignorable_field: false,
    text_value_type: "StringValue".to_string(),
    api_kind: SchemaTypeApiKind::Struct,
    attributes: keyword_attributes(&keyword.attributes),
    children: Vec::new(),
    particle: SchemaTypeParticle::default(),
    collection_sequence_root: false,
  });

  Ok(Schema {
    target_namespace: xsd.target_namespace,
    prefix: "cp".to_string(),
    typed_namespace: "opc_core_properties".to_string(),
    module_name: "opc_core_properties".to_string(),
    types,
    enums: vec![SchemaEnum {
      name: "XsiTypeValue".to_string(),
      r#type: "ST_XsiTypeValue".to_string(),
      facets: vec![SchemaEnumFacet {
        name: "DctermsW3cdtf".to_string(),
        value: "dcterms:W3CDTF".to_string(),
        version: String::new(),
      }],
      version: String::new(),
    }],
  })
}

fn simple_leaf_type(
  name: &str,
  class_name: &str,
  summary: &str,
  attributes: &[ParsedAttribute],
) -> SchemaType {
  SchemaType {
    name: name.to_string(),
    class_name: class_name.to_string(),
    summary: summary.to_string(),
    version: String::new(),
    part: String::new(),
    base_class: String::new(),
    kind: SchemaTypeKind::Leaf,
    composite_kind: SchemaTypeCompositeKind::None,
    xml_header: SchemaTypeXmlHeader::None,
    is_abstract: false,
    has_xmlns_fields: false,
    has_mc_ignorable_field: false,
    text_value_type: String::new(),
    api_kind: SchemaTypeApiKind::Struct,
    attributes: attributes_to_schema(attributes),
    children: Vec::new(),
    particle: SchemaTypeParticle::default(),
    collection_sequence_root: false,
  }
}

fn core_property_text_type(child: &ParsedChildElement) -> SchemaType {
  let attributes = if matches!(
    child.q_name.as_str(),
    "dcterms:created" | "dcterms:modified"
  ) {
    vec![SchemaTypeAttribute {
      q_name: "xsi:type".to_string(),
      property_name: "xsi_type".to_string(),
      r#type: "EnumValue<opc_core_properties.XsiTypeValue>".to_string(),
      property_comments: "type".to_string(),
      version: String::new(),
      validators: Vec::new(),
    }]
  } else {
    Vec::new()
  };

  SchemaType {
    name: core_property_child_type_name(child),
    class_name: core_property_class_name(child.q_name.as_str()).to_string(),
    summary: child.q_name.clone(),
    version: String::new(),
    part: String::new(),
    base_class: String::new(),
    kind: SchemaTypeKind::LeafText,
    composite_kind: SchemaTypeCompositeKind::None,
    xml_header: SchemaTypeXmlHeader::None,
    is_abstract: false,
    has_xmlns_fields: false,
    has_mc_ignorable_field: false,
    text_value_type: "StringValue".to_string(),
    api_kind: SchemaTypeApiKind::LeafTextWrapper,
    attributes,
    children: Vec::new(),
    particle: SchemaTypeParticle::default(),
    collection_sequence_root: false,
  }
}

fn core_property_child_type_name(child: &ParsedChildElement) -> String {
  match child.q_name.as_str() {
    "cp:category" => "cp:CT_Category/cp:category".to_string(),
    "cp:contentStatus" => "cp:CT_ContentStatus/cp:contentStatus".to_string(),
    "cp:keywords" => "cp:CT_Keywords/cp:keywords".to_string(),
    "cp:lastModifiedBy" => "cp:CT_LastModifiedBy/cp:lastModifiedBy".to_string(),
    "cp:lastPrinted" => "cp:CT_LastPrinted/cp:lastPrinted".to_string(),
    "cp:revision" => "cp:CT_Revision/cp:revision".to_string(),
    "cp:version" => "cp:CT_Version/cp:version".to_string(),
    "dc:creator" => "dc:CT_Creator/dc:creator".to_string(),
    "dc:description" => "dc:CT_Description/dc:description".to_string(),
    "dc:identifier" => "dc:CT_Identifier/dc:identifier".to_string(),
    "dc:language" => "dc:CT_Language/dc:language".to_string(),
    "dc:subject" => "dc:CT_Subject/dc:subject".to_string(),
    "dc:title" => "dc:CT_Title/dc:title".to_string(),
    "dcterms:created" => "dcterms:CT_Created/dcterms:created".to_string(),
    "dcterms:modified" => "dcterms:CT_Modified/dcterms:modified".to_string(),
    _ => child.q_name.clone(),
  }
}

fn core_property_class_name(q_name: &str) -> &'static str {
  match q_name {
    "cp:category" => "Category",
    "cp:contentStatus" => "ContentStatus",
    "cp:lastModifiedBy" => "LastModifiedBy",
    "cp:lastPrinted" => "LastPrinted",
    "cp:revision" => "Revision",
    "cp:version" => "Version",
    "dc:creator" => "Creator",
    "dc:description" => "Description",
    "dc:identifier" => "Identifier",
    "dc:language" => "Language",
    "dc:subject" => "Subject",
    "dc:title" => "Title",
    "dcterms:created" => "Created",
    "dcterms:modified" => "Modified",
    _ => "Value",
  }
}

fn core_property_field_name(q_name: &str) -> &'static str {
  match q_name {
    "cp:category" => "category",
    "cp:contentStatus" => "content_status",
    "dcterms:created" => "created",
    "dc:creator" => "creator",
    "dc:description" => "description",
    "dc:identifier" => "identifier",
    "cp:keywords" => "keywords",
    "dc:language" => "language",
    "cp:lastModifiedBy" => "last_modified_by",
    "cp:lastPrinted" => "last_printed",
    "dcterms:modified" => "modified",
    "cp:revision" => "revision",
    "dc:subject" => "subject",
    "dc:title" => "title",
    "cp:version" => "version",
    _ => "value",
  }
}

fn keyword_attributes(attributes: &[ParsedAttribute]) -> Vec<SchemaTypeAttribute> {
  attributes_to_schema(attributes)
}

fn attributes_to_schema(attributes: &[ParsedAttribute]) -> Vec<SchemaTypeAttribute> {
  attributes
    .iter()
    .map(|attribute| SchemaTypeAttribute {
      q_name: attribute.q_name.clone(),
      property_name: attribute.field.clone(),
      r#type: attribute.r#type.clone(),
      property_comments: attribute.field.clone(),
      version: String::new(),
      validators: if attribute.required {
        vec![SchemaTypeAttributeValidator {
          name: "RequiredValidator".to_string(),
          is_list: false,
          r#type: String::new(),
          union_id: 0,
          is_initial_version: false,
          arguments: Vec::new(),
        }]
      } else {
        Vec::new()
      },
    })
    .collect()
}

#[derive(Default)]
struct ParsedXsd {
  target_namespace: String,
  complex_types: BTreeMap<String, ParsedComplexType>,
  simple_types: BTreeMap<String, Vec<String>>,
}

#[derive(Default)]
struct ParsedComplexType {
  _mixed: bool,
  children: Vec<ParsedChildElement>,
  attributes: Vec<ParsedAttribute>,
}

#[derive(Clone, Default)]
struct ParsedChildElement {
  q_name: String,
  min_occurs: u64,
  max_occurs: u64,
}

#[derive(Clone, Default)]
struct ParsedAttribute {
  field: String,
  q_name: String,
  r#type: String,
  required: bool,
}

fn parse_xsd(source: &str) -> Result<ParsedXsd> {
  let mut reader = Reader::from_str(source);
  reader.config_mut().trim_text(true);
  let mut parsed = ParsedXsd::default();

  loop {
    match reader.read_event()? {
      Event::Start(e) => match local_name(e.name().as_ref()) {
        b"schema" => {
          parsed.target_namespace = required_attr(&reader, &e, b"targetNamespace")?;
        }
        b"complexType" => {
          let (name, complex_type) = parse_complex_type(&mut reader, e)?;
          parsed.complex_types.insert(name, complex_type);
        }
        b"simpleType" => {
          let (name, values) = parse_simple_type(&mut reader, e)?;
          parsed.simple_types.insert(name, values);
        }
        _ => skip_element(&mut reader, e.name().as_ref())?,
      },
      Event::Empty(_) | Event::Text(_) | Event::Comment(_) | Event::Decl(_) => {}
      Event::Eof => break,
      _ => {}
    }
  }

  Ok(parsed)
}

fn parse_complex_type(
  reader: &mut Reader<&[u8]>,
  start: BytesStart<'_>,
) -> Result<(String, ParsedComplexType)> {
  let name = required_attr(reader, &start, b"name")?;
  let mut complex_type = ParsedComplexType {
    _mixed: optional_attr(reader, &start, b"mixed")?.as_deref() == Some("true"),
    ..ParsedComplexType::default()
  };

  loop {
    match reader.read_event()? {
      Event::Start(e) => match local_name(e.name().as_ref()) {
        b"sequence" | b"choice" | b"all" | b"simpleContent" | b"extension" => {}
        b"element" => {
          complex_type.children.push(parse_child_element(reader, &e)?);
          skip_element(reader, e.name().as_ref())?;
        }
        b"attribute" => {
          if let Some(attribute) = parse_attribute(reader, &e)? {
            complex_type.attributes.push(attribute);
          }
          skip_element(reader, e.name().as_ref())?;
        }
        _ => skip_element(reader, e.name().as_ref())?,
      },
      Event::Empty(e) => match local_name(e.name().as_ref()) {
        b"element" => complex_type.children.push(parse_child_element(reader, &e)?),
        b"attribute" => {
          if let Some(attribute) = parse_attribute(reader, &e)? {
            complex_type.attributes.push(attribute);
          }
        }
        _ => {}
      },
      Event::End(e) if local_name(e.name().as_ref()) == b"complexType" => break,
      Event::Text(_) | Event::Comment(_) => {}
      Event::Eof => return Err("unexpected EOF in complexType".into()),
      _ => {}
    }
  }

  Ok((name, complex_type))
}

fn parse_simple_type(
  reader: &mut Reader<&[u8]>,
  start: BytesStart<'_>,
) -> Result<(String, Vec<String>)> {
  let name = required_attr(reader, &start, b"name")?;
  let mut values = vec![];

  loop {
    match reader.read_event()? {
      Event::Empty(e) if local_name(e.name().as_ref()) == b"enumeration" => {
        values.push(required_attr(reader, &e, b"value")?);
      }
      Event::Start(e) if local_name(e.name().as_ref()) == b"enumeration" => {
        values.push(required_attr(reader, &e, b"value")?);
        skip_element(reader, e.name().as_ref())?;
      }
      Event::End(e) if local_name(e.name().as_ref()) == b"simpleType" => break,
      Event::Text(_) | Event::Comment(_) => {}
      Event::Eof => return Err("unexpected EOF in simpleType".into()),
      _ => {}
    }
  }

  Ok((name, values))
}

fn parse_child_element(
  reader: &Reader<&[u8]>,
  element: &BytesStart<'_>,
) -> Result<ParsedChildElement> {
  let q_name = if let Some(reference) = optional_attr(reader, element, b"ref")? {
    reference
  } else {
    let name = required_attr(reader, element, b"name")?;
    let prefix = element_prefix(reader, element)?;
    if prefix.is_empty() {
      name
    } else {
      format!("{prefix}:{name}")
    }
  };

  Ok(ParsedChildElement {
    q_name,
    min_occurs: optional_attr(reader, element, b"minOccurs")?
      .as_deref()
      .unwrap_or("1")
      .parse()
      .unwrap_or(1),
    max_occurs: optional_attr(reader, element, b"maxOccurs")?
      .as_deref()
      .map(|value| {
        if value == "unbounded" {
          u64::MAX
        } else {
          value.parse().unwrap_or(1)
        }
      })
      .unwrap_or(1),
  })
}

fn parse_attribute(
  reader: &Reader<&[u8]>,
  element: &BytesStart<'_>,
) -> Result<Option<ParsedAttribute>> {
  if let Some(reference) = optional_attr(reader, element, b"ref")? {
    let q_name = reference;
    return Ok(Some(ParsedAttribute {
      field: attribute_field_name(&q_name),
      q_name,
      r#type: "StringValue".to_string(),
      required: optional_attr(reader, element, b"use")?.as_deref() == Some("required"),
    }));
  }

  let Some(name) = optional_attr(reader, element, b"name")? else {
    return Ok(None);
  };

  Ok(Some(ParsedAttribute {
    field: attribute_field_name(&name),
    q_name: name,
    r#type: map_xsd_type_to_schema_type(
      optional_attr(reader, element, b"type")?
        .unwrap_or_else(|| "xsd:string".to_string())
        .as_str(),
    ),
    required: optional_attr(reader, element, b"use")?.as_deref() == Some("required"),
  }))
}

fn attribute_field_name(name: &str) -> String {
  match name {
    "TargetMode" => "target_mode",
    "Target" => "target",
    "Type" => "type",
    "Id" | "ID" => "id",
    "Extension" => "extension",
    "ContentType" => "content_type",
    "PartName" => "part_name",
    "SchemaRef" => "schema_reference",
    "SchemaLanguage" => "schema_language",
    "xml:lang" => "lang",
    _ => name,
  }
  .to_string()
}

fn map_xsd_type_to_schema_type(value: &str) -> String {
  match value {
    "ST_TargetMode" => "ST_TargetMode".to_string(),
    other => match simple_type_mapping(other) {
      mapped if mapped != other => mapped.to_string(),
      _ => "StringValue".to_string(),
    },
  }
}

fn element_prefix(reader: &Reader<&[u8]>, element: &BytesStart<'_>) -> Result<String> {
  if let Some(reference) = optional_attr(reader, element, b"ref")?
    && let Some((prefix, _)) = reference.split_once(':')
  {
    return Ok(prefix.to_string());
  }

  let Some(type_name) = optional_attr(reader, element, b"type")? else {
    return Ok("cp".to_string());
  };

  Ok(
    match strip_prefix(type_name.as_str()) {
      "CT_Keyword" | "CT_Keywords" => "cp",
      _ => "cp",
    }
    .to_string(),
  )
}

fn required_attr(reader: &Reader<&[u8]>, start: &BytesStart<'_>, key: &[u8]) -> Result<String> {
  optional_attr(reader, start, key)?.ok_or_else(|| {
    format!(
      "missing attribute {} on {}",
      String::from_utf8_lossy(key),
      String::from_utf8_lossy(start.name().as_ref())
    )
    .into()
  })
}

fn optional_attr(
  reader: &Reader<&[u8]>,
  start: &BytesStart<'_>,
  key: &[u8],
) -> Result<Option<String>> {
  for attr in start.attributes().with_checks(false) {
    let attr = attr?;
    if attr.key == QName(key) {
      return Ok(Some(
        attr
          .decode_and_unescape_value(reader.decoder())?
          .into_owned(),
      ));
    }
  }

  Ok(None)
}

fn skip_element(reader: &mut Reader<&[u8]>, tag: &[u8]) -> Result<()> {
  let mut depth = 1usize;

  while depth > 0 {
    match reader.read_event()? {
      Event::Start(e) if e.name().as_ref() == tag => depth += 1,
      Event::End(e) if e.name().as_ref() == tag => depth -= 1,
      Event::Eof => return Err("unexpected EOF while skipping element".into()),
      _ => {}
    }
  }

  Ok(())
}

fn local_name(name: &[u8]) -> &[u8] {
  match name.iter().rposition(|byte| *byte == b':') {
    Some(index) => &name[index + 1..],
    None => name,
  }
}

fn strip_prefix(value: &str) -> &str {
  match value.rsplit_once(':') {
    Some((_, suffix)) => suffix,
    None => value,
  }
}
