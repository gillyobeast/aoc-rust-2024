default:
    just --choose

today:
    cargo today

check:
    cargo clippy --workspace --all-targets --tests -- -Dwarnings

fmt:
    taplo fmt
    cargo fmt

amend: fmt check add
    git commit -a --amend --no-edit

commit message: fmt check add
    git commit -a -m "{{ message }}"

add:
    git add .

push:
    #!/usr/bin/env zsh
    echo "Commits:"
    git rev-list --oneline --color=always origin..HEAD | sed 's/^/  - /'
    if read -sq "choice?Confirm push with 'y'"; then
        echo
        git push
    fi
