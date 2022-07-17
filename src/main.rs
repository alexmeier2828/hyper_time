mod time_card;
use time_card::TimeCard;

fn main() {
    let mut time_card = TimeCard::new();

    time_card.clock_in("task".to_string());
    println!("{}", time_card);
    time_card.clock_out("task".to_string());
    println!("{}", time_card);
}

