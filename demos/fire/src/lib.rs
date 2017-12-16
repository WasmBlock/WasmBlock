#[macro_use(wasmblock_setup)]
extern crate wasmblock;

use std::mem;
use std::os::raw::{c_char,c_void};
use std::ffi::{CString};
use wasmblock::{dom,canvas,console};

// needed for allocation and deallocation functions
wasmblock_setup!();

fn render() {
    let ctx = canvas::get_context("#screen");
    let bounds = (600,400);
    let width : f32 = bounds.0 as f32;
    let height : f32 = bounds.1 as f32;
    let mut pixels = vec![0 as u8;bounds.0*bounds.1*3];
    for column in 0..bounds.0 {
        for row in 0..bounds.1 {
            let x = column as f32;
            let y = column as f32;
            pixels[bounds.0*row+column]   = (x/width*255.0) as u8;
            pixels[bounds.0*row+column+1] = (y/height*255.0) as u8;
            pixels[bounds.0*row+column+2] = 0;
        }
    };
}

#[no_mangle]
pub fn start() {
    console::time();
    dom::create_element("body","style","game_styles");
    dom::set_inner_html("#game_styles",include_str!("fire.css"));
    dom::create_element("body","canvas","screen");
    dom::set_attribute("#screen","width","600");
    dom::set_attribute("#screen","height","400");
    console::time();
    render();
    console::time_end();
}
