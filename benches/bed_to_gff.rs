use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use rsomics_bed_to_gff::bed_to_gff;
use std::io::Cursor;

fn make_fixture(n: usize) -> String {
    (0..n)
        .map(|i| {
            format!(
                "chr1\t{}\t{}\tfeature_{i}\t100\t+\n",
                i * 200,
                i * 200 + 100
            )
        })
        .collect()
}

fn bench_bed_to_gff(c: &mut Criterion) {
    let fixture = make_fixture(100_000);
    let mut group = c.benchmark_group("bed-to-gff");
    group.throughput(Throughput::Elements(100_000));
    group.bench_function("to_gff_100k", |b| {
        b.iter(|| {
            let mut out = Vec::with_capacity(4 * 1024 * 1024);
            bed_to_gff(Cursor::new(fixture.as_str()), "rsomics", "gene", &mut out).unwrap();
        });
    });
    group.finish();
}

criterion_group!(benches, bench_bed_to_gff);
criterion_main!(benches);
