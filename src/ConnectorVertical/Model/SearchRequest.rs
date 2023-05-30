use super::EntityRequestContainer::EntityRequestContainer;


pub struct SearchRequest
{
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