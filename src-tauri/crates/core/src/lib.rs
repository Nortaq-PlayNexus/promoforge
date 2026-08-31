use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod error;
pub mod types;

pub use error::{Error, Result};
pub use types::*;

pub fn new_id() -> String {
    Uuid::new_v4().to_string()
}

pub fn now() -> DateTime<Utc> {
    Utc::now()
}
