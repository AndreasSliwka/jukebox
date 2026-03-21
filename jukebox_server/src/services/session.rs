use crate::types::DbPool;
use actix_session::SessionExt;
use actix_session::config::BrowserSession;
use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_web::{HttpRequest, HttpResponse, Responder, cookie::Key, get, web};
use jukebox_db;

pub const GIG_ID: &str = "gig_id";
pub const IS_ADMIN: &str = "isAdmin";
fn get_secret_key() -> Key {
    Key::from(b"aBcDeFGhIjKlIaBcDeFGhIjKlIaBcDeFGhIjKlIaBcDeFGhIjKlIaBcDeFGhIjKlIaBcDeFGhIjKlI")
}

pub fn middleware() -> SessionMiddleware<CookieSessionStore> {
    SessionMiddleware::builder(CookieSessionStore::default(), get_secret_key())
        .cookie_name(String::from("session"))
        .cookie_secure(false) // Defina como false para localhost, true para produção
        .session_lifecycle(BrowserSession::default())
        .cookie_same_site(actix_web::cookie::SameSite::Strict)
        .cookie_http_only(true)
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
    let maybe_gig_id: Option<i32> = request.get_session().get::<i32>(GIG_ID).unwrap();
    if let Some(_gig_id) = maybe_gig_id {
        return true;
    };

    // Houston, we have a Problem.
    return ALLOW_ACCES_WITHOUT_SESSION || false;
}

pub fn start_session_unless_present(request: &HttpRequest) -> Option<HttpResponse> {
    match is_present(request) {
        true => None,
        false => Some(
            HttpResponse::TemporaryRedirect()
                .append_header(("Location", "/start_session"))
                .body("no session"),
        ),
    }
}

#[get("/start_session")]
async fn service(
    request: HttpRequest,
    pool: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    let Some(gig) = web::block(move || {
        let mut connection = pool.get().expect("could not get connection");

        let result = jukebox_db::current_gig_from_db(&mut connection);
        result
    })
    .await?
    else {
        return Ok(HttpResponse::PermanentRedirect()
            .append_header(("Location", "/no_shoes_no_shirt"))
            .body("moved on"));
    };
    request.get_session().insert(GIG_ID, gig.id).unwrap();
    return Ok(HttpResponse::PermanentRedirect()
        .append_header(("Location", "/songs"))
        .body("Session started"));
}
