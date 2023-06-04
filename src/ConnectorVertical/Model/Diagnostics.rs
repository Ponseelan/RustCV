

use super::{Event::Event, ErrorReport::ErrorReport, ProviderExecutionResult::ProviderExecutionResult};


#[derive(PartialEq,Clone,serde::Serialize,serde::Deserialize,Debug,Default)]
pub struct Diagnostics
{
    #[serde(default)]
    //list of Event objects
    pub Events: Vec<Event>,
  
      #[serde(default)]
      pub ErrorReport:ErrorReport,
    //  #[serde(skip_deserializing)]
    // pub DebugMessages: Vec<(String, String)>,
     #[serde(default)]
     pub  ProviderExecutionResults:Vec<ProviderExecutionResult>
}


impl Diagnostics
{
    pub fn new() -> Diagnostics
    {
        Diagnostics
        {
            Events: Vec::new(),
            ErrorReport: ErrorReport::new(),
           // DebugMessages: Vec::new() ,
            ProviderExecutionResults:Vec::new()
        }
    }
}