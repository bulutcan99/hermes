# zero2prod

## cargo install cargo-watch
-> cargo watch -x check
-> cargo watch -x check -x test -x run

## cargo install cargo-tarpaulin
cargo tarpaulin --ignore-tests

## rustup component add clippy
cargo clippy
cargo clippy -- -D warnings

## rustup component add rustfmt
cargo fmt -- --check

## cargo install cargo-audit
cargo audit