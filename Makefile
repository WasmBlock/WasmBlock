build:
	cargo +nightly build --release --target wasm32-unknown-unknown

deploy:
	cargo +nightly build --release --target wasm32-unknown-unknown
	cargo package
	cargo publish
