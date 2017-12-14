use std;
use std::os::raw::{c_char};
use std::ffi::{CString};

pub fn export_string<T:Into<std::vec::Vec<u8>>>(s:T) -> *const c_char{
    let s = CString::new(s).unwrap();
    let p = s.as_ptr();
    std::mem::forget(s);
    return p;
}

extern {
    fn wasmblock_console_log(x: *const c_char);
}

pub fn log(msg:&str){
    unsafe {
        wasmblock_console_log(export_string(msg));
    }
}
