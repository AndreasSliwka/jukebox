mod filters;
mod services;
mod templates;
mod types;

use actix_web::{App, HttpServer, web};
use dotenvy::dotenv;
use jukebox_db;
use std::env;

fn binding_address_from_env() -> (String, u16) {
    let ip = env::var("HOST_IP").expect("Set HOST_IP in Environment or .env");
    let port = env::var("PORT")
        .expect("Set PORT in Environment or .env")
        .parse::<u16>()
        .expect("PORT must be a u16");

    (ip, port)
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv().ok();
    use actix_web::middleware::Logger;
    use env_logger::Env;
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    jukebox_db::run_migrations();
    let connection_pool = jukebox_db::create_connection_pool();

    HttpServer::new(move || {
        App::new()
            .wrap(services::session::middleware())
            .wrap(Logger::default())
            .wrap(Logger::new("%s \"%r\" %b %T"))
            .app_data(web::Data::new(connection_pool.clone()))
            .service(services::static_files::service())
            .service(services::admin::service)
            .service(services::welcome::service)
            .service(services::session::service)
            .service(services::no_shoes_no_shirt::service)
            .service(services::song::service)
            .service(services::songs::service)
    })
    .bind(binding_address_from_env())?
    .run()
    .await
}
