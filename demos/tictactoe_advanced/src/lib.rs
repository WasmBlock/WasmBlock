#[macro_use(wasmblock_setup)]
extern crate wasmblock;

//using special macros for global state, see below
use std::mem;
use std::cell::RefCell;
use std::os::raw::{c_char,c_void};
use std::ffi::{CString};
use wasmblock::{import_string};
use wasmblock::{dom,console};

// needed for allocation and deallocation functions
wasmblock_setup!();

// global game state
struct Game {
    player_turn: i32
}

//we can't have mutable statics by default so we use this to enable it
thread_local! {
    static GAME: RefCell<Game> = RefCell::new(
        Game {
            player_turn: 0
        }
    );
}

#[no_mangle]
pub fn start() {
    console::log("Let's play a game.");
    dom::create_element("body","style","game_styles");
    dom::set_inner_html("#game_styles",include_str!("tictactoe.css"));
    dom::create_element("body","div","board");
    for x in 0..3 {
        for y in 0..3 {
            dom::create_element("#board","div",&format!("box_{}{}",x,y));
            let target = &format!("#box_{}{}",x,y);
            dom::set_inner_html(target,".");
            dom::set_attribute(target,"class","box");
            dom::set_attribute(target,"class","box");
            dom::add_event_listener(target,"click","box_clicked");
        }
    }
}

#[no_mangle]
pub fn box_clicked(id_ptr: *mut c_char) -> () {
    GAME.with(|static_game| {
        let id = import_string(id_ptr);
        let target = &format!("#{}",id);
        let game = &mut static_game.borrow_mut();
        match game.player_turn {
            0 => {
                dom::set_inner_html(target,"X");
                game.player_turn = 1;
            }
            _ => {
                dom::set_inner_html(target,"O");
                game.player_turn = 0;
            }
        }
    })
}
