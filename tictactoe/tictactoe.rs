use std::ffi::CString;
use std::os::raw::{c_char};

#[no_mangle]
pub extern "C" fn dealloc_str(ptr: *mut c_char) {
    unsafe {
        let _ = CString::from_raw(ptr);
    }
}

fn export_string<T:Into<std::vec::Vec<u8>>>(s:T) -> *const c_char{
    let s = CString::new(s).unwrap();
    let p = s.as_ptr();
    std::mem::forget(s);
    return p;
}

extern {
    fn console_log(x: *const c_char);
    fn dom_create_element(targetPtr: *const c_char,elPtr: *const c_char, idPtr: *const c_char);
    fn dom_set_attribute(targetPtr: *const c_char,attrPtr: *const c_char,valPtr: *const c_char);
    fn dom_set_inner_html(targetPtr: *const c_char,htmlPtr: *const c_char);
    fn dom_add_event_listener(targetPtr: *const c_char,eventPtr: *const c_char,callbackPtr: *const c_char);
}

fn log(msg:&str){
    unsafe {
        console_log(export_string(msg));
    }
}

fn el(target:&str,dom_type:&str,id:&str){
    unsafe {
        dom_create_element(export_string(target),export_string(dom_type), export_string(id));
    }
}

fn set_html(target:&str,html:&str){
    unsafe {
        dom_set_inner_html(export_string(target),export_string(html));
    }
}

fn set_attr(target:&str,attr:&str,val:&str){
    unsafe {
        dom_set_attribute(export_string(target),export_string(attr),export_string(val));
    }
}

fn on_event(target:&str,event:&str,callback:&str){
    unsafe {
        dom_add_event_listener(export_string(target),export_string(event),export_string(callback));
    }
}

#[no_mangle]
pub fn start() -> () {
    log("Let's play a game.");
    el("body","style","game_styles");
    set_html("#game_styles",r#"
        body {
            display: flex;
            align-items: center;
            justify-content: center;
            font-family: arial;
            font-size: 50px;
        }

        #board {
            border: solid 1px #999;
            width: 300px;
            height: 300px;;
            padding: 0px;
            margin: 0px;
        }

        .box {
            display: inline-flex;
            align-items: center;
            justify-content: center;
            width: 98px;
            height: 98px;
            padding: 0px;
            margin: 0px;
            border: solid 1px #ccc;
            cursor: pointer;
        }

        .box:hover {
            background: #ccc;
        }
    "#);
    el("body","div","board");
    for x in 0..3 {
        for y in 0..3 {
            el("#board","div",&format!("box_{}{}",x,y));
            let target = &format!("#box_{}{}",x,y);
            set_html(target,".");
            set_attr(target,"class","box");
            set_attr(target,"class","box");
        }
    }
    on_event(".box","click","box_clicked");
}

#[no_mangle]
pub fn box_clicked() -> () {
    set_html("#box_00","X");
}
