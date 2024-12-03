default:
    just --choose

check:
    cargo clippy --all-targets

fmt:
    taplo fmt
    cargo fmt

amend: fmt check
    git commit -a --amend --no-edit

commit message: fmt check
    git commit -a -m "{{message}}"

push:
    git push
