use crate::quat::Quat;
use crate::vector::QuatVec;
use std::os::raw::{c_int, c_uchar};
use std::panic::catch_unwind;

// ==========================================
// 1. Bezbedne aritmetičke operacije (sa catch_unwind)
// ==========================================

#[unsafe(no_mangle)]
pub extern "C" fn quat_add(a: c_uchar, b: c_uchar, out: *mut c_uchar) -> c_int {
    let result = catch_unwind(|| {
        unsafe {
        let q1 = Quat::new_unchecked(a);
        let q2 = Quat::new_unchecked(b);
        (q1 + q2).value()
       }
    });

    match result {
        Ok(val) => {
            unsafe { *out = val; }
            0
        }
        Err(_) => -1, // Došlo je do panike u Rust-u
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn quat_mul(a: c_uchar, b: c_uchar, out: *mut c_uchar) -> c_int {
    let result = catch_unwind(|| {
        unsafe {
        let q1 = Quat::new_unchecked(a);
        let q2 = Quat::new_unchecked(b);
        (q1 * q2).value()
        }
    });

    match result {
        Ok(val) => {
            unsafe { *out = val; }
            0
        }
        Err(_) => -1,
    }
}

// ==========================================
// 2. Upravljanje QuatVec kontejnerom
// ==========================================

#[unsafe(no_mangle)]
pub extern "C" fn quat_vec_new() -> *mut QuatVec {
    Box::into_raw(Box::new(QuatVec::new()))
}

#[unsafe(no_mangle)]
pub extern "C" fn quat_vec_free(ptr: *mut QuatVec) {
    if !ptr.is_null() {
        unsafe { let _ = Box::from_raw(ptr); }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn quat_vec_len(ptr: *const QuatVec, out_len: *mut usize) -> c_int {
    if ptr.is_null() || out_len.is_null() { return -1; }
    unsafe {
        *out_len = (*ptr).len();
    }
    0
}

/// Bezbedno dodavanje niza vrednosti odjednom (Bulk push)
#[unsafe(no_mangle)]
pub extern "C" fn quat_vec_push_bulk(ptr: *mut QuatVec, data: *const c_uchar, len: usize) -> c_int {
    if ptr.is_null() || data.is_null() { return -1; }
    
    let result = catch_unwind(|| {
        let slice = unsafe { std::slice::from_raw_parts(data, len) };
        let vec = unsafe { &mut *ptr };
        for &val in slice {
            let q = Quat::new(val)?;
            vec.push(q);
        }
        Ok::<(), crate::error::QuatError>(())
    });

    match result {
        Ok(Ok(())) => 0,
        _ => -1,
    }
}

// ==========================================
// 3. Masovni pristup memoriji za NumPy / C++
// ==========================================

/// Vraća pokazivač na unutrašnji bafer i njegovu dužinu u bajtovima.
/// Spoljni jezik može direktno da čita memoriju bez kopiranja (Zero-Copy).
#[unsafe(no_mangle)]
pub extern "C" fn quat_vec_get_data_buffer(
    ptr: *const QuatVec, 
    out_data: *mut *const c_uchar, 
    out_size: *mut usize
) -> c_int {
    if ptr.is_null() || out_data.is_null() || out_size.is_null() {
        return -1;
    }
    unsafe {
        let vec = &*ptr;
        let raw = vec.raw_data();
        *out_data = raw.as_ptr();
        *out_size = raw.len();
    }
    0
}