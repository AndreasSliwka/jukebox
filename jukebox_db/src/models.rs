use crate::schema::{gigs, songs, songs_in_setlist};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::songs)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Song {
    pub id: i32,
    pub title: String,
    pub artist: String,
    pub tags: String,
    pub markdown: String,
    pub serialized_chord_pro: String,
}

#[derive(Debug, Clone, Insertable, HasQuery)]
#[diesel(table_name = crate::schema::gigs)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Gig {
    pub id: i32,
    pub name: String,
    pub location: String,
    pub date_start: String,
    pub date_end: String,
    pub admin_secret: String,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, HasQuery)]
#[diesel(table_name = crate::schema::songs)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SimplifiedSong {
    pub id: i32,
    pub title: String,
    pub artist: String,
    pub tags: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongWithLink {
    pub id: i32,
    pub title: String,
    pub artist: String,
    pub link: String,
}
fn link_to_song(song_id: i32) -> String {
    format!("/songs/{}", song_id)
}

impl SongWithLink {
    pub fn from(simplified: &SimplifiedSong) -> Self {
        Self {
            id: simplified.id,
            title: simplified.title.clone(),
            artist: simplified.artist.clone(),
            link: link_to_song(simplified.id),
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = songs)]
pub struct NewSong<'a> {
    pub title: &'a str,
    pub artist: &'a str,
    pub tags: String,
    pub markdown: &'a str,
    pub serialized_chord_pro: &'a str,
}

#[derive(Insertable)]
#[diesel(table_name = gigs)]
pub struct NewGig {
    pub name: String,
    pub location: String,
    pub date_start: String,
    pub date_end: String,
    pub admin_secret: String,
    pub notes: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = songs_in_setlist)]
pub struct NewSongInSetlist<'a> {
    pub song_id: i32,
    pub setlist_id: i32,
    pub played_at: Option<&'a str>,
}
