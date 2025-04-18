use heck::ToSnakeCase;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::fs::File;
use std::path::Path;

use crate::models::{
  OpenXmlNamespace, OpenXmlPart, OpenXmlSchema, OpenXmlSchemaEnum, OpenXmlSchemaType,
  OpenXmlSchemaTypeParticle, TypedNamespace, TypedSchema,
};
use crate::utils::get_or_panic;

#[derive(Debug)]
pub struct GenContext<'a> {
  pub parts: Vec<OpenXmlPart>,
  pub schemas: Vec<OpenXmlSchema>,
  pub namespaces: Vec<OpenXmlNamespace>,
  pub typed_namespaces: Vec<TypedNamespace>,
  pub schema_mods: Vec<String>,
  pub part_mods: Vec<String>,
  pub prefix_namespace_map: HashMap<&'a str, &'a OpenXmlNamespace>,
  pub uri_namespace_map: HashMap<&'a str, &'a OpenXmlNamespace>,
  pub prefix_schema_mod_map: HashMap<&'a str, &'a str>,
  pub uri_schema_mod_map: HashMap<&'a str, &'a str>,
  pub type_name_type_map: HashMap<&'a str, &'a OpenXmlSchemaType>,
  pub type_name_namespace_map: HashMap<&'a str, &'a OpenXmlNamespace>,
  pub enum_type_enum_map: HashMap<&'a str, &'a OpenXmlSchemaEnum>,
  pub enum_type_namespace_map: HashMap<&'a str, &'a OpenXmlNamespace>,
  pub enum_name_enum_map: HashMap<&'a str, &'a OpenXmlSchemaEnum>,
  pub part_name_type_map: HashMap<&'a str, &'a OpenXmlSchemaType>,
  pub prefix_schema_map: HashMap<&'a str, &'a OpenXmlSchema>,
  pub part_name_part_map: HashMap<&'a str, &'a OpenXmlPart>,
  pub part_name_part_mod_map: HashMap<&'a str, &'a str>,
  pub target_type_map: HashMap<String, &'a OpenXmlSchemaType>,
}

#[derive(Debug, Default)]
pub struct GenContextNeo<'a> {
  pub parts: Vec<OpenXmlPart>,
  pub schemas: Vec<OpenXmlSchema>,
  pub namespaces: Vec<OpenXmlNamespace>,
  pub typed_namespaces: Vec<TypedNamespace>,
  pub typed_schemas: Vec<Vec<TypedSchema>>,
  pub prefix_namespace_map: HashMap<&'a str, &'a OpenXmlNamespace>,
  pub uri_namespace_map: HashMap<&'a str, &'a OpenXmlNamespace>,
  pub prefix_schema_map: HashMap<&'a str, &'a OpenXmlSchema>,
  pub enum_type_enum_map: HashMap<&'a str, &'a OpenXmlSchemaEnum>,
  pub enum_type_namespace_map: HashMap<&'a str, &'a OpenXmlNamespace>,
  pub type_name_type_map: HashMap<&'a str, &'a OpenXmlSchemaType>,
  pub type_name_namespace_map: HashMap<&'a str, &'a OpenXmlNamespace>,
  pub namespace_typed_namespace_map: HashMap<&'a str, &'a TypedNamespace>,
  pub part_name_type_name_map: HashMap<&'a str, &'a str>,
}

impl<'a> GenContextNeo<'a> {
  pub(crate) fn new(data_dir: &'a str) -> Self {
    let data_dir_path = Path::new(data_dir);
    let data_parts_dir_path = &data_dir_path.join("parts");
    let data_schemas_dir_path = &data_dir_path.join("schemas");
    let data_typed_dir_path = &data_dir_path.join("typed");

    let mut parts: Vec<OpenXmlPart> = vec![];
    let mut schemas: Vec<OpenXmlSchema> = vec![];
    let mut typed_schemas: Vec<Vec<TypedSchema>> = vec![];

    for entry in fs::read_dir(data_parts_dir_path).unwrap() {
      let entry = entry.unwrap();

      let file = File::open(entry.path()).unwrap();

      let mut open_xml_part: OpenXmlPart = serde_json::from_reader(file).unwrap();

      let part_mod = entry
        .path()
        .file_stem()
        .unwrap()
        .to_string_lossy()
        .to_snake_case();

      open_xml_part.module_name = part_mod;

      parts.push(open_xml_part);
    }

    for entry in fs::read_dir(data_schemas_dir_path).unwrap() {
      let entry = entry.unwrap();

      let file = File::open(entry.path()).unwrap();

      let mut open_xml_schema: OpenXmlSchema = serde_json::from_reader(file).unwrap();

      let schema_mod = entry
        .path()
        .file_stem()
        .unwrap()
        .to_string_lossy()
        .to_snake_case();

      open_xml_schema.module_name = schema_mod;

      schemas.push(open_xml_schema);
    }

    for entry in fs::read_dir(data_typed_dir_path).unwrap() {
      let entry = entry.unwrap();

      if entry.file_name().to_string_lossy() != "namespaces.json" {
        let file = File::open(entry.path()).unwrap();

        let typed_schema: Vec<TypedSchema> = serde_json::from_reader(file).unwrap();

        typed_schemas.push(typed_schema);
      }
    }

    let file = File::open(data_dir_path.join("namespaces.json")).unwrap();

    let namespaces: Vec<OpenXmlNamespace> = serde_json::from_reader(file).unwrap();

    let file = File::open(data_dir_path.join("typed").join("namespaces.json")).unwrap();

    let typed_namespaces: Vec<TypedNamespace> = serde_json::from_reader(file).unwrap();

    let mut part_name_part_map: HashMap<String, &OpenXmlPart> = HashMap::new();
    let mut part_name_version_map: HashMap<String, String> = HashMap::new();

    for part in parts.iter() {
      part_name_part_map.insert(part.name.to_string(), part);

      part_name_version_map.insert(part.name.to_string(), part.version.to_string());
    }

    let mut uri_namespace_version_map: HashMap<&str, &str> = HashMap::new();

    for namespace in namespaces.iter() {
      uri_namespace_version_map.insert(&namespace.uri, &namespace.version);
    }

    let mut type_name_version_map: HashMap<String, String> = HashMap::new();

    for schema in schemas.iter() {
      for ty in schema.types.iter() {
        type_name_version_map.insert(ty.name.clone(), ty.version.clone());
      }
    }

    let mut part_type_name_map: HashMap<&str, &str> = HashMap::new();

    for typed_schema_list in typed_schemas.iter() {
      for typed_schema in typed_schema_list.iter() {
        if !typed_schema.part_class_name.is_empty() {
          part_type_name_map.insert(&typed_schema.part_class_name, &typed_schema.name);
        }
      }
    }

    let mut part_name_set: HashSet<String> = HashSet::new();

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

    parts.retain(|x| part_name_set.contains(&x.name));

    parts.retain(|x| {
      if let Some(part_type_name) = part_type_name_map.get(x.name.as_str()) {
        let type_version = get_or_panic!(type_name_version_map, *part_type_name);

        check_office_version(&x.version) && check_office_version(type_version)
      } else {
        check_office_version(&x.version)
      }
    });

    for part in parts.iter_mut() {
      part.children.retain(|x| {
        if x.is_data_part_reference {
          true
        } else {
          let child_version = get_or_panic!(part_name_version_map, &x.name);

          if let Some(part_type_name) = part_type_name_map.get(x.name.as_str()) {
            let type_version = get_or_panic!(type_name_version_map, *part_type_name);

            check_office_version(child_version) && check_office_version(type_version)
          } else {
            check_office_version(child_version)
          }
        }
      });
    }

    for schema in schemas.iter_mut() {
      for ty in schema.types.iter_mut() {
        ty.module_name = schema.module_name.clone();
      }

      for e in schema.enums.iter_mut() {
        e.module_name = schema.module_name.clone();
      }
    }

    let mut type_name_set: HashSet<String> = HashSet::new();

    let mut type_name_type_map: HashMap<String, &OpenXmlSchemaType> = HashMap::new();

    for schema in schemas.iter() {
      for ty in schema.types.iter() {
        type_name_type_map.insert(ty.name.clone(), ty);
      }
    }

    for part in parts.iter() {
      if part.base == "OpenXmlPart" && !part.root.is_empty() {
        let type_name = get_or_panic!(part_type_name_map, part.name.as_str());

        gen_type_name_set(&mut type_name_set, type_name, &type_name_type_map)
      }
    }

    for schema in schemas.iter_mut() {
      for e in schema.enums.iter_mut() {
        e.facets.retain(|x| check_office_version(&x.version));
      }

      schema.enums.retain(|x| check_office_version(&x.version));

      for ty in schema.types.iter_mut() {
        ty.attributes.retain(|x| check_office_version(&x.version));

        ty.children.retain(|x| {
          let child_type_version = get_or_panic!(type_name_version_map, x.name.as_str());

          check_office_version(child_type_version)
        });

        check_particle_version(&mut ty.particle);
      }

      schema
        .types
        .retain(|x: &OpenXmlSchemaType| check_office_version(&x.version));
    }

    schemas.retain(|x| {
      let schema_namespace_version =
        get_or_panic!(uri_namespace_version_map, x.target_namespace.as_str());

      check_office_version(schema_namespace_version)
    });

    Self {
      parts,
      schemas,
      namespaces,
      typed_schemas,
      typed_namespaces,
      ..Default::default()
    }
  }
}

pub(crate) fn check_particle_version(particle: &mut OpenXmlSchemaTypeParticle) {
  particle
    .items
    .retain(|x| check_office_version(&x.initial_version));

  for item in particle.items.iter_mut() {
    if !item.kind.is_empty() {
      check_particle_version(item);
    }
  }
}

pub(crate) fn gen_type_name_set(
  type_name_set: &mut HashSet<String>,
  type_name: &str,
  type_name_type_map: &HashMap<String, &OpenXmlSchemaType>,
) {
  if type_name_set.insert(type_name.to_string()) {
    let ty = type_name_type_map.get(type_name).unwrap();

    if ty.is_derived {
      type_name_set.insert(type_name[00..type_name.find('/').unwrap() + 1].to_string());
    }

    for type_child in ty.children.iter() {
      gen_type_name_set(type_name_set, &type_child.name, type_name_type_map)
    }
  }
}

pub(crate) fn gen_part_name_set(
  part_name_set: &mut HashSet<String>,
  part_name: &str,
  part_name_part_map: &HashMap<String, &OpenXmlPart>,
) {
  if part_name_set.insert(part_name.to_string()) {
    let part = part_name_part_map.get(part_name).unwrap();

    for part_child in part.children.iter() {
      if part_child.is_data_part_reference {
        continue;
      }

      gen_part_name_set(part_name_set, &part_child.name, part_name_part_map);
    }
  }
}

pub(crate) fn check_office_version(version: &str) -> bool {
  match version {
    #[cfg(feature = "microsoft365")]
    "Microsoft365" => true,
    #[cfg(not(feature = "microsoft365"))]
    "Microsoft365" => false,
    #[cfg(feature = "office2021")]
    "Office2021" => true,
    #[cfg(not(feature = "office2021"))]
    "Office2021" => false,
    #[cfg(feature = "office2019")]
    "Office2019" => true,
    #[cfg(not(feature = "office2019"))]
    "Office2019" => false,
    #[cfg(feature = "office2016")]
    "Office2016" => true,
    #[cfg(not(feature = "office2016"))]
    "Office2016" => false,
    #[cfg(feature = "office2013")]
    "Office2013" => true,
    #[cfg(not(feature = "office2013"))]
    "Office2013" => false,
    #[cfg(feature = "office2010")]
    "Office2010" => true,
    #[cfg(not(feature = "office2010"))]
    "Office2010" => false,
    "Office2007" => true,
    "" => true,
    _ => false,
  }
}
