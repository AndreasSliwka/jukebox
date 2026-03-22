use crate::services;
use crate::templates;
use crate::types::DbPool;
use actix_web::http::header::ContentType;
use actix_web::{HttpRequest, HttpResponse, Responder, error, get, web};
use askama::Template;
use jukebox_db;
use services::session::{gig_id_from_session, is_admin, start_session_unless_present};

#[get("/songs/{song_id}")]
pub async fn service(
    path: web::Path<i32>,
    pool: web::Data<DbPool>,
    request: HttpRequest,
) -> actix_web::Result<impl Responder> {
    if let Some(redirect) = start_session_unless_present(&request) {
        return Ok(redirect);
    }
    let mut song_to_be_added_to_gig = false;
    let maybe_gig_id = gig_id_from_session(&request);

    if is_admin(&request) {
        for (key, _value) in querystring::querify(request.query_string()) {
            if key == "add_to_setlist" {
                song_to_be_added_to_gig = true;
            }
        }
    }

    let song_id = path.into_inner();
    let song_from_db = web::block(move || {
        let mut connection = pool.get().expect("could not get connection");
        if let Some(gig_id) = maybe_gig_id
            && song_to_be_added_to_gig == true
        {
            jukebox_db::add_song_to_gig(song_id, gig_id, &mut connection);
        };
        jukebox_db::song_by_id_with_gig_info(&mut connection, song_id, maybe_gig_id)
    })
    .await
    .map_err(error::ErrorInternalServerError)?;
    match song_from_db {
        None => Ok(HttpResponse::PermanentRedirect()
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
