use askama::Template;
use chord_down::{Block, LineElement, Song};
use jukebox_db::models::SongWithLink;

#[derive(Template)]
#[template(path = "songs_index.html")]
pub struct SongsIndexTemplate {
    pub songs: Vec<SongWithLink>,
}

#[derive(Template)]
#[template(path = "song.html")]
pub struct SongsTemplate {
    pub song: Song,
}
