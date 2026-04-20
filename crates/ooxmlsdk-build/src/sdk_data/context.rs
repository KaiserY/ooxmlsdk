use heck::ToSnakeCase;
use std::{
  collections::{HashMap, HashSet},
  fs,
  fs::File,
  path::Path,
};

use crate::sdk_data::{
  Result,
  open_xml::{
    OpenXmlNamespace, OpenXmlPart, OpenXmlSchema, OpenXmlSchemaType, TypedNamespace, TypedSchema,
  },
};

#[derive(Debug, Default)]
pub struct Context {
  pub parts: Vec<OpenXmlPart>,
  pub schemas: Vec<OpenXmlSchema>,
  pub namespaces: Vec<OpenXmlNamespace>,
  pub part_name_type_name_map: HashMap<String, String>,
  pub type_name_module_name_map: HashMap<String, String>,
  pub namespace_uri_prefix_map: HashMap<String, String>,
  pub prefix_typed_namespace_map: HashMap<String, String>,
}

impl Context {
  pub fn new(data_dir: &Path) -> Result<Self> {
    let data_parts_dir_path = data_dir.join("parts");
    let data_schemas_dir_path = data_dir.join("schemas");
    let data_typed_dir_path = data_dir.join("typed");

    let mut parts: Vec<OpenXmlPart> = vec![];
    let mut schemas: Vec<OpenXmlSchema> = vec![];
    let mut typed_schemas: Vec<Vec<crate::sdk_data::sdk_data_model::TypedSchema>> = vec![];

    for entry in fs::read_dir(&data_parts_dir_path)? {
      let entry = entry?;
      let file = File::open(entry.path())?;
      let mut open_xml_part: OpenXmlPart = serde_json::from_reader(file)?;

      open_xml_part.module_name = entry
        .path()
        .file_stem()
        .map(|stem| stem.to_string_lossy().to_snake_case())
        .unwrap_or_default();

      parts.push(open_xml_part);
    }

    for entry in fs::read_dir(&data_schemas_dir_path)? {
      let entry = entry?;
      let file = File::open(entry.path())?;
      let mut open_xml_schema: OpenXmlSchema = serde_json::from_reader(file)?;

      open_xml_schema.module_name = entry
        .path()
        .file_stem()
        .map(|stem| stem.to_string_lossy().to_snake_case())
        .unwrap_or_default();

      schemas.push(open_xml_schema);
    }

    for entry in fs::read_dir(&data_typed_dir_path)? {
      let entry = entry?;

      if entry.file_name().to_string_lossy() == "namespaces.json" {
        continue;
      }

      let file = File::open(entry.path())?;
      let items: Vec<TypedSchema> = serde_json::from_reader(file)?;

      typed_schemas.push(
        items
          .into_iter()
          .map(|item| crate::sdk_data::sdk_data_model::TypedSchema {
            name: item.name,
            class_name: item.class_name,
            part_class_name: item.part_class_name,
          })
          .collect(),
      );
    }

    let namespaces: Vec<OpenXmlNamespace> =
      serde_json::from_reader(File::open(data_dir.join("namespaces.json"))?)?;
    let typed_namespaces: Vec<TypedNamespace> =
      serde_json::from_reader(File::open(data_dir.join("typed").join("namespaces.json"))?)?;

    let mut namespace_uri_prefix_map: HashMap<String, String> = HashMap::new();

    for namespace in &namespaces {
      namespace_uri_prefix_map.insert(namespace.uri.clone(), namespace.prefix.clone());
    }

    let mut prefix_typed_namespace_map: HashMap<String, String> = HashMap::new();

    for typed_namespace in &typed_namespaces {
      prefix_typed_namespace_map.insert(
        typed_namespace.prefix.clone(),
        typed_namespace.namespace.clone(),
      );
    }

    let mut part_name_type_name_map = HashMap::new();
    let mut typed_type_name_set: HashSet<String> = HashSet::new();

    for typed_schema_list in &typed_schemas {
      for typed_schema in typed_schema_list {
        typed_type_name_set.insert(typed_schema.name.clone());

        if !typed_schema.part_class_name.is_empty() {
          part_name_type_name_map.insert(
            typed_schema.part_class_name.clone(),
            typed_schema.name.clone(),
          );
        }
      }
    }

    part_name_type_name_map.insert(
      "StyleDefinitionsPart".to_string(),
      "w:CT_Styles/w:styles".to_string(),
    );
    part_name_type_name_map.insert(
      "StylesWithEffectsPart".to_string(),
      "w:CT_Styles/w:styles".to_string(),
    );

    for schema in &mut schemas {
      for ty in &mut schema.types {
        ty.module_name = schema.module_name.clone();
      }

      for e in &mut schema.enums {
        e.module_name = schema.module_name.clone();
      }
    }

    let mut type_name_set: HashSet<String> = HashSet::new();
    let mut type_name_type_map: HashMap<String, OpenXmlSchemaType> = HashMap::new();
    let mut type_name_module_name_map: HashMap<String, String> = HashMap::new();

    for schema in &schemas {
      for ty in &schema.types {
        type_name_type_map.insert(ty.name.clone(), ty.clone());
        type_name_module_name_map.insert(ty.name.clone(), schema.module_name.clone());
      }
    }

    for part in &parts {
      if part.base == "OpenXmlPart"
        && !part.root.is_empty()
        && let Some(type_name) = part_name_type_name_map.get(&part.name)
      {
        gen_type_name_set(&mut type_name_set, type_name, &type_name_type_map);
      }
    }

    for type_name in &typed_type_name_set {
      gen_type_name_set(&mut type_name_set, type_name, &type_name_type_map);
    }

    for schema in &mut schemas {
      schema.types.retain(|ty| {
        type_name_set.is_empty() || type_name_set.contains(&ty.name) || !ty.part.is_empty()
      });
    }

    schemas.sort_by(|a, b| a.module_name.cmp(&b.module_name));

    Ok(Self {
      parts,
      schemas,
      namespaces,
      part_name_type_name_map,
      type_name_module_name_map,
      namespace_uri_prefix_map,
      prefix_typed_namespace_map,
    })
  }
}

fn gen_type_name_set(
  type_name_set: &mut HashSet<String>,
  type_name: &str,
  type_name_type_map: &HashMap<String, OpenXmlSchemaType>,
) {
  if !type_name_set.insert(type_name.to_string()) {
    return;
  }

  let Some(schema_type) = type_name_type_map.get(type_name) else {
    return;
  };

  if let Some(index) = schema_type.name.find('/') {
    let base_type_name = &schema_type.name[..index + 1];

    if base_type_name != schema_type.name && type_name_type_map.contains_key(base_type_name) {
      gen_type_name_set(type_name_set, base_type_name, type_name_type_map);
    }
  }

  if !schema_type.base_class.is_empty()
    && schema_type.base_class != "OpenXmlPartRootElement"
    && type_name_type_map.contains_key(&schema_type.base_class)
  {
    gen_type_name_set(type_name_set, &schema_type.base_class, type_name_type_map);
  }

  for child in &schema_type.children {
    gen_type_name_set(type_name_set, &child.name, type_name_type_map);
  }
}
