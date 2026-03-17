use askama::Template;
use jukebox_db::models::SongWithLink;

#[derive(Template)]
#[template(path = "songs_index.html")]
pub struct SongsIndexTemplate {
    pub songs: Vec<SongWithLink>,
}
