#!/usr/bin/env bash
packages=" \
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
time for package in $packages ; do
  cargo fmt --package "$package" -- --check
done
time cargo clippy --workspace --exclude bench --all-targets --all-features -- -D clippy::pedantic
time cargo test --workspace --exclude bench --all-targets --all-features
time cargo test --workspace --exclude bench --all-features --doc

for package in $packages ; do
  cd "$top_level_dir/$package/"
  "$top_level_dir/check-readme.sh"
done

for package in $packages; do
  (cat "$top_level_dir/$package/Cargo.toml" |grep 'publish = false' >/dev/null) && continue || true;
  cd "$top_level_dir/$package/"
  time cargo publish --dry-run "$@"
done

echo "$0 finished"
