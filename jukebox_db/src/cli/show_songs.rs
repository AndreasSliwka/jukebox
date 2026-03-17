use jukebox_db::queries::*;
use jukebox_db::*;

fn main() {
    let connection = &mut establish_single_connection();
    let results = all_songs(connection);

    println!("Displaying {} posts", results.len());
    for simplified_song in results {
        println!("Song #{}:", simplified_song.id);
        println!("  Title: {}", simplified_song.title);
        println!(
            "  Artist: {}",
            simplified_song.artist.unwrap_or("(n/a)".to_string())
        );
    }
}
