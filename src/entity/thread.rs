use super::res::Res;
use chrono::{DateTime, Utc};

pub struct Thread {
    pub title: String,
    pub reses: Vec<Res>,
    pub updated_at: DateTime<Utc>,
}
