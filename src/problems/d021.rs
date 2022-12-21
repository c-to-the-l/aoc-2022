use std::collections::HashMap;

fn make_idx(s: &str) -> usize {
    s.as_bytes()
        .iter()
        .enumerate()
        .map(|(j, v)| 100usize.pow(j as u32) * (v - ('a' as u8)) as usize)
        .sum()
}

#[derive(Debug)]
enum Op {
    Add,
    Sub,
    Div,
    Mul,
}

impl Op {
    fn from_char(c: char) -> Self {
        use Op::*;
        match c {
            '-' => Sub,
            '/' => Div,
            '+' => Add,
            '*' => Mul,
            x => panic!("Unexpected Operator {}", x),
        }
    }
    fn operate(&self, l: i64, r: i64) -> i64 {
        use Op::*;
        match self {
            Add => l + r,
            Sub => l - r,
            Div => l / r,
            Mul => l * r,
        }
    }

    fn inputof_r(&self, l: i64, res: i64) -> i64 {
        use Op::*;
        match self {
            Add => res - l,
            Sub => l - res,
            Div => l / res,
            Mul => res / l,
        }
    }

    fn inputof_l(&self, r: i64, res: i64) -> i64 {
        use Op::*;
        match self {
            Add => res - r,
            Sub => res + r,
            Div => res * r,
            Mul => res / r,
        }
    }
}

#[derive(Debug)]
enum Monke {
    Number(i64),
    Do(Op, usize, usize),
}

impl Monke {
    fn from_str(s: &str) -> (usize, Self) {
        let mut parse = nom::sequence::tuple::<_, _, nom::error::Error<_>, _>((
            nom::bytes::complete::take(4usize),
            nom::bytes::complete::tag(": "),
            nom::combinator::opt(nom::character::complete::i64),
            nom::combinator::opt(nom::sequence::tuple((
                nom::bytes::complete::take(4usize),
                nom::bytes::complete::tag(" "),
                nom::character::complete::one_of::<_, _, _>("-+/*"),
                nom::bytes::complete::tag(" "),
                nom::bytes::complete::take(4usize),
            ))),
        ));

        let (_, (name, _, maybe_const, maybe_op)) = parse(s).unwrap();

        if let Some(i) = maybe_const {
            (make_idx(name), Self::Number(i))
        } else if let Some((l, _, op, _, r)) = maybe_op {
            (
                make_idx(name),
                Self::Do(Op::from_char(op), make_idx(l), make_idx(r)),
            )
        } else {
            panic!("Doesn't appear to have a const or operator: {}", s);
        }
    }
}

trait MonkeAdd {
    fn get_monke(&self, idx: usize) -> i64;

    fn get_roots(&self) -> (usize, usize);
    fn needs_idx(&self, idx: usize, needs: usize) -> bool;
    fn unknown_idx(&self, here: usize, target: usize, input: i64) -> i64;
}

impl MonkeAdd for HashMap<usize, Monke> {
    fn get_monke(&self, idx: usize) -> i64 {
        match self.get(&idx) {
            Some(Monke::Number(i)) => *i,
            Some(Monke::Do(op, l, r)) => op.operate(self.get_monke(*l), self.get_monke(*r)),
            None => panic!("Unexpected monke in bagging area: {}", idx),
        }
    }

    fn get_roots(&self) -> (usize, usize) {
        match self.get(&make_idx("root")) {
            Some(Monke::Do(_, l, r)) => (*l, *r),
            x => panic!("Bad root: {:?}", x),
        }
    }

    fn needs_idx(&self, idx: usize, needs: usize) -> bool {
        match self.get(&idx) {
            Some(Monke::Number(_i)) => idx == needs,
            Some(Monke::Do(_, l, r)) => {
                (idx == needs) | self.needs_idx(*l, needs) | self.needs_idx(*r, needs)
            }
            None => panic!("Unexpected monke in bagging area: {}", idx),
        }
    }

    fn unknown_idx(&self, here: usize, target: usize, input: i64) -> i64 {
        if here == target {
            return input;
        }
        match self.get(&here) {
            Some(Monke::Do(op, l, r)) => {
                if self.needs_idx(*l, target) {
                    self.unknown_idx(*l, target, op.inputof_l(self.get_monke(*r), input))
                } else {
                    self.unknown_idx(*r, target, op.inputof_r(self.get_monke(*l), input))
                }
            }
            x => panic!("Logic Error: Called unknown_idx on {:?}", x),
        }
    }
}

pub struct Day {
    p1: i64,
    p2: i64,
    input: String,
    monkes: HashMap<usize, Monke>,
}

impl crate::Problem for Day {
    const YEAR: u32 = crate::YEAR;
    const DAY: u32 = 21;

    fn new(input: String) -> Self {
        Self {
            p1: 0,
            p2: 0,
            input,
            monkes: HashMap::new(),
        }
    }

    fn do_p1(&mut self) {
        self.monkes = self.input.lines().map(Monke::from_str).collect();
        // assert_eq!(self.monkes.len(), self.input.lines().count(), "Problem with key generator - two monkey names result in the same index");
        // println!("{:#?}", self.monkes);
        self.p1 = self.monkes.get_monke(make_idx("root"))
    }

    fn do_p2(&mut self) {
        let (rootl, rootr) = self.monkes.get_roots();
        let (humn_head, target) = if self.monkes.needs_idx(rootl, make_idx("humn")) {
            (rootl, self.monkes.get_monke(rootr))
        } else {
            assert!(
                self.monkes.needs_idx(rootr, make_idx("humn")),
                "How does neither branch of the tree require the humn input?"
            );
            (rootr, self.monkes.get_monke(rootl))
        };
        self.p2 = self.monkes.unknown_idx(humn_head, make_idx("humn"), target)
    }

    fn p1_result(&self) -> String {
        format!("{}", self.p1)
    }

    fn p2_result(&self) -> String {
        format!("{}", self.p2)
    }
}
