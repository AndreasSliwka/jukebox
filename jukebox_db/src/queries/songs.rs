use crate::models::{NewSong, SimplifiedSong, Song};
use crate::schema::songs;
use diesel::prelude::*;

pub fn all_songs(connection: &mut SqliteConnection) -> Vec<SimplifiedSong> {
    SimplifiedSong::query()
        .order_by(crate::schema::songs::title.asc())
        .load(connection)
        .expect("Error loading songs")
}

pub fn create_song(conn: &mut SqliteConnection, title: &str, artist: Option<&str>) -> Song {
    use crate::schema::songs;
    let new_song = NewSong { title, artist };

    diesel::insert_into(songs::table)
        .values(&new_song)
        .returning(Song::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}
