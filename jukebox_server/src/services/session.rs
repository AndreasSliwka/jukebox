use crate::types::DbPool;
use actix_session::SessionExt;
use actix_session::config::BrowserSession;
use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_web::{HttpRequest, HttpResponse, Responder, cookie::Key, get, web};
use jukebox_db;

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
    let Ok(session_value) = request.get_session().get::<bool>("isAdmin") else {
        return false;
    };
    let Some(value) = session_value else {
        return false;
    };
    value
}

#[get("/start_session")]
async fn service(
    request: HttpRequest,
    pool: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    let Some(gig) = web::block(move || {
        let mut connection = pool.get().expect("could not get connection");

        let result = jukebox_db::current_gig_from_db(&mut connection);
        println!("  start_session: result = {:#?}", result);
        result
    })
    .await?
    else {
        println!("  start_session: No shoes, no shirt");
        return Ok(HttpResponse::PermanentRedirect()
            .append_header(("Location", "/no_shoes_no_shirt"))
            .body("moved on"));
    };
    request.get_session().insert("gig_id", gig.id).unwrap();
    println!("  found a gig! {:#?}", gig);
    return Ok(HttpResponse::PermanentRedirect()
        .append_header(("Location", "/songs"))
        .body("Session started"));
}
