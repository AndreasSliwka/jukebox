use chord_down;
use std::env;
use std::fs;
use std::process::exit;

fn get_file_from_command_line() -> (String, bool) {
    let args: Vec<String> = env::args().collect();
    println!("args.len() = {}", args.len());
    if args.len() < 2 {
        println!("usage: {} path/to/song_file.md", args[0]);
        exit(255)
    }
    let mut dunp_song_tree = false;
    if args.len() == 3 {
        dunp_song_tree = true;
    }
    let filename = args[1].clone();
    let metadata = fs::metadata(filename.clone()).unwrap();
    fs::exists(filename.clone())
        .expect(format!("Directory {} does not exist", filename.clone()).as_str());
    assert!(metadata.is_file(), "is not a file");
    (filename, dunp_song_tree)
}

fn main() {
    let (filename, dunp_song_tree) = get_file_from_command_line();
    let maybe_content = fs::read_to_string(filename.clone());
    if let Ok(content) = maybe_content {
        let song = chord_down::Song::parse(&content, true);
        if dunp_song_tree {
            println!("song = {:#?}", song)
        }
    } else {
        println!("failed to load the file.")
    }
}
