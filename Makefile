build-package:
	wasm-pack build --target web

build-npm:
	wasm-pack build --target bundler --out-dir lib