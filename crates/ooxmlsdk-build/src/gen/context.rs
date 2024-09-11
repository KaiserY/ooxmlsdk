use std::collections::HashMap;

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
  pub type_base_class_type_map: HashMap<&'a str, &'a OpenXmlSchemaType>,
  pub enum_type_enum_map: HashMap<&'a str, &'a OpenXmlSchemaEnum>,
  pub enum_type_namespace_map: HashMap<&'a str, &'a OpenXmlNamespace>,
  pub enum_name_enum_map: HashMap<&'a str, &'a OpenXmlSchemaEnum>,
  pub part_name_type_map: HashMap<&'a str, &'a OpenXmlSchemaType>,
  pub prefix_schema_map: HashMap<&'a str, &'a OpenXmlSchema>,
}
