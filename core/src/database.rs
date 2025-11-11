use std::{path::PathBuf, time::Duration};

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
                timestamp not null default (datetime('now', 'localtime'))
            )",
            (),
        )?;

        Ok(Database { connection })
    }

    pub fn log_focus_duration(&self, window_info: WindowInfo, duration: Duration) {
        self.connection
            .execute(
                "insert into usage (title, app_name, duration) values (?1, ?2, ?3)",
                (
                    &window_info.title,
                    &window_info.app_name,
                    duration.as_millis() as i64,
                ),
            )
            .unwrap();
    }
}
