use dotenvy::dotenv;
use jukebox_db::*;

use regex::Regex;
use std::collections::HashMap;
use std::fs;

fn get_directory_from_environment() -> String {
    dotenv().ok();
    let dir_name = std::env::var("SONGS_DIR").expect("SONGS_DIR must be set");
    println!("Reading songs from {}", dir_name);
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

fn tag_ids_for_song(
    song_id: i32,
    wanted_tags: Vec<String>,
    known_tags: &HashMap<String, (i32, String)>,
) -> Vec<i32> {
    wanted_tags
        .iter()
        .map(|wanted| match known_tags.get(wanted.as_str()) {
            None => {
                println!("Song #{} requests unknown tag {}", song_id, wanted);
                None
            }
            Some((id, _unicode)) => Some(id),
        })
        .flatten()
        .map(|i_ref| *i_ref)
        .collect()
}

fn main() {
    let mut connection = jukebox_db::establish_single_connection();
    jukebox_db::queries::ensure_seed_data_for_tags(&mut connection);
    let all_tags = jukebox_db::queries::all_tags_by_name(&mut connection);
    println!("#######\nall_tags = {:?}", all_tags);
    let source_dir = get_directory_from_environment();
    let song_files = get_songs_files_from_directory(source_dir);
    if song_files.len() == 0 {
        return;
    }
    let mut known_songs: Vec<i32> = vec![];
    for song_file in song_files {
        let mut song_loaded: bool = false;
        // println!("File: {}", song_file);
        let maybe_content = fs::read_to_string(song_file.clone());
        if let Ok(content) = maybe_content {
            let title: &str = get_title(song_file.as_str(), &content);
            let artist: &str = get_artist(&content);

            if let Some(song) =
                update_or_create_song(&mut connection, title, artist, content.as_str())
            {
                song_loaded = true;
                known_songs.push(song.id);
                let chord_down_song = chord_down::Song::from_ron(song.serialized_chord_pro);
                let tag_ids = tag_ids_for_song(song.id, chord_down_song.tags, &all_tags);
                set_tags_on_song(song.id, tag_ids.clone(), &mut connection);

                /* println!(
                    "  Song #{}: {} - {}, tags= {:?}",
                    song.id, song.title, song.artist, tag_ids
                ); */
            }
        }
        if !song_loaded {
            println!("! Could not load song from {}", song_file)
        }
    }
    delete_all_other_songs(known_songs, &mut connection);
}
