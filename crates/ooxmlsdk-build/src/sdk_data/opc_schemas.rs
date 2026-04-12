use std::ffi::OsStr;
use std::fs;
use std::path::Path;

use crate::Result;
use crate::sdk_data::sdk_data_model::{
  Schema, SchemaEnum, SchemaEnumFacet, SchemaType, SchemaTypeApiKind, SchemaTypeAttribute,
  SchemaTypeChild, SchemaTypeChildKind, SchemaTypeCompositeKind, SchemaTypeKind,
  SchemaTypeParticle, SchemaTypeParticleOccur, SchemaTypeXmlHeader,
};
use crate::sdk_data::xsd::{ParsedAttribute, ParsedChildElement, parse_xsd};

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
  let xsd = parse_xsd(source).map_err(
    |err| -> Box<dyn std::error::Error + Send + Sync + 'static> {
      format!("opc-relationships.xsd: {err}").into()
    },
  )?;
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
          kind: SchemaTypeChildKind::Child,
          optional: false,
          repeated: true,
          initial_version: String::new(),
          children: Vec::new(),
        }],
        particle: SchemaTypeParticle {
          kind: "Sequence".to_string(),
          name: String::new(),
          occurs: Vec::new(),
          items: vec![SchemaTypeParticle {
            kind: String::new(),
            name: "CT_Relationship/Relationship".to_string(),
            occurs: vec![SchemaTypeParticleOccur {
              min: Some(0),
              max: Some(u64::MAX),
              version: String::new(),
            }],
            items: Vec::new(),
            initial_version: String::new(),
          }],
          initial_version: String::new(),
        },
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
            required: attribute.required,
            ..Default::default()
          })
          .collect(),
        children: Vec::new(),
        particle: SchemaTypeParticle::default(),
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
          ..Default::default()
        })
        .collect(),
      version: String::new(),
    }],
  })
}

fn parse_opc_content_types_xsd(source: &str) -> Result<Schema> {
  let xsd = parse_xsd(source).map_err(
    |err| -> Box<dyn std::error::Error + Send + Sync + 'static> {
      format!("opc-contentTypes.xsd: {err}").into()
    },
  )?;
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
            kind: SchemaTypeChildKind::Child,
            optional: false,
            repeated: false,
            initial_version: String::new(),
            children: Vec::new(),
          })
          .collect(),
        particle: SchemaTypeParticle {
          kind: "Choice".to_string(),
          name: String::new(),
          occurs: vec![SchemaTypeParticleOccur {
            min: Some(0),
            max: Some(u64::MAX),
            version: String::new(),
          }],
          items: types
            .children
            .iter()
            .map(|child| SchemaTypeParticle {
              kind: String::new(),
              name: content_types_child_type_name(child.q_name.as_str()),
              occurs: vec![SchemaTypeParticleOccur {
                min: Some(1),
                max: Some(1),
                version: String::new(),
              }],
              items: Vec::new(),
              initial_version: String::new(),
            })
            .collect(),
          initial_version: String::new(),
        },
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
  let xsd = parse_xsd(source).map_err(
    |err| -> Box<dyn std::error::Error + Send + Sync + 'static> {
      format!("opc-coreProperties.xsd: {err}").into()
    },
  )?;
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
        kind: core_property_child_kind(child.q_name.as_str()),
        optional: false,
        repeated: false,
        initial_version: String::new(),
        children: Vec::new(),
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
            min: Some(child.min_occurs),
            max: Some(child.max_occurs),
            version: String::new(),
          }],
          items: Vec::new(),
          initial_version: String::new(),
        })
        .collect(),
      initial_version: String::new(),
    },
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
        kind: SchemaTypeChildKind::Child,
        optional: false,
        repeated: false,
        initial_version: String::new(),
        children: Vec::new(),
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
            min: Some(child.min_occurs),
            max: Some(child.max_occurs),
            version: String::new(),
          }],
          items: Vec::new(),
          initial_version: String::new(),
        })
        .collect(),
      initial_version: String::new(),
    },
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
        ..Default::default()
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
      required: false,
      ..Default::default()
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

fn core_property_child_kind(q_name: &str) -> SchemaTypeChildKind {
  match core_property_class_name(q_name) {
    "Category" | "ContentStatus" | "Creator" | "Description" | "Identifier" | "Language"
    | "LastModifiedBy" | "LastPrinted" | "Revision" | "Subject" | "Title" | "Version" => {
      SchemaTypeChildKind::TextChild
    }
    _ => SchemaTypeChildKind::Child,
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
      required: attribute.required,
      ..Default::default()
    })
    .collect()
}
