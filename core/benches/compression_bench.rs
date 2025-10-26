use criterion::{criterion_group, criterion_main, Criterion};
use owlsol_core::prelude::*;

fn bench_huffman(c: &mut Criterion) {
    let data = b"hello world hello world hello world";
    let compressor = Compressor::new();
    c.bench_function("huffman_compress", |b| {
        b.iter(|| compressor.compress_with_algorithm(data, Some(CompressionAlgorithm::Huffman)))
    });
}

fn bench_rle(c: &mut Criterion) {
    let data = vec![b'A'; 1000];
    let compressor = Compressor::new();
    c.bench_function("rle_compress", |b| {
        b.iter(|| compressor.compress_with_algorithm(&data, Some(CompressionAlgorithm::RunLength)))
    });
}

fn bench_dictionary(c: &mut Criterion) {
    let data = b"test data test data test data test data";
    let compressor = Compressor::new();
    c.bench_function("dictionary_compress", |b| {
        b.iter(|| compressor.compress_with_algorithm(data, Some(CompressionAlgorithm::Dictionary)))
    });
}

criterion_group!(benches, bench_huffman, bench_rle, bench_dictionary);
criterion_main!(benches);
