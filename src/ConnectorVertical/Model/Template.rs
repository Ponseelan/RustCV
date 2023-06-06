

//extern crate serde;
//#[cfg(feature = "serde")]
//extern crate serde;

//use serde::Serialize;

//create a new type

#[repr(C)]
#[derive(Debug,serde::Serialize,serde::Deserialize )]
pub  struct Template  {
    pub  Name: String,
    pub Content: String


}


impl Template {
    pub fn new(Name: String, Content: String) -> Template {
        Template {
            Name,
            Content,
        }
    }
}
//create a crate in rust
