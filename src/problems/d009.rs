use std::collections::HashSet;

fn new_rope() -> [Knot; 10] {
    [
        Knot::new(),
        Knot::new(),
        Knot::new(),
        Knot::new(),
        Knot::new(),
        Knot::new(),
        Knot::new(),
        Knot::new(),
        Knot::new(),
        Knot::new(),
    ]
}

struct Knot(i64, i64);

impl Knot {
    fn new() -> Self {
        Self(0, 0)
    }
    fn mov(&mut self, dir: &str) -> (i64, i64) {
        match dir {
            "R" => self.0 += 1,
            "L" => self.0 -= 1,
            "D" => self.1 += 1,
            "U" => self.1 -= 1,
            x => panic!("Unexpected direction {}", x),
        }
        (self.0, self.1)
    }

    // abuse of division's floor behaviour.
    // print!("{} ", -2i64.saturating_div(2));
    // print!("{} ", -1i64.saturating_div(2));
    // print!("{} ", 0i64.saturating_div(2));
    // print!("{} ", 1i64.saturating_div(2));
    // println!("{}", 2i64.saturating_div(2));
    // outputs "-1 0 0 0 1"
    // means that our follower only moves when the difference is more than 1, without requiring any branching.
    // We then abuse the division again, to use it as a condition for whether we should

    fn follow(&mut self, other: (i64, i64)) -> (i64, i64) {
        let lrdiff = (other.0 - self.0).min(2).max(-2).saturating_div(2).abs();
        let uddiff = (other.1 - self.1).min(2).max(-2).saturating_div(2).abs();

        self.0 += (other.0 - self.0)
            .min(2)
            .max(-2)
            .saturating_div(2 - uddiff)
            .min(1)
            .max(-1);
        self.1 += (other.1 - self.1)
            .min(2)
            .max(-2)
            .saturating_div(2 - lrdiff)
            .min(1)
            .max(-1);
        (self.0, self.1)
    }

    fn pos(&self) -> (i64, i64) {
        (self.0, self.1)
    }
}

pub struct Day {
    p1: usize,
    p2: usize,
    input: String,
    rope1_map: HashSet<(i64, i64)>,
    rope9_map: HashSet<(i64, i64)>,
}

impl crate::Problem for Day {
    const YEAR: u32 = crate::YEAR;
    const DAY: u32 = 9;

    fn new(input: String) -> Self {
        Self {
            p1: 0,
            p2: 0,
            input,
            rope1_map: HashSet::new(),
            rope9_map: HashSet::new(),
        }
    }

    fn do_p1(&mut self) {
        let mut rope = new_rope();
        for line in self.input.lines() {
            let (dir, count) = line.split_once(' ').unwrap();
            for _ in 0..count.parse::<u32>().unwrap() {
                let mut prev = rope[0].mov(dir);
                for i in 1..rope.len() {
                    prev = rope[i].follow(prev);
                }
                self.rope1_map.insert(rope[1].pos());
                self.rope9_map.insert(rope[9].pos());
            }
        }
        self.p1 = self.rope1_map.len()
    }

    fn do_p2(&mut self) {
        self.p2 = self.rope9_map.len()
    }

    fn p1_result(&self) -> String {
        format!("{}", self.p1)
    }

    fn p2_result(&self) -> String {
        format!("{}", self.p2)
    }
}
