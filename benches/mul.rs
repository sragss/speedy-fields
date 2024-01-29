use ark_bn254::Fr as ArkFr;
use halo2curves::{bn256::Fr as Halo2Fr, ff::Field};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ark_std::{rand::Rng, UniformRand};

fn ark_fr_mul(c: &mut Criterion) {
    let mut rng = ark_std::test_rng();
    let a_op = ArkFr::rand(&mut rng);
    let b_op = ArkFr::rand(&mut rng);
    c.bench_function("ArkFr multiplication", |b| {
        b.iter(|| black_box(a_op) * black_box(b_op))
    });
}

fn halo2_fr_mul(c: &mut Criterion) {
    let mut rng = ark_std::test_rng();
    let a_op = Halo2Fr::random(&mut rng);
    let b_op = Halo2Fr::random(&mut rng);
    c.bench_function("Halo2Fr multiplication", |b| {
        b.iter(|| black_box(a_op) * black_box(b_op))
    });
}

criterion_group!(benches, ark_fr_mul, halo2_fr_mul);
criterion_main!(benches);
