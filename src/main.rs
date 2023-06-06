
use std::{os::raw::c_char, ffi::CString};

use ConnectorVertical::Model::Template::Template;
pub mod  ConnectorVertical;
pub mod UnitTest;


fn main() {  
let mut template=Template::new(String::from("HelloWorld"),String::from("HelloWorld"));
let t1=& mut template;
let t2=& mut template;
modifyFuinction(& mut  template);
modifyFuinction(& mut  template);
print!("{:?}",template);
}


pub fn modifyFuinction(template:& mut Template)
{
    unsafe{
template.Name=String::from("Modified");
    }
}
