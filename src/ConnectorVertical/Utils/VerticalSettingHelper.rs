use std::{string, ffi::CString};

use crate::ConnectorVertical::Model::EntityRequest::EntityRequest;


//pub const SubstrateSearchServiceApplicationId: String = String::from("66a88757-258c-4c72-893c-3e8bed4d6899");

pub struct VerticalSettingHelper
{
    pub  SubstrateSearchServiceApplicationId: String //= String::from("66a88757-258c-4c72-893c-3e8bed4d6899");
}


impl VerticalSettingHelper
{

    pub fn new() -> VerticalSettingHelper
    {
        return VerticalSettingHelper
        {
            SubstrateSearchServiceApplicationId: String::from("66a88757-258c-4c72-893c-3e8bed4d6899")
        }
    }

    pub fn ShouldFetchVerticalSettings(&self,entityRequest:EntityRequest) -> bool
    {
        return true;
    }
}