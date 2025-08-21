.PHONY: install-deps-macos-openslide3 install-deps-macos-openslide4 install-deps-ubuntu-openslide3 install-deps-ubuntu-openslide4

CARGO_BIN = ${CARGO_HOME}/bin/cargo


install-deps-macos-openslide3:
	brew update
	curl https://raw.githubusercontent.com/Homebrew/homebrew-core/e6e41a54ec4d05000c1b95e515c85adb6f8f35af/Formula/o/openslide.rb > openslide.rb
	brew tap-new AzHicham/openslide
	cp openslide.rb /opt/homebrew/Library/Taps/azhicham/homebrew-openslide/Formula/
	HOMEBREW_NO_INSTALLED_DEPENDENTS_CHECK=1 brew install AzHicham/openslide/openslide

install-deps-macos-openslide4:
	brew update
	brew install openslide

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
	${CARGO_BIN} build
	${CARGO_BIN} test --features dynamic-link # cargo test with static-link fails on macos

test-release-ubuntu:
	cargo test --release

test-release-macos:
	${CARGO_BIN} build --release
	${CARGO_BIN} test --release --features dynamic-link # cargo test with static-link fails on macos
