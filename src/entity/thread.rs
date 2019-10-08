use super::res::Res;
use chrono::{DateTime, Utc};

pub struct Thread {
    pub id: uint64,
    pub title: String,
    pub reses: Vec<Res>,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}
