## ADDED Requirements

### Requirement: Game tick interval reduced for easier play
The game SHALL use a tick interval of 0.18 seconds instead of 0.12 seconds.

#### Scenario: Game runs at slower pace
- **WHEN** a game starts in any mode
- **THEN** the snake moves at 0.18 second intervals
- **AND** the game feels noticeably slower than before

### Requirement: Initial food count reduced
The game SHALL spawn 6 food items at game start instead of 10.

#### Scenario: Starting with fewer food items
- **WHEN** a game begins
- **THEN** exactly 6 food items appear on the board
- **AND** food refill logic remains unchanged (every 2 eaten + 3 food)

### Requirement: Collision grace period after spawn
The game SHALL provide a 3-tick grace period immediately after spawning.

#### Scenario: Spawn grace period prevents immediate death
- **WHEN** a snake spawns at game start
- **THEN** the snake is invulnerable for the first 3 ticks
- **AND** collisions during this period are ignored

### Requirement: Collision grace period after respawn
The game SHALL provide a 2-tick grace period after repositioning from collision.

#### Scenario: Respawn grace period in Invincible mode
- **WHEN** the snake respawns after a collision in Invincible mode
- **THEN** the snake is invulnerable for 2 ticks
- **AND** the grace period is tracked via grace_ticks_remaining field
