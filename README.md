# MorFlash

MorFlash is an offline flashcard application designed for effective spaced repetition learning. Written in Rust using the egui framework, it provides a focused, distraction-free environment for studying with extensive customization options.

## Features

- **Multi-format Import Support**: Import flashcards from various file formats, including tab-separated text files, with support for embedded media.
- **Flexible Study Interface**: 
  - Freely repositionable flashcards within the study window
  - Independent positioning of accompanying images or videos
  - Centered card layout with customizable margins
- **Comprehensive Customization**:
  | Feature                          | Description
  | ----------------------------| -----|
  | Background Images | Customizable tiling background images
  | Sound Effects   | Configurable sound effects for correct/incorrect answers and deck completion
  | Font Selection | Multiple font options including system fonts, Public Pixel font, and custom font files
- **Self-Contained Operation**: Completely offline with no external dependencies or network requirements.

## Screenshots

![MorFlash Study Interface](assets/screenshots/MorFlash_Screenshot.png)

The study interface features a movable flashcard that can be positioned anywhere within the study window. Accompanying media (images or videos) can also be independently repositioned as needed.

## Getting Started

1. Clone or download the repository
2. Build the project using Cargo: `cargo run`
3. Import flashcard decks from the main menu
4. Customize the application through the Options menu

## Usage

From the main menu, users can:
- Load existing deck files
- Import new flashcard content in supported formats
- Access customization options

During study sessions, flashcards can be freely moved around the screen. Media elements associated with cards can also be independently repositioned. The application automatically advances to the next card after a brief delay following each answer.

## Customization

All customization options are accessible through the Options menu, allowing users to:
- Replace the default tiling background image
- Enable or disable sound effects
- Select from available font options or load a custom font file

## Building the Project

MorFlash is a standard Rust project and can be built with:

```bash
cargo build --release
