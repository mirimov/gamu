default: lint test

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

setup:
  podman build -t gamu-env .

build: setup
  podman run --rm -v $(pwd):/volume -w /volume gamu-env \
    sh -c "just lint test release"

clean:
    podman run --rm -v $(pwd):/volume -w /volume gamu-env cargo clean

commit message:
  git commit -am "{{message}}"

push:
  git push

deploy version: lint test release
  git tag -a v{{version}} -m "Release v{{version}}"
  git push origin v{{version}}
  @echo "🚀 Release v{{version}} initiated! Check GitHub Actions for the build."

undeploy version:
  git tag -d v{{version}}
  git push --delete origin v{{version}}