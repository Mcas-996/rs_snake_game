## 1. Adjust Game Speed

- [x] 1.1 Update `SIM_TICK_SECONDS` constant in `src/gui.rs` from 0.12 to 0.18
- [x] 1.2 Verify game runs at slower pace by testing gameplay

## 2. Reduce Initial Food Count

- [x] 2.1 Update `INITIAL_FOOD_COUNT` constant in `src/gui.rs` from 10 to 6
- [x] 2.2 Verify only 6 food items spawn at game start

## 3. Implement Spawn Grace Period

- [x] 3.1 Modify `GameEngine::start_run()` in `src/lib.rs` to set `grace_ticks_remaining = 3` on new GameRun
- [x] 3.2 Verify spawn grace period works in Practice mode

## 4. Implement Respawn Grace Period

- [x] 4.1 Update collision handling in `GameEngine::handle_collision()` to set `grace_ticks_remaining = 2` when repositioning
- [x] 4.2 Verify grace period works in Invincible mode after collision

## 5. Update Tests

- [x] 5.1 Update any tests that depend on food count constants
- [x] 5.2 Add tests for grace period functionality
- [x] 5.3 Run full test suite to verify no regressions

## 6. Verification

- [x] 6.1 Manual testing: Game feels noticeably slower
- [x] 6.2 Manual testing: Initial food count is 6
- [x] 6.3 Manual testing: Grace periods prevent immediate deaths
