default:
  @just --list

# build the library
build:
    cargo build

# check for errors
check:
    cargo check

# regenerate test snapshots
snapshot:
    cargo run --bin ara-internal-snapshot

# detect linting problems.
lint:
    cargo fmt --all -- --check
    cargo clippy

# fix linting problems.
fix:
    cargo fmt
    cargo clippy --fix --allow-dirty --allow-staged
    cargo fix --allow-dirty --allow-staged

# run all integration tests, except third-party.
test filter='--all':
    cargo test -r {{filter}}
