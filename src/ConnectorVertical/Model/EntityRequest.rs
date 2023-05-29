use super::{EntityRequestType::EntityRequestType, Query::Query};




#[repr(C)]
// mod Model{
pub struct EntityRequest
{
    pub entityRequestType : EntityRequestType,
    pub query: Query,
    pub TenantId: String,
    pub CorrelationId: String
}
// }
