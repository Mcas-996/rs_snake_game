# Demo Notes: Relaxed Innovative Modes

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

- Challenge and experimental flows allow tool-influenced outcomes on shared boards by design.
- Reposition safety in invincible mode depends on available free tiles.
- Tool compatibility is modeled but intentionally minimal for demo scope.

## Out Of Scope

- Daily/weekly mission systems.
- Literary milestone achievements.
- External online services.
