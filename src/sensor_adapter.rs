use crate::measurement::Measurement;
use chrono::{Utc};

pub fn measure() -> Measurement {
    Measurement {
        temperature: 20.0,
        humidity: 50.5,
        timestamp: Utc::now()
    }
}