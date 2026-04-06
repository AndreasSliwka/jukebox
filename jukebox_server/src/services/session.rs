use crate::types::{AppState, ServerConfig};
use actix_session::config::BrowserSession;
use actix_session::{Session, SessionExt};
use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_web::{HttpRequest, HttpResponse, Responder, cookie::Key, get, web};
use jukebox_db;

pub const GIG_ID: &str = "gig.id";
pub const GIG_ADMIN_SECRET: &str = "gig.admin_secret";
pub const IS_ADMIN: &str = "isAdmin";

fn get_secret_key() -> Key {
    Key::from(b"aBcDeFGhIjKlIaBcDeFGhIjKlIaBcDeFGhIjKlIaBcDeFGhIjKlIaBcDeFGhIjKlIaBcDeFGhIjKlI")
}

pub fn middleware(server_config: &ServerConfig) -> SessionMiddleware<CookieSessionStore> {
    SessionMiddleware::builder(CookieSessionStore::default(), get_secret_key())
        .cookie_name(String::from("session"))
        .cookie_secure(!server_config.http_only) // Defina como false para localhost, true para produção
        .cookie_http_only(server_config.http_only)
        .session_lifecycle(BrowserSession::default())
        .cookie_same_site(actix_web::cookie::SameSite::Strict)
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

pub fn admin_secret_from_session(session: &Session) -> Option<String> {
    session.get::<String>(GIG_ADMIN_SECRET).unwrap()
}

pub fn redirect_to_start_session() -> HttpResponse {
    HttpResponse::SeeOther()
        .append_header(("Location", "/start_session"))
        .body("no session")
}
pub fn start_session_unless_present(request: &HttpRequest) -> Option<HttpResponse> {
    match is_present(request) {
        true => None,
        false => Some(redirect_to_start_session()),
    }
}

#[get("/start_session")]
async fn service(
    request: HttpRequest,
    app_state: web::Data<AppState>,
) -> actix_web::Result<impl Responder> {
    let maybe_gig = web::block(move || {
        let mut connection = app_state.pool.get().expect("could not get connection");

        jukebox_db::current_gig_from_db(&mut connection)
    })
    .await?;
    let Some(gig) = maybe_gig else {
        return Ok(HttpResponse::SeeOther()
            .append_header(("Location", "/no_shoes_no_shirt"))
            .body("moved on"));
    };
    request.get_session().insert(GIG_ID, gig.id).unwrap();

    request
        .get_session()
        .insert(GIG_ADMIN_SECRET, gig.admin_secret)
        .unwrap();

    println!("session = {:#?}", request.get_session().entries());

    return Ok(HttpResponse::SeeOther()
        .append_header(("Location", "/songs"))
        .body("Session started"));
}
