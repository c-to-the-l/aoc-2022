use std::cmp::Ordering;

#[derive(Debug, PartialEq, Clone)]
enum Packet {
    Integer(i64),
    SubPacket(Vec<Packet>),
}

fn atoi(b: &[u8], p: usize) -> (Packet, usize) {
    if b[p + 1].is_ascii_digit() {
        (
            Packet::Integer(((b[p] - '0' as u8) * 10 + b[p + 1] - '0' as u8) as i64),
            p + 2,
        )
    } else {
        (Packet::Integer((b[p] - '0' as u8) as i64), p + 1)
    }
}

impl Packet {
    fn from_bytes(b: &[u8], mut p: usize) -> (Self, usize) {
        match b[p] as char {
            '[' => {
                let mut sp = Vec::new();
                p += 1;
                loop {
                    match b[p] as char {
                        ']' => {
                            p += 1;
                            break;
                        }
                        ',' => p += 1,
                        _ => {
                            let (pk, new_p) = Packet::from_bytes(b, p);
                            p = new_p;
                            sp.push(pk);
                        }
                    }
                }
                (Self::SubPacket(sp), p)
            }
            x => {
                assert!(x.is_numeric());
                atoi(b, p)
            }
        }
    }

    fn correct(&self, lower: &Self) -> Ordering {
        match self {
            Packet::Integer(v) => match lower {
                Packet::Integer(y) => v.cmp(y),
                x => Packet::SubPacket(vec![Packet::Integer(*v)]).correct(x),
            },
            Packet::SubPacket(sp) => match lower {
                Packet::SubPacket(sp_l) => {
                    if let Some(val) = sp
                        .iter()
                        .zip(sp_l.iter())
                        .map(|(a, b)| a.correct(b))
                        .find(|v| v.is_ne())
                    {
                        val
                    } else {
                        sp.len().cmp(&sp_l.len())
                    }
                }
                Packet::Integer(y) => self.correct(&Packet::SubPacket(vec![Packet::Integer(*y)])),
            },
        }
    }
}

pub struct Day {
    p1: usize,
    p2: usize,
    input: String,
    packets: Vec<(Packet, Packet)>,
}

impl crate::Problem for Day {
    const YEAR: u32 = crate::YEAR;
    const DAY: u32 = 13;

    fn new(input: String) -> Self {
        Self {
            p1: 0,
            p2: 0,
            input,
            packets: Vec::new(),
        }
    }

    fn do_p1(&mut self) {
        self.packets = self
            .input
            .split("\n\n")
            .map(|pair| {
                let (upper, lower) = pair.split_once('\n').unwrap();
                (
                    Packet::from_bytes(upper.as_bytes(), 0).0,
                    Packet::from_bytes(lower.as_bytes(), 0).0,
                )
            })
            .collect();
        self.p1 = self
            .packets
            .iter()
            .enumerate()
            .map(|(n, (upper, lower))| {
                if upper.correct(lower).is_le() {
                    n + 1
                } else {
                    0
                }
            })
            .sum()
    }

    fn do_p2(&mut self) {
        let mut p2packets: Vec<Packet> = self
            .input
            .split("\n")
            .filter(|s| !s.is_empty())
            .map(|s| Packet::from_bytes(s.as_bytes(), 0).0)
            .collect();
        let six = Packet::from_bytes("[[6]]".as_bytes(), 0).0;
        let two = Packet::from_bytes("[[2]]".as_bytes(), 0).0;

        p2packets.push(six.clone());
        p2packets.push(two.clone());
        p2packets.sort_by(|a, b| a.correct(b));
        self.p2 = p2packets.binary_search_by(|v| v.correct(&six)).unwrap() + 1;
        self.p2 *= p2packets.binary_search_by(|v| v.correct(&two)).unwrap() + 1;
    }

    fn p1_result(&self) -> String {
        format!("{}", self.p1)
    }

    fn p2_result(&self) -> String {
        format!("{}", self.p2)
    }
}
