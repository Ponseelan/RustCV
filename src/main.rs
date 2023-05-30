
use std::{os::raw::c_char, ffi::CString};

use ConnectorVertical::Model::Template::Template;
pub mod  ConnectorVertical;
pub mod TestConnectorVertical;


fn main() {
    let mut a=vec![1,2,3];
    print(& mut a);
    print!("{}",a[0]);

}

fn print(list:& mut Vec<i32>)
{
    list[0]=11;
    print!("{:?}",list);
}
