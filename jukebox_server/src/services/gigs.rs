use crate::types::AppState;
use actix_web::http::Method;
use actix_web::{HttpRequest, Responder, get, route, web};
use chrono::Local;
use dashmap::DashMap;
use jukebox_db;
use jukebox_db::models::{Gig, GigWithPlayedSongs, NewGig};
use log::debug;
use serde::Serialize;
use std::collections::HashMap;
use url::Url;

#[derive(Serialize, Debug)]
struct AllGigsAndCurrent {
    all_gigs: Vec<GigWithPlayedSongs>,
    current_gig: Option<GigWithPlayedSongs>,
    updated_gig: Option<GigWithPlayedSongs>,
}

impl AllGigsAndCurrent {
    fn empty() -> Self {
        Self {
            all_gigs: Vec::new(),
            current_gig: None,
            updated_gig: None,
        }
    }
}

fn gig_with_played_songs(
    plain_gig: Option<Gig>,
    all_played_songs: &HashMap<i32, Vec<String>>,
) -> Option<GigWithPlayedSongs> {
    if let Some(gig) = plain_gig {
        Some(GigWithPlayedSongs::from(
            &gig,
            all_played_songs.get(&gig.id).cloned().unwrap_or_default(),
        ))
    } else {
        None
    }
}

fn all_gigs_and_the_current(
    app_state: &AppState,
    updated_gig_id: Option<i32>,
) -> AllGigsAndCurrent {
    let connection_pool = &app_state.pool;
    let mut connection = connection_pool.get().expect("could not get connection");
    let all_played_songs = crate::jukebox_db::all_played_songs(&mut connection);
    let all_gigs = crate::jukebox_db::all_gigs(&mut connection);
    let all_gigs_with_played_songs = all_gigs
        .into_iter()
        .map(|gig| {
            GigWithPlayedSongs::from(
                &gig,
                all_played_songs.get(&gig.id).cloned().unwrap_or_default(),
            )
        })
        .collect::<Vec<GigWithPlayedSongs>>();

    let current_gig = gig_with_played_songs(
        crate::jukebox_db::current_gig_from_db(&mut connection),
        &all_played_songs,
    );
    let updated_gig = if let Some(gig_id) = updated_gig_id {
        gig_with_played_songs(
            crate::jukebox_db::find_gig(&mut connection, gig_id),
            &all_played_songs,
        )
    } else {
        None
    };

    AllGigsAndCurrent {
        all_gigs: all_gigs_with_played_songs,
        current_gig,
        updated_gig,
    }
}

fn now() -> String {
    Local::now().naive_local().to_string().replace(" ", "T")
}

fn update_gig(app_state: &AppState, payload: web::Bytes) -> AllGigsAndCurrent {
    use diesel::prelude::*;
    let connection_pool = &app_state.pool;
    let mut connection = connection_pool.get().expect("could not get connection");

    let mut updates: Gig = serde_json::from_slice(&payload).unwrap();
    debug!("updates = {:?}", updates);
    if updates.date_end == "now" {
        updates.date_end = now();
    }

    diesel::update(crate::jukebox_db::schema::gigs::dsl::gigs.find(updates.id))
        .set(&updates)
        .execute(&mut connection)
        .expect("could not update gig");

    all_gigs_and_the_current(app_state, Some(updates.id))
}

fn create_gig(app_state: &AppState, payload: web::Bytes) -> AllGigsAndCurrent {
    use diesel::prelude::*;
    let mut connection = app_state.pool.get().expect("could not get connection");

    let mut new_gig: NewGig = serde_json::from_slice(&payload).unwrap();
    new_gig.date_start = now();

    debug!("new_gig: {:?}", new_gig);
    let new_gig: Gig = diesel::insert_into(crate::jukebox_db::schema::gigs::dsl::gigs)
        .values(&new_gig)
        .get_result::<Gig>(&mut connection)
        .expect("could not create gig");
    all_gigs_and_the_current(app_state, Some(new_gig.id))
}

#[route("/gigs", method = "GET", method = "POST", method = "PUT")]
pub async fn service(
    request: HttpRequest,
    payload: web::Bytes,
    app_state: web::Data<AppState>,
) -> actix_web::Result<impl Responder> {
    if !crate::services::session::is_admin(&request) {
        return Err(actix_web::error::ErrorUnauthorized("Unauthorized"));
    }
    let method = request.method().clone();

    let response = web::block(move || match method {
        Method::GET => all_gigs_and_the_current(&app_state, None),
        Method::POST => update_gig(&app_state, payload),
        Method::PUT => create_gig(&app_state, payload),
        _ => AllGigsAndCurrent::empty(),
    })
    .await
    .expect("Error reading/writing gig");
    Ok(web::Json(response))
}
#[get("/gig/{id}/songs")]
pub async fn songs_service(
    id: web::Path<i32>,
    request: HttpRequest,

    app_state: web::Data<AppState>,
) -> actix_web::Result<impl Responder> {
    if !crate::services::session::is_admin(&request) {
        return Err(actix_web::error::ErrorUnauthorized("Unauthorized"));
    }
    let gig_id = id.into_inner();
    let connection_pool = &app_state.pool;
    let mut connection = connection_pool.get().expect("could not get connection");

    let songs = web::block(move || {
        let song_ids = crate::jukebox_db::songs_played_in_gig(gig_id, &mut connection)
            .keys()
            .cloned()
            .collect::<Vec<_>>();
        let songs = crate::jukebox_db::all_listed_songs(&mut connection, song_ids);
        songs
    })
    .await
    .expect("Error reading/writing gig");
    debug!("songsPlayedInSelectedGig {} = {:#?}", gig_id, songs);
    Ok(web::Json(songs))
}

#[derive(Serialize, Debug)]
struct QRCode {
    svg: String,
    url: String,
}

impl QRCode {
    pub fn new(source: &Url, cache: &DashMap<String, String>) -> Self {
        let url = source.to_string();
        let svg = crate::services::qrcode::qr_code_as_svg(source, cache);
        Self { svg, url }
    }
}

#[get("/gigs/{id}/admin_qr")]
pub async fn admin_qr_service(
    id: web::Path<i32>,
    request: HttpRequest,
    app_state: web::Data<AppState>,
) -> actix_web::Result<impl Responder> {
    if !crate::services::session::is_admin(&request) {
        return Err(actix_web::error::ErrorUnauthorized("Unauthorized"));
    }
    let gig_id = id.into_inner();
    let connection_pool = &app_state.pool;
    let mut connection = connection_pool.get().expect("could not get connection");

    let gig = jukebox_db::find_gig(&mut connection, gig_id).unwrap();
    let app_url = crate::services::session::admin_url(&gig, &app_state);
    let qr_code = QRCode::new(&app_url, &app_state.cache);
    Ok(web::Json(qr_code))
}
