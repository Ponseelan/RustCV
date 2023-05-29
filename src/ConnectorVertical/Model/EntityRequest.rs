use super::{EntityRequestType::EntityRequestType, Query::Query};




#[repr(C)]
#[derive(Copy, Clone)]
// mod Model{
pub struct EntityRequest
{
    pub entityRequestType : EntityRequestType,
    pub query: Query,
    pub TenantId: String,
    pub CorrelationId: String,
    pub EntityType: EntityType
}
// }
