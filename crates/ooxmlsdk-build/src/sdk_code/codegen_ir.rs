use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(default, rename_all = "PascalCase")]
pub struct SchemaModuleDecl {
  pub module_name: String,
  pub target_namespace: String,
  pub prefix: String,
  pub typed_namespace: String,
  pub enums: Vec<EnumDecl>,
  pub types: Vec<TypeDecl>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(default, rename_all = "PascalCase")]
pub struct EnumDecl {
  pub rust_name: String,
  pub docs: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub version: Option<String>,
  pub value_type: EnumValueType,
  pub variants: Vec<EnumVariantDecl>,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum EnumValueType {
  #[default]
  StringLike,
  NumericLike,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(default, rename_all = "PascalCase")]
pub struct EnumVariantDecl {
  pub rust_name: String,
  pub xml_value: String,
  pub aliases: Vec<String>,
  pub version: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(default, rename_all = "PascalCase")]
pub struct TypeDecl {
  pub rust_name: String,
  #[serde(rename = "XmlQName", skip_serializing_if = "Option::is_none")]
  pub xml_qname: Option<String>,
  pub docs: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub version: Option<String>,
  pub is_abstract: bool,
  pub kind: TypeKind,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub element_kind: Option<ElementKind>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub content_model: Option<ContentModelDecl>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub base_rust_name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub xml_content: Option<TypeRefDecl>,
  pub support: SystemSupportDecl,
  pub members: Vec<MemberDecl>,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum TypeKind {
  #[default]
  ElementStruct,
  ChoiceEnum,
  HelperStruct,
  LeafTextAlias,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum ElementKind {
  #[default]
  LeafText,
  Leaf,
  Composite,
  Derived,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum ContentModelDecl {
  #[default]
  None,
  OneAllDirectChildren,
  OneChoiceSingle,
  MixedChoiceChildren,
  SequenceAnyOnly,
  SequenceSingleChoice,
  SequenceDirectChildren,
  OneSequenceFlatten,
  OneSequenceStructured,
  GenericChildrenFallback,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(default, rename_all = "PascalCase")]
pub struct SystemSupportDecl {
  pub xmlns_mode: XmlnsMode,
  pub xml_header: XmlHeaderMode,
  pub has_mce: bool,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum XmlnsMode {
  #[default]
  None,
  MapOnly,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum XmlHeaderMode {
  #[default]
  None,
  Plain,
  Standalone,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum MemberDecl {
  Field(FieldDecl),
  Variant(VariantDecl),
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(default, rename_all = "PascalCase")]
pub struct FieldDecl {
  pub rust_name: String,
  pub docs: String,
  pub version: String,
  pub wire: FieldWireDecl,
  pub cardinality: Cardinality,
  pub type_ref: TypeRefDecl,
  pub validators: Vec<ValidatorDecl>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase", rename_all_fields = "PascalCase")]
pub enum FieldWireDecl {
  Attribute {
    #[serde(rename = "QName")]
    qname: String,
    bit: Option<u32>,
  },
  Child {
    #[serde(rename = "QName")]
    qname: String,
  },
  TextChild {
    #[serde(rename = "QName")]
    qname: String,
  },
  #[default]
  Choice,
  Any,
  Text,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(default, rename_all = "PascalCase")]
pub struct VariantDecl {
  pub rust_name: String,
  pub docs: String,
  pub version: String,
  pub wire: VariantWireDecl,
  pub payload: TypeRefDecl,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase", rename_all_fields = "PascalCase")]
pub enum VariantWireDecl {
  Child {
    #[serde(rename = "QNames")]
    qnames: Vec<String>,
  },
  TextChild {
    #[serde(rename = "QNames")]
    qnames: Vec<String>,
  },
  Any,
  #[default]
  Text,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum Cardinality {
  #[default]
  One,
  Optional,
  Many,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(default, rename_all = "PascalCase")]
pub struct TypeRefDecl {
  pub rust_type: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub module_path: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(default, rename_all = "PascalCase")]
pub struct ValidatorDecl {
  pub version: String,
  pub kind: ValidatorKind,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase", rename_all_fields = "PascalCase")]
pub enum ValidatorKind {
  StringLength {
    min: Option<u32>,
    max: Option<u32>,
  },
  Pattern {
    regex: String,
  },
  NumberRange {
    min: Option<String>,
    max: Option<String>,
    min_inclusive: bool,
    max_inclusive: bool,
  },
  StringSet {
    values: Vec<String>,
  },
  #[default]
  Placeholder,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn serializes_codegen_ir_with_pascal_case_keys() {
    let module = SchemaModuleDecl {
      module_name: "schemas_example".to_string(),
      target_namespace: "urn:example".to_string(),
      prefix: "ex".to_string(),
      typed_namespace: "DocumentFormat.OpenXml.Example".to_string(),
      enums: vec![EnumDecl {
        rust_name: "ExampleEnum".to_string(),
        docs: String::new(),
        version: Some("Office2007".to_string()),
        value_type: EnumValueType::StringLike,
        variants: vec![EnumVariantDecl {
          rust_name: "Foo".to_string(),
          xml_value: "foo".to_string(),
          aliases: vec!["bar".to_string()],
          version: "Office2007".to_string(),
        }],
      }],
      types: vec![TypeDecl {
        rust_name: "ExampleType".to_string(),
        xml_qname: Some("ex:CT_Example/ex:example".to_string()),
        docs: "Example docs".to_string(),
        version: Some("Office2007".to_string()),
        is_abstract: false,
        kind: TypeKind::ElementStruct,
        element_kind: Some(ElementKind::Composite),
        content_model: Some(ContentModelDecl::GenericChildrenFallback),
        base_rust_name: None,
        xml_content: None,
        support: SystemSupportDecl {
          xmlns_mode: XmlnsMode::MapOnly,
          xml_header: XmlHeaderMode::Standalone,
          has_mce: true,
        },
        members: vec![
          MemberDecl::Field(FieldDecl {
            rust_name: "id".to_string(),
            docs: String::new(),
            version: "Office2007".to_string(),
            wire: FieldWireDecl::Attribute {
              qname: "ex:id".to_string(),
              bit: Some(1),
            },
            cardinality: Cardinality::One,
            type_ref: TypeRefDecl {
              rust_type: "StringValue".to_string(),
              module_path: Some("crate::simple_type".to_string()),
            },
            validators: vec![ValidatorDecl {
              version: "Office2007".to_string(),
              kind: ValidatorKind::Pattern {
                regex: "[A-Z]+".to_string(),
              },
            }],
          }),
          MemberDecl::Variant(VariantDecl {
            rust_name: "UnknownXml".to_string(),
            docs: String::new(),
            version: "Office2007".to_string(),
            wire: VariantWireDecl::Any,
            payload: TypeRefDecl {
              rust_type: "String".to_string(),
              module_path: None,
            },
          }),
        ],
      }],
    };

    let json = serde_json::to_value(&module).unwrap();
    assert!(json.get("ModuleName").is_some());
    assert!(json.get("TargetNamespace").is_some());
    assert!(json.get("Types").is_some());

    let ty = &json["Types"][0];
    assert!(ty.get("RustName").is_some());
    assert!(ty.get("XmlQName").is_some());
    assert!(ty.get("Support").is_some());

    let attr_field = &ty["Members"][0]["Field"];
    assert_eq!(attr_field["Wire"]["Attribute"]["QName"], "ex:id");
    assert_eq!(attr_field["TypeRef"]["RustType"], "StringValue");
    assert_eq!(
      attr_field["Validators"][0]["Kind"]["Pattern"]["Regex"],
      "[A-Z]+"
    );
  }

  #[test]
  fn round_trips_codegen_ir_json() {
    let module = SchemaModuleDecl {
      module_name: "schemas_example".to_string(),
      target_namespace: "urn:example".to_string(),
      prefix: "ex".to_string(),
      typed_namespace: "DocumentFormat.OpenXml.Example".to_string(),
      enums: vec![],
      types: vec![TypeDecl {
        rust_name: "ExampleChoice".to_string(),
        xml_qname: None,
        docs: String::new(),
        version: None,
        is_abstract: false,
        kind: TypeKind::ChoiceEnum,
        element_kind: None,
        content_model: None,
        base_rust_name: None,
        xml_content: None,
        support: SystemSupportDecl::default(),
        members: vec![MemberDecl::Variant(VariantDecl {
          rust_name: "Text".to_string(),
          docs: String::new(),
          version: String::new(),
          wire: VariantWireDecl::TextChild {
            qnames: vec!["w:t".to_string(), "m:t".to_string()],
          },
          payload: TypeRefDecl {
            rust_type: "StringValue".to_string(),
            module_path: Some("crate::simple_type".to_string()),
          },
        })],
      }],
    };

    let json = serde_json::to_string_pretty(&module).unwrap();
    let decoded: SchemaModuleDecl = serde_json::from_str(&json).unwrap();
    assert_eq!(decoded, module);
  }
}
