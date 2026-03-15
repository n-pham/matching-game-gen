# STRUCTURE.md

## 📂 Directory Layout
The project follows a "Flat-Isomorphic" pattern where features are grouped by domain but separated by compilation targets.

```text
.
├── Cargo.toml            # Dependencies and Feature flags
├── Dioxus.toml           # Dioxus CLI configuration
├── assets/               # Card images
│   ├── Artboard 1.svg    # Card image 1
│   ├── Artboard 2.svg    # Card image 2
│   ├── Artboard 3.svg    # Card image 3
│   ├── ...
│   └── face-down.svg     # Card face-down image
├── src
│   ├── main.rs           # Multi-target Entry Point (Server vs Client)
│   ├── lib.rs            # Module definitions and shared types
│   ├── models.rs         # Database & API Data Transfer Objects (DTOs)
│   ├── server.rs         # Server-only logic (DB connection, middleware)
│   ├── components.rs     # UI Library (Pure View components)
│   ├── routes
│   │   ├── mod.rs        # Router Enum definition
│   │   └── home.rs       # Individual page views
│   └── api.rs            # #[server] function definitions
└── migrations/           # SQLx migration files
```

## 📄 File-by-File Implementation Guide
1. **Cargo.toml**

**Purpose**: Define feature gates (`server` vs `web`).
**AI Instruction**: Use the "2026 Stable" dependency set. Ensure `dioxus` is used with `fullstack` and `router`. Include `axum` and `sqlx` as optional dependencies only enabled under the `server` feature.

2. **src/models.rs**

**Purpose**: Shared structs used by both Backend and Frontend.
**AI Instruction**: Use `serde` for all structs. Ensure types are `Copy` or `Clone` to simplify Signal usage.
Constraint: Wrap database-specific traits (like `sqlx::FromRow`) in `#[cfg(feature = "server")]` to prevent Wasm compilation errors.

3. **src/api.rs**

**Purpose**: RPC boundary using `#[server]` macros.
**AI Instruction**: Implement async functions called by the Frontend. Use `ServerFnError` for error responses.
Constraint: Function bodies must be wrapped in `#[cfg(feature = "server")]`.

4. **src/server.rs**

**Purpose**: Database pool management and Axum state.
**AI Instruction**: Apply `#[cfg(feature = "server")]` to the entire file. Implement `AppState` and SQLx connection logic. Use `sqlx::query!` for compile-time SQL validation.

5. **src/routes/mod.rs**

**Purpose**: Type-safe routing.
**AI Instruction**: Define a `Route` Enum using `#[derive(Routable)]`. Prohibit string-based navigation; use the `Route` enum exclusively.

6. **src/main.rs**

**Purpose**: The Dispatcher.
**AI Instruction**: Use `#[cfg(not(feature = "server"))]` to launch `dioxus_web` and `#[cfg(feature = "server")]` to launch the Axum server. Use `dioxus::fullstack::launch` patterns.

## 🛠 Workflow for the Agent
1. **Context Priority**: Read `Cargo.toml` and `src/models.rs` before generating any logic to ensure type alignment.

2. **Feature Isolation**: When implementing server logic, never import browser-specific crates. When implementing UI, never import `sqlx` or `axum` directly.

3. **Verification**: Check compilation for both targets independently:

   * `cargo check --features server`
   * `cargo check --features web`
