pub struct ResolveVerticalSettingPlugin{}


impl IPlugin for ResolveVerticalSettingPlugin
{

    fn Execute(&self,entityRequest:EntityRequestType)
    {
       let entityRequestContainers = entityRequest.EntityRequestContainers;
         for entityRequestContainer in entityRequestContainers
         {
              let entityRequest = entityRequestContainer.EntityRequest;
              let shouldFetchVerticalSettings = VerticalSettingHelper::ShouldFetchVerticalSettings(entityRequest);
              if(shouldFetchVerticalSettings)
              {
               let lssEntityRequest = entityRequestContainer.EntityRequest;
              }
         }
    }
}