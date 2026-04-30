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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "ma:CT_Dummy/ma:DummyContentTypeElement")]
pub struct Dummy {
  /// decimals
  #[sdk(attr(qname = ":decimals"))]
  pub decimals: Option<crate::simple_type::StringValue>,
  /// default
  #[sdk(attr(qname = ":default"))]
  pub default: Option<crate::simple_type::StringValue>,
  /// description
  #[sdk(attr(qname = ":description"))]
  pub description: Option<crate::simple_type::StringValue>,
  /// displayName
  #[sdk(attr(qname = ":displayName"))]
  pub display_name: Option<crate::simple_type::StringValue>,
  /// fieldsID
  #[sdk(attr(qname = ":fieldsID"))]
  pub fields_id: Option<crate::simple_type::StringValue>,
  /// format
  #[sdk(attr(qname = ":format"))]
  pub format: Option<crate::simple_type::StringValue>,
  /// hidden
  #[sdk(attr(qname = ":hidden"))]
  pub hidden: Option<crate::simple_type::StringValue>,
  /// index
  #[sdk(attr(qname = ":index"))]
  #[sdk(number_range(source = 0u32, min = "0", min_inclusive = true, max_inclusive = false))]
  pub index: Option<crate::simple_type::Int32Value>,
  /// internalName
  #[sdk(attr(qname = ":internalName"))]
  pub internal_name: Option<crate::simple_type::StringValue>,
  /// LCID
  #[sdk(attr(qname = ":LCID"))]
  pub lcid: Option<crate::simple_type::Int32Value>,
  /// list
  #[sdk(attr(qname = ":list"))]
  pub list: Option<crate::simple_type::StringValue>,
  /// percentage
  #[sdk(attr(qname = ":percentage"))]
  pub percentage: Option<crate::simple_type::StringValue>,
  /// readOnly
  #[sdk(attr(qname = ":readOnly"))]
  pub read_only: Option<crate::simple_type::StringValue>,
  /// requiredMultiChoice
  #[sdk(attr(qname = ":requiredMultiChoice"))]
  pub required_multi_choice: Option<crate::simple_type::StringValue>,
  /// root
  #[sdk(attr(qname = ":root"))]
  pub root: Option<TrueOnlyValues>,
  /// showField
  #[sdk(attr(qname = ":showField"))]
  pub show_field: Option<crate::simple_type::StringValue>,
  /// web
  #[sdk(attr(qname = ":web"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}"
  ))]
  pub web: Option<crate::simple_type::StringValue>,
}
