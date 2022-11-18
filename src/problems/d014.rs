
pub struct Day {
    p1: i64,
    p2: i64,
}

impl crate::Problem for Day {
    const YEAR: u32 = crate::YEAR;
    const DAY: u32 = 14;

    fn new<S: AsRef<str>>(_input: S) -> Self {
        Self{p1: 0, p2: 0}
    }

    fn do_p1(&mut self) {
        
    }

    fn do_p2(&mut self) {
        
    }

    fn p1_result(&self) -> String {
        format!("{}", self.p1)
    }

    fn p2_result(&self) -> String {
        format!("{}", self.p2)
    }
}