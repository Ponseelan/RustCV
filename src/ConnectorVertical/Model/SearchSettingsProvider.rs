use super::SearchSettingsRequest::SearchSettingRequest;

pub const ScenarioHeader: String =  "X-Scenario".to_string();


pub struct SearchSettingsProvider 
{
    pub SearchSettingRequest: SearchSettingRequest
}

impl SearchSettingsProvider
{
    pub fn new(searchSettingRequest:SearchSettingRequest) -> SearchSettingsProvider
    {
        return SearchSettingsProvider
        {
            SearchSettingRequest:searchSettingRequest
        }
    }
}