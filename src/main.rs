mod time_card;
mod event;
mod journal;
use journal::{Journal, EventType};


fn main() {
    let mut journal = Journal::new();
    
    journal.log_event("SS-12345".to_owned(), EventType::START);
    journal.log_event("SS-12345".to_owned(), EventType::STOP);

    print!("{}", journal);
}

