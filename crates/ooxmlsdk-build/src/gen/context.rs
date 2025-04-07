use heck::ToSnakeCase;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::fs::File;
use std::path::Path;

use crate::models::{
  OpenXmlNamespace, OpenXmlPart, OpenXmlSchema, OpenXmlSchemaEnum, OpenXmlSchemaType,
  TypedNamespace,
};

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
  pub schema_mods: Vec<String>,
  pub part_mods: Vec<String>,
  pub prefix_namespace_map: HashMap<&'a str, &'a OpenXmlNamespace>,
  pub uri_namespace_map: HashMap<&'a str, &'a OpenXmlNamespace>,
  pub prefix_schema_map: HashMap<&'a str, &'a OpenXmlSchema>,
  pub prefix_schema_mod_map: HashMap<&'a str, &'a str>,
  pub enum_type_enum_map: HashMap<&'a str, &'a OpenXmlSchemaEnum>,
  pub enum_type_namespace_map: HashMap<&'a str, &'a OpenXmlNamespace>,
  pub type_name_type_map: HashMap<&'a str, &'a OpenXmlSchemaType>,
  pub type_name_namespace_map: HashMap<&'a str, &'a OpenXmlNamespace>,
}

impl<'a> GenContextNeo<'a> {
  pub(crate) fn new(data_dir: &'a str) -> Self {
    let data_dir_path = Path::new(data_dir);
    let data_parts_dir_path = &data_dir_path.join("parts");
    let data_schemas_dir_path = &data_dir_path.join("schemas");

    let mut parts: Vec<OpenXmlPart> = vec![];
    let mut part_mods: Vec<String> = vec![];
    let mut schemas: Vec<OpenXmlSchema> = vec![];
    let mut schema_mods: Vec<String> = vec![];

    for entry in fs::read_dir(data_parts_dir_path).unwrap() {
      let entry = entry.unwrap();

      let file = File::open(entry.path()).unwrap();

      let open_xml_part: OpenXmlPart = serde_json::from_reader(file).unwrap();

      parts.push(open_xml_part);

      let part_mod = entry
        .path()
        .file_stem()
        .unwrap()
        .to_string_lossy()
        .to_snake_case();

      part_mods.push(part_mod);
    }

    for entry in fs::read_dir(data_schemas_dir_path).unwrap() {
      let entry = entry.unwrap();

      let file = File::open(entry.path()).unwrap();

      let open_xml_schema: OpenXmlSchema = serde_json::from_reader(file).unwrap();

      schemas.push(open_xml_schema);

      let schema_mod = entry
        .path()
        .file_stem()
        .unwrap()
        .to_string_lossy()
        .to_snake_case();

      schema_mods.push(schema_mod);
    }

    let file = File::open(data_dir_path.join("namespaces.json")).unwrap();

    let namespaces: Vec<OpenXmlNamespace> = serde_json::from_reader(file).unwrap();

    let file = File::open(data_dir_path.join("typed").join("namespaces.json")).unwrap();

    let typed_namespaces: Vec<TypedNamespace> = serde_json::from_reader(file).unwrap();

    let mut part_name_set: HashSet<String> = HashSet::new();

    {
      let mut part_name_part_map: HashMap<String, &OpenXmlPart> = HashMap::new();

      for part in parts.iter() {
        part_name_part_map.insert(
          part.name.split('/').collect::<Vec<&str>>()[0].to_string(),
          part,
        );
      }

      gen_part_name_set(
        &mut part_name_set,
        "WordprocessingDocument",
        &part_name_part_map,
      );

      // #[cfg(feature = "xlsx")]
      gen_part_name_set(
        &mut part_name_set,
        "SpreadsheetDocument",
        &part_name_part_map,
      );

      // #[cfg(feature = "pptx")]
      gen_part_name_set(
        &mut part_name_set,
        "PresentationDocument",
        &part_name_part_map,
      );
    }

    parts.retain(|x| part_name_set.contains(&x.name));

    let mut type_name_set: HashSet<String> = HashSet::new();

    {
      let mut type_name_type_map: HashMap<String, &OpenXmlSchemaType> = HashMap::new();

      for schema in schemas.iter() {
        for ty in schema.types.iter() {
          type_name_type_map.insert(ty.name.split('/').collect::<Vec<&str>>()[0].to_string(), ty);
        }
      }

      for part in parts.iter() {
        if part.base == "OpenXmlPart" && !part.root.is_empty() {
          gen_type_name_set(&mut type_name_set, &part.root, &type_name_type_map)
        }
      }
    }

    for schema in schemas.iter_mut() {
      schema
        .types
        .retain(|x| type_name_set.contains(x.name.split('/').collect::<Vec<&str>>()[0]));
    }

    Self {
      parts,
      schemas,
      namespaces,
      typed_namespaces,
      schema_mods,
      part_mods,
      ..Default::default()
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

    for type_child in ty.children.iter() {
      gen_type_name_set(
        type_name_set,
        type_child.name.split('/').collect::<Vec<&str>>()[0],
        type_name_type_map,
      )
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
