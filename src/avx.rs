// use std::arch::x86_64::*;
//
//
// #[target_feature(enable = "avx2")]
// #[inline]
// unsafe fn enc_reshuffle(input: __m256i) -> __m256i {
//     _mm256_shuffle_epi8(input, _mm256_setr_epi8(3, 2, 1, 0, 7, 6, 5, 4, 11,
//                                                 10, 9, 8, 15, 14, 13, 12, 3,
//                                                 2, 1, 0, 7, 6, 5, 4, 11, 10,
//                                                 9, 8, 15, 14, 13, 12))
// }
//
//
// #[target_feature(enable = "avx2")]
// #[inline]
// unsafe fn _mm256_bswap_epi32(input: __m256i) -> __m256i {
//     // _mm256_shuffle_epi8() works on two 128-bit lanes separately:
//     return _mm256_shuffle_epi8(input, _mm256_setr_epi8(3, 2, 1, 0, 7, 6, 5, 4, 11,
//                                                        10, 9, 8, 15, 14, 13, 12, 3,
//                                                        2, 1, 0, 7, 6, 5, 4, 11, 10,
//                                                        9, 8, 15, 14, 13, 12));
// }
//
// #[target_feature(enable = "avx2")]
// #[inline]
// unsafe fn enc_resuffle(input: __m256i) -> __m256i {
//     let mut inter = _mm256_permutevar8x32_epi32(input,
//                                                 _mm256_setr_epi32(0, 1, 2, -1, 3, 4, 5, -1));
//
//     // Slice into 32-bit chunks and operate on all chunks in parallel.
//     // All processing is done within the 32-bit chunk. First, shuffle:
//     // before: [eeeeeeff|ccdddddd|bbbbcccc|aaaaaabb]
//     // after:  [00000000|aaaaaabb|bbbbcccc|ccdddddd]
//     inter = _mm256_shuffle_epi8(inter,
//                                 _mm256_set_epi8(-1, 9, 10, 11, -1, 6, 7, 8, -1, 3, 4,
//                                                 5, -1, 0, 1, 2, -1, 9, 10, 11, -1, 6,
//                                                 7, 8, -1, 3, 4, 5, -1, 0, 1, 2));
//
//     // merged  = [0000aaaa|aabbbbbb|bbbbcccc|ccdddddd]
//     let merged = _mm256_blend_epi16::<0x55>(_mm256_slli_epi32::<4>(inter), inter);
//
//     // bd      = [00000000|00bbbbbb|00000000|00dddddd]
//     let bd = _mm256_and_si256(merged, _mm256_set1_epi32(0x003F003F));
//
//     // ac      = [00aaaaaa|00000000|00cccccc|00000000]
//     let ac = _mm256_and_si256(_mm256_slli_epi32::<2>(merged),
//                               _mm256_set1_epi32(0x3F003F00));
//
//     // indices = [00aaaaaa|00bbbbbb|00cccccc|00dddddd]
//     let indices = _mm256_or_si256(ac, bd);
//
//     // return  = [00dddddd|00cccccc|00bbbbbb|00aaaaaa]
//     return _mm256_bswap_epi32(indices);
// }
//
// unsafe fn cmpgt(a: __m256i, b: i8) -> __m256i {
//     return _mm256_cmpgt_epi8(a, _mm256_set1_epi8(b));
// }
//
// #[target_feature(enable = "avx2")]
// #[inline]
// unsafe fn enc_translate(input: __m256i) -> __m256i {
// // LUT contains Absolute offset for all ranges:
//     let lut = _mm256_setr_epi8(
//         65, 71, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -19, -16, 0, 0, 65, 71,
//         -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -19, -16, 0, 0);
// // Translate values 0..63 to the Base64 alphabet. There are five sets:
// // #  From      To         Abs    Index  Characters
// // 0  [0..25]   [65..90]   +65        0  ABCDEFGHIJKLMNOPQRSTUVWXYZ
// // 1  [26..51]  [97..122]  +71        1  abcdefghijklmnopqrstuvwxyz
// // 2  [52..61]  [48..57]    -4  [2..11]  0123456789
// // 3  [62]      [43]       -19       12  +
// // 4  [63]      [47]       -16       13  /
//
// // Create LUT indices from input:
// // the index for range #0 is right, others are 1 less than expected:
//     let indices = _mm256_subs_epu8(input, _mm256_set1_epi8(51));
//
// // mask is 0xFF (-1) for range #[1..4] and 0x00 for range #0:
//     let mask = cmpgt(input, 25);
//
// // substract -1, so add 1 to indices for range #[1..4], All indices are now
// // correct:
//     let indices = _mm256_sub_epi8(indices, mask);
//
// // Add offsets to input values:
//     let out = _mm256_add_epi8(input, _mm256_shuffle_epi8(lut, indices));
//
//     return out;
// }
//
// #[derive(Clone, Copy)]
// struct State {
//     eof: i32,
//     bytes: i32,
//     carry: u8,
// }
// #[target_feature(enable = "avx2")]
// #[inline]
// unsafe fn base64_stream_encode_avx2(state: State, src: &mut Vec<u8>, out: &mut Vec<u8>) -> String {
// // Assume that *out is large enough to contain the output.
// // Theoretically it should be 4/3 the length of src.
//
//     let mut c = src.as_ptr() as *mut __m256i;
//     let mut o = out.as_mut_ptr() as *mut __m256i;
//     let mut src_len = src.len();
//     let mut out_len = out.len();
// // Use local temporaries to avoid cache thrashing:
//     let tmp = state.clone();
//
//     while(true) {
//         if src_len == 0 {
//             break;
//         }
//         match tmp.bytes {
//             0 => {
//                 while src_len >= 32 {
//                     let mut str = _mm256_loadu_si256(c);
//                     str = enc_reshuffle(str);
//                     str = enc_translate(str);
//                     _mm256_storeu_si256(o, str);
//                     c = c.add(24);
//                     o = o.add(32);
//                     src_len -= 24;
//                     out_len += 32;
//                 }
//
//             }
//         }
//     }
//
//
//     String::new();
// }
//
//
//
// struct Avx;
//
// impl Avx {
//     fn new() -> Self {
//         Avx
//     }
//
//     fn encode(&self, input: &[u8]) -> Vec<u8> {
//         Vec::new()
//     }
// }