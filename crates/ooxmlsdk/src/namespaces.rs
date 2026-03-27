//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub(crate) fn prefix_by_uri(uri: &str) -> Option<&'static str> {
    match uri {
        "http://schemas.openxmlformats.org/drawingml/2006/main" => Some("a"),
        "http://schemas.microsoft.com/office/drawing/2010/main" => Some("a14"),
        "http://schemas.microsoft.com/office/drawing/2012/main" => Some("a15"),
        "http://schemas.microsoft.com/office/drawing/2014/main" => Some("a16"),
        "http://schemas.microsoft.com/office/drawing/2016/11/main" => Some("a1611"),
        "http://schemas.microsoft.com/office/drawing/2018/animation/model3d" => {
            Some("a3danim")
        }
        "http://schemas.microsoft.com/office/drawing/2018/animation" => Some("aanim"),
        "http://schemas.openxmlformats.org/officeDocument/2006/characteristics" => {
            Some("ac")
        }
        "http://schemas.microsoft.com/office/drawing/2020/classificationShape" => {
            Some("aclsh")
        }
        "http://schemas.microsoft.com/office/drawing/2017/decorative" => Some("adec"),
        "http://schemas.microsoft.com/office/drawing/2018/hyperlinkcolor" => Some("ahyp"),
        "http://schemas.microsoft.com/office/drawing/2022/imageformula" => Some("aif"),
        "http://schemas.microsoft.com/office/drawing/2016/ink" => Some("aink"),
        "http://schemas.microsoft.com/office/drawing/2021/livefeed" => Some("alf"),
        "http://schemas.microsoft.com/office/drawing/2017/model3d" => Some("am3d"),
        "http://schemas.microsoft.com/office/drawing/2021/oembed" => Some("aoe"),
        "http://schemas.openxmlformats.org/officeDocument/2006/extended-properties" => {
            Some("ap")
        }
        "http://schemas.microsoft.com/office/drawing/2018/sketchyshapes" => Some("ask"),
        "http://schemas.microsoft.com/office/drawing/2021/scriptlink" => Some("asl"),
        "http://schemas.microsoft.com/office/drawing/2016/SVG/main" => Some("asvg"),
        "http://schemas.microsoft.com/office/2006/activeX" => Some("ax"),
        "http://schemas.openxmlformats.org/officeDocument/2006/bibliography" => Some("b"),
        "http://schemas.openxmlformats.org/drawingml/2006/chart" => Some("c"),
        "http://schemas.microsoft.com/office/drawing/2007/8/2/chart" => Some("c14"),
        "http://schemas.microsoft.com/office/drawing/2012/chart" => Some("c15"),
        "http://schemas.microsoft.com/office/drawing/2014/chart" => Some("c16"),
        "http://schemas.microsoft.com/office/drawing/2014/chart/ac" => Some("c16ac"),
        "http://schemas.microsoft.com/office/drawing/2017/03/chart" => Some("c16r3"),
        "http://schemas.microsoft.com/office/2006/customDocumentInformationPanel" => {
            Some("cdip")
        }
        "http://schemas.openxmlformats.org/drawingml/2006/chartDrawing" => Some("cdr"),
        "http://schemas.microsoft.com/office/drawing/2010/chartDrawing" => Some("cdr14"),
        "http://schemas.microsoft.com/office/2020/mipLabelMetadata" => Some("clbl"),
        "http://schemas.microsoft.com/office/drawing/2010/compatibility" => Some("com14"),
        "http://schemas.openxmlformats.org/drawingml/2006/compatibility" => Some("comp"),
        "http://schemas.openxmlformats.org/package/2006/metadata/core-properties" => {
            Some("cp")
        }
        "http://schemas.microsoft.com/office/2006/coverPageProps" => Some("cppr"),
        "http://schemas.microsoft.com/office/comments/2020/reactions" => Some("cr"),
        "http://schemas.microsoft.com/office/drawing/2012/chartStyle" => Some("cs"),
        "http://schemas.microsoft.com/office/2006/metadata/contentType" => Some("ct"),
        "http://schemas.microsoft.com/office/drawing/2014/chartex" => Some("cx"),
        "http://purl.org/dc/elements/1.1/" => Some("dc"),
        "http://purl.org/dc/terms/" => Some("dcterms"),
        "http://schemas.openxmlformats.org/drawingml/2006/diagram" => Some("dgm"),
        "http://schemas.microsoft.com/office/drawing/2010/diagram" => Some("dgm14"),
        "http://schemas.microsoft.com/office/drawing/2016/11/diagram" => Some("dgm1611"),
        "http://schemas.microsoft.com/office/drawing/2016/12/diagram" => Some("dgm1612"),
        "http://schemas.openxmlformats.org/officeDocument/2006/customXml" => Some("ds"),
        "http://schemas.microsoft.com/office/drawing/2008/diagram" => Some("dsp"),
        "http://www.w3.org/2003/04/emma" => Some("emma"),
        "http://www.w3.org/2003/InkML" => Some("inkml"),
        "http://schemas.openxmlformats.org/drawingml/2006/lockedCanvas" => Some("lc"),
        "http://schemas.microsoft.com/office/2006/metadata/longProperties" => Some("lp"),
        "http://schemas.openxmlformats.org/officeDocument/2006/math" => Some("m"),
        "http://schemas.microsoft.com/office/2006/metadata/properties/metaAttributes" => {
            Some("ma")
        }
        "http://schemas.openxmlformats.org/markup-compatibility/2006" => Some("mc"),
        "http://schemas.microsoft.com/office/office/2011/9/metroDictionary" => Some("md"),
        "http://schemas.microsoft.com/ink/2010/main" => Some("msink"),
        "http://schemas.microsoft.com/office/2006/01/customui" => Some("mso"),
        "http://schemas.microsoft.com/office/2009/07/customui" => Some("mso14"),
        "http://schemas.microsoft.com/office/2006/metadata/customXsn" => Some("ntns"),
        "urn:schemas-microsoft-com:office:office" => Some("o"),
        "http://schemas.microsoft.com/office/drawing/2013/main/command" => Some("oac"),
        "http://schemas.microsoft.com/office/2019/extlst" => Some("oel"),
        "http://schemas.openxmlformats.org/officeDocument/2006/custom-properties" => {
            Some("op")
        }
        "http://schemas.openxmlformats.org/presentationml/2006/main" => Some("p"),
        "http://schemas.microsoft.com/office/powerpoint/2010/main" => Some("p14"),
        "http://schemas.microsoft.com/office/powerpoint/2012/main" => Some("p15"),
        "http://schemas.microsoft.com/office/powerpoint/2015/main" => Some("p16"),
        "http://schemas.microsoft.com/office/powerpoint/2017/10/main" => Some("p1710"),
        "http://schemas.microsoft.com/office/powerpoint/2017/3/main" => Some("p173"),
        "http://schemas.microsoft.com/office/powerpoint/2018/4/main" => Some("p184"),
        "http://schemas.microsoft.com/office/powerpoint/2018/8/main" => Some("p188"),
        "http://schemas.microsoft.com/office/powerpoint/2019/12/main" => Some("p1912"),
        "http://schemas.microsoft.com/office/powerpoint/2020/02/main" => Some("p202"),
        "http://schemas.microsoft.com/office/powerpoint/2021/06/main" => Some("p216"),
        "http://schemas.microsoft.com/office/powerpoint/2022/03/main" => Some("p223"),
        "http://schemas.microsoft.com/office/powerpoint/2022/08/main" => Some("p228"),
        "http://schemas.microsoft.com/office/powerpoint/2023/02/main" => Some("p232"),
        "http://schemas.microsoft.com/office/powerpoint/2012/roamingSettings" => {
            Some("pRoam")
        }
        "http://schemas.microsoft.com/office/internal/2007/ofapi/packaging" => {
            Some("packaging")
        }
        "http://schemas.microsoft.com/office/2007/6/19/audiovideo" => Some("pav"),
        "http://schemas.microsoft.com/office/powerpoint/2013/main/command" => Some("pc"),
        "http://schemas.microsoft.com/office/powerpoint/2019/9/main/command" => {
            Some("pc2")
        }
        "http://schemas.microsoft.com/office/powerpoint/2022/06/main/command" => {
            Some("pc226")
        }
        "http://schemas.openxmlformats.org/drawingml/2006/picture" => Some("pic"),
        "http://schemas.microsoft.com/office/drawing/2010/picture" => Some("pic14"),
        "http://schemas.microsoft.com/projectml/2012/main" => Some("pj15"),
        "urn:schemas-microsoft-com:office:powerpoint" => Some("pvml"),
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships" => {
            Some("r")
        }
        "http://schemas.openxmlformats.org/schemaLibrary/2006/main" => Some("sl"),
        "http://schemas.microsoft.com/office/drawing/2010/slicer" => Some("sle"),
        "http://schemas.microsoft.com/office/tasks/2019/documenttasks" => Some("t"),
        "http://schemas.microsoft.com/office/thememl/2012/main" => Some("thm15"),
        "http://schemas.microsoft.com/office/drawing/2012/timeslicer" => Some("tsle"),
        "urn:schemas-microsoft-com:vml" => Some("v"),
        "http://schemas.openxmlformats.org/officeDocument/2006/docPropsVTypes" => {
            Some("vt")
        }
        "http://schemas.openxmlformats.org/wordprocessingml/2006/main" => Some("w"),
        "urn:schemas-microsoft-com:office:word" => Some("w10"),
        "http://schemas.microsoft.com/office/word/2010/wordml" => Some("w14"),
        "http://schemas.microsoft.com/office/word/2012/wordml" => Some("w15"),
        "http://schemas.microsoft.com/office/word/2018/wordml/cex" => Some("w16cex"),
        "http://schemas.microsoft.com/office/word/2016/wordml/cid" => Some("w16cid"),
        "http://schemas.microsoft.com/office/word/2018/wordml" => Some("w16cur"),
        "http://schemas.microsoft.com/office/word/2023/wordml/word16du" => Some("w16du"),
        "http://schemas.microsoft.com/office/word/2020/wordml/sdtdatahash" => {
            Some("w16sdtdh")
        }
        "http://schemas.microsoft.com/office/word/2024/wordml/sdtformatlock" => {
            Some("w16sdtfl")
        }
        "http://schemas.microsoft.com/office/word/2015/wordml/symex" => Some("w16se"),
        "http://schemas.microsoft.com/office/webextensions/webextension/2010/11" => {
            Some("we")
        }
        "http://schemas.microsoft.com/office/webextensions/taskpanes/2010/11" => {
            Some("wetp")
        }
        "http://schemas.microsoft.com/office/word/2006/wordml" => Some("wne"),
        "http://schemas.microsoft.com/office/word/2020/oembed" => Some("woe"),
        "http://schemas.openxmlformats.org/drawingml/2006/wordprocessingDrawing" => {
            Some("wp")
        }
        "http://schemas.microsoft.com/office/word/2010/wordprocessingDrawing" => {
            Some("wp14")
        }
        "http://schemas.microsoft.com/office/word/2012/wordprocessingDrawing" => {
            Some("wp15")
        }
        "http://schemas.microsoft.com/office/word/2010/wordprocessingCanvas" => {
            Some("wpc")
        }
        "http://schemas.microsoft.com/office/word/2010/wordprocessingGroup" => {
            Some("wpg")
        }
        "http://schemas.microsoft.com/office/word/2010/wordprocessingShape" => {
            Some("wps")
        }
        "http://schemas.openxmlformats.org/spreadsheetml/2006/main" => Some("x"),
        "http://schemas.microsoft.com/office/spreadsheetml/2011/1/ac" => Some("x12ac"),
        "http://schemas.microsoft.com/office/spreadsheetml/2009/9/main" => Some("x14"),
        "http://schemas.microsoft.com/office/spreadsheetml/2009/9/ac" => Some("x14ac"),
        "http://schemas.microsoft.com/office/spreadsheetml/2010/11/main" => Some("x15"),
        "http://schemas.microsoft.com/office/spreadsheetml/2010/11/ac" => Some("x15ac"),
        "http://schemas.microsoft.com/office/spreadsheetml/2014/11/main" => Some("x16"),
        "http://schemas.microsoft.com/office/spreadsheetml/2015/02/main" => Some("x16r2"),
        "http://schemas.microsoft.com/office/spreadsheetml/2018/08/main" => Some("x16r3"),
        "http://schemas.microsoft.com/office/spreadsheetml/2018/calcfeatures" => {
            Some("xcalcf")
        }
        "http://schemas.microsoft.com/office/spreadsheetml/2017/dynamicarray" => {
            Some("xda")
        }
        "http://schemas.openxmlformats.org/drawingml/2006/spreadsheetDrawing" => {
            Some("xdr")
        }
        "http://schemas.microsoft.com/office/excel/2010/spreadsheetDrawing" => {
            Some("xdr14")
        }
        "http://schemas.microsoft.com/office/spreadsheetml/2022/featurepropertybag" => {
            Some("xfpb")
        }
        "http://schemas.microsoft.com/office/spreadsheetml/2016/01/main" => Some("xlPr"),
        "http://schemas.microsoft.com/office/spreadsheetml/2023/externalCodeService" => {
            Some("xlecs")
        }
        "http://schemas.microsoft.com/office/spreadsheetml/2025/externalCodeService2" => {
            Some("xlecs2")
        }
        "http://schemas.microsoft.com/office/spreadsheetml/2023/msForms" => {
            Some("xlmsforms")
        }
        "http://schemas.microsoft.com/office/spreadsheetml/2024/pivotAutoRefresh" => {
            Some("xlpar")
        }
        "http://schemas.microsoft.com/office/spreadsheetml/2023/pivot2023Calculation" => {
            Some("xlpcalc")
        }
        "http://schemas.microsoft.com/office/spreadsheetml/2024/pivotDynamicArrays" => {
            Some("xlpda")
        }
        "http://schemas.microsoft.com/office/spreadsheetml/2025/pivotDataSource" => {
            Some("xlpds")
        }
        "http://schemas.microsoft.com/office/spreadsheetml/2017/richdata" => Some("xlrd"),
        "http://schemas.microsoft.com/office/spreadsheetml/2017/richdata2" => {
            Some("xlrd2")
        }
        "http://schemas.microsoft.com/office/spreadsheetml/2020/richdatawebimage" => {
            Some("xlrdwi")
        }
        "http://schemas.microsoft.com/office/spreadsheetml/2020/richvaluerefresh" => {
            Some("xlrvr")
        }
        "http://schemas.microsoft.com/office/spreadsheetml/2022/richvaluerel" => {
            Some("xlrvrel")
        }
        "http://schemas.microsoft.com/office/spreadsheetml/2018/threadedcomments" => {
            Some("xltc")
        }
        "http://schemas.microsoft.com/office/spreadsheetml/2020/threadedcomments2" => {
            Some("xltc2")
        }
        "http://schemas.microsoft.com/office/spreadsheetml/2024/workbookCompatibilityVersion" => {
            Some("xlwcv")
        }
        "http://www.w3.org/XML/1998/namespace" => Some("xml"),
        "http://schemas.microsoft.com/office/excel/2006/main" => Some("xne"),
        "http://schemas.microsoft.com/office/spreadsheetml/2019/namedsheetviews" => {
            Some("xnsv")
        }
        "http://schemas.microsoft.com/office/spreadsheetml/2016/pivotdefaultlayout" => {
            Some("xpdl")
        }
        "http://schemas.microsoft.com/office/spreadsheetml/2022/pivotRichData" => {
            Some("xprd")
        }
        "http://schemas.microsoft.com/office/spreadsheetml/2014/revision" => Some("xr"),
        "http://schemas.microsoft.com/office/spreadsheetml/2016/revision10" => {
            Some("xr10")
        }
        "http://schemas.microsoft.com/office/spreadsheetml/2017/revision16" => {
            Some("xr16")
        }
        "http://schemas.microsoft.com/office/spreadsheetml/2015/revision2" => Some("xr2"),
        "http://schemas.microsoft.com/office/spreadsheetml/2016/revision3" => Some("xr3"),
        "http://schemas.microsoft.com/office/spreadsheetml/2016/revision5" => Some("xr5"),
        "http://schemas.microsoft.com/office/spreadsheetml/2016/revision6" => Some("xr6"),
        "http://schemas.microsoft.com/office/spreadsheetml/2016/revision9" => Some("xr9"),
        "http://www.w3.org/2001/XMLSchema" => Some("xsd"),
        "urn:schemas-microsoft-com:office:excel" => Some("xvml"),
        "http://schemas.microsoft.com/office/spreadsheetml/2023/dataSourceVersioning" => {
            Some("xxdsv")
        }
        "http://schemas.microsoft.com/office/spreadsheetml/2021/extlinks2021" => {
            Some("xxl21")
        }
        "http://schemas.microsoft.com/office/spreadsheetml/2019/extlinksprops" => {
            Some("xxlnp")
        }
        "http://schemas.microsoft.com/office/spreadsheetml/2020/pivotNov2020" => {
            Some("xxpim")
        }
        "http://schemas.microsoft.com/office/spreadsheetml/2022/pivotVersionInfo" => {
            Some("xxpvi")
        }
        _ => None,
    }
}
