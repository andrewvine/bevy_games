pub mod meteor;
mod player;

use meteor::MeteorPlugin;
use player::PlayerPlugin;

use bevy::prelude::*;


pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
            .add_plugins((
                MeteorPlugin,
                PlayerPlugin,
            ));
    }
}
