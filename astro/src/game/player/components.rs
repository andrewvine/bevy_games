use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub angle: f32,
}

#[derive(Component)]
pub struct Fire {
}

#[derive(Component)]
pub struct Laser {
    pub direction: Vec2,
}
