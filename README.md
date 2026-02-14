# Snake GUI Demo

A desktop Snake demo written in Rust with a shared simulation core and four gameplay modes:

- `practice`
- `challenge`
- `experimental`
- `invincible`

The project uses `macroquad` for native windowing, rendering, and keyboard input.

## Features

- Real-time desktop GUI with fixed-timestep simulation.
- Deterministic direction queue input.
- Mode-specific collision and scoring policy on top of one shared map ruleset.
- Replay-on-death toggle for mortal modes (`practice`, `challenge`, `experimental`).
- Invincible reposition behavior with no death on collision.
- Mode-scoped leaderboards with run metadata (`mode`, `survival_ticks`, `loadout`).
- Persistent invincible progression and threshold-based unlocks (`15 / 40 / 80 / 140 / ...`).
- Experimental mode with exactly 3 pre-run loadout slots.

## Requirements

- Rust toolchain (latest stable recommended)
- Windows/macOS/Linux with desktop graphics support

## Quick Start
### install 
View the release page.
### run
```bash
cargo run
```

This opens the game window directly.

## Controls

- `Arrow Keys` / `WASD`: Navigate menus and control movement
- `Enter` / `Space`: Confirm
- `Esc` / `Backspace`: Back
- During run, movement inputs are queued and consumed one per simulation tick
- During run, `Esc` ends the current run and shows summary

## Modes

- `practice`: Immediate death on fatal collision
- `challenge`: Immediate death; leaderboard ranking prioritizes survival time
- `experimental`: Immediate death; requires a valid 3-slot unlocked loadout
- `invincible`: Collision repositions snake instead of ending run

## Testing

```bash
cargo test
```

The test suite covers core rules, progression, loadout constraints, and GUI state/input flow.

## Project Layout

- `src/lib.rs`: Core domain model, policies, scoring, progression, leaderboard logic
- `src/gui.rs`: GUI app state machine, rendering, input handling, fixed-step run loop
- `src/main.rs`: Windowed app entrypoint
- `docs/demo-relaxed-innovative-modes.md`: Demo/release notes
- `openspec/changes/demo-relaxed-innovative-modes/tasks.md`: OpenSpec task tracking

## OpenSpec Status

Current implementation is near complete for change `demo-relaxed-innovative-modes` with one remaining manual verification task (`7.2`, end-to-end GUI verification across all modes).

## Tired of Compiling? 
If you're a Mac user who finds brew install too 'uncivilized', please donate $99 to my Apple Developer Fund. Help me bring this game to the App Store so you can enjoy it with a single click, just as Steve intended.
