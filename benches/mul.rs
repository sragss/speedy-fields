use std::ops::MulAssign;

use ark_bn254::Fr as ArkFr;
use halo2curves::{bn256::Fr as Halo2Fr, ff::Field};
use halo2curves::bn256::Fr as NewHalo2Fr;

use ark_std::UniformRand;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use speedy_fields::CIOS;

fn ark_fr_mul(c: &mut Criterion) {
    c.bench_function("ArkFr multiplication", |b| {
        b.iter_batched(
            || {
                let mut rng = ark_std::test_rng();
                let a_op = ArkFr::rand(&mut rng);
                let b_op = ArkFr::rand(&mut rng);
                (a_op, b_op)
            },
            |(a_op, b_op)| {
                let c = black_box(a_op) * black_box(b_op);
                black_box(c)
            },
            criterion::BatchSize::SmallInput,
        )
    });
}

fn halo2_fr_mul(c: &mut Criterion) {
    c.bench_function("Halo2Fr OLD multiplication", |b| {
        b.iter_batched(
            || {
                let mut rng = ark_std::test_rng();
                let a_op = Halo2Fr::random(&mut rng);
                let b_op = Halo2Fr::random(&mut rng);
                (a_op, b_op)
            },
            |(a_op, b_op)| {
                let c = black_box(a_op).mul(&black_box(b_op));
                black_box(c)
            },
            criterion::BatchSize::SmallInput,
        )
    });
}

fn halo2_fr_const_mul(c: &mut Criterion) {
    c.bench_function("Halo2Fr NEW multiplication", |b| {
        b.iter_batched(
            || {
                let mut rng = ark_std::test_rng();
                let a_op = NewHalo2Fr::random(&mut rng);
                let b_op = NewHalo2Fr::random(&mut rng);
                (a_op, b_op)
            },
            |(a_op, b_op)| {
                let c = black_box(a_op).mul(&black_box(b_op));
                black_box(c)
            },
            criterion::BatchSize::SmallInput,
        )
    });
}

fn cios_mul(c: &mut Criterion) {
    c.bench_function("CIOS multiplication", |b| {
        b.iter_batched(
            || {
                let a_op = CIOS::rand();
                let b_op = CIOS::rand();
                (a_op, b_op)
            },
            |(a_op, b_op)| {
                let c = black_box(a_op).mul(&black_box(b_op));
                black_box(c)
            },
            criterion::BatchSize::SmallInput,
        )
    });
}

fn cios_edmsm_mul(c: &mut Criterion) {
    c.bench_function("CIOS multiplication", |b| {
        b.iter_batched(
            || {
                let a_op = CIOS::rand();
                let b_op = CIOS::rand();
                (a_op, b_op)
            },
            |(a_op, b_op)| {
                let c = black_box(a_op).mul_edmsm(&black_box(b_op));
                black_box(c)
            },
            criterion::BatchSize::SmallInput,
        )
    });
}

criterion_group!(
    benches,
    ark_fr_mul,
    halo2_fr_mul,
    halo2_fr_const_mul,
    cios_mul,
    cios_edmsm_mul
);
criterion_main!(benches);
