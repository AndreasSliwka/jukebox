mod filters;
mod services;
mod templates;
mod types;

use actix_web::{App, HttpServer, web};
use jukebox_db;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let server_config = types::ServerConfig::from_env();
    let binding = server_config.binding();
    use actix_web::middleware::Logger;
    use env_logger::Env;
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    jukebox_db::run_migrations();
    let connection_pool = jukebox_db::create_connection_pool();

    HttpServer::new(move || {
        App::new()
            .wrap(services::session::middleware(&server_config))
            .wrap(Logger::default())
            // .wrap(Logger::new("%s \"%r\" %b %T"))
            .app_data(web::Data::new(connection_pool.clone()))
            .service(services::static_files::service())
            .service(services::admin::service)
            .service(services::welcome::service)
            .service(services::session::service)
            .service(services::no_shoes_no_shirt::service)
            .service(services::song::service)
            .service(services::songs::service)
    })
    .bind(binding)?
    .run()
    .await
}
