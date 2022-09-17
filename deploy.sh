#!/bin/bash
echo wasm-pack build --release --target web
wasm-pack build --release --target web
echo cp -r index.html interface.js style.css pkg ./deploy/
cp -r index.html interface.js style.css pkg ./deploy/
echo (cd ./deploy/ && firebase deploy)
(cd ./deploy/ && firebase deploy)