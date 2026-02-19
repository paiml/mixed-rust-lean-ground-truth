<p align="center">
  <img src=".github/mixed-hero.svg" width="800" alt="mixed-rust-lean-ground-truth">
</p>

<h1 align="center">mixed-rust-lean-ground-truth</h1>

<p align="center">
  <strong>Cross-Language Quality Analysis Corpus (Rust + Lean 4)</strong>
</p>

<p align="center">
  <a href="LICENSE">
    <img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="MIT License">
  </a>
  <img src="https://img.shields.io/badge/Rust-1.75+-orange?logo=rust" alt="Rust">
  <a href="https://lean-lang.org">
    <img src="https://img.shields.io/badge/Lean-4.15+-blue" alt="Lean 4">
  </a>
  <img src="https://img.shields.io/badge/PMAT-scored-blue" alt="PMAT Scored">
</p>

A ground-truth mixed-language corpus for validating [PMAT](https://github.com/paiml/paiml-mcp-agent-toolkit)'s ability to analyze projects that combine Rust implementations with Lean 4 formal proofs. Tests that `pmat comply check` runs both CB-500 (Rust) and CB-1050 (Lean) checks simultaneously, and that `pmat rust-project-score` correctly scores formal verification across both languages.

---

## Table of Contents

- [Overview](#overview)
- [Project Structure](#project-structure)
- [Architecture](#architecture)
- [PMAT Quality Metrics](#pmat-quality-metrics)
- [PMAT Verification](#pmat-verification)
- [Installation](#installation)
- [Usage](#usage)
- [Contract Format](#contract-format)
- [Contributing](#contributing)
- [License](#license)

## Overview

This corpus models a realistic pattern: **Rust for implementation, Lean 4 for mathematical proofs, YAML contracts as the bridge**. The softmax kernel serves as the example — a simple enough algorithm to prove, complex enough to require formal verification.

### What PMAT Validates Against This Corpus

| Capability | Test Target |
|-----------|-------------|
| Mixed Language Detection | `Cargo.toml` + `lakefile.lean` in same project |
| Dual Compliance | CB-500 (Rust) AND CB-1050 (Lean) checks together |
| Cross-Language Scoring | Formal verification across Rust (Kani/Verus) and Lean |
| Sorry Detection | 3 sorry markers in `lean/GroundTruth/Softmax.lean` |
| Workspace Analysis | Multi-crate Rust workspace + Lean subdirectory |
| Contract Linking | YAML contracts reference both Rust and Lean implementations |

## Project Structure

```
mixed-rust-lean-ground-truth/
  Cargo.toml                     # Rust workspace root
  Cargo.lock                     # Reproducible builds
  lakefile.lean                  # Lean 4 build (root level)
  lean-toolchain                 # Lean version pinning
  Makefile                       # Build automation
  crates/
    contract-parser/             # Rust: YAML contract parsing
      src/lib.rs
    verifier/                    # Rust: Verification orchestration
      src/lib.rs
  lean/
    GroundTruth/
      Softmax.lean               # Lean 4 softmax proofs (3 sorry markers)
  contracts/
    softmax-kernel-v1.yaml       # YAML contract linking Rust <-> Lean
```

## Architecture

```
  Paper                YAML Contract          Rust Implementation
  ┌─────┐            ┌──────────────┐        ┌─────────────────┐
  │ Eq. │  extract   │ equations:   │ scaffold│ fn softmax()    │
  │ (1) │ ────────>  │   softmax:   │ ──────> │   // impl       │
  └─────┘            │     ...      │        └─────────────────┘
                     │ obligations: │
                     │   - ...      │        Lean 4 Proof
                     └──────────────┘        ┌─────────────────┐
                            │      verify    │ theorem softmax  │
                            └──────────────> │   := by ...     │
                                             └─────────────────┘
```

## PMAT Quality Metrics

| Category | Score | Notes |
|----------|-------|-------|
| Formal Verification | 4.2/16 (26.2%) | 3 sorry markers reduce score |
| Code Quality | 20.0/26 (76.9%) | Clean Rust code |
| Known Defects | 20.0/20 (100%) | No unwrap/panic in production |
| Build Performance | 7.0/15 (46.7%) | Basic build config |
| Testing Excellence | 2.5/20 (12.5%) | Minimal test suite |
| Documentation | 1.0/15 (6.7%) | Needs improvement |

## PMAT Verification

```bash
# Full compliance check (runs both Rust and Lean checks)
pmat comply check -p /path/to/mixed-rust-lean-ground-truth

# Project quality score
pmat rust-project-score -p /path/to/mixed-rust-lean-ground-truth

# JSON output for CI/CD
pmat rust-project-score -p . -f json -o score.json

# Context generation (analyzes both .rs and .lean files)
pmat context -p /path/to/mixed-rust-lean-ground-truth
```

### Expected PMAT Output

**Compliance:**
- CB-500 (Rust): Warnings for missing clippy.toml, deny.toml, lints section
- CB-1050 (Lean): 3 sorry markers in Softmax.lean
- CB-1052 (Lean): Warning for 60% sorry ratio (exceeds 20% threshold)

**Scoring:**
- Formal Verification: ~26% (sorry markers penalize heavily)
- GPU/SIMD Quality: N/A (no GPU code)
- Overall: ~37% normalized

## Installation

### Prerequisites

- [Rust](https://rustup.rs) (1.75+)
- [Lean 4](https://lean-lang.org) (v4.15.0+)
- [PMAT](https://github.com/paiml/paiml-mcp-agent-toolkit) (v3.4.0+)

### Build

```bash
# Clone
git clone https://github.com/paiml/mixed-rust-lean-ground-truth.git
cd mixed-rust-lean-ground-truth

# Build Rust
cargo build

# Build Lean
lake build

# Run tests
make test
```

## Usage

This corpus is designed to be analyzed by PMAT. To validate PMAT's mixed-language capabilities:

```bash
# Verify mixed detection works
pmat comply check -p . --verbose

# Check that both language checks fire
pmat comply check -p . --failures-only

# Score the project
pmat rust-project-score -p .
```

## Contract Format

Contracts in `contracts/` follow the provable-contracts YAML schema:

```yaml
name: softmax-kernel-v1
equations:
  softmax:
    formula: "softmax(x_i) = exp(x_i) / sum(exp(x_j))"
    domain: "x in R^n, n > 0"
obligations:
  - name: output_sums_to_one
    type: invariant
    equation_ref: softmax
    description: "Output vector sums to 1.0 within epsilon"
```

## Contributing

1. All changes directly on `master` branch
2. Run `cargo build && lake build` to verify both languages compile
3. Run `pmat comply check -p .` before committing
4. Maintain exactly 3 sorry markers in `lean/GroundTruth/Softmax.lean`

## License

MIT
