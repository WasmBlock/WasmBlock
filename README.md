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
<script src="https://rawgit.com/WasmBlock/WasmBlock/master/wasmblock.js"></script>
<script src="https://rawgit.com/WasmBlock/WasmBlock/master/wasmblock-console.js"></script>
<wasm-module src="helloworld.wasm" entry="start"></wasm-module>
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

wasmblock-shout.js
```javascript
WasmBlock((module) => ({
  shout: function(strPtr) {
    // Copying string out of memory calls dealloc_str in web assembly
    let result = module.$copyCStr(strPtr);
    window.alert(result);
  }
}))
```

index.html
```html
<script src="https://rawgit.com/WasmBlock/WasmBlock/master/wasmblock.js"></script>
<script src="wasmblock-shout.js"></script>
<wasm-module src="shoutworld.wasm" entry="start"></wasm-module>
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

// An api function exposed by wasmblock-alert.js
extern {
    fn shout(x: *const c_char);
}

#[no_mangle]
pub fn start() -> () {
    unsafe {
        shout(export_string("Hello World!!!"));
    }
}
```

```bash
rustc +nightly --target wasm32-unknown-unknown -O --crate-type=cdylib shoutworld.rs -o shoutworld.wasm
```

# Combining WasmBlocks to expose more to web assembly

```html
<script src="https://rawgit.com/WasmBlock/WasmBlock/master/wasmblock.js"></script>
<script src="https://rawgit.com/WasmBlock/WasmBlock/master/wasmblock-console.js"></script>
<script src="https://rawgit.com/WasmBlock/WasmBlock/master/wasmblock-dom.js"></script>
<script src="future_work/wasmblock-webgl.js"></script>
<wasm-module src="my_app.wasm" entry="start"></wasm-module>
```

# APIs

Here are current APIs for you to mix and match. Every extern is shown, but remember, your Rust only needs to contain externs it uses for brevity.

## Console

All these apis show something in the browser console.

```html
<script src="https://rawgit.com/WasmBlock/WasmBlock/master/wasmblock-console.js"></script>
```

```rust
extern {
    // show various log levels
    fn console_log(msg: *const c_char);
    fn console_error(msg: *const c_char);
    fn console_info(msg: *const c_char);
    fn console_debug(msg: *const c_char);
    // clears the console
    fn console_clear();
    // useful for timing things, call console_time() then console_time_end()
    // and the elapsed time shows in console
    fn console_time();
    fn console_time_end();
}
```

## Timing
```html
<script src="https://rawgit.com/WasmBlock/WasmBlock/master/wasmblock-timing.js"></script>
```

```rust
extern {
    // This function requests the browser to call a web assembly function you
    // have exposed by name on the next render frame. This is used often in
    // games so you don't draw more often than the browser can (60 fps).
    fn timing_request_animation_frame(fnName: *const c_char);

    // This function requests the browser to call a web assembly function you
    // have exposed by name after  a certain amount of milliseconds
    fn timing_set_timeout(fnName: *const c_char, milliseconds:i32);
}
```

These functions take in a function name to call back. It's referring to your publically exposed Rust function names.

## DOM
```html
<script src="https://rawgit.com/WasmBlock/WasmBlock/master/wasmblock-dom.js"></script>
```

```rust
extern {
    // creates an element at a certain dom target with a given id
    fn dom_create_element(targetPtr: *const c_char,elPtr: *const c_char, idPtr: *const c_char);
    // set an attribute of a dom target
    fn dom_set_attribute(targetPtr: *const c_char,attrPtr: *const c_char,valPtr: *const c_char);
    // set the inner html of a dom target
    fn dom_set_inner_html(targetPtr: *const c_char,htmlPtr: *const c_char);
    // add a listener to a dom target
    fn dom_add_event_listener(targetPtr: *const c_char,eventPtr: *const c_char,callbackPtr: *const c_char);
}
```

All these functions work off a valid [css selector](https://www.w3schools.com/cssref/css_selectors.asp) as the dom target. Check out the [tic tac toe](https://github.com/WasmBlock/WasmBlock/tree/master/tictactoe/src/lib.rs) demo to see examples of how to use this API. And also look how 
