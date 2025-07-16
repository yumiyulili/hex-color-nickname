use std::ffi::CStr;
use std::ffi::CString;

use std::os::raw::c_char;

mod hex;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn encode(input: *const c_char) -> *mut c_char {
    let c_str = unsafe { CStr::from_ptr(input) };
    let str_slice = c_str.to_str().unwrap();
    let hex_string = hex::Hex::encode(str_slice);
    CString::new(hex_string).unwrap().into_raw()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn decode(input: *const c_char) -> *mut c_char {
    let c_str = unsafe { CStr::from_ptr(input) };
    let str_slice = c_str.to_str().unwrap();
    let decoded = hex::Hex::decode(str_slice);
    CString::new(decoded).unwrap().into_raw()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn hex_color_of(input: *const c_char) -> *mut c_char {
    let c_str = unsafe { CStr::from_ptr(input) };
    let str_slice = c_str.to_str().unwrap();
    let color = hex::Hex::color_from_str(str_slice);
    CString::new(color).unwrap().into_raw()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn alt_hex_color_of(input: *const c_char) -> *mut c_char {
    let c_str = unsafe { CStr::from_ptr(input) };
    let str_slice = c_str.to_str().unwrap();
    let color = hex::Hex::alternate_color_from_str(str_slice);
    CString::new(color).unwrap().into_raw()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe { drop(CString::from_raw(ptr)) };
    }
}


#[cfg(test)]
mod tests {
    use crate::hex::Hex;

    #[test]
    fn hex_test() {
        let hex = Hex::encode("My little string...");
        println!("Encoded: {}", hex);

        let str = Hex::decode(&hex);
        println!("Decoded: {}", str);

        assert_eq!(hex, "4d79206c6974746c6520737472696e672e2e2e");
        assert_eq!(str, "My little string...");
    }
}
