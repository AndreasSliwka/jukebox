use crate::services::session;
use crate::types::AppState;
use actix_session::{Session, SessionExt};
use actix_web::{HttpRequest, HttpResponse, Responder, get, web};
use chrono::Utc;
use log::debug;
use querystring;

fn validate_passkey(passkey: &str, session: &Session) -> bool {
    let Some(admin_secret) = session::admin_secret_from_session(session) else {
        log::info!("isAdmin = false, no admin_secret in session");
        return false;
    };
    session
        .insert(String::from("TIMESTAMP"), Utc::now().to_string())
        .unwrap();
    if admin_secret == passkey {
        log::info!("isAdmin = true");
        println!("  isAdmin = true");
        session.insert(String::from("isAdmin"), true).unwrap();
        true
    } else {
        log::info!("isAdmin = true");
        println!("  isAdmin = false, passkey = {}", passkey);
        session.insert(String::from("isAdmin"), false).unwrap();
        false
    }
}

#[get("/admin")]
async fn service(
    request: HttpRequest,
    app_state: web::Data<AppState>,
) -> actix_web::Result<impl Responder> {
    let connection_pool = app_state.pool.clone();

    let _gig_id = session::gig_id_from_session_or_db(
        &mut request.get_session(),
        &mut connection_pool.get().unwrap(),
    );
    let mut is_admin = false;
    for (key, value) in querystring::querify(request.query_string()) {
        if key == "passkey" {
            debug!("passkey: {}", value);
            let session = request.get_session();
            is_admin = validate_passkey(value, &session);
        }
    }

    debug!(
        "Session keys = {:?}",
        request.get_session().entries().keys()
    );
    debug!(
        "Timestamp: {:#?} ",
        request.get_session().get::<String>("TIMESTAMP").unwrap()
    );

    Ok(HttpResponse::SeeOther()
        .append_header(("Location", "/songs"))
        .body(format!("isAdmin: {}", is_admin)))
}
