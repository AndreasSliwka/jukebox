use crate::models::{NewSong, SimplifiedSong, Song};
use chord_down;
use diesel::debug_query;
use diesel::prelude::*;
use diesel::sqlite::Sqlite;
use log;
use ron;
use serde::{Deserialize, Serialize};
#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum SongListOrder {
    TitleAsc,
    TitleDesc,
    ArtistAsc,
    ArtistDesc,
}

fn excepted_songs(except_tags: Vec<i32>, connection: &mut SqliteConnection) -> Vec<i32> {
    use crate::schema::tags_on_songs::dsl::*;
    let query = tags_on_songs
        .select(song_id)
        .filter(tag_id.eq_any(except_tags));
    log::debug!("Excepted Songs: {}", debug_query::<Sqlite, _>(&query));
    query.load(connection).unwrap()
}

pub fn all_songs(
    connection: &mut SqliteConnection,
    order: SongListOrder,
    except_tags: Vec<i32>,
    search: Option<String>,
) -> Vec<SimplifiedSong> {
    use crate::schema::songs::dsl::*;
    let excepted_song_ids = excepted_songs(except_tags, connection);
    let query = SimplifiedSong::query().filter(diesel::dsl::not(id.eq_any(excepted_song_ids)));
    let query = if let Some(term) = search {
        query.filter(title.like(format!("%{}%", term)))
    } else {
        query.filter(title.like(String::from("%")))
    };
    log::debug!("All Songs query: {}", debug_query::<Sqlite, _>(&query));
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
    pub serialized_chord_pro: String,
    pub played_at_gig: Option<String>,
}

pub fn maybe_played_at_gig(
    song_id: i32,
    maybe_gig_id: Option<i32>,
    connection: &mut SqliteConnection,
) -> Option<String> {
    use crate::schema::songs_in_gigs::dsl;
    let Some(gig_id) = maybe_gig_id else {
        return None;
    };
    let query = dsl::songs_in_gigs
        .select(dsl::played_at)
        .filter(dsl::gig_id.eq(gig_id))
        .filter(dsl::song_id.eq(song_id));
    let result = query.first::<Option<String>>(connection);

    if let Ok(maybe_played_at) = result {
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
            serialized_chord_pro: song.serialized_chord_pro,
            played_at_gig,
        })
    } else {
        None
    }
}

pub fn song_by_title_and_artist(
    title: &str,
    artist: &str,
    connection: &mut SqliteConnection,
) -> Option<Song> {
    use crate::schema::songs::dsl;
    let maybe_song = dsl::songs
        .filter(dsl::title.eq(title))
        .filter(dsl::artist.eq(artist))
        .first::<Song>(connection);
    match maybe_song {
        Ok(song) => Some(song),
        Err(_) => None,
    }
}

fn update_song(song_id: i32, new_song: NewSong, conn: &mut SqliteConnection) -> Option<Song> {
    use crate::schema::songs::dsl;
    let song = Song {
        id: song_id,
        title: new_song.title.to_string(),
        artist: new_song.artist.to_string(),
        serialized_chord_pro: new_song.serialized_chord_pro.to_string(),
    };
    let result = diesel::update(dsl::songs.filter(dsl::id.eq(song.id)))
        .set((
            dsl::title.eq(song.title),
            dsl::artist.eq(song.artist),
            dsl::serialized_chord_pro.eq(song.serialized_chord_pro),
        ))
        .get_result::<Song>(conn);
    match result {
        Ok(song) => Some(song),
        Err(_) => None,
    }
}

pub fn create_song(new_song: NewSong, conn: &mut SqliteConnection) -> Option<Song> {
    use crate::schema::songs;

    Some(
        diesel::insert_into(songs::table)
            .values(&new_song)
            .returning(Song::as_returning())
            .get_result(conn)
            .expect("Error saving new post"),
    )
}

pub fn update_or_create_song(
    conn: &mut SqliteConnection,
    title: &str,
    artist: &str,
    markdown: &str,
) -> Option<Song> {
    let song = chord_down::Song::parse(&(markdown.to_string()), false);
    let _tags = serde_json::to_string(&song.tags).unwrap_or(String::from("[]"));
    let chord_pro = ron::ser::to_string(&song).unwrap();
    let serialized_chord_pro = chord_pro.as_str();
    let new_song = NewSong {
        title,
        artist,
        serialized_chord_pro,
    };

    if let Some(song) = song_by_title_and_artist(title, artist, conn) {
        update_song(song.id, new_song, conn)
    } else {
        create_song(new_song, conn)
    }
}

pub fn delete_all_other_songs(known_song_ids: Vec<i32>, connection: &mut SqliteConnection) {
    use crate::schema::songs::dsl::*;
    use diesel::dsl::not;
    diesel::delete(songs)
        .filter(not(id.eq_any(known_song_ids)))
        .execute(connection)
        .unwrap();
}

pub fn delete_all_songs(conn: &mut SqliteConnection) -> () {
    use crate::schema::songs::dsl::*;

    let _ = diesel::delete(songs).execute(conn);
}
