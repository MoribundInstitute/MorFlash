use chrono::{DateTime, Duration, Utc};

use crate::model::ReviewState;

/// Ratings: 0=Again, 1=Hard, 2=Good, 3=Easy
pub fn update_review_state(mut state: ReviewState, rating: u8, now: DateTime<Utc>) -> ReviewState {
    // interval logic
    if rating == 0 {
        state.repetitions = 0;
        state.interval_days = 1.0;
    } else {
        state.repetitions += 1;
        if state.repetitions == 1 {
            state.interval_days = 1.0;
        } else if state.repetitions == 2 {
            state.interval_days = 6.0;
        } else {
            state.interval_days *= state.ease_factor;
        }
    }

    // ease factor adjustments
    let mut ef = state.ease_factor;
    match rating {
        0 => ef -= 0.2,
        1 => ef -= 0.15,
        2 => {}
        3 => ef += 0.15,
        _ => {}
    }

    if ef < 1.3 {
        ef = 1.3;
    }
    state.ease_factor = ef;

    // next review time
    let seconds = (state.interval_days * 24.0 * 3600.0) as i64;
    state.next_review = now + Duration::seconds(seconds);

    state
}

pub fn is_due(state: &ReviewState, now: DateTime<Utc>) -> bool {
    state.next_review <= now
}
