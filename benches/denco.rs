use criterion::{black_box, criterion_group, criterion_main, Criterion, Bencher, Throughput, BenchmarkId};
use rand::rngs::{SmallRng};
use rand::{Rng, SeedableRng};
use denco::{encode, decode, Config, Encoder, parallel_encode, parallel_decode};
use denco::fast::FastConverter;
use denco::parallel::ParallelBase64;


const ARRAY_SIZES: [usize; 7] = [2 * 256, 2 * 1024, 2 * 4096, 2 * 65536,  2 * 1024 * 1024  ,   10 * 1024 * 1024 , 30 * 1024 * 1024];


fn do_decode(b: &mut Bencher, size: &usize){
    let mut data = Vec::with_capacity(size * 3 / 4);
    fill(&mut data);
    let encoded = encode(&data);
    b.iter(|| {
        decode(&encoded);
    });
}

fn do_encode(b: &mut Bencher, size: &usize){
    let mut data = Vec::with_capacity(*size);
    fill(&mut data);

    b.iter(|| {
        encode(&data);
    });
}

fn do_encode_parallel(b: &mut Bencher, size: &usize){
    let mut data = Vec::with_capacity(*size);
    fill(&mut data);

    b.iter(|| {
        parallel_encode(&data, Config::DEFAULT);
    });
}

fn do_decode_parallel(b: &mut Bencher, size: &usize){
    let mut data = Vec::with_capacity(size * 3 / 4);
    fill(&mut data);
    let encoded = parallel_encode(&data, Config::DEFAULT);
    b.iter(|| {
        parallel_decode(&encoded, Config::DEFAULT);
    });
}

fn decode_benchmark(c: &mut Criterion) {

    let mut group = c.benchmark_group("decode");
    for size in ARRAY_SIZES {
        group.throughput(Throughput::Bytes(size as u64));
        group.bench_with_input(BenchmarkId::new("decode", size), &size,do_decode);
        group.bench_with_input(BenchmarkId::new("decode_parallel", size), &size,do_decode_parallel);
    }


}

fn encode_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("encode");
    for size in ARRAY_SIZES {
        group.throughput(Throughput::Bytes(size as u64));
        group.bench_with_input(BenchmarkId::new("encode", size), &size,do_encode);
        group.bench_with_input(BenchmarkId::new("encode_parallel", size), &size,do_encode_parallel);
    }

}

fn fill(data: &mut Vec<u8>) {
    let mut rng = SmallRng::from_entropy();
    let capa = data.capacity();
    for _ in 0..capa {
        data.push(rng.gen());
    }
}

criterion_group!(benches,  encode_benchmark, decode_benchmark);
criterion_main!(benches);