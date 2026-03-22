mod filters;
mod services;
mod templates;
mod types;

use actix_web::{App, HttpServer, web};

use jukebox_db;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    use actix_web::middleware::Logger;
    use env_logger::Env;

    env_logger::init_from_env(Env::default().default_filter_or("info"));
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
    .bind(("192.168.178.120", 8080))?
    .run()
    .await
}
