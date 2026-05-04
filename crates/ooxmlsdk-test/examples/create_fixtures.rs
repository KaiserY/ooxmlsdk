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

fn root_rels_with_extra(office_doc_target: &str, extra: &str) -> Vec<u8> {
  format!(
    r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="{RELS_XMLNS}">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="{office_doc_target}"/>{extra}
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

fn create_opc_fixtures(root: &Path, png: &[u8]) {
  let minimal_doc = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:body><w:sectPr/></w:body>
</w:document>"#;

  // ── OPC-01: core_properties ──────────────────────────────────────────────
  // Tests that a core-properties part (Dublin Core + OPC, xsi:type on
  // dcterms:created/modified) doesn't break open/save/reopen.
  {
    let core_xml = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<cp:coreProperties
    xmlns:cp="http://schemas.openxmlformats.org/package/2006/metadata/core-properties"
    xmlns:dc="http://purl.org/dc/elements/1.1/"
    xmlns:dcterms="http://purl.org/dc/terms/"
    xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
  <dc:title>OPC Test Document</dc:title>
  <dc:creator>ooxmlsdk test</dc:creator>
  <dc:description>Fixture for OPC core-properties round-trip test</dc:description>
  <dc:language>en-US</dc:language>
  <dc:subject>OPC conformance</dc:subject>
  <cp:keywords>opc, test, fixture</cp:keywords>
  <cp:lastModifiedBy>ooxmlsdk test</cp:lastModifiedBy>
  <cp:revision>1</cp:revision>
  <cp:category>Test</cp:category>
  <cp:contentStatus>Draft</cp:contentStatus>
  <dcterms:created xsi:type="dcterms:W3CDTF">2026-05-02T00:00:00Z</dcterms:created>
  <dcterms:modified xsi:type="dcterms:W3CDTF">2026-05-02T00:00:00Z</dcterms:modified>
</cp:coreProperties>"#;
    let ct = docx_content_types(
      r#"
  <Override PartName="/docProps/core.xml" ContentType="application/vnd.openxmlformats-package.core-properties+xml"/>"#,
      "",
    );
    let root_r = root_rels_with_extra(
      "word/document.xml",
      r#"
  <Relationship Id="rId2" Type="http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties" Target="docProps/core.xml"/>"#,
    );
    let data = make_package(&[
      ("[Content_Types].xml", &ct),
      ("_rels/.rels", &root_r),
      ("word/document.xml", minimal_doc),
      ("word/_rels/document.xml.rels", empty_rels()),
      ("docProps/core.xml", core_xml),
    ]);
    save(root, "test-data/opc/core_properties.docx", &data);
  }

  // ── OPC-02: thumbnail ────────────────────────────────────────────────────
  // Tests that a binary thumbnail part referenced from /_rels/.rels doesn't
  // break open/save/reopen. Exercises the OPC thumbnail relationship type.
  {
    let ct = docx_content_types(
      "",
      "\n  <Default Extension=\"png\" ContentType=\"image/png\"/>",
    );
    let root_r = root_rels_with_extra(
      "word/document.xml",
      r#"
  <Relationship Id="rId2" Type="http://schemas.openxmlformats.org/package/2006/relationships/metadata/thumbnail" Target="docProps/thumbnail.png"/>"#,
    );
    let data = make_package(&[
      ("[Content_Types].xml", &ct),
      ("_rels/.rels", &root_r),
      ("word/document.xml", minimal_doc),
      ("word/_rels/document.xml.rels", empty_rels()),
      ("docProps/thumbnail.png", png),
    ]);
    save(root, "test-data/opc/thumbnail.docx", &data);
  }

  // ── OPC-03: multiple_rels ────────────────────────────────────────────────
  // Tests that a single .rels file with multiple relationships (styles + image)
  // all resolve correctly. Combines minimal_styles and minimal_image in one doc.
  {
    let doc = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"
            xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
            xmlns:wp="http://schemas.openxmlformats.org/drawingml/2006/wordprocessingDrawing"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
            xmlns:pic="http://schemas.openxmlformats.org/drawingml/2006/picture">
  <w:body>
    <w:p>
      <w:pPr><w:pStyle w:val="Heading1"/></w:pPr>
      <w:r><w:t>Heading</w:t></w:r>
    </w:p>
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
                    <a:blip r:embed="rId2"/>
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
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles" Target="styles.xml"/>
  <Relationship Id="rId2" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/image" Target="media/image1.png"/>"#,
    );
    let ct = docx_content_types(
      r#"
  <Override PartName="/word/styles.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml"/>"#,
      "\n  <Default Extension=\"png\" ContentType=\"image/png\"/>",
    );
    let data = make_package(&[
      ("[Content_Types].xml", &ct),
      ("_rels/.rels", &root_rels("word/document.xml")),
      ("word/document.xml", doc),
      ("word/_rels/document.xml.rels", &doc_rels),
      ("word/styles.xml", styles),
      ("word/media/image1.png", png),
    ]);
    save(root, "test-data/opc/multiple_rels.docx", &data);
  }
}

// ── Minimal theme XML ────────────────────────────────────────────────────────

const THEME1_XML: &[u8] = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<a:theme xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" name="Test Theme">
  <a:themeElements>
    <a:clrScheme name="Test">
      <a:dk1><a:sysClr val="windowText" lastClr="000000"/></a:dk1>
      <a:lt1><a:sysClr val="window" lastClr="FFFFFF"/></a:lt1>
      <a:dk2><a:srgbClr val="1F497D"/></a:dk2>
      <a:lt2><a:srgbClr val="EEECE1"/></a:lt2>
      <a:accent1><a:srgbClr val="4F81BD"/></a:accent1>
      <a:accent2><a:srgbClr val="C0504D"/></a:accent2>
      <a:accent3><a:srgbClr val="9BBB59"/></a:accent3>
      <a:accent4><a:srgbClr val="8064A2"/></a:accent4>
      <a:accent5><a:srgbClr val="4BACC6"/></a:accent5>
      <a:accent6><a:srgbClr val="F79646"/></a:accent6>
      <a:hlink><a:srgbClr val="0000FF"/></a:hlink>
      <a:folHlink><a:srgbClr val="800080"/></a:folHlink>
    </a:clrScheme>
    <a:fontScheme name="Test">
      <a:majorFont>
        <a:latin typeface="Calibri"/>
        <a:ea typeface=""/>
        <a:cs typeface=""/>
      </a:majorFont>
      <a:minorFont>
        <a:latin typeface="Calibri"/>
        <a:ea typeface=""/>
        <a:cs typeface=""/>
      </a:minorFont>
    </a:fontScheme>
    <a:fmtScheme name="Test">
      <a:fillStyleLst>
        <a:solidFill><a:schemeClr val="phClr"/></a:solidFill>
        <a:gradFill rotWithShape="1">
          <a:gsLst>
            <a:gs pos="0">
              <a:schemeClr val="phClr"><a:lumMod val="110000"/><a:satMod val="105000"/></a:schemeClr>
            </a:gs>
            <a:gs pos="100000">
              <a:schemeClr val="phClr"><a:lumMod val="60000"/><a:satMod val="99000"/></a:schemeClr>
            </a:gs>
          </a:gsLst>
          <a:lin ang="5400000" scaled="0"/>
        </a:gradFill>
        <a:solidFill><a:schemeClr val="phClr"><a:tint val="20000"/></a:schemeClr></a:solidFill>
      </a:fillStyleLst>
      <a:lnStyleLst>
        <a:ln w="9525" cap="flat" cmpd="sng" algn="ctr">
          <a:solidFill><a:schemeClr val="phClr"><a:shade val="95000"/></a:schemeClr></a:solidFill>
          <a:prstDash val="solid"/>
        </a:ln>
        <a:ln w="25400" cap="flat" cmpd="sng" algn="ctr">
          <a:solidFill><a:schemeClr val="phClr"/></a:solidFill>
          <a:prstDash val="solid"/>
        </a:ln>
        <a:ln w="38100" cap="flat" cmpd="sng" algn="ctr">
          <a:solidFill><a:schemeClr val="phClr"/></a:solidFill>
          <a:prstDash val="solid"/>
        </a:ln>
      </a:lnStyleLst>
      <a:effectStyleLst>
        <a:effectStyle><a:effectLst/></a:effectStyle>
        <a:effectStyle>
          <a:effectLst>
            <a:outerShdw blurRad="40000" dist="20000" dir="5400000" rotWithShape="0">
              <a:srgbClr val="000000"><a:alpha val="38000"/></a:srgbClr>
            </a:outerShdw>
          </a:effectLst>
        </a:effectStyle>
        <a:effectStyle>
          <a:effectLst>
            <a:outerShdw blurRad="40000" dist="23000" dir="5400000" rotWithShape="0">
              <a:srgbClr val="000000"><a:alpha val="35000"/></a:srgbClr>
            </a:outerShdw>
          </a:effectLst>
        </a:effectStyle>
      </a:effectStyleLst>
      <a:bgFillStyleLst>
        <a:solidFill><a:schemeClr val="phClr"/></a:solidFill>
        <a:solidFill><a:schemeClr val="phClr"><a:tint val="95000"/><a:satMod val="170000"/></a:schemeClr></a:solidFill>
        <a:gradFill rotWithShape="1">
          <a:gsLst>
            <a:gs pos="0">
              <a:schemeClr val="phClr"><a:tint val="93000"/><a:satMod val="150000"/><a:shade val="98000"/><a:lumMod val="102000"/></a:schemeClr>
            </a:gs>
            <a:gs pos="50000">
              <a:schemeClr val="phClr"><a:tint val="98000"/><a:satMod val="130000"/><a:shade val="90000"/><a:lumMod val="103000"/></a:schemeClr>
            </a:gs>
            <a:gs pos="100000">
              <a:schemeClr val="phClr"><a:shade val="63000"/><a:satMod val="120000"/></a:schemeClr>
            </a:gs>
          </a:gsLst>
          <a:lin ang="16200000" scaled="0"/>
        </a:gradFill>
      </a:bgFillStyleLst>
    </a:fmtScheme>
  </a:themeElements>
</a:theme>"#;

// ── Shared slide tree boilerplate ─────────────────────────────────────────────

fn slide_with_shapes(shapes: &str) -> Vec<u8> {
  format!(
    r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
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
{shapes}    </p:spTree>
  </p:cSld>
  <p:clrMapOvr>
    <a:masterClrMapping/>
  </p:clrMapOvr>
</p:sld>"#
  )
  .into_bytes()
}

fn dml_pptx(
  slide: Vec<u8>,
  extra_ct: &str,
  extra_master_rels: &str,
  extra_parts: Vec<(&'static str, Vec<u8>)>,
) -> Vec<u8> {
  let pres = presentation_xml(1);
  let pres_rels = presentation_rels(1);
  let master_rels = slide_master_rels(extra_master_rels);
  let layout_rels = slide_layout_back_rels();
  let slide_rels = slide_to_layout_rels("../slideLayouts/slideLayout1.xml", "");
  let ct = pptx_content_types(1, extra_ct);
  make_pptx(PptxParts {
    content_types: ct,
    pres_xml: pres,
    pres_rels,
    master_xml: SLIDE_MASTER_XML,
    master_rels,
    layout_xml: blank_slide_layout(),
    layout_rels,
    slide_xml: slide,
    slide_rels,
    extra: extra_parts,
  })
}

fn create_drawingml_fixtures(root: &Path) {
  // ── DML-01: solid_fill ───────────────────────────────────────────────────
  // Two shapes: one srgbClr solid fill, one schemeClr with lumMod + satMod.
  // Exercises solidFill, srgbClr, schemeClr, and colour transforms.
  {
    let slide = slide_with_shapes(
      r#"      <p:sp>
        <p:nvSpPr>
          <p:cNvPr id="2" name="RedRect"/>
          <p:cNvSpPr/>
          <p:nvPr/>
        </p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="457200" y="457200"/><a:ext cx="3657600" cy="1371600"/></a:xfrm>
          <a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
          <a:solidFill><a:srgbClr val="FF0000"/></a:solidFill>
        </p:spPr>
        <p:txBody><a:bodyPr/><a:lstStyle/><a:p><a:endParaRPr/></a:p></p:txBody>
      </p:sp>
      <p:sp>
        <p:nvSpPr>
          <p:cNvPr id="3" name="SchemeRect"/>
          <p:cNvSpPr/>
          <p:nvPr/>
        </p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="457200" y="2057400"/><a:ext cx="3657600" cy="1371600"/></a:xfrm>
          <a:prstGeom prst="roundRect"><a:avLst><a:gd name="adj" fmla="val 16667"/></a:avLst></a:prstGeom>
          <a:solidFill>
            <a:schemeClr val="accent1"><a:lumMod val="75000"/><a:satMod val="105000"/></a:schemeClr>
          </a:solidFill>
        </p:spPr>
        <p:txBody><a:bodyPr/><a:lstStyle/><a:p><a:endParaRPr/></a:p></p:txBody>
      </p:sp>
"#,
    );
    let data = dml_pptx(slide, "", "", vec![]);
    save(root, "test-data/drawingml/solid_fill.pptx", &data);
  }

  // ── DML-02: gradient_fill ────────────────────────────────────────────────
  // Shape with a two-stop linear gradient using schemeClr with transforms.
  // Exercises gradFill, gsLst, a:gs, a:lin.
  {
    let slide = slide_with_shapes(
      r#"      <p:sp>
        <p:nvSpPr>
          <p:cNvPr id="2" name="GradShape"/>
          <p:cNvSpPr/>
          <p:nvPr/>
        </p:nvSpPr>
        <p:spPr>
          <a:xfrm rot="900000">
            <a:off x="914400" y="914400"/>
            <a:ext cx="6400800" cy="2286000"/>
          </a:xfrm>
          <a:prstGeom prst="roundRect"><a:avLst><a:gd name="adj" fmla="val 20000"/></a:avLst></a:prstGeom>
          <a:gradFill rotWithShape="1">
            <a:gsLst>
              <a:gs pos="0">
                <a:schemeClr val="accent1"><a:lumMod val="110000"/><a:satMod val="105000"/></a:schemeClr>
              </a:gs>
              <a:gs pos="100000">
                <a:schemeClr val="accent1"><a:lumMod val="60000"/><a:satMod val="99000"/></a:schemeClr>
              </a:gs>
            </a:gsLst>
            <a:lin ang="5400000" scaled="0"/>
          </a:gradFill>
          <a:ln w="19050">
            <a:solidFill><a:schemeClr val="accent1"><a:lumMod val="75000"/></a:schemeClr></a:solidFill>
          </a:ln>
        </p:spPr>
        <p:txBody>
          <a:bodyPr anchor="ctr"/>
          <a:lstStyle/>
          <a:p>
            <a:pPr algn="ctr"/>
            <a:r>
              <a:rPr lang="en-US" sz="1800" b="1" dirty="0">
                <a:solidFill><a:srgbClr val="FFFFFF"/></a:solidFill>
                <a:latin typeface="+mj-lt"/>
              </a:rPr>
              <a:t>Gradient</a:t>
            </a:r>
          </a:p>
        </p:txBody>
      </p:sp>
"#,
    );
    let data = dml_pptx(slide, "", "", vec![]);
    save(root, "test-data/drawingml/gradient_fill.pptx", &data);
  }

  // ── DML-03: text_run_props ───────────────────────────────────────────────
  // Text box with multiple runs exercising the full rPr attribute set:
  // bold, italic, underline, strikethrough, sz, color fill, typeface,
  // paragraph alignment, line spacing, space before/after.
  {
    let slide = slide_with_shapes(
      r#"      <p:sp>
        <p:nvSpPr>
          <p:cNvPr id="2" name="RichText"/>
          <p:cNvSpPr txBox="1"/>
          <p:nvPr/>
        </p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="457200" y="457200"/><a:ext cx="8229600" cy="5486400"/></a:xfrm>
          <a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
          <a:noFill/>
        </p:spPr>
        <p:txBody>
          <a:bodyPr wrap="square" lIns="91440" rIns="91440" tIns="45720" bIns="45720" anchor="t"/>
          <a:lstStyle/>
          <a:p>
            <a:pPr algn="l" marL="0" indent="0">
              <a:spcBef><a:spcPts val="0"/></a:spcBef>
              <a:spcAft><a:spcPts val="0"/></a:spcAft>
            </a:pPr>
            <a:r>
              <a:rPr lang="en-US" sz="2800" b="1" dirty="0">
                <a:solidFill><a:srgbClr val="1F3864"/></a:solidFill>
                <a:latin typeface="+mj-lt"/>
              </a:rPr>
              <a:t>Bold Heading </a:t>
            </a:r>
            <a:r>
              <a:rPr lang="en-US" sz="2800" i="1" dirty="0">
                <a:solidFill><a:schemeClr val="accent2"/></a:solidFill>
                <a:latin typeface="+mn-lt"/>
              </a:rPr>
              <a:t>Italic Accent</a:t>
            </a:r>
          </a:p>
          <a:p>
            <a:pPr algn="ctr">
              <a:lnSpc><a:spcPct val="150000"/></a:lnSpc>
              <a:spcBef><a:spcPts val="600"/></a:spcBef>
            </a:pPr>
            <a:r>
              <a:rPr lang="en-US" sz="1800" u="sng" dirty="0">
                <a:solidFill><a:srgbClr val="0000FF"/></a:solidFill>
              </a:rPr>
              <a:t>Underlined text</a:t>
            </a:r>
            <a:r>
              <a:rPr lang="en-US" sz="1800" strike="sngStrike" dirty="0">
                <a:solidFill><a:srgbClr val="FF0000"/></a:solidFill>
              </a:rPr>
              <a:t> Strikethrough</a:t>
            </a:r>
          </a:p>
          <a:p>
            <a:r>
              <a:rPr lang="en-US" sz="1400" baseline="30000" dirty="0"/>
              <a:t>Super</a:t>
            </a:r>
            <a:r>
              <a:rPr lang="en-US" sz="1800" dirty="0"/>
              <a:t>script and </a:t>
            </a:r>
            <a:r>
              <a:rPr lang="en-US" sz="1400" baseline="-25000" dirty="0"/>
              <a:t>sub</a:t>
            </a:r>
            <a:r>
              <a:rPr lang="en-US" sz="1800" dirty="0"/>
              <a:t>script</a:t>
            </a:r>
            <a:endParaRPr lang="en-US" dirty="0"/>
          </a:p>
        </p:txBody>
      </p:sp>
"#,
    );
    let data = dml_pptx(slide, "", "", vec![]);
    save(root, "test-data/drawingml/text_run_props.pptx", &data);
  }

  // ── DML-04: shape_line ───────────────────────────────────────────────────
  // Shape with a custom outline: explicit width, dash pattern, arrowheads.
  // Also a line connector. Exercises a:ln, prstDash, headEnd, tailEnd.
  {
    let slide = slide_with_shapes(
      r#"      <p:sp>
        <p:nvSpPr>
          <p:cNvPr id="2" name="DashedBorder"/>
          <p:cNvSpPr/>
          <p:nvPr/>
        </p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="457200" y="457200"/><a:ext cx="4114800" cy="2057400"/></a:xfrm>
          <a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
          <a:noFill/>
          <a:ln w="38100" cap="rnd" cmpd="sng">
            <a:solidFill><a:srgbClr val="FF6600"/></a:solidFill>
            <a:prstDash val="lgDash"/>
          </a:ln>
        </p:spPr>
        <p:txBody><a:bodyPr/><a:lstStyle/><a:p><a:endParaRPr/></a:p></p:txBody>
      </p:sp>
      <p:cxnSp>
        <p:nvCxnSpPr>
          <p:cNvPr id="3" name="Arrow"/>
          <p:cNvCxnSpPr/>
          <p:nvPr/>
        </p:nvCxnSpPr>
        <p:spPr>
          <a:xfrm>
            <a:off x="5029200" y="1371600"/>
            <a:ext cx="2743200" cy="0"/>
          </a:xfrm>
          <a:prstGeom prst="line"><a:avLst/></a:prstGeom>
          <a:ln w="25400">
            <a:solidFill><a:srgbClr val="0070C0"/></a:solidFill>
            <a:prstDash val="solid"/>
            <a:headEnd type="oval" w="med" len="med"/>
            <a:tailEnd type="triangle" w="lg" len="lg"/>
          </a:ln>
        </p:spPr>
      </p:cxnSp>
"#,
    );
    let data = dml_pptx(slide, "", "", vec![]);
    save(root, "test-data/drawingml/shape_line.pptx", &data);
  }

  // ── DML-05: effects ──────────────────────────────────────────────────────
  // Shape with effectLst containing outerShdw and glow.
  // Exercises effectLst, outerShdw (blurRad/dist/dir/alpha), glow.
  {
    let slide = slide_with_shapes(
      r#"      <p:sp>
        <p:nvSpPr>
          <p:cNvPr id="2" name="ShadowShape"/>
          <p:cNvSpPr/>
          <p:nvPr/>
        </p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="914400" y="914400"/><a:ext cx="5486400" cy="2743200"/></a:xfrm>
          <a:prstGeom prst="roundRect"><a:avLst><a:gd name="adj" fmla="val 16667"/></a:avLst></a:prstGeom>
          <a:solidFill><a:srgbClr val="4472C4"/></a:solidFill>
          <a:ln w="12700">
            <a:solidFill><a:srgbClr val="2F5496"/></a:solidFill>
          </a:ln>
          <a:effectLst>
            <a:outerShdw blurRad="57150" dist="38100" dir="5400000" algn="ctr" rotWithShape="0">
              <a:srgbClr val="000000"><a:alpha val="50000"/></a:srgbClr>
            </a:outerShdw>
            <a:glow rad="127000">
              <a:srgbClr val="4472C4"><a:alpha val="50000"/></a:srgbClr>
            </a:glow>
          </a:effectLst>
        </p:spPr>
        <p:txBody>
          <a:bodyPr anchor="ctr"/>
          <a:lstStyle/>
          <a:p>
            <a:pPr algn="ctr"/>
            <a:r>
              <a:rPr lang="en-US" sz="2000" b="1" dirty="0">
                <a:solidFill><a:srgbClr val="FFFFFF"/></a:solidFill>
              </a:rPr>
              <a:t>Shadow + Glow</a:t>
            </a:r>
          </a:p>
        </p:txBody>
      </p:sp>
"#,
    );
    let data = dml_pptx(slide, "", "", vec![]);
    save(root, "test-data/drawingml/effects.pptx", &data);
  }

  // ── DML-06: theme ────────────────────────────────────────────────────────
  // PPTX with a full theme.xml part referenced from the slide master.
  // Exercises the theme relationship type and theme XML (clrScheme, fontScheme,
  // fmtScheme with fills/lines/effects/bgFills).
  {
    let slide = slide_with_shapes(
      r#"      <p:sp>
        <p:nvSpPr>
          <p:cNvPr id="2" name="ThemedShape"/>
          <p:cNvSpPr/>
          <p:nvPr/>
        </p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="914400" y="914400"/><a:ext cx="5486400" cy="2743200"/></a:xfrm>
          <a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
          <a:solidFill><a:schemeClr val="accent1"/></a:solidFill>
        </p:spPr>
        <p:txBody>
          <a:bodyPr anchor="ctr"/>
          <a:lstStyle/>
          <a:p>
            <a:pPr algn="ctr"/>
            <a:r>
              <a:rPr lang="en-US" sz="2000" dirty="0">
                <a:solidFill><a:schemeClr val="lt1"/></a:solidFill>
                <a:latin typeface="+mj-lt"/>
              </a:rPr>
              <a:t>Theme Color</a:t>
            </a:r>
          </a:p>
        </p:txBody>
      </p:sp>
"#,
    );
    let theme_ct = "\n  <Override PartName=\"/ppt/theme/theme1.xml\" ContentType=\"application/vnd.openxmlformats-officedocument.theme+xml\"/>";
    let theme_master_rel = r#"
  <Relationship Id="rId2" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme" Target="../theme/theme1.xml"/>"#;
    let data = dml_pptx(
      slide,
      theme_ct,
      theme_master_rel,
      vec![("ppt/theme/theme1.xml", THEME1_XML.to_vec())],
    );
    save(root, "test-data/drawingml/theme.pptx", &data);
  }
}

fn create_wml_runs_fixtures(root: &Path) {
  // ── WML-R-01: char_formatting ────────────────────────────────────────────
  // Paragraph with runs exercising every common rPr toggle and value property:
  // b, i, combined b+i, u single/double, strike, dstrike, sz, color,
  // highlight, vertAlign superscript/subscript, caps, smallCaps.
  {
    let doc = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:body>
    <w:p>
      <w:r><w:rPr><w:b/></w:rPr><w:t>Bold</w:t></w:r>
      <w:r><w:rPr><w:i/></w:rPr><w:t xml:space="preserve"> Italic</w:t></w:r>
      <w:r><w:rPr><w:b/><w:i/></w:rPr><w:t xml:space="preserve"> BoldItalic</w:t></w:r>
      <w:r><w:rPr><w:u w:val="single"/></w:rPr><w:t xml:space="preserve"> Underline</w:t></w:r>
      <w:r><w:rPr><w:u w:val="double"/></w:rPr><w:t xml:space="preserve"> DblUnderline</w:t></w:r>
      <w:r><w:rPr><w:strike/></w:rPr><w:t xml:space="preserve"> Strike</w:t></w:r>
      <w:r><w:rPr><w:dstrike/></w:rPr><w:t xml:space="preserve"> DblStrike</w:t></w:r>
      <w:r><w:rPr><w:sz w:val="28"/><w:szCs w:val="28"/></w:rPr><w:t xml:space="preserve"> 14pt</w:t></w:r>
      <w:r><w:rPr><w:color w:val="C00000"/></w:rPr><w:t xml:space="preserve"> Red</w:t></w:r>
      <w:r><w:rPr><w:highlight w:val="yellow"/></w:rPr><w:t xml:space="preserve"> Highlight</w:t></w:r>
      <w:r><w:rPr><w:vertAlign w:val="superscript"/></w:rPr><w:t>sup</w:t></w:r>
      <w:r><w:rPr><w:vertAlign w:val="subscript"/></w:rPr><w:t>sub</w:t></w:r>
      <w:r><w:rPr><w:caps/></w:rPr><w:t xml:space="preserve"> Caps</w:t></w:r>
      <w:r><w:rPr><w:smallCaps/></w:rPr><w:t xml:space="preserve"> SmallCaps</w:t></w:r>
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
    save(root, "test-data/wml/char_formatting.docx", &data);
  }

  // ── WML-R-02: run_fonts ──────────────────────────────────────────────────
  // Runs exercising rFonts: ascii/hAnsi/eastAsia/cs explicit names,
  // asciiTheme/hAnsiTheme theme references, hint attribute, and rStyle.
  // Includes a minimal styles.xml defining the referenced character style.
  {
    let doc = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:body>
    <w:p>
      <w:r>
        <w:rPr><w:rFonts w:ascii="Arial" w:hAnsi="Arial"/></w:rPr>
        <w:t>Arial</w:t>
      </w:r>
      <w:r>
        <w:rPr><w:rFonts w:eastAsia="SimSun"/></w:rPr>
        <w:t xml:space="preserve"> SimSun</w:t>
      </w:r>
      <w:r>
        <w:rPr><w:rFonts w:cs="Times New Roman"/></w:rPr>
        <w:t xml:space="preserve"> TimesCS</w:t>
      </w:r>
      <w:r>
        <w:rPr><w:rFonts w:asciiTheme="minorAscii" w:hAnsiTheme="minorAscii"/></w:rPr>
        <w:t xml:space="preserve"> MinorTheme</w:t>
      </w:r>
      <w:r>
        <w:rPr><w:rFonts w:hint="eastAsia" w:eastAsia="MS Mincho"/></w:rPr>
        <w:t xml:space="preserve"> Hint</w:t>
      </w:r>
      <w:r>
        <w:rPr><w:rStyle w:val="Strong"/></w:rPr>
        <w:t xml:space="preserve"> StrongStyle</w:t>
      </w:r>
    </w:p>
    <w:sectPr/>
  </w:body>
</w:document>"#;
    let styles = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:styles xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:style w:type="paragraph" w:default="1" w:styleId="Normal">
    <w:name w:val="Normal"/>
  </w:style>
  <w:style w:type="character" w:styleId="Strong">
    <w:name w:val="Strong"/>
    <w:rPr><w:b/><w:bCs/></w:rPr>
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
    save(root, "test-data/wml/run_fonts.docx", &data);
  }

  // ── WML-R-03: whitespace ─────────────────────────────────────────────────
  // Runs that probe xml:space="preserve" semantics: leading space, trailing
  // space, space-only run, internal spaces (no preserve needed), no spaces.
  {
    let doc = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:body>
    <w:p>
      <w:r><w:t xml:space="preserve"> leading</w:t></w:r>
      <w:r><w:t xml:space="preserve">trailing </w:t></w:r>
      <w:r><w:t xml:space="preserve"> </w:t></w:r>
      <w:r><w:t>word word</w:t></w:r>
      <w:r><w:t>nospace</w:t></w:r>
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
    save(root, "test-data/wml/whitespace.docx", &data);
  }

  // ── WML-R-04: breaks ─────────────────────────────────────────────────────
  // Runs with break elements: soft return (bare br), page break, tab.
  // The bare <w:br/> must not gain w:type="textWrapping" on round-trip.
  {
    let doc = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:body>
    <w:p>
      <w:r><w:t>Before soft return</w:t></w:r>
      <w:r><w:br/></w:r>
      <w:r><w:t>After soft return</w:t></w:r>
    </w:p>
    <w:p>
      <w:r><w:t>Before tab</w:t></w:r>
      <w:r><w:tab/></w:r>
      <w:r><w:t>After tab</w:t></w:r>
    </w:p>
    <w:p>
      <w:r><w:br w:type="page"/></w:r>
    </w:p>
    <w:p>
      <w:r><w:t>After page break</w:t></w:r>
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
    save(root, "test-data/wml/breaks.docx", &data);
  }
}

fn create_wml_paragraphs_fixtures(root: &Path) {
  // ── WML-P-01: para_alignment ─────────────────────────────────────────────
  // Five paragraphs covering every common jc value: left, center, right,
  // both (justified), distribute. Each is self-contained in its own <w:p>.
  {
    let doc = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:body>
    <w:p>
      <w:pPr><w:jc w:val="left"/></w:pPr>
      <w:r><w:t>Left aligned paragraph.</w:t></w:r>
    </w:p>
    <w:p>
      <w:pPr><w:jc w:val="center"/></w:pPr>
      <w:r><w:t>Center aligned paragraph.</w:t></w:r>
    </w:p>
    <w:p>
      <w:pPr><w:jc w:val="right"/></w:pPr>
      <w:r><w:t>Right aligned paragraph.</w:t></w:r>
    </w:p>
    <w:p>
      <w:pPr><w:jc w:val="both"/></w:pPr>
      <w:r><w:t>Justified paragraph with both margins aligned.</w:t></w:r>
    </w:p>
    <w:p>
      <w:pPr><w:jc w:val="distribute"/></w:pPr>
      <w:r><w:t>Distribute spacing between all characters.</w:t></w:r>
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
    save(root, "test-data/wml/para_alignment.docx", &data);
  }

  // ── WML-P-02: para_spacing ───────────────────────────────────────────────
  // Paragraphs exercising spacing before/after (twips) and the three
  // lineRule values: auto (multiple of single line), exact (fixed height),
  // atLeast (minimum height). A contextualSpacing pair is included.
  {
    let doc = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:body>
    <w:p>
      <w:pPr>
        <w:spacing w:before="240" w:after="120"/>
      </w:pPr>
      <w:r><w:t>Before 12pt after 6pt spacing (twips).</w:t></w:r>
    </w:p>
    <w:p>
      <w:pPr>
        <w:spacing w:line="240" w:lineRule="auto"/>
      </w:pPr>
      <w:r><w:t>Single line spacing (auto 240).</w:t></w:r>
    </w:p>
    <w:p>
      <w:pPr>
        <w:spacing w:line="360" w:lineRule="auto"/>
      </w:pPr>
      <w:r><w:t>One-and-a-half line spacing (auto 360).</w:t></w:r>
    </w:p>
    <w:p>
      <w:pPr>
        <w:spacing w:line="480" w:lineRule="auto"/>
      </w:pPr>
      <w:r><w:t>Double line spacing (auto 480).</w:t></w:r>
    </w:p>
    <w:p>
      <w:pPr>
        <w:spacing w:line="320" w:lineRule="exact"/>
      </w:pPr>
      <w:r><w:t>Exact 16pt line height.</w:t></w:r>
    </w:p>
    <w:p>
      <w:pPr>
        <w:spacing w:line="280" w:lineRule="atLeast"/>
      </w:pPr>
      <w:r><w:t>At-least 14pt line height.</w:t></w:r>
    </w:p>
    <w:p>
      <w:pPr>
        <w:spacing w:before="120" w:after="120"/>
        <w:contextualSpacing/>
      </w:pPr>
      <w:r><w:t>Contextual spacing first paragraph.</w:t></w:r>
    </w:p>
    <w:p>
      <w:pPr>
        <w:spacing w:before="120" w:after="120"/>
        <w:contextualSpacing/>
      </w:pPr>
      <w:r><w:t>Contextual spacing second paragraph (gap suppressed).</w:t></w:r>
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
    save(root, "test-data/wml/para_spacing.docx", &data);
  }

  // ── WML-P-03: para_indent ────────────────────────────────────────────────
  // Paragraphs with left/right indentation, firstLine, and hanging indent.
  {
    let doc = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:body>
    <w:p>
      <w:pPr>
        <w:ind w:left="720"/>
      </w:pPr>
      <w:r><w:t>Left indent 0.5 inch (720 twips).</w:t></w:r>
    </w:p>
    <w:p>
      <w:pPr>
        <w:ind w:left="1440" w:right="1440"/>
      </w:pPr>
      <w:r><w:t>Left and right indent 1 inch each.</w:t></w:r>
    </w:p>
    <w:p>
      <w:pPr>
        <w:ind w:left="720" w:firstLine="360"/>
      </w:pPr>
      <w:r><w:t>First-line indent: body at 0.5 inch, first line at 0.75 inch.</w:t></w:r>
    </w:p>
    <w:p>
      <w:pPr>
        <w:ind w:left="720" w:hanging="360"/>
      </w:pPr>
      <w:r><w:t>Hanging indent: first line at 0.25 inch, rest at 0.5 inch.</w:t></w:r>
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
    save(root, "test-data/wml/para_indent.docx", &data);
  }

  // ── WML-P-04: para_borders_shading ──────────────────────────────────────
  // Paragraphs with box borders and two shading styles:
  //   - single-line box border + clear fill (solid background)
  //   - no border + pct20 dot-fill shading pattern
  {
    let doc = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:body>
    <w:p>
      <w:pPr>
        <w:pBdr>
          <w:top    w:val="single" w:sz="6" w:space="1" w:color="4472C4"/>
          <w:left   w:val="single" w:sz="6" w:space="4" w:color="4472C4"/>
          <w:bottom w:val="single" w:sz="6" w:space="1" w:color="4472C4"/>
          <w:right  w:val="single" w:sz="6" w:space="4" w:color="4472C4"/>
        </w:pBdr>
        <w:shd w:val="clear" w:color="auto" w:fill="DEEAF1"/>
      </w:pPr>
      <w:r><w:t>Box border with solid light-blue fill.</w:t></w:r>
    </w:p>
    <w:p>
      <w:pPr>
        <w:shd w:val="pct20" w:color="FF0000" w:fill="FFFF00"/>
      </w:pPr>
      <w:r><w:t>20-percent dot pattern: red on yellow.</w:t></w:r>
    </w:p>
    <w:p>
      <w:pPr>
        <w:pBdr>
          <w:top    w:val="double" w:sz="4" w:space="1" w:color="000000"/>
          <w:bottom w:val="double" w:sz="4" w:space="1" w:color="000000"/>
        </w:pBdr>
      </w:pPr>
      <w:r><w:t>Top and bottom double-line borders only.</w:t></w:r>
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
    save(root, "test-data/wml/para_borders_shading.docx", &data);
  }

  // ── WML-P-05: para_keep ──────────────────────────────────────────────────
  // Keep/break control properties and outline level.
  // Covers keepNext, keepLines, pageBreakBefore, widowControl w:val="0",
  // and outlineLvl values 0 (H1) and 1 (H2).
  {
    let doc = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:body>
    <w:p>
      <w:pPr>
        <w:keepNext/>
        <w:outlineLvl w:val="0"/>
      </w:pPr>
      <w:r><w:rPr><w:b/></w:rPr><w:t>Heading 1 with keepNext.</w:t></w:r>
    </w:p>
    <w:p>
      <w:r><w:t>Body paragraph stays with the heading above.</w:t></w:r>
    </w:p>
    <w:p>
      <w:pPr>
        <w:keepLines/>
        <w:outlineLvl w:val="1"/>
      </w:pPr>
      <w:r><w:t>Heading 2 with keepLines.</w:t></w:r>
    </w:p>
    <w:p>
      <w:pPr>
        <w:pageBreakBefore/>
      </w:pPr>
      <w:r><w:t>This paragraph forces a page break before it.</w:t></w:r>
    </w:p>
    <w:p>
      <w:pPr>
        <w:widowControl w:val="0"/>
      </w:pPr>
      <w:r><w:t>Widow control explicitly disabled.</w:t></w:r>
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
    save(root, "test-data/wml/para_keep.docx", &data);
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
  create_opc_fixtures(&root, &png);
  create_drawingml_fixtures(&root);
  create_wml_runs_fixtures(&root);
  create_wml_paragraphs_fixtures(&root);
  create_wml_styles_fixtures(&root);
  create_wml_numbering_fixtures(&root);
  create_wml_tables_fixtures(&root);
  create_wml_drawing_fixtures(&root, &png);
  create_wml_headers_fixtures(&root);
}

fn create_wml_headers_fixtures(root: &Path) {
  // ── WML-H-01: header_footer ──────────────────────────────────────────────
  // Default header + default footer; US Letter page size; 1-inch margins;
  // xmlns:r on document element; sectPr with headerReference/footerReference.
  {
    let header1 = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:hdr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:p>
    <w:pPr><w:jc w:val="center"/></w:pPr>
    <w:r><w:t>Page Header</w:t></w:r>
  </w:p>
</w:hdr>"#;
    let footer1 = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:ftr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:p>
    <w:pPr><w:jc w:val="center"/></w:pPr>
    <w:r><w:t>Page Footer</w:t></w:r>
  </w:p>
</w:ftr>"#;
    let doc = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"
            xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
  <w:body>
    <w:p>
      <w:r><w:t>Document body text.</w:t></w:r>
    </w:p>
    <w:sectPr>
      <w:headerReference w:type="default" r:id="rId2"/>
      <w:footerReference w:type="default" r:id="rId3"/>
      <w:pgSz w:w="12240" w:h="15840"/>
      <w:pgMar w:top="1440" w:right="1440" w:bottom="1440"
               w:left="1440" w:header="720" w:footer="720" w:gutter="0"/>
    </w:sectPr>
  </w:body>
</w:document>"#;
    let doc_rels = docx_doc_rels(
      r#"
  <Relationship Id="rId2" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/header" Target="header1.xml"/>
  <Relationship Id="rId3" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/footer" Target="footer1.xml"/>"#,
    );
    let content_types = docx_content_types(
      r#"
  <Override PartName="/word/header1.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.header+xml"/>
  <Override PartName="/word/footer1.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.footer+xml"/>"#,
      "",
    );
    let data = make_package(&[
      ("[Content_Types].xml", &content_types),
      ("_rels/.rels", &root_rels("word/document.xml")),
      ("word/document.xml", doc),
      ("word/_rels/document.xml.rels", &doc_rels),
      ("word/header1.xml", header1),
      ("word/footer1.xml", footer1),
    ]);
    save(root, "test-data/wml/header_footer.docx", &data);
  }

  // ── WML-H-02: header_first_page ──────────────────────────────────────────
  // Default header + first-page header + default footer; titlePg; three
  // part relationships; xmlns:r on document element.
  {
    let header1 = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:hdr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:p>
    <w:pPr><w:jc w:val="right"/></w:pPr>
    <w:r><w:t>Default Header (odd pages)</w:t></w:r>
  </w:p>
</w:hdr>"#;
    let header2 = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:hdr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:p>
    <w:pPr><w:jc w:val="center"/></w:pPr>
    <w:r><w:t>First Page Header</w:t></w:r>
  </w:p>
</w:hdr>"#;
    let footer1 = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:ftr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:p>
    <w:pPr><w:jc w:val="center"/></w:pPr>
    <w:r><w:t>Default Footer</w:t></w:r>
  </w:p>
</w:ftr>"#;
    let doc = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"
            xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
  <w:body>
    <w:p>
      <w:r><w:t>First page body text.</w:t></w:r>
    </w:p>
    <w:p>
      <w:r><w:t>Second page body text.</w:t></w:r>
    </w:p>
    <w:sectPr>
      <w:headerReference w:type="default" r:id="rId2"/>
      <w:headerReference w:type="first" r:id="rId3"/>
      <w:footerReference w:type="default" r:id="rId4"/>
      <w:titlePg/>
      <w:pgSz w:w="12240" w:h="15840"/>
      <w:pgMar w:top="1440" w:right="1440" w:bottom="1440"
               w:left="1440" w:header="720" w:footer="720" w:gutter="0"/>
    </w:sectPr>
  </w:body>
</w:document>"#;
    let doc_rels = docx_doc_rels(
      r#"
  <Relationship Id="rId2" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/header" Target="header1.xml"/>
  <Relationship Id="rId3" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/header" Target="header2.xml"/>
  <Relationship Id="rId4" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/footer" Target="footer1.xml"/>"#,
    );
    let content_types = docx_content_types(
      r#"
  <Override PartName="/word/header1.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.header+xml"/>
  <Override PartName="/word/header2.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.header+xml"/>
  <Override PartName="/word/footer1.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.footer+xml"/>"#,
      "",
    );
    let data = make_package(&[
      ("[Content_Types].xml", &content_types),
      ("_rels/.rels", &root_rels("word/document.xml")),
      ("word/document.xml", doc),
      ("word/_rels/document.xml.rels", &doc_rels),
      ("word/header1.xml", header1),
      ("word/header2.xml", header2),
      ("word/footer1.xml", footer1),
    ]);
    save(root, "test-data/wml/header_first_page.docx", &data);
  }
}

fn create_wml_tables_fixtures(root: &Path) {
  // ── WML-T-01: table_borders ──────────────────────────────────────────────
  // 3×2 table with full tblBorders (outer + insideH/insideV), one cell with
  // a tcBorders override (dashed right), and one cell with shd fill.
  {
    let doc = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:body>
    <w:tbl>
      <w:tblPr>
        <w:tblW w:w="8640" w:type="dxa"/>
        <w:tblBorders>
          <w:top    w:val="single" w:sz="4" w:space="0" w:color="000000"/>
          <w:left   w:val="single" w:sz="4" w:space="0" w:color="000000"/>
          <w:bottom w:val="single" w:sz="4" w:space="0" w:color="000000"/>
          <w:right  w:val="single" w:sz="4" w:space="0" w:color="000000"/>
          <w:insideH w:val="single" w:sz="4" w:space="0" w:color="4472C4"/>
          <w:insideV w:val="single" w:sz="4" w:space="0" w:color="4472C4"/>
        </w:tblBorders>
      </w:tblPr>
      <w:tblGrid>
        <w:gridCol w:w="2880"/>
        <w:gridCol w:w="2880"/>
        <w:gridCol w:w="2880"/>
      </w:tblGrid>
      <w:tr>
        <w:tc>
          <w:tcPr><w:tcW w:w="2880" w:type="dxa"/></w:tcPr>
          <w:p><w:r><w:t>A1</w:t></w:r></w:p>
        </w:tc>
        <w:tc>
          <w:tcPr>
            <w:tcW w:w="2880" w:type="dxa"/>
            <w:tcBorders>
              <w:right w:val="dashed" w:sz="8" w:space="0" w:color="FF0000"/>
            </w:tcBorders>
          </w:tcPr>
          <w:p><w:r><w:t>A2 (dashed right border)</w:t></w:r></w:p>
        </w:tc>
        <w:tc>
          <w:tcPr>
            <w:tcW w:w="2880" w:type="dxa"/>
            <w:shd w:val="clear" w:color="auto" w:fill="DEEAF1"/>
          </w:tcPr>
          <w:p><w:r><w:t>A3 (shaded cell)</w:t></w:r></w:p>
        </w:tc>
      </w:tr>
      <w:tr>
        <w:tc>
          <w:tcPr><w:tcW w:w="2880" w:type="dxa"/></w:tcPr>
          <w:p><w:r><w:t>B1</w:t></w:r></w:p>
        </w:tc>
        <w:tc>
          <w:tcPr><w:tcW w:w="2880" w:type="dxa"/></w:tcPr>
          <w:p><w:r><w:t>B2</w:t></w:r></w:p>
        </w:tc>
        <w:tc>
          <w:tcPr><w:tcW w:w="2880" w:type="dxa"/></w:tcPr>
          <w:p><w:r><w:t>B3</w:t></w:r></w:p>
        </w:tc>
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
    save(root, "test-data/wml/table_borders.docx", &data);
  }

  // ── WML-T-02: table_merged ───────────────────────────────────────────────
  // 3×3 table demonstrating both merge directions:
  //   Row 0, cells 0-1: horizontal merge via gridSpan=2
  //   Column 2, rows 0-1: vertical merge via vMerge restart/continue
  {
    let doc = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:body>
    <w:tbl>
      <w:tblPr>
        <w:tblW w:w="0" w:type="auto"/>
        <w:tblBorders>
          <w:top    w:val="single" w:sz="4" w:space="0" w:color="000000"/>
          <w:left   w:val="single" w:sz="4" w:space="0" w:color="000000"/>
          <w:bottom w:val="single" w:sz="4" w:space="0" w:color="000000"/>
          <w:right  w:val="single" w:sz="4" w:space="0" w:color="000000"/>
          <w:insideH w:val="single" w:sz="4" w:space="0" w:color="000000"/>
          <w:insideV w:val="single" w:sz="4" w:space="0" w:color="000000"/>
        </w:tblBorders>
      </w:tblPr>
      <w:tblGrid>
        <w:gridCol w:w="2880"/>
        <w:gridCol w:w="2880"/>
        <w:gridCol w:w="2880"/>
      </w:tblGrid>
      <w:tr>
        <w:tc>
          <w:tcPr>
            <w:tcW w:w="5760" w:type="dxa"/>
            <w:gridSpan w:val="2"/>
          </w:tcPr>
          <w:p><w:r><w:t>A1+A2 (horizontal merge, gridSpan=2)</w:t></w:r></w:p>
        </w:tc>
        <w:tc>
          <w:tcPr>
            <w:tcW w:w="2880" w:type="dxa"/>
            <w:vMerge w:val="restart"/>
          </w:tcPr>
          <w:p><w:r><w:t>A3 top of vertical merge</w:t></w:r></w:p>
        </w:tc>
      </w:tr>
      <w:tr>
        <w:tc>
          <w:tcPr><w:tcW w:w="2880" w:type="dxa"/></w:tcPr>
          <w:p><w:r><w:t>B1</w:t></w:r></w:p>
        </w:tc>
        <w:tc>
          <w:tcPr><w:tcW w:w="2880" w:type="dxa"/></w:tcPr>
          <w:p><w:r><w:t>B2</w:t></w:r></w:p>
        </w:tc>
        <w:tc>
          <w:tcPr>
            <w:tcW w:w="2880" w:type="dxa"/>
            <w:vMerge/>
          </w:tcPr>
          <w:p/>
        </w:tc>
      </w:tr>
      <w:tr>
        <w:tc>
          <w:tcPr><w:tcW w:w="2880" w:type="dxa"/></w:tcPr>
          <w:p><w:r><w:t>C1</w:t></w:r></w:p>
        </w:tc>
        <w:tc>
          <w:tcPr><w:tcW w:w="2880" w:type="dxa"/></w:tcPr>
          <w:p><w:r><w:t>C2</w:t></w:r></w:p>
        </w:tc>
        <w:tc>
          <w:tcPr><w:tcW w:w="2880" w:type="dxa"/></w:tcPr>
          <w:p><w:r><w:t>C3 (below vertical merge)</w:t></w:r></w:p>
        </w:tc>
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
    save(root, "test-data/wml/table_merged.docx", &data);
  }

  // ── WML-T-03: table_props ────────────────────────────────────────────────
  // 2×3 table exercising row and cell property features:
  //   Row 0: tblHeader + trHeight exact 480 twips
  //   Column 0 cells: vAlign=top/center/bottom across rows
  //   One cell: noWrap
  //   Table width: 5000 pct (100%)
  {
    let doc = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:body>
    <w:tbl>
      <w:tblPr>
        <w:tblW w:w="5000" w:type="pct"/>
      </w:tblPr>
      <w:tblGrid>
        <w:gridCol w:w="4320"/>
        <w:gridCol w:w="4320"/>
      </w:tblGrid>
      <w:tr>
        <w:trPr>
          <w:tblHeader/>
          <w:trHeight w:val="480" w:hRule="exact"/>
        </w:trPr>
        <w:tc>
          <w:tcPr>
            <w:tcW w:w="4320" w:type="dxa"/>
            <w:vAlign w:val="center"/>
          </w:tcPr>
          <w:p><w:r><w:rPr><w:b/></w:rPr><w:t>Header Left (center valign)</w:t></w:r></w:p>
        </w:tc>
        <w:tc>
          <w:tcPr>
            <w:tcW w:w="4320" w:type="dxa"/>
            <w:vAlign w:val="center"/>
          </w:tcPr>
          <w:p><w:r><w:rPr><w:b/></w:rPr><w:t>Header Right (center valign)</w:t></w:r></w:p>
        </w:tc>
      </w:tr>
      <w:tr>
        <w:tc>
          <w:tcPr>
            <w:tcW w:w="4320" w:type="dxa"/>
            <w:vAlign w:val="top"/>
          </w:tcPr>
          <w:p><w:r><w:t>Top-aligned cell.</w:t></w:r></w:p>
        </w:tc>
        <w:tc>
          <w:tcPr>
            <w:tcW w:w="4320" w:type="dxa"/>
            <w:noWrap/>
          </w:tcPr>
          <w:p><w:r><w:t>No-wrap cell content that stays on one line.</w:t></w:r></w:p>
        </w:tc>
      </w:tr>
      <w:tr>
        <w:trPr><w:cantSplit/></w:trPr>
        <w:tc>
          <w:tcPr>
            <w:tcW w:w="4320" w:type="dxa"/>
            <w:vAlign w:val="bottom"/>
          </w:tcPr>
          <w:p><w:r><w:t>Bottom-aligned cell.</w:t></w:r></w:p>
        </w:tc>
        <w:tc>
          <w:tcPr><w:tcW w:w="4320" w:type="dxa"/></w:tcPr>
          <w:p><w:r><w:t>CantSplit row.</w:t></w:r></w:p>
        </w:tc>
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
    save(root, "test-data/wml/table_props.docx", &data);
  }
}

fn create_wml_drawing_fixtures(root: &Path, png: &[u8]) {
  let img_ct = "\n  <Default Extension=\"png\" ContentType=\"image/png\"/>";
  let img_rel = r#"
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/image" Target="media/image1.png"/>"#;

  // ── WML-I-01: image_inline_props ─────────────────────────────────────────
  // Inline image at 1 inch × 1 inch (914400 × 914400 EMU).
  // Exercises: non-zero distL/distR (114300 EMU = 0.125 in), altText on
  // docPr and cNvPr, cstate="print" on a:blip, picLocks noChangeAspect.
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
          <wp:inline distT="0" distB="0" distL="114300" distR="114300">
            <wp:extent cx="914400" cy="914400"/>
            <wp:effectExtent l="0" t="0" r="0" b="0"/>
            <wp:docPr id="1" name="Image 1" descr="Alt text for accessibility"/>
            <wp:cNvGraphicFramePr>
              <a:graphicFrameLocks noChangeAspect="1"/>
            </wp:cNvGraphicFramePr>
            <a:graphic>
              <a:graphicData uri="http://schemas.openxmlformats.org/drawingml/2006/picture">
                <pic:pic>
                  <pic:nvPicPr>
                    <pic:cNvPr id="0" name="image1.png" descr="Alt text for accessibility"/>
                    <pic:cNvPicPr>
                      <a:picLocks noChangeAspect="1"/>
                    </pic:cNvPicPr>
                  </pic:nvPicPr>
                  <pic:blipFill>
                    <a:blip r:embed="rId1" cstate="print"/>
                    <a:stretch><a:fillRect/></a:stretch>
                  </pic:blipFill>
                  <pic:spPr>
                    <a:xfrm>
                      <a:off x="0" y="0"/>
                      <a:ext cx="914400" cy="914400"/>
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
    let data = make_package(&[
      ("[Content_Types].xml", &docx_content_types("", img_ct)),
      ("_rels/.rels", &root_rels("word/document.xml")),
      ("word/document.xml", doc),
      ("word/_rels/document.xml.rels", &docx_doc_rels(img_rel)),
      ("word/media/image1.png", png),
    ]);
    save(root, "test-data/wml/image_inline_props.docx", &data);
  }

  // ── WML-I-02: image_floating ─────────────────────────────────────────────
  // Floating anchor at 1 inch × 1 inch.
  // Exercises: wp:anchor with wrapSquare (bothSides), column-relative
  // horizontal alignment (left), paragraph-relative vertical posOffset=0,
  // explicit distT/distB/distL/distR on both anchor and wrapSquare.
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
          <wp:anchor relativeHeight="251658240" behindDoc="0" locked="0"
                     layoutInCell="1" allowOverlap="1"
                     distT="114300" distB="114300" distL="114300" distR="114300">
            <wp:simplePos x="0" y="0"/>
            <wp:positionH relativeFrom="column">
              <wp:align>left</wp:align>
            </wp:positionH>
            <wp:positionV relativeFrom="paragraph">
              <wp:posOffset>0</wp:posOffset>
            </wp:positionV>
            <wp:extent cx="914400" cy="914400"/>
            <wp:effectExtent l="0" t="0" r="0" b="0"/>
            <wp:wrapSquare wrapText="bothSides" distT="114300" distB="114300"
                           distL="114300" distR="114300"/>
            <wp:docPr id="2" name="Image 2"/>
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
                      <a:ext cx="914400" cy="914400"/>
                    </a:xfrm>
                    <a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
                  </pic:spPr>
                </pic:pic>
              </a:graphicData>
            </a:graphic>
          </wp:anchor>
        </w:drawing>
      </w:r>
      <w:r><w:t xml:space="preserve">Text beside the floating image.</w:t></w:r>
    </w:p>
    <w:sectPr/>
  </w:body>
</w:document>"#;
    let data = make_package(&[
      ("[Content_Types].xml", &docx_content_types("", img_ct)),
      ("_rels/.rels", &root_rels("word/document.xml")),
      ("word/document.xml", doc),
      ("word/_rels/document.xml.rels", &docx_doc_rels(img_rel)),
      ("word/media/image1.png", png),
    ]);
    save(root, "test-data/wml/image_floating.docx", &data);
  }
}

fn create_wml_numbering_fixtures(root: &Path) {
  let num_ct = r#"
  <Override PartName="/word/numbering.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.numbering+xml"/>"#;
  let num_rel = r#"
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/numbering" Target="numbering.xml"/>"#;

  // ── WML-N-01: numbering_bullets ──────────────────────────────────────────
  // Single-level bullet list using numFmt=bullet with the Unicode bullet
  // character (U+2022). Hanging indent 360 twips creates the standard
  // 0.25-inch label space. Five items at level 0.
  {
    let numbering = "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n\
<w:numbering xmlns:w=\"http://schemas.openxmlformats.org/wordprocessingml/2006/main\">\n\
  <w:abstractNum w:abstractNumId=\"0\">\n\
    <w:multiLevelType w:val=\"singleLevel\"/>\n\
    <w:lvl w:ilvl=\"0\">\n\
      <w:start w:val=\"1\"/>\n\
      <w:numFmt w:val=\"bullet\"/>\n\
      <w:lvlText w:val=\"\u{2022}\"/>\n\
      <w:lvlJc w:val=\"left\"/>\n\
      <w:pPr><w:ind w:left=\"720\" w:hanging=\"360\"/></w:pPr>\n\
      <w:rPr><w:rFonts w:ascii=\"Arial\" w:hAnsi=\"Arial\"/></w:rPr>\n\
    </w:lvl>\n\
  </w:abstractNum>\n\
  <w:num w:numId=\"1\">\n\
    <w:abstractNumId w:val=\"0\"/>\n\
  </w:num>\n\
</w:numbering>";
    let doc = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:body>
    <w:p>
      <w:pPr><w:numPr><w:ilvl w:val="0"/><w:numId w:val="1"/></w:numPr></w:pPr>
      <w:r><w:t>First bullet item.</w:t></w:r>
    </w:p>
    <w:p>
      <w:pPr><w:numPr><w:ilvl w:val="0"/><w:numId w:val="1"/></w:numPr></w:pPr>
      <w:r><w:t>Second bullet item.</w:t></w:r>
    </w:p>
    <w:p>
      <w:pPr><w:numPr><w:ilvl w:val="0"/><w:numId w:val="1"/></w:numPr></w:pPr>
      <w:r><w:t>Third bullet item.</w:t></w:r>
    </w:p>
    <w:p>
      <w:pPr><w:numPr><w:ilvl w:val="0"/><w:numId w:val="1"/></w:numPr></w:pPr>
      <w:r><w:t>Fourth bullet item.</w:t></w:r>
    </w:p>
    <w:p>
      <w:pPr><w:numPr><w:ilvl w:val="0"/><w:numId w:val="1"/></w:numPr></w:pPr>
      <w:r><w:t>Fifth bullet item.</w:t></w:r>
    </w:p>
    <w:sectPr/>
  </w:body>
</w:document>"#;
    let data = make_package(&[
      ("[Content_Types].xml", &docx_content_types(num_ct, "")),
      ("_rels/.rels", &root_rels("word/document.xml")),
      ("word/document.xml", doc),
      ("word/_rels/document.xml.rels", &docx_doc_rels(num_rel)),
      ("word/numbering.xml", numbering.as_bytes()),
    ]);
    save(root, "test-data/wml/numbering_bullets.docx", &data);
  }

  // ── WML-N-02: numbering_ordered ──────────────────────────────────────────
  // Three-level multilevel list: decimal L0 ("%1."), lowerLetter L1 ("%2."),
  // lowerRoman L2 ("%3."). Document has top-level items with nested sub-items
  // demonstrating all three levels.
  {
    let numbering = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:numbering xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:abstractNum w:abstractNumId="0">
    <w:multiLevelType w:val="multilevel"/>
    <w:lvl w:ilvl="0">
      <w:start w:val="1"/>
      <w:numFmt w:val="decimal"/>
      <w:lvlText w:val="%1."/>
      <w:lvlJc w:val="left"/>
      <w:pPr><w:ind w:left="720" w:hanging="360"/></w:pPr>
    </w:lvl>
    <w:lvl w:ilvl="1">
      <w:start w:val="1"/>
      <w:numFmt w:val="lowerLetter"/>
      <w:lvlText w:val="%2."/>
      <w:lvlJc w:val="left"/>
      <w:pPr><w:ind w:left="1440" w:hanging="360"/></w:pPr>
    </w:lvl>
    <w:lvl w:ilvl="2">
      <w:start w:val="1"/>
      <w:numFmt w:val="lowerRoman"/>
      <w:lvlText w:val="%3."/>
      <w:lvlJc w:val="left"/>
      <w:pPr><w:ind w:left="2160" w:hanging="360"/></w:pPr>
    </w:lvl>
  </w:abstractNum>
  <w:num w:numId="1">
    <w:abstractNumId w:val="0"/>
  </w:num>
</w:numbering>"#;
    let doc = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:body>
    <w:p>
      <w:pPr><w:numPr><w:ilvl w:val="0"/><w:numId w:val="1"/></w:numPr></w:pPr>
      <w:r><w:t>First top-level item (1.).</w:t></w:r>
    </w:p>
    <w:p>
      <w:pPr><w:numPr><w:ilvl w:val="1"/><w:numId w:val="1"/></w:numPr></w:pPr>
      <w:r><w:t>Sub-item under first (a.).</w:t></w:r>
    </w:p>
    <w:p>
      <w:pPr><w:numPr><w:ilvl w:val="2"/><w:numId w:val="1"/></w:numPr></w:pPr>
      <w:r><w:t>Sub-sub-item (i.).</w:t></w:r>
    </w:p>
    <w:p>
      <w:pPr><w:numPr><w:ilvl w:val="1"/><w:numId w:val="1"/></w:numPr></w:pPr>
      <w:r><w:t>Second sub-item (b.).</w:t></w:r>
    </w:p>
    <w:p>
      <w:pPr><w:numPr><w:ilvl w:val="0"/><w:numId w:val="1"/></w:numPr></w:pPr>
      <w:r><w:t>Second top-level item (2.).</w:t></w:r>
    </w:p>
    <w:p>
      <w:pPr><w:numPr><w:ilvl w:val="0"/><w:numId w:val="1"/></w:numPr></w:pPr>
      <w:r><w:t>Third top-level item (3.).</w:t></w:r>
    </w:p>
    <w:sectPr/>
  </w:body>
</w:document>"#;
    let data = make_package(&[
      ("[Content_Types].xml", &docx_content_types(num_ct, "")),
      ("_rels/.rels", &root_rels("word/document.xml")),
      ("word/document.xml", doc),
      ("word/_rels/document.xml.rels", &docx_doc_rels(num_rel)),
      ("word/numbering.xml", numbering),
    ]);
    save(root, "test-data/wml/numbering_ordered.docx", &data);
  }

  // ── WML-N-03: numbering_restart ──────────────────────────────────────────
  // Two independent decimal lists sharing the same abstractNum (id=0).
  // numId=1 is the first list (counts 1, 2, 3).
  // numId=2 references the same abstractNum but has a lvlOverride with
  // startOverride val="1" so it restarts independently at 1.
  {
    let numbering = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:numbering xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:abstractNum w:abstractNumId="0">
    <w:multiLevelType w:val="singleLevel"/>
    <w:lvl w:ilvl="0">
      <w:start w:val="1"/>
      <w:numFmt w:val="decimal"/>
      <w:lvlText w:val="%1."/>
      <w:lvlJc w:val="left"/>
      <w:pPr><w:ind w:left="720" w:hanging="360"/></w:pPr>
    </w:lvl>
  </w:abstractNum>
  <w:num w:numId="1">
    <w:abstractNumId w:val="0"/>
  </w:num>
  <w:num w:numId="2">
    <w:abstractNumId w:val="0"/>
    <w:lvlOverride w:ilvl="0">
      <w:startOverride w:val="1"/>
    </w:lvlOverride>
  </w:num>
</w:numbering>"#;
    let doc = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:body>
    <w:p><w:r><w:t>First list:</w:t></w:r></w:p>
    <w:p>
      <w:pPr><w:numPr><w:ilvl w:val="0"/><w:numId w:val="1"/></w:numPr></w:pPr>
      <w:r><w:t>Item one (1.).</w:t></w:r>
    </w:p>
    <w:p>
      <w:pPr><w:numPr><w:ilvl w:val="0"/><w:numId w:val="1"/></w:numPr></w:pPr>
      <w:r><w:t>Item two (2.).</w:t></w:r>
    </w:p>
    <w:p>
      <w:pPr><w:numPr><w:ilvl w:val="0"/><w:numId w:val="1"/></w:numPr></w:pPr>
      <w:r><w:t>Item three (3.).</w:t></w:r>
    </w:p>
    <w:p><w:r><w:t>Second list (restarts at 1 via lvlOverride):</w:t></w:r></w:p>
    <w:p>
      <w:pPr><w:numPr><w:ilvl w:val="0"/><w:numId w:val="2"/></w:numPr></w:pPr>
      <w:r><w:t>Item one (1.).</w:t></w:r>
    </w:p>
    <w:p>
      <w:pPr><w:numPr><w:ilvl w:val="0"/><w:numId w:val="2"/></w:numPr></w:pPr>
      <w:r><w:t>Item two (2.).</w:t></w:r>
    </w:p>
    <w:p>
      <w:pPr><w:numPr><w:ilvl w:val="0"/><w:numId w:val="2"/></w:numPr></w:pPr>
      <w:r><w:t>Item three (3.).</w:t></w:r>
    </w:p>
    <w:sectPr/>
  </w:body>
</w:document>"#;
    let data = make_package(&[
      ("[Content_Types].xml", &docx_content_types(num_ct, "")),
      ("_rels/.rels", &root_rels("word/document.xml")),
      ("word/document.xml", doc),
      ("word/_rels/document.xml.rels", &docx_doc_rels(num_rel)),
      ("word/numbering.xml", numbering),
    ]);
    save(root, "test-data/wml/numbering_restart.docx", &data);
  }
}

fn create_wml_styles_fixtures(root: &Path) {
  // ── WML-S-01: style_inheritance ──────────────────────────────────────────
  // Three-level basedOn chain: Normal (default) → BodyText → BodyIndent.
  // The document uses all three styles plus docDefaults (Calibri 11pt,
  // spacing after 160 twips). Verifies that the inheritance chain and
  // docDefaults round-trip without dropping or reordering elements.
  {
    let styles = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:styles xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:docDefaults>
    <w:rPrDefault>
      <w:rPr>
        <w:rFonts w:ascii="Calibri" w:hAnsi="Calibri"/>
        <w:sz w:val="22"/>
        <w:szCs w:val="22"/>
      </w:rPr>
    </w:rPrDefault>
    <w:pPrDefault>
      <w:pPr>
        <w:spacing w:after="160" w:line="259" w:lineRule="auto"/>
      </w:pPr>
    </w:pPrDefault>
  </w:docDefaults>
  <w:style w:type="paragraph" w:default="1" w:styleId="Normal">
    <w:name w:val="Normal"/>
  </w:style>
  <w:style w:type="paragraph" w:styleId="BodyText">
    <w:name w:val="Body Text"/>
    <w:basedOn w:val="Normal"/>
    <w:next w:val="Normal"/>
    <w:rPr><w:sz w:val="24"/><w:szCs w:val="24"/></w:rPr>
  </w:style>
  <w:style w:type="paragraph" w:styleId="BodyIndent">
    <w:name w:val="Body Indent"/>
    <w:basedOn w:val="BodyText"/>
    <w:pPr><w:ind w:left="720"/></w:pPr>
  </w:style>
</w:styles>"#;
    let doc = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:body>
    <w:p>
      <w:r><w:t>No explicit style (Normal via default).</w:t></w:r>
    </w:p>
    <w:p>
      <w:pPr><w:pStyle w:val="BodyText"/></w:pPr>
      <w:r><w:t>Body Text style (basedOn Normal).</w:t></w:r>
    </w:p>
    <w:p>
      <w:pPr><w:pStyle w:val="BodyIndent"/></w:pPr>
      <w:r><w:t>Body Indent style (basedOn BodyText, which basedOn Normal).</w:t></w:r>
    </w:p>
    <w:sectPr/>
  </w:body>
</w:document>"#;
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
    save(root, "test-data/wml/style_inheritance.docx", &data);
  }

  // ── WML-S-02: style_linked ───────────────────────────────────────────────
  // Linked paragraph + character style pair (Quote ↔ QuoteChar).
  // The Quote paragraph style has a `next` pointing back to Normal.
  // The document uses Quote via pStyle and QuoteChar via rStyle.
  {
    let styles = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:styles xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:style w:type="paragraph" w:default="1" w:styleId="Normal">
    <w:name w:val="Normal"/>
  </w:style>
  <w:style w:type="character" w:styleId="DefaultParagraphFont">
    <w:name w:val="Default Paragraph Font"/>
    <w:uiPriority w:val="1"/>
    <w:semiHidden/>
    <w:unhideWhenUsed/>
  </w:style>
  <w:style w:type="paragraph" w:styleId="Quote">
    <w:name w:val="Quote"/>
    <w:basedOn w:val="Normal"/>
    <w:next w:val="Normal"/>
    <w:link w:val="QuoteChar"/>
    <w:uiPriority w:val="29"/>
    <w:qFormat/>
    <w:pPr><w:jc w:val="center"/><w:ind w:left="720" w:right="720"/></w:pPr>
    <w:rPr><w:i/><w:iCs/></w:rPr>
  </w:style>
  <w:style w:type="character" w:styleId="QuoteChar">
    <w:name w:val="Quote Char"/>
    <w:basedOn w:val="DefaultParagraphFont"/>
    <w:link w:val="Quote"/>
    <w:uiPriority w:val="29"/>
    <w:semiHidden/>
    <w:rPr><w:i/><w:iCs/></w:rPr>
  </w:style>
</w:styles>"#;
    let doc = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:body>
    <w:p>
      <w:r><w:t>Normal paragraph before the quote.</w:t></w:r>
    </w:p>
    <w:p>
      <w:pPr><w:pStyle w:val="Quote"/></w:pPr>
      <w:r><w:t>Full paragraph styled with Quote (pStyle).</w:t></w:r>
    </w:p>
    <w:p>
      <w:r><w:t xml:space="preserve">Inline </w:t></w:r>
      <w:r>
        <w:rPr><w:rStyle w:val="QuoteChar"/></w:rPr>
        <w:t>character-styled word</w:t>
      </w:r>
      <w:r><w:t xml:space="preserve"> in a normal paragraph.</w:t></w:r>
    </w:p>
    <w:sectPr/>
  </w:body>
</w:document>"#;
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
    save(root, "test-data/wml/style_linked.docx", &data);
  }
}
