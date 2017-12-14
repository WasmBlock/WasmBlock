use std::ffi::{CString, CStr};
use std::os::raw::{c_char};

pub fn import_string(data: *mut c_char) -> String{
    unsafe {
        CStr::from_ptr(data).to_string_lossy().into_owned()
    }
}

pub fn export_string<T:Into<std::vec::Vec<u8>>>(s:T) -> *const c_char{
    let s = CString::new(s).unwrap();
    let p = s.as_ptr();
    std::mem::forget(s);
    return p;
}
