use anyhow::Result;
use clap::Parser;

mod helpers;
use dotenv::dotenv;
pub use helpers::*;

mod problems;

mod args;
pub use args::*;

pub const YEAR: u32 = 2022;

fn main() -> Result<()> {
    dotenv()?;
    env_logger::init();
    let args = Args::parse();
    let probs: Vec<u32> = if args.problems.is_empty() {
        (1..=25).collect()
    } else {
        args.problems.iter().map(|v| match v {
            ProblemSelect::Day(v) => {
                *v..v+1
            }
            ProblemSelect::DayRange(v, y) => {
                *v..*y
            }
        }).flatten().collect()
    };
    log::debug!("Problems: {:?}", probs);
    let cache = AocCache::new()?;
    let d = problems::do_problems(&cache, probs)?;
    println!("All problems complete in {:?}", d);
    Ok(())
}
