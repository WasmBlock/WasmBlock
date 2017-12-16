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
    fn wasmblock_console_log(msg: *const c_char);
    fn wasmblock_console_error(msg: *const c_char);
    fn wasmblock_console_info(msg: *const c_char);
    fn wasmblock_console_debug(msg: *const c_char);
    fn wasmblock_console_clear();
    fn wasmblock_console_time();
    fn wasmblock_console_time_end();
}

#[inline]
pub fn log(msg:&str){
    unsafe {
        wasmblock_console_log(export_string(msg));
    }
}

#[inline]
pub fn error(msg:&str){
    unsafe {
        wasmblock_console_error(export_string(msg));
    }
}

#[inline]
pub fn info(msg:&str){
    unsafe {
        wasmblock_console_info(export_string(msg));
    }
}

#[inline]
pub fn debug(msg:&str){
    unsafe {
        wasmblock_console_debug(export_string(msg));
    }
}

#[inline]
pub fn clear(){
    unsafe {
        wasmblock_console_clear();
    }
}

#[inline]
pub fn time(){
    unsafe {
        wasmblock_console_time();
    }
}

#[inline]
pub fn time_end(){
    unsafe {
        wasmblock_console_time_end();
    }
}
