use crate::models::{NewSong, SimplifiedSong, Song};
use chord_down;
use diesel::{debug_query, prelude::*};
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

pub struct SongWithGigInfo {
    pub id: i32,
    pub title: String,
    pub artist: String,
    pub tags: String,
    pub serialized_chord_pro: String,
    pub played_at_gig: Option<String>,
}

pub fn maybe_played_at_gig(
    song_id: i32,
    maybe_gig_id: Option<i32>,
    connection: &mut SqliteConnection,
) -> Option<String> {
    use crate::schema::songs_in_gigs::dsl;
    println!("maybe_played_at_gig({},{:#?}", song_id, maybe_gig_id);
    let Some(gig_id) = maybe_gig_id else {
        return None;
    };
    let query = dsl::songs_in_gigs
        .select(dsl::played_at)
        .filter(dsl::gig_id.eq(gig_id))
        .filter(dsl::song_id.eq(song_id));
    println!(
        "  query: {}",
        debug_query::<diesel::sqlite::Sqlite, _>(&query)
    );
    let result = query.first::<Option<String>>(connection);

    if let Ok(maybe_played_at) = result {
        println!("  maybe_played_at = {:#?}", maybe_played_at);
        maybe_played_at
    } else {
        None
    }
}

pub fn song_by_id_with_gig_info(
    connection: &mut SqliteConnection,
    song_id: i32,
    maybe_gig_id: Option<i32>,
) -> Option<SongWithGigInfo> {
    use crate::schema::songs::dsl::*;

    let played_at_gig = maybe_played_at_gig(song_id, maybe_gig_id, connection);
    let maybe_song = songs.find(song_id).first::<Song>(connection);
    if let Ok(song) = maybe_song {
        Some(SongWithGigInfo {
            id: song.id,
            title: song.title,
            artist: song.artist,
            tags: song.tags,
            serialized_chord_pro: song.serialized_chord_pro,
            played_at_gig,
        })
    } else {
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
