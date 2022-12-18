use std::collections::HashSet;

const P2_N: usize = 1000000000000;

trait ArrayStuff<const N: usize> {
    fn left(&mut self);
    fn right(&mut self);
    fn down(&mut self);
    fn all_lr_gt(&self, v: usize) -> bool;
    fn all_lr_lt(&self, v: usize) -> bool;
    fn all_h_gt0(&self) -> bool;
    fn highest(&self) -> usize;
}

impl<const N: usize> ArrayStuff<N> for [(usize, usize); N] {
    fn left(&mut self) {
        for idx in 0..N {
            self[idx].1 -= 1;
        }
    }

    fn right(&mut self) {
        for idx in 0..N {
            self[idx].1 += 1;
        }
    }

    fn down(&mut self) {
        for idx in 0..N {
            self[idx].0 -= 1;
        }
    }

    fn all_lr_gt(&self, v: usize) -> bool {
        self.iter().all(|(_h, lr)| *lr > v)
    }
    fn all_lr_lt(&self, v: usize) -> bool {
        self.iter().all(|(_h, lr)| *lr < v)
    }
    fn all_h_gt0(&self) -> bool {
        self.iter().all(|(h, _lr)| *h > 0)
    }
    fn highest(&self) -> usize {
        self.iter().max_by(|(h1, _), (h2, _)| h1.cmp(h2)).unwrap().0
    }
}

impl Day {
    fn next_wind(&mut self) -> u8 {
        self.windex += 1;
        self.input[(self.windex - 1) % self.input.len()]
    }

    fn drop_rock<const N: usize>(&mut self, mut rock: [(usize, usize); N]) {
        loop {
            match self.next_wind() {
                60 => {
                    // less than
                    if rock.all_lr_gt(0)
                        && !rock
                            .iter()
                            .any(|(h, lr)| self.chamber.contains(&(*h, lr - 1)))
                    {
                        rock.left();
                    }
                }
                62 => {
                    // more than
                    if rock.all_lr_lt(6)
                        && !rock
                            .iter()
                            .any(|(h, lr)| self.chamber.contains(&(*h, lr + 1)))
                    {
                        rock.right();
                    }
                }
                x => panic!("Unexpected wind output: {:?}", x),
            }
            if rock.all_h_gt0()
                && !rock
                    .iter()
                    .any(|(h, lr)| self.chamber.contains(&(h - 1, *lr)))
            {
                rock.down();
            } else {
                break;
            }
        }
        self.chamber.extend(rock.iter());
        let new_h = self.height.max(rock.highest() + 1);
        self.height_deltas.push(new_h - self.height);
        self.height = new_h;
    }

    #[allow(dead_code)]
    fn print_chamber(&self) {
        let max_h = self
            .chamber
            .iter()
            .max_by(|(h1, _), (h2, _)| h1.cmp(h2))
            .unwrap_or(&(0, 0))
            .0;
        for h in (max_h - 8..max_h + 4).rev() {
            print!("|");
            for lr in 0..7 {
                if self.chamber.contains(&(h, lr)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("|");
        }
        println!("+       +");

        for h in (0..10).rev() {
            print!("|");
            for lr in 0..7 {
                if self.chamber.contains(&(h, lr)) {
                    print!("#");
                } else {
                    print!(" ");
                }
            }
            println!("|");
        }
        println!("+-------+");
    }

    /// search the height_deltas vec to find a repeating cycle.
    /// returns (offset, cycle_length)
    fn find_repeat_period(&self) -> (usize, usize) {
        for offset in 0..self.height_deltas.len() / 2 {
            'wind: for window in 50..(self.height_deltas.len() / 2 - offset) {
                for idx in offset..offset + window {
                    if self.height_deltas[idx] != self.height_deltas[idx + window] {
                        continue 'wind;
                    }
                }
                return (window, offset);
            }
        }
        todo!()
    }
}

fn make_hyphen(height: usize) -> [(usize, usize); 4] {
    [
        (height + 3, 2),
        (height + 3, 3),
        (height + 3, 4),
        (height + 3, 5),
    ]
}

fn make_plus(height: usize) -> [(usize, usize); 5] {
    [
        (height + 4, 2),
        (height + 4, 3),
        (height + 4, 4),
        (height + 3, 3),
        (height + 5, 3),
    ]
}

fn make_l(height: usize) -> [(usize, usize); 5] {
    [
        (height + 3, 2),
        (height + 3, 3),
        (height + 3, 4),
        (height + 4, 4),
        (height + 5, 4),
    ]
}

fn make_bar(height: usize) -> [(usize, usize); 4] {
    [
        (height + 3, 2),
        (height + 4, 2),
        (height + 5, 2),
        (height + 6, 2),
    ]
}

fn make_square(height: usize) -> [(usize, usize); 4] {
    [
        (height + 3, 2),
        (height + 3, 3),
        (height + 4, 2),
        (height + 4, 3),
    ]
}

pub struct Day {
    p1: usize,
    p2: usize,
    input: Vec<u8>,
    windex: usize,
    chamber: HashSet<(usize, usize)>,
    height_deltas: Vec<usize>,
    height: usize,
}

impl crate::Problem for Day {
    const YEAR: u32 = crate::YEAR;
    const DAY: u32 = 17;

    fn new(input: String) -> Self {
        Self {
            p1: 0,
            p2: 0,
            input: input.trim().as_bytes().to_vec(),
            windex: 0,
            chamber: HashSet::new(),
            height_deltas: Vec::new(),
            height: 0,
        }
    }

    fn do_p1(&mut self) {
        // move the input string so that borrowing the string with an iterator doesn't mess up the mutable borrow of &self
        let mut dropped = 0;
        loop {
            self.drop_rock(make_hyphen(self.height));
            self.drop_rock(make_plus(self.height));
            dropped += 2;
            if dropped >= 2022 {
                assert!(dropped == 2022); // else our shortcut will fall over
                break;
            }
            self.drop_rock(make_l(self.height));
            self.drop_rock(make_bar(self.height));
            self.drop_rock(make_square(self.height));
            dropped += 3;
        }
        self.p1 = self.height;
    }

    fn do_p2(&mut self) {
        self.drop_rock(make_l(self.height));
        self.drop_rock(make_bar(self.height));
        self.drop_rock(make_square(self.height));
        // The number of repeats below is a guess, mostly. We just need enough to be able to find some sort of repeating cycle in the height deltas.
        for _ in 0..600 {
            self.drop_rock(make_hyphen(self.height));
            self.drop_rock(make_plus(self.height));
            self.drop_rock(make_l(self.height));
            self.drop_rock(make_bar(self.height));
            self.drop_rock(make_square(self.height));
        }
        let (window, offset) = self.find_repeat_period();
        let offset_height: usize = self.height_deltas[..offset].iter().sum();
        let window_height: usize = self.height_deltas[offset..offset + window].iter().sum();
        let mult = (P2_N - offset) / window;
        let remain = (P2_N - offset) % window;
        let remain_height: usize = self.height_deltas[offset..offset + remain].iter().sum();
        self.p2 = offset_height + window_height * mult + remain_height;
    }

    fn p1_result(&self) -> String {
        format!("{}", self.p1)
    }

    fn p2_result(&self) -> String {
        format!("{}", self.p2)
    }
}
