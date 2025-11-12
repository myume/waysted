use chrono::{DateTime, Days, NaiveDate, NaiveTime, Utc};
use clap::{Parser, Subcommand};
use regex::Regex;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Screentime {
        #[arg(value_parser = DateQuery::parse_date_query)]
        date_query: DateQuery,
    },
    Clear,
}

#[derive(Debug, Clone)]
enum DateQuery {
    Range {
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    },
}

impl DateQuery {
    fn parse_date_query(s: &str) -> Result<DateQuery, String> {
        let day_start = NaiveTime::MIN;
        let day_end = NaiveTime::from_hms_opt(23, 59, 59).unwrap();
        if s.to_lowercase() == "today" {
            let date = Utc::now();
            return Ok(DateQuery::Range {
                start: date.with_time(day_start).unwrap(),
                end: date.with_time(day_end).unwrap(),
            });
        }

        if s.to_lowercase() == "yesterday" {
            let yesterday = Utc::now() - Days::new(1);
            return Ok(DateQuery::Range {
                start: yesterday.with_time(day_start).unwrap(),
                end: yesterday.with_time(day_end).unwrap(),
            });
        }

        let date_re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
        if date_re.is_match(s) {
            let date = DateQuery::parse_ymd_to_datetime(s);
            return Ok(DateQuery::Range {
                start: date.with_time(day_start).unwrap(),
                end: date.with_time(day_end).unwrap(),
            });
        }

        let date_range_re =
            Regex::new(r"^(?<from>\d{4}-\d{2}-\d{2}) to (?<to>\d{4}-\d{2}-\d{2})$").unwrap();
        if date_range_re.is_match(s) {
            let dates = date_range_re.captures(s).unwrap();
            let start = DateQuery::parse_ymd_to_datetime(&dates["from"])
                .with_time(day_start)
                .unwrap();
            let end = DateQuery::parse_ymd_to_datetime(&dates["to"])
                .with_time(day_end)
                .unwrap();
            return Ok(DateQuery::Range { start, end });
        }

        Err("\ndate_query must be in the form of `today`, `yesterday`, `YYYY-MM-DD` or `YYYY-MM-DD to YYYY-MM-DD`".to_string())
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

    match cli.command {
        Commands::Screentime { date_query } => {
            println!("{:?}", date_query);
        }
        Commands::Clear => todo!(),
    }
}
