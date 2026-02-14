## 1. Input Pipeline and Runtime State

- [ ] 1.1 Add pointer input sampling in `src/gui.rs` and normalize pointer signals into the existing `UiCommand` dispatch path.
- [ ] 1.2 Introduce a running sub-state for pointer-idle pause and wire transition checks for `idle >= 0.5s` with `<= 2px` displacement threshold.
- [ ] 1.3 Implement resume transitions from pointer-idle pause on arrow-key input or pointer movement `> 2px`.
- [ ] 1.4 Ensure replay phase behavior is unchanged and excluded from pointer-idle pause transitions.

## 2. Menu Interaction Without Click

- [ ] 2.1 Define pointer hit regions for menu-oriented screens and map hover/focus to existing cursor selection fields.
- [ ] 2.2 Add dwell-based `Confirm` behavior for focused menu targets without requiring click/tap.
- [ ] 2.3 Add pointer scroll-to-navigation mapping for list-style menus using existing Up/Down command paths.
- [ ] 2.4 Add a dwell-based back hotzone path that emits `Back` in menu-oriented screens.

## 3. UX Messaging and Verification

- [ ] 3.1 Update on-screen control text to describe keyboard-plus-pointer controls and pointer-idle pause/resume behavior.
- [ ] 3.2 Add GUI tests in `src/gui.rs` covering idle-pause entry, resume via keyboard, and resume via pointer movement.
- [ ] 3.3 Add GUI tests covering non-click pointer navigation equivalence for menu traversal and confirmation.
- [ ] 3.4 Run `cargo test` and adjust tests/messages until behavior and specs align.
