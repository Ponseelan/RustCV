use std::default;

use super::ErrorLevel::ErrorLevel;

#[derive(PartialEq,Clone,serde::Serialize,serde::Deserialize,Debug,Default)]
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

// impl Default for ErrorReport
// {
//     fn default() -> Self
//     {
//         ErrorReport
//         {
//             ErrorMessage : String::new(),
//             ErrorLevel : ErrorLevel::Warning
//         }
//     }
// }