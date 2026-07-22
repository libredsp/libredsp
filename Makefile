.PHONY: build clean test run
build:
	cargo build

build_web:
	wasm-pack build --target web

run:
	cargo run

test:
	cargo test

clean:
	cargo clean
	rm -rf pkg