use super::{EntityRequestType::EntityRequestType, Query::Query};




#[repr(C)]
// mod Model{
pub struct EntityRequest
{
    entityRequestType : EntityRequestType,
    query: Query
}
// }
