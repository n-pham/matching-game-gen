# AGENTS.md

## 🎯 Architecture: Dioxus Fullstack (2026)
**Goal:** Maximum compile-time safety and cross-platform portability (Web/Desktop) via a single Rust codebase.

---

## 🛠 Tech Stack
- **Framework:** Dioxus 0.6+ (Fullstack & Router features).
- **Reactivity:** Signal-based (`use_signal`, `use_memo`, `use_resource`).
- **Server:** Axum integration via `#[server]` macros.
- **Database:** SQLx with compile-time query verification.
- **Styling:** Tailwind CSS.

---

## 🏗 Implementation Rules

### 1. Isomorphic Data Flow
- **Boundary:** Use `#[server]` functions for all I/O. Manual REST/JSON fetch logic is prohibited.
- **Unified Types:** Share structs between UI and Backend modules.
- **Error Handling:** Use `ServerFnError` for boundaries and `Option`/`Result` for UI states. No `.unwrap()`.

### 2. Signal-Based UI
- **Fine-Grained Updates:** Use `use_signal` exclusively.
- **Async Data:** Use `use_resource` for server-side calls.
- **Side-Effect Prevention:** Never mutate state in the `rsx!` block; use `use_memo` for derived values.
- **Ownership:** Use `move` closures for event handlers to satisfy the borrow checker.

### 3. Verification & Guardrails
- **SQL Safety:** Use `sqlx::query!` macros to validate queries against the schema during compilation.
- **Strong Typing:** Use Enums for routing and internal state logic instead of strings.
- **Conditional Compilation:** Wrap server-only code (DB drivers, private keys) in `#[cfg(feature = "server")]`.

## Code Unit Test

### 1. Code Unit Test Rules
- Maintain **>80% line coverage** across the entire workspace. Tests must be categorized by execution environment to ensure the AI agent validates both Backend logic and Frontend reactivity.
- Before implementing a function, the AI agent must generate a `#[cfg(test)]` module at the bottom of the file. Use `tokio::test` for any asynchronous `#[server]` functions. Use the `mockall` crate for external service/database dependencies. Reactive Signals UI tests must verify that changing a `Signal` value updates the intended DOM element (using `dioxus-check` or manual `rsx!` inspection).

### 2. Server-Side Testing

For logic in `src/server.rs`, `src/api.rs`, and `src/models.rs`.
```bash
cargo llvm-cov --all-features --workspace --fail-under-lines 80
```

### 3. Frontend Component Testing (Wasm)

For UI logic in `src/components.rs` and `src/routes/`, since Wasm doesn't support standard LLVM instrumentation natively, the AI agent should use `wasm-bindgen-test-runner`.
```bash
wasm-pack test --node -- --all-features
```
