#[macro_use(wasmblock_setup)]
extern crate wasmblock;
extern crate rand;
use std::cell::RefCell;
use wasmblock::{dom,canvas,console,timing};

// needed for allocation and deallocation functions
wasmblock_setup!();

struct AppState {
    ctx: u32,
    width: usize,
    height: usize,
    pixels: Vec<u8>,
    life: Vec<bool>,
    next_life: Vec<bool>
}

const  PIXEL_SIZE: usize = 4;

thread_local! {
    static APP_STATE: RefCell<AppState> = RefCell::new(
        AppState {
            ctx: 0,
            life: vec![],
            next_life: vec![],
            pixels: vec![],
            width: 0,
            height: 0
        }
    );
}

fn render(app_state:&mut AppState) {
    for column in 0..app_state.width {
        for row in 0..app_state.height {
            let v = if app_state.life[app_state.width*row+column] {
                255
            } else {
                0
            };
            app_state.pixels[(app_state.width*row+column)*PIXEL_SIZE]   = v;
            app_state.pixels[(app_state.width*row+column)*PIXEL_SIZE+1] = v;
            app_state.pixels[(app_state.width*row+column)*PIXEL_SIZE+2] = v;
            app_state.pixels[(app_state.width*row+column)*PIXEL_SIZE+3] = 255;
        }
    };
    canvas::put_image_data(app_state.ctx,&app_state.pixels,0,0,app_state.width as i32,app_state.height as i32);
}

fn randomize_board(board: &mut Vec<bool>){
    for i in 0..board.len() {
        board[i] = if i%2 == 0 {
            true
        } else {
            false
        };
    }
}

#[no_mangle]
pub fn start() {
    console::time();
    dom::create_element("body","style","game_styles");
    dom::set_inner_html("#game_styles",include_str!("life.css"));
    dom::create_element("body","canvas","screen");
    let dimensions = (600,400);
    dom::set_attribute("#screen","width","600");
    dom::set_attribute("#screen","height","400");
    APP_STATE.with(|app_state_cell| {
        let app_state = &mut app_state_cell.borrow_mut();
        app_state.width= dimensions.0;
        app_state.height= dimensions.1;
        app_state.life = vec![false;dimensions.0*dimensions.1];
        randomize_board(&mut app_state.life);
        app_state.next_life = vec![false;dimensions.0*dimensions.1];
        app_state.pixels = vec![0 as u8;dimensions.0*dimensions.1*PIXEL_SIZE];
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
        render(app_state);
    });
    console::time_end();
    timing::request_animation_frame("run");
}
