build:
	cargo build --release --no-default-features --bins

clean:
	cargo clean

test:
	cargo test

udeps:
	cargo +nightly udeps --all-targets

sort-deps:
	cargo sort -w
