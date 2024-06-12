use std::ops::Range;
use bevy::math::Vec3;
use bevy::prelude::{Commands, Component, DespawnRecursiveExt, Entity, Query, Res, ResMut, Resource, SceneBundle, Time, TimerMode, Transform, With};
use bevy::app::{App, Plugin, Update};
use bevy::time::Timer;
use bevy::utils::default;
use rand::Rng;
use crate::asset_loader::SceneAssets;
use crate::colision_detection::Collider;
use crate::movement::{Acceleration, MovingObjectBundle, Velocity};

const VELOCITY_SCALAR: f32 = 5.0;
const ACCELERATION_SCALAR: f32 = 1.0;
const SPAWN_RANGE_X: Range<f32> = -25.0..25.0;
const SPAWN_RANGE_Y: Range<f32> = 0.0..25.0;
const SPAWN_TIME_SECONDS: f32 = 1.0;
const ROTATE_SPEED: f32 = 10.0;
const RADIUS:f32 = 2.5;
#[derive(Component, Debug)]
pub struct Asteroid;
#[derive(Resource, Debug)]
pub struct SpawnTimer {
    timer: Timer,
}

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(SpawnTimer{
                timer: Timer::from_seconds(SPAWN_TIME_SECONDS, TimerMode::Repeating)}
            )
            .add_systems(Update, (spawn_asteroid, rotate_asteroid, handle_asteroid_collisions));
    }
}

fn spawn_asteroid(
    mut commands: Commands,
    mut spawn_timer: ResMut<SpawnTimer>,
    time: Res<Time>,
    scene_assets: Res<SceneAssets>,
) {
    spawn_timer.timer.tick(time.delta());
    if !spawn_timer.timer.just_finished() {
        return;
    }

    let mut rng = rand::thread_rng();

    let translation = Vec3::new(
        rng.gen_range(SPAWN_RANGE_X),
        0.0,
        rng.gen_range(SPAWN_RANGE_Y),
    );

    let mut rand_unit_vector =
        || Vec3::new(rng.gen_range(-1.0..1.0), 0.,  rng.gen_range(-1.0..1.0)).normalize_or_zero();

    let velocity = rand_unit_vector() * VELOCITY_SCALAR;
    let acceleration = rand_unit_vector() * ACCELERATION_SCALAR;

    commands.spawn((MovingObjectBundle{
        velocity: Velocity::new(velocity),
        acceleration: Acceleration::new(acceleration),
        collider: Collider::new(RADIUS),
        model: SceneBundle{
            scene: scene_assets.asteroid.clone(),
            transform: Transform::from_translation(translation),
            ..default()
        },
    }, Asteroid));
}

fn rotate_asteroid(
    mut query: Query<&mut Transform, With<Asteroid>>,
    time: Res<Time>
) {
    for mut transform in query.iter_mut() {
        transform.rotate_local_z(ROTATE_SPEED * time.delta_seconds());
    }
}

fn handle_asteroid_collisions(
    mut commands: Commands,
    query: Query<(Entity, &Collider), With<Asteroid>>
) {
    for (entity, collider) in query.iter() {
        for &collided_entity in collider.colliding_entities.iter() {
            if query.get(collided_entity).is_ok() {
                continue;
            }
            commands.entity(entity).despawn_recursive();
        }
    }
}