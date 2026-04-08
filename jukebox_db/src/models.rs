use crate::schema::{gigs, songs, songs_in_gigs, tags, tags_on_songs};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::songs)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Song {
    pub id: i32,
    pub title: String,
    pub artist: String,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongWithLinkAndTags {
    pub id: i32,
    pub title: String,
    pub artist: String,
    pub link: String,
    pub tag_signs: Vec<String>,
    pub played_at: String,
}
fn link_to_song(song_id: i32) -> String {
    format!("/songs/{}", song_id)
}

impl SongWithLinkAndTags {
    pub fn from(
        simplified: &SimplifiedSong,
        songs_played: &HashMap<i32, Option<String>>,
        tags_by_song: &HashMap<i32, Vec<i32>>,
        all_tags: &HashMap<i32, (String, String, bool)>,
    ) -> Self {
        let played_at = match songs_played.get(&simplified.id).clone() {
            None => String::from(""),
            Some(option_ref) => match option_ref {
                None => String::from(""),
                Some(string_ref) => string_ref.as_str().to_string(),
            },
        };
        let tag_signs = tags_by_song
            .get(&simplified.id)
            .unwrap_or(&vec![])
            .iter()
            .map(|tag_id| all_tags.get(tag_id).unwrap().1.clone())
            .collect();
        Self {
            id: simplified.id,
            title: simplified.title.clone(),
            artist: simplified.artist.clone(),
            link: link_to_song(simplified.id),
            tag_signs,
            played_at,
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = songs)]
pub struct NewSong<'a> {
    pub title: &'a str,
    pub artist: &'a str,
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

#[derive(Insertable, Selectable, Queryable, Debug)]
#[diesel(belongs_to(Gig))]
#[diesel(belongs_to(SimplifiedSong))]
#[diesel(table_name = songs_in_gigs)]
#[diesel(primary_key(song_id, gig_id))]
pub struct SongInGig {
    pub song_id: i32,
    pub gig_id: i32,
    pub played_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, HasQuery)]
#[diesel(table_name = tags)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Tag {
    pub id: i32,
    pub name: String,
    pub unicode: String,
    pub private: i32,
}

#[derive(Insertable)]
#[diesel(table_name = tags)]
pub struct NewTag<'a> {
    pub name: &'a str,
    pub unicode: &'a str,
    pub private: i32,
}

#[derive(Insertable, HasQuery)]
#[diesel(table_name = tags_on_songs)]
pub struct TagOnSong {
    pub song_id: i32,
    pub tag_id: i32,
}
