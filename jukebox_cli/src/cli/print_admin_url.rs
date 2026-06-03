use dotenvy::dotenv;
use std::env;

fn main() {
    dotenv().ok();

    let mut connection = jukebox_db::establish_single_connection();
    let Some(gig) = jukebox_db::current_gig_from_db(&mut connection) else {
        println!("No current gig found");
        return;
    };
    if gig.default_gig == 1 {
        println!("Only gig found");
        return;
    }
    let base_url_string = env::var("BASE_URL").expect("BASE_URL must be set");
    let mut base_url =
        url::Url::parse(base_url_string.as_str()).expect("BASE_URL must be a valid URI");
    base_url.set_path("admin");
    base_url.set_query(Some(format!("passkey={}", gig.admin_secret).as_str()));
    println!("admin_url = {}", base_url);
}
