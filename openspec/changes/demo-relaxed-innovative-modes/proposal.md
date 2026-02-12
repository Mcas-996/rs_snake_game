## Why

The current Snake demo lacks a clear product loop that matches a relaxed, practice-friendly experience while still showcasing innovation. This change defines a stable mode system and progression contract now so later implementation stays coherent and testable.

## What Changes

- Introduce a three-mode structure (`practice`, `challenge`, `experimental`) that shares one map rule set.
- Add an `invincible` play mode with no death state (collision causes reposition), a separate high-score leaderboard, and persistent cross-run length accumulation.
- Define permanent unlock thresholds for experimental tools using invincible-mode cumulative length (`15/40/80/140/...`).
- Add experimental loadouts with three slots and mixed tool categories (control-assist and rule-modifying).
- Define challenge mode success metric as survival time, while allowing tool effects to influence same-board results.
- Remove textual milestone-achievement progression (no “+10 length literary achievements”).
- Keep sudden death behavior in mortal modes, with optional replay toggle in settings.
- Exclude daily/weekly mission systems from scope.

## Capabilities

### New Capabilities
- `mode-system-and-rules`: Defines practice/challenge/experimental/invincible mode behaviors, shared map policy, death/reposition semantics, and scoring boundaries.
- `invincible-length-progression`: Defines persistent cumulative length tracking in invincible mode and threshold-based permanent unlock logic.
- `experimental-tool-loadout`: Defines three-slot loadout behavior and eligible tool categories for experimental gameplay.
- `leaderboard-segmentation`: Defines separate leaderboard treatment for invincible mode and challenge scoring expectations.

### Modified Capabilities
- None.

## Impact

- Affected game systems: runtime mode state machine, collision/death pipeline, score calculation, progression persistence, tool activation flow, and end-of-run summary behavior.
- Affected UX surfaces: mode select, settings (replay toggle), tool loadout UI, run summary, and leaderboard views.
- Affected data/storage: persistent profile fields for cumulative invincible length, unlock state, and mode-specific scores.
- No external API or third-party dependency changes are required by this proposal.
