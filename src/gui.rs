use std::collections::VecDeque;
use std::time::{SystemTime, UNIX_EPOCH};

use macroquad::prelude::*;

use crate::{GameEngine, GameMode, GameRun, Point, Profile, ToolCategory, policy_for};

const WINDOW_WIDTH: i32 = 1000;
const WINDOW_HEIGHT: i32 = 760;
const SIM_TICK_SECONDS: f32 = 0.18;
const REPLAY_SECONDS: f32 = 0.85;
const CELL_SIZE: f32 = 32.0;
const INITIAL_FOOD_COUNT: usize = 6;
const FOOD_REFILL_EVERY_EATEN: u64 = 2;
const FOOD_REFILL_COUNT: usize = 3;
const POINTER_IDLE_SECONDS_OUTSIDE_BOARD: f32 = 0.01;
const POINTER_DISPLACEMENT_THRESHOLD: f32 = 2.0;
const POINTER_DWELL_SECONDS: f32 = 0.45;
const POINTER_IDLE_GRACE_SECONDS: f32 = 0.2;

const MAIN_MENU_ITEMS: [&str; 3] = ["Play", "Leaderboards", "Settings"];
const MODES: [GameMode; 4] = [
    GameMode::Practice,
    GameMode::Challenge,
    GameMode::Experimental,
    GameMode::Invincible,
];

pub fn window_conf() -> Conf {
    Conf {
        window_title: "Snake GUI Demo".to_string(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        window_resizable: true,
        ..Default::default()
    }
}

pub async fn run_app() {
    let mut app = SnakeGuiApp::new();
    loop {
        let dt = get_frame_time();
        app.update(dt);
        app.draw();
        next_frame().await;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ScreenState {
    MainMenu,
    ModeSelect,
    Loadout,
    Running,
    Summary,
    Leaderboard,
    Settings,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum UiCommand {
    Up,
    Down,
    Left,
    Right,
    Confirm,
    Back,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RunningPhase {
    Active,
    PointerIdlePause,
    Replay,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PointerFocusTarget {
    MainMenuItem(usize),
    ModeItem(usize),
    LoadoutSlot(usize),
    SettingsToggle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(self, other: Direction) -> bool {
        matches!(
            (self, other),
            (Direction::Up, Direction::Down)
                | (Direction::Down, Direction::Up)
                | (Direction::Left, Direction::Right)
                | (Direction::Right, Direction::Left)
        )
    }

    fn step(self, mut point: Point, board_width: i32, board_height: i32, wraps: bool) -> Point {
        match self {
            Direction::Up => point.y -= 1,
            Direction::Down => point.y += 1,
            Direction::Left => point.x -= 1,
            Direction::Right => point.x += 1,
        }

        if wraps {
            if point.x < 0 {
                point.x = board_width - 1;
            } else if point.x >= board_width {
                point.x = 0;
            }
            if point.y < 0 {
                point.y = board_height - 1;
            } else if point.y >= board_height {
                point.y = 0;
            }
        }

        point
    }
}

#[derive(Debug, Clone)]
struct LoadoutState {
    slot_cursor: usize,
    selected_tool_indices: [usize; 3],
}

impl Default for LoadoutState {
    fn default() -> Self {
        Self {
            slot_cursor: 0,
            selected_tool_indices: [0, 1, 2],
        }
    }
}

#[derive(Debug, Clone)]
struct RunningState {
    run: GameRun,
    direction: Direction,
    queued_directions: VecDeque<Direction>,
    phase: RunningPhase,
    replay_timer: f32,
    accumulator: f32,
    tick_seconds: f32,
    foods: Vec<Point>,
    spawn_seed: u64,
    replay_path: Vec<Point>,
    pointer_idle_anchor: Option<Vec2>,
    pointer_idle_elapsed: f32,
    idle_grace_timer: f32,
}

impl RunningState {
    fn new(run: GameRun) -> Self {
        Self {
            run,
            direction: Direction::Right,
            queued_directions: VecDeque::new(),
            phase: RunningPhase::Active,
            replay_timer: 0.0,
            accumulator: 0.0,
            tick_seconds: SIM_TICK_SECONDS,
            foods: Vec::new(),
            spawn_seed: 0,
            replay_path: Vec::new(),
            pointer_idle_anchor: None,
            pointer_idle_elapsed: 0.0,
            idle_grace_timer: 0.0,
        }
    }
}

#[derive(Debug, Clone)]
struct RunSummary {
    mode: GameMode,
    score: u64,
    survival_ticks: u64,
    food_eaten: u64,
    growth_units: u64,
    loadout_summary: String,
}

pub struct SnakeGuiApp {
    engine: GameEngine,
    screen: ScreenState,
    main_menu_cursor: usize,
    mode_cursor: usize,
    leaderboard_mode_cursor: usize,
    loadout_state: LoadoutState,
    running: Option<RunningState>,
    summary: Option<RunSummary>,
    message: Option<String>,
    pointer_last_position: Option<Vec2>,
    pointer_focus_target: Option<PointerFocusTarget>,
    pointer_focus_dwell: f32,
    pointer_focus_armed: bool,
    pointer_back_dwell: f32,
    pointer_back_armed: bool,
}

impl SnakeGuiApp {
    pub fn new() -> Self {
        Self::with_profile(Profile::default())
    }

    pub fn with_profile(profile: Profile) -> Self {
        Self {
            engine: GameEngine::new(profile),
            screen: ScreenState::MainMenu,
            main_menu_cursor: 0,
            mode_cursor: 0,
            leaderboard_mode_cursor: 0,
            loadout_state: LoadoutState::default(),
            running: None,
            summary: None,
            message: None,
            pointer_last_position: None,
            pointer_focus_target: None,
            pointer_focus_dwell: 0.0,
            pointer_focus_armed: false,
            pointer_back_dwell: 0.0,
            pointer_back_armed: false,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.poll_keyboard_commands();
        let (mx, my) = mouse_position();
        let (_, wheel_y) = mouse_wheel();
        self.apply_pointer_input(dt, vec2(mx, my), wheel_y);
        if self.screen == ScreenState::Running {
            self.update_running(dt);
        }
    }

    fn poll_keyboard_commands(&mut self) {
        if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) {
            self.apply_command(UiCommand::Up);
        }
        if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S) {
            self.apply_command(UiCommand::Down);
        }
        if is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::A) {
            self.apply_command(UiCommand::Left);
        }
        if is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::D) {
            self.apply_command(UiCommand::Right);
        }
        if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space) {
            self.apply_command(UiCommand::Confirm);
        }
        if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::Backspace) {
            self.apply_command(UiCommand::Back);
        }
    }

    fn apply_command(&mut self, command: UiCommand) {
        match self.screen {
            ScreenState::MainMenu => self.apply_main_menu_command(command),
            ScreenState::ModeSelect => self.apply_mode_select_command(command),
            ScreenState::Loadout => self.apply_loadout_command(command),
            ScreenState::Running => self.apply_running_command(command),
            ScreenState::Summary => self.apply_summary_command(command),
            ScreenState::Leaderboard => self.apply_leaderboard_command(command),
            ScreenState::Settings => self.apply_settings_command(command),
        }
    }

    fn apply_main_menu_command(&mut self, command: UiCommand) {
        match command {
            UiCommand::Up => {
                self.main_menu_cursor =
                    cycle_index(self.main_menu_cursor, -1, MAIN_MENU_ITEMS.len())
            }
            UiCommand::Down => {
                self.main_menu_cursor = cycle_index(self.main_menu_cursor, 1, MAIN_MENU_ITEMS.len())
            }
            UiCommand::Confirm => match self.main_menu_cursor {
                0 => {
                    self.mode_cursor = 0;
                    self.screen = ScreenState::ModeSelect;
                }
                1 => {
                    self.leaderboard_mode_cursor = 0;
                    self.screen = ScreenState::Leaderboard;
                }
                2 => self.screen = ScreenState::Settings,
                _ => {}
            },
            UiCommand::Back | UiCommand::Left | UiCommand::Right => {}
        }
    }

    fn apply_mode_select_command(&mut self, command: UiCommand) {
        match command {
            UiCommand::Left | UiCommand::Up => {
                self.mode_cursor = cycle_index(self.mode_cursor, -1, MODES.len())
            }
            UiCommand::Right | UiCommand::Down => {
                self.mode_cursor = cycle_index(self.mode_cursor, 1, MODES.len())
            }
            UiCommand::Confirm => {
                let mode = MODES[self.mode_cursor];
                if mode == GameMode::Experimental {
                    self.loadout_state = self.default_loadout_state();
                    self.screen = ScreenState::Loadout;
                } else {
                    self.start_mode(mode, None);
                }
            }
            UiCommand::Back => self.screen = ScreenState::MainMenu,
        }
    }

    fn apply_loadout_command(&mut self, command: UiCommand) {
        let all_tools = self.tool_ids();
        if all_tools.is_empty() {
            self.message = Some("no tools available in registry".to_string());
            return;
        }

        match command {
            UiCommand::Up => {
                self.loadout_state.slot_cursor = cycle_index(self.loadout_state.slot_cursor, -1, 3)
            }
            UiCommand::Down => {
                self.loadout_state.slot_cursor = cycle_index(self.loadout_state.slot_cursor, 1, 3)
            }
            UiCommand::Left => {
                let slot = self.loadout_state.slot_cursor;
                let current = self.loadout_state.selected_tool_indices[slot];
                self.loadout_state.selected_tool_indices[slot] =
                    cycle_index(current, -1, all_tools.len());
            }
            UiCommand::Right => {
                let slot = self.loadout_state.slot_cursor;
                let current = self.loadout_state.selected_tool_indices[slot];
                self.loadout_state.selected_tool_indices[slot] =
                    cycle_index(current, 1, all_tools.len());
            }
            UiCommand::Confirm => {
                let selected = self.selected_loadout_tool_ids();
                self.start_mode(GameMode::Experimental, Some(selected));
            }
            UiCommand::Back => self.screen = ScreenState::ModeSelect,
        }
    }

    fn apply_running_command(&mut self, command: UiCommand) {
        match command {
            UiCommand::Up => self.enqueue_running_direction(Direction::Up),
            UiCommand::Down => self.enqueue_running_direction(Direction::Down),
            UiCommand::Left => self.enqueue_running_direction(Direction::Left),
            UiCommand::Right => self.enqueue_running_direction(Direction::Right),
            UiCommand::Back => self.complete_running_session(),
            UiCommand::Confirm => {}
        }
    }

    fn apply_summary_command(&mut self, command: UiCommand) {
        match command {
            UiCommand::Confirm | UiCommand::Back => self.screen = ScreenState::MainMenu,
            UiCommand::Right => self.screen = ScreenState::Leaderboard,
            UiCommand::Up | UiCommand::Down | UiCommand::Left => {}
        }
    }

    fn apply_leaderboard_command(&mut self, command: UiCommand) {
        match command {
            UiCommand::Left | UiCommand::Up => {
                self.leaderboard_mode_cursor =
                    cycle_index(self.leaderboard_mode_cursor, -1, MODES.len())
            }
            UiCommand::Right | UiCommand::Down => {
                self.leaderboard_mode_cursor =
                    cycle_index(self.leaderboard_mode_cursor, 1, MODES.len())
            }
            UiCommand::Confirm | UiCommand::Back => self.screen = ScreenState::MainMenu,
        }
    }

    fn apply_settings_command(&mut self, command: UiCommand) {
        match command {
            UiCommand::Left | UiCommand::Right | UiCommand::Confirm => {
                let next = !self.engine.profile.replay_on_death;
                self.engine.enable_replay(next);
            }
            UiCommand::Back => self.screen = ScreenState::MainMenu,
            UiCommand::Up | UiCommand::Down => {}
        }
    }

    fn tool_ids(&self) -> Vec<String> {
        self.engine
            .registry
            .list()
            .map(|tool| tool.id.clone())
            .collect()
    }

    fn default_loadout_state(&self) -> LoadoutState {
        let ids = self.tool_ids();
        if ids.is_empty() {
            return LoadoutState::default();
        }

        let mut indices = [0usize, 0, 0];
        for (i, slot) in indices.iter_mut().enumerate() {
            *slot = i % ids.len();
        }
        LoadoutState {
            slot_cursor: 0,
            selected_tool_indices: indices,
        }
    }

    fn selected_loadout_tool_ids(&self) -> Vec<String> {
        let ids = self.tool_ids();
        self.loadout_state
            .selected_tool_indices
            .iter()
            .map(|idx| ids[*idx % ids.len()].clone())
            .collect()
    }

    fn enqueue_direction(&mut self, direction: Direction) {
        let Some(state) = self.running.as_mut() else {
            return;
        };
        if state.phase != RunningPhase::Active {
            return;
        }

        let reference = state
            .queued_directions
            .back()
            .copied()
            .unwrap_or(state.direction);
        if direction == reference || direction.opposite(reference) {
            return;
        }
        if state.queued_directions.len() < 3 {
            state.queued_directions.push_back(direction);
        }
    }

    fn enqueue_running_direction(&mut self, direction: Direction) {
        self.resume_from_pointer_idle_pause(None);
        self.enqueue_direction(direction);
    }

    fn apply_pointer_input(&mut self, dt: f32, pointer_position: Vec2, wheel_y: f32) {
        let pointer_delta = self
            .pointer_last_position
            .map(|last| pointer_position - last)
            .unwrap_or(Vec2::ZERO);
        self.pointer_last_position = Some(pointer_position);

        if self.screen == ScreenState::Running {
            self.apply_pointer_running(dt, pointer_position, pointer_delta);
            self.pointer_focus_target = None;
            self.pointer_focus_dwell = 0.0;
            self.pointer_focus_armed = false;
            self.pointer_back_dwell = 0.0;
            self.pointer_back_armed = false;
            return;
        }

        self.apply_pointer_menu(dt, pointer_position, pointer_delta, wheel_y);
    }

    fn apply_pointer_running(&mut self, dt: f32, pointer_position: Vec2, pointer_delta: Vec2) {
        let mut resume_due_to_pointer = false;
        let mut pointer_direction = None;
        let mut entered_idle_pause = false;

        if let Some(state) = self.running.as_mut() {
            match state.phase {
                RunningPhase::Replay => return,
                RunningPhase::PointerIdlePause => {
                    let anchor = state.pointer_idle_anchor.unwrap_or(pointer_position);
                    if pointer_position.distance(anchor) > POINTER_DISPLACEMENT_THRESHOLD {
                        resume_due_to_pointer = true;
                        pointer_direction = direction_from_delta(pointer_position - anchor);
                    }
                }
                RunningPhase::Active => {
                    if state.idle_grace_timer > 0.0 {
                        state.idle_grace_timer = (state.idle_grace_timer - dt).max(0.0);
                    }
                    let pointer_in_board = pointer_board_cell(state, pointer_position).is_some();
                    let has_pointer_intent =
                        direction_toward_pointer(state, pointer_position).is_some();
                    if let Some(direction) = direction_toward_pointer(state, pointer_position) {
                        pointer_direction = Some(direction);
                    } else if let Some(direction) = direction_from_delta(pointer_delta) {
                        pointer_direction = Some(direction);
                    }
                    if state.idle_grace_timer > 0.0 {
                        return;
                    }

                    if state.pointer_idle_anchor.is_none() {
                        state.pointer_idle_anchor = Some(pointer_position);
                    }

                    if pointer_in_board || has_pointer_intent {
                        state.pointer_idle_anchor = Some(pointer_position);
                        state.pointer_idle_elapsed = 0.0;
                    } else {
                        let anchor = state.pointer_idle_anchor.unwrap_or(pointer_position);
                        if pointer_position.distance(anchor) <= POINTER_DISPLACEMENT_THRESHOLD {
                            state.pointer_idle_elapsed += dt;
                            if state.pointer_idle_elapsed >= POINTER_IDLE_SECONDS_OUTSIDE_BOARD {
                                state.phase = RunningPhase::PointerIdlePause;
                                state.pointer_idle_anchor = Some(pointer_position);
                                state.pointer_idle_elapsed = 0.0;
                                entered_idle_pause = true;
                            }
                        } else {
                            state.pointer_idle_anchor = Some(pointer_position);
                            state.pointer_idle_elapsed = 0.0;
                        }
                    }
                }
            }
        }

        if resume_due_to_pointer {
            self.resume_from_pointer_idle_pause(Some(pointer_position));
        }
        if !entered_idle_pause {
            if let Some(direction) = pointer_direction {
                self.enqueue_running_direction(direction);
            }
        }
    }

    fn apply_pointer_menu(
        &mut self,
        dt: f32,
        pointer_position: Vec2,
        pointer_delta: Vec2,
        wheel_y: f32,
    ) {
        if self.supports_scroll_navigation() {
            if wheel_y > 0.0 {
                self.apply_command(UiCommand::Up);
            } else if wheel_y < 0.0 {
                self.apply_command(UiCommand::Down);
            }
        }

        let focus = self.pointer_focus_target(pointer_position);
        if let Some(target) = focus {
            self.apply_pointer_focus(target);
            if self.pointer_focus_target == Some(target)
                && pointer_delta.length() <= POINTER_DISPLACEMENT_THRESHOLD
            {
                self.pointer_focus_dwell += dt;
            } else {
                self.pointer_focus_dwell = dt;
                self.pointer_focus_target = Some(target);
                self.pointer_focus_armed = false;
            }
            if self.pointer_focus_dwell >= POINTER_DWELL_SECONDS && !self.pointer_focus_armed {
                self.apply_command(UiCommand::Confirm);
                self.pointer_focus_armed = true;
            }
        } else {
            self.pointer_focus_target = None;
            self.pointer_focus_dwell = 0.0;
            self.pointer_focus_armed = false;
        }

        if self.is_menu_oriented_screen() && pointer_in_back_hotzone(pointer_position) {
            if pointer_delta.length() <= POINTER_DISPLACEMENT_THRESHOLD {
                self.pointer_back_dwell += dt;
            } else {
                self.pointer_back_dwell = dt;
                self.pointer_back_armed = false;
            }
            if self.pointer_back_dwell >= POINTER_DWELL_SECONDS && !self.pointer_back_armed {
                self.apply_command(UiCommand::Back);
                self.pointer_back_armed = true;
            }
        } else {
            self.pointer_back_dwell = 0.0;
            self.pointer_back_armed = false;
        }
    }

    fn apply_pointer_focus(&mut self, target: PointerFocusTarget) {
        match target {
            PointerFocusTarget::MainMenuItem(index) => self.main_menu_cursor = index,
            PointerFocusTarget::ModeItem(index) => self.mode_cursor = index,
            PointerFocusTarget::LoadoutSlot(index) => self.loadout_state.slot_cursor = index,
            PointerFocusTarget::SettingsToggle => {}
        }
    }

    fn pointer_focus_target(&self, pointer_position: Vec2) -> Option<PointerFocusTarget> {
        match self.screen {
            ScreenState::MainMenu => {
                main_menu_item_at(pointer_position).map(PointerFocusTarget::MainMenuItem)
            }
            ScreenState::ModeSelect => {
                mode_item_at(pointer_position).map(PointerFocusTarget::ModeItem)
            }
            ScreenState::Loadout => {
                loadout_slot_at(pointer_position).map(PointerFocusTarget::LoadoutSlot)
            }
            ScreenState::Settings => {
                settings_toggle_hit(pointer_position).then_some(PointerFocusTarget::SettingsToggle)
            }
            _ => None,
        }
    }

    fn supports_scroll_navigation(&self) -> bool {
        matches!(
            self.screen,
            ScreenState::MainMenu
                | ScreenState::ModeSelect
                | ScreenState::Loadout
                | ScreenState::Leaderboard
        )
    }

    fn is_menu_oriented_screen(&self) -> bool {
        self.screen != ScreenState::Running
    }

    fn resume_from_pointer_idle_pause(&mut self, pointer_position: Option<Vec2>) {
        let Some(state) = self.running.as_mut() else {
            return;
        };
        if state.phase != RunningPhase::PointerIdlePause {
            return;
        }
        state.phase = RunningPhase::Active;
        state.pointer_idle_elapsed = 0.0;
        state.idle_grace_timer = POINTER_IDLE_GRACE_SECONDS;
        state.pointer_idle_anchor = pointer_position.or(state.pointer_idle_anchor);
    }

    fn start_mode(&mut self, mode: GameMode, requested_loadout: Option<Vec<String>>) {
        self.message = None;
        match self.engine.start_run(mode, requested_loadout) {
            Ok(run) => {
                let mut running = RunningState::new(run);
                let seed = random_seed();
                let (foods, next_seed) = spawn_food_positions(
                    seed,
                    &running.run,
                    &running.foods,
                    INITIAL_FOOD_COUNT,
                );
                running.foods = foods;
                running.spawn_seed = next_seed;
                self.running = Some(running);
                self.screen = ScreenState::Running;
            }
            Err(err) => {
                self.message = Some(err);
                self.screen = if mode == GameMode::Experimental {
                    ScreenState::Loadout
                } else {
                    ScreenState::ModeSelect
                };
            }
        }
    }

    fn update_running(&mut self, dt: f32) {
        let Some(phase) = self.running.as_ref().map(|state| state.phase) else {
            return;
        };

        match phase {
            RunningPhase::Active => {
                if let Some(state) = self.running.as_mut() {
                    state.accumulator += dt;
                }
                loop {
                    let should_step = if let Some(state) = self.running.as_mut() {
                        if state.phase == RunningPhase::Active
                            && state.accumulator >= state.tick_seconds
                        {
                            state.accumulator -= state.tick_seconds;
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    };

                    if !should_step {
                        break;
                    }
                    if self.step_running_tick() || self.screen != ScreenState::Running {
                        break;
                    }
                }
            }
            RunningPhase::Replay => {
                let should_finish = if let Some(state) = self.running.as_mut() {
                    state.replay_timer -= dt;
                    state.replay_timer <= 0.0
                } else {
                    false
                };
                if should_finish {
                    self.complete_running_session();
                }
            }
            RunningPhase::PointerIdlePause => {}
        }
    }

    fn step_running_tick(&mut self) -> bool {
        let (engine, running) = (&mut self.engine, &mut self.running);
        let Some(state) = running.as_mut() else {
            return false;
        };
        if state.phase != RunningPhase::Active {
            return false;
        }

        state.run.tick();
        if let Some(next_direction) = state.queued_directions.pop_front() {
            state.direction = next_direction;
        }

        let current_head = state.run.snake[0];
        let next_head = state.direction.step(
            current_head,
            state.run.board.width,
            state.run.board.height,
            state.run.effects.has_soft_wrap,
        );
        let eaten_food_index = state.foods.iter().position(|food| *food == next_head);
        let ate_food = eaten_food_index.is_some();
        let collides = snake_collides(next_head, &state.run, ate_food);

        if collides && state.run.grace_ticks_remaining == 0 {
            state.replay_path = state.run.snake.clone();
            let (respawn, next_seed) = next_respawn_position(state.spawn_seed, &state.run);
            state.spawn_seed = next_seed;
            if engine.handle_collision(&mut state.run, respawn).is_err() {
                state.run.ended = true;
            }
            if state.run.ended {
                if state.run.show_replay {
                    state.phase = RunningPhase::Replay;
                    state.replay_timer = REPLAY_SECONDS;
                } else {
                    self.complete_running_session();
                    return true;
                }
            }
            return false;
        }

        state.run.snake.insert(0, next_head);
        if ate_food {
            if let Some(index) = eaten_food_index {
                state.foods.remove(index);
            }
            state.run.add_food(1);
            let refill_count = if state.run.metrics.food_eaten % FOOD_REFILL_EVERY_EATEN == 0 {
                FOOD_REFILL_COUNT
            } else {
                0
            };
            let (foods, next_seed) =
                spawn_food_positions(state.spawn_seed, &state.run, &state.foods, refill_count);
            state.foods.extend(foods);
            state.spawn_seed = next_seed;
        } else {
            state.run.snake.pop();
        }
        false
    }

    fn complete_running_session(&mut self) {
        let Some(state) = self.running.take() else {
            return;
        };

        let score = policy_for(state.run.mode).score(&state.run.metrics, &state.run.effects);
        let loadout_summary = state
            .run
            .active_loadout
            .as_ref()
            .map(|loadout| loadout.summary())
            .unwrap_or_else(|| "none".to_string());

        if let Err(err) = self.engine.finish_run(&state.run) {
            self.message = Some(err);
        }

        self.summary = Some(RunSummary {
            mode: state.run.mode,
            score,
            survival_ticks: state.run.metrics.survival_ticks,
            food_eaten: state.run.metrics.food_eaten,
            growth_units: state.run.metrics.growth_units,
            loadout_summary,
        });
        self.leaderboard_mode_cursor = mode_index(state.run.mode);
        self.screen = ScreenState::Summary;
    }

    pub fn draw(&self) {
        clear_background(Color::from_rgba(13, 20, 26, 255));

        match self.screen {
            ScreenState::MainMenu => self.draw_main_menu(),
            ScreenState::ModeSelect => self.draw_mode_select(),
            ScreenState::Loadout => self.draw_loadout(),
            ScreenState::Running => self.draw_running(),
            ScreenState::Summary => self.draw_summary(),
            ScreenState::Leaderboard => self.draw_leaderboard(),
            ScreenState::Settings => self.draw_settings(),
        }
        self.draw_message();
    }

    fn draw_main_menu(&self) {
        draw_title("Snake GUI Demo");
        draw_text("Main Menu", 80.0, 140.0, 40.0, WHITE);
        for (i, item) in MAIN_MENU_ITEMS.iter().enumerate() {
            let y = 210.0 + i as f32 * 50.0;
            let color = if i == self.main_menu_cursor {
                YELLOW
            } else {
                LIGHTGRAY
            };
            draw_text(item, 100.0, y, 34.0, color);
        }
        draw_text(
            "Arrow/WASD or pointer hover+dwell. Scroll navigates. Top-left dwell = Back.",
            80.0,
            430.0,
            24.0,
            GRAY,
        );
    }

    fn draw_mode_select(&self) {
        draw_title("Select Mode");
        draw_text("Mode Select", 80.0, 130.0, 40.0, WHITE);
        for (i, mode) in MODES.iter().enumerate() {
            let y = 200.0 + i as f32 * 52.0;
            let color = if i == self.mode_cursor {
                Color::from_rgba(95, 242, 153, 255)
            } else {
                LIGHTGRAY
            };
            draw_text(mode_label(*mode), 100.0, y, 34.0, color);
        }
        draw_text(
            "Enter or dwell: Start    Esc or back hotzone: Back",
            80.0,
            460.0,
            24.0,
            GRAY,
        );
    }

    fn draw_loadout(&self) {
        draw_title("Experimental Loadout");
        draw_text(
            "Experimental Loadout (3 Slots Required)",
            80.0,
            120.0,
            38.0,
            WHITE,
        );
        draw_text(
            "Slot Focus: Up/Down or pointer hover    Change Tool: Left/Right or scroll",
            80.0,
            165.0,
            24.0,
            GRAY,
        );

        let tool_ids = self.tool_ids();
        for slot in 0..3 {
            let y = 230.0 + slot as f32 * 90.0;
            let focused = slot == self.loadout_state.slot_cursor;
            let border = if focused { YELLOW } else { DARKGRAY };
            draw_rectangle_lines(80.0, y - 42.0, 830.0, 62.0, 2.0, border);
            let index = self.loadout_state.selected_tool_indices[slot] % tool_ids.len();
            let tool_id = &tool_ids[index];
            let tool = self.engine.registry.tool(tool_id.as_str()).unwrap();
            let unlocked = self.engine.profile.unlocked_tool_ids.contains(tool_id);
            let category = tool_category_label(tool.category);
            let status = if unlocked { "Unlocked" } else { "Locked" };
            let color = if unlocked {
                Color::from_rgba(95, 242, 153, 255)
            } else {
                Color::from_rgba(255, 119, 119, 255)
            };

            draw_text(
                &format!("Slot {}: {}", slot + 1, tool_id),
                100.0,
                y,
                30.0,
                WHITE,
            );
            draw_text(&format!("{category} | {status}"), 420.0, y, 28.0, color);
        }

        draw_text(
            "Enter/dwell: Start Experimental    Esc/back hotzone: Back",
            80.0,
            560.0,
            24.0,
            GRAY,
        );
        draw_text(
            "Locked tools cannot be equipped. Unlock via invincible cumulative length: 15/40/80/140...",
            80.0,
            600.0,
            22.0,
            LIGHTGRAY,
        );
    }

    fn draw_running(&self) {
        let Some(state) = self.running.as_ref() else {
            return;
        };

        draw_title("Running");
        let score = policy_for(state.run.mode).score(&state.run.metrics, &state.run.effects);
        draw_text(
            &format!(
                "Mode: {}   Score: {}   Ticks: {}   Food: {}   Growth: {}",
                mode_label(state.run.mode),
                score,
                state.run.metrics.survival_ticks,
                state.run.metrics.food_eaten,
                state.run.metrics.growth_units
            ),
            40.0,
            52.0,
            30.0,
            WHITE,
        );
        draw_text(
            "Arrow/WASD or pointer movement to steer. Esc ends run.",
            40.0,
            82.0,
            24.0,
            GRAY,
        );
        draw_text(
            "Pointer outside board idle 10ms pauses. Inside board hover keeps steering.",
            40.0,
            108.0,
            22.0,
            LIGHTGRAY,
        );

        let board_width = state.run.board.width as f32 * CELL_SIZE;
        let board_height = state.run.board.height as f32 * CELL_SIZE;
        let origin_x = (screen_width() - board_width) / 2.0;
        let origin_y = 130.0;

        draw_rectangle_lines(
            origin_x - 2.0,
            origin_y - 2.0,
            board_width + 4.0,
            board_height + 4.0,
            2.0,
            GRAY,
        );

        for food in &state.foods {
            draw_cell(origin_x, origin_y, *food, Color::from_rgba(255, 90, 79, 255));
        }

        for (i, segment) in state.run.snake.iter().enumerate() {
            let color = if i == 0 {
                Color::from_rgba(127, 255, 90, 255)
            } else {
                Color::from_rgba(89, 196, 64, 255)
            };
            draw_cell(origin_x, origin_y, *segment, color);
        }

        match state.phase {
            RunningPhase::PointerIdlePause => {
                draw_text(
                    "Paused: pointer idle detected. Move pointer >2px or press arrow to resume.",
                    40.0,
                    660.0,
                    26.0,
                    YELLOW,
                );
            }
            RunningPhase::Replay => {
                draw_text(
                    "Replay on death active (mortal mode only)",
                    40.0,
                    660.0,
                    28.0,
                    YELLOW,
                );
                for segment in &state.replay_path {
                    draw_cell(
                        origin_x,
                        origin_y,
                        *segment,
                        Color::from_rgba(255, 255, 255, 38),
                    );
                }
            }
            RunningPhase::Active => {}
        }
    }

    fn draw_summary(&self) {
        draw_title("Run Summary");
        draw_text("Summary", 80.0, 120.0, 40.0, WHITE);
        if let Some(summary) = &self.summary {
            draw_text(
                &format!("Mode: {}", mode_label(summary.mode)),
                100.0,
                200.0,
                34.0,
                LIGHTGRAY,
            );
            draw_text(
                &format!("Score: {}", summary.score),
                100.0,
                245.0,
                34.0,
                LIGHTGRAY,
            );
            draw_text(
                &format!("Survival Ticks: {}", summary.survival_ticks),
                100.0,
                290.0,
                34.0,
                LIGHTGRAY,
            );
            draw_text(
                &format!("Food Eaten: {}", summary.food_eaten),
                100.0,
                335.0,
                34.0,
                LIGHTGRAY,
            );
            draw_text(
                &format!("Growth Units: {}", summary.growth_units),
                100.0,
                380.0,
                34.0,
                LIGHTGRAY,
            );
            draw_text(
                &format!("Loadout: {}", summary.loadout_summary),
                100.0,
                425.0,
                34.0,
                LIGHTGRAY,
            );
        }

        draw_text(
            "Enter/dwell: Main Menu    Right: Leaderboards    Back hotzone: Main Menu",
            80.0,
            560.0,
            24.0,
            GRAY,
        );
    }

    fn draw_leaderboard(&self) {
        draw_title("Leaderboards");
        let mode = MODES[self.leaderboard_mode_cursor];
        draw_text(
            &format!("Leaderboard: {}", mode_label(mode)),
            80.0,
            120.0,
            40.0,
            WHITE,
        );
        draw_text(
            "Rows include mode, score, survival, and loadout metadata",
            80.0,
            160.0,
            24.0,
            GRAY,
        );

        for (row_index, row) in self
            .engine
            .leaderboards
            .rows(mode)
            .iter()
            .take(10)
            .enumerate()
        {
            let y = 230.0 + row_index as f32 * 42.0;
            draw_text(
                &format!(
                    "{:02}. mode={} score={} ticks={} loadout={}",
                    row_index + 1,
                    mode_label(row.mode),
                    row.score,
                    row.survival_ticks,
                    row.loadout_summary
                ),
                100.0,
                y,
                28.0,
                LIGHTGRAY,
            );
        }
        if self.engine.leaderboards.rows(mode).is_empty() {
            draw_text("No runs yet for this mode.", 100.0, 230.0, 30.0, LIGHTGRAY);
        }

        draw_text(
            "Left/Right or scroll: Change Mode    Enter/dwell/Esc: Main Menu",
            80.0,
            640.0,
            24.0,
            GRAY,
        );
    }

    fn draw_settings(&self) {
        draw_title("Settings");
        draw_text("Settings", 80.0, 120.0, 40.0, WHITE);
        let status = if self.engine.profile.replay_on_death {
            "Enabled"
        } else {
            "Disabled"
        };
        draw_text(
            &format!("Replay On Death (Mortal Modes): {}", status),
            100.0,
            220.0,
            34.0,
            LIGHTGRAY,
        );
        draw_text(
            "Left/Right/Enter/dwell: Toggle    Esc/back hotzone: Back",
            80.0,
            300.0,
            24.0,
            GRAY,
        );
        draw_text(
            "Invincible mode always bypasses replay and continues with reposition.",
            80.0,
            360.0,
            24.0,
            LIGHTGRAY,
        );
    }

    fn draw_message(&self) {
        if let Some(message) = self.message.as_ref() {
            draw_rectangle(
                40.0,
                screen_height() - 80.0,
                screen_width() - 80.0,
                42.0,
                Color::from_rgba(35, 12, 12, 230),
            );
            draw_text(
                &format!("Message: {message}"),
                52.0,
                screen_height() - 50.0,
                24.0,
                Color::from_rgba(255, 135, 135, 255),
            );
        }
    }
}

fn draw_title(title: &str) {
    draw_text(title, 32.0, 56.0, 46.0, Color::from_rgba(95, 242, 153, 255));
}

fn draw_cell(origin_x: f32, origin_y: f32, point: Point, color: Color) {
    let x = origin_x + point.x as f32 * CELL_SIZE;
    let y = origin_y + point.y as f32 * CELL_SIZE;
    draw_rectangle(x + 1.0, y + 1.0, CELL_SIZE - 2.0, CELL_SIZE - 2.0, color);
}

fn mode_label(mode: GameMode) -> &'static str {
    match mode {
        GameMode::Practice => "practice",
        GameMode::Challenge => "challenge",
        GameMode::Experimental => "experimental",
        GameMode::Invincible => "invincible",
    }
}

fn tool_category_label(category: ToolCategory) -> &'static str {
    match category {
        ToolCategory::ControlAssist => "control-assist",
        ToolCategory::RuleModifying => "rule-modifying",
        ToolCategory::Hybrid => "hybrid",
    }
}

fn mode_index(mode: GameMode) -> usize {
    MODES
        .iter()
        .position(|candidate| *candidate == mode)
        .unwrap_or(0)
}

fn cycle_index(current: usize, delta: i32, len: usize) -> usize {
    if len == 0 {
        return 0;
    }
    let len_i = len as i32;
    (((current as i32 + delta) % len_i + len_i) % len_i) as usize
}

fn direction_from_delta(delta: Vec2) -> Option<Direction> {
    if delta.length() <= POINTER_DISPLACEMENT_THRESHOLD {
        return None;
    }
    if delta.x.abs() >= delta.y.abs() {
        if delta.x >= 0.0 {
            Some(Direction::Right)
        } else {
            Some(Direction::Left)
        }
    } else if delta.y >= 0.0 {
        Some(Direction::Down)
    } else {
        Some(Direction::Up)
    }
}

fn direction_toward_pointer(state: &RunningState, pointer_position: Vec2) -> Option<Direction> {
    let target = pointer_board_cell(state, pointer_position)?;
    let head = state.run.snake[0];
    let dx = target.x - head.x;
    let dy = target.y - head.y;
    if dx == 0 && dy == 0 {
        return None;
    }

    if dx.abs() >= dy.abs() {
        if dx > 0 {
            Some(Direction::Right)
        } else {
            Some(Direction::Left)
        }
    } else if dy > 0 {
        Some(Direction::Down)
    } else {
        Some(Direction::Up)
    }
}

fn pointer_board_cell(state: &RunningState, pointer_position: Vec2) -> Option<Point> {
    let board_width = state.run.board.width as f32 * CELL_SIZE;
    let board_height = state.run.board.height as f32 * CELL_SIZE;
    let origin_x = (ui_screen_width() - board_width) / 2.0;
    let origin_y = 130.0;

    if pointer_position.x < origin_x
        || pointer_position.x >= origin_x + board_width
        || pointer_position.y < origin_y
        || pointer_position.y >= origin_y + board_height
    {
        return None;
    }

    let x = ((pointer_position.x - origin_x) / CELL_SIZE).floor() as i32;
    let y = ((pointer_position.y - origin_y) / CELL_SIZE).floor() as i32;
    Some(Point { x, y })
}

#[cfg(test)]
fn ui_screen_width() -> f32 {
    WINDOW_WIDTH as f32
}

#[cfg(not(test))]
fn ui_screen_width() -> f32 {
    screen_width()
}

fn main_menu_item_at(pointer_position: Vec2) -> Option<usize> {
    if pointer_position.x < 80.0 || pointer_position.x > 480.0 {
        return None;
    }
    MAIN_MENU_ITEMS.iter().enumerate().find_map(|(index, _)| {
        let y = 210.0 + index as f32 * 50.0;
        let top = y - 36.0;
        let bottom = y + 12.0;
        (pointer_position.y >= top && pointer_position.y <= bottom).then_some(index)
    })
}

fn mode_item_at(pointer_position: Vec2) -> Option<usize> {
    if pointer_position.x < 80.0 || pointer_position.x > 520.0 {
        return None;
    }
    MODES.iter().enumerate().find_map(|(index, _)| {
        let y = 200.0 + index as f32 * 52.0;
        let top = y - 36.0;
        let bottom = y + 12.0;
        (pointer_position.y >= top && pointer_position.y <= bottom).then_some(index)
    })
}

fn loadout_slot_at(pointer_position: Vec2) -> Option<usize> {
    if pointer_position.x < 80.0 || pointer_position.x > 910.0 {
        return None;
    }
    (0..3).find(|slot| {
        let y = 230.0 + *slot as f32 * 90.0;
        pointer_position.y >= (y - 42.0) && pointer_position.y <= (y + 20.0)
    })
}

fn settings_toggle_hit(pointer_position: Vec2) -> bool {
    pointer_position.x >= 90.0
        && pointer_position.x <= 910.0
        && pointer_position.y >= 185.0
        && pointer_position.y <= 245.0
}

fn pointer_in_back_hotzone(pointer_position: Vec2) -> bool {
    pointer_position.x >= 16.0
        && pointer_position.x <= 136.0
        && pointer_position.y >= 18.0
        && pointer_position.y <= 72.0
}

fn is_within_board(point: Point, board_width: i32, board_height: i32) -> bool {
    point.x >= 0 && point.y >= 0 && point.x < board_width && point.y < board_height
}

fn snake_collides(next_head: Point, run: &GameRun, ate_food: bool) -> bool {
    if !is_within_board(next_head, run.board.width, run.board.height) {
        return true;
    }
    let body_limit = if ate_food {
        run.snake.len()
    } else {
        run.snake.len().saturating_sub(1)
    };
    run.snake
        .iter()
        .take(body_limit)
        .any(|segment| *segment == next_head)
}

fn spawn_food_positions(
    mut seed: u64,
    run: &GameRun,
    existing_foods: &[Point],
    count: usize,
) -> (Vec<Point>, u64) {
    let mut spawned = Vec::with_capacity(count);
    for _ in 0..count {
        let occupied: Vec<Point> = existing_foods
            .iter()
            .copied()
            .chain(spawned.iter().copied())
            .collect();
        let (food, next_seed) = next_food_position(seed, run, &occupied);
        seed = next_seed;
        spawned.push(food);
    }
    (spawned, seed)
}

fn next_food_position(seed: u64, run: &GameRun, occupied_foods: &[Point]) -> (Point, u64) {
    let width = run.board.width.max(1) as usize;
    let height = run.board.height.max(1) as usize;
    let total = width.saturating_mul(height).max(1);
    let mut rng = seed;

    for _ in 0..total.saturating_mul(2) {
        rng = lcg_next(rng);
        let idx = (rng as usize) % total;
        let x = (idx % width) as i32;
        let y = (idx / width) as i32;
        let candidate = Point { x, y };
        if !run.snake.contains(&candidate)
            && !occupied_foods.contains(&candidate)
            && !occupied_foods
                .iter()
                .any(|food| points_touch_or_adjacent(*food, candidate))
        {
            return (candidate, rng);
        }
    }

    for idx in 0..total {
        let x = (idx % width) as i32;
        let y = (idx / width) as i32;
        let candidate = Point { x, y };
        if !run.snake.contains(&candidate) && !occupied_foods.contains(&candidate) {
            return (candidate, lcg_next(rng));
        }
    }

    (run.snake[0], lcg_next(rng))
}

fn points_touch_or_adjacent(a: Point, b: Point) -> bool {
    (a.x - b.x).abs() <= 1 && (a.y - b.y).abs() <= 1
}

fn lcg_next(seed: u64) -> u64 {
    seed.wrapping_mul(6364136223846793005).wrapping_add(1)
}

fn random_seed() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0)
}

fn next_respawn_position(seed: u64, run: &GameRun) -> (Point, u64) {
    let width = run.board.width.max(1) as usize;
    let height = run.board.height.max(1) as usize;
    let total = width.saturating_mul(height).max(1);
    let start = seed as usize % total;

    for offset in 0..total {
        let idx = (start + offset) % total;
        let x = (idx % width) as i32;
        let y = (idx / width) as i32;
        let candidate = Point { x, y };
        if !run.snake.contains(&candidate) {
            return (candidate, seed.wrapping_add(1));
        }
    }

    (run.snake[0], seed.wrapping_add(1))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DEFAULT_THRESHOLDS;

    fn unlocked_profile() -> Profile {
        let mut profile = Profile::default();
        profile.invincible_cumulative_length = 200;
        profile.apply_threshold_unlocks(&crate::ToolRegistry::demo(), &DEFAULT_THRESHOLDS);
        profile
    }

    #[test]
    fn state_machine_reaches_all_required_screens() {
        let mut app = SnakeGuiApp::new();
        assert_eq!(app.screen, ScreenState::MainMenu);

        app.apply_command(UiCommand::Confirm);
        assert_eq!(app.screen, ScreenState::ModeSelect);

        app.apply_command(UiCommand::Right);
        app.apply_command(UiCommand::Right);
        app.apply_command(UiCommand::Confirm);
        assert_eq!(app.screen, ScreenState::Loadout);

        app.apply_command(UiCommand::Back);
        assert_eq!(app.screen, ScreenState::ModeSelect);

        app.apply_command(UiCommand::Back);
        assert_eq!(app.screen, ScreenState::MainMenu);

        app.apply_command(UiCommand::Down);
        app.apply_command(UiCommand::Confirm);
        assert_eq!(app.screen, ScreenState::Leaderboard);

        app.apply_command(UiCommand::Back);
        app.apply_command(UiCommand::Down);
        app.apply_command(UiCommand::Confirm);
        assert_eq!(app.screen, ScreenState::Settings);
    }

    #[test]
    fn direction_queue_applies_one_turn_per_tick() {
        let mut app = SnakeGuiApp::new();
        app.start_mode(GameMode::Practice, None);
        assert_eq!(app.screen, ScreenState::Running);

        app.enqueue_direction(Direction::Up);
        app.enqueue_direction(Direction::Left);

        let before = app.running.as_ref().unwrap().direction;
        assert_eq!(before, Direction::Right);

        app.step_running_tick();
        let after_first = app.running.as_ref().unwrap().direction;
        assert_eq!(after_first, Direction::Up);

        app.step_running_tick();
        let after_second = app.running.as_ref().unwrap().direction;
        assert_eq!(after_second, Direction::Left);
    }

    #[test]
    fn gui_flow_can_start_and_complete_each_mode() {
        let mut app = SnakeGuiApp::with_profile(unlocked_profile());

        for mode in MODES {
            let requested = if mode == GameMode::Experimental {
                Some(vec![
                    "turn-buffer".to_string(),
                    "slow-window".to_string(),
                    "soft-wrap".to_string(),
                ])
            } else {
                None
            };

            app.start_mode(mode, requested);
            assert_eq!(app.screen, ScreenState::Running);

            if mode == GameMode::Invincible {
                {
                    let running = app.running.as_mut().unwrap();
                    running.run.ended = true;
                }
                app.complete_running_session();
            } else {
                let (engine, running_opt) = (&mut app.engine, &mut app.running);
                let running = running_opt.as_mut().unwrap();
                engine
                    .handle_collision(&mut running.run, Point { x: -1, y: 0 })
                    .unwrap();
                app.complete_running_session();
            }

            assert_eq!(app.screen, ScreenState::Summary);
            assert!(app.summary.is_some());
            app.apply_command(UiCommand::Confirm);
            assert_eq!(app.screen, ScreenState::MainMenu);
        }
    }

    fn enter_pointer_idle_pause(app: &mut SnakeGuiApp) {
        app.start_mode(GameMode::Practice, None);
        assert_eq!(app.screen, ScreenState::Running);
        let outside_board = vec2(40.0, 90.0);
        app.apply_pointer_input(0.004, outside_board, 0.0);
        app.apply_pointer_input(0.007, outside_board, 0.0);
        assert_eq!(
            app.running.as_ref().unwrap().phase,
            RunningPhase::PointerIdlePause
        );
    }

    #[test]
    fn pointer_idle_pause_enters_and_resumes_with_keyboard() {
        let mut app = SnakeGuiApp::new();
        enter_pointer_idle_pause(&mut app);

        app.apply_command(UiCommand::Up);

        let running = app.running.as_ref().unwrap();
        assert_eq!(running.phase, RunningPhase::Active);
        assert_eq!(
            running.queued_directions.front().copied(),
            Some(Direction::Up)
        );
    }

    #[test]
    fn pointer_idle_pause_resumes_with_pointer_motion() {
        let mut app = SnakeGuiApp::new();
        enter_pointer_idle_pause(&mut app);

        app.apply_pointer_input(0.01, vec2(325.5, 320.0), 0.0);

        assert_eq!(app.running.as_ref().unwrap().phase, RunningPhase::Active);
    }

    #[test]
    fn pointer_edge_intent_does_not_trigger_idle_pause() {
        let mut app = SnakeGuiApp::new();
        app.start_mode(GameMode::Practice, None);
        assert_eq!(app.screen, ScreenState::Running);

        let hold_top_edge = vec2(470.0, 132.0);
        app.apply_pointer_input(0.01, hold_top_edge, 0.0);
        app.apply_pointer_input(0.30, hold_top_edge, 0.0);
        app.apply_pointer_input(0.30, hold_top_edge, 0.0);

        assert_eq!(app.running.as_ref().unwrap().phase, RunningPhase::Active);
    }

    #[test]
    fn pointer_hover_inside_board_does_not_trigger_idle_pause() {
        let mut app = SnakeGuiApp::new();
        app.start_mode(GameMode::Practice, None);
        assert_eq!(app.screen, ScreenState::Running);

        let inside_board = vec2(484.0, 306.0);
        app.apply_pointer_input(0.30, inside_board, 0.0);
        app.apply_pointer_input(0.30, inside_board, 0.0);

        assert_eq!(app.running.as_ref().unwrap().phase, RunningPhase::Active);
    }

    #[test]
    fn pointer_hover_outside_board_for_10ms_triggers_idle_pause() {
        let mut app = SnakeGuiApp::new();
        app.start_mode(GameMode::Practice, None);
        assert_eq!(app.screen, ScreenState::Running);

        let outside_board = vec2(40.0, 90.0);
        app.apply_pointer_input(0.005, outside_board, 0.0);
        assert_eq!(app.running.as_ref().unwrap().phase, RunningPhase::Active);

        app.apply_pointer_input(0.005, outside_board, 0.0);
        assert_eq!(
            app.running.as_ref().unwrap().phase,
            RunningPhase::PointerIdlePause
        );
    }

    #[test]
    fn pointer_hover_inside_board_steers_toward_pointer() {
        let mut app = SnakeGuiApp::new();
        app.start_mode(GameMode::Practice, None);
        assert_eq!(app.screen, ScreenState::Running);

        // Head starts near (5,5); hovering above it should request an Up turn.
        let inside_board_up = vec2(484.0, 210.0);
        app.apply_pointer_input(0.005, inside_board_up, 0.0);

        let queued = app
            .running
            .as_ref()
            .unwrap()
            .queued_directions
            .front()
            .copied();
        assert_eq!(queued, Some(Direction::Up));
        assert_eq!(app.running.as_ref().unwrap().phase, RunningPhase::Active);
    }

    #[test]
    fn initial_foods_are_non_contiguous() {
        let mut app = SnakeGuiApp::new();
        app.start_mode(GameMode::Practice, None);
        let foods = &app.running.as_ref().unwrap().foods;
        assert_eq!(foods.len(), INITIAL_FOOD_COUNT);

        for (i, food_a) in foods.iter().enumerate() {
            for food_b in foods.iter().skip(i + 1) {
                assert!(!points_touch_or_adjacent(*food_a, *food_b));
            }
        }
    }

    #[test]
    fn pointer_navigation_matches_menu_traversal_and_confirmation() {
        let mut app = SnakeGuiApp::new();
        assert_eq!(app.screen, ScreenState::MainMenu);

        app.apply_pointer_input(0.01, vec2(110.0, 260.0), 0.0);
        assert_eq!(app.main_menu_cursor, 1);

        app.apply_pointer_input(0.5, vec2(110.0, 260.0), 0.0);
        assert_eq!(app.screen, ScreenState::Leaderboard);

        app.apply_command(UiCommand::Back);
        app.apply_pointer_input(0.01, vec2(110.0, 210.0), 0.0);
        app.apply_command(UiCommand::Confirm);
        assert_eq!(app.screen, ScreenState::ModeSelect);

        app.apply_pointer_input(0.01, vec2(760.0, 90.0), -1.0);
        assert_eq!(app.mode_cursor, 1);

        app.apply_pointer_input(0.01, vec2(760.0, 90.0), 1.0);
        assert_eq!(app.mode_cursor, 0);
    }

    #[test]
    fn gui_menu_has_no_daily_weekly_or_literary_surfaces() {
        assert!(MAIN_MENU_ITEMS.iter().all(|item| !item.contains("Daily")
            && !item.contains("Weekly")
            && !item.contains("Literary")));
    }
}
