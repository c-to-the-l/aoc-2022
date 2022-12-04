// A X 1 Rock
// B Y 2 Paper
// C Z 3 Scissors

fn score(s: &str) -> i64 {
    match s {
        "A X" => 4,
        "A Y" => 8,
        "A Z" => 3,
        "B X" => 1,
        "B Y" => 5,
        "B Z" => 9,
        "C X" => 7,
        "C Y" => 2,
        "C Z" => 6,
        x => panic!("Unexpected: {}", x),
    }
}

// A rock
// B paper
// C scissors

// X Lose
// Y draw
// Z win

fn score_p2(s: &str) -> i64 {
    match s {
        "A X" => 3,
        "A Y" => 4,
        "A Z" => 8,
        "B X" => 1,
        "B Y" => 5,
        "B Z" => 9,
        "C X" => 2,
        "C Y" => 6,
        "C Z" => 7,
        x => panic!("Unexpected: {}", x),
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
