fn first_neq(s: &[u8], window_len: usize) -> usize {
    'outer: for start in 0..(s.len() - window_len) {
        for pos in start..start + window_len {
            if s[pos + 1..start + window_len].contains(&s[pos]) {
                continue 'outer;
            }
        }
        return start + window_len;
    }
    0
}

pub struct Day {
    p1: usize,
    p2: usize,
    input: String,
}

impl crate::Problem for Day {
    const YEAR: u32 = crate::YEAR;
    const DAY: u32 = 6;

    fn new(input: String) -> Self {
        Self {
            p1: 0,
            p2: 0,
            input,
        }
    }

    fn do_p1(&mut self) {
        self.p1 = first_neq(self.input.as_bytes(), 4);
    }

    fn do_p2(&mut self) {
        self.p2 = first_neq(self.input.as_bytes(), 14);
    }

    fn p1_result(&self) -> String {
        format!("{}", self.p1)
    }

    fn p2_result(&self) -> String {
        format!("{}", self.p2)
    }
}
