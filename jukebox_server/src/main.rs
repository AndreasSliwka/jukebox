mod filters;
mod services;
mod templates;
mod types;

use actix_web::middleware::Logger;
use actix_web::{App, HttpServer, web};
use env_logger::Env;
use jukebox_db;

use crate::types::AppState;
use log;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let server_config = types::ServerConfig::from_env();
    let binding = server_config.binding();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    jukebox_db::run_migrations();
    let app_state = AppState::load();
    log::debug!("app_state = {:#?}", app_state);
    HttpServer::new(move || {
        App::new()
            .wrap(services::session::middleware(&server_config))
            .wrap(Logger::default())
            .app_data(web::Data::new(app_state.clone()))
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
