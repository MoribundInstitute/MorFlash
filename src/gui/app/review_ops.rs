// src/gui/app/review_ops.rs

use rand::seq::SliceRandom;
use rand::thread_rng;

use super::MorflashGui;
use crate::gui::sound::play_sound;
use crate::model::Card;
use crate::srs::{is_due, update_review_state};

impl MorflashGui {
    pub(crate) fn pick_next_card(&mut self, now: chrono::DateTime<chrono::Utc>) {
        if let Some(card) = self
            .cards
            .iter()
            .find(|c| is_due(self.states.get(&c.id).unwrap(), now))
        {
            self.current_card_id = Some(card.id);
            self.feedback.clear();
            self.last_answer_correct = None;
            self.correct_term = None;
            self.wrong_term = None;
            self.build_options(card.id);
        } else {
            self.current_card_id = None;
            self.options.clear();
            self.feedback = "No more cards due right now. 🎉".to_string();
        }
    }

    fn build_options(&mut self, correct_id: u64) {
        let mut rng = thread_rng();
        let mut opts: Vec<Card> = Vec::new();

        if let Some(c) = self.cards.iter().find(|c| c.id == correct_id) {
            opts.push(c.clone());
        }

        let mut others: Vec<Card> = self
            .cards
            .iter()
            .filter(|c| c.id != correct_id)
            .cloned()
            .collect();

        others.shuffle(&mut rng);
        for c in others.into_iter().take(3) {
            opts.push(c);
        }

        opts.shuffle(&mut rng);
        self.options = opts;
    }

    pub(crate) fn handle_answer(&mut self, chosen_term: &str) {
        if let Some(card_id) = self.current_card_id {
            let card = self.cards.iter().find(|c| c.id == card_id).unwrap();

            let now = chrono::Utc::now();
            let state = self.states.get(&card.id).unwrap().clone();

            let is_correct = chosen_term == card.term;

            // feedback + highlighting
            if is_correct {
                self.feedback = "✓ Correct!".to_string();
                self.last_answer_correct = Some(true);
                self.correct_term = Some(card.term.clone());
                self.wrong_term = None;
                play_sound("assets/sfx/Correct-Tone.wav");
            } else {
                self.feedback = format!("✗ Wrong. Correct answer: {}", card.term);
                self.last_answer_correct = Some(false);
                self.correct_term = Some(card.term.clone());
                self.wrong_term = Some(chosen_term.to_string());
                play_sound("assets/sfx/Incorrect-Tone.wav");
            }

            // SRS update
            let rating = if is_correct { 3 } else { 0 };
            let new_state = update_review_state(state, rating, now);
            self.states.insert(card.id, new_state);

            // progress
            if self.total_cards > 0 {
                self.reviewed_count = (self.reviewed_count + 1).min(self.total_cards);
            }

            // schedule auto-advance
            self.pending_advance = true;
            self.last_answer_time = Some(now);
        }
    }
}
