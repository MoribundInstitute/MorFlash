// src/gui/app/review_ops.rs

use chrono::{DateTime, Utc};

use crate::gui::app::MorflashGui;

impl MorflashGui {
    /// Pick the next card to show.
    ///
    /// Simple behavior:
    /// - If there are no cards, nothing is shown.
    /// - If we've already reviewed all cards, we're done (no current card).
    /// - Otherwise, show cards[reviewed_count].
    pub(crate) fn pick_next_card(&mut self, _now: DateTime<Utc>) {
        // Whenever we move to a new card (or to "no card"), clear
        // the per-card visual/feedback state so highlights don't linger.
        self.feedback.clear();
        self.last_answer_correct = None;
        self.correct_term = None;
        self.wrong_term = None;
        self.pending_advance = false;

        // No cards at all.
        if self.cards.is_empty() {
            self.current_card_id = None;
            self.options.clear();
            return;
        }

        // We've already gone through every card once.
        if self.reviewed_count >= self.total_cards {
            // Signal "no more cards" so the Study screen can transition
            // to the Completion screen.
            self.current_card_id = None;
            self.options.clear();
            return;
        }

        // Use reviewed_count as the index into cards.
        let idx = self.reviewed_count;
        if let Some(card) = self.cards.get(idx) {
            self.current_card_id = Some(card.id);
        } else {
            // Safety fallback: shouldn't happen, but don't crash.
            self.current_card_id = None;
        }

        // Rebuild multiple-choice options for the current card.
        self.rebuild_answer_options();
    }

    /// Build multiple-choice options for the current card.
    ///
    /// Right now:
    /// - Always includes the current card.
    /// - Adds up to 3 other cards as simple distractors.
    fn rebuild_answer_options(&mut self) {
        self.options.clear();

        let Some(current_id) = self.current_card_id else {
            return;
        };

        // Include the current card first.
        if let Some(current) = self.cards.iter().find(|c| c.id == current_id) {
            self.options.push(current.clone());
        }

        // Add up to 3 other distinct cards as distractors.
        for card in self.cards.iter() {
            if self.options.len() >= 4 {
                break;
            }
            if card.id != current_id {
                self.options.push(card.clone());
            }
        }
    }

    /// Handle the user clicking an answer.
    ///
    /// - Updates feedback / correctness flags.
    /// - Advances `reviewed_count`.
    /// - Schedules auto-advance (handled in `handle_auto_advance`).
    pub(crate) fn handle_answer(&mut self, term: &str) {
        let now = Utc::now();

        let Some(current_id) = self.current_card_id else {
            return;
        };
        let Some(current) = self.cards.iter().find(|c| c.id == current_id) else {
            return;
        };

        let was_correct = term == current.term;

        self.last_answer_correct = Some(was_correct);
        self.correct_term = Some(current.term.clone());
        self.wrong_term = if was_correct {
            None
        } else {
            Some(term.to_string())
        };

        // Simple text feedback.
        self.feedback.clear();
        if was_correct {
            self.feedback.push_str("Correct!");
        } else {
            self.feedback.push_str(&format!(
                "Wrong — the correct answer was '{}'.",
                current.term
            ));
        }

        // Move progress forward: this is what drives which card
        // `pick_next_card` will show next.
        if self.reviewed_count < self.total_cards {
            self.reviewed_count += 1;
        }

        // Tell the app to auto-advance in ~700 ms (handled in handle_auto_advance).
        self.pending_advance = true;
        self.last_answer_time = Some(now);
    }
}
