#![forbid(unsafe_code)]
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::time::{SystemTime, Duration, UNIX_EPOCH};

/// Card model (immutable core)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    pub id: String,
    pub question: String,
    pub answer: String,
    #[serde(default)]
    pub category: Option<String>,
}

impl Card {
    pub fn new(question: String, answer: String, category: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            question,
            answer,
            category,
        }
    }
}

/// Progress per card: small Leitner implementation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Progress {
    pub card_id: String,
    pub box_index: u8, // 1..=5
    pub last_review_ts: u64,
    pub next_due_ts: u64,
}

impl Progress {
    pub fn new(card_id: &str) -> Self {
        let now = now_ts();
        Self {
            card_id: card_id.to_string(),
            box_index: 1,
            last_review_ts: now,
            next_due_ts: now,
        }
    }

    pub fn promote(&mut self) {
        if self.box_index < 5 { self.box_index += 1; }
        self.last_review_ts = now_ts();
        self.next_due_ts = self.last_review_ts + Self::box_interval_days(self.box_index) * 86400;
    }

    pub fn demote(&mut self) {
        self.box_index = 1;
        self.last_review_ts = now_ts();
        self.next_due_ts = self.last_review_ts; // immediate
    }

    fn box_interval_days(box_index: u8) -> u64 {
        match box_index {
            1 => 0,
            2 => 1,
            3 => 3,
            4 => 7,
            5 => 30,
            _ => 0,
        }
    }
}

pub(crate) fn now_ts() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or(Duration::from_secs(0)).as_secs()
}
