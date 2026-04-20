//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the Key Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is pRoam:key.
pub type Key = crate::simple_type::StringValue;
/// Defines the Value Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is pRoam:value.
pub type Value = crate::simple_type::StringValue;
/// Defines the RoamingProperty Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is pRoam:props.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "pRoam:CT_RoamingProperty/pRoam:props")]
pub struct RoamingProperty {
  /// _
  #[sdk(text_child(qname = "xsd:string/pRoam:key"))]
  pub key: crate::simple_type::StringValue,
  /// _
  #[sdk(text_child(qname = "xsd:string/pRoam:value"))]
  pub value: crate::simple_type::StringValue,
}
