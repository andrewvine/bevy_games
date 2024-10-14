pub mod components;
mod resources;
mod systems;

use resources::*;
use systems::*;

use crate::AppState;

use bevy::prelude::*;


pub const METEOR_SIZE: f32 = 84.0; // The enemy sprite is 64x64 pixels.
pub const NUMBER_OF_METEORS: usize = 4;

pub struct MeteorPlugin;

impl Plugin for MeteorPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<MeteorSpawnTimer>()
            .add_systems(OnEnter(AppState::Game), spawn_meteors)
            .add_systems(
                Update,
                (
                    meteor_movement,
                    tick_meteor_spawn_timer,
                    spawn_meteors_over_time,
                    despawn_meteor,
                )
            )
            .add_systems(OnExit(AppState::Game), despawn_meteors);
    }
}
