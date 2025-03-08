#!/usr/bin/env bash
projects=" \
  jtoo \
  jtoo_derive \
  jtoo_derive_impl \
"
cd "$(dirname $0)"
top_level_dir=$(pwd)
set -e
set -x

time cargo check --workspace --exclude bench --all-targets --all-features
time cargo build --workspace --exclude bench --all-targets --all-features
time cargo fmt --all -- --check
time cargo clippy --workspace --exclude bench --all-targets --all-features -- -D clippy::pedantic
time cargo test --workspace --exclude bench --all-targets --all-features
time cargo test --workspace --exclude bench --all-features --doc

for project in $projects ; do
  cd "$top_level_dir/$project/"
  "$top_level_dir/check-readme.sh"
done

for project in $projects; do
  (cat "$top_level_dir/$project/Cargo.toml" |grep 'publish = false' >/dev/null) && continue || true;
  cd "$top_level_dir/$project/"
  time cargo publish --dry-run "$@"
done

echo "$0 finished"
