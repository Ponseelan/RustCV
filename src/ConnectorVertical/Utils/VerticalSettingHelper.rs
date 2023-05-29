use crate::ConnectorVertical::Model::EntityRequest::EntityRequest;


pub const  SubstrateSearchServiceApplicationId: String = "66a88757-258c-4c72-893c-3e8bed4d6899".to_string();

pub struct VerticalSettingHelper
{
   
}


impl VerticalSettingHelper
{

    pub fn ShouldFetchVerticalSettings(entityRequest:EntityRequest) -> bool
    {
        return true;
    }
}