use crate::days::Day;

type Max = (usize, usize);

fn get_max(d: &str, idx: usize, curr_digit: usize, max_digits: usize) -> Max {
    let mut max: usize = 0;
    let mut tracked_idx = 0;

    let max_idx = d.len() - (max_digits - curr_digit) + 1;

    for i in idx..max_idx {
        let c = d.chars().nth(i).unwrap();
        let p = c.to_digit(10).unwrap() as usize;
        if p > max {
            max = p;
            tracked_idx = i;
        }
    }
    return (max, tracked_idx);
}

fn get_joltage(d: &str, digits: usize) -> u64 {
    let mut s = String::new();
    let mut prev = get_max(&d, 0, 0, digits);
    s.push_str(&prev.0.to_string());

    for i in 1..digits {
        let l = get_max(&d, prev.1 + 1, i, digits);
        s.push_str(&l.0.to_string());
        prev = l;
    }

    s.parse().unwrap()
}

fn part1(data: Vec<String>) -> u64 {
    let mut total_joltage = 0;

    for d in data {
        let j = get_joltage(&d, 2);
        total_joltage += j;
    }

    total_joltage
}

fn part2(data: Vec<String>) -> u64 {
    let mut total_joltage = 0;

    for d in data {
        total_joltage += get_joltage(&d, 12);
    }

    total_joltage
}

pub fn functions() -> Day {
    Day {
        part1: Some(part1),
        part2: Some(part2),
    }
}
