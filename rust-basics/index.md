# Table of contents

- Toolchain & project setup — rustup, cargo new, cargo run, cargo build, Cargo.toml
- Variables, mutability (let vs let mut), basic types (integers, floats, bool, char)
- Functions, expressions vs statements, control flow (if, loop, while, for)
- Ownership — move semantics, why it exists (the core Rust concept)
- Borrowing & references — &T, &mut T, the borrow checker rules
- Structs — defining, instantiating, methods (impl blocks)
- Enums & match — including Option<T> and Result<T, E>
- Error handling — Result, the ? operator, unwrap/expect (and why to avoid them in real code)
- Collections — Vec<T>, String vs &str, HashMap
- Traits & generics (basic level — enough to read library code, not write your own generic library)
- Modules & crates — mod, use, splitting code across files
- Closures — needed for event handlers/callbacks

- std::thread basics (if doing blocking I/O simply)
- Async basics — async/await, and picking tokio as your runtime
- Channels (mpsc) — how you'll pass events (keypresses, server responses) between threads/tasks

- How terminals actually work — raw mode, alternate screen, ANSI escapes (conceptual, crossterm handles it)
- crossterm — reading key/mouse events, terminal setup/teardown
- ratatui core loop — the render loop pattern (draw → handle event → update state → repeat)
- ratatui layouts — Layout, Constraint, splitting screen into regions
- ratatui widgets — Block, Paragraph, List, Gauge, Chart, custom widgets
- Styling — Style, Color, Modifier (bold/italic), theming
- Application state pattern — a central App struct holding all UI state, updated each loop tick

- HTTP client in Rust — reqwest (calling your Python server)
- JSON (de)serialization — serde / serde_json
- Audio input — cpal (mic capture) if you bring voice into the Rust side later
- Config/secrets — reading env vars or a config file (API keys, server URL)
- Error handling patterns at the app level — custom error enums, thiserror (optional but nice)
- Packaging/distribution — cargo build --release, cross-compilation basics if you ever target Linux/Mac/Windows binaries