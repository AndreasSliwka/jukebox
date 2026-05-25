use std::collections::HashMap;

use crate::filters;
use askama::Template;
use chord_down::{Block, Song};
use jukebox_db::models::SongWithLinkAndTags;

#[derive(Template)]
#[template(path = "song_list/module.html")]
pub struct SongListIndexTemplate {
    pub songs: Vec<SongWithLinkAndTags>,
    pub dark_background: bool,
    pub all_tags_by_name: HashMap<String, String>,
    pub zoom: u16,
    pub qr_code_svg: String,
    pub qr_code_url: String,
}

#[derive(Template)]
#[template(path = "single_song/module.html")]
pub struct SongsTemplate {
    pub song: Song,
    pub song_id: i32,
    pub played_at: Option<String>,
    pub is_admin: bool,
    pub dark_background: bool,
    pub zoom: u16,
    pub qr_code_svg: String,
    pub qr_code_url: String,
}

#[derive(Template)]
#[template(path = "qrcode.html")]
pub struct QrCodesTemplate {
    pub public_url_svg: String,
    pub admin_url_svg: String,
    pub is_admin: bool,
    pub dark_background: bool,
    pub zoom: u16,
}

#[derive(Template)]
#[template(path = "welcome.html")]
pub struct WelcomeTemplate {
    pub dark_background: bool,
}

#[derive(Template)]
#[template(path = "no_shoes_no_shirt.html")]
pub struct NoShoesNoShirtTemplate {}
