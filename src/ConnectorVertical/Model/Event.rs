use super::{EventType::EventType, ErrorReport::ErrorReport};

#[derive(PartialEq,Clone,serde::Serialize,serde::Deserialize)]
pub struct Event
{
pub EventType: EventType,
pub ErrorReport: ErrorReport,
pub AppErrorCode: String
}

 impl Event
 {
 pub fn createEvent(eventType: EventType , errorReport : ErrorReport,errorCode:String) -> Event
 {
    Event{EventType:eventType,ErrorReport:errorReport,AppErrorCode:errorCode}
 }   
}