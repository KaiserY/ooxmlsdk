//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.microsoft.com/office/2017/06/relationships/model3d";
pub const PATH_PREFIX: &str = "../media";
pub const CONTENT_TYPE: &str = "model/gltf-binary";
pub const TARGET_NAME: &str = "model3d";
pub const EXTENSION: &str = ".glb";
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct Model3DReferenceRelationshipPart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
}
