# flc_snx — Project Sheet

> Follow this sheet top-to-bottom. Do not write implementation code until a phase is checked off. Fill in every `[TODO]` before moving on.

---

## 1. Project Identity

| Field | Value |
|-------|-------|
| Project name | `flc_snx` |
| Workspace root | `/Volumes/Working/rust/flc_snx` |
| Rust edition | `2021` |
| Resolver | `2` |
| License | `[TODO]` |
| Author | `[TODO]` |

**One-line purpose** — what does `flc_snx` do?

```
[TODO: one sentence, e.g. "A fast X that ingests Y and produces Z."]
```

**Primary users / callers**

```
[TODO]
```

**Key inputs → outputs**

```
[TODO]
```

---

## 2. Workspace Architecture (yuhuu-function)

Multi-crate workspace. All crates live together under `src/`. No circular dependencies. `core` never depends on `io-adapters`.

```
flc_snx/
├── Cargo.toml              # workspace root (Coordinator)
├── PROJECT_SHEET.md        # this file
├── src/
│   ├── core/               # pure domain logic, zero side-effects, immutable state
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   └── io-adapters/        # impure layer: DB, CLI, Cloud Functions (depends on core)
│       ├── Cargo.toml
│       └── src/main.rs     # binary entry point
├── docs/
│   └── plans/
│       └── teammates/      # TA / Owner / Directors review docs (gitignored)
├── .gitignore
└── target/                 # build output (gitignored)
```

**Dependency rule**
- `core` → no deps on other workspace crates
- `io-adapters` → depends on `core`
- Never: `core` ← `io-adapters`

---

## 3. Technical Standards (from CLAUDE.md)

| Area | Standard |
|------|----------|
| Error handling (lib crates) | `thiserror` |
| Error handling (binaries/services) | `anyhow` |
| Async / I/O streaming | `tokio` |
| Testing | Property-based with `proptest` (structural calcs) + unit tests |
| Documentation | Document the **invariants** of every public function |
| Formatting | `cargo fmt`, `cargo clippy -- -D warnings` |

---

## 4. Crate Specs (fill before coding)

### 4.1 src/core — "the engineering brain"

**Purpose** `[TODO]`

**Pure functions** (state in → new state out, no I/O)

| Function | Signature | Invariant |
|----------|-----------|-----------|
| `sum` | `sum(a: i32, b: i32) -> i32` | result fits within `i32` (no overflow handling) |

**Errors** (`thiserror` enum `CoreError`)

```
[TODO: list error variants]
```

**Tests** — `proptest` properties + unit tests

```
[TODO: list properties to verify]
```

### 4.2 src/io-adapters — impure layer

**Purpose** `[TODO]`

**Depends on:** `flc-core`

**Adapters** (DB / CLI / Cloud Functions)

| Adapter | Tech | Side-effect |
|---------|------|-------------|
| `main.rs` binary | stdout | prints to console |

**Binary entry point** `src/main.rs` — orchestrates adapters, uses `anyhow` for errors.

```
[TODO]
```

---

## 5. Implementation Phases (checklist)

### Phase 0 — Scaffold
- [x] Create `flc_snx/Cargo.toml` workspace root with `members = ["src/core", "src/io-adapters"]` + `[workspace.dependencies]`
- [x] Create `src/core/Cargo.toml`, `src/io-adapters/Cargo.toml`
- [x] Create `src/core/src/lib.rs`, `src/io-adapters/src/main.rs`
- [x] Create `.gitignore` (ignore `target/`, `docs/plans/teammates/`)
- [x] `cargo check --workspace` passes
- [x] Working example: `src/core/src/lib.rs` has `pub fn sum(a: i32, b: i32) -> i32`, called from `src/io-adapters/src/main.rs` → prints `sum(40, 2) = 42`

### Phase 1 — core (pure domain)
- [ ] Define domain entities (structs/enums) beyond `sum`
- [ ] Define `CoreError` with `thiserror`
- [ ] Implement pure functions with documented invariants
- [ ] Write unit tests + `proptest` properties
- [ ] `cargo test -p flc-core` passes

### Phase 2 — io-adapters (impure layer)
- [ ] Implement adapters (DB / CLI / functions)
- [ ] Binary `main.rs` wiring with `anyhow` error handling
- [ ] `cargo check -p flc-io-adapters` passes

### Phase 3 — Workspace wiring
- [ ] `cargo check --workspace` passes
- [ ] `cargo build --release` passes
- [ ] Run the binary end-to-end with sample input

### Phase 4 — Testing & Review
- [ ] `cargo test --workspace` all green
- [ ] `cargo clippy --workspace -- -D warnings` clean
- [ ] `cargo fmt --check` clean
- [ ] TA review (technical approach) → `docs/plans/teammates/<date>-flc_snx-ta.md`
- [ ] Owner review (requirements alignment) → `docs/plans/teammates/<date>-flc_snx-owner.md`
- [ ] Directors verification (Architecture, Security, Code Quality, Testing + domain-specific) → `docs/plans/teammates/<date>-flc_snx-verification.md`

---

## 6. Definition of Done

- [ ] All crates compile with `cargo check --workspace`
- [ ] Full test suite green: `cargo test --workspace`
- [ ] Every public function documents its invariant
- [ ] Errors use `thiserror` (lib) / `anyhow` (bin)
- [ ] Async I/O uses `tokio`
- [ ] Property-based tests cover structural calculations
- [ ] TA + Owner + Directors review docs written (not committed)
- [ ] Critical review issues resolved

---

## 7. Build Commands (run from `/Volumes/Working/rust/flc_snx`)

```bash
cargo check --workspace      # type-check all crates
cargo test --workspace       # run all tests
cargo build --release        # release build
cargo clippy --workspace -- -D warnings
cargo fmt --check
```

---

## 8. Review Documentation Location

All review outputs go to `docs/plans/teammates/` and **must not** be committed:

```
docs/plans/teammates/          # gitignored
├── team-leads/
├── ta/
├── engineers/
└── review-owners/
```

Add to `.gitignore`:

```
target/
docs/plans/teammates/
```

---

## How to use this sheet

1. Fill every `[TODO]` in Sections 1–4 (this is the design step — no code yet).
2. Get TA review + Owner review sign-off on the design before Phase 1.
3. Work through Phases 0–5 in order, checking off each item.
4. Before claiming complete, run Directors verification and resolve critical issues.
