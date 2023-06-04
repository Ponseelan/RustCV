//how to import the struct in EntityRequest.rs

use super::EntityRequest::EntityRequest;

#[derive(Clone,serde::Serialize,serde::Deserialize,Default)]
pub struct EntityRequestContainer {
    #[serde(default)]
    pub EntityRequest: EntityRequest,
    #[serde(default)]
    pub RequireQAS : bool
}


impl EntityRequestContainer
{
    pub fn new() -> EntityRequestContainer
    {
        EntityRequestContainer
        {
           EntityRequest: EntityRequest::new(),
            //XapExecutionContext: String::new(),
            RequireQAS: false
        }
    }
}