use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use rand::distributions::{Distribution, Standard};
use rand::prelude::*;

fn bench(c: &mut Criterion) {
    const KB: usize = 1024;
    let mut rng = thread_rng();

    let mut encoding = c.benchmark_group("encoding");

    for size in [
        1,
        2,
        4,
        8,
        16,
        32,
        64,
        KB,
        2 * KB,
        4 * KB,
        8 * KB,
        16 * KB,
        32 * KB,
    ] {
        let input: Vec<u8> = Standard.sample_iter(&mut rng).take(size).collect();
        let param = format!("{}B", size);

        encoding.throughput(Throughput::Bytes(size as u64));
        encoding.bench_with_input(BenchmarkId::new("hex", &param), &input, |bench, input| {
            bench.iter(|| hex::encode(input))
        });

        encoding.bench_with_input(
            BenchmarkId::new("faster_hex", &param),
            &input,
            |bench, input| bench.iter(|| faster_hex::hex_string(input)),
        );

        encoding.bench_with_input(
            BenchmarkId::new("hexide", &param),
            &input,
            |bench, input| bench.iter(|| hexide::encode_str(input)),
        );
    }
}

criterion_group!(benches, bench);
criterion_main!(benches);
