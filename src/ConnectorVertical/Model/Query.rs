use super::QueryType::QueryType;

//#[derive(Copy, Clone)]
#[repr(C)]
pub struct Query
{
    queryType : QueryType,
    query : String
}