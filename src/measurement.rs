use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct Measurement{
    pub temperature: f32,
    pub pressure: f32,
    pub timestamp: DateTime<Utc>
} 