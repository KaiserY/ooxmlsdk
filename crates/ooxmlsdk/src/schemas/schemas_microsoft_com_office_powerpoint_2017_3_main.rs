//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
pub enum DisplayLocation {
  #[sdk(rename = "media")]
  #[default]
  Media,
  #[sdk(rename = "slide")]
  Slide,
}
/// Defines the TracksInfo Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is p173:tracksInfo.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p173:CT_TracksInfo/p173:tracksInfo")]
pub struct TracksInfo {
  /// displayLoc
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :displayLoc
  #[sdk(attr(qname = ":displayLoc"))]
  pub display_loc: DisplayLocation,
  /// _
  #[sdk(child(qname = "p173:CT_TrackList/p173:trackLst"))]
  pub track_list: Option<TrackList>,
}
/// Defines the Track Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is p173:track.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p173:CT_Track/p173:track")]
pub struct Track {
  /// id
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub id: crate::simple_type::StringValue,
  /// label
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :label
  #[sdk(attr(qname = ":label"))]
  pub label: crate::simple_type::StringValue,
  /// lang
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :lang
  #[sdk(attr(qname = ":lang"))]
  pub lang: Option<crate::simple_type::StringValue>,
  /// Embedded Picture Reference
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:embed
  #[sdk(attr(qname = "r:embed"))]
  pub embed: Option<crate::simple_type::StringValue>,
  /// Linked Picture Reference
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:link
  #[sdk(attr(qname = "r:link"))]
  pub link: Option<crate::simple_type::StringValue>,
}
/// Defines the TrackList Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is p173:trackLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p173:CT_TrackList/p173:trackLst")]
pub struct TrackList {
  /// _
  #[sdk(child(qname = "p173:CT_Track/p173:track"))]
  pub p173_track: Vec<Track>,
}
