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

    // =========================================================================
    // LEVEL 1: BASIC CREATION, CONSTANTS & VALIDATION (Easiest)
    // =========================================================================

    #[test]
    fn test_quat_constants() {
        assert_eq!(Quat::ZERO.value(), 0);
        assert_eq!(Quat::ONE.value(), 1);
        assert_eq!(Quat::TWO.value(), 2);
        assert_eq!(Quat::THREE.value(), 3);
    }

    #[test]
    fn test_quat_creation_and_validation() {
        // Valid ranges (0 to 3)
        let q = Quat::new(2);
        assert!(q.is_ok());
        assert_eq!(q.unwrap().value(), 2);

        // TryFrom validation
        let q_try = Quat::try_from(3);
        assert!(q_try.is_ok());
        assert_eq!(q_try.unwrap().value(), 3);

        // Invalid ranges (>= 4)
        let invalid = Quat::new(4);
        assert!(invalid.is_err());
        assert_eq!(invalid.unwrap_err(), QuatError::InvalidValue(4));

        let invalid_try = Quat::try_from(255);
        assert!(invalid_try.is_err());
    }

    // =========================================================================
    // LEVEL 2: LOGIC & MODULO 4 ARITHMETIC (Intermediate)
    // =========================================================================

    #[test]
    fn test_quat_bitwise_logic() {
        let q1 = Quat::try_from(0b01).unwrap(); // 1
        let q2 = Quat::try_from(0b10).unwrap(); // 2

        // NOT (bitwise complement on 2 bits)
        let not_q1 = !q1; // !01 -> 10 (2)
        assert_eq!(not_q1.value(), 2);

        // AND
        let and_res = q1 & q2; // 01 & 10 -> 00 (0)
        assert_eq!(and_res.value(), 0);

        // OR
        let or_res = q1 | q2; // 01 | 10 -> 11 (3)
        assert_eq!(or_res.value(), 3);

        // XOR
        let xor_res = q1 ^ q2; // 01 ^ 10 -> 11 (3)
        assert_eq!(xor_res.value(), 3);
    }

    #[test]
    fn test_quat_modular_arithmetic() {
        let two = Quat::try_from(2).unwrap();
        let three = Quat::try_from(3).unwrap();

        // Addition modulo 4: (2 + 3) % 4 = 1
        let sum = two + three;
        assert_eq!(sum.value(), 1);

        // Subtraction modulo 4: (2 - 3) -> -1 -> rem_euclid(4) = 3
        let diff = two - three;
        assert_eq!(diff.value(), 3);

        // Multiplication modulo 4: (2 * 3) % 4 = 2
        let prod = two * three;
        assert_eq!(prod.value(), 2);
    }

    // =========================================================================
    // LEVEL 3: FIXED-WORD PACKING (QuatWord) (Advanced)
    // =========================================================================

    #[test]
    fn test_quat_word_packing() {
        let q0 = Quat::ZERO;
        let q1 = Quat::ONE;
        let q2 = Quat::TWO;
        let q3 = Quat::THREE;

        let mut word = QuatWord::new(q0, q1, q2, q3);

        // Verify retrieval
        assert_eq!(word.get(0).unwrap(), q0);
        assert_eq!(word.get(1).unwrap(), q1);
        assert_eq!(word.get(2).unwrap(), q2);
        assert_eq!(word.get(3).unwrap(), q3);

        // Test modification via set
        word.set(1, Quat::THREE).unwrap();
        assert_eq!(word.get(1).unwrap(), Quat::THREE);

        // Test out of bounds on word
        assert!(word.get(4).is_err());
        assert!(word.set(4, Quat::ZERO).is_err());
    }

    // =========================================================================
    // LEVEL 4: DYNAMIC PACKED VECTORS & ITERATORS (Hard)
    // =========================================================================

    #[test]
    fn test_quat_vec_operations() {
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

        // Mutate element
        qvec.set(0, Quat::THREE).unwrap();
        assert_eq!(qvec.get(0).unwrap(), Quat::THREE);

        // Out of bounds check
        assert!(qvec.get(4).is_err());
        assert!(qvec.set(100, Quat::ZERO).is_err());
    }

    #[test]
    fn test_quat_vec_from_iterator() {
        let raw_elements = vec![Quat::ZERO, Quat::THREE, Quat::TWO, Quat::ONE, Quat::ZERO];
        let qvec: QuatVec = raw_elements.clone().into_iter().collect();

        assert_eq!(qvec.len(), raw_elements.len());
        for (i, &expected) in raw_elements.iter().enumerate() {
            assert_eq!(qvec.get(i).unwrap(), expected);
        }
    }

    // =========================================================================
    // LEVEL 5: STRESS TEST (10,000 Elements Integrity Check) (Most Stressful)
    // =========================================================================

    #[test]
    fn test_stress_quat_vec_large_scale() {
        const STRESS_COUNT: usize = 10_000;
        let mut qvec = QuatVec::with_capacity(STRESS_COUNT);

        // Phase 1: Push 10,000 quaternary values sequentially
        for i in 0..STRESS_COUNT {
            let val = (i % 4) as u8;
            let q = Quat::try_from(val).expect("Valid value under 4");
            qvec.push(q);
        }

        assert_eq!(qvec.len(), STRESS_COUNT);

        // Phase 2: Verify all 10,000 elements match their expected values
        for i in 0..STRESS_COUNT {
            let expected_val = (i % 4) as u8;
            let fetched = qvec.get(i).expect("Index must exist");
            assert_eq!(
                fetched.value(), 
                expected_val, 
                "Mismatch at index {} during initial verification", i
            );
        }

        // Phase 3: Mutate all 10,000 elements (shift pattern)
        for i in 0..STRESS_COUNT {
            let new_val = ((i + 2) % 4) as u8;
            let q = Quat::try_from(new_val).unwrap();
            qvec.set(i, q).expect("Index must exist for setting");
        }

        // Phase 4: Verify modified pattern across all elements
        for i in 0..STRESS_COUNT {
            let expected_val = ((i + 2) % 4) as u8;
            let fetched = qvec.get(i).expect("Index must exist");
            assert_eq!(
                fetched.value(), 
                expected_val, 
                "Mismatch at index {} after mutation stress", i
            );
        }
    }
    #[test]
    fn test_ffi_quat_arithmetic() {
        let mut out_val: u8 = 0;
        
        // Sabiranje: 1 (ONE) + 1 (ONE) = 2 (TWO)
        let status = unsafe { quat_add(1, 1, &mut out_val as *mut u8) };
        assert_eq!(status, 0, "quat_add failed");
        assert_eq!(out_val, 2, "Expected Quat value 2 (TWO)");
    }

    #[test]
    fn test_ffi_quat_vector_lifecycle() {
        unsafe {
            // 1. Kreiranje novog vektora
            let vec_ptr = quat_vec_new();
            assert!(!vec_ptr.is_null(), "Failed to allocate QuatVec");

            // 2. Masovno dodavanje elemenata (Bulk push)
            // Vrednosti 0, 1, 2, 3 (odgovaraju rasponu od 0 do 3 za Quat)[cite: 2]
            let raw_values: [u8; 4] = [0, 1, 2, 3];
            let push_status = quat_vec_push_bulk(vec_ptr, raw_values.as_ptr(), raw_values.len());
            assert_eq!(push_status, 0, "quat_vec_push_bulk failed");

            // 3. Provera dužine vektora
            let mut length: usize = 0;
            let len_status = quat_vec_len(vec_ptr, &mut length as *mut usize);
            assert_eq!(len_status, 0, "quat_vec_len failed");
            assert_eq!(length, 4, "Expected vector length to be 4");

            // 4. Testiranje Zero-Copy pristupa sirovom memorijskom baferu
            let mut buffer_ptr: *const u8 = std::ptr::null();
            let mut buffer_size: usize = 0;
            let buf_status = quat_vec_get_data_buffer(
                vec_ptr, 
                &mut buffer_ptr as *mut *const u8, 
                &mut buffer_size as *mut usize
            );
            assert_eq!(buf_status, 0, "quat_vec_get_data_buffer failed");
            assert!(!buffer_ptr.is_null(), "Buffer pointer is null");
            assert!(buffer_size > 0, "Buffer size must be greater than 0");

            // Sigurno čitanje prvog bajta spakovanih podataka iz memorije
            let first_packed_byte = *buffer_ptr;
            assert_eq!(first_packed_byte, 0b11100100); // 4-to-1 byte packing format[cite: 5, 7]

            // 5. Bezbedno oslobađanje memorije
            quat_vec_free(vec_ptr);
        }
    }
}
