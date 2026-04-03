use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use ooxmlsdk::parts::{
  presentation_document::PresentationDocument, spreadsheet_document::SpreadsheetDocument,
  wordprocessing_document::WordprocessingDocument,
};
use ooxmlsdk_test::fixtures;
use std::hint::black_box;
use std::io::Cursor;

macro_rules! bench_package_round_trip {
  ($c:expr, $group_name:literal, $file_name:literal, $ty:ty) => {{
    let bytes = std::fs::read(fixtures::doc_sample_path($file_name)).unwrap();
    let parsed = <$ty>::new(Cursor::new(bytes.as_slice())).unwrap();
    let mut group = $c.benchmark_group($group_name);

    group.throughput(Throughput::Bytes(bytes.len() as u64));
    group.bench_with_input(BenchmarkId::new("read", "cursor"), &bytes, |b, bytes| {
      b.iter(|| <$ty>::new(Cursor::new(black_box(bytes.as_slice()))).unwrap())
    });
    group.bench_with_input(BenchmarkId::new("write", "parsed"), &parsed, |b, value| {
      b.iter(|| {
        let mut output = Cursor::new(Vec::new());
        black_box(value).save(&mut output).unwrap();
        black_box(output.into_inner())
      })
    });
    group.bench_with_input(
      BenchmarkId::new("round_trip", "cursor"),
      &bytes,
      |b, bytes| {
        b.iter(|| {
          let parsed = <$ty>::new(Cursor::new(black_box(bytes.as_slice()))).unwrap();
          let mut output = Cursor::new(Vec::new());
          black_box(&parsed).save(&mut output).unwrap();
          let round_tripped = <$ty>::new(Cursor::new(output.into_inner())).unwrap();
          black_box(round_tripped)
        })
      },
    );

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

criterion_group!(benches, bench_packages);
criterion_main!(benches);
