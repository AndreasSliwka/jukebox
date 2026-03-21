use actix_files::Files;

pub fn service() -> actix_files::Files {
    Files::new("/static", "static")
        .use_last_modified(true)
        .prefer_utf8(true)
}
