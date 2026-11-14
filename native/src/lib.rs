use std::ffi::{c_char, CStr, CString};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::ptr;

pub const HXRUST_VERSION: &str = "0.1.0";

#[no_mangle]
pub extern "C" fn hxrust_version() -> *mut c_char {
    hxrust_string_new(HXRUST_VERSION.as_ptr(), HXRUST_VERSION.len())
}

#[no_mangle]
pub extern "C" fn hxrust_api_version() -> u32 {
    1
}

#[no_mangle]
pub extern "C" fn hxrust_is_available() -> bool {
    true
}

#[no_mangle]
pub extern "C" fn hxrust_add(a: f64, b: f64) -> f64 {
    a + b
}

#[no_mangle]
pub extern "C" fn hxrust_subtract(a: f64, b: f64) -> f64 {
    a - b
}

#[no_mangle]
pub extern "C" fn hxrust_multiply(a: f64, b: f64) -> f64 {
    a * b
}

#[no_mangle]
pub extern "C" fn hxrust_divide(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        f64::NAN
    } else {
        a / b
    }
}

#[no_mangle]
pub extern "C" fn hxrust_power(base: f64, exponent: f64) -> f64 {
    base.powf(exponent)
}

#[no_mangle]
pub extern "C" fn hxrust_sqrt(value: f64) -> f64 {
    value.sqrt()
}

#[no_mangle]
pub extern "C" fn hxrust_abs(value: f64) -> f64 {
    value.abs()
}

#[no_mangle]
pub extern "C" fn hxrust_min(a: f64, b: f64) -> f64 {
    a.min(b)
}

#[no_mangle]
pub extern "C" fn hxrust_max(a: f64, b: f64) -> f64 {
    a.max(b)
}

#[no_mangle]
pub extern "C" fn hxrust_clamp(
    value: f64,
    minimum: f64,
    maximum: f64,
) -> f64 {
    if minimum > maximum {
        return f64::NAN;
    }

    value.max(minimum).min(maximum)
}

#[no_mangle]
pub extern "C" fn hxrust_lerp(
    start: f64,
    end: f64,
    ratio: f64,
) -> f64 {
    start + (end - start) * ratio
}

#[no_mangle]
pub extern "C" fn hxrust_string_new(
    data: *const u8,
    length: usize,
) -> *mut c_char {
    if data.is_null() && length != 0 {
        return ptr::null_mut();
    }

    let result = catch_unwind(AssertUnwindSafe(|| {
        let bytes = if length == 0 {
            &[][..]
        } else {
            unsafe { std::slice::from_raw_parts(data, length) }
        };

        CString::new(bytes)
            .map(CString::into_raw)
            .unwrap_or(ptr::null_mut())
    }));

    result.unwrap_or(ptr::null_mut())
}

#[no_mangle]
pub unsafe extern "C" fn hxrust_string_free(
    value: *mut c_char,
) {
    if value.is_null() {
        return;
    }

    drop(CString::from_raw(value));
}

#[no_mangle]
pub unsafe extern "C" fn hxrust_string_length(
    value: *const c_char,
) -> usize {
    if value.is_null() {
        return 0;
    }

    CStr::from_ptr(value).to_bytes().len()
}

#[no_mangle]
pub unsafe extern "C" fn hxrust_string_equals(
    first: *const c_char,
    second: *const c_char,
) -> bool {
    if first.is_null() || second.is_null() {
        return false;
    }

    CStr::from_ptr(first) == CStr::from_ptr(second)
}

#[no_mangle]
pub unsafe extern "C" fn hxrust_string_concat(
    first: *const c_char,
    second: *const c_char,
) -> *mut c_char {
    if first.is_null() || second.is_null() {
        return ptr::null_mut();
    }

    let result = catch_unwind(AssertUnwindSafe(|| {
        let a = CStr::from_ptr(first).to_bytes();
        let b = CStr::from_ptr(second).to_bytes();

        let mut buffer = Vec::with_capacity(a.len() + b.len());
        buffer.extend_from_slice(a);
        buffer.extend_from_slice(b);

        CString::new(buffer)
            .map(CString::into_raw)
            .unwrap_or(ptr::null_mut())
    }));

    result.unwrap_or(ptr::null_mut())
}

#[no_mangle]
pub extern "C" fn hxrust_platform() -> *mut c_char {
    let platform = std::env::consts::OS;

    hxrust_string_new(platform.as_ptr(), platform.len())
}

#[no_mangle]
pub extern "C" fn hxrust_architecture() -> *mut c_char {
    let architecture = std::env::consts::ARCH;

    hxrust_string_new(architecture.as_ptr(), architecture.len())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CStr;

    #[test]
    fn test_version() {
        assert_eq!(hxrust_api_version(), 1);
        assert!(hxrust_is_available());
    }

    #[test]
    fn test_math() {
        assert_eq!(hxrust_add(5.0, 3.0), 8.0);
        assert_eq!(hxrust_subtract(5.0, 3.0), 2.0);
        assert_eq!(hxrust_multiply(5.0, 3.0), 15.0);
        assert_eq!(hxrust_divide(10.0, 2.0), 5.0);
        assert!(hxrust_divide(10.0, 0.0).is_nan());
        assert_eq!(hxrust_clamp(15.0, 0.0, 10.0), 10.0);
    }

    #[test]
    fn test_strings() {
        let text = b"Hello Rust";
        let value = hxrust_string_new(text.as_ptr(), text.len());

        assert!(!value.is_null());

        unsafe {
            assert_eq!(hxrust_string_length(value), 10);
            assert_eq!(
                CStr::from_ptr(value).to_str().unwrap(),
                "Hello Rust"
            );

            hxrust_string_free(value);
        }
    }

    #[test]
    fn test_concat() {
        let a = CString::new("Hello ").unwrap();
        let b = CString::new("Haxe").unwrap();

        let result = unsafe {
            hxrust_string_concat(a.as_ptr(), b.as_ptr())
        };

        assert!(!result.is_null());

        unsafe {
            assert_eq!(
                CStr::from_ptr(result).to_str().unwrap(),
                "Hello Haxe"
            );

            hxrust_string_free(result);
        }
    }
}
