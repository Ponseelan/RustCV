//how to import the struct in EntityRequest.rs

use super::EntityRequest::EntityRequest;



pub struct EntityRequestContainer {
    pub EntityRequest: EntityRequest,
    pub XapExecutionContext:String,
    pub RequireQAS : bool
}
