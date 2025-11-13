use std::io;

use chrono::{DateTime, Days, Local, NaiveDate, NaiveTime};
use clap::{Parser, Subcommand};
use regex::Regex;
use waysted_core::database::Database;

use crate::utils::{format_bytes, format_millis};

mod utils;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Query screentime data
    Screentime {
        #[arg(value_parser = DateRange::parse_date_query)]
        /// The range of dates to retrieve screentime from: one of `today`, `yesterday`, `YYYY-MM-DD` or `YYYY-MM-DD to YYYY-MM-DD`
        date_range: DateRange,

        /// Output as JSON
        #[arg(short, long)]
        json: bool,

        /// Breakdown screentime by window titles
        #[arg(short, long, group = "Mode")]
        titles: bool,

        /// Return raw screentime logs
        #[arg(short, long, group = "Mode")]
        logs: bool,
    },

    /// Clear collected screentime from database
    Clear {
        #[arg(short, long, value_parser = DateRange::parse_ymd_to_datetime)]
        start: Option<DateTime<Local>>,

        #[arg(short, long, value_parser = DateRange::parse_ymd_to_datetime)]
        end: Option<DateTime<Local>>,
    },

    /// Get database metadata
    Db {
        #[command(subcommand)]
        command: DbMetadataCommands,
    },
}

#[derive(Subcommand, Debug)]
enum DbMetadataCommands {
    /// Get the path to the screentime db
    Path,

    /// Get the total size of the db in bytes
    Size,
}

#[derive(Debug, Clone)]
struct DateRange {
    start: DateTime<Local>,
    end: DateTime<Local>,
}

impl DateRange {
    fn parse_date_query(s: &str) -> Result<DateRange, String> {
        let range = match s.to_lowercase().as_str() {
            "today" => {
                let date = Local::now();
                Ok(DateRange {
                    start: date,
                    end: date,
                })
            }
            "yesterday" => {
                let yesterday = Local::now() - Days::new(1);
                Ok(DateRange {
                    start: yesterday,
                    end: yesterday,
                })
            }
            s => {
                let date_re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
                let date_range_re =
                    Regex::new(r"^(?<from>\d{4}-\d{2}-\d{2}) to (?<to>\d{4}-\d{2}-\d{2})$")
                        .unwrap();

                if date_re.is_match(s) {
                    let date = DateRange::parse_ymd_to_datetime(s)?;
                    Ok(DateRange {
                        start: date,
                        end: date,
                    })
                } else if date_range_re.is_match(s) {
                    let dates = date_range_re.captures(s).unwrap();
                    let start = DateRange::parse_ymd_to_datetime(&dates["from"])?;
                    let end = DateRange::parse_ymd_to_datetime(&dates["to"])?;
                    Ok(DateRange { start, end })
                } else {
                    Err("\ndate_range must be in the form of `today`, `yesterday`, `YYYY-MM-DD` or `YYYY-MM-DD to YYYY-MM-DD`".to_string())
                }
            }
        };

        let day_start = NaiveTime::MIN;
        let day_end = NaiveTime::from_hms_opt(23, 59, 59).unwrap();
        range.map(|range| DateRange {
            start: range.start.with_time(day_start).unwrap(),
            end: range.end.with_time(day_end).unwrap(),
        })
    }

    fn parse_ymd_to_datetime(ymd: &str) -> Result<DateTime<Local>, String> {
        Ok(NaiveDate::parse_from_str(ymd, "%Y-%m-%d")
            .map_err(|e| e.to_string())?
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_local_timezone(Local)
            .earliest()
            .unwrap())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let db = Database::new()?;
    match cli.command {
        Commands::Screentime {
            date_range,
            json,
            titles,
            logs,
        } => {
            let (size, output) = if titles {
                let data =
                    db.get_title_breakdown(date_range.start.to_utc(), date_range.end.to_utc())?;

                let output = if json {
                    serde_json::to_string_pretty(&data)?
                } else {
                    let mut s = String::new();
                    for app in &data {
                        s.push_str(&format!(
                            "{} ({}):\n",
                            app.app_name,
                            format_millis(app.duration)
                        ));
                        for title in &app.instances {
                            s.push_str(&format!(
                                "\t\"{}\": {}\n",
                                title.title,
                                format_millis(title.duration)
                            ));
                        }
                    }
                    s
                };
                (data.len(), output)
            } else if logs {
                let data = db.get_logs(date_range.start.to_utc(), date_range.end.to_utc())?;
                (data.len(), serde_json::to_string_pretty(&data).unwrap())
            } else {
                let data =
                    db.get_screentime_in_range(date_range.start.to_utc(), date_range.end.to_utc())?;
                let output = if json {
                    serde_json::to_string_pretty(&data)?
                } else {
                    let mut s = String::new();
                    for app in &data {
                        s.push_str(&format!(
                            "{} ({}%): {}\n",
                            app.app_name,
                            app.percentage,
                            format_millis(app.duration)
                        ));
                    }
                    s
                };
                (data.len(), output)
            };

            if size == 0 {
                let date_format = "%Y-%m-%d %H:%M:%S";
                println!(
                    "No screentime was found from {} to {}",
                    date_range.start.format(date_format),
                    date_range.end.format(date_format),
                );
            }
            println!("{}", output);
        }
        Commands::Clear { start, end } => {
            print!("Are you sure you want to ");
            match (start, end) {
                (None, None) => print!("clear all screentime"),
                (None, Some(end)) => print!("clear all screentime before {end}"),
                (Some(start), None) => print!("clear all screentime after {start}"),
                (Some(start), Some(end)) => {
                    print!("clear all screentime between {start} and {end}")
                }
            }
            println!("? (y/N)");
            let mut confirmation = String::new();
            io::stdin()
                .read_line(&mut confirmation)
                .expect("Failed to read line");

            if confirmation.trim_end() == "y" {
                let num_deleted = db.clear_screentime_in_range(
                    start.map(|date| date.to_utc()),
                    end.map(|date| date.to_utc()),
                )?;
                println!("Removed {num_deleted} screentime entries.");
            } else {
                println!("Screentime was not cleared.");
            }
        }
        Commands::Db { command } => match command {
            DbMetadataCommands::Path => println!("{}", db.get_path().display()),
            DbMetadataCommands::Size => println!("{}", format_bytes(db.get_size())),
        },
    }

    Ok(())
}
