use anyhow::{bail, Result};
use chrono::{DateTime, Local};
use clap::Parser;
use colored::Colorize;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Time as timestamp in utc
    time: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let timestamp = args
        .time
        .chars()
        .filter(|&c| c.is_ascii_digit())
        .collect::<String>()
        .parse::<i64>();
    let time = match timestamp {
        Ok(time) => time,
        Err(err) => bail!("{}, timestamp should fit in 64-bit number", err),
    };
    let datetime = match DateTime::from_timestamp(time, 0) {
        Some(d) => d,
        None => bail!(
            "couldn't convert {} to timestamp, since it's an out-of-range number of seconds",
            time
        ),
    };
    println!(
        "{:20} {}\n{:20} {}\n{:20} {}\n{:20} {}",
        "Read:".cyan(),
        args.time.blue(),
        "As timestamp:".cyan(),
        time.to_string().blue(),
        "In UTC:".cyan(),
        datetime.to_string().blue(),
        "In Local Timezone:".cyan(),
        datetime.with_timezone(&Local).to_string().blue()
    );
    Ok(())
}
