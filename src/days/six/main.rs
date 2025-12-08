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

fn whole_column_empty(g: &Vec<Vec<char>>, x: usize) -> bool {
    for y in 0..g.len() {
        if g[y][x] != ' ' {
            return false;
        }
    }
    true
}

fn is_operator(c: char) -> bool {
    c == '+' || c == '*'
}

fn part2(data: Vec<String>) -> u64 {
    let g: Vec<Vec<char>> = data.into_iter().map(|x| x.chars().collect()).collect();
    let mut result = 0;
    let mut curr_digits_vec: Vec<u64> = Vec::new();
    let mut operator = '+';

    for x in 0..g[0].len() {
        if whole_column_empty(&g, x) {
            result += handle_operation(curr_digits_vec.to_vec(), operator);
            curr_digits_vec.clear();
            continue;
        }

        let mut curr_digit = String::new();

        for y in 0..g.len() {
            let curr_char = g[y][x];

            if curr_char.is_digit(10) {
                curr_digit.push_str(&curr_char.to_string());
            } else if is_operator(curr_char) {
                operator = curr_char;
                if !curr_digit.is_empty() {
                    let digit: u64 = curr_digit.parse().unwrap();
                    curr_digits_vec.push(digit);
                    curr_digit.clear();
                }
            } else {
                if !curr_digit.is_empty() {
                    let digit: u64 = curr_digit.parse().unwrap();
                    curr_digits_vec.push(digit);
                    curr_digit.clear();
                }
            }
        }
    }

    result += handle_operation(curr_digits_vec.to_vec(), operator);

    result
}

pub fn functions() -> Day {
    Day {
        part1: Some(part1),
        part2: Some(part2),
    }
}
