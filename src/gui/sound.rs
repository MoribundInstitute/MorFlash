// src/gui/sound.rs

use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::{collections::HashMap, io::Cursor, path::Path};

pub type SoundId = String;

pub struct SoundManager {
    _stream: OutputStream,
    handle: OutputStreamHandle,
    sounds: HashMap<SoundId, Vec<u8>>,
    volume: f32,
    enabled: bool,
}

impl SoundManager {
    pub fn new() -> Option<Self> {
        let Ok((stream, handle)) = OutputStream::try_default() else {
            eprintln!("MorFlash: audio unavailable");
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

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.5);
    }

    pub fn load_sound<S: Into<SoundId>, P: AsRef<Path>>(&mut self, id: S, path: P) {
        match std::fs::read(path.as_ref()) {
            Ok(bytes) => {
                self.sounds.insert(id.into(), bytes);
            }
            Err(err) => {
                eprintln!("MorFlash: failed to load sound {:?} ({err})", path.as_ref());
            }
        }
    }

    /// Load the core sounds used by the app:
    /// - "correct"
    /// - "wrong"
    /// - optional "finish" (completion)
    /// - "ui_select" (menu/UI selection)
    pub fn load_core_sounds<P1, P2, P3, P4>(
        &mut self,
        correct_path: P1,
        incorrect_path: P2,
        complete_path: Option<P3>,
        ui_select_path: P4,
    ) where
        P1: AsRef<Path>,
        P2: AsRef<Path>,
        P3: AsRef<Path>,
        P4: AsRef<Path>,
    {
        // Clear any previously loaded sounds so we reload from scratch
        self.clear();

        // Main quiz SFX
        self.load_sound("correct", correct_path);
        self.load_sound("wrong", incorrect_path);

        if let Some(p) = complete_path {
            // You currently use the id "finish" here; keep that for compatibility.
            self.load_sound("finish", p);
        }

        // NEW: UI select sound for menu navigation, etc.
        self.load_sound("ui_select", ui_select_path);
    }

    pub fn play(&self, id: &str) {
        if !self.enabled {
            return;
        }

        let Some(bytes) = self.sounds.get(id) else {
            eprintln!("MorFlash: unknown sound id '{id}'");
            return;
        };

        let cursor = Cursor::new(bytes.clone());
        let Ok(source) = Decoder::new(cursor) else {
            eprintln!("MorFlash: decode error for '{id}'");
            return;
        };

        let Ok(sink) = Sink::try_new(&self.handle) else {
            eprintln!("MorFlash: sink create error for '{id}'");
            return;
        };

        sink.set_volume(self.volume);
        sink.append(source);
        sink.detach();
    }

    pub fn clear(&mut self) {
        self.sounds.clear();
    }
}
