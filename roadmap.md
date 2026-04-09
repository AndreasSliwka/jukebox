# simple base server
* create webserver
* download & convert songs from setlisthelper as MD files
* serve Songs in MD files from directory as MD files
  * !!! Aborted, because lowlevel file handling is a drag and does not pay into the final goal
  * /songs/ returns json list with name, additional data and link to song as MD
  * actix_web::HttpResponse.content_type("text/markdown")  
* serve Songs in MD files out of database
  * show song list as HTML
    * Tera
  * parse chordpro file as JSON
    * !!! rust package chordpro reformates [Bb] to [Asharp], nope.
    * 
* find simple method to make chords be floating superscription, e.g. as guitartab.com does
* convert MD files to HTML
* serve Songs in MD files as HTTP files
* include static files in binary
  * !!!may be possible, but relies on build.rs an somehow breaks the language-server
* add tags to persisted Songs
  * add dedicated field for tags so songs can be filtered
  * remove my own songs from the normal listing
* add splash screen
* add admin mode, entered by adding a code phrase in the search box
* add time based auth, so the app can be switch on for a given evening with an QR code
* refactor song display to flex-rows containing flex-columns with chords and lyrics
* add songs to setlist when played (only admin)
* Chords in monospace
* fix display of inclu[D-]ed Chords
* create tags 
  * add tags and and tags_on_songs table
  * when loading a song link the song to the tags listed in the tags attribute
* change default MD format to [NamedPart]
  * Load and reexport all songs
  * add CLI to tag Songs from TSV
* create special page that shows QR codes for participants and admins

------>&-------------------------------------
* add zoom so people can adapt the font size to their device
