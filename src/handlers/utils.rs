use anyhow::Result;
use chrono::NaiveTime;

pub fn parse_start_time(start_time_str: &str) -> Result<NaiveTime> {
    NaiveTime::parse_from_str(start_time_str, "%H:%M:%S").map_err(|e| e.into())
}
