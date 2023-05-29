use super::QueryType::QueryType;

#[repr(C)]
pub struct Query
{
    queryType : QueryType,
    query : String
}