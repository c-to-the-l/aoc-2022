use std::ops::RangeInclusive;

pub struct Day {
    p1: i64,
    p2: i64,
    input: String,
    ranges: Vec<(RangeInclusive<i64>, RangeInclusive<i64>)>,
}

impl crate::Problem for Day {
    const YEAR: u32 = crate::YEAR;
    const DAY: u32 = 4;

    fn new(input: String) -> Self {
        Self {
            p1: 0,
            p2: 0,
            input,
            ranges: Vec::new(),
        }
    }

    fn do_p1(&mut self) {
        self.ranges = self
            .input
            .trim()
            .lines()
            .map(|s| {
                let (l, r) = s.split_once(',').unwrap();
                let (a, b) = l.split_once('-').unwrap();
                let (c, d) = r.split_once('-').unwrap();
                (
                    (a.parse().unwrap()..=b.parse().unwrap()),
                    (c.parse().unwrap()..=d.parse().unwrap()),
                )
            })
            .collect();
        self.p1 = self
            .ranges
            .iter()
            .filter(|(l, r)| {
                l.contains(r.start()) && l.contains(r.end())
                    || r.contains(l.start()) && r.contains(l.end())
            })
            .count() as i64;
    }

    fn do_p2(&mut self) {
        self.p2 = self
            .ranges
            .iter()
            .filter(|(l, r)| {
                l.contains(r.start())
                    || l.contains(r.end())
                    || r.contains(l.start())
                    || r.contains(l.end())
            })
            .count() as i64;
    }

    fn p1_result(&self) -> String {
        format!("{}", self.p1)
    }

    fn p2_result(&self) -> String {
        format!("{}", self.p2)
    }
}
