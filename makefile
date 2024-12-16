default: build

build:
	cargo build --target x86_64-pc-windows-gnu --bin builder -r
	cargo build --target x86_64-pc-windows-gnu --bin search_item -r