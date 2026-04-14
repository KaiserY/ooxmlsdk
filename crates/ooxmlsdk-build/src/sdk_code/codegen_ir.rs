#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SchemaModuleDecl {
  pub module_name: String,
  pub target_namespace: String,
  pub prefix: String,
  pub typed_namespace: String,
  pub enums: Vec<EnumDecl>,
  pub types: Vec<TypeDecl>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct EnumDecl {
  pub rust_name: String,
  pub docs: String,
  pub version: Option<String>,
  pub value_type: EnumValueType,
  pub variants: Vec<EnumVariantDecl>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum EnumValueType {
  #[default]
  StringLike,
  NumericLike,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct EnumVariantDecl {
  pub rust_name: String,
  pub xml_value: String,
  pub aliases: Vec<String>,
  pub version: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TypeDecl {
  pub rust_name: String,
  pub xml_qname: Option<String>,
  pub docs: String,
  pub version: Option<String>,
  pub kind: TypeKind,
  pub support: SystemSupportDecl,
  pub members: Vec<MemberDecl>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TypeKind {
  #[default]
  ElementStruct,
  ChoiceEnum,
  HelperStruct,
  LeafTextAlias,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SystemSupportDecl {
  pub xmlns_mode: XmlnsMode,
  pub xml_header: XmlHeaderMode,
  pub has_mce: bool,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum XmlnsMode {
  #[default]
  None,
  MapOnly,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum XmlHeaderMode {
  #[default]
  None,
  Plain,
  Standalone,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MemberDecl {
  Field(FieldDecl),
  Variant(VariantDecl),
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FieldDecl {
  pub rust_name: String,
  pub docs: String,
  pub version: String,
  pub wire: FieldWireDecl,
  pub cardinality: Cardinality,
  pub type_ref: TypeRefDecl,
  pub validators: Vec<ValidatorDecl>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum FieldWireDecl {
  Attribute {
    qname: String,
    bit: Option<u32>,
  },
  Child {
    qname: String,
  },
  TextChild {
    qname: String,
  },
  #[default]
  Choice,
  Any,
  Text,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct VariantDecl {
  pub rust_name: String,
  pub docs: String,
  pub version: String,
  pub wire: VariantWireDecl,
  pub payload: TypeRefDecl,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum VariantWireDecl {
  Child {
    qnames: Vec<String>,
  },
  TextChild {
    qnames: Vec<String>,
  },
  Any,
  #[default]
  Text,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Cardinality {
  #[default]
  One,
  Optional,
  Many,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TypeRefDecl {
  pub rust_type: String,
  pub module_path: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ValidatorDecl {
  pub version: String,
  pub kind: ValidatorKind,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
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
