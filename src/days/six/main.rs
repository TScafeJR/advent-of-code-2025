use crate::days::Day;
use regex::Regex;

fn handle_operation(ns: Vec<u64>, op: char) -> u64 {
    match op {
        '+' => ns.iter().sum(),
        '*' => ns.iter().product(),
        _ => 0,
    }
}

fn part1(data: Vec<String>) -> u64 {
    let re = Regex::new(r"(\d+)").unwrap();
    let mut nums = Vec::new();

    for _ in re.captures_iter(&data[0]) {
        let vec: Vec<u64> = Vec::new();
        nums.push(vec);
    }

    for line in &data {
        for (i, cap) in re.captures_iter(line).enumerate() {
            if i >= nums.len() {
                break;
            }

            let num: u64 = cap[1].parse().unwrap();
            nums[i].push(num);
        }
    }

    let mut ops = Vec::new();
    let op_re = Regex::new(r"[\+\*]").unwrap();
    for cap in op_re.captures_iter(&data[data.len() - 1]) {
        let op_char = cap[0].chars().next().unwrap();
        ops.push(op_char);
    }
    let mut result = 0;

    for (i, n_vec) in nums.iter().enumerate() {
        let op = ops[i];
        result += handle_operation(n_vec.to_vec(), op);
    }

    result
}

fn part2(_data: Vec<String>) -> u64 {
    0
}

pub fn functions() -> Day {
    Day {
        part1: Some(part1),
        part2: Some(part2),
    }
}
