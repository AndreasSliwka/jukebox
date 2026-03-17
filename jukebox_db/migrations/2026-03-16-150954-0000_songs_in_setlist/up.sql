-- Your SQL goes here
CREATE TABLE songs_in_setlist (
    song_id INTEGER NOT NULL,
    setlist_id INTEGER NOT NULL,
    played_at TEXT DEFAULT NULL,
    FOREIGN KEY (song_id) REFERENCES song(id),
    FOREIGN KEY (setlist_id) REFERENCES setlist(id),
    primary key (song_id, setlist_id)

)
