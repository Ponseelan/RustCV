use std::os::raw::c_char;
//create a new type
//#[derive(Deserialize, Debug)]
#[repr(C)]
pub  struct Template {
    pub  name: *mut  c_char,
    pub content: *mut c_char,
}


impl Template {
    pub fn new(name: *mut c_char, content: *mut c_char) -> Template {
        Template {
            name,
            content,
        }
    }
}
//create a crate in rust
