use crate::types::{AppState, ServerConfig};
use actix_session::config::BrowserSession;
use actix_session::{Session, SessionExt};
use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_web::{HttpRequest, HttpResponse, Responder, cookie::Key, get, web};
use base64::prelude::*;
use jukebox_db;
use log::debug;
use serde::Deserialize;

pub const GIG_ID: &str = "gig.id";
pub const GIG_ADMIN_SECRET: &str = "gig.admin_secret";
pub const IS_ADMIN: &str = "isAdmin";
pub const SHOW_PRIVATE: &str = "showPrivate";
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

pub fn show_private(request: &HttpRequest) -> bool {
    let Ok(session_value) = request.get_session().get::<bool>(SHOW_PRIVATE) else {
        return false;
    };
    let Some(value) = session_value else {
        return false;
    };
    value
}

fn toggle_is_private(request: &HttpRequest) {
    let new_show_private = !show_private(request);
    request
        .get_session()
        .insert(SHOW_PRIVATE, new_show_private)
        .unwrap();
}

const ALLOW_ACCES_WITHOUT_SESSION: bool = false;
pub fn is_present(request: &HttpRequest) -> bool {
    let maybe_gig_id: Option<i32> = gig_id_from_session(request);
    if let Some(_gig_id) = maybe_gig_id {
        return true;
    };

    // Houston, we have a Problem.
    return ALLOW_ACCES_WITHOUT_SESSION || false;
}

pub fn gig_id_from_session(request: &HttpRequest) -> Option<i32> {
    request.get_session().get::<i32>(GIG_ID).unwrap()
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

pub fn redirect_to_start_session(request: &HttpRequest) -> HttpResponse {
    let requested_path = request.path();
    let redirect_to = format!("/start_session?requested={}", requested_path);
    debug!("redirect_to_start_session, redirect_to = {}", redirect_to);
    HttpResponse::SeeOther()
        .append_header(("Location", redirect_to))
        .body("no session")
}
pub fn start_session_unless_present(request: &HttpRequest) -> Option<HttpResponse> {
    match is_present(request) {
        true => None,
        false => Some(redirect_to_start_session(request)),
    }
}

#[get("/toggle_private")]
async fn toggle_private_service(request: HttpRequest) -> actix_web::Result<impl Responder> {
    log::debug!("> toggle_private_service");
    if is_admin(&request) {
        log::debug!("   > is_admin!");
        toggle_is_private(&request);
    }
    log::debug!("show_private: {}", show_private(&request));

    return Ok(HttpResponse::SeeOther()
        .append_header(("Location", "/songs"))
        .body("Session started"));
}

#[derive(Deserialize, Debug)]
struct SessionQuery {
    requested: String,
}

#[get("/start_session")]
async fn service(
    request: HttpRequest,
    app_state: web::Data<AppState>,
    query: web::Query<SessionQuery>,
) -> actix_web::Result<impl Responder> {
    debug!("start_session::service()");
    let maybe_gig = web::block(move || {
        let mut connection = app_state.pool.get().expect("could not get connection");

        jukebox_db::current_gig_from_db(&mut connection)
    })
    .await?;
    let Some(gig) = maybe_gig else {
        debug!("No gig found");
        return Ok(HttpResponse::SeeOther()
            .append_header(("Location", "/no_shoes_no_shirt"))
            .body("moved on"));
    };
    request.get_session().insert(GIG_ID, gig.id).unwrap();

    request
        .get_session()
        .insert(GIG_ADMIN_SECRET, gig.admin_secret)
        .unwrap();

    request.get_session().insert(SHOW_PRIVATE, false).unwrap();
    request.get_session().insert(ZOOM, 3).unwrap();

    debug!("session = {:#?}", request.get_session().entries());
    debug!("query = {:#?}", query);
    let redirect_to = if query.requested.chars().count() > 0 {
        query.requested.as_str()
    } else {
        "/songs"
    };
    return Ok(HttpResponse::SeeOther()
        .append_header(("Location", redirect_to))
        .body("Session started"));
}
