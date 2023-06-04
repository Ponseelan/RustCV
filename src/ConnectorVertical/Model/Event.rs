use super::{EventType::EventType, ErrorReport::ErrorReport};

#[derive(PartialEq,Clone,serde::Serialize,serde::Deserialize,Debug)]
pub struct Event
{
   #[serde(default)]
pub EventType: EventType,
#[serde(default)]
pub ErrorReport: ErrorReport,
#[serde(default)]
pub AppErrorCode: String
}

 impl Event
 {
 pub fn createEvent(eventType: EventType , errorReport : ErrorReport,errorCode:String) -> Event
 {
    Event{EventType:eventType,ErrorReport:errorReport,AppErrorCode:errorCode}
 }   
}