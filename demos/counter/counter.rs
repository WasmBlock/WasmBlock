use std::ffi::CString;
use std::os::raw::{c_char};

#[no_mangle]
pub extern "C" fn dealloc_str(ptr: *mut c_char) {
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
    fn wasmblock_timing_set_timeout(x: *const c_char,milliseconds:i32);
}

#[no_mangle]
pub fn start() {
    run();
}

#[no_mangle]
pub fn run() -> () {
    unsafe {
        wasmblock_console_log(export_string("Running"));
        //call yourself in 1000 seconds
        wasmblock_timing_set_timeout(export_string("run"),1000);
    }
}
