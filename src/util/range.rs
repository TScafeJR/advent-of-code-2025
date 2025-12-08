use regex::Regex;

pub type ValidatorFn = fn(n: u64) -> bool;

#[derive(Clone, Debug)]
pub struct Range {
    pub low: u64,
    pub high: u64,
    pub validator: Option<ValidatorFn>,
}

impl PartialEq for Range {
    fn eq(&self, other: &Self) -> bool {
        self.low == other.low && self.high == other.high
    }
}
impl Eq for Range {}

impl Range {
    pub fn new(s: &str) -> Self {
        Range::parse_internal(s, None)
    }

    pub fn with_validator(s: &str, v: ValidatorFn) -> Self {
        Range::parse_internal(s, Some(v))
    }

    fn parse_internal(s: &str, v: Option<ValidatorFn>) -> Self {
        let re = Regex::new(r"(\d+)-(\d+)").unwrap();
        let mut h = 0;
        let mut l = 0;

        for cap in re.captures_iter(&s) {
            l = cap[1].parse().unwrap();
            h = cap[2].parse().unwrap();
        }

        Range {
            low: l,
            high: h,
            validator: v,
        }
    }
}
