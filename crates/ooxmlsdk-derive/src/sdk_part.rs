use super::*;

pub(crate) fn expand_sdk_part(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
  let Data::Struct(data_struct) = &input.data else {
    return Err(syn::Error::new_spanned(
      input,
      "SdkPart can only be derived for structs",
    ));
  };

  let Fields::Named(fields) = &data_struct.fields else {
    return Err(syn::Error::new_spanned(
      input,
      "SdkPart can only be derived for named-field structs",
    ));
  };

  if !is_part_handle_struct(fields) {
    return Err(syn::Error::new_spanned(
      input,
      "SdkPart supports generated part handle structs only",
    ));
  }

  expand_part_handle(input, fields)
}

struct PartHandleChildInfo {
  field_ident: Ident,
  part_ty: Type,
  kind: PartChildKind,
  relationship_type: PartRelationshipTypeSource,
}

struct PartChildMarkerInfo {
  part_ty: Type,
  kind: PartChildKind,
}

struct PartHandleRootInfo {
  root_ty: Type,
}

fn is_part_handle_struct(fields: &syn::FieldsNamed) -> bool {
  fields
    .named
    .iter()
    .any(|field| field.ident.as_ref().is_some_and(|ident| ident == "id"))
    && fields.named.iter().all(|field| {
      field.ident.as_ref().is_some_and(|ident| {
        ident == "id" || ident == "relationship_id" || is_relationship_model_field(ident)
      }) || part_child_field_info(&field.ty).is_some()
        || marker_inner_type(&field.ty, "PartRoot").is_some()
    })
}

fn expand_part_handle(
  input: &DeriveInput,
  fields: &syn::FieldsNamed,
) -> syn::Result<proc_macro2::TokenStream> {
  let ident = &input.ident;
  let field_inits = fields.named.iter().map(|field| {
    let field_ident = field.ident.as_ref().unwrap();
    if field_ident == "id" {
      quote! { #field_ident: part_id }
    } else if field_ident == "relationship_id" {
      quote! { #field_ident: None }
    } else {
      quote! { #field_ident: Default::default() }
    }
  });
  let mut child_infos = Vec::new();
  let mut root_info = None;
  for field in &fields.named {
    if field.ident.as_ref().is_some_and(|ident| {
      ident == "id" || ident == "relationship_id" || is_relationship_model_field(ident)
    }) {
      continue;
    }

    if let Some(marker) = part_child_field_info(&field.ty) {
      let relationship_type = parse_part_child_relationship_type_attr(&field.attrs)?
        .unwrap_or(PartRelationshipTypeSource::TypeConst);
      let field_ident = field.ident.clone().unwrap();
      child_infos.push(PartHandleChildInfo {
        field_ident,
        part_ty: marker.part_ty,
        kind: parse_part_child_kind_attr(&field.attrs)?.unwrap_or(marker.kind),
        relationship_type,
      });
      continue;
    }

    if let Some(root_ty) = marker_inner_type(&field.ty, "PartRoot") {
      root_info = Some(PartHandleRootInfo { root_ty });
      continue;
    }
  }

  let root_method = part_handle_root_method_tokens(ident, root_info.as_ref());
  let child_methods = part_handle_child_methods_tokens(ident, &child_infos);
  Ok(quote! {
    impl crate::sdk::SdkPart for #ident {
      #[inline]
      fn from_part_id(part_id: crate::common::PartId) -> Self {
        Self {
          #( #field_inits, )*
        }
      }

      #[inline]
      fn from_relationship_id(
        relationship_id: impl Into<String>,
        part_id: crate::common::PartId,
      ) -> Self {
        let mut part = Self::from_part_id(part_id);
        part.relationship_id = Some(relationship_id.into());
        part
      }

      #[inline]
      fn set_relationship_id(&mut self, relationship_id: String) {
        self.relationship_id = Some(relationship_id);
      }

      #[inline]
      fn part_id(&self) -> crate::common::PartId {
        self.id
      }

      #[inline]
      fn relationship_id(&self) -> Option<&str> {
        self.relationship_id.as_deref()
      }
    }

    impl #ident {
      #[inline]
      pub(crate) const fn part_id(&self) -> crate::common::PartId {
        self.id
      }

      #[inline]
      pub fn path<'a, P: crate::sdk::SdkPackage>(&self, package: &'a P) -> Option<&'a str> {
        <Self as crate::sdk::SdkPart>::path(self, package)
      }

      #[inline]
      pub fn content_type<'a, P: crate::sdk::SdkPackage>(&self, package: &'a P) -> Option<&'a str> {
        <Self as crate::sdk::SdkPart>::content_type(self, package)
      }

      #[inline]
      pub fn data<'a, P: crate::sdk::SdkPackage>(&self, package: &'a P) -> Option<&'a [u8]> {
        <Self as crate::sdk::SdkPart>::data(self, package)
      }

      #[inline]
      pub fn data_to_vec<P: crate::sdk::SdkPackage>(&self, package: &P) -> Option<Vec<u8>> {
        <Self as crate::sdk::SdkPart>::data_to_vec(self, package)
      }

      #[inline]
      pub fn data_as_str<'a, P: crate::sdk::SdkPackage>(
        &self,
        package: &'a P,
      ) -> Result<Option<&'a str>, crate::common::SdkError> {
        <Self as crate::sdk::SdkPart>::data_as_str(self, package)
      }

      #[inline]
      pub fn write_data_to<P: crate::sdk::SdkPackage, W: std::io::Write>(
        &self,
        package: &P,
        writer: &mut W,
      ) -> Result<bool, crate::common::SdkError> {
        <Self as crate::sdk::SdkPart>::write_data_to(self, package, writer)
      }

      #[inline]
      pub fn set_data<P: crate::sdk::SdkPackage>(
        &self,
        package: &mut P,
        data: impl Into<Vec<u8>>,
      ) -> Result<(), crate::common::SdkError> {
        <Self as crate::sdk::SdkPart>::set_data(self, package, data)
      }

      #[inline]
      pub fn feed_data<P: crate::sdk::SdkPackage, R: std::io::Read>(
        &self,
        package: &mut P,
        reader: &mut R,
      ) -> Result<(), crate::common::SdkError> {
        <Self as crate::sdk::SdkPart>::feed_data(self, package, reader)
      }

      #[inline]
      pub fn external_relationships<'a, P: crate::sdk::SdkPackage>(
        &'a self,
        package: &'a P,
      ) -> impl Iterator<Item = crate::common::RelationshipRef<'a>> {
        <Self as crate::sdk::SdkPart>::external_relationships(self, package)
      }

      #[inline]
      pub fn hyperlink_relationships<'a, P: crate::sdk::SdkPackage>(
        &'a self,
        package: &'a P,
      ) -> impl Iterator<Item = crate::common::RelationshipRef<'a>> {
        <Self as crate::sdk::SdkPart>::hyperlink_relationships(self, package)
      }

      #[inline]
      pub fn data_part_reference_relationships<'a, P: crate::sdk::SdkPackage>(
        &'a self,
        package: &'a P,
      ) -> impl Iterator<Item = crate::common::RelationshipRef<'a>> {
        <Self as crate::sdk::SdkPart>::data_part_reference_relationships(self, package)
      }

      #[inline]
      pub fn add_external_relationship<'a, P: crate::sdk::SdkPackage>(
        &self,
        package: &'a mut P,
        relationship_id: impl Into<String>,
        relationship_type: impl Into<String>,
        target: impl Into<String>,
      ) -> Result<crate::common::RelationshipRef<'a>, crate::common::SdkError> {
        <Self as crate::sdk::SdkPart>::add_external_relationship(self,
          package,
          relationship_id,
          relationship_type,
          target,
        )
      }

      #[inline]
      pub fn add_external_relationship_auto_id<'a, P: crate::sdk::SdkPackage>(
        &self,
        package: &'a mut P,
        relationship_type: impl Into<String>,
        target: impl Into<String>,
      ) -> Result<crate::common::RelationshipRef<'a>, crate::common::SdkError> {
        <Self as crate::sdk::SdkPart>::add_external_relationship_auto_id(self,
          package,
          relationship_type,
          target,
        )
      }

      #[inline]
      pub fn add_hyperlink_relationship<'a, P: crate::sdk::SdkPackage>(
        &self,
        package: &'a mut P,
        relationship_id: impl Into<String>,
        target: impl Into<String>,
      ) -> Result<crate::common::RelationshipRef<'a>, crate::common::SdkError> {
        <Self as crate::sdk::SdkPart>::add_hyperlink_relationship(self,
          package,
          relationship_id,
          target,
        )
      }

      #[inline]
      pub fn add_hyperlink_relationship_with_mode<'a, P: crate::sdk::SdkPackage>(
        &self,
        package: &'a mut P,
        relationship_id: impl Into<String>,
        target: impl Into<String>,
        target_mode: crate::schemas::opc_relationships::TargetMode,
      ) -> Result<crate::common::RelationshipRef<'a>, crate::common::SdkError> {
        <Self as crate::sdk::SdkPart>::add_hyperlink_relationship_with_mode(self,
          package,
          relationship_id,
          target,
          target_mode,
        )
      }

      #[inline]
      pub fn add_hyperlink_relationship_auto_id<'a, P: crate::sdk::SdkPackage>(
        &self,
        package: &'a mut P,
        target: impl Into<String>,
        target_mode: crate::schemas::opc_relationships::TargetMode,
      ) -> Result<crate::common::RelationshipRef<'a>, crate::common::SdkError> {
        <Self as crate::sdk::SdkPart>::add_hyperlink_relationship_auto_id(self,
          package,
          target,
          target_mode,
        )
      }

      #[inline]
      pub fn add_new_part<P, T>(
        &self,
        package: &mut P,
        relationship_id: impl Into<String>,
      ) -> Result<T, crate::common::SdkError>
      where
        P: crate::sdk::SdkPackage,
        T: crate::sdk::SdkPart,
      {
        <Self as crate::sdk::SdkPart>::add_new_part(self,
          package,
          relationship_id,
        )
      }

      #[inline]
      pub fn add_new_part_with_content_type<P, T>(
        &self,
        package: &mut P,
        relationship_id: impl Into<String>,
        content_type: impl Into<std::borrow::Cow<'static, str>>,
      ) -> Result<T, crate::common::SdkError>
      where
        P: crate::sdk::SdkPackage,
        T: crate::sdk::SdkPart,
      {
        <Self as crate::sdk::SdkPart>::add_new_part_with_content_type(self,
          package,
          relationship_id,
          content_type,
        )
      }

      #[inline]
      pub fn add_new_part_auto_id<P, T>(
        &self,
        package: &mut P,
      ) -> Result<T, crate::common::SdkError>
      where
        P: crate::sdk::SdkPackage,
        T: crate::sdk::SdkPart,
      {
        <Self as crate::sdk::SdkPart>::add_new_part_auto_id(self, package)
      }

      #[inline]
      pub fn add_new_part_with_content_type_auto_id<P, T>(
        &self,
        package: &mut P,
        content_type: impl Into<std::borrow::Cow<'static, str>>,
      ) -> Result<T, crate::common::SdkError>
      where
        P: crate::sdk::SdkPackage,
        T: crate::sdk::SdkPart,
      {
        <Self as crate::sdk::SdkPart>::add_new_part_with_content_type_auto_id(self,
          package,
          content_type,
        )
      }

      #[inline]
      pub fn add_new_part_with_content_type_and_extension<P, T>(
        &self,
        package: &mut P,
        relationship_id: impl Into<String>,
        content_type: impl Into<std::borrow::Cow<'static, str>>,
        extension: impl Into<std::borrow::Cow<'static, str>>,
      ) -> Result<T, crate::common::SdkError>
      where
        P: crate::sdk::SdkPackage,
        T: crate::sdk::SdkPart,
      {
        <Self as crate::sdk::SdkPart>::add_new_part_with_content_type_and_extension(self,
          package,
          relationship_id,
          content_type,
          extension,
        )
      }

      #[inline]
      pub fn add_new_part_with_content_type_and_extension_auto_id<P, T>(
        &self,
        package: &mut P,
        content_type: impl Into<std::borrow::Cow<'static, str>>,
        extension: impl Into<std::borrow::Cow<'static, str>>,
      ) -> Result<T, crate::common::SdkError>
      where
        P: crate::sdk::SdkPackage,
        T: crate::sdk::SdkPart,
      {
        <Self as crate::sdk::SdkPart>::add_new_part_with_content_type_and_extension_auto_id(self,
          package,
          content_type,
          extension,
        )
      }

      #[inline]
      pub fn add_extended_part<P>(
        &self,
        package: &mut P,
        relationship_type: impl Into<String>,
        content_type: impl Into<std::borrow::Cow<'static, str>>,
        target_extension: impl Into<std::borrow::Cow<'static, str>>,
      ) -> Result<crate::parts::extended_part::ExtendedPart, crate::common::SdkError>
      where
        P: crate::sdk::SdkPackage,
      {
        <Self as crate::sdk::SdkPart>::add_extended_part(self,
          package,
          relationship_type,
          content_type,
          target_extension,
        )
      }

      #[inline]
      pub fn add_extended_part_with_id<P>(
        &self,
        package: &mut P,
        relationship_type: impl Into<String>,
        content_type: impl Into<std::borrow::Cow<'static, str>>,
        target_extension: impl Into<std::borrow::Cow<'static, str>>,
        relationship_id: impl Into<String>,
      ) -> Result<crate::parts::extended_part::ExtendedPart, crate::common::SdkError>
      where
        P: crate::sdk::SdkPackage,
      {
        <Self as crate::sdk::SdkPart>::add_extended_part_with_id(self,
          package,
          relationship_type,
          content_type,
          target_extension,
          relationship_id,
        )
      }

      #[inline]
      pub fn add_image_part<P>(
        &self,
        package: &mut P,
        content_type: impl Into<std::borrow::Cow<'static, str>>,
      ) -> Result<crate::parts::image_part::ImagePart, crate::common::SdkError>
      where
        P: crate::sdk::SdkPackage,
      {
        <Self as crate::sdk::SdkPart>::add_image_part(self,
          package,
          content_type,
        )
      }

      #[inline]
      pub fn add_image_part_with_id<P>(
        &self,
        package: &mut P,
        content_type: impl Into<std::borrow::Cow<'static, str>>,
        relationship_id: impl Into<String>,
      ) -> Result<crate::parts::image_part::ImagePart, crate::common::SdkError>
      where
        P: crate::sdk::SdkPackage,
      {
        <Self as crate::sdk::SdkPart>::add_image_part_with_id(self,
          package,
          content_type,
          relationship_id,
        )
      }

      #[inline]
      pub fn add_new_part_with_content_type_and_path<P, T>(
        &self,
        package: &mut P,
        relationship_id: impl Into<String>,
        content_type: impl Into<std::borrow::Cow<'static, str>>,
        part_path: impl AsRef<str>,
      ) -> Result<T, crate::common::SdkError>
      where
        P: crate::sdk::SdkPackage,
        T: crate::sdk::SdkPart,
      {
        <Self as crate::sdk::SdkPart>::add_new_part_with_content_type_and_path::<P, T>(
          self,
          package,
          relationship_id,
          content_type,
          part_path,
        )
      }

      #[inline]
      pub fn add_alternative_format_import_part<P>(
        &self,
        package: &mut P,
        content_type: impl Into<std::borrow::Cow<'static, str>>,
      ) -> Result<
        crate::parts::alternative_format_import_part::AlternativeFormatImportPart,
        crate::common::SdkError,
      >
      where
        P: crate::sdk::SdkPackage,
      {
        <Self as crate::sdk::SdkPart>::add_alternative_format_import_part(self,
          package,
          content_type,
        )
      }

      #[inline]
      pub fn add_alternative_format_import_part_with_id<P>(
        &self,
        package: &mut P,
        content_type: impl Into<std::borrow::Cow<'static, str>>,
        relationship_id: impl Into<String>,
      ) -> Result<
        crate::parts::alternative_format_import_part::AlternativeFormatImportPart,
        crate::common::SdkError,
      >
      where
        P: crate::sdk::SdkPackage,
      {
        <Self as crate::sdk::SdkPart>::add_alternative_format_import_part_with_id(self,
          package,
          content_type,
          relationship_id,
        )
      }

      #[inline]
      pub fn add_alternative_format_import_part_by_type<P>(
        &self,
        package: &mut P,
        part_type: crate::sdk::AlternativeFormatImportPartType,
      ) -> Result<
        crate::parts::alternative_format_import_part::AlternativeFormatImportPart,
        crate::common::SdkError,
      >
      where
        P: crate::sdk::SdkPackage,
      {
        <Self as crate::sdk::SdkPart>::add_alternative_format_import_part_by_type(self,
          package,
          part_type,
        )
      }

      #[inline]
      pub fn add_alternative_format_import_part_by_type_with_id<P>(
        &self,
        package: &mut P,
        part_type: crate::sdk::AlternativeFormatImportPartType,
        relationship_id: impl Into<String>,
      ) -> Result<
        crate::parts::alternative_format_import_part::AlternativeFormatImportPart,
        crate::common::SdkError,
      >
      where
        P: crate::sdk::SdkPackage,
      {
        <Self as crate::sdk::SdkPart>::add_alternative_format_import_part_by_type_with_id(self,
          package,
          part_type,
          relationship_id,
        )
      }

      #[inline]
      pub fn add_custom_xml_part<P>(
        &self,
        package: &mut P,
        content_type: impl Into<std::borrow::Cow<'static, str>>,
      ) -> Result<crate::parts::custom_xml_part::CustomXmlPart, crate::common::SdkError>
      where
        P: crate::sdk::SdkPackage,
      {
        <Self as crate::sdk::SdkPart>::add_custom_xml_part(self,
          package,
          content_type,
        )
      }

      #[inline]
      pub fn add_custom_xml_part_with_id<P>(
        &self,
        package: &mut P,
        content_type: impl Into<std::borrow::Cow<'static, str>>,
        relationship_id: impl Into<String>,
      ) -> Result<crate::parts::custom_xml_part::CustomXmlPart, crate::common::SdkError>
      where
        P: crate::sdk::SdkPackage,
      {
        <Self as crate::sdk::SdkPart>::add_custom_xml_part_with_id(self,
          package,
          content_type,
          relationship_id,
        )
      }

      #[inline]
      pub fn add_custom_xml_part_by_type<P>(
        &self,
        package: &mut P,
        part_type: crate::sdk::CustomXmlPartType,
      ) -> Result<crate::parts::custom_xml_part::CustomXmlPart, crate::common::SdkError>
      where
        P: crate::sdk::SdkPackage,
      {
        <Self as crate::sdk::SdkPart>::add_custom_xml_part_by_type(self,
          package,
          part_type,
        )
      }

      #[inline]
      pub fn add_custom_xml_part_by_type_with_id<P>(
        &self,
        package: &mut P,
        part_type: crate::sdk::CustomXmlPartType,
        relationship_id: impl Into<String>,
      ) -> Result<crate::parts::custom_xml_part::CustomXmlPart, crate::common::SdkError>
      where
        P: crate::sdk::SdkPackage,
      {
        <Self as crate::sdk::SdkPart>::add_custom_xml_part_by_type_with_id(self,
          package,
          part_type,
          relationship_id,
        )
      }

      #[inline]
      pub fn add_custom_property_part_by_type<P>(
        &self,
        package: &mut P,
        part_type: crate::sdk::CustomPropertyPartType,
      ) -> Result<crate::parts::custom_property_part::CustomPropertyPart, crate::common::SdkError>
      where
        P: crate::sdk::SdkPackage,
      {
        <Self as crate::sdk::SdkPart>::add_custom_property_part_by_type(self, package, part_type,
        )
      }

      #[inline]
      pub fn add_custom_property_part_by_type_with_id<P>(
        &self,
        package: &mut P,
        part_type: crate::sdk::CustomPropertyPartType,
        relationship_id: impl Into<String>,
      ) -> Result<crate::parts::custom_property_part::CustomPropertyPart, crate::common::SdkError>
      where
        P: crate::sdk::SdkPackage,
      {
        <Self as crate::sdk::SdkPart>::add_custom_property_part_by_type_with_id(self,
          package,
          part_type,
          relationship_id,
        )
      }

      #[inline]
      pub fn add_embedded_object_part_by_type<P>(
        &self,
        package: &mut P,
        part_type: crate::sdk::EmbeddedObjectPartType,
      ) -> Result<crate::parts::embedded_object_part::EmbeddedObjectPart, crate::common::SdkError>
      where
        P: crate::sdk::SdkPackage,
      {
        <Self as crate::sdk::SdkPart>::add_embedded_object_part_by_type(self, package, part_type,
        )
      }

      #[inline]
      pub fn add_embedded_object_part_by_type_with_id<P>(
        &self,
        package: &mut P,
        part_type: crate::sdk::EmbeddedObjectPartType,
        relationship_id: impl Into<String>,
      ) -> Result<crate::parts::embedded_object_part::EmbeddedObjectPart, crate::common::SdkError>
      where
        P: crate::sdk::SdkPackage,
      {
        <Self as crate::sdk::SdkPart>::add_embedded_object_part_by_type_with_id(self,
          package,
          part_type,
          relationship_id,
        )
      }

      #[inline]
      pub fn add_embedded_package_part<P>(
        &self,
        package: &mut P,
        content_type: impl Into<std::borrow::Cow<'static, str>>,
      ) -> Result<crate::parts::embedded_package_part::EmbeddedPackagePart, crate::common::SdkError>
      where
        P: crate::sdk::SdkPackage,
      {
        <Self as crate::sdk::SdkPart>::add_embedded_package_part(self,
          package,
          content_type,
        )
      }

      #[inline]
      pub fn add_embedded_package_part_with_id<P>(
        &self,
        package: &mut P,
        content_type: impl Into<std::borrow::Cow<'static, str>>,
        relationship_id: impl Into<String>,
      ) -> Result<crate::parts::embedded_package_part::EmbeddedPackagePart, crate::common::SdkError>
      where
        P: crate::sdk::SdkPackage,
      {
        <Self as crate::sdk::SdkPart>::add_embedded_package_part_with_id(self,
          package,
          content_type,
          relationship_id,
        )
      }

      #[inline]
      pub fn add_embedded_package_part_by_type<P>(
        &self,
        package: &mut P,
        part_type: crate::sdk::EmbeddedPackagePartType,
      ) -> Result<crate::parts::embedded_package_part::EmbeddedPackagePart, crate::common::SdkError>
      where
        P: crate::sdk::SdkPackage,
      {
        <Self as crate::sdk::SdkPart>::add_embedded_package_part_by_type(self, package, part_type,
        )
      }

      #[inline]
      pub fn add_embedded_package_part_by_type_with_id<P>(
        &self,
        package: &mut P,
        part_type: crate::sdk::EmbeddedPackagePartType,
        relationship_id: impl Into<String>,
      ) -> Result<crate::parts::embedded_package_part::EmbeddedPackagePart, crate::common::SdkError>
      where
        P: crate::sdk::SdkPackage,
      {
        <Self as crate::sdk::SdkPart>::add_embedded_package_part_by_type_with_id(self,
          package,
          part_type,
          relationship_id,
        )
      }

      #[inline]
      pub fn add_font_part_by_type<P>(
        &self,
        package: &mut P,
        part_type: crate::sdk::FontPartType,
      ) -> Result<crate::parts::font_part::FontPart, crate::common::SdkError>
      where
        P: crate::sdk::SdkPackage,
      {
        <Self as crate::sdk::SdkPart>::add_font_part_by_type(self, package, part_type)
      }

      #[inline]
      pub fn add_font_part_by_type_with_id<P>(
        &self,
        package: &mut P,
        part_type: crate::sdk::FontPartType,
        relationship_id: impl Into<String>,
      ) -> Result<crate::parts::font_part::FontPart, crate::common::SdkError>
      where
        P: crate::sdk::SdkPackage,
      {
        <Self as crate::sdk::SdkPart>::add_font_part_by_type_with_id(self,
          package,
          part_type,
          relationship_id,
        )
      }

      #[inline]
      pub fn add_mail_merge_recipient_data_part_by_type<P>(
        &self,
        package: &mut P,
        part_type: crate::sdk::MailMergeRecipientDataPartType,
      ) -> Result<
        crate::parts::mail_merge_recipient_data_part::MailMergeRecipientDataPart,
        crate::common::SdkError,
      >
      where
        P: crate::sdk::SdkPackage,
      {
        <Self as crate::sdk::SdkPart>::add_mail_merge_recipient_data_part_by_type(self, package, part_type,
        )
      }

      #[inline]
      pub fn add_mail_merge_recipient_data_part_by_type_with_id<P>(
        &self,
        package: &mut P,
        part_type: crate::sdk::MailMergeRecipientDataPartType,
        relationship_id: impl Into<String>,
      ) -> Result<
        crate::parts::mail_merge_recipient_data_part::MailMergeRecipientDataPart,
        crate::common::SdkError,
      >
      where
        P: crate::sdk::SdkPackage,
      {
        <Self as crate::sdk::SdkPart>::add_mail_merge_recipient_data_part_by_type_with_id(self,
          package,
          part_type,
          relationship_id,
        )
      }

      #[inline]
      pub fn add_embedded_control_persistence_binary_data_part_by_type<P>(
        &self,
        package: &mut P,
        part_type: crate::sdk::EmbeddedControlPersistenceBinaryDataPartType,
      ) -> Result<
        crate::parts::embedded_control_persistence_binary_data_part::EmbeddedControlPersistenceBinaryDataPart,
        crate::common::SdkError,
      >
      where
        P: crate::sdk::SdkPackage,
      {
        <Self as crate::sdk::SdkPart>::add_embedded_control_persistence_binary_data_part_by_type(self, package, part_type,
        )
      }

      #[inline]
      pub fn add_embedded_control_persistence_binary_data_part_by_type_with_id<P>(
        &self,
        package: &mut P,
        part_type: crate::sdk::EmbeddedControlPersistenceBinaryDataPartType,
        relationship_id: impl Into<String>,
      ) -> Result<
        crate::parts::embedded_control_persistence_binary_data_part::EmbeddedControlPersistenceBinaryDataPart,
        crate::common::SdkError,
      >
      where
        P: crate::sdk::SdkPackage,
      {
        <Self as crate::sdk::SdkPart>::add_embedded_control_persistence_binary_data_part_by_type_with_id(self,
          package,
          part_type,
          relationship_id,
        )
      }

      #[inline]
      pub fn add_embedded_control_persistence_part_by_type<P>(
        &self,
        package: &mut P,
        part_type: crate::sdk::EmbeddedControlPersistencePartType,
      ) -> Result<
        crate::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart,
        crate::common::SdkError,
      >
      where
        P: crate::sdk::SdkPackage,
      {
        <Self as crate::sdk::SdkPart>::add_embedded_control_persistence_part_by_type(self, package, part_type,
        )
      }

      #[inline]
      pub fn add_embedded_control_persistence_part_by_type_with_id<P>(
        &self,
        package: &mut P,
        part_type: crate::sdk::EmbeddedControlPersistencePartType,
        relationship_id: impl Into<String>,
      ) -> Result<
        crate::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart,
        crate::common::SdkError,
      >
      where
        P: crate::sdk::SdkPackage,
      {
        <Self as crate::sdk::SdkPart>::add_embedded_control_persistence_part_by_type_with_id(self,
          package,
          part_type,
          relationship_id,
        )
      }

      #[inline]
      pub fn get_reference_relationship<'a, P: crate::sdk::SdkPackage>(
        &'a self,
        package: &'a P,
        relationship_id: &str,
      ) -> Option<crate::common::RelationshipRef<'a>> {
        <Self as crate::sdk::SdkPart>::get_reference_relationship(self,
          package,
          relationship_id,
        )
      }

      #[inline]
      pub fn get_external_relationship<'a, P: crate::sdk::SdkPackage>(
        &'a self,
        package: &'a P,
        relationship_id: &str,
      ) -> Option<crate::common::RelationshipRef<'a>> {
        <Self as crate::sdk::SdkPart>::get_external_relationship(self,
          package,
          relationship_id,
        )
      }

      #[inline]
      pub fn get_hyperlink_relationship<'a, P: crate::sdk::SdkPackage>(
        &'a self,
        package: &'a P,
        relationship_id: &str,
      ) -> Option<crate::common::RelationshipRef<'a>> {
        <Self as crate::sdk::SdkPart>::get_hyperlink_relationship(self,
          package,
          relationship_id,
        )
      }

      #[inline]
      pub fn delete_reference_relationship<P: crate::sdk::SdkPackage>(
        &self,
        package: &mut P,
        relationship_id: &str,
      ) -> Result<crate::common::Relationship, crate::common::SdkError> {
        <Self as crate::sdk::SdkPart>::delete_reference_relationship(self,
          package,
          relationship_id,
        )
      }

      #[inline]
      pub fn delete_external_relationship<P: crate::sdk::SdkPackage>(
        &self,
        package: &mut P,
        relationship_id: &str,
      ) -> Result<crate::common::Relationship, crate::common::SdkError> {
        <Self as crate::sdk::SdkPart>::delete_external_relationship(self,
          package,
          relationship_id,
        )
      }

      #[inline]
      pub fn change_relationship_id<P: crate::sdk::SdkPackage>(
        &self,
        package: &mut P,
        relationship_id: &str,
        new_relationship_id: impl Into<String>,
      ) -> Result<(), crate::common::SdkError> {
        <Self as crate::sdk::SdkPart>::change_relationship_id(self,
          package,
          relationship_id,
          new_relationship_id,
        )
      }
    }

    #root_method
    #child_methods
  })
}

fn part_handle_root_method_tokens(
  part_ident: &Ident,
  root_info: Option<&PartHandleRootInfo>,
) -> proc_macro2::TokenStream {
  let Some(root_info) = root_info else {
    return quote! {};
  };
  let root_ty = &root_info.root_ty;

  quote! {
    impl #part_ident {
      #[inline]
      pub fn is_root_element_loaded<P: crate::sdk::SdkPackage>(&self, package: &P) -> bool {
        crate::sdk::SdkPackage::is_root_element_loaded(package, self.id)
      }

      #[inline]
      pub fn unload_root_element<P: crate::sdk::SdkPackage>(
        &self,
        package: &mut P,
      ) -> Option<crate::parts::PartRootElement> {
        crate::sdk::SdkPackage::unload_root_element(package, self.id)
      }

      pub fn root_element<'a, P: crate::sdk::SdkPackage>(
        &self,
        package: &'a mut P,
      ) -> Result<&'a #root_ty, crate::common::SdkError> {
        if crate::sdk::SdkPackage::root_element(package, self.id)
          .and_then(<Self as crate::sdk::SdkPartRoot>::root_element_ref)
          .is_none()
        {
          let root_element = {
            let part = crate::sdk::SdkPackage::storage(package).part(self.id).ok_or_else(|| {
              crate::common::SdkError::CommonError(format!(
                "part id {:?} is not present in package storage",
                self.id,
              ))
            })?;
            let bytes = part.data().bytes();
            if let Some(bytes) = crate::common::decode_utf16_xml_bytes(bytes)? {
              #root_ty::from_reader(std::io::Cursor::new(bytes))?
            } else {
              #root_ty::from_bytes(bytes)?
            }
          };

          *crate::sdk::SdkPackage::root_element_slot_mut(package, self.id).ok_or_else(|| {
            crate::common::SdkError::CommonError(format!(
              "part id {:?} is not present in package root cache",
              self.id,
            ))
          })? = Some(<Self as crate::sdk::SdkPartRoot>::wrap_root_element(root_element));
        }

        crate::sdk::SdkPackage::root_element(package, self.id)
          .and_then(<Self as crate::sdk::SdkPartRoot>::root_element_ref)
          .ok_or_else(|| {
            crate::common::SdkError::CommonError(
              concat!("cached root element has unexpected type for ", stringify!(#part_ident))
                .to_string(),
            )
          })
      }

      pub fn root_element_mut<'a, P: crate::sdk::SdkPackage>(
        &self,
        package: &'a mut P,
      ) -> Result<&'a mut #root_ty, crate::common::SdkError> {
        if crate::sdk::SdkPackage::root_element(package, self.id)
          .and_then(<Self as crate::sdk::SdkPartRoot>::root_element_ref)
          .is_none()
        {
          let root_element = {
            let part = crate::sdk::SdkPackage::storage(package).part(self.id).ok_or_else(|| {
              crate::common::SdkError::CommonError(format!(
                "part id {:?} is not present in package storage",
                self.id,
              ))
            })?;
            let bytes = part.data().bytes();
            if let Some(bytes) = crate::common::decode_utf16_xml_bytes(bytes)? {
              #root_ty::from_reader(std::io::Cursor::new(bytes))?
            } else {
              #root_ty::from_bytes(bytes)?
            }
          };

          *crate::sdk::SdkPackage::root_element_slot_mut(package, self.id).ok_or_else(|| {
            crate::common::SdkError::CommonError(format!(
              "part id {:?} is not present in package root cache",
              self.id,
            ))
          })? = Some(<Self as crate::sdk::SdkPartRoot>::wrap_root_element(root_element));
        }

        crate::sdk::SdkPackage::root_element_slot_mut(package, self.id)
          .and_then(Option::as_mut)
          .and_then(<Self as crate::sdk::SdkPartRoot>::root_element_mut)
          .ok_or_else(|| {
            crate::common::SdkError::CommonError(
              concat!("cached root element has unexpected type for ", stringify!(#part_ident))
                .to_string(),
            )
          })
      }

      pub fn set_root_element<P: crate::sdk::SdkPackage>(
        &self,
        package: &mut P,
        root_element: #root_ty,
      ) -> Result<(), crate::common::SdkError> {
        *crate::sdk::SdkPackage::root_element_slot_mut(package, self.id).ok_or_else(|| {
          crate::common::SdkError::CommonError(format!(
            "part id {:?} is not present in package root cache",
            self.id,
          ))
        })? = Some(<Self as crate::sdk::SdkPartRoot>::wrap_root_element(root_element));

        Ok(())
      }
    }
  }
}

fn part_handle_child_methods_tokens(
  part_ident: &Ident,
  child_infos: &[PartHandleChildInfo],
) -> proc_macro2::TokenStream {
  let accessors = child_infos.iter().map(|child| {
    let method_ident = &child.field_ident;
    let part_ty = &child.part_ty;
    let relationship_matches = relationship_match_condition_tokens(
      &child.relationship_type,
      quote! { relationship },
      quote! { <#part_ty as crate::sdk::SdkPartDescriptor>::RELATIONSHIP_TYPE },
    );
    let map_relationship = quote! {
      move |relationship: &crate::common::RelationshipInfo| {
        if #relationship_matches {
          relationship
            .target_part_id()
            .map(|part_id| {
              <#part_ty as crate::sdk::SdkPart>::from_relationship_id(relationship.id(), part_id)
            })
        } else {
          None
        }
      }
    };

    match child.kind {
      PartChildKind::Repeated | PartChildKind::RequiredRepeated => quote! {
        pub fn #method_ident<'a, P: crate::sdk::SdkPackage>(
          &'a self,
          package: &'a P,
        ) -> impl Iterator<Item = #part_ty> + 'a {
          let _ = &self.#method_ident;
          crate::sdk::SdkPackage::storage(package).relationships(self.id)
            .into_iter()
            .flat_map(|relationships| relationships.iter())
            .filter_map(#map_relationship)
        }
      },
      PartChildKind::Required | PartChildKind::Optional => quote! {
        pub fn #method_ident<P: crate::sdk::SdkPackage>(
          &self,
          package: &P,
        ) -> Option<#part_ty> {
          let _ = &self.#method_ident;
          crate::sdk::SdkPackage::storage(package).relationships(self.id)
            .into_iter()
            .flat_map(|relationships| relationships.iter())
            .find_map(#map_relationship)
        }
      },
    }
  });

  let part_ref_from_relationship = quote! {
    fn part_ref_from_relationship<P: crate::sdk::SdkPackage>(
      package: &P,
      relationship: &crate::common::RelationshipInfo,
    ) -> Option<crate::parts::PartRef> {
      let _ = relationship.target_part_id()?;
      crate::parts::PartRef::from_relationship_storage(
        crate::sdk::SdkPackage::storage(package),
        relationship,
      )
    }
  };

  quote! {
    impl #part_ident {
      pub fn parts<'a, P: crate::sdk::SdkPackage>(
        &'a self,
        package: &'a P,
      ) -> impl Iterator<Item = crate::parts::IdPartPair<'a>> + 'a {
        crate::sdk::SdkPackage::storage(package).relationships(self.id)
          .into_iter()
          .flat_map(|relationships| relationships.iter())
          .filter_map(|relationship| {
            let part = Self::part_ref_from_relationship(package, relationship)?;
            Some(crate::parts::IdPartPair::new(relationship.id(), part))
          })
      }

      #[inline]
      pub fn get_all_parts<'a, P: crate::sdk::SdkPackage>(
        &'a self,
        package: &'a P,
      ) -> impl Iterator<Item = crate::parts::PartRef> + 'a {
        <Self as crate::sdk::SdkPart>::get_all_parts(self, package)
      }

      #[inline]
      pub fn get_parent_parts<'a, P: crate::sdk::SdkPackage>(
        &'a self,
        package: &'a P,
      ) -> impl Iterator<Item = crate::parts::PartRef> + 'a {
        <Self as crate::sdk::SdkPart>::get_parent_parts(self, package)
      }

      #[inline]
      pub fn get_part_by_id<P: crate::sdk::SdkPackage>(
        &self,
        package: &P,
        relationship_id: &str,
      ) -> Option<crate::parts::PartRef> {
        let relationship =
          crate::sdk::SdkPackage::storage(package).relationships(self.id)?.get(relationship_id)?;
        Self::part_ref_from_relationship(package, relationship)
      }

      #[inline]
      pub fn get_part_by_id_required<P: crate::sdk::SdkPackage>(
        &self,
        package: &P,
        relationship_id: &str,
      ) -> Result<crate::parts::PartRef, crate::common::SdkError> {
        self.get_part_by_id(package, relationship_id).ok_or_else(|| {
          crate::common::SdkError::CommonError(format!(
            "part relationship id {relationship_id} does not exist"
          ))
        })
      }

      #[inline]
      pub fn try_get_part_by_id<P: crate::sdk::SdkPackage>(
        &self,
        package: &P,
        relationship_id: &str,
      ) -> Option<crate::parts::PartRef> {
        self.get_part_by_id(package, relationship_id)
      }

      pub fn get_parts_of_type<'a, P: crate::sdk::SdkPackage, T: crate::sdk::SdkPart>(
        &'a self,
        package: &'a P,
      ) -> impl Iterator<Item = T> + 'a {
        <Self as crate::sdk::SdkPart>::get_parts_of_type::<P, T>(self, package)
      }

      pub fn get_id_of_part<'a, P: crate::sdk::SdkPackage, T: crate::sdk::SdkPart>(
        &'a self,
        package: &'a P,
        part: &T,
      ) -> Option<&'a str> {
        let target_part_id = part.part_id();
        crate::sdk::SdkPackage::storage(package).relationships(self.id)?.iter().find_map(|relationship| {
          (relationship.target_part_id() == Some(target_part_id)).then_some(relationship.id())
        })
      }

      #[inline]
      pub fn get_id_of_part_required<'a, P: crate::sdk::SdkPackage, T: crate::sdk::SdkPart>(
        &'a self,
        package: &'a P,
        part: &T,
      ) -> Result<&'a str, crate::common::SdkError> {
        <Self as crate::sdk::SdkPart>::get_id_of_part_required(self, package, part)
      }

      #[inline]
      pub fn change_id_of_part<P: crate::sdk::SdkPackage, T: crate::sdk::SdkPart>(
        &self,
        package: &mut P,
        part: &T,
        new_relationship_id: impl Into<String>,
      ) -> Result<String, crate::common::SdkError> {
        <Self as crate::sdk::SdkPart>::change_id_of_part(
          self,
          package,
          part,
          new_relationship_id,
        )
      }

      #[inline]
      pub fn delete_part_by_id<P: crate::sdk::SdkPackage>(
        &self,
        package: &mut P,
        relationship_id: &str,
      ) -> Result<bool, crate::common::SdkError> {
        <Self as crate::sdk::SdkPart>::delete_part_by_id(self,
          package,
          relationship_id,
        )
      }

      #[inline]
      pub fn delete_part<P: crate::sdk::SdkPackage, T: crate::sdk::SdkPart>(
        &self,
        package: &mut P,
        part: T,
      ) -> Result<bool, crate::common::SdkError> {
        <Self as crate::sdk::SdkPart>::delete_part(self, package, part)
      }

      #[inline]
      pub fn delete_parts<P, T, I>(
        &self,
        package: &mut P,
        parts: I,
      ) -> Result<(), crate::common::SdkError>
      where
        P: crate::sdk::SdkPackage,
        T: crate::sdk::SdkPart,
        I: IntoIterator<Item = T>,
      {
        <Self as crate::sdk::SdkPart>::delete_parts(self, package, parts)
      }

      #[inline]
      pub fn add_part<P: crate::sdk::SdkPackage, T: crate::sdk::SdkPart>(
        &self,
        package: &mut P,
        part: T,
      ) -> Result<T, crate::common::SdkError> {
        <Self as crate::sdk::SdkPart>::add_part(self, package, part)
      }

      #[inline]
      pub fn add_part_with_id<P: crate::sdk::SdkPackage, T: crate::sdk::SdkPart>(
        &self,
        package: &mut P,
        part: T,
        relationship_id: impl Into<String>,
      ) -> Result<T, crate::common::SdkError> {
        <Self as crate::sdk::SdkPart>::add_part_with_id(self,
          package,
          part,
          relationship_id,
        )
      }

      #[inline]
      pub fn add_part_from_package<
        P: crate::sdk::SdkPackage,
        S: crate::sdk::SdkPackage,
        T: crate::sdk::SdkPart,
      >(
        &self,
        package: &mut P,
        source_package: &S,
        part: &T,
      ) -> Result<T, crate::common::SdkError> {
        <Self as crate::sdk::SdkPart>::add_part_from_package(
          self,
          package,
          source_package,
          part,
        )
      }

      #[inline]
      pub fn add_part_from_package_with_id<
        P: crate::sdk::SdkPackage,
        S: crate::sdk::SdkPackage,
        T: crate::sdk::SdkPart,
      >(
        &self,
        package: &mut P,
        source_package: &S,
        part: &T,
        relationship_id: impl Into<String>,
      ) -> Result<T, crate::common::SdkError> {
        <Self as crate::sdk::SdkPart>::add_part_from_package_with_id(
          self,
          package,
          source_package,
          part,
          relationship_id,
        )
      }

      #[inline]
      pub fn create_relationship_to_part<P: crate::sdk::SdkPackage, T: crate::sdk::SdkPart>(
        &self,
        package: &mut P,
        part: T,
      ) -> Result<String, crate::common::SdkError> {
        <Self as crate::sdk::SdkPart>::create_relationship_to_part(self,
          package,
          part,
        )
      }

      #[inline]
      pub fn create_relationship_to_part_with_id<
        P: crate::sdk::SdkPackage,
        T: crate::sdk::SdkPart,
      >(
        &self,
        package: &mut P,
        part: T,
        relationship_id: impl Into<String>,
      ) -> Result<String, crate::common::SdkError> {
        <Self as crate::sdk::SdkPart>::create_relationship_to_part_with_id(self,
          package,
          part,
          relationship_id,
        )
      }

      #[inline]
      pub fn add_audio_reference_relationship<P: crate::sdk::SdkPackage>(
        &self,
        package: &mut P,
        media_data_part: &crate::common::MediaDataPart,
      ) -> Result<String, crate::common::SdkError> {
        <Self as crate::sdk::SdkPart>::add_audio_reference_relationship(self,
          package,
          media_data_part,
        )
      }

      #[inline]
      pub fn add_audio_reference_relationship_with_id<P: crate::sdk::SdkPackage>(
        &self,
        package: &mut P,
        media_data_part: &crate::common::MediaDataPart,
        relationship_id: impl Into<String>,
      ) -> Result<String, crate::common::SdkError> {
        <Self as crate::sdk::SdkPart>::add_audio_reference_relationship_with_id(self,
          package,
          media_data_part,
          relationship_id,
        )
      }

      #[inline]
      pub fn add_media_reference_relationship<P: crate::sdk::SdkPackage>(
        &self,
        package: &mut P,
        media_data_part: &crate::common::MediaDataPart,
      ) -> Result<String, crate::common::SdkError> {
        <Self as crate::sdk::SdkPart>::add_media_reference_relationship(self,
          package,
          media_data_part,
        )
      }

      #[inline]
      pub fn add_media_reference_relationship_with_id<P: crate::sdk::SdkPackage>(
        &self,
        package: &mut P,
        media_data_part: &crate::common::MediaDataPart,
        relationship_id: impl Into<String>,
      ) -> Result<String, crate::common::SdkError> {
        <Self as crate::sdk::SdkPart>::add_media_reference_relationship_with_id(self,
          package,
          media_data_part,
          relationship_id,
        )
      }

      #[inline]
      pub fn add_video_reference_relationship<P: crate::sdk::SdkPackage>(
        &self,
        package: &mut P,
        media_data_part: &crate::common::MediaDataPart,
      ) -> Result<String, crate::common::SdkError> {
        <Self as crate::sdk::SdkPart>::add_video_reference_relationship(self,
          package,
          media_data_part,
        )
      }

      #[inline]
      pub fn add_video_reference_relationship_with_id<P: crate::sdk::SdkPackage>(
        &self,
        package: &mut P,
        media_data_part: &crate::common::MediaDataPart,
        relationship_id: impl Into<String>,
      ) -> Result<String, crate::common::SdkError> {
        <Self as crate::sdk::SdkPart>::add_video_reference_relationship_with_id(self,
          package,
          media_data_part,
          relationship_id,
        )
      }

      #[inline]
      pub fn add_data_part_reference_relationship_from_existing<P: crate::sdk::SdkPackage>(
        &self,
        package: &mut P,
        relationship: crate::common::Relationship,
      ) -> Result<String, crate::common::SdkError> {
        <Self as crate::sdk::SdkPart>::add_data_part_reference_relationship_from_existing(self,
          package,
          relationship,
        )
      }

      #part_ref_from_relationship

      #( #accessors )*
    }
  }
}

fn marker_inner_type(ty: &Type, marker: &str) -> Option<Type> {
  let Type::Path(type_path) = ty else {
    return None;
  };
  let segment = type_path.path.segments.last()?;
  if segment.ident != marker {
    return None;
  }
  let syn::PathArguments::AngleBracketed(args) = &segment.arguments else {
    return None;
  };
  let syn::GenericArgument::Type(inner) = args.args.first()? else {
    return None;
  };
  Some(inner.clone())
}

fn part_child_marker_info(ty: &Type) -> Option<PartChildMarkerInfo> {
  let Type::Path(type_path) = ty else {
    return None;
  };
  let segment = type_path.path.segments.last()?;
  let marker_name = segment.ident.to_string();
  let syn::PathArguments::AngleBracketed(args) = &segment.arguments else {
    return None;
  };
  let syn::GenericArgument::Type(part_ty) = args.args.first()? else {
    return None;
  };

  let kind = match marker_name.as_str() {
    "OptionalPart" => PartChildKind::Optional,
    "RequiredPart" => PartChildKind::Required,
    "RepeatedPart" => PartChildKind::Repeated,
    "PartChild" => {
      let syn::GenericArgument::Type(kind_ty) = args.args.iter().nth(1)? else {
        return None;
      };
      part_child_kind_from_type(kind_ty)?
    }
    _ => return None,
  };

  Some(PartChildMarkerInfo {
    part_ty: part_ty.clone(),
    kind,
  })
}

fn part_child_field_info(ty: &Type) -> Option<PartChildMarkerInfo> {
  if let Some(part_ty) = marker_inner_type(ty, "Vec") {
    return Some(PartChildMarkerInfo {
      part_ty,
      kind: PartChildKind::Repeated,
    });
  }
  if let Some(part_ty) = marker_inner_type(ty, "Option") {
    let part_ty = marker_inner_type(&part_ty, "Box").unwrap_or(part_ty);
    return Some(PartChildMarkerInfo {
      part_ty,
      kind: PartChildKind::Optional,
    });
  }
  part_child_marker_info(ty)
}

fn part_child_kind_from_type(ty: &Type) -> Option<PartChildKind> {
  let Type::Path(type_path) = ty else {
    return None;
  };
  let segment = type_path.path.segments.last()?;
  match segment.ident.to_string().as_str() {
    "OptionalPartKind" => Some(PartChildKind::Optional),
    "RequiredPartKind" => Some(PartChildKind::Required),
    "RepeatedPartKind" => Some(PartChildKind::Repeated),
    _ => None,
  }
}
