use std::ffi::{CString, CStr};
use std::os::raw::{c_char};
pub mod console;
pub mod dom;
pub mod timing;
pub mod canvas;

#[macro_export]
macro_rules! wasmblock_setup {
    () => {
        #[no_mangle]
        pub fn alloc(size: usize) -> *mut std::os::raw::c_void {
            let mut buf = Vec::with_capacity(size);
            let ptr = buf.as_mut_ptr();
            std::mem::forget(buf);
            return ptr as *mut std::os::raw::c_void;
        }

        #[no_mangle]
        pub fn dealloc(ptr: *mut c_void, cap: usize) {
            unsafe  {
                let _buf = Vec::from_raw_parts(ptr, 0, cap);
            }
        }

        #[no_mangle]
        pub fn dealloc_str(ptr: *mut std::os::raw::c_char) {
            unsafe {
                let _ = std::ffi::CString::from_raw(ptr);
            }
        }
    };
}

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
