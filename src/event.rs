use chrono::DateTime;
use chrono::prelude::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Event {
    pub id: i32,
    pub sport: String,
    pub round: i32,
    pub country: String,
    pub location: String,
    // pub start_date: DateTime<Utc>,
    // pub end_date: DateTime<Utc>,
    pub sessions: Vec<Session>,
}

impl Event {
    pub fn get_start_date(&self) -> Option<Date<Utc>> {
        self.sessions.first().map(|s| s.date.date())
    }

    pub fn get_end_date(&self) -> Option<Date<Utc>> {
        self.sessions.last().map(|s| s.date.date())
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Session {
   pub id: i32,
   pub event_id: i32,
   pub name: String,
   pub date: DateTime<Utc>,
   pub time: Option<DateTime<Utc>>,
}
