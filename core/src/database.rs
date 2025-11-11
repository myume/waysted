use std::{env, fs, path::PathBuf, time::Duration};

use chrono::{DateTime, Utc};
use log::info;
use rusqlite::Connection;

use crate::compositor::WindowInfo;

pub struct Database {
    connection: Connection,
}

impl Database {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let data_dir = env::var("XDG_DATA_HOME")
            .or_else(|_| env::var("HOME").map(|home| format!("{home}/.local/share")))
            .unwrap_or_default();
        let db_dir = PathBuf::from(data_dir).join("waysted");
        let db_file = db_dir.join("waysted.db");

        if !db_dir.exists() {
            info!("Waysted db not found, creating new db.");
            fs::create_dir(db_dir)?;
        }

        let connection = Connection::open(&db_file)?;

        info!("Database loaded from {}", db_file.display());

        connection.execute(
            "create table if not exists screentime (
                id integer primary key,
                title text not null,
                app_name text not null,
                duration integer not null,
                start_timestamp not null,
                end_timestamp not null
            )",
            (),
        )?;

        Ok(Database { connection })
    }

    pub fn log_focus_duration(
        &self,
        window_info: WindowInfo,
        duration: Duration,
        start_timestamp: DateTime<Utc>,
        end_timestamp: DateTime<Utc>,
    ) {
        self.connection
            .execute(
                "insert into usage (title, app_name, duration, start_timestamp, end_timestamp) values (?1, ?2, ?3, ?4, ?5)",
                (
                    &window_info.title,
                    &window_info.app_name,
                    duration.as_millis() as i64,
                    // store timestamps in epoch millis so it's easily comparable
                    start_timestamp.timestamp_millis(),
                    end_timestamp.timestamp_millis()
                ),
            )
            .unwrap();
    }
}
