use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use ooxmlsdk::schemas::schemas_openxmlformats_org_presentationml_2006_main::Presentation;
use ooxmlsdk::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Workbook;
use ooxmlsdk::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Document;
use ooxmlsdk_test::fixtures::{
  PRESENTATION_PRESENTATION_XML, SPREADSHEET_WORKBOOK_XML, WORDPROCESSING_DOCUMENT_HELLO_WORLD_XML,
};
use std::hint::black_box;
use std::io::{BufReader, Cursor};

macro_rules! bench_xml_round_trip {
  ($c:expr, $group_name:literal, $xml:expr, $ty:ty) => {{
    let xml = $xml;
    let parsed = xml.parse::<$ty>().unwrap();
    let mut group = $c.benchmark_group($group_name);

    group.throughput(Throughput::Bytes(xml.len() as u64));
    group.bench_with_input(BenchmarkId::new("read", "slice"), &xml, |b, xml| {
      b.iter(|| black_box(xml).parse::<$ty>().unwrap())
    });
    group.bench_with_input(BenchmarkId::new("read", "stream_cursor"), &xml, |b, xml| {
      b.iter(|| <$ty>::from_reader(Cursor::new(black_box(xml.as_bytes()))).unwrap())
    });
    group.bench_with_input(
      BenchmarkId::new("read", "stream_bufreader"),
      &xml,
      |b, xml| {
        b.iter(|| {
          <$ty>::from_reader(BufReader::new(Cursor::new(black_box(xml.as_bytes())))).unwrap()
        })
      },
    );
    group.bench_with_input(BenchmarkId::new("write", "parsed"), &parsed, |b, value| {
      b.iter(|| black_box(value).to_xml().unwrap())
    });
    group.bench_with_input(BenchmarkId::new("round_trip", "slice"), &xml, |b, xml| {
      b.iter(|| {
        let value = black_box(xml).parse::<$ty>().unwrap();
        let serialized = black_box(&value).to_xml().unwrap();
        black_box(serialized)
      })
    });

    group.finish();
  }};
}

fn bench_xml(c: &mut Criterion) {
  bench_xml_round_trip!(
    c,
    "wordprocessing_document",
    WORDPROCESSING_DOCUMENT_HELLO_WORLD_XML,
    Document
  );
  bench_xml_round_trip!(
    c,
    "spreadsheet_workbook",
    SPREADSHEET_WORKBOOK_XML,
    Workbook
  );
  bench_xml_round_trip!(
    c,
    "presentation_presentation",
    PRESENTATION_PRESENTATION_XML,
    Presentation
  );
}

criterion_group!(benches, bench_xml);
criterion_main!(benches);
