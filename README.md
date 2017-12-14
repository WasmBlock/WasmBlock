# What's the purpose of this?
> Expose web api libraries as plugins in a way that is easy for Rust developers

# Why is Web Assembly is hard for Rust developers
* For many Javascript may not be their primary language
* Talking with Javascript from Rust can be unintuitive. Rust must not deallocate what it exports, and Javascript must deallocate it when it is done with what it has received.
* Making websites that use only what you need is important
* There are many libraries and APIs that people wish they could use

# Hello World

# Making a new library

index.html
```html
<script src="https://rawgit.com/WasmBlocks/WasmBlocks/master/wasmblocks.js"></script>
<script src="https://rawgit.com/WasmBlocks/WasmBlocks/master/wasmblocks-console.js"></script>
<wasm-module src="helloworld.wasm" entry="start" console></wasm-module>
```

hellworld.rs
```rust
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
    fn console_log(x: *const c_char);
}

#[no_mangle]
pub fn start() -> () {
    unsafe {
        console_log(export_string("Hello World!"));
    }
}
```
