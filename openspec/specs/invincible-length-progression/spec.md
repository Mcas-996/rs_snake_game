# invincible-length-progression Specification

## Purpose
TBD - created by archiving change demo-relaxed-innovative-modes. Update Purpose after archive.
## Requirements
### Requirement: Cumulative invincible length shall persist across runs
The system SHALL persist a profile-level cumulative length counter for invincible mode. The counter MUST increase based on growth achieved during invincible runs and MUST remain unchanged by non-invincible runs.

#### Scenario: End invincible run with growth
- **WHEN** an invincible run ends after the snake has grown
- **THEN** the profile cumulative invincible length increases by the run growth amount and is stored persistently

### Requirement: Unlocks shall be threshold-based and permanent
The system SHALL evaluate unlock thresholds against cumulative invincible length using ascending milestones of 15, 40, 80, and 140 with support for additional future milestones. Unlocks MUST not consume cumulative length once granted.

#### Scenario: Reach a new threshold
- **WHEN** cumulative invincible length reaches or exceeds a threshold for the first time
- **THEN** the corresponding unlock is granted permanently without reducing cumulative length

### Requirement: Threshold evaluation shall be deterministic
The system SHALL compute unlock state from persisted cumulative length and threshold definitions deterministically at load and after each invincible run.

#### Scenario: Reload profile with existing progress
- **WHEN** a player profile is loaded with previously stored cumulative length
- **THEN** unlock state is reconstructed consistently from thresholds and stored progress

