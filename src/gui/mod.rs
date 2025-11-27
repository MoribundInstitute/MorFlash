// src/gui/mod.rs

pub mod app;
pub mod deck_list_screen;
pub mod study_screen;
pub mod sound;

// re-export so you can use `morflash_core::gui::app::MorflashGui`
// or if you prefer: `morflash_core::gui::MorflashGui`
pub use app::MorflashGui;
