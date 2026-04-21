use criterion::{BatchSize, BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use ooxmlsdk::parts::{
  presentation_document::PresentationDocument, spreadsheet_document::SpreadsheetDocument,
  wordprocessing_document::WordprocessingDocument,
};
use ooxmlsdk::schemas::schemas_openxmlformats_org_presentationml_2006_main::Presentation;
use ooxmlsdk::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Workbook;
use ooxmlsdk::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Document;
use ooxmlsdk_test::fixtures::{
  self, PRESENTATION_PRESENTATION_XML, SPREADSHEET_WORKBOOK_XML,
  WORDPROCESSING_DOCUMENT_HELLO_WORLD_XML,
};
use std::hint::black_box;
use std::io::{BufReader, Cursor};
use std::time::Duration;

macro_rules! bench_package_round_trip {
  ($c:expr, $group_name:literal, $file_name:literal, $ty:ty) => {{
    let bytes = std::fs::read(fixtures::doc_sample_path($file_name)).unwrap();
    let parsed = <$ty>::new(Cursor::new(bytes.as_slice())).unwrap();
    let mut group = $c.benchmark_group($group_name);

    group.throughput(Throughput::Bytes(bytes.len() as u64));
    group.sample_size(50);
    group.measurement_time(Duration::from_secs(10));

    group.bench_with_input(BenchmarkId::new("read", "cursor"), &bytes, |b, bytes| {
      b.iter(|| <$ty>::new(Cursor::new(black_box(bytes.as_slice()))).unwrap())
    });

    group.bench_with_input(BenchmarkId::new("write", "parsed"), &parsed, |b, value| {
      b.iter_batched_ref(
        || Cursor::new(Vec::with_capacity(bytes.len())),
        |output| {
          output.get_mut().clear();
          output.set_position(0);
          value.save(&mut *output).unwrap();
          black_box(output.get_ref().len())
        },
        BatchSize::SmallInput,
      )
    });

    group.bench_with_input(
      BenchmarkId::new("round_trip", "cursor"),
      &bytes,
      |b, bytes| {
        b.iter_batched_ref(
          || Cursor::new(Vec::with_capacity(bytes.len())),
          |output| {
            output.get_mut().clear();
            output.set_position(0);
            let parsed = <$ty>::new(Cursor::new(black_box(bytes.as_slice()))).unwrap();
            parsed.save(&mut *output).unwrap();
            let round_tripped = <$ty>::new(Cursor::new(output.get_ref().as_slice())).unwrap();
            black_box(round_tripped)
          },
          BatchSize::SmallInput,
        )
      },
    );

    group.finish();
  }};
}

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

fn bench_packages(c: &mut Criterion) {
  bench_package_round_trip!(
    c,
    "wordprocessing_document",
    "Document.docx",
    WordprocessingDocument
  );
  bench_package_round_trip!(
    c,
    "spreadsheet_document",
    "Spreadsheet.xlsx",
    SpreadsheetDocument
  );
  bench_package_round_trip!(
    c,
    "presentation_document",
    "Presentation.pptx",
    PresentationDocument
  );
}

fn bench_xml(c: &mut Criterion) {
  bench_xml_round_trip!(
    c,
    "wordprocessing_document_xml",
    WORDPROCESSING_DOCUMENT_HELLO_WORLD_XML,
    Document
  );
  bench_xml_round_trip!(
    c,
    "spreadsheet_workbook_xml",
    SPREADSHEET_WORKBOOK_XML,
    Workbook
  );
  bench_xml_round_trip!(
    c,
    "presentation_presentation_xml",
    PRESENTATION_PRESENTATION_XML,
    Presentation
  );
}

criterion_group!(benches, bench_packages, bench_xml);
criterion_main!(benches);
