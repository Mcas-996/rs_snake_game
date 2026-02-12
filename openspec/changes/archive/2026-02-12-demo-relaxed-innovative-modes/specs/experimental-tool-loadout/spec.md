## ADDED Requirements

### Requirement: Experimental mode shall support exactly three loadout slots
The system SHALL require a pre-run loadout for experimental mode containing exactly three tool slots. A run MUST use the selected loadout snapshot for its full duration.

#### Scenario: Start experimental run with loadout
- **WHEN** the player confirms an experimental run
- **THEN** the run starts with exactly three selected tools bound as the active loadout

#### Scenario: Display loadout slots in GUI
- **WHEN** the player opens the experimental loadout screen
- **THEN** the interface shows exactly three selectable slots before run start

### Requirement: Tools shall support assist and rule-modifying categories
The system SHALL support tools categorized as control-assist, rule-modifying, or hybrid. Tool definitions MUST declare category metadata used by selection and validation.

#### Scenario: View selectable tools
- **WHEN** the player opens experimental loadout selection
- **THEN** each tool entry shows category metadata and can be validated for slot assignment

### Requirement: Only unlocked tools shall be selectable
The system SHALL prevent locked tools from being equipped in experimental loadout slots. Permanent unlocks from invincible progression MUST immediately permit selection in later run setup flows.

#### Scenario: Attempt to equip locked tool
- **WHEN** the player selects a tool that is not yet unlocked
- **THEN** the equip action is rejected and the slot remains unchanged
