use std::collections::{HashSet, VecDeque};

const UP: usize = 0;
const DOWN: usize = 1;
const RIGHT: usize = 2;
const LEFT: usize = 3;
const HERE: usize = 4;

const NUM_DIRS: usize = 4;

const MASKS: [u8; 5] = [1 << UP, 1 << DOWN, 1 << RIGHT, 1 << LEFT, 1 << HERE];
// const DIRS: [(i64, i64); 5] = [(1, 0), (-1, 0), (0, 1), (0, -1), (0, 0)];
const REV: [(i64, i64); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn to_blizzard(c: char) -> Option<u8> {
    match c {
        '.' => Some(0),
        '^' => Some(MASKS[UP]),
        '>' => Some(MASKS[RIGHT]),
        'v' => Some(MASKS[DOWN]),
        '<' => Some(MASKS[LEFT]),
        _ => None,
    }
}

fn cyc(v: i64, lim: usize) -> usize {
    (lim as i64 + (v % lim as i64)) as usize % lim
}

fn t_add(a: &(usize, usize), b: &(i64, i64), lim: (usize, usize)) -> (usize, usize) {
    (cyc(a.0 as i64 + b.0, lim.0), cyc(a.1 as i64 + b.1, lim.1))
}

trait NdVecOps {
    fn nd_get(&self, yx: (usize, usize)) -> u8;
}

impl NdVecOps for Vec<Vec<u8>> {
    fn nd_get(&self, yx: (usize, usize)) -> u8 {
        self[yx.0][yx.1]
    }
}

pub struct Day {
    p1: usize,
    p2: usize,
    ud_lim: usize,
    lr_lim: usize,
    big_lim: usize,
    input: String,
    blizzards: Vec<Vec<Vec<u8>>>,
}

impl Day {
    fn sim_blizzards(&mut self, to: usize) {
        while self.blizzards.len() <= to {
            self.blizzards.push(vec![vec![0; self.lr_lim]; self.ud_lim]);
            let last = self.blizzards.len() - 1;
            for x in 0..self.lr_lim {
                for y in 0..self.ud_lim {
                    self.blizzards[last][y][x] = (0..NUM_DIRS).fold(0, |a, d| {
                        a | (self.blizzards[last - 1].nd_get(t_add(
                            &(y, x),
                            &REV[d],
                            (self.ud_lim, self.lr_lim),
                        )) & MASKS[d])
                    })
                }
            }
        }
    }

    #[allow(dead_code)]
    fn print_blizz(&self, c: usize) {
        for _ in 0..self.blizzards[c][0].len() + 2 {
            print!("#");
        }
        println!("");
        for y in (0..self.blizzards[c].len()).rev() {
            print!("#");
            for x in 0..self.blizzards[c][0].len() {
                match self.blizzards[c][y][x].count_ones() {
                    0 => print!(" "),
                    1 => match self.blizzards[c][y][x] {
                        d if d == MASKS[0] => print!("^"),
                        d if d == MASKS[1] => print!("v"),
                        d if d == MASKS[2] => print!(">"),
                        d if d == MASKS[3] => print!("<"),
                        x => panic!("Unexpected blizzard value: {}", x),
                    },
                    x => print!("{}", x % 10),
                }
            }
            println!("#");
        }
        for _ in 0..self.blizzards[c][0].len() + 2 {
            print!("#");
        }
        println!("");
    }

    fn find_start(&mut self, from: usize, at: (usize, usize)) -> usize {
        let mut cycle = from;
        loop {
            self.sim_blizzards(cycle);
            if self.blizzards[cycle][at.0][at.1] == 0 {
                return cycle;
            }
            cycle += 1;
        }
    }

    fn path(
        &mut self,
        start_pos: (usize, usize),
        end_pos: (usize, usize),
        start_cycle: usize,
    ) -> usize {
        let mut start_cycle = self.find_start(start_cycle, start_pos);
        let mut visited: HashSet<(usize, (usize, usize))> = HashSet::new();
        let mut to_visit: VecDeque<(usize, (usize, usize))> = VecDeque::new();
        visited.insert((start_cycle, start_pos));
        to_visit.push_back((start_cycle, start_pos));
        while let Some((cycle, d)) = to_visit.pop_front() {
            if d == end_pos {
                return cycle + 1;
            }
            let cycle = cycle + 1;
            self.sim_blizzards(cycle);
            if self.blizzards[cycle][d.0][d.1] == 0 {
                visited.insert((cycle, d));
                to_visit.push_back((cycle, d));
            }
            if d.0 > 0
                && self.blizzards[cycle][d.0 - 1][d.1] == 0
                && !visited.contains(&(cycle, (d.0 - 1, d.1)))
            {
                visited.insert((cycle, (d.0 - 1, d.1)));
                to_visit.push_back((cycle, (d.0 - 1, d.1)));
            }
            if d.0 < self.blizzards[cycle].len() - 1
                && self.blizzards[cycle][d.0 + 1][d.1] == 0
                && !visited.contains(&(cycle, (d.0 + 1, d.1)))
            {
                visited.insert((cycle, (d.0 + 1, d.1)));
                to_visit.push_back((cycle, (d.0 + 1, d.1)));
            }
            if d.1 > 0
                && self.blizzards[cycle][d.0][d.1 - 1] == 0
                && !visited.contains(&(cycle, (d.0, d.1 - 1)))
            {
                visited.insert((cycle, (d.0, d.1 - 1)));
                to_visit.push_back((cycle, (d.0, d.1 - 1)));
            }
            if d.1 < self.blizzards[cycle][0].len() - 1
                && self.blizzards[cycle][d.0][d.1 + 1] == 0
                && !visited.contains(&(cycle, (d.0, d.1 + 1)))
            {
                visited.insert((cycle, (d.0, d.1 + 1)));
                to_visit.push_back((cycle, (d.0, d.1 + 1)));
            }
            if to_visit.len() == 0 {
                start_cycle = self.find_start(start_cycle + 1, start_pos);
                visited.insert((start_cycle, start_pos));
                to_visit.push_back((start_cycle, start_pos));
            }
        }
        0
    }
}

impl crate::Problem for Day {
    const YEAR: u32 = crate::YEAR;
    const DAY: u32 = 24;

    fn new(input: String) -> Self {
        Self {
            p1: 0,
            p2: 0,
            ud_lim: 0,
            lr_lim: 0,
            big_lim: 0,
            input,
            blizzards: Vec::new(),
        }
    }

    fn do_p1(&mut self) {
        self.blizzards = vec![self
            .input
            .lines()
            .rev()
            .skip(1)
            .take_while(|s| !s.starts_with("#.#"))
            .map(|s| s.chars().filter_map(to_blizzard).collect())
            .collect()];
        self.lr_lim = self.blizzards[0][0].len();
        self.ud_lim = self.blizzards[0].len();
        self.big_lim = self.lr_lim.max(self.ud_lim) + 10;
        let start = (self.blizzards[0].len() - 1, 0);
        let end = (0, self.blizzards[0][0].len() - 1);
        self.p1 = self.path(start, end, 0);
    }

    fn do_p2(&mut self) {
        let start_pos = (self.blizzards[0].len() - 1, 0);
        let end_pos = (0, self.blizzards[0][0].len() - 1);
        let goal_1 = self.p1;
        let goal_2 = self.path(end_pos, start_pos, goal_1);
        self.p2 = self.path(start_pos, end_pos, goal_2);
    }

    fn p1_result(&self) -> String {
        format!("{}", self.p1)
    }

    fn p2_result(&self) -> String {
        format!("{}", self.p2)
    }
}
