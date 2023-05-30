use crate::ConnectorVertical::Model::Diagnostics::Diagnostics;
use crate::ConnectorVertical::Model::Event::Event;
use crate::ConnectorVertical::Model::ProviderExecutionResult::ProviderExecutionResult;



pub struct DiagnosticsHelper
{
    
}

impl DiagnosticsHelper
{
  pub fn Merge(requiredDiagnostics: Vec<Diagnostics> , optionalDiagnostics: Vec<Diagnostics> ) -> Diagnostics
  {
   let diagnostics = Diagnostics::new();
   //initialize list of Event objects
   // let events = Vec::new();
    //initialize list of ProviderExecutionResult objects
    //let providerExecutionResults = Vec::new();

    diagnostics
  }


  pub fn PopulateMetricsForDiagnostics(events: Vec<Event> , 
    providerExecutionResults: Vec<ProviderExecutionResult> ,
     diagnostics:Vec<Diagnostics> )
  {
    //create array of Diagnostics with length same as diagnostics parameter
    // let updatedDiagnostics=Vec::new();

    // for diagnostic in diagnostics {
        
    // }  
  }


  fn FillDiagnosticErrorReport(diagnostic:Diagnostics)
  {

  }


}