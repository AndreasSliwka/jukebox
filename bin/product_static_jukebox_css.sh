#!/bin/bash
set -x
set -e
cd `git rev-parse --show-toplevel`
tailwindcss\
   -i jukebox_server/templates/tailwind_input.css\
   -o jukebox_server/static/jukebox.css \
   -c jukebox_server/static
