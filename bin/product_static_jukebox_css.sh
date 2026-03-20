#!/bin/bash
set -x
set -e
cd /Users/andreassliwka/src/jukebox/
/opt/homebrew/bin/tailwindcss\
   -i jukebox_server/templates/tailwind_input.css\
   -o jukebox_server/static/jukebox.css
