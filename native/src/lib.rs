use std::ffi::{c_char, CStr, CString};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::ptr;
use std::time::{SystemTime, UNIX_EPOCH};

pub const HXRUST_VERSION: &str = "0.1.0";

fn into_c_string(value: &str) -> *mut c_char {
    CString::new(value).map(CString::into_raw).unwrap_or(ptr::null_mut())
}

fn ffi_string<F: FnOnce() -> Option<String>>(f: F) -> *mut c_char {
    catch_unwind(AssertUnwindSafe(f))
        .ok()
        .flatten()
        .map(|s| into_c_string(&s))
        .unwrap_or(ptr::null_mut())
}

unsafe fn read_string<'a>(value: *const c_char) -> Option<&'a str> {
    if value.is_null() { return None; }
    CStr::from_ptr(value).to_str().ok()
}

unsafe fn read_array<'a>(data: *const f64, len: usize) -> Option<&'a [f64]> {
    if len == 0 { return Some(&[]); }
    if data.is_null() || len > isize::MAX as usize / std::mem::size_of::<f64>() { return None; }
    Some(std::slice::from_raw_parts(data, len))
}

#[no_mangle]
pub extern "C" fn hxrust_api_version() -> u32 { 1 }

#[no_mangle]
pub extern "C" fn hxrust_is_available() -> bool { true }

#[no_mangle]
pub extern "C" fn hxrust_version() -> *mut c_char { into_c_string(HXRUST_VERSION) }

#[no_mangle]
pub extern "C" fn hxrust_platform() -> *mut c_char { into_c_string(std::env::consts::OS) }

#[no_mangle]
pub extern "C" fn hxrust_architecture() -> *mut c_char { into_c_string(std::env::consts::ARCH) }

#[no_mangle]
pub extern "C" fn hxrust_add(a: f64, b: f64) -> f64 { a + b }
#[no_mangle]
pub extern "C" fn hxrust_subtract(a: f64, b: f64) -> f64 { a - b }
#[no_mangle]
pub extern "C" fn hxrust_multiply(a: f64, b: f64) -> f64 { a * b }
#[no_mangle]
pub extern "C" fn hxrust_divide(a: f64, b: f64) -> f64 { if b == 0.0 { f64::NAN } else { a / b } }
#[no_mangle]
pub extern "C" fn hxrust_power(a: f64, b: f64) -> f64 { a.powf(b) }
#[no_mangle]
pub extern "C" fn hxrust_sqrt(a: f64) -> f64 { a.sqrt() }
#[no_mangle]
pub extern "C" fn hxrust_abs(a: f64) -> f64 { a.abs() }
#[no_mangle]
pub extern "C" fn hxrust_min(a: f64, b: f64) -> f64 { a.min(b) }
#[no_mangle]
pub extern "C" fn hxrust_max(a: f64, b: f64) -> f64 { a.max(b) }
#[no_mangle]
pub extern "C" fn hxrust_clamp(value: f64, minimum: f64, maximum: f64) -> f64 {
    if minimum > maximum || minimum.is_nan() || maximum.is_nan() { f64::NAN }
    else { value.max(minimum).min(maximum) }
}
#[no_mangle]
pub extern "C" fn hxrust_lerp(a: f64, b: f64, t: f64) -> f64 { a + (b - a) * t }
#[no_mangle]
pub extern "C" fn hxrust_sin(a: f64) -> f64 { a.sin() }
#[no_mangle]
pub extern "C" fn hxrust_cos(a: f64) -> f64 { a.cos() }
#[no_mangle]
pub extern "C" fn hxrust_tan(a: f64) -> f64 { a.tan() }
#[no_mangle]
pub extern "C" fn hxrust_ln(a: f64) -> f64 { a.ln() }
#[no_mangle]
pub extern "C" fn hxrust_exp(a: f64) -> f64 { a.exp() }
#[no_mangle]
pub extern "C" fn hxrust_floor(a: f64) -> f64 { a.floor() }
#[no_mangle]
pub extern "C" fn hxrust_ceil(a: f64) -> f64 { a.ceil() }
#[no_mangle]
pub extern "C" fn hxrust_round(a: f64) -> f64 { a.round() }
#[no_mangle]
pub extern "C" fn hxrust_distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 { (x2 - x1).hypot(y2 - y1) }
#[no_mangle]
pub extern "C" fn hxrust_is_nan(a: f64) -> bool { a.is_nan() }
#[no_mangle]
pub extern "C" fn hxrust_is_finite(a: f64) -> bool { a.is_finite() }

#[no_mangle]
pub extern "C" fn hxrust_string_new(data: *const u8, length: usize) -> *mut c_char {
    if length == 0 { return into_c_string(""); }
    if data.is_null() || length > isize::MAX as usize { return ptr::null_mut(); }
    catch_unwind(AssertUnwindSafe(|| {
        let bytes = unsafe { std::slice::from_raw_parts(data, length) };
        CString::new(bytes).map(CString::into_raw).unwrap_or(ptr::null_mut())
    })).unwrap_or(ptr::null_mut())
}

/// # Safety
/// `value` must be null or a pointer returned by hxrust that has not yet been freed.
#[no_mangle]
pub unsafe extern "C" fn hxrust_string_free(value: *mut c_char) {
    if !value.is_null() { drop(CString::from_raw(value)); }
}

/// # Safety
/// `value` must point to a valid, NUL-terminated C string or be null.
#[no_mangle]
pub unsafe extern "C" fn hxrust_string_length(value: *const c_char) -> usize {
    if value.is_null() { 0 } else { CStr::from_ptr(value).to_bytes().len() }
}

/// # Safety
/// Both pointers must be null or point to valid NUL-terminated C strings.
#[no_mangle]
pub unsafe extern "C" fn hxrust_string_equals(a: *const c_char, b: *const c_char) -> bool {
    if a.is_null() || b.is_null() { return false; }
    CStr::from_ptr(a) == CStr::from_ptr(b)
}

/// # Safety
/// Both pointers must be null or point to valid NUL-terminated C strings.
#[no_mangle]
pub unsafe extern "C" fn hxrust_string_concat(a: *const c_char, b: *const c_char) -> *mut c_char {
    ffi_string(|| Some(format!("{}{}", read_string(a)?, read_string(b)?)))
}

/// # Safety
/// `value` must be null or a valid NUL-terminated UTF-8 C string.
#[no_mangle]
pub unsafe extern "C" fn hxrust_string_upper(value: *const c_char) -> *mut c_char {
    ffi_string(|| Some(read_string(value)?.to_uppercase()))
}

/// # Safety
/// `value` must be null or a valid NUL-terminated UTF-8 C string.
#[no_mangle]
pub unsafe extern "C" fn hxrust_string_lower(value: *const c_char) -> *mut c_char {
    ffi_string(|| Some(read_string(value)?.to_lowercase()))
}

/// # Safety
/// `value` must be null or a valid NUL-terminated UTF-8 C string.
#[no_mangle]
pub unsafe extern "C" fn hxrust_string_trim(value: *const c_char) -> *mut c_char {
    ffi_string(|| Some(read_string(value)?.trim().to_owned()))
}

/// # Safety
/// `value` must be null or a valid NUL-terminated UTF-8 C string.
#[no_mangle]
pub unsafe extern "C" fn hxrust_string_reverse(value: *const c_char) -> *mut c_char {
    ffi_string(|| Some(read_string(value)?.chars().rev().collect()))
}

/// # Safety
/// All pointers must be null or valid NUL-terminated UTF-8 C strings.
#[no_mangle]
pub unsafe extern "C" fn hxrust_string_replace(value: *const c_char, from: *const c_char, to: *const c_char) -> *mut c_char {
    ffi_string(|| Some(read_string(value)?.replace(read_string(from)?, read_string(to)?)))
}

/// # Safety
/// `data` must be null with length zero, or point to `length` readable f64 values.
#[no_mangle]
pub unsafe extern "C" fn hxrust_array_sum(data: *const f64, length: usize) -> f64 {
    match read_array(data, length) { Some(values) => values.iter().sum(), None => f64::NAN }
}

/// # Safety
/// `data` must be null with length zero, or point to `length` readable f64 values.
#[no_mangle]
pub unsafe extern "C" fn hxrust_array_average(data: *const f64, length: usize) -> f64 {
    match read_array(data, length) {
        Some(values) if !values.is_empty() => values.iter().sum::<f64>() / values.len() as f64,
        _ => f64::NAN,
    }
}

/// # Safety
/// `data` must be null with length zero, or point to `length` readable f64 values.
#[no_mangle]
pub unsafe extern "C" fn hxrust_array_min(data: *const f64, length: usize) -> f64 {
    match read_array(data, length) {
        Some(values) if !values.is_empty() => values.iter().copied().reduce(f64::min).unwrap_or(f64::NAN),
        _ => f64::NAN,
    }
}

/// # Safety
/// `data` must be null with length zero, or point to `length` readable f64 values.
#[no_mangle]
pub unsafe extern "C" fn hxrust_array_max(data: *const f64, length: usize) -> f64 {
    match read_array(data, length) {
        Some(values) if !values.is_empty() => values.iter().copied().reduce(f64::max).unwrap_or(f64::NAN),
        _ => f64::NAN,
    }
}

/// # Safety
/// `data` must be null with length zero, or point to `length` readable bytes.
#[no_mangle]
pub unsafe extern "C" fn hxrust_hash_fnv1a(data: *const u8, length: usize) -> u64 {
    if length != 0 && (data.is_null() || length > isize::MAX as usize) { return 0; }
    let bytes = if length == 0 { &[][..] } else { std::slice::from_raw_parts(data, length) };
    let mut hash = 0xcbf29ce484222325u64;
    for &byte in bytes { hash = (hash ^ byte as u64).wrapping_mul(0x100000001b3); }
    hash
}

/// # Safety
/// `data` must be null with length zero, or point to `length` readable bytes.
#[no_mangle]
pub unsafe extern "C" fn hxrust_crc32(data: *const u8, length: usize) -> u32 {
    if length != 0 && (data.is_null() || length > isize::MAX as usize) { return 0; }
    let bytes = if length == 0 { &[][..] } else { std::slice::from_raw_parts(data, length) };
    let mut crc = !0u32;
    for &byte in bytes {
        crc ^= byte as u32;
        for _ in 0..8 { crc = (crc >> 1) ^ (0xedb88320u32 & (0u32.wrapping_sub(crc & 1))); }
    }
    !crc
}

#[no_mangle]
pub extern "C" fn hxrust_timestamp_ms() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_millis().min(u64::MAX as u128) as u64).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn math() {
        assert_eq!(hxrust_add(2.0, 3.0), 5.0);
        assert!(hxrust_divide(1.0, 0.0).is_nan());
        assert_eq!(hxrust_clamp(12.0, 0.0, 10.0), 10.0);
        assert_eq!(hxrust_distance(0.0, 0.0, 3.0, 4.0), 5.0);
    }
    #[test]
    fn strings() {
        let a = CString::new("Olá ").unwrap();
        let b = CString::new("Rust").unwrap();
        unsafe {
            let value = hxrust_string_concat(a.as_ptr(), b.as_ptr());
            assert_eq!(CStr::from_ptr(value).to_str().unwrap(), "Olá Rust");
            hxrust_string_free(value);
        }
    }
    #[test]
    fn arrays_and_hashes() {
        let values = [2.0, 4.0, 6.0];
        unsafe {
            assert_eq!(hxrust_array_sum(values.as_ptr(), values.len()), 12.0);
            assert_eq!(hxrust_array_average(values.as_ptr(), values.len()), 4.0);
            assert_eq!(hxrust_crc32(b"123456789".as_ptr(), 9), 0xcbf43926);
            assert_eq!(hxrust_hash_fnv1a(b"".as_ptr(), 0), 0xcbf29ce484222325);
        }
    }
}
