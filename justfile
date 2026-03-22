default: lint test

build:
  cargo build

lint:
  cargo fmt --check
  cargo clippy -- -D warnings

run:
  cargo run

release:
  cargo build --release

test:
  cargo test

format:
  cargo fmt

deploy: lint test release
  # todo

