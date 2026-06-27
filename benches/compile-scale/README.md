# Compile-scale benchmarks

Generated crates under `quark-scale-{N}/` contain **N** `inventory::submit!` entries.

Regenerate with:

```bash
bash benches/compile-scale/generate.sh
```

Tiers: 1, 10, 100, 500, 1000, 5000, 10000.

Run cold builds:

```bash
./scripts/bench-compile-scale.sh
```

Run incremental scenarios:

```bash
./scripts/bench-incremental-scale.sh
```

These are **manual / scheduled** — not part of default CI.
