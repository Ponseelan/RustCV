pub struct SearchSettingRequest
{
    pub TenantId : String,
    pub AppId: String,
    pub RequestId: String
}

impl SearchSettingRequest
{
    pub fn new(tenantId:String, appId:String, requestId:String) -> SearchSettingRequest
    {
        return SearchSettingRequest
        {
            TenantId:tenantId,
            AppId:appId,
            RequestId:requestId
        }
    }
}
    
