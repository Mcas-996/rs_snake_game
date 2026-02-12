## 1. Runtime Mode Policy And Core Flow

- [x] 1.1 Add a mode enum and mode-policy interface that centralizes collision outcome, scoring strategy, and run-end behavior for `practice`, `challenge`, `experimental`, and `invincible`.
- [x] 1.2 Refactor the game loop to use one shared map simulation path and dispatch mode-specific policy hooks instead of scattered conditionals.
- [x] 1.3 Implement collision outcome handling contract (`die` vs `reposition`) and wire mortal modes to immediate run termination.
- [x] 1.4 Implement invincible reposition flow with safe-position validation and one-frame collision grace to prevent immediate re-collision.
- [x] 1.5 Add tests validating mode policy selection, mortal immediate death, and invincible non-terminal reposition behavior.

## 2. Scoring And Leaderboard Segmentation

- [x] 2.1 Introduce mode-scoped score calculators and ensure challenge ranking is driven by survival time.
- [x] 2.2 Implement leaderboard storage partitioned by mode key, including a dedicated invincible leaderboard.
- [x] 2.3 Persist run metadata (mode and loadout summary) on leaderboard entries and render metadata in leaderboard UI.
- [x] 2.4 Add validation tests that invincible scores never enter challenge/practice boards and challenge ordering honors survival time.

## 3. Invincible Progression And Unlock Engine

- [x] 3.1 Add persistent profile fields for `invincible_cumulative_length`, unlocked tool ids, replay preference, and schema versioning defaults.
- [x] 3.2 Implement cumulative length updates from invincible run growth and ensure non-invincible runs do not mutate the value.
- [x] 3.3 Implement threshold evaluator for `15/40/80/140/...` milestones with deterministic unlock reconstruction on profile load.
- [x] 3.4 Add migration/backfill handling for legacy profiles and rollback-safe schema version checks.
- [x] 3.5 Add tests for threshold crossing, idempotent permanent unlocks, and deterministic rebuild after restart.

## 4. Experimental Tool Loadout Core

- [x] 4.1 Implement tool definition schema with category metadata (`control-assist`, `rule-modifying`, `hybrid`) and compatibility validation hooks.
- [x] 4.2 Build pre-run experimental loadout validation requiring exactly three slots and snapshotting selected tools at run start.
- [x] 4.3 Enforce unlock gating so locked tools cannot be equipped and newly unlocked tools become selectable in subsequent setup flows.
- [x] 4.4 Apply active loadout effects during experimental runs and ensure effects stay fixed for the run duration.
- [x] 4.5 Add tests for slot-count enforcement, lock rejection, and runtime stability of active loadout snapshots.

## 5. GUI Runtime Foundation

- [x] 5.1 Add desktop GUI dependencies and create a windowed app shell replacing the placeholder CLI entrypoint.
- [x] 5.2 Implement screen state machine for `MainMenu`, `ModeSelect`, `Loadout`, `Running`, `Summary`, `Leaderboard`, and `Settings`.
- [x] 5.3 Implement fixed-timestep simulation loop integrated with frame-rate-independent rendering.
- [x] 5.4 Implement keyboard input capture and direction queueing for deterministic snake control.

## 6. GUI Screens And Interaction

- [x] 6.1 Build menu/settings/leaderboard screens and wire transitions between screens.
- [x] 6.2 Build experimental loadout screen with exactly three selectable slots and lock-state feedback.
- [x] 6.3 Build in-run HUD and end-of-run summary screens with mode-specific metrics.
- [x] 6.4 Implement replay-on-death visual path for mortal modes and explicit bypass path for invincible mode.

## 7. Integration QA And Demo Notes

- [x] 7.1 Wire GUI actions to core engine APIs for mode start, run updates, scoring, unlock updates, and leaderboard writes.
- [ ] 7.2 Run end-to-end manual verification across all four modes using the GUI build.
- [x] 7.3 Verify no daily/weekly mission surfaces or literary milestone achievement flows are reachable in GUI.
- [x] 7.4 Update release/demo notes to document GUI controls, screen flow, mode differences, unlock thresholds, and known trade-offs.
