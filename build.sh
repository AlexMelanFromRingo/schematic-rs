#!/bin/bash
set -e

targets=(
  i686-pc-windows-gnu
  x86_64-pc-windows-gnu
  x86_64-unknown-linux-gnu
)

for target in "${targets[@]}"; do
  echo "Building for $target"
  cargo build --target "$target" --release
done
