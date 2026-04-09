use std::collections::HashMap;

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

fn tags_by_name(
    tags_by_id: HashMap<i32, (String, String, bool)>,
    show_private: bool,
) -> HashMap<String, String> {
    let mut tags: HashMap<String, String> = HashMap::new();
    for (name, sign, is_private) in tags_by_id.into_values() {
        if show_private || !is_private {
            tags.insert(name, sign);
        }
    }
    tags
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
    let show_private = services::session::show_private(&request);
    let private_tag_ids = if show_private {
        vec![]
    } else {
        app_state.private_tag_ids.clone()
    };
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
    let show_private = services::session::show_private(&request);
    let tags_by_name: HashMap<String, String> =
        tags_by_name(app_state.tags_by_id.clone(), show_private);

    let template = SongsIndexTemplate {
        songs: songs_with_links,
        song_list_order,
        is_admin: services::session::is_admin(&request),
        show_private: services::session::show_private(&request),
        all_tags_by_name: tags_by_name,
        zoom: crate::services::session::zoom_from_session(&request),
    };

    let html = template.render().unwrap();
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}
