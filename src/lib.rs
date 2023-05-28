use std::os::raw::c_char;
use std::string;
use std::ffi::CString;
use ConnectorVertical::Model::Template::Template;
use std::iter;

extern crate libc;

pub mod  ConnectorVertical;

#[no_mangle]
pub extern "C"  fn stringReturnType(template : Template) -> *mut c_char {
    let mut song = String::from("💣 ");
    let length=5;
    song.extend(iter::repeat("na ").take(length as usize));
    song.push_str("Batman! 💣");

    let c_str_song = CString::new(song).unwrap();
    c_str_song.into_raw()
}


#[no_mangle]
pub extern "stdcall"  fn ObjectReturnType() -> *mut Template {
    let obj = Box::new(
        Template::new(CString::new("Hello from Rust").unwrap().into_raw()
    ,CString::new("Hello Wworlf").unwrap().into_raw()));
    Box::into_raw(obj)
}


#[no_mangle]
pub extern "C" fn free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            CString::from_raw(ptr);
        }
    }
}

