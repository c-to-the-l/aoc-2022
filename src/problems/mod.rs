mod d001;
mod d002;
mod d003;
mod d004;
mod d005;
mod d006;
mod d007;
mod d008;
mod d009;
mod d010;
mod d011;
mod d012;
mod d013;
mod d014;
mod d015;
mod d016;
mod d017;
mod d018;
mod d019;
mod d020;
mod d021;
mod d022;
mod d023;
mod d024;
mod d025;

use crate::{run_problem, AocCache, Result};
use anyhow::bail;
use std::time::Duration;

fn do_day(c: &AocCache, day: u32) -> Result<Duration> {
    match day {
        1 => run_problem::<d001::Day>(c),
        2 => run_problem::<d002::Day>(c),
        3 => run_problem::<d003::Day>(c),
        4 => run_problem::<d004::Day>(c),
        5 => run_problem::<d005::Day>(c),
        6 => run_problem::<d006::Day>(c),
        7 => run_problem::<d007::Day>(c),
        8 => run_problem::<d008::Day>(c),
        9 => run_problem::<d009::Day>(c),
        10 => run_problem::<d010::Day>(c),
        11 => run_problem::<d011::Day>(c),
        12 => run_problem::<d012::Day>(c),
        13 => run_problem::<d013::Day>(c),
        14 => run_problem::<d014::Day>(c),
        15 => run_problem::<d015::Day>(c),
        16 => run_problem::<d016::Day>(c),
        17 => run_problem::<d017::Day>(c),
        18 => run_problem::<d018::Day>(c),
        19 => run_problem::<d019::Day>(c),
        20 => run_problem::<d020::Day>(c),
        21 => run_problem::<d021::Day>(c),
        22 => run_problem::<d022::Day>(c),
        23 => run_problem::<d023::Day>(c),
        24 => run_problem::<d024::Day>(c),
        25 => run_problem::<d025::Day>(c),
        _ => bail!("Invalid day."),
    }
}

pub fn do_problems(c: &AocCache, problems: Vec<u32>) -> Result<Duration> {
    problems
        .iter()
        .try_fold(Duration::ZERO, |dur, day| Ok(dur + do_day(c, *day)?))
}
