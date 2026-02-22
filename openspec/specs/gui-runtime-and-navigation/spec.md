# gui-runtime-and-navigation Specification

## Purpose
TBD - created by archiving change demo-relaxed-innovative-modes. Update Purpose after archive.
## Requirements
### Requirement: Desktop GUI runtime shall provide a playable game window
The system SHALL start a native desktop window and continuously render the game board, snake, and food in real time while a run is active.

#### Scenario: Start the application
- **WHEN** the user launches the demo executable
- **THEN** a visible game window opens and presents the initial navigation screen

### Requirement: GUI shall provide explicit screen navigation flow
The system SHALL expose navigation between main menu, mode selection, experimental loadout, settings, leaderboard, active run, and run summary screens, and SHALL support non-click pointer navigation semantics for menu-oriented screens.

#### Scenario: Navigate from menu to run and back
- **WHEN** the user enters a mode from menu and then exits or completes the run
- **THEN** the user can reach the summary screen and return to main menu without restarting the application

#### Scenario: Navigate menu without click input
- **WHEN** the user uses pointer hover/focus, dwell confirmation, and pointer scroll on menu-oriented screens
- **THEN** the GUI executes equivalent navigation outcomes to keyboard command flow without requiring click/tap

### Requirement: Keyboard controls shall drive deterministic movement
The system SHALL accept keyboard direction input and pointer-derived direction intent for snake control, SHALL apply movement updates on a fixed simulation tick independent of rendering frame rate, and SHALL apply pointer-idle pause behavior during active runs.

#### Scenario: Hold stable frame rate and change direction
- **WHEN** the player presses a valid direction key during an active run
- **THEN** the next simulation tick applies that direction consistently regardless of current render frame timing

#### Scenario: Pause active run when pointer is idle
- **WHEN** pointer displacement remains less than or equal to 2 pixels for at least 0.2 seconds during active play
- **THEN** the active run enters pointer-idle pause and simulation ticks stop advancing until resume criteria is met

#### Scenario: Resume from pointer-idle pause
- **WHEN** the run is in pointer-idle pause and the player presses an arrow key or pointer displacement exceeds 2 pixels
- **THEN** the run resumes active simulation and movement updates continue on deterministic ticks

### Requirement: GUI shall expose replay toggle and mode-aware death presentation
The system SHALL provide a settings toggle for replay-on-death and SHALL apply replay behavior only to mortal modes, while invincible mode always continues via reposition.

#### Scenario: Fatal collision with replay enabled
- **WHEN** replay-on-death is enabled and a fatal collision occurs in a mortal mode
- **THEN** the GUI shows replay path before reset or summary, and invincible mode does not enter replay path

### Requirement: Leaderboard and summary views shall render mode metadata
The system SHALL display mode-scoped leaderboard and summary information including mode identifier, score, survival time, and loadout summary where applicable.

#### Scenario: View challenge leaderboard row
- **WHEN** the user opens the challenge leaderboard
- **THEN** each row includes mode and loadout metadata with survival-time-based rank ordering

