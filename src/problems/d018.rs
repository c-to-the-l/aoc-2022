use std::collections::{HashSet, VecDeque};

const ADJ: [(i32, i32, i32); 6] = [
    (1, 0, 0),
    (-1, 0, 0),
    (0, 1, 0),
    (0, -1, 0),
    (0, 0, 1),
    (0, 0, -1),
];

trait TupleOps {
    fn add(&self, other: &Self) -> Self;
    fn in_bounds(&self, lo: &Self, hi: &Self) -> bool;
}

impl TupleOps for (i32, i32, i32) {
    fn add(&self, other: &Self) -> Self {
        (self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }

    fn in_bounds(&self, lo: &Self, hi: &Self) -> bool {
        self.0 >= lo.0
            && self.1 >= lo.1
            && self.2 >= lo.2
            && self.0 <= hi.0
            && self.1 <= hi.1
            && self.2 <= hi.2
    }
}

trait SetCubeOps {
    fn num_adjacent(&self, other: &(i32, i32, i32)) -> usize;
}

impl SetCubeOps for HashSet<(i32, i32, i32)> {
    fn num_adjacent(&self, other: &(i32, i32, i32)) -> usize {
        ADJ.iter()
            .filter(|adj| self.contains(&other.add(*adj)))
            .count()
    }
}

pub struct Day {
    p1: usize,
    p2: usize,
    input: String,
    cubes: HashSet<(i32, i32, i32)>,
}

impl crate::Problem for Day {
    const YEAR: u32 = crate::YEAR;
    const DAY: u32 = 18;

    fn new(input: String) -> Self {
        Self {
            p1: 0,
            p2: 0,
            input,
            cubes: HashSet::new(),
        }
    }

    fn do_p1(&mut self) {
        let mut parse = nom::sequence::tuple::<_, _, nom::error::Error<_>, _>((
            nom::character::complete::i32,
            nom::character::complete::char(','),
            nom::character::complete::i32,
            nom::character::complete::char(','),
            nom::character::complete::i32,
        ));
        self.cubes = self
            .input
            .lines()
            .map(|s| {
                let (_, (a, _, b, _, c)) = parse(s).unwrap();
                (a, b, c)
            })
            .collect();
        self.p1 = self
            .cubes
            .iter()
            .map(|c| 6 - self.cubes.num_adjacent(c))
            .sum();
    }

    fn do_p2(&mut self) {
        let hi = self
            .cubes
            .iter()
            .fold((0, 0, 0), |(a1, b1, c1), (a2, b2, c2)| {
                (a1.max(*a2), b1.max(*b2), c1.max(*c2))
            });
        let lo = self
            .cubes
            .iter()
            .fold((21, 21, 21), |(a1, b1, c1), (a2, b2, c2)| {
                (a1.min(*a2), b1.min(*b2), c1.min(*c2))
            });
        let hi = hi.add(&(1, 1, 1));
        let lo = lo.add(&(-1, -1, -1));
        let mut visited_or_will_visit: HashSet<(i32, i32, i32)> = HashSet::from([(0, 0, 0)]);
        let mut to_visit: VecDeque<(i32, i32, i32)> = VecDeque::from([(0, 0, 0)]);
        while let Some(here) = to_visit.pop_front() {
            self.p2 += self.cubes.num_adjacent(&here);
            for adj in ADJ.iter() {
                let next = here.add(adj);
                if next.in_bounds(&lo, &hi)
                    && !visited_or_will_visit.contains(&next)
                    && !self.cubes.contains(&next)
                {
                    to_visit.push_back(next);
                    visited_or_will_visit.insert(next);
                }
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
