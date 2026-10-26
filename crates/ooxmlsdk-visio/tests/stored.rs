use std::io::Cursor;

use ooxmlsdk::parts::visio_master_part::VisioMasterPart;
use ooxmlsdk::parts::visio_masters_part::VisioMastersPart;
use ooxmlsdk::parts::visio_page_part::VisioPagePart;
use ooxmlsdk::parts::visio_pages_part::VisioPagesPart;
use ooxmlsdk::schemas::schemas_microsoft_com_office_visio_2012_main::{
  MasterContents, PageContents, Shapes, VisioDocument,
};
use ooxmlsdk::sdk::{SdkPart, SdkType};
use ooxmlsdk_visio::{
  Ids, MasterShapes, RowAddress, SectionAddress, StyleSlot, add_color, add_face_name, add_master,
  add_style, append_shape, append_tab, effective_shape_cell, minimal_drawing,
  read_document_direct_cell, read_document_section_cell, read_section_cell,
  read_style_section_cell, set_master_instance, shape_by_id, shape_by_id_mut, styled_shape_cell,
  write_actions, write_connection, write_document_direct_cell, write_document_section_cell,
  write_style_section_cell, write_user,
};

const PAGES: &str = "http://schemas.microsoft.com/visio/2010/relationships/pages";
const PAGE: &str = "http://schemas.microsoft.com/visio/2010/relationships/page";
const MASTERS: &str = "http://schemas.microsoft.com/visio/2010/relationships/masters";
const MASTER: &str = "http://schemas.microsoft.com/visio/2010/relationships/master";

fn part_bytes(part: &impl SdkPart, package: &impl ooxmlsdk::sdk::SdkPackage) -> Vec<u8> {
  let mut bytes = Vec::new();
  part.write_data_to(package, &mut bytes).unwrap();
  bytes
}

#[test]
fn master_value_fills_a_missing_instance_cell() {
  let mut ids = Ids::new();
  let mut package = minimal_drawing(&mut ids).unwrap();
  let master_id = add_master(&mut package, &mut ids, Some("Box")).unwrap();

  let document = package.visio_document_part().unwrap();
  let masters_part = document
    .child_part_by_relationship_type::<_, VisioMastersPart>(&package, MASTERS)
    .unwrap();
  let master_part = masters_part
    .child_part_by_relationship_type::<_, VisioMasterPart>(&package, MASTER)
    .unwrap();
  let mut master_contents = MasterContents::default();
  let shapes = master_contents.shapes.get_or_insert_with(Shapes::default);
  let master_shape_id = append_shape(shapes, &mut ids);
  write_user(
    shape_by_id_mut(shapes, master_shape_id).unwrap(),
    "Cost",
    &[("Value", "12"), ("Prompt", "ask")],
  );
  master_part
    .set_data(&mut package, master_contents.to_xml().unwrap().into_bytes())
    .unwrap();

  let pages_part = document
    .child_part_by_relationship_type::<_, VisioPagesPart>(&package, PAGES)
    .unwrap();
  let page_parts: Vec<_> = pages_part
    .child_related_parts_by_relationship_type::<_, VisioPagePart>(&package, PAGE)
    .map(|related| related.part().clone())
    .collect();
  let page_part = page_parts
    .into_iter()
    .find(|part| {
      PageContents::from_bytes(&part_bytes(part, &package))
        .ok()
        .and_then(|contents| contents.shapes)
        .is_some()
    })
    .unwrap();
  let mut page = PageContents::from_bytes(&part_bytes(&page_part, &package)).unwrap();
  let shapes = page.shapes.as_mut().unwrap();
  let instance_id = append_shape(shapes, &mut ids);
  set_master_instance(
    shape_by_id_mut(shapes, instance_id).unwrap(),
    master_id,
    master_shape_id,
  );
  page_part
    .set_data(&mut package, page.to_xml().unwrap().into_bytes())
    .unwrap();

  let saved = package.to_package_bytes().unwrap();
  let opened = ooxmlsdk::parts::visio_document::VisioDocument::new(Cursor::new(saved)).unwrap();
  let document = opened.visio_document_part().unwrap();
  let masters_part = document
    .child_part_by_relationship_type::<_, VisioMastersPart>(&opened, MASTERS)
    .unwrap();
  let master_part = masters_part
    .child_part_by_relationship_type::<_, VisioMasterPart>(&opened, MASTER)
    .unwrap();
  let master_contents = MasterContents::from_bytes(&part_bytes(&master_part, &opened)).unwrap();
  let pages_part = document
    .child_part_by_relationship_type::<_, VisioPagesPart>(&opened, PAGES)
    .unwrap();
  let page_parts: Vec<_> = pages_part
    .child_related_parts_by_relationship_type::<_, VisioPagePart>(&opened, PAGE)
    .map(|related| related.part().clone())
    .collect();
  let page = page_parts
    .iter()
    .find_map(|part| {
      let contents = PageContents::from_bytes(&part_bytes(part, &opened)).unwrap();
      contents
        .shapes
        .as_ref()?
        .shape
        .iter()
        .any(|shape| shape.id == instance_id)
        .then_some(contents)
    })
    .unwrap();
  let instance = shape_by_id(page.shapes.as_ref().unwrap(), instance_id).unwrap();
  let masters = [MasterShapes {
    id: master_id,
    shapes: master_contents.shapes.as_ref().unwrap(),
  }];
  let value = effective_shape_cell(instance, "User", "Value", &masters)
    .unwrap()
    .value;
  assert_eq!(value, Some("12"));
}

#[test]
fn sections_styles_document_fonts_and_colors_round_trip() {
  let mut shape = ooxmlsdk::schemas::schemas_microsoft_com_office_visio_2012_main::Shape {
    id: 1,
    ..Default::default()
  };
  write_user(&mut shape, "Cost", &[("Value", "12")]);
  let action_ix = write_actions(&mut shape, "Go", &[("Menu", "Go"), ("Action", "1")]);
  let connection_ix = write_connection(&mut shape, "C1", &[("X", "0"), ("Y", "1")]);
  let tab_ix = append_tab(&mut shape, "0.5", "1");
  let page = PageContents::from_bytes(
    PageContents {
      shapes: Some(Shapes { shape: vec![shape] }),
      ..Default::default()
    }
    .to_xml()
    .unwrap()
    .as_bytes(),
  )
  .unwrap();
  let shape = shape_by_id(page.shapes.as_ref().unwrap(), 1).unwrap();
  assert_eq!(
    read_section_cell(
      shape,
      SectionAddress {
        name: "User",
        index: None
      },
      Some(RowAddress {
        index: None,
        name: Some("Cost")
      }),
      "Value"
    )
    .unwrap()
    .value,
    Some("12")
  );
  assert_eq!(
    read_section_cell(
      shape,
      SectionAddress {
        name: "Actions",
        index: None
      },
      Some(RowAddress {
        index: Some(action_ix),
        name: Some("Go")
      }),
      "Menu"
    )
    .unwrap()
    .value,
    Some("Go")
  );
  assert_eq!(
    read_section_cell(
      shape,
      SectionAddress {
        name: "Connection",
        index: None
      },
      Some(RowAddress {
        index: Some(connection_ix),
        name: Some("C1")
      }),
      "Y"
    )
    .unwrap()
    .value,
    Some("1")
  );
  let tabs = shape.shape_choice.iter().find_map(|choice| match choice {
    ooxmlsdk::schemas::schemas_microsoft_com_office_visio_2012_main::ShapeChoice::Section(
      section,
    ) if section.n == "Tabs" => Some(section),
    _ => None,
  });
  let tab = tabs
    .unwrap()
    .section_choice
    .iter()
    .find_map(|item| {
      let ooxmlsdk::schemas::schemas_microsoft_com_office_visio_2012_main::SectionChoice::RowUnion(
        row,
      ) = item
      else {
        return None;
      };
      row.row_union_choice.iter().find_map(|choice| {
        let ooxmlsdk::schemas::schemas_microsoft_com_office_visio_2012_main::RowUnionChoice::Tab(
          tab,
        ) = choice
        else {
          return None;
        };
        (tab.ix == Some(tab_ix)).then_some(tab)
      })
    })
    .unwrap();
  assert_eq!(
    tab
      .cell
      .iter()
      .find(|cell| cell.n == "Position")
      .unwrap()
      .v
      .as_deref(),
    Some("0.5")
  );

  let mut ids = Ids::new();
  let mut package = minimal_drawing(&mut ids).unwrap();
  add_style(&mut package, 4, Some("Thin")).unwrap();
  add_face_name(&mut package, 3, "Calibri").unwrap();
  add_color(&mut package, 2, "#FF0000").unwrap();

  let document = package.visio_document_part().unwrap();
  let mut drawing = VisioDocument::from_bytes(&part_bytes(&document, &package)).unwrap();
  let style = drawing
    .style_sheets
    .as_mut()
    .unwrap()
    .style_sheet
    .iter_mut()
    .find(|style| style.id == 4)
    .unwrap();
  write_style_section_cell(
    style,
    SectionAddress {
      name: "Line",
      index: None,
    },
    None,
    "LineColor",
    "2",
    Some("THEMEVAL()"),
  );
  let sheet = drawing.document_sheet.get_or_insert_with(Default::default);
  write_document_direct_cell(sheet, "OutputFormat", "1", None);
  write_document_section_cell(
    sheet,
    SectionAddress {
      name: "User",
      index: None,
    },
    Some(RowAddress {
      index: None,
      name: Some("Doc"),
    }),
    "Value",
    "yes",
    None,
  );
  document
    .set_data(&mut package, drawing.to_xml().unwrap().into_bytes())
    .unwrap();

  let saved = package.to_package_bytes().unwrap();
  let opened = ooxmlsdk::parts::visio_document::VisioDocument::new(Cursor::new(saved)).unwrap();
  let document = opened.visio_document_part().unwrap();
  let drawing = VisioDocument::from_bytes(&part_bytes(&document, &opened)).unwrap();
  let style = drawing
    .style_sheets
    .as_ref()
    .unwrap()
    .style_sheet
    .iter()
    .find(|style| style.id == 4)
    .unwrap();
  assert_eq!(
    read_style_section_cell(
      style,
      SectionAddress {
        name: "Line",
        index: None
      },
      None,
      "LineColor"
    )
    .unwrap()
    .formula,
    Some("THEMEVAL()")
  );
  let instance = ooxmlsdk::schemas::schemas_microsoft_com_office_visio_2012_main::Shape {
    id: 1,
    line_style: Some(4),
    ..Default::default()
  };
  assert_eq!(
    styled_shape_cell(
      &instance,
      std::slice::from_ref(style),
      StyleSlot::Line,
      "Line",
      "LineColor"
    )
    .unwrap()
    .value,
    Some("2")
  );
  let sheet = drawing.document_sheet.as_ref().unwrap();
  assert_eq!(
    read_document_direct_cell(sheet, "OutputFormat")
      .unwrap()
      .value,
    Some("1")
  );
  assert_eq!(
    read_document_section_cell(
      sheet,
      SectionAddress {
        name: "User",
        index: None
      },
      Some(RowAddress {
        index: None,
        name: Some("Doc")
      }),
      "Value"
    )
    .unwrap()
    .value,
    Some("yes")
  );
  assert_eq!(
    drawing.face_names.as_ref().unwrap().face_name[0].name,
    "Calibri"
  );
  assert_eq!(
    drawing.colors.as_ref().unwrap().color_entry[0].rgb,
    "#FF0000"
  );
}
