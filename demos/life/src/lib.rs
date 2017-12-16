#[macro_use(wasmblock_setup)]
extern crate wasmblock;
extern crate rand;
use std::cell::RefCell;
use wasmblock::{dom,canvas,console,timing,random};
use rand::{Rng, SeedableRng, StdRng};

// needed for allocation and deallocation functions
wasmblock_setup!();

struct AppState {
    seed: f32,
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
            seed: 0.0,
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

fn surrounded_peers(x:usize,y:usize,app_state:&AppState) -> usize {
    let mut count = 0;
    let lx = usize::max(x-1,0);
    let hx = usize::min(app_state.width,x+2);
    let ly = usize::max(y-1,0);
    let hy = usize::min(app_state.height,y+2);
    for column in lx..hx {
        for row in ly..hy {
            if column == x && row == y {
                continue;
            }
            if app_state.life[app_state.width*row+column] {
                count += 1;
            }
        }
    }
    return count;
}

fn update_board(app_state:&mut AppState){
    /*
        1. Any live cell with fewer than two live neighbours dies, as if caused by underpopulation.
        2. Any live cell with two or three live neighbours lives on to the next generation.
        3. Any live cell with more than three live neighbours dies, as if by overpopulation.
        4. Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
    */
    for column in 0..app_state.width {
        for row in 0..app_state.height {
            let count = surrounded_peers(column,row,&app_state);
            if app_state.life[app_state.width*row+column] {
                if count == 2 || count == 3 {
                    app_state.next_life[app_state.width*row+column] = true;
                }
                else {
                    app_state.next_life[app_state.width*row+column] = false;
                }
            } else {
                if count == 3 {
                    app_state.next_life[app_state.width*row+column] = true;
                } else {
                    app_state.next_life[app_state.width*row+column] = false;
                }
            }
        }
    };
    for i in 0..app_state.life.len() {
        app_state.life[i] = app_state.next_life[i];
    }
}


fn randomize_board(s: f32,board: &mut Vec<bool>){
    let seed: &[_] = &[(std::i32::MAX as f32 * s) as usize, 0, 0, 0];
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    for i in 0..board.len() {
        board[i] = rng.gen::<bool>();
    }
}

#[no_mangle]
pub fn start() {
    dom::create_element("body","style","game_styles");
    dom::set_inner_html("#game_styles",include_str!("life.css"));
    dom::create_element("body","canvas","screen");
    let dimensions = (600,400);
    dom::set_attribute("#screen","width","600");
    dom::set_attribute("#screen","height","400");
    APP_STATE.with(|app_state_cell| {
        let app_state = &mut app_state_cell.borrow_mut();
        app_state.seed= random::get_seed();
        app_state.width= dimensions.0;
        app_state.height= dimensions.1;
        app_state.life = vec![false;dimensions.0*dimensions.1];
        randomize_board(app_state.seed,&mut app_state.life);
        app_state.next_life = vec![false;dimensions.0*dimensions.1];
        app_state.pixels = vec![0 as u8;dimensions.0*dimensions.1*PIXEL_SIZE];
        //store a reference to canvas once so we can reuse
        app_state.ctx = canvas::get_context("#screen");
    });
    run();
}

#[no_mangle]
pub fn run() -> () {
    APP_STATE.with(|app_state_cell| {
        let app_state = &mut app_state_cell.borrow_mut();
        render(app_state);
        update_board(app_state);
    });
    timing::request_animation_frame("run");
}
