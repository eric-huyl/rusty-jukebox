
use std::path::Path;
use id3::Tag;
use std::fs::File;

use crate::app::{Record};



//  fn read_mp3_metadata(file_path: &str) -> Record {
//      // 打开 MP3 文件
//      let path = Path::new(file_path);
//      let file = File::open(path).unwrap();
//  
//      // 解析 ID3 标签
//      let tag = Tag::read_from(file).expect("Failed to read tag");
//  
//      // 获取歌曲名称（title）
//      if let Some(title) = tag.title() {
//          println!("Song Title: {}", title);
//      } else {
//          println!("Song Title: Unknown");
//      }
//  
//      // 获取歌手（artist）
//      if let Some(artist) = tag.artist() {
//          println!("Artist: {}", artist);
//      } else {
//          println!("Artist: Unknown");
//      }
//  
//      // 获取专辑名称（album）
//      if let Some(album) = tag.album() {
//          println!("Album: {}", album);
//      } else {
//          println!("Album: Unknown");
//      }
//  
//      // 获取时长
//      if let Some(duration) = tag.duration() {
//          println!("Duration: {:} seconds", duration.to_string());
//      } else {
//          println!("Duration: Unknown");
//      }
//  
//      Record {
//          title: tag.title().unwrap_or("Unknown"),
//          file_path,
//          duration: tag.duration().unwrap(),
//          artist: tag.artist().unwrap_or("Unknown"),
//          album: tag.album().unwrap_or("Unknown"),
//      }
//  }


pub fn read_metadata(file_path: &str) -> Result<Record, Box<dyn std::error::Error>> {
    let title: &str = file_path;
    let artist: &str = "unknown";
    let album: &str = "unknown";
    let duration: u32 = 42;
    Ok(Record {
        title,
        file_path,
        duration,
        artist,
        album,
    })
//    let extension = Path::new(path).extension().and_then(|s| s.to_str());
//        // 根据文件扩展名调用相应的函数
//        match extension {
//            Some("wav") => read_wav_metadata(path)?,
//            Some("mp3") => {let record = read_mp3_metadata(path);},
//            _ => println!("Unsupported file format"),
//        }
}