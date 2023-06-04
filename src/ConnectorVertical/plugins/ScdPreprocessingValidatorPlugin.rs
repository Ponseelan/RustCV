use ConnectorVertical::Model::Diagnostics::Diagnostics;
use ConnectorVertical::Model::SearchRequest::SearchRequest;
use std::collections::HashSet;
use ConnectorVertical::Model::ProviderExecutionResult::ProviderExecutionResult;
use ConnectorVertical::Model::EntityType::EntityType;
use ConnectorVertical::Model::ProviderType::ProviderType;
use std::ptr::null;

use crate::ConnectorVertical::Model::ErrorReport::ErrorReport;
use crate::ConnectorVertical::Model::Event::Event;
use crate::ConnectorVertical::Model::EventType::EventType;

// use libc::{c_char, c_void};
// use std::ptr::{null, null_mut};

// #[global_allocator]
// static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

// extern "C" fn write_cb(_: *mut c_void, message: *const c_char) {
//     print!("{}", String::from_utf8_lossy(unsafe {
//         std::ffi::CStr::from_ptr(message as *const i8).to_bytes()
//     }));
// }

// fn mem_print() {
//     unsafe { jemalloc_sys::malloc_stats_print(Some(write_cb), null_mut(), null()) }
// }

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

    pub fn Execute(& mut self, inputDiagnostics:Diagnostics , searchRequest:SearchRequest) -> Diagnostics
    {
       // mem_print();
        let mut updatedDiagnostics:Diagnostics=Diagnostics::new();
        let entityRequestContainers=searchRequest.EntityRequestContainers;
        // check if the entity request has Entity type as "External"
        for entityRequestContainer in entityRequestContainers
        {
            let entityRequest=entityRequestContainer.EntityRequest;
            let entityType: EntityType=entityRequest.EntityType;
            let externalType=EntityType::External;
            

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
        }
       // mem_print();
        updatedDiagnostics
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