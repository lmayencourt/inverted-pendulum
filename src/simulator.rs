/* SPDX-License-Identifier: MIT
 * Copyright (c) 2024 Louis Mayencourt
 *
 * Provides the windows and physics simulation
 */

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct SimulatorPlugin;

/// Cart component for control queries
#[derive(Component)]
pub struct Cart;
/// Cart sprite size
const PENDULUM_CART_HEIGHT: f32 = 40.0;
const PENDULUM_CART_WIDTH: f32 = PENDULUM_CART_HEIGHT * 1.618;
/// Cart physics constants
const CART_MASS: f32 = 100.0;
const MAX_CART_SPEED: f32 = 250.0;
// Force to apply to reach MAX_CART_SPEED in 2 secs
const MOVING_FORCE: f32 = CART_MASS / 2.0 * 10.0 * MAX_CART_SPEED;

/// Pendulum size
const PENDULUM_WIDTH: f32 = 20.0;
const PENDULUM_HEIGHT: f32 = 100.0;
/// Cart physics constants
const PENDULUM_MASS: f32 = 10.0;

impl Plugin for SimulatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0));
        app.add_plugins(RapierDebugRenderPlugin::default());
        app.add_systems(Startup, setup_graphics);
        app.add_systems(Startup, setup_physics);
        app.add_systems(Update, bevy::window::close_on_esc);
        app.add_systems(FixedUpdate, cart_control);
    }
}

fn setup_graphics(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_physics(mut commands: Commands) {
    let parent = commands.spawn(SpriteBundle{
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            scale: Vec3::new(PENDULUM_CART_WIDTH, PENDULUM_CART_HEIGHT, 1.0),
            ..default()
        },
        ..default()
    })
    .insert(Cart)
    .insert(RigidBody::Dynamic)
    .insert(Collider::cuboid(0.5, 0.5))
    .insert(ColliderMassProperties::Mass(CART_MASS))
    .insert(ExternalForce::default())
    .insert(Velocity::default())
    .insert(LockedAxes::ROTATION_LOCKED)
    .insert(LockedAxes::TRANSLATION_LOCKED_Y)
    .id();

    let joint = RevoluteJointBuilder::new().local_anchor2(Vec2::new(0.0, PENDULUM_HEIGHT));
    commands
            .spawn((
                SpriteBundle{
                    transform: Transform {
                        translation: Vec3::new(0.0, PENDULUM_HEIGHT, 0.0),
                        scale: Vec3::new(PENDULUM_WIDTH, PENDULUM_WIDTH, 1.0),
                        ..default()
                    },
                    ..default()
                },
                RigidBody::Dynamic,
                Collider::cuboid(0.5, 0.5),
                ColliderMassProperties::Mass(PENDULUM_MASS),
                GravityScale(1.0),
            ))
            .insert(ImpulseJoint::new(parent, joint));
}

fn cart_control(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut ExternalForce, &mut Velocity), With<Cart>>,
) {
    let (mut cart_force, mut cart_velocity) = query.single_mut();

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        cart_force.force.x = - MOVING_FORCE;
    } else if keyboard_input.pressed(KeyCode::ArrowRight) {
        cart_force.force.x = MOVING_FORCE;
    } else {
        cart_force.force.x = 0.0;
    }

    limit_horizontal_speed(&mut cart_velocity);
}

fn limit_horizontal_speed(
    velocity: &mut Velocity,
) {
    if velocity.linvel.x > MAX_CART_SPEED {
        velocity.linvel.x = MAX_CART_SPEED;
    }
    if velocity.linvel.x < -MAX_CART_SPEED {
        velocity.linvel.x = -MAX_CART_SPEED;
    }
}