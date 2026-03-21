-- Your SQL goes here
CREATE TABLE songs_in_gigs (
    song_id INTEGER NOT NULL,
    gig_id INTEGER NOT NULL,
    played_at TEXT DEFAULT NULL,
    FOREIGN KEY (song_id) REFERENCES song(id),
    FOREIGN KEY (gig_id) REFERENCES gigs(id),
    primary key (song_id, gig_id)

)
