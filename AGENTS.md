# Repository Guidelines

## Project Structure & Module Organization
The codebase is a single Rust crate.
- `src/main.rs`: desktop entrypoint.
- `src/gui.rs`: `macroquad` window config, rendering, input, and screen state flow.
- `src/lib.rs`: shared game domain logic (modes, policies, progression, leaderboards) and core tests.
- `docs/`: feature/demo notes.
- `openspec/`: spec artifacts and change history.
- `target/`: build output (generated; do not edit).

## Build, Test, and Development Commands
Use Cargo from the repository root:
- `cargo run`: builds and launches the Snake GUI window.
- `cargo test`: runs unit tests in `src/lib.rs` and `src/gui.rs`.
- `cargo check`: fast compile validation without producing a binary.
- `cargo fmt`: applies standard Rust formatting.
- `cargo clippy --all-targets --all-features -D warnings`: strict lint pass before opening a PR.

## Coding Style & Naming Conventions
- Follow Rust defaults with `rustfmt` (4-space indentation, trailing commas where applicable).
- Naming: `snake_case` for functions/modules/variables, `PascalCase` for structs/enums, `SCREAMING_SNAKE_CASE` for constants.
- Keep simulation and policy logic in `src/lib.rs`; keep UI and input concerns in `src/gui.rs`.
- Prefer small, explicit enums/structs and focused `impl` blocks over large mixed-responsibility functions.

## Testing Guidelines
- Framework: Rust built-in test framework (`#[test]`, `#[cfg(test)]`).
- Keep tests near the code they validate (current pattern in `src/lib.rs` and `src/gui.rs`).
- Name tests by behavior, e.g. `challenge_mode_ranks_by_survival_ticks`.
- No numeric coverage gate is configured; add/adjust tests for every gameplay rule or state-transition change.

## Commit & Pull Request Guidelines
- Existing history uses short informal subjects; for new work, use clear imperative messages with scope, e.g. `feat(gui): add mode summary panel`.
- Keep commits atomic and reference OpenSpec tasks when relevant.
- PRs should include:
  - concise summary of behavior changes,
  - validation steps and command output summary (`cargo test`, `cargo clippy`),
  - screenshots/GIFs for UI-visible changes,
  - linked issue or OpenSpec change path.

## OpenSpec Notes
If a change is spec-driven, update artifacts under `openspec/changes/...` alongside code so implementation and specs remain aligned.

## Please try your best to reply in Chinese.
