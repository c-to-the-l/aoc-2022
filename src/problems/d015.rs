// const P1_Y: i64 = 10;
const P1_Y: i64 = 2_000_000;

// const P2_HI: i64 = 20;
const P2_HI: i64 = 4000000;

pub struct Day {
    p1: i64,
    p2: i64,
    input: String,
    beacon_manhattans: Vec<((i64, i64), i64)>
}

fn manhattan(sx: i64, sy: i64, bx: i64, by: i64) -> i64 {
    (sx.abs_diff(bx) + sy.abs_diff(by)) as i64
}

fn overlaps(a: &(i64, i64), b: &(i64, i64)) -> bool {
    (a.0..=a.1).contains(&b.0) | (a.0..=a.1).contains(&b.1) | (b.0..=b.1).contains(&a.0) |  (b.0..=b.1).contains(&a.1)
}

fn any_overlapping(v: &Vec<(i64, i64)>) -> bool {
    for (n, i) in v.iter().enumerate() {
        for j in v.iter().skip(n+1) {
            if overlaps(i, j) {
                return true
            }
        }
    }
    false
}

fn reduce_single(a: &(i64, i64), b: &(i64, i64)) -> (i64, i64) {
    (a.0.min(b.0), a.1.max(b.1))
}

fn reduce_overlapping(mut v: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    let mut rv = Vec::new();
    'outer:
    while let Some(a) = v.pop() {
        for i in 0..v.len() {
            if overlaps(&a, &v[i]) {
                rv.push(reduce_single(&a, &v.swap_remove(i)));
                continue 'outer;
            }
        }
        rv.push(a);
    }
    rv  
}


impl crate::Problem for Day {
    const YEAR: u32 = crate::YEAR;
    const DAY: u32 = 15;

    fn new(input: String) -> Self {
        Self { p1: 0, p2: 0, input, beacon_manhattans:Vec::new() }
    }

    fn do_p1(&mut self) {
        let mut line_parse = 
        nom::multi::separated_list1::<_, _, _, nom::error::Error<_>,_, _>(
            nom::character::complete::char('\n'), 
            nom::sequence::tuple((
                nom::bytes::complete::tag("Sensor at x="),
                nom::character::complete::i64,
                nom::bytes::complete::tag(", y="),
                nom::character::complete::i64,
                nom::bytes::complete::tag(": closest beacon is at x="),
                nom::character::complete::i64,
                nom::bytes::complete::tag(", y="),
                nom::character::complete::i64,
            ))
        );
        let (_, out) = line_parse(self.input.as_str()).unwrap();
        self.beacon_manhattans = out.iter().map(|(_, sx, _, sy, _, bx, _, by)| 
            (
                (*sx, *sy),
                manhattan(*sx, *sy, *bx, *by)
            )
        ).collect();

        let mut ranges: Vec<(i64, i64)> = self.beacon_manhattans.iter().filter_map(
            |((sx, sy), man)| {
                if ((sy-man)..=(sy+man)).contains(&P1_Y) {
                    let man_remain = man.abs_diff(sy.abs_diff(P1_Y) as i64) as i64;
                    Some((sx-man_remain,sx+man_remain))
                } else {

                    None
                }
            }
        ).collect();
        while any_overlapping(&ranges) {
            ranges = reduce_overlapping(ranges);
        }

        self.p1 = ranges.iter().map(|(a, b)| a.abs_diff(*b) as i64).sum();
        
    }   

    fn do_p2(&mut self) {
        let mut final_range = Vec::new();
        let mut final_y = 0;
        self.beacon_manhattans.sort_by(|((ax, _), _),((bx, _), _)|ax.cmp(bx));
        for y in 0..=P2_HI {
            let mut ranges: Vec<(i64, i64)> = self.beacon_manhattans
                .iter()
                .filter(|((_, sy), man)| ((sy-man)..=(sy+man)).contains(&y))
                .map(|((sx, sy), man)| {
                    let man_remain = man.abs_diff(sy.abs_diff(y) as i64) as i64;
                    (sx-man_remain, sx+man_remain)
                })
                .filter_map(|(a, b)|
                    if b < 0 || a > P2_HI {
                        None
                    } else {
                        Some((a.max(0), b.min(P2_HI)))
                    }
                ).collect();
            while any_overlapping(&ranges) {
                ranges = reduce_overlapping(ranges);
            }
            if ranges.len() > 1 && ranges.iter().map(|(a, b)| a.abs_diff(*b) as i64).sum::<i64>() < P2_HI-1 {
                final_range = ranges;
                final_y = y;
                break;
            }
        }
        for ((la,lb), (ua,ub)) in final_range.iter().zip(final_range.iter().skip(1)) {
            if la.abs_diff(*ub) == 2 {
                self.p2 = (la.min(ub) + 1) * 4000000 + final_y;
                break;
            }
            if lb.abs_diff(*ua) == 2 {
                self.p2 = (lb.min(ua) + 1) * 4000000 + final_y;
                break;
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
