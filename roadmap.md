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
------>&-------------------------------------


* add basic auth




# PWA - Progressive Web App



# sketches

## songlist
```verbatim
#-------------------------------#
| Logo  TJGSE Jukebox       |...|
|-------------------------------|
| Name v           Artist       |
| Song1            Artist1      |
| Song2            Artist2      |
| Song3            Artist1      |
| Song4            Artist3      |
| Song5            Artist4      |
| Song6            Artist1      |
|                               |
|------|---------|-------|------|
| Find | Contact | Whats |            |
| Song | Andreas | This? |      |
#------|---------|-------|------#
```

```verbatim
...
|------|---------|-------|---(X)|
| ┌──────────────────────╦════╗ |
| │ statue high          ║ Go ║ |
| └──────────────────────╩════╝ |
#------|---------|-------|------#
```
