pub struct Day {
    p1: i64,
    p2: i64,
    input: String,
    sums: Vec<i64>,
}

impl crate::Problem for Day {
    const YEAR: u32 = crate::YEAR;
    const DAY: u32 = 1;

    fn new(input: String) -> Self {
        Self {
            p1: 0,
            p2: 0,
            input,
            sums: Vec::new(),
        }
    }

    fn do_p1(&mut self) {
        self.sums = self
            .input
            .split("\n\n")
            .map(|v| v.lines().map(|x| x.parse::<i64>().unwrap()).sum())
            .collect();
        self.p1 = *self.sums.iter().max().unwrap();
    }

    fn do_p2(&mut self) {
        let (a, b, c): (i64, i64, i64) = self.sums.iter().fold((0, 0, 0), |(a, b, c), x| {
            (a.max(*x), a.min(b.max(*x)), a.min(b.min(c.max(*x))))
        });
        self.p2 = a + b + c;
    }

    fn p1_result(&self) -> String {
        format!("{}", self.p1)
    }

    fn p2_result(&self) -> String {
        format!("{}", self.p2)
    }
}
