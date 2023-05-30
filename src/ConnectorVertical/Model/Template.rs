use std::os::raw::c_char;

//extern crate serde;
//#[cfg(feature = "serde")]
//extern crate serde;

//use serde::Serialize;

//create a new type

#[repr(C)]
#[derive(Debug,serde::Serialize,serde::Deserialize )]
pub  struct Template {
    pub  name: String,
    pub content: String
}


impl Template {
    pub fn new(name: String, content: String) -> Template {
        Template {
            name,
            content,
        }
    }
}
//create a crate in rust
