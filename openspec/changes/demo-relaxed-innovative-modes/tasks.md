## 1. Runtime Mode Policy And Core Flow

- [ ] 1.1 Add a mode enum and mode-policy interface that centralizes collision outcome, scoring strategy, and run-end behavior for `practice`, `challenge`, `experimental`, and `invincible`.
- [ ] 1.2 Refactor the game loop to use one shared map simulation path and dispatch mode-specific policy hooks instead of scattered conditionals.
- [ ] 1.3 Implement collision outcome handling contract (`die` vs `reposition`) and wire mortal modes to immediate run termination.
- [ ] 1.4 Implement invincible reposition flow with safe-position validation and one-frame collision grace to prevent immediate re-collision.
- [ ] 1.5 Add tests validating mode policy selection, mortal immediate death, and invincible non-terminal reposition behavior.

## 2. Scoring And Leaderboard Segmentation

- [ ] 2.1 Introduce mode-scoped score calculators and ensure challenge ranking is driven by survival time.
- [ ] 2.2 Implement leaderboard storage partitioned by mode key, including a dedicated invincible leaderboard.
- [ ] 2.3 Persist run metadata (mode and loadout summary) on leaderboard entries and render metadata in leaderboard UI.
- [ ] 2.4 Add validation tests that invincible scores never enter challenge/practice boards and challenge ordering honors survival time.

## 3. Invincible Progression And Unlock Engine

- [ ] 3.1 Add persistent profile fields for `invincible_cumulative_length`, unlocked tool ids, replay preference, and schema versioning defaults.
- [ ] 3.2 Implement cumulative length updates from invincible run growth and ensure non-invincible runs do not mutate the value.
- [ ] 3.3 Implement threshold evaluator for `15/40/80/140/...` milestones with deterministic unlock reconstruction on profile load.
- [ ] 3.4 Add migration/backfill handling for legacy profiles and rollback-safe schema version checks.
- [ ] 3.5 Add tests for threshold crossing, idempotent permanent unlocks, and deterministic rebuild after restart.

## 4. Experimental Tool Loadout

- [ ] 4.1 Implement tool definition schema with category metadata (`control-assist`, `rule-modifying`, `hybrid`) and compatibility validation hooks.
- [ ] 4.2 Build pre-run experimental loadout flow requiring exactly three slots and snapshotting selected tools at run start.
- [ ] 4.3 Enforce unlock gating so locked tools cannot be equipped and newly unlocked tools become selectable in subsequent setup flows.
- [ ] 4.4 Apply active loadout effects during experimental runs and ensure effects stay fixed for the run duration.
- [ ] 4.5 Add tests for slot-count enforcement, lock rejection, and runtime stability of active loadout snapshots.

## 5. Death Presentation And Settings

- [ ] 5.1 Add a user setting to toggle replay-on-death behavior with sensible default and persistent storage.
- [ ] 5.2 Wire mortal death pipeline to either immediate restart/summary or short replay path based on toggle value.
- [ ] 5.3 Ensure invincible mode bypasses death replay path and always continues via reposition behavior.
- [ ] 5.4 Add tests for replay toggle behavior and regression coverage for sudden death pacing in mortal modes.

## 6. Integration QA And Definition Of Done

- [ ] 6.1 Run end-to-end manual verification across all four modes for mode rules, scoring, and progression updates.
- [ ] 6.2 Verify no daily/weekly mission surfaces or literary milestone achievement flows are reachable in UI.
- [ ] 6.3 Validate challenge mode accepts tool-affected runs on the same board and retains row metadata clarity.
- [ ] 6.4 Update release/demo notes documenting mode differences, unlock thresholds, and known trade-offs.
