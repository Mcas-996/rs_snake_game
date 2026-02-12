use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, HashMap};

pub mod gui;

pub const CURRENT_SCHEMA_VERSION: u32 = 2;
pub const DEFAULT_THRESHOLDS: [u64; 4] = [15, 40, 80, 140];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum GameMode {
    Practice,
    Challenge,
    Experimental,
    Invincible,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollisionOutcome {
    Die,
    Reposition,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolCategory {
    ControlAssist,
    RuleModifying,
    Hybrid,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolDefinition {
    pub id: String,
    pub category: ToolCategory,
    pub unlock_threshold: Option<u64>,
    pub incompatible_with: BTreeSet<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolLoadout {
    pub slots: [String; 3],
}

impl ToolLoadout {
    pub fn summary(&self) -> String {
        self.slots.join("+")
    }
}

#[derive(Debug, Clone)]
pub struct ToolRegistry {
    tools: BTreeMap<String, ToolDefinition>,
}

impl ToolRegistry {
    pub fn demo() -> Self {
        let mut tools = BTreeMap::new();
        tools.insert(
            "turn-buffer".to_string(),
            ToolDefinition {
                id: "turn-buffer".to_string(),
                category: ToolCategory::ControlAssist,
                unlock_threshold: Some(15),
                incompatible_with: BTreeSet::new(),
            },
        );
        tools.insert(
            "slow-window".to_string(),
            ToolDefinition {
                id: "slow-window".to_string(),
                category: ToolCategory::ControlAssist,
                unlock_threshold: Some(40),
                incompatible_with: BTreeSet::new(),
            },
        );
        tools.insert(
            "soft-wrap".to_string(),
            ToolDefinition {
                id: "soft-wrap".to_string(),
                category: ToolCategory::RuleModifying,
                unlock_threshold: Some(80),
                incompatible_with: BTreeSet::new(),
            },
        );
        tools.insert(
            "rewind-step".to_string(),
            ToolDefinition {
                id: "rewind-step".to_string(),
                category: ToolCategory::Hybrid,
                unlock_threshold: Some(140),
                incompatible_with: BTreeSet::new(),
            },
        );
        Self { tools }
    }

    pub fn tool(&self, id: &str) -> Option<&ToolDefinition> {
        self.tools.get(id)
    }

    pub fn list(&self) -> impl Iterator<Item = &ToolDefinition> {
        self.tools.values()
    }

    pub fn validate_loadout(
        &self,
        unlocked: &BTreeSet<String>,
        slots: &[String],
    ) -> Result<ToolLoadout, String> {
        if slots.len() != 3 {
            return Err("experimental loadout requires exactly three slots".to_string());
        }
        let mut seen = BTreeSet::new();
        for slot in slots {
            let Some(def) = self.tool(slot) else {
                return Err(format!("unknown tool: {slot}"));
            };
            if !unlocked.contains(slot) {
                return Err(format!("tool not unlocked: {slot}"));
            }
            for incompatible in &def.incompatible_with {
                if seen.contains(incompatible) {
                    return Err(format!(
                        "tool {slot} is incompatible with already selected tool {incompatible}"
                    ));
                }
            }
            seen.insert(slot.clone());
        }

        Ok(ToolLoadout {
            slots: [slots[0].clone(), slots[1].clone(), slots[2].clone()],
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Board {
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    fn within(self, board: Board) -> bool {
        self.x >= 0 && self.y >= 0 && self.x < board.width && self.y < board.height
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunMetrics {
    pub food_eaten: u64,
    pub growth_units: u64,
    pub survival_ticks: u64,
}

impl Default for RunMetrics {
    fn default() -> Self {
        Self {
            food_eaten: 0,
            growth_units: 0,
            survival_ticks: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeaderboardEntry {
    pub mode: GameMode,
    pub score: u64,
    pub survival_ticks: u64,
    pub loadout_summary: String,
}

#[derive(Debug, Clone, Default)]
pub struct Leaderboards {
    by_mode: HashMap<GameMode, Vec<LeaderboardEntry>>,
}

impl Leaderboards {
    pub fn submit(&mut self, entry: LeaderboardEntry) {
        let rows = self.by_mode.entry(entry.mode).or_default();
        rows.push(entry);
        rows.sort_by(|a, b| compare_entries(a, b));
    }

    pub fn rows(&self, mode: GameMode) -> &[LeaderboardEntry] {
        self.by_mode.get(&mode).map(Vec::as_slice).unwrap_or(&[])
    }
}

fn compare_entries(a: &LeaderboardEntry, b: &LeaderboardEntry) -> Ordering {
    match a.mode {
        GameMode::Challenge => b
            .survival_ticks
            .cmp(&a.survival_ticks)
            .then_with(|| b.score.cmp(&a.score)),
        _ => b.score.cmp(&a.score),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LegacyProfile {
    pub best_score: u64,
    pub replay_on_death: Option<bool>,
    pub schema_version: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Profile {
    pub schema_version: u32,
    pub replay_on_death: bool,
    pub invincible_cumulative_length: u64,
    pub unlocked_tool_ids: BTreeSet<String>,
    pub old_best_score: Option<u64>,
}

impl Default for Profile {
    fn default() -> Self {
        Self {
            schema_version: CURRENT_SCHEMA_VERSION,
            replay_on_death: false,
            invincible_cumulative_length: 0,
            unlocked_tool_ids: BTreeSet::new(),
            old_best_score: None,
        }
    }
}

impl Profile {
    pub fn from_legacy(legacy: LegacyProfile) -> Result<Self, String> {
        let mut profile = Self::default();
        profile.schema_version = legacy.schema_version.unwrap_or(1);
        if profile.schema_version > CURRENT_SCHEMA_VERSION {
            return Err(format!(
                "profile schema {} is newer than supported {}",
                profile.schema_version, CURRENT_SCHEMA_VERSION
            ));
        }
        profile.replay_on_death = legacy.replay_on_death.unwrap_or(false);
        profile.old_best_score = Some(legacy.best_score);
        migrate_profile(profile)
    }

    pub fn apply_threshold_unlocks(&mut self, registry: &ToolRegistry, thresholds: &[u64]) {
        let mut next = BTreeSet::new();
        for tool in registry.list() {
            if let Some(threshold) = tool.unlock_threshold {
                if thresholds.contains(&threshold) && self.invincible_cumulative_length >= threshold
                {
                    next.insert(tool.id.clone());
                }
            }
        }
        self.unlocked_tool_ids = next;
    }
}

pub fn migrate_profile(mut profile: Profile) -> Result<Profile, String> {
    if profile.schema_version > CURRENT_SCHEMA_VERSION {
        return Err(format!(
            "profile schema {} is newer than supported {}",
            profile.schema_version, CURRENT_SCHEMA_VERSION
        ));
    }
    if profile.schema_version < CURRENT_SCHEMA_VERSION {
        profile.replay_on_death = profile.replay_on_death && profile.schema_version >= 1;
        profile.schema_version = CURRENT_SCHEMA_VERSION;
    }
    Ok(profile)
}

pub trait ModePolicy {
    fn mode(&self) -> GameMode;
    fn collision_outcome(&self) -> CollisionOutcome;
    fn score(&self, metrics: &RunMetrics, effects: &ActiveEffects) -> u64;
    fn run_end_state(&self, replay_on_death: bool) -> RunEnd;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunEnd {
    Continue,
    End { show_replay: bool },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ActiveEffects {
    pub score_bonus_percent: u64,
    pub has_turn_buffer: bool,
    pub has_slow_window: bool,
    pub has_soft_wrap: bool,
    pub has_rewind_step: bool,
}

fn effects_from_loadout(loadout: Option<&ToolLoadout>) -> ActiveEffects {
    let mut effects = ActiveEffects::default();
    if let Some(loadout) = loadout {
        for slot in &loadout.slots {
            match slot.as_str() {
                "turn-buffer" => effects.has_turn_buffer = true,
                "slow-window" => effects.has_slow_window = true,
                "soft-wrap" => {
                    effects.has_soft_wrap = true;
                    effects.score_bonus_percent += 5;
                }
                "rewind-step" => {
                    effects.has_rewind_step = true;
                    effects.score_bonus_percent += 10;
                }
                _ => {}
            }
        }
    }
    effects
}

pub struct PracticePolicy;
pub struct ChallengePolicy;
pub struct ExperimentalPolicy;
pub struct InvinciblePolicy;

impl ModePolicy for PracticePolicy {
    fn mode(&self) -> GameMode {
        GameMode::Practice
    }

    fn collision_outcome(&self) -> CollisionOutcome {
        CollisionOutcome::Die
    }

    fn score(&self, metrics: &RunMetrics, _effects: &ActiveEffects) -> u64 {
        metrics.food_eaten * 10
    }

    fn run_end_state(&self, replay_on_death: bool) -> RunEnd {
        RunEnd::End {
            show_replay: replay_on_death,
        }
    }
}

impl ModePolicy for ChallengePolicy {
    fn mode(&self) -> GameMode {
        GameMode::Challenge
    }

    fn collision_outcome(&self) -> CollisionOutcome {
        CollisionOutcome::Die
    }

    fn score(&self, metrics: &RunMetrics, effects: &ActiveEffects) -> u64 {
        let base = metrics
            .survival_ticks
            .saturating_mul(1_000)
            .saturating_add(metrics.food_eaten * 10);
        base.saturating_add(base.saturating_mul(effects.score_bonus_percent) / 100)
    }

    fn run_end_state(&self, replay_on_death: bool) -> RunEnd {
        RunEnd::End {
            show_replay: replay_on_death,
        }
    }
}

impl ModePolicy for ExperimentalPolicy {
    fn mode(&self) -> GameMode {
        GameMode::Experimental
    }

    fn collision_outcome(&self) -> CollisionOutcome {
        CollisionOutcome::Die
    }

    fn score(&self, metrics: &RunMetrics, effects: &ActiveEffects) -> u64 {
        let base = metrics
            .food_eaten
            .saturating_mul(12)
            .saturating_add(metrics.survival_ticks / 5);
        base.saturating_add(base.saturating_mul(effects.score_bonus_percent) / 100)
    }

    fn run_end_state(&self, replay_on_death: bool) -> RunEnd {
        RunEnd::End {
            show_replay: replay_on_death,
        }
    }
}

impl ModePolicy for InvinciblePolicy {
    fn mode(&self) -> GameMode {
        GameMode::Invincible
    }

    fn collision_outcome(&self) -> CollisionOutcome {
        CollisionOutcome::Reposition
    }

    fn score(&self, metrics: &RunMetrics, effects: &ActiveEffects) -> u64 {
        let base = metrics
            .food_eaten
            .saturating_mul(8)
            .saturating_add(metrics.survival_ticks / 10);
        base.saturating_add(base.saturating_mul(effects.score_bonus_percent) / 100)
    }

    fn run_end_state(&self, _replay_on_death: bool) -> RunEnd {
        RunEnd::Continue
    }
}

pub fn policy_for(mode: GameMode) -> Box<dyn ModePolicy> {
    match mode {
        GameMode::Practice => Box::new(PracticePolicy),
        GameMode::Challenge => Box::new(ChallengePolicy),
        GameMode::Experimental => Box::new(ExperimentalPolicy),
        GameMode::Invincible => Box::new(InvinciblePolicy),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameRun {
    pub mode: GameMode,
    pub board: Board,
    pub snake: Vec<Point>,
    pub metrics: RunMetrics,
    pub ended: bool,
    pub show_replay: bool,
    pub grace_ticks_remaining: u8,
    pub active_loadout: Option<ToolLoadout>,
    pub effects: ActiveEffects,
}

impl GameRun {
    pub fn tick(&mut self) {
        self.metrics.survival_ticks = self.metrics.survival_ticks.saturating_add(1);
        if self.grace_ticks_remaining > 0 {
            self.grace_ticks_remaining -= 1;
        }
    }

    pub fn add_food(&mut self, growth: u64) {
        self.metrics.food_eaten = self.metrics.food_eaten.saturating_add(1);
        self.metrics.growth_units = self.metrics.growth_units.saturating_add(growth);
    }

    pub fn runtime_loadout(&self) -> Option<&ToolLoadout> {
        self.active_loadout.as_ref()
    }

    pub fn update_runtime_loadout(&mut self, _new_loadout: ToolLoadout) -> Result<(), String> {
        Err("active loadout is immutable during a run".to_string())
    }
}

#[derive(Debug, Clone)]
pub struct GameEngine {
    pub profile: Profile,
    pub leaderboards: Leaderboards,
    pub registry: ToolRegistry,
    pub thresholds: Vec<u64>,
    pub board: Board,
}

impl GameEngine {
    pub fn new(profile: Profile) -> Self {
        let registry = ToolRegistry::demo();
        let thresholds = DEFAULT_THRESHOLDS.to_vec();
        let mut profile = migrate_profile(profile).unwrap_or_default();
        profile.apply_threshold_unlocks(&registry, &thresholds);
        Self {
            profile,
            leaderboards: Leaderboards::default(),
            registry,
            thresholds,
            board: Board {
                width: 12,
                height: 12,
            },
        }
    }

    pub fn enable_replay(&mut self, enabled: bool) {
        self.profile.replay_on_death = enabled;
    }

    pub fn start_run(
        &self,
        mode: GameMode,
        requested_loadout: Option<Vec<String>>,
    ) -> Result<GameRun, String> {
        let loadout = match mode {
            GameMode::Experimental => {
                let slots = requested_loadout.ok_or_else(|| {
                    "experimental mode requires selecting three unlocked tools".to_string()
                })?;
                Some(
                    self.registry
                        .validate_loadout(&self.profile.unlocked_tool_ids, &slots)?,
                )
            }
            _ => None,
        };

        let effects = effects_from_loadout(loadout.as_ref());
        Ok(GameRun {
            mode,
            board: self.board,
            snake: vec![
                Point { x: 5, y: 5 },
                Point { x: 4, y: 5 },
                Point { x: 3, y: 5 },
            ],
            metrics: RunMetrics::default(),
            ended: false,
            show_replay: false,
            grace_ticks_remaining: 0,
            active_loadout: loadout,
            effects,
        })
    }

    pub fn handle_collision(
        &mut self,
        run: &mut GameRun,
        candidate_respawn: Point,
    ) -> Result<(), String> {
        if run.ended {
            return Err("run has already ended".to_string());
        }

        let policy = policy_for(run.mode);
        match policy.collision_outcome() {
            CollisionOutcome::Die => {
                run.ended = true;
                run.show_replay = matches!(
                    policy.run_end_state(self.profile.replay_on_death),
                    RunEnd::End { show_replay: true }
                );
            }
            CollisionOutcome::Reposition => {
                let safe = self.find_safe_respawn(run, candidate_respawn)?;
                if let Some(head) = run.snake.first_mut() {
                    *head = safe;
                }
                run.grace_ticks_remaining = 1;
            }
        }
        Ok(())
    }

    pub fn finish_run(&mut self, run: &GameRun) -> Result<(), String> {
        let policy = policy_for(run.mode);
        let score = policy.score(&run.metrics, &run.effects);

        if run.mode == GameMode::Invincible {
            self.profile.invincible_cumulative_length = self
                .profile
                .invincible_cumulative_length
                .saturating_add(run.metrics.growth_units);
            self.profile
                .apply_threshold_unlocks(&self.registry, &self.thresholds);
        }

        let loadout_summary = run
            .active_loadout
            .as_ref()
            .map(ToolLoadout::summary)
            .unwrap_or_else(|| "none".to_string());

        self.leaderboards.submit(LeaderboardEntry {
            mode: run.mode,
            score,
            survival_ticks: run.metrics.survival_ticks,
            loadout_summary,
        });
        Ok(())
    }

    fn find_safe_respawn(&self, run: &GameRun, desired: Point) -> Result<Point, String> {
        if desired.within(run.board) && !run.snake.contains(&desired) {
            return Ok(desired);
        }

        for y in 0..run.board.height {
            for x in 0..run.board.width {
                let p = Point { x, y };
                if !run.snake.contains(&p) {
                    return Ok(p);
                }
            }
        }

        Err("no safe respawn tile found".to_string())
    }

    pub fn build_menu_items(&self) -> Vec<&'static str> {
        vec![
            "Start Practice",
            "Start Challenge",
            "Start Experimental",
            "Start Invincible",
            "Leaderboards",
            "Settings",
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn unlocked_profile() -> Profile {
        let mut profile = Profile::default();
        profile.invincible_cumulative_length = 200;
        let registry = ToolRegistry::demo();
        profile.apply_threshold_unlocks(&registry, &DEFAULT_THRESHOLDS);
        profile
    }

    #[test]
    fn policy_collision_outcomes_are_mode_scoped() {
        assert_eq!(
            policy_for(GameMode::Practice).collision_outcome(),
            CollisionOutcome::Die
        );
        assert_eq!(
            policy_for(GameMode::Challenge).collision_outcome(),
            CollisionOutcome::Die
        );
        assert_eq!(
            policy_for(GameMode::Experimental).collision_outcome(),
            CollisionOutcome::Die
        );
        assert_eq!(
            policy_for(GameMode::Invincible).collision_outcome(),
            CollisionOutcome::Reposition
        );
    }

    #[test]
    fn mortal_collision_ends_immediately_with_optional_replay() {
        let mut engine = GameEngine::new(Profile::default());
        engine.enable_replay(true);
        let mut run = engine.start_run(GameMode::Practice, None).unwrap();

        engine
            .handle_collision(&mut run, Point { x: 0, y: 0 })
            .unwrap();

        assert!(run.ended);
        assert!(run.show_replay);
    }

    #[test]
    fn invincible_collision_repositions_without_ending() {
        let mut engine = GameEngine::new(Profile::default());
        let mut run = engine.start_run(GameMode::Invincible, None).unwrap();
        let body = run.snake.clone();

        engine
            .handle_collision(&mut run, body[0])
            .expect("must reposition to safe tile");

        assert!(!run.ended);
        assert_eq!(run.grace_ticks_remaining, 1);
    }

    #[test]
    fn challenge_leaderboard_orders_by_survival_time_first() {
        let mut engine = GameEngine::new(Profile::default());

        let mut short = engine.start_run(GameMode::Challenge, None).unwrap();
        short.metrics.survival_ticks = 30;
        short.metrics.food_eaten = 100;
        engine.finish_run(&short).unwrap();

        let mut long = engine.start_run(GameMode::Challenge, None).unwrap();
        long.metrics.survival_ticks = 60;
        long.metrics.food_eaten = 1;
        engine.finish_run(&long).unwrap();

        let rows = engine.leaderboards.rows(GameMode::Challenge);
        assert_eq!(rows[0].survival_ticks, 60);
    }

    #[test]
    fn invincible_scores_are_isolated_from_other_modes() {
        let mut engine = GameEngine::new(Profile::default());

        let mut invincible = engine.start_run(GameMode::Invincible, None).unwrap();
        invincible.metrics.food_eaten = 10;
        engine.finish_run(&invincible).unwrap();

        let practice = engine.start_run(GameMode::Practice, None).unwrap();
        engine.finish_run(&practice).unwrap();

        assert_eq!(engine.leaderboards.rows(GameMode::Invincible).len(), 1);
        assert_eq!(engine.leaderboards.rows(GameMode::Practice).len(), 1);
        assert_eq!(engine.leaderboards.rows(GameMode::Challenge).len(), 0);
    }

    #[test]
    fn invincible_growth_updates_cumulative_length_only_for_invincible() {
        let mut engine = GameEngine::new(Profile::default());

        let mut invincible = engine.start_run(GameMode::Invincible, None).unwrap();
        invincible.add_food(20);
        engine.finish_run(&invincible).unwrap();
        assert_eq!(engine.profile.invincible_cumulative_length, 20);

        let mut practice = engine.start_run(GameMode::Practice, None).unwrap();
        practice.add_food(99);
        engine.finish_run(&practice).unwrap();
        assert_eq!(engine.profile.invincible_cumulative_length, 20);
    }

    #[test]
    fn threshold_unlocks_are_idempotent_and_deterministic() {
        let mut profile = Profile::default();
        let registry = ToolRegistry::demo();
        profile.invincible_cumulative_length = 80;

        profile.apply_threshold_unlocks(&registry, &DEFAULT_THRESHOLDS);
        let first = profile.unlocked_tool_ids.clone();

        profile.apply_threshold_unlocks(&registry, &DEFAULT_THRESHOLDS);
        let second = profile.unlocked_tool_ids.clone();

        assert_eq!(first, second);
        assert!(second.contains("turn-buffer"));
        assert!(second.contains("slow-window"));
        assert!(second.contains("soft-wrap"));
        assert!(!second.contains("rewind-step"));
    }

    #[test]
    fn legacy_profile_migration_applies_defaults_and_guards_newer_schema() {
        let migrated = Profile::from_legacy(LegacyProfile {
            best_score: 123,
            replay_on_death: None,
            schema_version: Some(1),
        })
        .unwrap();
        assert_eq!(migrated.schema_version, CURRENT_SCHEMA_VERSION);
        assert_eq!(migrated.old_best_score, Some(123));

        let err = Profile::from_legacy(LegacyProfile {
            best_score: 0,
            replay_on_death: Some(true),
            schema_version: Some(CURRENT_SCHEMA_VERSION + 1),
        });
        assert!(err.is_err());
    }

    #[test]
    fn experimental_loadout_requires_three_unlocked_tools() {
        let engine = GameEngine::new(unlocked_profile());
        let ok = engine.start_run(
            GameMode::Experimental,
            Some(vec![
                "turn-buffer".to_string(),
                "slow-window".to_string(),
                "soft-wrap".to_string(),
            ]),
        );
        assert!(ok.is_ok());

        let bad_len = engine.start_run(
            GameMode::Experimental,
            Some(vec!["turn-buffer".to_string()]),
        );
        assert!(bad_len.is_err());
    }

    #[test]
    fn locked_tools_cannot_be_equipped() {
        let engine = GameEngine::new(Profile::default());
        let run = engine.start_run(
            GameMode::Experimental,
            Some(vec![
                "turn-buffer".to_string(),
                "slow-window".to_string(),
                "soft-wrap".to_string(),
            ]),
        );
        assert!(run.is_err());
    }

    #[test]
    fn active_loadout_is_snapshot_and_immutable_during_run() {
        let engine = GameEngine::new(unlocked_profile());
        let mut run = engine
            .start_run(
                GameMode::Experimental,
                Some(vec![
                    "turn-buffer".to_string(),
                    "slow-window".to_string(),
                    "soft-wrap".to_string(),
                ]),
            )
            .unwrap();

        assert_eq!(
            run.runtime_loadout().unwrap().summary(),
            "turn-buffer+slow-window+soft-wrap"
        );

        let change_result = run.update_runtime_loadout(ToolLoadout {
            slots: [
                "turn-buffer".to_string(),
                "slow-window".to_string(),
                "rewind-step".to_string(),
            ],
        });
        assert!(change_result.is_err());
    }

    #[test]
    fn challenge_rows_keep_mode_and_loadout_metadata() {
        let mut engine = GameEngine::new(unlocked_profile());
        let mut run = engine.start_run(GameMode::Challenge, None).unwrap();
        run.metrics.survival_ticks = 7;
        run.metrics.food_eaten = 2;
        engine.finish_run(&run).unwrap();

        let row = &engine.leaderboards.rows(GameMode::Challenge)[0];
        assert_eq!(row.mode, GameMode::Challenge);
        assert_eq!(row.loadout_summary, "none");
    }

    #[test]
    fn no_daily_or_weekly_menu_surfaces_exist() {
        let engine = GameEngine::new(Profile::default());
        let items = engine.build_menu_items();
        assert!(!items.iter().any(|i| i.contains("Daily")));
        assert!(!items.iter().any(|i| i.contains("Weekly")));
    }

    #[test]
    fn e2e_mode_rule_coverage() {
        let mut engine = GameEngine::new(unlocked_profile());

        let mut practice = engine.start_run(GameMode::Practice, None).unwrap();
        practice.tick();
        engine
            .handle_collision(&mut practice, Point { x: 0, y: 0 })
            .unwrap();
        assert!(practice.ended);

        let mut invincible = engine.start_run(GameMode::Invincible, None).unwrap();
        invincible.add_food(50);
        engine
            .handle_collision(&mut invincible, Point { x: 5, y: 5 })
            .unwrap();
        assert!(!invincible.ended);
        engine.finish_run(&invincible).unwrap();
        assert!(engine.profile.invincible_cumulative_length >= 50);
    }
}
