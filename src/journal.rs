#[path = "time_card.rs"] mod time_card;

use std::vec::Vec; 
use std::fmt;
use chrono::Local;
use timespan::DateTimeSpan;

// public namespaces 
pub use time_card::{TimeCard, Event, EventType};

/// Represents a list of timed events 
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

    /// creates a time card for the specified time span
    pub fn get_timecard_for_timespan(&self, timespan: DateTimeSpan<Local>) -> TimeCard{
        let filtered_events = get_events_for_timespan(self.events.clone(), timespan);
        TimeCard::new(filtered_events)
    }
}

impl std::fmt::Display for Journal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string_pretty(&self.events).unwrap();

        write!(f, "{}", json)
    }
}

/// returns the events that occured within the given timespan
fn get_events_for_timespan(events: Vec<Event>, timespan: DateTimeSpan<Local>) -> Vec<Event> {
    let mut filtered_events = Vec::new();
    
    for event in events{
        if timespan.contains(&event.time.unwrap()) {
            filtered_events.push(event)
        }
    }

    return filtered_events;
}
