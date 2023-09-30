// Import Bevy game engine essentials
use bevy::{prelude::*, core_pipeline::clear_color::ClearColorConfig, render::camera::ScalingMode};
use bevy_kira_audio::{Audio, AudioControl};
// Import components, resources, and events
use crate::{derivables::*, post_processing::PostProcessSettings};

// Plugin for handling all initial one time setup 
// such as camera spawning, loading save data and 
// initializing resources
pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app
			// States
			.add_state::<GameState>()
			.add_state::<PauseState>()
			// Resources
			.insert_resource(BallHit(false))
			.insert_resource(Strikes(0))
			.insert_resource(Score(0))
			.insert_resource(BootTimer(Timer::from_seconds(BOOT_DURATION, TimerMode::Repeating)))
			.insert_resource(SwingTimer(Timer::from_seconds(SWING_SPEED, TimerMode::Repeating)))
			.insert_resource(PitchTimer(Timer::from_seconds(PITCH_SPEED, TimerMode::Repeating)))
			.insert_resource(PitchDelay(Timer::from_seconds(PITCHER_PATIENCE, TimerMode::Repeating)))
			// Systems
			.add_systems( Startup,(
				spawn_camera,
				spawn_splash_screen,
			))
			.add_systems( Update,(
				advance_splash_screen,
			).run_if(in_state(GameState::Boot)))
			.add_systems( Update,(
				advance_splash_screen,
			).run_if(in_state(GameState::End)))
			.add_systems( Update,(
				advance_menu,
			).run_if(in_state(GameState::Menu)))
			.add_systems(OnEnter(GameState::Menu), (
				spawn_menu,
			))
			.add_systems(OnEnter(GameState::End), (
				spawn_end,
			))
			.add_systems(OnExit(GameState::Boot), (
				despawn_entities_with::<DespawnOnExitGameState>,
			))
			.add_systems(OnExit(GameState::Menu), (
				despawn_entities_with::<MenuText>,
			))
			.add_systems(OnExit(GameState::Level), (
				despawn_entities_with::<DespawnOnExitGameState>,
			))
			.add_systems(OnExit(GameState::End), (
				despawn_entities_with::<DespawnOnExitGameState>,
			))
		;
	}
}

fn spawn_camera(
	mut commands: Commands,
) {
	// Main camera
	commands.spawn((
		Camera2dBundle{
			camera_2d: Camera2d{
				clear_color: ClearColorConfig::Custom(Color::BLACK),
				..default()
			},
			transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1000.0)),
			projection: OrthographicProjection {
				scaling_mode: ScalingMode::Fixed{width: ORTHO_WIDTH, height: ORTHO_HEIGHT},
				..Default::default()
			},
			..default()
		},
		PostProcessSettings {
            intensity: 0.0,
            ..default()
        },
	));
}

fn spawn_splash_screen(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	audio: Res<Audio>,
) {
	commands.spawn((
		SpriteBundle{
			transform: Transform::from_xyz(0.0, 0.0, 0.0),
			texture: asset_server.load("sprites/splash.png"),
			sprite: Sprite {
				custom_size: Some(Vec2::new(ORTHO_WIDTH, ORTHO_HEIGHT)),
				..default()
			},
			..default()
		},
		DespawnOnExitGameState,
	));

	audio.play(asset_server.load("bgm/ball_game.wav")).looped().with_volume(1.0);
}

fn spawn_menu(
	mut commands: Commands,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
	asset_server: Res<AssetServer>,
) {
	commands.spawn((
		SpriteBundle{
			transform: Transform::from_xyz(0.0, 0.0, 0.0),
			texture: asset_server.load("sprites/menu.png"),
			sprite: Sprite {
				custom_size: Some(Vec2::new(ORTHO_WIDTH, ORTHO_HEIGHT)),
				..default()
			},
			..default()
		},
		MenuText,
	));

	commands
		.spawn((SpriteSheetBundle {
			transform: Transform::from_xyz(1.0, -39.0, 200.0),
			texture_atlas: texture_atlases.add(TextureAtlas::from_grid(asset_server.load("sprites/batter.png"), Vec2::new(120.0, 80.0), 2, 3, None, None)).clone(),
			sprite: TextureAtlasSprite{
				index: 0,
				custom_size: Some(Vec2::new(320.0, 240.0)),
				..default()
			},
			..default()
		},
		Batter,
		DespawnOnExitGameState,
	));
}

fn spawn_end(
	asset_server: Res<AssetServer>,
	mut commands: Commands,
	mut score: ResMut<Score>,
) {
	commands.spawn((
		SpriteBundle{
			transform: Transform::from_xyz(0.0, 0.0, 0.0),
			texture: asset_server.load("sprites/end_screen.png"),
			sprite: Sprite {
				custom_size: Some(Vec2::new(ORTHO_WIDTH, ORTHO_HEIGHT)),
				..default()
			},
			..default()
		},
		DespawnOnExitGameState,
	));
	commands.spawn((
		Text2dBundle{
			transform: Transform::from_xyz(0.0, -80.0, 50.0),
			text: Text::from_sections([TextSection::new(format!("Score: {}", score.0), TextStyle {
				font_size: 32.0,
				color: Color::WHITE,
				..default()
			})]),
			..default()
		},
		DespawnOnExitGameState,
	));
	score.0 = 0;
}

fn advance_splash_screen(
	keyboard: Res<Input<KeyCode>>,
	time: Res<Time>,
	mut boot_timer: ResMut<BootTimer>,
	mut next_game_state: ResMut<NextState<GameState>>,
) {
	boot_timer.0.tick(time.delta());
	if keyboard.just_pressed(KeyCode::Space) 
	|| boot_timer.0.just_finished() {
		next_game_state.set(GameState::Menu);
		boot_timer.0.reset();
	}
}

fn advance_menu(
	keyboard: Res<Input<KeyCode>>,
	asset_server: Res<AssetServer>,
	audio: Res<Audio>,
	mut commands: Commands,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
	mut batter_query: Query<(&mut TextureAtlasSprite, With<Batter>)>,
	mut next_game_state: ResMut<NextState<GameState>>,
) {
	if keyboard.just_pressed(KeyCode::Space) {
		audio.play(asset_server.load("sfx/swing.wav")).with_volume(0.6);
		for (mut sprite, _) in batter_query.iter_mut() {
			sprite.index = (sprite.index + 1) % 6;
		}
		commands
			.spawn((SpriteSheetBundle {
				transform: Transform::from_xyz(0.0, 0.0, 100.0),
				texture_atlas: texture_atlases.add(TextureAtlas::from_grid(asset_server.load("sprites/pitcher.png"), Vec2::new(320.0, 320.0), 4, 2, None, None)).clone(),
				sprite: TextureAtlasSprite{
					index: 0,
					custom_size: Some(Vec2::new(320.0, 320.0)),
					..default()
				},
				..default()
			},
			Pitcher,
			DespawnOnExitGameState,
		));
		commands
			.spawn((SpriteSheetBundle {
				transform: Transform::from_xyz(0.0, 144.0, 150.0),
				texture_atlas: texture_atlases.add(TextureAtlas::from_grid(asset_server.load("sprites/strikes.png"), Vec2::new(320.0, 32.0), 1, 4, None, None)).clone(),
				sprite: TextureAtlasSprite{
					index: 0,
					custom_size: Some(Vec2::new(320.0, 32.0)),
					..default()
				},
				..default()
			},
			StrikeSprite,
			DespawnOnExitGameState,
		));
		next_game_state.set(GameState::Level);
	}
}

// Generic function used for despawning all entities with a specific component,
// mainly used for cleanup on state transitions
pub fn despawn_entities_with<T: Component>(
	mut commands: Commands,
	to_despawn: Query<Entity, With<T>>, 
) {
	for entity in &to_despawn {
		commands.entity(entity).despawn_recursive();
	}
}