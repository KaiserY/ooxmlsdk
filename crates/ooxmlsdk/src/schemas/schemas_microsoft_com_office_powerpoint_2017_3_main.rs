//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum DisplayLocation {
  #[sdk(rename = "media")]
  #[default]
  Media,
  #[sdk(rename = "slide")]
  Slide,
}
/// Defines the TracksInfo Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p173:tracksInfo")]
pub struct TracksInfo {
  /// displayLoc
  #[sdk(attr(qname = ":displayLoc"))]
  pub display_loc: DisplayLocation,
  /// Defines the TrackList Class.
  #[sdk(child(qname = "p173:trackLst"))]
  pub track_list: Option<TrackList>,
}
/// Defines the Track Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p173:track")]
pub struct Track {
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub id: crate::simple_type::StringValue,
  /// label
  #[sdk(attr(qname = ":label"))]
  pub label: crate::simple_type::StringValue,
  /// lang
  #[sdk(attr(qname = ":lang"))]
  pub lang: Option<crate::simple_type::StringValue>,
  /// Embedded Picture Reference
  #[sdk(attr(qname = "r:embed"))]
  pub embed: Option<crate::simple_type::StringValue>,
  /// Linked Picture Reference
  #[sdk(attr(qname = "r:link"))]
  pub link: Option<crate::simple_type::StringValue>,
}
/// Defines the TrackList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p173:trackLst")]
pub struct TrackList {
  /// Defines the Track Class.
  #[sdk(child(qname = "p173:track"))]
  pub track: Vec<Track>,
}
