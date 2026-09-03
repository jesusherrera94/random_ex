pub fn weekday_from_number(day: u8) -> &'static str {
    match day {
        1 => "Monday",
        2 => "Tuesday",
        3 => "Wednesday",
        4 => "Thursday",
        5 => "Friday",
        6 => "Saturday",
        7 => "Sunday",
        _ => "Invalid day number",
    }
}

pub fn main() {
    println!("{}", weekday_from_number(1));
    println!("{}", weekday_from_number(2));
    println!("{}", weekday_from_number(3));
    println!("{}", weekday_from_number(4));
    println!("{}", weekday_from_number(5));
    println!("{}", weekday_from_number(6));
    println!("{}", weekday_from_number(7));
    println!("{}", weekday_from_number(8));
}