## Context

The change now targets a playable desktop GUI demo, not only core logic. The current repository has game-domain primitives but does not expose a graphical loop, interactive screen flow, or live rendering. The updated scope must combine GUI runtime behavior with existing mode, progression, and leaderboard rules.

This remains cross-cutting because window lifecycle, rendering, input, screen state transitions, persistence, and simulation policy must stay consistent with the existing mode contract.

## Goals / Non-Goals

**Goals:**
- Deliver a playable desktop GUI with real-time board rendering and keyboard controls.
- Define a single runtime rules model for `practice`, `challenge`, `experimental`, and `invincible`.
- Keep mode-specific scoring/death behavior isolated from shared map and simulation rules.
- Define persistent data for cumulative invincible length, unlocked tools, replay preference, and mode-specific scores.
- Support threshold unlocks (`15/40/80/140/...`) and 3-slot experimental loadouts.
- Keep challenge mode metric centered on survival time while allowing tool effects to influence outcomes.
- Provide screen flow for menu, settings, loadout, in-run, summary, and leaderboard views.

**Non-Goals:**
- Adding daily or weekly tasks.
- Building literary or milestone achievement systems.
- Introducing external services or backend multiplayer.
- Changing map topology by mode (all modes share the same map rules for this change).
- Building a networked account system or cloud save.

## Decisions

1. GUI stack selection: `macroquad`
- Decision: Use `macroquad` for native windowing, frame loop, rendering, and input handling; build required menu/settings/leaderboard/loadout surfaces with in-app 2D UI components.
- Rationale: Better fit for game-style real-time rendering and deterministic loop control while keeping dependency footprint small.
- Alternatives considered:
  - `bevy`: strong ecosystem but larger setup and architectural overhead for this demo scope.
  - `ggez`: solid rendering loop, but ecosystem and ergonomics are less streamlined than `macroquad` for rapid demo iteration.

2. Unified mode policy layer over one simulation core
- Decision: Keep one board simulation loop and one map rule set, then apply mode policy hooks for collision outcome, scoring, and end-of-run behavior.
- Rationale: This minimizes divergence bugs and preserves practice transfer across modes.
- Alternatives considered:
  - Separate mode-specific loops: simpler per mode but high maintenance and behavior drift risk.
  - Single loop with ad hoc conditionals everywhere: fast to start but hard to test and reason about.

3. Screen state machine for GUI navigation
- Decision: Introduce explicit app screens (`MainMenu`, `ModeSelect`, `Loadout`, `Running`, `Summary`, `Leaderboard`, `Settings`) with controlled transitions.
- Rationale: Prevents implicit UI coupling and keeps replay/loadout/leaderboard paths deterministic.
- Alternatives considered:
  - One mega-screen with conditional widgets: quick start but poor maintainability.

4. Fixed-timestep simulation with frame-rate-independent rendering
- Decision: Advance snake simulation on fixed ticks; render every frame and consume queued inputs per tick.
- Rationale: Keeps gameplay deterministic and practice-friendly across different machine speeds.
- Alternatives considered:
  - Frame-driven simulation: simpler loop but unstable pace with variable FPS.

5. Explicit collision outcome contract
- Decision: Introduce outcome enum from collision handling:
  - `die` for mortal modes (`practice`, `challenge`, `experimental`)
  - `reposition` for `invincible`
- Rationale: Prevents implicit mode checks in many systems and makes replay/death handling deterministic.
- Alternatives considered:
  - Skip collision in invincible mode: easier but removes meaningful feedback and leaderboard signal.

6. Mode-scoped scoring and leaderboard segmentation
- Decision: Keep score calculators mode-scoped and persist leaderboard entries with mode key. `invincible` uses an isolated leaderboard based on per-run score, not cumulative lifetime data.
- Rationale: Avoids cross-mode score inflation and keeps comparisons interpretable.
- Alternatives considered:
  - One global board: simpler UI, but fairness and interpretation degrade quickly.

7. Progression source of truth is invincible cumulative length
- Decision: Persist `invincible_cumulative_length` as the only unlock driver. Unlocks are threshold-based and permanent; no length spending.
- Rationale: Aligns with user direction and keeps progression explainable.
- Alternatives considered:
  - Spendable progression currency: adds economy complexity and balancing overhead.
  - Session-only progression: weak long-term loop for demo exploration.

8. Experimental tool system as pre-run 3-slot loadout
- Decision: Tools are selected before run start into exactly 3 slots. Tool effects can be control-assist, rule-modifying, or both.
- Rationale: Predictable run setup and easier balancing than random in-run pickups.
- Alternatives considered:
  - In-run pickups only: more excitement, but less reproducible for practice-oriented demo.
  - Unlimited tool selection: high combinatorial test surface.

9. Sudden death with optional replay toggle
- Decision: Mortal-mode death remains immediate. A user setting controls whether a short replay clip is shown before reset.
- Rationale: Preserves intended abrupt failure feel while supporting optional reflection.
- Alternatives considered:
  - Mandatory replay: slows pacing for users who prefer instant restart.
  - No replay support at all: loses useful feedback opportunity.

10. Challenge metric remains survival time, even with tools
- Decision: Challenge ranking emphasizes survival duration; tool effects are allowed and remain in same board.
- Rationale: Matches requested behavior and keeps challenge mode simple.
- Alternatives considered:
  - Separate "no-tool" ladder: more fair but adds ranking fragmentation not required for demo scope.

## Risks / Trade-offs

- [Score comparability ambiguity when tools are enabled in challenge] -> Mitigation: display run metadata (mode and loadout summary) alongside leaderboard rows.
- [GUI framework integration grows compile/startup time] -> Mitigation: keep rendering/UI adapter layer thin and preserve headless core for fast tests.
- [Input feel may drift across frame rates] -> Mitigation: fixed simulation tick with input queue and deterministic movement tests.
- [UI flow regressions between screens] -> Mitigation: centralize transition logic and add integration tests for key screen paths.
- [Policy hooks may still leak mode conditionals into simulation] -> Mitigation: centralize mode policy interface and test each policy against shared simulation fixtures.
- [Unlock pacing may feel too fast or too slow] -> Mitigation: keep thresholds data-driven and tune values without code-level rule rewrites.
- [Reposition in invincible can create edge-case overlaps] -> Mitigation: enforce safe spawn validation and brief collision grace on reposition frame.
- [3-slot tool combinations increase QA matrix] -> Mitigation: start with limited tool count and compatibility flags for invalid combinations.

## Migration Plan

1. Add GUI dependencies and app state structure (`Screen` + transition handlers) while preserving headless core APIs.
2. Add fixed-timestep runner and board renderer in GUI runtime.
3. Add screen implementations for mode select, loadout, settings, leaderboard, run HUD, and summary.
4. Keep and validate persistent profile fields with safe defaults:
- `invincible_cumulative_length = 0`
- `unlocked_tool_ids = []`
- `replay_on_death = false` (or current default)
- mode-scoped leaderboard storage
5. Wire GUI actions to existing mode policies, collision outcomes, and scoring contracts.
6. Finalize replay toggle UX and death/reposition visual paths.
7. Backfill or clear incompatible local leaderboard records if old schema is detected.

Rollback strategy:
- Keep migration reversible by versioning profile schema and preserving raw legacy score fields until rollout is stable.

## Open Questions

- Should challenge leaderboard display a "tools used" marker by default or only in detail view?
- What exact invincible score formula should be used for per-run ranking (raw food count vs weighted efficiency)?
- Should replay-on-death default to on or off for first launch?
- Should initial GUI target fixed window size first, then responsive scaling as follow-up?
