use crate::services;
use crate::templates;
use crate::types::AppState;
use actix_session::SessionExt;
use actix_web::http::header::ContentType;
use actix_web::{HttpRequest, HttpResponse, Responder, get, web};
use askama::Template;
use diesel::SqliteConnection;
use jukebox_db;
use log::{debug, info};
use services::session;

const ORIGINAL_SONG_LINK: &str = "never-gonna-give-you-up--rick-astley";

fn maybe_rickroll_to_original(song: &chord_down::Song) -> Option<HttpResponse> {
    if !song.tags.contains(&"rickrolling".to_string()) {
        return None;
    }
    Some(
        HttpResponse::SeeOther()
            .append_header(("Location", ORIGINAL_SONG_LINK))
            .body("Rick will never!"),
    )
}

fn set_played_at_now(
    song_id: i32,
    gig_id: i32,
    request: &HttpRequest,
    connection: &mut SqliteConnection,
) -> bool {
    if !session::is_admin(&request) {
        return false;
    }
    if jukebox_db::is_default_gig(gig_id, connection) {
        debug!(
            "Could not set played_at for song {} in default_gig ",
            song_id
        );
        return false;
    }
    let mut song_to_be_added_to_gig = false;
    let mut add_to_setlist = false;
    for (key, value) in querystring::querify(request.query_string()) {
        debug!("  Request query {}: {}", key, value);
        if key == "add_to_setlist" {
            song_to_be_added_to_gig = true;
            if value == "1" {
                add_to_setlist = true;
            }
        }
    }
    if !song_to_be_added_to_gig {
        return false;
    };

    info!("Marking song {} as played in gig {}", song_id, gig_id);
    if add_to_setlist {
        jukebox_db::add_song_to_gig(song_id, gig_id, connection);
    } else {
        jukebox_db::remove_song_from_gig(song_id, gig_id, connection);
    }
    true
}

#[get("/songs/{song_handle}")]
pub async fn service(
    path: web::Path<String>,
    app_state: web::Data<AppState>,
    request: HttpRequest,
) -> actix_web::Result<impl Responder> {
    let song_handle = path.into_inner();
    let song_path = format!("songs/{}", song_handle);
    let connection_pool = app_state.pool.clone();
    let gig_id = session::gig_id_from_session_or_db(
        &mut request.get_session(),
        &mut connection_pool.get().unwrap(),
    );
    let app_url = app_state.base_url.clone();
    let mut connection = app_state.pool.get().expect("could not get connection");

    let song_from_db = web::block(move || {
        jukebox_db::song_by_handle_with_gig_info(&mut connection, song_handle, gig_id)
    })
    .await?;
    let mut connection = app_state.pool.get().expect("could not get connection");

    match song_from_db {
        None => Ok(HttpResponse::SeeOther()
            .append_header(("Location", "/songs"))
            .body("moved on")),
        Some(song) => {
            if set_played_at_now(song.id, gig_id, &request, &mut connection) {
                debug!("Song marked as played");
                return Ok(HttpResponse::Ok()
                    .content_type(ContentType::plaintext())
                    .body("well played."));
            }

            let chord_down_song = chord_down::Song::from_ron(song.serialized_chord_pro);
            if let Some(redirect) = maybe_rickroll_to_original(&chord_down_song) {
                return Ok(redirect);
            }
            let page_url = crate::services::qrcode::full_url(&app_url, &song_path);
            let template = templates::SongsTemplate {
                song: chord_down_song,
                song_id: song.id,
                played_at: song.played_at_gig,
                is_admin: crate::services::session::is_admin(&request),
                dark_background: false,
                zoom: crate::services::session::zoom_from_session(&request),
                qr_code_svg: crate::services::qrcode::qr_code_as_svg(&page_url, &app_state.cache),
                qr_code_url: page_url.to_string(),
                is_dev_mode: app_state.is_dev_mode(),
            };
            let html = template.render().unwrap();
            Ok(HttpResponse::Ok()
                .content_type(ContentType::html())
                .body(html))
        }
    }
}
