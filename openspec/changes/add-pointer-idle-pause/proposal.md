## Why

Current runtime control assumes continuous keyboard input and provides no pointer-first flow for touchpad users. We need pointer-centric interaction with explicit idle behavior so users can play and navigate without mandatory click actions while preserving deterministic movement.

## What Changes

- Add pointer-first runtime control semantics for active runs, including directional intent from pointer movement.
- Add idle-detection behavior during active runs: if pointer movement remains within a 2px threshold for 0.2 seconds, the run pauses.
- Define resume behavior for pointer-idle pause: resume on arrow-key input or renewed pointer movement above threshold.
- Add non-click pointer navigation semantics for menu-oriented screens (hover/focus, dwell confirmation, and scroll navigation) so click is not required.
- Update GUI control/help text expectations to reflect keyboard-plus-pointer behavior.

## Capabilities

### New Capabilities
None.

### Modified Capabilities
- `gui-runtime-and-navigation`: expand input requirements from keyboard-only to keyboard-plus-pointer, including pointer-idle pause/resume rules and non-click pointer navigation expectations.

## Impact

- Affected code:
  - `src/gui.rs` input polling, command mapping, running-state update loop, and control text rendering.
  - GUI tests in `src/gui.rs` for state transitions and control behavior.
- APIs/dependencies:
  - No external API changes.
  - No new runtime dependencies required.
- Systems:
  - Desktop interaction model shifts to pointer-first support while retaining keyboard compatibility.

