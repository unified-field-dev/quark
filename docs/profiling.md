# Profiling and benchmarks

Benchmark baselines, interpretation, and adoption guidance: [`PERFORMANCE_STUDY.md`](PERFORMANCE_STUDY.md).

## Quick commands

```bash
cargo test
cargo bench --bench runtime
bash benches/compile-scale/generate.sh
./scripts/bench-compile-scale.sh
./scripts/bench-incremental-scale.sh
```

HTML criterion report: `target/criterion/report/index.html`

Run large criterion groups individually to avoid OOM from `Box::leak` in `register_build` loops:

```bash
cargo bench --bench runtime -- "get_hit" --sample-size 10
cargo bench --bench runtime -- "register_build/10000" --sample-size 10
```
