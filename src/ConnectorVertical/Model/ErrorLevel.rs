//create enum for error level
#[derive(Debug,Clone,PartialEq,serde::Serialize,serde::Deserialize)]
pub enum ErrorLevel {
    Warning,
    Error
}