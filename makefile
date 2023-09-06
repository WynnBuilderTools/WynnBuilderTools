default: build

build:
	cargo build --target x86_64-pc-windows-gnu --bin builder
	cargo build --target x86_64-pc-windows-gnu --bin search_item