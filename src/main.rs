use anyhow::Result;
use chrono::Utc;
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
        log::debug!("{}", num_available_problems(YEAR, Utc::now()));
        let p: Vec<u32> = (1..=num_available_problems(YEAR, Utc::now())).collect();
        if p.is_empty() {
            log::error!(
                "Looks like you've started too early mate, there's still {:.2} days to wait.",
                delta_start(YEAR, Utc::now()).num_seconds().abs() as f64 / (86400.0)
            )
        }
        p
    } else {
        args.problems
            .iter()
            .map(|v| match v {
                ProblemSelect::Day(v) => *v..v + 1,
                ProblemSelect::DayRange(v, y) => *v..*y,
            })
            .flatten()
            .collect()
    };
    log::debug!("Problems: {:?}", probs);
    let cache = AocCache::new()?;
    let d = problems::do_problems(&cache, probs)?;
    println!("All problems complete in {:?}", d);
    Ok(())
}
