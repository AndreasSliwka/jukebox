use crate::services;
use crate::templates::SongListIndexTemplate;
use crate::types::AppState;
use actix_session::SessionExt;
use actix_web::http::header::ContentType;
use actix_web::{HttpRequest, HttpResponse, Responder, get, web};
use askama::Template;
use jukebox_db::{self, models::SongWithLinkAndTags};
use log::debug;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn tags_by_name(
    tags_by_id: HashMap<i32, (String, String, bool, bool)>,
    show_private: bool,
) -> HashMap<String, String> {
    let mut tags: HashMap<String, String> = HashMap::new();
    for (name, sign, is_private, is_hidden_tag) in tags_by_id.into_values() {
        if !is_hidden_tag && (show_private || !is_private) {
            tags.insert(name, sign);
        }
    }
    tags
}

#[derive(Serialize, Deserialize)]
struct SongListResponse {
    songs: Vec<SongWithLinkAndTags>,
    tags: Vec<String>,
    admin: bool,
}

#[get("/songs.json")]
pub async fn service_json(
    request: HttpRequest,
    app_state: web::Data<AppState>,
) -> actix_web::Result<impl Responder> {
    let connection_pool = app_state.pool.clone();
    let is_admin = services::session::is_admin(&request);
    let _gig_id = services::session::gig_id_from_session_or_db(
        &mut request.get_session(),
        &mut connection_pool.get().unwrap(),
    );
    let cache_key = format!("json/songs/admin={}", is_admin);
    let cache = &app_state.cache;
    if let Some(cache_entry) = cache.get(&cache_key) {
        debug!("cache hit: {}", cache_key);
        let cached_value = cache_entry.value().clone();
        let response = serde_json::from_str(&cached_value).unwrap();
        return Ok(web::Json(response));
    }

    let private_tag_ids = if is_admin {
        vec![]
    } else {
        (*app_state.private_tag_ids).clone()
    };

    let (songs_without_links, songs_played, tags_by_song) = web::block(move || {
        let mut connection = connection_pool.get().expect("could not get connection");
        let current_gig = jukebox_db::current_gig_from_db_or_default(&mut connection);
        (
            jukebox_db::all_songs(&mut connection, private_tag_ids),
            jukebox_db::songs_played_in_gig(current_gig.id, &mut connection),
            jukebox_db::tags_by_song(&mut connection),
        )
    })
    .await
    .expect("things happened");
    let songs_with_links: Vec<SongWithLinkAndTags> = songs_without_links
        .iter()
        .map(|song| {
            SongWithLinkAndTags::from(song, &songs_played, &tags_by_song, &app_state.tags_by_id)
        })
        .collect();
    let tags_by_name: HashMap<String, String> =
        tags_by_name((*app_state.tags_by_id).clone(), is_admin);
    let response = SongListResponse {
        songs: songs_with_links,
        tags: tags_by_name.values().cloned().collect(),
        admin: is_admin,
    };

    let cache_entry = serde_json::to_string(&response).unwrap();
    debug!("cache entry under {}", cache_key);
    cache.insert(cache_key, cache_entry);

    Ok(web::Json(response))
}

#[get("/songs")]
pub async fn service(
    request: HttpRequest,
    app_state: web::Data<AppState>,
) -> actix_web::Result<impl Responder> {
    let connection_pool = app_state.pool.clone();
    let app_url = app_state.base_url.clone();
    let cache = &app_state.cache;
    let is_admin = services::session::is_admin(&request);
    let _gig_id = services::session::gig_id_from_session_or_db(
        &mut request.get_session(),
        &mut connection_pool.get().unwrap(),
    );
    let current_gig = web::block(move || {
        let mut connection = connection_pool.get().expect("could not get connection");

        jukebox_db::current_gig_from_db_or_default(&mut connection)
    })
    .await
    .expect("things happened");
    let page_url = crate::services::qrcode::full_url(&app_url, "songs");
    let template = SongListIndexTemplate {
        dark_background: true,
        zoom: crate::services::session::zoom_from_session(&request),
        qr_code_svg: crate::services::qrcode::qr_code_as_svg(&page_url, &cache),
        qr_code_url: page_url.to_string(),
        is_dev_mode: app_state.is_dev_mode(),
        is_admin: is_admin,
        current_gig,
    };

    let html = template.render().unwrap();
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}
