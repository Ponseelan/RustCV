use crate::ConnectorVertical::Model::SearchRequest::SearchRequest;
use crate::ConnectorVertical::Model::EntityRequest::EntityRequest;
use crate::ConnectorVertical::Model::SearchSettingsRequest::SearchSettingRequest;
use crate::ConnectorVertical::Model::LssEntityRequest::LssEntityRequest;
use crate::ConnectorVertical::Utils::VerticalSettingHelper::VerticalSettingHelper;
use crate::ConnectorVertical::Utils::VerticalSettingHelper::SubstrateSearchServiceApplicationId;
use crate::ConnectorVertical::Model::SearchSettingsProvider::SearchSettingsProvider;
use crate::ConnectorVertical::Model::FSLConfiguration::FSLConfiguration;

#[repr(C)]
pub struct ResolveVerticalSettingPlugin{}

impl ResolveVerticalSettingPlugin{
    pub fn UpdateVerticalSettingForEntityRequest(){}



    pub fn Execute(searchRequest: SearchRequest, fslConfiguration:  FSLConfiguration)
    {
        let entityRequestContainers = searchRequest.EntityRequestContainers;
        for entityRequestContainer in entityRequestContainers
        {
            let entityRequest = entityRequestContainer.EntityRequest;
            let shouldFetchVerticalSettings = VerticalSettingHelper::ShouldFetchVerticalSettings(entityRequest);
            if shouldFetchVerticalSettings
            {
                let lssEntityRequest=LssEntityRequest::new(entityRequest);
                let searchSettingRequest=SearchSettingRequest::new(lssEntityRequest.EntityRequest.TenantId,lssEntityRequest.EntityRequest.CorrelationId,SubstrateSearchServiceApplicationId);
                let searchSettingsProvider = SearchSettingsProvider::new(searchSettingRequest);
                if fslConfiguration.UseScdConfigSdk
                {

                }
                
                
            }
    }
}
}