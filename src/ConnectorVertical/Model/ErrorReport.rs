use super::ErrorLevel::ErrorLevel;

#[derive(PartialEq,Clone,serde::Serialize,serde::Deserialize)]
pub struct ErrorReport
{
    pub ErrorMessage : String,
    pub ErrorLevel : ErrorLevel
}

impl ErrorReport
{
    pub fn new() -> ErrorReport
    {
        ErrorReport
        {
            ErrorMessage : String::new(),
            ErrorLevel : ErrorLevel::Warning
        }
    }
}