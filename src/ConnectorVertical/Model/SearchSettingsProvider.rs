use super::SearchSettingsRequest::SearchSettingRequest;



pub struct SearchSettingsProvider 
{
    pub SearchSettingRequest: SearchSettingRequest,
    pub ScenarioHeader: String
}

impl SearchSettingsProvider
{
    pub fn new(searchSettingRequest:SearchSettingRequest) -> SearchSettingsProvider
    {
        return SearchSettingsProvider
        {
            SearchSettingRequest:searchSettingRequest,
            ScenarioHeader: "X-Scenario".to_string()
        }
    }
}