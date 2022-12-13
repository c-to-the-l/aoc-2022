use std::collections::VecDeque;

pub struct Day {
    p1: usize,
    p2: usize,
    input: String,
    map: Vec<Vec<u8>>,
    distances: Vec<Vec<usize>>,
    distances2: Vec<Vec<usize>>,
    start: (usize, usize),
    end: (usize, usize),
}

impl Day {
    fn can_path(&self, from_x: usize, from_y: usize, to_x: usize, to_y: usize) -> bool {
        let height = self.map[from_y][from_x];
        self.distances[to_y][to_x] == usize::MAX && self.map[to_y][to_x] <= (height + 1)
    }

    fn can_path_p2(&self, from_x: usize, from_y: usize, to_x: usize, to_y: usize) -> bool {
        let height = self.map[from_y][from_x];
        self.distances2[to_y][to_x] == usize::MAX && self.map[to_y][to_x] >= (height - 1)
    }

    fn build_distances(&mut self) {
        let mut point_q: VecDeque<(usize, usize)> = VecDeque::new();
        point_q.push_back(self.start);
        while let Some((p_x, p_y)) = point_q.pop_front() {
            let dist = self.distances[p_y][p_x];
            if p_x > 0 && self.can_path(p_x, p_y, p_x - 1, p_y) {
                self.distances[p_y][p_x - 1] = dist + 1;
                point_q.push_back((p_x - 1, p_y))
            }
            if p_y > 0 && self.can_path(p_x, p_y, p_x, p_y - 1) {
                self.distances[p_y - 1][p_x] = dist + 1;
                point_q.push_back((p_x, p_y - 1))
            }

            if p_x < self.map[0].len() - 1 && self.can_path(p_x, p_y, p_x + 1, p_y) {
                self.distances[p_y][p_x + 1] = dist + 1;
                point_q.push_back((p_x + 1, p_y))
            }

            if p_y < self.map.len() - 1 && self.can_path(p_x, p_y, p_x, p_y + 1) {
                self.distances[p_y + 1][p_x] = dist + 1;
                point_q.push_back((p_x, p_y + 1))
            }
        }
    }

    fn get_shortest_a(&mut self) -> usize {
        let mut point_q: VecDeque<(usize, usize)> = VecDeque::new();
        point_q.push_back(self.end);
        while let Some((p_x, p_y)) = point_q.pop_front() {
            let dist = self.distances2[p_y][p_x];
            // this algorithm operates breadth-first, so we are safe to short-circuit return
            // as soon as we reach any 'a' value.
            if self.map[p_y][p_x] == 0 {
                return dist;
            }
            if p_x > 0 && self.can_path_p2(p_x, p_y, p_x - 1, p_y) {
                self.distances2[p_y][p_x - 1] = dist + 1;
                point_q.push_back((p_x - 1, p_y))
            }
            if p_y > 0 && self.can_path_p2(p_x, p_y, p_x, p_y - 1) {
                self.distances2[p_y - 1][p_x] = dist + 1;
                point_q.push_back((p_x, p_y - 1))
            }

            if p_x < self.map[0].len() - 1 && self.can_path_p2(p_x, p_y, p_x + 1, p_y) {
                self.distances2[p_y][p_x + 1] = dist + 1;
                point_q.push_back((p_x + 1, p_y))
            }

            if p_y < self.map.len() - 1 && self.can_path_p2(p_x, p_y, p_x, p_y + 1) {
                self.distances2[p_y + 1][p_x] = dist + 1;
                point_q.push_back((p_x, p_y + 1))
            }
        }
        0
    }
}

impl crate::Problem for Day {
    const YEAR: u32 = crate::YEAR;
    const DAY: u32 = 12;

    fn new(input: String) -> Self {
        Self {
            p1: 0,
            p2: 0,
            input,
            map: Vec::new(),
            distances: Vec::new(),
            distances2: Vec::new(),
            start: (0, 0),
            end: (0, 0),
        }
    }

    fn do_p1(&mut self) {
        self.map = self
            .input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        'S' => {
                            self.start = (x, y);
                            0
                        }
                        'E' => {
                            self.end = (x, y);
                            'z' as u8 - 'a' as u8
                        }
                        x => x as u8 - 'a' as u8,
                    })
                    .collect()
            })
            .collect();
        self.distances = vec![vec![usize::MAX; self.map[0].len()]; self.map.len()];
        self.distances[self.start.1][self.start.0] = 0;
        self.build_distances();
        self.p1 = self.distances[self.end.1][self.end.0];
    }

    fn do_p2(&mut self) {
        self.distances2 = vec![vec![usize::MAX; self.map[0].len()]; self.map.len()];
        self.distances2[self.end.1][self.end.0] = 0;
        self.p2 = self.get_shortest_a();
    }

    fn p1_result(&self) -> String {
        format!("{}", self.p1)
    }

    fn p2_result(&self) -> String {
        format!("{}", self.p2)
    }
}
