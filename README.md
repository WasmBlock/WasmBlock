# What's the purpose of this?
> Expose web api libraries as plugins in a way that is easy for Rust developers

# Why might Web Assembly be hard for Rust developers
* For many Javascript may not be their primary language
* Talking with Javascript from Rust can be unintuitive. Rust might not deallocate what it exports, and if so Javascript must deallocate it when it is done with what it has received.
* Making websites that use only what you need is important
* There are many libraries and APIs that people wish they could use

# Hello World

[Demo](https://wasmblock.github.io/WasmBlock/helloworld/index.html)

index.html
```html
<script src="https://cdnjs.cloudflare.com/ajax/libs/webcomponentsjs/1.0.20/webcomponents-loader.js"></script>
<script src="https://rawgit.com/WasmBlock/WasmBlock/master/wasmblock.js"></script>
<script src="https://rawgit.com/WasmBlock/WasmBlock/master/wasmblock-console.js"></script>
<wasm-module src="helloworld.wasm" entry="start" console></wasm-module>
```

helloworld.rs
```rust
use std::ffi::CString;
use std::os::raw::{c_char};

// When we send strings to javascript we will have to create strings and have
// Rust forget about them so they remain in Web Assembly's memory for
// javascript to consume
fn export_string<T:Into<std::vec::Vec<u8>>>(s:T) -> *const c_char{
    let s = CString::new(s).unwrap();
    let p = s.as_ptr();
    std::mem::forget(s);
    return p;
}

// Give javascript a means to deallocate our strings
#[no_mangle]
pub extern "C" fn dealloc_str(ptr: *mut c_char) {
    unsafe {
        let _ = CString::from_raw(ptr);
    }
}

// An api function exposed by wasmblock-console.js
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

```bash
curl -s https://static.rust-lang.org/rustup.sh | sh -s -- --channel=nightly
rustup update nightly
rustup target add wasm32-unknown-unknown --toolchain=nightly
rustc +nightly --target wasm32-unknown-unknown -O --crate-type=cdylib helloworld.rs -o helloworld.wasm
```

# Making a new library

shoutworld.js
```javascript
// extension name "alert" here defines how you will expose it to a module below in html
WebBlock.extensions.alert = function(Module){
  return {
    alert: function(strPtr) {
      // Copying this string from web assemblies memory will deallocate it
      let str = Module.$copyCStr(Module, strPtr);
      window.alert(str)
    }
  }
}
```

index.html
```html
<script src="https://cdnjs.cloudflare.com/ajax/libs/webcomponentsjs/1.0.20/webcomponents-loader.js"></script>
<script src="https://rawgit.com/WasmBlock/WasmBlock/master/wasmblock.js"></script>
<script src="webblock-alert.js"></script>
<wasm-module src="shoutworld.wasm" entry="start" alert></wasm-module>
```

shoutworld.rs
```rust
use std::ffi::CString;
use std::os::raw::{c_char};

// When we send strings to javascript we will have to create strings and have
// Rust forget about them so they remain in Web Assembly's memory for
// javascript to consume
fn export_string<T:Into<std::vec::Vec<u8>>>(s:T) -> *const c_char{
    let s = CString::new(s).unwrap();
    let p = s.as_ptr();
    std::mem::forget(s);
    return p;
}

// Give javascript a means to deallocate our strings
#[no_mangle]
pub extern "C" fn dealloc_str(ptr: *mut c_char) {
    unsafe {
        let _ = CString::from_raw(ptr);
    }
}

// An api function exposed by wasmblock-console.js
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

```bash
rustc +nightly --target wasm32-unknown-unknown -O --crate-type=cdylib shoutworld.rs -o shoutworld.wasm
```
