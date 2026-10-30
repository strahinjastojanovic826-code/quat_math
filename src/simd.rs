use crate::vector::QuatVec;

/// High-performance SIMD-optimized operations for bulk processing QuatVec data
pub struct QuatSimd;

impl QuatSimd {
    /// Bulk parallel Post Inversion N(x) = (3 - x) across raw packed byte slices.
    /// Uses 64-bit word chunks for CPU vectorization/pipelining.
    pub fn bulk_post_neg(input: &[u8], output: &mut [u8]) {
        assert_eq!(input.len(), output.len());

        let chunks_in = input.chunks_exact(8);
        let chunks_out = output.chunks_exact_mut(8);
        let remainder_start = chunks_in.remainder().len();

        // Process 8 bytes (32 Quats) at a time using 64-bit CPU registers
        for (in_chunk, out_chunk) in chunks_in.zip(chunks_out) {
            let val = u64::from_le_bytes(in_chunk.try_into().unwrap());
            // Since each byte is 4 quats, bitwise NOT (~val) inverts 00->11 (0->3), 01->10 (1->2), etc.
            // This perfectly calculates (3 - x) for all packed bits simultaneously!
            let neg_val = !val;
            out_chunk.copy_from_slice(&neg_val.to_le_bytes());
        }

        // Process remaining tail bytes
        let tail_start = input.len() - remainder_start;
        for i in tail_start..input.len() {
            output[i] = !input[i];
        }
    }

    /// Fast element-wise Minimum (Min/AND in Łukasiewicz logic) for raw packed slices
    pub fn bulk_min(a: &[u8], b: &[u8], output: &mut [u8]) {
        assert!(a.len() == b.len() && a.len() == output.len());

        for i in 0..a.len() {
            let byte_a = a[i];
            let byte_b = b[i];

            // Extract pairs of bits, compute min, and rebuild packed byte
            let q0_a = byte_a & 0b11;
            let q0_b = byte_b & 0b11;
            let res0 = if q0_a <= q0_b { q0_a } else { q0_b };

            let q1_a = (byte_a >> 2) & 0b11;
            let q1_b = (byte_b >> 2) & 0b11;
            let res1 = if q1_a <= q1_b { q1_a } else { q1_b };

            let q2_a = (byte_a >> 4) & 0b11;
            let q2_b = (byte_b >> 4) & 0b11;
            let res2 = if q2_a <= q2_b { q2_a } else { q2_b };

            let q3_a = (byte_a >> 6) & 0b11;
            let q3_b = (byte_b >> 6) & 0b11;
            let res3 = if q3_a <= q3_b { q3_a } else { q3_b };

            output[i] = res0 | (res1 << 2) | (res2 << 4) | (res3 << 6);
        }
    }
}

/// Extension trait for QuatVec to easily invoke SIMD operations
impl QuatVec {
    /// In-place fast inversion of all quaternary digits in the vector
    pub fn invert_all(&mut self) {
        let raw = self.raw_data_mut();
        for byte in raw.iter_mut() {
            *byte = !(*byte);
        }
    }
}