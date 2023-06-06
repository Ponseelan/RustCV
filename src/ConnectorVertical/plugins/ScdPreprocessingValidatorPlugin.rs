use ConnectorVertical::Model::Diagnostics::Diagnostics;
use ConnectorVertical::Model::SearchRequest::SearchRequest;
use std::collections::HashSet;
use ConnectorVertical::Model::ProviderExecutionResult::ProviderExecutionResult;
use ConnectorVertical::Model::EntityType::EntityType;
use ConnectorVertical::Model::ProviderType::ProviderType;

use ConnectorVertical::Model::ErrorReport::ErrorReport;
use ConnectorVertical::Model::Event::Event;
use ConnectorVertical::Model::EventType::EventType;
use ConnectorVertical::Model::EntityRequest::EntityRequest;
use ConnectorVertical::Model::VerticalSetting::VerticalSetting;
use ConnectorVertical::Utils::ConnectorHelper::ConnectorHelper;
use ConnectorVertical::Model::EntityRequestContainer::EntityRequestContainer;


pub struct ScdPreprocessingValidatorPlugin
{
    pub failedProvider: ProviderExecutionResult,

    pub totalLatency:i32
}



impl ScdPreprocessingValidatorPlugin
{

pub fn new() -> ScdPreprocessingValidatorPlugin
{
    ScdPreprocessingValidatorPlugin
    {
        failedProvider: ProviderExecutionResult::default(),
        totalLatency:0
    }
}

    pub fn Execute(& mut self, inputDiagnostics:Diagnostics , searchRequest:& mut SearchRequest) -> Diagnostics
    {
       // mem_print();
        let mut updatedDiagnostics:Diagnostics=Diagnostics::new();
        let entityRequestContainers=searchRequest.EntityRequestContainers.clone();
        // check if the entity request has Entity type as "External"
        for entityRequestContainer in entityRequestContainers
        {
            let entityRequest=&entityRequestContainer.EntityRequest;
            let entityType:&EntityType=&entityRequest.EntityType;
            let externalType=&EntityType::External;
            

            if entityType == externalType
            {
               self.GetTotalLatencyAndFailedProvider(&inputDiagnostics);
               
               if(self.failedProvider != ProviderExecutionResult::default())
               {
                    let errorCode = format!("{}{}","PreprocessingFailure",self.failedProvider.ProviderName);
                    let errorMessage = format!("{}","Preprocessing failure detected in entity request");
                    let connectorExternalProviderType=ProviderType::ConnectorExternal;
                    if(entityRequest.ProviderType == connectorExternalProviderType)
                    {
                       Self::UpdateDiagnostics(& mut updatedDiagnostics, errorCode,errorMessage);
                    }
               }
            }
            else
            {
             let validVerticalSetting=Self::GetValidVerticalSettings(entityRequest);
             if validVerticalSetting.len() != entityRequest.VerticalSettings.len()
             {
                Self::UpdateRequestWithValidVerticalSettings(searchRequest,validVerticalSetting);
                
             }
            }
        }
       // mem_print();
        updatedDiagnostics
    }


    pub fn GetValidVerticalSettings(entityRequest:&EntityRequest) -> Vec<VerticalSetting>
    {
        //check if entity request has vertical settings
        let mut verticalSettings=entityRequest.VerticalSettings.clone();
        if(verticalSettings.len() == 0)
        {
            return Vec::new();
        }

        let mut updatedVerticalSettings=Vec::new();

        //iterate vertical settings
        for verticalSetting in verticalSettings.into_iter()
        {
            let clonedVerticalSetting=verticalSetting.clone();
           if ConnectorHelper::IsValidFlexibleSchemaSettingForAllConnections(entityRequest,verticalSetting)
           {
            updatedVerticalSettings.push(clonedVerticalSetting);
           }
        }
        updatedVerticalSettings
    }


    pub fn UpdateRequestWithValidVerticalSettings(searchRequest:& mut SearchRequest, verticalSettingsToReplaced:Vec<VerticalSetting>)
    {
        let mut scdEntityRequestContainer=& mut EntityRequestContainer::new();
        let mut index:usize=0;
        for entityRequestContainer in & mut searchRequest.EntityRequestContainers
        {
            if(entityRequestContainer.EntityRequest.EntityType == EntityType::External)
            {
                scdEntityRequestContainer = entityRequestContainer;
               
                break;
            }
            index+=1;
        }

        if verticalSettingsToReplaced.len() ==0 && scdEntityRequestContainer.EntityRequest.ProviderType == ProviderType::ConnectorVertical
        {
            searchRequest.EntityRequestContainers.remove(index);
        }
        else if verticalSettingsToReplaced.len() < scdEntityRequestContainer.EntityRequest.VerticalSettings.len()
        {
            scdEntityRequestContainer.EntityRequest.VerticalSettings=verticalSettingsToReplaced;
        }
      
    }

    pub fn UpdateDiagnostics(diagnostics :  & mut Diagnostics, errorCode:String, errorMessage:String) 
    {
        let mut errorReport=ErrorReport::new(); 
        errorReport.ErrorMessage=errorMessage;
        let event:Event = Event::createEvent(EventType::None,errorReport,errorCode);
        diagnostics.Events.push(event);
    }

    pub fn  GetTotalLatencyAndFailedProvider(& mut self, diagnostics :  &Diagnostics)
    {

        let SCD_PREPPROCESSING_PROVIDERS:HashSet<String> = HashSet::from(["VertSetting".to_string(),"ConnSetting".to_string(),"MRTDataBuilder".to_string(),"FslTotalTime".to_string(),"EssReplicaApi".to_string()]);
        let SCD_PREPPROCESSING_LATENCY_PROVIDERS:HashSet<String> = HashSet::from(["VertSetting".to_string(),"ConnSetting".to_string(),"MRTDataBuilder".to_string(),"EssReplicaApi".to_string()]);
        unsafe{
      let providerExecutionResults = &(diagnostics.ProviderExecutionResults);
      for providerExecutionResult in providerExecutionResults
      {
          let providerName = &providerExecutionResult.ProviderName;
          let providerLatency = providerExecutionResult.Latency;
       
         
          if (SCD_PREPPROCESSING_PROVIDERS.contains(providerName) && !providerExecutionResult.Success && self.failedProvider== ProviderExecutionResult::default())
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