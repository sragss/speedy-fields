# Speedy Fields
Compare speeds on different fields and attempt acceleration.

# Levels of optimization
- `mul` function
- `mac` function `ark_ff::biginteger::algebra::mac` –– used everywhere

# Ideas

- [Apple/Accelerate/vBigNum](https://developer.apple.com/documentation/accelerate/veclib/vbignum#1806683) library
- [Apple/Accelerate/SIMD](https://developer.apple.com/documentation/accelerate/simd) library
- [NEON SIMD](https://docs.rs/simd/latest/simd/aarch64/index.html)