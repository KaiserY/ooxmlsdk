//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TrueOnlyValues {
  #[sdk(rename = "true")]
  #[default]
  True,
}
/// Defines the Dummy Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is ma:DummyContentTypeElement.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "ma:CT_Dummy/ma:DummyContentTypeElement")]
pub struct Dummy {
  /// Markup compatibility attribute mc:Ignorable.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:Ignorable
  #[sdk(attr(qname = "mc:Ignorable"))]
  pub mc_ignorable: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:MustUnderstand.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:MustUnderstand
  #[sdk(attr(qname = "mc:MustUnderstand"))]
  pub mc_must_understand: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:ProcessContent.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:ProcessContent
  #[sdk(attr(qname = "mc:ProcessContent"))]
  pub mc_process_content: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:PreserveAttributes.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:PreserveAttributes
  #[sdk(attr(qname = "mc:PreserveAttributes"))]
  pub mc_preserve_attributes: Option<crate::simple_type::StringValue>,
  /// decimals
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :decimals
  #[sdk(attr(qname = ":decimals"))]
  pub decimals: Option<crate::simple_type::StringValue>,
  /// default
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :default
  #[sdk(attr(qname = ":default"))]
  pub default: Option<crate::simple_type::StringValue>,
  /// description
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :description
  #[sdk(attr(qname = ":description"))]
  pub description: Option<crate::simple_type::StringValue>,
  /// displayName
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :displayName
  #[sdk(attr(qname = ":displayName"))]
  pub display_name: Option<crate::simple_type::StringValue>,
  /// fieldsID
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :fieldsID
  #[sdk(attr(qname = ":fieldsID"))]
  pub fields_id: Option<crate::simple_type::StringValue>,
  /// format
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :format
  #[sdk(attr(qname = ":format"))]
  pub format: Option<crate::simple_type::StringValue>,
  /// hidden
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :hidden
  #[sdk(attr(qname = ":hidden"))]
  pub hidden: Option<crate::simple_type::StringValue>,
  /// index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :index
  #[sdk(attr(qname = ":index"))]
  #[sdk(number_range(source = 0u32, min = "0", min_inclusive = true, max_inclusive = false))]
  pub index: Option<crate::simple_type::Int32Value>,
  /// internalName
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :internalName
  #[sdk(attr(qname = ":internalName"))]
  pub internal_name: Option<crate::simple_type::StringValue>,
  /// LCID
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :LCID
  #[sdk(attr(qname = ":LCID"))]
  pub lcid: Option<crate::simple_type::Int32Value>,
  /// list
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :list
  #[sdk(attr(qname = ":list"))]
  pub list: Option<crate::simple_type::StringValue>,
  /// percentage
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :percentage
  #[sdk(attr(qname = ":percentage"))]
  pub percentage: Option<crate::simple_type::StringValue>,
  /// readOnly
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :readOnly
  #[sdk(attr(qname = ":readOnly"))]
  pub read_only: Option<crate::simple_type::StringValue>,
  /// requiredMultiChoice
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :requiredMultiChoice
  #[sdk(attr(qname = ":requiredMultiChoice"))]
  pub required_multi_choice: Option<crate::simple_type::StringValue>,
  /// root
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :root
  #[sdk(attr(qname = ":root"))]
  pub root: Option<TrueOnlyValues>,
  /// showField
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showField
  #[sdk(attr(qname = ":showField"))]
  pub show_field: Option<crate::simple_type::StringValue>,
  /// web
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :web
  #[sdk(attr(qname = ":web"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}"
  ))]
  pub web: Option<crate::simple_type::StringValue>,
}
