use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

use super::components::*;
use super::resources::*;
use super::{METEOR_SIZE, NUMBER_OF_METEORS};



pub fn spawn_meteors(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    let half_meteor_size = METEOR_SIZE / 2.0; // 32.0
    let y_max = window.height() - half_meteor_size;

    for _ in 0..NUMBER_OF_METEORS {
        let meteor_type = random_type();

        let random_x = random::<f32>() * window.width();
        let y = y_max;

        let mut direction = Vec2::new(random::<f32>(), random::<f32>()).normalize();
        direction.y *= -1.0;

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, y, 0.0),
                texture: asset_server.load(&meteor_type.image),
                ..default()
            },
            Meteor {
                direction: direction,
                size: meteor_type.size,
                speed: meteor_type.speed,
            },
        ));
    }
}

pub fn despawn_meteors(mut commands: Commands, meteor_query: Query<Entity, With<Meteor>>) {
    for meteor_entity in meteor_query.iter() {
        commands.entity(meteor_entity).despawn();
    }
}

pub fn meteor_movement(mut meteor_query: Query<(&mut Transform, &Meteor)>, time: Res<Time>) {
    for (mut transform, meteor) in meteor_query.iter_mut() {
        let direction = Vec3::new(meteor.direction.x, meteor.direction.y, 0.0);
        transform.translation += direction * meteor.speed * time.delta_seconds();
    }
}


pub fn despawn_meteor(
    mut commands: Commands, 
    meteor_query: Query<(Entity, (&Transform, &Meteor)), With<Meteor>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();


    for (meteor_entity, (transform, meteor) ) in meteor_query.iter() {
        let half_meteor_size = meteor.size as f32 / 2.0;
        let x_min = 0.0 + half_meteor_size;
        let x_max = window.width() - half_meteor_size;
        let y_min = 0.0 + half_meteor_size;
        let y_max = window.height() - half_meteor_size;

        let translation = transform.translation;

        if translation.x < x_min || translation.x > x_max {
            commands.entity(meteor_entity).despawn();
        }
        if translation.y < y_min || translation.y > y_max {
            commands.entity(meteor_entity).despawn();
        }
    }
}

pub fn tick_meteor_spawn_timer(mut meteor_spawn_timer: ResMut<MeteorSpawnTimer>, time: Res<Time>) {
    meteor_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_meteors_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    meteor_spawn_timer: Res<MeteorSpawnTimer>,
) {
    if meteor_spawn_timer.timer.finished() {
        let meteor_type = random_type();

        let window = window_query.get_single().unwrap();
        let half_meteor_size = meteor_type.size as f32 / 2.0; // 32.0
        let y_max = window.height() - half_meteor_size;


        let random_x = random::<f32>() * window.width();
        let y = y_max;

        let mut direction = Vec2::new(random::<f32>(), random::<f32>()).normalize();
        direction.y *= -1.0;

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, y, 0.0),
                texture: asset_server.load(&meteor_type.image),
                ..default()
            },
            Meteor {
                direction: direction,
                size: meteor_type.size,
                speed: meteor_type.speed,
            },
        ));
    }
}
