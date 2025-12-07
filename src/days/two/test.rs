#[cfg(test)]
use super::main::get_factors;
#[cfg(test)]
use super::main::is_invalid;
#[cfg(test)]
use super::main::is_invalid_p2;
#[cfg(test)]
use crate::days;
#[cfg(test)]
use crate::util;
#[cfg(test)]
use std::path::PathBuf;

#[cfg(test)]
mod tests {
    use super::*;

    fn get_data() -> Vec<String> {
        let data_path = PathBuf::from(format!("src/days/{}/data/test.txt", "two"));
        util::files::read_file_line_by_line(data_path)
    }

    #[test]
    fn valid_id() {
        let result = is_invalid(123);
        assert_eq!(result, false)
    }

    #[test]
    fn invalid_id() {
        let result = is_invalid(123123);
        assert_eq!(result, true)
    }

    #[test]
    fn valid_id_p2() {
        let result = is_invalid_p2(123);
        assert_eq!(result, false)
    }

    #[test]
    fn invalid_id_p2() {
        let result = is_invalid_p2(123123);
        assert_eq!(result, true)
    }

    #[test]
    fn invalid_id_p2_special() {
        let result = is_invalid_p2(111);
        assert_eq!(result, true)
    }

    #[test]
    fn invalid_id_p2_special_repeating() {
        let result = is_invalid_p2(2121212121);
        assert_eq!(result, true)
    }

    #[test]
    fn get_factors_test() {
        let factors = get_factors(9);
        assert_eq!(factors.len(), 2);
    }

    #[test]
    fn p1() {
        if let Some(day) = days::get_day_from_str("two") {
            if let Some(parsed_fns) = days::get_day(day) {
                if let Some(part1_fn) = parsed_fns.part1 {
                    let result = part1_fn(get_data());
                    assert_eq!(result, 1227775554);
                    return;
                }

                panic!("Part 1 is not supported.");
            } else {
                panic!("Part 1 is not supported.");
            }
        } else {
            panic!("Day {} is not supported.", "two");
        }
    }

    #[test]
    fn p2() {
        if let Some(day) = days::get_day_from_str("two") {
            if let Some(parsed_fns) = days::get_day(day) {
                if let Some(part2_fn) = parsed_fns.part2 {
                    let result = part2_fn(get_data());
                    assert_eq!(result, 4174379265);
                    return;
                }

                panic!("Part 2 is not supported.");
            } else {
                panic!("Part 2 is not supported.");
            }
        } else {
            panic!("Day {} is not supported.", "two");
        }
    }
}
