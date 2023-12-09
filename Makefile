.PHONY: install-deps-brew install-deps-apt

install-deps-macos:
	brew install openslide

install-deps-ubuntu:
	sudo apt-get update
	sudo apt-get install -y --no-install-recommends libopenslide-dev

test-debug-ubuntu:
	cargo test

test-debug-macos:
	cargo build
	cargo test --features dynamic-link # cargo test with static-link fails on macos

test-release-ubuntu:
	cargo test --release

test-release-macos:
	cargo build --release
	cargo test --release --features dynamic-link # cargo test with static-link fails on macos
