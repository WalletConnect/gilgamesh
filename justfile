binary-crate            := "."

export JUST_ROOT        := justfile_directory()

# Default to listing recipes
_default:
  @just --list --list-prefix '  > '

# Open project documentation in your local browser
docs: (_build-docs "open" "nodeps")
  @echo '==> Opening documentation in system browser'

# Fast check project for errors
check:
  @echo '==> Checking project for compile errors'
  cargo check

# Build service for development
build:
  @echo '==> Building project'
  cargo build

# Build project documentation
build-docs: (_build-docs "" "nodeps")

# Run the service
run: build
  @echo '==> Running project (ctrl+c to exit)'
  cargo run

# Run project test suite, skipping storage tests
test:
  @echo '==> Testing project (default)'
  cargo test

# Run project test suite, including storage tests (requires storage docker services to be running)
test-all:
  @echo '==> Testing project (all features)'
  cargo test --all-features

# Run test from project documentation
test-doc:
  @echo '==> Testing project docs'
  cargo test --doc

# Clean build artifacts
clean:
  @echo '==> Cleaning project target/*'
  cargo clean

# Build docker image
build-docker:
  @echo '=> Build gilgamesh docker image'
  docker-compose -f ./ops/docker-compose.gilgamesh.yml -f ./ops/docker-compose.storage.yml build gilgamesh

# Start gilgamesh & storage services on docker
run-docker:
  @echo '==> Start services on docker'
  @echo '==> Use run gilgamesh app on docker with "cargo-watch"'
  @echo '==> for more details check https://crates.io/crates/cargo-watch'
  docker-compose -f ./ops/docker-compose.gilgamesh.yml -f ./ops/docker-compose.storage.yml up -d

# Stop gilgamesh & storage services on docker
stop-docker:
  @echo '==> Stop services on docker'
  docker-compose -f ./ops/docker-compose.gilgamesh.yml -f ./ops/docker-compose.storage.yml down

# Clean up docker gilgamesh & storage services
clean-docker:
  @echo '==> Clean services on docker'
  docker-compose  -f ./ops/docker-compose.gilgamesh.yml -f ./ops/docker-compose.storage.yml stop
  docker-compose -f ./ops/docker-compose.gilgamesh.yml -f ./ops/docker-compose.storage.yml rm -f

# Start storage services on docker
run-storage-docker:
  @echo '==> Start storage services on docker'
  docker-compose -f ./ops/docker-compose.storage.yml up -d

# Stop gilgamesh & storage services on docker
stop-storage-docker:
  @echo '==> Stop storage services on docker'
  docker-compose -f ./ops/docker-compose.storage.yml down

# Clean up docker storage services
clean-storage-docker:
  @echo '==> Clean storage services on docker'
  docker-compose -f ./ops/docker-compose.storage.yml stop
  docker-compose -f ./ops/docker-compose.storage.yml rm -f

# List services running on docker
ps-docker:
  @echo '==> List services on docker'
  docker-compose -f ./ops/docker-compose.gilgamesh.yml -f ./ops/docker-compose.storage.yml ps

# Run project test suite on docker containers
test-docker:
  @echo '==> Run tests on docker container'
  docker-compose -f ./ops/docker-compose.storage.yml -f ./ops/docker-compose.test.yml run --rm gilgamesh-test

run-jaeger:
  @echo '==> Run opentelemetry jaeger docker container'
  docker run --rm -p4317:4317 -p16686:16686 jaegertracing/all-in-one:latest

# Bumps the binary version to the given version
bump-version to: (_bump-cargo-version to binary-crate + "/Cargo.toml")

# Lint the project for any quality issues
lint: check fmt clippy commit-check

# Run project linter
clippy:
  #!/bin/bash
  set -euo pipefail

  if command -v cargo-clippy >/dev/null; then
    echo '==> Running clippy'
    cargo clippy --all-features --tests -- -D clippy::all -W clippy::style
  else
    echo '==> clippy not found in PATH, skipping'
    echo '    ^^^^^^ To install `rustup component add clippy`, see https://github.com/rust-lang/rust-clippy for details'
  fi

# Run code formatting check
fmt:
  #!/bin/bash
  set -euo pipefail

  if command -v cargo-fmt >/dev/null; then
    echo '==> Running rustfmt'
    cargo +nightly fmt -- --check
  else
    echo '==> rustfmt not found in PATH, skipping'
    echo '    ^^^^^^ To install `rustup component add rustfmt`, see https://github.com/rust-lang/rustfmt for details'
  fi

  if command -v terraform -version >/dev/null; then
    echo '==> Running terraform fmt'
    terraform -chdir=terraform fmt -recursive
  else
    echo '==> terraform not found in PATH, skipping'
    echo '    ^^^^^^^^^ To install see https://developer.hashicorp.com/terraform/downloads'
  fi

# Run commit checker
commit-check:
  #!/bin/bash
  set -euo pipefail

  if command -v cog >/dev/null; then
    echo '==> Running cog check'
    cog check --from-latest-tag
  else
    echo '==> cog not found in PATH, skipping'
    echo '    ^^^ To install `cargo install --locked cocogitto`, see https://github.com/cocogitto/cocogitto for details'
  fi

# Update documentation with any changes detected
update-docs: (_regenerate-metrics "docs/Metrics.md")

# Build project documentation
_build-docs $open="" $nodeps="":
  @echo "==> Building project documentation @$JUST_ROOT/target/doc"
  @cargo doc --all-features --document-private-items ${nodeps:+--no-deps} ${open:+--open}

# Update the metrics documentation with current metrics
_regenerate-metrics file temp=`mktemp`: build
  @echo '==> Regenerating metrics to @{{file}}'
  @cd scripts && ./metrics-apply.awk <(./metrics-fetch.sh | ./metrics-doc.pl | ./metrics-format.pl) < $JUST_ROOT/{{file}} > {{temp}}
  @mv -f {{temp}} {{file}}

# Bump the version field of a given Cargo.toml file
_bump-cargo-version version file temp=`mktemp`:
  @echo '==> Bumping {{file}} version to {{version}}'
  @perl -spe 'if (/^version/) { s/("[\w.]+")/"$version"/ }' -- -version={{version}} < {{file}} > {{temp}}
  @mv -f {{temp}} {{file}}

restart-gilgamesh-docker:
  @echo '==> Restart gilgamesh service on docker'
  docker-compose up -d --build --force-recreate --no-deps gilgamesh
