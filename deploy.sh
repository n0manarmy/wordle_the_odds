#!/bin/bash
wasm-pack build --release --target web
cp -r index.html interface.js style.css pkg /home/user/public/

echo run ""firebase deploy"" from ~/
