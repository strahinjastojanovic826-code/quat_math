//! # Quat Math Advanced Library
//! 
//! An advanced library providing 4-state quaternary logic, modular arithmetic,
//! fixed-word packing, and high-performance packed dynamic vectors (`QuatVec`).

pub mod error;
pub mod quat;
pub mod logic;
pub mod arithmetic;
pub mod packed;
pub mod vector;
pub mod ffi;
pub mod simd;
pub mod algebra;

pub use error::QuatError;
pub use quat::Quat;
pub use packed::QuatWord;
pub use vector::QuatVec;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ffi::quat_add;
    use crate::ffi::quat_vec_free;
    use crate::ffi::quat_vec_get_data_buffer;
    use crate::ffi::quat_vec_len;
    use crate::ffi::quat_vec_new;
    use crate::ffi::quat_vec_push_bulk;
    use std::time::Instant;

    // =========================================================================
    // LEVEL 1: BASIC CREATION, CONSTANTS & VALIDATION (Easiest)
    // =========================================================================

    #[test]
    fn test_quat_constants() {
        let start = Instant::now();
        let iterations = 100_000;
        
        for _ in 0..iterations {
            assert_eq!(Quat::ZERO.value(), 0);
            assert_eq!(Quat::ONE.value(), 1);
            assert_eq!(Quat::TWO.value(), 2);
            assert_eq!(Quat::THREE.value(), 3);
        }
        
        println!("\n[BENCHMARK] test_quat_constants ({} iter): {:?}", iterations, start.elapsed());
    }

    #[test]
    fn test_quat_creation_and_validation() {
        let start = Instant::now();
        let iterations = 50_000;

        for _ in 0..iterations {
            let q = Quat::new(2);
            assert!(q.is_ok());
            assert_eq!(q.unwrap().value(), 2);

            let q_try = Quat::try_from(3);
            assert!(q_try.is_ok());
            assert_eq!(q_try.unwrap().value(), 3);

            let invalid = Quat::new(4);
            assert!(invalid.is_err());
            assert_eq!(invalid.unwrap_err(), QuatError::InvalidValue(4));

            let invalid_try = Quat::try_from(255);
            assert!(invalid_try.is_err());
        }

        println!("\n[BENCHMARK] test_quat_creation_and_validation ({} iter): {:?}", iterations, start.elapsed());
    }

    // ==========================================================
    // LEVEL 2: LOGIC & MODULO 4 ARITHMETIC (Intermediate)
    // ==========================================================

    #[test]
    fn test_quat_bitwise_logic() {
        let start = Instant::now();
        let iterations = 50_000;

        for _ in 0..iterations {
            let q1 = Quat::try_from(0b01).unwrap();
            let q2 = Quat::try_from(0b10).unwrap();

            let not_q1 = !q1;
            assert_eq!(not_q1.value(), 2);

            let and_res = q1 & q2;
            assert_eq!(and_res.value(), 0);

            let or_res = q1 | q2;
            assert_eq!(or_res.value(), 3);

            let xor_res = q1 ^ q2;
            assert_eq!(xor_res.value(), 3);
        }

        println!("\n[BENCHMARK] test_quat_bitwise_logic ({} iter): {:?}", iterations, start.elapsed());
    }

    #[test]
    fn test_quat_modular_arithmetic() {
        let start = Instant::now();
        let iterations = 50_000;

        for _ in 0..iterations {
            let two = Quat::try_from(2).unwrap();
            let three = Quat::try_from(3).unwrap();

            let sum = two + three;
            assert_eq!(sum.value(), 1);

            let diff = two - three;
            assert_eq!(diff.value(), 3);

            let prod = two * three;
            assert_eq!(prod.value(), 2);
        }

        println!("\n[BENCHMARK] test_quat_modular_arithmetic ({} iter): {:?}", iterations, start.elapsed());
    }

    // ==========================================================
    // LEVEL 3: FIXED-WORD PACKING (QuatWord) (Advanced)
    // ==========================================================

    #[test]
    fn test_quat_word_packing() {
        let start = Instant::now();
        let iterations = 50_000;

        for _ in 0..iterations {
            let q0 = Quat::ZERO;
            let q1 = Quat::ONE;
            let q2 = Quat::TWO;
            let q3 = Quat::THREE;

            let mut word = QuatWord::new(q0, q1, q2, q3);

            assert_eq!(word.get(0).unwrap(), q0);
            assert_eq!(word.get(1).unwrap(), q1);
            assert_eq!(word.get(2).unwrap(), q2);
            assert_eq!(word.get(3).unwrap(), q3);

            word.set(1, Quat::THREE).unwrap();
            assert_eq!(word.get(1).unwrap(), Quat::THREE);

            assert!(word.get(4).is_err());
            assert!(word.set(4, Quat::ZERO).is_err());
        }

        println!("\n[BENCHMARK] test_quat_word_packing ({} iter): {:?}", iterations, start.elapsed());
    }

    // ==========================================================
    // LEVEL 4: DYNAMIC PACKED VECTORS & ITERATORS (Hard)
    // ==========================================================

    #[test]
    fn test_quat_vec_operations() {
        let start = Instant::now();
        let iterations = 10_000;

        for _ in 0..iterations {
            let mut qvec = QuatVec::new();
            assert!(qvec.is_empty());
            assert_eq!(qvec.len(), 0);

            qvec.push(Quat::ZERO);
            qvec.push(Quat::ONE);
            qvec.push(Quat::TWO);
            qvec.push(Quat::THREE);

            assert_eq!(qvec.len(), 4);
            assert!(!qvec.is_empty());

            assert_eq!(qvec.get(0).unwrap(), Quat::ZERO);
            assert_eq!(qvec.get(1).unwrap(), Quat::ONE);
            assert_eq!(qvec.get(2).unwrap(), Quat::TWO);
            assert_eq!(qvec.get(3).unwrap(), Quat::THREE);

            qvec.set(0, Quat::THREE).unwrap();
            assert_eq!(qvec.get(0).unwrap(), Quat::THREE);

            assert!(qvec.get(4).is_err());
            assert!(qvec.set(100, Quat::ZERO).is_err());
        }

        println!("\n[BENCHMARK] test_quat_vec_operations ({} iter): {:?}", iterations, start.elapsed());
    }

    #[test]
    fn test_quat_vec_from_iterator() {
        let start = Instant::now();
        let iterations = 10_000;

        let raw_elements = vec![Quat::ZERO, Quat::THREE, Quat::TWO, Quat::ONE, Quat::ZERO];

        for _ in 0..iterations {
            let qvec: QuatVec = raw_elements.clone().into_iter().collect();
            assert_eq!(qvec.len(), raw_elements.len());
            for (i, &expected) in raw_elements.iter().enumerate() {
                assert_eq!(qvec.get(i).unwrap(), expected);
            }
        }

        println!("\n[BENCHMARK] test_quat_vec_from_iterator ({} iter): {:?}", iterations, start.elapsed());
    }

    // ==========================================================
    // LEVEL 5: STRESS TEST (10,000 Elements Integrity Check)
    // ==========================================================

    #[test]
    fn test_stress_quat_vec_large_scale() {
        let start = Instant::now();
        const STRESS_COUNT: usize = 1_000; // Smanjeno na 1,000 radi bržeg benchmark merenja u petlji
        let iterations = 100;

        for _ in 0..iterations {
            let mut qvec = QuatVec::with_capacity(STRESS_COUNT);

            for i in 0..STRESS_COUNT {
                let val = (i % 4) as u8;
                let q = Quat::try_from(val).unwrap();
                qvec.push(q);
            }

            assert_eq!(qvec.len(), STRESS_COUNT);

            for i in 0..STRESS_COUNT {
                let expected_val = (i % 4) as u8;
                let fetched = qvec.get(i).unwrap();
                assert_eq!(fetched.value(), expected_val);
            }

            for i in 0..STRESS_COUNT {
                let new_val = ((i + 2) % 4) as u8;
                let q = Quat::try_from(new_val).unwrap();
                qvec.set(i, q).unwrap();
            }
        }

        println!("\n[BENCHMARK] test_stress_quat_vec_large_scale ({} iter of 1k elements): {:?}", iterations, start.elapsed());
    }

    #[test]
    fn test_ffi_quat_arithmetic() {
        let start = Instant::now();
        let iterations = 50_000;

        for _ in 0..iterations {
            let mut out_val: u8 = 0;
            let status = unsafe { quat_add(1, 1, &mut out_val as *mut u8) };
            assert_eq!(status, 0);
            assert_eq!(out_val, 2);
        }

        println!("\n[BENCHMARK] test_ffi_quat_arithmetic ({} iter): {:?}", iterations, start.elapsed());
    }

    #[test]
    fn test_ffi_quat_vector_lifecycle() {
        let start = Instant::now();
        let iterations = 5_000;

        for _ in 0..iterations {
            unsafe {
                let vec_ptr = quat_vec_new();
                assert!(!vec_ptr.is_null());

                let raw_values: [u8; 4] = [0, 1, 2, 3];
                let push_status = quat_vec_push_bulk(vec_ptr, raw_values.as_ptr(), raw_values.len());
                assert_eq!(push_status, 0);

                let mut length: usize = 0;
                let len_status = quat_vec_len(vec_ptr, &mut length as *mut usize);
                assert_eq!(len_status, 0);
                assert_eq!(length, 4);

                let mut buffer_ptr: *const u8 = std::ptr::null();
                let mut buffer_size: usize = 0;
                let buf_status = quat_vec_get_data_buffer(
                    vec_ptr,
                    &mut buffer_ptr as *mut *const u8,
                    &mut buffer_size as *mut usize,
                );
                assert_eq!(buf_status, 0);
                assert_eq!(*buffer_ptr, 0b11100100);

                quat_vec_free(vec_ptr);
            }
        }

        println!("\n[BENCHMARK] test_ffi_quat_vector_lifecycle ({} iter): {:?}", iterations, start.elapsed());
    }

    #[test]
    fn test_quaternary_algebra() {
        use crate::algebra::*;
        let start = Instant::now();
        let iterations = 50_000;

        for _ in 0..iterations {
            let q0 = Quat::ZERO;
            let q1 = Quat::ONE;
            let q2 = Quat::TWO;
            let q3 = Quat::THREE;

            assert_eq!(q1.min(q3), q1);
            assert_eq!(q1.max(q3), q3);
            assert_eq!(q0.cycle(), q1);
            assert_eq!(q3.cycle(), q0);
            assert_eq!(q0.post_neg(), q3);
            assert_eq!(q1.post_neg(), q2);
            assert_eq!(q3.lukasiewicz_impl(q1), q1);
            assert_eq!(q2.literal(2), Quat::THREE);
        }

        println!("\n[BENCHMARK] test_quaternary_algebra ({} iter): {:?}", iterations, start.elapsed());
    }

    #[test]
    fn test_simd_bulk_operations() {
        use crate::simd::QuatSimd;
        let start = Instant::now();
        let iterations = 50_000;

        let input = vec![0b11100100u8; 8];
        let mut output = vec![0u8; 8];
        let vec_a = vec![0b11_10_01_00u8];
        let vec_b = vec![0b01_11_00_10u8];
        let mut res_min = vec![0u8];

        for _ in 0..iterations {
            QuatSimd::bulk_post_neg(&input, &mut output);
            QuatSimd::bulk_min(&vec_a, &vec_b, &mut res_min);
        }

        println!("\n[BENCHMARK] test_simd_bulk_operations ({} iter): {:?}", iterations, start.elapsed());
    }

    #[test]
    fn test_new_algebra_features() {
        let start = Instant::now();
        let iterations = 50_000;

        for _ in 0..iterations {
            let q1 = Quat::ONE;
            let q2 = Quat::TWO;
            let q0 = Quat::ZERO;
            let q3 = Quat::THREE;

            assert_eq!(q1.godel_impl(q2).value(), 3);
            assert_eq!(q2.godel_impl(q1).value(), 1);
            assert_eq!(q1.goguen_impl(q2).value(), 3);
            assert_eq!(q1.kleene_dienes_impl(q2).value(), 2);

            assert_eq!(q1.t_norm_min(q2).value(), 1);
            assert_eq!(q1.t_norm_product(q2).value(), 0);
            assert_eq!(q1.t_norm_lukasiewicz(q2).value(), 0);
            assert_eq!(q1.s_norm_lukasiewicz(q2).value(), 3);

            assert_eq!(q3.necessity().value(), 3);
            assert_eq!(q1.necessity().value(), 0);
            assert_eq!(q0.possibility().value(), 0);
            assert_eq!(q2.possibility().value(), 3);
        }

        println!("\n[BENCHMARK] test_new_algebra_features ({} iter): {:?}", iterations, start.elapsed());
    }

    #[test]
    fn test_inline_performance_benchmark() {
        use crate::simd::QuatSimd;
        let start = Instant::now();
        let iterations = 10_000;

        let input = vec![0b11100100u8; 1000];
        let mut output = vec![0u8; 1000];

        for _ in 0..iterations {
            #[cfg(target_arch = "x86_64")]
            {
                if std::is_x86_feature_detected!("avx2") {
                    unsafe {
                        QuatSimd::bulk_post_neg_avx2(&input, &mut output);
                    }
                } else {
                    QuatSimd::bulk_post_neg(&input, &mut output);
                }
            }
            #[cfg(not(target_arch = "x86_64"))]
            {
                QuatSimd::bulk_post_neg(&input, &mut output);
            }
        }

        println!("\n[BENCHMARK] test_inline_performance_benchmark ({} iter): {:?}", iterations, start.elapsed());
        assert_eq!(output.len(), input.len());
    }

}
