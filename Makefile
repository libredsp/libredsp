.PHONY: build clean test run
build:
	cargo build

wasm:
	wasm-pack build --target web

run:
	cargo run

test:
	cargo test

clean:
	cargo clean
	rm -rf pkg
