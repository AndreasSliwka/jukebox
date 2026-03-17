-- Your SQL goes here
create table songs (
   id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
   title VARCHAR NOT NULL,
   artist VARCHAR DEFAULT NULL,
   lyrics_as_chordpro TEXT DEFAULT ''

)
