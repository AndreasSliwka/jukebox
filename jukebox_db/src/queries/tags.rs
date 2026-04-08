use std::collections::HashMap;

use crate::models::{Tag, TagOnSong};
use diesel::prelude::*;

pub fn tag_by_name(name: &str, connection: &mut SqliteConnection) -> Option<Tag> {
    use crate::schema::tags::dsl;
    let maybe_tag = dsl::tags
        .filter(dsl::name.eq(name))
        .first::<Tag>(connection);
    match maybe_tag {
        Ok(tag) => Some(tag),
        Err(_) => None,
    }
}

pub fn update_tag(
    tag_id: i32,
    name: &str,
    unicode: &str,
    private: i32,
    connection: &mut SqliteConnection,
) -> Tag {
    use crate::schema::tags::dsl;

    let result = diesel::update(dsl::tags.filter(dsl::id.eq(tag_id)))
        .set((
            dsl::name.eq(name),
            dsl::unicode.eq(unicode),
            dsl::private.eq(private),
        ))
        .get_result::<Tag>(connection);
    match result {
        Ok(song) => Some(song),
        Err(_) => None,
    }
    .unwrap()
}

pub fn create_tag(
    name: &str,
    unicode: &str,
    private: i32,
    connection: &mut SqliteConnection,
) -> Tag {
    use crate::schema::tags;
    let new_tag = crate::models::NewTag {
        name,
        unicode,
        private,
    };

    diesel::insert_into(tags::table)
        .values(&new_tag)
        .returning(Tag::as_returning())
        .get_result(connection)
        .expect("Error saving new post")
}

pub fn update_or_create_tag(
    name: &str,
    unicode: &str,
    private: i32,
    connection: &mut SqliteConnection,
) -> Tag {
    if let Some(tag) = tag_by_name(name, connection) {
        update_tag(tag.id, name, unicode, private, connection)
    } else {
        create_tag(name, unicode, private, connection)
    }
}

fn required_tags() -> Vec<(&'static str, &'static str, bool)> {
    vec![
        ("English", "🇬🇧", false),
        ("German", "🇩🇪", false),
        ("Private", "😎", true),
        ("rockig", "🪨", false),
        ("Metal", "🔨", false),
        ("episch", "🛢", false),
        ("party", "🍺", false),
        ("Liebe", "💋", false),
        ("Soft", "🍦", false),
        ("XMas", "🎄", false),
        ("Children", "👶", false),
        ("60er", "[60]", false),
        ("70er", "[70]", false),
        ("80er", "[80]", false),
        ("90er", "[90]", false),
        ("00er", "[00]", false),
        ("10er", "[10]", false),
    ]
}

pub fn ensure_seed_data_for_tags(connection: &mut SqliteConnection) -> () {
    for (name, unicode, private) in required_tags() {
        let private_as_int = if private { 1 } else { 0 };
        update_or_create_tag(name, unicode, private_as_int, connection);
    }
}

pub fn all_tags_by_name(connection: &mut SqliteConnection) -> HashMap<String, (i32, String)> {
    let mut them_tags: HashMap<String, (i32, String)> = HashMap::new();

    for tag in Tag::query().load(connection).unwrap() {
        them_tags.insert(tag.name.to_lowercase(), (tag.id, tag.unicode));
    }
    them_tags
}

pub fn all_tags_by_id(connection: &mut SqliteConnection) -> HashMap<i32, (String, String, bool)> {
    let mut them_tags: HashMap<i32, (String, String, bool)> = HashMap::new();

    for tag in Tag::query().load(connection).unwrap() {
        them_tags.insert(
            tag.id,
            (tag.name.to_lowercase(), tag.unicode, tag.private == 1),
        );
    }
    them_tags
}

pub fn all_private_tag_ids(connection: &mut SqliteConnection) -> Vec<i32> {
    Tag::query()
        .load(connection)
        .unwrap()
        .iter()
        .filter(|tag| tag.private == 1)
        .map(|tag| tag.id)
        .collect()
}

pub fn set_tags_on_song(song_id: i32, tag_ids: Vec<i32>, connection: &mut SqliteConnection) {
    use crate::schema::tags_on_songs::dsl;
    diesel::delete(dsl::tags_on_songs)
        .filter(dsl::song_id.eq(song_id))
        .execute(connection)
        .unwrap();

    let tags_on_this_song: Vec<TagOnSong> = tag_ids
        .iter()
        .map(|tag_id| TagOnSong {
            song_id,
            tag_id: *tag_id,
        })
        .collect();

    diesel::insert_into(dsl::tags_on_songs)
        .values(tags_on_this_song)
        .execute(connection)
        .unwrap();
}

pub fn tags_by_song(connection: &mut SqliteConnection) -> HashMap<i32, Vec<i32>> {
    let mut tags: HashMap<i32, Vec<i32>> = HashMap::new();
    for TagOnSong { song_id, tag_id } in TagOnSong::query().load(connection).unwrap() {
        match tags.get_mut(&song_id) {
            Some(tags) => tags.push(tag_id),
            None => {
                tags.insert(song_id, vec![tag_id]);
            }
        };
    }
    tags
}
