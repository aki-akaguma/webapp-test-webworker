
all:

clean:
	cargo clean

build:
	wasm-pack build --dev --no-pack --target no-modules --out-dir ./dist/pkg --out-name wasm-worker

build-release:
	wasm-pack build --release --no-pack --target no-modules --out-dir ./dist/pkg --out-name wasm-worker

