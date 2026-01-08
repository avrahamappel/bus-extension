#!/usr/bin/env bash

rm -rf pkg
rm bus-extension.xpi

wasm-pack build --target no-modules --no-typescript --no-pack

cp manifest.json pkg/
cp script.js pkg/

zip -j -r bus-extension.xpi pkg -x '*.gitignore'
