build:
	cross build --target x86_64-pc-windows-gnu

run:
	cargo run -- --input-file examples/sample.mbox --output-file examples/sample.csv

test:
	cargo watch -x test -x "clippy -- -D warnings"

install:
	cargo install -f cross

