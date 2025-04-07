use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;
use syn::{parse2, parse_str, Ident, ItemEnum, Type, Variant};

use crate::gen::context::check_office_version;
use crate::gen::simple_type::simple_type_mapping;
use crate::models::{
  OpenXmlNamespace, OpenXmlSchema, OpenXmlSchemaType, OpenXmlSchemaTypeAttribute,
  OpenXmlSchemaTypeChild,
};
use crate::utils::{escape_snake_case, escape_upper_camel_case};
use crate::{GenContext, GenContextNeo};

pub fn gen_open_xml_schema_neo(schema: &OpenXmlSchema, gen_context: &GenContextNeo) -> TokenStream {
  let mut token_stream_list: Vec<TokenStream> = vec![];

  let schema_namespace = gen_context
    .uri_namespace_map
    .get(schema.target_namespace.as_str())
    .ok_or(format!("{:?}", schema.target_namespace))
    .unwrap();

  for e in &schema.enums {
    if !check_office_version(&e.version) {
      continue;
    }

    let e_enum_name_ident: Ident = parse_str(&e.name.to_upper_camel_case()).unwrap();

    let mut variants: Vec<Variant> = vec![];

    for (i, facet) in e.facets.iter().enumerate() {
      if !check_office_version(&facet.version) {
        continue;
      }

      let variant_ident: Ident = if facet.name.is_empty() {
        parse_str(&escape_upper_camel_case(facet.value.to_upper_camel_case())).unwrap()
      } else {
        parse_str(&escape_upper_camel_case(facet.name.to_upper_camel_case())).unwrap()
      };

      if i == 0 {
        variants.push(
          parse2(quote! {
            #[default]
            #variant_ident
          })
          .unwrap(),
        );
      } else {
        variants.push(
          parse2(quote! {
            #variant_ident
          })
          .unwrap(),
        );
      }
    }

    token_stream_list.push(quote! {
      #[derive(Clone, Default)]
      pub enum #e_enum_name_ident {
        #( #variants, )*
      }
    })
  }

  for t in &schema.types {
    if !check_office_version(&t.version) {
      continue;
    }

    let mut fields: Vec<TokenStream> = vec![];

    let mut child_choice_enum_option: Option<ItemEnum> = None;

    if t.base_class == "OpenXmlLeafTextElement" {
      for attr in &t.attributes {
        if check_office_version(&attr.version) {
          fields.push(gen_attr_neo(attr, schema_namespace, gen_context));
        }
      }

      let simple_type_name = gen_child_type_neo(t, schema_namespace, gen_context);

      fields.push(quote! {
        pub child: Option<#simple_type_name>,
      });
    } else if t.base_class == "OpenXmlLeafElement" {
      for attr in &t.attributes {
        if check_office_version(&attr.version) {
          fields.push(gen_attr_neo(attr, schema_namespace, gen_context));
        }
      }
    } else if t.base_class == "OpenXmlCompositeElement"
      || t.base_class == "CustomXmlElement"
      || t.base_class == "OpenXmlPartRootElement"
      || t.base_class == "SdtElement"
    {
      if !t.part.is_empty()
        || t.base_class == "OpenXmlPartRootElement"
        || schema.target_namespace == "http://schemas.openxmlformats.org/drawingml/2006/main"
        || schema.target_namespace == "http://schemas.openxmlformats.org/drawingml/2006/picture"
      {
        fields.push(quote! {
          pub xmlns: Option<String>,
        });

        fields.push(quote! {
          pub xmlns_map: std::collections::HashMap<String, String>,
        });

        fields.push(quote! {
          pub mc_ignorable: Option<String>,
        });
      }

      for attr in &t.attributes {
        if check_office_version(&attr.version) {
          fields.push(gen_attr_neo(attr, schema_namespace, gen_context));
        }
      }

      if t.is_one_sequence_flatten() {
        let one_sequence_fields = gen_one_sequence_fields_neo(t, schema_namespace, gen_context);

        fields.extend(one_sequence_fields);
      } else {
        let (field_option, enum_option) =
          gen_children_neo(&t.class_name, &t.children, schema_namespace, gen_context);

        if let Some(field) = field_option {
          fields.push(field);
        }

        child_choice_enum_option = enum_option;
      }
    } else if t.is_derived {
      let base_class_type = gen_context
        .type_name_type_map
        .get(&t.name[0..t.name.find('/').unwrap()])
        .ok_or(format!("{:?}", t.name))
        .unwrap();

      for attr in &t.attributes {
        if check_office_version(&attr.version) {
          fields.push(gen_attr_neo(attr, schema_namespace, gen_context));
        }
      }

      for attr in &base_class_type.attributes {
        if check_office_version(&attr.version) {
          fields.push(gen_attr_neo(attr, schema_namespace, gen_context));
        }
      }

      if t.is_one_sequence_flatten() && base_class_type.composite_type == "OneSequence" {
        let one_sequence_fields = gen_one_sequence_fields_neo(t, schema_namespace, gen_context);

        fields.extend(one_sequence_fields);
      } else {
        let (field_option, enum_option) =
          gen_children_neo(&t.class_name, &t.children, schema_namespace, gen_context);

        if let Some(field) = field_option {
          fields.push(field);
        }

        child_choice_enum_option = enum_option;
      }

      if t.children.is_empty() && base_class_type.base_class == "OpenXmlLeafTextElement" {
        let simple_type_name = gen_child_type_neo(base_class_type, schema_namespace, gen_context);

        fields.push(quote! {
          pub child: Option<#simple_type_name>,
        });
      }
    } else {
      panic!("{:?}", t);
    }

    let struct_name_ident: Ident = parse_str(&t.class_name.to_upper_camel_case()).unwrap();

    let summary_doc = format!(" {}", t.summary);

    let version_doc = if t.version.is_empty() {
      " Available in Office2007 and above.".to_string()
    } else {
      format!(" Available in {} and above.", t.version)
    };

    let qualified_doc = if t.name.ends_with('/') {
      " When the object is serialized out as xml, it's qualified name is .".to_string()
    } else {
      let qualified_str = &t.name[t.name.find('/').unwrap() + 1..t.name.len()];

      format!(
        " When the object is serialized out as xml, it's qualified name is {}.",
        qualified_str
      )
    };

    token_stream_list.push(quote! {
      #[doc = #summary_doc]
      #[doc = #version_doc]
      #[doc = #qualified_doc]
      #[derive(Clone, Default)]
      pub struct #struct_name_ident {
        #( #fields )*
      }

      #child_choice_enum_option
    });
  }

  quote! {
    #( #token_stream_list )*
  }
}

fn gen_attr_neo(
  attr: &OpenXmlSchemaTypeAttribute,
  schema_namespace: &OpenXmlNamespace,
  gen_context: &GenContextNeo,
) -> TokenStream {
  let attr_name_ident: Ident = if attr.property_name.is_empty() {
    parse_str(&escape_snake_case(attr.q_name.to_snake_case())).unwrap()
  } else {
    parse_str(&escape_snake_case(attr.property_name.to_snake_case())).unwrap()
  };

  let type_ident: Type = if attr.r#type.starts_with("ListValue<") {
    parse_str("String").unwrap()
  } else if attr.r#type.starts_with("EnumValue<") {
    let e_typed_namespace_str =
      &attr.r#type[attr.r#type.find("<").unwrap() + 1..attr.r#type.rfind(".").unwrap()];

    let enum_name = &attr.r#type[attr.r#type.rfind(".").unwrap() + 1..attr.r#type.len() - 1];

    let mut e_prefix = "";

    for typed_namespace in &gen_context.typed_namespaces {
      if e_typed_namespace_str == typed_namespace.namespace {
        let e_schema = gen_context
          .prefix_schema_map
          .get(typed_namespace.prefix.as_str())
          .ok_or(format!("{:?}", typed_namespace))
          .unwrap();

        for e in &e_schema.enums {
          if e.name == enum_name {
            e_prefix = &typed_namespace.prefix;
          }
        }
      }
    }

    let e_namespace = gen_context
      .prefix_namespace_map
      .get(e_prefix)
      .ok_or(format!("{:?}", e_prefix))
      .unwrap();

    if e_namespace.prefix != schema_namespace.prefix {
      let scheme_mod = gen_context
        .prefix_schema_mod_map
        .get(e_namespace.prefix.as_str())
        .ok_or(format!("{:?}", e_namespace.prefix))
        .unwrap();

      parse_str(&format!(
        "crate::schemas::{}::{}",
        scheme_mod,
        enum_name.to_upper_camel_case()
      ))
      .unwrap()
    } else {
      parse_str(&enum_name.to_upper_camel_case()).unwrap()
    }
  } else {
    parse_str(&format!("crate::schemas::simple_type::{}", &attr.r#type)).unwrap()
  };

  let mut required = false;

  for validator in &attr.validators {
    if validator.name == "RequiredValidator" {
      required = true;
    }
  }

  let property_comments_doc = format!(" {}", attr.property_comments);

  let version_doc = if attr.version.is_empty() {
    " Available in Office2007 and above.".to_string()
  } else {
    format!(" Available in {} and above.", attr.version)
  };

  let qualified_doc = format!(
    " Represents the following attribute in the schema: {}",
    attr.q_name
  );

  if required {
    quote! {
      #[doc = #property_comments_doc]
      #[doc = #version_doc]
      #[doc = #qualified_doc]
      pub #attr_name_ident: #type_ident,
    }
  } else {
    quote! {
      #[doc = #property_comments_doc]
      #[doc = #version_doc]
      #[doc = #qualified_doc]
      pub #attr_name_ident: Option<#type_ident>,
    }
  }
}

fn gen_children_neo(
  class_name: &str,
  children: &Vec<OpenXmlSchemaTypeChild>,
  schema_namespace: &OpenXmlNamespace,
  gen_context: &GenContextNeo,
) -> (Option<TokenStream>, Option<ItemEnum>) {
  if children.is_empty() {
    return (None, None);
  }

  let child_choice_enum_ident: Ident =
    parse_str(&format!("{}ChildChoice", class_name.to_upper_camel_case())).unwrap();

  let field_option = Some(quote! {
    pub children: Vec<#child_choice_enum_ident>,
  });

  let mut variants: Vec<TokenStream> = vec![];

  for child in children {
    let child_type = gen_context
      .type_name_type_map
      .get(&child.name[0..child.name.find('/').unwrap()])
      .ok_or(format!("{:?}", child.name))
      .unwrap();

    let child_namespace = gen_context
      .type_name_namespace_map
      .get(&child.name[0..child.name.find('/').unwrap()])
      .ok_or(format!("{:?}", child.name))
      .unwrap();

    let child_name_list: Vec<&str> = child.name.split('/').collect();

    let child_rename_ser_str = child_name_list
      .last()
      .ok_or(format!("{:?}", child.name))
      .unwrap();

    let child_variant_name_ident: Ident =
      parse_str(&child_rename_ser_str.to_upper_camel_case()).unwrap();

    let child_variant_type: Type = if child_namespace.prefix != schema_namespace.prefix {
      let scheme_mod = gen_context
        .prefix_schema_mod_map
        .get(child_namespace.prefix.as_str())
        .ok_or(format!("{:?}", child_namespace.prefix))
        .unwrap();

      parse_str(&format!(
        "crate::schemas::{}::{}",
        scheme_mod,
        child_type.class_name.to_upper_camel_case()
      ))
      .unwrap()
    } else {
      parse_str(&child_type.class_name.to_upper_camel_case()).unwrap()
    };

    variants.push(quote! {
      #child_variant_name_ident(std::boxed::Box<#child_variant_type>),
    });
  }

  let enum_option = Some(
    parse2(quote! {
      #[derive(Clone, Debug)]
      pub enum #child_choice_enum_ident {
        #( #variants )*
      }
    })
    .unwrap(),
  );

  (field_option, enum_option)
}

fn gen_child_type_neo(
  t: &OpenXmlSchemaType,
  schema_namespace: &OpenXmlNamespace,
  gen_context: &GenContextNeo,
) -> Type {
  let name_list: Vec<&str> = t.name.split('/').collect();

  let first_name = name_list.first().ok_or(format!("{:?}", t.name)).unwrap();

  if let Some(e) = gen_context.enum_type_enum_map.get(first_name) {
    let e_namespace = gen_context
      .enum_type_namespace_map
      .get(e.r#type.as_str())
      .ok_or(format!("{:?}", e.r#type))
      .unwrap();

    if e_namespace.prefix != schema_namespace.prefix {
      let scheme_mod = gen_context
        .prefix_schema_mod_map
        .get(e_namespace.prefix.as_str())
        .ok_or(format!("{:?}", e_namespace.prefix))
        .unwrap();

      parse_str(&format!(
        "crate::schemas::{}::{}",
        scheme_mod,
        e.name.to_upper_camel_case()
      ))
      .unwrap()
    } else {
      parse_str(&e.name.to_upper_camel_case()).unwrap()
    }
  } else {
    parse_str(&format!(
      "crate::schemas::simple_type::{}",
      simple_type_mapping(first_name)
    ))
    .unwrap()
  }
}

fn gen_one_sequence_fields_neo(
  t: &OpenXmlSchemaType,
  schema_namespace: &OpenXmlNamespace,
  gen_context: &GenContextNeo,
) -> Vec<TokenStream> {
  let mut fields: Vec<TokenStream> = vec![];

  let mut child_map: HashMap<&str, &OpenXmlSchemaTypeChild> = HashMap::new();

  for child in &t.children {
    child_map.insert(&child.name, child);
  }

  for p in &t.particle.items {
    let child = child_map
      .get(p.name.as_str())
      .ok_or(format!("{:?}", p.name))
      .unwrap();

    let child_type = gen_context
      .type_name_type_map
      .get(child.name.as_str())
      .ok_or(format!("{:?}", child.name))
      .unwrap();

    let child_namespace = gen_context
      .type_name_namespace_map
      .get(child.name.as_str())
      .ok_or(format!("{:?}", child.name))
      .unwrap();

    let child_variant_type: Type = if child_namespace.prefix != schema_namespace.prefix {
      let scheme_mod = gen_context
        .prefix_schema_mod_map
        .get(child_namespace.prefix.as_str())
        .ok_or(format!("{:?}", child_namespace.prefix))
        .unwrap();

      parse_str(&format!(
        "crate::schemas::{}::{}",
        scheme_mod,
        child_type.class_name.to_upper_camel_case()
      ))
      .unwrap()
    } else {
      parse_str(&child_type.class_name.to_upper_camel_case()).unwrap()
    };

    let child_name_ident: Ident = if child.property_name.is_empty() {
      let child_name_list: Vec<&str> = child.name.split('/').collect();

      let child_rename_ser_str = child_name_list
        .last()
        .ok_or(format!("{:?}", child.name))
        .unwrap();

      parse_str(&child_rename_ser_str.to_snake_case()).unwrap()
    } else {
      parse_str(&escape_snake_case(child.property_name.to_snake_case())).unwrap()
    };

    let property_comments = if child.property_comments.is_empty() {
      " _"
    } else {
      &child.property_comments
    };

    if p.occurs.is_empty() {
      fields.push(quote! {
        #[doc = #property_comments]
        pub #child_name_ident: std::boxed::Box<#child_variant_type>,
      });
    } else if p.occurs[0].min == 0 && p.occurs[0].max == 1 {
      fields.push(quote! {
        #[doc = #property_comments]
        pub #child_name_ident: Option<std::boxed::Box<#child_variant_type>>,
      });
    } else {
      fields.push(quote! {
        #[doc = #property_comments]
        pub #child_name_ident: Vec<#child_variant_type>,
      });
    }
  }

  fields
}

pub fn gen_open_xml_schema(schema: &OpenXmlSchema, context: &GenContext) -> TokenStream {
  let mut token_stream_list: Vec<TokenStream> = vec![];

  let schema_namespace = context
    .uri_namespace_map
    .get(schema.target_namespace.as_str())
    .ok_or(format!("{:?}", schema.target_namespace))
    .unwrap();

  for e in &schema.enums {
    let e_enum_name_ident: Ident = parse_str(&e.name.to_upper_camel_case()).unwrap();

    let mut variants: Vec<Variant> = vec![];

    for (i, facet) in e.facets.iter().enumerate() {
      let variant_ident: Ident = if facet.name.is_empty() {
        parse_str(&escape_upper_camel_case(facet.value.to_upper_camel_case())).unwrap()
      } else {
        parse_str(&escape_upper_camel_case(facet.name.to_upper_camel_case())).unwrap()
      };

      if i == 0 {
        variants.push(
          parse2(quote! {
            #[default]
            #variant_ident
          })
          .unwrap(),
        );
      } else {
        variants.push(
          parse2(quote! {
            #variant_ident
          })
          .unwrap(),
        );
      }
    }

    token_stream_list.push(quote! {
      #[derive(Clone, Debug, Default)]
      pub enum #e_enum_name_ident {
        #( #variants, )*
      }
    })
  }

  for t in &schema.types {
    let mut fields: Vec<TokenStream> = vec![];

    let mut child_choice_enum_option: Option<ItemEnum> = None;

    if t.base_class == "OpenXmlLeafTextElement" {
      for attr in &t.attributes {
        fields.push(gen_attr(attr, schema_namespace, context));
      }

      let simple_type_name = gen_child_type(t, schema_namespace, context);

      fields.push(quote! {
        pub child: Option<#simple_type_name>,
      });
    } else if t.base_class == "OpenXmlLeafElement" {
      for attr in &t.attributes {
        fields.push(gen_attr(attr, schema_namespace, context));
      }
    } else if t.base_class == "OpenXmlCompositeElement"
      || t.base_class == "CustomXmlElement"
      || t.base_class == "OpenXmlPartRootElement"
      || t.base_class == "SdtElement"
    {
      if !t.part.is_empty()
        || t.base_class == "OpenXmlPartRootElement"
        || schema.target_namespace == "http://schemas.openxmlformats.org/drawingml/2006/main"
        || schema.target_namespace == "http://schemas.openxmlformats.org/drawingml/2006/picture"
      {
        fields.push(quote! {
          pub xmlns: Option<String>,
        });

        fields.push(quote! {
          pub xmlns_map: std::collections::HashMap<String, String>,
        });

        fields.push(quote! {
          pub mc_ignorable: Option<String>,
        });
      }

      for attr in &t.attributes {
        fields.push(gen_attr(attr, schema_namespace, context));
      }

      if t.is_one_sequence_flatten() {
        let one_sequence_fields = gen_one_sequence_fields(t, schema_namespace, context);

        fields.extend(one_sequence_fields);
      } else {
        let (field_option, enum_option) =
          gen_children(&t.class_name, &t.children, schema_namespace, context);

        if let Some(field) = field_option {
          fields.push(field);
        }

        child_choice_enum_option = enum_option;
      }
    } else if t.is_derived {
      let base_class_type = context
        .type_name_type_map
        .get(&t.name[0..t.name.find('/').unwrap() + 1])
        .ok_or(format!("{:?}", t.base_class))
        .unwrap();

      for attr in &t.attributes {
        fields.push(gen_attr(attr, schema_namespace, context));
      }

      for attr in &base_class_type.attributes {
        fields.push(gen_attr(attr, schema_namespace, context));
      }

      if t.is_one_sequence_flatten() && base_class_type.composite_type == "OneSequence" {
        let one_sequence_fields = gen_one_sequence_fields(t, schema_namespace, context);

        fields.extend(one_sequence_fields);
      } else {
        let (field_option, enum_option) =
          gen_children(&t.class_name, &t.children, schema_namespace, context);

        if let Some(field) = field_option {
          fields.push(field);
        }

        child_choice_enum_option = enum_option;
      }

      if t.children.is_empty() && base_class_type.base_class == "OpenXmlLeafTextElement" {
        let simple_type_name = gen_child_type(base_class_type, schema_namespace, context);

        fields.push(quote! {
          pub child: Option<#simple_type_name>,
        });
      }
    } else {
      panic!("{:?}", t);
    }

    let struct_name_ident: Ident = parse_str(&t.class_name.to_upper_camel_case()).unwrap();

    let summary_doc = format!(" {}", t.summary);

    let qualified_doc = if t.name.ends_with('/') {
      " When the object is serialized out as xml, it's qualified name is .".to_string()
    } else {
      let qualified_str = &t.name[t.name.find('/').unwrap() + 1..t.name.len()];

      format!(
        " When the object is serialized out as xml, it's qualified name is {}.",
        qualified_str
      )
    };

    if let Some(child_choice_enum) = child_choice_enum_option {
      token_stream_list.push(quote! {
        #[doc = #summary_doc]
        #[doc = #qualified_doc]
        #[derive(Clone, Debug, Default)]
        pub struct #struct_name_ident {
          #( #fields )*
        }

        #child_choice_enum
      });
    } else {
      token_stream_list.push(quote! {
        #[doc = #summary_doc]
        #[doc = #qualified_doc]
        #[derive(Clone, Debug, Default)]
        pub struct #struct_name_ident {
          #( #fields )*
        }
      });
    }
  }

  quote! {
    #( #token_stream_list )*
  }
}

fn gen_attr(
  attr: &OpenXmlSchemaTypeAttribute,
  schema_namespace: &OpenXmlNamespace,
  context: &GenContext,
) -> TokenStream {
  let attr_name_ident: Ident = if attr.property_name.is_empty() {
    parse_str(&escape_snake_case(attr.q_name.to_snake_case())).unwrap()
  } else {
    parse_str(&escape_snake_case(attr.property_name.to_snake_case())).unwrap()
  };

  let type_ident: Type = if attr.r#type.starts_with("ListValue<") {
    parse_str("String").unwrap()
  } else if attr.r#type.starts_with("EnumValue<") {
    let e_typed_namespace_str =
      &attr.r#type[attr.r#type.find("<").unwrap() + 1..attr.r#type.rfind(".").unwrap()];

    let enum_name = &attr.r#type[attr.r#type.rfind(".").unwrap() + 1..attr.r#type.len() - 1];

    let mut e_prefix = "";

    for typed_namespace in &context.typed_namespaces {
      if e_typed_namespace_str == typed_namespace.namespace {
        let e_schema = context
          .prefix_schema_map
          .get(typed_namespace.prefix.as_str())
          .ok_or(format!("{:?}", typed_namespace))
          .unwrap();

        for e in &e_schema.enums {
          if e.name == enum_name {
            e_prefix = &typed_namespace.prefix;
          }
        }
      }
    }

    let e_namespace = context
      .prefix_namespace_map
      .get(e_prefix)
      .ok_or(format!("{:?}", e_prefix))
      .unwrap();

    if e_namespace.prefix != schema_namespace.prefix {
      let scheme_mod = context
        .prefix_schema_mod_map
        .get(e_namespace.prefix.as_str())
        .ok_or(format!("{:?}", e_namespace.prefix))
        .unwrap();

      parse_str(&format!(
        "crate::schemas::{}::{}",
        scheme_mod,
        enum_name.to_upper_camel_case()
      ))
      .unwrap()
    } else {
      parse_str(&enum_name.to_upper_camel_case()).unwrap()
    }
  } else {
    parse_str(&format!("crate::schemas::simple_type::{}", &attr.r#type)).unwrap()
  };

  let mut required = false;

  for validator in &attr.validators {
    if validator.name == "RequiredValidator" {
      required = true;
    }
  }

  let property_comments_doc = format!(" {}", attr.property_comments);

  let qualified_doc = format!(
    " Represents the following attribute in the schema: {}",
    attr.q_name
  );

  if required {
    quote! {
      #[doc = #property_comments_doc]
      #[doc = #qualified_doc]
      pub #attr_name_ident: #type_ident,
    }
  } else {
    quote! {
      #[doc = #property_comments_doc]
      #[doc = #qualified_doc]
      pub #attr_name_ident: Option<#type_ident>,
    }
  }
}

fn gen_children(
  class_name: &str,
  children: &Vec<OpenXmlSchemaTypeChild>,
  schema_namespace: &OpenXmlNamespace,
  context: &GenContext,
) -> (Option<TokenStream>, Option<ItemEnum>) {
  if children.is_empty() {
    return (None, None);
  }

  let child_choice_enum_ident: Ident =
    parse_str(&format!("{}ChildChoice", class_name.to_upper_camel_case())).unwrap();

  let field_option = Some(quote! {
    pub children: Vec<#child_choice_enum_ident>,
  });

  let mut variants: Vec<TokenStream> = vec![];

  for child in children {
    let child_type = context
      .type_name_type_map
      .get(child.name.as_str())
      .ok_or(format!("{:?}", child.name))
      .unwrap();

    let child_namespace = context
      .type_name_namespace_map
      .get(child.name.as_str())
      .ok_or(format!("{:?}", child.name))
      .unwrap();

    let child_name_list: Vec<&str> = child.name.split('/').collect();

    let child_rename_ser_str = child_name_list
      .last()
      .ok_or(format!("{:?}", child.name))
      .unwrap();

    let child_variant_name_ident: Ident =
      parse_str(&child_rename_ser_str.to_upper_camel_case()).unwrap();

    let child_variant_type: Type = if child_namespace.prefix != schema_namespace.prefix {
      let scheme_mod = context
        .prefix_schema_mod_map
        .get(child_namespace.prefix.as_str())
        .ok_or(format!("{:?}", child_namespace.prefix))
        .unwrap();

      parse_str(&format!(
        "crate::schemas::{}::{}",
        scheme_mod,
        child_type.class_name.to_upper_camel_case()
      ))
      .unwrap()
    } else {
      parse_str(&child_type.class_name.to_upper_camel_case()).unwrap()
    };

    variants.push(quote! {
      #child_variant_name_ident(std::boxed::Box<#child_variant_type>),
    });
  }

  let enum_option = Some(
    parse2(quote! {
      #[derive(Clone, Debug)]
      pub enum #child_choice_enum_ident {
        #( #variants )*
      }
    })
    .unwrap(),
  );

  (field_option, enum_option)
}

fn gen_child_type(
  t: &OpenXmlSchemaType,
  schema_namespace: &OpenXmlNamespace,
  context: &GenContext,
) -> Type {
  let name_list: Vec<&str> = t.name.split('/').collect();

  let first_name = name_list.first().ok_or(format!("{:?}", t.name)).unwrap();

  if let Some(e) = context.enum_type_enum_map.get(first_name) {
    let e_namespace = context
      .enum_type_namespace_map
      .get(e.r#type.as_str())
      .ok_or(format!("{:?}", e.r#type))
      .unwrap();

    if e_namespace.prefix != schema_namespace.prefix {
      let scheme_mod = context
        .prefix_schema_mod_map
        .get(e_namespace.prefix.as_str())
        .ok_or(format!("{:?}", e_namespace.prefix))
        .unwrap();

      parse_str(&format!(
        "crate::schemas::{}::{}",
        scheme_mod,
        e.name.to_upper_camel_case()
      ))
      .unwrap()
    } else {
      parse_str(&e.name.to_upper_camel_case()).unwrap()
    }
  } else {
    parse_str(&format!(
      "crate::schemas::simple_type::{}",
      simple_type_mapping(first_name)
    ))
    .unwrap()
  }
}

fn gen_one_sequence_fields(
  t: &OpenXmlSchemaType,
  schema_namespace: &OpenXmlNamespace,
  context: &GenContext,
) -> Vec<TokenStream> {
  let mut fields: Vec<TokenStream> = vec![];

  let mut child_map: HashMap<&str, &OpenXmlSchemaTypeChild> = HashMap::new();

  for child in &t.children {
    child_map.insert(&child.name, child);
  }

  for p in &t.particle.items {
    let child = child_map
      .get(p.name.as_str())
      .ok_or(format!("{:?}", p.name))
      .unwrap();

    let child_type = context
      .type_name_type_map
      .get(child.name.as_str())
      .ok_or(format!("{:?}", child.name))
      .unwrap();

    let child_namespace = context
      .type_name_namespace_map
      .get(child.name.as_str())
      .ok_or(format!("{:?}", child.name))
      .unwrap();

    let child_variant_type: Type = if child_namespace.prefix != schema_namespace.prefix {
      let scheme_mod = context
        .prefix_schema_mod_map
        .get(child_namespace.prefix.as_str())
        .ok_or(format!("{:?}", child_namespace.prefix))
        .unwrap();

      parse_str(&format!(
        "crate::schemas::{}::{}",
        scheme_mod,
        child_type.class_name.to_upper_camel_case()
      ))
      .unwrap()
    } else {
      parse_str(&child_type.class_name.to_upper_camel_case()).unwrap()
    };

    let child_name_ident: Ident = if child.property_name.is_empty() {
      let child_name_list: Vec<&str> = child.name.split('/').collect();

      let child_rename_ser_str = child_name_list
        .last()
        .ok_or(format!("{:?}", child.name))
        .unwrap();

      parse_str(&child_rename_ser_str.to_snake_case()).unwrap()
    } else {
      parse_str(&escape_snake_case(child.property_name.to_snake_case())).unwrap()
    };

    let property_comments = if child.property_comments.is_empty() {
      " _"
    } else {
      &child.property_comments
    };

    if p.occurs.is_empty() {
      fields.push(quote! {
        #[doc = #property_comments]
        pub #child_name_ident: std::boxed::Box<#child_variant_type>,
      });
    } else if p.occurs[0].min == 0 && p.occurs[0].max == 1 {
      fields.push(quote! {
        #[doc = #property_comments]
        pub #child_name_ident: Option<std::boxed::Box<#child_variant_type>>,
      });
    } else {
      fields.push(quote! {
        #[doc = #property_comments]
        pub #child_name_ident: Vec<#child_variant_type>,
      });
    }
  }

  fields
}
