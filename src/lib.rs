use std::os::raw::c_char;
use std::string;
use std::ffi::CString;
use ConnectorVertical::Model::Diagnostics;
use ConnectorVertical::Model::Template::Template;
use std::iter;
use std::fs::File;
use std::io::Write;
use std::ffi::CStr;

extern crate libc;

fn log_message(file: &mut File, message: &str)  {
    // Write the message to the file
    unsafe{
    file.write_all(message.as_bytes());
    file.write_all(b"\n");  // Add a newline after the message
    }
}

pub mod  ConnectorVertical;

#[no_mangle]
pub extern "C"  fn stringReturnType(template : Template) -> *mut c_char {
    let mut song: String = String::from("💣 ");
    let length=5;
    song.extend(iter::repeat("na ").take(length as usize));
    song.push_str("Batman! 💣");

    let c_str_song = CString::new(song).unwrap();
    c_str_song.into_raw()
}


//#[no_mangle]
//pub extern "stdcall"  fn ObjectReturnType() -> *mut Template {
    // let obj = Box::new(
    //     Template::new(CString::new("Hello from Rust").unwrap().into_raw()
    // ,CString::new("Hello Wworlf").unwrap().into_raw()));
    // Box::into_raw(obj)
//}

#[no_mangle]
pub extern "C"  fn ObjectTypeAsInput(wrapper:*mut Template) {

    let  my_obj= unsafe { &*wrapper };
    // Perform operations with the object
    let mut file: Result<File, std::io::Error> = File::create("C://log.txt");
    unsafe
    {
       // let c_string: CString = CString::from_raw(my_obj.content);
       //  let str_slice = c_string.to_str().unwrap();
        // log_message(& mut file.expect("msg"), CString::from_raw(my_obj.content).to_str().unwrap());
        // let modified_str = format!("Modified: {}", str_slice);
       //  let modified_c_string: CString = CString::new("Terferiend").unwrap();
        // (*wrapper).name = modified_c_string.into_raw();
    }
   
}


#[no_mangle]
pub extern "C" fn free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            CString::from_raw(ptr);
        }
    }
}


#[no_mangle]
pub extern "C" fn ScdPreprocessingValidatorPlugin(s: *mut c_char) -> *mut c_char
{
    let c_str = unsafe {
        assert!(!s.is_null());

        CStr::from_ptr(s)
    };
    let templates=c_str.to_str().unwrap();
//deserialize the templates 
let mut file: Result<File, std::io::Error> = File::create("C://log.txt");
let fileobj=& mut file.expect("msg");
log_message(fileobj, templates);
let templates: Template = serde_json::from_str(templates).unwrap();
log_message(fileobj, &templates.content);

//serialize the template type
let mut serialized = serde_json::to_string(&templates).unwrap();
CString::new(serialized).unwrap().into_raw()
}
