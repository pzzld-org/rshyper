# Repository agent contract — rshyper

This file governs work in `pzzld-org/rshyper`. Repository code/PR/checks are engineering authority; [Puzzled Linear rshyper](https://linear.app/pzzld/project/rshyper-6605c340fb75) tracks objectives, and the connected Silo `rshyper` project holds formal semantics and dated handoffs. Stored notes are not runtime proof.

## Read before modifying
1. Verify `git remote -v`, branch, HEAD, worktrees, dirty/untracked state and merge-base; preserve unknown work. Read [PR #100](https://github.com/pzzld-org/rshyper/pull/100) and `.artifacts/docs/v0.2.0-handoff.md`. `main` holds the latest release, v0.2.0, and work lands on a version-named working patch branch. Never overwrite/force-push or merge without explicit authorization.
2. Read the four-crate graph: `rshyper` public facade, `core` semantics, `hmap` representation, `algo` kernels. Preserve general-purpose semantics; do not add a required database, network, agent or Chaos runtime.
3. Read the Silo semantic contract before making claims about graph IDs, incidence multiplicity, directed edge tails/heads, graph-level attributes, projections or shortest paths. Name the exact supported semantics; reject unsupported operations explicitly.

## Patch priorities
- M0: [#104](https://github.com/pzzld-org/rshyper/issues/104) ordered iterators; [#105](https://github.com/pzzld-org/rshyper/issues/105) invariant-safe mutations; [#108](https://github.com/pzzld-org/rshyper/issues/108) CI, feature matrix, toolchains, release profiles.
- M1: iterator and model contracts. M2: [#106](https://github.com/pzzld-org/rshyper/issues/106) algorithm oracle corpus and [#110](https://github.com/pzzld-org/rshyper/issues/110) benchmarked reverse incidence. M3: [#107](https://github.com/pzzld-org/rshyper/issues/107) opt-in CLI/TUI and [#95](https://github.com/pzzld-org/rshyper/issues/95) plot umbrella. M4: optional content-addressed adapters.
- v0.2.0 shipped from PR #100. Record public API, SemVer and wire-format changes in each PR (#111); never silently relabel history.

## Development reproducibility
The pre-sprint source had `rust-toolchain.toml` set to floating `stable`, Nix using floating `rust-bin.stable.latest.default`, `CARGO_HOME=$PWD/.cargo`, release `opt-level=0`, Rust CI triggered only on PR synchronize, and Nix CI allowed failure. **Do not claim deterministic builds until these are repaired and witnessed.** Select a verified, fixed Rust version compatible with declared MSRV; align rustup/Nix/CI/docs/locks, and avoid repo-local mutable Cargo caches without reason. Keep `flake.lock`, `Cargo.lock`, package metadata and platform/target evidence under source custody.

Run and record actual outcomes of `cargo fmt --all -- --check`, `cargo test --workspace --locked`, `cargo test --workspace --doc --locked`, `cargo clippy --workspace --all-targets --locked -- -D warnings`, `cargo doc --workspace --no-deps --locked`, `nix flake check --no-write-lock-file` and `nix build --no-write-lock-file` when applicable. Distinguish stable/default/full, nightly, no_std+alloc, Rayon, WASM/WASI configurations; stable all-features may activate nightly and is not automatically valid. Avoid `|| true` or `continue-on-error` for release-required checks. Baseline failures are not evidence of repair.

## Agent waves and acceptance
For limited Claude Code quota: Opus root owns decisions/integration/acceptance; Haiku waves perform small independent audits, red/green fixtures and disjoint files. Declare worktree and file ownership; do not spawn overlapping writers or untracked branch fanout. Correctness before broad algorithms/UI. Every completed slice records exact commit, minimal diff, negative case, deterministic test command/result, outstanding blockers, and links to owning GitHub issue and Puzzled Linear task. Update README/QUICKSTART/CONTRIBUTING and this file with verified behavior. No release, tag, publish, merge, hidden credential, or destructive cleanup without separate authorization.
