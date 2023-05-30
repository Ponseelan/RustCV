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
