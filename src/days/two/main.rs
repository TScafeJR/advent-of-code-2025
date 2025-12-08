use crate::days::Day;
use crate::util::range;
use std::collections::HashSet;

impl range::Range {
    pub fn find_duplicates(&self) -> Vec<u64> {
        let mut d = Vec::new();

        for n in self.low..self.high + 1 {
            let is_valid = self.validator.map_or(true, |f| f(n));

            if is_valid {
                d.push(n)
            }
        }
        d
    }
}

pub fn get_factors(n: u64) -> Vec<u64> {
    let mut v = Vec::new();
    for i in 1..n {
        if n % i == 0 {
            v.push(i)
        }
    }
    v
}

pub fn split_into_chunks(s: &str, n: u64) -> Vec<String> {
    s.chars()
        .collect::<Vec<char>>()
        .chunks(n as usize)
        .map(|chunk| chunk.iter().collect())
        .collect()
}

pub fn is_invalid(n: u64) -> bool {
    let s = n.to_string();

    if s.len() % 2 != 0 {
        return false;
    }

    let (l, r) = s.split_at(s.len() / 2);

    return l == r;
}

pub fn is_invalid_p2(n: u64) -> bool {
    let s = n.to_string();
    let f = get_factors(s.len() as u64);

    for i in f {
        let mut seen: HashSet<String> = HashSet::new();
        let buckets = split_into_chunks(&s, i);
        let mut i_valid = false;

        for b in buckets {
            if seen.is_empty() {
                seen.insert(b);
            } else {
                if !seen.contains(&b) {
                    i_valid = true;
                    break;
                }
            }
        }

        if !i_valid {
            return true;
        }
    }

    return false;
}

fn part1(data: Vec<String>) -> u64 {
    let mut sum = 0;

    for i in 0..data.len() {
        let s = data[i].split(",");
        for entry in s {
            let r = range::Range::with_validator(entry, is_invalid);
            let d = r.find_duplicates();
            for j in 0..d.len() {
                sum += d[j]
            }
        }
    }

    sum
}

fn part2(data: Vec<String>) -> u64 {
    let mut sum = 0;

    for i in 0..data.len() {
        let s = data[i].split(",");
        for entry in s {
            let r = range::Range::with_validator(entry, is_invalid_p2);
            let d = r.find_duplicates();
            for j in 0..d.len() {
                sum += d[j]
            }
        }
    }

    sum
}

pub fn functions() -> Day {
    Day {
        part1: Some(part1),
        part2: Some(part2),
    }
}
