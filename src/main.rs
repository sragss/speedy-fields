use ark_bn254::Fr as ArkFr;
use halo2curves::{bn256::Fr as Halo2Fr, ff::Field};
use fast_halo2curves::{bn256::Fr as NewHalo2Fr, ff::{Field as NewField, PrimeFieldBits}};
use ark_std::UniformRand;
use speedy_fields::CIOS;
use std::{time::Instant, hint::black_box};
use ark_std::rand::Rng;
use ark_ff::biginteger::BigInt;

use speedy_fields::metal::{run, ark_run};

const SIZE: usize = 10_000_000;

fn main() {
    run();
    ark_run();

    let mut rng = ark_std::test_rng();

    let rands: Vec<([u64; 4], [u64; 4])> = (0..SIZE).map(|_| {
        let mut arr1 = [0u64; 4];
        let mut arr2 = [0u64; 4];
        for i in 0..4 {
            arr1[i] = rng.gen();
            arr2[i] = rng.gen();
        }
        (arr1, arr2)
    }).collect();

    let ark_fr_mul_vec: Vec<(ArkFr, ArkFr)> = rands.iter().map(|&(arr1, arr2)| {
        let a = ArkFr::new(BigInt::new(arr1));
        let b = ArkFr::new(BigInt::new(arr2));
        (a, b)
    }).collect();
    let ark_fr_mul_start = Instant::now();
    for (a, b) in ark_fr_mul_vec {
        let c = black_box(a) * black_box(b);
        black_box(c);
    }
    let ark_fr_mul_duration = ark_fr_mul_start.elapsed();
    let avg_fr = ark_fr_mul_duration / SIZE as u32;
    println!("Time per ArkFr multiplication: {:?}", avg_fr);

    let cios_mul_vec: Vec<_> = rands.iter().map(|&(arr1, arr2)| {
        let a = CIOS(arr1);
        let b = CIOS(arr2);
        (a, b)
    }).collect();
    let cios_mul_start = Instant::now();
    for (a, b) in cios_mul_vec.clone() {
        let c = a.mul(&black_box(b));
        black_box(c);
    }
    let cios_mul_duration = cios_mul_start.elapsed();
    let cios_mul_multiple = cios_mul_duration.as_secs_f64() / ark_fr_mul_duration.as_secs_f64();
    println!("Time per CIOS multiplication: {:?} ({:.2}x ArkFr)", cios_mul_duration / SIZE as u32, cios_mul_multiple);

    let cios_edmsm_mul_start = Instant::now();
    for (a, b) in cios_mul_vec {
        let c = a.mul_edmsm(&black_box(b));
        black_box(c);
    }
    let cios_edmsm_mul_duration = cios_edmsm_mul_start.elapsed();
    let cios_edmsm_mul_multiple = cios_edmsm_mul_duration.as_secs_f64() / ark_fr_mul_duration.as_secs_f64();
    println!("Time per CIOS multiplication with EDMSM: {:?} ({:.2}x ArkFr)", cios_edmsm_mul_duration / SIZE as u32, cios_edmsm_mul_multiple);

    let halo2_fr_mul_vec: Vec<_> = rands.iter().map(|&(arr1, arr2)| {
        let a = Halo2Fr::from_raw(arr1);
        let b = Halo2Fr::from_raw(arr2);
        (a, b)
    }).collect();
    let halo2_fr_mul_start = Instant::now();
    for (mut a, b) in halo2_fr_mul_vec {
        let c = black_box(a) * black_box(b);
        black_box(c);
    }
    let halo2_fr_mul_duration = halo2_fr_mul_start.elapsed();
    let halo2_fr_mul_multiple = halo2_fr_mul_duration.as_secs_f64() / ark_fr_mul_duration.as_secs_f64();
    println!("Time per OLD Halo2Fr multiplication: {:?} ({:.2}x ArkFr)", halo2_fr_mul_duration / SIZE as u32, halo2_fr_mul_multiple);

    let halo2_fr_mul_vec: Vec<_> = rands.iter().map(|&(arr1, arr2)| {
        let a = NewHalo2Fr::from_raw(arr1);
        let b = NewHalo2Fr::from_raw(arr2);
        (a, b)
    }).collect();
    let halo2_fr_mul_start = Instant::now();
    for (mut a, b) in halo2_fr_mul_vec {
        let c = black_box(a) * black_box(b);
        black_box(c);
    }
    let halo2_fr_mul_duration = halo2_fr_mul_start.elapsed();
    let halo2_fr_mul_multiple = halo2_fr_mul_duration.as_secs_f64() / ark_fr_mul_duration.as_secs_f64();
    println!("Time per NEW Halo2Fr multiplication: {:?} ({:.2}x ArkFr)", halo2_fr_mul_duration / SIZE as u32, halo2_fr_mul_multiple);
}