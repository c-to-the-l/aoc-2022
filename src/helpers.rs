use anyhow::Result;
use chrono::{DateTime, TimeZone, Utc};
use reqwest::{blocking::Client, cookie::Jar, Url};
use std::io::prelude::*;
use std::{
    fs::File,
    path::PathBuf,
    sync::Arc,
    time::{Duration, Instant},
};

pub struct AocCache {
    client: Client,
    cache: PathBuf,
}

impl AocCache {
    pub fn new() -> Result<Self> {
        let cache = PathBuf::from(std::env::var("AOC_CACHE").unwrap_or(".cache".to_string()));
        std::fs::create_dir_all(&cache)?;
        let aoc_session = std::env::var("AOC_SESSION")?;
        let cookie = format!("session={}, Domain=.adventofcode.com", aoc_session);
        let domain = "https://adventofcode.com".parse::<Url>()?;
        let jar = Jar::default();
        jar.add_cookie_str(&cookie, &domain);
        let client = Client::builder()
            .cookie_provider(Arc::new(jar))
            .user_agent("https://github.com/c-to-the-l/aoc-2022.git by 39165068+c-to-the-l@users.noreply.github.com")
            .build()?;
        log::debug!("AocCache Constructed");
        Ok(Self {
            client,
            cache: PathBuf::from(cache),
        })
    }

    /// Render the full cache path for a given year and day
    fn path_for(&self, year: u32, day: u32) -> PathBuf {
        self.cache.join(format!("y{}d{}.txt", year, day))
    }

    /// Load the input for problem `year`-`day` from the cache or the web
    pub fn get_input(&self, year: u32, day: u32) -> Result<String> {
        if let Ok(mut file) = File::open(self.path_for(year, day)) {
            log::debug!("Using cached input for {}-{}", year, day);
            let mut resp: String = String::new();
            file.read_to_string(&mut resp)?;
            return Ok(resp);
        }
        log::debug!("Need to fetch fresh input for {}-{}", year, day);
        let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
        let resp = self.client.get(url).send()?.error_for_status()?.text()?;
        log::debug!("Received {} bytes of input", resp.len());
        let mut file = File::create(self.path_for(year, day))?;
        log::debug!("Created cache file for {}-{}", year, day);
        file.write_all(resp.as_ref())?;
        log::debug!("Written cache for {}-{}", year, day);
        Ok(resp)
    }
}

pub fn run_problem<P: Problem>(c: &AocCache) -> Result<Duration> {
    let input = c.get_input(P::YEAR, P::DAY)?;

    let s = Instant::now();
    let mut p = P::new(input);
    let p_s = s.elapsed();

    let s = Instant::now();
    p.do_p1();
    let p1_t = s.elapsed();

    let s = Instant::now();
    p.do_p2();
    let p2_t: Duration = s.elapsed();

    println!(
        "Problem {}-{:02} in {:>8.1?} (Setup {:>8.1?}) - P1: {:<10} ({:>8.1?}) - P2: {:<10} ({:>8.1?}).",
        P::YEAR,
        P::DAY,
        p_s + p1_t + p2_t,
        p_s,
        p.p1_result(),
        p1_t,
        p.p2_result(),
        p2_t
    );
    Ok(p1_t + p2_t)
}

pub fn delta_start(year: u32, now: DateTime<Utc>) -> chrono::Duration {
    let start = chrono_tz::EST
        .with_ymd_and_hms(year as i32, 12, 1, 00, 00, 00)
        .unwrap();
    now.signed_duration_since(start)
}

pub fn num_available_problems(year: u32, now: DateTime<Utc>) -> u32 {
    (1 + delta_start(year, now).num_days().clamp(-1, 24)) as u32
}

pub trait Problem {
    const YEAR: u32;
    const DAY: u32;
    fn new(input: String) -> Self;
    fn do_p1(&mut self);
    fn do_p2(&mut self);
    fn p1_result(&self) -> String;
    fn p2_result(&self) -> String;
}

#[cfg(test)]
mod tests {
    use chrono::{Duration, TimeZone};

    use crate::{delta_start, num_available_problems};

    #[test]
    fn test_start_check() {
        let fake_now_before = chrono::Utc
            .with_ymd_and_hms(crate::YEAR as i32, 11, 30, 00, 00, 00)
            .unwrap();
        let fake_now_after = chrono::Utc
            .with_ymd_and_hms(crate::YEAR as i32, 12, 2, 00, 00, 00)
            .unwrap();
        let fake_keen_player = chrono::Utc
            .with_ymd_and_hms(crate::YEAR as i32, 12, 1, 5, 00, 01)
            .unwrap();

        assert_eq!(
            delta_start(crate::YEAR, fake_now_before),
            Duration::hours(-29)
        );
        assert_eq!(
            delta_start(crate::YEAR, fake_now_after),
            Duration::hours(19)
        );
        assert_eq!(
            delta_start(crate::YEAR, fake_keen_player),
            Duration::seconds(1)
        );

        assert_eq!(num_available_problems(crate::YEAR, fake_now_before), 0);
        assert_eq!(num_available_problems(crate::YEAR, fake_now_after), 1);
        assert_eq!(num_available_problems(crate::YEAR, fake_keen_player), 1);
    }
}
