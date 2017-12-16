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
    fn wasmblock_timing_request_animation_frame(fn_name: *const c_char);
    fn wasmblock_timing_set_timeout(fn_name: *const c_char, milliseconds:i32);
}

#[inline]
pub fn request_animation_frame(fn_name:&str){
    unsafe {
        wasmblock_timing_request_animation_frame(export_string(fn_name));
    }
}

#[inline]
pub fn set_timeout(fn_name:&str,milliseconds:i32){
    unsafe {
        wasmblock_timing_set_timeout(export_string(fn_name),milliseconds);
    }
}
