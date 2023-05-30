use ConnectorVertical::Model::Diagnostics::Diagnostics;
use ConnectorVertical::Model::SearchRequest::SearchRequest;
use std::collections::HashSet;
use ConnectorVertical::Model::ProviderExecutionResult::ProviderExecutionResult;
use ConnectorVertical::Model::EntityType::EntityType;
use ConnectorVertical::Model::ProviderType::ProviderType;
use std::ptr::null;



pub struct ScdPreprocessingValidatorPlugin
{
    pub failedProvider: ProviderExecutionResult,

    pub totalLatency:i32
}



impl ScdPreprocessingValidatorPlugin
{
    pub fn Execute(& mut self, inputDiagnostics:Diagnostics , searchRequest:SearchRequest) 
    {
        let entityRequestContainers=searchRequest.EntityRequestContainers;
        // check if the entity request has Entity type as "External"
        for entityRequestContainer in entityRequestContainers
        {
            let entityRequest=entityRequestContainer.EntityRequest;
            let entityType=entityRequest.EntityType;
            let externalType=EntityType::External;
            

            if entityType == externalType
            {
               self.GetTotalLatencyAndFailedProvider(&inputDiagnostics);
               
               if(self.failedProvider == ProviderExecutionResult::default())
               {
                    let errorCode = format!("{}{}","PreprocessingFailure",self.failedProvider.ProviderName);
                    let errorMessage = format!("{}","Preprocessing failure detected in entity request");
                    let connectorExternalProviderType=ProviderType::ConnectorExternal;
                    if(entityRequest.ProviderType == connectorExternalProviderType)
                    {

                    }
               }
            }
        }
    }

    pub fn  GetTotalLatencyAndFailedProvider(& mut self, diagnostics :  &Diagnostics)
    {

        let   SCD_PREPPROCESSING_PROVIDERS:HashSet<String> = HashSet::from(["VertSetting".to_string(),"ConnSetting".to_string(),"MRTDataBuilder".to_string(),"FslTotalTime".to_string(),"EssReplicaApi".to_string()]);
        let SCD_PREPPROCESSING_LATENCY_PROVIDERS:HashSet<String> = HashSet::from(["VertSetting".to_string(),"ConnSetting".to_string(),"MRTDataBuilder".to_string(),"EssReplicaApi".to_string()]);
        unsafe{
      let providerExecutionResults = &diagnostics.ProviderExecutionResults;
      for providerExecutionResult in providerExecutionResults
      {
          let providerName = &providerExecutionResult.ProviderName;
          let providerLatency = providerExecutionResult.Latency;
         // let providerFailed = providerExecutionResult.Failed;
         
          if SCD_PREPPROCESSING_PROVIDERS.contains(providerName) && !providerExecutionResult.Success && self.failedProvider== ProviderExecutionResult::default()
          {
           self.failedProvider = providerExecutionResult.clone();
          }

          if(SCD_PREPPROCESSING_LATENCY_PROVIDERS.contains(providerName) && providerExecutionResult.Latency > 0)
          {
              self.totalLatency += providerExecutionResult.Latency;
          }
      }
    }
    }


    

}