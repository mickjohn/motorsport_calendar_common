use chrono::{DateTime, Local};
use chrono::prelude::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Event {
    pub sport: String,
    pub round: u64,
    pub country: String,
    pub location: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub sessions: Vec<Session>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Session {
   pub name: String,
   pub date: DateTime<Utc>,
   pub time: Option<DateTime<Local>>,
}
