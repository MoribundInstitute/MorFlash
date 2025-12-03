// src/srs/mod.rs
//
// SRS helpers + .mflash support.
// For now, the scheduling functions are very simple stubs so that
// the app compiles; we can refine the algorithm later.

use chrono::{DateTime, Utc};

use crate::model::ReviewState;

pub mod mflash;

/// Very simple placeholder: treat every card as "due".
pub fn is_due(_state: &ReviewState, _now: DateTime<Utc>) -> bool {
    true
}

/// Placeholder SRS update: take the old state and a rating,
/// and just return the state unchanged for now.
///
/// We make this generic over the rating type so it works whether
/// `rating` is an `i32`, `u8`, etc.
pub fn update_review_state<T>(state: ReviewState, _rating: T, _now: DateTime<Utc>) -> ReviewState
where
    T: Copy + Into<i32>,
{
    // TODO: later, actually change the state based on rating & time.
    state
}
