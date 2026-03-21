use askama::Template;
use chord_down::{Block, Song};
use jukebox_db::models::SongWithLink;
use jukebox_db::queries::SongListOrder;

#[derive(Template)]
#[template(path = "songs_index.html")]
pub struct SongsIndexTemplate {
    pub songs: Vec<SongWithLink>,
    pub song_list_order: SongListOrder,
    pub is_admin: bool,
}

#[derive(Template)]
#[template(path = "song.html")]
pub struct SongsTemplate {
    pub song: Song,
    pub is_admin: bool,
}

#[derive(Template)]
#[template(path = "welcome.html")]
pub struct WelcomeTemplate {}

#[derive(Template)]
#[template(path = "no_shoes_no_shirt.html")]
pub struct NoShoesNoShirtTemplate {}
