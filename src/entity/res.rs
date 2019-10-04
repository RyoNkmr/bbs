use chrono::{Utc, DateTime}

pub struct Res {
    pub username: String,
    pub id: String,
    pub email: String,
    pub body: String,
    pub created_at: DateTime<Utc>,
}
