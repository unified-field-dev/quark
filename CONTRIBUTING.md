# Contributing to Quark

Thank you for your interest in contributing. This project is an early release (v0.1.x); small, focused changes are easiest to review.

## Prerequisites

- **Stable Rust** — the workspace uses stable via [`rust-toolchain.toml`](rust-toolchain.toml).

## Verify locally

Run these before opening a pull request:

```bash
cargo test
cargo doc --no-deps
```

Optional:

```bash
cargo bench --bench runtime -- --sample-size 10
```

## Pull request expectations

- Keep diffs focused; one logical change per PR when possible.
- Run the verify commands above and ensure CI would pass.
- Update docs, rustdoc, or [`docs/PERFORMANCE_STUDY.md`](docs/PERFORMANCE_STUDY.md) when behavior or benchmark baselines change.
- Do not commit secrets, `.env` files, or compiled binaries.

## Benchmarks

Compile-time and runtime benchmarks live under `benches/` and `scripts/`. See [`docs/profiling.md`](docs/profiling.md) for run commands and [`docs/PERFORMANCE_STUDY.md`](docs/PERFORMANCE_STUDY.md) for baseline interpretation.

Large compile-scale tiers (5000/10000) are manual only — not required for every PR.

## Code of conduct

This project follows the [Contributor Covenant](CODE_OF_CONDUCT.md). By participating, you agree to uphold it.

## Security

Do not open public issues for security vulnerabilities. See [SECURITY.md](SECURITY.md).
