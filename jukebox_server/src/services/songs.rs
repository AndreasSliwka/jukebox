use crate::services;
use crate::templates::SongsIndexTemplate;
use actix_session::SessionExt;
use actix_web::http::header::ContentType;
use actix_web::{HttpRequest, HttpResponse, Responder, error, get, web};
use askama::Template;
use jukebox_db::{self, SongListOrder, models::SongWithLink};
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
    pool: web::Data<crate::types::DbPool>,
) -> actix_web::Result<impl Responder> {
    println!("session: {:#?}", request.get_session().entries());
    let song_list_order = song_list_order(request.query_string());
    let songs_without_links = web::block(move || {
        let mut connection = pool.get().expect("could not get connection");
        jukebox_db::all_songs(&mut connection, song_list_order, None)
    })
    .await
    .map_err(error::ErrorInternalServerError)?;
    let songs_with_links: Vec<SongWithLink> = songs_without_links
        .iter()
        .map(|song| SongWithLink::from(song))
        .collect();

    let template = SongsIndexTemplate {
        songs: songs_with_links,
        song_list_order,
        is_admin: services::session::is_admin(&request),
    };

    let html = template.render().unwrap();
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}
