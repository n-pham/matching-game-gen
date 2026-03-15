# STRUCTURE.md

## 📂 Directory Layout
The project follows a "Flat-Isomorphic" pattern where features are grouped by domain but separated by compilation targets.

```text
.
├── Cargo.toml            # Dependencies and Feature flags (server, web, desktop)
├── Dioxus.toml           # Dioxus CLI configuration (Asset dir, bundle ID)
├── assets/               # Card images
│   ├── Artboard 1.svg    # Card image 1
│   ├── Artboard 10.svg   # Card image 10 (Note: files 2-9 are missing)
│   ├── ...
│   └── face-down.svg     # Card face-down image
├── src
│   ├── main.rs           # Multi-target Entry Point (Server vs Client vs Desktop)
│   ├── lib.rs            # Module definitions and shared types
│   ├── models.rs         # Database & API Data Transfer Objects (DTOs)
│   ├── server.rs         # Server-only logic (In-memory storage or DB connection)
│   ├── components.rs     # UI Library (Pure View components using asset! macro)
│   ├── routes
│   │   ├── mod.rs        # Router Enum definition
│   │   └── home.rs       # Individual page views with Signal-based game logic
│   └── api.rs            # #[server] RPCs and shared shuffling logic
└── migrations/           # Optional SQLx migration files (Not needed for in-memory)
```

## 📄 File-by-File Implementation Guide

1. **Cargo.toml**
   * **Purpose**: Define feature gates (`server`, `web`, `desktop`).
   * **Instruction**: Use Dioxus 0.7. Include `dioxus-server`, `dioxus-fullstack`, `rand`, and `futures-timer`. Ensure `rand` is a required dependency for the shuffling logic.

2. **src/models.rs**
   * **Purpose**: Shared structs used by Backend and Frontend.
   * **Instruction**: Ensure types are `Copy` or `Clone` for Signal compatibility. Wrap `sqlx::FromRow` in `#[cfg(feature = "server")]`.

3. **src/api.rs**
   * **Purpose**: RPC boundary and shared game initialization.
   * **Instruction**: Implement `shuffle_cards_logic` using the `asset!` macro to ensure assets are bundled correctly for Desktop/Wasm. Hard-code the verified list of existing SVG files to avoid runtime FS failures. Map errors to `ServerFnError`.

4. **src/server.rs**
   * **Purpose**: State management.
   * **Instruction**: Implement a global `Mutex<Vec<HighScore>>` for in-memory persistence when a physical database is not used.

5. **src/routes/home.rs**
   * **Purpose**: Game engine.
   * **Instruction**: Use 12 cards (6 pairs) for a 4x3 grid. Use `use_callback` for event handlers that capture mutable signals. Wrap matching delays in `futures_timer::Delay`.

6. **src/main.rs**
   * **Purpose**: The Dispatcher.
   * **Instruction**: Use `dioxus::LaunchBuilder` to dispatch between `server`, `web`, and `desktop`. For Desktop, use `Config::new()` to allow the `asset!` macro to resolve paths.

## 🛠 Workflow for the Agent
1. **Context Priority**: Read `Cargo.toml` and `src/models.rs` before generating any logic to ensure type alignment.

2. **Feature Isolation**: When implementing server logic, never import browser-specific crates. When implementing UI, never import `sqlx` or `axum` directly.
3. **Asset Integrity**: Use the `asset!` macro with absolute-style paths (e.g., `"/assets/file.svg"`) for all images. Never use raw relative strings for assets.
4. **Feature Testing**: Verify compilation for all targets:
   * `cargo check --features server`
   * `cargo check --features web`
   * `cargo check --features desktop`
5. **Regression Guard**: Maintain static analysis tests in `src/api.rs` that verify:
   * Existence of assets on disk.
   * Absence of failing configurations (like `.with_resource_directory(current_dir)`) in `main.rs`.
   * Correct card count and pair generation.
