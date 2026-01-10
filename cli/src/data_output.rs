use chrono::{DateTime, Local};
use waysted_core::database::{AppGroup, AppScreentime, ScreenTimeInstance};

use crate::utils::format_millis;

pub trait DataOutput {
    fn to_string(&self, json: bool) -> String;
    fn size(&self) -> usize;
}

impl DataOutput for Vec<AppScreentime> {
    fn to_string(&self, json: bool) -> String {
        if json {
            serde_json::to_string_pretty(self).unwrap()
        } else {
            let mut s = String::new();
            for app in self {
                s.push_str(&format!(
                    "{} ({}%): {}\n",
                    app.app_name,
                    app.percentage,
                    format_millis(app.duration)
                ));
            }
            s
        }
    }

    fn size(&self) -> usize {
        self.len()
    }
}

impl DataOutput for Vec<AppGroup> {
    fn to_string(&self, json: bool) -> String {
        if json {
            serde_json::to_string_pretty(self).unwrap()
        } else {
            let mut s = String::new();
            for app in self {
                s.push_str(&format!(
                    "{} ({}):\n",
                    app.app_name,
                    format_millis(app.duration)
                ));
                for title in &app.instances {
                    s.push_str(&format!(
                        "    -> \"{}\" ({})\n",
                        title.title.trim(),
                        format_millis(title.duration)
                    ));
                }
            }
            s
        }
    }

    fn size(&self) -> usize {
        self.len()
    }
}

impl DataOutput for Vec<ScreenTimeInstance> {
    fn to_string(&self, json: bool) -> String {
        if json {
            serde_json::to_string_pretty(self).unwrap()
        } else {
            let mut s = String::new();
            for log in self {
                s.push_str(&format!(
                    "[{} - {}] {:<15} >> {} ({})\n",
                    DateTime::from_timestamp_millis(log.start_timestamp)
                        .unwrap()
                        .with_timezone(&Local)
                        .format("%Y-%m-%d %H:%M:%S%.f"),
                    DateTime::from_timestamp_millis(log.end_timestamp)
                        .unwrap()
                        .with_timezone(&Local)
                        .format("%Y-%m-%d %H:%M:%S%.f"),
                    log.app_name,
                    log.title,
                    format_millis(log.duration)
                ));
            }
            s
        }
    }

    fn size(&self) -> usize {
        self.len()
    }
}
