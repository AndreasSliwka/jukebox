use crate::services::session;
use crate::templates;
use actix_session::SessionExt;
use actix_web::http::header;
use actix_web::{HttpRequest, HttpResponse, Responder, get};
use askama::Template;
use qrcode::QrCode;
use qrcode::render::svg;
use regex::Regex;
use std::sync::LazyLock;

fn sanitized_svg(source: String) -> String {
    static RE1: LazyLock<Regex> =
        LazyLock::new(|| Regex::new("width=\"\\d+\" height=\"\\d+\"").unwrap());
    static RE2: LazyLock<Regex> = LazyLock::new(|| Regex::new("<rect.*fill=\"#fff\"/").unwrap());
    let bounded_svg = QrCode::new(source).unwrap().render::<svg::Color>().build();

    let unbounded_svg = RE1.replace(bounded_svg.as_str(), "").to_string();
    let without_background = RE2.replace(unbounded_svg.as_str(), "").to_string();
    without_background
}
#[get("/qrcode")]
async fn service(
    request: HttpRequest,
    app_state: actix_web::web::Data<crate::types::AppState>,
) -> actix_web::Result<impl Responder> {
    if session::is_admin(&request) {
        let passkey =
            crate::services::session::admin_secret_from_session(&request.get_session()).unwrap();
        let mut admin_url = app_state.base_url.clone();
        admin_url.set_path("admin");
        admin_url.set_query(Some(format!("passkey={}", passkey).as_str()));
        log::debug!("admin_url = {}", admin_url);
        let mut public_url = request.full_url();
        public_url.set_path("songs/");

        let template = templates::QrCodesTemplate {
            public_url_svg: sanitized_svg(public_url.to_string()),
            admin_url_svg: sanitized_svg(admin_url.to_string()),
            is_admin: session::is_admin(&request),
            show_private: session::show_private(&request),
        };
        let html = template.render().unwrap();
        Ok(HttpResponse::Ok()
            .content_type(header::ContentType::html())
            .body(html))
    } else {
        Ok(HttpResponse::NotFound().body("moved on"))
    }
}
