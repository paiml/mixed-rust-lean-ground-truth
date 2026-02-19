# Mixed Rust + Lean Ground Truth

Ground truth corpus for testing PMAT's mixed-language analysis capabilities.

## Structure

- `crates/` — Rust workspace (contract parsing, verification)
- `lean/` — Lean 4 proofs (softmax theorems, contract verification)
- `contracts/` — YAML kernel contracts linking Rust ↔ Lean

## Purpose

Validates that `pmat comply check` and `pmat rust-project-score` correctly:
- Detect `.lean` and `.rs` files in the same project
- Run CB-500 (Rust) AND CB-1050 (Lean) checks together
- Score formal verification across both Rust (Kani/Verus) and Lean (sorry/theorem)
- Handle mixed Cargo.toml + lakefile.lean project roots
