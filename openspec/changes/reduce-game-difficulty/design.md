# Design: Reduce Game Difficulty

## Context

The snake game currently has a fast pace (0.12s tick interval) and dense food placement (10 initial foods on a 12x12 board). New players report that the game feels "too frantic" and they die quickly before learning the controls. The current grace period for collisions is minimal (0 ticks by default).

Current state:
- Game tick: 0.12 seconds
- Board: 12x12 cells
- Initial food: 10 pieces
- No spawn grace period
- No collision recovery grace period

## Goals / Non-Goals

**Goals:**
- Reduce game speed to give players more reaction time
- Lower early-game pressure by reducing initial food density
- Provide brief invulnerability periods to prevent "cheap" deaths
- Maintain the existing game modes and scoring systems

**Non-Goals:**
- Changing game mode rules or scoring formulas
- Adding new game mechanics or power-ups
- Modifying the board size
- Altering the food respawn logic beyond initial count

## Decisions

**Decision 1: Increase tick interval from 0.12s to 0.18s**
- Rationale: 50% speed reduction provides noticeable relief while keeping game engaging
- Alternative considered: 0.24s (felt too slow in testing mental model)
- Trade-off: Games last longer, which affects session length

**Decision 2: Reduce initial food from 10 to 6**
- Rationale: Reduces early-game "clutter" and decision pressure
- Food refill logic remains unchanged (refills every 2 eaten + 3 food)
- Alternative considered: 8 food (not enough perceptible difference)

**Decision 3: Add 2-tick grace period after respawn, 3-tick spawn grace**
- Rationale: Prevents immediate death from spawn/respawn collisions
- Spawn grace (3 ticks) longer than respawn grace (2 ticks) to account for player orientation time
- Applied via `grace_ticks_remaining` field already present in `GameRun`

**Decision 4: Keep grace period collisions as "no-op" rather than reposition**
- Rationale: Simpler mental model - player is simply invulnerable briefly
- Alternative: Reposition during grace (rejected - confusing behavior)

## Risks / Trade-offs

**[Risk]** Experienced players may find game too easy
- **Mitigation**: This change only affects base constants. Challenge/Experimental modes retain their difficulty through scoring multipliers and tool restrictions. Players seeking challenge can use those modes.

**[Risk]** Longer games may feel tedious
- **Mitigation**: 0.18s is still reasonably fast (5.5 ticks/second). The difference is noticeable but not dramatic.

**[Risk]** Grace period abuse (intentional collision exploitation)
- **Mitigation**: Grace periods are very short (2-3 ticks = 0.36-0.54s). Not long enough to meaningfully exploit. Invincible mode already allows unlimited repositioning anyway.

**[Trade-off]** Reduced food means slower early growth
- **Acceptance**: This is intentional - reduces early-game chaos and allows players to focus on movement patterns first.

## Migration Plan

No migration required - these are runtime constants. Changes take effect immediately on next game start.

**Rollback**: Simply revert constant values if needed.
