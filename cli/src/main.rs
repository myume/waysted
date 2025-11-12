use chrono::{DateTime, Days, NaiveDate, NaiveTime, Utc};
use clap::{Parser, Subcommand};
use regex::Regex;
use waysted_core::database::Database;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Query Screentime data
    Screentime {
        #[arg(value_parser = DateRange::parse_date_query)]
        /// The range of dates to retrieve screentime from: one of `today`, `yesterday`, `YYYY-MM-DD` or `YYYY-MM-DD to YYYY-MM-DD`
        date_range: DateRange,
    },

    /// Clear collected screentime from database
    Clear,
}

#[derive(Debug, Clone)]
struct DateRange {
    start: DateTime<Utc>,
    end: DateTime<Utc>,
}

impl DateRange {
    fn parse_date_query(s: &str) -> Result<DateRange, String> {
        let range = match s.to_lowercase().as_str() {
            "today" => {
                let date = Utc::now();
                Ok(DateRange {
                    start: date,
                    end: date,
                })
            }
            "yesterday" => {
                let yesterday = Utc::now() - Days::new(1);
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
                    let date = DateRange::parse_ymd_to_datetime(s);
                    Ok(DateRange {
                        start: date,
                        end: date,
                    })
                } else if date_range_re.is_match(s) {
                    let dates = date_range_re.captures(s).unwrap();
                    let start = DateRange::parse_ymd_to_datetime(&dates["from"]);
                    let end = DateRange::parse_ymd_to_datetime(&dates["to"]);
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

    fn parse_ymd_to_datetime(ymd: &str) -> DateTime<Utc> {
        NaiveDate::parse_from_str(ymd, "%Y-%m-%d")
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_utc()
    }
}

fn main() {
    let cli = Cli::parse();

    let db = Database::new().unwrap();
    match cli.command {
        Commands::Screentime { date_range } => {
            let screentime = db
                .get_screentime_in_range(date_range.start, date_range.end)
                .unwrap();

            if screentime.is_empty() {
                println!(
                    "No screentime found from {} to {}",
                    date_range.start, date_range.end
                );
            } else {
                println!("Screentime from {} to {}", date_range.start, date_range.end);
            }
            for app in screentime {
                println!("{} ({}%): {}ms", app.app_name, app.percentage, app.duration);
            }
        }
        Commands::Clear => todo!(),
    }
}
