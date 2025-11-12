use chrono::{DateTime, Days, Utc};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Screentime {
        #[arg(value_parser = DateQuery::parse_date_query)]
        date_query: DateQuery,
    },
    Clear,
}

#[derive(Clone)]
enum DateQuery {
    Exact(DateTime<Utc>),
    Range {
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    },
}

impl DateQuery {
    fn parse_date_query(s: &str) -> Result<DateQuery, String> {
        if s.to_lowercase() == "today" {
            return Ok(DateQuery::Exact(Utc::now()));
        }

        if s.to_lowercase() == "yesterday" {
            return Ok(DateQuery::Exact(Utc::now() - Days::new(1)));
        }

        todo!()
    }
}

fn main() {
    let cli = Cli::parse();
}
