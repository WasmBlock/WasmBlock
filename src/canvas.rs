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
    fn wasmblock_canvas_get_context(target: *const c_char) -> u32;
    fn wasmblock_canvas_set_fill_style(ctx: u32, style: *const c_char);
    fn wasmblock_canvas_set_fill_style_color(ctx: u32, r: u8, g: u8, b: u8, a: f32);
    fn wasmblock_canvas_fill_rect(ctx: u32, x:f32,y:f32,width:f32,height:f32);
}

#[inline]
pub fn get_context(target:&str) -> u32 {
    unsafe {
        return wasmblock_canvas_get_context(export_string(target));
    }
}

#[inline]
pub fn set_fill_style(ctx: u32, style:&str){
    unsafe {
        wasmblock_canvas_set_fill_style(ctx, export_string(style));
    }
}

#[inline]
pub fn set_fill_style_color(ctx: u32, r:u8, g:u8, b:u8, a:f32){
    unsafe {
        wasmblock_canvas_set_fill_style_color(ctx, r, g, b, a);
    }
}

#[inline]
pub fn fill_rect(ctx: u32, x:f32, y:f32, width:f32, height:f32){
    unsafe {
        wasmblock_canvas_fill_rect(ctx, x, y, width, height);
    }
}
