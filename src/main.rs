// Disable Windows console on release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Import Bevy game engine essentials
use bevy::{prelude::*, window::WindowResolution};
use derivables::*;

// MODULES
mod game;
mod post_processing;
mod setup;
mod derivables;

// Only include in debug builds
#[cfg(debug_assertions)]
mod debug;

// Can't forget main!
fn main() {
	// Create app to hold all our plugins, resources, events, and systems
	let mut app = App::new();
	let mut resolution = WindowResolution::new(ORTHO_WIDTH, ORTHO_HEIGHT);
	resolution.set_physical_resolution(640, 640);
	app.insert_resource(Msaa::Off);
	app
		// Default plugins provided by Bevy handles all essentials for a game
		// such as the game window, asset management, input handling, and time
		.add_plugins(DefaultPlugins
			.set(WindowPlugin {
				primary_window: Some(Window {
					resolution: resolution,
					resizable: false,
					decorations: false,
					position: WindowPosition::Centered(MonitorSelection::Primary),
					// Set custom window title
					title: "Space Batter".to_string(),
					..default()
				}),
				..default()
			})
			// Prevents pixel art sprites from becoming blurry
			.set(ImagePlugin::default_nearest())
		)

		// Plugins
		.add_plugins((
			// Kira audio plugin for Bevy for playing sound files
			bevy_kira_audio::AudioPlugin,
			game::GamePlugin,
			post_processing::PostProcessingPlugin,
			setup::SetupPlugin,
		))
		;

	{
		// Only include in debug builds
		#[cfg(debug_assertions)]
		app
			// Debug module for dev tools
			.add_plugins(debug::DebugPlugin)
		;
	}

	app.run();
}