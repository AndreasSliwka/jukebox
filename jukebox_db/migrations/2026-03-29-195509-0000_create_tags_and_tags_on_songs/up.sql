-- Your SQL goes here
CREATE TABLE tags (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name VARCHAR NOT NULL,
    unicode VARCHAR NOT NULL,
    private INTEGER not null -- actually its a boolean
);

CREATE TABLE tags_on_songs (
    song_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    primary key (song_id, tag_id)
);
