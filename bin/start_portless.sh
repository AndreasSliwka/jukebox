#!/bin/bash
set -x
set -e
cd `git rev-parse --show-toplevel`
bin/product_static_jukebox_css.sh
cd ./jukebox_server
portless run cargo run --bin jukebox_server
cd ..
git grep TODO:
