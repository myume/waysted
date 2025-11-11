use std::{path::PathBuf, time::Duration};

use chrono::{DateTime, Utc};
use log::info;
use rusqlite::{Connection, Error};

use crate::compositor::WindowInfo;

pub struct Database {
    connection: Connection,
}

impl Database {
    pub fn new() -> Result<Self, Error> {
        let db_path = PathBuf::from("waysted.db");
        if !db_path.exists() {
            info!("Waysted db not found, creating new db.");
        }

        let connection = Connection::open(db_path)?;
        connection.execute(
            "create table if not exists usage (
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
