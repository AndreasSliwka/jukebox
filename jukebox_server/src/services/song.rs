use crate::services;
use crate::templates;
use crate::types::DbPool;
use actix_web::http::header::ContentType;
use actix_web::{HttpRequest, HttpResponse, Responder, get, web};
use askama::Template;
use diesel::SqliteConnection;
use jukebox_db;
use services::session::{gig_id_from_session, is_admin, start_session_unless_present};

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

#[get("/songs/{song_id}")]
pub async fn service(
    path: web::Path<i32>,
    pool: web::Data<DbPool>,
    request: HttpRequest,
) -> actix_web::Result<impl Responder> {
    if let Some(redirect) = start_session_unless_present(&request) {
        return Ok(redirect);
    }
    let song_id = path.into_inner();
    let maybe_gig_id = gig_id_from_session(&request);

    let mut connection = web::block(move || pool.get().expect("could not get connection")).await?;

    if let Some(redirect) =
        set_played_at_now_and_redirect(song_id, maybe_gig_id, &request, &mut connection).await
    {
        return Ok(redirect);
    }

    let song_from_db = web::block(move || {
        jukebox_db::song_by_id_with_gig_info(&mut connection, song_id, maybe_gig_id)
    })
    .await?;

    match song_from_db {
        None => Ok(HttpResponse::SeeOther()
            .append_header(("Location", "/songs"))
            .body("moved on")),
        Some(song) => {
            let chord_down_song = chord_down::Song::from_ron(song.serialized_chord_pro);
            println!("song.played_at_gig = {:#?}", song.played_at_gig);
            let template = templates::SongsTemplate {
                song: chord_down_song,
                played_at: song.played_at_gig,
                is_admin: crate::services::session::is_admin(&request),
            };
            let html = template.render().unwrap();
            Ok(HttpResponse::Ok()
                .content_type(ContentType::html())
                .body(html))
        }
    }
}
