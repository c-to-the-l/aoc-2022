use std::{collections::HashMap, iter::once, str::Lines};

struct Directory {
    subdirectories: HashMap<String, Directory>,
    files: HashMap<String, usize>,
}

impl Directory {
    fn new() -> Self {
        Directory {
            subdirectories: HashMap::new(),
            files: HashMap::new(),
        }
    }
    fn build(&mut self, l: &mut Lines) {
        loop {
            let s = l.next();
            if s.is_none() {
                return;
            }
            let mut words = s.unwrap().split(' ');
            match words.next().unwrap() {
                "$" => match words.next().unwrap() {
                    "cd" => {
                        let subdir = words.next().unwrap();
                        match subdir {
                            ".." => return,
                            x => self
                                .subdirectories
                                .entry(x.to_string())
                                .or_insert(Directory::new())
                                .build(l),
                        }
                    }
                    "ls" => {}
                    x => panic!("Unexpected command: {}", x),
                },
                "dir" => {
                    let dirname = words.next().unwrap();
                    self.subdirectories
                        .insert(dirname.to_string(), Directory::new());
                }
                size => {
                    let fname = words.next().unwrap();
                    self.files.insert(fname.to_string(), size.parse().unwrap());
                }
            }
        }
    }
    fn size(&self) -> usize {
        self.subdirectories
            .iter()
            .map(|(_, d)| d.size())
            .sum::<usize>()
            + self.files.iter().map(|(_, s)| s).sum::<usize>()
    }
    fn threshold_size(&self, threshold: usize) -> usize {
        if self.size() > threshold {
            0 + self
                .subdirectories
                .iter()
                .map(|(_, d)| d.threshold_size(threshold))
                .sum::<usize>()
        } else {
            self.size()
                + self
                    .subdirectories
                    .iter()
                    .map(|(_, d)| d.threshold_size(threshold))
                    .sum::<usize>()
        }
    }

    fn size_closest_to(&self, s: usize) -> Option<usize> {
        let self_size = self.size();
        if self_size > s {
            once(self_size)
                .chain(
                    self.subdirectories
                        .iter()
                        .filter_map(|(_, d)| d.size_closest_to(s)),
                )
                .min()
        } else {
            None
        }
    }
}

pub struct Day {
    p1: usize,
    p2: usize,
    input: String,
    root: Directory,
}

impl crate::Problem for Day {
    const YEAR: u32 = crate::YEAR;
    const DAY: u32 = 7;

    fn new(input: String) -> Self {
        Self {
            p1: 0,
            p2: 0,
            input,
            root: Directory::new(),
        }
    }

    fn do_p1(&mut self) {
        let mut l = self.input.lines();
        self.root.build(&mut l);
        self.p1 = self.root.threshold_size(100000)
    }

    fn do_p2(&mut self) {
        let to_free = self.root.size() - (70_000_000 - 30_000_000);
        log::debug!("Will free {}", to_free);
        self.p2 = self.root.size_closest_to(to_free).unwrap();
    }

    fn p1_result(&self) -> String {
        format!("{}", self.p1)
    }

    fn p2_result(&self) -> String {
        format!("{}", self.p2)
    }
}
