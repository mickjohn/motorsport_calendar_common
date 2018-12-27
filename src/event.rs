use chrono::prelude::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Event {
    pub id: i32,
    pub sport: String,
    pub title: String,
    pub track: String,
    pub country: String,
    pub location: String,
    pub sessions: Vec<Session>,
}

impl Event {
    pub fn get_start_date(&self) -> Option<NaiveDate> {
        self.sessions.first().and_then(|s| s.time.map(|t|t.date()))
    }

    pub fn get_end_date(&self) -> Option<NaiveDate> {
        self.sessions.last().and_then(|s| s.time.map(|t|t.date()))
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Session {
   pub id: i32,
   pub event_id: i32,
   pub name: String,
   pub time: Option<NaiveDateTime>,
}
