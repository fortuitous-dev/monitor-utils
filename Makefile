BINARY_PATH := /opt/bin
build:
	cargo build

release:
	cargo build -r

clean:
	cargo clean

~/.cargo/bin/disk_space:
	cargo install --path disk_space
	cp $(HOME)/.cargo/bin/disk_space $(BINARY_PATH)

install: ~/.cargo/bin/disk_space

doc:
	cargo doc

test: install
	export RUST_TEST_NOCAPTURE=1; cargo test --release
