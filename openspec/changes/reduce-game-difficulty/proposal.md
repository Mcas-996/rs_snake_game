# Change: Reduce Game Difficulty

## Why

The current game difficulty is too high for casual players, resulting in frequent early deaths and a steep learning curve. Reducing the game speed and adjusting collision mechanics will make the game more accessible to new players while maintaining challenge for experienced players through existing mode selection.

## What Changes

- Increase game tick interval (slow down snake movement)
  - Change `SIM_TICK_SECONDS` from 0.12 to 0.18 seconds per tick
- Reduce initial food density to lower early-game pressure
  - Change `INITIAL_FOOD_COUNT` from 10 to 6
- Increase grace period for collision recovery
  - Change default `grace_ticks_remaining` from 0 to 2 ticks
- Add brief grace period after mode start before collision detection
  - First 3 ticks after spawn are collision-free

## Capabilities

### New Capabilities
<!-- No new capabilities being introduced -->

### Modified Capabilities
<!-- Existing capabilities whose requirements are changing -->
- `game-speed`: Modify tick interval constant from 0.12s to 0.18s
- `food-spawn`: Reduce initial food count from 10 to 6
- `collision-grace`: Increase default grace ticks from 0 to 2, add spawn grace period of 3 ticks

## Impact

**Affected code:**
- `src/gui.rs`: `SIM_TICK_SECONDS` and `INITIAL_FOOD_COUNT` constants
- `src/lib.rs`: `GameRun::grace_ticks_remaining` initialization and `tick()` method
- `src/lib.rs`: `GameEngine::start_run()` to set initial grace period

**No breaking changes to:**
- Game modes or their scoring logic
- Leaderboard data format
- Profile persistence
- Tool/loadout system
