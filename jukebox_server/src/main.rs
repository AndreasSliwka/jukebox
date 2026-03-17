use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};
use diesel::{prelude::*, r2d2};
use jukebox_db;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

async fn songs(pool: web::Data<DbPool>) -> impl Responder {
    let mut connection = pool.get().expect("could not get connection");
    let songs = jukebox_db::all_songs(&mut connection);

    HttpResponse::Ok().json(songs)
}

#[get("/songs/{song_id}")]
async fn single_song(path: web::Path<i32>, pool: web::Data<DbPool>) -> impl Responder {
    let mut connection = pool.get().expect("could not get connection");
    let song_id = path.into_inner();
    match jukebox_db::song_by_id(&mut connection, song_id) {
        None => HttpResponse::NotFound().body("nope"),
        Some(song) => HttpResponse::Ok().content_type("text/plain").body(
            song.lyrics_as_chordpro
                .unwrap_or(String::from(" nothing here ")),
        ),
    }
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let connection_pool = jukebox_db::create_connection_pool();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(connection_pool.clone()))
            .service(single_song)
            .route("/songs", web::get().to(songs))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
