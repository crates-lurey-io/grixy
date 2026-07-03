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

format-fix:
    cargo fmt --all

fix:
    cargo just format-fix
    cargo just lint-fix

check:
    cargo just format-check
    cargo just lint
    cargo just doc-check

bench *ARGS:
    cargo bench --all-features {{ARGS}}

profile *ARGS:
    cargo tool flamegraph --bench {{ARGS}} -- --bench

doc:
    cargo doc --all-features --no-deps --open --lib

doc-check:
    RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features

semver-checks:
    # No --baseline-version pin: previously published 0.6.0-alpha.* releases all depend on a
    # loose (non-exact) ixy requirement, so cargo-semver-checks re-resolves their `ixy` dependency
    # against whatever is newest on crates.io when building the baseline - which breaks the
    # instant ixy publishes another breaking prerelease in the same 0.6.0 track (as happened
    # going from ixy 0.6.0-alpha.5 to 0.6.0-alpha.9). Letting cargo-semver-checks auto-select the
    # baseline compares against the latest actual stable release instead, which isn't affected.
    cargo tool cargo-semver-checks

msrv:
    cargo tool cargo-hack check --rust-version --workspace --all-targets --all-features --ignore-private

doc-gen:
    cargo clean --doc
    RUSTDOCFLAGS="-D warnings" cargo doc --all-features --no-deps
    echo '<meta http-equiv="refresh" content="0;url=grixy/index.html">' > target/doc/index.html
    rm target/doc/.lock

test *ARGS:
    cargo tool cargo-nextest run {{ARGS}}

test-doc *ARGS:
    cargo test {{ARGS}} --doc --all-features

test-all:
    cargo just test --all-features
    cargo just test-doc
    
coverage *ARGS:
    cargo tool cargo-llvm-cov --lib --open --all-features

coverage-gen:
    cargo tool cargo-llvm-cov --lib --lcov --all-features --output-path lcov.info
