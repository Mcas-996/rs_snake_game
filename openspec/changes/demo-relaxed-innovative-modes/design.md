## Context

The change introduces multiple gameplay modes with shared map rules but different death, scoring, and progression semantics. The existing proposal sets product-level behavior: relaxed and practice-oriented core loop, sudden death in mortal modes, invincible mode with reposition instead of death, and a progression path where only invincible mode contributes permanent cumulative length. Experimental tools are unlocked at length thresholds and can alter play behavior.

This is cross-cutting because mode selection, runtime simulation, scoring, persistence, UI surfaces, and leaderboard handling all need consistent contracts. A design artifact is needed to avoid hidden coupling between runtime rules and progression storage.

## Goals / Non-Goals

**Goals:**
- Define a single runtime rules model for `practice`, `challenge`, `experimental`, and `invincible`.
- Isolate mode-specific scoring and death behavior without forking map logic.
- Define persistent data for cumulative invincible length, unlocked tools, replay preference, and mode-specific scores.
- Support threshold unlocks (`15/40/80/140/...`) and 3-slot experimental loadouts.
- Keep challenge mode metric centered on survival time while allowing tool effects to influence outcomes.

**Non-Goals:**
- Adding daily or weekly tasks.
- Building literary or milestone achievement systems.
- Introducing external services or backend multiplayer.
- Changing map topology by mode (all modes share the same map rules for this change).

## Decisions

1. Unified mode policy layer over one simulation core
- Decision: Keep one board simulation loop and one map rule set, then apply mode policy hooks for collision outcome, scoring, and end-of-run behavior.
- Rationale: This minimizes divergence bugs and preserves practice transfer across modes.
- Alternatives considered:
  - Separate mode-specific loops: simpler per mode but high maintenance and behavior drift risk.
  - Single loop with ad hoc conditionals everywhere: fast to start but hard to test and reason about.

2. Explicit collision outcome contract
- Decision: Introduce outcome enum from collision handling:
  - `die` for mortal modes (`practice`, `challenge`, `experimental`)
  - `reposition` for `invincible`
- Rationale: Prevents implicit mode checks in many systems and makes replay/death handling deterministic.
- Alternatives considered:
  - Skip collision in invincible mode: easier but removes meaningful feedback and leaderboard signal.

3. Mode-scoped scoring and leaderboard segmentation
- Decision: Keep score calculators mode-scoped and persist leaderboard entries with mode key. `invincible` uses an isolated leaderboard based on per-run score, not cumulative lifetime data.
- Rationale: Avoids cross-mode score inflation and keeps comparisons interpretable.
- Alternatives considered:
  - One global board: simpler UI, but fairness and interpretation degrade quickly.

4. Progression source of truth is invincible cumulative length
- Decision: Persist `invincible_cumulative_length` as the only unlock driver. Unlocks are threshold-based and permanent; no length spending.
- Rationale: Aligns with user direction and keeps progression explainable.
- Alternatives considered:
  - Spendable progression currency: adds economy complexity and balancing overhead.
  - Session-only progression: weak long-term loop for demo exploration.

5. Experimental tool system as pre-run 3-slot loadout
- Decision: Tools are selected before run start into exactly 3 slots. Tool effects can be control-assist, rule-modifying, or both.
- Rationale: Predictable run setup and easier balancing than random in-run pickups.
- Alternatives considered:
  - In-run pickups only: more excitement, but less reproducible for practice-oriented demo.
  - Unlimited tool selection: high combinatorial test surface.

6. Sudden death with optional replay toggle
- Decision: Mortal-mode death remains immediate. A user setting controls whether a short replay clip is shown before reset.
- Rationale: Preserves intended abrupt failure feel while supporting optional reflection.
- Alternatives considered:
  - Mandatory replay: slows pacing for users who prefer instant restart.
  - No replay support at all: loses useful feedback opportunity.

7. Challenge metric remains survival time, even with tools
- Decision: Challenge ranking emphasizes survival duration; tool effects are allowed and remain in same board.
- Rationale: Matches requested behavior and keeps challenge mode simple.
- Alternatives considered:
  - Separate "no-tool" ladder: more fair but adds ranking fragmentation not required for demo scope.

## Risks / Trade-offs

- [Score comparability ambiguity when tools are enabled in challenge] -> Mitigation: display run metadata (mode and loadout summary) alongside leaderboard rows.
- [Policy hooks may still leak mode conditionals into simulation] -> Mitigation: centralize mode policy interface and test each policy against shared simulation fixtures.
- [Unlock pacing may feel too fast or too slow] -> Mitigation: keep thresholds data-driven and tune values without code-level rule rewrites.
- [Reposition in invincible can create edge-case overlaps] -> Mitigation: enforce safe spawn validation and brief collision grace on reposition frame.
- [3-slot tool combinations increase QA matrix] -> Mitigation: start with limited tool count and compatibility flags for invalid combinations.

## Migration Plan

1. Add persistent profile fields with safe defaults:
- `invincible_cumulative_length = 0`
- `unlocked_tool_ids = []`
- `replay_on_death = false` (or current default)
- mode-scoped leaderboard storage
2. Introduce mode policy interfaces while preserving current single-mode behavior behind `practice` defaults.
3. Add invincible mode and cumulative length writes.
4. Add threshold unlock evaluator and tool loadout persistence.
5. Add challenge survival-time ranking with mode-scoped storage key.
6. Add replay toggle in settings and wire death pipeline to optional replay path.
7. Backfill or clear incompatible local leaderboard records if old schema is detected.

Rollback strategy:
- Keep migration reversible by versioning profile schema and preserving raw legacy score fields until rollout is stable.

## Open Questions

- Should challenge leaderboard display a "tools used" marker by default or only in detail view?
- What exact invincible score formula should be used for per-run ranking (raw food count vs weighted efficiency)?
- Should replay-on-death default to on or off for first launch?
