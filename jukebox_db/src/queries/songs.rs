use crate::models::{NewSong, SimplifiedSong, Song};
use chord_down;
use diesel::prelude::*;
use ron;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum SongListOrder {
    TitleAsc,
    TitleDesc,
    ArtistAsc,
    ArtistDesc,
}

pub fn all_songs(
    connection: &mut SqliteConnection,
    order: SongListOrder,
    search: Option<String>,
) -> Vec<SimplifiedSong> {
    use crate::schema::songs::dsl::*;
    let query = SimplifiedSong::query().filter(tags.not_like("%\"private\"%"));
    let query = if let Some(term) = search {
        query.filter(title.like(format!("%{}%", term)))
    } else {
        query.filter(title.like(String::from("%")))
    };
    match order {
        SongListOrder::TitleDesc => query
            .order(crate::schema::songs::title.desc())
            .load(connection),
        SongListOrder::TitleAsc => query
            .order(crate::schema::songs::title.asc())
            .load(connection),
        SongListOrder::ArtistAsc => query
            .order(crate::schema::songs::artist.asc())
            .load(connection),
        SongListOrder::ArtistDesc => query
            .order(crate::schema::songs::artist.desc())
            .load(connection),
    }
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

pub fn create_song(conn: &mut SqliteConnection, title: &str, artist: &str, markdown: &str) -> Song {
    use crate::schema::songs;

    let song = chord_down::Song::parse(&(markdown.to_string()), false);
    let tags = serde_json::to_string(&song.tags).unwrap_or(String::from("[]"));
    let chord_pro = ron::ser::to_string(&song).unwrap();
    let serialized_chord_pro = chord_pro.as_str();
    let new_song = NewSong {
        title,
        artist,
        tags,
        markdown,
        serialized_chord_pro,
    };

    diesel::insert_into(songs::table)
        .values(&new_song)
        .returning(Song::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn delete_all_songs(conn: &mut SqliteConnection) -> () {
    use crate::schema::songs::dsl::*;

    let _ = diesel::delete(songs).execute(conn);
}
