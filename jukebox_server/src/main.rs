mod templates;

use actix_files::Files;
use actix_web::http::header::ContentType;
use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};
use actix_web::{HttpRequest, error};
use askama::Template;
use diesel::{prelude::*, r2d2};
use jukebox_db::{self, SongListOrder, models::SongWithLink};
use querystring;

type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

fn parse_query(query: &str) -> SongListOrder {
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

enum RequestedMimeType {
    Html,
    Json,
    Other,
}

fn requested_mime_type(path_tail: String) -> RequestedMimeType {
    match path_tail.as_str() {
        "" | ".html" => RequestedMimeType::Html,

        ".json" => RequestedMimeType::Json,
        _ => RequestedMimeType::Other,
    }
}

#[get("/songs{tail:.*}")]
async fn songs(
    path: web::Path<String>,
    request: HttpRequest,
    pool: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    let song_list_order = parse_query(request.query_string());
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
    match requested_mime_type(path.into_inner()) {
        RequestedMimeType::Json => Ok(HttpResponse::Ok().json(songs_with_links)),
        RequestedMimeType::Html => {
            let template = templates::SongsIndexTemplate {
                songs: songs_with_links,
                song_list_order,
            };
            let html = template.render().unwrap();
            Ok(HttpResponse::Ok()
                .content_type(ContentType::html())
                .body(html))
        }
        _ => Ok(HttpResponse::ImATeapot().body(".. and so are you!")),
    }
}

#[get("/songs/{song_id}")]
async fn single_song(
    path: web::Path<i32>,
    pool: web::Data<DbPool>,
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
            };
            let html = template.render().unwrap();
            Ok(HttpResponse::Ok()
                .content_type(ContentType::html())
                .body(html))
        }
    }
}

#[get("/")]
async fn welcome() -> actix_web::Result<impl Responder> {
    let html = templates::WelcomeTemplate {}.render().unwrap();
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}

fn service_serving_static_files() -> actix_files::Files {
    Files::new("/static", "static")
        .use_last_modified(true)
        .prefer_utf8(true)
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    use actix_web::middleware::Logger;
    use env_logger::Env;

    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let connection_pool = jukebox_db::create_connection_pool();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%s \"%r\" %b %T"))
            .app_data(web::Data::new(connection_pool.clone()))
            .service(service_serving_static_files())
            .service(welcome)
            .service(single_song)
            .service(songs)
    })
    .bind(("192.168.178.120", 8080))?
    .run()
    .await
}
