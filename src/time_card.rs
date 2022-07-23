#[path = "event.rs"] mod event;

use std::fmt; 
use std::vec::Vec;
use std::collections::HashMap;
use chrono::{DateTime, Local};
use serde::{Serialize, Deserialize};
pub use event::{Event, EventType};

#[derive(Serialize, Deserialize)]
pub enum ActionType {
    START,
    STOP
}

/// Object representing an action taken for a task, for example:
/// * Clocking in 
/// * Clocking out 
#[derive(Serialize, Deserialize)]
pub struct TaskAction {
    action_type: ActionType,
    time: Option<DateTime<Local>>
}

/// Object representing a time card.  Time cards can have multiple tasks, but only one will be
/// active at a given time.  
#[derive(Serialize, Deserialize)]
pub struct TimeCard {
    pub current_task: String,
    tasks_by_name: HashMap<String, Vec<TaskAction>>,
}

impl TimeCard {
    pub fn new(events: Vec<Event>) -> TimeCard {

        let mut time_card = TimeCard{
            current_task: "".to_string(),
            tasks_by_name: HashMap::new()
        };

        for event in events {
            time_card.add_event(&event);
        }

        return time_card;
    }


    /// Parse event into time card.  Sorts by event key
    fn add_event(&mut self, event: &Event){
        let action_type = match event.event_type {
            EventType::START => {
                self.current_task = event.key.clone();
                ActionType::START
            },
            EventType::STOP => ActionType::STOP
        };

        let action = TaskAction {
            action_type,
            time: event.time.clone() 
        };

        if self.tasks_by_name.contains_key(&event.key){
            self.tasks_by_name.get_mut(&event.key).unwrap().push(action);
        } else {
            let mut new_list = Vec::new();
            new_list.push(action);
            self.tasks_by_name.insert(event.key.clone(), new_list);
        }

    }
}

impl std::fmt::Display for TimeCard{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = serde_json::to_string(self).unwrap();
        write!(
            f,"{}", &string )
    }
}
impl std::fmt::Display for TaskAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, 
            "action_type: {} time: {}", 
            self.action_type,
            self.time.unwrap().format("%Y-%m-%d][%H:%M:%S")
            )
    }
}

impl std::fmt::Display for ActionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ActionType::START => write!(f, "Start"),
            ActionType::STOP => write!(f, "Stop")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_returns_empty_time_card(){
        let empty_event_list = Vec::new();
        let time_card = TimeCard::new(empty_event_list);
        assert!(time_card.current_task == "")
    }

    #[test]
    fn multiple_events_one_in_progress(){
        let mut events = Vec::with_capacity(10);
        events.push(get_event("event1", EventType::START));
        events.push(get_event("event1", EventType::STOP));
        events.push(get_event("event2", EventType::START));

        let time_card = TimeCard::new(events);
        
        assert_eq!("event2", time_card.current_task);
    }

    fn get_event(key: &str, event_type: EventType) -> Event{
        Event::new(Local::now(), event_type, key.to_owned())
    }
}
