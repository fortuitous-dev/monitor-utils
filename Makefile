BINARY_PATH := /opt/bin
build:
	cargo build

release:
	cargo build -r

clean:
	cargo clean

install:
	cargo install --path disk_space
	cp $(HOME)/.cargo/bin/disk_space $(BINARY_PATH)

doc:
	cargo doc

test: install
	export RUST_TEST_NOCAPTURE=1; cargo test --release
