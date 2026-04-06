use crate::services;
use crate::templates::SongsIndexTemplate;
use crate::types::AppState;
use actix_web::http::header::ContentType;

use actix_web::{HttpRequest, HttpResponse, Responder, error, get, web};
use askama::Template;
use jukebox_db::{self, SongListOrder, models::SongWithLinkAndTags};
use querystring;
fn song_list_order(query: &str) -> SongListOrder {
    for (key, value) in querystring::querify(query) {
        if key == "sort" {
            return match value {
                "title_desc" => SongListOrder::TitleDesc,
                "artist_asc" => SongListOrder::ArtistAsc,
                "artist_desc" => SongListOrder::ArtistDesc,
                _ => SongListOrder::TitleAsc,
            };
        }
    }
    SongListOrder::TitleAsc
}

#[get("/songs")]
pub async fn service(
    request: HttpRequest,
    app_state: web::Data<AppState>,
) -> actix_web::Result<impl Responder> {
    if let Some(redirect) = services::session::start_session_unless_present(&request) {
        return Ok(redirect);
    }
    let Some(gig_id) = services::session::gig_id_from_session(&request) else {
        return Ok(services::session::redirect_to_start_session());
    };

    let song_list_order = song_list_order(request.query_string());
    let connection_pool = app_state.pool.clone();
    let private_tag_ids = app_state.private_tag_ids.clone();
    let (songs_without_links, songs_played, tags_by_song) = web::block(move || {
        let mut connection = connection_pool.get().expect("could not get connection");
        (
            jukebox_db::all_songs(&mut connection, song_list_order, private_tag_ids, None),
            jukebox_db::songs_played_in_gig(gig_id, &mut connection),
            jukebox_db::tags_by_song(&mut connection),
        )
    })
    .await
    .map_err(error::ErrorInternalServerError)?;
    let songs_with_links: Vec<SongWithLinkAndTags> = songs_without_links
        .iter()
        .map(|song| {
            SongWithLinkAndTags::from(song, &songs_played, &tags_by_song, &app_state.tags_by_id)
        })
        .collect();

    let template = SongsIndexTemplate {
        songs: songs_with_links,
        song_list_order,
        is_admin: services::session::is_admin(&request),
        all_tags_by_name: String::from("{}"),
    };

    let html = template.render().unwrap();
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}
