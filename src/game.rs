// Import Bevy game engine essentials
use bevy::{prelude::*, app::AppExit};
use bevy_kira_audio::{Audio, AudioControl};
// Import components, resources, and events
use crate::derivables::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
			.add_systems(Update, (
				pitch,
				swing_bat,
				update_strike_sprite,
			).run_if(in_state(GameState::Level)))
			.add_systems(Update, (
				quit_game,
			))
		;
	}
}

fn pitch(
	time: Res<Time>,
	audio: Res<Audio>,
	asset_server: Res<AssetServer>,
	mut hit: ResMut<BallHit>,
	mut strikes: ResMut<Strikes>,
	mut score: ResMut<Score>,
	mut pitch_timer: ResMut<PitchTimer>,
	mut pitch_delay: ResMut<PitchDelay>,
	mut pitcher_query: Query<(&mut TextureAtlasSprite, With<Pitcher>)>,
	mut next_state: ResMut<NextState<GameState>>,
) {
	for (mut sprite, _) in pitcher_query.iter_mut() {
		if sprite.index == 0 {
			pitch_delay.0.tick(time.delta().mul_f32(rand::random::<f32>() + 1.0 + (score.0 as f32/10.0)));
			if pitch_delay.0.just_finished() {
				if strikes.0 == 3 {
					next_state.set(GameState::End);
					strikes.0 = 0;
				} else {
					sprite.index = sprite.index + 1;
				}
			}
		} else {
			pitch_timer.0.tick(time.delta().mul_f32(rand::random::<f32>() + 1.0 + (score.0 as f32/10.0)));
			if pitch_timer.0.just_finished() {
				if sprite.index < 3 {
					sprite.index = sprite.index + 1;
				} else if sprite.index == 3 && hit.0 == false {
					sprite.index = 4;
					hit.0 = false;
					strikes.0 += 1;
					audio.play(asset_server.load("sfx/miss.wav")).with_volume(0.6);
				} else if sprite.index == 3 && hit.0 == true {
					sprite.index = 5;
					hit.0 = false;
					score.0 += 1;
					audio.play(asset_server.load("sfx/hit.wav")).with_volume(0.6);
				} else if sprite.index == 4 {
					sprite.index = 0;
				} else if sprite.index == 6 {
					sprite.index = (sprite.index + 1) % 8;
					audio.play(asset_server.load("sfx/home_run.wav")).with_volume(0.6);
				} else if sprite.index > 4 {
					sprite.index = (sprite.index + 1) % 8;
				}
			}
		}
	}
}

fn swing_bat(
	time: Res<Time>,
	keyboard: Res<Input<KeyCode>>,
	audio: Res<Audio>,
	asset_server: Res<AssetServer>,
	pitcher_query: Query<(&TextureAtlasSprite, (With<Pitcher>, Without<Batter>))>,
	mut batter_query: Query<(&mut TextureAtlasSprite, With<Batter>)>,
	mut hit: ResMut<BallHit>,
	mut swing_timer: ResMut<SwingTimer>,
) {
	for (mut sprite, _) in batter_query.iter_mut() {
		if sprite.index != 0 {
			swing_timer.0.tick(time.delta());
			if swing_timer.0.just_finished() {
				sprite.index = (sprite.index + 1) % 6;
			}
		} else {
			if keyboard.just_pressed(KeyCode::Space) {
				audio.play(asset_server.load("sfx/swing.wav")).with_volume(0.6);
				sprite.index = (sprite.index + 1) % 6;
				for (sprite, _) in pitcher_query.iter() {
					if sprite.index == 2 {
						hit.0 = true;
					}
				}
			}
		}
	}
}

fn update_strike_sprite(
	strikes: Res<Strikes>,
	mut strike_query: Query<(&mut TextureAtlasSprite, With<StrikeSprite>)>,
) {
	for (mut sprite, _) in strike_query.iter_mut() {
		sprite.index = strikes.0.clamp(0, 3);
	}
}

fn quit_game(
	keyboard: Res<Input<KeyCode>>,
	mut ev_w_exit: EventWriter<AppExit>,
) {
	if keyboard.just_pressed(KeyCode::Escape) {
		ev_w_exit.send(AppExit);
	}
}