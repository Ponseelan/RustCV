//create enum for error level
#[derive(Debug,Clone,PartialEq,serde::Serialize,serde::Deserialize,Default)]
pub enum ErrorLevel {
    #[default] Warning,
    Error
}