// A X 1 Rock
// B Y 2 Paper
// C Z 3 Scissors


fn score(s: &str) -> i64 {
    match s.split_once(' ') {
        Some(("A", d)) => {
            match d {
                "X" => 1 + 3,
                "Y" => 2 + 6,
                "Z" => 3,
                x => panic!("Sub Unexpected: {}", x),
            }
        }
        Some(("B", d)) => {
            match d {
                "X" => 1,
                "Y" => 2 + 3,
                "Z" => 3 + 6,
                x => panic!("Sub Unexpected: {}", x),
            }
        }
        Some(("C", d)) => {
            match d {
                "X" => 1 + 6,
                "Y" => 2,
                "Z" => 3 + 3,
                x => panic!("Sub Unexpected: {}", x),
            }
        }
        x => panic!("Unexpected: {:?}", x),
    }
}

// A rock
// B paper
// C scissors

// X Lose
// Y draw
// Z win

fn score_p2 (s: &str) -> i64 {
    match s.split_once(' ') {
        Some(("A", d)) => {
            match d {
                "X" => 3 + 0,
                "Y" => 1 + 3,
                "Z" => 2 + 6,
                x => panic!("Sub Unexpected: {}", x),
            }
        }
        Some(("B", d)) => {
            match d {
                "X" => 1,
                "Y" => 2 + 3,
                "Z" => 3 + 6,
                x => panic!("Sub Unexpected: {}", x),
            }
        }
        Some(("C", d)) => {
            match d {
                "X" => 0 + 2,
                "Y" => 3 + 3,
                "Z" => 6 + 1,
                x => panic!("Sub Unexpected: {}", x),
            }
        }
        x => panic!("Unexpected: {:?}", x),
    }
}

pub struct Day {
    p1: i64,
    p2: i64,
    input: String,
}

impl crate::Problem for Day {
    const YEAR: u32 = crate::YEAR;
    const DAY: u32 = 2;

    fn new(input: String) -> Self {
        Self {
            p1: 0,
            p2: 0,
            input,
        }
    }

    fn do_p1(&mut self) {
        self.p1 = self.input.lines().map(|s| score(s)).sum();
    }

    fn do_p2(&mut self) {
        self.p2 = self.input.lines().map(|s| score_p2(s)).sum();
    }

    fn p1_result(&self) -> String {
        format!("{}", self.p1)
    }

    fn p2_result(&self) -> String {
        format!("{}", self.p2)
    }
}
