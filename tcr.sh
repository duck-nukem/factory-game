cargo clippy || exit 1

git add -A
git commit -m "$1"

cargo test && git push || git reset --hard
