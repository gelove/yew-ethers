.PHONY: run build

NOW = $(shell date -u '+%Y%m%d%I%M%S')
GIT_COUNT = $(shell git rev-list --all --count)
GIT_HASH = $(shell git rev-parse --short HEAD)

all: run

dev:
	cd frontend && trunk serve --open

# update crate
update:
	cd frontend && cargo update -p ethers 

# install contracts dependencies
install:
	cd contracts && forge install

config:
	cd contracts && forge config --basic

forge-build:
	forge build --root ./contracts/ -w

test:
	forge test --root ./contracts/ --ffi -vvv -w

bind:
	forge build --root ./contracts/
	forge bind --root ./contracts --bindings-path ./frontend/bindings --crate-name bindings

run:
	RUST_BACKTRACE=1 cargo tauri dev

build:
	cargo tauri build --target x86_64-apple-darwin
