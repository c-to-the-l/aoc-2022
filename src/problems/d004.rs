use nom::{character::complete, combinator, multi::separated_list1, sequence::tuple};

pub struct Day {
    p1: i64,
    p2: i64,
    input: String,
    ranges: Vec<(i32, i32, i32, i32)>,
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
        let mut parse_all = separated_list1::<&str, _, _, nom::error::Error<_>, _, _>(
            complete::char('\n'),
            combinator::map(
                tuple((
                    complete::i32,
                    complete::char('-'),
                    complete::i32,
                    complete::char(','),
                    complete::i32,
                    complete::char('-'),
                    complete::i32,
                )),
                |(a, _, b, _, c, _, d)| (a, b, c, d),
            ),
        );

        self.ranges = parse_all(self.input.as_str()).unwrap().1;
        self.p1 = self
            .ranges
            .iter()
            .filter(|(a, b, c, d)| {
                c >= a && c <= b && d >= a && d <= b || a >= c && a <= d && b >= c && b <= d
            })
            .count() as i64;
    }

    fn do_p2(&mut self) {
        self.p2 = self
            .ranges
            .iter()
            .filter(|(a, b, c, d)| {
                c >= a && c <= b || d >= a && d <= b || a >= c && a <= d || b >= c && b <= d
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
