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
    fn wasmblock_dom_create_element(targetPtr: *const c_char,elPtr: *const c_char, idPtr: *const c_char);
    fn wasmblock_dom_set_attribute(targetPtr: *const c_char,attrPtr: *const c_char,valPtr: *const c_char);
    fn wasmblock_dom_set_inner_html(targetPtr: *const c_char,htmlPtr: *const c_char);
    fn wasmblock_dom_add_event_listener(targetPtr: *const c_char,eventPtr: *const c_char,callbackPtr: *const c_char);
}

#[inline]
pub fn create_element(target:&str,dom_type:&str,id:&str){
    unsafe {
        wasmblock_dom_create_element(export_string(target),export_string(dom_type), export_string(id));
    }
}

#[inline]
pub fn set_inner_html(target:&str,html:&str){
    unsafe {
        wasmblock_dom_set_inner_html(export_string(target),export_string(html));
    }
}

#[inline]
pub fn set_attribute(target:&str,attr:&str,val:&str){
    unsafe {
        wasmblock_dom_set_attribute(export_string(target),export_string(attr),export_string(val));
    }
}

#[inline]
pub fn add_event_listener(target:&str,event:&str,callback:&str){
    unsafe {
        wasmblock_dom_add_event_listener(export_string(target),export_string(event),export_string(callback));
    }
}
