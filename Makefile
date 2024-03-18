.PHONY: install-deps-macos-openslide3 install-deps-macos-openslide4 install-deps-ubuntu-openslide3 install-deps-ubuntu-openslide4

install-deps-macos-openslide3:
	brew update
	curl https://raw.githubusercontent.com/Homebrew/homebrew-core/e6e41a54ec4d05000c1b95e515c85adb6f8f35af/Formula/o/openslide.rb > openslide.rb
	HOMEBREW_NO_INSTALLED_DEPENDENTS_CHECK=1 brew install --overwrite --formula openslide.rb

install-deps-macos-openslide4:
	brew update
	curl https://raw.githubusercontent.com/Homebrew/homebrew-core/f92e0a24754ed265ff7a032b89b336dd612e5559/Formula/o/openslide.rb > openslide.rb
	HOMEBREW_NO_INSTALLED_DEPENDENTS_CHECK=1 brew install --overwrite --formula openslide.rb

install-deps-ubuntu-openslide3:
	sudo apt-get update
	sudo apt-get install -y --no-install-recommends libopenslide-dev=3.*

install-deps-ubuntu-openslide4:
	sudo apt-get update
	sudo apt install -y --no-install-recommends software-properties-common gnupg
	sudo add-apt-repository -y ppa:openslide/openslide
	sudo apt-get update
	sudo apt install -y --no-install-recommends libopenslide-dev=4.* libdicom-dev=1.* libwebp-dev=1.*

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
