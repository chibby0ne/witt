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

fn convert_input(input: &str) -> Result<i64, std::num::ParseIntError> {
    input
        .chars()
        .filter(|&c| c.is_ascii_digit())
        .collect::<String>()
        .parse::<i64>()
}

fn get_timestamp_from_input(
    timestamp: Result<i64, std::num::ParseIntError>,
) -> Result<i64, anyhow::Error> {
    match timestamp {
        Ok(time) => Ok(time),
        Err(err) => bail!("{}, timestamp should fit in 64-bit number", err),
    }
}

fn get_datetime_from_timestamp(timestamp: i64) -> Result<DateTime<chrono::Utc>, anyhow::Error> {
    match DateTime::from_timestamp(timestamp, 0) {
        Some(d) => Ok(d),
        None => bail!(
            "couldn't convert {} to timestamp, since it's an out-of-range number of seconds",
            timestamp
        ),
    }
}

fn produce_output_string(time: &str, timestamp: i64, datetime: &DateTime<chrono::Utc>) -> String {
    format!(
        "{:20} {}\n{:20} {}\n{:20} {}\n{:20} {}",
        "Read:".cyan(),
        time.blue(),
        "As timestamp:".cyan(),
        timestamp.to_string().blue(),
        "In UTC:".cyan(),
        datetime.to_string().blue(),
        "In Local Timezone:".cyan(),
        datetime.with_timezone(&Local).to_string().blue()
    )
}

fn main() -> Result<()> {
    let args = Args::parse();
    let timestamp = convert_input(&args.time);
    let time = get_timestamp_from_input(timestamp)?;
    let datetime = get_datetime_from_timestamp(time)?;
    println!("{}", produce_output_string(&args.time, time, &datetime));
    Ok(())
}

mod test {
    #[test]
    fn test_invalid_arg() {
        todo!();
    }
}
