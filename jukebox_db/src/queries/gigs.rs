use std::collections::HashMap;

use crate::models::Gig;
use crate::models::NewGig;
use crate::models::SongInGig;
use chrono::{self, Local};
use diesel::debug_query;
use diesel::prelude::*;
use log::debug;

pub fn all_gigs(connection: &mut SqliteConnection) -> Vec<Gig> {
    let query = Gig::query()
        .order(crate::schema::gigs::dsl::date_start.desc())
        .load(connection)
        .expect("Error loading songs");
    query
}

pub fn find_gig(connection: &mut SqliteConnection, gig_id: i32) -> Option<Gig> {
    use crate::schema::gigs::dsl::*;
    Gig::query().filter(id.eq(gig_id)).first(connection).ok()
}

pub fn current_gig_from_db(connection: &mut SqliteConnection) -> Option<Gig> {
    use crate::schema::gigs::dsl::*;
    let now = Local::now().naive_local().to_string().replace(" ", "T");
    let query = Gig::query()
        .filter(date_start.lt(now.clone()))
        .filter(default_gig.eq(0))
        .filter(date_end.gt(now).or(date_end.eq("")));
    match query.first::<Gig>(connection) {
        Ok(gig) => Some(gig),
        Err(_) => None,
    }
}

pub fn default_gig(connection: &mut SqliteConnection) -> Gig {
    use crate::schema::gigs::dsl::*;

    Gig::query()
        .filter(default_gig.eq(1))
        .first::<Gig>(connection)
        .expect("No default Gig found")
}

pub fn current_gig_from_db_or_default(connection: &mut SqliteConnection) -> Gig {
    if let Some(gig) = current_gig_from_db(connection) {
        return gig;
    }
    default_gig(connection)
}

pub fn add_song_to_gig(song_id: i32, gig_id: i32, connection: &mut SqliteConnection) -> () {
    use crate::schema::songs_in_gigs;
    let now = Local::now().naive_local().to_string().replace(" ", "T");

    let new_song_in_gig = SongInGig {
        song_id,
        gig_id,
        played_at: Some(now),
    };
    diesel::insert_into(songs_in_gigs::table)
        .values(&new_song_in_gig)
        .execute(connection)
        .expect("Error saving new song in gig");
}

pub fn remove_song_from_gig(song_id: i32, gig_id: i32, connection: &mut SqliteConnection) -> () {
    use crate::schema::songs_in_gigs::dsl;
    let query = diesel::delete(
        dsl::songs_in_gigs
            .filter(dsl::song_id.eq(song_id))
            .filter(dsl::gig_id.eq(gig_id)),
    );
    log::debug!(
        "Delete Song from Gig: {}",
        debug_query::<diesel::sqlite::Sqlite, _>(&query)
    );

    query
        .execute(connection)
        .expect(format!("Error removing song {} from gig {}", song_id, gig_id).as_str());
}

pub fn songs_played_in_gig(
    gig_id_i32: i32,
    connection: &mut SqliteConnection,
) -> HashMap<i32, String> {
    use crate::schema::songs_in_gigs::dsl::*;
    let query = songs_in_gigs
        .select(SongInGig::as_select())
        .order(played_at.asc())
        .filter(gig_id.eq(gig_id_i32));
    let maybe_songs_in_gigs = query.load::<SongInGig>(connection);
    let Ok(songs_in_gig) = maybe_songs_in_gigs else {
        return HashMap::new();
    };
    let list = songs_in_gig
        .iter()
        .map(|sig| (sig.song_id, format!("{}", sig.played_at.as_ref().unwrap())));
    HashMap::from_iter(list)
}

pub fn delete_all_songs_in_gigs(connection: &mut SqliteConnection) -> () {
    use crate::schema::songs_in_gigs::dsl::*;
    let _ = diesel::delete(songs_in_gigs).execute(connection);
}

pub fn save_new_gig(new_gig: NewGig, connection: &mut SqliteConnection) -> () {
    use crate::schema::gigs;

    diesel::insert_into(gigs::table)
        .values(&new_gig)
        .execute(connection)
        .expect("Error saving new gig");
}

pub fn is_default_gig(gig_id: i32, connection: &mut SqliteConnection) -> bool {
    use crate::schema::gigs::dsl::id;
    let query = Gig::query().filter(id.eq(gig_id));

    let maybe_gig = query.first::<Gig>(connection);

    let Ok(gig) = maybe_gig else {
        return false;
    };
    gig.default_gig == 1
}

pub fn all_played_songs(connection: &mut SqliteConnection) -> HashMap<i32, Vec<String>> {
    use crate::schema::songs_in_gigs::dsl::*;
    let query = songs_in_gigs
        .select(SongInGig::as_select())
        .order(played_at.asc());
    let maybe_songs_in_gigs = query.load::<SongInGig>(connection);
    let Ok(songs_in_gig) = maybe_songs_in_gigs else {
        return HashMap::new();
    };
    let mut result: HashMap<i32, Vec<String>> = HashMap::new();
    for sig in songs_in_gig {
        result
            .entry(sig.gig_id)
            .or_default()
            .push(format!("song-{}", sig.song_id));
    }
    debug!("all_played_songs: result = {:?}", result);
    result
}
