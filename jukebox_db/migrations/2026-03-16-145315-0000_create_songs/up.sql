-- Your SQL goes here
create table songs (
   id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
   title VARCHAR NOT NULL,
   artist VARCHAR NOT NULL,
   tags TEXT NOT NULL,
   markdown TEXT NOT NULL,
   serialized_chord_pro TEXT NOT NULL
)
