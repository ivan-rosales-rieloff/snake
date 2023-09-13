REM wasm-pack build --target web
trunk build --release
mkdir .\site
copy .\dist .\site