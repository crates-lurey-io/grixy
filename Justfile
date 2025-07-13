_default:
    cargo just --list -u

init:
    cargo tool --install

lint: lint-check

lint-check:
    cargo clippy --no-deps --all-targets --all-features -- -D warnings

lint-fix:
    cargo clippy --no-deps --all-targets --all-features --fix

format: format-fix

format-check:
    cargo fmt --all -- --check
    cargo tool taplo format --check

format-fix:
    cargo fmt --all
    cargo tool taplo format

fix:
    cargo just format-fix
    cargo just lint-fix

check:
    cargo just format
    cargo just lint

doc:
    cargo doc --all-features --no-deps --open --lib

doc-gen:
    cargo clean --doc
    cargo doc --no-deps
    echo '<meta http-equiv="refresh" content="0;url=grixy/index.html">' > target/doc/index.html
    rm target/doc/.lock

test *ARGS:
    cargo tool cargo-nextest run {{ARGS}}

test-doc *ARGS:
    cargo test {{ARGS}} --doc


test-all:
    cargo just test --all-features
    cargo just test-doc --all-features
    
coverage *ARGS:
    cargo tool cargo-llvm-cov --lib --open

coverage-gen:
    cargo tool cargo-llvm-cov --lib --lcov --output-path lcov.info
