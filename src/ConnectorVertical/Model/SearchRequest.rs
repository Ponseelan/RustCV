use super::EntityRequestContainer::EntityRequestContainer;

#[derive(Clone,serde::Serialize,serde::Deserialize,Default)]
pub struct SearchRequest
{
    #[serde(default)]
    pub EntityRequestContainers: Vec<EntityRequestContainer>
}


impl SearchRequest
{
    pub fn new() -> SearchRequest
    {
        SearchRequest
        {
            EntityRequestContainers: Vec::new()
        }
    }
}