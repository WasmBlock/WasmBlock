use std::ffi::{CString, CStr};
use std::os::raw::{c_char};
pub mod console;
pub mod dom;

#[macro_export]
macro_rules! wasmblock_setup {
    () => {
        #[no_mangle]
        pub extern fn alloc(size: usize) -> *mut c_void {
            let mut buf = Vec::with_capacity(size);
            let ptr = buf.as_mut_ptr();
            mem::forget(buf);
            return ptr as *mut c_void;
        }

        #[no_mangle]
        pub extern fn dealloc_str(ptr: *mut c_char) {
            unsafe {
                let _ = CString::from_raw(ptr);
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
