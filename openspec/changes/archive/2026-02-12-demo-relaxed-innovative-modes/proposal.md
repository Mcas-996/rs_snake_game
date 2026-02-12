## Why

The current repository only provides core logic scaffolding and does not deliver a playable graphical Snake experience. This change is needed now to make the demo visually interactive in a GUI while preserving the relaxed training loop and innovation-focused mode design.

## What Changes

- Deliver a playable desktop GUI Snake demo with real-time board rendering, keyboard input, and run lifecycle feedback.
- Provide GUI navigation for mode selection, including `practice`, `challenge`, `experimental`, and `invincible`, while keeping one shared map rule set.
- Keep sudden death in mortal modes and provide an optional replay toggle in settings.
- Keep invincible behavior as non-terminal collision with safe reposition, plus a dedicated invincible leaderboard.
- Persist invincible cumulative length and use permanent unlock thresholds (`15/40/80/140/...`) for experimental tools.
- Provide a pre-run experimental loadout UI with exactly three slots and mixed tool categories (control-assist and rule-modifying).
- Show mode-scoped leaderboard entries with run metadata (mode and loadout summary).
- Explicitly exclude literary milestone achievements and daily/weekly mission systems.

## Capabilities

### New Capabilities
- `gui-runtime-and-navigation`: Defines desktop window lifecycle, board rendering, keyboard control handling, and screen flow for menu/settings/leaderboard/run states.
- `mode-system-and-rules`: Defines practice/challenge/experimental/invincible mode behaviors, shared map policy, death/reposition semantics, and scoring boundaries.
- `invincible-length-progression`: Defines persistent cumulative length tracking in invincible mode and threshold-based permanent unlock logic.
- `experimental-tool-loadout`: Defines three-slot loadout behavior, GUI selection flow, and eligible tool categories for experimental gameplay.
- `leaderboard-segmentation`: Defines separate leaderboard treatment for invincible mode, challenge ranking expectations, and metadata display requirements.

### Modified Capabilities
- None.

## Impact

- Affected runtime systems: simulation loop integration with render loop, input handling, collision/death pipeline, scoring, progression persistence, and leaderboard persistence.
- Affected UX surfaces: mode select screen, in-game HUD, run summary, settings (replay toggle), loadout panel, and leaderboard views.
- Affected dependencies: GUI/windowing stack selection and integration (framework-specific choice to be finalized in design).
- Affected quality scope: automated tests for core logic plus manual GUI verification for navigation, rendering, and mode-specific run behavior.
