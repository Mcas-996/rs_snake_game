## Context

The GUI runtime currently accepts keyboard-only navigation and direction control through a shared `UiCommand` dispatch path in `src/gui.rs`. The user experience target is pointer-first touchpad play without mandatory click actions, while preserving deterministic movement and existing mode/replay behavior.

The requested pointer idle semantics are explicit and fixed:
- Idle threshold time: 0.2 seconds
- Idle motion threshold: pointer movement <= 2px counts as idle
- Resume from idle pause: arrow-key input OR pointer movement > 2px

## Goals / Non-Goals

**Goals:**
- Introduce pointer-first control as a first-class input path across menu and running screens.
- Define non-click interaction semantics for menu navigation (hover/focus, dwell confirm, scroll traversal).
- Define deterministic idle-pause behavior in active runs using the agreed 0.2s/2px policy.
- Keep keyboard controls fully functional and backward compatible.

**Non-Goals:**
- No gameplay rule changes (collision, scoring, mode policy, progression).
- No mandatory gesture recognition beyond pointer movement, dwell, and scroll signals.
- No mobile-specific redesign or virtual on-screen buttons.
- No external dependency changes.

## Decisions

### 1) Input adaptation remains centralized in GUI command mapping
The system will continue to map all input modalities into `UiCommand` before screen-specific handlers execute.

Rationale:
- Preserves existing state-machine behavior and test structure.
- Avoids duplicating per-screen transition logic for pointer and keyboard paths.

Alternatives considered:
- Directly mutating screen state from pointer handlers was rejected because it duplicates logic and increases regression risk.

### 2) Running screen uses pointer-idle gate with explicit paused sub-state
Active runs gain a paused-by-idle sub-state. Simulation ticks do not advance while paused-by-idle.

Entry condition:
- Pointer displacement remains <= 2px for >= 0.2s during active run.

Exit condition:
- Arrow key input OR pointer movement > 2px.

Rationale:
- Matches requested behavior exactly.
- Keeps deterministic simulation by halting tick advancement, not by slowing dt.

Alternatives considered:
- Gradual slow-down instead of pause was rejected for ambiguous semantics and test complexity.

### 3) Menu screens support click-free pointer navigation
Menu-oriented screens adopt:
- Hover/focus selection by hit region.
- Dwell-to-confirm after stable focus duration.
- Scroll-based Up/Down traversal for list-like menus.
- Dedicated back hotzone with dwell-based Back action.

Rationale:
- Satisfies pointer-first requirement without forcing click/tap.
- Coexists with keyboard and does not require schema/API changes.

Alternatives considered:
- Click-only pointer support was rejected because the requirement explicitly avoids mandatory click.

### 4) Replay behavior remains unchanged
Pointer-idle pause applies only to running active play, not replay phase.

Rationale:
- Replay is presentation flow, not interactive control phase.
- Avoids introducing ambiguous stop/resume behavior during death replay.

## Risks / Trade-offs

- [Risk] Pointer jitter could cause unwanted unpause/repause loops. -> Mitigation: apply strict >2px exit condition and brief post-resume grace window before re-evaluating idle.
- [Risk] Dwell interactions may feel slow for expert users. -> Mitigation: keep keyboard parity and tune dwell thresholds through testing.
- [Risk] Pointer semantics may differ across desktop environments. -> Mitigation: constrain behavior to broadly available pointer position and wheel signals.
- [Risk] Increased input-state complexity can regress navigation. -> Mitigation: add focused GUI tests for idle pause transitions and non-click menu flows.
