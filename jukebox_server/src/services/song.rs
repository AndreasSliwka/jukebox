use crate::templates;
use crate::types::DbPool;
use actix_web::http::header::ContentType;
use actix_web::{HttpRequest, HttpResponse, Responder, error, get, web};
use askama::Template;
use jukebox_db;

#[get("/songs/{song_id}")]
pub async fn service(
    path: web::Path<i32>,
    pool: web::Data<DbPool>,
    request: HttpRequest,
) -> actix_web::Result<impl Responder> {
    let song_id = path.into_inner();
    let song_from_db = web::block(move || {
        let mut connection = pool.get().expect("could not get connection");
        jukebox_db::song_by_id(&mut connection, song_id)
    })
    .await
    .map_err(error::ErrorInternalServerError)?;
    match song_from_db {
        None => Ok(HttpResponse::PermanentRedirect()
            .append_header(("Location", "/songs"))
            .body("moved on")),
        Some(song) => {
            let chord_down_song = chord_down::Song::from_ron(song.serialized_chord_pro);
            let template = templates::SongsTemplate {
                song: chord_down_song,
                is_admin: crate::services::session::is_admin(&request),
            };
            let html = template.render().unwrap();
            Ok(HttpResponse::Ok()
                .content_type(ContentType::html())
                .body(html))
        }
    }
}
