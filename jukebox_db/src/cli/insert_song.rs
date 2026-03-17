use jukebox_db::*;
use std::env;
use std::process::exit;

fn get_title_and_artist_from_command_line() -> (String, Option<String>) {
    let args: Vec<String> = env::args().collect();
    println!("args.len() = {}", args.len());
    if args.len() < 2 {
        println!("usage: {} 'Song title' ['Artist name']", args[0]);
        exit(255)
    }
    let title = args[1].clone();
    let mut artist: Option<String> = None;

    if args.len() > 2 {
        artist = Some(args[2].clone());
    }

    (title, artist)
}

fn main() {
    let (a_title, an_artist) = get_title_and_artist_from_command_line();
    let connection = &mut establish_single_connection();

    let song = create_song(connection, a_title.as_str(), an_artist.as_deref());
    println!("\nSaved draft {a_title} with id {}", song.id);
}
