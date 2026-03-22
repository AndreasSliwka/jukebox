use actix_session::{Session, SessionExt};
use actix_web::{HttpRequest, HttpResponse, Responder, get};
use querystring;

fn validate_passkey(passkey: &str, session: &Session) {
    let Some(admin_secret) = crate::services::session::admin_secret_from_session(session) else {
        return;
    };
    if admin_secret == passkey {
        println!("  isAdmin = true");
        session.insert(String::from("isAdmin"), true).unwrap();
    } else {
        println!("  isAdmin = false, passkey = {}", passkey);
        session.insert(String::from("isAdmin"), false).unwrap();
    }
}

#[get("/admin")]
async fn service(request: HttpRequest) -> actix_web::Result<impl Responder> {
    if let Some(redirect) = crate::services::session::start_session_unless_present(&request) {
        return Ok(redirect);
    }
    for (key, value) in querystring::querify(request.query_string()) {
        if key == "passkey" {
            let session = request.get_session();
            validate_passkey(value, &session);
        }
    }

    Ok(HttpResponse::TemporaryRedirect()
        .append_header(("Location", "/songs"))
        .body("moved on"))
}
