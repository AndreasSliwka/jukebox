// @generated automatically by Diesel CLI.

diesel::table! {
    gigs (id) {
        id -> Integer,
        name -> Text,
        location -> Text,
        date_start -> Text,
        date_end -> Text,
        notes -> Nullable<Text>,
        admin_secret -> Text,
    }
}

diesel::table! {
    songs (id) {
        id -> Integer,
        title -> Text,
        artist -> Text,
        serialized_chord_pro -> Text,
    }
}

diesel::table! {
    songs_in_gigs (song_id, gig_id) {
        song_id -> Integer,
        gig_id -> Integer,
        played_at -> Nullable<Text>,
    }
}

diesel::table! {
    tags (id) {
        id -> Integer,
        name -> Text,
        unicode -> Text,
        private -> Integer,
    }
}

diesel::table! {
    tags_on_songs (song_id, tag_id) {
        song_id -> Integer,
        tag_id -> Integer,
    }
}

diesel::joinable!(songs_in_gigs -> gigs (gig_id));

diesel::allow_tables_to_appear_in_same_query!(gigs, songs, songs_in_gigs, tags, tags_on_songs,);
