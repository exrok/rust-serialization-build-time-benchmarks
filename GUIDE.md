# Benchmark Guide

## Prerequisites

- Linux with `perf stat` available
- Rust toolchain
- `gnuplot` (for per-scenario SVG charts)
- `uv` + Python (for aggregate PNG charts, in `aggregate_graph/`)

## Quick Start

```bash
# Build the tool
cargo build

# Run all libraries, all scenarios, all tasks
cargo run -- run

# Generate reports from collected results
cargo run -- report
```

## Commands

Benchy uses three subcommands:

| Command  | Description                                                       |
| -------- | ----------------------------------------------------------------- |
| `run`    | Run benchmarks and save per-library results to `results/`         |
| `report` | Generate markdown reports and charts from collected results       |
| `verify` | Check that social task produces identical output across libraries |

## Running Benchmarks

### Tasks

There are three benchmark tasks:

| Task           | Description                                                                                |
| -------------- | ------------------------------------------------------------------------------------------ |
| `synthetic`    | N random structs with random field types (default: 75). Tests raw derive macro throughput. |
| `social`       | Realistic data model (users, posts, comments, reactions). Tests real-world-like code.      |
| `multi-format` | JSON + binary serialization. Only libraries supporting both formats.                       |

Select tasks with `--task` (default: `all`):

```bash
cargo run -- run --task synthetic
cargo run -- run --task social
cargo run -- run --task multi-format
cargo run -- run --task all
```

### Libraries

Available libraries: `jsony`, `serde`, `nanoserde`, `miniserde`, `midiserde`, `merde`, `musli`, `facet`, `sonic`.

```bash
# Specific libraries (comma-separated)
cargo run -- run --lib jsony,serde

# All libraries (default when --lib is omitted)
cargo run -- run
```

Not all libraries support every task. The tool skips unsupported combinations automatically.

### Scenarios

| Scenario      | Description                                                                |
| ------------- | -------------------------------------------------------------------------- |
| `warm-build`  | Dependencies cached, only bin crate rebuilt via direct `rustc` invocation. |
| `warm-check`  | Dependencies cached, only bin crate checked via direct `rustc`.            |
| `clean-build` | Full `cargo build` with empty target directory.                            |
| `runtime`     | Execute the built binary with JSON input and high iteration count.         |

```bash
cargo run -- run --scenario warm-build,warm-check
cargo run -- run --scenario clean-build --profile debug,release
cargo run -- run --scenario warm-build --incremental disabled,postfix
```

### Profile and Incremental Filters

Profiles: `debug`, `release`, `release-lto`, `release-lto-native`

Incremental modes: `disabled`, `unchanged`, `postfix`, `prefix`, `type-transform`

These filter which sub-scenarios run within `warm-build`, `warm-check`, `clean-build`, and `runtime`:

```bash
# Only debug warm builds with postfix incremental
cargo run -- run --scenario warm-build --profile debug --incremental postfix
```

### Struct Count (Synthetic Task)

```bash
cargo run -- run --task synthetic --count 200
```

## Results Directory

Each library's results are saved as individual JSON files under `--results-dir` (default: `results/`):

```
results/
  synthetic/
    jsony.json
    serde.json
    nanoserde.json
    baseline.json
  social/
    jsony.json
    serde.json
    baseline.json
  multi-format/
    jsony.json
    serde.json
```

Each file contains a single library entry: `["jsony", [["jsony", "0.1.9"]], [[scenario, perf], ...]]`

Rerunning a specific library overwrites only its file -- other libraries' results are untouched.

## Generating Reports

```bash
cargo run -- report
```

This reads all per-library JSON files from `--results-dir` (default: `results/`), then:

1. Generates `BENCH-{task}.md` with per-scenario SVG bar charts and metric tables
2. Writes SVGs to `assets/{task}/`
3. Runs `aggregate_graph/main.py` to produce `assets/{task}_aggregate.png`
4. Generates `README.md` with methodology and links to detailed results

Filter to a specific task with `--task`:

```bash
cargo run -- report --task social
```

Specify directories explicitly:

```bash
cargo run -- report --results-dir results/ --report-dir .
```

## Common Workflows

### Full benchmark run with reports

```bash
cargo run -- run
cargo run -- report
```

### Iterate on a single library

```bash
# Only rerun jsony on the social task
cargo run -- run --task social --lib jsony

# Regenerate reports (picks up all existing results)
cargo run -- report
```

### Fast compile-time check (skip runtime and clean builds)

```bash
cargo run -- run --scenario warm-build,warm-check
```

### Compare two libraries on one scenario

```bash
cargo run -- run --task synthetic --lib jsony,serde --scenario warm-build --profile debug --incremental disabled
```

### Test a local jsony checkout

```bash
cargo run -- run --lib jsony --jsony-path /path/to/jsony --task social
```

### Verify correctness across libraries

```bash
cargo run -- verify
```

This checks that the social task produces identical serialization output across all supported libraries.
