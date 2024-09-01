use std::collections::HashMap;

use crate::models::{
  OpenXmlNamespace, OpenXmlPart, OpenXmlSchema, OpenXmlSchemaEnum, OpenXmlSchemaType,
};

#[derive(Debug)]
pub struct GenContext<'a> {
  pub parts: Vec<OpenXmlPart>,
  pub schemas: Vec<OpenXmlSchema>,
  pub namespaces: Vec<OpenXmlNamespace>,
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
}
