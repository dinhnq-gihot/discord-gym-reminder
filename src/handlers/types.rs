use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Schedule {
    pub day: String,
    pub start_time: String,
    pub musculatures: Vec<String>,
}
