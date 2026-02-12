## ADDED Requirements

### Requirement: Mode selection shall define runtime policy
The system SHALL provide four selectable modes: practice, challenge, experimental, and invincible. Each mode MUST bind a runtime policy that defines collision outcome, scoring model, and end-of-run behavior while sharing the same map rule set.

#### Scenario: Start run in selected mode
- **WHEN** the player starts a run after selecting any mode
- **THEN** the run uses the shared map rules and applies the selected mode policy for scoring and collision behavior

### Requirement: Mortal modes shall end immediately on fatal collision
In practice, challenge, and experimental modes, the system SHALL treat fatal collision as immediate death and end the run without delayed game-over flow.

#### Scenario: Fatal collision in mortal mode
- **WHEN** a fatal collision occurs in practice, challenge, or experimental mode
- **THEN** the run ends immediately and transitions to run summary or restart flow

### Requirement: Invincible mode shall reposition instead of death
In invincible mode, the system SHALL never terminate the run due to collision. Fatal collision MUST trigger reposition behavior to a valid safe location.

#### Scenario: Collision in invincible mode
- **WHEN** a fatal collision occurs while the run mode is invincible
- **THEN** the run remains active and the snake is repositioned to a valid location
