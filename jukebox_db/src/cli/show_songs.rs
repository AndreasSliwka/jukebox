use jukebox_db::queries::*;
use jukebox_db::*;

fn main() {
    let connection = &mut establish_single_connection();
    let results = all_songs(connection);

    println!("Displaying {} posts", results.len());
    for song in results {
        println!("Song #{}:", song.id);
        println!("  Title: {}", song.title);
        println!("  Artist: {}", song.artist.unwrap_or("(n/a)".to_string()));
    }
}
