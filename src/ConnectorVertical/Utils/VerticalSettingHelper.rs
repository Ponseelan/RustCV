use crate::ConnectorVertical::Model::EntityRequest::EntityRequest;



pub struct VerticalSettingHelper
{
 
}

impl VerticalSettingHelper
{
    fn ShouldFetchVerticalSettings(entityRequest:EntityRequest) -> bool
    {
        return true;
    }
}