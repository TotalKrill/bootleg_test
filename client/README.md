# wasm client bevy

start by:

    cargo build --target wasm32-unknown-unknown --release; wasm-bindgen --out-dir wasm --target web target/wasm32-unknown-unknown/release/client.wasm; basic-http-server .

server client should now be reachable on localhost:4000

    base64 /dev/urandom | head -c 1095 > textfile.txt
