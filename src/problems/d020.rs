pub struct Day {
    p1: i64,
    p2: i64,
    input: String,
    stack: Vec<i64>,
    stack_p2: Vec<i64>,

    indices: Vec<usize>,
    indices_p2: Vec<usize>,
}

fn mov<T>(v: &mut Vec<T>, idx: i64, by: i64) {
    let len = v.len() as i64 - 1;
    if by % len == 0 {
        return;
    }
    let new_idx = (idx + (by % len) + len) % len;
    // println!("{} {} {} {}", idx, e_by, by, len);
    let v_r = v.remove(idx as usize);
    v.insert(new_idx as usize, v_r);
}

impl crate::Problem for Day {
    const YEAR: u32 = crate::YEAR;
    const DAY: u32 = 20;

    fn new(input: String) -> Self {
        Self {
            p1: 0,
            p2: 0,
            input,
            stack: Vec::new(),
            stack_p2: Vec::new(),
            indices: Vec::new(),
            indices_p2: Vec::new(),
        }
    }

    fn do_p1(&mut self) {
        let mut parse = nom::multi::separated_list1::<_, _, _, nom::error::Error<_>, _, _>(
            nom::character::complete::char('\n'),
            nom::character::complete::i64,
        );
        self.stack = parse(self.input.as_str()).unwrap().1;
        self.indices = (0..self.stack.len()).collect();
        for i in 0..self.stack.len() {
            let idx = self.indices.iter().position(|v| *v == i).unwrap();
            let by = self.stack[idx];
            mov(&mut self.indices, idx as i64, by);
            mov(&mut self.stack, idx as i64, by);
        }

        let z_idx = self.stack.iter().position(|v| *v == 0).unwrap();
        let len = self.stack.len();
        self.p1 = self.stack[(1000 + z_idx) % len]
            + self.stack[(2000 + z_idx) % len]
            + self.stack[(3000 + z_idx) % len];
    }

    fn do_p2(&mut self) {
        let mut parse = nom::multi::separated_list1::<_, _, _, nom::error::Error<_>, _, _>(
            nom::character::complete::char('\n'),
            nom::character::complete::i64,
        );
        self.stack_p2 = parse(self.input.as_str())
            .unwrap()
            .1
            .iter()
            .map(|v| v * 811589153)
            .collect();
        self.indices_p2 = (0..self.stack_p2.len()).collect();
        for _ in 0..10 {
            for i in 0..self.stack_p2.len() {
                let idx = self.indices_p2.iter().position(|v| *v == i).unwrap();
                let by = self.stack_p2[idx];
                mov(&mut self.indices_p2, idx as i64, by);
                mov(&mut self.stack_p2, idx as i64, by);
            }
        }
        let z_idx = self.stack_p2.iter().position(|v| *v == 0).unwrap();
        let len = self.stack_p2.len();
        self.p2 = self.stack_p2[(1000 + z_idx) % len]
            + self.stack_p2[(2000 + z_idx) % len]
            + self.stack_p2[(3000 + z_idx) % len];
    }

    fn p1_result(&self) -> String {
        format!("{}", self.p1)
    }

    fn p2_result(&self) -> String {
        format!("{}", self.p2)
    }
}
