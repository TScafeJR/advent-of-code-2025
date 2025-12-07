use crate::days::Day;

fn is_valid(g: &Vec<Vec<char>>, x: usize, y: usize, invalid_char: char, max: usize) -> bool {
    let mut invalids = 0;
    if y != 0 {
        if x != 0 {
            if g[y - 1][x - 1] == invalid_char {
                invalids += 1;
            }
        }

        if g[y - 1][x] == invalid_char {
            invalids += 1;
        }

        if x + 1 < g[0].len() {
            if g[y - 1][x + 1] == invalid_char {
                invalids += 1;
            }
        }
    }

    if x != 0 {
        if g[y][x - 1] == invalid_char {
            invalids += 1;
        }
    }

    if x + 1 < g[0].len() {
        if g[y][x + 1] == invalid_char {
            invalids += 1;
        }
    }

    if y < g[0].len() - 1 {
        if x != 0 {
            if g[y + 1][x - 1] == invalid_char {
                invalids += 1;
            }
        }

        if g[y + 1][x] == invalid_char {
            invalids += 1;
        }

        if x + 1 < g[0].len() {
            if g[y + 1][x + 1] == invalid_char {
                invalids += 1;
            }
        }
    }

    invalids > max
}

fn part1(data: Vec<String>) -> u64 {
    let mut count = 0;
    let g: Vec<Vec<char>> = data.into_iter().map(|x| x.chars().collect()).collect();
    for y in 0..g.len() {
        for x in 0..g[0].len() {
            if g[y][x] == '@' && !is_valid(&g, x, y, '@', 3) {
                count += 1;
            }
        }
    }
    count
}

fn part2(data: Vec<String>) -> u64 {
    let mut count = 0;
    let mut g: Vec<Vec<char>> = data.into_iter().map(|x| x.chars().collect()).collect();
    let mut run_again = true;
    while run_again {
        run_again = false;
        for y in 0..g.len() {
            for x in 0..g[0].len() {
                if g[y][x] == '@' && !is_valid(&g, x, y, '@', 3) {
                    g[y][x] = '.';
                    count += 1;
                    run_again = true;
                }
            }
        }
    }
    count
}

pub fn functions() -> Day {
    Day {
        part1: Some(part1),
        part2: Some(part2),
    }
}
