use ConnectorVertical::{
    Model::{SearchRequest::SearchRequest, Diagnostics::Diagnostics, ProviderExecutionResult::ProviderExecutionResult, EntityType::EntityType, ProviderType::ProviderType, EntityRequestContainer::EntityRequestContainer, EntityRequest::EntityRequest},
    Plugins::ScdPreprocessingValidatorPlugin::ScdPreprocessingValidatorPlugin};



pub fn add(a: i32, b: i32) -> i32 {
    a + b
}


#[cfg(test)]
pub mod tests{

use super::*;

#[test]
pub fn ScdPreprocessingValidatorPlugin_Test()
{
let diagnostics=CreateDiagnostics();
let searchRequest=CreateSearchRequest();
let mut ScdPreprocessingValidatorPlugin=ScdPreprocessingValidatorPlugin::new();
let diagnostcs=ScdPreprocessingValidatorPlugin.Execute(diagnostics,searchRequest);
assert_eq!(diagnostcs.Events.len(),1);
}
}



pub fn CreateSearchRequest() -> SearchRequest {
    let mut searchRequest = SearchRequest::new();
    let mut entityRequestContainer = EntityRequestContainer::new();
    let mut entityRequest = EntityRequest::new();
    let mut entityType = EntityType::External;
    
    entityRequest.EntityType = entityType;
    let mut providerType = ProviderType::ConnectorExternal;
    entityRequest.ProviderType = providerType;
    entityRequestContainer.EntityRequest = entityRequest;
    let mut entityRequestContainers = Vec::new();
    entityRequestContainers.push(entityRequestContainer);
    searchRequest.EntityRequestContainers = entityRequestContainers;
    searchRequest
}



pub fn CreateDiagnostics() -> Diagnostics {
    //let mut diagnostics = Vec::new();
    let mut diagnostic = Diagnostics::new(
    );
    let mut providerExecutionResult: ProviderExecutionResult=ProviderExecutionResult::default();
    providerExecutionResult.ProviderName="VertSetting".to_string();
    providerExecutionResult.Success=false;
    providerExecutionResult.Latency=10;
    providerExecutionResult.ResultsCount=10;
    let providerExecutionResults=vec![providerExecutionResult];
    diagnostic.ProviderExecutionResults=providerExecutionResults;
    //diagnostics.push(diagnostic);
    diagnostic
    
}