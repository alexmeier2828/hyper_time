#[path = "event.rs"] mod event;
#[path = "time_card.rs"] mod time_card;

use std::vec::Vec; 
use std::fmt;
use chrono::Local;
use event::Event;

pub use event::EventType;

pub struct Journal {
    events: Vec<Event>
}

impl Journal {
    pub fn new() -> Journal {
        Journal {
            events: Vec::new()
        }
    }

    pub fn log_event(&mut self, key: String, event_type: EventType){
        let event = Event::new(
            Local::now(),
            event_type,
            key.to_string());

        self.events.push(event);
    }
}

impl std::fmt::Display for Journal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string_pretty(&self.events).unwrap();

        write!(f, "{}", json)
    }
}
