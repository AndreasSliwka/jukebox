use dotenvy::dotenv;
use jukebox_db::*;
use regex::Regex;
use std::fs;

fn get_directory_from_environment() -> String {
    dotenv().ok();
    let dir_name = std::env::var("SONGS_DIR").expect("SONGS_DIR must be set");
    let metadata = fs::metadata(dir_name.clone()).unwrap();
    fs::exists(dir_name.clone())
        .expect(format!("Directory {} does not exist", dir_name.clone()).as_str());
    assert!(metadata.is_dir(), "is not a Directory");
    dir_name
}

fn get_songs_files_from_directory<'a>(dir_name: String) -> Vec<String> {
    let mut song_files: Vec<String> = vec![];
    for dir_entry in fs::read_dir(dir_name.clone()).unwrap() {
        let file = dir_entry.unwrap();
        let file_name = file.file_name().to_str().unwrap().to_string();
        if file_name.ends_with(".md") {
            song_files.push(format!("{}/{}", dir_name, file_name));
        }
    }
    song_files
}

fn get_title<'a>(file_name: &'a str, content: &'a String) -> &'a str {
    let title_from_content = Regex::new(r"\{title: *(.*?) *\}").unwrap();
    if let Some(content_captures) = title_from_content.captures(content) {
        content_captures.get(1).unwrap().as_str()
    } else {
        Regex::new(r".*/ *(.*?) *.md$")
            .unwrap()
            .captures(file_name)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
    }
}

fn get_artist<'a>(content: &'a String) -> &'a str {
    let artist_from_content = Regex::new(r"\{artist: *(.*?) *\}").unwrap();
    if let Some(content_captures) = artist_from_content.captures(content) {
        content_captures.get(1).unwrap().as_str()
    } else {
        ""
    }
}

fn main() {
    let mut connection = jukebox_db::establish_single_connection();
    let source_dir = get_directory_from_environment();
    let song_files = get_songs_files_from_directory(source_dir);
    if song_files.len() > 0 {
        for song_file in song_files {
            let mut song_loaded: bool = false;
            let maybe_content = fs::read_to_string(song_file.clone());
            if let Ok(content) = maybe_content {
                let title: &str = get_title(song_file.as_str(), &content);
                let artist: &str = get_artist(&content);

                if let Some(song) =
                    update_or_create_song(&mut connection, title, artist, content.as_str())
                {
                    song_loaded = true;
                    println!("Song #{}: {} - {}", song.id, song.title, song.artist);
                    if song.tags != "" && song.tags != "[]" {
                        println!("  Tags: {}", song.tags);
                    }
                }
            }
            if !song_loaded {
                println!("! Could not load song from {}", song_file)
            }
        }
    }
}
