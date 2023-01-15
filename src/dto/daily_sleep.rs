use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DailySleepData {
    data: Vec<DailySleepResponse>
}

#[derive(Debug, Deserialize)]
struct DailySleepResponse {
    day: String,
    score: i64
}

