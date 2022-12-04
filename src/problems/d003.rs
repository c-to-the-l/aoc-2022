pub struct Day {
    p1: i64,
    p2: i64,
    input: String,
}

fn priority(c: char) -> i64 {
    if c.is_ascii_uppercase() {
        c as i64 + 26 - '@' as i64
    } else {
        c as i64 - 96
    }
}

impl crate::Problem for Day {
    const YEAR: u32 = crate::YEAR;
    const DAY: u32 = 3;

    fn new(input: String) -> Self {
        Self {
            p1: 0,
            p2: 0,
            input,
        }
    }

    fn do_p1(&mut self) {
        self.p1 = self
            .input
            .lines()
            .filter_map(|s| {
                let (l, r) = s.split_at(s.len() / 2);
                l.chars()
                    .filter(|c| r.contains(*c))
                    .map(|c| priority(c))
                    .next()
            })
            .sum();
    }

    fn do_p2(&mut self) {
        self.p2 = self
            .input
            .lines()
            .step_by(3)
            .zip(self.input.lines().skip(1).step_by(3))
            .zip(self.input.lines().skip(2).step_by(3))
            .filter_map(|((a, b), c)| {
                a.chars()
                    .filter(|v| b.contains(*v))
                    .filter(|v| c.contains(*v))
                    .next()
            })
            .map(|v| priority(v))
            .sum();
    }

    fn p1_result(&self) -> String {
        format!("{}", self.p1)
    }

    fn p2_result(&self) -> String {
        format!("{}", self.p2)
    }
}
