

use super::QueryType::QueryType;

#[derive( Clone)]
#[repr(C)]
pub struct Query
{
    queryType : QueryType,
    query : String
}

impl Query
{
    pub fn new() -> Query
    {
        Query
        {
            queryType: QueryType::new(),
            query: String::new()
        }
    }
}