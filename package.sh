#!/usr/bin/env bash

wasm-pack build --target no-modules --no-typescript --no-pack

cp manifest.json pkg/

zip -j -r bus-extension.xpi pkg -x '*.gitignore'
