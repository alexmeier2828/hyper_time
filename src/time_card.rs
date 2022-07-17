use std::fmt; 
use std::vec::Vec;
use std::collections::HashMap;
use chrono::{DateTime, Local};
use std::io::Write;

use serde::Serialize;
use serializable::serializable;

pub enum ActionType {
    START,
    STOP
}

/// Object representing an action taken for a task, for example:
/// * Clocking in 
/// * Clocking out 
pub struct TaskAction {
    action_type: ActionType,
    time: DateTime<Local>
}

/// Object representing a time card.  Time cards can have multiple tasks, but only one will be
/// active at a given time.  
pub struct TimeCard {
    current_task: String,
    is_active: bool,
    tasks_by_name: HashMap<String, Vec<TaskAction>>
}

impl TimeCard {
    pub fn new() -> TimeCard {
        TimeCard{
            current_task: "".to_string(),
            is_active: false,
            tasks_by_name: HashMap::new()
        }
    }

    /// Clock in to a task given a string
    pub fn clock_in(&mut self, task: String){
        self.current_task = task.clone();
        self.is_active = true;
    
        let action =  TaskAction {
            action_type: ActionType::START,
            time: Local::now()
        };
        
        if let Some(action_list) = self.tasks_by_name.get_mut(&task){
            action_list.push(action);
        }
        else 
        {
            let mut new_action_list = Vec::new();
            new_action_list.push(action);
            self.tasks_by_name.insert(task, new_action_list);
        }
    }

    /// Clock out of a given task with a string
    pub fn clock_out(&mut self, task: String){
        self.current_task = "".to_string();
        self.is_active = false;

        let action =  TaskAction {
            action_type: ActionType::STOP,
            time: Local::now()
        };
        
        if let Some(action_list) = self.tasks_by_name.get_mut(&task){
            action_list.push(action);
        }
        else 
        {
            let mut new_action_list = Vec::new();
            new_action_list.push(action);
            self.tasks_by_name.insert(task, new_action_list);
        }
    }
}

impl std::fmt::Display for TimeCard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let task = &self.current_task;
        let active = self.is_active;
        let mut result: String = format!("current_task: {task}, is_active: {active}");
        

        for (key, value) in &self.tasks_by_name {
            result.push_str(&format!("{key} -------- \n"));

            for item in value {
                result.push_str(&format!("{item}\n"));
            }
        }

        write!(f, "{}", result)
    }
}

impl std::fmt::Display for TaskAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, 
            "action_type: {} time: {}", 
            self.action_type,
            self.time.format("%Y-%m-%d][%H:%M:%S")
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

impl serializable for TimeCard {
    fn serialize(&self) -> &'static str {
        serde_json::from_str(self).unwrap();
    }
}
