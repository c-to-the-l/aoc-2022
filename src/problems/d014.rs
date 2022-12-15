use std::collections::HashSet;

pub struct Day {
    p1: usize,
    p2: usize,
    input: String,
    rock: HashSet<(i32, i32)>,
    p1s: HashSet<(i32, i32)>,
    p2s: HashSet<(i32, i32)>,
}

impl Day {
    fn move_grain(&self, grain: (i32, i32)) -> Option<(i32, i32)> {
        if !self.p1s.contains(&(grain.0, grain.1 + 1)) {
            Some((grain.0, grain.1 + 1))
        } else if !self.p1s.contains(&(grain.0 - 1, grain.1 + 1)) {
            Some((grain.0 - 1, grain.1 + 1))
        } else if !self.p1s.contains(&(grain.0 + 1, grain.1 + 1)) {
            Some((grain.0 + 1, grain.1 + 1))
        } else {
            None
        }
    }
    fn move_grain_p2(&self, grain: (i32, i32)) -> Option<(i32, i32)> {
        if !self.p2s.contains(&(grain.0, grain.1 + 1)) {
            Some((grain.0, grain.1 + 1))
        } else if !self.p2s.contains(&(grain.0 - 1, grain.1 + 1)) {
            Some((grain.0 - 1, grain.1 + 1))
        } else if !self.p2s.contains(&(grain.0 + 1, grain.1 + 1)) {
            Some((grain.0 + 1, grain.1 + 1))
        } else {
            None
        }
    }
}

impl crate::Problem for Day {
    const YEAR: u32 = crate::YEAR;
    const DAY: u32 = 14;

    fn new(input: String) -> Self {
        Self {
            p1: 0,
            p2: 0,
            input,
            rock: HashSet::new(),
            p1s: HashSet::new(),
            p2s: HashSet::new(),
        }
    }

    fn do_p1(&mut self) {
        for line in self.input.lines() {
            for (prev, next) in line.split(" -> ").zip(line.split(" -> ").skip(1)) {
                let ((prev_l, prev_r), (next_l, next_r)) =
                    (prev.split_once(',').unwrap(), next.split_once(',').unwrap());
                let (prev_l, prev_r, next_l, next_r) = (
                    prev_l.parse::<i32>().unwrap(),
                    prev_r.parse::<i32>().unwrap(),
                    next_l.parse::<i32>().unwrap(),
                    next_r.parse::<i32>().unwrap(),
                );
                for x in prev_l.min(next_l)..=prev_l.max(next_l) {
                    for y in prev_r.min(next_r)..=prev_r.max(next_r) {
                        self.rock.insert((x, y));
                    }
                }
            }
        }
        let lim = *self.rock.iter().map(|(_, a)| a).max().unwrap();
        self.p1s = self.rock.clone();

        'outer: loop {
            let mut grain = (500i32, 0i32);
            while let Some(new_g) = self.move_grain(grain) {
                grain = new_g;
                if grain.1 > lim {
                    break 'outer;
                }
            }
            self.p1 += 1;
            self.p1s.insert(grain);
        }
    }

    fn do_p2(&mut self) {
        self.p2s = self.rock.clone();
        let lim = *self.rock.iter().map(|(_, a)| a).max().unwrap() + 1;
        loop {
            let mut grain = (500i32, 0i32);
            while let Some(new_g) = self.move_grain_p2(grain) {
                grain = new_g;
                if grain.1 >= lim {
                    break;
                }
            }
            self.p2 += 1;
            if grain == (500, 0) {
                break;
            }
            self.p2s.insert(grain);
        }
    }

    fn p1_result(&self) -> String {
        format!("{}", self.p1)
    }

    fn p2_result(&self) -> String {
        format!("{}", self.p2)
    }
}
