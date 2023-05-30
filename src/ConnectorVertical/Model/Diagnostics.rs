

use super::{Event::Event, ErrorReport::ErrorReport, ProviderExecutionResult::ProviderExecutionResult};

#[derive(PartialEq,Clone,serde::Serialize,serde::Deserialize)]
pub struct Diagnostics
{
    //list of Event objects
    pub Events: Vec<Event>,
    pub ErrorReport: ErrorReport,
    pub DebugMessages: Vec<(String, String)>,
    pub  ProviderExecutionResults: Vec<ProviderExecutionResult>
}


impl Diagnostics
{
    pub fn new() -> Diagnostics
    {
        Diagnostics
        {
            Events: Vec::new(),
            ErrorReport: ErrorReport::new(),
            DebugMessages: Vec::new(),
            ProviderExecutionResults:Vec::new()
        }
    }
}