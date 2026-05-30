//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[repr(u16)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum XmlKnownNamespace {
  A,
  A14,
  A15,
  A16,
  A1611,
  A3danim,
  Aanim,
  Ac,
  Aclsh,
  Adec,
  Ahyp,
  Aif,
  Aink,
  Alf,
  Am3d,
  Aoe,
  Ap,
  Ask,
  Asl,
  Asvg,
  Ax,
  B,
  C,
  C14,
  C15,
  C16,
  C16ac,
  C16r2,
  C16r3,
  Cdip,
  Cdr,
  Cdr14,
  Clbl,
  Com14,
  Comp,
  Cp,
  Cppr,
  Cr,
  Cs,
  Ct,
  Cx,
  Cx1,
  Cx2,
  Cx3,
  Cx4,
  Cx5,
  Cx6,
  Cx7,
  Cx8,
  Dc,
  Dcmitype,
  Dcterms,
  Dgm,
  Dgm14,
  Dgm1611,
  Dgm1612,
  Dms,
  Ds,
  Dsp,
  Emma,
  Inkml,
  Lc,
  Loext,
  Lp,
  M,
  Ma,
  Ma14,
  Mc,
  Md,
  Mo,
  Msink,
  Mso,
  Mso14,
  Mv,
  Mx,
  Ntns,
  O,
  Oa,
  Oac,
  Odoc,
  Oel,
  Op,
  Outs,
  P,
  P14,
  P15,
  P1510,
  P16,
  P1710,
  P173,
  P184,
  P188,
  P1912,
  P202,
  P216,
  P223,
  P228,
  P232,
  PRoam,
  Packaging,
  Pav,
  Pc,
  Pc2,
  Pc226,
  Pic,
  Pic14,
  Pj15,
  Pvml,
  R,
  Sl,
  Sle,
  Spe,
  T,
  Thm15,
  Tsle,
  V,
  Vt,
  W,
  W10,
  W14,
  W15,
  W16cex,
  W16cid,
  W16cur,
  W16du,
  W16sdtdh,
  W16sdtfl,
  W16se,
  We,
  Wetp,
  Wne,
  Woe,
  Wp,
  Wp14,
  Wp15,
  Wpc,
  Wpg,
  Wpi,
  Wps,
  X,
  X12ac,
  X14,
  X14ac,
  X15,
  X15ac,
  X16,
  X16r2,
  X16r3,
  Xcalcf,
  Xda,
  Xdr,
  Xdr14,
  Xfpb,
  XlPr,
  Xlecs,
  Xlecs2,
  Xlmsforms,
  Xlpar,
  Xlpcalc,
  Xlpda,
  Xlpds,
  Xlrd,
  Xlrd2,
  Xlrdwi,
  Xlrvr,
  Xlrvrel,
  Xltc,
  Xltc2,
  Xlwcv,
  Xml,
  Xne,
  Xnsv,
  Xpdl,
  Xprd,
  Xr,
  Xr10,
  Xr16,
  Xr2,
  Xr3,
  Xr5,
  Xr6,
  Xr9,
  Xsd,
  Xsi,
  Xvml,
  Xxdsv,
  Xxl21,
  Xxlnp,
  Xxpim,
  Xxpvi,
}
impl Default for XmlKnownNamespace {
  #[inline]
  fn default() -> Self {
    Self::A
  }
}
impl XmlKnownNamespace {
  pub const fn prefix_bytes(self) -> &'static [u8] {
    match self {
      Self::A => "a".as_bytes(),
      Self::A14 => "a14".as_bytes(),
      Self::A15 => "a15".as_bytes(),
      Self::A16 => "a16".as_bytes(),
      Self::A1611 => "a1611".as_bytes(),
      Self::A3danim => "a3danim".as_bytes(),
      Self::Aanim => "aanim".as_bytes(),
      Self::Ac => "ac".as_bytes(),
      Self::Aclsh => "aclsh".as_bytes(),
      Self::Adec => "adec".as_bytes(),
      Self::Ahyp => "ahyp".as_bytes(),
      Self::Aif => "aif".as_bytes(),
      Self::Aink => "aink".as_bytes(),
      Self::Alf => "alf".as_bytes(),
      Self::Am3d => "am3d".as_bytes(),
      Self::Aoe => "aoe".as_bytes(),
      Self::Ap => "ap".as_bytes(),
      Self::Ask => "ask".as_bytes(),
      Self::Asl => "asl".as_bytes(),
      Self::Asvg => "asvg".as_bytes(),
      Self::Ax => "ax".as_bytes(),
      Self::B => "b".as_bytes(),
      Self::C => "c".as_bytes(),
      Self::C14 => "c14".as_bytes(),
      Self::C15 => "c15".as_bytes(),
      Self::C16 => "c16".as_bytes(),
      Self::C16ac => "c16ac".as_bytes(),
      Self::C16r2 => "c16r2".as_bytes(),
      Self::C16r3 => "c16r3".as_bytes(),
      Self::Cdip => "cdip".as_bytes(),
      Self::Cdr => "cdr".as_bytes(),
      Self::Cdr14 => "cdr14".as_bytes(),
      Self::Clbl => "clbl".as_bytes(),
      Self::Com14 => "com14".as_bytes(),
      Self::Comp => "comp".as_bytes(),
      Self::Cp => "cp".as_bytes(),
      Self::Cppr => "cppr".as_bytes(),
      Self::Cr => "cr".as_bytes(),
      Self::Cs => "cs".as_bytes(),
      Self::Ct => "ct".as_bytes(),
      Self::Cx => "cx".as_bytes(),
      Self::Cx1 => "cx1".as_bytes(),
      Self::Cx2 => "cx2".as_bytes(),
      Self::Cx3 => "cx3".as_bytes(),
      Self::Cx4 => "cx4".as_bytes(),
      Self::Cx5 => "cx5".as_bytes(),
      Self::Cx6 => "cx6".as_bytes(),
      Self::Cx7 => "cx7".as_bytes(),
      Self::Cx8 => "cx8".as_bytes(),
      Self::Dc => "dc".as_bytes(),
      Self::Dcmitype => "dcmitype".as_bytes(),
      Self::Dcterms => "dcterms".as_bytes(),
      Self::Dgm => "dgm".as_bytes(),
      Self::Dgm14 => "dgm14".as_bytes(),
      Self::Dgm1611 => "dgm1611".as_bytes(),
      Self::Dgm1612 => "dgm1612".as_bytes(),
      Self::Dms => "dms".as_bytes(),
      Self::Ds => "ds".as_bytes(),
      Self::Dsp => "dsp".as_bytes(),
      Self::Emma => "emma".as_bytes(),
      Self::Inkml => "inkml".as_bytes(),
      Self::Lc => "lc".as_bytes(),
      Self::Loext => "loext".as_bytes(),
      Self::Lp => "lp".as_bytes(),
      Self::M => "m".as_bytes(),
      Self::Ma => "ma".as_bytes(),
      Self::Ma14 => "ma14".as_bytes(),
      Self::Mc => "mc".as_bytes(),
      Self::Md => "md".as_bytes(),
      Self::Mo => "mo".as_bytes(),
      Self::Msink => "msink".as_bytes(),
      Self::Mso => "mso".as_bytes(),
      Self::Mso14 => "mso14".as_bytes(),
      Self::Mv => "mv".as_bytes(),
      Self::Mx => "mx".as_bytes(),
      Self::Ntns => "ntns".as_bytes(),
      Self::O => "o".as_bytes(),
      Self::Oa => "oa".as_bytes(),
      Self::Oac => "oac".as_bytes(),
      Self::Odoc => "odoc".as_bytes(),
      Self::Oel => "oel".as_bytes(),
      Self::Op => "op".as_bytes(),
      Self::Outs => "outs".as_bytes(),
      Self::P => "p".as_bytes(),
      Self::P14 => "p14".as_bytes(),
      Self::P15 => "p15".as_bytes(),
      Self::P1510 => "p1510".as_bytes(),
      Self::P16 => "p16".as_bytes(),
      Self::P1710 => "p1710".as_bytes(),
      Self::P173 => "p173".as_bytes(),
      Self::P184 => "p184".as_bytes(),
      Self::P188 => "p188".as_bytes(),
      Self::P1912 => "p1912".as_bytes(),
      Self::P202 => "p202".as_bytes(),
      Self::P216 => "p216".as_bytes(),
      Self::P223 => "p223".as_bytes(),
      Self::P228 => "p228".as_bytes(),
      Self::P232 => "p232".as_bytes(),
      Self::PRoam => "pRoam".as_bytes(),
      Self::Packaging => "packaging".as_bytes(),
      Self::Pav => "pav".as_bytes(),
      Self::Pc => "pc".as_bytes(),
      Self::Pc2 => "pc2".as_bytes(),
      Self::Pc226 => "pc226".as_bytes(),
      Self::Pic => "pic".as_bytes(),
      Self::Pic14 => "pic14".as_bytes(),
      Self::Pj15 => "pj15".as_bytes(),
      Self::Pvml => "pvml".as_bytes(),
      Self::R => "r".as_bytes(),
      Self::Sl => "sl".as_bytes(),
      Self::Sle => "sle".as_bytes(),
      Self::Spe => "spe".as_bytes(),
      Self::T => "t".as_bytes(),
      Self::Thm15 => "thm15".as_bytes(),
      Self::Tsle => "tsle".as_bytes(),
      Self::V => "v".as_bytes(),
      Self::Vt => "vt".as_bytes(),
      Self::W => "w".as_bytes(),
      Self::W10 => "w10".as_bytes(),
      Self::W14 => "w14".as_bytes(),
      Self::W15 => "w15".as_bytes(),
      Self::W16cex => "w16cex".as_bytes(),
      Self::W16cid => "w16cid".as_bytes(),
      Self::W16cur => "w16cur".as_bytes(),
      Self::W16du => "w16du".as_bytes(),
      Self::W16sdtdh => "w16sdtdh".as_bytes(),
      Self::W16sdtfl => "w16sdtfl".as_bytes(),
      Self::W16se => "w16se".as_bytes(),
      Self::We => "we".as_bytes(),
      Self::Wetp => "wetp".as_bytes(),
      Self::Wne => "wne".as_bytes(),
      Self::Woe => "woe".as_bytes(),
      Self::Wp => "wp".as_bytes(),
      Self::Wp14 => "wp14".as_bytes(),
      Self::Wp15 => "wp15".as_bytes(),
      Self::Wpc => "wpc".as_bytes(),
      Self::Wpg => "wpg".as_bytes(),
      Self::Wpi => "wpi".as_bytes(),
      Self::Wps => "wps".as_bytes(),
      Self::X => "x".as_bytes(),
      Self::X12ac => "x12ac".as_bytes(),
      Self::X14 => "x14".as_bytes(),
      Self::X14ac => "x14ac".as_bytes(),
      Self::X15 => "x15".as_bytes(),
      Self::X15ac => "x15ac".as_bytes(),
      Self::X16 => "x16".as_bytes(),
      Self::X16r2 => "x16r2".as_bytes(),
      Self::X16r3 => "x16r3".as_bytes(),
      Self::Xcalcf => "xcalcf".as_bytes(),
      Self::Xda => "xda".as_bytes(),
      Self::Xdr => "xdr".as_bytes(),
      Self::Xdr14 => "xdr14".as_bytes(),
      Self::Xfpb => "xfpb".as_bytes(),
      Self::XlPr => "xlPr".as_bytes(),
      Self::Xlecs => "xlecs".as_bytes(),
      Self::Xlecs2 => "xlecs2".as_bytes(),
      Self::Xlmsforms => "xlmsforms".as_bytes(),
      Self::Xlpar => "xlpar".as_bytes(),
      Self::Xlpcalc => "xlpcalc".as_bytes(),
      Self::Xlpda => "xlpda".as_bytes(),
      Self::Xlpds => "xlpds".as_bytes(),
      Self::Xlrd => "xlrd".as_bytes(),
      Self::Xlrd2 => "xlrd2".as_bytes(),
      Self::Xlrdwi => "xlrdwi".as_bytes(),
      Self::Xlrvr => "xlrvr".as_bytes(),
      Self::Xlrvrel => "xlrvrel".as_bytes(),
      Self::Xltc => "xltc".as_bytes(),
      Self::Xltc2 => "xltc2".as_bytes(),
      Self::Xlwcv => "xlwcv".as_bytes(),
      Self::Xml => "xml".as_bytes(),
      Self::Xne => "xne".as_bytes(),
      Self::Xnsv => "xnsv".as_bytes(),
      Self::Xpdl => "xpdl".as_bytes(),
      Self::Xprd => "xprd".as_bytes(),
      Self::Xr => "xr".as_bytes(),
      Self::Xr10 => "xr10".as_bytes(),
      Self::Xr16 => "xr16".as_bytes(),
      Self::Xr2 => "xr2".as_bytes(),
      Self::Xr3 => "xr3".as_bytes(),
      Self::Xr5 => "xr5".as_bytes(),
      Self::Xr6 => "xr6".as_bytes(),
      Self::Xr9 => "xr9".as_bytes(),
      Self::Xsd => "xsd".as_bytes(),
      Self::Xsi => "xsi".as_bytes(),
      Self::Xvml => "xvml".as_bytes(),
      Self::Xxdsv => "xxdsv".as_bytes(),
      Self::Xxl21 => "xxl21".as_bytes(),
      Self::Xxlnp => "xxlnp".as_bytes(),
      Self::Xxpim => "xxpim".as_bytes(),
      Self::Xxpvi => "xxpvi".as_bytes(),
    }
  }
  pub const fn prefix(self) -> &'static str {
    unsafe { std::str::from_utf8_unchecked(self.prefix_bytes()) }
  }
  pub const fn uri_bytes(self) -> &'static [u8] {
    match self {
      Self::A => "http://schemas.openxmlformats.org/drawingml/2006/main".as_bytes(),
      Self::A14 => "http://schemas.microsoft.com/office/drawing/2010/main".as_bytes(),
      Self::A15 => "http://schemas.microsoft.com/office/drawing/2012/main".as_bytes(),
      Self::A16 => "http://schemas.microsoft.com/office/drawing/2014/main".as_bytes(),
      Self::A1611 => "http://schemas.microsoft.com/office/drawing/2016/11/main".as_bytes(),
      Self::A3danim => {
        "http://schemas.microsoft.com/office/drawing/2018/animation/model3d".as_bytes()
      }
      Self::Aanim => "http://schemas.microsoft.com/office/drawing/2018/animation".as_bytes(),
      Self::Ac => {
        "http://schemas.openxmlformats.org/officeDocument/2006/characteristics".as_bytes()
      }
      Self::Aclsh => {
        "http://schemas.microsoft.com/office/drawing/2020/classificationShape".as_bytes()
      }
      Self::Adec => "http://schemas.microsoft.com/office/drawing/2017/decorative".as_bytes(),
      Self::Ahyp => "http://schemas.microsoft.com/office/drawing/2018/hyperlinkcolor".as_bytes(),
      Self::Aif => "http://schemas.microsoft.com/office/drawing/2022/imageformula".as_bytes(),
      Self::Aink => "http://schemas.microsoft.com/office/drawing/2016/ink".as_bytes(),
      Self::Alf => "http://schemas.microsoft.com/office/drawing/2021/livefeed".as_bytes(),
      Self::Am3d => "http://schemas.microsoft.com/office/drawing/2017/model3d".as_bytes(),
      Self::Aoe => "http://schemas.microsoft.com/office/drawing/2021/oembed".as_bytes(),
      Self::Ap => {
        "http://schemas.openxmlformats.org/officeDocument/2006/extended-properties".as_bytes()
      }
      Self::Ask => "http://schemas.microsoft.com/office/drawing/2018/sketchyshapes".as_bytes(),
      Self::Asl => "http://schemas.microsoft.com/office/drawing/2021/scriptlink".as_bytes(),
      Self::Asvg => "http://schemas.microsoft.com/office/drawing/2016/SVG/main".as_bytes(),
      Self::Ax => "http://schemas.microsoft.com/office/2006/activeX".as_bytes(),
      Self::B => "http://schemas.openxmlformats.org/officeDocument/2006/bibliography".as_bytes(),
      Self::C => "http://schemas.openxmlformats.org/drawingml/2006/chart".as_bytes(),
      Self::C14 => "http://schemas.microsoft.com/office/drawing/2007/8/2/chart".as_bytes(),
      Self::C15 => "http://schemas.microsoft.com/office/drawing/2012/chart".as_bytes(),
      Self::C16 => "http://schemas.microsoft.com/office/drawing/2014/chart".as_bytes(),
      Self::C16ac => "http://schemas.microsoft.com/office/drawing/2014/chart/ac".as_bytes(),
      Self::C16r2 => "http://schemas.microsoft.com/office/drawing/2015/06/chart".as_bytes(),
      Self::C16r3 => "http://schemas.microsoft.com/office/drawing/2017/03/chart".as_bytes(),
      Self::Cdip => {
        "http://schemas.microsoft.com/office/2006/customDocumentInformationPanel".as_bytes()
      }
      Self::Cdr => "http://schemas.openxmlformats.org/drawingml/2006/chartDrawing".as_bytes(),
      Self::Cdr14 => "http://schemas.microsoft.com/office/drawing/2010/chartDrawing".as_bytes(),
      Self::Clbl => "http://schemas.microsoft.com/office/2020/mipLabelMetadata".as_bytes(),
      Self::Com14 => "http://schemas.microsoft.com/office/drawing/2010/compatibility".as_bytes(),
      Self::Comp => "http://schemas.openxmlformats.org/drawingml/2006/compatibility".as_bytes(),
      Self::Cp => {
        "http://schemas.openxmlformats.org/package/2006/metadata/core-properties".as_bytes()
      }
      Self::Cppr => "http://schemas.microsoft.com/office/2006/coverPageProps".as_bytes(),
      Self::Cr => "http://schemas.microsoft.com/office/comments/2020/reactions".as_bytes(),
      Self::Cs => "http://schemas.microsoft.com/office/drawing/2012/chartStyle".as_bytes(),
      Self::Ct => "http://schemas.microsoft.com/office/2006/metadata/contentType".as_bytes(),
      Self::Cx => "http://schemas.microsoft.com/office/drawing/2014/chartex".as_bytes(),
      Self::Cx1 => "http://schemas.microsoft.com/office/drawing/2015/9/8/chartex".as_bytes(),
      Self::Cx2 => "http://schemas.microsoft.com/office/drawing/2015/10/21/chartex".as_bytes(),
      Self::Cx3 => "http://schemas.microsoft.com/office/drawing/2016/5/9/chartex".as_bytes(),
      Self::Cx4 => "http://schemas.microsoft.com/office/drawing/2016/5/10/chartex".as_bytes(),
      Self::Cx5 => "http://schemas.microsoft.com/office/drawing/2016/5/11/chartex".as_bytes(),
      Self::Cx6 => "http://schemas.microsoft.com/office/drawing/2016/5/12/chartex".as_bytes(),
      Self::Cx7 => "http://schemas.microsoft.com/office/drawing/2016/5/13/chartex".as_bytes(),
      Self::Cx8 => "http://schemas.microsoft.com/office/drawing/2016/5/14/chartex".as_bytes(),
      Self::Dc => "http://purl.org/dc/elements/1.1/".as_bytes(),
      Self::Dcmitype => "http://purl.org/dc/dcmitype/".as_bytes(),
      Self::Dcterms => "http://purl.org/dc/terms/".as_bytes(),
      Self::Dgm => "http://schemas.openxmlformats.org/drawingml/2006/diagram".as_bytes(),
      Self::Dgm14 => "http://schemas.microsoft.com/office/drawing/2010/diagram".as_bytes(),
      Self::Dgm1611 => "http://schemas.microsoft.com/office/drawing/2016/11/diagram".as_bytes(),
      Self::Dgm1612 => "http://schemas.microsoft.com/office/drawing/2016/12/diagram".as_bytes(),
      Self::Dms => "http://schemas.microsoft.com/office/2006/documentManagement/types".as_bytes(),
      Self::Ds => "http://schemas.openxmlformats.org/officeDocument/2006/customXml".as_bytes(),
      Self::Dsp => "http://schemas.microsoft.com/office/drawing/2008/diagram".as_bytes(),
      Self::Emma => "http://www.w3.org/2003/04/emma".as_bytes(),
      Self::Inkml => "http://www.w3.org/2003/InkML".as_bytes(),
      Self::Lc => "http://schemas.openxmlformats.org/drawingml/2006/lockedCanvas".as_bytes(),
      Self::Loext => "http://schemas.libreoffice.org/".as_bytes(),
      Self::Lp => "http://schemas.microsoft.com/office/2006/metadata/longProperties".as_bytes(),
      Self::M => "http://schemas.openxmlformats.org/officeDocument/2006/math".as_bytes(),
      Self::Ma => {
        "http://schemas.microsoft.com/office/2006/metadata/properties/metaAttributes".as_bytes()
      }
      Self::Ma14 => "http://schemas.microsoft.com/office/mac/drawingml/2011/main".as_bytes(),
      Self::Mc => "http://schemas.openxmlformats.org/markup-compatibility/2006".as_bytes(),
      Self::Md => "http://schemas.microsoft.com/office/office/2011/9/metroDictionary".as_bytes(),
      Self::Mo => "http://schemas.microsoft.com/office/mac/office/2008/main".as_bytes(),
      Self::Msink => "http://schemas.microsoft.com/ink/2010/main".as_bytes(),
      Self::Mso => "http://schemas.microsoft.com/office/2006/01/customui".as_bytes(),
      Self::Mso14 => "http://schemas.microsoft.com/office/2009/07/customui".as_bytes(),
      Self::Mv => "urn:schemas-microsoft-com:mac:vml".as_bytes(),
      Self::Mx => "http://schemas.microsoft.com/office/mac/excel/2008/main".as_bytes(),
      Self::Ntns => "http://schemas.microsoft.com/office/2006/metadata/customXsn".as_bytes(),
      Self::O => "urn:schemas-microsoft-com:office:office".as_bytes(),
      Self::Oa => "urn:schemas-microsoft-com:office:activation".as_bytes(),
      Self::Oac => "http://schemas.microsoft.com/office/drawing/2013/main/command".as_bytes(),
      Self::Odoc => "http://schemas.microsoft.com/internal/obd".as_bytes(),
      Self::Oel => "http://schemas.microsoft.com/office/2019/extlst".as_bytes(),
      Self::Op => {
        "http://schemas.openxmlformats.org/officeDocument/2006/custom-properties".as_bytes()
      }
      Self::Outs => "http://schemas.microsoft.com/office/2009/outspace/metadata".as_bytes(),
      Self::P => "http://schemas.openxmlformats.org/presentationml/2006/main".as_bytes(),
      Self::P14 => "http://schemas.microsoft.com/office/powerpoint/2010/main".as_bytes(),
      Self::P15 => "http://schemas.microsoft.com/office/powerpoint/2012/main".as_bytes(),
      Self::P1510 => "http://schemas.microsoft.com/office/powerpoint/2015/10/main".as_bytes(),
      Self::P16 => "http://schemas.microsoft.com/office/powerpoint/2015/main".as_bytes(),
      Self::P1710 => "http://schemas.microsoft.com/office/powerpoint/2017/10/main".as_bytes(),
      Self::P173 => "http://schemas.microsoft.com/office/powerpoint/2017/3/main".as_bytes(),
      Self::P184 => "http://schemas.microsoft.com/office/powerpoint/2018/4/main".as_bytes(),
      Self::P188 => "http://schemas.microsoft.com/office/powerpoint/2018/8/main".as_bytes(),
      Self::P1912 => "http://schemas.microsoft.com/office/powerpoint/2019/12/main".as_bytes(),
      Self::P202 => "http://schemas.microsoft.com/office/powerpoint/2020/02/main".as_bytes(),
      Self::P216 => "http://schemas.microsoft.com/office/powerpoint/2021/06/main".as_bytes(),
      Self::P223 => "http://schemas.microsoft.com/office/powerpoint/2022/03/main".as_bytes(),
      Self::P228 => "http://schemas.microsoft.com/office/powerpoint/2022/08/main".as_bytes(),
      Self::P232 => "http://schemas.microsoft.com/office/powerpoint/2023/02/main".as_bytes(),
      Self::PRoam => {
        "http://schemas.microsoft.com/office/powerpoint/2012/roamingSettings".as_bytes()
      }
      Self::Packaging => {
        "http://schemas.microsoft.com/office/internal/2007/ofapi/packaging".as_bytes()
      }
      Self::Pav => "http://schemas.microsoft.com/office/2007/6/19/audiovideo".as_bytes(),
      Self::Pc => "http://schemas.microsoft.com/office/powerpoint/2013/main/command".as_bytes(),
      Self::Pc2 => "http://schemas.microsoft.com/office/powerpoint/2019/9/main/command".as_bytes(),
      Self::Pc226 => {
        "http://schemas.microsoft.com/office/powerpoint/2022/06/main/command".as_bytes()
      }
      Self::Pic => "http://schemas.openxmlformats.org/drawingml/2006/picture".as_bytes(),
      Self::Pic14 => "http://schemas.microsoft.com/office/drawing/2010/picture".as_bytes(),
      Self::Pj15 => "http://schemas.microsoft.com/projectml/2012/main".as_bytes(),
      Self::Pvml => "urn:schemas-microsoft-com:office:powerpoint".as_bytes(),
      Self::R => "http://schemas.openxmlformats.org/officeDocument/2006/relationships".as_bytes(),
      Self::Sl => "http://schemas.openxmlformats.org/schemaLibrary/2006/main".as_bytes(),
      Self::Sle => "http://schemas.microsoft.com/office/drawing/2010/slicer".as_bytes(),
      Self::Spe => "http://schemas.microsoft.com/sharepoint/events".as_bytes(),
      Self::T => "http://schemas.microsoft.com/office/tasks/2019/documenttasks".as_bytes(),
      Self::Thm15 => "http://schemas.microsoft.com/office/thememl/2012/main".as_bytes(),
      Self::Tsle => "http://schemas.microsoft.com/office/drawing/2012/timeslicer".as_bytes(),
      Self::V => "urn:schemas-microsoft-com:vml".as_bytes(),
      Self::Vt => "http://schemas.openxmlformats.org/officeDocument/2006/docPropsVTypes".as_bytes(),
      Self::W => "http://schemas.openxmlformats.org/wordprocessingml/2006/main".as_bytes(),
      Self::W10 => "urn:schemas-microsoft-com:office:word".as_bytes(),
      Self::W14 => "http://schemas.microsoft.com/office/word/2010/wordml".as_bytes(),
      Self::W15 => "http://schemas.microsoft.com/office/word/2012/wordml".as_bytes(),
      Self::W16cex => "http://schemas.microsoft.com/office/word/2018/wordml/cex".as_bytes(),
      Self::W16cid => "http://schemas.microsoft.com/office/word/2016/wordml/cid".as_bytes(),
      Self::W16cur => "http://schemas.microsoft.com/office/word/2018/wordml".as_bytes(),
      Self::W16du => "http://schemas.microsoft.com/office/word/2023/wordml/word16du".as_bytes(),
      Self::W16sdtdh => {
        "http://schemas.microsoft.com/office/word/2020/wordml/sdtdatahash".as_bytes()
      }
      Self::W16sdtfl => {
        "http://schemas.microsoft.com/office/word/2024/wordml/sdtformatlock".as_bytes()
      }
      Self::W16se => "http://schemas.microsoft.com/office/word/2015/wordml/symex".as_bytes(),
      Self::We => {
        "http://schemas.microsoft.com/office/webextensions/webextension/2010/11".as_bytes()
      }
      Self::Wetp => {
        "http://schemas.microsoft.com/office/webextensions/taskpanes/2010/11".as_bytes()
      }
      Self::Wne => "http://schemas.microsoft.com/office/word/2006/wordml".as_bytes(),
      Self::Woe => "http://schemas.microsoft.com/office/word/2020/oembed".as_bytes(),
      Self::Wp => {
        "http://schemas.openxmlformats.org/drawingml/2006/wordprocessingDrawing".as_bytes()
      }
      Self::Wp14 => {
        "http://schemas.microsoft.com/office/word/2010/wordprocessingDrawing".as_bytes()
      }
      Self::Wp15 => {
        "http://schemas.microsoft.com/office/word/2012/wordprocessingDrawing".as_bytes()
      }
      Self::Wpc => "http://schemas.microsoft.com/office/word/2010/wordprocessingCanvas".as_bytes(),
      Self::Wpg => "http://schemas.microsoft.com/office/word/2010/wordprocessingGroup".as_bytes(),
      Self::Wpi => "http://schemas.microsoft.com/office/word/2010/wordprocessingInk".as_bytes(),
      Self::Wps => "http://schemas.microsoft.com/office/word/2010/wordprocessingShape".as_bytes(),
      Self::X => "http://schemas.openxmlformats.org/spreadsheetml/2006/main".as_bytes(),
      Self::X12ac => "http://schemas.microsoft.com/office/spreadsheetml/2011/1/ac".as_bytes(),
      Self::X14 => "http://schemas.microsoft.com/office/spreadsheetml/2009/9/main".as_bytes(),
      Self::X14ac => "http://schemas.microsoft.com/office/spreadsheetml/2009/9/ac".as_bytes(),
      Self::X15 => "http://schemas.microsoft.com/office/spreadsheetml/2010/11/main".as_bytes(),
      Self::X15ac => "http://schemas.microsoft.com/office/spreadsheetml/2010/11/ac".as_bytes(),
      Self::X16 => "http://schemas.microsoft.com/office/spreadsheetml/2014/11/main".as_bytes(),
      Self::X16r2 => "http://schemas.microsoft.com/office/spreadsheetml/2015/02/main".as_bytes(),
      Self::X16r3 => "http://schemas.microsoft.com/office/spreadsheetml/2018/08/main".as_bytes(),
      Self::Xcalcf => {
        "http://schemas.microsoft.com/office/spreadsheetml/2018/calcfeatures".as_bytes()
      }
      Self::Xda => "http://schemas.microsoft.com/office/spreadsheetml/2017/dynamicarray".as_bytes(),
      Self::Xdr => "http://schemas.openxmlformats.org/drawingml/2006/spreadsheetDrawing".as_bytes(),
      Self::Xdr14 => "http://schemas.microsoft.com/office/excel/2010/spreadsheetDrawing".as_bytes(),
      Self::Xfpb => {
        "http://schemas.microsoft.com/office/spreadsheetml/2022/featurepropertybag".as_bytes()
      }
      Self::XlPr => "http://schemas.microsoft.com/office/spreadsheetml/2016/01/main".as_bytes(),
      Self::Xlecs => {
        "http://schemas.microsoft.com/office/spreadsheetml/2023/externalCodeService".as_bytes()
      }
      Self::Xlecs2 => {
        "http://schemas.microsoft.com/office/spreadsheetml/2025/externalCodeService2".as_bytes()
      }
      Self::Xlmsforms => {
        "http://schemas.microsoft.com/office/spreadsheetml/2023/msForms".as_bytes()
      }
      Self::Xlpar => {
        "http://schemas.microsoft.com/office/spreadsheetml/2024/pivotAutoRefresh".as_bytes()
      }
      Self::Xlpcalc => {
        "http://schemas.microsoft.com/office/spreadsheetml/2023/pivot2023Calculation".as_bytes()
      }
      Self::Xlpda => {
        "http://schemas.microsoft.com/office/spreadsheetml/2024/pivotDynamicArrays".as_bytes()
      }
      Self::Xlpds => {
        "http://schemas.microsoft.com/office/spreadsheetml/2025/pivotDataSource".as_bytes()
      }
      Self::Xlrd => "http://schemas.microsoft.com/office/spreadsheetml/2017/richdata".as_bytes(),
      Self::Xlrd2 => "http://schemas.microsoft.com/office/spreadsheetml/2017/richdata2".as_bytes(),
      Self::Xlrdwi => {
        "http://schemas.microsoft.com/office/spreadsheetml/2020/richdatawebimage".as_bytes()
      }
      Self::Xlrvr => {
        "http://schemas.microsoft.com/office/spreadsheetml/2020/richvaluerefresh".as_bytes()
      }
      Self::Xlrvrel => {
        "http://schemas.microsoft.com/office/spreadsheetml/2022/richvaluerel".as_bytes()
      }
      Self::Xltc => {
        "http://schemas.microsoft.com/office/spreadsheetml/2018/threadedcomments".as_bytes()
      }
      Self::Xltc2 => {
        "http://schemas.microsoft.com/office/spreadsheetml/2020/threadedcomments2".as_bytes()
      }
      Self::Xlwcv => {
        "http://schemas.microsoft.com/office/spreadsheetml/2024/workbookCompatibilityVersion"
          .as_bytes()
      }
      Self::Xml => "http://www.w3.org/XML/1998/namespace".as_bytes(),
      Self::Xne => "http://schemas.microsoft.com/office/excel/2006/main".as_bytes(),
      Self::Xnsv => {
        "http://schemas.microsoft.com/office/spreadsheetml/2019/namedsheetviews".as_bytes()
      }
      Self::Xpdl => {
        "http://schemas.microsoft.com/office/spreadsheetml/2016/pivotdefaultlayout".as_bytes()
      }
      Self::Xprd => {
        "http://schemas.microsoft.com/office/spreadsheetml/2022/pivotRichData".as_bytes()
      }
      Self::Xr => "http://schemas.microsoft.com/office/spreadsheetml/2014/revision".as_bytes(),
      Self::Xr10 => "http://schemas.microsoft.com/office/spreadsheetml/2016/revision10".as_bytes(),
      Self::Xr16 => "http://schemas.microsoft.com/office/spreadsheetml/2017/revision16".as_bytes(),
      Self::Xr2 => "http://schemas.microsoft.com/office/spreadsheetml/2015/revision2".as_bytes(),
      Self::Xr3 => "http://schemas.microsoft.com/office/spreadsheetml/2016/revision3".as_bytes(),
      Self::Xr5 => "http://schemas.microsoft.com/office/spreadsheetml/2016/revision5".as_bytes(),
      Self::Xr6 => "http://schemas.microsoft.com/office/spreadsheetml/2016/revision6".as_bytes(),
      Self::Xr9 => "http://schemas.microsoft.com/office/spreadsheetml/2016/revision9".as_bytes(),
      Self::Xsd => "http://www.w3.org/2001/XMLSchema".as_bytes(),
      Self::Xsi => "http://www.w3.org/2001/XMLSchema-instance".as_bytes(),
      Self::Xvml => "urn:schemas-microsoft-com:office:excel".as_bytes(),
      Self::Xxdsv => {
        "http://schemas.microsoft.com/office/spreadsheetml/2023/dataSourceVersioning".as_bytes()
      }
      Self::Xxl21 => {
        "http://schemas.microsoft.com/office/spreadsheetml/2021/extlinks2021".as_bytes()
      }
      Self::Xxlnp => {
        "http://schemas.microsoft.com/office/spreadsheetml/2019/extlinksprops".as_bytes()
      }
      Self::Xxpim => {
        "http://schemas.microsoft.com/office/spreadsheetml/2020/pivotNov2020".as_bytes()
      }
      Self::Xxpvi => {
        "http://schemas.microsoft.com/office/spreadsheetml/2022/pivotVersionInfo".as_bytes()
      }
    }
  }
  pub const fn uri(self) -> &'static str {
    unsafe { std::str::from_utf8_unchecked(self.uri_bytes()) }
  }
  pub fn from_uri(uri: &str) -> Option<Self> {
    Self::from_uri_bytes(uri.as_bytes())
  }
  pub fn from_uri_bytes(uri: &[u8]) -> Option<Self> {
    match uri {
      b"http://schemas.openxmlformats.org/drawingml/2006/main" => Some(Self::A),
      b"http://schemas.microsoft.com/office/drawing/2010/main" => Some(Self::A14),
      b"http://schemas.microsoft.com/office/drawing/2012/main" => Some(Self::A15),
      b"http://schemas.microsoft.com/office/drawing/2014/main" => Some(Self::A16),
      b"http://schemas.microsoft.com/office/drawing/2016/11/main" => Some(Self::A1611),
      b"http://schemas.microsoft.com/office/drawing/2018/animation/model3d" => Some(Self::A3danim),
      b"http://schemas.microsoft.com/office/drawing/2018/animation" => Some(Self::Aanim),
      b"http://schemas.openxmlformats.org/officeDocument/2006/characteristics" => Some(Self::Ac),
      b"http://schemas.microsoft.com/office/drawing/2020/classificationShape" => Some(Self::Aclsh),
      b"http://schemas.microsoft.com/office/drawing/2017/decorative" => Some(Self::Adec),
      b"http://schemas.microsoft.com/office/drawing/2018/hyperlinkcolor" => Some(Self::Ahyp),
      b"http://schemas.microsoft.com/office/drawing/2022/imageformula" => Some(Self::Aif),
      b"http://schemas.microsoft.com/office/drawing/2016/ink" => Some(Self::Aink),
      b"http://schemas.microsoft.com/office/drawing/2021/livefeed" => Some(Self::Alf),
      b"http://schemas.microsoft.com/office/drawing/2017/model3d" => Some(Self::Am3d),
      b"http://schemas.microsoft.com/office/drawing/2021/oembed" => Some(Self::Aoe),
      b"http://schemas.openxmlformats.org/officeDocument/2006/extended-properties" => {
        Some(Self::Ap)
      }
      b"http://schemas.microsoft.com/office/drawing/2018/sketchyshapes" => Some(Self::Ask),
      b"http://schemas.microsoft.com/office/drawing/2021/scriptlink" => Some(Self::Asl),
      b"http://schemas.microsoft.com/office/drawing/2016/SVG/main" => Some(Self::Asvg),
      b"http://schemas.microsoft.com/office/2006/activeX" => Some(Self::Ax),
      b"http://schemas.openxmlformats.org/officeDocument/2006/bibliography" => Some(Self::B),
      b"http://schemas.openxmlformats.org/drawingml/2006/chart" => Some(Self::C),
      b"http://schemas.microsoft.com/office/drawing/2007/8/2/chart" => Some(Self::C14),
      b"http://schemas.microsoft.com/office/drawing/2012/chart" => Some(Self::C15),
      b"http://schemas.microsoft.com/office/drawing/2014/chart" => Some(Self::C16),
      b"http://schemas.microsoft.com/office/drawing/2014/chart/ac" => Some(Self::C16ac),
      b"http://schemas.microsoft.com/office/drawing/2015/06/chart" => Some(Self::C16r2),
      b"http://schemas.microsoft.com/office/drawing/2017/03/chart" => Some(Self::C16r3),
      b"http://schemas.microsoft.com/office/2006/customDocumentInformationPanel" => {
        Some(Self::Cdip)
      }
      b"http://schemas.openxmlformats.org/drawingml/2006/chartDrawing" => Some(Self::Cdr),
      b"http://schemas.microsoft.com/office/drawing/2010/chartDrawing" => Some(Self::Cdr14),
      b"http://schemas.microsoft.com/office/2020/mipLabelMetadata" => Some(Self::Clbl),
      b"http://schemas.microsoft.com/office/drawing/2010/compatibility" => Some(Self::Com14),
      b"http://schemas.openxmlformats.org/drawingml/2006/compatibility" => Some(Self::Comp),
      b"http://schemas.openxmlformats.org/package/2006/metadata/core-properties" => Some(Self::Cp),
      b"http://schemas.microsoft.com/office/2006/coverPageProps" => Some(Self::Cppr),
      b"http://schemas.microsoft.com/office/comments/2020/reactions" => Some(Self::Cr),
      b"http://schemas.microsoft.com/office/drawing/2012/chartStyle" => Some(Self::Cs),
      b"http://schemas.microsoft.com/office/2006/metadata/contentType" => Some(Self::Ct),
      b"http://schemas.microsoft.com/office/drawing/2014/chartex" => Some(Self::Cx),
      b"http://schemas.microsoft.com/office/drawing/2015/9/8/chartex" => Some(Self::Cx1),
      b"http://schemas.microsoft.com/office/drawing/2015/10/21/chartex" => Some(Self::Cx2),
      b"http://schemas.microsoft.com/office/drawing/2016/5/9/chartex" => Some(Self::Cx3),
      b"http://schemas.microsoft.com/office/drawing/2016/5/10/chartex" => Some(Self::Cx4),
      b"http://schemas.microsoft.com/office/drawing/2016/5/11/chartex" => Some(Self::Cx5),
      b"http://schemas.microsoft.com/office/drawing/2016/5/12/chartex" => Some(Self::Cx6),
      b"http://schemas.microsoft.com/office/drawing/2016/5/13/chartex" => Some(Self::Cx7),
      b"http://schemas.microsoft.com/office/drawing/2016/5/14/chartex" => Some(Self::Cx8),
      b"http://purl.org/dc/elements/1.1/" => Some(Self::Dc),
      b"http://purl.org/dc/dcmitype/" => Some(Self::Dcmitype),
      b"http://purl.org/dc/terms/" => Some(Self::Dcterms),
      b"http://schemas.openxmlformats.org/drawingml/2006/diagram" => Some(Self::Dgm),
      b"http://schemas.microsoft.com/office/drawing/2010/diagram" => Some(Self::Dgm14),
      b"http://schemas.microsoft.com/office/drawing/2016/11/diagram" => Some(Self::Dgm1611),
      b"http://schemas.microsoft.com/office/drawing/2016/12/diagram" => Some(Self::Dgm1612),
      b"http://schemas.microsoft.com/office/2006/documentManagement/types" => Some(Self::Dms),
      b"http://schemas.openxmlformats.org/officeDocument/2006/customXml" => Some(Self::Ds),
      b"http://schemas.microsoft.com/office/drawing/2008/diagram" => Some(Self::Dsp),
      b"http://www.w3.org/2003/04/emma" => Some(Self::Emma),
      b"http://www.w3.org/2003/InkML" => Some(Self::Inkml),
      b"http://schemas.openxmlformats.org/drawingml/2006/lockedCanvas" => Some(Self::Lc),
      b"http://schemas.libreoffice.org/" => Some(Self::Loext),
      b"http://schemas.microsoft.com/office/2006/metadata/longProperties" => Some(Self::Lp),
      b"http://schemas.openxmlformats.org/officeDocument/2006/math" => Some(Self::M),
      b"http://schemas.microsoft.com/office/2006/metadata/properties/metaAttributes" => {
        Some(Self::Ma)
      }
      b"http://schemas.microsoft.com/office/mac/drawingml/2011/main" => Some(Self::Ma14),
      b"http://schemas.openxmlformats.org/markup-compatibility/2006" => Some(Self::Mc),
      b"http://schemas.microsoft.com/office/office/2011/9/metroDictionary" => Some(Self::Md),
      b"http://schemas.microsoft.com/office/mac/office/2008/main" => Some(Self::Mo),
      b"http://schemas.microsoft.com/ink/2010/main" => Some(Self::Msink),
      b"http://schemas.microsoft.com/office/2006/01/customui" => Some(Self::Mso),
      b"http://schemas.microsoft.com/office/2009/07/customui" => Some(Self::Mso14),
      b"urn:schemas-microsoft-com:mac:vml" => Some(Self::Mv),
      b"http://schemas.microsoft.com/office/mac/excel/2008/main" => Some(Self::Mx),
      b"http://schemas.microsoft.com/office/2006/metadata/customXsn" => Some(Self::Ntns),
      b"urn:schemas-microsoft-com:office:office" => Some(Self::O),
      b"urn:schemas-microsoft-com:office:activation" => Some(Self::Oa),
      b"http://schemas.microsoft.com/office/drawing/2013/main/command" => Some(Self::Oac),
      b"http://schemas.microsoft.com/internal/obd" => Some(Self::Odoc),
      b"http://schemas.microsoft.com/office/2019/extlst" => Some(Self::Oel),
      b"http://schemas.openxmlformats.org/officeDocument/2006/custom-properties" => Some(Self::Op),
      b"http://schemas.microsoft.com/office/2009/outspace/metadata" => Some(Self::Outs),
      b"http://schemas.openxmlformats.org/presentationml/2006/main" => Some(Self::P),
      b"http://schemas.microsoft.com/office/powerpoint/2010/main" => Some(Self::P14),
      b"http://schemas.microsoft.com/office/powerpoint/2012/main" => Some(Self::P15),
      b"http://schemas.microsoft.com/office/powerpoint/2015/10/main" => Some(Self::P1510),
      b"http://schemas.microsoft.com/office/powerpoint/2015/main" => Some(Self::P16),
      b"http://schemas.microsoft.com/office/powerpoint/2017/10/main" => Some(Self::P1710),
      b"http://schemas.microsoft.com/office/powerpoint/2017/3/main" => Some(Self::P173),
      b"http://schemas.microsoft.com/office/powerpoint/2018/4/main" => Some(Self::P184),
      b"http://schemas.microsoft.com/office/powerpoint/2018/8/main" => Some(Self::P188),
      b"http://schemas.microsoft.com/office/powerpoint/2019/12/main" => Some(Self::P1912),
      b"http://schemas.microsoft.com/office/powerpoint/2020/02/main" => Some(Self::P202),
      b"http://schemas.microsoft.com/office/powerpoint/2021/06/main" => Some(Self::P216),
      b"http://schemas.microsoft.com/office/powerpoint/2022/03/main" => Some(Self::P223),
      b"http://schemas.microsoft.com/office/powerpoint/2022/08/main" => Some(Self::P228),
      b"http://schemas.microsoft.com/office/powerpoint/2023/02/main" => Some(Self::P232),
      b"http://schemas.microsoft.com/office/powerpoint/2012/roamingSettings" => Some(Self::PRoam),
      b"http://schemas.microsoft.com/office/internal/2007/ofapi/packaging" => Some(Self::Packaging),
      b"http://schemas.microsoft.com/office/2007/6/19/audiovideo" => Some(Self::Pav),
      b"http://schemas.microsoft.com/office/powerpoint/2013/main/command" => Some(Self::Pc),
      b"http://schemas.microsoft.com/office/powerpoint/2019/9/main/command" => Some(Self::Pc2),
      b"http://schemas.microsoft.com/office/powerpoint/2022/06/main/command" => Some(Self::Pc226),
      b"http://schemas.openxmlformats.org/drawingml/2006/picture" => Some(Self::Pic),
      b"http://schemas.microsoft.com/office/drawing/2010/picture" => Some(Self::Pic14),
      b"http://schemas.microsoft.com/projectml/2012/main" => Some(Self::Pj15),
      b"urn:schemas-microsoft-com:office:powerpoint" => Some(Self::Pvml),
      b"http://schemas.openxmlformats.org/officeDocument/2006/relationships" => Some(Self::R),
      b"http://schemas.openxmlformats.org/schemaLibrary/2006/main" => Some(Self::Sl),
      b"http://schemas.microsoft.com/office/drawing/2010/slicer" => Some(Self::Sle),
      b"http://schemas.microsoft.com/sharepoint/events" => Some(Self::Spe),
      b"http://schemas.microsoft.com/office/tasks/2019/documenttasks" => Some(Self::T),
      b"http://schemas.microsoft.com/office/thememl/2012/main" => Some(Self::Thm15),
      b"http://schemas.microsoft.com/office/drawing/2012/timeslicer" => Some(Self::Tsle),
      b"urn:schemas-microsoft-com:vml" => Some(Self::V),
      b"http://schemas.openxmlformats.org/officeDocument/2006/docPropsVTypes" => Some(Self::Vt),
      b"http://schemas.openxmlformats.org/wordprocessingml/2006/main" => Some(Self::W),
      b"urn:schemas-microsoft-com:office:word" => Some(Self::W10),
      b"http://schemas.microsoft.com/office/word/2010/wordml" => Some(Self::W14),
      b"http://schemas.microsoft.com/office/word/2012/wordml" => Some(Self::W15),
      b"http://schemas.microsoft.com/office/word/2018/wordml/cex" => Some(Self::W16cex),
      b"http://schemas.microsoft.com/office/word/2016/wordml/cid" => Some(Self::W16cid),
      b"http://schemas.microsoft.com/office/word/2018/wordml" => Some(Self::W16cur),
      b"http://schemas.microsoft.com/office/word/2023/wordml/word16du" => Some(Self::W16du),
      b"http://schemas.microsoft.com/office/word/2020/wordml/sdtdatahash" => Some(Self::W16sdtdh),
      b"http://schemas.microsoft.com/office/word/2024/wordml/sdtformatlock" => Some(Self::W16sdtfl),
      b"http://schemas.microsoft.com/office/word/2015/wordml/symex" => Some(Self::W16se),
      b"http://schemas.microsoft.com/office/webextensions/webextension/2010/11" => Some(Self::We),
      b"http://schemas.microsoft.com/office/webextensions/taskpanes/2010/11" => Some(Self::Wetp),
      b"http://schemas.microsoft.com/office/word/2006/wordml" => Some(Self::Wne),
      b"http://schemas.microsoft.com/office/word/2020/oembed" => Some(Self::Woe),
      b"http://schemas.openxmlformats.org/drawingml/2006/wordprocessingDrawing" => Some(Self::Wp),
      b"http://schemas.microsoft.com/office/word/2010/wordprocessingDrawing" => Some(Self::Wp14),
      b"http://schemas.microsoft.com/office/word/2012/wordprocessingDrawing" => Some(Self::Wp15),
      b"http://schemas.microsoft.com/office/word/2010/wordprocessingCanvas" => Some(Self::Wpc),
      b"http://schemas.microsoft.com/office/word/2010/wordprocessingGroup" => Some(Self::Wpg),
      b"http://schemas.microsoft.com/office/word/2010/wordprocessingInk" => Some(Self::Wpi),
      b"http://schemas.microsoft.com/office/word/2010/wordprocessingShape" => Some(Self::Wps),
      b"http://schemas.openxmlformats.org/spreadsheetml/2006/main" => Some(Self::X),
      b"http://schemas.microsoft.com/office/spreadsheetml/2011/1/ac" => Some(Self::X12ac),
      b"http://schemas.microsoft.com/office/spreadsheetml/2009/9/main" => Some(Self::X14),
      b"http://schemas.microsoft.com/office/spreadsheetml/2009/9/ac" => Some(Self::X14ac),
      b"http://schemas.microsoft.com/office/spreadsheetml/2010/11/main" => Some(Self::X15),
      b"http://schemas.microsoft.com/office/spreadsheetml/2010/11/ac" => Some(Self::X15ac),
      b"http://schemas.microsoft.com/office/spreadsheetml/2014/11/main" => Some(Self::X16),
      b"http://schemas.microsoft.com/office/spreadsheetml/2015/02/main" => Some(Self::X16r2),
      b"http://schemas.microsoft.com/office/spreadsheetml/2018/08/main" => Some(Self::X16r3),
      b"http://schemas.microsoft.com/office/spreadsheetml/2018/calcfeatures" => Some(Self::Xcalcf),
      b"http://schemas.microsoft.com/office/spreadsheetml/2017/dynamicarray" => Some(Self::Xda),
      b"http://schemas.openxmlformats.org/drawingml/2006/spreadsheetDrawing" => Some(Self::Xdr),
      b"http://schemas.microsoft.com/office/excel/2010/spreadsheetDrawing" => Some(Self::Xdr14),
      b"http://schemas.microsoft.com/office/spreadsheetml/2022/featurepropertybag" => {
        Some(Self::Xfpb)
      }
      b"http://schemas.microsoft.com/office/spreadsheetml/2016/01/main" => Some(Self::XlPr),
      b"http://schemas.microsoft.com/office/spreadsheetml/2023/externalCodeService" => {
        Some(Self::Xlecs)
      }
      b"http://schemas.microsoft.com/office/spreadsheetml/2025/externalCodeService2" => {
        Some(Self::Xlecs2)
      }
      b"http://schemas.microsoft.com/office/spreadsheetml/2023/msForms" => Some(Self::Xlmsforms),
      b"http://schemas.microsoft.com/office/spreadsheetml/2024/pivotAutoRefresh" => {
        Some(Self::Xlpar)
      }
      b"http://schemas.microsoft.com/office/spreadsheetml/2023/pivot2023Calculation" => {
        Some(Self::Xlpcalc)
      }
      b"http://schemas.microsoft.com/office/spreadsheetml/2024/pivotDynamicArrays" => {
        Some(Self::Xlpda)
      }
      b"http://schemas.microsoft.com/office/spreadsheetml/2025/pivotDataSource" => {
        Some(Self::Xlpds)
      }
      b"http://schemas.microsoft.com/office/spreadsheetml/2017/richdata" => Some(Self::Xlrd),
      b"http://schemas.microsoft.com/office/spreadsheetml/2017/richdata2" => Some(Self::Xlrd2),
      b"http://schemas.microsoft.com/office/spreadsheetml/2020/richdatawebimage" => {
        Some(Self::Xlrdwi)
      }
      b"http://schemas.microsoft.com/office/spreadsheetml/2020/richvaluerefresh" => {
        Some(Self::Xlrvr)
      }
      b"http://schemas.microsoft.com/office/spreadsheetml/2022/richvaluerel" => Some(Self::Xlrvrel),
      b"http://schemas.microsoft.com/office/spreadsheetml/2018/threadedcomments" => {
        Some(Self::Xltc)
      }
      b"http://schemas.microsoft.com/office/spreadsheetml/2020/threadedcomments2" => {
        Some(Self::Xltc2)
      }
      b"http://schemas.microsoft.com/office/spreadsheetml/2024/workbookCompatibilityVersion" => {
        Some(Self::Xlwcv)
      }
      b"http://www.w3.org/XML/1998/namespace" => Some(Self::Xml),
      b"http://schemas.microsoft.com/office/excel/2006/main" => Some(Self::Xne),
      b"http://schemas.microsoft.com/office/spreadsheetml/2019/namedsheetviews" => Some(Self::Xnsv),
      b"http://schemas.microsoft.com/office/spreadsheetml/2016/pivotdefaultlayout" => {
        Some(Self::Xpdl)
      }
      b"http://schemas.microsoft.com/office/spreadsheetml/2022/pivotRichData" => Some(Self::Xprd),
      b"http://schemas.microsoft.com/office/spreadsheetml/2014/revision" => Some(Self::Xr),
      b"http://schemas.microsoft.com/office/spreadsheetml/2016/revision10" => Some(Self::Xr10),
      b"http://schemas.microsoft.com/office/spreadsheetml/2017/revision16" => Some(Self::Xr16),
      b"http://schemas.microsoft.com/office/spreadsheetml/2015/revision2" => Some(Self::Xr2),
      b"http://schemas.microsoft.com/office/spreadsheetml/2016/revision3" => Some(Self::Xr3),
      b"http://schemas.microsoft.com/office/spreadsheetml/2016/revision5" => Some(Self::Xr5),
      b"http://schemas.microsoft.com/office/spreadsheetml/2016/revision6" => Some(Self::Xr6),
      b"http://schemas.microsoft.com/office/spreadsheetml/2016/revision9" => Some(Self::Xr9),
      b"http://www.w3.org/2001/XMLSchema" => Some(Self::Xsd),
      b"http://www.w3.org/2001/XMLSchema-instance" => Some(Self::Xsi),
      b"urn:schemas-microsoft-com:office:excel" => Some(Self::Xvml),
      b"http://schemas.microsoft.com/office/spreadsheetml/2023/dataSourceVersioning" => {
        Some(Self::Xxdsv)
      }
      b"http://schemas.microsoft.com/office/spreadsheetml/2021/extlinks2021" => Some(Self::Xxl21),
      b"http://schemas.microsoft.com/office/spreadsheetml/2019/extlinksprops" => Some(Self::Xxlnp),
      b"http://schemas.microsoft.com/office/spreadsheetml/2020/pivotNov2020" => Some(Self::Xxpim),
      b"http://schemas.microsoft.com/office/spreadsheetml/2022/pivotVersionInfo" => {
        Some(Self::Xxpvi)
      }
      b"http://purl.oclc.org/ooxml/drawingml/chart" => Some(Self::C),
      b"http://purl.oclc.org/ooxml/drawingml/chartDrawing" => Some(Self::Cdr),
      b"http://purl.oclc.org/ooxml/drawingml/compatibility" => Some(Self::Comp),
      b"http://purl.oclc.org/ooxml/drawingml/diagram" => Some(Self::Dgm),
      b"http://purl.oclc.org/ooxml/drawingml/lockedCanvas" => Some(Self::Lc),
      b"http://purl.oclc.org/ooxml/drawingml/main" => Some(Self::A),
      b"http://purl.oclc.org/ooxml/drawingml/picture" => Some(Self::Pic),
      b"http://purl.oclc.org/ooxml/drawingml/spreadsheetDrawing" => Some(Self::Xdr),
      b"http://purl.oclc.org/ooxml/drawingml/wordprocessingDrawing" => Some(Self::Wp),
      b"http://purl.oclc.org/ooxml/officeDocument/bibliography" => Some(Self::B),
      b"http://purl.oclc.org/ooxml/officeDocument/customProperties" => Some(Self::Op),
      b"http://purl.oclc.org/ooxml/officeDocument/customXml" => Some(Self::Ds),
      b"http://purl.oclc.org/ooxml/officeDocument/docPropsVTypes" => Some(Self::Vt),
      b"http://purl.oclc.org/ooxml/officeDocument/extendedProperties" => Some(Self::Ap),
      b"http://purl.oclc.org/ooxml/officeDocument/math" => Some(Self::M),
      b"http://purl.oclc.org/ooxml/officeDocument/relationships" => Some(Self::R),
      b"http://purl.oclc.org/ooxml/presentationml/main" => Some(Self::P),
      b"http://purl.oclc.org/ooxml/schemaLibrary/main" => Some(Self::Sl),
      b"http://purl.oclc.org/ooxml/spreadsheetml/main" => Some(Self::X),
      b"http://purl.oclc.org/ooxml/wordprocessingml/main" => Some(Self::W),
      _ => None,
    }
  }
}
