

use super::QueryType::QueryType;

#[derive(Clone,serde::Serialize,serde::Deserialize,Default)]
#[repr(C)]
pub struct Query
{
    #[serde(default)]
    queryType : QueryType,
    #[serde(default)]
    query : String
}

impl Query
{
    pub fn new() -> Query
    {
        Query
        {
            queryType: QueryType::None,
            query: String::new()
        }
    }
}