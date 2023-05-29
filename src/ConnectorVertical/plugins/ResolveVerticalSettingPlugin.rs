use crate::ConnectorVertical::Model::SearchRequest::SearchRequest;
use crate::ConnectorVertical::Model::EntityRequest::EntityRequest;
use crate::ConnectorVertical::Model::SearchSettingsRequest::SearchSettingRequest;
use crate::ConnectorVertical::Model::LssEntityRequest::LssEntityRequest;
use crate::ConnectorVertical::Utils::VerticalSettingHelper::VerticalSettingHelper;
use crate::ConnectorVertical::Utils::VerticalSettingHelper::SubstrateSearchServiceApplicationId;

#[repr(C)]
pub struct ResolveVerticalSettingPlugin{}

impl ResolveVerticalSettingPlugin{
    pub fn Execute(searchRequest: SearchRequest)
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
            }
    }
}
}