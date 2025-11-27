use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deck {
    pub name: String,
    pub description: Option<String>,
    pub cards: Vec<Card>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    pub id: u64,
    pub term: String,
    pub definition: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewState {
    pub card_id: u64,
    pub interval_days: f64,
    pub ease_factor: f64,
    pub repetitions: u32,
    pub next_review: DateTime<Utc>,
}

impl ReviewState {
    pub fn new(card_id: u64, now: DateTime<Utc>) -> Self {
        Self {
            card_id,
            interval_days: 0.0,
            ease_factor: 2.5,
            repetitions: 0,
            next_review: now,
        }
    }
}

impl Deck {
    pub fn from_json_file<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let data = fs::read_to_string(path)?;
        let deck: Deck = serde_json::from_str(&data)?;
        Ok(deck)
    }

    pub fn to_json_file<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<()> {
        let data = serde_json::to_string_pretty(self)?;
        fs::write(path, data)?;
        Ok(())
    }
}
