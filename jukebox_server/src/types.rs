use diesel::{prelude::*, r2d2};
use dotenvy::dotenv;
use std::env;

pub type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

#[derive(Clone)]
pub struct ServerConfig {
    pub binding_ip: String,
    pub port: u16,
    pub http_only: bool,
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
        }
    }
    pub fn binding(&self) -> (String, u16) {
        (self.binding_ip.clone(), self.port)
    }
}
