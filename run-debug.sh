#!/bin/bash

npx tailwindcss -i ./src/input.css -o ./assets/output.css --minify

if [ ! -f assets/htmx.min.js ]; then
    wget https://unpkg.com/htmx.org@1.9.4/dist/htmx.min.js -O assets/htmx.min.js
fi

if [ ! -f assets/hyperscript.min.js ]; then
    wget https://unpkg.com/hyperscript.org@0.9.9/dist/_hyperscript.min.js -O assets/hyperscript.min.js
fi

cp -rf assets/ target/assets/

RUST_LOG=info cargo run

