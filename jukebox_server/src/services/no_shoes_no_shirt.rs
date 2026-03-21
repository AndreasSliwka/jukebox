use crate::templates;
use actix_web::http::header::ContentType;
use actix_web::{HttpResponse, Responder, get};
use askama::Template;

#[get("/no_shoes_no_shirt")]
pub async fn service() -> actix_web::Result<impl Responder> {
    let html = templates::NoShoesNoShirtTemplate {}.render().unwrap();

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}
