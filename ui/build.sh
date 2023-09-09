#!/bin/bash

wasm-pack build --target no-modules

cp pkg/musmatic_ui.js html/musmatic/musmatic_ui.js

cp pkg/musmatic_ui_bg.wasm html/musmatic/musmatic_ui_bg.wasm
