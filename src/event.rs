use serde::Serialize as SerializeMacro;
use chrono::{DateTime, Local};

#[derive(SerializeMacro)]
pub enum EventType {
    START,
    STOP,
}

/// Represents a start or stop event for a given task
#[derive(SerializeMacro)]
pub struct Event {
    pub time: Option<DateTime<Local>>, 
    pub event_type: EventType,
    pub key: String
}

impl Event {
    pub fn new(time: DateTime<Local>, event_type: EventType, key: String) -> Event {
        Event {
            time: Some(time.clone()),
            event_type,
            key: key.to_string()
        }
    }
}

