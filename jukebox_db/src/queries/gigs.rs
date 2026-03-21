use crate::models::Gig;
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
