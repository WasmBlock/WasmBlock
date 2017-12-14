use std::ffi::CString;
use std::os::raw::{c_char};

#[no_mangle]
pub fn dealloc_str(ptr: *mut c_char) {
    unsafe {
        let _ = CString::from_raw(ptr);
    }
}

fn export_string<T:Into<std::vec::Vec<u8>>>(s:T) -> *const c_char{
    let s = CString::new(s).unwrap();
    let p = s.as_ptr();
    std::mem::forget(s);
    return p;
}

extern {
    fn wasmblock_console_log(x: *const c_char);
}

#[no_mangle]
pub fn start() -> () {
    unsafe {
        wasmblock_console_log(export_string("Hello World!"));
    }
}
