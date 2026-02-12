# leaderboard-segmentation Specification

## Purpose
TBD - created by archiving change demo-relaxed-innovative-modes. Update Purpose after archive.
## Requirements
### Requirement: Leaderboards shall be scoped by mode
The system SHALL persist leaderboard records with an explicit mode scope. Invincible mode records MUST be stored in an isolated leaderboard that does not merge with mortal mode boards.

#### Scenario: Submit invincible run result
- **WHEN** an invincible run is completed with a score
- **THEN** the score is written only to the invincible leaderboard scope

### Requirement: Challenge ranking shall prioritize survival time
Challenge mode leaderboard ordering SHALL prioritize higher survival time. The system MUST allow tool-affected runs in the same challenge board.

#### Scenario: Compare two challenge runs
- **WHEN** two challenge runs have different survival times
- **THEN** the run with longer survival time ranks higher regardless of tool usage

### Requirement: Run metadata shall remain attached to entries
The system SHALL retain mode and loadout metadata for each leaderboard entry to preserve interpretation of tool-influenced results.

#### Scenario: Render challenge leaderboard row
- **WHEN** a challenge leaderboard row is displayed
- **THEN** the row includes mode identifier and loadout metadata for that run

