use crate::templates;
use actix_web::http::header::ContentType;
use actix_web::{HttpResponse, Responder, get};
use askama::Template;

#[get("/")]
async fn service() -> actix_web::Result<impl Responder> {
    let html = templates::WelcomeTemplate {}.render().unwrap();

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}
