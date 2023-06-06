use std::os::raw::c_char;
use std::ffi::{CString};
use ConnectorVertical::Model::Diagnostics::Diagnostics;
use ConnectorVertical::Model::SearchRequest::SearchRequest;
use ConnectorVertical::Model::Template::Template;
use ConnectorVertical::Plugins::ScdPreprocessingValidatorPlugin;
use std::{iter, i8, string};
use std::fs::File;
use std::io::Write;
use std::ffi::CStr;

pub mod ConnectorVertical;

fn log_message(file: &mut File, message: &str)  {
    // Write the message to the file
    unsafe{
    file.write_all(message.as_bytes());
    file.write_all(b"\n");  // Add a newline after the message
    }
}


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
#[repr(C)]
pub struct MyClass {
    id: libc::c_int,
    name: *const libc::c_char,
}

#[no_mangle]
pub extern "C" fn Test(obj: *mut MyClass) {
    unsafe {
        // Access and modify the object properties
        let my_obj = &mut *obj;
        my_obj.id *= 2;

        // Convert the name C string to a Rust string and append a suffix
        // let name_cstr = std::ffi::CStr::from_ptr(my_obj.name);
        // let name_str = name_cstr.to_str().expect("Invalid UTF-8 string");
        let modified_name = format!("{} Processed", "Hello World");
        
        // Convert the modified name back to a C string and update the object
        let modified_name_cstr = std::ffi::CString::new(modified_name).unwrap().as_ptr();
        my_obj.name = modified_name_cstr;
    }
}


#[no_mangle]
pub extern "C" fn StringMOdificationBYReference(mut input:  * mut  i32  , f: unsafe extern "C" fn(* mut i32)) {
    unsafe{
        
// Convert the reference to a mutable pointer to c_char
let reference = input as *mut i8;

// Access the string and modify its contents
let mut string = CStr::from_ptr(reference).to_string_lossy().into_owned();
string.push_str(", Rust!");

// Convert the modified string back to C-style string
let modified_string = CString::new(string).unwrap();

// Copy the modified string back to the original memory location
std::ptr::copy_nonoverlapping(modified_string.as_ptr(), reference, modified_string.as_bytes_with_nul().len());
    }
}


#[no_mangle]
pub extern "C" fn ScdPreprocessingValidatorPluginRust(diagnostics: *mut c_char , searchRequest : *mut c_char) -> *mut c_char
{
    //deserialize the templates 
let mut file: Result<File, std::io::Error> = File::create("C://log.txt");
let fileobj=& mut file.expect("msg");
    log_message(fileobj, &"Received Request");
    let c_str = unsafe {
        assert!(!diagnostics.is_null());

        CStr::from_ptr(diagnostics)
    };
    let templates=c_str.to_str().unwrap();
    let unwrappedSearchRerquest = unsafe {
        assert!(!searchRequest.is_null());

        CStr::from_ptr(searchRequest)
    };
    let unwrappedSearchRerquest=unwrappedSearchRerquest.to_str().unwrap();

//log_message(fileobj, templates);
let diagnostics: Diagnostics = serde_json::from_str(templates).unwrap();
let mut searchRequest: SearchRequest = serde_json::from_str(unwrappedSearchRerquest).unwrap();
let mut scdPreProcessingValidatorPlugin = ScdPreprocessingValidatorPlugin::ScdPreprocessingValidatorPlugin::new();
let response=scdPreProcessingValidatorPlugin.Execute(diagnostics, & mut searchRequest);
let messafes=serde_json::to_string(&response).unwrap();
log_message(fileobj, &messafes);
CString::new(messafes).unwrap().into_raw()
//serialize the template type

}
