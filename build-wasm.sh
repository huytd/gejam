export EMCC_CFLAGS="-g4"
cargo build --target=wasm32-unknown-emscripten
rm -rf out
mkdir out
cp index.html out/index.html
cp target/wasm32-unknown-emscripten/debug/*.* out/
cp target/wasm32-unknown-emscripten/debug/deps/gejam-*.* out/
rm out/gejam-*.js
