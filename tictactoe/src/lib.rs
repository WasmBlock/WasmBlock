//using special macros for global state, see below
use std::cell::RefCell;
use std::mem;
use std::ffi::{CString, CStr};
use std::os::raw::{c_char,c_void};

#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut c_void {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    mem::forget(buf);
    return ptr as *mut c_void;
}

#[no_mangle]
pub extern "C" fn dealloc_str(ptr: *mut c_char) {
    unsafe {
        let _ = CString::from_raw(ptr);
    }
}

fn import_string(data: *mut c_char) -> String{
    unsafe {
        CStr::from_ptr(data).to_string_lossy().into_owned()
    }
}

fn export_string<T:Into<std::vec::Vec<u8>>>(s:T) -> *const c_char{
    let s = CString::new(s).unwrap();
    let p = s.as_ptr();
    std::mem::forget(s);
    return p;
}

extern {
    fn wasmblock_console_log(x: *const c_char);
    fn wasmblock_dom_create_element(targetPtr: *const c_char,elPtr: *const c_char, idPtr: *const c_char);
    fn wasmblock_dom_set_attribute(targetPtr: *const c_char,attrPtr: *const c_char,valPtr: *const c_char);
    fn wasmblock_dom_set_inner_html(targetPtr: *const c_char,htmlPtr: *const c_char);
    fn wasmblock_dom_add_event_listener(targetPtr: *const c_char,eventPtr: *const c_char,callbackPtr: *const c_char);
}

fn log(msg:&str){
    unsafe {
        wasmblock_console_log(export_string(msg));
    }
}

fn el(target:&str,dom_type:&str,id:&str){
    unsafe {
        wasmblock_dom_create_element(export_string(target),export_string(dom_type), export_string(id));
    }
}

fn set_html(target:&str,html:&str){
    unsafe {
        wasmblock_dom_set_inner_html(export_string(target),export_string(html));
    }
}

fn set_attr(target:&str,attr:&str,val:&str){
    unsafe {
        wasmblock_dom_set_attribute(export_string(target),export_string(attr),export_string(val));
    }
}

fn on_event(target:&str,event:&str,callback:&str){
    unsafe {
        wasmblock_dom_add_event_listener(export_string(target),export_string(event),export_string(callback));
    }
}

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
pub fn start() -> () {
    log("Let's play a game.");
    el("body","style","game_styles");
    set_html("#game_styles",include_str!("tictactoe.css"));
    el("body","div","board");
    for x in 0..3 {
        for y in 0..3 {
            el("#board","div",&format!("box_{}{}",x,y));
            let target = &format!("#box_{}{}",x,y);
            set_html(target,".");
            set_attr(target,"class","box");
            set_attr(target,"class","box");
            on_event(target,"click","box_clicked");
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
                set_html(target,"X");
                game.player_turn = 1;
            }
            _ => {
                set_html(target,"O");
                game.player_turn = 0;
            }
        }
    })
}
