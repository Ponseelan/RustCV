use super::{EntityRequestType::EntityRequestType, Query::Query, EntityType::EntityType, ProviderType::ProviderType, SearchSettingsProvider};




#[repr(C)]
#[derive(Clone,serde::Serialize,serde::Deserialize,Default)]
pub struct EntityRequest
{
    #[serde(default)]
    pub EntityRequestType : EntityRequestType,
    #[serde(default)]
    pub query: Query,
    // #[serde(default)]
    // pub TenantId: String,
    #[serde(default)]
    pub CorrelationId: String,
    #[serde(default)]
    pub EntityType: EntityType,
    #[serde(default)]
    pub ProviderType:ProviderType
}


impl EntityRequest{
    pub fn new() -> EntityRequest
    {
        EntityRequest
        {
            EntityRequestType: EntityRequestType::LssEntityRequest,
            query: Query::new(),
           // TenantId: String::new(),
            CorrelationId: String::new(),
            EntityType: EntityType::External,
            ProviderType: ProviderType::ConnectorExternal
        }
    }
}