/* SPDX-License-Identifier: MIT
 * Copyright (c) 2024 Louis Mayencourt
 *
 * Provides the windows and physics simulation
 */

use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
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

/// Track size
const TRACK_WIDTH: f32 = 320.0;

impl Plugin for SimulatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0));
        // app.add_plugins(RapierDebugRenderPlugin::default());
        app.add_systems(Startup, setup_graphics);
        app.add_systems(Startup, setup_physics);
        app.add_systems(Update, bevy::window::close_on_esc);
        app.add_systems(FixedUpdate, cart_control);
    }
}

fn setup_graphics(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_physics(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands
) {
    let color_green = Color::rgb(134.0/255.0, 161.0/255.0, 99.0/255.0);
    let color_orange = Color::rgb(198.0/255.0, 113.0/255.0, 94.0/255.0);
    let color_blue = Color::rgb(132.0/255.0, 166.0/255.0, 199.0/255.0);
    let color_white = Color::rgb(233.0/255.0, 228.0/255.0, 217.0/255.0);

    // Create the pendulum cart
    let parent = commands.spawn(
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Capsule2d{radius: PENDULUM_CART_WIDTH/4.0, half_length: PENDULUM_CART_HEIGHT/2.0})),
            material: materials.add(color_blue),
            transform: Transform {
                translation: Vec3::splat(0.0),
                rotation: Quat::from_rotation_z(3.1415/2.0),
                ..default()
            },
            ..default()
        })
    .insert(Cart)
    .insert(RigidBody::Dynamic)
    .insert(Collider::cuboid(PENDULUM_CART_HEIGHT/2.0, PENDULUM_CART_WIDTH/2.0))
    .insert(ColliderMassProperties::Mass(CART_MASS))
    .insert(ExternalForce::default())
    .insert(Velocity::default())
    .insert(LockedAxes::ROTATION_LOCKED)
    .insert(LockedAxes::TRANSLATION_LOCKED_Y)
    .id();

    // Create the pendulum mass
    let joint = RevoluteJointBuilder::new().local_anchor2(Vec2::new(0.0, PENDULUM_HEIGHT));
    commands
            .spawn((
                MaterialMesh2dBundle {
                    mesh: Mesh2dHandle(meshes.add(Circle{radius: 10.0})),
                    material: materials.add(color_orange),
                    transform: Transform {
                        translation: Vec3::new(0.0, PENDULUM_HEIGHT, 0.0),
                        ..default()
                    },
                    ..default()
                },
                RigidBody::Dynamic,
                Collider::ball(10.0),
                ColliderMassProperties::Mass(PENDULUM_MASS),
                GravityScale(1.0),
            ))
            .insert(ImpulseJoint::new(parent, joint));

    // Create the cart track
    commands.spawn(
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Capsule2d{radius: 2.0, half_length: TRACK_WIDTH})),
            material: materials.add(color_white),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, -10.0),
                rotation: Quat::from_rotation_z(3.1415/2.0),
                ..default()
            },
            ..default()
        },
    );
}

fn cart_control(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut ExternalForce, &mut Velocity, &mut Transform), With<Cart>>,
) {
    let (mut cart_force, mut cart_velocity, mut transform) = query.single_mut();

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        cart_force.force.x = - MOVING_FORCE;
    } else if keyboard_input.pressed(KeyCode::ArrowRight) {
        cart_force.force.x = MOVING_FORCE;
    } else {
        cart_force.force.x = 0.0;
    }

    limit_horizontal_speed(&mut cart_velocity);
    limit_horizontal_position(&mut transform, &mut cart_velocity);
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

fn limit_horizontal_position(
    transform: &mut Transform,
    velocity: &mut Velocity,
) {
    if transform.translation.x > TRACK_WIDTH {
        transform.translation.x = TRACK_WIDTH;
        velocity.linvel.x = 0.0;
    }
    if transform.translation.x < -TRACK_WIDTH {
        transform.translation.x = -TRACK_WIDTH;
        velocity.linvel.x = 0.0;
    }
}