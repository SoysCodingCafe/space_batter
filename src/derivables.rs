// Import Bevy game engine essentials
use bevy::prelude::*;

// CONSTANTS
// Window Resolution
pub const ORTHO_WIDTH: f32 = 320.0;
pub const ORTHO_HEIGHT: f32 = 320.0;

pub const BOOT_DURATION: f32 = 5.0;
pub const SWING_SPEED: f32 = 0.125;
pub const PITCH_SPEED: f32 = 0.3;
pub const PITCHER_PATIENCE: f32 = 3.0;

// STATES
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
	#[default]
	Boot,
	Menu,
	Level,
	End,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum PauseState {
	#[default]
	Unpaused,
	Paused,
}

// COMPONENTS
#[derive(Component)]
pub struct DespawnOnExitGameState;

#[derive(Component)]
pub struct MenuText;

#[derive(Component)]
pub struct StrikeSprite;

#[derive(Component)]
pub struct Batter;

#[derive(Component)]
pub struct Pitcher;


// RESOURCES
#[derive(Resource)]
pub struct Strikes(pub usize);

#[derive(Resource)]
pub struct Score(pub usize);

#[derive(Resource)]
pub struct BallHit(pub bool);

#[derive(Resource)]
pub struct BootTimer(pub Timer);

#[derive(Resource)]
pub struct SwingTimer(pub Timer);

#[derive(Resource)]
pub struct PitchDelay(pub Timer);

#[derive(Resource)]
pub struct PitchTimer(pub Timer);