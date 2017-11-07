export EMCC_CFLAGS="-O3"
cargo build --release --target=wasm32-unknown-emscripten
rm -rf out
mkdir out
cp index.html out/index.html
cp target/wasm32-unknown-emscripten/release/*.* out/
cp target/wasm32-unknown-emscripten/release/deps/gejam-*.* out/
rm out/gejam-*.js
