use super::*;

pub unsafe trait Sse41Token: Ssse3Token {}

pub trait Sse41: Sse41Token {
    delegate! {
        fn _mm_blendv_epi8(a: __m128i, b: __m128i, mask: __m128i) -> __m128i;
        fn _mm_blend_epi16<const IMM8: i32>(a: __m128i, b: __m128i) -> __m128i;
        fn _mm_blendv_pd(a: __m128d, b: __m128d, mask: __m128d) -> __m128d;
        fn _mm_blendv_ps(a: __m128, b: __m128, mask: __m128) -> __m128;
        fn _mm_blend_pd<const IMM2: i32>(a: __m128d, b: __m128d) -> __m128d;
        fn _mm_blend_ps<const IMM4: i32>(a: __m128, b: __m128) -> __m128;
        fn _mm_extract_ps<const IMM8: i32>(a: __m128) -> i32;
        fn _mm_extract_epi8<const IMM8: i32>(a: __m128i) -> i32;
        fn _mm_extract_epi32<const IMM8: i32>(a: __m128i) -> i32;
        fn _mm_insert_ps<const IMM8: i32>(a: __m128, b: __m128) -> __m128;
        fn _mm_insert_epi8<const IMM8: i32>(a: __m128i, i: i32) -> __m128i;
        fn _mm_insert_epi32<const IMM8: i32>(a: __m128i, i: i32) -> __m128i;
        fn _mm_max_epi8(a: __m128i, b: __m128i) -> __m128i;
        fn _mm_max_epu16(a: __m128i, b: __m128i) -> __m128i;
        fn _mm_max_epi32(a: __m128i, b: __m128i) -> __m128i;
        fn _mm_max_epu32(a: __m128i, b: __m128i) -> __m128i;
        fn _mm_min_epi8(a: __m128i, b: __m128i) -> __m128i;
        fn _mm_min_epu16(a: __m128i, b: __m128i) -> __m128i;
        fn _mm_min_epi32(a: __m128i, b: __m128i) -> __m128i;
        fn _mm_min_epu32(a: __m128i, b: __m128i) -> __m128i;
        fn _mm_packus_epi32(a: __m128i, b: __m128i) -> __m128i;
        fn _mm_cmpeq_epi64(a: __m128i, b: __m128i) -> __m128i;
        fn _mm_cvtepi8_epi16(a: __m128i) -> __m128i;
        fn _mm_cvtepi8_epi32(a: __m128i) -> __m128i;
        fn _mm_cvtepi8_epi64(a: __m128i) -> __m128i;
        fn _mm_cvtepi16_epi32(a: __m128i) -> __m128i;
        fn _mm_cvtepi16_epi64(a: __m128i) -> __m128i;
        fn _mm_cvtepi32_epi64(a: __m128i) -> __m128i;
        fn _mm_cvtepu8_epi16(a: __m128i) -> __m128i;
        fn _mm_cvtepu8_epi32(a: __m128i) -> __m128i;
        fn _mm_cvtepu8_epi64(a: __m128i) -> __m128i;
        fn _mm_cvtepu16_epi32(a: __m128i) -> __m128i;
        fn _mm_cvtepu16_epi64(a: __m128i) -> __m128i;
        fn _mm_cvtepu32_epi64(a: __m128i) -> __m128i;
        fn _mm_dp_pd<const IMM8: i32>(a: __m128d, b: __m128d) -> __m128d;
        fn _mm_dp_ps<const IMM8: i32>(a: __m128, b: __m128) -> __m128;
        fn _mm_floor_pd(a: __m128d) -> __m128d;
        fn _mm_floor_ps(a: __m128) -> __m128;
        fn _mm_floor_sd(a: __m128d, b: __m128d) -> __m128d;
        fn _mm_floor_ss(a: __m128, b: __m128) -> __m128;
        fn _mm_ceil_pd(a: __m128d) -> __m128d;
        fn _mm_ceil_ps(a: __m128) -> __m128;
        fn _mm_ceil_sd(a: __m128d, b: __m128d) -> __m128d;
        fn _mm_ceil_ss(a: __m128, b: __m128) -> __m128;
        fn _mm_round_pd<const ROUNDING: i32>(a: __m128d) -> __m128d;
        fn _mm_round_ps<const ROUNDING: i32>(a: __m128) -> __m128;
        fn _mm_round_sd<const ROUNDING: i32>(a: __m128d, b: __m128d) -> __m128d;
        fn _mm_round_ss<const ROUNDING: i32>(a: __m128, b: __m128) -> __m128;
        fn _mm_minpos_epu16(a: __m128i) -> __m128i;
        fn _mm_mul_epi32(a: __m128i, b: __m128i) -> __m128i;
        fn _mm_mullo_epi32(a: __m128i, b: __m128i) -> __m128i;
        fn _mm_mpsadbw_epu8<const IMM8: i32>(a: __m128i, b: __m128i) -> __m128i;
        fn _mm_testz_si128(a: __m128i, mask: __m128i) -> i32;
        fn _mm_testc_si128(a: __m128i, mask: __m128i) -> i32;
        fn _mm_testnzc_si128(a: __m128i, mask: __m128i) -> i32;
        fn _mm_test_all_zeros(a: __m128i, mask: __m128i) -> i32;
        fn _mm_test_all_ones(a: __m128i) -> i32;
        fn _mm_test_mix_ones_zeros(a: __m128i, mask: __m128i) -> i32;
    }
}

impl<T: Sse41Token> Sse41 for T {}