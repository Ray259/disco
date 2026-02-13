# Architecture Specification

## IPC & Concurrency Model

### Message Passing (Frontend <-> Backend)
The application uses Tauri's asynchronous IPC bridge.
1.  **Frontend**: Invokes `window.__TAURI__.invoke(command, payload)`. Payload is serialized to JSON.
2.  **Bridge**: Tauri runtime deserializes JSON into Rust structs deriving `serde::Deserialize`.
3.  **Backend**: Commands execute on the Tokio thread pool managed by Tauri.
4.  **Response**: Rust return values `Result<T, String>` are serialized to JSON.

**Implication**: All command arguments and return types **must** implement `serde::Serialize` and `serde::Deserialize`.

### State Management
State is managed via Tauri's `State<T>` guards.
*   **Initialization**: `EncyclopediaDb` is initialized in `lib.rs` inside `tauri::Builder::setup`.
*   **Injection**: `app.manage(db)` stores the instance in the application state.
*   **Access**: Commands request `State<'_, EncyclopediaDb>`. Tauri handles the read/write locking of the state container.

## Module Resolution

resolution of commands follows the `invoke_handler!` macro in `lib.rs`.
*   **Namespace**: Commands are flat-mapped. `create_figure` is exposed directly, not as `figure::create`.
*   **Registration**: New commands must be manually added to the `generate_handler!` macro in `src-tauri/src/lib.rs`.

## Build Artifacts
*   **Frontend**: Vite bundles React to `dist/`.
*   **Backend**: Cargo compiles binary linking the webview and native Cocoa/GTK/Windows libs.
*   **Final Binary**: Contains the frontend assets embedded as a resource.
