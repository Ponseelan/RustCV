use std::os::raw::c_char;
use std::string;
use std::ffi::CString;
use ConnectorVertical::Model::Diagnostics::Diagnostics;
use ConnectorVertical::Model::SearchRequest::SearchRequest;
use ConnectorVertical::Model::Template::Template;
use ConnectorVertical::Plugins::ScdPreprocessingValidatorPlugin::ScdPreprocessingValidatorPlugin;
use std::iter;
use std::fs::File;
use std::io::Write;
use std::ffi::CStr;



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
pub extern "C" fn ScdPreprocessingValidatorPluginRust(diagnostics: *mut c_char , searchRequest : *mut c_char) -> *mut c_char
{
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
//deserialize the templates 
let mut file: Result<File, std::io::Error> = File::create("C://log.txt");
let fileobj=& mut file.expect("msg");
log_message(fileobj, templates);
let diagnostics: Diagnostics = serde_json::from_str(templates).unwrap();
let searchRequest: SearchRequest = serde_json::from_str(unwrappedSearchRerquest).unwrap();
let mut scdPreProcessingValidatorPlugin = ScdPreprocessingValidatorPlugin::new();
let diagnostics=scdPreProcessingValidatorPlugin.Execute(diagnostics, searchRequest);
let messafes=serde_json::to_string(&diagnostics).unwrap();
log_message(fileobj, &messafes);
CString::new(messafes).unwrap().into_raw()
//serialize the template type

}
