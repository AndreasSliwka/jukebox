use crate::schema::{setlists, songs, songs_in_setlist};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::songs)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Song {
    pub id: i32,
    pub title: String,
    pub artist: Option<String>,
    pub lyrics_as_chordpro: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = songs)]
pub struct NewSong<'a> {
    pub title: &'a str,
    pub artist: Option<&'a str>,
}

#[derive(Insertable)]
#[diesel(table_name = setlists)]
pub struct NewSetlist<'a> {
    pub name: &'a str,
    pub location: &'a str,
    pub date: &'a str,
    pub notes: Option<&'a str>,
}

#[derive(Insertable)]
#[diesel(table_name = songs_in_setlist)]
pub struct NewSongInSetlist<'a> {
    pub song_id: i32,
    pub setlist_id: i32,
    pub played_at: Option<&'a str>,
}
