pub mod one;
pub mod three;
pub mod two;

pub struct Day {
    pub part1: Option<fn(data: Vec<String>) -> u64>,
    pub part2: Option<fn(data: Vec<String>) -> u64>,
}

#[cfg(test)]
pub fn get_day_from_str(day: &str) -> Option<u8> {
    match day {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        _ => None,
    }
}

pub fn get_day_str(day: u8) -> Option<String> {
    match day {
        1 => Some("one".to_string()),
        2 => Some("two".to_string()),
        3 => Some("three".to_string()),
        _ => None,
    }
}

pub fn get_day(day: u8) -> Option<Day> {
    match day {
        1 => Some(one::functions()),
        2 => Some(two::functions()),
        3 => Some(three::functions()),
        _ => None,
    }
}
