#!/bin/bash
cd `git rev-parse --show-toplevel`
cd ./jukebox_server
cargo run --bin jukebox_server
cd ..
git grep TODO:
