# simple base server
* create webserver
* download & convert songs from setlisthelper as MD files
* serve Songs in MD files from directory as MD files
  * !!! Aborted, because lowlevel file handling is a drag and does not pay into the final goal
  * /songs/ returns json list with name, additional data and link to song as MD
  * actix_web::HttpResponse.content_type("text/markdown")
  *
  
* serve Songs in MD files out of database
* find simple method to make chords be floating superscription, e.g. as guitartab.com does
* convert MD files to HTTP 
* serve Songs in MD files as HTTP files ?? from JSON?
* add basic auth




# PWA - Progressive Web App
