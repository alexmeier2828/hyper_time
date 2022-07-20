use serde::ser::{Serialize, Serializer, SerializeStruct};
use serde::Serialize as SerializeMacro;
use chrono::{DateTime, Local};

#[derive(SerializeMacro)]
pub enum EventType {
    START,
    STOP,
}

/// Represents a start or stop event for a given task
pub struct Event {
    time: DateTime<Local>, 
    event_type: EventType,
    key: String
}

impl Event {
    pub fn new(time: DateTime<Local>, event_type: EventType, key: String) -> Event {
        Event {
            time,
            event_type,
            key: key.to_string()
        }
    }
}

impl Serialize for Event {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut state = serializer.serialize_struct("Event", 3)?;
        state.serialize_field("time", &self.time.to_string())?;
        state.serialize_field("event_type", &self.event_type)?;
        state.serialize_field("key", &self.key)?;
        state.end()
    }
}
