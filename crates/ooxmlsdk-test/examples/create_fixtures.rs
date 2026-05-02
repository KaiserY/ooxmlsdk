use std::io::Write as _;
use std::path::Path;

use base64::Engine as _;

// 1×1 red pixel PNG (deterministic binary content)
const PNG_BASE64: &str = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8BQDwADhQGAWjR9awAAAABJRU5ErkJggg==";

fn workspace_root() -> std::path::PathBuf {
  Path::new(env!("CARGO_MANIFEST_DIR"))
    .parent()
    .expect("parent of ooxmlsdk-test crates dir")
    .parent()
    .expect("workspace root")
    .to_path_buf()
}

fn file_options() -> zip::write::SimpleFileOptions {
  // MS-DOS epoch — deterministic output across runs
  zip::write::SimpleFileOptions::default()
    .compression_method(zip::CompressionMethod::Stored)
    .last_modified_time(zip::DateTime::default())
}

fn make_package(parts: &[(&str, &[u8])]) -> Vec<u8> {
  let mut buf = std::io::Cursor::new(Vec::new());
  {
    let mut zip = zip::ZipWriter::new(&mut buf);
    for (name, data) in parts {
      zip
        .start_file(*name, file_options())
        .expect("start zip entry");
      zip.write_all(data).expect("write zip entry data");
    }
    zip.finish().expect("finish zip");
  }
  buf.into_inner()
}

fn save(root: &Path, rel_path: &str, data: &[u8]) {
  let full = root.join(rel_path);
  if let Some(parent) = full.parent() {
    std::fs::create_dir_all(parent).expect("create fixture dir");
  }
  std::fs::write(&full, data).expect("write fixture file");
  println!("Created {rel_path}");
}

// ── shared XML fragments ─────────────────────────────────────────────────────

const RELS_XMLNS: &str = "http://schemas.openxmlformats.org/package/2006/relationships";

fn root_rels(office_doc_target: &str) -> Vec<u8> {
  format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="{RELS_XMLNS}">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="{office_doc_target}"/>
</Relationships>"#
    )
    .into_bytes()
}

fn empty_rels() -> &'static [u8] {
  br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"/>"#
}

// ── DOCX helpers ─────────────────────────────────────────────────────────────

fn docx_content_types(extra_overrides: &str, extra_defaults: &str) -> Vec<u8> {
  format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
  <Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
  <Default Extension="xml" ContentType="application/xml"/>{extra_defaults}
  <Override PartName="/word/document.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"/>{extra_overrides}
</Types>"#
    )
    .into_bytes()
}

fn docx_doc_rels(extra: &str) -> Vec<u8> {
  if extra.is_empty() {
    return empty_rels().to_vec();
  }
  format!(
    r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="{RELS_XMLNS}">{extra}
</Relationships>"#
  )
  .into_bytes()
}

// ── XLSX helpers ─────────────────────────────────────────────────────────────

fn xlsx_content_types(sheet_count: usize, extra_overrides: &str) -> Vec<u8> {
  let mut sheet_overrides = String::new();
  for i in 1..=sheet_count {
    sheet_overrides.push_str(&format!(
            "\n  <Override PartName=\"/xl/worksheets/sheet{i}.xml\" ContentType=\"application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml\"/>"
        ));
  }
  format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
  <Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
  <Default Extension="xml" ContentType="application/xml"/>
  <Override PartName="/xl/workbook.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml"/>{sheet_overrides}{extra_overrides}
</Types>"#
    )
    .into_bytes()
}

fn workbook_xml(sheets: &[(&str, u32, &str)]) -> Vec<u8> {
  let mut sheet_nodes = String::new();
  for (name, id, rid) in sheets {
    sheet_nodes.push_str(&format!(
      "\n    <x:sheet name=\"{name}\" sheetId=\"{id}\" r:id=\"{rid}\"/>"
    ));
  }
  format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<x:workbook xmlns:x="http://schemas.openxmlformats.org/spreadsheetml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
  <x:sheets>{sheet_nodes}
  </x:sheets>
</x:workbook>"#
    )
    .into_bytes()
}

fn workbook_rels(extra: &str) -> Vec<u8> {
  format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="{RELS_XMLNS}">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/sheet1.xml"/>{extra}
</Relationships>"#
    )
    .into_bytes()
}

fn empty_worksheet() -> &'static [u8] {
  br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<x:worksheet xmlns:x="http://schemas.openxmlformats.org/spreadsheetml/2006/main">
  <x:sheetData/>
</x:worksheet>"#
}

// ── PPTX helpers ─────────────────────────────────────────────────────────────

fn pptx_content_types(slide_count: usize, extra_overrides: &str) -> Vec<u8> {
  let mut slide_overrides = String::new();
  for i in 1..=slide_count {
    slide_overrides.push_str(&format!(
            "\n  <Override PartName=\"/ppt/slides/slide{i}.xml\" ContentType=\"application/vnd.openxmlformats-officedocument.presentationml.slide+xml\"/>"
        ));
  }
  format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
  <Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
  <Default Extension="xml" ContentType="application/xml"/>
  <Override PartName="/ppt/presentation.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml"/>
  <Override PartName="/ppt/slideMasters/slideMaster1.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slideMaster+xml"/>
  <Override PartName="/ppt/slideLayouts/slideLayout1.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slideLayout+xml"/>{slide_overrides}{extra_overrides}
</Types>"#
    )
    .into_bytes()
}

fn presentation_xml(slide_count: usize) -> Vec<u8> {
  let mut sld_ids = String::new();
  for i in 0..slide_count {
    let id = 256 + i as u32;
    let rid_n = i + 2; // rId1 = slideMaster, rId2+ = slides
    sld_ids.push_str(&format!("\n    <p:sldId id=\"{id}\" r:id=\"rId{rid_n}\"/>"));
  }
  format!(
    r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:presentation xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
                xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
                xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
  <p:sldMasterIdLst>
    <p:sldMasterId id="2147483648" r:id="rId1"/>
  </p:sldMasterIdLst>
  <p:sldIdLst>{sld_ids}
  </p:sldIdLst>
  <p:sldSz cx="9144000" cy="6858000"/>
  <p:notesSz cx="6858000" cy="9144000"/>
</p:presentation>"#
  )
  .into_bytes()
}

fn presentation_rels(slide_count: usize) -> Vec<u8> {
  let mut slide_rels = String::new();
  for i in 1..=slide_count {
    let rid_n = i + 1;
    slide_rels.push_str(&format!(
            "\n  <Relationship Id=\"rId{rid_n}\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide\" Target=\"slides/slide{i}.xml\"/>"
        ));
  }
  format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="{RELS_XMLNS}">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideMaster" Target="slideMasters/slideMaster1.xml"/>{slide_rels}
</Relationships>"#
    )
    .into_bytes()
}

const SLIDE_MASTER_XML: &[u8] = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sldMaster xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
             xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
             xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
  <p:cSld>
    <p:spTree>
      <p:nvGrpSpPr>
        <p:cNvPr id="1" name=""/>
        <p:cNvGrpSpPr/>
        <p:nvPr/>
      </p:nvGrpSpPr>
      <p:grpSpPr>
        <a:xfrm>
          <a:off x="0" y="0"/>
          <a:ext cx="0" cy="0"/>
          <a:chOff x="0" y="0"/>
          <a:chExt cx="0" cy="0"/>
        </a:xfrm>
      </p:grpSpPr>
    </p:spTree>
  </p:cSld>
  <p:clrMap bg1="lt1" tx1="dk1" bg2="lt2" tx2="dk2" accent1="accent1" accent2="accent2" accent3="accent3" accent4="accent4" accent5="accent5" accent6="accent6" hlink="hlink" folHlink="folHlink"/>
  <p:sldLayoutIdLst>
    <p:sldLayoutId id="2147483649" r:id="rId1"/>
  </p:sldLayoutIdLst>
  <p:txStyles>
    <p:titleStyle>
      <a:lvl1pPr>
        <a:defRPr lang="en-US"/>
      </a:lvl1pPr>
    </p:titleStyle>
    <p:bodyStyle>
      <a:lvl1pPr>
        <a:defRPr lang="en-US"/>
      </a:lvl1pPr>
    </p:bodyStyle>
    <p:otherStyle>
      <a:defPPr>
        <a:defRPr lang="en-US"/>
      </a:defPPr>
    </p:otherStyle>
  </p:txStyles>
</p:sldMaster>"#;

fn slide_master_rels(extra: &str) -> Vec<u8> {
  format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="{RELS_XMLNS}">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideLayout" Target="../slideLayouts/slideLayout1.xml"/>{extra}
</Relationships>"#
    )
    .into_bytes()
}

fn blank_slide_layout() -> &'static [u8] {
  br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sldLayout xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
             xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
             xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
             type="blank">
  <p:cSld name="Blank Slide">
    <p:spTree>
      <p:nvGrpSpPr>
        <p:cNvPr id="1" name=""/>
        <p:cNvGrpSpPr/>
        <p:nvPr/>
      </p:nvGrpSpPr>
      <p:grpSpPr>
        <a:xfrm>
          <a:off x="0" y="0"/>
          <a:ext cx="0" cy="0"/>
          <a:chOff x="0" y="0"/>
          <a:chExt cx="0" cy="0"/>
        </a:xfrm>
      </p:grpSpPr>
    </p:spTree>
  </p:cSld>
  <p:clrMapOvr>
    <a:masterClrMapping/>
  </p:clrMapOvr>
</p:sldLayout>"#
}

fn slide_layout_back_rels() -> Vec<u8> {
  format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="{RELS_XMLNS}">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideMaster" Target="../slideMasters/slideMaster1.xml"/>
</Relationships>"#
    )
    .into_bytes()
}

fn blank_slide() -> &'static [u8] {
  br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sld xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
       xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
       xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
  <p:cSld>
    <p:spTree>
      <p:nvGrpSpPr>
        <p:cNvPr id="1" name=""/>
        <p:cNvGrpSpPr/>
        <p:nvPr/>
      </p:nvGrpSpPr>
      <p:grpSpPr>
        <a:xfrm>
          <a:off x="0" y="0"/>
          <a:ext cx="0" cy="0"/>
          <a:chOff x="0" y="0"/>
          <a:chExt cx="0" cy="0"/>
        </a:xfrm>
      </p:grpSpPr>
    </p:spTree>
  </p:cSld>
  <p:clrMapOvr>
    <a:masterClrMapping/>
  </p:clrMapOvr>
</p:sld>"#
}

fn slide_to_layout_rels(layout_target: &str, extra: &str) -> Vec<u8> {
  format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="{RELS_XMLNS}">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideLayout" Target="{layout_target}"/>{extra}
</Relationships>"#
    )
    .into_bytes()
}

// ── Standard PPTX package builder ────────────────────────────────────────────

struct PptxParts {
  content_types: Vec<u8>,
  pres_xml: Vec<u8>,
  pres_rels: Vec<u8>,
  master_xml: &'static [u8],
  master_rels: Vec<u8>,
  layout_xml: &'static [u8],
  layout_rels: Vec<u8>,
  slide_xml: Vec<u8>,
  slide_rels: Vec<u8>,
  extra: Vec<(&'static str, Vec<u8>)>,
}

fn make_pptx(p: PptxParts) -> Vec<u8> {
  let root_r = root_rels("ppt/presentation.xml");
  let mut parts: Vec<(&str, &[u8])> = vec![
    ("[Content_Types].xml", &p.content_types),
    ("_rels/.rels", &root_r),
    ("ppt/presentation.xml", &p.pres_xml),
    ("ppt/_rels/presentation.xml.rels", &p.pres_rels),
    ("ppt/slideMasters/slideMaster1.xml", p.master_xml),
    (
      "ppt/slideMasters/_rels/slideMaster1.xml.rels",
      &p.master_rels,
    ),
    ("ppt/slideLayouts/slideLayout1.xml", p.layout_xml),
    (
      "ppt/slideLayouts/_rels/slideLayout1.xml.rels",
      &p.layout_rels,
    ),
    ("ppt/slides/slide1.xml", &p.slide_xml),
    ("ppt/slides/_rels/slide1.xml.rels", &p.slide_rels),
  ];
  for (k, v) in &p.extra {
    parts.push((k, v.as_slice()));
  }
  make_package(&parts)
}

// ── Title-slide layout for minimal_layout ────────────────────────────────────

const TITLE_SLIDE_LAYOUT_XML: &[u8] = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sldLayout xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
             xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
             xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
             type="title">
  <p:cSld name="Title Slide">
    <p:spTree>
      <p:nvGrpSpPr>
        <p:cNvPr id="1" name=""/>
        <p:cNvGrpSpPr/>
        <p:nvPr/>
      </p:nvGrpSpPr>
      <p:grpSpPr>
        <a:xfrm>
          <a:off x="0" y="0"/>
          <a:ext cx="0" cy="0"/>
          <a:chOff x="0" y="0"/>
          <a:chExt cx="0" cy="0"/>
        </a:xfrm>
      </p:grpSpPr>
      <p:sp>
        <p:nvSpPr>
          <p:cNvPr id="2" name="Title 1"/>
          <p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
          <p:nvPr><p:ph type="ctrTitle"/></p:nvPr>
        </p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="457200" y="1600200"/><a:ext cx="8229600" cy="1143000"/></a:xfrm>
          <a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
        </p:spPr>
        <p:txBody>
          <a:bodyPr/>
          <a:lstStyle/>
          <a:p/>
        </p:txBody>
      </p:sp>
      <p:sp>
        <p:nvSpPr>
          <p:cNvPr id="3" name="Subtitle 2"/>
          <p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
          <p:nvPr><p:ph type="subTitle" idx="1"/></p:nvPr>
        </p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="457200" y="3170600"/><a:ext cx="8229600" cy="1143000"/></a:xfrm>
          <a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
        </p:spPr>
        <p:txBody>
          <a:bodyPr/>
          <a:lstStyle/>
          <a:p/>
        </p:txBody>
      </p:sp>
    </p:spTree>
  </p:cSld>
  <p:clrMapOvr>
    <a:masterClrMapping/>
  </p:clrMapOvr>
</p:sldLayout>"#;

// ── Fixture generators ───────────────────────────────────────────────────────

fn create_docx_fixtures(root: &Path, png: &[u8]) {
  // ── minimal_empty ────────────────────────────────────────────────────────
  {
    let doc = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:body><w:sectPr/></w:body>
</w:document>"#;
    let data = make_package(&[
      ("[Content_Types].xml", &docx_content_types("", "")),
      ("_rels/.rels", &root_rels("word/document.xml")),
      ("word/document.xml", doc),
      ("word/_rels/document.xml.rels", empty_rels()),
    ]);
    save(root, "test-data/document/minimal_empty.docx", &data);
  }

  // ── minimal_text ─────────────────────────────────────────────────────────
  {
    let doc = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:body>
    <w:p><w:r><w:t>Hello ooxmlsdk</w:t></w:r></w:p>
    <w:sectPr/>
  </w:body>
</w:document>"#;
    let data = make_package(&[
      ("[Content_Types].xml", &docx_content_types("", "")),
      ("_rels/.rels", &root_rels("word/document.xml")),
      ("word/document.xml", doc),
      ("word/_rels/document.xml.rels", empty_rels()),
    ]);
    save(root, "test-data/document/minimal_text.docx", &data);
  }

  // ── minimal_styles ───────────────────────────────────────────────────────
  {
    let doc = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:body>
    <w:p>
      <w:pPr><w:pStyle w:val="Heading1"/></w:pPr>
      <w:r><w:t>Heading Text</w:t></w:r>
    </w:p>
    <w:sectPr/>
  </w:body>
</w:document>"#;
    let styles = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:styles xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:style w:type="paragraph" w:default="1" w:styleId="Normal">
    <w:name w:val="Normal"/>
  </w:style>
  <w:style w:type="paragraph" w:styleId="Heading1">
    <w:name w:val="heading 1"/>
    <w:basedOn w:val="Normal"/>
    <w:pPr><w:outlineLvl w:val="0"/></w:pPr>
    <w:rPr><w:b/><w:sz w:val="32"/></w:rPr>
  </w:style>
</w:styles>"#;
    let doc_rels = docx_doc_rels(
      r#"
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles" Target="styles.xml"/>"#,
    );
    let data = make_package(&[
      (
        "[Content_Types].xml",
        &docx_content_types(
          r#"
  <Override PartName="/word/styles.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml"/>"#,
          "",
        ),
      ),
      ("_rels/.rels", &root_rels("word/document.xml")),
      ("word/document.xml", doc),
      ("word/_rels/document.xml.rels", &doc_rels),
      ("word/styles.xml", styles),
    ]);
    save(root, "test-data/document/minimal_styles.docx", &data);
  }

  // ── minimal_table ────────────────────────────────────────────────────────
  {
    let doc = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:body>
    <w:tbl>
      <w:tblPr><w:tblW w:w="0" w:type="auto"/></w:tblPr>
      <w:tblGrid>
        <w:gridCol w:w="4675"/>
        <w:gridCol w:w="4675"/>
      </w:tblGrid>
      <w:tr>
        <w:tc><w:p><w:r><w:t>A1</w:t></w:r></w:p></w:tc>
        <w:tc><w:p><w:r><w:t>A2</w:t></w:r></w:p></w:tc>
      </w:tr>
      <w:tr>
        <w:tc><w:p><w:r><w:t>B1</w:t></w:r></w:p></w:tc>
        <w:tc><w:p><w:r><w:t>B2</w:t></w:r></w:p></w:tc>
      </w:tr>
    </w:tbl>
    <w:sectPr/>
  </w:body>
</w:document>"#;
    let data = make_package(&[
      ("[Content_Types].xml", &docx_content_types("", "")),
      ("_rels/.rels", &root_rels("word/document.xml")),
      ("word/document.xml", doc),
      ("word/_rels/document.xml.rels", empty_rels()),
    ]);
    save(root, "test-data/document/minimal_table.docx", &data);
  }

  // ── minimal_image ────────────────────────────────────────────────────────
  {
    let doc = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"
            xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
            xmlns:wp="http://schemas.openxmlformats.org/drawingml/2006/wordprocessingDrawing"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
            xmlns:pic="http://schemas.openxmlformats.org/drawingml/2006/picture">
  <w:body>
    <w:p>
      <w:r>
        <w:drawing>
          <wp:inline distT="0" distB="0" distL="0" distR="0">
            <wp:extent cx="9525" cy="9525"/>
            <wp:effectExtent l="0" t="0" r="0" b="0"/>
            <wp:docPr id="1" name="Image 1"/>
            <wp:cNvGraphicFramePr>
              <a:graphicFrameLocks noChangeAspect="1"/>
            </wp:cNvGraphicFramePr>
            <a:graphic>
              <a:graphicData uri="http://schemas.openxmlformats.org/drawingml/2006/picture">
                <pic:pic>
                  <pic:nvPicPr>
                    <pic:cNvPr id="0" name="image1.png"/>
                    <pic:cNvPicPr/>
                  </pic:nvPicPr>
                  <pic:blipFill>
                    <a:blip r:embed="rId1"/>
                    <a:stretch><a:fillRect/></a:stretch>
                  </pic:blipFill>
                  <pic:spPr>
                    <a:xfrm>
                      <a:off x="0" y="0"/>
                      <a:ext cx="9525" cy="9525"/>
                    </a:xfrm>
                    <a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
                  </pic:spPr>
                </pic:pic>
              </a:graphicData>
            </a:graphic>
          </wp:inline>
        </w:drawing>
      </w:r>
    </w:p>
    <w:sectPr/>
  </w:body>
</w:document>"#;
    let doc_rels = docx_doc_rels(
      r#"
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/image" Target="media/image1.png"/>"#,
    );
    let data = make_package(&[
      (
        "[Content_Types].xml",
        &docx_content_types(
          "",
          "\n  <Default Extension=\"png\" ContentType=\"image/png\"/>",
        ),
      ),
      ("_rels/.rels", &root_rels("word/document.xml")),
      ("word/document.xml", doc),
      ("word/_rels/document.xml.rels", &doc_rels),
      ("word/media/image1.png", png),
    ]);
    save(root, "test-data/document/minimal_image.docx", &data);
  }
}

fn create_xlsx_fixtures(root: &Path) {
  // ── minimal_empty ────────────────────────────────────────────────────────
  {
    let data = make_package(&[
      ("[Content_Types].xml", &xlsx_content_types(1, "")),
      ("_rels/.rels", &root_rels("xl/workbook.xml")),
      ("xl/workbook.xml", &workbook_xml(&[("Sheet1", 1, "rId1")])),
      ("xl/_rels/workbook.xml.rels", &workbook_rels("")),
      ("xl/worksheets/sheet1.xml", empty_worksheet()),
    ]);
    save(root, "test-data/spreadsheet/minimal_empty.xlsx", &data);
  }

  // ── minimal_values ───────────────────────────────────────────────────────
  {
    let sheet = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<x:worksheet xmlns:x="http://schemas.openxmlformats.org/spreadsheetml/2006/main">
  <x:sheetData>
    <x:row r="1">
      <x:c r="A1" t="s"><x:v>0</x:v></x:c>
      <x:c r="B1"><x:v>42</x:v></x:c>
      <x:c r="C1" t="b"><x:v>1</x:v></x:c>
      <x:c r="D1"><x:v>44927</x:v></x:c>
    </x:row>
  </x:sheetData>
</x:worksheet>"#;
    let sst = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<x:sst xmlns:x="http://schemas.openxmlformats.org/spreadsheetml/2006/main" count="1" uniqueCount="1">
  <x:si><x:t>Hello</x:t></x:si>
</x:sst>"#;
    let wb_rels = workbook_rels(
      r#"
  <Relationship Id="rId2" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/sharedStrings" Target="sharedStrings.xml"/>"#,
    );
    let data = make_package(&[
      (
        "[Content_Types].xml",
        &xlsx_content_types(
          1,
          r#"
  <Override PartName="/xl/sharedStrings.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.sharedStrings+xml"/>"#,
        ),
      ),
      ("_rels/.rels", &root_rels("xl/workbook.xml")),
      ("xl/workbook.xml", &workbook_xml(&[("Sheet1", 1, "rId1")])),
      ("xl/_rels/workbook.xml.rels", &wb_rels),
      ("xl/worksheets/sheet1.xml", sheet),
      ("xl/sharedStrings.xml", sst),
    ]);
    save(root, "test-data/spreadsheet/minimal_values.xlsx", &data);
  }

  // ── minimal_formula ──────────────────────────────────────────────────────
  {
    let sheet = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<x:worksheet xmlns:x="http://schemas.openxmlformats.org/spreadsheetml/2006/main">
  <x:sheetData>
    <x:row r="1"><x:c r="A1"><x:v>1</x:v></x:c></x:row>
    <x:row r="2"><x:c r="A2"><x:v>2</x:v></x:c></x:row>
    <x:row r="3"><x:c r="A3"><x:f>A1+A2</x:f><x:v>3</x:v></x:c></x:row>
  </x:sheetData>
</x:worksheet>"#;
    let data = make_package(&[
      ("[Content_Types].xml", &xlsx_content_types(1, "")),
      ("_rels/.rels", &root_rels("xl/workbook.xml")),
      ("xl/workbook.xml", &workbook_xml(&[("Sheet1", 1, "rId1")])),
      ("xl/_rels/workbook.xml.rels", &workbook_rels("")),
      ("xl/worksheets/sheet1.xml", sheet),
    ]);
    save(root, "test-data/spreadsheet/minimal_formula.xlsx", &data);
  }

  // ── minimal_styles ───────────────────────────────────────────────────────
  {
    let styles = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<x:styleSheet xmlns:x="http://schemas.openxmlformats.org/spreadsheetml/2006/main">
  <x:fonts count="2">
    <x:font><x:sz val="11"/><x:name val="Calibri"/></x:font>
    <x:font><x:b/><x:sz val="11"/><x:name val="Calibri"/></x:font>
  </x:fonts>
  <x:fills count="2">
    <x:fill><x:patternFill patternType="none"/></x:fill>
    <x:fill><x:patternFill patternType="gray125"/></x:fill>
  </x:fills>
  <x:borders count="1">
    <x:border><x:left/><x:right/><x:top/><x:bottom/><x:diagonal/></x:border>
  </x:borders>
  <x:cellStyleXfs count="1">
    <x:xf numFmtId="0" fontId="0" fillId="0" borderId="0"/>
  </x:cellStyleXfs>
  <x:cellXfs count="3">
    <x:xf numFmtId="0" fontId="0" fillId="0" borderId="0" xfId="0"/>
    <x:xf numFmtId="0" fontId="1" fillId="0" borderId="0" xfId="0" applyFont="1"/>
    <x:xf numFmtId="9" fontId="0" fillId="0" borderId="0" xfId="0" applyNumberFormat="1"/>
  </x:cellXfs>
</x:styleSheet>"#;
    let sheet = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<x:worksheet xmlns:x="http://schemas.openxmlformats.org/spreadsheetml/2006/main">
  <x:sheetData>
    <x:row r="1">
      <x:c r="A1" s="1"><x:v>100</x:v></x:c>
      <x:c r="B1" s="2"><x:v>0.1234</x:v></x:c>
    </x:row>
  </x:sheetData>
</x:worksheet>"#;
    let wb_rels = workbook_rels(
      r#"
  <Relationship Id="rId2" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles" Target="styles.xml"/>"#,
    );
    let data = make_package(&[
      (
        "[Content_Types].xml",
        &xlsx_content_types(
          1,
          r#"
  <Override PartName="/xl/styles.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.styles+xml"/>"#,
        ),
      ),
      ("_rels/.rels", &root_rels("xl/workbook.xml")),
      ("xl/workbook.xml", &workbook_xml(&[("Sheet1", 1, "rId1")])),
      ("xl/_rels/workbook.xml.rels", &wb_rels),
      ("xl/worksheets/sheet1.xml", sheet),
      ("xl/styles.xml", styles),
    ]);
    save(root, "test-data/spreadsheet/minimal_styles.xlsx", &data);
  }

  // ── minimal_multisheet ───────────────────────────────────────────────────
  {
    let sheet1 = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<x:worksheet xmlns:x="http://schemas.openxmlformats.org/spreadsheetml/2006/main">
  <x:sheetData>
    <x:row r="1"><x:c r="A1" t="inlineStr"><x:is><x:t>first</x:t></x:is></x:c></x:row>
  </x:sheetData>
</x:worksheet>"#;
    let sheet2 = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<x:worksheet xmlns:x="http://schemas.openxmlformats.org/spreadsheetml/2006/main">
  <x:sheetData>
    <x:row r="1"><x:c r="A1" t="inlineStr"><x:is><x:t>second</x:t></x:is></x:c></x:row>
  </x:sheetData>
</x:worksheet>"#;
    let wb_rels = format!(
      r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="{RELS_XMLNS}">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/sheet1.xml"/>
  <Relationship Id="rId2" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/sheet2.xml"/>
</Relationships>"#
    );
    let data = make_package(&[
      ("[Content_Types].xml", &xlsx_content_types(2, "")),
      ("_rels/.rels", &root_rels("xl/workbook.xml")),
      (
        "xl/workbook.xml",
        &workbook_xml(&[("Sheet1", 1, "rId1"), ("Sheet2", 2, "rId2")]),
      ),
      ("xl/_rels/workbook.xml.rels", wb_rels.as_bytes()),
      ("xl/worksheets/sheet1.xml", sheet1),
      ("xl/worksheets/sheet2.xml", sheet2),
    ]);
    save(root, "test-data/spreadsheet/minimal_multisheet.xlsx", &data);
  }
}

fn create_pptx_fixtures(root: &Path, png: &[u8]) {
  let pres1 = presentation_xml(1);
  let pres1_rels = presentation_rels(1);
  let master_rels = slide_master_rels("");
  let layout_rels = slide_layout_back_rels();
  let blank_sl_rels = slide_to_layout_rels("../slideLayouts/slideLayout1.xml", "");

  // ── minimal_empty ────────────────────────────────────────────────────────
  {
    let ct = pptx_content_types(1, "");
    let data = make_pptx(PptxParts {
      content_types: ct,
      pres_xml: pres1.clone(),
      pres_rels: pres1_rels.clone(),
      master_xml: SLIDE_MASTER_XML,
      master_rels: master_rels.clone(),
      layout_xml: blank_slide_layout(),
      layout_rels: layout_rels.clone(),
      slide_xml: blank_slide().to_vec(),
      slide_rels: blank_sl_rels.clone(),
      extra: vec![],
    });
    save(root, "test-data/slideshow/minimal_empty.pptx", &data);
  }

  // ── minimal_text ─────────────────────────────────────────────────────────
  {
    let slide = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sld xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
       xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
       xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
  <p:cSld>
    <p:spTree>
      <p:nvGrpSpPr>
        <p:cNvPr id="1" name=""/>
        <p:cNvGrpSpPr/>
        <p:nvPr/>
      </p:nvGrpSpPr>
      <p:grpSpPr>
        <a:xfrm>
          <a:off x="0" y="0"/>
          <a:ext cx="0" cy="0"/>
          <a:chOff x="0" y="0"/>
          <a:chExt cx="0" cy="0"/>
        </a:xfrm>
      </p:grpSpPr>
      <p:sp>
        <p:nvSpPr>
          <p:cNvPr id="2" name="TextBox 1"/>
          <p:cNvSpPr txBox="1"/>
          <p:nvPr/>
        </p:nvSpPr>
        <p:spPr>
          <a:xfrm>
            <a:off x="914400" y="914400"/>
            <a:ext cx="2743200" cy="1143000"/>
          </a:xfrm>
          <a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
          <a:noFill/>
        </p:spPr>
        <p:txBody>
          <a:bodyPr/>
          <a:lstStyle/>
          <a:p><a:r><a:t>Hello ooxmlsdk</a:t></a:r></a:p>
        </p:txBody>
      </p:sp>
    </p:spTree>
  </p:cSld>
  <p:clrMapOvr>
    <a:masterClrMapping/>
  </p:clrMapOvr>
</p:sld>"#;
    let ct = pptx_content_types(1, "");
    let data = make_pptx(PptxParts {
      content_types: ct,
      pres_xml: pres1.clone(),
      pres_rels: pres1_rels.clone(),
      master_xml: SLIDE_MASTER_XML,
      master_rels: master_rels.clone(),
      layout_xml: blank_slide_layout(),
      layout_rels: layout_rels.clone(),
      slide_xml: slide.to_vec(),
      slide_rels: blank_sl_rels.clone(),
      extra: vec![],
    });
    save(root, "test-data/slideshow/minimal_text.pptx", &data);
  }

  // ── minimal_layout ───────────────────────────────────────────────────────
  {
    let slide = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sld xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
       xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
       xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
  <p:cSld>
    <p:spTree>
      <p:nvGrpSpPr>
        <p:cNvPr id="1" name=""/>
        <p:cNvGrpSpPr/>
        <p:nvPr/>
      </p:nvGrpSpPr>
      <p:grpSpPr>
        <a:xfrm>
          <a:off x="0" y="0"/>
          <a:ext cx="0" cy="0"/>
          <a:chOff x="0" y="0"/>
          <a:chExt cx="0" cy="0"/>
        </a:xfrm>
      </p:grpSpPr>
      <p:sp>
        <p:nvSpPr>
          <p:cNvPr id="2" name="Title 1"/>
          <p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
          <p:nvPr><p:ph type="ctrTitle"/></p:nvPr>
        </p:nvSpPr>
        <p:spPr/>
        <p:txBody>
          <a:bodyPr/>
          <a:lstStyle/>
          <a:p><a:r><a:t>Title Slide</a:t></a:r></a:p>
        </p:txBody>
      </p:sp>
    </p:spTree>
  </p:cSld>
  <p:clrMapOvr>
    <a:masterClrMapping/>
  </p:clrMapOvr>
</p:sld>"#;
    // Master references both layouts; layout1=blank, layout2=title
    let layout_ct: &[u8] = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
  <Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
  <Default Extension="xml" ContentType="application/xml"/>
  <Override PartName="/ppt/presentation.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml"/>
  <Override PartName="/ppt/slideMasters/slideMaster1.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slideMaster+xml"/>
  <Override PartName="/ppt/slideLayouts/slideLayout1.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slideLayout+xml"/>
  <Override PartName="/ppt/slideLayouts/slideLayout2.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slideLayout+xml"/>
  <Override PartName="/ppt/slides/slide1.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slide+xml"/>
</Types>"#;
    let master_rels_layout = slide_master_rels(
      r#"
  <Relationship Id="rId2" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideLayout" Target="../slideLayouts/slideLayout2.xml"/>"#,
    );
    let layout2_back_rels = slide_layout_back_rels();
    let slide_rels = slide_to_layout_rels("../slideLayouts/slideLayout2.xml", "");
    let root_r = root_rels("ppt/presentation.xml");
    let data = make_package(&[
      ("[Content_Types].xml", layout_ct),
      ("_rels/.rels", &root_r),
      ("ppt/presentation.xml", &pres1),
      ("ppt/_rels/presentation.xml.rels", &pres1_rels),
      ("ppt/slideMasters/slideMaster1.xml", SLIDE_MASTER_XML),
      (
        "ppt/slideMasters/_rels/slideMaster1.xml.rels",
        &master_rels_layout,
      ),
      ("ppt/slideLayouts/slideLayout1.xml", blank_slide_layout()),
      ("ppt/slideLayouts/_rels/slideLayout1.xml.rels", &layout_rels),
      ("ppt/slideLayouts/slideLayout2.xml", TITLE_SLIDE_LAYOUT_XML),
      (
        "ppt/slideLayouts/_rels/slideLayout2.xml.rels",
        &layout2_back_rels,
      ),
      ("ppt/slides/slide1.xml", slide),
      ("ppt/slides/_rels/slide1.xml.rels", &slide_rels),
    ]);
    save(root, "test-data/slideshow/minimal_layout.pptx", &data);
  }

  // ── minimal_table ────────────────────────────────────────────────────────
  {
    let slide = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sld xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
       xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
       xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
  <p:cSld>
    <p:spTree>
      <p:nvGrpSpPr>
        <p:cNvPr id="1" name=""/>
        <p:cNvGrpSpPr/>
        <p:nvPr/>
      </p:nvGrpSpPr>
      <p:grpSpPr>
        <a:xfrm>
          <a:off x="0" y="0"/>
          <a:ext cx="0" cy="0"/>
          <a:chOff x="0" y="0"/>
          <a:chExt cx="0" cy="0"/>
        </a:xfrm>
      </p:grpSpPr>
      <p:graphicFrame>
        <p:nvGraphicFramePr>
          <p:cNvPr id="2" name="Table 1"/>
          <p:cNvGraphicFramePr>
            <a:graphicFrameLocks noGrp="1"/>
          </p:cNvGraphicFramePr>
          <p:nvPr/>
        </p:nvGraphicFramePr>
        <p:xfrm>
          <a:off x="457200" y="457200"/>
          <a:ext cx="8229600" cy="1143000"/>
        </p:xfrm>
        <a:graphic>
          <a:graphicData uri="http://schemas.openxmlformats.org/drawingml/2006/table">
            <a:tbl>
              <a:tblPr firstRow="1" bandRow="1"/>
              <a:tblGrid>
                <a:gridCol w="4114800"/>
                <a:gridCol w="4114800"/>
              </a:tblGrid>
              <a:tr h="571500">
                <a:tc><a:txBody><a:bodyPr/><a:lstStyle/><a:p><a:r><a:t>A1</a:t></a:r></a:p></a:txBody><a:tcPr/></a:tc>
                <a:tc><a:txBody><a:bodyPr/><a:lstStyle/><a:p><a:r><a:t>A2</a:t></a:r></a:p></a:txBody><a:tcPr/></a:tc>
              </a:tr>
              <a:tr h="571500">
                <a:tc><a:txBody><a:bodyPr/><a:lstStyle/><a:p><a:r><a:t>B1</a:t></a:r></a:p></a:txBody><a:tcPr/></a:tc>
                <a:tc><a:txBody><a:bodyPr/><a:lstStyle/><a:p><a:r><a:t>B2</a:t></a:r></a:p></a:txBody><a:tcPr/></a:tc>
              </a:tr>
            </a:tbl>
          </a:graphicData>
        </a:graphic>
      </p:graphicFrame>
    </p:spTree>
  </p:cSld>
  <p:clrMapOvr>
    <a:masterClrMapping/>
  </p:clrMapOvr>
</p:sld>"#;
    let ct = pptx_content_types(1, "");
    let data = make_pptx(PptxParts {
      content_types: ct,
      pres_xml: pres1.clone(),
      pres_rels: pres1_rels.clone(),
      master_xml: SLIDE_MASTER_XML,
      master_rels: master_rels.clone(),
      layout_xml: blank_slide_layout(),
      layout_rels: layout_rels.clone(),
      slide_xml: slide.to_vec(),
      slide_rels: blank_sl_rels.clone(),
      extra: vec![],
    });
    save(root, "test-data/slideshow/minimal_table.pptx", &data);
  }

  // ── minimal_image ────────────────────────────────────────────────────────
  {
    let slide = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sld xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
       xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
       xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
  <p:cSld>
    <p:spTree>
      <p:nvGrpSpPr>
        <p:cNvPr id="1" name=""/>
        <p:cNvGrpSpPr/>
        <p:nvPr/>
      </p:nvGrpSpPr>
      <p:grpSpPr>
        <a:xfrm>
          <a:off x="0" y="0"/>
          <a:ext cx="0" cy="0"/>
          <a:chOff x="0" y="0"/>
          <a:chExt cx="0" cy="0"/>
        </a:xfrm>
      </p:grpSpPr>
      <p:pic>
        <p:nvPicPr>
          <p:cNvPr id="2" name="Image 1"/>
          <p:cNvPicPr>
            <a:picLocks noChangeAspect="1"/>
          </p:cNvPicPr>
          <p:nvPr/>
        </p:nvPicPr>
        <p:blipFill>
          <a:blip r:embed="rId2"/>
          <a:stretch><a:fillRect/></a:stretch>
        </p:blipFill>
        <p:spPr>
          <a:xfrm>
            <a:off x="914400" y="914400"/>
            <a:ext cx="9525" cy="9525"/>
          </a:xfrm>
          <a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
        </p:spPr>
      </p:pic>
    </p:spTree>
  </p:cSld>
  <p:clrMapOvr>
    <a:masterClrMapping/>
  </p:clrMapOvr>
</p:sld>"#;
    let img_slide_rels = slide_to_layout_rels(
      "../slideLayouts/slideLayout1.xml",
      r#"
  <Relationship Id="rId2" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/image" Target="../media/image1.png"/>"#,
    );
    let ct = pptx_content_types(
      1,
      "\n  <Default Extension=\"png\" ContentType=\"image/png\"/>",
    );
    let data = make_pptx(PptxParts {
      content_types: ct,
      pres_xml: pres1.clone(),
      pres_rels: pres1_rels.clone(),
      master_xml: SLIDE_MASTER_XML,
      master_rels: master_rels.clone(),
      layout_xml: blank_slide_layout(),
      layout_rels: layout_rels.clone(),
      slide_xml: slide.to_vec(),
      slide_rels: img_slide_rels,
      extra: vec![("ppt/media/image1.png", png.to_vec())],
    });
    save(root, "test-data/slideshow/minimal_image.pptx", &data);
  }
}

fn create_mce_fixtures(root: &Path) {
  // ── MCE-01: ignorable_unknown_ns.docx ────────────────────────────────────
  {
    let doc = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:document
    xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"
    xmlns:ext="http://example.com/ooxmlsdk-test/ext/1"
    xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006"
    mc:Ignorable="ext">
  <w:body>
    <w:p>
      <w:r>
        <w:rPr>
          <ext:decoration ext:style="fancy"/>
        </w:rPr>
        <w:t>hello</w:t>
      </w:r>
    </w:p>
    <w:sectPr/>
  </w:body>
</w:document>"#;
    let data = make_package(&[
      ("[Content_Types].xml", &docx_content_types("", "")),
      ("_rels/.rels", &root_rels("word/document.xml")),
      ("word/document.xml", doc),
      ("word/_rels/document.xml.rels", empty_rels()),
    ]);
    save(root, "test-data/mce/ignorable_unknown_ns.docx", &data);
  }

  // ── MCE-02: process_content.docx ─────────────────────────────────────────
  {
    let doc = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:document
    xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"
    xmlns:wrap="http://example.com/ooxmlsdk-test/wrap/1"
    xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006"
    mc:Ignorable="wrap"
    mc:ProcessContent="wrap:group">
  <w:body>
    <wrap:group>
      <w:p><w:r><w:t>inside wrapper</w:t></w:r></w:p>
    </wrap:group>
    <w:sectPr/>
  </w:body>
</w:document>"#;
    let data = make_package(&[
      ("[Content_Types].xml", &docx_content_types("", "")),
      ("_rels/.rels", &root_rels("word/document.xml")),
      ("word/document.xml", doc),
      ("word/_rels/document.xml.rels", empty_rels()),
    ]);
    save(root, "test-data/mce/process_content.docx", &data);
  }

  // ── MCE-03: alternate_content_fallback.docx ───────────────────────────────
  {
    let doc = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:document
    xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"
    xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006">
  <w:body>
    <mc:AlternateContent>
      <mc:Choice Requires="nonexistent"
                 xmlns:nonexistent="http://example.com/ooxmlsdk-test/ne/1">
        <w:p><w:r><w:t>new content</w:t></w:r></w:p>
      </mc:Choice>
      <mc:Fallback>
        <w:p><w:r><w:t>fallback content</w:t></w:r></w:p>
      </mc:Fallback>
    </mc:AlternateContent>
    <w:sectPr/>
  </w:body>
</w:document>"#;
    let data = make_package(&[
      ("[Content_Types].xml", &docx_content_types("", "")),
      ("_rels/.rels", &root_rels("word/document.xml")),
      ("word/document.xml", doc),
      ("word/_rels/document.xml.rels", empty_rels()),
    ]);
    save(root, "test-data/mce/alternate_content_fallback.docx", &data);
  }

  // ── MCE-04: alternate_content_pptx.pptx (issue #11) ──────────────────────
  {
    let pres = presentation_xml(1);
    let pres_rels = presentation_rels(1);
    let master_rels = slide_master_rels("");
    let layout_rels = slide_layout_back_rels();
    let slide = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sld xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
       xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
       xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
       xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006">
  <p:cSld>
    <p:spTree>
      <p:nvGrpSpPr>
        <p:cNvPr id="1" name=""/>
        <p:cNvGrpSpPr/>
        <p:nvPr/>
      </p:nvGrpSpPr>
      <p:grpSpPr>
        <a:xfrm>
          <a:off x="0" y="0"/>
          <a:ext cx="0" cy="0"/>
          <a:chOff x="0" y="0"/>
          <a:chExt cx="0" cy="0"/>
        </a:xfrm>
      </p:grpSpPr>
      <p:sp>
        <p:nvSpPr>
          <p:cNvPr id="2" name="TextBox 1"/>
          <p:cNvSpPr txBox="1"/>
          <p:nvPr/>
        </p:nvSpPr>
        <p:spPr>
          <a:xfrm>
            <a:off x="914400" y="914400"/>
            <a:ext cx="2743200" cy="1143000"/>
          </a:xfrm>
          <a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
        </p:spPr>
        <p:txBody>
          <a:bodyPr/>
          <a:lstStyle/>
          <a:p>
            <a:r>
              <mc:AlternateContent>
                <mc:Choice Requires="a14"
                           xmlns:a14="http://schemas.microsoft.com/office/drawing/2010/main">
                  <a14:hiddenLine val="1"/>
                </mc:Choice>
                <mc:Fallback/>
              </mc:AlternateContent>
              <a:t>test text</a:t>
            </a:r>
          </a:p>
        </p:txBody>
      </p:sp>
    </p:spTree>
  </p:cSld>
  <p:clrMapOvr>
    <a:masterClrMapping/>
  </p:clrMapOvr>
</p:sld>"#;
    let ct = pptx_content_types(1, "");
    let sl_rels = slide_to_layout_rels("../slideLayouts/slideLayout1.xml", "");
    let data = make_pptx(PptxParts {
      content_types: ct,
      pres_xml: pres,
      pres_rels,
      master_xml: SLIDE_MASTER_XML,
      master_rels,
      layout_xml: blank_slide_layout(),
      layout_rels,
      slide_xml: slide.to_vec(),
      slide_rels: sl_rels,
      extra: vec![],
    });
    save(root, "test-data/mce/alternate_content_pptx.pptx", &data);
  }

  // ── MCE-05: extlst_unknown_ns.pptx (issue #8) ────────────────────────────
  {
    let pres = presentation_xml(1);
    let pres_rels = presentation_rels(1);
    let master_rels = slide_master_rels("");
    let layout_rels = slide_layout_back_rels();
    let slide = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sld xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
       xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
       xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
  <p:cSld>
    <p:spTree>
      <p:nvGrpSpPr>
        <p:cNvPr id="1" name=""/>
        <p:cNvGrpSpPr/>
        <p:nvPr/>
      </p:nvGrpSpPr>
      <p:grpSpPr>
        <a:xfrm>
          <a:off x="0" y="0"/>
          <a:ext cx="0" cy="0"/>
          <a:chOff x="0" y="0"/>
          <a:chExt cx="0" cy="0"/>
        </a:xfrm>
      </p:grpSpPr>
      <p:sp>
        <p:nvSpPr>
          <p:cNvPr id="3" name="Shape 2">
            <a:extLst>
              <a:ext uri="{FF2B5EF4-FFF2-40B4-BE49-F238E27FC236}">
                <a16:creationId
                    xmlns:a16="http://schemas.microsoft.com/office/drawing/2014/main"
                    id="{AAAAAAAA-BBBB-CCCC-DDDD-EEEEEEEEEEEE}"/>
              </a:ext>
            </a:extLst>
          </p:cNvPr>
          <p:cNvSpPr txBox="1"/>
          <p:nvPr/>
        </p:nvSpPr>
        <p:spPr>
          <a:xfrm>
            <a:off x="914400" y="914400"/>
            <a:ext cx="2743200" cy="1143000"/>
          </a:xfrm>
          <a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
        </p:spPr>
        <p:txBody>
          <a:bodyPr/>
          <a:lstStyle/>
          <a:p><a:r><a:t>extlst test</a:t></a:r></a:p>
        </p:txBody>
      </p:sp>
    </p:spTree>
  </p:cSld>
  <p:clrMapOvr>
    <a:masterClrMapping/>
  </p:clrMapOvr>
</p:sld>"#;
    let ct = pptx_content_types(1, "");
    let sl_rels = slide_to_layout_rels("../slideLayouts/slideLayout1.xml", "");
    let data = make_pptx(PptxParts {
      content_types: ct,
      pres_xml: pres,
      pres_rels,
      master_xml: SLIDE_MASTER_XML,
      master_rels,
      layout_xml: blank_slide_layout(),
      layout_rels,
      slide_xml: slide.to_vec(),
      slide_rels: sl_rels,
      extra: vec![],
    });
    save(root, "test-data/mce/extlst_unknown_ns.pptx", &data);
  }

  // ── MCE-06: must_understand_ok.docx ──────────────────────────────────────
  {
    let doc = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:document
    xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"
    xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006"
    mc:MustUnderstand="w">
  <w:body>
    <w:p><w:r><w:t>text</w:t></w:r></w:p>
    <w:sectPr/>
  </w:body>
</w:document>"#;
    let data = make_package(&[
      ("[Content_Types].xml", &docx_content_types("", "")),
      ("_rels/.rels", &root_rels("word/document.xml")),
      ("word/document.xml", doc),
      ("word/_rels/document.xml.rels", empty_rels()),
    ]);
    save(root, "test-data/mce/must_understand_ok.docx", &data);
  }

  // ── MCE-07: nested_alternate_content.docx ─────────────────────────────────
  {
    let doc = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:document
    xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"
    xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006">
  <w:body>
    <mc:AlternateContent>
      <mc:Choice Requires="outer"
                 xmlns:outer="http://example.com/outer">
        <mc:AlternateContent>
          <mc:Choice Requires="inner"
                     xmlns:inner="http://example.com/inner">
            <w:p><w:r><w:t>both</w:t></w:r></w:p>
          </mc:Choice>
          <mc:Fallback>
            <w:p><w:r><w:t>outer only</w:t></w:r></w:p>
          </mc:Fallback>
        </mc:AlternateContent>
      </mc:Choice>
      <mc:Fallback>
        <w:p><w:r><w:t>neither</w:t></w:r></w:p>
      </mc:Fallback>
    </mc:AlternateContent>
    <w:sectPr/>
  </w:body>
</w:document>"#;
    let data = make_package(&[
      ("[Content_Types].xml", &docx_content_types("", "")),
      ("_rels/.rels", &root_rels("word/document.xml")),
      ("word/document.xml", doc),
      ("word/_rels/document.xml.rels", empty_rels()),
    ]);
    save(root, "test-data/mce/nested_alternate_content.docx", &data);
  }
}

fn main() {
  let root = workspace_root();
  let png = base64::engine::general_purpose::STANDARD
    .decode(PNG_BASE64)
    .expect("decode 1x1 PNG bytes");

  create_docx_fixtures(&root, &png);
  create_xlsx_fixtures(&root);
  create_pptx_fixtures(&root, &png);
  create_mce_fixtures(&root);
}
