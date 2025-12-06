mod days;
mod util;

use clap::Parser;
use days::{get_day, get_day_str, Day};
use std::path::PathBuf;
use std::time::Instant;

#[derive(Parser, Debug)]
struct Submission {
    day: u8,
    part: u8,
    #[clap(short, long, action)]
    test: bool,
}

fn get_data(args: &Submission) -> Vec<String> {
    let parsed_day = get_day_str(args.day).unwrap_or_else(|| {
        panic!("Day {} is not supported.", args.day);
    });

    let file_name = if args.test { "test" } else { "input" };

    let data_path = PathBuf::from(format!("src/days/{}/data/{}.txt", parsed_day, file_name));
    util::files::read_file_line_by_line(data_path)
}

fn get_functions(args: &Submission) -> Option<Day> {
    get_day(args.day)
}

fn main() {
    let start_time = Instant::now();

    let args = Submission::parse();
    let data = get_data(&args);

    if let Some(parsed_fns) = get_functions(&args) {
        if args.part == 1 {
            if let Some(part1_fn) = parsed_fns.part1 {
                let res = part1_fn(data.clone());
                println!("day {}, part {}: {}", args.day, args.part, res);
            } else {
                println!("part1 is not defined for day {}.", args.day);
            }
        }

        if args.part == 2 {
            if let Some(part2_fn) = parsed_fns.part2 {
                let res = part2_fn(data.clone());
                println!("day {}, part {}: {}", args.day, args.part, res);
            } else {
                println!("part2 is not defined for day {}.", args.day);
            }
        }
    } else {
        println!("Day {} is not supported.", args.day);
    }

    let elapsed_time = start_time.elapsed();
    println!("Execution time: {:.2?} seconds", elapsed_time);
}
