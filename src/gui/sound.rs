// src/gui/sound.rs
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;

/// Play a short sound effect from a file path (blocking until it finishes).
pub fn play_sound(path: &str) {
    // Try to open an audio output stream (PulseAudio/ALSA/etc. under the hood)
    let Ok((stream, stream_handle)) = OutputStream::try_default() else {
        eprintln!("Failed to open audio output");
        return;
    };

    // Try to open the file
    let Ok(file) = File::open(path) else {
        eprintln!("Failed to open sound file: {path}");
        return;
    };

    let reader = BufReader::new(file);

    // Try to decode the audio
    let Ok(source) = Decoder::new(reader) else {
        eprintln!("Failed to decode audio file: {path}");
        return;
    };

    // Create a sink and play the sound
    let Ok(sink) = Sink::try_new(&stream_handle) else {
        eprintln!("Failed to create audio sink");
        return;
    };

    sink.append(source);

    // Important: wait until the sound is done, so it doesn’t get cut off
    sink.sleep_until_end();

    // `stream` and `sink` are dropped here, which is fine; playback is done.
}
