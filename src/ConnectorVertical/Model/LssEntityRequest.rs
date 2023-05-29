use super::EntityRequest::EntityRequest;

pub struct LssEntityRequest
{
    pub EntityRequest: EntityRequest

}

impl LssEntityRequest
{
    pub fn new(entityRequest:EntityRequest) -> LssEntityRequest
    {
        return LssEntityRequest
        {
            EntityRequest:entityRequest
        }
    }
}