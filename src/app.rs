use std::str;
use crate::metadata::read_metadata;
use crate::player::Player;
use crate::explorer::Explorer;
use std::thread;

pub struct TabState<'a> {
    pub titles: Vec<&'a str>,
    pub tab_index: usize,
}

impl<'a> TabState<'a> {
    pub fn new() -> Self {
        TabState {
            titles: vec!["Playlist", "Explorer", "Tab3"],
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
    pub selected_index: usize,
}

impl<'a> Playlist<'a> {
    pub fn new() -> Playlist<'a> {
        Playlist {
            records: vec![Record::new("leiqiao.mp3"), Record::new("audio2.wav")],
            selected_index: 0,
        }
    }

    pub fn add_record(&mut self, record: Record<'a>) {
        self.records.push(record);
    }


    pub fn next(&mut self) {
        self.selected_index = (self.selected_index + 1) % self.records.len();
    }

    pub fn previous(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        } else {
            self.selected_index = self.records.len() - 1;
        }
    }


    pub fn current(&self) -> Option<&Record<'a>> {
        self.records.get(self.selected_index)
    }
}

pub struct App<'a> {
    pub title: &'a str,
    pub tabs: TabState<'a>,
    pub should_quit: bool,
    pub playlist: Playlist<'a>,
    pub message: String,
    pub player: Player,
    pub explorer: Explorer,
}

impl<'a> App<'a> {
    pub fn new(title:  &'a str)->App<'a> {
        App {
            title,
            tabs: TabState::new(),
            should_quit: false,
            playlist: Playlist::new(),
            message: "Welcome to Rust Music Player!".to_string(),
            player: Player::new(),
            explorer: Explorer::new(),
        }
    }

    pub fn on_left(&mut self) {
        self.tabs.previous();
    }

    pub fn on_right(&mut self) {
        self.tabs.next();
    }

    pub fn on_up(&mut self) {

        if self.tabs.tab_index==0{
            if self.playlist.selected_index > 0 {
                self.playlist.selected_index -= 1;
            }
        }
        else if self.tabs.tab_index==1{
            if self.explorer.selected_index > 0 {
                self.explorer.selected_index -= 1;
            }
        }
        
    }

    pub fn handle_char_i(&mut self) {
        // let file_path = self.explorer.entries[self.explorer.selected_index].path().to_str().unwrap();
        // if file_path.ends_with(".mp3") || file_path.ends_with(".wav") {
        //     self.insert_file_to_playlist(file_path);
        // }
        // else {
        //     self.message = "Invalid file format to play".to_string();  
        // }
        // self.insert_file_to_playlist(file_path);
    }


    fn insert_file_to_playlist(&mut self, file_path: &'a str) {
        let record = Record::new(file_path);
        self.playlist.add_record(record);
    }

    pub fn on_down(&mut self) {
        

        if self.tabs.tab_index==0{
            if self.playlist.selected_index < self.playlist.records.len() - 1 {
                self.playlist.selected_index += 1;
            }
        }
        else if self.tabs.tab_index==1{
            if self.explorer.selected_index < self.explorer.entries_len() - 1 {
                self.explorer.selected_index += 1;
            }
        }
    }

    pub fn on_quit(&mut self) {
        self.should_quit = true;
    }

    pub fn play(&mut self) {
        if let Some(record) = self.playlist.records.get(self.playlist.selected_index) {
            let file_path = record.file_path.to_string();
            self.message = self.player.play_next(&file_path);
        }
    }

    pub fn toggle(&mut self) {
        if !self.player.is_paused() {
            self.player.pause();
            self.message = "Paused".to_string();
        }
        else {
            self.player.resume();
            self.message = "Resumed".to_string();
        }
    }

}

