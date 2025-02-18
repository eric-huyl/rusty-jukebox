use std::fs::File;
use std::io::BufReader;
use std::str;
use rodio::{Decoder, OutputStream, Sink};


pub struct Player {
    sink: Sink,
    _stream: OutputStream,
    stream_handle: rodio::OutputStreamHandle,
}

impl Player {
    pub fn new() -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        Player {
            sink,
            _stream,
            stream_handle,
        }
    }
    
    pub fn play_next(&self, file_path: &str) -> String {
        //println!("Playing music: {}", file_path);
        let file = BufReader::new(File::open(file_path).unwrap());
        let source = Decoder::new(file).unwrap();
        self.sink.append(source);
        //self.sink.sleep_until_end();
        "Playing music done".to_string()
    }


    pub fn is_paused(&self) -> bool {
        self.sink.is_paused()
    }
    
    pub fn pause(&self) {
        self.sink.pause();
    }

    pub fn resume(&self) {
        self.sink.play();
    }
}