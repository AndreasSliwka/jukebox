use crate::schema::{gigs, songs, songs_in_gigs, tags, tags_on_songs};
use diesel::prelude::*;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::LazyLock;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::songs)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Song {
    pub id: i32,
    pub title: String,
    pub artist: String,
    pub handle: String,
    pub serialized_chord_pro: String,
}

#[derive(Debug, Clone, Insertable, Serialize, AsChangeset, Deserialize, HasQuery)]
#[diesel(table_name = crate::schema::gigs)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Gig {
    pub id: i32,
    pub name: String,
    pub location: String,
    pub date_start: String,
    pub date_end: String,
    pub admin_secret: String,
    pub notes: String,
    pub default_gig: i32,
    pub show_private: i32,
}

#[derive(Debug, Serialize)]
pub struct GigWithPlayedSongs {
    pub id: i32,
    pub name: String,
    pub location: String,
    pub date_start: String,
    pub date_end: String,
    pub admin_secret: String,
    pub notes: String,
    pub default_gig: i32,
    pub show_private: i32,
    pub played_song_ids: Vec<String>,
}

impl GigWithPlayedSongs {
    pub fn from(gig: &Gig, played_song_ids: Vec<String>) -> Self {
        Self {
            id: gig.id,
            name: gig.name.clone(),
            location: gig.location.clone(),
            date_start: gig.date_start.clone(),
            date_end: gig.date_end.clone(),
            admin_secret: gig.admin_secret.clone(),
            notes: gig.notes.clone(),
            default_gig: gig.default_gig,
            show_private: gig.show_private,
            played_song_ids,
        }
    }
}

#[derive(Debug, Clone, Serialize, HasQuery)]
#[diesel(table_name = crate::schema::songs)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SimplifiedSong {
    pub id: i32,
    pub title: String,
    pub artist: String,
    pub handle: String,
}

fn handle(title: String, artist: String) -> String {
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[^-A-Za-z0-9]").unwrap());
    let title_and_artist = format!("{}--{}", title, artist);
    let lowercase = title_and_artist.to_lowercase();
    let no_spaces = lowercase.replace(" ", "-");
    let handle = RE.replace_all(no_spaces.as_str(), "");
    handle.into()
}

impl SimplifiedSong {
    fn link(&self) -> String {
        format!("/songs/{}", handle(self.title.clone(), self.artist.clone()))
    }
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

fn _only_hours_and_minutes(played_at: String) -> String {
    // 2026-06-01T13:15:22.328178
    static HOURS_AND_MINUTES: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r".*T(..:..).*").unwrap());
    if let Some(captures) = HOURS_AND_MINUTES.captures(played_at.as_str()) {
        return captures[1].to_string();
    }
    played_at
}

impl SongWithLinkAndTags {
    pub fn from(
        simplified: &SimplifiedSong,
        songs_played: &HashMap<i32, String>,
        tags_by_song: &HashMap<i32, Vec<i32>>,
        all_tags: &HashMap<i32, (String, String, bool, bool)>,
    ) -> Self {
        let played_at: &str = match songs_played.get(&simplified.id).clone() {
            None => "",
            Some(played_at) => played_at.as_str(),
        };
        let tag_signs = tags_by_song
            .get(&simplified.id)
            .unwrap_or(&vec![])
            .iter()
            .filter(|tag_id| all_tags.get(tag_id).unwrap().3 == false)
            .map(|tag_id| all_tags.get(tag_id).unwrap().1.clone())
            .collect();
        Self {
            id: simplified.id,
            title: simplified.title.clone(),
            artist: simplified.artist.clone(),
            link: simplified.link(),
            tag_signs,
            played_at: String::from(played_at),
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = songs)]
pub struct NewSong {
    pub title: String,
    pub artist: String,
    pub handle: String,
    pub serialized_chord_pro: String,
}

impl NewSong {
    pub fn new(title: &str, artist: &str, chords: &str) -> NewSong {
        NewSong {
            title: title.to_string(),
            artist: artist.to_string(),
            handle: handle(title.to_string(), artist.to_string()),
            serialized_chord_pro: chords.to_string(),
        }
    }
}

#[derive(Insertable, Debug, Deserialize, Serialize)]
#[diesel(table_name = gigs)]
pub struct NewGig {
    pub name: String,
    pub location: String,
    pub date_start: String,
    pub date_end: String,
    pub admin_secret: String,
    pub notes: String,
    pub default_gig: i32,
    pub show_private: i32,
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
    pub hidden_tag: i32,
}

#[derive(Insertable)]
#[diesel(table_name = tags)]
pub struct NewTag<'a> {
    pub name: &'a str,
    pub unicode: &'a str,
    pub private: i32,
    pub hidden_tag: i32,
}

#[derive(Insertable, HasQuery)]
#[diesel(table_name = tags_on_songs)]
pub struct TagOnSong {
    pub song_id: i32,
    pub tag_id: i32,
}
