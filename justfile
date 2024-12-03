default:
    just --choose

check:
    cargo clippy --all-targets

fmt:
    taplo fmt
    cargo fmt

amend:
    git commit -a --amend
