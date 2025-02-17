use std::str;
use crate::metadata::read_metadata;
use crate::player::play_music;
use std::thread;

pub struct TabState<'a> {
    pub titles: Vec<&'a str>,
    pub tab_index: usize,
}

impl<'a> TabState<'a> {
    pub fn new(titles: Vec<&'a str>) -> TabState<'a> {
        TabState {
            titles,
            tab_index: 0,
        }
    }
    pub fn next(&mut self) {
        self.tab_index = (self.tab_index + 1) % self.titles.len();
    }
    pub fn previous(&mut self) {
        if self.tab_index > 0 {
            self.tab_index -= 1;
        } else {
            self.tab_index = self.titles.len() - 1;
        }
    }
}

pub struct Record<'a> {
    pub title: &'a str,
    pub file_path: &'a str,
    pub duration: u32, // duration in seconds
    pub artist: &'a str,
    pub album: &'a str,
}

impl<'a> Record<'a> {
    pub fn new(file_path: &'a str) -> Record<'a> {
        read_metadata(file_path).unwrap()
    }
}
pub struct Playlist<'a> {
    pub records: Vec<Record<'a>>,
    pub current_index: usize,
}

impl<'a> Playlist<'a> {
    pub fn new() -> Playlist<'a> {
        Playlist {
            records: vec![Record::new("leiqiao.mp3"), Record::new("audio2.wav")],
            current_index: 0,
        }
    }

    pub fn add_record(&mut self, record: Record<'a>) {
        self.records.push(record);
    }


    pub fn next(&mut self) {
        self.current_index = (self.current_index + 1) % self.records.len();
    }

    pub fn previous(&mut self) {
        if self.current_index > 0 {
            self.current_index -= 1;
        } else {
            self.current_index = self.records.len() - 1;
        }
    }


    pub fn current(&self) -> Option<&Record<'a>> {
        self.records.get(self.current_index)
    }
}

pub struct App<'a> {
    pub title: &'a str,
    pub tabs: TabState<'a>,
    pub should_quit: bool,
    pub playlist: Playlist<'a>,
    pub selected: usize,
}

impl<'a> App<'a> {
    pub fn new(title:  &'a str)->App<'a> {
        App {
            title,
            tabs: TabState::new(vec!["Tab1", "Tab2", "Tab3"]),
            should_quit: false,
            playlist: Playlist::new(),
            selected: 0,
        }
    }

    pub fn on_left(&mut self) {
        self.tabs.previous();
    }

    pub fn on_right(&mut self) {
        self.tabs.next();
    }

    pub fn on_up(&mut self) {
        if self.playlist.current_index > 0 {
            self.playlist.current_index -= 1;
        }
    }

    pub fn on_down(&mut self) {
        if self.playlist.current_index < self.playlist.records.len() - 1 {
            self.playlist.current_index += 1;
        }
    }

    pub fn on_quit(&mut self) {
        self.should_quit = true;
    }

    pub fn play(&self) {
        if let Some(record) = self.playlist.records.get(self.playlist.current_index) {
            let file_path = record.file_path.to_string();
            thread::spawn(move || {
                play_music(&file_path);
            });
        }
    }

}

