#!/usr/bin/env bash
# Cold compile benchmarks for inventory::submit! scale tiers.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

bash benches/compile-scale/generate.sh

TIERS=(1 10 100 500 1000 5000 10000)

echo "=== Cold compile-scale builds (cargo build --timings) ==="
for n in "${TIERS[@]}"; do
  echo "--- quark-scale-$n ---"
  cargo build -p "quark-scale-$n" --bins --timings 2>&1 | tail -5
done

echo "Done. Record results in docs/profiling.md"
