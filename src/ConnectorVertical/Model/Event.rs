use super::{EventType::EventType, ErrorReport::ErrorReport};

#[derive(PartialEq,Clone)]
pub struct Event
{
pub EventType: EventType,
pub ErrorReport: ErrorReport

}