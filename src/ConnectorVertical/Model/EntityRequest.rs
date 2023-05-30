use super::{EntityRequestType::EntityRequestType, Query::Query, EntityType::EntityType, ProviderType::ProviderType};




#[repr(C)]
#[derive(Clone)]
pub struct EntityRequest
{
    pub entityRequestType : EntityRequestType,
    pub query: Query,
    pub TenantId: String,
    pub CorrelationId: String,
    pub EntityType: EntityType,
    pub ProviderType:ProviderType
}


impl EntityRequest{
    pub fn new() -> EntityRequest
    {
        EntityRequest
        {
            entityRequestType: EntityRequestType::new(),
            query: Query::new(),
            TenantId: String::new(),
            CorrelationId: String::new(),
            EntityType: EntityType::External,
            ProviderType: ProviderType::ConnectorExternal
        }
    }
}