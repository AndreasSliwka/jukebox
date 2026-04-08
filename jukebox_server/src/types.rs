use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenvy::dotenv;
use jukebox_db;
use std::collections::HashMap;
use std::env;

#[derive(Clone)]
pub struct ServerConfig {
    pub binding_ip: String,
    pub port: u16,
    pub http_only: bool,
    pub secret_key: String,
}

impl ServerConfig {
    pub fn from_env() -> Self {
        dotenv().ok();

        Self {
            binding_ip: env::var("BINDING_HOST")
                .expect("Set BINDING_HOST (ip) in Environment or .env"),
            port: env::var("PORT")
                .expect("Set PORT (u16) in Environment or .env")
                .parse::<u16>()
                .expect("PORT must be a u16"),
            http_only: env::var("HTTP_ONLY").expect("Set HTTP_ONLY (bool) in Environment or .env")
                == "true",
            secret_key: env::var("SECRET_KEY").expect("SECRET_KEY must be set"),
        }
    }
    pub fn binding(&self) -> (String, u16) {
        (self.binding_ip.clone(), self.port)
    }
}

#[derive(Clone, Debug)]
pub struct AppState {
    pub pool: Pool<ConnectionManager<SqliteConnection>>,
    pub private_tag_ids: Vec<i32>,
    pub tags_by_id: HashMap<i32, (String, String, bool)>,
}

impl AppState {
    pub fn load() -> Self {
        dotenv().ok();
        let pool = jukebox_db::create_connection_pool();
        let mut connection = pool.get().expect("could not get connection");
        let private_tag_ids = jukebox_db::all_private_tag_ids(&mut connection);
        let tags_by_id = jukebox_db::all_tags_by_id(&mut connection);

        Self {
            pool,
            private_tag_ids,
            tags_by_id,
        }
    }
}
