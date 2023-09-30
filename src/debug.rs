// Import Bevy game engine essentials
use bevy::prelude::*;

//use crate::derivables::*;

// Plugin for devtools only available in the
// debug version of the game
pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
			.add_systems( Startup, (
				filler,
			))
			.add_systems( Update, (
				filler,
			))
		;
	}
}

fn filler(

) {

}

