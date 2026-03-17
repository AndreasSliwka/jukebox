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

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there ...")
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let connection_pool = jukebox_db::create_connection_pool();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(connection_pool.clone()))
            .service(hello)
            .route("/songs", web::get().to(songs))
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
