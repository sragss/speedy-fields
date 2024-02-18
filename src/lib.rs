pub mod metal;

/// This file implements the EdMSM paper for Montgomery Multiplication with constants for Bn254.
/// The arithmetic functions are lifted from Arkworks / Halo2Curves.
/// https://eprint.iacr.org/2022/1400.pdf
use ark_std::rand;
use unroll::unroll_for_loops;

/// INV = -(r^{-1} mod 2^64) mod 2^64
const INV: u64 = 0xc2e1f593efffffff;

/// `R = 2^256 mod r`
/// `0xe0a77c19a07df2f666ea36f7879462e36fc76959f60cd29ac96341c4ffffffb`
const R: [u64; 4] = [
    0xac96341c4ffffffb,
    0x36fc76959f60cd29,
    0x666ea36f7879462e,
    0x0e0a77c19a07df2f,
];

/// `R^2 = 2^512 mod r`
/// `0x216d0b17f4e44a58c49833d53bb808553fe3ab1e35c59e31bb8e645ae216da7`
const R2: [u64; 4] = [
    0x1bb8e645ae216da7,
    0x53fe3ab1e35c59e3,
    0x8c49833d53bb8085,
    0x0216d0b17f4e44a5,
];

/// `R^3 = 2^768 mod r`
/// `0xcf8594b7fcc657c893cc664a19fcfed2a489cbe1cfbb6b85e94d8e1b4bf0040`
const R3: [u64; 4] = [
    0x5e94d8e1b4bf0040,
    0x2a489cbe1cfbb6b8,
    0x893cc664a19fcfed,
    0x0cf8594b7fcc657c,
];

const MODULUS: [u64; 4] = [
    4891460686036598785u64,
    2896914383306846353u64,
    13281191951274694749u64,
    3486998266802970665u64,
];

const N: usize = 4;

#[derive(Debug, Clone)]
pub struct CIOS(pub [u64; 4]);

impl CIOS {
    /// Implements normal CIOS
    #[unroll_for_loops]
    #[inline(always)]
    pub fn mul(&self, rhs: &Self) -> Self {
        let mut t: [u64; N] = [0u64; N];
        let mut c1 = 0u64;
        let mut c2;
        for i in 0..4 {
            let mut c: u64 = 0u64;
            for j in 0..4 {
                (t[j], c) = mac(t[j], self.0[i], rhs.0[j], c);
            }
            (c1, c2) = adc(c1, c, 0);

            let m = t[0].wrapping_mul(INV);
            (_, c) = macx(t[0], m, MODULUS[0]);

            for j in 1..4 {
                (t[j-1], c) = mac(t[j], m, MODULUS[j], c);
            }
            (t[N-1], c) = adc(c1, c, 0);
            c1 = c2 + c;
        }

        if bigint_greater_eq(&t, &MODULUS) {
            let mut borrow = 0;
            borrow = sbb(&mut t[0], MODULUS[0], borrow);
            borrow = sbb(&mut t[1], MODULUS[1], borrow);
            borrow = sbb(&mut t[2], MODULUS[2], borrow);
            sbb(&mut t[3], MODULUS[3], borrow);
        }

        Self(t)
    }

    /// Implements the carry optimization for CIOS.
    /// Only works when the modulus high-bit < (D - 1)/2 - 1, 
    /// where D is word size. Fine for Bn254, problem for Secp256k1.
    /// Experimentally on ARM it's only marginally faster. Likely 1-4
    /// instructions.
    #[unroll_for_loops]
    #[inline(always)]
    pub fn mul_edmsm(self, rhs: &Self) -> Self {
        let mut t: [u64; N+1] = [0u64; N+1];
        for i in 0..4 {
            let mut c: u64 = 0u64;
            for j in 0..4 {
                (t[j], c) = mac(t[j], self.0[j], rhs.0[i], c);
            }
            t[N] = c;

            let m = t[0].wrapping_mul(INV); 
            (_, c) = macx(t[0], m, MODULUS[0]);

            for j in 1..4 {
                (t[j-1], c) = mac(t[j], m, MODULUS[j], c);
            }
            t[N-1] = t[N] + c;
        }
        t[N] = 0;

        let mut trunc_t: [u64; N] = t[0..N].try_into().unwrap();
        if bigint_greater_eq(&trunc_t, &MODULUS) {
            let mut borrow = 0;
            borrow = sbb(&mut trunc_t[0], MODULUS[0], borrow);
            borrow = sbb(&mut trunc_t[1], MODULUS[1], borrow);
            borrow = sbb(&mut trunc_t[2], MODULUS[2], borrow);
            sbb(&mut trunc_t[3], MODULUS[3], borrow);
        }

        Self(trunc_t)
    }

    pub fn rand() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        Self([rng.gen(), rng.gen(), rng.gen(), rng.gen()])
    }

    #[inline(always)]
    pub fn sub_mod_if_big(a: &mut [u64; 4]) {
        if bigint_greater_eq(a, &MODULUS) {
            sub_with_borrow(a, &MODULUS);
        }
    }
}


// Halo2Curves versions

/// Compute a + (b * c) + carry, returning the result and the new carry over.
#[inline(always)]
pub(crate) const fn mac(a: u64, b: u64, c: u64, carry: u64) -> (u64, u64) {
    let ret = (a as u128) + ((b as u128) * (c as u128)) + (carry as u128);
    (ret as u64, (ret >> 64) as u64)
}

/// Compute a + (b * c), returning the result and the new carry over.
#[inline(always)]
pub(crate) const fn macx(a: u64, b: u64, c: u64) -> (u64, u64) {
    let res = (a as u128) + ((b as u128) * (c as u128));
    (res as u64, (res >> 64) as u64)
}

/// Compute a + b + carry, returning the result and the new carry over.
#[inline(always)]
pub(crate) const fn adc(a: u64, b: u64, carry: u64) -> (u64, u64) {
    let ret = (a as u128) + (b as u128) + (carry as u128);
    (ret as u64, (ret >> 64) as u64)
}

/// Compute a + b , returning the result and the new carry over.
#[inline(always)]
pub(crate) const fn adcx(a: u64, b: u64) -> (u64, u64) {
    let ret = (a as u128) + (b as u128);
    (ret as u64, (ret >> 64) as u64)
}

#[inline(always)]
#[unroll_for_loops]
pub(crate) const fn bigint_greater_eq(a: &[u64; 4], b: &[u64; 4]) -> bool {
    for i in 0..4 {
        if a[3-i] > b[3-i] {
            return true;
        } else if a[3-i] < b[3-i] {
            return false;
        }
    }
    true
}


// (Arkworks)
/// Sets a = a - b - borrow, and returns the borrow.
#[inline(always)]
#[allow(unused_mut)]
pub(crate) fn sbb(a: &mut u64, b: u64, borrow: u64) -> u64 {
    let tmp = (1u128 << 64) + (*a as u128) - (b as u128) - (borrow as u128);
    *a = tmp as u64;
    (tmp >> 64 == 0) as u64
}


#[inline(always)]
fn sub_with_borrow(a: &mut [u64; 4], b: &[u64; 4]) -> bool {
    let mut borrow = 0;
    borrow = sbb(&mut a[0], b[0], borrow);
    borrow = sbb(&mut a[1], b[1], borrow);
    borrow = sbb(&mut a[2], b[2], borrow);
    borrow = sbb(&mut a[3], b[3], borrow);
    borrow != 0
}