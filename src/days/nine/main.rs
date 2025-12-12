use crate::days::Day;
use rayon::prelude::*;
use std::collections::HashSet;

#[derive(Clone, Copy, Debug, Eq, Hash)]
struct Point {
    x: u64,
    y: u64,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Point {
    fn from_str(s: &str) -> Point {
        let coords: Vec<u64> = s
            .trim()
            .split(',')
            .map(|val| val.parse().expect("Error parsing coordinate integer"))
            .collect();
        Point {
            x: coords[0],
            y: coords[1],
        }
    }

    fn calc_distance(self, p: &Point) -> u64 {
        ((self.x).abs_diff(p.x) + 1) * ((self.y).abs_diff(p.y) + 1)
    }

    fn get_points_between(self, p: &Point) -> Vec<Point> {
        let mut points: Vec<Point> = Vec::new();
        for x in self.x.min(p.x)..=self.x.max(p.x) {
            for y in self.y.min(p.y)..=self.y.max(p.y) {
                points.push(Point { x, y });
            }
        }
        points
    }
}

fn part1(data: Vec<String>) -> u64 {
    let mut points: Vec<Point> = Vec::new();
    let mut distances: Vec<u64> = Vec::new();
    for entry in data {
        let p = Point::from_str(&entry);
        if !points.is_empty() {
            for entered_p in &points {
                let d = p.calc_distance(&entered_p);
                distances.push(d);
            }
        }

        points.push(p);
    }

    distances.sort_by(|a, b| b.cmp(a));

    distances[0]
}

fn part2(data: Vec<String>) -> u64 {
    let mut points: Vec<Point> = Vec::new();
    let mut boundary_set = HashSet::<Point>::new();

    for entry in data {
        let p = Point::from_str(&entry);
        if !points.is_empty() {
            let last = points.last().unwrap();
            for gp in p.get_points_between(last) {
                boundary_set.insert(gp);
            }
        }
        points.push(p);
    }
    if points.len() > 1 {
        let first = points.first().unwrap();
        let last = points.last().unwrap();
        for gp in first.get_points_between(last) {
            boundary_set.insert(gp);
        }
    }

    if points.is_empty() {
        return 0;
    }

    let mut min_x = u64::MAX;
    let mut max_x = 0;
    let mut min_y = u64::MAX;
    let mut max_y = 0;

    for p in &boundary_set {
        min_x = min_x.min(p.x);
        max_x = max_x.max(p.x);
        min_y = min_y.min(p.y);
        max_y = max_y.max(p.y);
    }

    let width = (max_x - min_x + 1) as usize;
    let height = (max_y - min_y + 1) as usize;

    let stride = width + 1;
    let mut pref = vec![0_i32; (width + 1) * (height + 1)];

    for y in 0..height {
        let mut inside = false;

        let mut row_sum = 0;

        for x in 0..width {
            let global_x = min_x + x as u64;
            let global_y = min_y + y as u64;
            let p = Point {
                x: global_x,
                y: global_y,
            };

            let mut val = 0;

            if boundary_set.contains(&p) {
                val = 1; // It's a green boundary
                         // Parity check: Look South
                let south = Point {
                    x: global_x,
                    y: global_y + 1,
                };
                if boundary_set.contains(&south) {
                    inside = !inside;
                }
            } else if inside {
                val = 1; // It's green interior
            }

            row_sum += val;

            let top_index = y * stride + (x + 1);
            let current_index = (y + 1) * stride + (x + 1);

            pref[current_index] = row_sum + pref[top_index];
        }
    }

    let pref_slice = &pref;

    let get_sum = |x1: u64, y1: u64, x2: u64, y2: u64| -> i32 {
        let c1 = (x1 - min_x) as usize;
        let r1 = (y1 - min_y) as usize;
        let c2 = (x2 - min_x) as usize;
        let r2 = (y2 - min_y) as usize;

        let br = (r2 + 1) * stride + (c2 + 1);
        let bl = (r2 + 1) * stride + c1;
        let tr = r1 * stride + (c2 + 1);
        let tl = r1 * stride + c1;

        pref_slice[br] - pref_slice[bl] - pref_slice[tr] + pref_slice[tl]
    };

    let max_area = (0..points.len() - 1)
        .into_par_iter()
        .map(|i| {
            let mut local_max = 0;
            let p1 = points[i];

            for j in (i + 1)..points.len() {
                let p2 = points[j];

                let x1 = p1.x.min(p2.x);
                let x2 = p1.x.max(p2.x);
                let y1 = p1.y.min(p2.y);
                let y2 = p1.y.max(p2.y);

                let area_calc = ((x2 - x1 + 1) * (y2 - y1 + 1)) as i32;
                let sum_greens = get_sum(x1, y1, x2, y2);

                if sum_greens == area_calc {
                    if (area_calc as u64) > local_max {
                        local_max = area_calc as u64;
                    }
                }
            }
            local_max
        })
        .max()
        .unwrap_or(0);

    max_area
}
pub fn functions() -> Day {
    Day {
        part1: Some(part1),
        part2: Some(part2),
    }
}
