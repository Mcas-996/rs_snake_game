# add-pointer-idle-pause Feature Proposal

## Problem Statement

Players want to use pointer (mouse/touch) input to control the snake game without clicking, and want the game to pause when they step away from the keyboard while the game is running.

## Proposed Solution

1. **Pointer Idle Pause**: Add automatic pause when pointer doesn't move for 0.2s during active play, with resume via arrow key or pointer movement > 2px.

2. **Non-click Menu Navigation**: Implement pointer-based menu navigation using hover/focus, dwell confirmation, and pointer scroll without requiring click/tap.

## Scope

- Active run pointer-idle detection and pause/resume
- Menu screen pointer navigation (hover, dwell, scroll)
- No changes to replay behavior

## Out of Scope

- Click-based interactions
- Replay phase pointer behavior
