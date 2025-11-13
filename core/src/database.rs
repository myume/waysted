use std::{
    collections::HashMap,
    env, fs,
    os::unix::fs::MetadataExt,
    path::{Path, PathBuf},
    time::Duration,
};

use chrono::{DateTime, Utc};
use log::info;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

use crate::compositor::WindowInfo;

pub struct Database {
    connection: Connection,

    db_path: Box<Path>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppScreentime {
    pub id: i32,
    pub app_name: String,

    /// duration in ms
    pub duration: u128,
    pub percentage: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScreenTimeInstance {
    pub id: i32,
    pub title: String,
    pub app_name: String,

    /// duration in ms
    pub duration: u128,
    pub start_timestamp: i64,
    pub end_timestamp: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppGroup {
    pub app_name: String,
    pub duration: u128,
    pub instances: Vec<ScreenTimeInstance>,
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
            "CREATE TABLE IF NOT EXISTS screentime (
                id INTEGER PRIMARY KEY,
                title TEXT NOT NULL,
                app_name TEXT NOT NULL,
                duration INTEGER NOT NULL,
                start_timestamp NOT NULL,
                end_timestamp NOT NULL
            )",
            (),
        )?;

        Ok(Database {
            connection,
            db_path: db_file.into(),
        })
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
                "INSERT INTO screentime (title, app_name, duration, start_timestamp, end_timestamp) VALUES (?1, ?2, ?3, ?4, ?5)",
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

    pub fn get_screentime_in_range(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<AppScreentime>, rusqlite::Error> {
        let mut stmt = self.connection.prepare(
            "SELECT id, app_name, SUM(duration) AS duration,
             CAST(ROUND(CAST(SUM(duration) AS REAL) / CAST((SELECT SUM(duration) FROM screentime 
             WHERE ?1 <= start_timestamp AND start_timestamp <= ?2) AS real) * 100.0) AS INTEGER) FROM screentime 
             WHERE ?1 <= start_timestamp AND start_timestamp <= ?2
             GROUP BY app_name
             ORDER BY duration DESC",
        )?;

        stmt.query_map([start.timestamp_millis(), end.timestamp_millis()], |row| {
            Ok(AppScreentime {
                id: row.get(0)?,
                app_name: row.get(1)?,
                duration: row.get::<usize, i64>(2)? as u128,
                percentage: row.get(3)?,
            })
        })?
        .collect()
    }

    pub fn get_logs(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<ScreenTimeInstance>, rusqlite::Error> {
        let mut stmt = self.connection.prepare(
            "SELECT * FROM screentime
             WHERE ?1 <= start_timestamp AND start_timestamp <= ?2
             ORDER BY start_timestamp",
        )?;

        stmt.query_map([start.timestamp_millis(), end.timestamp_millis()], |row| {
            Ok(ScreenTimeInstance {
                id: row.get(0)?,
                title: row.get(1)?,
                app_name: row.get(2)?,
                duration: row.get::<usize, i64>(3)? as u128,
                start_timestamp: row.get(4)?,
                end_timestamp: row.get(5)?,
            })
        })?
        .collect()
    }

    pub fn get_title_breakdown(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<AppGroup>, rusqlite::Error> {
        let logs = self.get_logs(start, end)?;
        let mut app_groups = HashMap::new();
        for log in logs {
            if !app_groups.contains_key(&log.app_name) {
                app_groups.insert(log.app_name.clone(), vec![log]);
            } else {
                let instances = app_groups.get_mut(&log.app_name).unwrap();
                instances.push(log);
            }
        }

        app_groups
            .iter_mut()
            .for_each(|(_, instances)| instances.sort_by(|a, b| b.duration.cmp(&a.duration)));

        Ok(app_groups
            .into_iter()
            .map(|(app_name, instances)| AppGroup {
                app_name,
                duration: instances.iter().map(|x| x.duration).sum(),
                instances,
            })
            .collect())
    }

    /// Clear all screentime between [`start`] and [`end`]
    /// if [`start`] is None, then this function clears all screentime before [`end`]
    /// if [`end`] is None, then this function clears all screentime after [`start`]
    /// if both are None, then clear all screetime.
    pub fn clear_screentime_in_range(
        &self,
        start: Option<DateTime<Utc>>,
        end: Option<DateTime<Utc>>,
    ) -> Result<usize, rusqlite::Error> {
        match (start, end) {
            (None, None) => self.connection.execute("DELETE FROM screentime", ()),
            (None, Some(end)) => self.connection.execute(
                "DELETE FROM screentime WHERE start_timestamp <= ?1",
                (end.timestamp_millis(),),
            ),
            (Some(start), None) => self.connection.execute(
                "DELETE FROM screentime WHERE ?1 <= start_timestamp",
                (start.timestamp_millis(),),
            ),
            (Some(start), Some(end)) => self.connection.execute(
                "DELETE FROM screentime 
             WHERE ?1 <= start_timestamp AND start_timestamp <= ?2",
                (start.timestamp_millis(), end.timestamp_millis()),
            ),
        }
    }

    pub fn get_path(&self) -> PathBuf {
        self.db_path.to_path_buf()
    }

    pub fn get_size(&self) -> u64 {
        fs::metadata(&self.db_path).unwrap().size()
    }
}
