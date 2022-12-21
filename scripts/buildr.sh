#!/bin/bash
cargo build --release
shopt -s extglob
mv ./target/release/blocc_game?(.exe) ./blocc_game/env/
mv ./target/release/blocc_server?(.exe) ./blocc_server/env/