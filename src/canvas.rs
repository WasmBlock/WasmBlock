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
    fn wasmblock_canvas_put_image_data(ctx: u32, pixelPtr:  *const u8, pixelLength: u32, x:i32,y:i32,width:i32,height:i32);
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

#[inline]
pub fn put_image_data(ctx: u32, pixel: Vec<u8>, x:i32,y:i32,width:i32,height:i32) {
    unsafe {
        let pix = pixel.clone();
        let l = pixel.len();
        let p = pixel.as_ptr();
        std::mem::forget(pix);
        wasmblock_canvas_put_image_data(ctx, p as *const u8, l as u32, x,y,width,height);
    }
}
