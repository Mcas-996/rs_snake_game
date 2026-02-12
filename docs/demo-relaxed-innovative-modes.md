# Demo Notes: Relaxed Innovative Modes

## GUI Controls

- Menu and screen navigation: `Arrow Keys` or `WASD`
- Confirm action: `Enter` or `Space`
- Back action: `Esc` or `Backspace`
- During run: directional input is queued and consumed one direction per simulation tick
- During run: `Esc` ends the current run and opens summary

## Screen Flow

- `MainMenu` -> `ModeSelect` -> (`Loadout` for experimental only) -> `Running` -> `Summary` -> `MainMenu`
- `MainMenu` -> `Leaderboards` -> `MainMenu`
- `MainMenu` -> `Settings` -> `MainMenu`
- `Summary` -> `Leaderboards` (optional) -> `MainMenu`

## Mode Summary

- Practice: shared map rules, immediate death on fatal collision, practice-oriented scoring.
- Challenge: shared map rules, immediate death, leaderboard ranking prioritizes survival time.
- Experimental: shared map rules, immediate death, pre-run 3-slot loadout with unlocked tools only.
- Invincible: collision never ends run; collision triggers safe reposition with one-frame grace.

## Progression And Unlocks

- Only invincible mode contributes persistent cumulative length.
- Unlock thresholds are permanent and non-spendable: `15`, `40`, `80`, `140`, then extendable.
- Unlocks are rebuilt deterministically from cumulative length on profile load.

## Leaderboards

- Entries are segmented by mode key.
- Invincible leaderboard is isolated from mortal mode boards.
- Challenge mode ordering prioritizes survival time.
- Rows include mode and loadout metadata for interpretation.

## Known Trade-offs

- Challenge flow allows tool-influenced outcomes on a shared board by design.
- Experimental mode depends on unlock progression and may block early setup until thresholds are reached.
- Reposition safety in invincible mode depends on available free tiles.
- Replay-on-death is intentionally short and only applies to mortal modes.
- Tool compatibility metadata is modeled, but tool count and compatibility depth are intentionally minimal for demo scope.

## Out Of Scope

- Daily/weekly mission systems.
- Literary milestone achievements.
- External online services.
