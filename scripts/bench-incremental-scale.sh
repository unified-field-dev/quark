#!/usr/bin/env bash
# Incremental compile benchmarks: +1 submit, touch unrelated, link-only rebuild.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

bash benches/compile-scale/generate.sh

run_tier() {
  local n="$1"
  local crate_dir="$ROOT/benches/compile-scale/quark-scale-$n"
  local lib="$crate_dir/src/lib.rs"
  local backup="$crate_dir/src/lib.rs.bak"

  echo "=== Incremental benchmarks for quark-scale-$n ==="

  cp "$lib" "$backup"

  echo "--- Cold lib build ---"
  cargo clean -p "quark-scale-$n" 2>/dev/null || true
  cargo build -p "quark-scale-$n" --timings 2>&1 | tail -3

  echo "--- +1 submit (append to lib.rs) ---"
  echo 'inventory::submit! { ScaleDescriptor { key: "item_extra", index: 99999 } }' >>"$lib"
  cargo build -p "quark-scale-$n" --timings 2>&1 | tail -3
  cp "$backup" "$lib"

  echo "--- Touch unrelated comment ---"
  echo "// bench touch $(date +%s)" >>"$lib"
  cargo build -p "quark-scale-$n" --timings 2>&1 | tail -3
  cp "$backup" "$lib"

  echo "--- Link-only (bin rebuild, lib unchanged) ---"
  cargo build -p "quark-scale-$n" --bins --timings 2>&1 | tail -3

  rm -f "$backup"
}

for n in 100 1000; do
  run_tier "$n"
done

echo "Done. Record results in docs/profiling.md"
