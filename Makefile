.PHONY: install-deps-brew install-deps-apt

install-deps-brew:
	brew install openslide

install-deps-apt:
	sudo apt-get update
	sudo apt-get install -y --no-install-recommends libopenslide-dev
