use crate::templates;
use crate::types::AppState;
use actix_web::http::header::ContentType;
use actix_web::{HttpResponse, Responder, get, web};
use askama::Template;

#[get("/")]
async fn service(app_state: web::Data<AppState>) -> actix_web::Result<impl Responder> {
    let html = templates::WelcomeTemplate {
        dark_background: true,
        is_dev_mode: app_state.is_dev_mode(),
    }
    .render()
    .unwrap();

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}
