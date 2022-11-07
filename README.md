
# OpenSlide-sys
Low-level bindings to the openslide library

## Dependencies

To be able to build this crate you need to install all build dependencies of [OpenSlide](https://github.com/openslide/openslide)

You will also find a Makefile to help you install all required dependencies for Ubuntu and MacOs

## MacOs

`brew install openslide --only-dependencies`

## Ubuntu

```
apt-get install -y --no-install-recommends libwebp-dev libzstd-dev pkg-config clang
apt-get build-dep -y --no-install-recommends libopenslide0
```
