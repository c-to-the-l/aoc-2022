use std::collections:: VecDeque;

use nom::multi::separated_list1;

#[derive(Debug)]
enum Operation {
    Square,
    Add(u64),
    Multiply(u64),
}

impl Operation {
    fn apply(&self, v: u64) -> u64 {
        match self {
            Self::Square => v * v,
            Self::Add(n) => *n + v,
            Self::Multiply(n) => *n * v,
        }
    }

    fn from_input(op: char, val: &str) -> Self {
        match op {
            '+' => Self::Add(val.parse().unwrap()),
            '*' => match val {
                    "old" => Self::Square,
                    x => Self::Multiply(x.parse().unwrap())
                }
            x => panic!("Unexpected item in bagging area: {}", x),

        }
    }
}

pub struct Monkey {
    starting_items: Vec<u64>,
    items: VecDeque<u64>,
    operation: Operation,
    test: u64,
    if_false: usize,
    if_true: usize,
    inspected_items: usize,
    cap: u64,
}

impl Monkey {
    fn new(starting_items: Vec<u64>, operator: char, op_value: &str, test: u64, if_false: usize, if_true: usize) -> Self {
        Self {
            
            items: VecDeque::from(starting_items.clone()), 
            starting_items, 
            operation: Operation::from_input(operator, op_value), 
            test, 
            if_false,
            if_true,
            inspected_items: 0,
            cap: 0,
        }
    }

    fn yeet(&mut self) -> Option<(u64, usize)> {
        if let Some(v) = self.items.pop_front() {
            // log::debug!("Monkey {} inspecting item with worry level {}.", self.monkey_num, v);
            self.inspected_items += 1;
            let v = self.operation.apply(v);
            // log::debug!("Monkey {} applies {:?} to give {}", self.monkey_num, self.operation, v);
            let v = v / 3;
            // log::debug!("Value is divided by 3 to give {}", v);
            if v % self.test == 0 {
                // log::debug!("Value is divisible by {}: {}, yote from {} to {}", self.test, v, self.monkey_num, self.if_true);
                Some((v, self.if_true))
            } else {
                // log::debug!("Value is not divisible by {}: {}, yote from {} to {}", self.test, v, self.monkey_num, self.if_false);
                Some((v, self.if_false))
            }
        } else {
            None
        }
    }

    fn yeet_p2(&mut self) -> Option<(u64, usize)> {
        if let Some(v) = self.items.pop_front() {
            self.inspected_items += 1;
            let v = self.operation.apply(v);
            if v % self.test == 0 {
                Some((v % self.cap, self.if_true))
            } else {
                Some((v % self.cap, self.if_false))
            }
        } else {
            None
        }
    }

    fn yoink(&mut self, val: u64) {
        self.items.push_back(val);
    }

    fn reset(&mut self) {
        self.inspected_items = 0;
        self.items = VecDeque::from(self.starting_items.clone());
    }

    pub fn inspected_items(&self) -> usize {
        self.inspected_items
    }
}

pub struct Day {
    p1: usize,
    p2: usize,
    input: String,
    monkeys: Vec<Monkey>,
}

impl crate::Problem for Day {
    const YEAR: u32 = crate::YEAR;
    const DAY: u32 = 11;

    fn new(input: String) -> Self {
        Self {
            p1: 0,
            p2: 0,
            input,
            monkeys: Vec::new(),
        }
    }

    fn do_p1(&mut self) {
        let mut monkey_parser = nom::sequence::tuple::<&str, _, nom::error::Error<_>, _>((
            nom::bytes::complete::tag("Monkey "),
            nom::character::complete::u64,
            nom::bytes::complete::tag(":\n  Starting items: "),
            separated_list1(
                nom::bytes::complete::tag(", "),
                nom::character::complete::u64,
            ),
            nom::bytes::complete::tag("\n  Operation: new = old "),
            nom::character::complete::one_of("+*"),
            nom::character::complete::char(' '),
            nom::character::complete::not_line_ending,
            nom::bytes::complete::tag("\n  Test: divisible by "),
            nom::character::complete::u64,
            nom::bytes::complete::tag("\n    If true: throw to monkey "),
            nom::character::complete::u64,
            nom::bytes::complete::tag("\n    If false: throw to monkey "),
            nom::character::complete::u64,
        ));

        for block in self.input.split("\n\n") {
            let (
                _,
                (
                    _,
                    _monkey_num,
                    _,
                    starting_items,
                    _,
                    op,
                    _,
                    val_or_old,
                    _,
                    divisible_by,
                    _,
                    if_true,
                    _,
                    if_false,
                ),
            ) = monkey_parser(block).unwrap();
            self.monkeys.push(Monkey::new(starting_items, op, val_or_old, divisible_by, if_false as usize, if_true as usize))
        }

        for _ in 0..20 {
            for monkey in 0..self.monkeys.len() {
                while let Some((value, yeet_to)) = self.monkeys[monkey].yeet() {
                    self.monkeys[yeet_to].yoink(value)
                }
            }
        }

        let (first, second) = self.monkeys.iter().fold((0,0), |(a, b), m| {
            (
                a.max(m.inspected_items()),
                a.min(b.max(m.inspected_items()))
            )
        });

        self.p1 = first * second;

    }

    fn do_p2(&mut self) {
        let cap: u64 = self.monkeys.iter().map(|m| m.test).product();
        for i in 0..self.monkeys.len() {
            self.monkeys[i].reset();
            self.monkeys[i].cap = cap;
        }
        
        for _i in 0..10000 {
            for m in 0..self.monkeys.len() {
                while let Some((value, yeet_to)) = self.monkeys[m].yeet_p2() {
                    self.monkeys[yeet_to].yoink(value)
                }
            }
        }

        let (first, second) = self.monkeys.iter().fold((0,0), |(a, b), m| {
            (
                a.max(m.inspected_items()),
                a.min(b.max(m.inspected_items()))
            )
        });

        self.p2 = first * second;
    }

    fn p1_result(&self) -> String {
        format!("{}", self.p1)
    }

    fn p2_result(&self) -> String {
        format!("{}", self.p2)
    }
}
