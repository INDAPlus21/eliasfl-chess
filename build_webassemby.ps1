# Make sure wasm-pack is installed: https://rustwasm.github.io/wasm-pack/installer/
wasm-pack build --target web # --release
Remove-Item .\pkg\.gitignore