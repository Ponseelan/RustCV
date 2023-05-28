//how to import the struct in EntityRequest.rs

use super::EntityRequest::EntityRequest;



pub struct EntityRequestContainer {
    entityRequest: EntityRequest,
    xapExecutionContext:String,
    requireQAS : bool
}
