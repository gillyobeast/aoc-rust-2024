default:
    just --choose

check:
    cargo clippy --all-targets

fmt:
    taplo fmt
    cargo fmt

amend:
    git commit -a --amend --no-edit

commit message: fmt
    git commit -a -m "{{message}}"

push:
    git push
