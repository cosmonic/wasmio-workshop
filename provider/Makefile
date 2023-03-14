CAPABILITY_ID = "wasmcloud:keyvalue"
NAME = "My Keyvalue"
VENDOR = "Acme"
PROJECT = mykeyvalue
VERSION ?= $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[] .version' | head -1)
REVISION = 0

include ./provider.mk

test:
	cargo test -- --nocapture
