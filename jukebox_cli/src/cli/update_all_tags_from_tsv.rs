use chord_down;
use dotenvy::dotenv;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, BufRead};
use std::process::exit;

#[derive(Debug)]
struct Tags {
    pub names: Vec<String>,
}

#[derive(Debug, Clone)]
struct TaggedSong {
    line: u16,
    pub title: String,
    pub artist: String,
    pub tags: Vec<String>,
}

fn get_file_from_command_line() -> String {
    let args: Vec<String> = env::args().collect();
    println!("args.len() = {}", args.len());
    if args.len() < 2 {
        println!("usage: {} path_to__songs_and_tags.tsv", args[0]);
        exit(255)
    }

    let filename = args[1].clone();
    let metadata = fs::metadata(filename.clone()).unwrap();
    fs::exists(filename.clone())
        .expect(format!("File {} does not exist", filename.clone()).as_str());
    assert!(metadata.is_file(), "is not a file");
    filename
}

fn get_songs_directory_from_environment() -> String {
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

fn marked_tags(marks: &[String], tags: &Vec<String>) -> Vec<String> {
    let mut marked_tags: Vec<String> = vec![];
    for (index, mark) in marks.iter().enumerate() {
        if index < tags.len() {
            match mark.to_lowercase().trim() {
                "" => (),
                "x" => marked_tags.push(tags[index].clone()),
                _ => println!("unknown mark: '{}'", mark),
            }
        }
    }
    marked_tags
}

fn next_line(lines: &mut io::Lines<io::BufReader<fs::File>>) -> Option<Vec<String>> {
    let Some(Ok(line)) = lines.next() else {
        return None;
    };
    let entries: Vec<String> = line.split("\t").map(String::from).collect();
    Some(entries)
}

fn load_tags_and_songs_from_tsv(filename: String) -> (Tags, Vec<TaggedSong>) {
    let mut tag_names: Vec<String> = vec![];
    let mut songs: Vec<TaggedSong> = vec![];

    let file = fs::File::open(filename).unwrap();
    let mut lines = io::BufReader::new(file).lines();
    let header = next_line(&mut lines).unwrap();
    let header: Vec<String> = header.iter().map(|s| s.to_lowercase()).collect();
    if header[0] != "title" || header[1] != "artist" {
        println!("Header line of is not 'Title\\tArtist\\..' ");
        exit(255)
    }
    tag_names = header[2..].into();
    let mut line_number: u16 = 1;
    while let Some(line) = next_line(&mut lines) {
        line_number += 1;
        let song = TaggedSong {
            line: line_number,
            title: line[0].to_lowercase(),
            artist: line[1].to_lowercase(),
            tags: marked_tags(&line[2..], &tag_names),
        };
        songs.push(song);
    }
    (Tags { names: tag_names }, songs)
}

fn find_tagged_song(
    tagged_songs: &Vec<TaggedSong>,
    song_in_file: &chord_down::Song,
) -> Option<TaggedSong> {
    let artist = song_in_file.artist.to_lowercase();
    let title = song_in_file.title.to_lowercase();
    for tagged_song in tagged_songs {
        if tagged_song.artist == artist && tagged_song.title == title {
            return Some(tagged_song.clone());
        }
    }
    None
}
fn main() {
    let filename = get_file_from_command_line();

    let (tags, tsv_songs) = load_tags_and_songs_from_tsv(filename);
    println!("Found these tags: \"{}\"", tags.names.join(" "));

    let songs_dir = get_songs_directory_from_environment();
    let song_in_git = get_songs_files_from_directory(songs_dir);
    let mut tag_usage: HashMap<String, u16> = HashMap::new();
    let mut found_tagged_songs: Vec<u16> = vec![];
    println!("Songs missing in TSV:");
    for git_song in song_in_git {
        let maybe_git_content = fs::read_to_string(git_song.clone());
        if let Ok(git_content) = maybe_git_content {
            let mut parsed_git_content = chord_down::Song::parse(&git_content, false);
            if let Some(tsv_song) = find_tagged_song(&tsv_songs, &parsed_git_content) {
                for tag in &tsv_song.tags {
                    *tag_usage.entry(tag.clone()).or_default() += 1;
                }
                parsed_git_content.tags = tsv_song.tags;
                found_tagged_songs.push(tsv_song.line);
                parsed_git_content
                    .write_to_file(git_song)
                    .expect("Could not dump song");
            } else {
                println!(
                    "{}\t{}",
                    parsed_git_content.title.to_lowercase(),
                    parsed_git_content.artist.to_lowercase()
                );
            };
        }
    }
    println!("Tag usage:");
    let mut tag_usage_by_count: Vec<(&String, &u16)> = tag_usage.iter().collect();
    tag_usage_by_count.sort_by(|a, b| b.1.cmp(a.1));
    for (tag, count) in tag_usage_by_count {
        println!("  {}: {}", tag, count);
    }
    println!("Songs missing in GIT:");
    for tagged_song in tsv_songs {
        if !found_tagged_songs.contains(&tagged_song.line) {
            println!("{} -- {}", tagged_song.title, tagged_song.artist);
        }
    }
}
