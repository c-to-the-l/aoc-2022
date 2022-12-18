#![allow(dead_code)]

//! This code is not done, it does not compute anything.

use std::collections::{HashMap, HashSet};

use petgraph::{algo::floyd_warshall, prelude::GraphMap, Directed};

fn name_ident(i: &str) -> u32 {
    (i.as_bytes()[0] - 'A' as u8) as u32 * 26 + (i.as_bytes()[1] - 'A' as u8) as u32
}

impl Day {
    fn final_result(&mut self, per_minute: u32, total: u32, time: u32) {
        self.combos += 1;
        let fin = total + per_minute * (30 - time);
        if self.p1 < fin {
            self.p1 = fin;
            println!("New max: {} {}", self.p1, self.combos);
        }
    }

    fn walk(
        &mut self,
        flows: &HashMap<u32, u32>,
        here: u32,
        per_minute: u32,
        total: u32,
        time: u32,
        journey_time: u32,
    ) {
        if time + journey_time >= 30 {
            self.final_result(per_minute, total, time);
            return;
        }
        let new_time = time + journey_time;
        let new_total = total + per_minute * journey_time;
        let here_flow = *flows.get(&here).unwrap_or(&0);
        for (next, _) in flows.iter().filter(|v| *v.0 != here) {
            let next_time = *self.costs.get(&(here, *next)).unwrap();

            if !self.opened.contains(&here) && new_time < 29 {
                self.opened.insert(here);
                self.walk(
                    flows,
                    *next,
                    per_minute + here_flow,
                    new_total + per_minute,
                    new_time + 1,
                    next_time,
                );
                self.opened.remove(&here);
            }
            self.walk(flows, *next, per_minute, new_total, new_time, next_time);
        }
    }
}

pub struct Day {
    p1: u32,
    p2: u32,
    combos: usize,
    input: String,
    graph: GraphMap<u32, (), Directed>,
    costs: HashMap<(u32, u32), u32>,
    flows: HashMap<u32, u32>,
    opened: HashSet<u32>,
}

impl crate::Problem for Day {
    const YEAR: u32 = crate::YEAR;
    const DAY: u32 = 16;

    fn new(input: String) -> Self {
        Self {
            p1: 0,
            p2: 0,
            input,
            combos: 0,
            graph: GraphMap::new(),
            costs: HashMap::new(),
            flows: HashMap::new(),
            opened: HashSet::new(),
        }
    }

    fn do_p1(&mut self) {
        let mut parse = nom::sequence::tuple::<_, _, nom::error::Error<_>, _>((
            nom::bytes::complete::tag("Valve "),
            nom::bytes::complete::take(2usize),
            nom::bytes::complete::tag(" has flow rate="),
            nom::character::complete::u32,
            nom::branch::alt((
                nom::bytes::complete::tag("; tunnels lead to valves "),
                nom::bytes::complete::tag("; tunnel leads to valve "),
            )),
            nom::multi::separated_list1(
                nom::bytes::complete::tag(", "),
                nom::bytes::complete::take(2usize),
            ),
        ));
        let mut flows: HashMap<u32, u32> = HashMap::new();
        for line in self.input.lines() {
            let (_, (_, id_s, _, flow, _, adj)) = parse(line).unwrap();
            let id = name_ident(id_s);
            if flow > 0 {
                flows.insert(id, flow);
            }
            for a in adj.iter() {
                let a_id = name_ident(a);
                self.graph.add_edge(a_id, id, ());
                self.graph.add_edge(id, a_id, ());
            }
        }
        drop(parse);
        self.costs = floyd_warshall(&self.graph, |_| 1_u32).unwrap();

        // self.walk(&flows, name_ident("AA"), 0, 0, 0, 0);
    }

    fn do_p2(&mut self) {}

    fn p1_result(&self) -> String {
        format!(":(")
    }

    fn p2_result(&self) -> String {
        format!(":(")
    }
}
