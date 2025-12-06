use crate::days::Day;
use regex::Regex;

fn part1(data: Vec<String>) -> u64 {
    let mut count = 0;
    let re = Regex::new(r"(\D)(\d+)").unwrap();
    let mut curr = 50;
    let max = 100;

    for i in 0..data.len() {
        let line = &data[i];
        for cap in re.captures_iter(&line) {
            let dir = &cap[1];
            let num: i64 = cap[2].parse().unwrap();
            if dir == "R" {
                curr = (curr + num) % max;
            } else {
                curr = (curr - num).rem_euclid(max);
            }

            if curr == 0 {
                count += 1;
            }
        }
    }

    count
}

fn part2(data: Vec<String>) -> u64 {
    let mut count = 0;
    let re = Regex::new(r"(\D)(\d+)").unwrap();
    let mut curr = 50;

    for i in 0..data.len() {
        let line = &data[i];
        for cap in re.captures_iter(&line) {
            let dir = &cap[1];
            let num: u32 = cap[2].parse().unwrap();

            for _ in 0..num {
                if dir == "R" {
                    curr = (curr + 1) % 100;
                } else {
                    if curr == 0 {
                        curr = 99;
                    } else {
                        curr -= 1;
                    }
                }

                if curr == 0 {
                    count += 1;
                }
            }
        }
    }

    count.try_into().unwrap()
}

pub fn functions() -> Day {
    Day {
        part1: Some(part1),
        part2: Some(part2),
    }
}
