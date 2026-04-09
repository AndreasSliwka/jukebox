use std::collections::HashMap;

use crate::filters;
use askama::Template;
use chord_down::{Block, Song};
use jukebox_db::models::SongWithLinkAndTags;
use jukebox_db::queries::SongListOrder;

#[derive(Template)]
#[template(path = "songs_index.html")]
pub struct SongsIndexTemplate {
    pub songs: Vec<SongWithLinkAndTags>,
    pub song_list_order: SongListOrder,
    pub is_admin: bool,
    pub show_private: bool,
    pub all_tags_by_name: HashMap<String, String>,
}

#[derive(Template)]
#[template(path = "song.html")]
pub struct SongsTemplate {
    pub song: Song,
    pub played_at: Option<String>,
    pub is_admin: bool,
    pub show_private: bool,
}

#[derive(Template)]
#[template(path = "qrcode.html")]
pub struct QrCodesTemplate {
    pub public_url_svg: String,
    pub admin_url_svg: String,
    pub is_admin: bool,
    pub show_private: bool,
}

#[derive(Template)]
#[template(path = "welcome.html")]
pub struct WelcomeTemplate {}

#[derive(Template)]
#[template(path = "no_shoes_no_shirt.html")]
pub struct NoShoesNoShirtTemplate {}
