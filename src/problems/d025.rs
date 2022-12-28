fn snafu_num(c: char) -> i64 {
    match c {
        '=' => -2,
        '-' => -1,
        '0' => 0,
        '1' => 1,
        '2' => 2,
        x => panic!("Unexpected input character: {}", x),
    }
}

const SNAFU_CHAR: [char; 5] = ['0', '1', '2', '=', '-'];

fn parse_snafu(s: &str) -> i64 {
    s.chars()
        .rev()
        .enumerate()
        .map(|(ord, c)| snafu_num(c) * 5i64.pow(ord as u32))
        .sum::<i64>()
}

fn print_snafu(v: i64) -> String {
    if v != 0 {
        match v % 5 {
            0..=2 => {
                let mut s = print_snafu(v / 5);
                s.push(SNAFU_CHAR[v as usize % 5]);
                s
            }
            // we can't represent our number if it's 3 or 4, so we
            // add one to the next digit up, and then subtract
            // to create the correct number.
            3 | 4 => {
                let mut s = print_snafu((v / 5) + 1);
                s.push(SNAFU_CHAR[v as usize % 5]);
                s
            }
            x => panic!("Unexpected snafu print value: {}", x),
        }
    } else {
        String::new()
    }
}

pub struct Day {
    p1: String,
    p2: i64,
    input: String,
    in_snafu: Vec<i64>,
}

impl crate::Problem for Day {
    const YEAR: u32 = crate::YEAR;
    const DAY: u32 = 25;

    fn new(input: String) -> Self {
        Self {
            p1: String::new(),
            p2: 0,
            input,
            in_snafu: Vec::new(),
        }
    }

    fn do_p1(&mut self) {
        self.in_snafu = self.input.lines().map(parse_snafu).collect();
        self.p1 = print_snafu(self.in_snafu.iter().sum());
    }

    fn do_p2(&mut self) {}

    fn p1_result(&self) -> String {
        format!("{}", self.p1)
    }

    fn p2_result(&self) -> String {
        format!("{}", self.p2)
    }
}
