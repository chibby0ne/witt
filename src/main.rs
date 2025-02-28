use chrono::{DateTime, Local};
use clap::Parser;
use colored::Colorize;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Time as timestamp in utc
    time: String,
}

fn main() {
    let args = Args::parse();
    let timestamp = args
        .time
        .chars()
        .filter(|&c| c.is_ascii_digit())
        .collect::<String>()
        .parse::<i64>();
    let time = match timestamp {
        Ok(time) => time,
        Err(err) => return eprintln!("This wasn't meant to be. Error: {}", err),
    };
    let datetime = DateTime::from_timestamp(time, 0).unwrap();
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
}
