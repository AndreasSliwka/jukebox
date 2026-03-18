mod templates;

use actix_web::http::header::ContentType;
use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};
use askama::Template;
use chord_down;
use diesel::{prelude::*, r2d2};
use jukebox_db::{self, models::SongWithLink};
use ron;

type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

#[get("/songs{tail:.*}")]
async fn songs(path: web::Path<String>, pool: web::Data<DbPool>) -> impl Responder {
    let mut connection = pool.get().expect("could not get connection");
    let songs_without_links = jukebox_db::all_songs(&mut connection);
    let songs_with_links: Vec<SongWithLink> = songs_without_links
        .iter()
        .map(|song| SongWithLink::from(song))
        .collect();
    let path_tail = path.into_inner();
    println!("path_tail = {}", path_tail);
    match path_tail.as_str() {
        ".json" => HttpResponse::Ok().json(songs_with_links),
        "" => {
            let template = templates::SongsIndexTemplate {
                songs: songs_with_links,
            };
            let html = template.render().unwrap();
            HttpResponse::Ok()
                .content_type(ContentType::html())
                .body(html)
        }
        _ => HttpResponse::ImATeapot().body(".. and so are you!"),
    }
}

#[get("/songs/{song_id}")]
async fn single_song(path: web::Path<i32>, pool: web::Data<DbPool>) -> impl Responder {
    let mut connection = pool.get().expect("could not get connection");
    let song_id = path.into_inner();
    match jukebox_db::song_by_id(&mut connection, song_id) {
        None => HttpResponse::NotFound().body("nope"),
        Some(song) => {
            if let Some(lyrics) = song.lyrics_as_chordpro {
                use ron::ser::{PrettyConfig, to_string_pretty};

                let structured_lyrics = chord_down::Song::parse(lyrics.clone());
                let ron_lyrics =
                    to_string_pretty(&structured_lyrics, PrettyConfig::default()).unwrap();

                println!("ron says: {}", ron_lyrics);
                HttpResponse::Ok()
                    .content_type(ContentType::plaintext())
                    .body(lyrics)
            } else {
                HttpResponse::FailedDependency().body("No lyrics found")
            }
        }
    }
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let connection_pool = jukebox_db::create_connection_pool();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(connection_pool.clone()))
            .service(single_song)
            .service(songs)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
