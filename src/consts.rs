/// Speed at which a Slow arrow moves
pub const BASE_SPEED: f32 = 200.;
/// X coordinate value at which arrows spawn, should be out of screen
pub const SPAWN_POSITION: f32 = -400.;
/// X coordinate value where the arrows should be clicked
pub const TARGET_POSITION: f32 = 200.;
/// Margin of error for clicking an arrow
pub const THRESHOLD: f32 = 20.;
/// Total distance traveled by an arrow, from spawn to target
pub const DISTANCE: f32 = TARGET_POSITION - SPAWN_POSITION;

/// Stage for our systems
pub const APP_STATE_STAGE: &str = "app_state_stage";

/// States
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    Menu,
    Game,
    MakeMap,
}
