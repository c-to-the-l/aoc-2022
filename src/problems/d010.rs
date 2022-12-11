use std::collections::HashSet;

pub struct Day {
    p1: i64,
    p2: String,
    input: String,
}

fn draw(out: &mut String, cycle: i64, regx: i64) {
    if (0..=2).contains(&((cycle % 40) - regx)) {
        out.push('#');
    } else {
        out.push(' ');
    }
    if cycle % 40 == 0 {
        out.push('\n');
    }
}

impl crate::Problem for Day {
    const YEAR: u32 = crate::YEAR;
    const DAY: u32 = 10;

    fn new(input: String) -> Self {
        Self {
            p1: 0,
            p2: String::new(),
            input,
        }
    }

    fn do_p1(&mut self) {
        let mut cycles = 0;
        let mut regx = 1;
        let breakpoints = HashSet::from([20, 60, 100, 140, 180, 220]);

        for (instr, maybe_val) in self.input.lines().map(|l| l.split_at(4)) {
            cycles += 1;
            if breakpoints.contains(&cycles) {
                // println!("Cycle {}, x: {}, result {}. p1 {} ", cycles, regx, cycles*regx, self.p1);
                self.p1 += regx * cycles;
            }
            match instr {
                "noop" => {}
                "addx" => {
                    cycles += 1;
                    if breakpoints.contains(&cycles) {
                        // println!("Cycle {}, x: {}, result {}. p1 {} ", cycles, regx, cycles*regx, self.p1);
                        self.p1 += regx * cycles;
                    }
                    regx += maybe_val.trim().parse::<i64>().unwrap();
                }
                x => panic!("Unexpected item in bagging area: {}", x),
            }
        }
    }

    fn do_p2(&mut self) {
        self.p2.push('\n');
        let mut cycles = 0;
        let mut regx = 1;
        for (instr, maybe_val) in self.input.lines().map(|l| l.split_at(4)) {
            cycles += 1;
            draw(&mut self.p2, cycles, regx);
            match instr {
                "noop" => {}
                "addx" => {
                    cycles += 1;
                    draw(&mut self.p2, cycles, regx);
                    regx += maybe_val.trim().parse::<i64>().unwrap();
                }
                x => panic!("Unexpected item in bagging area: {}", x),
            }
        }
    }

    fn p1_result(&self) -> String {
        format!("{}", self.p1)
    }

    fn p2_result(&self) -> String {
        format!("{}", self.p2)
    }
}
