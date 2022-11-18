use anyhow::{bail, Result};
use clap::Parser;
use nom::{self, sequence::tuple, character::complete, combinator::{opt, eof}, error::VerboseError};

#[derive(Debug, Clone)]
pub enum ProblemSelect {
    Day(u32),
    DayRange(u32, u32)
}

impl std::str::FromStr for ProblemSelect {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, (l, r, _)) = match tuple((
            complete::u32::<_, VerboseError<_>>,
            opt(tuple((complete::char('-'), complete::u32))),
            eof
        ))(s) {
            Ok(s) => s,
            Err(e) => bail!("{}", e)
        };
        if let Some((_, r)) = r {
            if !(1..=25).contains(&r) || !(1..=25).contains(&l) {
                bail!("Day values must be in the range 1..=25");
            }
            if l >= r {
                bail!("Right range value must be larger than left");
            }
            Ok(Self::DayRange(l, r))
        } else {
            if !(1..=25).contains(&l) {
                bail!("Day values must be in the range 1..=25");
            }
            Ok(Self::Day(l))
        }
    }
}

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about)]
pub struct Args {
    /// One or more problem numbers or hyphenated ranges of the form "x-y"
    pub problems: Vec<ProblemSelect>,
}
