use std::collections::{HashMap, HashSet};

fn t_add(a: &(i64, i64), b: &(i64, i64)) -> (i64, i64) {
    (a.0 + b.0, a.1 + b.1)
}

const AROUND: [(i64, i64); 8] = [
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, 1),
];

const TESTS: [[[(i64, i64); 3]; 4]; 4] = [
    [
        [(1, 1), (0, 1), (-1, 1)],    // north
        [(1, -1), (0, -1), (-1, -1)], // south
        [(-1, 1), (-1, 0), (-1, -1)], // west
        [(1, 1), (1, 0), (1, -1)],    // east
    ],
    [
        [(1, -1), (0, -1), (-1, -1)], // south
        [(-1, 1), (-1, 0), (-1, -1)], // west
        [(1, 1), (1, 0), (1, -1)],    // east
        [(1, 1), (0, 1), (-1, 1)],    // north
    ],
    [
        [(-1, 1), (-1, 0), (-1, -1)], // west
        [(1, 1), (1, 0), (1, -1)],    // east
        [(1, 1), (0, 1), (-1, 1)],    // north
        [(1, -1), (0, -1), (-1, -1)], // south
    ],
    [
        [(1, 1), (1, 0), (1, -1)],    // east
        [(1, 1), (0, 1), (-1, 1)],    // north
        [(1, -1), (0, -1), (-1, -1)], // south
        [(-1, 1), (-1, 0), (-1, -1)], // west
    ],
];

const MOVES: [[(i64, i64); 4]; 4] = [
    //n         s       w        e
    [(0, 1), (0, -1), (-1, 0), (1, 0)],
    // s         w       e      n
    [(0, -1), (-1, 0), (1, 0), (0, 1)],
    //  w        e       n       s
    [(-1, 0), (1, 0), (0, 1), (0, -1)],
    // e        n      s        w
    [(1, 0), (0, 1), (0, -1), (-1, 0)],
];

trait ElfSetOps {
    fn get_move(&self, elf: &(i64, i64), cycle: usize) -> Option<(i64, i64)>;
    fn bounds(&self) -> ((i64, i64), (i64, i64));
    fn display(&self);
}

impl ElfSetOps for HashSet<(i64, i64)> {
    fn get_move(&self, elf: &(i64, i64), cycle: usize) -> Option<(i64, i64)> {
        if AROUND.iter().all(|p| !self.contains(&t_add(elf, p))) {
            return None;
        }
        for (test, mov) in TESTS[cycle % 4].iter().zip(MOVES[cycle % 4].iter()) {
            if test.iter().all(|p| !self.contains(&t_add(elf, p))) {
                return Some(t_add(elf, mov));
            }
        }
        None
    }

    fn bounds(&self) -> ((i64, i64), (i64, i64)) {
        let y_min = self.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap().1;
        let y_max = self.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1;

        let x_min = self.iter().min_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
        let x_max = self.iter().max_by(|a, b| a.0.cmp(&b.0)).unwrap().0;

        ((x_min, y_min), (x_max, y_max))
    }
    fn display(&self) {
        let ((x_min, y_min), (x_max, y_max)) = self.bounds();
        for y in (y_min..=y_max).rev() {
            for x in x_min..=x_max {
                if self.contains(&(x, y)) {
                    print!("#")
                } else {
                    print!(".")
                }
            }
            println!("");
        }
    }
}

pub struct Day {
    p1: i64,
    p2: usize,
    input: String,
    elves: HashSet<(i64, i64)>,
}

impl crate::Problem for Day {
    const YEAR: u32 = crate::YEAR;
    const DAY: u32 = 23;

    fn new(input: String) -> Self {
        Self {
            p1: 0,
            p2: 0,
            input,
            elves: HashSet::new(),
        }
    }

    fn do_p1(&mut self) {
        self.elves = self
            .input
            .lines()
            .rev()
            .enumerate()
            .map(|(y, line)| {
                line.chars().enumerate().filter_map(move |(x, c)| {
                    if c == '#' {
                        Some((x as i64, y as i64))
                    } else {
                        None
                    }
                })
            })
            .flatten()
            .collect();
        let mut prop_moves: HashMap<(i64, i64), ((i64, i64), usize)> =
            HashMap::with_capacity(self.elves.len());
        prop_moves.insert((0, 0), ((0, 0), 0));
        for c in 0..10 {
            prop_moves.clear();
            for (elf, mov) in self
                .elves
                .iter()
                .filter_map(|e| self.elves.get_move(e, c).map(|v| (*e, v)))
            {
                let prop = prop_moves.entry(mov).or_insert(((0, 0), 0));
                prop.0 = elf;
                prop.1 += 1;
            }

            for (mov, (prev, _)) in prop_moves.iter().filter(|(_, (_, c))| *c == 1) {
                self.elves.remove(prev);
                self.elves.insert(*mov);
            }
        }
        let ((xmin, ymin), (xmax, ymax)) = self.elves.bounds();
        self.p1 = (1 + xmax - xmin) * (1 + ymax - ymin) - self.elves.len() as i64;
    }

    fn do_p2(&mut self) {
        let mut prop_moves: HashMap<(i64, i64), ((i64, i64), usize)> =
            HashMap::with_capacity(self.elves.len());
        prop_moves.insert((0, 0), ((0, 0), 0));
        self.p2 = 10;
        while !prop_moves.is_empty() {
            prop_moves.clear();
            for (elf, mov) in self
                .elves
                .iter()
                .filter_map(|e| self.elves.get_move(e, self.p2).map(|v| (*e, v)))
            {
                let prop = prop_moves.entry(mov).or_insert(((0, 0), 0));
                prop.0 = elf;
                prop.1 += 1;
            }

            for (mov, (prev, _)) in prop_moves.iter().filter(|(_, (_, c))| *c == 1) {
                self.elves.remove(prev);
                self.elves.insert(*mov);
            }
            self.p2 += 1;
        }
    }

    fn p1_result(&self) -> String {
        format!("{}", self.p1)
    }

    fn p2_result(&self) -> String {
        format!("{}", self.p2)
    }
}
