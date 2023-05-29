use ConnectorVertical::Model::Diagnostics::Diagnostics;
use ConnectorVertical::Model::SearchRequest::SearchRequest;
use std::collections::HashSet;
use ConnectorVertical::Model::ProviderExecutionResult::ProviderExecutionResult;
use std::ptr::null;

pub const SCD_PREPPROCESSING_PROVIDERS:HashSet<String> = HashSet::from(["VertSetting".to_string(),"ConnSetting".to_string(),"MRTDataBuilder".to_string(),"FslTotalTime".to_string(),"EssReplicaApi".to_string()]);

pub const SCD_PREPPROCESSING_LATENCY_PROVIDERS:HashSet<String> = HashSet::from(["VertSetting".to_string(),"ConnSetting".to_string(),"MRTDataBuilder".to_string(),"EssReplicaApi".to_string()]);

pub struct ScdPreprocessingValidatorPlugin
{
    pub failedProvider:ProviderExecutionResult,

    pub totalLatency:i32
}



impl ScdPreprocessingValidatorPlugin
{
    pub fn Execute(inputDiagnostics:  Vec<Diagnostics> , searchRequest:SearchRequest) 
    {
        let entityRequestContainers=searchRequest.EntityRequestContainers;
        // check if the entity request has Entity type as "External"
        for entityRequestContainer in entityRequestContainers
        {
            let entityRequest=entityRequestContainer.EntityRequest;
            let entityType=entityRequest.EntityType;
            if entityType=="External"
            {
               
            }
        }
    }

    pub fn GetTotalLatencyAndFailedProvider(&self, diagnostics : *mut Diagnostics)
    {
      let providerExecutionResults = diagnostics.ProviderExecutionResults;
      for providerExecutionResult in providerExecutionResults
      {
          let providerName = providerExecutionResult.ProviderName;
          let providerLatency = providerExecutionResult.Latency;
          let providerFailed = providerExecutionResult.Failed;
          if SCD_PREPPROCESSING_PROVIDERS.contains(providerName) && !providerExecutionResult.Success && self.failedProvider== null
          {
            self.failedProvider = providerExecutionResult;
          }

          if(SCD_PREPPROCESSING_LATENCY_PROVIDERS.contains(providerName) && providerExecutionResult.Latency > 0)
          {
              self.totalLatency += providerLatency;
          }
      }
    }
}