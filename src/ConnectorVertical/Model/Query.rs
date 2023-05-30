

use super::QueryType::QueryType;

#[derive( Clone)]
#[repr(C)]
pub struct Query
{
    queryType : QueryType,
    query : String
}