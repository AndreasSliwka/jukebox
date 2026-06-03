use crate::types::ServerConfig;
use actix_session::config::BrowserSession;
use actix_session::{Session, SessionExt};
use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_web::{HttpRequest, cookie::Key};
use base64::prelude::*;
use diesel::prelude::*;
use jukebox_db;
// use log::debug;

pub const GIG_ID: &str = "gig.id";
pub const GIG_ADMIN_SECRET: &str = "gig.admin_secret";
pub const IS_ADMIN: &str = "isAdmin";
pub const ZOOM: &str = "zoom";

fn get_secret_key(server_config: &ServerConfig) -> Key {
    let binary = BASE64_STANDARD
        .decode(server_config.secret_key.clone())
        .expect("SECRET_KEY must be valid Base64");
    if binary.len() < 64 {
        panic!("SECRET_KEY must be >64 bytes after Base64 Decoding")
    }
    Key::from(&binary[0..])
}

pub fn middleware(server_config: &ServerConfig) -> SessionMiddleware<CookieSessionStore> {
    SessionMiddleware::builder(CookieSessionStore::default(), get_secret_key(server_config))
        .cookie_name(String::from("session"))
        .cookie_secure(!server_config.http_only) // Defina como false para localhost, true para produção
        .cookie_http_only(server_config.http_only)
        .session_lifecycle(BrowserSession::default())
        .cookie_same_site(actix_web::cookie::SameSite::Lax)
        .build()
}

pub fn is_admin(request: &HttpRequest) -> bool {
    let Ok(session_value) = request.get_session().get::<bool>(IS_ADMIN) else {
        return false;
    };
    let Some(value) = session_value else {
        return false;
    };
    value
}

pub fn zoom_from_session(request: &HttpRequest) -> u16 {
    if let Some(zoom_cookie) = request.cookie("zoom") {
        let zoom_level = zoom_cookie.value().parse::<u16>().unwrap_or(7);
        request.get_session().insert(ZOOM, zoom_level).unwrap();
        return zoom_level;
    } else {
        if let Some(from_session) = request.get_session().get::<u16>(ZOOM).unwrap() {
            from_session
        } else {
            3
        }
    }
}

pub fn admin_secret_from_session(session: &Session) -> Option<String> {
    session.get::<String>(GIG_ADMIN_SECRET).unwrap()
}

pub fn admin_url(
    gig: &crate::jukebox_db::models::Gig,
    app_state: &crate::types::AppState,
) -> url::Url {
    let mut url = app_state.base_url.clone();
    url.set_path("admin");
    url.set_query(Some(format!("passkey={}", gig.admin_secret).as_str()));
    url
}

pub fn gig_id_from_session_or_db(session: &mut Session, connection: &mut SqliteConnection) -> i32 {
    if let Ok(Some(gig_id)) = session.get::<i32>(GIG_ID) {
        return gig_id;
    };

    let gig = jukebox_db::current_gig_from_db_or_default(connection);

    session.insert(GIG_ID, gig.id).unwrap();
    session.insert(GIG_ADMIN_SECRET, gig.admin_secret).unwrap();
    session.insert(ZOOM, 3).unwrap();
    gig.id
}

pub fn set_id_in_session(session: &mut Session, gig_id: i32) {
    session.insert(GIG_ID, gig_id).unwrap();
}
