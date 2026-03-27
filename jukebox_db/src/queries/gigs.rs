use std::collections::HashMap;

use crate::models::Gig;
use crate::models::NewGig;
use crate::models::SongInGig;
use chrono::{self, Local};
use diesel::prelude::*;

pub fn current_gig_from_db(connection: &mut SqliteConnection) -> Option<Gig> {
    use crate::schema::gigs::dsl::*;
    let now = Local::now().naive_local().to_string().replace(" ", "T");
    let query = Gig::query()
        .filter(date_start.lt(now.clone()))
        .filter(date_end.gt(now));
    let maybe_gig = query.first::<Gig>(connection);
    if let Ok(gig) = maybe_gig {
        Some(gig)
    } else {
        None
    }
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

pub fn songs_played_in_gig(
    gig_id_i32: i32,
    connection: &mut SqliteConnection,
) -> HashMap<i32, Option<String>> {
    use crate::schema::songs_in_gigs::dsl::*;
    let query = songs_in_gigs
        .select(SongInGig::as_select())
        .filter(gig_id.eq(gig_id_i32));
    let maybe_songs_in_gigs = query.load::<SongInGig>(connection);
    let Ok(songs_in_gig) = maybe_songs_in_gigs else {
        return HashMap::new();
    };
    let list = songs_in_gig
        .iter()
        .map(|sig| (sig.song_id, sig.played_at.clone()));
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
