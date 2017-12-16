#[macro_use(wasmblock_setup)]
extern crate wasmblock;
use std::cell::RefCell;
use wasmblock::{dom,canvas,console,timing};

// needed for allocation and deallocation functions
wasmblock_setup!();

struct AppState {
    ctx: u32
}

//we can't have mutable statics by default so we use this to enable it
thread_local! {
    static APP_STATE: RefCell<AppState> = RefCell::new(
        AppState {
            ctx: 0
        }
    );
}

fn render(ctx:u32) {
    let bounds = (600,400);
    let width : f32 = bounds.0 as f32;
    let height : f32 = bounds.1 as f32;
    let pixel_size = 4;
    let mut pixels = vec![0 as u8;bounds.0*bounds.1*pixel_size];
    for column in 0..bounds.0 {
        for row in 0..bounds.1 {
            let x = column as f32;
            let y = row as f32;
            pixels[(bounds.0*row+column)*pixel_size]   = (x/width*255.0) as u8;
            pixels[(bounds.0*row+column)*pixel_size+1] = (y/height*255.0) as u8;
            pixels[(bounds.0*row+column)*pixel_size+2] = 0;
            pixels[(bounds.0*row+column)*pixel_size+3] = 255;
        }
    };
    canvas::put_image_data(ctx,pixels,0,0,bounds.0 as i32,bounds.1 as i32);
}

#[no_mangle]
pub fn start() {
    console::time();
    dom::create_element("body","style","game_styles");
    dom::set_inner_html("#game_styles",include_str!("fire.css"));
    dom::create_element("body","canvas","screen");
    dom::set_attribute("#screen","width","600");
    dom::set_attribute("#screen","height","400");
    APP_STATE.with(|app_state_cell| {
        let app_state = &mut app_state_cell.borrow_mut();
        //store a reference to canvas once so we can reuse
        app_state.ctx = canvas::get_context("#screen");
    });
    run();
}

#[no_mangle]
pub fn run() -> () {
    console::time();
    APP_STATE.with(|app_state_cell| {
        let app_state = &mut app_state_cell.borrow_mut();
        render(app_state.ctx);
    });
    console::time_end();
    timing::request_animation_frame("run");
}
