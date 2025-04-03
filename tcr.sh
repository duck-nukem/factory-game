cargo clippy || exit 1

cargo test && git add -A && git commit -m "$1" && git push || git reset --hard
