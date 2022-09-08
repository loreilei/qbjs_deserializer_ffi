pub use std::ffi::{CString, NulError};
pub use std::os::raw::c_char;
pub use std::ptr;

pub use qbjs_deserializer::qbjs;

#[no_mangle]
pub extern "C" fn deserialize_to_json_string(
    qbjs_content: *const u8,
    qbjs_content_length: u64,
) -> *mut c_char {
    let qbjs = unsafe { std::slice::from_raw_parts(qbjs_content, qbjs_content_length as usize) };
    let json_string = match qbjs::deserialize_to_json(&qbjs) {
        Ok(value) => value.to_string(),
        Err(_) => {
            return ptr::null_mut::<c_char>();
        }
    };

    match CString::new(json_string) {
        Ok(string) => string.into_raw(),
        Err(_) => ptr::null_mut::<c_char>(),
    }
}

#[no_mangle]
pub extern "C" fn free_deserialized_string(cstring: *mut c_char) {
    unsafe {
        if cstring.is_null() {
            return;
        }
        CString::from_raw(cstring)
    };
}
