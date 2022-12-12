use std::collections::HashSet;

pub struct Day {
    p1: usize,
    p2: usize,
    input: String,
    trees: Vec<Vec<i8>>,
}

fn scenic_score(x: usize, y: usize, trees: &Vec<Vec<i8>>) -> usize {
    (trees[x][y+1..].iter().take_while(|v| **v < trees[x][y]).count().max(1) + 1).min(trees[0].len() - y - 1) * // right
    (trees[x][..y].iter().rev().take_while(|v| **v < trees[x][y]).count().max(1) + 1 ).min(y) * // left
    ((x+1..trees.len()).take_while(|xsub| trees[*xsub][y] < trees[x][y]).count().max(1)+1 ).min(trees.len() - x - 1) * // down
    ((0..x).rev().take_while(|xsub| trees[*xsub][y] < trees[x][y]).count().max(1)+1 ).min(x) // up
}

impl crate::Problem for Day {
    const YEAR: u32 = crate::YEAR;
    const DAY: u32 = 8;

    fn new(input: String) -> Self {
        Self {
            p1: 0,
            p2: 0,
            input,
            trees: Vec::new(),
        }
    }

    fn do_p1(&mut self) {
        self.trees = self
            .input
            .lines()
            .map(|l| l.bytes().map(|c| c as i8 - 48).collect())
            .collect();
        let mut tree_map: HashSet<(usize, usize)> = HashSet::new();
        for (x, row) in self.trees.iter().enumerate() {
            let (mut prev, mut prev_r) = (-1, -1);
            for (y, tree) in row.iter().enumerate() {
                if *tree > prev {
                    prev = *tree;
                    tree_map.insert((x, y));
                }
            }
            for (y, tree) in row.iter().enumerate().rev() {
                if *tree > prev_r {
                    prev_r = *tree;
                    tree_map.insert((x, y));
                }
            }
        }

        for y in 0..self.trees[0].len() {
            let (mut prev, mut prev_r) = (-1i8, -1i8);
            for x in 0..self.trees.len() {
                if self.trees[x][y] as i8 > prev {
                    prev = self.trees[x][y] as i8;
                    tree_map.insert((x, y));
                }
            }
            for x in (0..self.trees.len()).rev() {
                if self.trees[x][y] as i8 > prev_r {
                    prev_r = self.trees[x][y] as i8;
                    tree_map.insert((x, y));
                }
            }
        }

        self.p1 = tree_map.len();
    }

    fn do_p2(&mut self) {
        self.p2 = (0..self.trees.len())
            .map(|x| {
                (0..self.trees[0].len())
                    .map(|y| scenic_score(x, y, &self.trees))
                    .max()
                    .unwrap()
            })
            .max()
            .unwrap();
    }

    fn p1_result(&self) -> String {
        format!("{}", self.p1)
    }

    fn p2_result(&self) -> String {
        format!("{}", self.p2)
    }
}
