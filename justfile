default:
    just --choose

check:
    cargo clippy --all-targets

fmt:
    taplo fmt
    cargo fmt

amend: fmt check add
    git commit -a --amend --no-edit

commit message: fmt check add
    git commit -a -m "{{message}}"

add:
    git add .

push:
    #!/usr/bin/env zsh
    if read -sq "choice?Confirm push with 'y'"; then
        echo
        git push
    fi
