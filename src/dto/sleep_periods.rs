use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SleepPeriodsData {
    data: Vec<SleepPeriodData>
}

#[derive(Debug, Deserialize)]
struct SleepPeriodData {
    low_battery_alert: bool,
    bedtime_start: String,
    day: String
}

