use crate::services;
use crate::templates;
use crate::types::AppState;
use actix_web::http::header::ContentType;
use actix_web::{HttpRequest, HttpResponse, Responder, get, web};
use askama::Template;
use diesel::SqliteConnection;
use jukebox_db;
use log::debug;
use services::session::{gig_id_from_session, is_admin, start_session_unless_present};

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

async fn set_played_at_now_and_redirect(
    song_id: i32,
    maybe_gig_id: Option<i32>,
    request: &HttpRequest,
    connection: &mut SqliteConnection,
) -> Option<HttpResponse> {
    if !is_admin(&request) {
        return None;
    }
    let mut song_to_be_added_to_gig = false;
    for (key, _value) in querystring::querify(request.query_string()) {
        if key == "add_to_setlist" {
            song_to_be_added_to_gig = true;
        }
    }
    if !song_to_be_added_to_gig {
        return None;
    };

    if let Some(gig_id) = maybe_gig_id {
        jukebox_db::add_song_to_gig(song_id, gig_id, connection);
    };
    Some(
        HttpResponse::SeeOther()
            .append_header(("Location", request.path()))
            .body("no session"),
    )
}

#[get("/songs/{song_handle}")]
pub async fn service(
    path: web::Path<String>,
    app_state: web::Data<AppState>,
    request: HttpRequest,
) -> actix_web::Result<impl Responder> {
    if let Some(redirect) = start_session_unless_present(&request) {
        return Ok(redirect);
    }
    let song_handle = path.into_inner();
    let song_path = format!("songs/{}", song_handle);
    let maybe_gig_id = gig_id_from_session(&request);
    let app_url = app_state.base_url.clone();
    let mut connection = app_state.pool.get().expect("could not get connection");

    let song_from_db = web::block(move || {
        jukebox_db::song_by_handle_with_gig_info(&mut connection, song_handle, maybe_gig_id)
    })
    .await?;
    debug!("song_from_db = {:#?}", song_from_db);
    let mut connection = app_state.pool.get().expect("could not get connection");

    match song_from_db {
        None => Ok(HttpResponse::SeeOther()
            .append_header(("Location", "/songs"))
            .body("moved on")),
        Some(song) => {
            if let Some(redirect) =
                set_played_at_now_and_redirect(song.id, maybe_gig_id, &request, &mut connection)
                    .await
            {
                return Ok(redirect);
            }

            let chord_down_song = chord_down::Song::from_ron(song.serialized_chord_pro);
            if let Some(redirect) = maybe_rickroll_to_original(&chord_down_song) {
                return Ok(redirect);
            }
            debug!("song.played_at_gig = {:#?}", song.played_at_gig);
            debug!("tags = {:#?}", chord_down_song.tags);
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
            };
            let html = template.render().unwrap();
            Ok(HttpResponse::Ok()
                .content_type(ContentType::html())
                .body(html))
        }
    }
}
