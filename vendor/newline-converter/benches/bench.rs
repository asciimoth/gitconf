use criterion::{black_box, criterion_group, criterion_main, Criterion};
use newline_converter::{dos2unix, unix2dos};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("unix2dos", |b| {
        b.iter(|| unix2dos(black_box("\nfoo\nbar\n")))
    });
    c.bench_function("unix2dos NOOP", |b| {
        b.iter(|| unix2dos(black_box("\r\nfoo\r\nbar\r\n")))
    });

    c.bench_function("dos2unix", |b| {
        b.iter(|| dos2unix(black_box("\r\nfoo\r\nbar\r\n")))
    });
    c.bench_function("dos2unix NOOP", |b| {
        b.iter(|| dos2unix(black_box("\nfoo\nbar\n")))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
