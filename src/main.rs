use anyhow::{anyhow, Result};
use chrono::{DateTime, Local};
use clap::Parser;
use colored::Colorize;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, arg_required_else_help(true))]
struct Args {
    /// Time as timestamp in utc
    times: Vec<String>,
}

fn convert_vector_inputs(times: &[String]) -> Vec<Result<i64, anyhow::Error>> {
    times
        .iter()
        .map(|x| get_timestamp_from_input(convert_string_timestamp(x)))
        .collect()
}

fn convert_string_timestamp(input: &str) -> Result<i64, std::num::ParseIntError> {
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
        Err(err) => Err(anyhow!("{}, timestamp should fit in 64-bit number", err)),
    }
}

fn convert_vector_timestamp_to_datetime(
    v: &[Result<i64, anyhow::Error>],
) -> Vec<Result<DateTime<chrono::Utc>, anyhow::Error>> {
    v.iter()
        .map(|x| match x {
            Ok(timestamp) => get_datetime_from_timestamp(*timestamp),
            Err(e) => Err(anyhow!("{}", e)),
        })
        .collect()
}

fn get_datetime_from_timestamp(timestamp: i64) -> Result<DateTime<chrono::Utc>, anyhow::Error> {
    match DateTime::from_timestamp(timestamp, 0) {
        Some(d) => Ok(d),
        None => Err(anyhow!("invalid number of seconds")),
    }
}

fn form_first_line(length: usize) -> String {
    let mut res = format!("{:20}", "Item:".cyan());
    for i in 0..length {
        res.push_str(&format!("{:30}", i.to_string().white()));
    }
    res
}

fn form_second_line(times: &[String]) -> String {
    let mut res = format!("{:20}", "Read:".cyan());
    for s in times {
        res.push_str(&format!("{:30}", s.blue()));
    }
    res
}

#[allow(dead_code)]
fn form_third_line(timestamps: &[Result<i64, anyhow::Error>]) -> String {
    let mut res = format!("{:20}", "As timestamp:".cyan());
    for s in timestamps {
        match s {
            Ok(timestamp) => res.push_str(&format!("{:30}", timestamp.to_string().blue())),
            Err(e) => res.push_str(&format!("{:30}", e.to_string().blue())),
        }
    }
    res
}

#[allow(dead_code)]
fn form_fourth_and_fifth_line(
    datetimes: &[Result<DateTime<chrono::Utc>, anyhow::Error>],
) -> String {
    let mut res = format!("{:20}", "In UTC:".cyan());
    for s in datetimes {
        match s {
            Ok(datetime) => res.push_str(&format!("{:30}", datetime.to_string().blue())),
            Err(e) => res.push_str(&format!("{:30}", e.to_string().blue())),
        }
    }
    res.push('\n');
    res.push_str(&format!("{:20}", "In Local Timezone:".cyan()));
    for s in datetimes {
        match s {
            Ok(datetime) => res.push_str(&format!(
                "{:30}",
                datetime.with_timezone(&Local).to_string().blue()
            )),
            Err(e) => res.push_str(&format!("{:30}", e.to_string().blue())),
        }
    }
    res
}

#[allow(dead_code)]
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
    let times = convert_vector_inputs(&args.times);
    let datetimes = convert_vector_timestamp_to_datetime(&times);
    println!("{}", form_first_line(datetimes.len()));
    println!("{}", form_second_line(&args.times));
    println!("{}", form_third_line(&times));
    println!("{}", form_fourth_and_fifth_line(&datetimes));
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_produce_output_string() {
        let time = "1213-123831231";
        let timestamp: i64 = 1213123831231;
        let datetime = DateTime::from_timestamp(timestamp, 0).unwrap();
        let output = produce_output_string(time, timestamp, &datetime);

        assert!(output.contains(&format!("{}", time.blue())));
        assert!(output.contains(&format!("{}", timestamp.to_string().blue())));
        assert!(output.contains(&format!(
            "{}",
            datetime.with_timezone(&Local).to_string().blue()
        )));
    }

    #[test]
    fn test_get_datetime_from_timestamp_with_valid_timestamp() {
        let timestamp = 1213123831231;
        let output = get_datetime_from_timestamp(timestamp);
        assert!(output.is_ok());
        assert_eq!(output.unwrap().timestamp(), timestamp);
    }

    #[test]
    fn test_get_datetime_from_timestamp_with_invalid_timestamp() {
        let timestamp = 12131238312310000;
        let output = get_datetime_from_timestamp(timestamp);
        assert!(output.is_err_and(|x| x.to_string()
            == format!(
                "couldn't convert {} to timestamp, since it's an out-of-range number of seconds",
                timestamp
            )));
    }

    #[test]
    fn test_get_timestamp_from_input_valid() {
        let timestamp = 1213123831231;
        let output = get_timestamp_from_input(Ok(timestamp));
        assert!(output.is_ok());
    }

    #[test]
    fn test_get_timestamp_from_input_invalid() {
        let timestamp = "121312383123100000000".parse::<i64>();
        let output = get_timestamp_from_input(timestamp);
        assert!(output.is_err_and(|x| x
            .to_string()
            .contains("timestamp should fit in 64-bit number")));
    }

    #[test]
    fn test_convert_input_valid() {
        let timestamp = "1_213_123_831_231";
        let output = convert_string_timestamp(timestamp);
        assert!(output.is_ok_and(|x| x == 1213123831231));
    }

    #[test]
    fn test_convert_input_invalid() {
        let input = "121312383123100000000";
        let output = convert_string_timestamp(input);
        assert!(output.is_err_and(|x| *x.kind() == std::num::IntErrorKind::PosOverflow));
    }
}
