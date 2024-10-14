use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::components::{Player, Fire, Laser};

use crate::game::meteor::components::*;
use crate::game::meteor::METEOR_SIZE;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 74.0; // This is the player sprite size.
pub const PLAYER_ROTATION_SPEED: f32 = 90.0; // Degrees per second.

pub const FIRE_SIZE: f32 = 50.0; 
pub const LASER_SIZE: f32 = 50.0; 
pub const LASER_SPEED: f32 = 500.0;



pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    let start_angle: f32 = 90.0;
    let player = commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/playerShip1_red.png"),
            ..default()
        },
        Player {
            angle: start_angle.to_radians(),
        },
    )).id();
    let fire = commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0.0, -1.0 * FIRE_SIZE, 0.0),
            texture: asset_server.load("sprites/fire03.png"),
            visibility: Visibility::Hidden,
            ..default()
        },
        Fire {},
    )).id();
    commands.entity(player).push_children(&[fire]);
}

pub fn despawn_player(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    if let Ok(player_entity) = player_query.get_single() {
        commands.entity(player_entity).despawn_recursive();
    }
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut Player)>,
    mut fire_query: Query<(&mut Visibility, &Fire)>,
    time: Res<Time>,
) {
    if let Ok((mut transform, mut player)) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        let rotation_speed = f32::to_radians(PLAYER_ROTATION_SPEED);

        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
            let change = rotation_speed * time.delta_seconds(); 
            player.angle += change;
            transform.rotate_z(change);
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
            let change = -1.0 * rotation_speed * time.delta_seconds(); 
            player.angle += change;
            transform.rotate_z(change);
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
            direction = Vec3::new(player.angle.cos(), player.angle.sin(), 0.0);
            if direction.length() > 0.0 {
                direction = direction.normalize();
            }
            transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
            if let Ok((mut fire_visibility, _)) = fire_query.get_single_mut() {
                *fire_visibility = Visibility::Visible;
            }
        }
        if keyboard_input.just_released(KeyCode::ArrowUp){
            if let Ok((mut fire_visibility, _)) = fire_query.get_single_mut() {
                *fire_visibility = Visibility::Hidden;
            }

        }
    }
}

pub fn handle_fire_laser(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&Transform, &Player)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if let Ok((mut transform, mut player)) = player_query.get_single() {
        if keyboard_input.just_released(KeyCode::Space) {
            let mut direction = Vec3::new(player.angle.cos(), player.angle.sin(), 0.0);
            if direction.length() > 0.0 {
                direction = direction.normalize();
            }
            let mut transform = Transform::from_translation(transform.translation).with_rotation(transform.rotation);
            let sound_effect = asset_server.load("audio/laserLarge_000.ogg");

            commands.spawn((
                AudioBundle {
                    source: sound_effect,
                    settings: PlaybackSettings::ONCE
                },
                SpriteBundle {
                    transform: transform,
                    texture: asset_server.load("sprites/laserRed03.png"),
                    ..default()
                },
                Laser {
                    direction: Vec2::new(direction.x, direction.y)
                },
            ));
        }
    }
}

pub fn laser_movement(mut laser_query: Query<(&mut Transform, &Laser)>, time: Res<Time>) {
    for (mut transform, laser) in laser_query.iter_mut() {
        let direction = Vec3::new(laser.direction.x, laser.direction.y, 0.0);
        transform.translation += direction * LASER_SPEED * time.delta_seconds();
    }
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let half_player_size = PLAYER_SIZE / 2.0; // 32.0
        let x_min = 0.0 + half_player_size;
        let x_max = window.width() - half_player_size;
        let y_min = 0.0 + half_player_size;
        let y_max = window.height() - half_player_size;

        let mut translation = player_transform.translation;

        // Bound the player x position
        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }
        // Bound the players y position.
        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        player_transform.translation = translation;
    }
}

pub fn meteor_hit_player(
    mut commands: Commands,
    player_query: Query<(Entity, &Transform), With<Player>>,
    meteor_query: Query<&Transform, With<Meteor>>,
    asset_server: Res<AssetServer>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single() {
        for meteor_transform in meteor_query.iter() {
            let distance = player_transform
                .translation
                .distance(meteor_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let meteor_radius = METEOR_SIZE / 2.0;
            if distance < player_radius + meteor_radius {
                let sound_effect = asset_server.load("audio/explosionCrunch_000.ogg");
                commands.spawn(AudioBundle {
                    source: sound_effect,
                    settings: PlaybackSettings::DESPAWN,
                });
                commands.entity(player_entity).despawn_recursive();
            }
        }
    }
}

pub fn laser_hit_meteor(
    mut commands: Commands,
    laser_query: Query<(Entity, &Transform), With<Laser>>,
    meteor_query: Query<(Entity, &Transform, &Meteor), With<Meteor>>,
    asset_server: Res<AssetServer>,
) {
    for (laser_entity, laser_transform) in laser_query.iter() {
        for (meteor_entity, meteor_transform, meteor) in meteor_query.iter() {
            let distance = laser_transform
                .translation
                .distance(meteor_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let enemy_radius = meteor.size as f32 / 2.0;

            if distance < player_radius + enemy_radius {
                let sound_effect = asset_server.load("audio/explosionCrunch_000.ogg");
                commands.spawn((
                    AudioBundle {
                        source: sound_effect,
                        settings: PlaybackSettings::DESPAWN
                    },
                    SpriteBundle {
                        transform: Transform::from_translation(meteor_transform.translation),
                        texture: asset_server.load(meteor.explosion()), 
                        ..default() },

                ));
                commands.entity(meteor_entity).despawn();
                commands.entity(laser_entity).despawn();
            }
        }
    }
}


pub fn despawn_laser(
    mut commands: Commands, 
    enemy_query: Query<(Entity, &Transform), With<Laser>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    let half_laser_size = LASER_SIZE / 2.0;
    let x_min = 0.0 + half_laser_size;
    let x_max = window.width() - half_laser_size;
    let y_min = 0.0 + half_laser_size;
    let y_max = window.height() - half_laser_size;

    for (laser_entity, transform ) in enemy_query.iter() {
        let translation = transform.translation;

        if translation.x < x_min || translation.x > x_max {
            commands.entity(laser_entity).despawn();
        }
        if translation.y < y_min || translation.y > y_max {
            commands.entity(laser_entity).despawn();
        }
    }
}