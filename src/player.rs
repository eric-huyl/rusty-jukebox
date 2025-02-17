use rodio::{Decoder, OutputStream, source::Source};
use std::fs::File;
use std::io::BufReader;

pub fn play_music(file_path: &str) {
    println!("Playing music: {}", file_path);
    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    // Load a sound from a file, using a path relative to Cargo.toml
    let file = File::open(file_path).unwrap();
    let source = Decoder::new(BufReader::new(file)).unwrap();

    // Play the sound directly on the device
    stream_handle.play_raw(source.convert_samples()).unwrap();

    println!("Playing music done: {}", file_path);
}