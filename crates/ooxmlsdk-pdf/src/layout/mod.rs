pub(crate) use ooxmlsdk_layout::compat::{
  FillItem, FollowFrameKind, ImageItem, LayoutDocument, LineItem, LineItemKind, LinkAreaItem,
  OutlineEntry, PageItem, PdfTextSegmentation, PolylineItem, RectItem, TextItem,
};

pub(crate) fn from_compat_document(
  document: ooxmlsdk_layout::compat::LayoutDocument,
) -> LayoutDocument {
  document
}
