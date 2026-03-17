use crate::models::{NewSong, SimplifiedSong, Song};
use diesel::prelude::*;

pub fn all_songs(connection: &mut SqliteConnection) -> Vec<SimplifiedSong> {
    SimplifiedSong::query()
        .order_by(crate::schema::songs::title.asc())
        .load(connection)
        .expect("Error loading songs")
}

pub fn song_by_id(connection: &mut SqliteConnection, song_id: i32) -> Option<Song> {
    use crate::schema::songs::dsl::*;
    let maybe_song = songs.find(song_id).first::<Song>(connection);
    if let Ok(song) = maybe_song {
        println!("  found song #{}: {}", song.id, song.title);
        Some(song)
    } else {
        println!("   could not find song #{}", song_id);
        None
    }
}

pub fn create_song(
    conn: &mut SqliteConnection,
    title: &str,
    artist: Option<&str>,
    lyrics_as_chordpro: Option<&str>,
) -> Song {
    use crate::schema::songs;
    let new_song = NewSong {
        title,
        artist,
        lyrics_as_chordpro,
    };

    diesel::insert_into(songs::table)
        .values(&new_song)
        .returning(Song::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}
