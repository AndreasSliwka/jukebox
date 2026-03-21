use crate::types;
use actix_session::{Session, SessionExt};
use actix_web::{HttpRequest, HttpResponse, Responder, get, web};
use diesel::prelude::*;
use querystring;

fn validate_passkey(passkey: &str, connection: &mut SqliteConnection, session: &Session) {
    let maybe_gig = jukebox_db::current_gig_from_db(connection);
    println!("  maybe_gig = {:#?}", maybe_gig);
    if let Some(gig) = maybe_gig
        && gig.admin_secret == passkey
    {
        println!("  setting isAdmin in session");
        session.insert(String::from("isAdmin"), true).unwrap();
    }
    println!("  unknown passkey {}", passkey);
    session.insert(String::from("isAdmin"), false).unwrap();
}

#[get("/admin")]
async fn service(
    request: HttpRequest,
    pool: web::Data<types::DbPool>,
) -> actix_web::Result<impl Responder> {
    println!("admin!");
    for (key, value) in querystring::querify(request.query_string()) {
        println!("  get value {} = {}", key, value);
        if key == "passkey" {
            let mut connection = pool.get().expect("could not get connection");
            let session = request.get_session();
            validate_passkey(value, &mut connection, &session);
        }
    }

    println!(
        "admin done, session: {:#?}",
        request.get_session().entries()
    );

    Ok(HttpResponse::PermanentRedirect()
        .append_header(("Location", "/songs"))
        .body("moved on"))
}
