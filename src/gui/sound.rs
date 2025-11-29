// src/gui/sound.rs

use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::{
    collections::HashMap,
    io::Cursor,
    path::{Path, PathBuf},
};

/// Identifier used to look up a sound effect.
pub type SoundId = String;

/// Central audio manager for MorFlash.
///
/// - Keeps a single audio output stream alive for the whole app.
/// - Loads short sound effects into memory.
/// - Plays them **non-blockingly** (UI does not freeze).
///
/// Typical usage:
/// ```ignore
/// // in MorflashGui:
/// let mut sound = SoundManager::new();
/// if let Some(ref mut sm) = sound {
///     sm.load_core_sounds(
///         "assets/sfx/Correct-Tone.wav",
///         "assets/sfx/Incorrect-Tone.wav",
///         Some("assets/sfx/Celebration-Noise.wav"),
///     );
/// }
///
/// // later, in your study logic:
/// if let Some(ref sm) = sound {
///     sm.play("correct");
/// }
/// ```
pub struct SoundManager {
    // Keep the stream alive as long as the manager exists.
    _stream: OutputStream,
    handle: OutputStreamHandle,
    sounds: HashMap<SoundId, Vec<u8>>,
    volume: f32,
    enabled: bool,
}

impl SoundManager {
    /// Try to create a new SoundManager.
    ///
    /// Returns `Some(Self)` on success, or `None` if audio output
    /// cannot be opened. This lets the rest of the app continue
    /// running with sounds disabled.
    pub fn new() -> Option<Self> {
        let Ok((stream, handle)) = OutputStream::try_default() else {
            eprintln!("MorFlash: failed to open audio output; sounds will be disabled.");
            return None;
        };

        Some(Self {
            _stream: stream,
            handle,
            sounds: HashMap::new(),
            volume: 1.0,
            enabled: true,
        })
    }

    /// Globally enable or disable all sounds.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Set master volume for all sounds.
    ///
    /// Values are clamped between 0.0 and 1.5.
    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.5);
    }

    /// Load a sound effect from disk and store it in memory.
    ///
    /// `id` is how you will refer to this sound later (e.g. "correct").
    pub fn load_sound<S: Into<SoundId>>(&mut self, id: S, path: PathBuf) {
        let id = id.into();

        match std::fs::read(&path) {
            Ok(bytes) => {
                self.sounds.insert(id, bytes);
            }
            Err(err) => {
                eprintln!("MorFlash: failed to load sound '{:?}' ({})", path, err);
            }
        }
    }

    /// Convenience helper to load the three core MorFlash sounds.
    ///
    /// - `correct_path`   → sound id `"correct"`
    /// - `incorrect_path` → sound id `"wrong"`
    /// - `complete_path`  → if `Some`, sound id `"finish"`
    ///
    /// This clears any previously loaded sounds so you can treat it as
    /// "switch sound pack" safely.
    pub fn load_core_sounds<P1, P2, P3>(
        &mut self,
        correct_path: P1,
        incorrect_path: P2,
        complete_path: Option<P3>,
    ) where
        P1: AsRef<Path>,
        P2: AsRef<Path>,
        P3: AsRef<Path>,
    {
        // Avoid mixing old and new packs.
        self.clear();

        self.load_sound("correct", correct_path.as_ref().to_path_buf());
        self.load_sound("wrong", incorrect_path.as_ref().to_path_buf());

        if let Some(p) = complete_path {
            // celebration sound is stored under id "finish"
            self.load_sound("finish", p.as_ref().to_path_buf());
        }
    }

    /// Play a previously loaded sound by its `id`.
    ///
    /// This is **non-blocking**: it returns immediately, and the sound
    /// continues playing in the background.
    pub fn play(&self, id: &str) {
        if !self.enabled {
            return;
        }

        let Some(bytes) = self.sounds.get(id) else {
            eprintln!("MorFlash: tried to play unknown sound id '{id}'");
            return;
        };

        // Clone the bytes for this playback; they are small SFX, so this is fine.
        let cursor = Cursor::new(bytes.clone());

        let Ok(source) = Decoder::new(cursor) else {
            eprintln!("MorFlash: failed to decode sound '{id}'");
            return;
        };

        let Ok(sink) = Sink::try_new(&self.handle) else {
            eprintln!("MorFlash: failed to create audio sink for '{id}'");
            return;
        };

        sink.set_volume(self.volume);
        sink.append(source);

        // Detach so playback continues even after this method returns.
        sink.detach();
    }

    /// Remove all loaded sounds (useful if switching sound packs).
    pub fn clear(&mut self) {
        self.sounds.clear();
    }
}
