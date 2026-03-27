use chrono::{Local, Months};
use jukebox_db::models::NewGig;
use jukebox_db::save_new_gig;

fn main() {
    let mut connection = jukebox_db::establish_single_connection();
    let iso_format = "%Y-%m-%dT%H:%M:%S";
    let now = Local::now().naive_local();
    let then = now.checked_add_months(Months::new(1)).unwrap();
    let new_gig = NewGig {
        name: String::from("The default Gig in the Sky"),
        location: String::from("Auf der Bune am Bolzplatz"),
        date_start: now.format(iso_format).to_string(),
        date_end: then.format(iso_format).to_string(),
        admin_secret: String::from("very_secret"),
        notes: Some(String::from("Warme Jacke wär gut, weil abends wirds kühl")),
    };
    save_new_gig(new_gig, &mut connection);
}
