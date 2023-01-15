use serde::Deserialize;
use chrono::{DateTime, Local};

#[derive(Debug, Deserialize)]
pub struct SleepPeriodsData {
    data: Vec<SleepPeriodData>
}

impl SleepPeriodsData {
    pub fn is_low_battery(self, today: &DateTime<Local>) -> bool{
        self
        .data
        .into_iter()
        .filter(
            |data: &SleepPeriodData|
            data.day == today.format("%Y-%m-%d").to_string()
        )
        .any(
            |data: SleepPeriodData|
            data.low_battery_alert
        )
    }
}

#[derive(Debug, Deserialize)]
struct SleepPeriodData {
    low_battery_alert: bool,
    bedtime_start: String,
    day: String
}


// !TODO:test
