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
    OpenXmlNamespace, OpenXmlPart, OpenXmlSchema, OpenXmlSchemaType, OpenXmlSchemaTypeParticle,
    TypedNamespace, TypedSchema,
  },
};

#[derive(Debug, Default)]
pub struct Context {
  pub schemas: Vec<OpenXmlSchema>,
  pub namespaces: Vec<OpenXmlNamespace>,
  pub typed_namespaces: Vec<TypedNamespace>,
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

    let mut part_name_part_map: HashMap<String, OpenXmlPart> = HashMap::new();
    let mut part_name_version_map: HashMap<String, String> = HashMap::new();

    for part in &parts {
      part_name_part_map.insert(part.name.clone(), part.clone());
      part_name_version_map.insert(part.name.clone(), part.version.clone());
    }

    let mut uri_namespace_version_map: HashMap<String, String> = HashMap::new();
    let mut namespace_uri_prefix_map: HashMap<String, String> = HashMap::new();

    for namespace in &namespaces {
      uri_namespace_version_map.insert(namespace.uri.clone(), namespace.version.clone());
      namespace_uri_prefix_map.insert(namespace.uri.clone(), namespace.prefix.clone());
    }

    let mut prefix_typed_namespace_map: HashMap<String, String> = HashMap::new();

    for typed_namespace in &typed_namespaces {
      prefix_typed_namespace_map.insert(
        typed_namespace.prefix.clone(),
        typed_namespace.namespace.clone(),
      );
    }

    let mut type_name_version_map: HashMap<String, String> = HashMap::new();

    for schema in &schemas {
      for ty in &schema.types {
        type_name_version_map.insert(ty.name.clone(), ty.version.clone());
      }
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

    #[allow(unused_mut)]
    let mut part_name_set: HashSet<String> = HashSet::new();

    #[cfg(feature = "docx")]
    gen_part_name_set(
      &mut part_name_set,
      "WordprocessingDocument",
      &part_name_part_map,
    );

    #[cfg(feature = "xlsx")]
    gen_part_name_set(
      &mut part_name_set,
      "SpreadsheetDocument",
      &part_name_part_map,
    );

    #[cfg(feature = "pptx")]
    gen_part_name_set(
      &mut part_name_set,
      "PresentationDocument",
      &part_name_part_map,
    );

    parts.retain(|part| part_name_set.contains(&part.name));

    parts.retain(|part| {
      if let Some(part_type_name) = part_name_type_name_map.get(&part.name) {
        type_name_version_map
          .get(part_type_name)
          .map(|type_version| {
            check_office_version(&part.version) && check_office_version(type_version)
          })
          .unwrap_or_else(|| check_office_version(&part.version))
      } else {
        check_office_version(&part.version)
      }
    });

    for part in &mut parts {
      part.children.retain(|child| {
        if child.is_data_part_reference {
          return true;
        }

        let Some(child_version) = part_name_version_map.get(&child.name) else {
          return false;
        };

        if let Some(part_type_name) = part_name_type_name_map.get(&child.name) {
          type_name_version_map
            .get(part_type_name)
            .map(|type_version| {
              check_office_version(child_version) && check_office_version(type_version)
            })
            .unwrap_or_else(|| check_office_version(child_version))
        } else {
          check_office_version(child_version)
        }
      });
    }

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
      for e in &mut schema.enums {
        e.facets
          .retain(|facet| check_office_version(&facet.version));
      }

      schema.enums.retain(|e| check_office_version(&e.version));

      for ty in &mut schema.types {
        ty.attributes
          .retain(|attribute| check_office_version(&attribute.version));

        ty.children.retain(|child| {
          type_name_version_map
            .get(&child.name)
            .map(|version| check_office_version(version))
            .unwrap_or(false)
        });

        check_particle_version(&mut ty.particle);
      }

      schema.types.retain(|ty| check_office_version(&ty.version));
    }

    schemas.retain(|schema| {
      uri_namespace_version_map
        .get(&schema.target_namespace)
        .map(|version| check_office_version(version))
        .unwrap_or(false)
    });

    for schema in &mut schemas {
      schema.types.retain(|ty| {
        type_name_set.is_empty() || type_name_set.contains(&ty.name) || !ty.part.is_empty()
      });
    }

    schemas.sort_by(|a, b| a.module_name.cmp(&b.module_name));

    Ok(Self {
      schemas,
      namespaces,
      typed_namespaces,
      type_name_module_name_map,
      namespace_uri_prefix_map,
      prefix_typed_namespace_map,
    })
  }
}

fn check_particle_version(particle: &mut OpenXmlSchemaTypeParticle) {
  particle
    .items
    .retain(|item| check_office_version(&item.initial_version));

  for item in &mut particle.items {
    check_particle_version(item);
  }

  particle
    .occurs
    .retain(|occur| check_office_version(&occur.version));
}

fn gen_part_name_set(
  part_name_set: &mut HashSet<String>,
  part_name: &str,
  part_name_part_map: &HashMap<String, OpenXmlPart>,
) {
  if !part_name_set.insert(part_name.to_string()) {
    return;
  }

  let Some(part) = part_name_part_map.get(part_name) else {
    return;
  };

  for child in &part.children {
    if !child.is_data_part_reference {
      gen_part_name_set(part_name_set, &child.name, part_name_part_map);
    }
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

fn check_office_version(version: &str) -> bool {
  if version.is_empty() || cfg!(feature = "microsoft365") {
    return true;
  }

  match version {
    "Office2007" => cfg!(feature = "office2007"),
    "Office2010" => cfg!(feature = "office2010"),
    "Office2013" => cfg!(feature = "office2013"),
    "Office2016" => cfg!(feature = "office2016"),
    "Office2019" => cfg!(feature = "office2019"),
    "Office2021" => cfg!(feature = "office2021"),
    "Microsoft365" => cfg!(feature = "microsoft365"),
    _ => true,
  }
}
