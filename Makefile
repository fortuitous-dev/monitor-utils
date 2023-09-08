
build:
	cargo build

release:
	cargo build -r

clean:
	cargo clean

install:
	cargo install --path disk_space

doc:
	cargo doc

test: install
	export RUST_TEST_NOCAPTURE=1; cargo test --release
